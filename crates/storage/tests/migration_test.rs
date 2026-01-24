//! Migration tests for storage layer.
//!
//! These tests verify that data survives migrations and that migrations
//! can be applied correctly across all database types.
//!
//! Run with: `cargo test -p rustgram-storage --test migration_test --features "message,user,chat,file"`

use bytes::Bytes;
use rusqlite::params;
use rustgram_storage::connection::DbConnection;

// ============================================================================
// MessageDb Migration Tests
// ============================================================================

#[cfg(feature = "message")]
#[test]
fn test_message_migration_data_survival() {
    use rustgram_storage::message::MessageDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("messages.db");

    // Create database with initial schema (v1)
    let db = DbConnection::new(&db_path).unwrap();
    let mut conn = db.connect().unwrap();

    // Apply only v1 migration
    #[cfg(feature = "message")]
    {
        use rustgram_storage::message::schema::MessageMigrationV1;
        use rustgram_storage::migrations::Migration;
        MessageMigrationV1.apply(&mut conn).unwrap();
    }

    // Insert data using v1 schema
    conn.execute(
        "INSERT INTO messages (dialog_id, message_id, sender_id, date, content) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![12345i64, 1i64, 67890i64, 1704067200i32, b"v1 data".as_slice()],
    ).unwrap();

    // Now apply remaining migrations through MessageDb::init
    let message_db = MessageDb::new(db);
    message_db.init().unwrap();

    // Verify data still exists after all migrations
    let mut msg_db = message_db.sync();
    let msg = msg_db.get_message(12345, 1).unwrap();
    assert_eq!(msg.content, Bytes::from("v1 data"));
}

#[cfg(feature = "message")]
#[test]
fn test_message_forward_migrations() {
    use rustgram_storage::message::MessageDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("messages.db");

    // Create database and run all migrations
    let db = DbConnection::new(&db_path).unwrap();
    let message_db = MessageDb::new(db);
    message_db.init().unwrap();

    // Verify all migrations applied
    let conn = message_db.sync().db().connect().unwrap();

    // Check tables exist
    let tables: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('messages', 'scheduled_messages')",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(tables, 2);

    // Check columns exist (from v5)
    let columns: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('messages') WHERE name IN ('ttl_expires_at', 'text', 'random_id', 'unique_message_id', 'search_id', 'top_thread_message_id')",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(columns, 6);
}

#[cfg(feature = "message")]
#[test]
fn test_message_migration_v2_data_preservation() {
    use rustgram_storage::message::schema::{MessageMigrationV1, MessageMigrationV2};
    use rustgram_storage::migrations::Migration;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("messages.db");

    // Create database with v1 schema
    let db = DbConnection::new(&db_path).unwrap();
    let mut conn = db.connect().unwrap();

    MessageMigrationV1.apply(&mut conn).unwrap();

    // Add message without TTL
    conn.execute(
        "INSERT INTO messages (dialog_id, message_id, sender_id, date, content) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![12345i64, 1i64, 67890i64, 1704067200i32, b"test".as_slice()],
    ).unwrap();

    // Apply v2 (adds TTL column)
    MessageMigrationV2.apply(&mut conn).unwrap();

    // Verify old data still accessible
    let msg: Vec<u8> = conn
        .query_row(
            "SELECT content FROM messages WHERE dialog_id = ?1",
            [12345i64],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(msg, b"test".as_slice());
}

#[cfg(feature = "message")]
#[test]
fn test_message_migration_v4_scheduled_messages() {
    use rustgram_storage::message::MessageDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("messages.db");

    // Create database and run all migrations
    let db = DbConnection::new(&db_path).unwrap();
    let message_db = MessageDb::new(db);
    message_db.init().unwrap();

    // Verify scheduled_messages table exists
    let conn = message_db.sync().db().connect().unwrap();
    let table_exists: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='scheduled_messages'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(table_exists, 1);

    // Add scheduled message
    let mut msg_db = message_db.sync();
    msg_db
        .add_scheduled_message(12345, 100, 1704067200, Bytes::from("scheduled"))
        .unwrap();

    // Verify can retrieve
    let messages = msg_db.get_scheduled_messages(12345, 10).unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, Bytes::from("scheduled"));
}

// ============================================================================
// UserDb Migration Tests
// ============================================================================

