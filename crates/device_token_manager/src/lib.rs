// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Rustgram DeviceTokenManager
//!
//! Push notification device token management for Telegram MTProto client.
//!
//! This crate provides types and utilities for managing push notification tokens
//! from various platforms (APNS, FCM, WNS, WebPush, etc.).
//!
//! ## Overview
//!
//! - [`TokenType`] - Types of push notification services
//! - [`TokenInfo`] - Device token information with state
//! - [`DeviceTokenManager`] - Manages device tokens
//!
//! ## Examples
//!
//! Creating a token manager:
//!
//! ```
//! use rustgram_device_token_manager::{DeviceTokenManager, TokenType, TokenInfo, TokenState};
//!
//! let mut manager = DeviceTokenManager::new();
//! let token_info = TokenInfo::new("apns-token".to_string(), TokenType::Apns);
//! manager.register_token(token_info, vec![]);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::derivable_impls)]

use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur in device token operations.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum TokenError {
    /// Invalid token format
    #[error("Invalid token format: {0}")]
    InvalidTokenFormat(String),

    /// Token not found
    #[error("Token not found for type: {0:?}")]
    TokenNotFound(TokenType),

    /// Token registration failed
    #[error("Token registration failed: {0}")]
    RegistrationFailed(String),

    /// Token state error
    #[error("Invalid token state: {0:?}")]
    InvalidTokenState(TokenState),
}

/// Types of push notification services.
///
/// Each variant represents a different push notification platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TokenType {
    /// Apple Push Notification Service
    Apns = 1,

    /// Firebase Cloud Messaging
    Fcm = 2,

    /// Microsoft Push Notification Service
    Mpns = 3,

    /// SimplePush (Firefox)
    SimplePush = 4,

    /// Ubuntu Phone
    UbuntuPhone = 5,

    /// BlackBerry Push
    BlackBerry = 6,

    /// Unused/reserved
    #[allow(dead_code)]
    Unused = 7,

    /// Windows Push Notification Services
    Wns = 8,

    /// Apple Push Notification Service VoIP
    ApnsVoip = 9,

    /// Web Push API
    WebPush = 10,

    /// Microsoft Push Notification Service VoIP
    MpnsVoip = 11,

    /// Tizen Push
    Tizen = 12,

    /// Huawei Push
    Huawei = 13,
}

impl TokenType {
    /// Gets all token types.
    #[must_use]
    pub const fn all() -> [TokenType; 13] {
        [
            TokenType::Apns,
            TokenType::Fcm,
            TokenType::Mpns,
            TokenType::SimplePush,
            TokenType::UbuntuPhone,
            TokenType::BlackBerry,
            TokenType::Unused,
            TokenType::Wns,
            TokenType::ApnsVoip,
            TokenType::WebPush,
            TokenType::MpnsVoip,
            TokenType::Tizen,
            TokenType::Huawei,
        ]
    }

    /// Gets the integer representation of the token type.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Gets the token type from an integer.
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(TokenType::Apns),
            2 => Some(TokenType::Fcm),
            3 => Some(TokenType::Mpns),
            4 => Some(TokenType::SimplePush),
            5 => Some(TokenType::UbuntuPhone),
            6 => Some(TokenType::BlackBerry),
            7 => Some(TokenType::Unused),
            8 => Some(TokenType::Wns),
            9 => Some(TokenType::ApnsVoip),
            10 => Some(TokenType::WebPush),
            11 => Some(TokenType::MpnsVoip),
            12 => Some(TokenType::Tizen),
            13 => Some(TokenType::Huawei),
            _ => None,
        }
    }

    /// Gets the name of the token type.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            TokenType::Apns => "Apns",
            TokenType::Fcm => "Fcm",
            TokenType::Mpns => "Mpns",
            TokenType::SimplePush => "SimplePush",
            TokenType::UbuntuPhone => "UbuntuPhone",
            TokenType::BlackBerry => "BlackBerry",
            TokenType::Unused => "Unused",
            TokenType::Wns => "Wns",
            TokenType::ApnsVoip => "ApnsVoip",
            TokenType::WebPush => "WebPush",
            TokenType::MpnsVoip => "MpnsVoip",
            TokenType::Tizen => "Tizen",
            TokenType::Huawei => "Huawei",
        }
    }
}

