//! # Rustgram Storage
//!
//! Storage layer for Telegram MTProto client.
//!
//! ## Features
//!
//! - **dialog**: Dialog database with SQLite backend (default)
//! - **secure**: Encrypted storage for sensitive data (optional)
//!
//! ## Modules
//!
//! - [`error`] — Error types for storage operations
//! - [`connection`] — Database connection management with pooling
//! - [`migrations`] — Schema migration management
//! - [`dialog`] — Dialog database implementation
//! - [`secure`] — Encrypted storage (feature "secure")

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod error;
pub mod connection;
pub mod migrations;

#[cfg(feature = "dialog")]
pub mod dialog;

#[cfg(feature = "secure")]
pub mod secure;

// Re-export commonly used types
pub use error::{StorageError, StorageResult};
pub use connection::{DbConnection, Transaction};
pub use migrations::{Migration, MigrationManager};

#[cfg(feature = "dialog")]
pub use dialog::{DialogDb, DialogDbAsync, DialogDbSync};

#[cfg(feature = "secure")]
pub use secure::{Secret, ValueHash, encrypt_value, decrypt_value};

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-storage";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-storage");
    }

    #[test]
    fn test_error_is_transient() {
        let err = StorageError::DatabaseLocked;
        assert!(err.is_transient());

        let err = StorageError::NotFound("test".to_string());
        assert!(!err.is_transient());
    }
}
