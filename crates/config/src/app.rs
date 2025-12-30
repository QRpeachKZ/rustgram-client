// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Application config module.
//!
//! Implements the application configuration returned by help.getAppConfig.
//! This config is cached and periodically refreshed.

use bytes::Bytes;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::{ConfigError, Result};
use crate::tl::AppConfigTl;

/// Application configuration version constant (110).
pub const APP_CONFIG_VERSION: i32 = 110;

/// Default TTL for app config cache (24 hours).
pub const DEFAULT_APP_CONFIG_TTL: Duration = Duration::from_secs(24 * 60 * 60);

/// Minimum TTL for app config cache (1 hour).
pub const MIN_APP_CONFIG_TTL: Duration = Duration::from_secs(60 * 60);

/// Application configuration.
///
/// Contains application-level configuration data from Telegram servers.
/// This configuration is versioned and cached.
///
/// # Examples
///
/// ```no_run
/// use rustgram_config::AppConfig;
///
/// let config = AppConfig::new(110, 1234567890, 1234567900, false);
/// assert!(config.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    /// Config version (should be 110).
    pub version: i32,
    /// Config creation timestamp (Unix timestamp).
    pub date: i32,
    /// Config expiration timestamp (Unix timestamp).
    pub expires: i32,
    /// Whether this is test mode configuration.
    pub test_mode: bool,
}

impl AppConfig {
    /// Creates a new AppConfig.
    ///
    /// # Arguments
    ///
    /// * `version` - Config version (should be 110)
    /// * `date` - Config creation timestamp (Unix timestamp)
    /// * `expires` - Config expiration timestamp (Unix timestamp)
    /// * `test_mode` - Whether this is test mode
    pub fn new(version: i32, date: i32, expires: i32, test_mode: bool) -> Self {
        Self {
            version,
            date,
            expires,
            test_mode,
        }
    }

    /// Creates an AppConfig from TL response bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The serialized TL response data
    pub fn from_tl_bytes(data: Bytes) -> Result<Self> {
        let tl_config = AppConfigTl::deserialize_tl_bytes(data)?;
        Ok(Self::from_tl(tl_config))
    }

    /// Creates an AppConfig from the TL AppConfigTl type.
    pub fn from_tl(tl_config: AppConfigTl) -> Self {
        Self {
            version: tl_config.version,
            date: tl_config.date,
            expires: tl_config.expires,
            test_mode: tl_config.test_mode.as_bool(),
        }
    }

    /// Converts to the TL representation.
    pub fn to_tl(&self) -> AppConfigTl {
        AppConfigTl {
            version: self.version,
            date: self.date,
            expires: self.expires,
            test_mode: rustgram_types::TlBool::from_bool(self.test_mode),
            config: Vec::new(), // Empty config for now
        }
    }

    /// Returns the config version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Returns the config creation timestamp.
    pub fn date(&self) -> i32 {
        self.date
    }

    /// Returns the config expiration timestamp.
    pub fn expires(&self) -> i32 {
        self.expires
    }

    /// Returns `true` if this is test mode configuration.
    pub fn is_test_mode(&self) -> bool {
        self.test_mode
    }

