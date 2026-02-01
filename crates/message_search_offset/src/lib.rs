// Copyright 2025 rustgram-client contributors
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

//! # Message Search Offset
//!
//! Offset for paginating message search results.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `MessageSearchOffset` functionality.
//! It provides pagination support for message search operations.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_search_offset::MessageSearchOffset;
//! use rustgram_types::{DialogId, MessageId};
//!
//! let offset = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), DialogId::default());
//! assert_eq!(offset.date(), 1234567890);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Errors that can occur when working with message search offsets.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Invalid offset string format.
    #[error("Invalid offset string: {0}")]
    InvalidOffset(String),
}

/// Result type for message search offset operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Message search offset.
///
/// Used for pagination when searching messages. Contains the date, message ID,
/// and dialog ID of the last message in the previous page.
///
/// # TDLib Correspondence
///
/// - TDLib Type: `MessageSearchOffset`
/// - Used for: `searchMessages` pagination
///
/// # Example
///
/// ```rust
/// use rustgram_message_search_offset::MessageSearchOffset;
/// use rustgram_types::{DialogId, MessageId};
///
/// let offset = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), DialogId::default());
/// assert_eq!(offset.date(), 1234567890);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageSearchOffset {
    date: i32,
    message_id: MessageId,
    dialog_id: DialogId,
}

impl MessageSearchOffset {
    /// Creates a new message search offset.
    ///
    /// # Arguments
    ///
    /// * `date` - Unix timestamp of the message
    /// * `message_id` - Message ID
    /// * `dialog_id` - Dialog ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_search_offset::MessageSearchOffset;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let offset = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), DialogId::default());
    /// ```
    pub fn new(date: i32, message_id: MessageId, dialog_id: DialogId) -> Self {
        Self {
            date,
            message_id,
            dialog_id,
        }
    }

    /// Returns the date (Unix timestamp).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_search_offset::MessageSearchOffset;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let offset = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), DialogId::default());
    /// assert_eq!(offset.date(), 1234567890);
    /// ```
    #[must_use]
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns the message ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_search_offset::MessageSearchOffset;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let offset = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), DialogId::default());
    /// assert_eq!(offset.message_id(), MessageId::from_server_id(100));
    /// ```
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Returns the dialog ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_search_offset::MessageSearchOffset;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let dialog_id = DialogId::default();
    /// let offset = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), dialog_id);
    /// assert_eq!(offset.dialog_id(), dialog_id);
    /// ```
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Checks if this offset is empty (all zero/default values).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_search_offset::MessageSearchOffset;
    ///
    /// let offset = MessageSearchOffset::default();
    /// assert!(offset.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.date == 0 && self.message_id.get() == 0 && !self.dialog_id.is_valid()
    }

    /// Converts this offset to a string for storage/transmission.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_search_offset::MessageSearchOffset;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let offset = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), DialogId::default());
    /// let s = offset.to_string();
    /// assert!(!s.is_empty());
    /// ```
    pub fn to_string(&self) -> String {
        format!(
            "{}_{}_{}",
            self.date,
            self.message_id.get(),
            self.dialog_id.to_encoded()
        )
    }

    /// Parses an offset from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - String representation of the offset
    ///
    /// # Returns
    ///
    /// Returns `Error::InvalidOffset` if the string format is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_search_offset::MessageSearchOffset;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let offset1 = MessageSearchOffset::new(1234567890, MessageId::from_server_id(100), DialogId::default());
    /// let s = offset1.to_string();
    /// let offset2 = MessageSearchOffset::from_string(&s).unwrap();
    /// assert_eq!(offset1, offset2);
    /// ```
    pub fn from_string(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 3 {
            return Err(Error::InvalidOffset(format!(
                "Expected 3 parts separated by '_', got {}",
                parts.len()
            )));
        }

        let date = parts[0]
            .parse::<i32>()
            .map_err(|_| Error::InvalidOffset(format!("Invalid date: {}", parts[0])))?;

        let message_id = parts[1]
            .parse::<i64>()
            .map_err(|_| Error::InvalidOffset(format!("Invalid message_id: {}", parts[1])))?;
        let message_id = MessageId::new(message_id)
            .map_err(|_| Error::InvalidOffset(format!("Invalid message_id: {}", parts[1])))?;

        let dialog_id_encoded = parts[2]
            .parse::<i64>()
            .map_err(|_| Error::InvalidOffset(format!("Invalid dialog_id: {}", parts[2])))?;
        let dialog_id = DialogId::from_encoded(dialog_id_encoded)
            .map_err(|_| Error::InvalidOffset(format!("Invalid dialog_id: {}", parts[2])))?;

        Ok(Self {
            date,
            message_id,
            dialog_id,
        })
    }
}

impl fmt::Display for MessageSearchOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let offset = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        assert_eq!(offset.date(), 1234567890);
        assert_eq!(offset.message_id(), MessageId::from_server_id(100));
    }

    #[test]
    fn test_default() {
        let offset = MessageSearchOffset::default();
        assert_eq!(offset.date(), 0);
        assert_eq!(offset.message_id().get(), 0);
        assert!(offset.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let offset = MessageSearchOffset::default();
        assert!(offset.is_empty());

        let offset = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        assert!(!offset.is_empty());
    }

    #[test]
    fn test_to_string() {
        let offset = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        let s = offset.to_string();
        assert!(!s.is_empty());
        assert!(s.contains("1234567890"));
    }

    #[test]
    fn test_from_string_valid() {
        let offset1 = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        let s = offset1.to_string();
        let offset2 = MessageSearchOffset::from_string(&s).unwrap();
        assert_eq!(offset1, offset2);
    }

    #[test]
    fn test_from_string_invalid_parts() {
        let result = MessageSearchOffset::from_string("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_string_invalid_date() {
        let result = MessageSearchOffset::from_string("abc_100_0");
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let offset = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        let s = format!("{}", offset);
        assert_eq!(s, offset.to_string());
    }

    #[test]
    fn test_equality() {
        let offset1 = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        let offset2 = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        assert_eq!(offset1, offset2);

        let offset3 = MessageSearchOffset::new(
            1234567891,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        assert_ne!(offset1, offset3);
    }

    #[test]
    fn test_serialization() {
        let offset = MessageSearchOffset::new(
            1234567890,
            MessageId::from_server_id(100),
            DialogId::default(),
        );
        let json = serde_json::to_string(&offset).unwrap();
        let parsed: MessageSearchOffset = serde_json::from_str(&json).unwrap();
        assert_eq!(offset, parsed);
    }
}
