//! Performance benchmarks for TdDb database operations.
//!
//! These benchmarks measure the performance of common database operations
//! to ensure they meet the specified targets:
//! - Single entity query: < 10ms target
//! - Batch query (100 messages): < 100ms target
//! - Migration time: < 1s target

use bytes::Bytes;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustgram_storage::{
    message::{sync::AddMessageParams, MessageDb},
    user::UserDb,
    chat::ChatDb,
    file::FileDb,
    connection::DbConnection,
};
use tempfile::TempDir;

fn setup_benchmark_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

fn benchmark_single_entity_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_entity_query");

    group.bench_function("user_db_single_get", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("users.db");
            let db = DbConnection::new(&db_path).unwrap();
            let user_db = UserDb::new(db);
            user_db.init().unwrap();
            let mut user_db = user_db.sync();

            user_db.add_user(
                12345,
                Bytes::from("test user data with some content"),
                None,
                None,
            ).ok();

            let result = user_db.get_user(12345);
            drop(dir);
            black_box(result)
        });
    });

    group.bench_function("chat_db_single_get", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("chats.db");
            let db = DbConnection::new(&db_path).unwrap();
            let chat_db = ChatDb::new(db);
            chat_db.init().unwrap();
            let mut chat_db = chat_db.sync();

            chat_db.add_chat(
                12345,
                Bytes::from("test chat data"),
                None,
                None,
            ).ok();

            let result = chat_db.get_chat(12345);
            drop(dir);
            black_box(result)
        });
    });

    group.finish();
}

fn benchmark_batch_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_query");

    group.bench_function("user_db_batch_100", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("users.db");
            let db = DbConnection::new(&db_path).unwrap();
            let user_db = UserDb::new(db);
            user_db.init().unwrap();
            let mut user_db = user_db.sync();

            for i in 1..=100 {
                user_db.add_user(
                    i,
                    Bytes::from(format!("user data {}", i)),
                    None,
                    None,
                ).ok();
            }

            let ids: Vec<i32> = (1..=100).collect();
            let result = user_db.get_users(ids);
            drop(dir);
            black_box(result)
        });
    });

    group.bench_function("message_db_batch_100", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("messages.db");
            let db = DbConnection::new(&db_path).unwrap();
            let message_db = MessageDb::new(db);
            message_db.init().unwrap();
            let mut message_db = message_db.sync();

            for i in 1..=100 {
                message_db.add_message(
                    AddMessageParams::new(
                        12345,
                        i,
                        67890,
                        1704067200 + i,
                        Bytes::from(format!("message content {}", i)),
                    )
                ).ok();
            }

            let result = message_db.get_dialog_messages(12345, 0, 0, 100, None);
            drop(dir);
            black_box(result)
        });
    });

    group.finish();
}

fn benchmark_migration_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("migration");

    group.bench_function("message_db_full_migration", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("messages.db");
            let start = std::time::Instant::now();

            let db = DbConnection::new(&db_path).unwrap();
            let message_db = MessageDb::new(db);
            message_db.init().unwrap();

            let duration = start.elapsed();
            drop(dir);
            black_box(duration)
        });
    });

    group.bench_function("user_db_full_migration", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("users.db");
            let start = std::time::Instant::now();

            let db = DbConnection::new(&db_path).unwrap();
            let user_db = UserDb::new(db);
            user_db.init().unwrap();

            let duration = start.elapsed();
            drop(dir);
            black_box(duration)
        });
    });

    group.bench_function("chat_db_full_migration", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("chats.db");
            let start = std::time::Instant::now();

            let db = DbConnection::new(&db_path).unwrap();
            let chat_db = ChatDb::new(db);
            chat_db.init().unwrap();

            let duration = start.elapsed();
            drop(dir);
            black_box(duration)
        });
    });

    group.bench_function("file_db_full_migration", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("files.db");
            let start = std::time::Instant::now();

            let db = DbConnection::new(&db_path).unwrap();
            let file_db = FileDb::new(db);
            file_db.init().unwrap();

            let duration = start.elapsed();
            drop(dir);
            black_box(duration)
        });
    });

    group.finish();
}

fn benchmark_write_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_operations");

    group.bench_function("message_db_insert", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("messages.db");
            let db = DbConnection::new(&db_path).unwrap();
            let message_db = MessageDb::new(db);
            message_db.init().unwrap();
            let mut message_db = message_db.sync();

            let result = message_db.add_message(
                AddMessageParams::new(
                    12345,
                    1,
                    67890,
                    1704067200,
                    Bytes::from("test message content for benchmarking"),
                )
            );
            drop(dir);
            black_box(result)
        });
    });

    group.bench_function("user_db_insert", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("users.db");
            let db = DbConnection::new(&db_path).unwrap();
            let user_db = UserDb::new(db);
            user_db.init().unwrap();
            let mut user_db = user_db.sync();

            let result = user_db.add_user(
                12345,
                Bytes::from("test user data"),
                None,
                None,
            );
            drop(dir);
            black_box(result)
        });
    });

    group.finish();
}

fn benchmark_large_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_data");

    group.bench_function("message_db_large_content", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("messages.db");
            let db = DbConnection::new(&db_path).unwrap();
            let message_db = MessageDb::new(db);
            message_db.init().unwrap();
            let mut message_db = message_db.sync();

            // 10KB message content
            let large_content = Bytes::from(vec![0xABu8; 10240]);
            let result = message_db.add_message(
                AddMessageParams::new(
                    12345,
                    1,
                    67890,
                    1704067200,
                    large_content,
                )
            );
            drop(dir);
            black_box(result)
        });
    });

    group.bench_function("file_db_large_data", |b| {
        b.iter(|| {
            let dir = setup_benchmark_dir();
            let db_path = dir.path().join("files.db");
            let db = DbConnection::new(&db_path).unwrap();
            let file_db = FileDb::new(db);
            file_db.init().unwrap();
            let mut file_db = file_db.sync();

            let id = file_db.get_next_file_db_id().ok();
            let large_data = Bytes::from(vec![0xCDu8; 102400]); // 100KB
            let result = file_db.set_file_data(id.unwrap(), "test_key", large_data);
            drop(dir);
            black_box(result)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_single_entity_query,
    benchmark_batch_query,
    benchmark_migration_time,
    benchmark_write_operations,
    benchmark_large_data
);

criterion_main!(benches);
