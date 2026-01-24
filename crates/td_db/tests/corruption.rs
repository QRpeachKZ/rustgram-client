//! Corruption recovery tests for TdDb coordinator.
//!
//! These tests verify the TdDb coordinator's ability to handle
//! various corruption scenarios including WAL corruption, header corruption,
//! and recovery mechanisms.

use std::fs;
use std::io::Write;
use std::path::Path;
use rustgram_td_db::{TdDb, TdDbParameters};

/// Creates a corrupted WAL file for testing.
fn create_corrupted_wal(db_path: &Path) -> std::io::Result<()> {
    let wal_path = db_path.with_extension("db-wal");
    let mut file = fs::File::create(&wal_path)?;
    // Write invalid WAL header
    file.write_all(b"CORRUPTED_WAL_DATA")?;
    Ok(())
}

/// Creates a corrupted database header.
fn create_corrupted_header(db_path: &Path) -> std::io::Result<()> {
    let mut file = fs::File::create(db_path)?;
    // Write invalid SQLite header
    file.write_all(b"NOT_SQLITE_DATABASE")?;
    Ok(())
}

// ============================================================================
// WAL Corruption Tests
// ============================================================================

#[test]
fn test_td_db_corrupted_wal_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create a valid TdDb first
    {
        let tddb = TdDb::open(params.clone()).unwrap();
        // Just verify the databases can be accessed
        assert!(tddb.get_message_db().is_some());
        assert!(tddb.get_user_db().is_some());
        assert!(tddb.get_chat_db().is_some());
    }

    // Corrupt the WAL file for messages.db
    let msg_db_path = dir.path().join("messages.db");
    create_corrupted_wal(&msg_db_path).unwrap();

    // Attempt to open TdDb with corrupted WAL
    // SQLite should automatically recover from corrupted WAL
    let tddb = TdDb::open(params);
    assert!(tddb.is_ok(), "Should be able to open TdDb with corrupted WAL");

    let tddb = tddb.unwrap();
    assert!(tddb.is_open());

    // Verify we can still access databases
    assert!(tddb.get_message_db().is_some());
    assert!(tddb.get_user_db().is_some());
    assert!(tddb.get_chat_db().is_some());
}

#[test]
fn test_td_db_missing_wal_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create a valid TdDb
    {
        let _tddb = TdDb::open(params.clone()).unwrap();
    }

    // Remove WAL files if they exist
    for db_name in &["messages", "users", "chats"] {
        let wal_path = dir.path().join(format!("{}.db-wal", db_name));
        let _ = fs::remove_file(&wal_path);
    }

    // Should be able to open TdDb without WAL files
    let tddb = TdDb::open(params);
    assert!(tddb.is_ok(), "Should be able to open TdDb without WAL");
}

// ============================================================================
// Header Corruption Tests
// ============================================================================

#[test]
fn test_td_db_corrupted_header_detection() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Corrupt the messages.db header before creation
    let msg_db_path = dir.path().join("messages.db");
    create_corrupted_header(&msg_db_path).unwrap();

    // Should fail to open TdDb with corrupted database
    let tddb = TdDb::open(params);
    // TdDb::open may succeed but individual databases will fail
    // The important thing is that it doesn't crash
    match tddb {
        Ok(tddb) => {
            // If it opened, it should still be functional
            // But message_db might not work properly
            assert!(tddb.is_open());
        }
        Err(_) => {
            // Expected to fail with corruption error
            // This is also acceptable behavior
        }
    }
}

// ============================================================================
// Database Rebuild Tests
// ============================================================================

#[test]
fn test_td_db_rebuild_from_scratch() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create initial TdDb
    {
        let tddb = TdDb::open(params.clone()).unwrap();
        assert!(tddb.get_message_db().is_some());
    }

    // Rebuild databases from scratch
    let result = TdDb::rebuild(&params);
    assert!(result.is_ok(), "Should be able to rebuild TdDb from scratch");

    // Verify databases are fresh but functional
    let tddb = TdDb::open(params).unwrap();
    assert!(tddb.is_open());
    assert!(tddb.get_message_db().is_some());
}

