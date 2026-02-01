// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Connection pool for MTProto sessions.
//!
//! This module implements a connection pool that manages multiple
//! SessionConnection instances for different DCs and purposes.
//!
//! # Architecture
//!
//! The pool maintains separate sub-pools for:
//! - Main DC connections (for user data)
//! - Download DC connections (for media downloads)
//! - Upload DC connections (for media uploads)
//!
//! Each sub-pool maintains:
//! - Active connections (currently in use)
//! - Idle connections (available for reuse)
//! - Pending acquisitions (waiting for available connections)
//!
//! # References
//!
//! - TDLib: `td/telegram/net/ConnectionCreator.cpp`
//! - TDLib: `td/telegram/net/DcConnection.h`

use parking_lot::Mutex;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::dc::{DcId, DcOptionsSet};
use crate::health_check::{HealthChecker, HealthCheckConfig, HealthStatus};
use crate::session::{SessionConnection, SessionConnectionConfig};

/// Connection purpose determines which sub-pool to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionPurpose {
    /// Main connection for user data
    Main,

    /// Connection for downloading media
    Download,

    /// Connection for uploading media
    Upload,
}

/// Error types for connection pool operations.
#[derive(Debug, Error, Clone)]
pub enum PoolError {
    /// No connection available
    #[error("No connection available for DC {dc_id:?}")]
    NoConnection {
        /// The DC ID
        dc_id: DcId
    },

    /// Pool is closed
    #[error("Connection pool is closed")]
    Closed,

    /// Connection acquisition timeout
    #[error("Connection acquisition timeout after {timeout:?}")]
    Timeout {
        /// The timeout duration
        timeout: Duration
    },

    /// Invalid DC ID
    #[error("Invalid DC ID: {0:?}")]
    InvalidDcId(DcId),

    /// Connection failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}

/// Configuration for the connection pool.
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of connections per DC per purpose.
    /// Default: 8
    pub max_connections_per_dc: usize,

    /// Maximum number of idle connections to keep.
    /// Default: 2
    pub max_idle_connections: usize,

    /// Connection TTL before idle connections are closed.
    /// Default: 5 minutes
    pub connection_ttl: Duration,

    /// Maximum number of pending acquisitions.
    /// Default: 100
    pub max_pending_acquires: usize,

    /// Timeout for acquiring a connection.
    /// Default: 30 seconds
    pub acquire_timeout: Duration,

    /// Cleanup interval for idle connections.
    /// Default: 1 minute
    pub cleanup_interval: Duration,

    /// Maximum retry attempts for connection acquisition.
    /// Default: 5
    pub max_retry_attempts: u32,

    /// Base delay for exponential backoff (first retry).
    /// Default: 100ms
    pub retry_base_delay: Duration,

    /// Maximum delay for exponential backoff.
    /// Default: 10 seconds
    pub retry_max_delay: Duration,

    /// Whether to add jitter to retry delays.
    /// Default: true
    pub retry_with_jitter: bool,

    /// Whether to enable circuit breaker.
    /// Default: true
    pub circuit_breaker_enabled: bool,

    /// Circuit breaker configuration.
    pub circuit_breaker_config: CircuitBreakerConfig,

    /// Health check configuration.
    pub health_check_config: HealthCheckConfig,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections_per_dc: 8,
            max_idle_connections: 2,
            connection_ttl: Duration::from_secs(300), // 5 minutes
            max_pending_acquires: 100,
            acquire_timeout: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            max_retry_attempts: 5,
            retry_base_delay: Duration::from_millis(100),
            retry_max_delay: Duration::from_secs(10),
            retry_with_jitter: true,
            circuit_breaker_enabled: true,
            circuit_breaker_config: CircuitBreakerConfig::default(),
            health_check_config: HealthCheckConfig::default(),
        }
    }
}

/// A pooled connection wrapper.
///
/// When dropped, the connection is returned to the pool
/// unless explicitly marked as invalid.
pub struct PooledConnection {
    /// The underlying session connection
    connection: Arc<SessionConnection>,

    /// The DC this connection is for
    dc_id: DcId,

    /// The purpose of this connection
    purpose: ConnectionPurpose,

    /// Whether this connection should be returned to the pool
    valid: bool,

    /// Reference to the pool for returning the connection
    pool: Option<Arc<ConnectionPoolInner>>,
}

