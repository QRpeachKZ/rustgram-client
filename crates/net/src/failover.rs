// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Multi-DC failover for high availability.
//!
//! This module implements automatic failover between Telegram data centers,
//! ensuring continuous connectivity even when individual DCs fail.
//!
//! # Architecture
//!
//! The failover manager:
//! - Tracks health metrics for each DC
//! - Automatically switches to backup DCs on failure
//! - Implements health checks and recovery detection
//! - Supports different failover policies
//!
//! # References
//!
//! - TDLib: `td/telegram/net/DcOptions.h`
//! - TDLib: `td/telegram/net/ConnectionCreator.h`

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::dc::DcId;
use crate::query::QueryError;

/// Request type determines failover strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestType {
    /// User data requests (require main DC)
    Main,

    /// Download requests (can use any DC)
    Download,

    /// Upload requests (prefer main DC, can failover)
    Upload,

    /// Auth requests (must use specific DC)
    Auth,
}

/// Error types for failover operations.
#[derive(Debug, Error)]
pub enum FailoverError {
    /// No healthy DCs available
    #[error("No healthy DCs available for request type: {0:?}")]
    NoHealthyDc(RequestType),

    /// All DCs failed
    #[error("All DCs failed")]
    AllDcsFailed,

    /// Invalid DC ID
    #[error("Invalid DC ID: {0:?}")]
    InvalidDcId(DcId),

    /// Failover disabled
    #[error("Failover is disabled")]
    Disabled,
}

/// Health status of a DC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcHealth {
    /// DC is healthy
    Healthy,

    /// DC is degraded (slow but responding)
    Degraded,

    /// DC is unhealthy (not responding)
    Unhealthy,

    /// DC is in cooldown (recently failed, waiting before retry)
    Cooldown,
}

impl DcHealth {
    /// Returns `true` if the DC is considered available.
    pub fn is_available(self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Returns `true` if the DC is considered healthy.
    pub fn is_healthy(self) -> bool {
        matches!(self, Self::Healthy)
    }
}

/// Health metrics for a DC.
#[derive(Debug, Clone)]
struct DcMetrics {
    /// Current health status
    health: DcHealth,

    /// Number of successful requests
    success_count: u64,

    /// Number of failed requests
    failure_count: u64,

    /// Average latency (moving average)
    avg_latency: Duration,

    /// Time of last success
    last_success: Option<Instant>,

    /// Time of last failure
    last_failure: Option<Instant>,

    /// When cooldown ends (if in cooldown)
    cooldown_until: Option<Instant>,

    /// Number of consecutive failures
    consecutive_failures: u32,
}

impl DcMetrics {
    /// Creates new DC metrics.
    fn new() -> Self {
        Self {
            health: DcHealth::Healthy,
            success_count: 0,
            failure_count: 0,
            avg_latency: Duration::from_millis(100),
            last_success: None,
            last_failure: None,
            cooldown_until: None,
            consecutive_failures: 0,
        }
    }

    /// Updates metrics with a successful request.
    fn record_success(&mut self, latency: Duration, config: &FailoverPolicy) {
        self.success_count += 1;
        self.consecutive_failures = 0;
        self.last_success = Some(Instant::now());

        // Update average latency (exponential moving average)
        let alpha = 0.2;
        let avg_ms = self.avg_latency.as_millis() as f64;
        let new_ms = latency.as_millis() as f64;
        self.avg_latency = Duration::from_millis((alpha * new_ms + (1.0 - alpha) * avg_ms) as u64);

        // Update health based on latency
        if self.avg_latency > config.slow_threshold {
            self.health = DcHealth::Degraded;
        } else {
            self.health = DcHealth::Healthy;
        }
    }

    /// Updates metrics with a failed request.
    fn record_failure(&mut self, _error: &QueryError, config: &FailoverPolicy) {
        self.failure_count += 1;
        self.consecutive_failures += 1;
        self.last_failure = Some(Instant::now());

        // Mark as unhealthy after threshold failures
        if self.consecutive_failures >= config.failure_threshold {
            self.health = DcHealth::Unhealthy;

            // Enter cooldown
            self.cooldown_until = Some(Instant::now() + config.cooldown_duration);
        }
    }

    /// Enters cooldown.
    fn enter_cooldown(&mut self, duration: Duration) {
        self.health = DcHealth::Cooldown;
        self.cooldown_until = Some(Instant::now() + duration);
    }

