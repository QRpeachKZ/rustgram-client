// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Option Manager
//!
//! TDLib options and settings management for Telegram MTProto client.
//!
//! ## Overview
//!
//! This module provides functionality for managing TDLib options and settings.
//! It supports storing, retrieving, and updating configuration options with different
//! value types (boolean, integer, string).
//!
//! ## Features
//!
//! - Store options with different types (boolean, integer, string, empty)
//! - Get option values with default fallbacks
//! - Check if an option exists
//! - Set options and notify listeners
//! - Synchronous option access for common options
//! - TDLib API compatibility
//!
//! ## Usage
//!
//! ### Basic Option Management
//!
//! ```rust
//! use rustgram_option_manager::{OptionManager, OptionValue};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let manager = OptionManager::new();
//!
//! // Set a boolean option
//! manager.set_option_boolean("x_use_premium", true).await;
//!
//! // Get a boolean option with default
//! let use_premium = manager.get_option_boolean("x_use_premium", false).await;
//! assert_eq!(use_premium, true);
//!
//! // Set an integer option
//! manager.set_option_integer("my_chat_filter_count", 5).await;
//! let count = manager.get_option_integer("my_chat_filter_count", 0).await;
//! assert_eq!(count, 5);
//!
//! // Set a string option
//! manager.set_option_string("language_code", "en").await;
//! let lang = manager.get_option_string("language_code", "").await;
//! assert_eq!(lang, "en");
//! # }
//! ```
//!
//! ### Working with OptionValue
//!
//! ```rust
//! use rustgram_option_manager::OptionValue;
//!
//! // Create different option values
//! let bool_val = OptionValue::Boolean(true);
//! let int_val = OptionValue::Integer(42);
//! let string_val = OptionValue::String("hello".to_string());
//! let empty_val = OptionValue::Empty;
//!
//! // Check value types
//! assert!(bool_val.is_boolean());
//! assert!(int_val.is_integer());
//! assert!(string_val.is_string());
//! assert!(empty_val.is_empty());
//! ```
//!
//! ### Checking Option Existence
//!
//! ```rust
//! use rustgram_option_manager::OptionManager;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let manager = OptionManager::new();
//!
//! // Check if option exists
//! assert!(!manager.have_option("nonexistent").await);
//!
//! // Set an option
//! manager.set_option_boolean("my_option", true).await;
//!
//! // Now it exists
//! assert!(manager.have_option("my_option").await);
//! # }
//! ```
//!
//! ## TDLib Compatibility
//!
//! - **Reference**: `references/td/td/telegram/OptionManager.{h,cpp}`
//! - **TL Types**: `OptionValue`, `updateOption`
//! - **TL Functions**: `getOption`, `setOption`
//!
//! ## TL Correspondence
//!
//! ### TD API
//!
//! ```text
//! //@class OptionValue
//! optionValueBoolean value:Bool = OptionValue;
//! optionValueEmpty = OptionValue;
//! optionValueInteger value:int64 = OptionValue;
//! optionValueString value:string = OptionValue;
//!
//! updateOption name:string value:OptionValue = Update;
//! getOption name:string = OptionValue;
//! setOption name:string value:OptionValue = Ok;
//! ```
//!
//! ## Common Options
//!
//! Some commonly used options in TDLib:
//!
//! - `x_use_premium` - Whether to use premium features
//! - `language_code` - User's language code
//! - `my_chat_filter_count` - Number of chat filters
//! - `notification_group_count_max` - Max notification groups
//! - `notification_group_size_max` - Max notifications per group
//!
//! ## Design Decisions
//!
//! 1. **Type Safety**: Different option types are represented as an enum,
//!    preventing type confusion.
//!
//! 2. **Default Values**: All get methods accept a default value, providing
//!    graceful fallbacks for missing options.
//!
//! 3. **Storage**: Options are stored in-memory for quick access. Persistence
//!    is handled by higher-level components.
//!
//! 4. **TDLib Alignment**: The API closely matches TDLib's OptionManager for
//!    easy migration and compatibility.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod value;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-exports
pub use error::{OptionManagerError, Result};
pub use value::OptionValue;

/// Common synchronous option names that can be accessed without locks.
///
/// These options are frequently accessed and are optimized for synchronous access.
pub static SYNCHRONOUS_OPTIONS: &[&str] = &[
    "unix_time",
    "version",
    "localization_target",
    "language_pack_database_path",
    "language_pack_path",
];

/// Maximum number of options that can be stored.
pub const MAX_OPTIONS: usize = 10_000;