impl PooledConnection {
    /// Creates a new pooled connection wrapper.
    fn new(
        connection: Arc<SessionConnection>,
        dc_id: DcId,
        purpose: ConnectionPurpose,
        pool: Arc<ConnectionPoolInner>,
    ) -> Self {
        Self {
            connection,
            dc_id,
            purpose,
            valid: true,
            pool: Some(pool),
        }
    }

    /// Returns a reference to the underlying connection.
    pub fn connection(&self) -> &SessionConnection {
        &self.connection
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns the connection purpose.
    pub fn purpose(&self) -> ConnectionPurpose {
        self.purpose
    }

    /// Marks this connection as invalid, preventing it from being returned to the pool.
    pub fn invalidate(&mut self) {
        self.valid = false;
    }

    /// Explicitly returns the connection to the pool early.
    pub fn release(mut self) {
        self.valid = false;
        // Drop will handle the rest
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.take() {
            if self.valid {
                pool.return_connection(self.dc_id, self.purpose, self.connection.clone());
            } else {
                pool.remove_connection(self.dc_id, self.purpose, &self.connection);
            }
        }
    }
}

/// Entry for a connection in the pool.
#[derive(Debug)]
struct PoolEntry {
    /// The connection
    connection: Arc<SessionConnection>,

    /// When this connection was last used
    last_used: Instant,

    /// Whether this connection is currently in use
    in_use: bool,
}

/// Inner state of the connection pool.
#[derive(Debug)]
struct ConnectionPoolInner {
    /// Main DC connections: (dc_id -> [entries])
    main_pools: Mutex<HashMap<DcId, Vec<PoolEntry>>>,

    /// Download connections: (dc_id -> [entries])
    download_pools: Mutex<HashMap<DcId, Vec<PoolEntry>>>,

    /// Upload connections: (dc_id -> [entries])
    upload_pools: Mutex<HashMap<DcId, Vec<PoolEntry>>>,

    /// Pool configuration
    config: PoolConfig,

    /// Whether the pool is closed
    closed: Mutex<bool>,

    /// Number of pending acquisitions
    pending_count: Mutex<usize>,

    /// DC options for all data centers
    dc_options: Mutex<DcOptionsSet>,

    /// Circuit breakers for each DC
    circuit_breakers: Mutex<HashMap<DcId, Arc<CircuitBreaker>>>,

    /// Health checker for connection validation
    health_checker: HealthChecker,
}

impl ConnectionPoolInner {
    /// Returns the pool for a specific purpose.
    fn get_pool(&self, purpose: ConnectionPurpose) -> &Mutex<HashMap<DcId, Vec<PoolEntry>>> {
        match purpose {
            ConnectionPurpose::Main => &self.main_pools,
            ConnectionPurpose::Download => &self.download_pools,
            ConnectionPurpose::Upload => &self.upload_pools,
        }
    }

    /// Returns a connection to the pool.
    fn return_connection(&self, dc_id: DcId, purpose: ConnectionPurpose, connection: Arc<SessionConnection>) {
        let pools = self.get_pool(purpose);
        let mut pools = pools.lock();

        if *self.closed.lock() {
            return;
        }

        let pool = pools.entry(dc_id).or_default();

        // Find existing entry or create new one
        if let Some(entry) = pool.iter_mut().find(|e| Arc::ptr_eq(&e.connection, &connection)) {
            entry.in_use = false;
            entry.last_used = Instant::now();
        } else {
            pool.push(PoolEntry {
                connection,
                last_used: Instant::now(),
                in_use: false,
            });
        }

        // Clean up excess idle connections
        self.cleanup_pool_locked(pool);
    }

    /// Removes a connection from the pool.
    fn remove_connection(&self, dc_id: DcId, purpose: ConnectionPurpose, connection: &Arc<SessionConnection>) {
        let pools = self.get_pool(purpose);
        let mut pools = pools.lock();

        if let Some(pool) = pools.get_mut(&dc_id) {
            pool.retain(|e| !Arc::ptr_eq(&e.connection, connection));
        }
    }

