// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Integration tests for the net module.

use bytes::Bytes;
use rustgram_net::prelude::*;

#[test]
fn test_dc_id_operations() {
    // Test DC ID creation
    let dc = DcId::internal(2);
    assert_eq!(dc.get_raw_id(), 2);
    assert!(!dc.is_external());
    assert!(dc.is_exact());
    assert!(!dc.is_empty());

    // Test external DC
    let dc_ext = DcId::external(2);
    assert_eq!(dc_ext.get_raw_id(), 2);
    assert!(dc_ext.is_external());

    // Test special DC IDs
    assert!(DcId::empty().is_empty());
    assert!(DcId::main().is_main());
    assert!(DcId::invalid().is_empty());

    // Test validation
    assert!(DcId::is_valid(1));
    assert!(DcId::is_valid(1000));
    assert!(!DcId::is_valid(0));
    assert!(!DcId::is_valid(1001));
}

#[test]
fn test_dc_options_integration() {
    let mut options = DcOptions::new();

    // Add some options
    options.add(DcOption::new(
        DcId::internal(1),
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(149, 154, 167, 51)),
        443,
    ));
    options.add(DcOption::new(
        DcId::internal(2),
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(149, 154, 167, 52)),
        443,
    ));

    assert_eq!(options.len(), 2);

    // Get options for DC
    let dc1_options = options.get_options(DcId::internal(1));
    assert_eq!(dc1_options.len(), 1);

    // Test DC options set
    let mut set = DcOptionsSet::new();
    set.add_options(options.clone());

    let best = set.find_best_option(DcId::internal(2), false);
    assert!(best.is_some());
}

#[test]
fn test_net_query_lifecycle() {
    let query = NetQuery::new(
        1,
        Bytes::from_static(b"test query"),
        DcId::internal(2),
        NetQueryType::Common,
        AuthFlag::On,
        GzipFlag::On,
        0x12345678,
    );

    // Initial state
    assert!(!query.is_ready());
    assert!(!query.is_ok());
    assert!(!query.is_error());

    // Set success
    let response = Bytes::from_static(b"response");
    query.set_ok(response.clone());

    assert!(query.is_ready());
    assert!(query.is_ok());
    assert_eq!(query.ok(), response);

    // Reset and set error
    query.clear();
    let error = QueryError::new(500, "Internal error");
    query.set_error(error.clone());

    assert!(query.is_ready());
    assert!(query.is_error());
    assert_eq!(query.error().code(), 500);
}

#[test]
fn test_query_error_codes() {
    let error = QueryError::new(202, "Resend");
    assert!(error.is_resend());
    assert_eq!(error.code(), 202);

    let error = QueryError::new(203, "Canceled");
    assert!(error.is_canceled());
    assert_eq!(error.code(), 203);

    let error = QueryError::new(204, "ResendInvokeAfter");
    assert!(error.is_resend_invoke_after());
    assert_eq!(error.code(), 204);
}

#[test]
fn test_proxy_operations() {
    // SOCKS5 proxy
    let socks5 = Proxy::socks5(
        "127.0.0.1".into(),
        1080,
        Some("user".into()),
        Some("pass".into()),
    );
    assert!(socks5.use_proxy());
    assert!(socks5.use_socks5_proxy());
    assert!(socks5.validate().is_ok());

    // MTProto proxy
    let secret = vec![1, 2, 3, 4];
    let mtproto = Proxy::mtproto("example.com".into(), 443, secret);
    assert!(mtproto.use_proxy());
    assert!(mtproto.use_mtproto_proxy());
    assert!(mtproto.validate().is_ok());

    // No proxy
    let none = Proxy::none();
    assert!(!none.use_proxy());

    // Invalid proxy
    let invalid = Proxy::socks5("".into(), 0, None, None);
    assert!(invalid.validate().is_err());
}

#[test]
fn test_connection_creator() {
    let creator = ConnectionCreator::new();

    // Check initial state
    assert!(creator.network_flag());
    assert_eq!(creator.network_generation(), 0);
    assert_eq!(creator.net_type(), NetType::Other);

    // Test network type
    creator.set_net_type(NetType::WiFi);
    assert_eq!(creator.net_type(), NetType::WiFi);

    // Test network flag
    creator.set_network_flag(false);
    assert!(!creator.network_flag());
    assert_eq!(creator.network_generation(), 1);

    // Test proxy
    let proxy = Proxy::socks5("127.0.0.1".into(), 1080, None, None);
    creator.set_proxy(proxy.clone());
    assert_eq!(creator.proxy(), proxy);
}

