// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Config - Configuration management for Telegram MTProto client.
//!
//! This module provides centralized configuration management for the Telegram
//! MTProto client, including caching and automatic refresh of configuration data.
//!
//! ## Overview
//!
//! The config module manages three types of configuration:
//!
//! - **Simple Config** - Basic connection configuration (DC options, default DC)
//! - **App Config** - Application-level configuration with versioning and caching
//! - **DH Config** - Diffie-Hellman parameters for key exchange
//!
//! ## Examples
//!
//! ```no_run
//! use rustgram_config::ConfigManager;
//! use std::sync::Arc;
//!
//! struct MockCallback;
//! impl rustgram_config::NetQueryCallback for MockCallback {
//!     fn send_query(&self, _query: rustgram_config::NetQuery) -> rustgram_config::Result<bytes::Bytes> {
//!         Ok(bytes::Bytes::new())
//!     }
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let sender = Arc::new(MockCallback);
//! let manager = ConfigManager::new(sender);
//!
//! // Fetch simple configuration
//! let simple_config = manager.get_simple_config().await?;
//! println!("DC ID: {}", simple_config.dc_id());
//!
//! // Fetch application configuration
//! let app_config = manager.get_app_config().await?;
//! println!("App version: {}", app_config.version());
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The module follows TDLib's architecture with:
//! - Type-safe configuration structures
//! - Thread-safe caching with `Arc` and `RwLock`
//! - TL (Type Language) serialization for protocol messages
//! - Automatic cache invalidation and refresh

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

mod app;
mod dh;
pub mod error;
mod manager;
mod simple;
pub mod tl;

// Re-export public API
pub use app::{AppConfig, APP_CONFIG_VERSION, DEFAULT_APP_CONFIG_TTL, MIN_APP_CONFIG_TTL};
pub use dh::DhConfig;
pub use error::{ConfigError, Result};
pub use manager::{ConfigManager, ConfigUpdate, NetQuery, NetQueryCallback, NetQuerySender};
pub use simple::SimpleConfig;

// Re-export TL types that are part of public API
pub use tl::{
    AppConfigTl, ConfigDcOptions, DcOption as DcOptionTl, DhConfigNotModified, DhConfigTl,
    GetAppConfig, GetConfig, GetDhConfig, APP_CONFIG_MAGIC, CONFIG_DC_OPTIONS_MAGIC,
    CONFIG_EMPTY_MAGIC, DH_CONFIG_MAGIC, DH_CONFIG_NOT_MODIFIED_MAGIC,
};

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-config";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-config");
    }

    #[test]
    fn test_constants() {
        assert_eq!(APP_CONFIG_VERSION, 110);
        assert!(DEFAULT_APP_CONFIG_TTL.as_secs() > 0);
        assert!(MIN_APP_CONFIG_TTL.as_secs() > 0);
    }

    #[test]
    fn test_tl_magic_constants() {
        // Verify magic values are unique
        let magics = [
            CONFIG_DC_OPTIONS_MAGIC,
            CONFIG_EMPTY_MAGIC,
            APP_CONFIG_MAGIC,
            DH_CONFIG_MAGIC,
            DH_CONFIG_NOT_MODIFIED_MAGIC,
        ];

        let mut unique = std::collections::HashSet::new();
        for magic in magics {
            assert!(
                unique.insert(magic),
                "Duplicate magic value: 0x{:08x}",
                magic
            );
        }
    }

    #[test]
    fn test_simple_config_default() {
        let config = SimpleConfig::default();
        assert!(!config.is_test_mode());
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.version(), APP_CONFIG_VERSION);
        assert!(config.is_valid());
    }

    #[test]
    fn test_config_update_variants() {
        // Test that ConfigUpdate variants are accessible
        let _update = ConfigUpdate::Ttl(300);
        let config = AppConfig::default();
        let _update = ConfigUpdate::app_config(config);
    }
}
