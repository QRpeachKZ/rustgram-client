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

//! # Input Group Call
//!
//! Represents an input reference to a group voice chat or video chat.
//!
//! ## Overview
//!
//! `InputGroupCall` represents a reference to a group call in Telegram.
//! It can be specified either by a join link (slug) or by a message containing the link.
//!
//! ## TDLib Correspondence
//!
//! | Rust Type | TDLib Type | TL Schema |
//! |-----------|-----------|-----------|
//! | [`InputGroupCall::Link`] | `inputGroupCallLink` | `td_api.tl:6288` |
//! | [`InputGroupCall::Message`] | `inputGroupCallMessage` | `td_api.tl:6293` |
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_input_group_call::InputGroupCall;
//! use rustgram_types::{ChatId, MessageId};
//!
//! // Via link slug
//! let call = InputGroupCall::link("abc123");
//! assert!(call.is_link());
//!
//! // Via message
//! let chat_id = ChatId(123456);
//! let message_id = MessageId(789);
//! let call = InputGroupCall::message(chat_id, message_id);
//! assert!(call.is_message());
//! ```

use rustgram_types::{ChatId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Input reference to a group voice chat.
///
/// A group call can be referenced either by:
/// - A join link (slug) string
/// - A message in a chat that contains the join link
///
/// # Examples
///
/// ```
/// use rustgram_input_group_call::InputGroupCall;
/// use rustgram_types::{ChatId, MessageId};
///
/// // Create via link
/// let call = InputGroupCall::link("slug");
/// assert!(matches!(call, InputGroupCall::Link(_)));
///
/// // Create via message
/// let call = InputGroupCall::message(ChatId(123), MessageId(456));
/// assert!(matches!(call, InputGroupCall::Message { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputGroupCall {
    /// TDLib: `inputGroupCallLink`
    ///
    /// A join link to the group call.
    Link(String),

    /// TDLib: `inputGroupCallMessage`
    ///
    /// A message in a chat that contains the join link.
    Message {
        /// The chat that contains the message
        chat_id: ChatId,
        /// The message identifier
        message_id: MessageId,
    },
}

impl InputGroupCall {
    /// Creates an `InputGroupCall` from a link slug.
    ///
    /// # Arguments
    ///
    /// * `slug` - The join link slug string
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_group_call::InputGroupCall;
    ///
    /// let call = InputGroupCall::link("abc123def");
    /// assert_eq!(call.as_slug(), Some("abc123def"));
    /// ```
    pub fn link(slug: &str) -> Self {
        Self::Link(slug.to_string())
    }

    /// Creates an `InputGroupCall` from a chat and message ID.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat containing the message with the link
    /// * `message_id` - The message identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_group_call::InputGroupCall;
    /// use rustgram_types::{ChatId, MessageId};
    ///
    /// let call = InputGroupCall::message(ChatId(123), MessageId(456));
    /// let (chat, msg) = call.as_message().unwrap();
    /// assert_eq!(chat.get(), 123);
    /// assert_eq!(msg.get(), 456);
    /// ```
    pub fn message(chat_id: ChatId, message_id: MessageId) -> Self {
        Self::Message {
            chat_id,
            message_id,
        }
    }

    /// Returns `true` if this is a link variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_group_call::InputGroupCall;
    ///
    /// let call = InputGroupCall::link("slug");
    /// assert!(call.is_link());
    /// ```
    pub fn is_link(&self) -> bool {
        matches!(self, Self::Link(_))
    }

