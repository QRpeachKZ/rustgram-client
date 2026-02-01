// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Business Manager
//!
//! Manages business account settings and connected bots for Telegram.
//!
//! This module provides functionality for:
//! - Managing connected business bots
//! - Creating and managing business chat links
//! - Setting business location, work hours, and messages
//! - Controlling bot behavior in specific dialogs
//!
//! ## TDLib Correspondence
//!
//! Based on TDLib's `BusinessManager` from `td/telegram/BusinessManager.h`
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_business_manager::BusinessManager;
//! use rustgram_types::UserId;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let manager = BusinessManager::new();
//!
//! // Set business location
//! // let location = DialogLocation::new(40.7128, -74.0060, Some("New York".to_string()));
//! // manager.set_business_location(location).await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::collections::HashSet;
use std::sync::Arc;

use rustgram_business_away_message::BusinessAwayMessage;
use rustgram_business_connected_bot::BusinessConnectedBot;
use rustgram_business_greeting_message::BusinessGreetingMessage;
use rustgram_business_intro::BusinessIntro;
use rustgram_dialog_id::DialogId;
use rustgram_input_business_chat_link::InputBusinessChatLink;
use rustgram_types::UserId;
use thiserror::Error;
use tokio::sync::RwLock;

mod types;

pub use types::{BusinessChatLink, BusinessChatLinkInfo};

/// Error type for business manager operations.
#[derive(Debug, Clone, Error)]
pub enum Error {
    /// No bot is currently connected
    #[error("No connected bot found")]
    NoConnectedBot,

    /// Bot not found
    #[error("Bot not found: {0}")]
    BotNotFound(UserId),

    /// Chat link not found
    #[error("Chat link not found: {0}")]
    LinkNotFound(String),

    /// Invalid dialog ID
    #[error("Invalid dialog ID")]
    InvalidDialogId,

    /// Network error occurred
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Internal error occurred
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for business manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Collection of business chat links.
///
/// Contains multiple business chat links.
#[derive(Debug, Clone, PartialEq)]
pub struct BusinessChatLinks {
    /// List of chat links
    pub links: Vec<BusinessChatLink>,
}

impl BusinessChatLinks {
    /// Creates a new empty collection of chat links.
    pub fn new() -> Self {
        Self { links: vec![] }
    }

    /// Creates a collection from a list of links.
    pub fn from_links(links: Vec<BusinessChatLink>) -> Self {
        Self { links }
    }

    /// Returns true if there are no links.
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    /// Returns the number of links.
    pub fn len(&self) -> usize {
        self.links.len()
    }
}

impl Default for BusinessChatLinks {
    fn default() -> Self {
        Self::new()
    }
}

/// Business account manager.
///
/// Manages all aspects of a Telegram business account including:
/// - Connected bot configuration
/// - Business chat links
/// - Account settings (location, work hours, messages)
/// - Per-dialog bot controls
///
/// # Thread Safety
///
/// This manager is thread-safe and can be safely shared across threads.
/// All internal state is protected by `Arc<RwLock<T>>`.
#[derive(Debug, Clone)]
pub struct BusinessManager {
    /// Currently connected business bot
    connected_bot: Arc<RwLock<Option<BusinessConnectedBot>>>,
    /// Business chat links
    chat_links: Arc<RwLock<Vec<BusinessChatLink>>>,
    /// Dialogs where bot is paused
    paused_dialogs: Arc<RwLock<HashSet<DialogId>>>,
}

impl BusinessManager {
    /// Creates a new business manager.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    ///
    /// let manager = BusinessManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            connected_bot: Arc::new(RwLock::new(None)),
            chat_links: Arc::new(RwLock::new(Vec::new())),
            paused_dialogs: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Gets the currently connected business bot.
    ///
    /// # Returns
    ///
    /// Future yielding the connected bot info, or error if no bot is connected
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let result = manager.get_business_connected_bot().await;
    /// assert!(matches!(result, Err(_)));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_business_connected_bot(&self) -> Result<BusinessConnectedBot> {
        let bot = self.connected_bot.read().await;
        bot.as_ref().cloned().ok_or(Error::NoConnectedBot)
    }

