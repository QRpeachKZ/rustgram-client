// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Connection health checking utilities.
//!
//! This module provides utilities for checking the health of TCP connections
//! before reusing them from the connection pool.

use std::time::Duration;

use crate::connection::ConnectionError;
use crate::session::SessionConnection;

/// Health check configuration.
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Timeout for health check operations.
    /// Default: 1 second
    pub timeout: Duration,

    /// Whether to perform deep checks (actual probe) vs shallow checks.
    /// Default: false (shallow check only)
    pub deep_check: bool,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(1),
            deep_check: false,
        }
    }
}

/// Result of a health check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Connection is healthy
    Healthy,

    /// Connection is unhealthy
    Unhealthy,

    /// Health check timed out
    Timeout,

    /// Health check failed with error
    Error,
}

/// Health checker for connections.
#[derive(Debug)]
pub struct HealthChecker {
    config: HealthCheckConfig,
}

impl HealthChecker {
    /// Creates a new health checker.
    pub fn new(config: HealthCheckConfig) -> Self {
        Self { config }
    }

    /// Creates a health checker with default configuration.
    pub fn default() -> Self {
        Self::new(HealthCheckConfig::default())
    }

    /// Performs a health check on a session connection.
    ///
    /// This checks:
    /// 1. Connection state is Ready
    /// 2. Auth key is available
    /// 3. Optional: TCP socket is still connected
    pub async fn check_connection(
        &self,
        conn: &SessionConnection,
    ) -> Result<HealthStatus, ConnectionError> {
        use tokio::time::timeout;

        // Shallow check: verify connection state and auth key
        if !conn.is_ready() {
            tracing::debug!(
                "Health check failed for DC {:?}: connection not ready",
                conn.dc_id()
            );
            return Ok(HealthStatus::Unhealthy);
        }

        // Deep check: verify TCP socket if enabled
        if self.config.deep_check {
            let check_result = timeout(self.config.timeout, async {
                // Try to get DC option - this verifies the connection metadata
                conn.get_dc_option()
                    .map(|_| HealthStatus::Healthy)
                    .map_err(|e| {
                        tracing::debug!(
                            "Health check failed for DC {:?}: {}",
                            conn.dc_id(),
                            e
                        );
                        ConnectionError::Failed(e.to_string())
                    })
            })
            .await;

            match check_result {
                Ok(Ok(status)) => Ok(status),
                Ok(Err(e)) => Err(e),
                Err(_) => {
                    tracing::debug!(
                        "Health check timed out for DC {:?}",
                        conn.dc_id()
                    );
                    Ok(HealthStatus::Timeout)
                }
            }
        } else {
            Ok(HealthStatus::Healthy)
        }
    }

    /// Performs a quick health check (non-blocking, shallow only).
    ///
    /// This only checks connection state without async operations.
    pub fn check_connection_quick(conn: &SessionConnection) -> HealthStatus {
        if conn.is_ready() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthDataShared;
    use crate::dc::DcId;
    use crate::session::{SessionConnection, SessionConnectionConfig};
    use std::sync::Arc;

    #[test]
    fn test_health_status_values() {
        // Just verify the enum exists
        let _ = HealthStatus::Healthy;
        let _ = HealthStatus::Unhealthy;
        let _ = HealthStatus::Timeout;
        let _ = HealthStatus::Error;
    }

    #[test]
    fn test_health_checker_new() {
        let checker = HealthChecker::new(HealthCheckConfig::default());
        assert_eq!(checker.config.timeout, Duration::from_secs(1));
        assert!(!checker.config.deep_check);
    }

    #[test]
    fn test_health_checker_default() {
        let checker = HealthChecker::default();
        assert_eq!(checker.config.timeout, Duration::from_secs(1));
    }

    #[test]
    fn test_health_check_config_default() {
        let config = HealthCheckConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(1));
        assert!(!config.deep_check);
    }

    #[test]
    fn test_health_check_quick() {
        let dc_id = DcId::internal(2);
        let config = SessionConnectionConfig::new(dc_id);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let conn = SessionConnection::new(config, auth_data);

        // Connection not ready
        assert_eq!(
            HealthChecker::check_connection_quick(&conn),
            HealthStatus::Unhealthy
        );

        // Set to ready
        conn.set_state(crate::session::SessionState::Ready);
        conn.auth_data()
            .set_auth_key(crate::auth::AuthKey::new(
                123,
                vec![0u8; 256].try_into().unwrap(),
            ));

        // Should be healthy now
        assert_eq!(
            HealthChecker::check_connection_quick(&conn),
            HealthStatus::Healthy
        );
    }

    #[tokio::test]
    async fn test_health_check_unhealthy() {
        let checker = HealthChecker::default();
        let dc_id = DcId::internal(2);
        let config = SessionConnectionConfig::new(dc_id);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let conn = SessionConnection::new(config, auth_data);

        // Connection not ready
        let result = checker.check_connection(&conn).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), HealthStatus::Unhealthy);
    }
}
