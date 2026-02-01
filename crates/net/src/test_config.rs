// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Test configuration for MTProto servers.
//!
//! This module provides configuration for switching between production and test
//! Telegram data centers. It manages DC IP addresses and RSA keys for both
//! environments.
//!
//! # Example
//!
//! ```ignore
//! use rustgram_net::{set_test_mode, is_test_dc, get_dc_options};
//!
//! // Switch to test DC
//! set_test_mode(true);
//! assert!(is_test_dc());
//!
//! // Get test DC options
//! let dc_options = get_dc_options();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use std::net::{IpAddr, Ipv4Addr};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::dc::{DcId, DcOption, DcOptions};
use crate::rsa_key_shared::RsaKey;

/// Global state for environment (production/test).
static IS_TEST_DC: AtomicBool = AtomicBool::new(false);

/// Sets the mode for test DC.
///
/// When `is_test` is `true`, all subsequent calls to [`get_dc_options`] and
/// [`get_rsa_keys`] will return test environment configuration.
///
/// # Arguments
///
/// * `is_test` - `true` for test environment, `false` for production
///
/// # Example
///
/// ```
/// use rustgram_net::set_test_mode;
///
/// // Enable test DC mode
/// set_test_mode(true);
///
/// // Switch back to production
/// set_test_mode(false);
/// ```
pub fn set_test_mode(is_test: bool) {
    IS_TEST_DC.store(is_test, Ordering::SeqCst);
}

/// Returns `true` if test DC mode is enabled.
///
/// # Example
///
/// ```
/// use rustgram_net::{is_test_dc, set_test_mode};
///
/// assert!(!is_test_dc());
/// set_test_mode(true);
/// assert!(is_test_dc());
/// ```
pub fn is_test_dc() -> bool {
    IS_TEST_DC.load(Ordering::SeqCst)
}

/// Returns DC options for the current environment.
///
/// Returns test DC options if [`is_test_dc`] is `true`, otherwise returns
/// production DC options.
///
/// # Example
///
/// ```
/// use rustgram_net::{get_dc_options, set_test_mode, DcId};
///
/// // Get production DC options
/// set_test_mode(false);
/// let prod_options = get_dc_options();
///
/// // Get test DC options
/// set_test_mode(true);
/// let test_options = get_dc_options();
/// ```
pub fn get_dc_options() -> DcOptions {
    if is_test_dc() {
        test_dc_options()
    } else {
        production_dc_options()
    }
}

/// Parses a single DC option from an environment variable.
///
/// Reads the `RUSTGRAM_DC{N}` environment variable (e.g., `RUSTGRAM_DC2`)
/// and parses it in the format `IP:PORT`.
///
/// # Arguments
///
/// * `dc_id` - The numeric DC ID (1-5)
///
/// # Returns
///
/// * `Some(DcOption)` if the environment variable is set and valid
/// * `None` if the variable is not set or contains invalid data
///
/// # Example
///
/// ```ignore
/// std::env::set_var("RUSTGRAM_DC2", "149.154.167.51:443");
/// let dc2 = parse_dc_from_env(2);
/// assert!(dc2.is_some());
/// ```
fn parse_dc_from_env(dc_id: i32) -> Option<DcOption> {
    let env_var = format!("RUSTGRAM_DC{}", dc_id);
    std::env::var(&env_var).ok().and_then(|value| {
        // Handle IPv6 addresses which contain colons
        // Format: IP:PORT where IP can be IPv4 or IPv6
        // For IPv6, we look for the last colon which separates IP from PORT
        if let Some(last_colon_pos) = value.rfind(':') {
            let ip_str = &value[..last_colon_pos];
            let port_str = &value[last_colon_pos + 1..];

            // Strip brackets from IPv6 addresses if present
            let ip_str = ip_str.strip_prefix('[').unwrap_or(ip_str);
            let ip_str = ip_str.strip_suffix(']').unwrap_or(ip_str);

            let ip: IpAddr = ip_str.parse().ok()?;
            let port: u16 = port_str.parse().ok()?;
            Some(DcOption::new(DcId::internal(dc_id), ip, port))
        } else {
            None
        }
    })
}

