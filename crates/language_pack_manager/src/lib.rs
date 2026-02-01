// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Language Pack Manager
//!
//! Manages language packs and translations for Telegram clients.
//!
//! Based on TDLib's `LanguagePackManager` from `td/telegram/LanguagePackManager.h`.
//!
//! ## Overview
//!
//! The language pack manager handles:
//! - Loading and caching language pack strings
//! - Managing custom language packs
//! - Synchronizing language packs with server
//! - Validation of language codes and pack names
//! - Pluralization support for translations
//!
//! ## Architecture
//!
//! The manager maintains:
//! - Current language pack and code
//! - Base language code for fallback translations
//! - Database of language strings
//! - Custom language packs
//! - Server language pack information
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_language_pack_manager::LanguagePackManager;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create manager
//! let manager = LanguagePackManager::new("en", "USD");
//!
//! // Check if language code is valid
//! assert!(LanguagePackManager::check_language_code_name("en"));
//! assert!(!LanguagePackManager::check_language_code_name("invalid!"));
//!
//! // Get strings from language pack
//! // let strings = manager.get_language_pack_strings("en", vec!["key".to_string()]).await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod error;

use crate::error::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum length for a language pack string value.
#[allow(dead_code)]
const MAX_STRING_VALUE_LENGTH: usize = 50000;

/// Regular expression for validating language pack names.
static LANGUAGE_PACK_RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    #[allow(clippy::expect_used)]
    Regex::new(r"^[a-zA-Z0-9_-]+$").expect("invalid regex")
});

/// Regular expression for validating language codes.
static LANGUAGE_CODE_RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    #[allow(clippy::expect_used)]
    Regex::new(r"^[a-zA-Z]{2,3}$").expect("invalid regex")
});

/// Language pack string value type.
///
/// TDLib reference: `td_api::LanguagePackStringValue`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum LanguagePackStringValue {
    /// Plain string value.
    ///
    /// TDLib reference: `languagePackStringValueOrdinary`
    Ordinary {
        /// The string value.
        value: String,
    },
    /// Pluralized string value.
    ///
    /// TDLib reference: `languagePackStringValuePluralized`
    Pluralized {
        /// Value for one item.
        one: String,
        /// Value for two items.
        two: Option<String>,
        /// Value for few items.
        few: Option<String>,
        /// Value for many items.
        many: Option<String>,
        /// Value for other items.
        other: String,
    },
    /// Deleted string value.
    ///
    /// TDLib reference: `languagePackStringValueDeleted`
    Deleted,
}

impl LanguagePackStringValue {
    /// Creates a new ordinary string value.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value
    #[must_use]
    pub fn ordinary(value: String) -> Self {
        Self::Ordinary { value }
    }

    /// Creates a new pluralized string value.
    ///
    /// # Arguments
    ///
    /// * `other` - The fallback value
    #[must_use]
    pub fn pluralized(other: String) -> Self {
        Self::Pluralized {
            one: String::new(),
            two: None,
            few: None,
            many: None,
            other,
        }
    }

    /// Returns the ordinary value if this is an ordinary string.
    #[must_use]
    pub fn as_ordinary(&self) -> Option<&str> {
        match self {
            Self::Ordinary { value } => Some(value),
            _ => None,
        }
    }

    /// Returns true if this is a deleted value.
    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted)
    }
}

/// Language pack string.
///
/// TDLib reference: `td_api::languagePackString`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguagePackString {
    /// The key of the string.
    pub key: String,
    /// The string value.
    pub value: LanguagePackStringValue,
}

impl LanguagePackString {
    /// Creates a new language pack string.
    ///
    /// # Arguments
    ///
    /// * `key` - The string key
    /// * `value` - The string value
    #[must_use]
    pub fn new(key: String, value: LanguagePackStringValue) -> Self {
        Self { key, value }
    }

