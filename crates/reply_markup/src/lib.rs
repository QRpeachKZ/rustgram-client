// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Reply Markup
//!
//! Reply markup for Telegram messages.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Type of reply markup.
///
/// Based on TDLib's `ReplyMarkup` class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplyMarkup {
    /// Force reply.
    ForceReply {
        /// Whether to force a reply.
        force: bool,
        /// Input field placeholder.
        input_field_placeholder: String,
    },
    /// Inline keyboard.
    InlineKeyboard(InlineKeyboardMarkup),
    /// Remove keyboard.
    RemoveKeyboard {
        /// Whether to be selective.
        is_personal: bool,
    },
    /// Reply keyboard.
    ReplyKeyboard(ReplyKeyboardMarkup),
    /// Show keyboard.
    ShowKeyboard {
        /// Rows of buttons.
        rows: Vec<Vec<KeyboardButton>>,
        /// Whether to resize.
        resize: bool,
        /// Whether to use once.
        one_time: bool,
        /// Whether to be personal.
        is_personal: bool,
        /// Input field placeholder.
        input_field_placeholder: String,
    },
}

impl Default for ReplyMarkup {
    fn default() -> Self {
        Self::RemoveKeyboard { is_personal: false }
    }
}

impl ReplyMarkup {
    /// Checks if this is inline keyboard.
    #[must_use]
    pub fn is_inline(&self) -> bool {
        matches!(self, Self::InlineKeyboard(_))
    }

    /// Checks if this forces a reply.
    #[must_use]
    pub fn is_force_reply(&self) -> bool {
        matches!(self, Self::ForceReply { .. })
    }

    /// Checks if this removes keyboard.
    #[must_use]
    pub fn is_remove(&self) -> bool {
        matches!(self, Self::RemoveKeyboard { .. })
    }
}

/// Inline keyboard markup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Rows of buttons.
    rows: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Creates a new inline keyboard.
    #[must_use]
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    /// Adds a row of buttons.
    pub fn add_row(&mut self, row: Vec<InlineKeyboardButton>) {
        self.rows.push(row);
    }

    /// Returns the rows.
    #[must_use]
    pub fn rows(&self) -> &[Vec<InlineKeyboardButton>] {
        &self.rows
    }

    /// Returns the number of buttons.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.iter().map(|row| row.len()).sum()
    }

    /// Checks if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl Default for InlineKeyboardMarkup {
    fn default() -> Self {
        Self::new()
    }
}

/// Inline keyboard button type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InlineKeyboardButton {
    /// Buy button.
    Buy,
    /// Callback game.
    CallbackGame,
    /// Callback with data.
    Callback {
        /// Button text.
        text: String,
        /// Callback data.
        data: Vec<u8>,
    },
    /// Callback with password.
    CallbackWithPassword {
        /// Button text.
        text: String,
        /// Callback data.
        data: Vec<u8>,
    },
    /// Login URL.
    LoginUrl {
        /// Button text.
        text: String,
        /// Login URL.
        url: String,
        /// Forward text.
        forward_text: String,
        /// Bot username.
        bot_username: String,
        /// Request write access.
        request_write_access: bool,
    },
    /// Switch inline.
    SwitchInline {
        /// Button text.
        text: String,
        /// Query in current chat.
        query_in_current_chat: bool,
        /// Query.
        query: String,
    },
    /// URL button.
    Url {
        /// Button text.
        text: String,
        /// URL.
        url: String,
        /// ID.
        id: i64,
    },
    /// User profile.
    UserProfile {
        /// Button text.
        text: String,
        /// User ID.
        user_id: i64,
    },
    /// Web app.
    WebApp {
        /// Button text.
        text: String,
        /// URL.
        url: String,
    },
}

/// Reply keyboard markup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    /// Rows of buttons.
    rows: Vec<Vec<KeyboardButton>>,
    /// Whether to resize.
    resize: bool,
    /// Whether to use once.
    one_time: bool,
    /// Whether to be personal.
    is_personal: bool,
}