    /// Exits cooldown (if ready).
    fn check_cooldown(&mut self) -> bool {
        if self.health == DcHealth::Cooldown {
            if let Some(cooldown_until) = self.cooldown_until {
                if Instant::now() >= cooldown_until {
                    self.health = DcHealth::Healthy;
                    self.consecutive_failures = 0;
                    return true;
                }
            }
        }
        false
    }

    /// Returns `true` if the DC is available for requests.
    fn is_available(&self) -> bool {
        if self.health == DcHealth::Cooldown {
            if let Some(cooldown_until) = self.cooldown_until {
                return Instant::now() >= cooldown_until;
            }
        }
        self.health.is_available()
    }
}

impl Default for DcMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Failover policy configuration.
#[derive(Debug, Clone)]
pub struct FailoverPolicy {
    /// Number of consecutive failures before marking DC as unhealthy.
    /// Default: 3
    pub failure_threshold: u32,

    /// Cooldown duration before retrying a failed DC.
    /// Default: 60 seconds
    pub cooldown_duration: Duration,

    /// Latency threshold for considering a DC degraded.
    /// Default: 500ms
    pub slow_threshold: Duration,

    /// Whether automatic failover is enabled.
    /// Default: true
    pub auto_failover: bool,

    /// Maximum number of failover attempts.
    /// Default: 3
    pub max_failovers: usize,

    /// Whether to prefer the main DC when it recovers.
    /// Default: true
    pub prefer_main_on_recovery: bool,
}

impl Default for FailoverPolicy {
    fn default() -> Self {
        Self {
            failure_threshold: 3,
            cooldown_duration: Duration::from_secs(60),
            slow_threshold: Duration::from_millis(500),
            auto_failover: true,
            max_failovers: 3,
            prefer_main_on_recovery: true,
        }
    }
}

/// Multi-DC failover manager.
///
/// Tracks DC health and automatically switches between DCs
/// to maintain connectivity.
pub struct FailoverManager {
    /// DC health metrics (dc_id -> metrics)
    dc_health: Mutex<HashMap<i32, DcMetrics>>,

    /// Current main DC
    current_dc: Arc<Mutex<DcId>>,

    /// Failover policy
    policy: FailoverPolicy,

    /// Available DCs
    available_dcs: Mutex<Vec<DcId>>,

    /// Failover count
    failover_count: Arc<Mutex<usize>>,
}

impl FailoverManager {
    /// Creates a new failover manager with default policy.
    pub fn new(main_dc: DcId) -> Self {
        Self::with_policy(main_dc, FailoverPolicy::default())
    }

    /// Creates a new failover manager with the given policy.
    pub fn with_policy(main_dc: DcId, policy: FailoverPolicy) -> Self {
        let manager = Self {
            dc_health: Mutex::new(HashMap::new()),
            current_dc: Arc::new(Mutex::new(main_dc)),
            policy,
            available_dcs: Mutex::new(Vec::new()),
            failover_count: Arc::new(Mutex::new(0)),
        };

        // Initialize main DC as healthy
        manager.add_dc(main_dc);

        manager
    }

    /// Returns the failover policy.
    pub fn policy(&self) -> &FailoverPolicy {
        &self.policy
    }

    /// Returns the current DC.
    pub fn current_dc(&self) -> DcId {
        *self.current_dc.lock()
    }

    /// Adds a DC to the available list.
    #[allow(clippy::unwrap_or_default)]
    pub fn add_dc(&self, dc_id: DcId) {
        let mut health = self.dc_health.lock();
        health.entry(dc_id.get_raw_id()).or_insert_with(DcMetrics::new);

        let mut available = self.available_dcs.lock();
        available.push(dc_id);
        available.sort_by_key(|dc| dc.get_raw_id());
        available.dedup();
    }

    /// Removes a DC from the available list.
    pub fn remove_dc(&self, dc_id: DcId) {
        let mut available = self.available_dcs.lock();
        available.retain(|dc| dc != &dc_id);
        let mut health = self.dc_health.lock();
        health.remove(&dc_id.get_raw_id());
    }

    /// Records a successful request to a DC.
    pub fn report_success(&self, dc_id: DcId, latency: Duration) {
        let mut health = self.dc_health.lock();
        if let Some(metrics) = health.get_mut(&dc_id.get_raw_id()) {
            metrics.record_success(latency, &self.policy);
        }
    }

    /// Records a failed request to a DC.
    pub fn report_failure(&self, dc_id: DcId, error: &QueryError) {
        let mut health = self.dc_health.lock();
        if let Some(metrics) = health.get_mut(&dc_id.get_raw_id()) {
            metrics.record_failure(error, &self.policy);
        }
    }

