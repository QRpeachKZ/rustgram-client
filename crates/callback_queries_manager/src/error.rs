// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for CallbackQueriesManager

use thiserror::Error;

/// Errors that can occur in CallbackQueriesManager operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid callback query ID
    #[error("Invalid callback query ID: {0}")]
    InvalidCallbackQueryId(i64),

    /// Invalid message ID
    #[error("Invalid message ID")]
    InvalidMessageId,

    /// Invalid payload
    #[error("Invalid callback query payload")]
    InvalidPayload,

    /// Unauthorized operation
    #[error("Unauthorized operation")]
    Unauthorized,
}

/// Result type for CallbackQueriesManager operations
pub type Result<T> = std::result::Result<T, Error>;
