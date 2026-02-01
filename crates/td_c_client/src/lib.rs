// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram TdCClient - C client interface wrapper for Telegram MTProto.
//!
//! This module provides type definitions for the TDLib C client interface.
//! The actual FFI calls are only available when the TDLib C library is linked.
//!
//! ## Overview
//!
//! The C client interface provides a synchronous C API for interacting with TDLib:
//!
//! - [`TdRequest`] - Request type for sending to TDLib
//! - [`TdResponse`] - Response type received from TDLib
//! - [`TdFunction`] - Opaque function type
//! - [`TdObject`] - Opaque object type
//!
//! ## TDLib Correspondence
//!
//! | Rust type | C type | File |
//! |-----------|-------|------|
//! | [`TdRequest`] | `struct TdRequest` | `td_c_client.h:20-23` |
//! | [`TdResponse`] | `struct TdResponse` | `td_c_client.h:25-29` |
//!
//! ## Examples
//!
//! ```no_run
//! use rustgram_td_c_client::{TdRequest, TdFunction};
//!
//! // Create a request (would be sent via actual TDLib C API)
//! let request = TdRequest {
//!     request_id: 1,
//!     function: std::ptr::null_mut(),
//! };
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use libc::c_int;

/// A request to be sent to a TDLib client.
///
/// This is a raw FFI type that corresponds to the `TdRequest` struct in TDLib.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TdRequest {
    /// The request identifier (must be > 0)
    pub request_id: i64,
    /// Pointer to the request function/object
    pub function: *mut TdFunction,
}

/// A response received from a TDLib client.
///
/// This is a raw FFI type that corresponds to the `TdResponse` struct in TDLib.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TdResponse {
    /// The request identifier
    pub request_id: i64,
    /// The client ID
    pub client_id: c_int,
    /// Pointer to the response object
    pub object: *mut TdObject,
}

/// Opaque TDLib function type.
///
/// This is an opaque type representing a TDLib function/request.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TdFunction {
    _private: [u8; 0],
}

/// Opaque TDLib object type.
///
/// This is an opaque type representing a TDLib object/response.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TdObject {
    _private: [u8; 0],
}

/// Errors that can occur when working with the C client interface.
#[derive(thiserror::Error, Debug)]
pub enum TdCClientError {
    /// Invalid client ID
    #[error("Invalid client ID: {0}")]
    InvalidClientId(c_int),

    /// Null pointer received
    #[error("Null pointer encountered")]
    NullPointer,

    /// Invalid UTF-8 string
    #[error("Invalid UTF-8 string")]
    InvalidUtf8,
}

/// Result type for C client operations.
pub type Result<T> = std::result::Result<T, TdCClientError>;

impl TdResponse {
    /// Checks if the response object is null.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_td_c_client::TdResponse;
    ///
    /// let response = TdResponse {
    ///     request_id: 0,
    ///     client_id: 0,
    ///     object: std::ptr::null_mut(),
    /// };
    /// assert!(response.is_null());
    /// ```
    #[must_use]
    pub fn is_null(self) -> bool {
        self.object.is_null()
    }
}

// Implement Send and Sync for these types
// Safety: The C client interface is designed to be thread-safe for send operations
unsafe impl Send for TdRequest {}
unsafe impl Send for TdResponse {}
unsafe impl Sync for TdRequest {}

// Note: TdResponse is not Sync because only one thread should call receive at a time

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-td-c-client";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-td-c-client");
    }

    #[test]
    fn test_td_request_size() {
        // TdRequest should be representable in C
        assert_eq!(
            std::mem::size_of::<TdRequest>(),
            std::mem::size_of::<i64>() * 2
        );
    }

    #[test]
    fn test_td_response_size() {
        // TdResponse should be representable in C
        // The actual size may include padding for alignment
        assert!(std::mem::size_of::<TdResponse>() >= std::mem::size_of::<i64>());
        assert!(std::mem::size_of::<TdResponse>() >= std::mem::size_of::<c_int>());
        assert!(std::mem::size_of::<TdResponse>() >= std::mem::size_of::<*mut ()>());
    }

    #[test]
    fn test_response_null_check() {
        let response = TdResponse {
            request_id: 0,
            client_id: 0,
            object: std::ptr::null_mut(),
        };
        assert!(response.is_null());
    }

    #[test]
    fn test_response_non_null_check() {
        let dummy = std::ptr::NonNull::dangling().as_ptr();
        let response = TdResponse {
            request_id: 1,
            client_id: 1,
            object: dummy,
        };
        assert!(!response.is_null());
    }
}