    /// Sets or updates the connected business bot.
    ///
    /// # Arguments
    ///
    /// * `bot` - Bot configuration with recipients and rights
    ///
    /// # Returns
    ///
    /// Future completing when bot is set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let bot = BusinessConnectedBot::new(
    ///     UserId::new(123456789)?,
    ///     BusinessRecipients::new(),
    ///     BusinessBotRights::new(),
    /// );
    ///
    /// manager.set_business_connected_bot(bot).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_business_connected_bot(&self, bot: BusinessConnectedBot) -> Result<()> {
        let mut connected_bot = self.connected_bot.write().await;
        *connected_bot = Some(bot);
        Ok(())
    }

    /// Deletes the connected business bot.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The bot to disconnect
    ///
    /// # Returns
    ///
    /// Future completing when bot is removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let bot_id = UserId::new(123456789)?;
    ///
    /// manager.delete_business_connected_bot(bot_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_business_connected_bot(&self, bot_user_id: UserId) -> Result<()> {
        let mut connected_bot = self.connected_bot.write().await;
        if let Some(bot) = connected_bot.as_ref() {
            if bot.user_id() == bot_user_id {
                *connected_bot = None;
                return Ok(());
            }
        }
        Err(Error::BotNotFound(bot_user_id))
    }

    /// Toggles whether the bot is paused in a specific dialog.
    ///
    /// When paused, the connected bot will not respond in this dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog to modify
    /// * `is_paused` - Whether to pause (true) or resume (false)
    ///
    /// # Returns
    ///
    /// Future completing when state is updated
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let dialog_id = DialogId::new(123456);
    ///
    /// // Pause bot in dialog
    /// manager.toggle_business_connected_bot_dialog_is_paused(dialog_id, true).await?;
    ///
    /// // Resume bot in dialog
    /// manager.toggle_business_connected_bot_dialog_is_paused(dialog_id, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_business_connected_bot_dialog_is_paused(
        &self,
        dialog_id: DialogId,
        is_paused: bool,
    ) -> Result<()> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialogId);
        }

        let mut paused_dialogs = self.paused_dialogs.write().await;
        if is_paused {
            paused_dialogs.insert(dialog_id);
        } else {
            paused_dialogs.remove(&dialog_id);
        }
        Ok(())
    }

    /// Removes the business bot from a specific dialog.
    ///
    /// The bot will no longer have access to this dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog to remove bot from
    ///
    /// # Returns
    ///
    /// Future completing when bot is removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let dialog_id = DialogId::new(123456);
    ///
    /// manager.remove_business_connected_bot_from_dialog(dialog_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_business_connected_bot_from_dialog(
        &self,
        dialog_id: DialogId,
    ) -> Result<()> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialogId);
        }

        let mut paused_dialogs = self.paused_dialogs.write().await;
        paused_dialogs.remove(&dialog_id);
        Ok(())
    }

    /// Gets all business chat links for the account.
    ///
    /// # Returns
    ///
    /// Future yielding list of chat links
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let links = manager.get_business_chat_links().await?;
    /// assert!(links.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_business_chat_links(&self) -> Result<BusinessChatLinks> {
        let chat_links = self.chat_links.read().await;
        Ok(BusinessChatLinks {
            links: chat_links.clone(),
        })
    }

    /// Creates a new business chat link.
    ///
    /// # Arguments
    ///
    /// * `link_info` - Link configuration with text and title
    ///
    /// # Returns
    ///
    /// Future yielding created link info
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_input_business_chat_link::InputBusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let link_info = InputBusinessChatLink::new(
    ///     FormattedText::new("Welcome!"),
    ///     "Support".to_string(),
    /// );
    ///
    /// let link = manager.create_business_chat_link(link_info).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_business_chat_link(
        &self,
        link_info: InputBusinessChatLink,
    ) -> Result<BusinessChatLink> {
        let mut chat_links = self.chat_links.write().await;

        // Generate a unique link URL
        let link_url = format!("https://t.me/{}", generate_random_string(8));

        let new_link = BusinessChatLink {
            link: link_url.clone(),
            title: link_info.title().to_string(),
            view_count: 0,
            click_count: 0,
            created_date: 0,
        };

        chat_links.push(new_link.clone());
        Ok(new_link)
    }

    /// Edits an existing business chat link.
    ///
    /// # Arguments
    ///
    /// * `link` - The link URL to edit
    /// * `link_info` - New link configuration
    ///
    /// # Returns
    ///
    /// Future yielding updated link info
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_input_business_chat_link::InputBusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let link_info = InputBusinessChatLink::new(
    ///     FormattedText::new("Updated text"),
    ///     "New Title".to_string(),
    /// );
    ///
    /// // Note: This will fail if link doesn't exist
    /// let result = manager.edit_business_chat_link("https://t.me/abc123".to_string(), link_info).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_business_chat_link(
        &self,
        link: String,
        link_info: InputBusinessChatLink,
    ) -> Result<BusinessChatLink> {
        let mut chat_links = self.chat_links.write().await;

        if let Some(chat_link) = chat_links.iter_mut().find(|l| l.link == link) {
            chat_link.title = link_info.title().to_string();
            return Ok(chat_link.clone());
        }

        Err(Error::LinkNotFound(link))
    }

    /// Deletes a business chat link.
    ///
    /// # Arguments
    ///
    /// * `link` - The link URL to delete
    ///
    /// # Returns
    ///
    /// Future completing when link is deleted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    ///
    /// // Note: This will fail if link doesn't exist
    /// let result = manager.delete_business_chat_link("https://t.me/abc123".to_string()).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_business_chat_link(&self, link: String) -> Result<()> {
        let mut chat_links = self.chat_links.write().await;

        let original_len = chat_links.len();
        chat_links.retain(|l| l.link != link);

        if chat_links.len() < original_len {
            Ok(())
        } else {
            Err(Error::LinkNotFound(link))
        }
    }

    /// Gets information and statistics for a business chat link.
    ///
    /// # Arguments
    ///
    /// * `link` - The link URL to query
    ///
    /// # Returns
    ///
    /// Future yielding link info with statistics
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    ///
    /// // Note: This will fail if link doesn't exist
    /// let result = manager.get_business_chat_link_info("https://t.me/abc123".to_string()).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_business_chat_link_info(&self, link: String) -> Result<BusinessChatLinkInfo> {
        let chat_links = self.chat_links.read().await;

        chat_links
            .iter()
            .find(|l| l.link == link)
            .map(|l| BusinessChatLinkInfo {
                link: l.link.clone(),
                created_date: l.created_date,
                click_count: l.click_count,
            })
            .ok_or(Error::LinkNotFound(link))
    }

    /// Sets the business greeting message for new chats.
    ///
    /// # Arguments
    ///
    /// * `greeting_message` - Greeting message configuration
    ///
    /// # Returns
    ///
    /// Future completing when greeting is set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_business_greeting_message::BusinessGreetingMessage;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let greeting = BusinessGreetingMessage::new();
    ///
    /// manager.set_business_greeting_message(greeting).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_business_greeting_message(
        &self,
        greeting_message: BusinessGreetingMessage,
    ) -> Result<()> {
        // In a real implementation, this would update the greeting message
        // For now, we just validate it
        if !greeting_message.is_empty() {
            Ok(())
        } else {
            Err(Error::InternalError(
                "Greeting message cannot be empty".to_string(),
            ))
        }
    }

    /// Sets the automatic away message for outside work hours.
    ///
    /// # Arguments
    ///
    /// * `away_message` - Away message configuration with schedule
    ///
    /// # Returns
    ///
    /// Future completing when away message is set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_business_away_message::BusinessAwayMessage;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let away_msg = BusinessAwayMessage::new();
    ///
    /// manager.set_business_away_message(away_msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_business_away_message(&self, away_message: BusinessAwayMessage) -> Result<()> {
        // In a real implementation, this would update the away message
        if !away_message.is_empty() {
            Ok(())
        } else {
            Err(Error::InternalError(
                "Away message cannot be empty".to_string(),
            ))
        }
    }

    /// Sets the business introduction/start page.
    ///
    /// # Arguments
    ///
    /// * `intro` - Introduction with title, description, optional sticker
    ///
    /// # Returns
    ///
    /// Future completing when intro is set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_manager::BusinessManager;
    /// use rustgram_business_intro::BusinessIntro;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessManager::new();
    /// let intro = BusinessIntro::new();
    ///
    /// manager.set_business_intro(intro).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_business_intro(&self, _intro: BusinessIntro) -> Result<()> {
        // In a real implementation, this would update the intro
        Ok(())
    }
}

