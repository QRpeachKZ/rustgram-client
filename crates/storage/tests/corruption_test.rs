//! Corruption and recovery tests for storage layer.
//!
//! These tests verify the database's ability to handle various corruption scenarios
//! including WAL corruption, header corruption, and recovery mechanisms.

use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;
use rustgram_storage::{
    chat::ChatDb,
    connection::DbConnection,
    error::{StorageError, StorageResult},
    user::UserDb,
};

/// Creates a corrupted WAL file for testing.
fn create_corrupted_wal(db_path: &Path) -> StorageResult<()> {
    let wal_path = db_path.with_extension("db-wal");
    let mut file = fs::File::create(&wal_path)?;
    // Write invalid WAL header
    file.write_all(b"CORRUPTED_WAL_DATA")?;
    Ok(())
}

/// Creates a corrupted database header.
fn create_corrupted_header(db_path: &Path) -> StorageResult<()> {
    let mut file = fs::File::create(db_path)?;
    // Write invalid SQLite header
    file.write_all(b"NOT_SQLITE_DATABASE")?;
    Ok(())
}

/// Creates a valid database with some data.
fn create_valid_database(dir: &TempDir) -> StorageResult<DbConnection> {
    let db_path = dir.path().join("test.db");
    let db = DbConnection::new(&db_path)?;

    // Initialize and add some data
    let user_db = UserDb::new(db.clone());
    user_db.init()?;
    let mut user_sync = user_db.sync();
    user_sync.add_user(1, bytes::Bytes::from("test user"), None, None)?;

    Ok(db)
}

// ============================================================================
// WAL Corruption Tests
// ============================================================================

#[test]
fn test_corrupted_wal_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    // Create a valid database first
    {
        let db = DbConnection::new(&db_path).unwrap();
        let user_db = UserDb::new(db.clone());
        user_db.init().unwrap();
        let mut user_sync = user_db.sync();
        user_sync.add_user(1, bytes::Bytes::from("user1"), None, None).unwrap();
        user_sync.add_user(2, bytes::Bytes::from("user2"), None, None).unwrap();
    }

    // Corrupt the WAL file
    create_corrupted_wal(&db_path).unwrap();

    // Attempt to open and use the database
    // SQLite should automatically recover from corrupted WAL
    let db = DbConnection::new(&db_path);
    assert!(db.is_ok(), "Should be able to open database with corrupted WAL");

    let user_db = UserDb::new(db.unwrap());
    user_db.init().unwrap();

    // Verify we can still read data
    let mut user_sync = user_db.sync();
    let result = user_sync.get_user(1);
    // Data might be lost from WAL, but database should be accessible
    assert!(result.is_ok() || result.unwrap_err().is_not_found());
}

#[test]
fn test_missing_wal_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    // Create a valid database
    {
        let db = DbConnection::new(&db_path).unwrap();
        let user_db = UserDb::new(db);
        user_db.init().unwrap();
    }

    // Remove WAL file if it exists
    let wal_path = db_path.with_extension("db-wal");
    let _ = fs::remove_file(&wal_path);

    // Should be able to open database without WAL
    let db = DbConnection::new(&db_path);
    assert!(db.is_ok(), "Should be able to open database without WAL");
}

// ============================================================================
// Header Corruption Tests
// ============================================================================

#[test]
fn test_corrupted_header_detection() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    // Create corrupted database header
    create_corrupted_header(&db_path).unwrap();

    // Should fail to open corrupted database
    let db = DbConnection::new(&db_path);
    assert!(db.is_err(), "Should fail to open database with corrupted header");

    if let Err(StorageError::ConnectionError(err)) = db {
        // SQLite should report corruption or invalid format
        let err_msg = err.to_string().to_lowercase();
        assert!(
            err_msg.contains("file is encrypted") ||
            err_msg.contains("database disk image is malformed") ||
            err_msg.contains("not a database"),
            "Error should indicate corruption: {}",
            err
        );
    } else {
        panic!("Expected ConnectionError for corrupted header");
    }
}

// ============================================================================
// Database Rebuild Tests
// ============================================================================

#[test]
fn test_database_rebuild_from_scratch() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    // Create initial database
    {
        let db = DbConnection::new(&db_path).unwrap();
        let user_db = UserDb::new(db.clone());
        user_db.init().unwrap();
        let mut user_sync = user_db.sync();
        user_sync.add_user(1, bytes::Bytes::from("user1"), None, None).unwrap();
    }

    // Delete database file
    fs::remove_file(&db_path).unwrap();

    // Rebuild database from scratch
    let db = DbConnection::new(&db_path).unwrap();
    let user_db = UserDb::new(db);
    let result = user_db.init();
    assert!(result.is_ok(), "Should be able to rebuild database from scratch");

    // Verify database is empty but functional
    let mut user_sync = user_db.sync();
    assert!(user_sync.get_user(1).is_err(), "Old data should not exist");
    assert_eq!(user_sync.get_user_count().unwrap(), 0);
}