#[cfg(feature = "user")]
#[test]
fn test_user_migration_data_survival() {
    use rustgram_storage::user::UserDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("users.db");

    // Create database with v1 schema only
    let db = DbConnection::new(&db_path).unwrap();
    let mut conn = db.connect().unwrap();

    #[cfg(feature = "user")]
    {
        use rustgram_storage::migrations::Migration;
        use rustgram_storage::user::schema::UserMigrationV1;
        UserMigrationV1.apply(&mut conn).unwrap();
    }

    // Insert user data
    conn.execute(
        "INSERT INTO users (user_id, data, profile_photo, bio) VALUES (?1, ?2, ?3, ?4)",
        params![12345i32, b"user data".as_slice(), b"photo".as_slice(), "bio text"],
    ).unwrap();

    // Apply v2 migration
    let user_db = UserDb::new(db);
    user_db.init().unwrap();

    // Verify data survived
    let mut user_db = user_db.sync();
    let data = user_db.get_user(12345).unwrap();
    assert_eq!(data, Bytes::from("user data"));

    let photo = user_db.get_user_profile_photo(12345).unwrap();
    assert_eq!(photo, Some(Bytes::from("photo")));

    let bio = user_db.get_user_bio(12345).unwrap();
    assert_eq!(bio, Some("bio text".to_string()));
}

#[cfg(feature = "user")]
#[test]
fn test_user_migration_v2_full_info() {
    use rustgram_storage::user::UserDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("users.db");

    // Create database and run all migrations
    let db = DbConnection::new(&db_path).unwrap();
    let user_db = UserDb::new(db);
    user_db.init().unwrap();

    // Verify users_full table exists
    let conn = user_db.sync().db().connect().unwrap();
    let table_exists: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users_full'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(table_exists, 1);

    // Add full info
    let mut user_db = user_db.sync();
    user_db
        .add_user_full(12345, Bytes::from("full info"))
        .unwrap();

    // Verify can retrieve
    let full = user_db.get_user_full(12345).unwrap();
    assert_eq!(full, Bytes::from("full info"));
}

// ============================================================================
// ChatDb Migration Tests
// ============================================================================

#[cfg(feature = "chat")]
#[test]
fn test_chat_migration_data_survival() {
    use rustgram_storage::chat::ChatDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("chats.db");

    // Create database with v1 schema only
    let db = DbConnection::new(&db_path).unwrap();
    let mut conn = db.connect().unwrap();

    #[cfg(feature = "chat")]
    {
        use rustgram_storage::chat::schema::ChatMigrationV1;
        use rustgram_storage::migrations::Migration;
        ChatMigrationV1.apply(&mut conn).unwrap();
    }

    // Insert chat data
    conn.execute(
        "INSERT INTO chats (chat_id, data, photo, permissions) VALUES (?1, ?2, ?3, ?4)",
        params![12345i64, b"chat data".as_slice(), b"chat photo".as_slice(), b"perms".as_slice()],
    ).unwrap();

    // Apply v2 migration
    let chat_db = ChatDb::new(db);
    chat_db.init().unwrap();

    // Verify data survived
    let mut chat_db = chat_db.sync();
    let data = chat_db.get_chat(12345).unwrap();
    assert_eq!(data, Bytes::from("chat data"));

    let photo = chat_db.get_chat_photo(12345).unwrap();
    assert_eq!(photo, Some(Bytes::from("chat photo")));

    let perms = chat_db.get_chat_permissions(12345).unwrap();
    assert_eq!(perms, Some(Bytes::from("perms")));
}

#[cfg(feature = "chat")]
#[test]
fn test_chat_migration_v2_full_info() {
    use rustgram_storage::chat::ChatDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("chats.db");

    // Create database and run all migrations
    let db = DbConnection::new(&db_path).unwrap();
    let chat_db = ChatDb::new(db);
    chat_db.init().unwrap();

    // Verify chats_full table exists
    let conn = chat_db.sync().db().connect().unwrap();
    let table_exists: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='chats_full'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(table_exists, 1);

    // Add full info
    let mut chat_db = chat_db.sync();
    chat_db
        .add_chat_full(12345, Bytes::from("full chat info"))
        .unwrap();

    // Verify can retrieve
    let full = chat_db.get_chat_full(12345).unwrap();
    assert_eq!(full, Bytes::from("full chat info"));
}