    /// Returns `true` if this is a message variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_group_call::InputGroupCall;
    /// use rustgram_types::{ChatId, MessageId};
    ///
    /// let call = InputGroupCall::message(ChatId(123), MessageId(456));
    /// assert!(call.is_message());
    /// ```
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message { .. })
    }

    /// Returns the slug if this is a link variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_group_call::InputGroupCall;
    ///
    /// let call = InputGroupCall::link("abc123");
    /// assert_eq!(call.as_slug(), Some("abc123"));
    /// ```
    pub fn as_slug(&self) -> Option<&str> {
        match self {
            Self::Link(slug) => Some(slug),
            _ => None,
        }
    }

    /// Returns the chat and message ID if this is a message variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_group_call::InputGroupCall;
    /// use rustgram_types::{ChatId, MessageId};
    ///
    /// let call = InputGroupCall::message(ChatId(123), MessageId(456));
    /// let (chat, msg) = call.as_message().unwrap();
    /// assert_eq!(chat.get(), 123);
    /// assert_eq!(msg.get(), 456);
    /// ```
    pub fn as_message(&self) -> Option<(ChatId, MessageId)> {
        match self {
            Self::Message {
                chat_id,
                message_id,
            } => Some((*chat_id, *message_id)),
            _ => None,
        }
    }

    /// Computes a hash value for this group call reference.
    ///
    /// This matches TDLib's hash computation which uses either the slug hash
    /// or the message ID depending on the variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_group_call::InputGroupCall;
    /// use std::collections::hash_map::DefaultHasher;
    /// use std::hash::{Hash, Hasher};
    ///
    /// let call1 = InputGroupCall::link("slug");
    /// let call2 = InputGroupCall::link("slug");
    ///
    /// let mut h1 = DefaultHasher::new();
    /// let mut h2 = DefaultHasher::new();
    /// call1.hash(&mut h1);
    /// call2.hash(&mut h2);
    /// assert_eq!(h1.finish(), h2.finish());
    /// ```
    pub fn compute_hash(&self) -> u32 {
        match self {
            Self::Link(slug) if !slug.is_empty() => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                slug.hash(&mut hasher);
                hasher.finish() as u32
            }
            Self::Message { message_id, .. } => message_id.get() as u32,
            _ => 0,
        }
    }
}

impl Hash for InputGroupCall {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Link(slug) => {
                slug.hash(state);
            }
            Self::Message {
                chat_id,
                message_id,
            } => {
                chat_id.hash(state);
                message_id.hash(state);
            }
        }
    }
}