#[test]
fn test_session_operations() {
    let session = Session::new(2, DcId::internal(2), true, false, true, false);

    assert_eq!(session.raw_dc_id(), 2);
    assert_eq!(session.dc_id(), DcId::internal(2));
    assert!(session.is_primary());
    assert!(!session.is_main());
    assert!(session.use_pfs());
    assert!(!session.is_cdn());

    // Test query sending
    let query = NetQuery::new(
        1,
        Bytes::new(),
        DcId::internal(2),
        NetQueryType::Common,
        AuthFlag::On,
        GzipFlag::Off,
        0,
    );
    session.send(query);

    let stats = session.stats();
    assert_eq!(stats.queries_sent, 1);
}

#[test]
fn test_session_proxy() {
    let mut proxy = SessionProxy::new();

    assert!(proxy.main_session().is_none());
    assert!(proxy.download_session().is_none());
    assert!(proxy.upload_session().is_none());

    let main = Session::new(1, DcId::internal(1), true, true, true, false);
    proxy.set_main_session(main);

    assert!(proxy.main_session().is_some());
    assert!(proxy.main_session().unwrap().is_main());

    let download = Session::new(2, DcId::internal(2), true, false, true, false);
    proxy.set_download_session(download);

    assert!(proxy.download_session().is_some());
}

#[test]
fn test_network_statistics() {
    let mut stats = NetworkStats::new();

    assert!(stats.is_empty());
    assert_eq!(stats.len(), 0);

    // Add some entries
    stats.add_bytes(FileType::Photo, NetType::WiFi, 1000, 500);
    stats.add_bytes(FileType::Video, NetType::WiFi, 2000, 1000);

    assert!(!stats.is_empty());
    assert_eq!(stats.len(), 2);
    assert_eq!(stats.total_bytes(), 4500);

    // Filter active entries
    let active = stats.filter_active();
    assert_eq!(active.len(), 2);

    // Reset
    stats.reset();
    assert!(stats.is_empty());
}

#[test]
fn test_net_stats_manager() {
    let mut manager = NetStatsManager::new();

    // Add file stats - Photo goes to media_stats
    manager.add_stats(&NetworkStatsEntry::file(
        FileType::Photo,
        NetType::WiFi,
        1000,
        500,
    ));

    // Photo is a media file, so check media_stats
    let media = manager.media_stats();
    assert_eq!(media.rx, 1000);
    assert_eq!(media.tx, 500);

    // Get snapshot
    let snapshot = manager.get_network_stats();
    assert_eq!(snapshot.len(), 1);

    // Reset
    manager.reset();
    let snapshot = manager.get_network_stats();
    assert!(snapshot.is_empty());
}

#[test]
fn test_auth_data_shared() {
    let auth_data = AuthDataShared::new(DcId::internal(2));

    assert_eq!(auth_data.dc_id(), DcId::internal(2));
    assert_eq!(auth_data.auth_key_state(), AuthKeyState::Empty);

    // Set auth key
    let key = AuthKey::new(123, vec![1, 2, 3, 4]);
    auth_data.set_auth_key(key.clone());

    assert_eq!(auth_data.auth_key_state(), AuthKeyState::Ready);
    assert_eq!(auth_data.get_auth_key(), Some(key));

    // Clear
    auth_data.clear();
    assert_eq!(auth_data.auth_key_state(), AuthKeyState::Empty);
}

#[test]
fn test_auth_key_operations() {
    let key = AuthKey::new(123, vec![1, 2, 3, 4]);
    assert_eq!(key.id, 123);
    assert_eq!(key.len(), 4);
    assert!(!key.is_temporary());
    assert!(!key.is_expired());

    let expires = std::time::Instant::now() + std::time::Duration::from_secs(60);
    let temp_key = AuthKey::temporary(456, vec![5, 6, 7, 8], expires);
    assert!(temp_key.is_temporary());
    assert!(!temp_key.is_expired());
}

#[test]
fn test_server_salt() {
    let salt = ServerSalt::new(12345, 1000);
    assert_eq!(salt.salt, 12345);
    assert!(salt.is_valid(1000));
    assert!(salt.is_valid(2000));
    assert!(!salt.is_valid(999));
}

