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

//! # Reaction Manager
//!
//! Manages reactions, message effects, and saved messages tags for Telegram MTProto client.
//!
//! ## Overview
//!
//! The ReactionManager is responsible for:
//! - Tracking active and recent reactions
//! - Managing message effects (stickers and animations)
//! - Handling saved messages tags with reactions
//! - Managing default paid reaction types
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's `ReactionManager` from `td/telegram/ReactionManager.h`.
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_reaction_manager::{ReactionManager, Effect};
//! use rustgram_message_effect_id::MessageEffectId;
//! use rustgram_file_id::FileId;
//! use rustgram_paid_reaction_type::PaidReactionType;
//!
//! // Create a new reaction manager
//! let mut manager = ReactionManager::new();
//!
//! // Add a message effect
//! let effect = Effect::new(
//!     MessageEffectId::new(123),
//!     "🎉",
//!     FileId::new(1, 0),
//!     FileId::new(2, 0),
//!     FileId::new(3, 0),
//! );
//! manager.add_effect(effect);
//!
//! // Get an effect by ID
//! let effect = manager.get_effect(MessageEffectId::new(123));
//! assert!(effect.is_some());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::len_without_is_empty)]

mod error;

use rustgram_chat_reactions::ChatReactions;
use rustgram_file_id::FileId;
use rustgram_message_effect_id::MessageEffectId;
use rustgram_paid_reaction_type::PaidReactionType;
use rustgram_reaction_type::ReactionType;
use rustgram_saved_messages_manager::SavedMessagesTopicId;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub use error::{Error, Result};

/// Maximum number of recent reactions to track.
pub const MAX_RECENT_REACTIONS: usize = 100;

/// Maximum length for tag titles.
pub const MAX_TAG_TITLE_LENGTH: i32 = 12;

/// Message effect.
///
/// Represents a visual effect that can be applied to messages.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `ReactionManager::Effect`.
///
/// # Examples
///
/// ```
/// use rustgram_reaction_manager::Effect;
/// use rustgram_message_effect_id::MessageEffectId;
/// use rustgram_file_id::FileId;
///
/// let effect = Effect::new(
///     MessageEffectId::new(123),
///     "🎉",
///     FileId::new(1, 0),
///     FileId::new(2, 0),
///     FileId::new(3, 0),
/// );
/// assert!(effect.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    /// Unique effect identifier.
    id: MessageEffectId,
    /// Emoji representation of the effect.
    emoji: String,
    /// Static icon file ID.
    static_icon_id: FileId,
    /// Effect sticker file ID.
    effect_sticker_id: FileId,
    /// Effect animation file ID.
    effect_animation_id: FileId,
    /// Whether this is a premium effect.
    is_premium: bool,
}

impl Effect {
    /// Creates a new effect.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique effect identifier
    /// * `emoji` - Emoji representation
    /// * `static_icon_id` - Static icon file ID
    /// * `effect_sticker_id` - Effect sticker file ID
    /// * `effect_animation_id` - Effect animation file ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::Effect;
    /// use rustgram_message_effect_id::MessageEffectId;
    /// use rustgram_file_id::FileId;
    ///
    /// let effect = Effect::new(
    ///     MessageEffectId::new(123),
    ///     "🎉",
    ///     FileId::new(1, 0),
    ///     FileId::new(2, 0),
    ///     FileId::new(3, 0),
    /// );
    /// ```
    #[must_use]
    pub fn new(
        id: MessageEffectId,
        emoji: impl Into<String>,
        static_icon_id: FileId,
        effect_sticker_id: FileId,
        effect_animation_id: FileId,
    ) -> Self {
        Self {
            id,
            emoji: emoji.into(),
            static_icon_id,
            effect_sticker_id,
            effect_animation_id,
            is_premium: false,
        }
    }

