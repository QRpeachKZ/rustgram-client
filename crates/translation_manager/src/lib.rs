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

//! # Translation Manager
//!
//! Manages message translations.
//!
//! ## Overview
//!
//! This module provides functionality for translating message text
//! between different languages using Telegram's translation service.
//!
//! ## TDLib Correspondence
//!
//! Corresponds to `td/telegram/TranslationManager.h` in TDLib.
//!
//! ## Example
//!
//! ```rust
//! use rustgram-translation_manager::TranslationManager;
//! use rustgram-formatted-text::FormattedText;
//!
//! let manager = TranslationManager::new();
//! let text = FormattedText::new("Hello, world!");
//! let result = manager.translate_text(text, "es", false, 0, None);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_formatted_text::{FormattedText, MessageEntity};
use rustgram_message_full_id::MessageFullId;
use thiserror::Error;
use tracing::{debug, info};

/// Errors that can occur in the translation manager.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TranslationError {
    /// Language code is invalid or not supported
    #[error("Invalid language code: {0}")]
    InvalidLanguageCode(String),

    /// Text is empty
    #[error("Cannot translate empty text")]
    EmptyText,

    /// Translation service unavailable
    #[error("Translation service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Rate limit exceeded
    #[error("Translation rate limit exceeded")]
    RateLimitExceeded,

    /// Message ID is required but not provided
    #[error("Message ID is required for this translation")]
    MessageIdRequired,
}

/// Result type for translation operations.
pub type TranslationResult<T> = Result<T, TranslationError>;

/// Language code for translation.
///
/// Language codes follow ISO 639-1 standard (e.g., "en", "es", "fr").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageCode(String);

impl LanguageCode {
    /// Creates a new language code.
    ///
    /// # Arguments
    ///
    /// * `code` - The ISO 639-1 language code (2-3 letters)
    ///
    /// # Errors
    ///
    /// Returns an error if the code is invalid.
    pub fn new(code: &str) -> TranslationResult<Self> {
        let code = code.trim().to_lowercase();

        if code.is_empty() {
            return Err(TranslationError::InvalidLanguageCode(
                "Empty language code".to_string(),
            ));
        }

        // Basic validation for ISO 639-1 codes (2-3 letters)
        if !(2..=3).contains(&code.len()) || !code.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(TranslationError::InvalidLanguageCode(format!(
                "Invalid format: {}",
                code
            )));
        }

        Ok(Self(code))
    }

    /// Returns the language code as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether this is a valid language code.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        (2..=3).contains(&self.0.len()) && self.0.chars().all(|c| c.is_ascii_alphabetic())
    }
}

impl AsRef<str> for LanguageCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Translation request options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationOptions {
    /// Skip bot commands in the translation
    skip_bot_commands: bool,
    /// Maximum media timestamp to translate
    max_media_timestamp: i32,
}

impl Default for TranslationOptions {
    fn default() -> Self {
        Self {
            skip_bot_commands: true,
            max_media_timestamp: 0,
        }
    }
}

impl TranslationOptions {
    /// Creates new translation options.
    ///
    /// # Arguments
    ///
    /// * `skip_bot_commands` - Whether to skip bot commands
    /// * `max_media_timestamp` - Maximum media timestamp
    #[must_use]
    pub const fn new(skip_bot_commands: bool, max_media_timestamp: i32) -> Self {
        Self {
            skip_bot_commands,
            max_media_timestamp,
        }
    }

    /// Returns whether to skip bot commands.
    #[must_use]
    pub const fn skip_bot_commands(&self) -> bool {
        self.skip_bot_commands
    }

    /// Returns the maximum media timestamp.
    #[must_use]
    pub const fn max_media_timestamp(&self) -> i32 {
        self.max_media_timestamp
    }
}

/// Manager for translating message text.
///
/// This manager handles translation of formatted text between languages,
/// preserving formatting entities like bold, italic, links, etc.
///
/// # Example
///
/// ```rust
/// use rustgram-translation_manager::TranslationManager;
/// use rustgram-formatted-text::FormattedText;
///
/// let manager = TranslationManager::new();
/// let text = FormattedText::new("Hello, world!");
/// let result = manager.translate_text(text, "es", false, 0, None);
/// ```
#[derive(Debug, Default)]
pub struct TranslationManager {
    /// Number of translations performed
    translation_count: std::sync::atomic::AtomicU64,
}