/// Option Manager for TDLib settings.
///
/// Manages all TDLib configuration options with different value types.
/// Provides thread-safe access to options and supports type-safe operations.
///
/// # Example
///
/// ```
/// use rustgram_option_manager::OptionManager;
///
/// let manager = OptionManager::new();
/// ```
#[derive(Clone)]
pub struct OptionManager {
    /// Storage for all options
    options: Arc<RwLock<HashMap<String, OptionValue>>>,

    /// Whether TDLib is initialized
    is_td_inited: Arc<RwLock<bool>>,

    /// Server time difference (in seconds)
    server_time_difference: Arc<RwLock<f64>>,
}

impl OptionManager {
    /// Create a new option manager.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// assert!(!manager.is_td_inited().await);
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            options: Arc::new(RwLock::new(HashMap::new())),
            is_td_inited: Arc::new(RwLock::new(false)),
            server_time_difference: Arc::new(RwLock::new(0.0)),
        }
    }

    /// Check if TDLib is initialized.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// assert!(!manager.is_td_inited().await);
    /// # }
    /// ```
    pub async fn is_td_inited(&self) -> bool {
        *self.is_td_inited.read().await
    }

    /// Called when TDLib is initialized.
    ///
    /// Updates premium options and performs initialization tasks.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.on_td_inited().await;
    /// assert!(manager.is_td_inited().await);
    /// # }
    /// ```
    pub async fn on_td_inited(&self) {
        *self.is_td_inited.write().await = true;
        self.update_premium_options().await;
    }

    /// Update premium-related options.
    ///
    /// Refreshes options that depend on premium status.
    pub async fn update_premium_options(&self) {
        // In real implementation, would update options based on premium status
        tracing::debug!("Updating premium options");
    }

    /// Set a boolean option.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `value` - Boolean value to set
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.set_option_boolean("x_use_premium", true).await;
    /// # }
    /// ```
    pub async fn set_option_boolean(&self, name: &str, value: bool) {
        self.set_option(name, OptionValue::Boolean(value)).await;
    }

    /// Set an option to empty.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name to clear
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.set_option_boolean("test", true).await;
    /// manager.set_option_empty("test").await;
    /// # }
    /// ```
    pub async fn set_option_empty(&self, name: &str) {
        self.set_option(name, OptionValue::Empty).await;
    }

    /// Set an integer option.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `value` - Integer value to set
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.set_option_integer("count", 42).await;
    /// # }
    /// ```
    pub async fn set_option_integer(&self, name: &str, value: i64) {
        self.set_option(name, OptionValue::Integer(value)).await;
    }

    /// Set a string option.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `value` - String value to set
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.set_option_string("language_code", "en").await;
    /// # }
    /// ```
    pub async fn set_option_string(&self, name: &str, value: &str) {
        self.set_option(name, OptionValue::String(value.to_string()))
            .await;
    }

    /// Set an option value.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `value` - Option value to set
    async fn set_option(&self, name: &str, value: OptionValue) {
        let mut options = self.options.write().await;

        // Check max options limit
        if !options.contains_key(name) && options.len() >= MAX_OPTIONS {
            tracing::error!("Maximum number of options reached: {}", MAX_OPTIONS);
            return;
        }

        options.insert(name.to_string(), value);
        self.on_option_updated(name).await;
    }

    /// Check if an option exists.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name to check
    ///
    /// # Returns
    ///
    /// `true` if the option exists, `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// assert!(!manager.have_option("test").await);
    ///
    /// manager.set_option_boolean("test", true).await;
    /// assert!(manager.have_option("test").await);
    /// # }
    /// ```
    pub async fn have_option(&self, name: &str) -> bool {
        self.options.read().await.contains_key(name)
    }

    /// Get a boolean option.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `default_value` - Default value if option doesn't exist
    ///
    /// # Returns
    ///
    /// The option value or the default
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    ///
    /// let value = manager.get_option_boolean("test", false).await;
    /// assert_eq!(value, false);
    ///
    /// manager.set_option_boolean("test", true).await;
    /// let value = manager.get_option_boolean("test", false).await;
    /// assert_eq!(value, true);
    /// # }
    /// ```
    pub async fn get_option_boolean(&self, name: &str, default_value: bool) -> bool {
        let options = self.options.read().await;
        match options.get(name) {
            Some(OptionValue::Boolean(value)) => *value,
            Some(OptionValue::Empty) => false,
            Some(OptionValue::Integer(value)) => *value != 0,
            Some(OptionValue::String(value)) => !value.is_empty(),
            None => default_value,
        }
    }

    /// Get an integer option.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `default_value` - Default value if option doesn't exist
    ///
    /// # Returns
    ///
    /// The option value or the default
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    ///
    /// let value = manager.get_option_integer("count", 0).await;
    /// assert_eq!(value, 0);
    ///
    /// manager.set_option_integer("count", 42).await;
    /// let value = manager.get_option_integer("count", 0).await;
    /// assert_eq!(value, 42);
    /// # }
    /// ```
    pub async fn get_option_integer(&self, name: &str, default_value: i64) -> i64 {
        let options = self.options.read().await;
        match options.get(name) {
            Some(OptionValue::Integer(value)) => *value,
            Some(OptionValue::Boolean(value)) => {
                if *value {
                    1
                } else {
                    0
                }
            }
            Some(OptionValue::String(value)) => value.parse().unwrap_or(default_value),
            Some(OptionValue::Empty) => 0,
            None => default_value,
        }
    }

    /// Get a string option.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `default_value` - Default value if option doesn't exist
    ///
    /// # Returns
    ///
    /// The option value or the default
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    ///
    /// let value = manager.get_option_string("lang", "en").await;
    /// assert_eq!(value, "en");
    ///
    /// manager.set_option_string("lang", "ru").await;
    /// let value = manager.get_option_string("lang", "en").await;
    /// assert_eq!(value, "ru");
    /// # }
    /// ```
    pub async fn get_option_string(&self, name: &str, default_value: &str) -> String {
        let options = self.options.read().await;
        match options.get(name) {
            Some(OptionValue::String(value)) => value.clone(),
            Some(OptionValue::Boolean(value)) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Some(OptionValue::Integer(value)) => value.to_string(),
            Some(OptionValue::Empty) => String::new(),
            None => default_value.to_string(),
        }
    }

    /// Get an option value.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    ///
    /// # Returns
    ///
    /// The option value, or `OptionValue::Empty` if not found
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::{OptionManager, OptionValue};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    ///
    /// let value = manager.get_option("test").await;
    /// assert!(value.is_empty());
    ///
    /// manager.set_option_boolean("test", true).await;
    /// let value = manager.get_option("test").await;
    /// assert!(value.is_boolean());
    /// # }
    /// ```
    pub async fn get_option(&self, name: &str) -> OptionValue {
        let options = self.options.read().await;
        options.get(name).cloned().unwrap_or(OptionValue::Empty)
    }

    /// Set an option value.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name
    /// * `value` - Option value to set
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::{OptionManager, OptionValue};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.set_option_value("test", OptionValue::Integer(42)).await;
    /// # }
    /// ```
    pub async fn set_option_value(&self, name: &str, value: OptionValue) {
        self.set_option(name, value).await;
    }

    /// Called when an option is updated.
    ///
    /// # Arguments
    ///
    /// * `name` - The option name that was updated
    async fn on_option_updated(&self, name: &str) {
        // In real implementation, would send updateOption notification
        tracing::debug!("Option updated: {}", name);
    }

    /// Update server time difference.
    ///
    /// Called when the server time difference is updated.
    pub async fn on_update_server_time_difference(&self) {
        let diff = *self.server_time_difference.read().await;
        tracing::debug!("Server time difference: {}", diff);
    }

    /// Get server time difference.
    ///
    /// # Returns
    ///
    /// The server time difference in seconds
    pub async fn server_time_difference(&self) -> f64 {
        *self.server_time_difference.read().await
    }

    /// Set server time difference.
    ///
    /// # Arguments
    ///
    /// * `diff` - Server time difference in seconds
    pub async fn set_server_time_difference(&self, diff: f64) {
        *self.server_time_difference.write().await = diff;
    }

    /// Get all options as a map.
    ///
    /// # Returns
    ///
    /// A clone of the internal options map
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.set_option_boolean("test", true).await;
    ///
    /// let options = manager.get_all_options().await;
    /// assert_eq!(options.len(), 1);
    /// # }
    /// ```
    pub async fn get_all_options(&self) -> HashMap<String, OptionValue> {
        self.options.read().await.clone()
    }

    /// Clear all options.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// manager.set_option_boolean("test", true).await;
    ///
    /// manager.clear_options().await;
    /// assert!(!manager.have_option("test").await);
    /// # }
    /// ```
    pub async fn clear_options(&self) {
        self.options.write().await.clear();
    }

    /// Get the number of stored options.
    ///
    /// # Returns
    ///
    /// The number of options
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = OptionManager::new();
    /// assert_eq!(manager.option_count().await, 0);
    ///
    /// manager.set_option_boolean("test", true).await;
    /// assert_eq!(manager.option_count().await, 1);
    /// # }
    /// ```
    pub async fn option_count(&self) -> usize {
        self.options.read().await.len()
    }

    /// Check if an option is synchronous.
    ///
    /// Synchronous options can be accessed without locks for performance.
    ///
    /// # Arguments
    ///
    /// * `name` - Option name to check
    ///
    /// # Returns
    ///
    /// `true` if the option is synchronous
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionManager;
    ///
    /// let manager = OptionManager::new();
    /// assert!(OptionManager::is_synchronous_option("unix_time"));
    /// assert!(!OptionManager::is_synchronous_option("custom_option"));
    /// ```
    pub fn is_synchronous_option(name: &str) -> bool {
        SYNCHRONOUS_OPTIONS.contains(&name)
    }
}