    /// Creates a premium effect.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique effect identifier
    /// * `emoji` - Emoji representation
    /// * `static_icon_id` - Static icon file ID
    /// * `effect_sticker_id` - Effect sticker file ID
    /// * `effect_animation_id` - Effect animation file ID
    #[must_use]
    pub fn premium(
        id: MessageEffectId,
        emoji: impl Into<String>,
        static_icon_id: FileId,
        effect_sticker_id: FileId,
        effect_animation_id: FileId,
    ) -> Self {
        Self {
            id,
            emoji: emoji.into(),
            static_icon_id,
            effect_sticker_id,
            effect_animation_id,
            is_premium: true,
        }
    }

    /// Returns the effect ID.
    #[must_use]
    pub const fn id(&self) -> MessageEffectId {
        self.id
    }

    /// Returns the emoji representation.
    #[must_use]
    pub fn emoji(&self) -> &str {
        &self.emoji
    }

    /// Returns the static icon file ID.
    #[must_use]
    pub const fn static_icon_id(&self) -> FileId {
        self.static_icon_id
    }

    /// Returns the effect sticker file ID.
    #[must_use]
    pub const fn effect_sticker_id(&self) -> FileId {
        self.effect_sticker_id
    }

    /// Returns the effect animation file ID.
    #[must_use]
    pub const fn effect_animation_id(&self) -> FileId {
        self.effect_animation_id
    }

    /// Checks if this is a premium effect.
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        self.is_premium
    }

    /// Checks if this is a sticker effect (no animation).
    #[must_use]
    pub const fn is_sticker(&self) -> bool {
        self.effect_animation_id.get() == 0
    }

    /// Checks if this effect is valid.
    ///
    /// A valid effect has a valid ID and valid sticker file ID.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.id.is_valid() && self.effect_sticker_id.is_valid()
    }
}

/// Saved reaction tag.
///
/// Represents a tag for organizing saved messages by reaction.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `ReactionManager::SavedReactionTag`.
///
/// # Examples
///
/// ```
/// use rustgram_reaction_manager::SavedReactionTag;
/// use rustgram_reaction_type::ReactionType;
///
/// let tag = SavedReactionTag::new(
///     ReactionType::emoji("❤️"),
///     "Favorites",
///     10,
/// );
/// assert!(tag.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedReactionTag {
    /// Reaction type for this tag.
    reaction_type: ReactionType,
    /// Hash for quick comparison.
    hash: u64,
    /// Tag title.
    title: String,
    /// Number of messages with this tag.
    count: i32,
}

impl SavedReactionTag {
    /// Creates a new saved reaction tag.
    ///
    /// # Arguments
    ///
    /// * `reaction_type` - Reaction type for this tag
    /// * `title` - Tag title
    /// * `count` - Number of messages with this tag
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::SavedReactionTag;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let tag = SavedReactionTag::new(
    ///     ReactionType::emoji("❤️"),
    ///     "Favorites",
    ///     10,
    /// );
    /// ```
    #[must_use]
    pub fn new(reaction_type: ReactionType, title: impl Into<String>, count: i32) -> Self {
        let title = title.into();
        let hash = Self::calculate_hash(&reaction_type, &title);
        Self {
            reaction_type,
            hash,
            title,
            count: count.max(0),
        }
    }

    /// Calculates a hash for the tag.
    fn calculate_hash(reaction_type: &ReactionType, title: &str) -> u64 {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        reaction_type.md5_hash().hash(&mut hash);
        title.hash(&mut hash);
        hash.finish()
    }

    /// Returns the reaction type.
    #[must_use]
    pub const fn reaction_type(&self) -> &ReactionType {
        &self.reaction_type
    }

    /// Returns the hash.
    #[must_use]
    pub const fn hash(&self) -> u64 {
        self.hash
    }

    /// Returns the title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the count.
    #[must_use]
    pub const fn count(&self) -> i32 {
        self.count
    }

    /// Sets the title.
    ///
    /// Returns an error if the title is too long.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::SavedReactionTag;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let mut tag = SavedReactionTag::new(
    ///     ReactionType::emoji("❤️"),
    ///     "Favorites",
    ///     10,
    /// );
    /// assert!(tag.set_title("New Title").is_ok());
    /// ```
    pub fn set_title(&mut self, title: impl Into<String>) -> Result<()> {
        let title = title.into();
        if title.len() > MAX_TAG_TITLE_LENGTH as usize {
            return Err(Error::InvalidTagTitle);
        }
        self.title = title;
        self.hash = Self::calculate_hash(&self.reaction_type, &self.title);
        Ok(())
    }

    /// Checks if this tag is valid.
    ///
    /// A valid tag has a non-empty reaction type and valid count.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.reaction_type.as_str().is_empty()
            && self.count >= 0
            && (self.count > 0 || !self.title.is_empty())
    }
}