/// Returns RSA keys for the current environment.
///
/// Returns test RSA keys if [`is_test_dc`] is `true`, otherwise returns
/// production RSA keys.
///
/// # Note
///
/// Currently returns empty vectors. The keys should be loaded from `.pem` files
/// and passed to [`RsaKeyManager`].
///
/// # Example
///
/// ```
/// use rustgram_net::{get_rsa_keys, set_test_mode};
///
/// // Get production RSA keys
/// set_test_mode(false);
/// let prod_keys = get_rsa_keys();
///
/// // Get test RSA keys
/// set_test_mode(true);
/// let test_keys = get_rsa_keys();
/// ```
pub fn get_rsa_keys() -> Vec<RsaKey> {
    if is_test_dc() {
        test_rsa_keys()
    } else {
        production_rsa_keys()
    }
}

/// Production DC options from TDLib.
///
/// Returns the default Data Centers that Telegram uses for production connections.
/// These are the official Telegram production servers.
///
/// Environment variables (`RUSTGRAM_DC1` through `RUSTGRAM_DC5`) override
/// the hardcoded defaults. The format is `IP:PORT` (e.g., `149.154.167.51:443`).
///
/// # Production DCs (defaults)
///
/// - DC 1: `149.154.175.50:443`
/// - DC 2: `149.154.167.51:443` (default for new auth)
/// - DC 3: `149.154.175.100:443`
/// - DC 4: `149.154.167.91:443`
/// - DC 5: `149.154.171.5:443`
fn production_dc_options() -> DcOptions {
    let mut options = DcOptions::new();

    // Try to load from environment variables, fall back to hardcoded defaults
    for dc_id in 1..=5 {
        if let Some(opt) = parse_dc_from_env(dc_id) {
            options.add(opt);
        } else {
            // Fall back to hardcoded defaults
            match dc_id {
                1 => options.add(DcOption::new(
                    DcId::internal(1),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 175, 50)),
                    443,
                )),
                2 => options.add(DcOption::new(
                    DcId::internal(2),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 167, 51)),
                    443,
                )),
                3 => options.add(DcOption::new(
                    DcId::internal(3),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 175, 100)),
                    443,
                )),
                4 => options.add(DcOption::new(
                    DcId::internal(4),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 167, 91)),
                    443,
                )),
                5 => options.add(DcOption::new(
                    DcId::internal(5),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 171, 5)),
                    443,
                )),
                _ => {}
            }
        }
    }

    options
}

/// Test DC options from TDLib.
///
/// Returns the Data Centers that Telegram uses for test environment.
/// These are the official Telegram test servers.
///
/// Environment variables (`RUSTGRAM_DC1` through `RUSTGRAM_DC5`) override
/// the hardcoded defaults. The format is `IP:PORT` (e.g., `149.154.167.40:443`).
///
/// # Test DCs (defaults)
///
/// - Test DC 1: `149.154.175.10:443`
/// - Test DC 2: `149.154.167.40:443`
/// - Test DC 3: `149.154.175.117:443`
fn test_dc_options() -> DcOptions {
    let mut options = DcOptions::new();

    // Try to load from environment variables, fall back to hardcoded defaults
    for dc_id in 1..=3 {
        if let Some(opt) = parse_dc_from_env(dc_id) {
            options.add(opt);
        } else {
            // Fall back to hardcoded defaults
            match dc_id {
                1 => options.add(DcOption::new(
                    DcId::internal(1),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 175, 10)),
                    443,
                )),
                2 => options.add(DcOption::new(
                    DcId::internal(2),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 167, 40)),
                    443,
                )),
                3 => options.add(DcOption::new(
                    DcId::internal(3),
                    IpAddr::V4(Ipv4Addr::new(149, 154, 175, 117)),
                    443,
                )),
                _ => {}
            }
        }
    }

    options
}

/// Production RSA key from TDLib.
///
/// # Note
///
/// Currently returns an empty vector. The production RSA key should be:
/// 1. Extracted from TDLib source (`td/mtproto/Rsa.cpp`)
/// 2. Or obtained from https://my.telegram.org/apps
/// 3. Loaded from a `.pem` file at runtime
///
/// TODO: Add real production RSA key from TDLib
fn production_rsa_keys() -> Vec<RsaKey> {
    // TODO: Add real production RSA key from TDLib
    // The key can be found in TDLib: td/mtproto/Rsa.cpp
    // For now, empty vector - keys should be loaded from .pem file
    Vec::new()
}

