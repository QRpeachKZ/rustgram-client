// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the link manager.

use thiserror::Error;

/// Result type for link manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the link manager.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    /// Invalid link provided.
    #[error("invalid link: {0}")]
    InvalidLink(String),

    /// Link is too long.
    #[error("link too long: {0} bytes")]
    LinkTooLong(usize),

    /// Empty link provided.
    #[error("empty link")]
    EmptyLink,

    /// Unsupported link type.
    #[error("unsupported link type: {0}")]
    UnsupportedLinkType(String),

    /// Invalid URL format.
    #[error("invalid URL format: {0}")]
    InvalidUrlFormat(String),

    /// Missing required parameter.
    #[error("missing required parameter: {0}")]
    MissingParameter(String),

    /// Network error occurred.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Preview generation failed.
    #[error("preview generation failed: {0}")]
    PreviewError(String),

    /// Invalid dialog invite hash.
    #[error("invalid dialog invite hash: {0}")]
    InvalidInviteHash(String),

    /// Invalid username.
    #[error("invalid username: {0}")]
    InvalidUsername(String),

    /// Internal error.
    #[error("internal error: {0}")]
    InternalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidLink("not a url".to_string());
        assert!(err.to_string().contains("invalid link"));
        assert!(err.to_string().contains("not a url"));
    }

    #[test]
    fn test_error_eq() {
        let err1 = Error::EmptyLink;
        let err2 = Error::EmptyLink;
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_link_too_long() {
        let err = Error::LinkTooLong(100000);
        assert!(err.to_string().contains("too long"));
        assert!(err.to_string().contains("100000"));
    }

    #[test]
    fn test_error_invalid_username() {
        let err = Error::InvalidUsername("invalid_user!".to_string());
        assert!(err.to_string().contains("invalid username"));
        assert!(err.to_string().contains("invalid_user!"));
    }

    #[test]
    fn test_error_missing_parameter() {
        let err = Error::MissingParameter("domain".to_string());
        assert!(err.to_string().contains("missing required parameter"));
        assert!(err.to_string().contains("domain"));
    }
}