    /// Gets the best DC for a request type.
    ///
    /// # Arguments
    ///
    /// * `request_type` - The type of request
    ///
    /// # Returns
    ///
    /// The recommended DC ID
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustgram_net::failover::{FailoverManager, RequestType};
    ///
    /// let manager = FailoverManager::new(main_dc);
    /// let dc = manager.get_best_dc(RequestType::Main)?;
    /// ```
    pub fn get_best_dc(&self, request_type: RequestType) -> Result<DcId, FailoverError> {
        if !self.policy.auto_failover {
            return Ok(self.current_dc());
        }

        match request_type {
            RequestType::Auth => {
                // Auth requests must use specific DC (current DC)
                if self.is_dc_healthy(self.current_dc()) {
                    Ok(self.current_dc())
                } else {
                    Err(FailoverError::NoHealthyDc(RequestType::Auth))
                }
            }
            RequestType::Main => {
                // Main requests prefer current DC, but can failover
                if self.is_dc_healthy(self.current_dc()) {
                    Ok(self.current_dc())
                } else {
                    self.get_best_alternative_dc()
                }
            }
            RequestType::Download | RequestType::Upload => {
                // Can use any healthy DC
                self.get_best_alternative_dc()
            }
        }
    }

    /// Performs a failover to a backup DC.
    ///
    /// # Arguments
    ///
    /// * `from_dc` - The DC that failed
    ///
    /// # Returns
    ///
    /// The new DC ID
    pub async fn failover(&self, _from_dc: DcId) -> Result<DcId, FailoverError> {
        if !self.policy.auto_failover {
            return Err(FailoverError::Disabled);
        }

        let new_dc = self.get_best_alternative_dc()?;

        // Update failover count
        let mut count = self.failover_count.lock();
        *count += 1;

        if *count > self.policy.max_failovers {
            return Err(FailoverError::AllDcsFailed);
        }

        // Switch to new DC
        *self.current_dc.lock() = new_dc;

        Ok(new_dc)
    }

    /// Gets the best alternative DC (excluding current).
    fn get_best_alternative_dc(&self) -> Result<DcId, FailoverError> {
        let health = self.dc_health.lock();
        let available = self.available_dcs.lock();

        let mut best_dc = None;
        let mut best_score = -1i32;

        for dc in available.iter() {
            if let Some(metrics) = health.get(&dc.get_raw_id()) {
                if !metrics.is_available() {
                    continue;
                }

                // Score: healthy > degraded, lower latency is better
                let score = if metrics.health == DcHealth::Healthy {
                    1000
                } else {
                    500
                } - (metrics.avg_latency.as_millis() as i32).min(500);

                if score > best_score {
                    best_score = score;
                    best_dc = Some(*dc);
                }
            }
        }

        best_dc.ok_or(FailoverError::NoHealthyDc(RequestType::Main))
    }

    /// Checks if a DC is healthy.
    pub fn is_dc_healthy(&self, dc_id: DcId) -> bool {
        let health = self.dc_health.lock();
        health
            .get(&dc_id.get_raw_id())
            .map(|m| m.is_available())
            .unwrap_or(false)
    }

    /// Returns the health status of a DC.
    pub fn dc_health_status(&self, dc_id: DcId) -> Option<DcHealth> {
        let health = self.dc_health.lock();
        health.get(&dc_id.get_raw_id()).map(|m| m.health)
    }

    /// Returns the number of failovers that have occurred.
    pub fn failover_count(&self) -> usize {
        *self.failover_count.lock()
    }

    /// Resets the failover count.
    pub fn reset_failover_count(&self) {
        *self.failover_count.lock() = 0;
    }

