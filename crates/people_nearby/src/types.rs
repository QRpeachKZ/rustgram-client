// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Types for people nearby functionality.
//!
//! This module implements types for representing nearby chats and users
//! in the Telegram MTProto protocol.

use crate::{PeopleNearbyError, Result};
use rustgram_types::ChatId;

/// Maximum reasonable distance in meters.
///
/// Distances larger than this are considered invalid (roughly 1000 km).
pub const MAX_DISTANCE_METERS: i32 = 1_000_000;

/// A chat nearby with distance information.
///
/// Represents a chat or user that is nearby, along with the distance
/// to that chat/user.
///
/// Corresponds to TDLib type `chatNearby` from TD API.
///
/// # TL Correspondence
///
/// TD API:
/// ```text
/// chatNearby#bde26775 chat_id:int53 distance:int = ChatNearby;
/// ```
///
/// # Examples
///
/// ```
/// use rustgram_people_nearby::ChatNearby;
/// use rustgram_types::ChatId;
///
/// // Create a nearby chat with distance
/// let chat_id = ChatId::new(12345).unwrap();
/// let nearby = ChatNearby::new(chat_id, 150).unwrap();
///
/// assert_eq!(nearby.chat_id().get(), 12345);
/// assert_eq!(nearby.distance(), 150);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatNearby {
    /// Chat ID (can be user, chat, or channel) stored as i64 for Int53 compatibility
    chat_id: i64,
    /// Distance in meters
    distance: i32,
}

impl ChatNearby {
    /// Creates a new ChatNearby.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID
    /// * `distance` - Distance in meters (must be non-negative and <= MAX_DISTANCE_METERS)
    ///
    /// # Errors
    ///
    /// Returns an error if distance is negative or unreasonably large.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatNearby;
    /// use rustgram_types::ChatId;
    ///
    /// let chat_id = ChatId::new(12345).unwrap();
    /// let nearby = ChatNearby::new(chat_id, 150);
    /// assert!(nearby.is_ok());
    ///
    /// // Negative distance is invalid
    /// let invalid = ChatNearby::new(chat_id, -10);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(chat_id: ChatId, distance: i32) -> Result<Self> {
        if distance < 0 {
            return Err(PeopleNearbyError::InvalidDistance(distance));
        }
        if distance > MAX_DISTANCE_METERS {
            return Err(PeopleNearbyError::InvalidDistance(distance));
        }

        Ok(Self {
            chat_id: chat_id.get(),
            distance,
        })
    }

    /// Creates a ChatNearby from raw components (for TL deserialization).
    ///
    /// This method creates a ChatNearby without validation, for use when
    /// deserializing from trusted sources (like MTProto responses).
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat ID as i64 (Int53)
    /// * `distance` - Distance in meters
    pub fn from_tl(chat_id: i64, distance: i32) -> Self {
        Self { chat_id, distance }
    }

    /// Returns the chat ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatNearby;
    /// use rustgram_types::ChatId;
    ///
    /// let chat_id = ChatId::new(12345).unwrap();
    /// let nearby = ChatNearby::new(chat_id, 150).unwrap();
    /// assert_eq!(nearby.chat_id().get(), 12345);
    /// ```
    pub fn chat_id(&self) -> ChatId {
        ChatId::new(self.chat_id).unwrap_or_else(|_| ChatId::default())
    }

    /// Returns the distance in meters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatNearby;
    /// use rustgram_types::ChatId;
    ///
    /// let chat_id = ChatId::new(12345).unwrap();
    /// let nearby = ChatNearby::new(chat_id, 150).unwrap();
    /// assert_eq!(nearby.distance(), 150);
    /// ```
    pub fn distance(&self) -> i32 {
        self.distance
    }

    /// Returns the distance in kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatNearby;
    /// use rustgram_types::ChatId;
    ///
    /// let chat_id = ChatId::new(12345).unwrap();
    /// let nearby = ChatNearby::new(chat_id, 1500).unwrap();
    /// assert!((nearby.distance_km() - 1.5).abs() < 0.01);
    /// ```
    pub fn distance_km(&self) -> f64 {
        self.distance as f64 / 1000.0
    }
}

