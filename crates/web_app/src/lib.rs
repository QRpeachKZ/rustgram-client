// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Web App
//!
//! Web app integration for Telegram.
//!
//! ## Overview
//!
//! Represents Telegram Mini Apps (Web Apps).
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_web_app::WebApp;
//! use rustgram_types::UserId;
//!
//! let app = WebApp::new(UserId::new(123).unwrap(), "my_app".to_string());
//! assert_eq!(app.short_name(), "my_app");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Telegram Web App (Mini App)
///
/// Represents a web application integrated into Telegram.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebApp {
    /// Bot user ID that owns the app
    bot_user_id: UserId,
    /// Short name of the app
    short_name: String,
    /// Title of the app
    title: String,
    /// Description
    description: String,
    /// Photo URL
    photo_url: Option<String>,
    /// Animation URL
    animation_url: Option<String>,
    /// Hash
    hash: String,
}

impl WebApp {
    /// Creates a new web app
    #[must_use]
    pub fn new(bot_user_id: UserId, short_name: String) -> Self {
        Self {
            bot_user_id,
            short_name,
            title: String::new(),
            description: String::new(),
            photo_url: None,
            animation_url: None,
            hash: String::new(),
        }
    }

    /// Returns the bot user ID
    #[must_use]
    pub const fn bot_user_id(&self) -> UserId {
        self.bot_user_id
    }

    /// Returns the short name
    #[must_use]
    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    /// Returns the title
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Sets the title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Returns the description
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Sets the description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns the photo URL
    #[must_use]
    pub const fn photo_url(&self) -> Option<&String> {
        self.photo_url.as_ref()
    }

    /// Sets the photo URL
    pub fn set_photo_url(&mut self, url: String) {
        self.photo_url = Some(url);
    }

    /// Returns the animation URL
    #[must_use]
    pub const fn animation_url(&self) -> Option<&String> {
        self.animation_url.as_ref()
    }

    /// Sets the animation URL
    pub fn set_animation_url(&mut self, url: String) {
        self.animation_url = Some(url);
    }

    /// Returns the hash
    #[must_use]
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Sets the hash
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
}

impl fmt::Display for WebApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebApp({})", self.short_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_app_new() {
        let bot_id = UserId::new(123).unwrap();
        let app = WebApp::new(bot_id, "test_app".to_string());
        assert_eq!(app.bot_user_id(), bot_id);
        assert_eq!(app.short_name(), "test_app");
    }

    #[test]
    fn test_web_app_title() {
        let mut app = WebApp::new(UserId::new(123).unwrap(), "app".to_string());
        app.set_title("My App".to_string());
        assert_eq!(app.title(), "My App");
    }

    #[test]
    fn test_web_app_photo_url() {
        let mut app = WebApp::new(UserId::new(123).unwrap(), "app".to_string());
        assert!(app.photo_url().is_none());
        app.set_photo_url("https://example.com/photo.png".to_string());
        assert_eq!(
            app.photo_url(),
            Some(&"https://example.com/photo.png".to_string())
        );
    }

    #[test]
    fn test_web_app_display() {
        let app = WebApp::new(UserId::new(123).unwrap(), "my_app".to_string());
        assert_eq!(format!("{}", app), "WebApp(my_app)");
    }
}
