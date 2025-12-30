// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Error types for log event operations

use std::io;

/// Result type for log event operations
pub type Result<T> = std::result::Result<T, LogEventError>;

/// Errors that can occur during log event operations
#[derive(Debug, thiserror::Error)]
pub enum LogEventError {
    /// Invalid version number encountered
    #[error("Invalid version: {0}")]
    InvalidVersion(i32),

    /// Invalid flags bit pattern
    #[error("Invalid flags")]
    InvalidFlags,

    /// Unknown event type encountered
    #[error("Unknown event type: {0}")]
    UnknownEventType(u32),

    /// Generic parse error with message
    #[error("Parse error: {0}")]
    ParseError(String),

    /// I/O error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Unexpected end of data during parsing
    #[error("Unexpected end of data")]
    UnexpectedEnd,

    /// Encrypted input file magic mismatch
    ///
    /// Occurs when the magic number in the encrypted input file doesn't match
    /// the expected value.
    #[error("EncryptedInputFile magic mismatch: expected 0x{expected:08x}, got 0x{got:08x}")]
    MagicMismatch {
        /// The expected magic number value
        expected: u32,
        /// The actual magic number value found
        got: u32,
    },

    /// Invalid data length
    ///
    /// Occurs when data has an unexpected length during parsing.
    #[error("Invalid length: expected {expected}, got {got}")]
    InvalidLength {
        /// The expected length value
        expected: usize,
        /// The actual length value found
        got: usize,
    },
}

impl LogEventError {
    /// Creates a parse error with a message
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::ParseError(msg.into())
    }
}
