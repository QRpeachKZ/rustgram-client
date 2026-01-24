// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for InlineMessageManager

use thiserror::Error;

/// Errors that can occur in InlineMessageManager operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Message not found
    #[error("Inline message {0} not found")]
    MessageNotFound(String),
}

/// Result type for InlineMessageManager operations
pub type Result<T> = std::result::Result<T, Error>;
