// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Synchronous Requests
//!
//! Synchronous request tracking for TDLib.
//!
//! ## Overview
//!
//! This module provides functionality for handling synchronous TDLib requests
//! that can be executed immediately without waiting for network responses.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_synchronous_requests::SynchronousRequests;
//!
//! // Check if a request type is synchronous
//! let is_sync = SynchronousRequests::is_synchronous_request("getTextEntities");
//! assert!(is_sync);
//! ```

use std::collections::HashSet;
use std::sync::LazyLock;

/// Synchronous TDLib requests that can be executed immediately.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `SynchronousRequests` class.
///
/// # Example
///
/// ```rust
/// use rustgram_synchronous_requests::SynchronousRequests;
///
/// assert!(SynchronousRequests::is_synchronous_request("getTextEntities"));
/// assert!(!SynchronousRequests::is_synchronous_request("sendMessage"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct SynchronousRequests;

/// Set of synchronous request names.
static SYNCHRONOUS_REQUESTS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "searchQuote",
        "getTextEntities",
        "parseTextEntities",
        "parseMarkdown",
        "getMarkdownText",
        "searchStringsByPrefix",
        "checkQuickReplyShortcutName",
        "getCountryFlagEmoji",
        "getFileMimeType",
        "getFileExtension",
        "cleanFileName",
        "getLanguagePackString",
        "getPhoneNumberInfoSync",
        "getChatFolderDefaultIconName",
        "getJsonValue",
        "getJsonString",
        "getThemeParametersJsonString",
        "getPushReceiverId",
        "setLogStream",
        "getLogStream",
        "setLogVerbosityLevel",
        "getLogVerbosityLevel",
        "getLogTags",
        "setLogTagVerbosityLevel",
        "getLogTagVerbosityLevel",
        "addLogMessage",
        "testReturnError",
    ]
    .into_iter()
    .collect()
});

impl SynchronousRequests {
    /// Checks if a request type is synchronous.
    ///
    /// # Arguments
    ///
    /// * `request_name` - The name of the request function
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_synchronous_requests::SynchronousRequests;
    ///
    /// assert!(SynchronousRequests::is_synchronous_request("getTextEntities"));
    /// assert!(!SynchronousRequests::is_synchronous_request("sendMessage"));
    /// ```
    #[must_use]
    pub fn is_synchronous_request(request_name: &str) -> bool {
        // Check if it's a getOption request
        if request_name == "getOption" {
            return true; // Simplified: in TDLib this depends on the option name
        }

        SYNCHRONOUS_REQUESTS.contains(request_name)
    }

    /// Returns all synchronous request names.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_synchronous_requests::SynchronousRequests;
    ///
    /// let all = SynchronousRequests::all_synchronous_requests();
    /// assert!(all.contains(&"getTextEntities"));
    /// ```
    #[must_use]
    pub fn all_synchronous_requests() -> Vec<&'static str> {
        SYNCHRONOUS_REQUESTS.iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_synchronous_request_true() {
        assert!(SynchronousRequests::is_synchronous_request(
            "getTextEntities"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "parseTextEntities"
        ));
        assert!(SynchronousRequests::is_synchronous_request("parseMarkdown"));
        assert!(SynchronousRequests::is_synchronous_request(
            "getMarkdownText"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "searchStringsByPrefix"
        ));
    }

    #[test]
    fn test_is_synchronous_request_false() {
        assert!(!SynchronousRequests::is_synchronous_request("sendMessage"));
        assert!(!SynchronousRequests::is_synchronous_request("getChats"));
        assert!(!SynchronousRequests::is_synchronous_request("getUser"));
        assert!(!SynchronousRequests::is_synchronous_request(
            "unknownMethod"
        ));
    }

    #[test]
    fn test_is_synchronous_request_logging() {
        assert!(SynchronousRequests::is_synchronous_request("setLogStream"));
        assert!(SynchronousRequests::is_synchronous_request("getLogStream"));
        assert!(SynchronousRequests::is_synchronous_request(
            "setLogVerbosityLevel"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getLogVerbosityLevel"
        ));
        assert!(SynchronousRequests::is_synchronous_request("getLogTags"));
        assert!(SynchronousRequests::is_synchronous_request(
            "setLogTagVerbosityLevel"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getLogTagVerbosityLevel"
        ));
        assert!(SynchronousRequests::is_synchronous_request("addLogMessage"));
    }

    #[test]
    fn test_is_synchronous_request_file_operations() {
        assert!(SynchronousRequests::is_synchronous_request(
            "getFileMimeType"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getFileExtension"
        ));
        assert!(SynchronousRequests::is_synchronous_request("cleanFileName"));
    }

    #[test]
    fn test_is_synchronous_request_json() {
        assert!(SynchronousRequests::is_synchronous_request("getJsonValue"));
        assert!(SynchronousRequests::is_synchronous_request("getJsonString"));
        assert!(SynchronousRequests::is_synchronous_request(
            "getThemeParametersJsonString"
        ));
    }

    #[test]
    fn test_is_synchronous_request_get_option() {
        assert!(SynchronousRequests::is_synchronous_request("getOption"));
    }

    #[test]
    fn test_all_synchronous_requests() {
        let all = SynchronousRequests::all_synchronous_requests();
        assert!(all.len() > 20);
        assert!(all.contains(&"getTextEntities"));
        assert!(all.contains(&"parseMarkdown"));
    }

    #[test]
    fn test_all_synchronous_requests_count() {
        let all = SynchronousRequests::all_synchronous_requests();
        // Verify we have a reasonable number of synchronous requests
        assert!(all.len() >= 20);
    }

    #[test]
    fn test_case_sensitive() {
        assert!(SynchronousRequests::is_synchronous_request(
            "getTextEntities"
        ));
        assert!(!SynchronousRequests::is_synchronous_request(
            "gettextentities"
        ));
        assert!(!SynchronousRequests::is_synchronous_request(
            "GETTEXTENTITIES"
        ));
    }

    #[test]
    fn test_empty_string() {
        assert!(!SynchronousRequests::is_synchronous_request(""));
    }

    #[test]
    fn test_miscellaneous_requests() {
        assert!(SynchronousRequests::is_synchronous_request("searchQuote"));
        assert!(SynchronousRequests::is_synchronous_request(
            "checkQuickReplyShortcutName"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getCountryFlagEmoji"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getLanguagePackString"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getPhoneNumberInfoSync"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getChatFolderDefaultIconName"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "getPushReceiverId"
        ));
        assert!(SynchronousRequests::is_synchronous_request(
            "testReturnError"
        ));
    }
}