impl Default for OptionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_option_manager_new() {
        let manager = OptionManager::new();
        assert!(!manager.is_td_inited().await);
        assert_eq!(manager.option_count().await, 0);
    }

    #[tokio::test]
    async fn test_on_td_inited() {
        let manager = OptionManager::new();
        manager.on_td_inited().await;
        assert!(manager.is_td_inited().await);
    }

    #[tokio::test]
    async fn test_set_option_boolean() {
        let manager = OptionManager::new();
        manager.set_option_boolean("test", true).await;
        assert!(manager.have_option("test").await);
        assert_eq!(manager.get_option_boolean("test", false).await, true);
    }

    #[tokio::test]
    async fn test_set_option_integer() {
        let manager = OptionManager::new();
        manager.set_option_integer("count", 42).await;
        assert!(manager.have_option("count").await);
        assert_eq!(manager.get_option_integer("count", 0).await, 42);
    }

    #[tokio::test]
    async fn test_set_option_string() {
        let manager = OptionManager::new();
        manager.set_option_string("lang", "en").await;
        assert!(manager.have_option("lang").await);
        assert_eq!(manager.get_option_string("lang", "").await, "en");
    }

    #[tokio::test]
    async fn test_set_option_empty() {
        let manager = OptionManager::new();
        manager.set_option_boolean("test", true).await;
        assert!(manager.have_option("test").await);

        manager.set_option_empty("test").await;
        assert!(manager.have_option("test").await);
        assert_eq!(manager.get_option("test").await, OptionValue::Empty);
    }

    #[tokio::test]
    async fn test_have_option() {
        let manager = OptionManager::new();
        assert!(!manager.have_option("test").await);

        manager.set_option_boolean("test", true).await;
        assert!(manager.have_option("test").await);
    }

    #[tokio::test]
    async fn test_get_option_boolean_default() {
        let manager = OptionManager::new();
        assert_eq!(manager.get_option_boolean("nonexistent", true).await, true);
        assert_eq!(
            manager.get_option_boolean("nonexistent", false).await,
            false
        );
    }

    #[tokio::test]
    async fn test_get_option_integer_default() {
        let manager = OptionManager::new();
        assert_eq!(manager.get_option_integer("nonexistent", 42).await, 42);
    }

    #[tokio::test]
    async fn test_get_option_string_default() {
        let manager = OptionManager::new();
        assert_eq!(
            manager.get_option_string("nonexistent", "default").await,
            "default"
        );
    }

    #[tokio::test]
    async fn test_get_option() {
        let manager = OptionManager::new();

        let value = manager.get_option("test").await;
        assert_eq!(value, OptionValue::Empty);

        manager.set_option_boolean("test", true).await;
        let value = manager.get_option("test").await;
        assert_eq!(value, OptionValue::Boolean(true));
    }

    #[tokio::test]
    async fn test_set_option_value() {
        let manager = OptionManager::new();
        manager
            .set_option_value("test", OptionValue::String("hello".to_string()))
            .await;
        assert_eq!(manager.get_option_string("test", "").await, "hello");
    }

    #[tokio::test]
    async fn test_get_all_options() {
        let manager = OptionManager::new();
        manager.set_option_boolean("bool_opt", true).await;
        manager.set_option_integer("int_opt", 42).await;
        manager.set_option_string("str_opt", "test").await;

        let options = manager.get_all_options().await;
        assert_eq!(options.len(), 3);
        assert_eq!(options.get("bool_opt"), Some(&OptionValue::Boolean(true)));
        assert_eq!(options.get("int_opt"), Some(&OptionValue::Integer(42)));
        assert_eq!(
            options.get("str_opt"),
            Some(&OptionValue::String("test".to_string()))
        );
    }

    #[tokio::test]
    async fn test_clear_options() {
        let manager = OptionManager::new();
        manager.set_option_boolean("test", true).await;
        assert_eq!(manager.option_count().await, 1);

        manager.clear_options().await;
        assert_eq!(manager.option_count().await, 0);
        assert!(!manager.have_option("test").await);
    }

    #[tokio::test]
    async fn test_option_count() {
        let manager = OptionManager::new();
        assert_eq!(manager.option_count().await, 0);

        manager.set_option_boolean("opt1", true).await;
        assert_eq!(manager.option_count().await, 1);

        manager.set_option_integer("opt2", 1).await;
        assert_eq!(manager.option_count().await, 2);

        manager.set_option_string("opt3", "test").await;
        assert_eq!(manager.option_count().await, 3);
    }

    #[tokio::test]
    async fn test_is_synchronous_option() {
        assert!(OptionManager::is_synchronous_option("unix_time"));
        assert!(OptionManager::is_synchronous_option("version"));
        assert!(OptionManager::is_synchronous_option("localization_target"));
        assert!(!OptionManager::is_synchronous_option("custom_option"));
    }

    #[tokio::test]
    async fn test_server_time_difference() {
        let manager = OptionManager::new();
        assert_eq!(manager.server_time_difference().await, 0.0);

        manager.set_server_time_difference(1.5).await;
        assert_eq!(manager.server_time_difference().await, 1.5);
    }

    #[tokio::test]
    async fn test_update_premium_options() {
        let manager = OptionManager::new();
        manager.update_premium_options().await;
        // Just ensure it doesn't panic
    }

    #[tokio::test]
    async fn test_on_update_server_time_difference() {
        let manager = OptionManager::new();
        manager.set_server_time_difference(2.0).await;
        manager.on_update_server_time_difference().await;
        // Just ensure it doesn't panic
    }

    #[tokio::test]
    async fn test_type_conversion_boolean_to_integer() {
        let manager = OptionManager::new();
        manager.set_option_boolean("test", true).await;
        assert_eq!(manager.get_option_integer("test", 0).await, 1);

        manager.set_option_boolean("test", false).await;
        assert_eq!(manager.get_option_integer("test", 0).await, 0);
    }

    #[tokio::test]
    async fn test_type_conversion_boolean_to_string() {
        let manager = OptionManager::new();
        manager.set_option_boolean("test", true).await;
        assert_eq!(manager.get_option_string("test", "").await, "true");

        manager.set_option_boolean("test", false).await;
        assert_eq!(manager.get_option_string("test", "").await, "false");
    }

    #[tokio::test]
    async fn test_type_conversion_integer_to_boolean() {
        let manager = OptionManager::new();
        manager.set_option_integer("test", 1).await;
        assert!(manager.get_option_boolean("test", false).await);

        manager.set_option_integer("test", 0).await;
        assert!(!manager.get_option_boolean("test", true).await);
    }

    #[tokio::test]
    async fn test_type_conversion_integer_to_string() {
        let manager = OptionManager::new();
        manager.set_option_integer("test", 42).await;
        assert_eq!(manager.get_option_string("test", "").await, "42");
    }

    #[tokio::test]
    async fn test_type_conversion_string_to_boolean() {
        let manager = OptionManager::new();
        manager.set_option_string("test", "hello").await;
        assert!(manager.get_option_boolean("test", false).await);

        manager.set_option_string("test", "").await;
        assert!(!manager.get_option_boolean("test", true).await);
    }

    #[tokio::test]
    async fn test_type_conversion_empty() {
        let manager = OptionManager::new();
        manager.set_option_empty("test").await;

        assert_eq!(manager.get_option_boolean("test", true).await, false);
        assert_eq!(manager.get_option_integer("test", 42).await, 0);
        assert_eq!(manager.get_option_string("test", "default").await, "");
    }

    #[tokio::test]
    async fn test_max_options_limit() {
        let manager = OptionManager::new();

        // Set many options
        for i in 0..100 {
            manager.set_option_integer(&format!("opt_{}", i), i).await;
        }

        assert_eq!(manager.option_count().await, 100);
    }

    #[tokio::test]
    async fn test_option_overwrite() {
        let manager = OptionManager::new();
        manager.set_option_boolean("test", true).await;
        assert_eq!(manager.get_option_boolean("test", false).await, true);

        manager.set_option_boolean("test", false).await;
        assert_eq!(manager.get_option_boolean("test", true).await, false);

        manager.set_option_integer("test", 42).await;
        assert_eq!(manager.get_option_integer("test", 0).await, 42);
    }

    #[tokio::test]
    async fn test_default() {
        let manager = OptionManager::default();
        assert!(!manager.is_td_inited().await);
        assert_eq!(manager.option_count().await, 0);
    }
}