    /// Checks for DCs that are ready to exit cooldown.
    pub fn check_cooldowns(&self) -> Vec<DcId> {
        let mut health = self.dc_health.lock();
        let mut ready = Vec::new();

        for (dc_id, metrics) in health.iter_mut() {
            if metrics.check_cooldown() {
                ready.push(DcId::internal(*dc_id));
            }
        }

        ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failover_policy_default() {
        let policy = FailoverPolicy::default();
        assert_eq!(policy.failure_threshold, 3);
        assert_eq!(policy.cooldown_duration, Duration::from_secs(60));
        assert_eq!(policy.slow_threshold, Duration::from_millis(500));
        assert!(policy.auto_failover);
        assert_eq!(policy.max_failovers, 3);
        assert!(policy.prefer_main_on_recovery);
    }

    #[test]
    fn test_dc_health_is_available() {
        assert!(DcHealth::Healthy.is_available());
        assert!(DcHealth::Degraded.is_available());
        assert!(!DcHealth::Unhealthy.is_available());

        // Cooldown DC is not available until cooldown ends
        assert!(!DcHealth::Cooldown.is_available());
    }

    #[test]
    fn test_dc_health_is_healthy() {
        assert!(DcHealth::Healthy.is_healthy());
        assert!(!DcHealth::Degraded.is_healthy());
        assert!(!DcHealth::Unhealthy.is_healthy());
        assert!(!DcHealth::Cooldown.is_healthy());
    }

    #[test]
    fn test_dc_metrics_new() {
        let metrics = DcMetrics::new();
        assert_eq!(metrics.health, DcHealth::Healthy);
        assert_eq!(metrics.success_count, 0);
        assert_eq!(metrics.failure_count, 0);
        assert_eq!(metrics.consecutive_failures, 0);
    }

    #[test]
    fn test_dc_metrics_record_success() {
        let config = FailoverPolicy::default();
        let mut metrics = DcMetrics::new();

        metrics.record_success(Duration::from_millis(50), &config);

        assert_eq!(metrics.success_count, 1);
        assert_eq!(metrics.consecutive_failures, 0);
        assert!(metrics.last_success.is_some());
        assert_eq!(metrics.health, DcHealth::Healthy);
    }

    #[test]
    fn test_dc_metrics_record_failure() {
        let config = FailoverPolicy::default();
        let mut metrics = DcMetrics::new();

        let error = QueryError::Generic("test error".into());

        // Record failures below threshold
        for _ in 0..2 {
            metrics.record_failure(&error, &config);
        }
        assert_eq!(metrics.consecutive_failures, 2);
        assert_eq!(metrics.health, DcHealth::Healthy);

        // Cross threshold
        metrics.record_failure(&error, &config);
        assert_eq!(metrics.consecutive_failures, 3);
        assert_eq!(metrics.health, DcHealth::Unhealthy);
        assert!(metrics.cooldown_until.is_some());
    }

    #[test]
    fn test_failover_manager_new() {
        let main_dc = DcId::internal(2);
        let manager = FailoverManager::new(main_dc);

        assert_eq!(manager.current_dc(), main_dc);
        assert!(manager.is_dc_healthy(main_dc));
        assert_eq!(manager.failover_count(), 0);
    }

    #[test]
    fn test_failover_manager_add_dc() {
        let main_dc = DcId::internal(2);
        let backup_dc = DcId::internal(4);
        let manager = FailoverManager::new(main_dc);

        manager.add_dc(backup_dc);

        assert!(manager.is_dc_healthy(backup_dc));
    }

    #[test]
    fn test_failover_manager_report_success() {
        let main_dc = DcId::internal(2);
        let manager = FailoverManager::new(main_dc);

        manager.report_success(main_dc, Duration::from_millis(100));

        assert!(manager.is_dc_healthy(main_dc));
    }

    #[test]
    fn test_failover_manager_report_failure() {
        let main_dc = DcId::internal(2);
        let manager = FailoverManager::new(main_dc);

        let error = QueryError::Generic("test error".into());

        // Below threshold
        for _ in 0..2 {
            manager.report_failure(main_dc, &error);
        }
        assert!(manager.is_dc_healthy(main_dc));

        // Cross threshold
        manager.report_failure(main_dc, &error);
        assert!(!manager.is_dc_healthy(main_dc));
    }

    #[test]
    fn test_failover_error_display() {
        let err = FailoverError::NoHealthyDc(RequestType::Main);
        assert!(err.to_string().contains("No healthy DCs available"));
        assert!(err.to_string().contains("Main"));

        let err = FailoverError::AllDcsFailed;
        assert_eq!(err.to_string(), "All DCs failed");

        let dc = DcId::internal(2);
        let err = FailoverError::InvalidDcId(dc);
        assert!(err.to_string().contains("Invalid DC ID"));

        let err = FailoverError::Disabled;
        assert_eq!(err.to_string(), "Failover is disabled");
    }

    #[test]
    fn test_request_type_equality() {
        assert_eq!(RequestType::Main, RequestType::Main);
        assert_ne!(RequestType::Main, RequestType::Download);
        assert_ne!(RequestType::Download, RequestType::Upload);
        assert_ne!(RequestType::Upload, RequestType::Auth);
    }
}
