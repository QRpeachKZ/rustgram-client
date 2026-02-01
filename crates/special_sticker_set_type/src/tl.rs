// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! TL (Type Language) placeholders for special sticker set types.
//!
//! This module contains placeholder types for TL constructors that will be
//! implemented when the full TL layer is added to the project.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

/// Input sticker set identifier.
///
/// This is a placeholder enum for future TL implementation.
/// The actual TL types from Telegram will be implemented in the tl layer.
///
/// # Variants
///
/// - `InputStickerSetEmpty` - Empty sticker set
/// - `InputStickerSetId` - Sticker set by ID
/// - `InputStickerSetShortName` - Sticker set by short name
/// - `InputStickerSetAnimatedEmoji` - Animated emoji sticker set
/// - `InputStickerSetAnimatedEmojiClick` - Animated emoji click sticker set
/// - `InputStickerSetAnimatedDice` - Animated dice sticker set
/// - `InputStickerSetPremiumGifts` - Premium gifts sticker set
/// - `InputStickerSetGenericAnimations` - Generic animations sticker set
/// - `InputStickerSetGenericEffects` - Generic effects sticker set
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputStickerSet {
    /// Empty sticker set
    Empty,

    /// Sticker set by ID
    ///
    /// # Fields
    ///
    /// * `id` - Sticker set identifier
    /// * `access_hash` - Access hash for the sticker set
    Id {
        /// Sticker set identifier
        id: i64,
        /// Access hash for the sticker set
        access_hash: i64,
    },

    /// Sticker set by short name
    ///
    /// # Fields
    ///
    /// * `short_name` - Short name of the sticker set
    ShortName {
        /// Short name of the sticker set
        short_name: String,
    },

    /// Animated emoji sticker set
    AnimatedEmoji,

    /// Animated emoji click sticker set
    AnimatedEmojiClick {
        /// Emoji character that triggers the animation
        emoji: String,
    },

    /// Animated dice sticker set
    ///
    /// # Fields
    ///
    /// * `emoji` - Emoji representing the dice type (e.g., "", "", "")
    AnimatedDice {
        /// Emoji character for the dice type
        emoji: String,
    },

    /// Premium gifts sticker set
    PremiumGifts,

    /// Generic animations sticker set
    GenericAnimations,

    /// Generic effects sticker set
    GenericEffects,
}