/// Collection of nearby chats with pagination support.
///
/// Represents the result of searching for nearby chats/users.
/// Includes the list of nearby chats and a pagination offset.
///
/// Corresponds to TDLib type `chatsNearby` from TD API.
///
/// # TL Correspondence
///
/// TD API:
/// ```text
/// chatsNearby#e482a098 users_nearby:vector<chatNearby> next_offset:int = ChatsNearby;
/// ```
///
/// # Examples
///
/// ```
/// use rustgram_people_nearby::{ChatNearby, ChatsNearby};
/// use rustgram_types::ChatId;
///
/// let chat_id1 = ChatId::new(12345).unwrap();
/// let chat_id2 = ChatId::new(67890).unwrap();
///
/// let nearby1 = ChatNearby::new(chat_id1, 100).unwrap();
/// let nearby2 = ChatNearby::new(chat_id2, 200).unwrap();
///
/// let results = ChatsNearby::new(vec![nearby1, nearby2], "next_offset");
///
/// assert_eq!(results.len(), 2);
/// assert!(!results.is_empty());
/// assert_eq!(results.next_offset(), Some("next_offset"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatsNearby {
    /// List of nearby chats
    users_nearby: Vec<ChatNearby>,
    /// Offset for pagination (empty string if no more results)
    next_offset: String,
}

impl ChatsNearby {
    /// Creates a new ChatsNearby.
    ///
    /// # Arguments
    ///
    /// * `users_nearby` - List of nearby chats
    /// * `next_offset` - Pagination offset (empty string if no more results)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::{ChatNearby, ChatsNearby};
    /// use rustgram_types::ChatId;
    ///
    /// let nearby = ChatNearby::new(ChatId::new(123).unwrap(), 100).unwrap();
    /// let results = ChatsNearby::new(vec![nearby], "");
    ///
    /// assert_eq!(results.len(), 1);
    /// assert!(results.next_offset().is_none());
    /// ```
    pub fn new(users_nearby: Vec<ChatNearby>, next_offset: impl Into<String>) -> Self {
        Self {
            users_nearby,
            next_offset: next_offset.into(),
        }
    }

    /// Creates an empty ChatsNearby result.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatsNearby;
    ///
    /// let empty = ChatsNearby::empty();
    /// assert!(empty.is_empty());
    /// assert_eq!(empty.len(), 0);
    /// ```
    pub fn empty() -> Self {
        Self::default()
    }

    /// Returns the list of nearby chats.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::{ChatNearby, ChatsNearby};
    /// use rustgram_types::ChatId;
    ///
    /// let nearby = ChatNearby::new(ChatId::new(123).unwrap(), 100).unwrap();
    /// let results = ChatsNearby::new(vec![nearby.clone()], "");
    ///
    /// assert_eq!(results.users_nearby(), &[nearby]);
    /// ```
    pub fn users_nearby(&self) -> &[ChatNearby] {
        &self.users_nearby
    }