impl fmt::Display for InputGroupCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Link(slug) => write!(f, "group_call:link:{}", slug),
            Self::Message {
                chat_id,
                message_id,
            } => write!(
                f,
                "group_call:message:{}/{}",
                chat_id.get(),
                message_id.get()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_message_call() -> InputGroupCall {
        InputGroupCall::message(ChatId(123456), MessageId(789012))
    }

    #[test]
    fn test_link_variant() {
        let call = InputGroupCall::link("test_slug");
        assert!(call.is_link());
        assert!(!call.is_message());
        assert_eq!(call.as_slug(), Some("test_slug"));
        assert_eq!(call.as_message(), None);
    }

    #[test]
    fn test_message_variant() {
        let call = create_test_message_call();
        assert!(!call.is_link());
        assert!(call.is_message());
        assert_eq!(call.as_slug(), None);
        let (chat, msg) = call.as_message().unwrap();
        assert_eq!(chat.get(), 123456);
        assert_eq!(msg.get(), 789012);
    }

    #[test]
    fn test_link_empty() {
        let call = InputGroupCall::link("");
        assert!(call.is_link());
        assert_eq!(call.as_slug(), Some(""));
        assert_eq!(call.compute_hash(), 0);
    }

    #[test]
    fn test_message_hash() {
        let call = InputGroupCall::message(ChatId(123), MessageId(456));
        // Message variant hashes by message_id
        assert_eq!(call.compute_hash(), 456);
    }

    #[test]
    fn test_link_hash() {
        let call1 = InputGroupCall::link("same_slug");
        let call2 = InputGroupCall::link("same_slug");
        assert_eq!(call1.compute_hash(), call2.compute_hash());
    }

    #[test]
    fn test_equality() {
        let call1 = InputGroupCall::link("slug");
        let call2 = InputGroupCall::link("slug");
        assert_eq!(call1, call2);

        let call3 = InputGroupCall::link("different");
        assert_ne!(call1, call3);
    }

    #[test]
    fn test_message_equality() {
        let call1 = InputGroupCall::message(ChatId(123), MessageId(456));
        let call2 = InputGroupCall::message(ChatId(123), MessageId(456));
        assert_eq!(call1, call2);

        let call3 = InputGroupCall::message(ChatId(123), MessageId(789));
        assert_ne!(call1, call3);

        let call4 = InputGroupCall::message(ChatId(456), MessageId(789));
        assert_ne!(call1, call4);
    }

    #[test]
    fn test_cross_variant_inequality() {
        let link = InputGroupCall::link("slug");
        let message = InputGroupCall::message(ChatId(123), MessageId(456));
        assert_ne!(link, message);
    }

    #[test]
    fn test_clone() {
        let call = create_test_message_call();
        let cloned = call.clone();
        assert_eq!(call, cloned);
    }

    #[test]
    fn test_debug() {
        let call = create_test_message_call();
        let debug = format!("{:?}", call);
        assert!(debug.contains("Message"));
    }

    #[test]
    fn test_display_link() {
        let call = InputGroupCall::link("abc123");
        assert_eq!(format!("{}", call), "group_call:link:abc123");
    }

    #[test]
    fn test_display_message() {
        let call = InputGroupCall::message(ChatId(123), MessageId(456));
        assert_eq!(format!("{}", call), "group_call:message:123/456");
    }

    #[test]
    fn test_hash_trait() {
        use std::collections::hash_map::DefaultHasher;

        let call1 = InputGroupCall::link("slug");
        let call2 = InputGroupCall::link("slug");

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        call1.hash(&mut h1);
        call2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_message_hash_trait() {
        use std::collections::hash_map::DefaultHasher;

        let call1 = InputGroupCall::message(ChatId(123), MessageId(456));
        let call2 = InputGroupCall::message(ChatId(123), MessageId(456));

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        call1.hash(&mut h1);
        call2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_different_slugs_different_hash() {
        let call1 = InputGroupCall::link("slug1");
        let call2 = InputGroupCall::link("slug2");
        assert_ne!(call1.compute_hash(), call2.compute_hash());
    }

    #[test]
    fn test_serialization_link() {
        let call = InputGroupCall::link("slug");
        let json = serde_json::to_string(&call).unwrap();
        let parsed: InputGroupCall = serde_json::from_str(&json).unwrap();
        assert_eq!(call, parsed);
    }

    #[test]
    fn test_serialization_message() {
        let call = InputGroupCall::message(ChatId(123), MessageId(456));
        let json = serde_json::to_string(&call).unwrap();
        let parsed: InputGroupCall = serde_json::from_str(&json).unwrap();
        assert_eq!(call, parsed);
    }

    #[test]
    fn test_link_with_special_chars() {
        let slug = "abc-123_xyz";
        let call = InputGroupCall::link(slug);
        assert_eq!(call.as_slug(), Some(slug));
    }

    #[test]
    fn test_message_with_zero_ids() {
        let call = InputGroupCall::message(ChatId(0), MessageId(0));
        assert!(call.is_message());
        let (chat, msg) = call.as_message().unwrap();
        assert_eq!(chat.get(), 0);
        assert_eq!(msg.get(), 0);
    }

    #[test]
    fn test_compute_hash_consistency() {
        let call = InputGroupCall::link("consistent_slug");
        let hash1 = call.compute_hash();
        let hash2 = call.compute_hash();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_empty_link_hash() {
        let call = InputGroupCall::link("");
        assert_eq!(call.compute_hash(), 0);
    }

    #[test]
    fn test_message_zero_hash() {
        let call = InputGroupCall::message(ChatId(123), MessageId(0));
        assert_eq!(call.compute_hash(), 0);
    }
}
