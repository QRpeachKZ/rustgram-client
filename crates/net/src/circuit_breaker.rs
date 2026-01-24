// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Circuit breaker for connection failure management.
//!
//! This module implements the circuit breaker pattern to prevent cascading
//! failures when a DC endpoint becomes unavailable.
//!
//! # Circuit States
//!
//! - **Closed**: Normal operation, requests pass through
//! - **Open**: Failures exceeded threshold, requests fail immediately
//! - **HalfOpen**: Testing if the endpoint has recovered

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::time::Duration;

use crate::dc::DcId;

/// Circuit breaker configuration.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Maximum consecutive failures before opening circuit.
    /// Default: 5
    pub failure_threshold: u32,

    /// Timeout before attempting recovery (half-open state).
    /// Default: 60 seconds
    pub open_timeout: Duration,

    /// Number of successful requests needed to close circuit in half-open state.
    /// Default: 2
    pub success_threshold: u32,

    /// Time window to track failure rate (optional).
    /// Default: None (consecutive failures only)
    pub rolling_window: Option<Duration>,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            open_timeout: Duration::from_secs(60),
            success_threshold: 2,
            rolling_window: None,
        }
    }
}

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, normal operation
    Closed,

    /// Circuit is open, failing fast
    Open,

    /// Circuit is half-open, testing recovery
    HalfOpen,
}

/// Per-endpoint circuit breaker.
#[derive(Debug)]
pub struct CircuitBreaker {
    /// DC identifier
    dc_id: DcId,

    /// Current state
    state: AtomicU8,

    /// Consecutive failure count
    failure_count: AtomicU32,

    /// Consecutive success count (for half-open)
    success_count: AtomicU32,

    /// Last failure time
    last_failure_time: AtomicU64,

    /// Last state change time
    last_state_change: AtomicU64,

    /// Configuration
    config: CircuitBreakerConfig,

    /// Whether this breaker is enabled
    enabled: AtomicBool,
}

