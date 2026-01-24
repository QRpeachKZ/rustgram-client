// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Benchmarks for file_db operations.

use bytes::Bytes;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use file_db::{FileDb, InMemoryFileDb};
use rustgram_file_db_id::FileDbId;
use std::hint::black_box as std_black_box;

fn bench_get_next_file_db_id(c: &mut Criterion) {
    let db = InMemoryFileDb::new();

    c.bench_function("get_next_file_db_id", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let _ = black_box(db.get_next_file_db_id().await);
            });
    });
}

fn bench_set_and_get_file_data(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("set_and_get_file_data", |b| {
        b.to_async(&rt).iter(|| async {
            let db = InMemoryFileDb::new();
            let id = FileDbId::new(1);
            let key = "test_key";
            let data = Bytes::from(vec![0u8; 1024]);

            db.set_file_data(id, black_box(key), black_box(data.clone()))
                .await
                .unwrap();
            let _ = black_box(db.get_file_data(key).await.unwrap());
        });
    });
}

fn bench_file_data_size(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("file_data_size");

    for size in [64, 1024, 16384, 262144].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.to_async(&rt).iter(|| async {
                let db = InMemoryFileDb::new();
                let id = FileDbId::new(1);
                let key = "test_key";
                let data = Bytes::from(vec![0u8; size]);

                db.set_file_data(id, black_box(key), black_box(data.clone()))
                    .await
                    .unwrap();
                let _ = black_box(db.get_file_data(key).await.unwrap());
            });
        });
    }

    group.finish();
}

fn bench_multiple_files(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("multiple_files");

    for count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            b.to_async(&rt).iter(|| async {
                let db = InMemoryFileDb::new();

                for i in 0..count {
                    let id = FileDbId::new(i as u64 + 1);
                    let key = format!("key_{}", i);
                    let data = Bytes::from(vec![0u8; 1024]);

                    db.set_file_data(id, black_box(&key), black_box(data))
                        .await
                        .unwrap();
                }

                // Verify all files
                for i in 0..count {
                    let key = format!("key_{}", i);
                    let _ = black_box(db.get_file_data(&key).await.unwrap());
                }
            });
        });
    }

    group.finish();
}

fn bench_concurrent_access(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("concurrent_access", |b| {
        b.to_async(&rt).iter(|| async {
            let db = InMemoryFileDb::new();

            // Store initial data
            for i in 0..100 {
                let id = FileDbId::new(i + 1);
                let key = format!("key_{}", i);
                let data = Bytes::from(vec![0u8; 1024]);

                db.set_file_data(id, &key, data).await.unwrap();
            }

            // Concurrent reads
            let mut handles = vec![];
            for i in 0..100 {
                let db = db.clone();
                let key = format!("key_{}", i);
                handles.push(tokio::spawn(async move {
                    let _ = black_box(db.get_file_data(&key).await.unwrap());
                }));
            }

            for handle in handles {
                handle.await.unwrap();
            }
        });
    });
}

fn bench_file_references(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("resolve_file_ref_chain_3", |b| {
        b.to_async(&rt).iter(|| async {
            let db = InMemoryFileDb::new();
            let id1 = FileDbId::new(1);
            let id2 = FileDbId::new(2);
            let id3 = FileDbId::new(3);
            let key = "test_key";
            let data = Bytes::from(vec![0u8; 1024]);

            db.set_file_data(id3, black_box(key), black_box(data))
                .await
                .unwrap();
            db.set_file_ref(id1, id2).await.unwrap();
            db.set_file_ref(id2, id3).await.unwrap();

            let _ = black_box(db.resolve_file_ref(id1).await.unwrap());
        });
    });
}

criterion_group!(
    benches,
    bench_get_next_file_db_id,
    bench_set_and_get_file_data,
    bench_file_data_size,
    bench_multiple_files,
    bench_concurrent_access,
    bench_file_references
);
criterion_main!(benches);
