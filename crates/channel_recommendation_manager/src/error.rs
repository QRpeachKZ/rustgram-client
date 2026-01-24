// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for ChannelRecommendationManager

use rustgram_types::DialogId;
use thiserror::Error;

/// Errors that can occur in ChannelRecommendationManager operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Dialog not found
    #[error("Dialog not found: {0:?}")]
    DialogNotFound(DialogId),

    /// Invalid dialog type (not a channel)
    #[error("Invalid dialog type: expected channel")]
    InvalidDialogType,

    /// Channel not found
    #[error("Channel not found")]
    ChannelNotFound,

    /// Recommendations not available
    #[error("Recommendations not available")]
    NotAvailable,
}

/// Result type for ChannelRecommendationManager operations
pub type Result<T> = std::result::Result<T, Error>;
