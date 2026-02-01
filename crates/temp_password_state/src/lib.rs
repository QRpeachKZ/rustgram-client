// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! # Rustgram Temp Password State
//!
//! Temporary password management for Telegram 2FA.
//!
//! ## Overview
//!
//! This module provides temporary password state management for Telegram's
//! two-factor authentication system. Temporary passwords are time-limited tokens
//! used for secure operations like payment confirmation.
//!
//! ## Example
//!
//! ```
//! use rustgram_temp_password_state::TempPasswordState;
//! use std::time::{Duration, SystemTime, UNIX_EPOCH};
//!
//! // Create a temp password valid for 1 hour
//! let valid_until = SystemTime::now()
//!     .duration_since(UNIX_EPOCH)
//!     .map(|d| d.as_secs() as i32 + 3600)
//!     .unwrap_or(0);
//!
//! let state = TempPasswordState::new("secret123", valid_until);
//!
//! assert!(state.has_temp_password());
//! assert_eq!(state.temp_password(), "secret123");
//! assert_eq!(state.valid_until(), valid_until);
//! ```
//!
//! ## TDLib Compatibility
//!
//! - **Reference**: `references/td/td/telegram/TempPasswordState.{h,cpp,hpp}`
//! - **TL Type**: `temporaryPasswordState`
//! - **RPC**: `account.getTmpPassword`

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod state;
mod tl;

// Re-exports
pub use error::{Result, TempPasswordError};
pub use state::TempPasswordState;

/// Current version of the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-temp-password-state";

/// Binlog key for temp password storage (matches TDLib).
pub const BINLOG_KEY: &str = "temp_password";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
        assert_eq!(CRATE_NAME, "rustgram-temp-password-state");
    }

    #[test]
    fn test_binlog_key() {
        assert_eq!(BINLOG_KEY, "temp_password");
    }
}
