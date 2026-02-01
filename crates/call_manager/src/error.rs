// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for CallManager

use rustgram_call_id::CallId;
use thiserror::Error;

/// Errors that can occur in CallManager operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Call not found
    #[error("Call {0} not found")]
    CallNotFound(CallId),

    /// Call with this user already exists
    #[error("Call already exists: {0}")]
    CallAlreadyExists(CallId),

    /// Invalid call state for this operation
    #[error("Invalid call state for this operation")]
    InvalidState,

    /// Invalid rating value (must be 1-5)
    #[error("Invalid rating: must be between 1 and 5")]
    InvalidRating,
}

/// Result type for CallManager operations
pub type Result<T> = std::result::Result<T, Error>;
