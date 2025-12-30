//! # Telegram Authentication Module
//!
//! This module handles Telegram authentication flow according to MTProto protocol.
//!
//! ## Reference Implementation
//! - Official TDLib: `references/td/td/telegram/`
//! - MTProto spec: https://core.telegram.org/mtproto
//!
//! ## TODO
//! - Implement PQ handshake
//! - Implement DH key exchange
//! - Implement authorization flow

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Authentication state machine
pub mod state;

/// Authentication errors
pub mod error;

/// Re-exports commonly used types
pub use error::AuthError;
pub use state::AuthState;

/// Current version of authentication protocol
pub const AUTH_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_format() {
        // AUTH_VERSION follows semver format: "0.1.0"
        assert!(AUTH_VERSION.contains('.'));
    }
}
