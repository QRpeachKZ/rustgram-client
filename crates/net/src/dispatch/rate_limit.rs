// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Flood control for rate limiting.
//!
//! This module implements flood control to prevent hitting Telegram rate limits.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use parking_lot::Mutex;

use crate::dc::DcId;
use crate::query::NetQuery;

/// Flood control configuration.
#[derive(Debug, Clone, Copy)]
pub struct FloodControlConfig {
    /// Maximum queries per second
    pub max_queries_per_second: u32,

    /// Burst size (how many queries can be sent instantly)
    pub burst_size: u32,

    /// Time window for rate limiting
    pub window_duration: Duration,

    /// Whether to enforce per-DC limits
    pub per_dc_limits: bool,
}

impl Default for FloodControlConfig {
    fn default() -> Self {
        Self {
            max_queries_per_second: 30,
            burst_size: 5,
            window_duration: Duration::from_secs(1),
            per_dc_limits: true,
        }
    }
}

/// Flood control result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloodControlResult {
    /// Query is allowed
    Allowed,

    /// Query should be delayed
    Delayed(Duration),

    /// Query should be dropped
    Dropped,

    /// Wait is required (flood wait)
    FloodWait(u32), // seconds to wait
}

/// Per-DC flood control statistics.
#[derive(Debug, Clone)]
struct DcFloodStats {
    /// Queries sent in current window
    queries_in_window: u32,

    /// Window start time
    window_start: Instant,

    /// Current burst tokens
    burst_tokens: u32,

    /// Last burst refill
    last_refill: Instant,
}

impl Default for DcFloodStats {
    fn default() -> Self {
        Self {
            queries_in_window: 0,
            window_start: Instant::now(),
            burst_tokens: 0,
            last_refill: Instant::now(),
        }
    }
}

/// Flood control manager.
///
/// Prevents hitting Telegram rate limits by managing query send rates.
pub struct FloodControl {
    /// Configuration
    config: FloodControlConfig,

    /// Global flood control stats
    global_stats: Mutex<DcFloodStats>,

    /// Per-DC stats (if enabled)
    dc_stats: Mutex<HashMap<i32, DcFloodStats>>,

    /// Total query counter
    total_sent: AtomicU32,

    /// Flood wait counter
    flood_wait_count: AtomicU32,
}

impl FloodControl {
    /// Creates a new flood control manager.
    pub fn new(config: FloodControlConfig) -> Self {
        Self {
            config,
            global_stats: Mutex::new(DcFloodStats::default()),
            dc_stats: Mutex::new(HashMap::new()),
            total_sent: AtomicU32::new(0),
            flood_wait_count: AtomicU32::new(0),
        }
    }

    /// Checks if a query is allowed to be sent.
    pub fn check_query(&self, query: &NetQuery) -> FloodControlResult {
        let now = Instant::now();

        // Check global limits first
        let global_result = self.check_global(now);
        if !matches!(global_result, FloodControlResult::Allowed) {
            return global_result;
        }

        // Check per-DC limits if enabled
        if self.config.per_dc_limits {
            let dc_result = self.check_dc(query.dc_id(), now);
            if !matches!(dc_result, FloodControlResult::Allowed) {
                return dc_result;
            }
        }

        // Record the query
        self.record_query(query);

        FloodControlResult::Allowed
    }

    fn check_global(&self, now: Instant) -> FloodControlResult {
        let mut stats = self.global_stats.lock();

        // Refill burst tokens
        let elapsed = now.duration_since(stats.last_refill);
        let refill_tokens = (elapsed.as_secs_f64() * self.config.burst_size as f64
            / self.config.window_duration.as_secs_f64()) as u32;

        stats.burst_tokens = (stats.burst_tokens + refill_tokens).min(self.config.burst_size);
        stats.last_refill = now;

        // Check if we have burst tokens
        if stats.burst_tokens > 0 {
            stats.burst_tokens -= 1;
            return FloodControlResult::Allowed;
        }

        // Check window-based rate limit
        if now.duration_since(stats.window_start) >= self.config.window_duration {
            stats.queries_in_window = 0;
            stats.window_start = now;
        }

        if stats.queries_in_window < self.config.max_queries_per_second {
            stats.queries_in_window += 1;
            FloodControlResult::Allowed
        } else {
            // Calculate wait time
            let wait = self.config.window_duration - now.duration_since(stats.window_start);
            FloodControlResult::Delayed(wait)
        }
    }