    /// Cleans up idle connections in a pool (must hold lock).
    fn cleanup_pool_locked(&self, pool: &mut Vec<PoolEntry>) {
        let now = Instant::now();

        // Remove connections that are:
        // 1. Not in use
        // 2. Past TTL
        // 3. In excess of max_idle_connections

        // First, separate idle and in-use connections
        let mut idle: Vec<PoolEntry> = pool
            .drain(..)
            .filter(|e| !e.in_use)
            .collect();

        // Sort by last_used (oldest first)
        idle.sort_by_key(|e| e.last_used);

        // Remove old connections
        idle.retain(|e| now.duration_since(e.last_used) < self.config.connection_ttl);

        // Keep only max_idle_connections
        if idle.len() > self.config.max_idle_connections {
            idle.truncate(self.config.max_idle_connections);
        }

        // Recreate pool with in-use connections + remaining idle
        *pool = idle;
    }

    /// Cleans up all idle connections.
    pub fn cleanup_idle(&self) {
        for purpose in [ConnectionPurpose::Main, ConnectionPurpose::Download, ConnectionPurpose::Upload] {
            let pools = self.get_pool(purpose);
            let mut pools = pools.lock();

            for pool in pools.values_mut() {
                self.cleanup_pool_locked(pool);
            }
        }
    }

    /// Closes all connections in the pool.
    pub fn close_all(&self) {
        *self.closed.lock() = true;

        for purpose in [ConnectionPurpose::Main, ConnectionPurpose::Download, ConnectionPurpose::Upload] {
            let pools = self.get_pool(purpose);
            let mut pools = pools.lock();
            pools.clear();
        }
    }
}

/// Connection pool for MTProto sessions.
///
/// The pool manages connections to different DCs for different purposes
/// (main, download, upload), automatically reusing idle connections and
/// creating new ones when needed.
#[derive(Debug, Clone)]
pub struct ConnectionPool {
    /// Inner pool state
    inner: Arc<ConnectionPoolInner>,
}

impl ConnectionPool {
    /// Creates a new connection pool with default configuration.
    pub fn new() -> Self {
        Self::with_config(PoolConfig::default())
    }

    /// Creates a new connection pool with the given configuration.
    pub fn with_config(config: PoolConfig) -> Self {
        let health_checker = HealthChecker::new(config.health_check_config.clone());

        Self {
            inner: Arc::new(ConnectionPoolInner {
                main_pools: Mutex::new(HashMap::new()),
                download_pools: Mutex::new(HashMap::new()),
                upload_pools: Mutex::new(HashMap::new()),
                config,
                closed: Mutex::new(false),
                pending_count: Mutex::new(0),
                dc_options: Mutex::new(DcOptionsSet::new()),
                circuit_breakers: Mutex::new(HashMap::new()),
                health_checker,
            }),
        }
    }

    /// Returns the pool configuration.
    pub fn config(&self) -> &PoolConfig {
        &self.inner.config
    }

    /// Sets the DC options for this connection pool.
    ///
    /// DC options are passed to new connections when they are created.
    /// This allows connections to find the correct IP addresses and ports
    /// for their configured data centers.
    pub fn set_dc_options(&self, options: DcOptionsSet) {
        *self.inner.dc_options.lock() = options;
    }

    /// Acquires a connection for the given DC and purpose.
    ///
    /// If an idle connection is available, it will be reused.
    /// Otherwise, a new connection will be created.
    ///
    /// This method implements:
    /// - Circuit breaker pattern to prevent cascading failures
    /// - Retry logic with exponential backoff
    /// - Health checks for idle connections
    ///
    /// # Arguments
    ///
    /// * `dc_id` - The DC to connect to
    /// * `purpose` - The purpose of the connection
    /// * `config` - Configuration for creating new connections
    ///
    /// # Returns
    ///
    /// A pooled connection
    pub async fn acquire(
        &self,
        dc_id: DcId,
        purpose: ConnectionPurpose,
        config: SessionConnectionConfig,
    ) -> Result<PooledConnection, PoolError> {
        if *self.inner.closed.lock() {
            return Err(PoolError::Closed);
        }

        // Check circuit breaker
        if self.inner.config.circuit_breaker_enabled {
            let breaker = self.get_or_create_circuit_breaker(dc_id);
            if !breaker.allow_request() {
                tracing::warn!(
                    "Circuit breaker OPEN for DC {:?}, rejecting connection request",
                    dc_id
                );
                return Err(PoolError::ConnectionFailed(format!(
                    "Circuit breaker open for DC {:?}",
                    dc_id
                )));
            }
        }

        // Check pending count
        {
            let mut pending = self.inner.pending_count.lock();
            if *pending >= self.inner.config.max_pending_acquires {
                return Err(PoolError::Closed);
            }
            *pending += 1;
        }

        // Try to get an idle connection with health check
        let connection = self.try_acquire_idle_with_health_check(dc_id, purpose);

        // Decrement pending count
        {
            let mut pending = self.inner.pending_count.lock();
            *pending = pending.saturating_sub(1);
        }

        match connection {
            Some(conn) => {
                // Record success on circuit breaker
                if self.inner.config.circuit_breaker_enabled {
                    if let Some(breaker) = self.get_circuit_breaker(dc_id) {
                        breaker.record_success();
                    }
                }
                Ok(conn)
            }
            None => {
                // Create new connection with retry logic
                self.create_connection_with_retry(dc_id, purpose, config).await
            }
        }
    }

