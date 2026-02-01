// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for GroupCallManager

use thiserror::Error;

/// Errors that can occur in GroupCallManager operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Call not found
    #[error("Group call {0} not found")]
    CallNotFound(i64),

    /// Call with this dialog already exists
    #[error("Call already exists: {0}")]
    CallAlreadyExists(i64),

    /// Already joined this call
    #[error("Already joined this call")]
    AlreadyJoined,

    /// Invalid state for this operation
    #[error("Invalid state for this operation")]
    InvalidState,
}

/// Result type for GroupCallManager operations
pub type Result<T> = std::result::Result<T, Error>;