impl CircuitBreaker {
    /// Creates a new circuit breaker.
    pub fn new(dc_id: DcId, config: CircuitBreakerConfig) -> Self {
        Self {
            dc_id,
            state: AtomicU8::new(CircuitState::Closed as u8),
            failure_count: AtomicU32::new(0),
            success_count: AtomicU32::new(0),
            last_failure_time: AtomicU64::new(0),
            last_state_change: AtomicU64::new(0),
            config,
            enabled: AtomicBool::new(true),
        }
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns the current circuit state.
    pub fn state(&self) -> CircuitState {
        match self.state.load(Ordering::Relaxed) {
            0 => CircuitState::Closed,
            1 => CircuitState::Open,
            2 => CircuitState::HalfOpen,
            _ => CircuitState::Closed,
        }
    }

    /// Sets the circuit state.
    fn set_state(&self, state: CircuitState) {
        self.state
            .store(state as u8, Ordering::Release);
        self.last_state_change
            .store(Self::now_secs(), Ordering::Release);

        tracing::info!(
            "Circuit breaker for DC {:?} changed to {:?}",
            self.dc_id,
            state
        );
    }

    /// Returns whether the circuit is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Enables or disables the circuit breaker.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    /// Records a successful connection attempt.
    pub fn record_success(&self) {
        let state = self.state();

        match state {
            CircuitState::Closed => {
                // Reset failure count on success
                self.failure_count.store(0, Ordering::Relaxed);
            }
            CircuitState::HalfOpen => {
                // Increment success count, check if we can close
                let successes = self.success_count.fetch_add(1, Ordering::Relaxed) + 1;

                if successes >= self.config.success_threshold {
                    self.set_state(CircuitState::Closed);
                    self.success_count.store(0, Ordering::Relaxed);
                    self.failure_count.store(0, Ordering::Relaxed);

                    tracing::info!(
                        "Circuit breaker for DC {:?} closed after {} successful probes",
                        self.dc_id,
                        successes
                    );
                }
            }
            CircuitState::Open => {
                // Shouldn't happen, but reset if it does
                tracing::warn!(
                    "Recorded success while circuit for DC {:?} was open",
                    self.dc_id
                );
            }
        }
    }

    /// Records a failed connection attempt.
    pub fn record_failure(&self) {
        self.last_failure_time
            .store(Self::now_secs(), Ordering::Relaxed);

        let state = self.state();

        match state {
            CircuitState::Closed => {
                let failures = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;

                tracing::debug!(
                    "Circuit breaker for DC {:?} recorded failure {}/{}",
                    self.dc_id,
                    failures,
                    self.config.failure_threshold
                );

                if failures >= self.config.failure_threshold {
                    self.set_state(CircuitState::Open);

                    tracing::warn!(
                        "Circuit breaker for DC {:?} opened after {} consecutive failures",
                        self.dc_id,
                        failures
                    );
                }
            }
            CircuitState::HalfOpen => {
                // Failure in half-open, reopen immediately
                self.set_state(CircuitState::Open);
                self.success_count.store(0, Ordering::Relaxed);

                tracing::warn!(
                    "Circuit breaker for DC {:?} reopened after failure in half-open",
                    self.dc_id
                );
            }
            CircuitState::Open => {
                // Already open, just update failure time
            }
        }
    }

    /// Checks if a request should be allowed through.
    pub fn allow_request(&self) -> bool {
        if !self.is_enabled() {
            return true;
        }

        let state = self.state();

        match state {
            CircuitState::Closed => true,
            CircuitState::HalfOpen => true,
            CircuitState::Open => {
                // Check if timeout has elapsed
                let last_change = self.last_state_change.load(Ordering::Relaxed);
                let elapsed = Self::now_secs().saturating_sub(last_change);

                if elapsed >= self.config.open_timeout.as_secs() as u64 {
                    // Transition to half-open
                    self.set_state(CircuitState::HalfOpen);
                    self.success_count.store(0, Ordering::Relaxed);

                    tracing::info!(
                        "Circuit breaker for DC {:?} entering half-open after {}s",
                        self.dc_id,
                        elapsed
                    );

                    true
                } else {
                    false
                }
            }
        }
    }

    /// Returns the number of consecutive failures.
    pub fn failure_count(&self) -> u32 {
        self.failure_count.load(Ordering::Relaxed)
    }

    /// Returns the time since last failure.
    pub fn time_since_last_failure(&self) -> Option<Duration> {
        let last_fail = self.last_failure_time.load(Ordering::Relaxed);
        if last_fail == 0 {
            return None;
        }

        let now = Self::now_secs();
        Some(Duration::from_secs(now.saturating_sub(last_fail)))
    }

    /// Returns the time since last state change.
    pub fn time_since_state_change(&self) -> Duration {
        let last_change = self.last_state_change.load(Ordering::Relaxed);
        let now = Self::now_secs();
        Duration::from_secs(now.saturating_sub(last_change))
    }

    /// Resets the circuit breaker to closed state.
    pub fn reset(&self) {
        self.set_state(CircuitState::Closed);
        self.failure_count.store(0, Ordering::Relaxed);
        self.success_count.store(0, Ordering::Relaxed);

        tracing::info!("Circuit breaker for DC {:?} manually reset", self.dc_id);
    }

    /// Returns current time as seconds since epoch.
    fn now_secs() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_state_values() {
        assert_eq!(CircuitState::Closed as u8, 0);
        assert_eq!(CircuitState::Open as u8, 1);
        assert_eq!(CircuitState::HalfOpen as u8, 2);
    }

    #[test]
    fn test_circuit_breaker_new() {
        let dc_id = DcId::internal(2);
        let config = CircuitBreakerConfig::default();
        let breaker = CircuitBreaker::new(dc_id, config);

        assert_eq!(breaker.dc_id(), dc_id);
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert_eq!(breaker.failure_count(), 0);
        assert!(breaker.is_enabled());
    }

    #[test]
    fn test_circuit_breaker_record_success_closed() {
        let dc_id = DcId::internal(2);
        let config = CircuitBreakerConfig::default();
        let breaker = CircuitBreaker::new(dc_id, config);

        // Record some failures
        breaker.failure_count.store(3, Ordering::Relaxed);

        // Success should reset
        breaker.record_success();
        assert_eq!(breaker.failure_count(), 0);
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_open_after_threshold() {
        let dc_id = DcId::internal(2);
        let mut config = CircuitBreakerConfig::default();
        config.failure_threshold = 3;
        let breaker = CircuitBreaker::new(dc_id, config);

        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.allow_request());

        // Record failures up to threshold
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);

        // Should not allow requests
        assert!(!breaker.allow_request());
    }

    #[test]
    fn test_circuit_breaker_half_open_to_closed() {
        let dc_id = DcId::internal(2);
        let mut config = CircuitBreakerConfig::default();
        config.failure_threshold = 2;
        config.success_threshold = 2;
        let breaker = CircuitBreaker::new(dc_id, config);

        // Open the circuit
        breaker.record_failure();
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);

        // Simulate timeout (manually set state)
        breaker.set_state(CircuitState::HalfOpen);
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        // Record successes
        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_half_open_failure() {
        let dc_id = DcId::internal(2);
        let mut config = CircuitBreakerConfig::default();
        config.failure_threshold = 2;
        let breaker = CircuitBreaker::new(dc_id, config);

        // Open the circuit
        breaker.record_failure();
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);

        // Move to half-open
        breaker.set_state(CircuitState::HalfOpen);

        // Record failure in half-open
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
    }

    #[test]
    fn test_circuit_breaker_reset() {
        let dc_id = DcId::internal(2);
        let config = CircuitBreakerConfig::default();
        let breaker = CircuitBreaker::new(dc_id, config);

        // Record failures
        breaker.record_failure();
        breaker.record_failure();

        // Reset
        breaker.reset();
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_disabled() {
        let dc_id = DcId::internal(2);
        let config = CircuitBreakerConfig::default();
        let breaker = CircuitBreaker::new(dc_id, config);

        breaker.set_enabled(false);
        assert!(!breaker.is_enabled());

        // Even with failures, should always allow
        breaker.record_failure();
        breaker.record_failure();
        assert!(breaker.allow_request());
    }
}