    /// Tries to acquire an idle connection from the pool.
    fn try_acquire_idle(
        &self,
        dc_id: DcId,
        purpose: ConnectionPurpose,
    ) -> Option<PooledConnection> {
        let pools = self.inner.get_pool(purpose);
        let mut pools = pools.lock();

        if let Some(pool) = pools.get_mut(&dc_id) {
            // Find an idle, ready connection
            if let Some(entry) = pool.iter_mut().find(|e| !e.in_use && e.connection.is_ready()) {
                entry.in_use = true;
                entry.last_used = Instant::now();

                let connection = entry.connection.clone();
                return Some(PooledConnection::new(
                    connection,
                    dc_id,
                    purpose,
                    self.inner.clone(),
                ));
            }
        }

        None
    }

    /// Creates a new connection.
    async fn create_connection(
        &self,
        dc_id: DcId,
        purpose: ConnectionPurpose,
        config: SessionConnectionConfig,
    ) -> Result<PooledConnection, PoolError> {
        use crate::auth::AuthDataShared;

        // Check connection count limit
        {
            let pools = self.inner.get_pool(purpose);
            let pools = pools.lock();

            if let Some(pool) = pools.get(&dc_id) {
                let active_count = pool.iter().filter(|e| e.in_use).count();
                if active_count >= self.inner.config.max_connections_per_dc {
                    return Err(PoolError::NoConnection { dc_id });
                }
            }
        }

        // Create new session connection
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let connection = Arc::new(SessionConnection::new(config, auth_data));

        // Set DC options on the connection
        let dc_options = self.inner.dc_options.lock().clone();
        connection.set_dc_options(dc_options);

        // Start the connection
        connection
            .start()
            .await
            .map_err(|e| PoolError::ConnectionFailed(e.to_string()))?;

        // Add to pool
        {
            let pools = self.inner.get_pool(purpose);
            let mut pools = pools.lock();

            if !*self.inner.closed.lock() {
                let pool = pools.entry(dc_id).or_default();
                pool.push(PoolEntry {
                    connection: connection.clone(),
                    last_used: Instant::now(),
                    in_use: true,
                });
            }
        }

        Ok(PooledConnection::new(
            connection,
            dc_id,
            purpose,
            self.inner.clone(),
        ))
    }

    /// Gets or creates a circuit breaker for the given DC.
    fn get_or_create_circuit_breaker(&self, dc_id: DcId) -> Arc<CircuitBreaker> {
        let mut breakers = self.inner.circuit_breakers.lock();

        breakers
            .entry(dc_id)
            .or_insert_with(|| {
                Arc::new(CircuitBreaker::new(
                    dc_id,
                    self.inner.config.circuit_breaker_config.clone(),
                ))
            })
            .clone()
    }

    /// Gets an existing circuit breaker for the given DC.
    fn get_circuit_breaker(&self, dc_id: DcId) -> Option<Arc<CircuitBreaker>> {
        let breakers = self.inner.circuit_breakers.lock();
        breakers.get(&dc_id).cloned()
    }

