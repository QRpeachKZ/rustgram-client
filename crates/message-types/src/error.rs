// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0

//! Error types for message operations.

use thiserror::Error;

/// Errors that can occur during message operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// Message not found.
    #[error("message not found")]
    NotFound,

    /// Invalid message ID.
    #[error("invalid message ID: {0}")]
    InvalidId(String),

    /// Message is not editable.
    #[error("message is not editable")]
    NotEditable,

    /// Message is not deletable.
    #[error("message is not deletable")]
    NotDeletable,

    /// Send operation failed.
    #[error("send failed: {0}")]
    SendFailed(String),

    /// Edit operation failed.
    #[error("edit failed: {0}")]
    EditFailed(String),

    /// Network error.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Validation error.
    #[error("validation error: {0}")]
    ValidationError(String),
}

/// Result type for message operations.
pub type Result<T> = std::result::Result<T, Error>;
