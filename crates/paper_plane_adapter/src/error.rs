// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the paper_plane_adapter.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::time::Duration;
use thiserror::Error;

/// Error type for adapter operations.
///
/// Represents all possible errors that can occur when using the TDLib-compatible
/// JSON API adapter.
#[derive(Debug, Error)]
pub enum AdapterError {
    /// JSON parsing failed.
    #[error("JSON parse error: {0}")]
    JsonParse(String),

    /// Invalid request structure.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Unknown request type.
    #[error("Unknown request type: {0}")]
    UnknownType(String),

    /// Missing required field.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid field value.
    ///
    /// Provides details about validation failures for specific request fields.
    #[error("Invalid field value for '{field}': {reason}")]
    InvalidValue {
        /// The field name that failed validation.
        field: String,
        /// Human-readable reason for the validation failure.
        reason: String,
    },

    /// Invalid client ID.
    #[error("Invalid client ID: {0}")]
    InvalidClientId(i32),

    /// Rate limit exceeded.
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Unknown TDLib method.
    #[error("Unknown TDLib method: {0}")]
    UnknownMethod(String),

    /// Invalid JSON format.
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    /// Manager call failed.
    #[error("Manager error: {0}")]
    Manager(String),

    /// Network error.
    #[error("Network error: {0}")]
    Network(String),

    /// Request timeout.
    #[error("Request timeout after {0:?}")]
    Timeout(Duration),

    /// Response serialization failed.
    #[error("Response serialization failed: {0}")]
    Serialization(String),

    /// Actor not available.
    #[error("Actor not available: {0}")]
    ActorNotAvailable(String),

    /// Invalid response format.
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// Operation not supported.
    #[error("Operation not supported: {0}")]
    NotSupported(String),
}

impl AdapterError {
    /// Creates an error for a missing field.
    #[must_use]
    pub fn missing_field(field: &'static str) -> Self {
        Self::MissingField(field.to_string())
    }

    /// Creates an error for an invalid field value.
    #[must_use]
    pub fn invalid_value(field: &'static str, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.to_string(),
            reason: reason.into(),
        }
    }

    /// Creates a manager error from any error type.
    pub fn manager_error<E: std::error::Error>(err: E) -> Self {
        Self::Manager(err.to_string())
    }

    /// Returns the TDLib error code for this error.
    pub fn error_code(&self) -> i32 {
        match self {
            Self::InvalidJson(_) | Self::JsonParse(_) => 400,
            Self::Manager(_) | Self::ActorNotAvailable(_) => 500,
            Self::Network(_) => 500,
            Self::InvalidClientId(_) => 400,
            Self::UnknownMethod(_) | Self::UnknownType(_) => 400,
            Self::Serialization(_) => 500,
            Self::MissingField(_) | Self::InvalidValue { .. } | Self::InvalidRequest(_) => 400,
            Self::Timeout(_) => 408,
            Self::InvalidResponse(_) => 500,
            Self::NotSupported(_) => 501,
            Self::RateLimitExceeded => 429,
        }
    }

    /// Converts this error to TDLib JSON format.
    pub fn to_tdlib_json(&self) -> serde_json::Value {
        serde_json::json!({
            "@type": "error",
            "code": self.error_code(),
            "message": self.to_string(),
        })
    }
}

impl From<serde_json::Error> for AdapterError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonParse(err.to_string())
    }
}

/// Result type for adapter operations.
pub type Result<T> = std::result::Result<T, AdapterError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AdapterError::UnknownType("testRequest".to_string());
        assert!(err.to_string().contains("testRequest"));
    }

    #[test]
    fn test_error_code_invalid_json() {
        let error = AdapterError::InvalidJson("test".to_string());
        assert_eq!(error.error_code(), 400);
    }

    #[test]
    fn test_error_code_network_error() {
        let error = AdapterError::Network("test".to_string());
        assert_eq!(error.error_code(), 500);
    }

    #[test]
    fn test_error_code_invalid_client_id() {
        let error = AdapterError::InvalidClientId(999);
        assert_eq!(error.error_code(), 400);
    }

    #[test]
    fn test_error_code_unknown_method() {
        let error = AdapterError::UnknownMethod("unknownMethod".to_string());
        assert_eq!(error.error_code(), 400);
    }

    #[test]
    fn test_error_code_timeout() {
        assert_eq!(
            AdapterError::Timeout(Duration::from_secs(30)).error_code(),
            408
        );
    }

    #[test]
    fn test_to_tdlib_json() {
        let error = AdapterError::InvalidClientId(999);
        let json = error.to_tdlib_json();

        assert_eq!(json["@type"], "error");
        assert_eq!(json["code"], 400);
        assert!(json["message"].as_str().unwrap().contains("999"));
    }

    #[test]
    fn test_missing_field() {
        let err = AdapterError::missing_field("chat_id");
        assert!(matches!(err, AdapterError::MissingField(_)));
        assert!(err.to_string().contains("chat_id"));
    }

    #[test]
    fn test_invalid_value() {
        let err = AdapterError::invalid_value("limit", "must be positive");
        assert!(matches!(err, AdapterError::InvalidValue { .. }));
        assert!(err.to_string().contains("limit"));
        assert!(err.to_string().contains("must be positive"));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let adapter_err: AdapterError = json_err.into();
        assert!(matches!(adapter_err, AdapterError::JsonParse(_)));
    }

    #[test]
    fn test_manager_error() {
        let source_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let adapter_err = AdapterError::manager_error(source_err);
        assert!(matches!(adapter_err, AdapterError::Manager(_)));
    }

    #[test]
    fn test_all_error_variants_display() {
        // Verify all error variants implement Display correctly
        let errors = vec![
            AdapterError::JsonParse("parse error".to_string()),
            AdapterError::InvalidRequest("bad request".to_string()),
            AdapterError::UnknownType("unknownType".to_string()),
            AdapterError::MissingField("field".to_string()),
            AdapterError::InvalidValue {
                field: "test".to_string(),
                reason: "bad value".to_string(),
            },
            AdapterError::InvalidClientId(123),
            AdapterError::UnknownMethod("unknownMethod".to_string()),
            AdapterError::InvalidJson("bad json".to_string()),
            AdapterError::Manager("manager error".to_string()),
            AdapterError::Network("network error".to_string()),
            AdapterError::Timeout(Duration::from_secs(10)),
            AdapterError::Serialization("serial error".to_string()),
            AdapterError::ActorNotAvailable("actor".to_string()),
            AdapterError::InvalidResponse("response".to_string()),
            AdapterError::NotSupported("unsupported".to_string()),
        ];

        for error in errors {
            let display = format!("{}", error);
            assert!(!display.is_empty());
        }
    }
}