// ============================================================================
// FileDb Migration Tests
// ============================================================================

#[cfg(feature = "file")]
#[test]
fn test_file_migration_sequence_persistence() {
    use rustgram_storage::file::FileDb;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("files.db");

    // Create database
    let db = DbConnection::new(&db_path).unwrap();
    let file_db = FileDb::new(db);
    file_db.init().unwrap();

    // Generate some IDs
    let mut file_db = file_db.sync();
    let id1 = file_db.get_next_file_db_id().unwrap();
    let id2 = file_db.get_next_file_db_id().unwrap();
    let id3 = file_db.get_next_file_db_id().unwrap();

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);

    // Add some data
    file_db.set_file_data(1, "key1", Bytes::from("data1")).unwrap();
    file_db.set_file_data(2, "key2", Bytes::from("data2")).unwrap();

    // Verify sequence persisted
    let id4 = file_db.get_next_file_db_id().unwrap();
    assert_eq!(id4, 4);

    // Verify data accessible
    let data1 = file_db.get_file_data("key1").unwrap();
    assert_eq!(data1, Bytes::from("data1"));
}

// ============================================================================
// Cross-Database Migration Tests
// ============================================================================

#[cfg(all(feature = "message", feature = "user", feature = "chat", feature = "file"))]
#[test]
fn test_all_databases_migration_sequence() {
    use rustgram_storage::chat::ChatDb;
    use rustgram_storage::file::FileDb;
    use rustgram_storage::message::{sync::AddMessageParams, MessageDb};
    use rustgram_storage::user::UserDb;

    let dir = tempfile::tempdir().unwrap();

    let msg_db_path = dir.path().join("messages.db");
    let user_db_path = dir.path().join("users.db");
    let chat_db_path = dir.path().join("chats.db");
    let file_db_path = dir.path().join("files.db");

    // Initialize all databases
    let msg_db = MessageDb::new(DbConnection::new(&msg_db_path).unwrap());
    let user_db = UserDb::new(DbConnection::new(&user_db_path).unwrap());
    let chat_db = ChatDb::new(DbConnection::new(&chat_db_path).unwrap());
    let file_db = FileDb::new(DbConnection::new(&file_db_path).unwrap());

    // Run all migrations
    msg_db.init().unwrap();
    user_db.init().unwrap();
    chat_db.init().unwrap();
    file_db.init().unwrap();

    // Verify all databases are functional
    let mut msg_db = msg_db.sync();
    let mut user_db = user_db.sync();
    let mut chat_db = chat_db.sync();
    let mut file_db = file_db.sync();

    // Add test data
    msg_db
        .add_message(AddMessageParams::new(
            1,
            1,
            1,
            1704067200,
            Bytes::from("test"),
        ))
        .unwrap();
    user_db
        .add_user(1, Bytes::from("user"), None, None)
        .unwrap();
    chat_db.add_chat(1, Bytes::from("chat"), None, None).unwrap();
    let file_id = file_db.get_next_file_db_id().unwrap();

    // Verify data persisted
    assert!(msg_db.get_message(1, 1).is_ok());
    assert!(user_db.get_user(1).is_ok());
    assert!(chat_db.get_chat(1).is_ok());
    assert_eq!(file_id, 1);
}

// ============================================================================
// Migration Error Handling Tests
// ============================================================================

#[cfg(feature = "message")]
#[test]
fn test_migration_idempotency() {
    use rustgram_storage::message::{sync::AddMessageParams, MessageDb};

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("messages.db");

    // Create database and initialize
    let db = DbConnection::new(&db_path).unwrap();
    let message_db = MessageDb::new(db.clone());
    message_db.init().unwrap();

    // Add some data
    let mut msg_db = message_db.sync();
    msg_db
        .add_message(AddMessageParams::new(
            1,
            1,
            1,
            1704067200,
            Bytes::from("test"),
        ))
        .unwrap();

    // Re-initialize (should handle existing migrations)
    drop(msg_db);
    let message_db2 = MessageDb::new(db);
    message_db2.init().unwrap();

    // Verify data still exists
    let mut msg_db = message_db2.sync();
    assert!(msg_db.get_message(1, 1).is_ok());
}
