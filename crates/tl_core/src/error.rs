// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for TL deserialization.
//!
//! This module defines error types used throughout the TL core crate.

//! Error types for TL deserialization.
//!
//! This module defines error types used throughout the TL core crate.

/// Result type for TL operations.
pub type Result<T> = std::result::Result<T, TlError>;

/// Error that can occur during TL deserialization.
#[derive(Debug, thiserror::Error)]
pub enum TlError {
    /// Unknown constructor ID encountered during deserialization.
    #[error("Unknown constructor ID: 0x{found:08x}, expected one of: {expected:?}")]
    UnknownConstructor {
        /// Expected constructor IDs.
        expected: Vec<u32>,
        /// Actual constructor ID found.
        found: u32,
        /// Context where the error occurred.
        context: String,
    },

    /// Unexpected end of buffer.
    #[error(
        "Unexpected EOF: requested {requested} bytes, {remaining} remaining for type '{type_name}'"
    )]
    UnexpectedEof {
        /// Number of bytes requested.
        requested: usize,
        /// Number of bytes actually remaining.
        remaining: usize,
        /// Name of the type being deserialized.
        type_name: String,
    },

    /// Invalid UTF-8 string data.
    #[error("Invalid UTF-8 string in field '{field_name}': {cause}")]
    InvalidUtf8 {
        /// Field name being deserialized.
        field_name: String,
        /// Underlying UTF-8 error.
        #[source]
        cause: std::string::FromUtf8Error,
    },

    /// Vector deserialization error.
    #[error("Vector deserialization failed: {0}")]
    VectorError(#[from] VectorError),

    /// Flag field reading error.
    #[error("Flag field error at bit {flag_index} ('{field_name}'): {cause}")]
    FlagFieldError {
        /// Index of the flag bit.
        flag_index: u32,
        /// Name of the field.
        field_name: String,
        /// Underlying error.
        #[source]
        cause: Box<TlError>,
    },

    /// Validation failed for a field value.
    #[error("Validation failed for field '{field_name}': value={value}, reason={reason}")]
    ValidationFailed {
        /// Field name.
        field_name: String,
        /// Invalid value.
        value: String,
        /// Reason for validation failure.
        reason: String,
    },

    /// Generic deserialization error.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Type conversion error.
    #[error("Type conversion error: {0}")]
    TypeConversionError(String),
}

impl TlError {
    /// Creates an unknown constructor error.
    pub fn unknown_constructor(expected: Vec<u32>, found: u32, context: impl Into<String>) -> Self {
        Self::UnknownConstructor {
            expected,
            found,
            context: context.into(),
        }
    }

    /// Creates an unexpected EOF error.
    pub fn unexpected_eof(
        requested: usize,
        remaining: usize,
        type_name: impl Into<String>,
    ) -> Self {
        Self::UnexpectedEof {
            requested,
            remaining,
            type_name: type_name.into(),
        }
    }

    /// Creates an invalid UTF-8 error.
    pub fn invalid_utf8(field_name: impl Into<String>, cause: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8 {
            field_name: field_name.into(),
            cause,
        }
    }

    /// Creates a validation error.
    pub fn validation_failed(
        field_name: impl Into<String>,
        value: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::ValidationFailed {
            field_name: field_name.into(),
            value: value.into(),
            reason: reason.into(),
        }
    }

    /// Creates a deserialization error.
    pub fn deserialize_error(msg: impl Into<String>) -> Self {
        Self::DeserializationError(msg.into())
    }
}

/// Error that can occur during vector deserialization.
#[derive(Debug, thiserror::Error)]
pub enum VectorError {
    /// Vector size exceeds maximum allowed.
    #[error("Vector size {size} exceeds maximum {max}")]
    TooLarge {
        /// Actual vector size.
        size: usize,
        /// Maximum allowed size.
        max: usize,
    },

    /// Invalid vector prefix.
    #[error("Vector prefix invalid: {0}")]
    InvalidPrefix(u32),
}

impl VectorError {
    /// Creates a "too large" error.
    pub fn too_large(size: usize, max: usize) -> Self {
        Self::TooLarge { size, max }
    }

    /// Creates an invalid prefix error.
    pub fn invalid_prefix(prefix: u32) -> Self {
        Self::InvalidPrefix(prefix)
    }
}

/// Convert `TlError` to `rustgram_types::TypeError` for compatibility.
impl From<TlError> for rustgram_types::TypeError {
    fn from(err: TlError) -> Self {
        rustgram_types::TypeError::DeserializationError(err.to_string())
    }
}

/// Convert `rustgram_types::TypeError` to `TlError` for compatibility.
impl From<rustgram_types::TypeError> for TlError {
    fn from(err: rustgram_types::TypeError) -> Self {
        TlError::DeserializationError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_constructor_error() {
        let err = TlError::unknown_constructor(vec![0x12345678, 0x87654321], 0xAAAAAAAA, "Photo");
        assert!(matches!(err, TlError::UnknownConstructor { .. }));
        // The actual format uses Debug formatting for the Vec
        assert!(err
            .to_string()
            .contains("Unknown constructor ID: 0xaaaaaaaa"));
        assert!(err.to_string().contains("expected one of:"));
    }

    #[test]
    fn test_unexpected_eof_error() {
        let err = TlError::unexpected_eof(10, 5, "PhotoSize");
        assert!(matches!(err, TlError::UnexpectedEof { .. }));
        assert_eq!(
            err.to_string(),
            "Unexpected EOF: requested 10 bytes, 5 remaining for type 'PhotoSize'"
        );
    }

    #[test]
    fn test_validation_error() {
        let err = TlError::validation_failed("id", "-1", "ID must be positive");
        assert!(matches!(err, TlError::ValidationFailed { .. }));
        assert_eq!(
            err.to_string(),
            "Validation failed for field 'id': value=-1, reason=ID must be positive"
        );
    }

    #[test]
    fn test_vector_too_large_error() {
        let err = VectorError::too_large(10000, 1000);
        assert!(matches!(err, VectorError::TooLarge { .. }));
        assert_eq!(err.to_string(), "Vector size 10000 exceeds maximum 1000");
    }

    #[test]
    fn test_vector_invalid_prefix_error() {
        let err = VectorError::invalid_prefix(0x12345678);
        assert!(matches!(err, VectorError::InvalidPrefix(_)));
        assert_eq!(err.to_string(), "Vector prefix invalid: 305419896");
    }

    #[test]
    fn test_vector_error_conversion() {
        let vec_err = VectorError::too_large(10000, 1000);
        let tl_err: TlError = vec_err.into();
        assert!(matches!(tl_err, TlError::VectorError(_)));
    }
}
