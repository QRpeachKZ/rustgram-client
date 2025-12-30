// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Benchmarks for network query operations.

use bytes::Bytes;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rustgram_net::prelude::*;

fn bench_dc_id_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("dc_id");

    group.bench_function("internal", |b| {
        b.iter(|| DcId::internal(black_box(2)));
    });

    group.bench_function("external", |b| {
        b.iter(|| DcId::external(black_box(2)));
    });

    group.bench_function("validation", |b| {
        b.iter(|| DcId::is_valid(black_box(2)));
    });

    group.finish();
}

fn bench_net_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("net_query");

    group.bench_function("creation", |b| {
        b.iter(|| {
            NetQuery::new(
                black_box(1),
                black_box(Bytes::new()),
                black_box(DcId::internal(2)),
                black_box(NetQueryType::Common),
                black_box(AuthFlag::On),
                black_box(GzipFlag::Off),
                black_box(0),
            )
        });
    });

    let query = NetQuery::new(
        1,
        Bytes::new(),
        DcId::internal(2),
        NetQueryType::Common,
        AuthFlag::On,
        GzipFlag::Off,
        0,
    );

    group.bench_function("state_check", |b| {
        b.iter(|| black_box(query.is_ready()));
    });

    group.bench_function("set_ok", |b| {
        b.iter(|| {
            let data = black_box(Bytes::from_static(b"test"));
            black_box(&query).set_ok(data);
        });
    });

    group.finish();
}

fn bench_dc_options(c: &mut Criterion) {
    let mut group = c.benchmark_group("dc_options");

    group.bench_function("add_option", |b| {
        let mut options = DcOptions::new();
        let dc_id = DcId::internal(2);
        let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(149, 154, 167, 51));

        b.iter(|| {
            let option = DcOption::new(black_box(dc_id), black_box(ip), 443);
            black_box(&mut options).add(option);
        });
    });

    let mut options = DcOptions::new();
    for i in 1..=10 {
        options.add(DcOption::new(
            DcId::internal(i),
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(149, 154, 167, 50 + i as u8)),
            443,
        ));
    }

    group.bench_function("get_options_10", |b| {
        b.iter(|| {
            black_box(&options).get_options(black_box(DcId::internal(2)));
        });
    });

    group.finish();
}

fn bench_proxy(c: &mut Criterion) {
    let mut group = c.benchmark_group("proxy");

    group.bench_function("socks5_creation", |b| {
        b.iter(|| {
            Proxy::socks5(
                black_box("127.0.0.1".to_string()),
                black_box(1080),
                black_box(Some("user".to_string())),
                black_box(Some("pass".to_string())),
            )
        });
    });

    let proxy = Proxy::socks5(
        "127.0.0.1".into(),
        1080,
        Some("user".into()),
        Some("pass".into()),
    );

    group.bench_function("validate", |b| {
        b.iter(|| black_box(&proxy).validate());
    });

    group.finish();
}

fn bench_stats(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats");

    group.bench_function("entry_add_bytes", |b| {
        let mut entry = NetworkStatsEntry::file(FileType::Photo, NetType::WiFi, 0, 0);
        b.iter(|| {
            black_box(&mut entry).add_bytes(black_box(1000), black_box(500));
        });
    });

    group.bench_function("manager_add_stats", |b| {
        let mut manager = NetStatsManager::new();
        let entry = NetworkStatsEntry::file(FileType::Photo, NetType::WiFi, 1000, 500);

        b.iter(|| {
            black_box(&mut manager).add_stats(black_box(&entry));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_dc_id_creation,
    bench_net_query,
    bench_dc_options,
    bench_proxy,
    bench_stats
);
criterion_main!(benches);
