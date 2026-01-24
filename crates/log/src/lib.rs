// Copyright 2024 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Log Types
//!
//! Types for TDLib logging interface.
//!
//! ## Overview
//!
//! This module provides type definitions for managing TDLib's internal logging:
//! - [`LogVerbosityLevel`] - Verbosity level for logging
//! - [`LogTags`] - Tags for filtering log messages
//! - [`LogStream`] - Output destination for logs
//!
//! ## TDLib Correspondence
//!
//! | Rust Type | TDLib Type | TL Schema |
//! |-----------|-----------|-----------|
//! | [`LogVerbosityLevel`] | `logVerbosityLevel` | `td_api.tl:10003` |
//! | [`LogTags`] | `logTags` | `td_api.tl:10006` |
//! | [`LogStream::Default`] | `logStreamDefault` | `td_api.tl:9990` |
//! | [`LogStream::File`] | `logStreamFile` | `td_api.tl:9996` |
//! | [`LogStream::Empty`] | `logStreamEmpty` | `td_api.tl:9999` |
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_log::{LogVerbosityLevel, LogTags, LogStream};
//!
//! // Set verbosity level
//! let level = LogVerbosityLevel::new(5);
//!
//! // Set log tags
//! let tags = LogTags::new(vec!["tdlib".to_string()]);
//!
//! // Use default log stream
//! let stream = LogStream::default();
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Log verbosity level.
///
/// Controls the verbosity of TDLib logging:
/// - 0: Fatal errors only
/// - 1: Errors
/// - 2: Warnings and debug warnings
/// - 3: Informational
/// - 4: Debug
/// - 5: Verbose debug (default)
/// - >5: Even more verbose logging (up to 1024)
///
/// # Examples
///
/// ```
/// use rustgram_log::LogVerbosityLevel;
///
/// let level = LogVerbosityLevel::new(5);
/// assert_eq!(level.value(), 5);
///
/// let fatal = LogVerbosityLevel::fatal();
/// assert_eq!(fatal.value(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LogVerbosityLevel {
    /// The verbosity level value (0-1024)
    value: i32,
}

impl Default for LogVerbosityLevel {
    fn default() -> Self {
        Self::TDLIB_DEFAULT
    }
}

impl LogVerbosityLevel {
    /// Minimum verbosity level (fatal errors only).
    pub const MIN: i32 = 0;

    /// Maximum verbosity level.
    pub const MAX: i32 = 1024;

    /// TDLib's default verbosity level.
    pub const TDLIB_DEFAULT: LogVerbosityLevel = LogVerbosityLevel { value: 5 };

    /// Fatal errors only.
    pub const FATAL: LogVerbosityLevel = LogVerbosityLevel { value: 0 };

    /// Errors.
    pub const ERROR: LogVerbosityLevel = LogVerbosityLevel { value: 1 };

    /// Warnings and debug warnings.
    pub const WARNING: LogVerbosityLevel = LogVerbosityLevel { value: 2 };

    /// Informational.
    pub const INFO: LogVerbosityLevel = LogVerbosityLevel { value: 3 };

    /// Debug.
    pub const DEBUG: LogVerbosityLevel = LogVerbosityLevel { value: 4 };

    /// Verbose debug.
    pub const VERBOSE: LogVerbosityLevel = LogVerbosityLevel { value: 5 };

    /// Creates a new log verbosity level.
    ///
    /// The value will be clamped to the valid range [0, 1024].
    ///
    /// # Arguments
    ///
    /// * `value` - The verbosity level (0-1024)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::new(5);
    /// assert_eq!(level.value(), 5);
    ///
    /// let clamped = LogVerbosityLevel::new(2000);
    /// assert_eq!(clamped.value(), 1024);
    /// ```
    pub fn new(value: i32) -> Self {
        Self {
            value: value.clamp(Self::MIN, Self::MAX),
        }
    }

    /// Creates a fatal-only verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::fatal();
    /// assert_eq!(level.value(), 0);
    /// ```
    pub fn fatal() -> Self {
        Self::FATAL
    }

    /// Creates an error verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::error();
    /// assert_eq!(level.value(), 1);
    /// ```
    pub fn error() -> Self {
        Self::ERROR
    }

    /// Creates a warning verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::warning();
    /// assert_eq!(level.value(), 2);
    /// ```
    pub fn warning() -> Self {
        Self::WARNING
    }

    /// Creates an info verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::info();
    /// assert_eq!(level.value(), 3);
    /// ```
    pub fn info() -> Self {
        Self::INFO
    }

