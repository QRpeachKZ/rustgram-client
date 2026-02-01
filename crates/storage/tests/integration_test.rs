//! Integration tests for storage layer.
//!
//! These tests verify the interaction patterns between managers and databases,
//! including batch operations, pagination, and cross-database scenarios.

use bytes::Bytes;
use rustgram_storage::{
    chat::ChatDb,
    connection::DbConnection,
    error::StorageResult,
    file::FileDb,
    message::{sync::AddMessageParams, MessageDb, MessageSearchFilter},
    user::UserDb,
};
use std::time::Instant;
use tempfile::TempDir;

/// Creates an initialized test database with all schemas.
/// Note: Creates separate database files for each database type to avoid migration conflicts.
fn setup_test_dbs() -> StorageResult<(TempDir, MessageDb, UserDb, ChatDb, FileDb)> {
    let dir = tempfile::tempdir()?;

    // Create separate database files for each type
    let msg_db_path = dir.path().join("messages.db");
    let user_db_path = dir.path().join("users.db");
    let chat_db_path = dir.path().join("chats.db");
    let file_db_path = dir.path().join("files.db");

    let msg_db_conn = DbConnection::new(&msg_db_path)?;
    let user_db_conn = DbConnection::new(&user_db_path)?;
    let chat_db_conn = DbConnection::new(&chat_db_path)?;
    let file_db_conn = DbConnection::new(&file_db_path)?;

    // Initialize all databases
    let message_db = MessageDb::new(msg_db_conn);
    message_db.init()?;

    let user_db = UserDb::new(user_db_conn);
    user_db.init()?;

    let chat_db = ChatDb::new(chat_db_conn);
    chat_db.init()?;

    let file_db = FileDb::new(file_db_conn);
    file_db.init()?;

    Ok((dir, message_db, user_db, chat_db, file_db))
}

// ============================================================================
// MessageDb Integration Tests
// ============================================================================