    /// Checks if this config is currently valid.
    ///
    /// Returns `true` if the current time is before the expiration time.
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        now < self.expires
    }

    /// Checks if this config is expired.
    pub fn is_expired(&self) -> bool {
        !self.is_valid()
    }

    /// Returns the time until expiration.
    ///
    /// Returns `None` if already expired.
    pub fn time_until_expiration(&self) -> Option<Duration> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        if self.expires > now {
            let seconds = (self.expires - now) as u64;
            Some(Duration::from_secs(seconds))
        } else {
            None
        }
    }

    /// Checks if this config needs refresh.
    ///
    /// Returns `true` if expired or nearing expiration (within 1 hour).
    pub fn needs_refresh(&self) -> bool {
        if self.is_expired() {
            return true;
        }

        // Check if expiring within 1 hour
        if let Some(remaining) = self.time_until_expiration() {
            remaining < Duration::from_secs(60 * 60)
        } else {
            true
        }
    }

    /// Validates the config.
    ///
    /// Checks that version is correct and timestamps are valid.
    pub fn validate(&self) -> Result<()> {
        // Check version
        if self.version != APP_CONFIG_VERSION {
            return Err(ConfigError::invalid_config(format!(
                "Invalid app config version: expected {}, got {}",
                APP_CONFIG_VERSION, self.version
            )));
        }

        // Check dates are reasonable
        if self.date <= 0 {
            return Err(ConfigError::invalid_config(
                "Invalid date: must be positive",
            ));
        }

        if self.expires <= self.date {
            return Err(ConfigError::invalid_config(
                "Invalid expiration: must be after date",
            ));
        }

        Ok(())
    }

    /// Calculates the hash for caching purposes.
    ///
    /// TODO: Implement proper hash calculation based on config content.
    /// The hash should be calculated from the actual config JSON data
    /// for use with the hash parameter in getAppConfig. Current implementation
    /// uses a simplified algorithm based on version and date only.
    pub fn cache_hash(&self) -> i32 {
        // Simple hash based on version and date
        // In real implementation, this would be a proper hash
        (self.version.wrapping_mul(31)).wrapping_add(self.date) % 1_000_000
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let expires = now + DEFAULT_APP_CONFIG_TTL.as_secs() as i32;

        Self {
            version: APP_CONFIG_VERSION,
            date: now,
            expires,
            test_mode: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_valid_config() -> AppConfig {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        AppConfig::new(APP_CONFIG_VERSION, now, now + 3600, false)
    }

    #[test]
    fn test_app_config_creation() {
        let config = AppConfig::new(110, 1234567890, 1234571490, false);

        assert_eq!(config.version(), 110);
        assert_eq!(config.date(), 1234567890);
        assert_eq!(config.expires(), 1234571490);
        assert!(!config.is_test_mode());
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();

        assert_eq!(config.version(), APP_CONFIG_VERSION);
        assert!(config.is_valid());
    }

    #[test]
    fn test_app_config_validate() {
        let config = make_valid_config();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_app_config_validate_wrong_version() {
        let config = AppConfig::new(100, 1234567890, 1234571490, false);

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_app_config_validate_invalid_date() {
        let config = AppConfig::new(110, 0, 1234571490, false);

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_app_config_validate_expires_before_date() {
        let config = AppConfig::new(110, 1234571490, 1234567890, false);

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_app_config_is_valid() {
        let config = make_valid_config();

        assert!(config.is_valid());
        assert!(!config.is_expired());
    }

    #[test]
    fn test_app_config_is_expired() {
        let past = 1234567890i32;
        let config = AppConfig::new(110, past - 3600, past, false);

        assert!(!config.is_valid());
        assert!(config.is_expired());
    }

    #[test]
    fn test_app_config_needs_refresh() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        // Not expiring soon
        let config = AppConfig::new(110, now, now + 7200, false);
        assert!(!config.needs_refresh());

        // Expiring within 1 hour
        let config = AppConfig::new(110, now, now + 1800, false);
        assert!(config.needs_refresh());

        // Already expired
        let config = AppConfig::new(110, now - 3600, now - 1, false);
        assert!(config.needs_refresh());
    }

    #[test]
    fn test_app_config_time_until_expiration() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .expect("System time should be available");

        let config = AppConfig::new(110, now, now + 3600, false);

        let remaining = config.time_until_expiration();
        assert!(remaining.is_some());
        let remaining = remaining.expect("Remaining time should be available");
        assert!(remaining.as_secs() <= 3600);
        assert!(remaining.as_secs() >= 3590); // Allow 10 seconds for test execution
    }

    #[test]
    fn test_app_config_cache_hash() {
        let config = AppConfig::new(110, 1234567890, 1234571490, false);

        let hash = config.cache_hash();
        assert!(hash >= 0);
        assert!(hash < 1_000_000);
    }

    #[test]
    fn test_app_config_roundtrip_tl() {
        let config = make_valid_config();

        let tl_config = config.to_tl();
        let restored = AppConfig::from_tl(tl_config);

        assert_eq!(restored.version, config.version);
        assert_eq!(restored.date, config.date);
        assert_eq!(restored.expires, config.expires);
        assert_eq!(restored.test_mode, config.test_mode);
    }

    #[test]
    fn test_constants() {
        assert_eq!(APP_CONFIG_VERSION, 110);
        assert_eq!(DEFAULT_APP_CONFIG_TTL, Duration::from_secs(24 * 60 * 60));
        assert_eq!(MIN_APP_CONFIG_TTL, Duration::from_secs(60 * 60));
    }

    #[cfg(feature = "proptest")]
    #[test]
    fn proptest_app_config_expiration() {
        use proptest::prelude::*;

        proptest!(|(date_offset in -86400i32..86400i32, ttl in 60i32..86400i32)| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i32)
                .unwrap_or(0);

            let date = now + date_offset;
            let expires = date + ttl;

            let config = AppConfig::new(APP_CONFIG_VERSION, date, expires, false);

            // Validate should succeed if timestamps are reasonable
            let validation_result = config.validate();

            if date <= 0 || expires <= date {
                assert!(validation_result.is_err());
            } else {
                assert!(validation_result.is_ok());
            }
        });
    }
}