/// Saved reaction tags for a topic.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `ReactionManager::SavedReactionTags`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedReactionTags {
    /// Tags in this collection.
    tags: Vec<SavedReactionTag>,
    /// Hash for quick comparison.
    hash: i64,
    /// Whether this has been initialized.
    is_inited: bool,
}

impl SavedReactionTags {
    /// Creates a new saved reaction tags collection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            hash: 0,
            is_inited: false,
        }
    }

    /// Returns the tags.
    #[must_use]
    pub fn tags(&self) -> &[SavedReactionTag] {
        &self.tags
    }

    /// Returns the hash.
    #[must_use]
    pub const fn hash(&self) -> i64 {
        self.hash
    }

    /// Checks if this is initialized.
    #[must_use]
    pub const fn is_inited(&self) -> bool {
        self.is_inited
    }

    /// Calculates the hash from the tags.
    fn calculate_hash(&self) -> i64 {
        self.tags.iter().map(|tag| tag.hash() as i64).sum()
    }

    /// Updates tags based on old and new reaction lists.
    pub fn update_saved_messages_tags(
        &mut self,
        old_tags: &[ReactionType],
        new_tags: &[ReactionType],
    ) -> bool {
        let mut modified = false;

        // Decrease counts for removed tags
        for old_tag in old_tags {
            if !new_tags.contains(old_tag) {
                if let Some(tag) = self.tags.iter_mut().find(|t| t.reaction_type() == old_tag) {
                    tag.count -= 1;
                    modified = true;
                }
            }
        }

        // Increase counts for added tags
        for new_tag in new_tags {
            if !old_tags.contains(new_tag) {
                if let Some(tag) = self.tags.iter_mut().find(|t| t.reaction_type() == new_tag) {
                    tag.count += 1;
                    modified = true;
                } else {
                    // Add new tag
                    self.tags
                        .push(SavedReactionTag::new(new_tag.clone(), "", 1));
                    modified = true;
                }
            }
        }

        if modified {
            self.hash = self.calculate_hash();
        }

        modified
    }

    /// Sets a tag title.
    pub fn set_tag_title(&mut self, reaction_type: &ReactionType, title: &str) -> Result<()> {
        if title.len() > MAX_TAG_TITLE_LENGTH as usize {
            return Err(Error::InvalidTagTitle);
        }

        if let Some(tag) = self
            .tags
            .iter_mut()
            .find(|t| t.reaction_type() == reaction_type)
        {
            tag.set_title(title)?;
            self.hash = self.calculate_hash();
            return Ok(());
        }

        Err(Error::TagNotFound)
    }

    /// Gets tags sorted by count.
    #[must_use]
    pub fn sorted_tags(&self) -> Vec<SavedReactionTag> {
        let mut tags = self.tags.clone();
        tags.sort_by(|a, b| b.count.cmp(&a.count));
        tags
    }
}

impl Default for SavedReactionTags {
    fn default() -> Self {
        Self::new()
    }
}