impl ReplyKeyboardMarkup {
    /// Creates a new reply keyboard.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            resize: false,
            one_time: false,
            is_personal: false,
        }
    }

    /// Adds a row of buttons.
    pub fn add_row(&mut self, row: Vec<KeyboardButton>) {
        self.rows.push(row);
    }

    /// Sets resize.
    pub fn set_resize(&mut self, resize: bool) {
        self.resize = resize;
    }

    /// Sets one time.
    pub fn set_one_time(&mut self, one_time: bool) {
        self.one_time = one_time;
    }

    /// Sets is personal.
    pub fn set_is_personal(&mut self, is_personal: bool) {
        self.is_personal = is_personal;
    }

    /// Returns the rows.
    #[must_use]
    pub fn rows(&self) -> &[Vec<KeyboardButton>] {
        &self.rows
    }

    /// Returns whether to resize.
    #[must_use]
    pub const fn resize(&self) -> bool {
        self.resize
    }

    /// Returns whether to use once.
    #[must_use]
    pub const fn one_time(&self) -> bool {
        self.one_time
    }

    /// Returns whether to be personal.
    #[must_use]
    pub const fn is_personal(&self) -> bool {
        self.is_personal
    }
}

impl Default for ReplyKeyboardMarkup {
    fn default() -> Self {
        Self::new()
    }
}

/// Keyboard button type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyboardButton {
    /// Request contact.
    RequestContact {
        /// Button text.
        text: String,
    },
    /// Request geo location.
    RequestGeoLocation {
        /// Button text.
        text: String,
    },
    /// Request peer.
    RequestPeer {
        /// Button text.
        text: String,
        /// Button ID.
        button_id: i32,
        /// Request type.
        request_type: String,
        /// Max quantity.
        max_quantity: i32,
        /// Whether to request users.
        request_users: bool,
        /// Whether to request chats.
        request_chats: bool,
        /// Whether to request channels.
        request_channels: bool,
        /// Whether for group creation.
        for_group_creation: bool,
        /// Whether user is bot.
        user_is_bot: bool,
        /// Whether user is premium.
        user_is_premium: bool,
    },
    /// Request poll.
    RequestPoll {
        /// Button text.
        text: String,
        /// Force regular.
        force_regular: bool,
        /// Force quiz.
        force_quiz: bool,
    },
    /// Text button.
    Text {
        /// Button text.
        text: String,
    },
    /// Web app.
    WebApp {
        /// Button text.
        text: String,
        /// URL.
        url: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let markup = ReplyMarkup::default();
        assert!(markup.is_remove());
    }

    #[test]
    fn test_inline_keyboard() {
        let mut keyboard = InlineKeyboardMarkup::new();
        assert!(keyboard.is_empty());

        keyboard.add_row(vec![InlineKeyboardButton::Url {
            text: "Open".to_string(),
            url: "https://example.com".to_string(),
            id: 123,
        }]);

        assert_eq!(keyboard.len(), 1);
        assert!(!keyboard.is_empty());
    }

    #[test]
    fn test_reply_keyboard() {
        let mut keyboard = ReplyKeyboardMarkup::new();
        keyboard.set_resize(true);
        assert!(keyboard.resize());

        keyboard.add_row(vec![KeyboardButton::Text {
            text: "Button".to_string(),
        }]);

        assert_eq!(keyboard.rows().len(), 1);
    }

    #[test]
    fn test_markup_types() {
        let inline = ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new());
        assert!(inline.is_inline());

        let force = ReplyMarkup::ForceReply {
            force: true,
            input_field_placeholder: "Type...".to_string(),
        };
        assert!(force.is_force_reply());

        let remove = ReplyMarkup::RemoveKeyboard { is_personal: false };
        assert!(remove.is_remove());
    }

    #[test]
    fn test_keyboard_button_variants() {
        let text_btn = KeyboardButton::Text {
            text: "Click".to_string(),
        };
        assert!(matches!(text_btn, KeyboardButton::Text { .. }));

        let contact_btn = KeyboardButton::RequestContact {
            text: "Share Contact".to_string(),
        };
        assert!(matches!(contact_btn, KeyboardButton::RequestContact { .. }));
    }

    #[test]
    fn test_inline_button_variants() {
        let url_btn = InlineKeyboardButton::Url {
            text: "Open".to_string(),
            url: "https://example.com".to_string(),
            id: 123,
        };
        assert!(matches!(url_btn, InlineKeyboardButton::Url { .. }));

        let callback_btn = InlineKeyboardButton::Callback {
            text: "Click".to_string(),
            data: vec![1, 2, 3],
        };
        assert!(matches!(
            callback_btn,
            InlineKeyboardButton::Callback { .. }
        ));
    }
}