    /// Tries to acquire an idle connection with health check.
    fn try_acquire_idle_with_health_check(
        &self,
        dc_id: DcId,
        purpose: ConnectionPurpose,
    ) -> Option<PooledConnection> {
        let pools = self.inner.get_pool(purpose);
        let mut pools = pools.lock();

        if let Some(pool) = pools.get_mut(&dc_id) {
            // Find an unhealthy connection to remove first
            let unhealthy_idx = pool.iter().position(|e| {
                !e.in_use && {
                    let health = HealthChecker::check_connection_quick(&e.connection);
                    health != HealthStatus::Healthy || !e.connection.is_ready()
                }
            });

            if let Some(idx) = unhealthy_idx {
                tracing::debug!(
                    "Removing unhealthy idle connection for DC {:?}, purpose {:?}",
                    dc_id,
                    purpose
                );
                pool.remove(idx);
            }

            // Now find a healthy idle connection
            if let Some(entry) = pool.iter_mut().find(|e| !e.in_use) {
                let health = HealthChecker::check_connection_quick(&entry.connection);

                if health == HealthStatus::Healthy && entry.connection.is_ready() {
                    entry.in_use = true;
                    entry.last_used = Instant::now();

                    let connection = entry.connection.clone();
                    tracing::debug!(
                        "Reusing idle connection for DC {:?}, purpose {:?}",
                        dc_id,
                        purpose
                    );

                    return Some(PooledConnection::new(
                        connection,
                        dc_id,
                        purpose,
                        self.inner.clone(),
                    ));
                }
            }
        }

        None
    }

    /// Creates a new connection with retry logic.
    async fn create_connection_with_retry(
        &self,
        dc_id: DcId,
        purpose: ConnectionPurpose,
        config: SessionConnectionConfig,
    ) -> Result<PooledConnection, PoolError> {
        let max_attempts = self.inner.config.max_retry_attempts;
        let base_delay = self.inner.config.retry_base_delay;
        let max_delay = self.inner.config.retry_max_delay;
        let use_jitter = self.inner.config.retry_with_jitter;

        let mut last_error = None;

        for attempt in 0..max_attempts {
            // Log attempt
            if attempt > 0 {
                tracing::info!(
                    "Retrying connection to DC {:?}, attempt {}/{}",
                    dc_id,
                    attempt + 1,
                    max_attempts
                );
            }

            // Try to create connection
            match self.create_connection(dc_id, purpose, config.clone()).await {
                Ok(conn) => {
                    // Success! Record success on circuit breaker
                    if self.inner.config.circuit_breaker_enabled {
                        if let Some(breaker) = self.get_circuit_breaker(dc_id) {
                            breaker.record_success();
                        }
                    }

                    tracing::info!(
                        "Successfully connected to DC {:?} after {} attempt(s)",
                        dc_id,
                        attempt + 1
                    );

                    return Ok(conn);
                }
                Err(e) => {
                    last_error = Some(e.clone());

                    // Record failure on circuit breaker
                    if self.inner.config.circuit_breaker_enabled {
                        if let Some(breaker) = self.get_circuit_breaker(dc_id) {
                            breaker.record_failure();
                        }
                    }

                    tracing::warn!(
                        "Failed to connect to DC {:?} on attempt {}/{}: {}",
                        dc_id,
                        attempt + 1,
                        max_attempts,
                        e
                    );

                    // Don't sleep after the last attempt
                    if attempt < max_attempts - 1 {
                        // Calculate delay with exponential backoff
                        let delay_ms = base_delay.as_millis() * 2u128.pow(attempt as u32);
                        let delay = Duration::from_millis(
                            delay_ms.min(max_delay.as_millis()) as u64,
                        );

                        // Add jitter if enabled
                        let actual_delay = if use_jitter {
                            let jitter = rand::thread_rng().gen_range(0..=delay.as_millis() / 4);
                            Duration::from_millis((delay.as_millis() + jitter) as u64)
                        } else {
                            delay
                        };

                        tracing::debug!(
                            "Waiting {:?} before retry (attempt {}/{})",
                            actual_delay,
                            attempt + 1,
                            max_attempts
                        );

                        tokio::time::sleep(actual_delay).await;
                    }
                }
            }
        }

        // All attempts failed
        let error_msg = format!(
            "Failed to connect to DC {:?} after {} attempts: {:?}",
            dc_id, max_attempts, last_error
        );

        tracing::error!("{}", error_msg);

        Err(PoolError::ConnectionFailed(error_msg))
    }

    /// Cleans up idle connections.
    pub async fn cleanup_idle(&self) {
        self.inner.cleanup_idle();
    }

    /// Closes all connections in the pool.
    pub async fn close_all(&self) {
        self.inner.close_all();
    }

