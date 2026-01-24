//! # Rustgram Storage
//!
//! Storage layer for Telegram MTProto client.
//!
//! ## Features
//!
//! - **dialog**: Dialog database with SQLite backend (default)
//! - **message**: Message database with SQLite backend
//! - **user**: User database with SQLite backend
//! - **chat**: Chat database with SQLite backend
//! - **file**: File database with SQLite backend
//! - **secure**: Encrypted storage for sensitive data (optional)
//!
//! ## Modules
//!
//! - [`error`] — Error types for storage operations
//! - [`connection`] — Database connection management with pooling
//! - [`migrations`] — Schema migration management
//! - [`dialog`] — Dialog database implementation
//! - [`message`] — Message database implementation
//! - [`user`] — User database implementation
//! - [`chat`] — Chat database implementation
//! - [`file`] — File database implementation
//! - [`secure`] — Encrypted storage (feature "secure")

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod connection;
pub mod error;
pub mod migrations;

#[cfg(feature = "dialog")]
pub mod dialog;

#[cfg(feature = "message")]
pub mod message;

#[cfg(feature = "user")]
pub mod user;

#[cfg(feature = "chat")]
pub mod chat;

#[cfg(feature = "file")]
pub mod file;

#[cfg(feature = "secure")]
pub mod secure;

// Re-export commonly used types
pub use connection::{DbConnection, Transaction};
pub use error::{StorageError, StorageResult};
pub use migrations::{Migration, MigrationManager};

#[cfg(feature = "dialog")]
pub use dialog::{DialogDb, DialogDbAsync, DialogDbSync};

#[cfg(feature = "message")]
pub use message::{MessageDb, MESSAGE_DB_VERSION};

#[cfg(feature = "user")]
pub use user::{UserDb, USER_DB_VERSION};

#[cfg(feature = "chat")]
pub use chat::{ChatDb, CHAT_DB_VERSION};

#[cfg(feature = "file")]
pub use file::{FileDb, FILE_DB_VERSION};

#[cfg(feature = "secure")]
pub use secure::{decrypt_value, encrypt_value, Secret, ValueHash};

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
