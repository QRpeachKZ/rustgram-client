// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # TDLib Callback Interface
//!
//! Callback interface for low-level interaction with TDLib.
//!
//! ## Overview
//!
//! This module provides the [`TdCallback`] trait, which defines the interface
//! for receiving responses and updates from TDLib. Implementations of this
//! trait can handle asynchronous results and errors from TDLib operations.
//!
//! ## Example
//!
//! ```rust,ignore
//! use rustgram_td_callback::TdCallback;
//!
//! struct MyCallback;
//!
//! impl TdCallback for MyCallback {
//!     fn on_result(&mut self, id: u64, result: Option<Object>) {
//!         println!("Received result for request {}: {:?}", id, result);
//!     }
//!
//!     fn on_error(&mut self, id: u64, error: Option<Error>) {
//!         eprintln!("Received error for request {}: {:?}", id, error);
//!     }
//! }
//! ```

use std::fmt;

/// Callback interface for low-level interaction with TDLib.
///
/// This trait defines how implementations receive responses to TDLib requests
/// and incoming updates. Each request to TDLib has a unique identifier, and
/// responses are delivered through the [`on_result`] or [`on_error`] methods.
/// Updates from TDLib have an ID of 0.
///
/// # TDLib Correspondence
///
/// This corresponds to TDLib's `td::TdCallback` interface.
///
/// # Example
///
/// ```rust,ignore
/// use rustgram_td_callback::TdCallback;
///
/// struct MyCallback {
///     // Your fields here
/// }
///
/// impl TdCallback for MyCallback {
///     fn on_result(&mut self, id: u64, result: Option<Object>) {
///         if id == 0 {
///             // This is an update
///         } else {
///             // This is a response to request `id`
///         }
///     }
///
///     fn on_error(&mut self, id: u64, error: Option<Error>) {
///         eprintln!("Request {} failed: {:?}", id, error);
///     }
/// }
/// ```
pub trait TdCallback: Send {
    /// Called for every answer to a request made to TDLib and for every
    /// incoming update.
    ///
    /// # Arguments
    ///
    /// * `id` - Request identifier, or 0 for incoming updates
    /// * `result` - Answer to the TDLib request or an incoming update
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use rustgram_td_callback::TdCallback;
    /// # struct MyCallback;
    /// # impl TdCallback for MyCallback {
    /// fn on_result(&mut self, id: u64, result: Option<Object>) {
    ///     if id == 0 {
    ///         println!("Received update: {:?}", result);
    ///     } else {
    ///         println!("Request {} completed: {:?}", id, result);
    ///     }
    /// }
    /// # }
    /// ```
    fn on_result(&mut self, id: u64, result: Option<Box<dyn fmt::Display + Send>>);

    /// Called for every unsuccessful request made to TDLib.
    ///
    /// # Arguments
    ///
    /// * `id` - Request identifier
    /// * `error` - Error information for the failed request
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use rustgram_td_callback::TdCallback;
    /// # struct MyCallback;
    /// # impl TdCallback for MyCallback {
    /// fn on_error(&mut self, id: u64, error: Option<Error>) {
    ///     eprintln!("Request {} failed: {:?}", id, error);
    /// }
    /// # }
    /// ```
    fn on_error(&mut self, id: u64, error: Option<Box<dyn fmt::Display + Send>>);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCallback {
        results: Vec<(u64, String)>,
        errors: Vec<(u64, String)>,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                results: Vec::new(),
                errors: Vec::new(),
            }
        }
    }

    impl TdCallback for TestCallback {
        fn on_result(&mut self, id: u64, result: Option<Box<dyn fmt::Display + Send>>) {
            let message = result.as_ref().map(|d| d.to_string()).unwrap_or_default();
            self.results.push((id, message));
        }

        fn on_error(&mut self, id: u64, error: Option<Box<dyn fmt::Display + Send>>) {
            let message = error.as_ref().map(|d| d.to_string()).unwrap_or_default();
            self.errors.push((id, message));
        }
    }

    #[test]
    fn test_callback_on_result() {
        let mut callback = TestCallback::new();

        struct DisplayString(String);
        impl fmt::Display for DisplayString {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        callback.on_result(123, Some(Box::new(DisplayString("success".to_string()))));
        assert_eq!(callback.results.len(), 1);
        assert_eq!(callback.results[0].0, 123);
        assert_eq!(callback.results[0].1, "success");
    }

    #[test]
    fn test_callback_on_error() {
        let mut callback = TestCallback::new();

        struct DisplayString(String);
        impl fmt::Display for DisplayString {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        callback.on_error(456, Some(Box::new(DisplayString("error".to_string()))));
        assert_eq!(callback.errors.len(), 1);
        assert_eq!(callback.errors[0].0, 456);
        assert_eq!(callback.errors[0].1, "error");
    }

    #[test]
    fn test_callback_update_id_zero() {
        let mut callback = TestCallback::new();

        callback.on_result(0, None);
        assert_eq!(callback.results.len(), 1);
        assert_eq!(callback.results[0].0, 0);
    }

    #[test]
    fn test_callback_none_result() {
        let mut callback = TestCallback::new();

        callback.on_result(789, None);
        assert_eq!(callback.results.len(), 1);
        assert_eq!(callback.results[0].0, 789);
        assert_eq!(callback.results[0].1, "");
    }

    #[test]
    fn test_callback_none_error() {
        let mut callback = TestCallback::new();

        callback.on_error(999, None);
        assert_eq!(callback.errors.len(), 1);
        assert_eq!(callback.errors[0].0, 999);
        assert_eq!(callback.errors[0].1, "");
    }

    #[test]
    fn test_callback_multiple_results() {
        let mut callback = TestCallback::new();

        callback.on_result(1, None);
        callback.on_result(2, None);
        callback.on_result(3, None);

        assert_eq!(callback.results.len(), 3);
        assert_eq!(callback.results[0].0, 1);
        assert_eq!(callback.results[1].0, 2);
        assert_eq!(callback.results[2].0, 3);
    }

    #[test]
    fn test_callback_multiple_errors() {
        let mut callback = TestCallback::new();

        callback.on_error(1, None);
        callback.on_error(2, None);

        assert_eq!(callback.errors.len(), 2);
        assert_eq!(callback.errors[0].0, 1);
        assert_eq!(callback.errors[1].0, 2);
    }

    #[test]
    fn test_callback_mixed_results_and_errors() {
        let mut callback = TestCallback::new();

        callback.on_result(1, None);
        callback.on_error(2, None);
        callback.on_result(3, None);
        callback.on_error(4, None);

        assert_eq!(callback.results.len(), 2);
        assert_eq!(callback.errors.len(), 2);
        assert_eq!(callback.results[0].0, 1);
        assert_eq!(callback.errors[0].0, 2);
        assert_eq!(callback.results[1].0, 3);
        assert_eq!(callback.errors[1].0, 4);
    }
}