    /// Returns the number of nearby chats.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::{ChatNearby, ChatsNearby};
    /// use rustgram_types::ChatId;
    ///
    /// let nearby = ChatNearby::new(ChatId::new(123).unwrap(), 100).unwrap();
    /// let results = ChatsNearby::new(vec![nearby], "");
    ///
    /// assert_eq!(results.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.users_nearby.len()
    }

    /// Returns true if there are no nearby chats.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatsNearby;
    ///
    /// let empty = ChatsNearby::empty();
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.users_nearby.is_empty()
    }

    /// Returns the pagination offset if more results are available.
    ///
    /// Returns `None` if the offset string is empty (no more results).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatsNearby;
    ///
    /// let with_more = ChatsNearby::new(vec![], "next_page");
    /// assert_eq!(with_more.next_offset(), Some("next_page"));
    ///
    /// let no_more = ChatsNearby::new(vec![], "");
    /// assert!(no_more.next_offset().is_none());
    /// ```
    pub fn next_offset(&self) -> Option<&str> {
        if self.next_offset.is_empty() {
            None
        } else {
            Some(&self.next_offset)
        }
    }

    /// Returns true if more results are available.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::ChatsNearby;
    ///
    /// let with_more = ChatsNearby::new(vec![], "next_page");
    /// assert!(with_more.has_more());
    ///
    /// let no_more = ChatsNearby::new(vec![], "");
    /// assert!(!no_more.has_more());
    /// ```
    pub fn has_more(&self) -> bool {
        !self.next_offset.is_empty()
    }

    /// Sorts the nearby chats by distance (closest first).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::{ChatNearby, ChatsNearby};
    /// use rustgram_types::ChatId;
    ///
    /// let chat1 = ChatNearby::new(ChatId::new(1).unwrap(), 300).unwrap();
    /// let chat2 = ChatNearby::new(ChatId::new(2).unwrap(), 100).unwrap();
    /// let chat3 = ChatNearby::new(ChatId::new(3).unwrap(), 200).unwrap();
    ///
    /// let mut results = ChatsNearby::new(vec![chat1, chat2, chat3], "");
    /// results.sort_by_distance();
    ///
    /// assert_eq!(results.users_nearby()[0].distance(), 100);
    /// assert_eq!(results.users_nearby()[1].distance(), 200);
    /// assert_eq!(results.users_nearby()[2].distance(), 300);
    /// ```
    pub fn sort_by_distance(&mut self) {
        self.users_nearby.sort_by_key(|c| c.distance);
    }

    /// Filters nearby chats by maximum distance.
    ///
    /// # Arguments
    ///
    /// * `max_distance` - Maximum distance in meters
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::{ChatNearby, ChatsNearby};
    /// use rustgram_types::ChatId;
    ///
    /// let chat1 = ChatNearby::new(ChatId::new(1).unwrap(), 150).unwrap();
    /// let chat2 = ChatNearby::new(ChatId::new(2).unwrap(), 500).unwrap();
    ///
    /// let mut results = ChatsNearby::new(vec![chat1, chat2], "");
    /// results.filter_by_distance(200);
    ///
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results.users_nearby()[0].distance(), 150);
    /// ```
    pub fn filter_by_distance(&mut self, max_distance: i32) {
        self.users_nearby.retain(|c| c.distance <= max_distance);
    }
}

/// Update for nearby users.
///
/// Sent 60 seconds after a successful `searchChatsNearby` request
/// to refresh the list of nearby users.
///
/// Corresponds to TDLib type `updateUsersNearby` from TD API.
///
/// # TL Correspondence
///
/// TD API:
/// ```text
/// updateUsersNearby#b090efb9 = Update;
/// ```
///
/// # Examples
///
/// ```
/// use rustgram_people_nearby::UsersNearbyUpdate;
///
/// let update = UsersNearbyUpdate::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsersNearbyUpdate;

impl Default for UsersNearbyUpdate {
    fn default() -> Self {
        Self
    }
}

impl UsersNearbyUpdate {
    /// Creates a new UsersNearbyUpdate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::UsersNearbyUpdate;
    ///
    /// let update = UsersNearbyUpdate::new();
    /// ```
    pub const fn new() -> Self {
        Self
    }