    /// Returns the number of active connections for a DC and purpose.
    pub fn active_count(&self, dc_id: DcId, purpose: ConnectionPurpose) -> usize {
        let pools = self.inner.get_pool(purpose);
        let pools = pools.lock();

        pools
            .get(&dc_id)
            .map(|pool| pool.iter().filter(|e| e.in_use).count())
            .unwrap_or(0)
    }

    /// Returns the number of idle connections for a DC and purpose.
    pub fn idle_count(&self, dc_id: DcId, purpose: ConnectionPurpose) -> usize {
        let pools = self.inner.get_pool(purpose);
        let pools = pools.lock();

        pools
            .get(&dc_id)
            .map(|pool| pool.iter().filter(|e| !e.in_use).count())
            .unwrap_or(0)
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.max_connections_per_dc, 8);
        assert_eq!(config.max_idle_connections, 2);
        assert_eq!(config.connection_ttl, Duration::from_secs(300));
        assert_eq!(config.max_pending_acquires, 100);
        assert_eq!(config.acquire_timeout, Duration::from_secs(30));
        assert_eq!(config.cleanup_interval, Duration::from_secs(60));
        assert_eq!(config.max_retry_attempts, 5);
        assert_eq!(config.retry_base_delay, Duration::from_millis(100));
        assert_eq!(config.retry_max_delay, Duration::from_secs(10));
        assert!(config.retry_with_jitter);
        assert!(config.circuit_breaker_enabled);
    }

    #[test]
    fn test_connection_purpose_equality() {
        assert_eq!(ConnectionPurpose::Main, ConnectionPurpose::Main);
        assert_ne!(ConnectionPurpose::Main, ConnectionPurpose::Download);
        assert_ne!(ConnectionPurpose::Download, ConnectionPurpose::Upload);
    }

    #[test]
    fn test_connection_pool_new() {
        let pool = ConnectionPool::new();
        assert_eq!(pool.config().max_connections_per_dc, 8);
    }

    #[test]
    fn test_connection_pool_with_config() {
        let mut config = PoolConfig::default();
        config.max_connections_per_dc = 16;
        config.max_idle_connections = 4;

        let pool = ConnectionPool::with_config(config.clone());
        assert_eq!(pool.config().max_connections_per_dc, 16);
        assert_eq!(pool.config().max_idle_connections, 4);
    }