#[test]
fn test_net_type_operations() {
    assert_eq!(NetType::WiFi.as_str(), "wifi");
    assert_eq!(NetType::Mobile.as_str(), "mobile");
    assert_eq!(NetType::MobileRoaming.as_str(), "mobile_roaming");

    assert!(NetType::Mobile.is_mobile());
    assert!(NetType::MobileRoaming.is_mobile());
    assert!(!NetType::WiFi.is_mobile());

    assert!(NetType::MobileRoaming.is_roaming());
    assert!(!NetType::Mobile.is_roaming());

    assert_eq!(NetType::from_str("wifi"), Some(NetType::WiFi));
    assert_eq!(NetType::from_str("invalid"), None);
}

#[test]
fn test_connection_stats() {
    let mut stats = ConnectionStats::default();

    stats.record_success(100, 200, std::time::Duration::from_millis(100));
    stats.record_success(50, 100, std::time::Duration::from_millis(200));

    assert_eq!(stats.bytes_sent, 150);
    assert_eq!(stats.bytes_received, 300);
    assert_eq!(stats.connection_count, 2);
    assert_eq!(stats.avg_rtt, std::time::Duration::from_millis(150));
    assert!((stats.success_rate() - 1.0).abs() < f64::EPSILON);

    stats.record_failure();
    assert_eq!(stats.failure_count, 1);
    assert!((stats.success_rate() - 0.666).abs() < 0.01);
}

#[test]
fn test_dc_option_flags() {
    let dc_id = DcId::internal(2);

    let mut option = DcOption::new(
        dc_id,
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(149, 154, 167, 51)),
        443,
    );

    assert!(!option.is_static());
    assert!(!option.is_ipv6());
    assert!(!option.is_cdn());

    option = option.with_flag(DcOptionFlag::Static);
    assert!(option.is_static());

    option = option.with_flag(DcOptionFlag::Cdn);
    assert!(option.is_cdn());
}

#[test]
fn test_query_timeout_operations() {
    let query = NetQuery::new(
        1,
        Bytes::new(),
        DcId::internal(2),
        NetQueryType::Common,
        AuthFlag::On,
        GzipFlag::Off,
        0,
    );

    assert_eq!(query.total_timeout(), std::time::Duration::ZERO);
    assert_eq!(
        query.total_timeout_limit(),
        std::time::Duration::from_secs(60)
    );

    query.add_total_timeout(std::time::Duration::from_secs(10));
    assert_eq!(query.total_timeout(), std::time::Duration::from_secs(10));

    query.set_next_timeout(std::time::Duration::from_secs(5));
    assert_eq!(query.next_timeout(), std::time::Duration::from_secs(5));

    query.set_source("test_source".into());
    assert_eq!(query.source(), "test_source");
}

#[test]
fn test_net_query_callback_trait() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    struct TestCallback {
        called: Arc<AtomicBool>,
    }

    #[async_trait::async_trait]
    impl NetQueryCallback for TestCallback {
        async fn on_result(&self, _query: NetQuery) {
            self.called.store(true, Ordering::Relaxed);
        }
    }

    let called = Arc::new(AtomicBool::new(false));
    let callback = TestCallback {
        called: called.clone(),
    };

    let query = NetQuery::new(
        1,
        Bytes::new(),
        DcId::internal(2),
        NetQueryType::Common,
        AuthFlag::On,
        GzipFlag::Off,
        0,
    );

    query.set_callback(Box::new(callback));

    // The callback would be invoked in a real scenario
    assert!(!called.load(Ordering::Relaxed));
}

#[test]
fn test_raw_connection() {
    let conn = RawConnection::new(DcId::internal(2), ConnectionMode::Tcp, false);

    assert_eq!(conn.dc_id, DcId::internal(2));
    assert_eq!(conn.mode, ConnectionMode::Tcp);
    assert!(!conn.is_media);
    assert!(conn.socket.is_none());
    assert!(!conn.is_valid()); // No socket, so not valid
}

#[test]
fn test_session_stats() {
    let mut stats = SessionStats::default();

    assert_eq!(stats.queries_sent, 0);
    assert_eq!(stats.queries_received, 0);

    stats.queries_sent = 10;
    stats.queries_received = 8;
    stats.failures = 2;

    assert_eq!(stats.queries_sent, 10);
    assert_eq!(stats.queries_received, 8);
    assert_eq!(stats.failures, 2);
}