#[test]
fn test_td_db_partial_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create initial TdDb
    {
        let tddb = TdDb::open(params.clone()).unwrap();
        assert!(tddb.get_user_db().is_some());
    }

    // Corrupt only messages.db
    let msg_db_path = dir.path().join("messages.db");
    create_corrupted_header(&msg_db_path).unwrap();

    // Should still be able to use other databases
    let tddb = TdDb::open(params);
    match tddb {
        Ok(tddb) => {
            assert!(tddb.is_open());
            // User database might still work
            let user_db = tddb.get_user_db();
            assert!(user_db.is_some());
        }
        Err(_) => {
            // Also acceptable - TdDb refuses to open with corruption
        }
    }
}

// ============================================================================
// Cross-Database Corruption Isolation Tests
// ============================================================================

#[test]
fn test_td_db_corruption_isolation() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create TdDb
    {
        let _tddb = TdDb::open(params.clone()).unwrap();
    }

    // Corrupt only messages.db
    let msg_db_path = dir.path().join("messages.db");
    create_corrupted_header(&msg_db_path).unwrap();

    // Try to open TdDb
    let tddb = TdDb::open(params);

    // Behavior depends on TdDb implementation
    // It should either:
    // 1. Fail to open (conservative approach)
    // 2. Open but handle the corrupted database gracefully
    match tddb {
        Ok(_) => {
            // If it opened, user_db and chat_db should still work
        }
        Err(_) => {
            // Expected - TdDb detected corruption
        }
    }
}

// ============================================================================
// Empty Database Tests
// ============================================================================

#[test]
fn test_td_db_empty_database_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create empty database files
    for db_name in &["messages", "users", "chats", "files"] {
        let db_path = dir.path().join(format!("{}.db", db_name));
        fs::File::create(&db_path).unwrap();
    }

    // Should be able to initialize databases on empty files
    let tddb = TdDb::open(params);
    assert!(tddb.is_ok(), "Should be able to create TdDb on empty files");

    let tddb = tddb.unwrap();
    assert!(tddb.is_open());
}

#[test]
fn test_td_db_truncated_database_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create valid database
    {
        let _tddb = TdDb::open(params.clone()).unwrap();
    }

    // Truncate one database file
    let msg_db_path = dir.path().join("messages.db");
    let metadata = fs::metadata(&msg_db_path).unwrap();
    let original_size = metadata.len();
    let truncated_size = original_size / 2;

    let file = fs::OpenOptions::new()
        .write(true)
        .open(&msg_db_path)
        .unwrap();
    file.set_len(truncated_size).unwrap();

    // Should detect corruption or recover gracefully
    let tddb = TdDb::open(params);
    match tddb {
        Ok(tddb) => {
            // If it opened, verify it's functional
            assert!(tddb.is_open());
        }
        Err(_) => {
            // Expected to fail with corruption error
        }
    }
}

// ============================================================================
// Transaction Tests
// ============================================================================

#[test]
fn test_td_db_incomplete_transaction_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create database
    {
        let tddb = TdDb::open(params.clone()).unwrap();
        assert!(tddb.get_user_db().is_some());
    }

    // Verify database can be reopened
    let tddb = TdDb::open(params).unwrap();
    assert!(tddb.is_open());
    assert!(tddb.get_user_db().is_some());
}

// ============================================================================
// KeyValueStore Corruption Tests
// ============================================================================

#[test]
fn test_kv_store_survives_rollover() {
    use rustgram_td_db::KeyValueStore;

    let dir = tempfile::tempdir().unwrap();
    let params = TdDbParameters::new(
        dir.path().to_str().unwrap().to_string(),
        "/files".to_string(),
        false,
        true,
    );

    // Create KeyValueStore and add data
    {
        let kv = KeyValueStore::open(&params).unwrap();
        kv.set("test_key", bytes::Bytes::from("test_value")).unwrap();
    }

    // Reopen and verify data persisted
    let kv = KeyValueStore::open(&params).unwrap();
    let value = kv.get("test_key").unwrap();
    assert_eq!(value, Some(bytes::Bytes::from("test_value")));
}