    #[test]
    fn test_pool_error_display() {
        let dc_id = DcId::internal(2);
        let err = PoolError::NoConnection { dc_id };
        assert!(err.to_string().contains("No connection available"));

        let err = PoolError::Closed;
        assert_eq!(err.to_string(), "Connection pool is closed");

        let timeout = Duration::from_secs(5);
        let err = PoolError::Timeout { timeout };
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn test_pooled_connection_invalidate() {
        // This test verifies that invalidate() marks the connection as invalid
        // but doesn't actually test the pool behavior (would need async)
        let dc_id = DcId::internal(2);
        let purpose = ConnectionPurpose::Main;

        // Create a mock connection for testing
        let config = SessionConnectionConfig::new(dc_id);
        let auth_data = Arc::new(crate::auth::AuthDataShared::new(dc_id));
        let connection = Arc::new(SessionConnection::new(config, auth_data));

        // Create a mock pool
        let pool = Arc::new(ConnectionPoolInner {
            main_pools: Mutex::new(HashMap::new()),
            download_pools: Mutex::new(HashMap::new()),
            upload_pools: Mutex::new(HashMap::new()),
            config: PoolConfig::default(),
            closed: Mutex::new(false),
            pending_count: Mutex::new(0),
            dc_options: Mutex::new(DcOptionsSet::new()),
            circuit_breakers: Mutex::new(HashMap::new()),
            health_checker: HealthChecker::default(),
        });

        let mut pooled = PooledConnection::new(connection, dc_id, purpose, pool);
        assert!(pooled.valid);

        pooled.invalidate();
        assert!(!pooled.valid);
    }

    #[test]
    fn test_pool_entry_creation() {
        let dc_id = DcId::internal(2);
        let config = SessionConnectionConfig::new(dc_id);
        let auth_data = Arc::new(crate::auth::AuthDataShared::new(dc_id));
        let connection = Arc::new(SessionConnection::new(config, auth_data));

        let entry = PoolEntry {
            connection: connection.clone(),
            last_used: Instant::now(),
            in_use: false,
        };

        assert!(!entry.in_use);
        assert!(Arc::ptr_eq(&entry.connection, &connection));
    }

    #[tokio::test]
    async fn test_pool_active_and_idle_count() {
        let pool = ConnectionPool::new();
        let dc_id = DcId::internal(2);

        // Initially, no connections
        assert_eq!(pool.active_count(dc_id, ConnectionPurpose::Main), 0);
        assert_eq!(pool.idle_count(dc_id, ConnectionPurpose::Main), 0);

        // After cleanup, still no connections
        pool.cleanup_idle().await;
        assert_eq!(pool.active_count(dc_id, ConnectionPurpose::Main), 0);
        assert_eq!(pool.idle_count(dc_id, ConnectionPurpose::Main), 0);
    }

    #[tokio::test]
    async fn test_pool_close_all() {
        let pool = ConnectionPool::new();
        pool.close_all().await;

        // Pool should be closed
        assert!(*pool.inner.closed.lock());
    }

    #[test]
    fn test_pooled_connection_getters() {
        let dc_id = DcId::internal(2);
        let purpose = ConnectionPurpose::Main;

        // Create a mock connection for testing
        let config = SessionConnectionConfig::new(dc_id);
        let auth_data = Arc::new(crate::auth::AuthDataShared::new(dc_id));
        let connection = Arc::new(SessionConnection::new(config, auth_data));

        // Create a mock pool
        let pool = Arc::new(ConnectionPoolInner {
            main_pools: Mutex::new(HashMap::new()),
            download_pools: Mutex::new(HashMap::new()),
            upload_pools: Mutex::new(HashMap::new()),
            config: PoolConfig::default(),
            closed: Mutex::new(false),
            pending_count: Mutex::new(0),
            dc_options: Mutex::new(DcOptionsSet::new()),
            circuit_breakers: Mutex::new(HashMap::new()),
            health_checker: HealthChecker::default(),
        });

        let pooled = PooledConnection::new(connection.clone(), dc_id, purpose, pool);

        assert_eq!(pooled.dc_id(), dc_id);
        assert_eq!(pooled.purpose(), purpose);
        // Check that both point to the same connection by comparing addresses
        assert!(std::ptr::eq(pooled.connection(), connection.as_ref()));
    }

    #[test]
    fn test_set_dc_options() {
        use crate::dc::{DcOption, DcOptions};
        use std::net::Ipv4Addr;

        let pool = ConnectionPool::new();

        // Create DC options with some options
        let mut dc_options = DcOptions::new();
        let option1 = DcOption::new(
            DcId::internal(2),
            Ipv4Addr::new(149, 154, 167, 51).into(),
            443,
        );
        dc_options.add(option1);

        let mut options_set = DcOptionsSet::new();
        options_set.add_options(dc_options);

        // Set DC options on the pool
        pool.set_dc_options(options_set.clone());

        // Verify DC options are stored
        let stored_options = pool.inner.dc_options.lock();
        assert_eq!(stored_options.get_options().dc_options.len(), 1);
        assert_eq!(
            stored_options.get_options().dc_options[0].dc_id,
            DcId::internal(2)
        );
    }

    #[test]
    fn test_set_dc_options_overwrites() {
        use crate::dc::{DcOption, DcOptions};
        use std::net::Ipv4Addr;

        let pool = ConnectionPool::new();

        // Set initial DC options
        let mut dc_options1 = DcOptions::new();
        let option1 =
            DcOption::new(DcId::internal(2), Ipv4Addr::new(149, 154, 167, 51).into(), 443);
        dc_options1.add(option1);

        let mut options_set1 = DcOptionsSet::new();
        options_set1.add_options(dc_options1);
        pool.set_dc_options(options_set1);

        // Set new DC options (should overwrite)
        let mut dc_options2 = DcOptions::new();
        let option2 =
            DcOption::new(DcId::internal(3), Ipv4Addr::new(149, 154, 167, 52).into(), 443);
        dc_options2.add(option2);

        let mut options_set2 = DcOptionsSet::new();
        options_set2.add_options(dc_options2);
        pool.set_dc_options(options_set2);

        // Verify DC options were overwritten
        let stored_options = pool.inner.dc_options.lock();
        assert_eq!(stored_options.get_options().dc_options.len(), 1);
        assert_eq!(
            stored_options.get_options().dc_options[0].dc_id,
            DcId::internal(3)
        );
    }
}