    /// Creates a debug verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::debug();
    /// assert_eq!(level.value(), 4);
    /// ```
    pub fn debug() -> Self {
        Self::DEBUG
    }

    /// Creates a verbose verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::verbose();
    /// assert_eq!(level.value(), 5);
    /// ```
    pub fn verbose() -> Self {
        Self::VERBOSE
    }

    /// Returns the verbosity level value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let level = LogVerbosityLevel::new(5);
    /// assert_eq!(level.value(), 5);
    /// ```
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Sets the verbosity level value.
    ///
    /// The value will be clamped to the valid range [0, 1024].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// let mut level = LogVerbosityLevel::new(5);
    /// level.set_value(10);
    /// assert_eq!(level.value(), 10);
    /// ```
    pub fn set_value(&mut self, value: i32) {
        self.value = value.clamp(Self::MIN, Self::MAX);
    }

    /// Returns `true` if this level will log fatal errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// assert!(LogVerbosityLevel::fatal().logs_fatal());
    /// assert!(LogVerbosityLevel::error().logs_fatal());
    /// ```
    pub fn logs_fatal(&self) -> bool {
        self.value >= 0
    }

    /// Returns `true` if this level will log errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// assert!(!LogVerbosityLevel::fatal().logs_errors());
    /// assert!(LogVerbosityLevel::error().logs_errors());
    /// ```
    pub fn logs_errors(&self) -> bool {
        self.value >= 1
    }

    /// Returns `true` if this level will log warnings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// assert!(!LogVerbosityLevel::error().logs_warnings());
    /// assert!(LogVerbosityLevel::warning().logs_warnings());
    /// ```
    pub fn logs_warnings(&self) -> bool {
        self.value >= 2
    }

    /// Returns `true` if this level will log info.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// assert!(!LogVerbosityLevel::warning().logs_info());
    /// assert!(LogVerbosityLevel::info().logs_info());
    /// ```
    pub fn logs_info(&self) -> bool {
        self.value >= 3
    }

    /// Returns `true` if this level will log debug messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// assert!(!LogVerbosityLevel::info().logs_debug());
    /// assert!(LogVerbosityLevel::debug().logs_debug());
    /// ```
    pub fn logs_debug(&self) -> bool {
        self.value >= 4
    }

    /// Returns `true` if this level will log verbose messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// assert!(!LogVerbosityLevel::debug().logs_verbose());
    /// assert!(LogVerbosityLevel::verbose().logs_verbose());
    /// ```
    pub fn logs_verbose(&self) -> bool {
        self.value >= 5
    }

    /// Returns the description of this verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogVerbosityLevel;
    ///
    /// assert_eq!(LogVerbosityLevel::fatal().description(), "Fatal");
    /// assert_eq!(LogVerbosityLevel::verbose().description(), "Verbose");
    /// ```
    pub fn description(&self) -> &str {
        match self.value {
            0 => "Fatal",
            1 => "Error",
            2 => "Warning",
            3 => "Info",
            4 => "Debug",
            5 => "Verbose",
            _ => "Custom",
        }
    }
}

impl fmt::Display for LogVerbosityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (level {})", self.description(), self.value)
    }
}

/// Log tags for filtering log messages.
///
/// Tags can be used to filter log messages by category.
///
/// # Examples
///
/// ```
/// use rustgram_log::LogTags;
///
/// let tags = LogTags::new(vec!["tdlib".to_string(), "network".to_string()]);
/// assert_eq!(tags.tags().len(), 2);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LogTags {
    /// The log tags
    tags: Vec<String>,
}

impl LogTags {
    /// Creates new log tags.
    ///
    /// # Arguments
    ///
    /// * `tags` - Vector of tag strings
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// let tags = LogTags::new(vec!["tdlib".to_string()]);
    /// assert_eq!(tags.tags().len(), 1);
    /// ```
    pub fn new(tags: Vec<String>) -> Self {
        Self { tags }
    }

    /// Returns the tags.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// let tags = LogTags::new(vec!["tdlib".to_string()]);
    /// assert_eq!(tags.tags(), &["tdlib"]);
    /// ```
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Returns `true` if there are no tags.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// assert!(LogTags::default().is_empty());
    /// assert!(!LogTags::new(vec!["tag".to_string()]).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// Returns the number of tags.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// let tags = LogTags::new(vec!["a".to_string(), "b".to_string()]);
    /// assert_eq!(tags.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Adds a tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// let mut tags = LogTags::default();
    /// tags.add("tdlib");
    /// assert_eq!(tags.len(), 1);
    /// ```
    pub fn add(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }

    /// Removes a tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// let mut tags = LogTags::new(vec!["tdlib".to_string()]);
    /// tags.remove("tdlib");
    /// assert!(tags.is_empty());
    /// ```
    pub fn remove(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Returns `true` if the tags contain the specified tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// let tags = LogTags::new(vec!["tdlib".to_string()]);
    /// assert!(tags.contains("tdlib"));
    /// assert!(!tags.contains("other"));
    /// ```
    pub fn contains(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Clears all tags.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogTags;
    ///
    /// let mut tags = LogTags::new(vec!["tdlib".to_string()]);
    /// tags.clear();
    /// assert!(tags.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.tags.clear();
    }
}

impl fmt::Display for LogTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.tags.join(", "))
    }
}

/// Log stream destination.
///
/// Controls where TDLib writes log messages.
///
/// # Examples
///
/// ```
/// use rustgram_log::LogStream;
///
/// let default = LogStream::default();
/// assert!(matches!(default, LogStream::Default));
///
/// let file = LogStream::file("/path/to/log.txt", 10 * 1024 * 1024, false);
/// assert!(matches!(file, LogStream::File { .. }));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogStream {
    /// TDLib: `logStreamDefault`
    ///
    /// Logs to stderr or OS-specific log.
    #[default]
    Default,

    /// TDLib: `logStreamFile`
    ///
    /// Logs to a file with auto-rotation.
    File {
        /// Path to the log file
        path: PathBuf,
        /// Maximum file size before rotation (bytes)
        max_file_size: i64,
        /// Whether to redirect stderr to the file
        redirect_stderr: bool,
    },

    /// TDLib: `logStreamEmpty`
    ///
    /// Disables logging.
    Empty,
}

impl LogStream {
    /// Creates a file log stream.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the log file
    /// * `max_file_size` - Maximum file size before rotation (bytes)
    /// * `redirect_stderr` - Whether to redirect stderr to the file
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogStream;
    ///
    /// let stream = LogStream::file("/tmp/tdlib.log", 10 * 1024 * 1024, false);
    /// ```
    pub fn file(path: &str, max_file_size: i64, redirect_stderr: bool) -> Self {
        Self::File {
            path: PathBuf::from(path),
            max_file_size,
            redirect_stderr,
        }
    }

    /// Returns `true` if this is the default stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogStream;
    ///
    /// assert!(LogStream::default().is_default());
    /// ```
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    /// Returns `true` if this is a file stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogStream;
    ///
    /// let stream = LogStream::file("/tmp/log.txt", 1024, false);
    /// assert!(stream.is_file());
    /// ```
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    /// Returns `true` if this is the empty stream (logging disabled).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogStream;
    ///
    /// assert!(LogStream::Empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns the file path if this is a file stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_log::LogStream;
    ///
    /// let stream = LogStream::file("/tmp/log.txt", 1024, false);
    /// assert_eq!(stream.as_path(), Some(&std::path::Path::new("/tmp/log.txt")));
    /// ```
    pub fn as_path(&self) -> Option<&std::path::Path> {
        match self {
            Self::File { path, .. } => Some(path),
            _ => None,
        }
    }
}