/// Reaction manager.
///
/// Manages reactions, message effects, and saved messages tags.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `ReactionManager` from `td/telegram/ReactionManager.h`.
///
/// # Examples
///
/// ```
/// use rustgram_reaction_manager::ReactionManager;
/// use rustgram_reaction_type::ReactionType;
///
/// let mut manager = ReactionManager::new();
///
/// // Check if a reaction is active
/// assert!(!manager.is_active_reaction(&ReactionType::emoji("👍")));
///
/// // Add a recent reaction
/// manager.add_recent_reaction(ReactionType::emoji("👍"));
/// ```
#[derive(Debug, Clone)]
pub struct ReactionManager {
    /// Active reactions.
    active_reactions: Vec<ReactionType>,
    /// Recent reactions.
    recent_reactions: Vec<ReactionType>,
    /// Message effects indexed by ID.
    message_effects: HashMap<MessageEffectId, Effect>,
    /// Active message effect IDs.
    active_message_effects: Vec<MessageEffectId>,
    /// Saved tags for all topics.
    #[allow(dead_code)]
    all_tags: SavedReactionTags,
    /// Tags for specific topics.
    topic_tags: HashMap<SavedMessagesTopicId, SavedReactionTags>,
    /// Default paid reaction type.
    default_paid_type: PaidReactionType,
}