#[test]
fn test_partial_database_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let user_db_path = dir.path().join("users.db");

    // Create multiple databases
    {
        let msg_db = DbConnection::new(&db_path).unwrap();
        let user_db = DbConnection::new(&user_db_path).unwrap();

        let user = UserDb::new(user_db);
        user.init().unwrap();
        let mut user_sync = user.sync();
        user_sync.add_user(1, bytes::Bytes::from("user1"), None, None).unwrap();
    }

    // Corrupt one database
    create_corrupted_header(&db_path).unwrap();

    // Should still be able to use the other database
    let user_db = DbConnection::new(&user_db_path);
    assert!(user_db.is_ok(), "Uncorrelated database should be accessible");

    let user = UserDb::new(user_db.unwrap());
    user.init().unwrap();
    let mut user_sync = user.sync();
    assert!(user_sync.get_user(1).is_ok(), "Data should be intact");
}

// ============================================================================
// Transaction Corruption Tests
// ============================================================================

#[test]
fn test_incomplete_transaction_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    // Create database with data
    let db = DbConnection::new(&db_path).unwrap();
    let user_db = UserDb::new(db.clone());
    user_db.init().unwrap();

    let mut user_sync = user_db.sync();
    user_sync.add_user(1, bytes::Bytes::from("user1"), None, None).unwrap();
    user_sync.add_user(2, bytes::Bytes::from("user2"), None, None).unwrap();

    // Simulate crash during transaction by not committing
    // (This is handled by SQLite's automatic rollback)
    let mut conn = db.connect().unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute("INSERT INTO users (user_id, data) VALUES (3, ?)", [b"partial"]).unwrap();
    // Don't commit - transaction will be rolled back

    // Reopen database
    let db = DbConnection::new(&db_path).unwrap();
    let user_db = UserDb::new(db);
    user_db.init().unwrap();

    // Verify uncommitted data was rolled back
    let mut user_sync = user_db.sync();
    assert!(user_sync.get_user(3).is_err(), "Uncommitted data should not exist");
    assert!(user_sync.get_user(1).is_ok(), "Committed data should exist");
}

// ============================================================================
// Cross-Database Corruption Isolation Tests
// ============================================================================

#[test]
fn test_corruption_isolation_between_databases() {
    let dir = tempfile::tempdir().unwrap();
    let user_db_path = dir.path().join("users.db");
    let chat_db_path = dir.path().join("chats.db");

    // Create both databases
    {
        let user_db_conn = DbConnection::new(&user_db_path).unwrap();
        let chat_db_conn = DbConnection::new(&chat_db_path).unwrap();

        let user_db = UserDb::new(user_db_conn);
        user_db.init().unwrap();
        let mut user_sync = user_db.sync();
        user_sync.add_user(1, bytes::Bytes::from("user1"), None, None).unwrap();

        let chat_db = ChatDb::new(chat_db_conn);
        chat_db.init().unwrap();
        let mut chat_sync = chat_db.sync();
        chat_sync.add_chat(1, bytes::Bytes::from("chat1"), None, None).unwrap();
    }

    // Corrupt only user database
    create_corrupted_header(&user_db_path).unwrap();

    // User database should be inaccessible
    let user_db_conn = DbConnection::new(&user_db_path);
    assert!(user_db_conn.is_err(), "Corrupted user database should fail");

    // Chat database should still work
    let chat_db_conn = DbConnection::new(&chat_db_path);
    assert!(chat_db_conn.is_ok(), "Chat database should be accessible");

    let chat_db = ChatDb::new(chat_db_conn.unwrap());
    chat_db.init().unwrap();
    let mut chat_sync = chat_db.sync();
    assert!(chat_sync.get_chat(1).is_ok(), "Chat data should be intact");
}

// ============================================================================
// Empty Database Tests
// ============================================================================

#[test]
fn test_empty_database_file_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    // Create empty file
    fs::File::create(&db_path).unwrap();

    // Should be able to initialize database on empty file
    let db = DbConnection::new(&db_path);
    assert!(db.is_ok(), "Should be able to create database on empty file");

    let user_db = UserDb::new(db.unwrap());
    assert!(user_db.init().is_ok(), "Should be able to initialize schema");
}

#[test]
fn test_truncated_database_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    // Create valid database
    {
        let db = DbConnection::new(&db_path).unwrap();
        let user_db = UserDb::new(db);
        user_db.init().unwrap();
    }

    // Truncate the file
    let metadata = fs::metadata(&db_path).unwrap();
    let original_size = metadata.len();
    let truncated_size = original_size / 2;

    let file = fs::OpenOptions::new()
        .write(true)
        .open(&db_path)
        .unwrap();
    file.set_len(truncated_size).unwrap();

    // Should detect corruption
    let db = DbConnection::new(&db_path);
    // Behavior depends on where truncation occurred
    // May succeed with partial data or fail with corruption error
    match db {
        Ok(db_conn) => {
            // If it opened, verify it's still functional
            let user_db = UserDb::new(db_conn);
            let init_result = user_db.init();
            // Either init succeeds or reports corruption
            assert!(init_result.is_ok() || init_result.unwrap_err().to_string().contains("corrupted"));
        }
        Err(_) => {
            // Expected to fail with corruption error
        }
    }
}