/// Token state machine.
///
/// Represents the current state of a token in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenState {
    /// Token is synchronized with the server
    Sync,

    /// Token needs to be unregistered
    Unregister,

    /// Token needs to be registered
    Register,

    /// Token needs to be re-registered
    Reregister,
}

impl TokenState {
    /// Gets the name of the token state.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            TokenState::Sync => "Synchronized",
            TokenState::Unregister => "Unregister",
            TokenState::Register => "Register",
            TokenState::Reregister => "Reregister",
        }
    }
}

/// Device token information.
///
/// Contains all information about a push notification token.
///
/// # Examples
///
/// ```
/// use rustgram_device_token_manager::{TokenInfo, TokenType, TokenState};
///
/// let info = TokenInfo::new("device-token-123".to_string(), TokenType::Apns);
/// assert_eq!(info.token(), "device-token-123");
/// assert_eq!(info.token_type(), TokenType::Apns);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenInfo {
    /// The actual token string
    token: String,

    /// Type of push service
    token_type: TokenType,

    /// Current state of the token
    state: TokenState,

    /// Other user IDs for this token (multi-user support)
    other_user_ids: Vec<i64>,

    /// Whether the app is in sandbox mode (APNS)
    is_app_sandbox: bool,

    /// Whether encryption is enabled
    encrypt: bool,

    /// Encryption key (if encrypt is true)
    encryption_key: Option<String>,

    /// Encryption key ID
    encryption_key_id: Option<i64>,
}

impl TokenInfo {
    /// Creates a new token info with minimal settings.
    ///
    /// # Arguments
    ///
    /// * `token` - The device token string
    /// * `token_type` - The type of push service
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{TokenInfo, TokenType};
    ///
    /// let info = TokenInfo::new("token123".to_string(), TokenType::Fcm);
    /// ```
    #[must_use]
    pub const fn new(token: String, token_type: TokenType) -> Self {
        Self {
            token,
            token_type,
            state: TokenState::Sync,
            other_user_ids: Vec::new(),
            is_app_sandbox: false,
            encrypt: false,
            encryption_key: None,
            encryption_key_id: None,
        }
    }

    /// Gets the token string.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Gets the token type.
    #[must_use]
    pub const fn token_type(&self) -> TokenType {
        self.token_type
    }

    /// Gets the token state.
    #[must_use]
    pub const fn state(&self) -> TokenState {
        self.state
    }

    /// Sets the token state.
    pub fn set_state(&mut self, state: TokenState) {
        self.state = state;
    }

    /// Gets the other user IDs.
    #[must_use]
    pub fn other_user_ids(&self) -> &[i64] {
        &self.other_user_ids
    }

    /// Sets other user IDs.
    pub fn set_other_user_ids(&mut self, other_user_ids: Vec<i64>) {
        self.other_user_ids = other_user_ids;
    }

    /// Gets whether the app is in sandbox mode.
    #[must_use]
    pub const fn is_app_sandbox(&self) -> bool {
        self.is_app_sandbox
    }

    /// Sets sandbox mode.
    pub fn set_app_sandbox(&mut self, is_app_sandbox: bool) {
        self.is_app_sandbox = is_app_sandbox;
    }

    /// Gets whether encryption is enabled.
    #[must_use]
    pub const fn is_encrypted(&self) -> bool {
        self.encrypt
    }

    /// Gets the encryption key.
    #[must_use]
    pub fn encryption_key(&self) -> Option<&str> {
        self.encryption_key.as_deref()
    }

    /// Gets the encryption key ID.
    #[must_use]
    pub const fn encryption_key_id(&self) -> Option<i64> {
        self.encryption_key_id
    }

    /// Sets encryption settings.
    pub fn set_encryption(&mut self, key: String, key_id: i64) {
        self.encrypt = true;
        self.encryption_key = Some(key);
        self.encryption_key_id = Some(key_id);
    }

    /// Marks the token for registration.
    pub fn mark_for_registration(&mut self) {
        self.state = TokenState::Register;
    }

    /// Marks the token for unregistration.
    pub fn mark_for_unregistration(&mut self) {
        self.state = TokenState::Unregister;
    }

    /// Marks the token for re-registration.
    pub fn mark_for_reregistration(&mut self) {
        self.state = TokenState::Reregister;
    }

    /// Marks the token as synchronized.
    pub fn mark_as_synchronized(&mut self) {
        self.state = TokenState::Sync;
    }
}