    /// Returns the TL constructor ID for this update.
    ///
    /// TDLib reference: `updateUsersNearby#b090efb9`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::UsersNearbyUpdate;
    ///
    /// let update = UsersNearbyUpdate::new();
    /// assert_eq!(update.constructor_id(), 0xb090efb9);
    /// ```
    pub const fn constructor_id(&self) -> u32 {
        0xb090efb9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_nearby_valid() {
        let chat_id = ChatId::new(12345).unwrap();
        let nearby = ChatNearby::new(chat_id, 150).unwrap();

        assert_eq!(nearby.chat_id().get(), 12345);
        assert_eq!(nearby.distance(), 150);
        assert!((nearby.distance_km() - 0.15).abs() < 0.001);
    }

    #[test]
    fn test_chat_nearby_negative_distance() {
        let chat_id = ChatId::new(12345).unwrap();
        let result = ChatNearby::new(chat_id, -10);

        assert!(matches!(
            result,
            Err(PeopleNearbyError::InvalidDistance(-10))
        ));
    }

    #[test]
    fn test_chat_nearby_too_large_distance() {
        let chat_id = ChatId::new(12345).unwrap();
        let result = ChatNearby::new(chat_id, MAX_DISTANCE_METERS + 1);

        assert!(matches!(result, Err(PeopleNearbyError::InvalidDistance(_))));
    }

    #[test]
    fn test_chat_nearby_zero_distance() {
        let chat_id = ChatId::new(12345).unwrap();
        let nearby = ChatNearby::new(chat_id, 0).unwrap();

        assert_eq!(nearby.distance(), 0);
        assert_eq!(nearby.distance_km(), 0.0);
    }

    #[test]
    fn test_chat_nearby_max_distance() {
        let chat_id = ChatId::new(12345).unwrap();
        let nearby = ChatNearby::new(chat_id, MAX_DISTANCE_METERS).unwrap();

        assert_eq!(nearby.distance(), MAX_DISTANCE_METERS);
    }

    #[test]
    fn test_chat_nearby_from_tl() {
        let chat_id = 12345i64;
        let nearby = ChatNearby::from_tl(chat_id, 150);

        assert_eq!(nearby.chat_id().get(), 12345);
        assert_eq!(nearby.distance(), 150);
    }

    #[test]
    fn test_chats_nearby_new() {
        let chat_id1 = ChatId::new(12345).unwrap();
        let chat_id2 = ChatId::new(67890).unwrap();

        let nearby1 = ChatNearby::new(chat_id1, 100).unwrap();
        let nearby2 = ChatNearby::new(chat_id2, 200).unwrap();

        let results = ChatsNearby::new(vec![nearby1, nearby2], "next_offset");

        assert_eq!(results.len(), 2);
        assert!(results.has_more());
        assert_eq!(results.next_offset(), Some("next_offset"));
    }

    #[test]
    fn test_chats_nearby_empty() {
        let empty = ChatsNearby::empty();

        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert!(!empty.has_more());
        assert!(empty.next_offset().is_none());
    }

    #[test]
    fn test_chats_nearby_default() {
        let default = ChatsNearby::default();

        assert!(default.is_empty());
        assert_eq!(default.len(), 0);
    }

    #[test]
    fn test_chats_nearby_no_more_results() {
        let results = ChatsNearby::new(vec![], "");

        assert!(!results.has_more());
        assert!(results.next_offset().is_none());
    }

    #[test]
    fn test_chats_nearby_sort_by_distance() {
        let chat1 = ChatNearby::new(ChatId::new(1).unwrap(), 300).unwrap();
        let chat2 = ChatNearby::new(ChatId::new(2).unwrap(), 100).unwrap();
        let chat3 = ChatNearby::new(ChatId::new(3).unwrap(), 200).unwrap();

        let mut results = ChatsNearby::new(vec![chat1, chat2, chat3], "");
        results.sort_by_distance();

        assert_eq!(results.users_nearby()[0].distance(), 100);
        assert_eq!(results.users_nearby()[1].distance(), 200);
        assert_eq!(results.users_nearby()[2].distance(), 300);
    }

    #[test]
    fn test_chats_nearby_filter_by_distance() {
        let chat1 = ChatNearby::new(ChatId::new(1).unwrap(), 150).unwrap();
        let chat2 = ChatNearby::new(ChatId::new(2).unwrap(), 200).unwrap();
        let chat3 = ChatNearby::new(ChatId::new(3).unwrap(), 500).unwrap();

        let mut results = ChatsNearby::new(vec![chat1, chat2, chat3], "");
        results.filter_by_distance(200);

        assert_eq!(results.len(), 2);
        assert_eq!(results.users_nearby()[0].distance(), 150);
        assert_eq!(results.users_nearby()[1].distance(), 200);
    }

    #[test]
    fn test_chats_nearby_users_nearby() {
        let chat_id = ChatId::new(123).unwrap();
        let nearby = ChatNearby::new(chat_id, 100).unwrap();

        let results = ChatsNearby::new(vec![nearby.clone()], "");

        assert_eq!(results.users_nearby(), &[nearby]);
    }

    #[test]
    fn test_users_nearby_update() {
        let update = UsersNearbyUpdate::new();

        assert_eq!(update.constructor_id(), 0xb090efb9);
    }

    #[test]
    fn test_users_nearby_update_default() {
        let update = UsersNearbyUpdate::default();

        assert_eq!(update.constructor_id(), 0xb090efb9);
    }

    #[test]
    fn test_distance_km() {
        let chat_id = ChatId::new(123).unwrap();
        let nearby = ChatNearby::new(chat_id, 1500).unwrap();

        assert!((nearby.distance_km() - 1.5).abs() < 0.001);
    }
}
