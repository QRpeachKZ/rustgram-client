// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Network query creator.
//!
//! This module implements TDLib's NetQueryCreator from `td/telegram/net/NetQueryCreator.h`.
//!
//! Provides factory methods for creating NetQuery instances with proper configuration.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use bytes::Bytes;

use crate::dc::DcId;
use crate::query::{AuthFlag, GzipFlag, NetQuery, NetQueryId, NetQueryType};

/// Unique ID generator for queries.
static NEXT_QUERY_ID: AtomicU64 = AtomicU64::new(1);

/// Network query statistics.
///
/// Based on TDLib's NetQueryStats from `td/telegram/net/NetQueryStats.h`.
#[derive(Debug, Default, Clone)]
pub struct NetQueryStats {
    /// Total number of queries created
    pub total_queries: u64,
    /// Number of active queries
    pub active_queries: u64,
    /// Number of completed queries
    pub completed_queries: u64,
}

impl NetQueryStats {
    /// Creates new query statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a query creation.
    pub fn on_query_created(&self) {
        // In a real implementation, this would use atomic counters
        // For now, we'll keep it simple
    }

    /// Records a query completion.
    pub fn on_query_completed(&self) {
        // In a real implementation, this would use atomic counters
    }
}

/// Network query creator.
///
/// Factory for creating NetQuery instances with proper IDs and configuration.
/// Based on TDLib's NetQueryCreator from `td/telegram/net/NetQueryCreator.h`.
#[derive(Debug, Clone)]
pub struct NetQueryCreator {
    /// Query statistics tracker
    stats: Arc<NetQueryStats>,
    /// Whether to stop checking for empty objects
    check_empty: bool,
}

impl NetQueryCreator {
    /// Creates a new query creator.
    ///
    /// # Arguments
    ///
    /// * `stats` - Query statistics tracker
    pub fn new(stats: Arc<NetQueryStats>) -> Self {
        Self {
            stats,
            check_empty: true,
        }
    }

    /// Stops empty object checking.
    ///
    /// This should be called during shutdown to allow cleanup.
    pub fn stop_check(&mut self) {
        self.check_empty = false;
    }

    /// Creates a new network query.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - Target data center ID
    /// * `query_type` - Type of query (common, upload, download)
    /// * `auth_flag` - Whether authentication is required
    /// * `gzip_flag` - Whether to use gzip compression
    /// * `data` - Query data/bytes
    ///
    /// # Returns
    ///
    /// A new NetQuery instance with a unique ID.
    pub fn create(
        &self,
        dc_id: DcId,
        query_type: NetQueryType,
        auth_flag: AuthFlag,
        gzip_flag: GzipFlag,
        data: Vec<u8>,
    ) -> NetQuery {
        let id = NEXT_QUERY_ID.fetch_add(1, Ordering::SeqCst);
        self.stats.on_query_created();

        NetQuery::new(
            id,
            Bytes::from(data),
            dc_id,
            query_type,
            auth_flag,
            gzip_flag,
            0, // tl_constructor
        )
    }

    /// Creates an unauthenticated query.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - Target data center ID
    /// * `data` - Query data/bytes
    ///
    /// # Returns
    ///
    /// A new unauthenticated NetQuery instance.
    pub fn create_unauth(&self, dc_id: DcId, data: Vec<u8>) -> NetQuery {
        self.create(
            dc_id,
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            data,
        )
    }

    /// Creates a query with a specific ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Specific query ID to use
    /// * `dc_id` - Target data center ID
    /// * `query_type` - Type of query
    /// * `auth_flag` - Whether authentication is required
    /// * `gzip_flag` - Whether to use gzip compression
    /// * `data` - Query data/bytes
    ///
    /// # Returns
    ///
    /// A new NetQuery instance with the specified ID.
    pub fn create_with_id(
        &self,
        id: NetQueryId,
        dc_id: DcId,
        query_type: NetQueryType,
        auth_flag: AuthFlag,
        gzip_flag: GzipFlag,
        data: Vec<u8>,
    ) -> NetQuery {
        self.stats.on_query_created();
        NetQuery::new(
            id,
            Bytes::from(data),
            dc_id,
            query_type,
            auth_flag,
            gzip_flag,
            0, // tl_constructor
        )
    }