/// Device token manager.
///
/// Manages push notification tokens for different platforms.
///
/// # Examples
///
/// ```
/// use rustgram_device_token_manager::{DeviceTokenManager, TokenType, TokenInfo};
///
/// let mut manager = DeviceTokenManager::new();
///
/// // Register a token
/// let token_info = TokenInfo::new("apns-token".to_string(), TokenType::Apns);
/// manager.register_token(token_info, vec![]);
///
/// // Get the token back
/// let token = manager.get_token(TokenType::Apns);
/// assert!(token.is_some());
/// ```
#[derive(Debug, Default)]
pub struct DeviceTokenManager {
    /// Tokens by type
    tokens: HashMap<TokenType, TokenInfo>,
}

impl DeviceTokenManager {
    /// Creates a new device token manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::DeviceTokenManager;
    ///
    /// let manager = DeviceTokenManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a device token.
    ///
    /// # Arguments
    ///
    /// * `token_info` - The token information
    /// * `other_user_ids` - Other user IDs for multi-user support
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// let token_info = TokenInfo::new("token123".to_string(), TokenType::Fcm);
    /// manager.register_token(token_info, vec![]);
    /// ```
    pub fn register_token(&mut self, mut token_info: TokenInfo, other_user_ids: Vec<i64>) {
        token_info.set_other_user_ids(other_user_ids);
        token_info.mark_for_registration();
        self.tokens.insert(token_info.token_type(), token_info);
    }

    /// Unregisters a device token.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token to unregister
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(TokenError)` if token not found
    ///
    /// # Errors
    ///
    /// Returns `TokenError::TokenNotFound` if the token doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType, TokenError};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// let token_info = TokenInfo::new("token123".to_string(), TokenType::Fcm);
    /// manager.register_token(token_info, vec![]);
    ///
    /// manager.unregister_token(TokenType::Fcm).unwrap();
    /// ```
    pub fn unregister_token(&mut self, token_type: TokenType) -> Result<(), TokenError> {
        let token = self
            .tokens
            .get_mut(&token_type)
            .ok_or(TokenError::TokenNotFound(token_type))?;
        token.mark_for_unregistration();
        Ok(())
    }

    /// Reregisters a device token.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token to reregister
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(TokenError)` if token not found
    ///
    /// # Errors
    ///
    /// Returns `TokenError::TokenNotFound` if the token doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// let token_info = TokenInfo::new("token123".to_string(), TokenType::Fcm);
    /// manager.register_token(token_info, vec![]);
    ///
    /// manager.reregister_token(TokenType::Fcm).unwrap();
    /// ```
    pub fn reregister_token(&mut self, token_type: TokenType) -> Result<(), TokenError> {
        let token = self
            .tokens
            .get_mut(&token_type)
            .ok_or(TokenError::TokenNotFound(token_type))?;
        token.mark_for_reregistration();
        Ok(())
    }

    /// Gets a token by type.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token to get
    ///
    /// # Returns
    ///
    /// `Some(TokenInfo)` if found, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// let token_info = TokenInfo::new("token123".to_string(), TokenType::Fcm);
    /// manager.register_token(token_info, vec![]);
    ///
    /// let token = manager.get_token(TokenType::Fcm);
    /// assert!(token.is_some());
    /// ```
    #[must_use]
    pub fn get_token(&self, token_type: TokenType) -> Option<&TokenInfo> {
        self.tokens.get(&token_type)
    }

    /// Gets a mutable token by type.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token to get
    ///
    /// # Returns
    ///
    /// `Some(&mut TokenInfo)` if found, `None` otherwise
    #[must_use]
    pub fn get_token_mut(&mut self, token_type: TokenType) -> Option<&mut TokenInfo> {
        self.tokens.get_mut(&token_type)
    }

    /// Removes a token by type.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token to remove
    ///
    /// # Returns
    ///
    /// `Some(TokenInfo)` if removed, `None` if not found
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// let token_info = TokenInfo::new("token123".to_string(), TokenType::Fcm);
    /// manager.register_token(token_info, vec![]);
    ///
    /// let removed = manager.remove_token(TokenType::Fcm);
    /// assert!(removed.is_some());
    /// ```
    pub fn remove_token(&mut self, token_type: TokenType) -> Option<TokenInfo> {
        self.tokens.remove(&token_type)
    }