impl ReactionManager {
    /// Creates a new reaction manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::ReactionManager;
    ///
    /// let manager = ReactionManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_reactions: Vec::new(),
            recent_reactions: Vec::new(),
            message_effects: HashMap::new(),
            active_message_effects: Vec::new(),
            all_tags: SavedReactionTags::new(),
            topic_tags: HashMap::new(),
            default_paid_type: PaidReactionType::regular(),
        }
    }

    /// Checks if a reaction is active.
    ///
    /// # Arguments
    ///
    /// * `reaction` - The reaction to check
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::ReactionManager;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let manager = ReactionManager::new();
    /// assert!(!manager.is_active_reaction(&ReactionType::emoji("👍")));
    /// ```
    #[must_use]
    pub fn is_active_reaction(&self, reaction: &ReactionType) -> bool {
        self.active_reactions.contains(reaction)
    }

    /// Adds a recent reaction.
    ///
    /// # Arguments
    ///
    /// * `reaction` - The reaction to add
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::ReactionManager;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let mut manager = ReactionManager::new();
    /// manager.add_recent_reaction(ReactionType::emoji("👍"));
    /// ```
    pub fn add_recent_reaction(&mut self, reaction: ReactionType) {
        // Remove if already present
        self.recent_reactions.retain(|r| r != &reaction);

        // Add to front
        self.recent_reactions.insert(0, reaction);

        // Trim to max size
        if self.recent_reactions.len() > MAX_RECENT_REACTIONS {
            self.recent_reactions.truncate(MAX_RECENT_REACTIONS);
        }
    }

    /// Clears recent reactions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::ReactionManager;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let mut manager = ReactionManager::new();
    /// manager.add_recent_reaction(ReactionType::emoji("👍"));
    /// manager.clear_recent_reactions();
    /// assert!(manager.recent_reactions().is_empty());
    /// ```
    pub fn clear_recent_reactions(&mut self) {
        self.recent_reactions.clear();
    }

    /// Returns recent reactions.
    #[must_use]
    pub fn recent_reactions(&self) -> &[ReactionType] {
        &self.recent_reactions
    }

    /// Returns active reactions.
    #[must_use]
    pub fn active_reactions(&self) -> &[ReactionType] {
        &self.active_reactions
    }

    /// Sets active reactions.
    pub fn set_active_reactions(&mut self, reactions: Vec<ReactionType>) {
        self.active_reactions = reactions;
    }

    /// Adds a message effect.
    ///
    /// # Arguments
    ///
    /// * `effect` - The effect to add
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::{ReactionManager, Effect};
    /// use rustgram_message_effect_id::MessageEffectId;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = ReactionManager::new();
    /// let effect = Effect::new(
    ///     MessageEffectId::new(123),
    ///     "🎉",
    ///     FileId::new(1, 0),
    ///     FileId::new(2, 0),
    ///     FileId::new(3, 0),
    /// );
    /// manager.add_effect(effect);
    /// ```
    pub fn add_effect(&mut self, effect: Effect) {
        let id = effect.id();
        self.message_effects.insert(id, effect);
    }

    /// Gets a message effect by ID.
    ///
    /// # Arguments
    ///
    /// * `effect_id` - The effect ID
    ///
    /// # Returns
    ///
    /// `Some(effect)` if found, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_manager::{ReactionManager, Effect};
    /// use rustgram_message_effect_id::MessageEffectId;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = ReactionManager::new();
    /// let effect = Effect::new(
    ///     MessageEffectId::new(123),
    ///     "🎉",
    ///     FileId::new(1, 0),
    ///     FileId::new(2, 0),
    ///     FileId::new(3, 0),
    /// );
    /// manager.add_effect(effect.clone());
    ///
    /// let retrieved = manager.get_effect(MessageEffectId::new(123));
    /// assert_eq!(retrieved, Some(&effect));
    /// ```
    #[must_use]
    pub fn get_effect(&self, effect_id: MessageEffectId) -> Option<&Effect> {
        self.message_effects.get(&effect_id)
    }

    /// Returns all message effects.
    #[must_use]
    pub fn effects(&self) -> Vec<&Effect> {
        self.message_effects.values().collect()
    }

    /// Returns active message effect IDs.
    #[must_use]
    pub fn active_message_effects(&self) -> &[MessageEffectId] {
        &self.active_message_effects
    }

    /// Sets active message effects.
    pub fn set_active_message_effects(&mut self, effects: Vec<MessageEffectId>) {
        self.active_message_effects = effects;
    }

    /// Gets saved messages tags for a topic.
    ///
    /// # Arguments
    ///
    /// * `topic_id` - The topic ID
    ///
    /// # Returns
    ///
    /// `Some(tags)` if found, `None` otherwise
    #[must_use]
    pub fn get_saved_messages_tags(
        &self,
        topic_id: SavedMessagesTopicId,
    ) -> Option<&SavedReactionTags> {
        self.topic_tags.get(&topic_id)
    }

    /// Gets or creates saved messages tags for a topic.
    pub fn get_or_create_saved_messages_tags(
        &mut self,
        topic_id: SavedMessagesTopicId,
    ) -> &mut SavedReactionTags {
        self.topic_tags.entry(topic_id).or_default()
    }

    /// Updates saved messages tags for a topic.
    ///
    /// # Arguments
    ///
    /// * `topic_id` - The topic ID
    /// * `old_tags` - Previous tags
    /// * `new_tags` - New tags
    ///
    /// # Returns
    ///
    /// `true` if tags were modified, `false` otherwise
    pub fn update_saved_messages_tags(
        &mut self,
        topic_id: SavedMessagesTopicId,
        old_tags: &[ReactionType],
        new_tags: &[ReactionType],
    ) -> bool {
        let tags = self.get_or_create_saved_messages_tags(topic_id);
        tags.update_saved_messages_tags(old_tags, new_tags)
    }

    /// Sets a saved messages tag title.
    ///
    /// # Arguments
    ///
    /// * `topic_id` - The topic ID
    /// * `reaction_type` - The reaction type
    /// * `title` - The new title
    pub fn set_saved_messages_tag_title(
        &mut self,
        topic_id: SavedMessagesTopicId,
        reaction_type: &ReactionType,
        title: &str,
    ) -> Result<()> {
        let tags = self.get_or_create_saved_messages_tags(topic_id);
        tags.set_tag_title(reaction_type, title)
    }

    /// Returns the default paid reaction type.
    #[must_use]
    pub const fn default_paid_type(&self) -> &PaidReactionType {
        &self.default_paid_type
    }

    /// Sets the default paid reaction type.
    pub fn set_default_paid_type(&mut self, paid_type: PaidReactionType) {
        self.default_paid_type = paid_type;
    }

    /// Gets sorted available reactions.
    ///
    /// This method returns reactions sorted by:
    /// 1. Active reactions first
    /// 2. Recent reactions next
    /// 3. Other available reactions last
    ///
    /// # Arguments
    ///
    /// * `available` - Available reactions from the chat
    /// * `active` - Active reactions in the chat
    ///
    /// # Returns
    ///
    /// Sorted list of available reactions
    #[must_use]
    pub fn get_sorted_available_reactions(
        &self,
        available: &ChatReactions,
        active: &ChatReactions,
    ) -> Vec<ReactionType> {
        let mut result = Vec::new();

        // Add active reactions first
        for reaction in available.reaction_types() {
            if active.is_allowed(reaction) {
                result.push(reaction.clone());
            }
        }

        // Add recent reactions not already in result
        for reaction in &self.recent_reactions {
            if available.is_allowed(reaction) && !result.contains(reaction) {
                result.push(reaction.clone());
            }
        }

        // Add remaining available reactions
        for reaction in available.reaction_types() {
            if !result.iter().any(|r| r == reaction) {
                result.push(reaction.clone());
            }
        }

        result
    }
}

