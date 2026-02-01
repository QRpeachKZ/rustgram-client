// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Transcription Info
//!
//! Audio transcription information for messages.
//!
//! ## Overview
//!
//! Tracks audio transcription status and results.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_transcription_info::TranscriptionInfo;
//!
//! let mut info = TranscriptionInfo::new();
//! info.start_transcription(123);
//! info.complete_transcription("Transcribed text".to_string(), 123);
//! assert!(info.is_transcribed());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Transcription info for audio messages
///
/// Tracks the status and results of audio transcription.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptionInfo {
    /// Whether transcription is complete
    is_transcribed: bool,
    /// Transcription ID
    transcription_id: i64,
    /// Transcribed text
    text: String,
    /// Whether there was an error
    has_error: bool,
    /// Error message
    error: Option<String>,
}

impl Default for TranscriptionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptionInfo {
    /// Creates a new empty transcription info
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_transcribed: false,
            transcription_id: 0,
            text: String::new(),
            has_error: false,
            error: None,
        }
    }

    /// Returns whether transcription is complete
    #[must_use]
    pub const fn is_transcribed(&self) -> bool {
        self.is_transcribed
    }

    /// Returns the transcription ID
    #[must_use]
    pub const fn transcription_id(&self) -> i64 {
        self.transcription_id
    }

    /// Returns the transcribed text
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns whether there was an error
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.has_error
    }

    /// Returns the error message
    #[must_use]
    pub fn error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    /// Starts a new transcription
    pub fn start_transcription(&mut self, id: i64) {
        self.transcription_id = id;
        self.is_transcribed = false;
        self.has_error = false;
        self.error = None;
    }

    /// Completes the transcription with text
    pub fn complete_transcription(&mut self, text: String, id: i64) {
        self.text = text;
        self.is_transcribed = true;
        self.transcription_id = id;
        self.has_error = false;
        self.error = None;
    }

    /// Updates with partial transcription text
    pub fn update_partial(&mut self, text: String, id: i64) {
        if self.transcription_id == id {
            self.text = text;
        }
    }

    /// Marks the transcription as failed
    pub fn fail(&mut self, error: String) {
        self.has_error = true;
        self.error = Some(error);
        self.is_transcribed = false;
    }

    /// Resets the transcription info
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl fmt::Display for TranscriptionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_error {
            if let Some(err) = &self.error {
                write!(f, "Transcription error: {}", err)
            } else {
                write!(f, "Transcription error")
            }
        } else if self.is_transcribed {
            write!(f, "Transcribed: {}", self.text)
        } else {
            write!(f, "Transcription in progress...")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = TranscriptionInfo::new();
        assert!(!info.is_transcribed());
        assert!(!info.has_error());
        assert_eq!(info.text(), "");
    }

    #[test]
    fn test_start_transcription() {
        let mut info = TranscriptionInfo::new();
        info.start_transcription(123);
        assert_eq!(info.transcription_id(), 123);
        assert!(!info.is_transcribed());
    }

    #[test]
    fn test_complete_transcription() {
        let mut info = TranscriptionInfo::new();
        info.complete_transcription("Hello world".to_string(), 456);
        assert!(info.is_transcribed());
        assert_eq!(info.text(), "Hello world");
        assert_eq!(info.transcription_id(), 456);
    }

    #[test]
    fn test_update_partial() {
        let mut info = TranscriptionInfo::new();
        info.start_transcription(789);
        info.update_partial("Partial".to_string(), 789);
        assert_eq!(info.text(), "Partial");
        assert!(!info.is_transcribed());
    }

    #[test]
    fn test_fail() {
        let mut info = TranscriptionInfo::new();
        info.fail("Network error".to_string());
        assert!(info.has_error());
        assert_eq!(info.error(), Some(&"Network error".to_string()));
        assert!(!info.is_transcribed());
    }

    #[test]
    fn test_reset() {
        let mut info = TranscriptionInfo::new();
        info.complete_transcription("Text".to_string(), 1);
        info.reset();
        assert!(!info.is_transcribed());
        assert_eq!(info.text(), "");
    }

    #[test]
    fn test_display() {
        let mut info = TranscriptionInfo::new();
        info.complete_transcription("Test".to_string(), 1);
        assert!(format!("{}", info).contains("Test"));
    }

    #[test]
    fn test_display_error() {
        let mut info = TranscriptionInfo::new();
        info.fail("Error".to_string());
        assert!(format!("{}", info).contains("Error"));
    }
}