    /// Gets all registered tokens.
    ///
    /// # Returns
    ///
    /// Iterator over all token info
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// manager.register_token(TokenInfo::new("token1".to_string(), TokenType::Apns), vec![]);
    /// manager.register_token(TokenInfo::new("token2".to_string(), TokenType::Fcm), vec![]);
    ///
    /// let all_tokens: Vec<_> = manager.get_all_tokens().collect();
    /// assert_eq!(all_tokens.len(), 2);
    /// ```
    pub fn get_all_tokens(&self) -> impl Iterator<Item = &TokenInfo> {
        self.tokens.values()
    }

    /// Gets the count of registered tokens.
    ///
    /// # Returns
    ///
    /// Number of registered tokens
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// assert_eq!(manager.token_count(), 0);
    ///
    /// manager.register_token(TokenInfo::new("token1".to_string(), TokenType::Apns), vec![]);
    /// assert_eq!(manager.token_count(), 1);
    /// ```
    #[must_use]
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    /// Clears all tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_device_token_manager::{DeviceTokenManager, TokenInfo, TokenType};
    ///
    /// let mut manager = DeviceTokenManager::new();
    /// manager.register_token(TokenInfo::new("token1".to_string(), TokenType::Apns), vec![]);
    ///
    /// manager.clear();
    /// assert_eq!(manager.token_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.tokens.clear();
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-device-token-manager";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== TokenType Tests ==========

    #[test]
    fn test_token_type_as_i32() {
        assert_eq!(TokenType::Apns.as_i32(), 1);
        assert_eq!(TokenType::Fcm.as_i32(), 2);
        assert_eq!(TokenType::WebPush.as_i32(), 10);
    }

    #[test]
    fn test_token_type_from_i32() {
        assert_eq!(TokenType::from_i32(1), Some(TokenType::Apns));
        assert_eq!(TokenType::from_i32(2), Some(TokenType::Fcm));
        assert_eq!(TokenType::from_i32(999), None);
    }

    #[test]
    fn test_token_type_name() {
        assert_eq!(TokenType::Apns.name(), "Apns");
        assert_eq!(TokenType::Fcm.name(), "Fcm");
        assert_eq!(TokenType::WebPush.name(), "WebPush");
    }

    #[test]
    fn test_token_type_all() {
        let all = TokenType::all();
        assert_eq!(all.len(), 13);
        assert!(all.contains(&TokenType::Apns));
        assert!(all.contains(&TokenType::Fcm));
    }

    // ========== TokenState Tests ==========

    #[test]
    fn test_token_state_name() {
        assert_eq!(TokenState::Sync.name(), "Synchronized");
        assert_eq!(TokenState::Register.name(), "Register");
        assert_eq!(TokenState::Unregister.name(), "Unregister");
        assert_eq!(TokenState::Reregister.name(), "Reregister");
    }

    // ========== TokenInfo Tests ==========

    #[test]
    fn test_token_info_new() {
        let info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        assert_eq!(info.token(), "token123");
        assert_eq!(info.token_type(), TokenType::Apns);
        assert_eq!(info.state(), TokenState::Sync);
        assert!(!info.is_app_sandbox());
        assert!(!info.is_encrypted());
    }