impl Default for ReactionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{DialogId, UserId};

    // === Effect Tests ===

    #[test]
    fn test_effect_new() {
        let effect = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );
        assert_eq!(effect.id(), MessageEffectId::new(123));
        assert_eq!(effect.emoji(), "🎉");
        assert!(!effect.is_premium());
        assert!(!effect.is_sticker());
    }

    #[test]
    fn test_effect_premium() {
        let effect = Effect::premium(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );
        assert!(effect.is_premium());
    }

    #[test]
    fn test_effect_is_sticker() {
        let effect_with_animation = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );
        assert!(!effect_with_animation.is_sticker());

        let effect_sticker = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::empty(),
        );
        assert!(effect_sticker.is_sticker());
    }

    #[test]
    fn test_effect_is_valid() {
        let valid = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );
        assert!(valid.is_valid());

        let invalid_id = Effect::new(
            MessageEffectId::new(0),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );
        assert!(!invalid_id.is_valid());

        let invalid_sticker = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::empty(),
            FileId::new(3, 0),
        );
        assert!(!invalid_sticker.is_valid());
    }

    // === SavedReactionTag Tests ===

    #[test]
    fn test_saved_reaction_tag_new() {
        let tag = SavedReactionTag::new(ReactionType::emoji("❤️"), "Favorites", 10);
        assert_eq!(tag.reaction_type(), &ReactionType::emoji("❤️"));
        assert_eq!(tag.title(), "Favorites");
        assert_eq!(tag.count(), 10);
        assert!(tag.is_valid());
    }

    #[test]
    fn test_saved_reaction_tag_set_title() {
        let mut tag = SavedReactionTag::new(ReactionType::emoji("❤️"), "Favorites", 10);
        assert!(tag.set_title("New Title").is_ok());
        assert_eq!(tag.title(), "New Title");
    }

    #[test]
    fn test_saved_reaction_tag_set_title_too_long() {
        let mut tag = SavedReactionTag::new(ReactionType::emoji("❤️"), "Favorites", 10);
        let long_title = "a".repeat(MAX_TAG_TITLE_LENGTH as usize + 1);
        assert!(tag.set_title(long_title).is_err());
    }

    #[test]
    fn test_saved_reaction_tag_is_valid() {
        let valid = SavedReactionTag::new(ReactionType::emoji("❤️"), "Favorites", 10);
        assert!(valid.is_valid());

        let valid_empty_title = SavedReactionTag::new(ReactionType::emoji("❤️"), "", 10);
        assert!(valid_empty_title.is_valid());

        let valid_zero_count = SavedReactionTag::new(ReactionType::emoji("❤️"), "Title", 0);
        assert!(valid_zero_count.is_valid());

        let invalid_both_empty = SavedReactionTag::new(ReactionType::emoji(""), "", 0);
        assert!(!invalid_both_empty.is_valid());
    }

    // === SavedReactionTags Tests ===

    #[test]
    fn test_saved_reaction_tags_new() {
        let tags = SavedReactionTags::new();
        assert!(tags.tags().is_empty());
        assert!(!tags.is_inited());
    }

    #[test]
    fn test_saved_reaction_tags_update() {
        let mut tags = SavedReactionTags::new();
        let old_tags = vec![ReactionType::emoji("❤️")];
        let new_tags = vec![ReactionType::emoji("👍")];

        let modified = tags.update_saved_messages_tags(&old_tags, &new_tags);
        assert!(modified);
    }

    #[test]
    fn test_saved_reaction_tags_set_title() {
        let mut tags = SavedReactionTags::new();
        tags.tags
            .push(SavedReactionTag::new(ReactionType::emoji("❤️"), "Old", 10));

        assert!(tags
            .set_tag_title(&ReactionType::emoji("❤️"), "New")
            .is_ok());
        assert_eq!(tags.tags()[0].title(), "New");
    }

    #[test]
    fn test_saved_reaction_tags_set_title_not_found() {
        let mut tags = SavedReactionTags::new();
        assert!(tags
            .set_tag_title(&ReactionType::emoji("❤️"), "New")
            .is_err());
    }

    #[test]
    fn test_saved_reaction_tags_sorted() {
        let mut tags = SavedReactionTags::new();
        tags.tags
            .push(SavedReactionTag::new(ReactionType::emoji("❤️"), "", 5));
        tags.tags
            .push(SavedReactionTag::new(ReactionType::emoji("👍"), "", 10));

        let sorted = tags.sorted_tags();
        assert_eq!(sorted[0].count(), 10);
        assert_eq!(sorted[1].count(), 5);
    }

    // === ReactionManager Tests ===

    #[test]
    fn test_reaction_manager_new() {
        let manager = ReactionManager::new();
        assert!(manager.active_reactions().is_empty());
        assert!(manager.recent_reactions().is_empty());
        assert!(manager.effects().is_empty());
    }

    #[test]
    fn test_is_active_reaction() {
        let mut manager = ReactionManager::new();
        manager.set_active_reactions(vec![ReactionType::emoji("👍")]);

        assert!(manager.is_active_reaction(&ReactionType::emoji("👍")));
        assert!(!manager.is_active_reaction(&ReactionType::emoji("❤️")));
    }

    #[test]
    fn test_add_recent_reaction() {
        let mut manager = ReactionManager::new();
        manager.add_recent_reaction(ReactionType::emoji("👍"));

        assert_eq!(manager.recent_reactions().len(), 1);
        assert_eq!(manager.recent_reactions()[0], ReactionType::emoji("👍"));
    }

    #[test]
    fn test_add_recent_duplicate() {
        let mut manager = ReactionManager::new();
        manager.add_recent_reaction(ReactionType::emoji("👍"));
        manager.add_recent_reaction(ReactionType::emoji("❤️"));
        manager.add_recent_reaction(ReactionType::emoji("👍"));

        // 👍 should be at the front after being added again
        assert_eq!(manager.recent_reactions()[0], ReactionType::emoji("👍"));
        assert_eq!(manager.recent_reactions()[1], ReactionType::emoji("❤️"));
    }

    #[test]
    fn test_clear_recent_reactions() {
        let mut manager = ReactionManager::new();
        manager.add_recent_reaction(ReactionType::emoji("👍"));
        manager.clear_recent_reactions();

        assert!(manager.recent_reactions().is_empty());
    }

    #[test]
    fn test_add_effect() {
        let mut manager = ReactionManager::new();
        let effect = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );

        manager.add_effect(effect.clone());
        assert_eq!(manager.effects().len(), 1);
    }

    #[test]
    fn test_get_effect() {
        let mut manager = ReactionManager::new();
        let effect = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );

        manager.add_effect(effect.clone());
        let retrieved = manager.get_effect(MessageEffectId::new(123));

        assert_eq!(retrieved, Some(&effect));
    }

    #[test]
    fn test_get_effect_not_found() {
        let manager = ReactionManager::new();
        let retrieved = manager.get_effect(MessageEffectId::new(123));

        assert!(retrieved.is_none());
    }

    #[test]
    fn test_get_saved_messages_tags() {
        let manager = ReactionManager::new();
        let topic_id = SavedMessagesTopicId::new(DialogId::from_user(UserId::new(123456).unwrap()));

        assert!(manager.get_saved_messages_tags(topic_id).is_none());
    }

    #[test]
    fn test_get_or_create_saved_messages_tags() {
        let mut manager = ReactionManager::new();
        let topic_id = SavedMessagesTopicId::new(DialogId::from_user(UserId::new(123456).unwrap()));

        let tags = manager.get_or_create_saved_messages_tags(topic_id);
        assert!(!tags.is_inited());
    }

    #[test]
    fn test_update_saved_messages_tags() {
        let mut manager = ReactionManager::new();
        let topic_id = SavedMessagesTopicId::new(DialogId::from_user(UserId::new(123456).unwrap()));

        let old_tags = vec![ReactionType::emoji("❤️")];
        let new_tags = vec![ReactionType::emoji("👍")];

        let modified = manager.update_saved_messages_tags(topic_id, &old_tags, &new_tags);
        assert!(modified);
    }

    #[test]
    fn test_default_paid_type() {
        let manager = ReactionManager::new();
        assert_eq!(manager.default_paid_type(), &PaidReactionType::regular());
    }

    #[test]
    fn test_set_default_paid_type() {
        let mut manager = ReactionManager::new();
        manager.set_default_paid_type(PaidReactionType::anonymous());

        assert_eq!(manager.default_paid_type(), &PaidReactionType::anonymous());
    }

    #[test]
    fn test_get_sorted_available_reactions() {
        let mut manager = ReactionManager::new();
        manager.set_active_reactions(vec![ReactionType::emoji("👍")]);
        manager.add_recent_reaction(ReactionType::emoji("❤️"));

        let available = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::emoji("❤️"),
            ReactionType::emoji("😂"),
        ]);

        let active = ChatReactions::with_reactions(vec![ReactionType::emoji("👍")]);

        let sorted = manager.get_sorted_available_reactions(&available, &active);

        // 👍 should be first (active)
        assert_eq!(sorted[0], ReactionType::emoji("👍"));
        // ❤️ should be second (recent)
        assert_eq!(sorted[1], ReactionType::emoji("❤️"));
        // 😂 should be last (remaining)
        assert_eq!(sorted[2], ReactionType::emoji("😂"));
    }

    #[test]
    fn test_active_message_effects() {
        let mut manager = ReactionManager::new();
        let effects = vec![MessageEffectId::new(123), MessageEffectId::new(456)];

        manager.set_active_message_effects(effects);
        assert_eq!(manager.active_message_effects().len(), 2);
    }

    #[test]
    fn test_max_recent_reactions() {
        let mut manager = ReactionManager::new();

        // Add more than MAX_RECENT_REACTIONS
        for i in 0..=MAX_RECENT_REACTIONS {
            manager.add_recent_reaction(ReactionType::emoji(&format!("{}", i)));
        }

        // Should be capped at MAX_RECENT_REACTIONS
        assert_eq!(manager.recent_reactions().len(), MAX_RECENT_REACTIONS);
    }

    #[test]
    fn test_effect_equality() {
        let effect1 = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );

        let effect2 = Effect::new(
            MessageEffectId::new(123),
            "🎉",
            FileId::new(1, 0),
            FileId::new(2, 0),
            FileId::new(3, 0),
        );

        assert_eq!(effect1, effect2);
    }

    #[test]
    fn test_saved_reaction_tag_count_clamp() {
        let tag = SavedReactionTag::new(ReactionType::emoji("❤️"), "", -10);
        // Count should be clamped to 0
        assert_eq!(tag.count(), 0);
    }

    #[test]
    fn test_reaction_manager_default() {
        let manager = ReactionManager::default();
        assert!(manager.active_reactions().is_empty());
        assert!(manager.recent_reactions().is_empty());
    }

    #[test]
    fn test_saved_reaction_tags_default() {
        let tags = SavedReactionTags::default();
        assert!(tags.tags().is_empty());
        assert!(!tags.is_inited());
    }
}