    /// Creates an ordinary string.
    #[must_use]
    pub fn ordinary(key: String, value: String) -> Self {
        Self {
            key,
            value: LanguagePackStringValue::ordinary(value),
        }
    }

    /// Creates a deleted string.
    #[must_use]
    pub fn deleted(key: String) -> Self {
        Self {
            key,
            value: LanguagePackStringValue::Deleted,
        }
    }
}

/// Multiple language pack strings.
///
/// TDLib reference: `td_api::languagePackStrings`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguagePackStrings {
    /// The list of strings.
    pub strings: Vec<LanguagePackString>,
}

impl LanguagePackStrings {
    /// Creates a new language pack strings object.
    ///
    /// # Arguments
    ///
    /// * `strings` - The list of strings
    #[must_use]
    pub fn new(strings: Vec<LanguagePackString>) -> Self {
        Self { strings }
    }

    /// Creates empty language pack strings.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            strings: Vec::new(),
        }
    }

    /// Returns true if there are no strings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Returns the number of strings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.strings.len()
    }
}

/// Language pack information.
///
/// TDLib reference: `td_api::languagePackInfo`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguagePackInfo {
    /// Unique language pack identifier.
    pub id: String,
    /// Language code.
    pub language_code: String,
    /// Language name.
    pub name: String,
    /// Language native name.
    pub native_name: String,
    /// Base language code for pluralization.
    pub base_language_code: Option<String>,
    /// Language pack plural forms.
    pub plural_code: String,
}

impl LanguagePackInfo {
    /// Creates a new language pack info.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `language_code` - Language code
    /// * `name` - Language name
    /// * `native_name` - Native name
    #[must_use]
    pub fn new(id: String, language_code: String, name: String, native_name: String) -> Self {
        Self {
            id,
            language_code,
            name,
            native_name,
            base_language_code: None,
            plural_code: "other".to_string(),
        }
    }

    /// Sets the base language code.
    #[must_use]
    pub fn with_base_language_code(mut self, base_code: String) -> Self {
        self.base_language_code = Some(base_code);
        self
    }
}

/// Information about available localization targets.
///
/// TDLib reference: `td_api::localizationTargetInfo`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalizationTargetInfo {
    /// List of available language packs.
    pub language_packs: Vec<LanguagePackInfo>,
}

impl LocalizationTargetInfo {
    /// Creates a new localization target info.
    ///
    /// # Arguments
    ///
    /// * `language_packs` - List of language packs
    #[must_use]
    pub fn new(language_packs: Vec<LanguagePackInfo>) -> Self {
        Self { language_packs }
    }

    /// Creates empty localization target info.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            language_packs: Vec::new(),
        }
    }
}

/// Manages language packs and translations.
///
/// This manager handles loading language pack strings, managing
/// custom language packs, and synchronizing with the server.
///
/// TDLib reference: `td::LanguagePackManager` from `LanguagePackManager.h`
#[derive(Debug)]
pub struct LanguagePackManager {
    /// Current language pack.
    #[allow(dead_code)]
    language_pack: Arc<RwLock<String>>,
    /// Current language code.
    language_code: Arc<RwLock<String>>,
    /// Base language code for fallback.
    base_language_code: Arc<RwLock<String>>,
    /// Cached language strings.
    cached_strings: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    /// Custom language packs.
    custom_packs: Arc<RwLock<HashMap<String, LanguagePackInfo>>>,
}

