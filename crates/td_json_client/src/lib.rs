// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram TdJsonClient - JSON client interface types for Telegram MTProto.
//!
//! This module provides utility types and functions for working with TDLib's
//! JSON client interface. The actual FFI calls are only available when the
//! TDLib C library is linked.
//!
//! ## Overview
//!
//! The JSON client interface provides a way to interact with TDLib using JSON
//! serialization:
//!
//! - [`validate_json`] - Validates JSON strings for TDLib
//!
//! ## TDLib Correspondence
//!
//! | Rust function | C function | File |
//! |---------------|------------|------|
//! | [`validate_json`] | N/A (helper) | - |
//!
//! ## Examples
//!
//! ```
//! use rustgram_td_json_client::validate_json;
//!
//! // Validate JSON strings
//! assert!(validate_json(r#"{"test": 123}"#).is_ok());
//! assert!(validate_json("invalid json").is_err());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

/// Errors that can occur when working with the JSON client interface.
#[derive(thiserror::Error, Debug)]
pub enum TdJsonClientError {
    /// Invalid client ID
    #[error("Invalid client ID: {0}")]
    InvalidClientId(i32),

    /// Null pointer received
    #[error("Null pointer encountered")]
    NullPointer,

    /// Invalid UTF-8 string
    #[error("Invalid UTF-8 string: {0}")]
    InvalidUtf8(String),

    /// JSON serialization error
    #[error("JSON error: {0}")]
    JsonError(String),

    /// Client creation failed
    #[error("Failed to create client")]
    ClientCreationFailed,
}

/// Result type for JSON client operations.
pub type Result<T> = std::result::Result<T, TdJsonClientError>;

/// Helper function to validate JSON string.
///
/// # Arguments
///
/// * `json` - The JSON string to validate
///
/// # Returns
///
/// `Ok(())` if valid, `Err(TdJsonClientError::JsonError)` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_td_json_client::validate_json;
///
/// assert!(validate_json(r#"{"test": 123}"#).is_ok());
/// assert!(validate_json(r#"null"#).is_ok());
/// assert!(validate_json(r#"true"#).is_ok());
/// assert!(validate_json("invalid json").is_err());
/// ```
pub fn validate_json(json: &str) -> Result<()> {
    serde_json::from_str::<serde_json::Value>(json)
        .map_err(|e| TdJsonClientError::JsonError(e.to_string()))?;
    Ok(())
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-td-json-client";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-td-json-client");
    }

    #[test]
    fn test_validate_json() {
        assert!(validate_json(r#"{"test": 123}"#).is_ok());
        assert!(validate_json(r#"null"#).is_ok());
        assert!(validate_json(r#"true"#).is_ok());
        assert!(validate_json(r#""test""#).is_ok());
        assert!(validate_json(r#"123"#).is_ok());
        assert!(validate_json("invalid json").is_err());
        assert!(validate_json("{incomplete").is_err());
    }

    #[test]
    fn test_json_error_message() {
        let result = validate_json("not json");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, TdJsonClientError::JsonError(_)));
    }

    #[test]
    fn test_validate_empty_json() {
        // Empty string is not valid JSON
        assert!(validate_json("").is_err());
    }

    #[test]
    fn test_validate_array() {
        assert!(validate_json(r#"[1, 2, 3]"#).is_ok());
        assert!(validate_json(r#"[]"#).is_ok());
    }

    #[test]
    fn test_validate_nested() {
        assert!(validate_json(r#"{"a": {"b": {"c": 1}}}"#).is_ok());
    }
}