/// Test RSA key from TDLib.
///
/// # Note
///
/// Currently returns an empty vector. The test RSA key should be:
/// 1. Extracted from TDLib source (`td/mtproto/Rsa.cpp`)
/// 2. Loaded from a `.pem` file at runtime
///
/// TODO: Add real test RSA key from TDLib
fn test_rsa_keys() -> Vec<RsaKey> {
    // TODO: Add real test RSA key from TDLib
    // The key can be found in TDLib: td/mtproto/Rsa.cpp
    // For now, empty vector - keys should be loaded from .pem file
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Tests that modify global state should ideally be run serially.
    // For now, each test explicitly sets its desired state.

    #[test]
    fn test_environment_switching() {
        // Set initial state
        set_test_mode(false);
        assert!(!is_test_dc());

        // Switch to test mode
        set_test_mode(true);
        assert!(is_test_dc());

        // Switch back to production
        set_test_mode(false);
        assert!(!is_test_dc());
    }

    #[test]
    fn test_production_dc_ips() {
        // Ensure production mode is set and no env vars override defaults
        set_test_mode(false);
        assert!(!is_test_dc());

        // Clean up any env vars that might interfere
        std::env::remove_var("RUSTGRAM_DC1");
        std::env::remove_var("RUSTGRAM_DC2");
        std::env::remove_var("RUSTGRAM_DC3");
        std::env::remove_var("RUSTGRAM_DC4");
        std::env::remove_var("RUSTGRAM_DC5");

        let options = get_dc_options();

        // Check that we have DC options
        assert!(!options.is_empty());

        // DC 2 should be 149.154.167.51:443 in production
        let dc2_opts = options.get_options(DcId::internal(2));
        assert!(!dc2_opts.is_empty());
        assert_eq!(
            dc2_opts[0].ip_address,
            IpAddr::V4(Ipv4Addr::new(149, 154, 167, 51))
        );
        assert_eq!(dc2_opts[0].port, 443);
    }

    #[test]
    fn test_test_dc_ips() {
        // Ensure test mode is set and no env vars override defaults
        set_test_mode(true);
        assert!(is_test_dc());

        // Clean up any env vars that might interfere
        std::env::remove_var("RUSTGRAM_DC1");
        std::env::remove_var("RUSTGRAM_DC2");
        std::env::remove_var("RUSTGRAM_DC3");
        std::env::remove_var("RUSTGRAM_DC4");
        std::env::remove_var("RUSTGRAM_DC5");

        let options = get_dc_options();

        // Test DC 2 should be 149.154.167.40:443
        let dc2_opts = options.get_options(DcId::internal(2));
        assert!(!dc2_opts.is_empty());
        assert_eq!(
            dc2_opts[0].ip_address,
            IpAddr::V4(Ipv4Addr::new(149, 154, 167, 40))
        );
        assert_eq!(dc2_opts[0].port, 443);

        // Reset to production mode for other tests
        set_test_mode(false);
    }

    #[test]
    fn test_production_dc_count() {
        // Set production mode
        set_test_mode(false);
        let options = get_dc_options();

        // Should have 5 production DCs
        assert!(options.len() >= 5);
    }

    #[test]
    fn test_test_dc_count() {
        // Set test mode
        set_test_mode(true);
        let options = get_dc_options();

        // Should have at least 3 test DCs
        assert!(options.len() >= 3);

        // Reset to production mode
        set_test_mode(false);
    }

    #[test]
    fn test_dc_options_isolation() {
        // Get production options
        set_test_mode(false);
        let prod_options = get_dc_options();
        let prod_dc2 = prod_options.get_options(DcId::internal(2));

        // Get test options
        set_test_mode(true);
        let test_options = get_dc_options();
        let test_dc2 = test_options.get_options(DcId::internal(2));

        // DC 2 IPs should be different
        assert_ne!(prod_dc2[0].ip_address, test_dc2[0].ip_address);

        // Reset to production mode
        set_test_mode(false);
    }

    #[test]
    fn test_get_rsa_keys_returns_vec() {
        // Test production keys
        set_test_mode(false);
        let prod_keys = get_rsa_keys();
        assert!(prod_keys.is_empty()); // Currently empty

        // Test keys
        set_test_mode(true);
        let test_keys = get_rsa_keys();
        assert!(test_keys.is_empty()); // Currently empty

        // Reset to production mode
        set_test_mode(false);
    }

    #[test]
    fn test_parse_dc_from_env_valid() {
        // Set environment variable
        std::env::set_var("RUSTGRAM_DC2", "1.2.3.4:443");

        let result = parse_dc_from_env(2);
        assert!(result.is_some());
        let dc_option = result.unwrap();
        assert_eq!(dc_option.dc_id, DcId::internal(2));
        assert_eq!(dc_option.ip_address, IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)));
        assert_eq!(dc_option.port, 443);

        // Clean up
        std::env::remove_var("RUSTGRAM_DC2");
    }

    #[test]
    fn test_parse_dc_from_env_invalid_format() {
        // Test with invalid format (missing port)
        std::env::set_var("RUSTGRAM_DC2", "1.2.3.4");
        let result = parse_dc_from_env(2);
        assert!(result.is_none());
        std::env::remove_var("RUSTGRAM_DC2");

        // Test with invalid format (too many parts)
        std::env::set_var("RUSTGRAM_DC2", "1.2.3.4:443:extra");
        let result = parse_dc_from_env(2);
        assert!(result.is_none());
        std::env::remove_var("RUSTGRAM_DC2");

        // Test with invalid IP
        std::env::set_var("RUSTGRAM_DC2", "invalid-ip:443");
        let result = parse_dc_from_env(2);
        assert!(result.is_none());
        std::env::remove_var("RUSTGRAM_DC2");

        // Test with invalid port
        std::env::set_var("RUSTGRAM_DC2", "1.2.3.4:invalid");
        let result = parse_dc_from_env(2);
        assert!(result.is_none());
        std::env::remove_var("RUSTGRAM_DC2");
    }

    #[test]
    fn test_parse_dc_from_env_not_set() {
        // Ensure the variable is not set
        std::env::remove_var("RUSTGRAM_DC3");

        let result = parse_dc_from_env(3);
        assert!(result.is_none());
    }

    #[test]
    fn test_production_dc_options_with_env_override() {
        // Override DC2 with custom IP
        std::env::set_var("RUSTGRAM_DC2", "5.6.7.8:443");

        set_test_mode(false);
        let options = get_dc_options();

        let dc2_opts = options.get_options(DcId::internal(2));
        assert!(!dc2_opts.is_empty());
        // Should use the env var value, not the default
        assert_eq!(
            dc2_opts[0].ip_address,
            IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8))
        );
        assert_eq!(dc2_opts[0].port, 443);

        // Clean up
        std::env::remove_var("RUSTGRAM_DC2");
    }

    #[test]
    fn test_production_dc_options_fallback_to_defaults() {
        // Ensure no env vars are set that might interfere
        std::env::remove_var("RUSTGRAM_DC1");
        std::env::remove_var("RUSTGRAM_DC2");
        std::env::remove_var("RUSTGRAM_DC3");
        std::env::remove_var("RUSTGRAM_DC4");
        std::env::remove_var("RUSTGRAM_DC5");

        set_test_mode(false);
        let options = get_dc_options();

        let dc1_opts = options.get_options(DcId::internal(1));
        assert!(!dc1_opts.is_empty());
        // Should use the hardcoded default
        assert_eq!(
            dc1_opts[0].ip_address,
            IpAddr::V4(Ipv4Addr::new(149, 154, 175, 50))
        );
        assert_eq!(dc1_opts[0].port, 443);
    }

    #[test]
    fn test_test_dc_options_with_env_override() {
        // Override DC2 with custom IP for test mode
        std::env::set_var("RUSTGRAM_DC2", "9.10.11.12:8080");

        set_test_mode(true);
        let options = get_dc_options();

        let dc2_opts = options.get_options(DcId::internal(2));
        assert!(!dc2_opts.is_empty());
        // Should use the env var value, not the test default
        assert_eq!(
            dc2_opts[0].ip_address,
            IpAddr::V4(Ipv4Addr::new(9, 10, 11, 12))
        );
        assert_eq!(dc2_opts[0].port, 8080);

        // Clean up
        std::env::remove_var("RUSTGRAM_DC2");
        set_test_mode(false);
    }

    #[test]
    fn test_env_var_ipv6() {
        // Test IPv6 address parsing
        std::env::set_var("RUSTGRAM_DC4", "[2001:db8::1]:443");

        let result = parse_dc_from_env(4);
        assert!(result.is_some());
        let dc_option = result.unwrap();
        assert_eq!(dc_option.dc_id, DcId::internal(4));
        assert_eq!(dc_option.port, 443);

        // Clean up
        std::env::remove_var("RUSTGRAM_DC4");
    }
}