impl TranslationManager {
    /// Creates a new translation manager.
    #[must_use]
    pub fn new() -> Self {
        info!("Creating new TranslationManager");
        Self {
            translation_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Translates formatted text to a target language.
    ///
    /// # Arguments
    ///
    /// * `text` - The formatted text to translate
    /// * `to_language_code` - Target language code (ISO 639-1)
    /// * `skip_bot_commands` - Whether to skip bot commands
    /// * `max_media_timestamp` - Maximum media timestamp
    /// * `message_full_id` - Optional message full ID for context
    ///
    /// # Returns
    ///
    /// The translated formatted text on success.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The text is empty
    /// - The language code is invalid
    /// - The translation service is unavailable
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram-translation_manager::TranslationManager;
    /// use rustgram-formatted-text::FormattedText;
    ///
    /// let manager = TranslationManager::new();
    /// let text = FormattedText::new("Hello, world!");
    /// let result = manager.translate_text(text, "es", false, 0, None);
    /// ```
    pub fn translate_text(
        &self,
        text: FormattedText,
        to_language_code: &str,
        skip_bot_commands: bool,
        max_media_timestamp: i32,
        message_full_id: Option<MessageFullId>,
    ) -> TranslationResult<FormattedText> {
        // Validate text
        if text.is_empty() {
            return Err(TranslationError::EmptyText);
        }

        // Validate language code
        let lang = LanguageCode::new(to_language_code)?;

        // Log the translation request
        debug!(
            "Translating text to language={}, skip_bot_commands={}, max_media_timestamp={}, message_id={:?}",
            lang, skip_bot_commands, max_media_timestamp, message_full_id
        );

        // In a real implementation, this would call the Telegram translation API
        // For now, we return a mock translation
        let translated_text = self.mock_translate(&text, &lang)?;

        // Increment counter
        self.translation_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Ok(translated_text)
    }

    /// Translates text with full options.
    ///
    /// # Arguments
    ///
    /// * `text` - The formatted text to translate
    /// * `to_language_code` - Target language code
    /// * `options` - Translation options
    /// * `message_full_id` - Optional message full ID
    pub fn translate_text_with_options(
        &self,
        text: FormattedText,
        to_language_code: &str,
        options: TranslationOptions,
        message_full_id: Option<MessageFullId>,
    ) -> TranslationResult<FormattedText> {
        self.translate_text(
            text,
            to_language_code,
            options.skip_bot_commands(),
            options.max_media_timestamp(),
            message_full_id,
        )
    }

    /// Mock translation for testing purposes.
    ///
    /// In a real implementation, this would call the Telegram translation API.
    /// This stub returns the text with a language prefix.
    fn mock_translate(
        &self,
        text: &FormattedText,
        lang: &LanguageCode,
    ) -> TranslationResult<FormattedText> {
        let prefix = format!("[{}] ", lang.as_str());
        let translated = format!("{}{}", prefix, text.text());

        // Preserve entities from the original text
        let mut result = FormattedText::new(&translated);
        for entity in text.entities() {
            // Adjust entity offsets to account for the prefix
            let adjusted_offset = entity.offset() + prefix.len() as i32;
            result.add_entity(rustgram_formatted_text::MessageEntity::with_argument(
                entity.entity_type(),
                adjusted_offset,
                entity.length(),
                entity.argument(),
            ));
        }

        Ok(result)
    }

    /// Returns the number of translations performed.
    #[must_use]
    pub fn translation_count(&self) -> u64 {
        self.translation_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Validates a language code.
    ///
    /// # Arguments
    ///
    /// * `code` - The language code to validate
    ///
    /// # Returns
    ///
    /// `Ok` if the code is valid, `Err` otherwise.
    #[must_use]
    pub fn validate_language_code(&self, code: &str) -> TranslationResult<()> {
        LanguageCode::new(code)?;
        Ok(())
    }

    /// Returns supported language codes.
    ///
    /// This is a subset of commonly supported languages.
    #[must_use]
    pub fn supported_languages() -> &'static [&'static str] {
        &[
            "en", // English
            "es", // Spanish
            "fr", // French
            "de", // German
            "it", // Italian
            "pt", // Portuguese
            "ru", // Russian
            "ar", // Arabic
            "hi", // Hindi
            "zh", // Chinese
            "ja", // Japanese
            "ko", // Korean
        ]
    }

    /// Checks if a language code is supported.
    ///
    /// # Arguments
    ///
    /// * `code` - The language code to check
    #[must_use]
    pub fn is_language_supported(code: &str) -> bool {
        let code_lower = code.to_lowercase();
        Self::supported_languages()
            .iter()
            .any(|&lang| lang == code_lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = TranslationManager::new();
        assert_eq!(manager.translation_count(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = TranslationManager::default();
        assert_eq!(manager.translation_count(), 0);
    }

    #[test]
    fn test_language_code_valid() {
        let lang = LanguageCode::new("en").unwrap();
        assert_eq!(lang.as_str(), "en");
        assert!(lang.is_valid());
    }

    #[test]
    fn test_language_code_lowercase() {
        let lang = LanguageCode::new("EN").unwrap();
        assert_eq!(lang.as_str(), "en");
    }

    #[test]
    fn test_language_code_trim() {
        let lang = LanguageCode::new("  es  ").unwrap();
        assert_eq!(lang.as_str(), "es");
    }

    #[test]
    fn test_language_code_empty() {
        let result = LanguageCode::new("");
        assert!(matches!(
            result,
            Err(TranslationError::InvalidLanguageCode(_))
        ));
    }

    #[test]
    fn test_language_code_too_short() {
        let result = LanguageCode::new("e");
        assert!(matches!(
            result,
            Err(TranslationError::InvalidLanguageCode(_))
        ));
    }

    #[test]
    fn test_language_code_too_long() {
        let result = LanguageCode::new("english");
        assert!(matches!(
            result,
            Err(TranslationError::InvalidLanguageCode(_))
        ));
    }

    #[test]
    fn test_language_code_invalid_chars() {
        let result = LanguageCode::new("e1");
        assert!(matches!(
            result,
            Err(TranslationError::InvalidLanguageCode(_))
        ));
    }

    #[test]
    fn test_language_code_three_letters() {
        let lang = LanguageCode::new("spa").unwrap();
        assert_eq!(lang.as_str(), "spa");
        assert!(lang.is_valid());
    }

    #[test]
    fn test_translation_options_default() {
        let options = TranslationOptions::default();
        assert!(options.skip_bot_commands());
        assert_eq!(options.max_media_timestamp(), 0);
    }

    #[test]
    fn test_translation_options_new() {
        let options = TranslationOptions::new(false, 100);
        assert!(!options.skip_bot_commands());
        assert_eq!(options.max_media_timestamp(), 100);
    }

    #[test]
    fn test_translate_text_success() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("Hello, world!");
        let result = manager.translate_text(text, "es", false, 0, None);

        assert!(result.is_ok());
        let translated = result.unwrap();
        assert!(translated.text().contains("[es]"));
        assert!(translated.text().contains("Hello, world!"));
    }

    #[test]
    fn test_translate_text_empty() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("");
        let result = manager.translate_text(text, "es", false, 0, None);

        assert!(matches!(result, Err(TranslationError::EmptyText)));
    }

    #[test]
    fn test_translate_text_invalid_language() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("Hello");
        let result = manager.translate_text(text, "invalid", false, 0, None);

        assert!(matches!(
            result,
            Err(TranslationError::InvalidLanguageCode(_))
        ));
    }

    #[test]
    fn test_translate_text_preserves_entities() {
        let manager = TranslationManager::new();
        let mut text = FormattedText::new("Hello world");
        text.add_entity(rustgram_formatted_text::MessageEntity::new("bold", 0, 5));

        let result = manager.translate_text(text, "fr", false, 0, None);
        assert!(result.is_ok());

        let translated = result.unwrap();
        assert_eq!(translated.entities().len(), 1);
        // Entity offset should be adjusted for the prefix
        let entity = &translated.entities()[0];
        assert!(entity.offset() > 0);
    }

    #[test]
    fn test_translate_text_with_options() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("Test");
        let options = TranslationOptions::new(true, 50);

        let result = manager.translate_text_with_options(text, "de", options, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_translation_count_increments() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("Test");

        assert_eq!(manager.translation_count(), 0);

        manager
            .translate_text(text.clone(), "es", false, 0, None)
            .unwrap();
        assert_eq!(manager.translation_count(), 1);

        manager
            .translate_text(text.clone(), "fr", false, 0, None)
            .unwrap();
        assert_eq!(manager.translation_count(), 2);
    }

    #[test]
    fn test_validate_language_code_valid() {
        let manager = TranslationManager::new();
        assert!(manager.validate_language_code("en").is_ok());
        assert!(manager.validate_language_code("ES").is_ok());
        assert!(manager.validate_language_code("de").is_ok());
    }

    #[test]
    fn test_validate_language_code_invalid() {
        let manager = TranslationManager::new();
        assert!(manager.validate_language_code("").is_err());
        assert!(manager.validate_language_code("e").is_err());
        assert!(manager.validate_language_code("english").is_err());
        assert!(manager.validate_language_code("123").is_err());
    }

    #[test]
    fn test_supported_languages() {
        let languages = TranslationManager::supported_languages();
        assert!(languages.len() > 10);
        assert!(languages.contains(&"en"));
        assert!(languages.contains(&"es"));
        assert!(languages.contains(&"fr"));
    }

    #[test]
    fn test_is_language_supported() {
        assert!(TranslationManager::is_language_supported("en"));
        assert!(TranslationManager::is_language_supported("ES"));
        assert!(TranslationManager::is_language_supported("de"));
        assert!(!TranslationManager::is_language_supported("xx"));
        assert!(!TranslationManager::is_language_supported(""));
    }

    #[test]
    fn test_language_code_display() {
        let lang = LanguageCode::new("en").unwrap();
        assert_eq!(format!("{}", lang), "en");
    }

    #[test]
    fn test_language_code_as_ref() {
        let lang = LanguageCode::new("fr").unwrap();
        let s: &str = lang.as_ref();
        assert_eq!(s, "fr");
    }

    #[test]
    fn test_translate_multiple_times() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("Hello");

        for lang in &["es", "fr", "de", "it"] {
            let result = manager.translate_text(text.clone(), lang, false, 0, None);
            assert!(result.is_ok());
        }

        assert_eq!(manager.translation_count(), 4);
    }

    #[test]
    fn test_translate_text_skip_bot_commands() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("/start Hello");

        let result = manager.translate_text(text.clone(), "es", true, 0, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_translate_text_with_max_media_timestamp() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("Caption");

        let result = manager.translate_text(text, "es", false, 100, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_translation_prefix() {
        let manager = TranslationManager::new();
        let text = FormattedText::new("Hello");
        let result = manager.translate_text(text, "es", false, 0, None).unwrap();

        assert!(result.text().starts_with("[es]"));
    }

    #[test]
    fn test_translation_error_messages() {
        assert_eq!(
            format!("{}", TranslationError::EmptyText),
            "Cannot translate empty text"
        );

        assert_eq!(
            format!("{}", TranslationError::RateLimitExceeded),
            "Translation rate limit exceeded"
        );

        assert_eq!(
            format!("{}", TranslationError::MessageIdRequired),
            "Message ID is required for this translation"
        );
    }

    #[test]
    fn test_common_language_codes() {
        let valid_codes = [
            "en", "es", "fr", "de", "it", "pt", "ru", "zh", "ja", "ko", "ar", "hi",
        ];

        for code in &valid_codes {
            let lang = LanguageCode::new(code);
            assert!(lang.is_ok(), "Language code {} should be valid", code);
        }
    }

    #[test]
    fn test_translation_options_equality() {
        let options1 = TranslationOptions::new(true, 50);
        let options2 = TranslationOptions::new(true, 50);
        let options3 = TranslationOptions::new(false, 50);

        assert_eq!(options1, options2);
        assert_ne!(options1, options3);
    }

    #[test]
    fn test_language_code_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let lang1 = LanguageCode::new("en").unwrap();
        let lang2 = LanguageCode::new("en").unwrap();
        let lang3 = LanguageCode::new("es").unwrap();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let mut hasher3 = DefaultHasher::new();

        lang1.hash(&mut hasher1);
        lang2.hash(&mut hasher2);
        lang3.hash(&mut hasher3);

        assert_eq!(hasher1.finish(), hasher2.finish());
        assert_ne!(hasher1.finish(), hasher3.finish());
    }
}
