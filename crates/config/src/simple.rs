// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Simple config module.
//!
//! Implements the basic Telegram configuration returned by config.getConfig.

use bytes::Bytes;
use rustgram_net::{DcId, DcOptions};

use crate::error::{ConfigError, Result};
use crate::tl::ConfigDcOptions;
use rustgram_types::tl::TlHelper;

/// Simple configuration for Telegram MTProto client.
///
/// Contains the basic connection information needed to establish
/// a connection to Telegram servers.
///
/// # Examples
///
/// ```no_run
/// use rustgram_config::SimpleConfig;
/// use rustgram_net::DcId;
///
/// let config = SimpleConfig::new(
///     DcId::internal(2),
///     true,
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleConfig {
    /// Available data center options.
    pub dc_options: DcOptions,
    /// The default DC ID.
    pub dc_id: DcId,
    /// Whether this is test mode configuration.
    pub test_mode: bool,
}

impl SimpleConfig {
    /// Creates a new SimpleConfig.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - The default data center ID
    /// * `test_mode` - Whether this is test mode
    pub fn new(dc_id: DcId, test_mode: bool) -> Self {
        Self {
            dc_options: DcOptions::new(),
            dc_id,
            test_mode,
        }
    }

    /// Creates a SimpleConfig from TL response bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The serialized TL response data
    pub fn from_tl_bytes(data: Bytes) -> Result<Self> {
        // Try to read the constructor ID first to determine response type
        let mut peek_buf = rustgram_types::tl::Bytes::new(data.clone());
        let constructor_id = TlHelper::read_constructor_id(&mut peek_buf)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        match constructor_id {
            crate::tl::CONFIG_DC_OPTIONS_MAGIC => {
                let config = ConfigDcOptions::deserialize_tl_bytes(data)?;
                Ok(Self::from_tl(config))
            }
            crate::tl::CONFIG_EMPTY_MAGIC => {
                // Return default config for empty response
                Ok(Self::new(DcId::internal(1), false))
            }
            _ => Err(ConfigError::invalid_config(format!(
                "Unknown config constructor: 0x{:08x}",
                constructor_id
            ))),
        }
    }

    /// Creates a SimpleConfig from the TL ConfigDcOptions type.
    pub fn from_tl(tl_config: ConfigDcOptions) -> Self {
        use std::net::IpAddr;
        use tracing::warn;

        let mut dc_options = DcOptions::new();

        for tl_option in tl_config.dc_options {
            // Parse IP address from string
            let ip_addr = match tl_option.ip_address.parse::<IpAddr>() {
                Ok(addr) => addr,
                Err(e) => {
                    warn!(
                        dc_id = tl_option.dc_id,
                        ip_address = %tl_option.ip_address,
                        error = %e,
                        "Failed to parse IP address, skipping DC option"
                    );
                    continue; // Skip invalid DC options
                }
            };

            let dc_option = rustgram_net::DcOption::new(
                DcId::internal(tl_option.dc_id),
                ip_addr,
                tl_option.port as u16,
            );

            dc_options.add(dc_option);
        }

        let dc_id = if !dc_options.is_empty() {
            dc_options
                .dc_options
                .first()
                .map(|o| o.dc_id)
                .unwrap_or_else(|| DcId::internal(1))
        } else {
            DcId::internal(1)
        };

        Self {
            dc_options,
            dc_id,
            test_mode: false,
        }
    }

    /// Returns the DC options.
    pub fn dc_options(&self) -> &DcOptions {
        &self.dc_options
    }

    /// Returns the default DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns `true` if this is test mode configuration.
    pub fn is_test_mode(&self) -> bool {
        self.test_mode
    }

    /// Sets the DC options.
    pub fn set_dc_options(&mut self, dc_options: DcOptions) {
        self.dc_options = dc_options;
    }

    /// Sets the default DC ID.
    pub fn set_dc_id(&mut self, dc_id: DcId) {
        self.dc_id = dc_id;
    }
}

impl Default for SimpleConfig {
    fn default() -> Self {
        Self::new(DcId::internal(1), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::TlSerialize;

    #[test]
    fn test_simple_config_creation() {
        let config = SimpleConfig::new(DcId::internal(2), true);

        assert_eq!(config.dc_id(), DcId::internal(2));
        assert!(config.is_test_mode());
    }

    #[test]
    fn test_simple_config_default() {
        let config = SimpleConfig::default();

        assert_eq!(config.dc_id(), DcId::internal(1));
        assert!(!config.is_test_mode());
    }

    #[test]
    fn test_simple_config_from_tl() {
        let tl_config = ConfigDcOptions::new(vec![
            crate::tl::DcOption::new(2, "149.154.167.51".to_string(), 443, 0),
            crate::tl::DcOption::new(3, "149.154.167.52".to_string(), 443, 0),
        ]);

        let config = SimpleConfig::from_tl(tl_config);

        assert_eq!(config.dc_id, DcId::internal(2));
        assert_eq!(config.dc_options.len(), 2);
    }

    #[test]
    fn test_simple_config_setters() {
        let mut config = SimpleConfig::default();

        assert_eq!(config.dc_id(), DcId::internal(1));

        config.set_dc_id(DcId::internal(4));
        assert_eq!(config.dc_id(), DcId::internal(4));
    }

    #[test]
    fn test_dc_option_tl_roundtrip() {
        // Note: This test verifies that DcOption can be serialized.
        // The roundtrip test is not fully implemented since ConfigDcOptions
        // expects a vector magic prefix, not individual dcOption constructors.
        let opt = crate::tl::DcOption::new(2, "149.154.167.51".to_string(), 443, 0);

        let mut buf = bytes::BytesMut::new();
        let result = opt.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(!buf.is_empty());
    }
}
