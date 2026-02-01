// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the misc module.

use thiserror::Error;

/// Errors that can occur in misc operations.
#[derive(Error, Debug)]
pub enum MiscError {
    /// Invalid input string (from venue crate)
    #[error("Invalid input string: {0}")]
    InvalidInputString(String),

    /// Invalid username format
    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    /// Username not allowed (reserved or too short)
    #[error("Username not allowed: {0}")]
    UsernameNotAllowed(String),

    /// Invalid currency amount
    #[error("Invalid currency amount: {0}")]
    InvalidCurrencyAmount(i64),

    /// Invalid bot language code
    #[error("Invalid bot language code: {0}")]
    InvalidBotLanguageCode(String),

    /// Invalid color value
    #[error("Invalid color value: {0}")]
    InvalidColor(i32),
}

/// Result type for misc operations.
pub type Result<T> = std::result::Result<T, MiscError>;