impl fmt::Display for LogStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::File {
                path,
                max_file_size,
                redirect_stderr,
            } => write!(
                f,
                "file: path={}, max_size={}, redirect_stderr={}",
                path.display(),
                max_file_size,
                redirect_stderr
            ),
            Self::Empty => write!(f, "empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // LogVerbosityLevel tests
    #[test]
    fn test_verbosity_new() {
        let level = LogVerbosityLevel::new(5);
        assert_eq!(level.value(), 5);
    }

    #[test]
    fn test_verbosity_clamp_max() {
        let level = LogVerbosityLevel::new(2000);
        assert_eq!(level.value(), 1024);
    }

    #[test]
    fn test_verbosity_clamp_min() {
        let level = LogVerbosityLevel::new(-10);
        assert_eq!(level.value(), 0);
    }

    #[test]
    fn test_verbosity_fatal() {
        let level = LogVerbosityLevel::fatal();
        assert_eq!(level.value(), 0);
    }

    #[test]
    fn test_verbosity_error() {
        let level = LogVerbosityLevel::error();
        assert_eq!(level.value(), 1);
    }

    #[test]
    fn test_verbosity_warning() {
        let level = LogVerbosityLevel::warning();
        assert_eq!(level.value(), 2);
    }

    #[test]
    fn test_verbosity_info() {
        let level = LogVerbosityLevel::info();
        assert_eq!(level.value(), 3);
    }

    #[test]
    fn test_verbosity_debug() {
        let level = LogVerbosityLevel::debug();
        assert_eq!(level.value(), 4);
    }

    #[test]
    fn test_verbosity_verbose() {
        let level = LogVerbosityLevel::verbose();
        assert_eq!(level.value(), 5);
    }

    #[test]
    fn test_verbosity_default() {
        let level = LogVerbosityLevel::default();
        assert_eq!(level.value(), 5);
    }

    #[test]
    fn test_verbosity_set_value() {
        let mut level = LogVerbosityLevel::new(5);
        level.set_value(10);
        assert_eq!(level.value(), 10);
    }

    #[test]
    fn test_verbosity_set_value_clamped() {
        let mut level = LogVerbosityLevel::new(5);
        level.set_value(2000);
        assert_eq!(level.value(), 1024);
    }

    #[test]
    fn test_verbosity_logs_fatal() {
        assert!(LogVerbosityLevel::fatal().logs_fatal());
        assert!(LogVerbosityLevel::error().logs_fatal());
    }

    #[test]
    fn test_verbosity_logs_errors() {
        assert!(!LogVerbosityLevel::fatal().logs_errors());
        assert!(LogVerbosityLevel::error().logs_errors());
    }

    #[test]
    fn test_verbosity_logs_warnings() {
        assert!(!LogVerbosityLevel::error().logs_warnings());
        assert!(LogVerbosityLevel::warning().logs_warnings());
    }

    #[test]
    fn test_verbosity_logs_info() {
        assert!(!LogVerbosityLevel::warning().logs_info());
        assert!(LogVerbosityLevel::info().logs_info());
    }

    #[test]
    fn test_verbosity_logs_debug() {
        assert!(!LogVerbosityLevel::info().logs_debug());
        assert!(LogVerbosityLevel::debug().logs_debug());
    }

    #[test]
    fn test_verbosity_logs_verbose() {
        assert!(!LogVerbosityLevel::debug().logs_verbose());
        assert!(LogVerbosityLevel::verbose().logs_verbose());
    }

    #[test]
    fn test_verbosity_description() {
        assert_eq!(LogVerbosityLevel::fatal().description(), "Fatal");
        assert_eq!(LogVerbosityLevel::error().description(), "Error");
        assert_eq!(LogVerbosityLevel::warning().description(), "Warning");
        assert_eq!(LogVerbosityLevel::info().description(), "Info");
        assert_eq!(LogVerbosityLevel::debug().description(), "Debug");
        assert_eq!(LogVerbosityLevel::verbose().description(), "Verbose");
        assert_eq!(LogVerbosityLevel::new(100).description(), "Custom");
    }

    #[test]
    fn test_verbosity_equality() {
        let level1 = LogVerbosityLevel::new(5);
        let level2 = LogVerbosityLevel::new(5);
        assert_eq!(level1, level2);

        let level3 = LogVerbosityLevel::new(10);
        assert_ne!(level1, level3);
    }

    #[test]
    fn test_verbosity_display() {
        let level = LogVerbosityLevel::verbose();
        assert_eq!(format!("{}", level), "Verbose (level 5)");
    }

    // LogTags tests
    #[test]
    fn test_tags_new() {
        let tags = LogTags::new(vec!["tdlib".to_string()]);
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_tags_default() {
        let tags = LogTags::default();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_tags_is_empty() {
        assert!(LogTags::default().is_empty());
        assert!(!LogTags::new(vec!["tag".to_string()]).is_empty());
    }

    #[test]
    fn test_tags_len() {
        let tags = LogTags::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn test_tags_add() {
        let mut tags = LogTags::default();
        tags.add("tdlib");
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_tags_add_duplicate() {
        let mut tags = LogTags::default();
        tags.add("tdlib");
        tags.add("tdlib");
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_tags_remove() {
        let mut tags = LogTags::new(vec!["tdlib".to_string()]);
        tags.remove("tdlib");
        assert!(tags.is_empty());
    }

    #[test]
    fn test_tags_contains() {
        let tags = LogTags::new(vec!["tdlib".to_string()]);
        assert!(tags.contains("tdlib"));
        assert!(!tags.contains("other"));
    }

    #[test]
    fn test_tags_clear() {
        let mut tags = LogTags::new(vec!["tdlib".to_string()]);
        tags.clear();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_tags_tags() {
        let tags = LogTags::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(tags.tags(), &["a", "b"]);
    }

    #[test]
    fn test_tags_equality() {
        let tags1 = LogTags::new(vec!["a".to_string(), "b".to_string()]);
        let tags2 = LogTags::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(tags1, tags2);

        let tags3 = LogTags::new(vec!["a".to_string()]);
        assert_ne!(tags1, tags3);
    }

    #[test]
    fn test_tags_display() {
        let tags = LogTags::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(format!("{}", tags), "[a, b]");
    }

    #[test]
    fn test_tags_display_empty() {
        let tags = LogTags::default();
        assert_eq!(format!("{}", tags), "[]");
    }

    // LogStream tests
    #[test]
    fn test_stream_default() {
        let stream = LogStream::default();
        assert!(stream.is_default());
    }

    #[test]
    fn test_stream_file() {
        let stream = LogStream::file("/tmp/log.txt", 1024, false);
        assert!(stream.is_file());
        assert!(!stream.is_default());
    }

    #[test]
    fn test_stream_empty() {
        assert!(LogStream::Empty.is_empty());
    }

    #[test]
    fn test_stream_is_default() {
        assert!(LogStream::default().is_default());
        assert!(!LogStream::file("/tmp/log.txt", 1024, false).is_default());
    }

    #[test]
    fn test_stream_is_file() {
        assert!(!LogStream::default().is_file());
        assert!(LogStream::file("/tmp/log.txt", 1024, false).is_file());
    }

    #[test]
    fn test_stream_as_path() {
        let stream = LogStream::file("/tmp/log.txt", 1024, false);
        assert_eq!(stream.as_path(), Some(std::path::Path::new("/tmp/log.txt")));
    }

    #[test]
    fn test_stream_as_path_none() {
        assert_eq!(LogStream::default().as_path(), None);
    }

    #[test]
    fn test_stream_equality() {
        let stream1 = LogStream::file("/tmp/log.txt", 1024, false);
        let stream2 = LogStream::file("/tmp/log.txt", 1024, false);
        assert_eq!(stream1, stream2);

        let stream3 = LogStream::file("/tmp/other.txt", 1024, false);
        assert_ne!(stream1, stream3);
    }

    #[test]
    fn test_stream_display_default() {
        assert_eq!(format!("{}", LogStream::default()), "default");
    }

    #[test]
    fn test_stream_display_file() {
        let stream = LogStream::file("/tmp/log.txt", 1024, true);
        let display = format!("{}", stream);
        assert!(display.contains("file"));
        assert!(display.contains("/tmp/log.txt"));
        assert!(display.contains("1024"));
    }

    #[test]
    fn test_stream_display_empty() {
        assert_eq!(format!("{}", LogStream::Empty), "empty");
    }

    // Serialization tests
    #[test]
    fn test_verbosity_serialization() {
        let level = LogVerbosityLevel::new(5);
        let json = serde_json::to_string(&level).unwrap();
        let parsed: LogVerbosityLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(level, parsed);
    }

    #[test]
    fn test_tags_serialization() {
        let tags = LogTags::new(vec!["tdlib".to_string()]);
        let json = serde_json::to_string(&tags).unwrap();
        let parsed: LogTags = serde_json::from_str(&json).unwrap();
        assert_eq!(tags, parsed);
    }

    #[test]
    fn test_stream_serialization() {
        let stream = LogStream::file("/tmp/log.txt", 1024, false);
        let json = serde_json::to_string(&stream).unwrap();
        let parsed: LogStream = serde_json::from_str(&json).unwrap();
        assert_eq!(stream, parsed);
    }

    // Hash tests
    #[test]
    fn test_verbosity_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let level1 = LogVerbosityLevel::new(5);
        let level2 = LogVerbosityLevel::new(5);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        level1.hash(&mut h1);
        level2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_tags_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let tags1 = LogTags::new(vec!["a".to_string(), "b".to_string()]);
        let tags2 = LogTags::new(vec!["a".to_string(), "b".to_string()]);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        tags1.hash(&mut h1);
        tags2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_stream_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let stream1 = LogStream::file("/tmp/log.txt", 1024, false);
        let stream2 = LogStream::file("/tmp/log.txt", 1024, false);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        stream1.hash(&mut h1);
        stream2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // Constants tests
    #[test]
    fn test_verbosity_constants() {
        assert_eq!(LogVerbosityLevel::MIN, 0);
        assert_eq!(LogVerbosityLevel::MAX, 1024);
    }
}