    #[test]
    fn test_token_info_set_state() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        info.set_state(TokenState::Register);
        assert_eq!(info.state(), TokenState::Register);
    }

    #[test]
    fn test_token_info_other_user_ids() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        assert_eq!(info.other_user_ids().len(), 0);

        info.set_other_user_ids(vec![123, 456]);
        assert_eq!(info.other_user_ids(), &[123, 456]);
    }

    #[test]
    fn test_token_info_sandbox() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        assert!(!info.is_app_sandbox());

        info.set_app_sandbox(true);
        assert!(info.is_app_sandbox());
    }

    #[test]
    fn test_token_info_encryption() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        assert!(!info.is_encrypted());
        assert_eq!(info.encryption_key(), None);
        assert_eq!(info.encryption_key_id(), None);

        info.set_encryption("secret-key".to_string(), 12345);
        assert!(info.is_encrypted());
        assert_eq!(info.encryption_key(), Some("secret-key"));
        assert_eq!(info.encryption_key_id(), Some(12345));
    }

    #[test]
    fn test_token_info_mark_for_registration() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        info.mark_for_registration();
        assert_eq!(info.state(), TokenState::Register);
    }

    #[test]
    fn test_token_info_mark_for_unregistration() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        info.mark_for_unregistration();
        assert_eq!(info.state(), TokenState::Unregister);
    }

    #[test]
    fn test_token_info_mark_for_reregistration() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        info.mark_for_reregistration();
        assert_eq!(info.state(), TokenState::Reregister);
    }

    #[test]
    fn test_token_info_mark_as_synchronized() {
        let mut info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        info.mark_for_registration();
        assert_eq!(info.state(), TokenState::Register);

        info.mark_as_synchronized();
        assert_eq!(info.state(), TokenState::Sync);
    }

    // ========== DeviceTokenManager Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = DeviceTokenManager::new();
        assert_eq!(manager.token_count(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = DeviceTokenManager::default();
        assert_eq!(manager.token_count(), 0);
    }

    #[test]
    fn test_register_token() {
        let mut manager = DeviceTokenManager::new();
        let token_info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        manager.register_token(token_info, vec![]);

        assert_eq!(manager.token_count(), 1);

        let token = manager.get_token(TokenType::Apns);
        assert!(token.is_some());
        assert_eq!(token.unwrap().token(), "token123");
    }

    #[test]
    fn test_unregister_token() {
        let mut manager = DeviceTokenManager::new();
        let token_info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        manager.register_token(token_info, vec![]);

        let result = manager.unregister_token(TokenType::Apns);
        assert!(result.is_ok());

        let token = manager.get_token(TokenType::Apns);
        assert!(token.is_some());
        assert_eq!(token.unwrap().state(), TokenState::Unregister);
    }

    #[test]
    fn test_unregister_token_not_found() {
        let mut manager = DeviceTokenManager::new();
        let result = manager.unregister_token(TokenType::Apns);
        assert_eq!(result, Err(TokenError::TokenNotFound(TokenType::Apns)));
    }

    #[test]
    fn test_reregister_token() {
        let mut manager = DeviceTokenManager::new();
        let token_info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        manager.register_token(token_info, vec![]);

        let result = manager.reregister_token(TokenType::Apns);
        assert!(result.is_ok());

        let token = manager.get_token(TokenType::Apns);
        assert!(token.is_some());
        assert_eq!(token.unwrap().state(), TokenState::Reregister);
    }

    #[test]
    fn test_get_token_mut() {
        let mut manager = DeviceTokenManager::new();
        let token_info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        manager.register_token(token_info, vec![]);

        if let Some(token) = manager.get_token_mut(TokenType::Apns) {
            token.set_app_sandbox(true);
        }

        let token = manager.get_token(TokenType::Apns);
        assert!(token.is_some());
        assert!(token.unwrap().is_app_sandbox());
    }

    #[test]
    fn test_remove_token() {
        let mut manager = DeviceTokenManager::new();
        let token_info = TokenInfo::new("token123".to_string(), TokenType::Apns);
        manager.register_token(token_info, vec![]);

        let removed = manager.remove_token(TokenType::Apns);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().token(), "token123");

        assert_eq!(manager.token_count(), 0);
    }

    #[test]
    fn test_get_all_tokens() {
        let mut manager = DeviceTokenManager::new();
        manager.register_token(
            TokenInfo::new("token1".to_string(), TokenType::Apns),
            vec![],
        );
        manager.register_token(TokenInfo::new("token2".to_string(), TokenType::Fcm), vec![]);

        let all_tokens: Vec<_> = manager.get_all_tokens().collect();
        assert_eq!(all_tokens.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut manager = DeviceTokenManager::new();
        manager.register_token(
            TokenInfo::new("token1".to_string(), TokenType::Apns),
            vec![],
        );
        manager.register_token(TokenInfo::new("token2".to_string(), TokenType::Fcm), vec![]);

        manager.clear();
        assert_eq!(manager.token_count(), 0);
    }

    #[test]
    fn test_multiple_tokens_same_type() {
        let mut manager = DeviceTokenManager::new();
        manager.register_token(
            TokenInfo::new("token1".to_string(), TokenType::Apns),
            vec![],
        );

        // Second registration should overwrite
        manager.register_token(
            TokenInfo::new("token2".to_string(), TokenType::Apns),
            vec![],
        );

        assert_eq!(manager.token_count(), 1);
        assert_eq!(
            manager.get_token(TokenType::Apns).unwrap().token(),
            "token2"
        );
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-device-token-manager");
    }
}
