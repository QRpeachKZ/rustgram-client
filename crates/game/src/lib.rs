// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

//! # Game
//!
//! Represents a Telegram bot game.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `Game` class from
//! `td/telegram/Game.h`.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_file_id::FileId;
use rustgram_formatted_text::FormattedText;
use rustgram_user_id::UserId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Telegram bot game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    id: i64,
    access_hash: i64,
    bot_user_id: UserId,
    short_name: String,
    title: String,
    description: String,
    animation_file_id: FileId,
    text: FormattedText,
}

impl Game {
    /// Creates a new Game.
    #[must_use]
    pub fn new(id: i64, access_hash: i64, bot_user_id: UserId, short_name: String) -> Self {
        Self {
            id,
            access_hash,
            bot_user_id,
            short_name,
            title: String::new(),
            description: String::new(),
            animation_file_id: FileId::empty(),
            text: FormattedText::new(""),
        }
    }

    /// Returns `true` if the game is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id == 0
    }

    /// Returns the bot user ID.
    #[must_use]
    pub const fn bot_user_id(&self) -> UserId {
        self.bot_user_id
    }

    /// Returns the game text.
    #[must_use]
    pub const fn text(&self) -> &FormattedText {
        &self.text
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            id: 0,
            access_hash: 0,
            bot_user_id: UserId::default(),
            short_name: String::new(),
            title: String::new(),
            description: String::new(),
            animation_file_id: FileId::empty(),
            text: FormattedText::new(""),
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game({})", self.short_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game = Game::new(123, 456, UserId::default(), "test_game".to_string());
        assert_eq!(game.id, 123);
        assert_eq!(game.short_name, "test_game");
    }

    #[test]
    fn test_default() {
        let game = Game::default();
        assert!(game.is_empty());
    }

    #[test]
    fn test_is_empty_true() {
        let game = Game::default();
        assert!(game.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let game = Game::new(123, 456, UserId::default(), "test".to_string());
        assert!(!game.is_empty());
    }

    #[test]
    fn test_bot_user_id() {
        let game = Game::new(123, 456, UserId::default(), "test".to_string());
        assert_eq!(game.bot_user_id(), UserId::default());
    }

    #[test]
    fn test_text() {
        let game = Game::new(123, 456, UserId::default(), "test".to_string());
        assert_eq!(game.text().text(), "");
    }

    #[test]
    fn test_short_name() {
        let game = Game::new(123, 456, UserId::default(), "my_game".to_string());
        assert_eq!(game.short_name, "my_game");
    }

    #[test]
    fn test_display() {
        let game = Game::new(123, 456, UserId::default(), "test".to_string());
        let display = format!("{}", game);
        assert!(display.contains("test"));
    }

    #[test]
    fn test_equality() {
        let g1 = Game::new(123, 456, UserId::default(), "test".to_string());
        let g2 = Game::new(123, 456, UserId::default(), "test".to_string());
        assert_eq!(g1, g2);
    }

    #[test]
    fn test_inequality() {
        let g1 = Game::new(123, 456, UserId::default(), "test1".to_string());
        let g2 = Game::new(124, 456, UserId::default(), "test2".to_string());
        assert_ne!(g1, g2);
    }
}