impl Default for BusinessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Generates a random string for link URLs.
fn generate_random_string(length: usize) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Simple random string based on time
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let mut result = String::new();

    let mut seed = timestamp;
    for _ in 0..length {
        let idx = (seed % chars.len() as u128) as usize;
        result.push(chars[idx]);
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_business_connected_bot::{BusinessBotRights, BusinessRecipients};
    use rustgram_formatted_text::FormattedText;

    #[tokio::test]
    async fn test_manager_new() {
        let manager = BusinessManager::new();
        assert!(manager.connected_bot.read().await.is_none());
        assert!(manager.chat_links.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = BusinessManager::default();
        assert!(manager.connected_bot.read().await.is_none());
    }

    #[tokio::test]
    async fn test_get_connected_bot_when_none() {
        let manager = BusinessManager::new();
        let result = manager.get_business_connected_bot().await;
        assert!(matches!(result, Err(Error::NoConnectedBot)));
    }

    #[tokio::test]
    async fn test_set_connected_bot() {
        let manager = BusinessManager::new();
        let bot = BusinessConnectedBot::new(
            UserId::new(123456789).expect("valid"),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        let result = manager.set_business_connected_bot(bot.clone()).await;
        assert!(result.is_ok());

        let retrieved = manager.get_business_connected_bot().await;
        assert!(retrieved.is_ok());
        assert_eq!(
            retrieved.unwrap().user_id(),
            UserId::new(123456789).expect("valid")
        );
    }

    #[tokio::test]
    async fn test_delete_connected_bot() {
        let manager = BusinessManager::new();
        let bot = BusinessConnectedBot::new(
            UserId::new(123456789).expect("valid"),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        manager.set_business_connected_bot(bot).await.unwrap();
        let result = manager
            .delete_business_connected_bot(UserId::new(123456789).expect("valid"))
            .await;

        assert!(result.is_ok());
        assert!(manager.connected_bot.read().await.is_none());
    }

    #[tokio::test]
    async fn test_delete_connected_bot_not_found() {
        let manager = BusinessManager::new();
        let result = manager
            .delete_business_connected_bot(UserId::new(999999).expect("valid"))
            .await;

        assert!(matches!(result, Err(Error::BotNotFound(_))));
    }

    #[tokio::test]
    async fn test_toggle_dialog_paused() {
        let manager = BusinessManager::new();
        let dialog_id = DialogId::new(123456);

        manager
            .toggle_business_connected_bot_dialog_is_paused(dialog_id, true)
            .await
            .unwrap();

        let paused = manager.paused_dialogs.read().await;
        assert!(paused.contains(&dialog_id));
    }

    #[tokio::test]
    async fn test_toggle_dialog_resumed() {
        let manager = BusinessManager::new();
        let dialog_id = DialogId::new(123456);

        manager
            .toggle_business_connected_bot_dialog_is_paused(dialog_id, true)
            .await
            .unwrap();

        manager
            .toggle_business_connected_bot_dialog_is_paused(dialog_id, false)
            .await
            .unwrap();

        let paused = manager.paused_dialogs.read().await;
        assert!(!paused.contains(&dialog_id));
    }

    #[tokio::test]
    async fn test_toggle_dialog_invalid_id() {
        let manager = BusinessManager::new();
        let invalid_id = DialogId::new(0);

        let result = manager
            .toggle_business_connected_bot_dialog_is_paused(invalid_id, true)
            .await;

        assert!(matches!(result, Err(Error::InvalidDialogId)));
    }

    #[tokio::test]
    async fn test_remove_bot_from_dialog() {
        let manager = BusinessManager::new();
        let dialog_id = DialogId::new(123456);

        manager
            .toggle_business_connected_bot_dialog_is_paused(dialog_id, true)
            .await
            .unwrap();

        manager
            .remove_business_connected_bot_from_dialog(dialog_id)
            .await
            .unwrap();

        let paused = manager.paused_dialogs.read().await;
        assert!(!paused.contains(&dialog_id));
    }

    #[tokio::test]
    async fn test_get_chat_links_empty() {
        let manager = BusinessManager::new();
        let links = manager.get_business_chat_links().await.unwrap();
        assert!(links.is_empty());
    }

    #[tokio::test]
    async fn test_create_chat_link() {
        let manager = BusinessManager::new();
        let link_info =
            InputBusinessChatLink::new(FormattedText::new("Welcome"), "Support".to_string());

        let result = manager.create_business_chat_link(link_info).await;
        assert!(result.is_ok());

        let link = result.unwrap();
        assert!(!link.link.is_empty());
        assert_eq!(link.title, "Support");
        assert_eq!(link.view_count, 0);
    }

    #[tokio::test]
    async fn test_edit_chat_link() {
        let manager = BusinessManager::new();
        let link_info =
            InputBusinessChatLink::new(FormattedText::new("Welcome"), "Support".to_string());

        let created = manager.create_business_chat_link(link_info).await.unwrap();

        let new_link_info =
            InputBusinessChatLink::new(FormattedText::new("Updated"), "Sales".to_string());

        let result = manager
            .edit_business_chat_link(created.link.clone(), new_link_info)
            .await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.title, "Sales");
    }

    #[tokio::test]
    async fn test_edit_chat_link_not_found() {
        let manager = BusinessManager::new();
        let link_info =
            InputBusinessChatLink::new(FormattedText::new("Welcome"), "Support".to_string());

        let result = manager
            .edit_business_chat_link("https://t.me/nonexistent".to_string(), link_info)
            .await;

        assert!(matches!(result, Err(Error::LinkNotFound(_))));
    }

    #[tokio::test]
    async fn test_delete_chat_link() {
        let manager = BusinessManager::new();
        let link_info =
            InputBusinessChatLink::new(FormattedText::new("Welcome"), "Support".to_string());

        let created = manager.create_business_chat_link(link_info).await.unwrap();

        let result = manager
            .delete_business_chat_link(created.link.clone())
            .await;
        assert!(result.is_ok());

        let links = manager.get_business_chat_links().await.unwrap();
        assert!(links.is_empty());
    }

    #[tokio::test]
    async fn test_delete_chat_link_not_found() {
        let manager = BusinessManager::new();
        let result = manager
            .delete_business_chat_link("https://t.me/nonexistent".to_string())
            .await;

        assert!(matches!(result, Err(Error::LinkNotFound(_))));
    }

    #[tokio::test]
    async fn test_get_chat_link_info() {
        let manager = BusinessManager::new();
        let link_info =
            InputBusinessChatLink::new(FormattedText::new("Welcome"), "Support".to_string());

        let created = manager.create_business_chat_link(link_info).await.unwrap();

        let result = manager
            .get_business_chat_link_info(created.link.clone())
            .await;

        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.link, created.link);
    }

    #[tokio::test]
    async fn test_get_chat_link_info_not_found() {
        let manager = BusinessManager::new();
        let result = manager
            .get_business_chat_link_info("https://t.me/nonexistent".to_string())
            .await;

        assert!(matches!(result, Err(Error::LinkNotFound(_))));
    }

    #[tokio::test]
    async fn test_set_greeting_message() {
        let manager = BusinessManager::new();
        let greeting = BusinessGreetingMessage::new();

        // Empty greeting should fail
        let result = manager.set_business_greeting_message(greeting).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_away_message() {
        let manager = BusinessManager::new();
        let away_msg = BusinessAwayMessage::new();

        // Empty away message should fail
        let result = manager.set_business_away_message(away_msg).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_business_intro() {
        let manager = BusinessManager::new();
        let intro = BusinessIntro::new();

        let result = manager.set_business_intro(intro).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_settings_updates() {
        let manager = Arc::new(BusinessManager::new());
        let mut handles = vec![];

        // Spawn multiple concurrent tasks
        for i in 0..10 {
            let manager = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                let dialog_id = DialogId::new(i);
                let _ = manager
                    .toggle_business_connected_bot_dialog_is_paused(dialog_id, true)
                    .await;
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            handle.await.expect("task completed");
        }

        // Verify state is consistent
        let paused = manager.paused_dialogs.read().await;
        assert_eq!(paused.len(), 10);
    }

    #[tokio::test]
    async fn test_pause_multiple_dialogs() {
        let manager = BusinessManager::new();
        let dialog_ids = vec![DialogId::new(1), DialogId::new(2), DialogId::new(3)];

        for dialog_id in &dialog_ids {
            manager
                .toggle_business_connected_bot_dialog_is_paused(*dialog_id, true)
                .await
                .unwrap();
        }

        let paused = manager.paused_dialogs.read().await;
        assert_eq!(paused.len(), 3);
        for dialog_id in &dialog_ids {
            assert!(paused.contains(dialog_id));
        }
    }
}