impl LanguagePackManager {
    /// Creates a new language pack manager.
    ///
    /// # Arguments
    ///
    /// * `language_code` - Initial language code (e.g., "en")
    /// * `base_language_code` - Base language code for fallback (e.g., "en")
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_language_pack_manager::LanguagePackManager;
    ///
    /// let manager = LanguagePackManager::new("en", "en");
    /// ```
    pub fn new(language_code: &str, base_language_code: &str) -> Self {
        Self {
            language_pack: Arc::new(RwLock::new("desktop".to_string())),
            language_code: Arc::new(RwLock::new(language_code.to_string())),
            base_language_code: Arc::new(RwLock::new(base_language_code.to_string())),
            cached_strings: Arc::new(RwLock::new(HashMap::new())),
            custom_packs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Checks if a language pack name is valid.
    ///
    /// # Arguments
    ///
    /// * `name` - The language pack name to validate
    ///
    /// Returns true if the name is valid.
    ///
    /// TDLib reference: `LanguagePackManager.h:40`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_language_pack_manager::LanguagePackManager;
    ///
    /// assert!(LanguagePackManager::check_language_pack_name("desktop"));
    /// assert!(LanguagePackManager::check_language_pack_name("android"));
    /// assert!(!LanguagePackManager::check_language_pack_name("invalid name!"));
    /// ```
    pub fn check_language_pack_name(name: &str) -> bool {
        if name.is_empty() || name.len() > 40 {
            return false;
        }
        LANGUAGE_PACK_RE.is_match(name)
    }

    /// Checks if a language code name is valid.
    ///
    /// # Arguments
    ///
    /// * `name` - The language code to validate
    ///
    /// Returns true if the code is valid.
    ///
    /// TDLib reference: `LanguagePackManager.h:42`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_language_pack_manager::LanguagePackManager;
    ///
    /// assert!(LanguagePackManager::check_language_code_name("en"));
    /// assert!(LanguagePackManager::check_language_code_name("es"));
    /// assert!(!LanguagePackManager::check_language_code_name("invalid"));
    /// assert!(!LanguagePackManager::check_language_code_name("e!"));
    /// ```
    pub fn check_language_code_name(name: &str) -> bool {
        if name.is_empty() || name.len() > 10 {
            return false;
        }
        LANGUAGE_CODE_RE.is_match(name)
    }

    /// Checks if a language code is a custom language code.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The language code to check
    ///
    /// Returns true if this is a custom language code.
    ///
    /// TDLib reference: `LanguagePackManager.h:44`
    pub fn is_custom_language_code(language_code: &str) -> bool {
        // Custom language codes contain an underscore and are reasonable length
        language_code.contains('_') && language_code.len() <= 10
    }

    /// Gets the main language code.
    ///
    /// Returns the current language code.
    ///
    /// TDLib reference: `LanguagePackManager.h:46`
    pub async fn get_main_language_code(&self) -> String {
        self.language_code.read().await.clone()
    }

    /// Gets the list of used language codes.
    ///
    /// Returns all language codes that have been used.
    ///
    /// TDLib reference: `LanguagePackManager.h:48`
    pub async fn get_used_language_codes(&self) -> Vec<String> {
        let codes = self.language_code.read().await.clone();
        let base_codes = self.base_language_code.read().await.clone();
        let mut result = vec![codes];
        if !base_codes.is_empty() && base_codes != result[0] {
            result.push(base_codes);
        }
        result
    }

    /// Gets language pack strings.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The language code
    /// * `keys` - The string keys to retrieve
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The language code is invalid
    /// - The strings cannot be loaded
    ///
    /// TDLib reference: `LanguagePackManager.h:62-63`
    pub async fn get_language_pack_strings(
        &self,
        language_code: &str,
        keys: Vec<String>,
    ) -> Result<LanguagePackStrings> {
        if !Self::check_language_code_name(language_code) {
            return Err(Error::InvalidLanguageCode(language_code.to_string()));
        }

        let mut strings = Vec::new();
        let cache = self.cached_strings.read().await;

        for key in &keys {
            if let Some(lang_strings) = cache.get(language_code) {
                if let Some(value) = lang_strings.get(key) {
                    strings.push(LanguagePackString::ordinary(key.clone(), value.clone()));
                }
            }
        }

        Ok(LanguagePackStrings::new(strings))
    }

    /// Synchronizes a language pack.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The language code to synchronize
    ///
    /// # Errors
    ///
    /// Returns an error if the language code is invalid.
    ///
    /// TDLib reference: `LanguagePackManager.h:69`
    pub async fn synchronize_language_pack(&self, language_code: &str) -> Result<()> {
        if !Self::check_language_code_name(language_code) {
            return Err(Error::InvalidLanguageCode(language_code.to_string()));
        }

        // In a real implementation, this would sync with the server
        Ok(())
    }

    /// Adds a custom server language.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The custom language code
    ///
    /// # Errors
    ///
    /// Returns an error if the language code is not a valid custom language code.
    ///
    /// TDLib reference: `LanguagePackManager.h:73`
    pub async fn add_custom_server_language(&self, language_code: &str) -> Result<()> {
        // Custom language codes must contain underscore and be reasonable length
        if !language_code.contains('_') || language_code.len() > 10 {
            return Err(Error::InvalidCustomLanguageCode(language_code.to_string()));
        }

        // In a real implementation, this would add the custom language
        Ok(())
    }

    /// Sets a custom language.
    ///
    /// # Arguments
    ///
    /// * `language_pack_info` - The language pack information
    /// * `strings` - The language strings
    ///
    /// # Errors
    ///
    /// Returns an error if the language code is invalid.
    ///
    /// TDLib reference: `LanguagePackManager.h:75-76`
    pub async fn set_custom_language(
        &self,
        language_pack_info: LanguagePackInfo,
        strings: Vec<LanguagePackString>,
    ) -> Result<()> {
        let language_code = language_pack_info.language_code.clone();
        // Custom language codes are allowed
        if language_code.len() > 10 {
            return Err(Error::InvalidLanguageCode(language_code.to_string()));
        }

        // Store custom pack
        let mut custom = self.custom_packs.write().await;
        custom.insert(language_code.clone(), language_pack_info);
        drop(custom);

        // Cache strings
        let mut cache = self.cached_strings.write().await;
        let lang_strings = cache.entry(language_code.clone()).or_default();

        for string in strings {
            if let LanguagePackStringValue::Ordinary { value } = string.value {
                lang_strings.insert(string.key, value);
            }
        }

        Ok(())
    }

    /// Edits custom language information.
    ///
    /// # Arguments
    ///
    /// * `language_pack_info` - The updated language pack information
    ///
    /// # Errors
    ///
    /// Returns an error if the language pack is not found.
    ///
    /// TDLib reference: `LanguagePackManager.h:78-79`
    pub async fn edit_custom_language_info(
        &self,
        language_pack_info: LanguagePackInfo,
    ) -> Result<()> {
        let language_code = &language_pack_info.language_code;
        let mut custom = self.custom_packs.write().await;

        if !custom.contains_key(language_code) {
            return Err(Error::LanguagePackNotFound(language_code.to_string()));
        }

        custom.insert(language_code.clone(), language_pack_info);
        Ok(())
    }

    /// Sets a custom language string.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The language code
    /// * `str` - The language string to set
    ///
    /// # Errors
    ///
    /// Returns an error if the language code is invalid.
    ///
    /// TDLib reference: `LanguagePackManager.h:81-82`
    pub async fn set_custom_language_string(
        &self,
        language_code: &str,
        str: LanguagePackString,
    ) -> Result<()> {
        if !Self::check_language_code_name(language_code) {
            return Err(Error::InvalidLanguageCode(language_code.to_string()));
        }

        let key = str.key.clone();
        if let LanguagePackStringValue::Ordinary { value } = str.value {
            let mut cache = self.cached_strings.write().await;
            let lang_strings = cache.entry(language_code.to_string()).or_default();
            lang_strings.insert(key, value);
        }

        Ok(())
    }

    /// Deletes a language.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The language code to delete
    ///
    /// # Errors
    ///
    /// Returns an error if the language code is invalid.
    ///
    /// TDLib reference: `LanguagePackManager.h:84`
    pub async fn delete_language(&self, language_code: &str) -> Result<()> {
        if !Self::check_language_code_name(language_code) {
            return Err(Error::InvalidLanguageCode(language_code.to_string()));
        }

        let mut cache = self.cached_strings.write().await;
        cache.remove(language_code);

        let mut custom = self.custom_packs.write().await;
        custom.remove(language_code);

        Ok(())
    }

    /// Called when the language pack changes.
    ///
    /// TDLib reference: `LanguagePackManager.h:50`
    pub async fn on_language_pack_changed(&self) {
        // In a real implementation, this would trigger a reload
    }

    /// Called when the language code changes.
    ///
    /// TDLib reference: `LanguagePackManager.h:52`
    pub async fn on_language_code_changed(&self) {
        // In a real implementation, this would trigger a reload
    }

    /// Called when the language pack version changes.
    ///
    /// # Arguments
    ///
    /// * `is_base` - Whether this is the base language pack
    /// * `new_version` - The new version number
    ///
    /// TDLib reference: `LanguagePackManager.h:54`
    pub async fn on_language_pack_version_changed(&self, is_base: bool, new_version: i32) {
        // In a real implementation, this would trigger a sync
        let _ = (is_base, new_version);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_manager() -> LanguagePackManager {
        LanguagePackManager::new("en", "en")
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = create_test_manager();
        assert_eq!(manager.get_main_language_code().await.as_str(), "en");
    }

    #[test]
    fn test_check_language_pack_name_valid() {
        assert!(LanguagePackManager::check_language_pack_name("desktop"));
        assert!(LanguagePackManager::check_language_pack_name("android"));
        assert!(LanguagePackManager::check_language_pack_name("ios"));
        assert!(LanguagePackManager::check_language_pack_name("tdesktop"));
        assert!(LanguagePackManager::check_language_pack_name("web"));
        assert!(LanguagePackManager::check_language_pack_name("macos"));
        assert!(LanguagePackManager::check_language_pack_name(
            "ubuntu-touch"
        ));
    }

    #[test]
    fn test_check_language_pack_name_invalid() {
        assert!(!LanguagePackManager::check_language_pack_name(""));
        assert!(!LanguagePackManager::check_language_pack_name(
            "invalid name"
        ));
        assert!(!LanguagePackManager::check_language_pack_name("name!"));
        assert!(!LanguagePackManager::check_language_pack_name("name@"));
        assert!(!LanguagePackManager::check_language_pack_name("name#"));
    }

    #[test]
    fn test_check_language_pack_name_too_long() {
        let long_name = "a".repeat(41);
        assert!(!LanguagePackManager::check_language_pack_name(&long_name));
    }

    #[test]
    fn test_check_language_code_name_valid() {
        assert!(LanguagePackManager::check_language_code_name("en"));
        assert!(LanguagePackManager::check_language_code_name("es"));
        assert!(LanguagePackManager::check_language_code_name("ru"));
        assert!(LanguagePackManager::check_language_code_name("zh"));
        assert!(LanguagePackManager::check_language_code_name("de"));
        assert!(LanguagePackManager::check_language_code_name("fr"));
    }

    #[test]
    fn test_check_language_code_name_invalid() {
        assert!(!LanguagePackManager::check_language_code_name(""));
        assert!(!LanguagePackManager::check_language_code_name("e"));
        assert!(!LanguagePackManager::check_language_code_name("en-US"));
        assert!(!LanguagePackManager::check_language_code_name("en_us"));
        assert!(!LanguagePackManager::check_language_code_name("123"));
        assert!(!LanguagePackManager::check_language_code_name("e!"));
    }

    #[test]
    fn test_check_language_code_name_three_letters() {
        // Three letter codes are valid
        assert!(LanguagePackManager::check_language_code_name("eng"));
        assert!(LanguagePackManager::check_language_code_name("zho"));
    }

    #[test]
    fn test_is_custom_language_code() {
        assert!(LanguagePackManager::is_custom_language_code("en_US"));
        assert!(LanguagePackManager::is_custom_language_code("es_MX"));
        assert!(LanguagePackManager::is_custom_language_code("zh_Hans"));
        assert!(!LanguagePackManager::is_custom_language_code("en"));
        assert!(!LanguagePackManager::is_custom_language_code("es"));
        assert!(!LanguagePackManager::is_custom_language_code(""));
        assert!(!LanguagePackManager::is_custom_language_code("en-US")); // hyphen, not underscore
    }

    #[test]
    fn test_language_pack_string_value_ordinary() {
        let value = LanguagePackStringValue::ordinary("test".to_string());
        assert_eq!(value.as_ordinary(), Some("test"));
        assert!(!value.is_deleted());
    }

    #[test]
    fn test_language_pack_string_value_pluralized() {
        let value = LanguagePackStringValue::pluralized("items".to_string());
        assert!(value.as_ordinary().is_none());
        assert!(!value.is_deleted());
    }

    #[test]
    fn test_language_pack_string_value_deleted() {
        let value = LanguagePackStringValue::Deleted;
        assert!(value.as_ordinary().is_none());
        assert!(value.is_deleted());
    }

    #[test]
    fn test_language_pack_string_new() {
        let value = LanguagePackStringValue::ordinary("test".to_string());
        let string = LanguagePackString::new("key".to_string(), value);
        assert_eq!(string.key, "key");
    }

    #[test]
    fn test_language_pack_string_ordinary() {
        let string = LanguagePackString::ordinary("key".to_string(), "value".to_string());
        assert_eq!(string.key, "key");
        assert_eq!(string.value.as_ordinary(), Some("value"));
    }

    #[test]
    fn test_language_pack_string_deleted() {
        let string = LanguagePackString::deleted("key".to_string());
        assert_eq!(string.key, "key");
        assert!(string.value.is_deleted());
    }

    #[test]
    fn test_language_pack_strings_new() {
        let strings = LanguagePackStrings::new(vec![]);
        assert!(strings.is_empty());
        assert_eq!(strings.len(), 0);
    }

    #[test]
    fn test_language_pack_strings_empty() {
        let strings = LanguagePackStrings::empty();
        assert!(strings.is_empty());
        assert_eq!(strings.len(), 0);
    }

    #[test]
    fn test_language_pack_info_new() {
        let info = LanguagePackInfo::new(
            "desktop".to_string(),
            "en".to_string(),
            "English".to_string(),
            "English".to_string(),
        );
        assert_eq!(info.id, "desktop");
        assert_eq!(info.language_code, "en");
        assert_eq!(info.name, "English");
        assert_eq!(info.native_name, "English");
        assert!(info.base_language_code.is_none());
    }

    #[test]
    fn test_language_pack_info_with_base() {
        let info = LanguagePackInfo::new(
            "desktop".to_string(),
            "en".to_string(),
            "English".to_string(),
            "English".to_string(),
        )
        .with_base_language_code("en".to_string());
        assert_eq!(info.base_language_code.as_deref(), Some("en"));
    }

    #[test]
    fn test_localization_target_info_new() {
        let info = LocalizationTargetInfo::new(vec![]);
        assert!(info.language_packs.is_empty());
    }

    #[test]
    fn test_localization_target_info_empty() {
        let info = LocalizationTargetInfo::empty();
        assert!(info.language_packs.is_empty());
    }

    #[tokio::test]
    async fn test_get_main_language_code() {
        let manager = create_test_manager();
        assert_eq!(manager.get_main_language_code().await.as_str(), "en");
    }

    #[tokio::test]
    async fn test_get_used_language_codes() {
        let manager = create_test_manager();
        let codes = manager.get_used_language_codes().await;
        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0], "en");
    }

    #[tokio::test]
    async fn test_get_language_pack_strings_invalid_code() {
        let manager = create_test_manager();
        let result = manager.get_language_pack_strings("invalid!", vec![]).await;
        assert!(matches!(result, Err(Error::InvalidLanguageCode(_))));
    }

    #[tokio::test]
    async fn test_get_language_pack_strings_success() {
        let manager = create_test_manager();
        let result = manager.get_language_pack_strings("en", vec![]).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_synchronize_language_pack_invalid_code() {
        let manager = create_test_manager();
        let result = manager.synchronize_language_pack("invalid!").await;
        assert!(matches!(result, Err(Error::InvalidLanguageCode(_))));
    }

    #[tokio::test]
    async fn test_synchronize_language_pack_success() {
        let manager = create_test_manager();
        let result = manager.synchronize_language_pack("en").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_custom_server_language_invalid() {
        let manager = create_test_manager();
        let result = manager.add_custom_server_language("en").await;
        assert!(matches!(result, Err(Error::InvalidCustomLanguageCode(_))));
    }

    #[tokio::test]
    async fn test_add_custom_server_language_success() {
        let manager = create_test_manager();
        let result = manager.add_custom_server_language("en_US").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_custom_language_invalid_code() {
        let manager = create_test_manager();
        let info = LanguagePackInfo::new(
            "desktop".to_string(),
            "a_very_long_language_code".to_string(),
            "Test".to_string(),
            "Test".to_string(),
        );
        let result = manager.set_custom_language(info, vec![]).await;
        assert!(matches!(result, Err(Error::InvalidLanguageCode(_))));
    }

    #[tokio::test]
    async fn test_set_custom_language_success() {
        let manager = create_test_manager();
        let info = LanguagePackInfo::new(
            "desktop".to_string(),
            "en_US".to_string(),
            "English (US)".to_string(),
            "English (US)".to_string(),
        );
        let strings = vec![LanguagePackString::ordinary(
            "key".to_string(),
            "value".to_string(),
        )];
        let result = manager.set_custom_language(info, strings).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_custom_language_info_not_found() {
        let manager = create_test_manager();
        let info = LanguagePackInfo::new(
            "desktop".to_string(),
            "en_US".to_string(),
            "English (US)".to_string(),
            "English (US)".to_string(),
        );
        let result = manager.edit_custom_language_info(info).await;
        assert!(matches!(result, Err(Error::LanguagePackNotFound(_))));
    }

    #[tokio::test]
    async fn test_set_custom_language_string_invalid_code() {
        let manager = create_test_manager();
        let string = LanguagePackString::ordinary("key".to_string(), "value".to_string());
        let result = manager.set_custom_language_string("invalid!", string).await;
        assert!(matches!(result, Err(Error::InvalidLanguageCode(_))));
    }

    #[tokio::test]
    async fn test_delete_language_invalid_code() {
        let manager = create_test_manager();
        let result = manager.delete_language("invalid!").await;
        assert!(matches!(result, Err(Error::InvalidLanguageCode(_))));
    }

    #[tokio::test]
    async fn test_delete_language_success() {
        let manager = create_test_manager();
        let result = manager.delete_language("en").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_on_language_pack_changed() {
        let manager = create_test_manager();
        manager.on_language_pack_changed().await;
        // Should not panic
    }

    #[tokio::test]
    async fn test_on_language_code_changed() {
        let manager = create_test_manager();
        manager.on_language_code_changed().await;
        // Should not panic
    }

    #[tokio::test]
    async fn test_on_language_pack_version_changed() {
        let manager = create_test_manager();
        manager.on_language_pack_version_changed(false, 1).await;
        // Should not panic
    }

    #[test]
    fn test_max_string_value_length_const() {
        assert_eq!(MAX_STRING_VALUE_LENGTH, 50000);
    }
}
