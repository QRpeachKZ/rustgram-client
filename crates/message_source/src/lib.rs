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

//! # Message Source
//!
//! Source from which a message was retrieved.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `MessageSource` functionality.
//! It identifies where a message came from (history, search, notification, etc.).
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_source::MessageSource;
//!
//! let source = MessageSource::DialogHistory;
//! assert_eq!(source.as_str(), "DialogHistory");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Message source.
///
/// Identifies where a message was retrieved from.
///
/// # TDLib Correspondence
///
/// - TDLib Type: `MessageSource`
/// - Used for tracking message view origins and analytics
///
/// # Example
///
/// ```rust
/// use rustgram_message_source::MessageSource;
///
/// let source = MessageSource::DialogHistory;
/// assert_eq!(source.as_str(), "DialogHistory");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum MessageSource {
    /// Automatic source (internal use).
    Auto = 0,
    /// Dialog history.
    DialogHistory = 1,
    /// Message thread history.
    MessageThreadHistory = 2,
    /// Forum topic history.
    ForumTopicHistory = 3,
    /// Monoforum history.
    MonoforumHistory = 4,
    /// History preview.
    HistoryPreview = 5,
    /// Dialog list.
    DialogList = 6,
    /// Search results.
    Search = 7,
    /// Dialog event log.
    DialogEventLog = 8,
    /// Notification.
    Notification = 9,
    /// Screenshot.
    Screenshot = 10,
    /// Other source.
    Other = 11,
}

impl MessageSource {
    /// Returns the string representation of this source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_source::MessageSource;
    ///
    /// assert_eq!(MessageSource::DialogHistory.as_str(), "DialogHistory");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::DialogHistory => "DialogHistory",
            Self::MessageThreadHistory => "MessageThreadHistory",
            Self::ForumTopicHistory => "ForumTopicHistory",
            Self::MonoforumHistory => "MonoforumHistory",
            Self::HistoryPreview => "HistoryPreview",
            Self::DialogList => "DialogList",
            Self::Search => "Search",
            Self::DialogEventLog => "DialogEventLog",
            Self::Notification => "Notification",
            Self::Screenshot => "Screenshot",
            Self::Other => "Other",
        }
    }

    /// Checks if this is a history-based source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_source::MessageSource;
    ///
    /// assert!(MessageSource::DialogHistory.is_history());
    /// assert!(MessageSource::ForumTopicHistory.is_history());
    /// assert!(!MessageSource::Search.is_history());
    /// ```
    #[must_use]
    pub const fn is_history(self) -> bool {
        matches!(
            self,
            Self::DialogHistory
                | Self::MessageThreadHistory
                | Self::ForumTopicHistory
                | Self::MonoforumHistory
        )
    }

    /// Checks if this is a user-initiated source (vs automatic).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_source::MessageSource;
    ///
    /// assert!(!MessageSource::Auto.is_user_initiated());
    /// assert!(MessageSource::Search.is_user_initiated());
    /// ```
    #[must_use]
    pub const fn is_user_initiated(self) -> bool {
        !matches!(self, Self::Auto)
    }
}

impl Default for MessageSource {
    fn default() -> Self {
        Self::Auto
    }
}

impl fmt::Display for MessageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(MessageSource::Auto.as_str(), "Auto");
        assert_eq!(MessageSource::DialogHistory.as_str(), "DialogHistory");
        assert_eq!(MessageSource::Search.as_str(), "Search");
    }

    #[test]
    fn test_is_history() {
        assert!(MessageSource::DialogHistory.is_history());
        assert!(MessageSource::MessageThreadHistory.is_history());
        assert!(MessageSource::ForumTopicHistory.is_history());
        assert!(MessageSource::MonoforumHistory.is_history());
        assert!(!MessageSource::Search.is_history());
        assert!(!MessageSource::Notification.is_history());
    }

    #[test]
    fn test_is_user_initiated() {
        assert!(!MessageSource::Auto.is_user_initiated());
        assert!(MessageSource::DialogHistory.is_user_initiated());
        assert!(MessageSource::Search.is_user_initiated());
        assert!(MessageSource::Notification.is_user_initiated());
    }

    #[test]
    fn test_default() {
        assert_eq!(MessageSource::default(), MessageSource::Auto);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", MessageSource::DialogHistory), "DialogHistory");
        assert_eq!(format!("{}", MessageSource::Search), "Search");
    }

    #[test]
    fn test_equality() {
        assert_eq!(MessageSource::Search, MessageSource::Search);
        assert_ne!(MessageSource::Search, MessageSource::Notification);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let source1 = MessageSource::Search;
        let source2 = MessageSource::Search;

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        source1.hash(&mut hasher1);
        source2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_serialization() {
        let source = MessageSource::Search;
        let json = serde_json::to_string(&source).unwrap();
        let parsed: MessageSource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, parsed);
    }

    #[test]
    fn test_all_sources() {
        let sources = [
            MessageSource::Auto,
            MessageSource::DialogHistory,
            MessageSource::MessageThreadHistory,
            MessageSource::ForumTopicHistory,
            MessageSource::MonoforumHistory,
            MessageSource::HistoryPreview,
            MessageSource::DialogList,
            MessageSource::Search,
            MessageSource::DialogEventLog,
            MessageSource::Notification,
            MessageSource::Screenshot,
            MessageSource::Other,
        ];

        for source in sources {
            let str_repr = source.as_str();
            assert!(!str_repr.is_empty());
            assert_eq!(format!("{}", source), str_repr);
        }
    }
}