    /// Creates a common query with authentication.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - Target data center ID (defaults to main DC)
    /// * `data` - Query data/bytes
    ///
    /// # Returns
    ///
    /// A new authenticated common NetQuery instance.
    pub fn create_common(&self, dc_id: DcId, data: Vec<u8>) -> NetQuery {
        self.create(
            dc_id,
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            data,
        )
    }

    /// Creates an upload query.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - Target data center ID
    /// * `data` - Query data/bytes
    ///
    /// # Returns
    ///
    /// A new upload NetQuery instance.
    pub fn create_upload(&self, dc_id: DcId, data: Vec<u8>) -> NetQuery {
        self.create(
            dc_id,
            NetQueryType::Upload,
            AuthFlag::On,
            GzipFlag::Off,
            data,
        )
    }

    /// Creates a download query.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - Target data center ID
    /// * `data` - Query data/bytes
    /// * `small` - Whether this is a small file download
    ///
    /// # Returns
    ///
    /// A new download NetQuery instance.
    pub fn create_download(&self, dc_id: DcId, data: Vec<u8>, small: bool) -> NetQuery {
        let query_type = if small {
            NetQueryType::DownloadSmall
        } else {
            NetQueryType::Download
        };

        self.create(dc_id, query_type, AuthFlag::On, GzipFlag::Off, data)
    }

    /// Gets reference to the query statistics.
    pub fn stats(&self) -> &Arc<NetQueryStats> {
        &self.stats
    }
}

impl Default for NetQueryCreator {
    fn default() -> Self {
        Self::new(Arc::new(NetQueryStats::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_query() {
        let creator = NetQueryCreator::default();
        let dc_id = DcId::main();

        let query = creator.create_unauth(dc_id, vec![1, 2, 3]);

        assert_eq!(query.dc_id(), dc_id);
        assert_eq!(query.auth_flag(), AuthFlag::Off);
        assert_eq!(query.query_type(), NetQueryType::Common);
        assert!(!query.query().is_empty());
    }

    #[test]
    fn test_create_with_id() {
        let creator = NetQueryCreator::default();
        let dc_id = DcId::main();
        let custom_id = 12345;

        let query = creator.create_with_id(
            custom_id,
            dc_id,
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            vec![1, 2, 3],
        );

        assert_eq!(query.id(), custom_id);
    }

    #[test]
    fn test_query_ids_are_unique() {
        let creator = NetQueryCreator::default();
        let dc_id = DcId::main();

        let query1 = creator.create_unauth(dc_id, vec![1]);
        let query2 = creator.create_unauth(dc_id, vec![2]);

        assert_ne!(query1.id(), query2.id());
    }

    #[test]
    fn test_create_common() {
        let creator = NetQueryCreator::default();
        let dc_id = DcId::main();

        let query = creator.create_common(dc_id, vec![1, 2, 3]);

        assert_eq!(query.auth_flag(), AuthFlag::On);
        assert_eq!(query.query_type(), NetQueryType::Common);
    }

    #[test]
    fn test_create_upload() {
        let creator = NetQueryCreator::default();
        let dc_id = DcId::main();

        let query = creator.create_upload(dc_id, vec![1, 2, 3]);

        assert_eq!(query.query_type(), NetQueryType::Upload);
        assert_eq!(query.auth_flag(), AuthFlag::On);
    }

    #[test]
    fn test_create_download() {
        let creator = NetQueryCreator::default();
        let dc_id = DcId::main();

        let query = creator.create_download(dc_id, vec![1, 2, 3], false);

        assert_eq!(query.query_type(), NetQueryType::Download);
    }

    #[test]
    fn test_create_download_small() {
        let creator = NetQueryCreator::default();
        let dc_id = DcId::main();

        let query = creator.create_download(dc_id, vec![1, 2, 3], true);

        assert_eq!(query.query_type(), NetQueryType::DownloadSmall);
    }

    #[test]
    fn test_stop_check() {
        let mut creator = NetQueryCreator::default();
        assert!(creator.check_empty);

        creator.stop_check();
        assert!(!creator.check_empty);
    }

    #[test]
    fn test_stats_reference() {
        let creator = NetQueryCreator::default();
        let stats = creator.stats();

        assert_eq!(stats.total_queries, 0);
        assert_eq!(stats.active_queries, 0);
    }
}