#[test]
fn test_message_pagination_large_dataset() {
    let (_dir, message_db, _user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut msg_db = message_db.sync();

    let dialog_id = 12345i64;
    let sender_id = 67890i64;

    // Add 100 messages
    for i in 1..=100 {
        msg_db
            .add_message(
                AddMessageParams::new(
                    dialog_id,
                    i,
                    sender_id,
                    1704067200 + i,
                    Bytes::from(format!("message content {}", i)),
                )
                .with_text(format!("text {}", i)),
            )
            .unwrap();
    }

    // Test pagination: get 10 messages at a time
    let mut all_message_ids = Vec::new();
    let mut from_id = 0i32;
    let page_size = 10;

    loop {
        let page = msg_db
            .get_dialog_messages(dialog_id, from_id, 0, page_size, None)
            .unwrap();

        if page.is_empty() {
            break;
        }

        for msg in &page {
            all_message_ids.push(msg.message_id);
        }

        from_id = page.last().unwrap().message_id;
    }

    // Should have retrieved all 100 messages
    assert_eq!(all_message_ids.len(), 100);
    assert_eq!(msg_db.get_message_count(dialog_id).unwrap(), 100);

    // Verify descending order
    for i in 0..all_message_ids.len() - 1 {
        assert!(all_message_ids[i] > all_message_ids[i + 1]);
    }
}

#[test]
fn test_message_batch_operations() {
    let (_dir, message_db, _user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut msg_db = message_db.sync();

    let dialog_id = 12345i64;
    let sender_id = 67890i64;

    // Batch insert 50 messages
    let start = Instant::now();
    for i in 1..=50 {
        msg_db
            .add_message(AddMessageParams::new(
                dialog_id,
                i,
                sender_id,
                1704067200 + i,
                Bytes::from(format!("batch message {}", i)),
            ))
            .unwrap();
    }
    let insert_duration = start.elapsed();

    // Verify all inserted
    let count = msg_db.get_message_count(dialog_id).unwrap();
    assert_eq!(count, 50);

    // Batch query all messages
    let start = Instant::now();
    let messages = msg_db
        .get_dialog_messages(dialog_id, 0, 0, 100, None)
        .unwrap();
    let query_duration = start.elapsed();

    assert_eq!(messages.len(), 50);

    // Performance assertions (adjust thresholds as needed)
    assert!(
        insert_duration.as_millis() < 200,
        "Batch insert took {}ms, expected < 200ms",
        insert_duration.as_millis()
    );
    assert!(
        query_duration.as_millis() < 100,
        "Batch query took {}ms, expected < 100ms",
        query_duration.as_millis()
    );
}

#[test]
fn test_message_complex_search_filter() {
    let (_dir, message_db, _user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut msg_db = message_db.sync();

    let dialog_id = 12345i64;
    let sender1 = 100i64;
    let sender2 = 200i64;

    // Add messages from different senders with different content
    for i in 1..=20 {
        let sender = if i % 2 == 0 { sender1 } else { sender2 };
        let text = if i % 3 == 0 {
            "urgent: important message".to_string()
        } else if i % 2 == 0 {
            "normal message".to_string()
        } else {
            "casual text".to_string()
        };

        msg_db
            .add_message(
                AddMessageParams::new(
                    dialog_id,
                    i,
                    sender,
                    1704067200 + i * 60, // 1 minute apart
                    Bytes::from(format!("content {}", i)),
                )
                .with_text(text),
            )
            .unwrap();
    }

    // Search by sender1 and text containing "urgent"
    let filter = MessageSearchFilter {
        sender_id: Some(sender1),
        text_query: Some("urgent".to_string()),
        ..Default::default()
    };

    let results = msg_db
        .get_dialog_messages(dialog_id, 0, 0, 100, Some(filter.clone()))
        .unwrap();

    // Should find messages from sender1 containing "urgent"
    assert!(!results.is_empty());
    for msg in &results {
        assert_eq!(msg.sender_id, sender1);
        assert!(msg.text.as_ref().unwrap().contains("urgent"));
    }

    // Search by date range
    let filter = MessageSearchFilter {
        min_date: Some(1704067200 + 5 * 60),
        max_date: Some(1704067200 + 15 * 60),
        ..Default::default()
    };

    let results = msg_db
        .get_dialog_messages(dialog_id, 0, 0, 100, Some(filter))
        .unwrap();

    assert!(results.len() > 0);
}

// ============================================================================
// UserDb Integration Tests
// ============================================================================

#[test]
fn test_user_batch_queries() {
    let (_dir, _message_db, user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut user_db = user_db.sync();

    // Add 50 users
    let user_ids: Vec<i32> = (1..=50).collect();
    for &user_id in &user_ids {
        user_db
            .add_user(
                user_id,
                Bytes::from(format!("user_data_{}", user_id)),
                Some(Bytes::from(format!("photo_{}", user_id))),
                Some(format!("Bio for user {}", user_id)),
            )
            .unwrap();
    }

    // Batch query all users
    let start = Instant::now();
    let users = user_db.get_users(user_ids.clone()).unwrap();
    let duration = start.elapsed();

    assert_eq!(users.len(), 50);
    assert!(
        duration.as_millis() < 50,
        "Batch query took {}ms, expected < 50ms",
        duration.as_millis()
    );

    // Batch query subset
    let subset = vec![5, 10, 15, 20, 25];
    let users = user_db.get_users(subset).unwrap();
    assert_eq!(users.len(), 5);

    // Verify count
    assert_eq!(user_db.get_user_count().unwrap(), 50);
}

#[test]
fn test_user_full_info_integration() {
    let (_dir, _message_db, user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut user_db = user_db.sync();

    let user_id = 12345i32;

    // Add basic user info
    user_db
        .add_user(
            user_id,
            Bytes::from("basic user data"),
            Some(Bytes::from("profile photo")),
            Some("user bio".to_string()),
        )
        .unwrap();

    // Add full info
    user_db
        .add_user_full(user_id, Bytes::from("full user info"))
        .unwrap();

    // Retrieve both
    let basic = user_db.get_user(user_id).unwrap();
    let full = user_db.get_user_full(user_id).unwrap();

    assert_eq!(basic, Bytes::from("basic user data"));
    assert_eq!(full, Bytes::from("full user info"));

    // Verify independence: deleting basic shouldn't affect full
    user_db.delete_user(user_id).unwrap();
    assert!(user_db.get_user(user_id).is_err());
    assert!(user_db.get_user_full(user_id).is_ok());
}

// ============================================================================
// ChatDb Integration Tests
// ============================================================================

#[test]
fn test_chat_batch_operations() {
    let (_dir, _message_db, _user_db, chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut chat_db = chat_db.sync();

    // Add 30 chats with various data
    let chat_ids: Vec<i64> = (1..=30).map(|i| i as i64).collect();
    for &chat_id in &chat_ids {
        chat_db
            .add_chat(
                chat_id,
                Bytes::from(format!("chat_data_{}", chat_id)),
                Some(Bytes::from(format!("photo_{}", chat_id))),
                Some(Bytes::from(format!("permissions_{}", chat_id))),
            )
            .unwrap();
    }

    // Batch query
    let start = Instant::now();
    let chats = chat_db.get_chats(chat_ids.clone()).unwrap();
    let duration = start.elapsed();

    assert_eq!(chats.len(), 30);
    assert!(
        duration.as_millis() < 50,
        "Batch query took {}ms, expected < 50ms",
        duration.as_millis()
    );

    // Query subset
    let subset = vec![10i64, 20, 30];
    let chats = chat_db.get_chats(subset).unwrap();
    assert_eq!(chats.len(), 3);

    // Verify count
    assert_eq!(chat_db.get_chat_count().unwrap(), 30);
}

#[test]
fn test_chat_negative_ids() {
    let (_dir, _message_db, _user_db, chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut chat_db = chat_db.sync();

    // Test supergroup/channel IDs (negative)
    let negative_ids = vec![-100001, -100002, -100003];

    for &chat_id in &negative_ids {
        chat_db
            .add_chat(
                chat_id,
                Bytes::from(format!("supergroup_{}", chat_id)),
                None,
                None,
            )
            .unwrap();
    }

    // Query should work with negative IDs
    let chats = chat_db.get_chats(negative_ids.clone()).unwrap();
    assert_eq!(chats.len(), 3);
}

// ============================================================================
// FileDb Integration Tests
// ============================================================================

#[test]
fn test_file_sequence_generation() {
    let (_dir, _message_db, _user_db, _chat_db, file_db) = setup_test_dbs().unwrap();
    let mut file_db = file_db.sync();

    // Generate multiple IDs sequentially
    let ids: Vec<i32> = (0..10)
        .map(|_| file_db.get_next_file_db_id().unwrap())
        .collect();

    // Should be sequential starting from 1
    assert_eq!(ids, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_file_large_data() {
    let (_dir, _message_db, _user_db, _chat_db, file_db) = setup_test_dbs().unwrap();
    let mut file_db = file_db.sync();

    // Test with various data sizes
    let sizes = vec![
        (1, "1KB"),
        (10, "10KB"),
        (100, "100KB"),
        (500, "500KB"),
        (1024, "1MB"),
    ];

    for (size_kb, label) in sizes {
        let data = vec![0xABu8; size_kb * 1024];
        let bytes = Bytes::from(data);

        let file_id = file_db.get_next_file_db_id().unwrap();
        let key = format!("test_file_{}", label);

        let start = Instant::now();
        file_db.set_file_data(file_id, &key, bytes.clone()).unwrap();
        let write_duration = start.elapsed();

        let start = Instant::now();
        let retrieved = file_db.get_file_data(&key).unwrap();
        let read_duration = start.elapsed();

        assert_eq!(retrieved.len(), size_kb * 1024);

        // Performance assertions for large files
        assert!(
            write_duration.as_millis() < 500,
            "Write {} took {}ms, expected < 500ms",
            label,
            write_duration.as_millis()
        );
        assert!(
            read_duration.as_millis() < 100,
            "Read {} took {}ms, expected < 100ms",
            label,
            read_duration.as_millis()
        );
    }
}

// ============================================================================
// Cross-Database Integration Tests
// ============================================================================

#[test]
fn test_cross_database_consistency() {
    let (_dir, message_db, user_db, chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut msg_db = message_db.sync();
    let mut user_db = user_db.sync();
    let mut chat_db = chat_db.sync();

    // Scenario: Add users and chats, then add messages referencing them
    let user_id = 12345i32;
    let chat_id = 67890i64;
    let dialog_id = chat_id; // In real scenario, encode properly

    // Add user
    user_db
        .add_user(user_id, Bytes::from("user data"), None, None)
        .unwrap();

    // Add chat
    chat_db
        .add_chat(chat_id, Bytes::from("chat data"), None, None)
        .unwrap();

    // Add messages in the dialog
    for i in 1..=5 {
        msg_db
            .add_message(AddMessageParams::new(
                dialog_id,
                i,
                user_id as i64,
                1704067200 + i,
                Bytes::from(format!("message {}", i)),
            ))
            .unwrap();
    }

    // Verify all databases have consistent data
    assert!(user_db.get_user(user_id).is_ok());
    assert!(chat_db.get_chat(chat_id).is_ok());
    assert_eq!(msg_db.get_message_count(dialog_id).unwrap(), 5);
}

#[test]
fn test_performance_single_entity_query() {
    let (_dir, _message_db, user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut user_db = user_db.sync();

    let user_id = 12345i32;
    user_db
        .add_user(user_id, Bytes::from("test data"), None, None)
        .unwrap();

    // Warm up
    for _ in 0..10 {
        let _ = user_db.get_user(user_id).unwrap();
    }

    // Measure 100 queries
    let start = Instant::now();
    for _ in 0..100 {
        let _ = user_db.get_user(user_id).unwrap();
    }
    let duration = start.elapsed();

    let avg_per_query = duration.as_micros() as f64 / 100.0;

    // Assert average query time < 10ms
    assert!(
        avg_per_query < 10000.0,
        "Average query time: {}us, expected < 10ms",
        avg_per_query
    );
}

#[test]
fn test_performance_batch_query() {
    let (_dir, _message_db, user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut user_db = user_db.sync();

    // Add 100 users
    for i in 1..=100 {
        user_db
            .add_user(i, Bytes::from(format!("user {}", i)), None, None)
            .unwrap();
    }

    // Measure batch query performance
    let start = Instant::now();
    let users = user_db.get_users((1..=100).collect()).unwrap();
    let duration = start.elapsed();

    assert_eq!(users.len(), 100);

    // Assert batch query < 100ms
    assert!(
        duration.as_millis() < 100,
        "Batch query took {}ms, expected < 100ms",
        duration.as_millis()
    );
}

#[test]
fn test_performance_migration_time() {
    let start = Instant::now();

    let _dbs = setup_test_dbs().unwrap();

    let duration = start.elapsed();

    // Assert initialization + migrations < 1s
    assert!(
        duration.as_secs() < 1,
        "Initialization took {}ms, expected < 1s",
        duration.as_millis()
    );
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_error_not_found_handling() {
    let (_dir, _message_db, user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut user_db = user_db.sync();

    let result = user_db.get_user(99999);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.is_not_found());
    assert!(!err.is_transient());
}

#[test]
fn test_error_batch_with_missing_ids() {
    let (_dir, _message_db, user_db, _chat_db, _file_db) = setup_test_dbs().unwrap();
    let mut user_db = user_db.sync();

    // Add only some users
    user_db
        .add_user(1, Bytes::from("user1"), None, None)
        .unwrap();
    user_db
        .add_user(3, Bytes::from("user3"), None, None)
        .unwrap();

    // Query including non-existent IDs
    let users = user_db.get_users(vec![1, 2, 3, 4, 5]).unwrap();

    // Should only return existing users
    assert_eq!(users.len(), 2);
}