    fn check_dc(&self, dc_id: DcId, now: Instant) -> FloodControlResult {
        let mut dc_stats = self.dc_stats.lock();
        let dc_raw = dc_id.get_raw_id();

        let stats = dc_stats.entry(dc_raw).or_insert_with(|| DcFloodStats {
            queries_in_window: 0,
            window_start: now,
            burst_tokens: self.config.burst_size,
            last_refill: now,
        });

        // Same logic as global check but for specific DC
        let elapsed = now.duration_since(stats.last_refill);
        let refill_tokens = (elapsed.as_secs_f64() * self.config.burst_size as f64
            / self.config.window_duration.as_secs_f64()) as u32;

        stats.burst_tokens = (stats.burst_tokens + refill_tokens).min(self.config.burst_size);
        stats.last_refill = now;

        if stats.burst_tokens > 0 {
            stats.burst_tokens -= 1;
            return FloodControlResult::Allowed;
        }

        if now.duration_since(stats.window_start) >= self.config.window_duration {
            stats.queries_in_window = 0;
            stats.window_start = now;
        }

        if stats.queries_in_window < self.config.max_queries_per_second {
            stats.queries_in_window += 1;
            FloodControlResult::Allowed
        } else {
            let wait = self.config.window_duration - now.duration_since(stats.window_start);
            FloodControlResult::Delayed(wait)
        }
    }

    fn record_query(&self, _query: &NetQuery) {
        self.total_sent.fetch_add(1, Ordering::Relaxed);
    }

    /// Handles a flood wait error from the server.
    pub fn on_flood_wait(&self, dc_id: DcId, seconds: u32) {
        self.flood_wait_count.fetch_add(1, Ordering::Relaxed);

        if self.config.per_dc_limits {
            let mut dc_stats = self.dc_stats.lock();
            let stats = dc_stats.entry(dc_id.get_raw_id()).or_default();

            // Throttle this DC
            stats.queries_in_window = self.config.max_queries_per_second;
            stats.window_start = Instant::now() + Duration::from_secs(seconds as u64);
        }
    }

    /// Returns the total number of queries sent.
    pub fn total_sent(&self) -> u32 {
        self.total_sent.load(Ordering::Relaxed)
    }

    /// Returns the number of flood waits received.
    pub fn flood_wait_count(&self) -> u32 {
        self.flood_wait_count.load(Ordering::Relaxed)
    }

    /// Resets statistics.
    pub fn reset(&self) {
        *self.global_stats.lock() = DcFloodStats::default();
        self.dc_stats.lock().clear();
        self.total_sent.store(0, Ordering::Relaxed);
        self.flood_wait_count.store(0, Ordering::Relaxed);
    }
}

impl Default for FloodControl {
    fn default() -> Self {
        Self::new(FloodControlConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{AuthFlag, GzipFlag, NetQueryType};
    use bytes::Bytes;

    fn create_test_query(dc_id: DcId) -> NetQuery {
        NetQuery::new(
            1,
            Bytes::new(),
            dc_id,
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        )
    }

    #[test]
    fn test_flood_control_config_default() {
        let config = FloodControlConfig::default();
        assert_eq!(config.max_queries_per_second, 30);
        assert_eq!(config.burst_size, 5);
        assert!(config.per_dc_limits);
    }

    #[test]
    fn test_flood_control_new() {
        let flood = FloodControl::new(FloodControlConfig::default());

        assert_eq!(flood.total_sent(), 0);
        assert_eq!(flood.flood_wait_count(), 0);
    }

    #[test]
    fn test_flood_control_check_allowed() {
        let flood = FloodControl::new(FloodControlConfig::default());
        let query = create_test_query(DcId::internal(2));

        let result = flood.check_query(&query);

        assert_eq!(result, FloodControlResult::Allowed);
        assert_eq!(flood.total_sent(), 1);
    }

    #[test]
    fn test_flood_control_burst() {
        let config = FloodControlConfig {
            max_queries_per_second: 100,
            burst_size: 5,
            window_duration: Duration::from_secs(1),
            per_dc_limits: false,
        };

        let flood = FloodControl::new(config);

        // Send burst queries - all should be allowed within burst size
        for i in 0..5 {
            let query = create_test_query(DcId::internal(2));
            let result = flood.check_query(&query);
            assert_eq!(
                result,
                FloodControlResult::Allowed,
                "Query {} should be allowed",
                i
            );
        }

        // With high max_queries_per_second and burst size,
        // the 6th query should still be allowed
        let query = create_test_query(DcId::internal(2));
        let result = flood.check_query(&query);
        assert_eq!(result, FloodControlResult::Allowed);
    }

    #[test]
    fn test_flood_control_on_flood_wait() {
        let flood = FloodControl::new(FloodControlConfig::default());

        flood.on_flood_wait(DcId::internal(2), 60);

        assert_eq!(flood.flood_wait_count(), 1);
    }

    #[test]
    fn test_flood_control_reset() {
        let flood = FloodControl::new(FloodControlConfig::default());

        let query = create_test_query(DcId::internal(2));
        flood.check_query(&query);
        flood.on_flood_wait(DcId::internal(2), 10);

        flood.reset();

        assert_eq!(flood.total_sent(), 0);
        assert_eq!(flood.flood_wait_count(), 0);
    }

    #[test]
    fn test_flood_control_result_variants() {
        let _ = FloodControlResult::Allowed;
        let _ = FloodControlResult::Delayed(Duration::from_secs(1));
        let _ = FloodControlResult::Dropped;
        let _ = FloodControlResult::FloodWait(60);
    }
}
