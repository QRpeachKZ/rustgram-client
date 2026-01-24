// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # TDLib Database Interface
//!
//! Database interface for TDLib.
//!
//! ## Overview
//!
//! This module provides a complete implementation for TDLib's database interface,
//! including SQLite databases, binlog, and key-value storage.
//!
//! ## Components
//!
//! - [`TdDb`] - Central coordinator for all databases
//! - [`TdDbParameters`] - Configuration for database initialization
//! - [`KeyValueStore`] - Key-value store for settings and preferences
//!
//! ## Example
//!
//! ```rust,no_run
//! use rustgram_td_db::{TdDb, TdDbParameters};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let params = TdDbParameters::new(
//!     "/path/to/db".to_string(),
//!     "/path/to/files".to_string(),
//!     false,
//!     true
//! );
//!
//! let mut tddb = TdDb::open(params)?;
//!
//! // Access individual databases
//! if let Some(message_db) = tddb.get_message_db() {
//!     let mut sync = message_db.sync();
//!     // Use message database...
//! }
//!
//! tddb.close()?;
//! # Ok(())
//! # }
//! ```

pub mod coordinator;
pub mod kv_store;

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Re-export the main types
pub use coordinator::TdDb;
pub use kv_store::{KeyValueStore, KvError};

/// Parameters for opening TDLib database.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `TdDb::Parameters` struct.
///
/// # Example
///
/// ```rust
/// use rustgram_td_db::TdDbParameters;
///
/// let params = TdDbParameters::new(
///     "/path/to/db".to_string(),
///     "/path/to/files".to_string(),
///     true,
///     false
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TdDbParameters {
    database_directory: String,
    files_directory: String,
    is_test_dc: bool,
    use_file_database: bool,
}

impl TdDbParameters {
    /// Creates new database parameters.
    ///
    /// # Arguments
    ///
    /// * `database_directory` - Directory for database files
    /// * `files_directory` - Directory for downloaded files
    /// * `is_test_dc` - Whether using test DC
    /// * `use_file_database` - Whether to use file database
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_td_db::TdDbParameters;
    ///
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     true,
    ///     false
    /// );
    /// ```
    #[must_use]
    pub fn new(
        database_directory: String,
        files_directory: String,
        is_test_dc: bool,
        use_file_database: bool,
    ) -> Self {
        Self {
            database_directory,
            files_directory,
            is_test_dc,
            use_file_database,
        }
    }

    /// Returns the database directory path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_td_db::TdDbParameters;
    ///
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     true,
    ///     false
    /// );
    /// assert_eq!(params.database_directory(), "/path/to/db");
    /// ```
    #[must_use]
    pub fn database_directory(&self) -> &str {
        &self.database_directory
    }

    /// Returns the files directory path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_td_db::TdDbParameters;
    ///
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     true,
    ///     false
    /// );
    /// assert_eq!(params.files_directory(), "/path/to/files");
    /// ```
    #[must_use]
    pub fn files_directory(&self) -> &str {
        &self.files_directory
    }

    /// Returns whether this is for test DC.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_td_db::TdDbParameters;
    ///
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     true,
    ///     false
    /// );
    /// assert!(params.is_test_dc());
    /// ```
    #[must_use]
    pub const fn is_test_dc(&self) -> bool {
        self.is_test_dc
    }

    /// Returns whether to use file database.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_td_db::TdDbParameters;
    ///
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     true,
    ///     false
    /// );
    /// assert!(!params.use_file_database());
    /// ```
    #[must_use]
    pub const fn use_file_database(&self) -> bool {
        self.use_file_database
    }
}

impl fmt::Display for TdDbParameters {
    /// Formats the parameters for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_td_db::TdDbParameters;
    ///
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     true,
    ///     false
    /// );
    /// let s = format!("{}", params);
    /// assert!(s.contains("TdDbParameters"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TdDbParameters(db: {}, files: {}, test_dc: {}, use_files: {})",
            self.database_directory, self.files_directory, self.is_test_dc, self.use_file_database
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let params = TdDbParameters::new(
            "/path/to/db".to_string(),
            "/path/to/files".to_string(),
            true,
            false,
        );
        assert_eq!(params.database_directory(), "/path/to/db");
        assert_eq!(params.files_directory(), "/path/to/files");
        assert!(params.is_test_dc());
        assert!(!params.use_file_database());
    }

    #[test]
    fn test_database_directory() {
        let params = TdDbParameters::new("/my/db".to_string(), "/files".to_string(), false, true);
        assert_eq!(params.database_directory(), "/my/db");
    }

    #[test]
    fn test_files_directory() {
        let params = TdDbParameters::new("/db".to_string(), "/my/files".to_string(), false, true);
        assert_eq!(params.files_directory(), "/my/files");
    }

    #[test]
    fn test_is_test_dc_true() {
        let params = TdDbParameters::new("/db".to_string(), "/files".to_string(), true, false);
        assert!(params.is_test_dc());
    }

    #[test]
    fn test_is_test_dc_false() {
        let params = TdDbParameters::new("/db".to_string(), "/files".to_string(), false, false);
        assert!(!params.is_test_dc());
    }

    #[test]
    fn test_use_file_database_true() {
        let params = TdDbParameters::new("/db".to_string(), "/files".to_string(), false, true);
        assert!(params.use_file_database());
    }

    #[test]
    fn test_use_file_database_false() {
        let params = TdDbParameters::new("/db".to_string(), "/files".to_string(), false, false);
        assert!(!params.use_file_database());
    }

    #[test]
    fn test_equality() {
        let params1 = TdDbParameters::new("/db".to_string(), "/files".to_string(), true, false);
        let params2 = TdDbParameters::new("/db".to_string(), "/files".to_string(), true, false);
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_inequality() {
        let params1 = TdDbParameters::new("/db1".to_string(), "/files".to_string(), true, false);
        let params2 = TdDbParameters::new("/db2".to_string(), "/files".to_string(), true, false);
        assert_ne!(params1, params2);
    }

    #[test]
    fn test_clone_semantics() {
        let params1 = TdDbParameters::new("/db".to_string(), "/files".to_string(), true, false);
        let params2 = params1.clone();
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_display_format() {
        let params = TdDbParameters::new("/db".to_string(), "/files".to_string(), true, false);
        let s = format!("{}", params);
        assert!(s.contains("TdDbParameters"));
    }

    #[test]
    fn test_debug_format() {
        let params = TdDbParameters::new("/db".to_string(), "/files".to_string(), true, false);
        let debug_str = format!("{:?}", params);
        assert!(debug_str.contains("TdDbParameters"));
    }

    #[test]
    fn test_empty_paths() {
        let params = TdDbParameters::new("".to_string(), "".to_string(), false, false);
        assert_eq!(params.database_directory(), "");
        assert_eq!(params.files_directory(), "");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = TdDbParameters::new("/db".to_string(), "/files".to_string(), true, false);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TdDbParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
