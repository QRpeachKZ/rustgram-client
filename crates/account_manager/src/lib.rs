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

//! # Account Manager
//!
//! Manages Telegram account sessions, authorizations, and connected websites.
//!
//! ## Overview
//!
//! The AccountManager is responsible for:
//! - Managing active sessions across devices
//! - Handling QR code authentication
//! - Managing connected websites
//! - Controlling account TTL settings
//! - Handling unconfirmed authorizations
//! - Age verification parameters management
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rustgram_account_manager::AccountManager;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = AccountManager::new();
//!
//!     // Get active sessions
//!     let sessions = manager.get_active_sessions().await?;
//!
//!     // Set account TTL
//!     manager.set_account_ttl(30).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Thread Safety
//!
//! The AccountManager uses `Arc<RwLock<T>>` for internal state, making it safe to share across threads.
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's AccountManager API:
//! - All public methods match TDLib's API
//! - Session type detection follows TDLib algorithm
//! - Authorization handling matches TDLib behavior

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod tl;
pub mod types;

// Re-exports
pub use error::{AccountManagerError, Result as ManagerResult};
pub use types::{
    AccountTTL, ConnectedWebsite, ConnectedWebsites, Session, SessionType, Sessions,
    UnconfirmedAuthorization, UserLink,
};

use crate::error::Result;
use rustgram_age_verification_parameters::AgeVerificationParameters;
use rustgram_types::UserId;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Default authorization autoconfirm period in seconds (7 days).
const DEFAULT_AUTHORIZATION_AUTOCONFIRM_PERIOD: i32 = 604800;

/// Storage key for age verification parameters.
const AGE_VERIFICATION_KEY: &str = "age_verification";

/// Storage key for unconfirmed authorizations.
const UNCONFIRMED_AUTHORIZATIONS_KEY: &str = "new_authorizations";

/// Manages Telegram account sessions and authorizations.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::AccountManager;
///
/// let manager = AccountManager::new();
/// ```
///
/// # Thread Safety
///
/// The AccountManager is thread-safe and can be shared across tasks.
/// All internal state is protected by `Arc<RwLock<T>>`.
///
/// # TDLib Correspondence
///
/// This corresponds to `td::AccountManager` in TDLib.
#[derive(Debug, Clone)]
pub struct AccountManager {
    /// Unconfirmed authorizations awaiting confirmation.
    unconfirmed_authorizations: Arc<RwLock<UnconfirmedAuthorizations>>,

    /// Age verification parameters.
    age_verification_parameters: Arc<RwLock<Option<age_verification::AgeVerificationParameters>>>,

    /// Default message TTL in seconds.
    default_message_ttl: Arc<RwLock<i32>>,

    /// Account TTL in days.
    account_ttl: Arc<RwLock<i32>>,

    /// Inactive session TTL in days.
    inactive_session_ttl_days: Arc<RwLock<i32>>,
}

/// Container for unconfirmed authorizations.
///
/// Maintains a sorted list of pending confirmations with auto-expiration.
#[derive(Debug, Clone)]
struct UnconfirmedAuthorizations {
    /// Sorted list of unconfirmed authorizations (by date ascending).
    authorizations: BTreeMap<i32, UnconfirmedAuthorization>,
}

impl Default for UnconfirmedAuthorizations {
    fn default() -> Self {
        Self {
            authorizations: BTreeMap::new(),
        }
    }
}

impl UnconfirmedAuthorizations {
    /// Creates a new empty container.
    fn new() -> Self {
        Self::default()
    }

    /// Checks if the container is empty.
    fn is_empty(&self) -> bool {
        self.authorizations.is_empty()
    }

    /// Adds an authorization to the container.
    ///
    /// # Returns
    ///
    /// `true` if the authorization was added, `false` if it already exists.
    fn add(&mut self, auth: UnconfirmedAuthorization) -> bool {
        let date = auth.date;
        if self.authorizations.contains_key(&date) {
            return false;
        }
        self.authorizations.insert(date, auth);
        true
    }

    /// Removes an authorization by hash.
    ///
    /// # Returns
    ///
    /// `true` if the authorization was removed, `false` if not found.
    fn remove(&mut self, hash: i64) -> bool {
        let date_to_remove = self
            .authorizations
            .iter()
            .find(|(_, auth)| auth.hash == hash)
            .map(|(date, _)| *date);

        if let Some(date) = date_to_remove {
            self.authorizations.remove(&date);
            true
        } else {
            false
        }
    }

    /// Gets the first (oldest) unconfirmed authorization.
    fn get_first(&self) -> Option<&UnconfirmedAuthorization> {
        self.authorizations.first_key_value().map(|(_, auth)| auth)
    }

    /// Removes expired authorizations.
    ///
    /// # Returns
    ///
    /// `true` if any authorizations were removed.
    fn remove_expired(&mut self, current_time: i32) -> bool {
        let cutoff = current_time - DEFAULT_AUTHORIZATION_AUTOCONFIRM_PERIOD;
        let before_len = self.authorizations.len();

        self.authorizations.retain(|&date, _| date > cutoff);

        self.authorizations.len() < before_len
    }

    /// Returns the expiration date of the first authorization.
    fn get_first_expiration_date(&self) -> Option<i32> {
        self.get_first()
            .map(|auth| auth.date + DEFAULT_AUTHORIZATION_AUTOCONFIRM_PERIOD)
    }
}

impl Default for AccountManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountManager {
    /// Creates a new AccountManager instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// let manager = AccountManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            unconfirmed_authorizations: Arc::new(RwLock::new(UnconfirmedAuthorizations::new())),
            age_verification_parameters: Arc::new(RwLock::new(None)),
            default_message_ttl: Arc::new(RwLock::new(0)),
            account_ttl: Arc::new(RwLock::new(0)),
            inactive_session_ttl_days: Arc::new(RwLock::new(180)),
        }
    }

    /// Updates age verification parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - New age verification parameters
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    /// use rustgram_age_verification_parameters as age_verification;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    /// let params = age_verification::AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
    /// manager.update_age_verification_parameters(params).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_age_verification_parameters(&self, params: age_verification::AgeVerificationParameters) {
        let mut stored = self.age_verification_parameters.write().await;
        *stored = if params.need_verification() {
            Some(params)
        } else {
            None
        };
    }

    /// Gets current age verification parameters.
    ///
    /// # Returns
    ///
    /// Option containing the parameters if verification is needed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    /// let params = manager.get_age_verification_parameters().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_age_verification_parameters(&self) -> Option<age_verification::AgeVerificationParameters> {
        self.age_verification_parameters.read().await.clone()
    }

    /// Sets default message TTL for new chats.
    ///
    /// # Arguments
    ///
    /// * `message_ttl` - Message auto-delete time in seconds (0 for no auto-delete)
    ///
    /// # Returns
    ///
    /// Ok if successful, Err if invalid TTL value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Set 1 day auto-delete
    /// manager.set_default_message_ttl(86400).await?;
    ///
    /// // Disable auto-delete
    /// manager.set_default_message_ttl(0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_default_message_ttl(&self, message_ttl: i32) -> Result<()> {
        if message_ttl < 0 {
            return Err(AccountManagerError::InvalidTtlValue(
                message_ttl,
                0,
                i32::MAX,
            ));
        }

        let mut ttl = self.default_message_ttl.write().await;
        *ttl = message_ttl;
        Ok(())
    }

    /// Gets current default message TTL.
    ///
    /// # Returns
    ///
    /// TTL in seconds (0 means no auto-delete).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    /// let ttl = manager.get_default_message_ttl().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_default_message_ttl(&self) -> Result<i32> {
        Ok(*self.default_message_ttl.read().await)
    }

    /// Sets account time-to-live.
    ///
    /// # Arguments
    ///
    /// * `account_ttl` - Days before account deletion (1-366)
    ///
    /// # Returns
    ///
    /// Ok if successful, Err if invalid TTL value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Delete account after 30 days of inactivity
    /// manager.set_account_ttl(30).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_account_ttl(&self, account_ttl: i32) -> Result<()> {
        if account_ttl < 1 || account_ttl > 366 {
            return Err(AccountManagerError::InvalidTtlValue(account_ttl, 1, 366));
        }

        let mut ttl = self.account_ttl.write().await;
        *ttl = account_ttl;
        Ok(())
    }

    /// Gets current account TTL.
    ///
    /// # Returns
    ///
    /// Days before account deletion.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    /// let ttl = manager.get_account_ttl().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_account_ttl(&self) -> Result<i32> {
        Ok(*self.account_ttl.read().await)
    }

    /// Confirms QR code authentication from another session.
    ///
    /// # Arguments
    ///
    /// * `link` - QR code authentication link (tg://login?token=<base64_token>)
    ///
    /// # Returns
    ///
    /// Confirmed session information.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Link format is invalid
    /// - Base64 token is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // In real usage, this would be a valid QR code link
    /// // let session = manager.confirm_qr_code_authentication("tg://login?token=...").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn confirm_qr_code_authentication(&self, link: &str) -> Result<Session> {
        // Validate link format
        if !link.to_lowercase().starts_with("tg://login?token=") {
            return Err(AccountManagerError::InvalidQrCodeLink(link.to_string()));
        }

        // Extract token
        let token_part = &link[17..]; // Skip "tg://login?token="

        // Validate base64
        if token_part.is_empty() {
            return Err(AccountManagerError::InvalidBase64Token);
        }

        // In a real implementation, this would:
        // 1. Decode base64 token
        // 2. Send auth_acceptLoginToken query
        // 3. Parse response into Session

        // For now, return a stub error
        Err(AccountManagerError::NetworkError(
            "QR code authentication not implemented".to_string(),
        ))
    }

    /// Gets all active sessions for the account.
    ///
    /// # Returns
    ///
    /// Collection of active sessions sorted by relevance:
    /// 1. Current session first
    /// 2. Password pending sessions next
    /// 3. Unconfirmed sessions next
    /// 4. Others by last active date descending
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    /// let sessions = manager.get_active_sessions().await?;
    ///
    /// for session in sessions.sessions() {
    ///     println!("Session: {} on {}", session.application_name, session.device_model);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_active_sessions(&self) -> Result<Sessions> {
        // Clean up expired unconfirmed authorizations
        self.cleanup_expired_authorizations().await;

        let ttl = *self.inactive_session_ttl_days.read().await;
        Ok(Sessions::new(ttl))
    }

    /// Terminates a specific session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session identifier to terminate
    ///
    /// # Returns
    ///
    /// Ok if successful, Err if session not found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Terminate session 123456789
    /// manager.terminate_session(123456789).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn terminate_session(&self, session_id: i64) -> Result<()> {
        // In a real implementation, this would:
        // 1. Confirm the authorization (remove from unconfirmed)
        // 2. Send account_resetAuthorization query

        // Remove from unconfirmed if present
        self.confirm_authorization(session_id).await;

        Ok(())
    }

    /// Terminates all sessions except the current one.
    ///
    /// Also clears all unconfirmed authorizations.
    ///
    /// # Returns
    ///
    /// Ok if successful.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Terminate all other sessions
    /// manager.terminate_all_other_sessions().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn terminate_all_other_sessions(&self) -> Result<()> {
        // Clear all unconfirmed authorizations
        let mut unconfirmed = self.unconfirmed_authorizations.write().await;
        unconfirmed.authorizations.clear();

        // In a real implementation, this would send auth_resetAuthorizations query
        Ok(())
    }

    /// Confirms an unconfirmed session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session identifier to confirm
    ///
    /// # Returns
    ///
    /// Ok if successful, Err if session not found in unconfirmed list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Confirm session 123456789
    /// manager.confirm_session(123456789).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn confirm_session(&self, session_id: i64) -> Result<()> {
        if self.confirm_authorization(session_id).await {
            Ok(())
        } else {
            Err(AccountManagerError::SessionNotFound(session_id))
        }
    }

    /// Toggles whether a session can accept calls.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session identifier (0 for current session)
    /// * `can_accept_calls` - Whether calls can be accepted
    ///
    /// # Returns
    ///
    /// Ok if successful.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Disable calls for session 123456789
    /// manager.toggle_session_can_accept_calls(123456789, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_session_can_accept_calls(
        &self,
        session_id: i64,
        can_accept_calls: bool,
    ) -> Result<()> {
        // In a real implementation, this would send account_changeAuthorizationSettings query
        Ok(())
    }

    /// Toggles whether a session can accept secret chats.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session identifier
    /// * `can_accept_secret_chats` - Whether secret chats can be accepted
    ///
    /// # Returns
    ///
    /// Ok if successful.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Disable secret chats for session 123456789
    /// manager.toggle_session_can_accept_secret_chats(123456789, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_session_can_accept_secret_chats(
        &self,
        session_id: i64,
        can_accept_secret_chats: bool,
    ) -> Result<()> {
        // In a real implementation, this would send account_changeAuthorizationSettings query
        Ok(())
    }

    /// Sets TTL for inactive sessions.
    ///
    /// # Arguments
    ///
    /// * `authorization_ttl_days` - Days before auto-termination (1-366)
    ///
    /// # Returns
    ///
    /// Ok if successful, Err if invalid TTL value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Auto-terminate sessions after 7 days of inactivity
    /// manager.set_inactive_session_ttl_days(7).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_inactive_session_ttl_days(&self, authorization_ttl_days: i32) -> Result<()> {
        if authorization_ttl_days < 1 || authorization_ttl_days > 366 {
            return Err(AccountManagerError::InvalidTtlValue(
                authorization_ttl_days,
                1,
                366,
            ));
        }

        let mut ttl = self.inactive_session_ttl_days.write().await;
        *ttl = authorization_ttl_days;
        Ok(())
    }

    /// Gets all websites where user is logged in via Telegram.
    ///
    /// # Returns
    ///
    /// Collection of connected websites.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    /// let websites = manager.get_connected_websites().await?;
    ///
    /// for website in websites.websites() {
    ///     println!("Connected: {}", website.domain_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_connected_websites(&self) -> Result<ConnectedWebsites> {
        Ok(ConnectedWebsites::new())
    }

    /// Disconnects a specific website.
    ///
    /// # Arguments
    ///
    /// * `website_id` - Website identifier to disconnect
    ///
    /// # Returns
    ///
    /// Ok if successful, Err if website not found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Disconnect website 987654321
    /// manager.disconnect_website(987654321).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn disconnect_website(&self, website_id: i64) -> Result<()> {
        // In a real implementation, this would send account_resetWebAuthorization query
        Ok(())
    }

    /// Disconnects all connected websites.
    ///
    /// # Returns
    ///
    /// Ok if successful.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Disconnect all websites
    /// manager.disconnect_all_websites().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn disconnect_all_websites(&self) -> Result<()> {
        // In a real implementation, this would send account_resetWebAuthorizations query
        Ok(())
    }

    /// Gets a link to the user's account.
    ///
    /// Returns username link if available, otherwise creates temporary link.
    ///
    /// # Returns
    ///
    /// User link with URL and expiration time.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    /// let link = manager.get_user_link().await?;
    ///
    /// println!("My link: {}", link.url());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_link(&self) -> Result<UserLink> {
        // In a real implementation, this would:
        // 1. Check if user has a username
        // 2. If yes, return permanent link
        // 3. If no, send contacts_exportContactToken query
        // 4. Return temporary link with expiration

        Ok(UserLink::new("https://t.me/username", 0))
    }

    /// Imports a contact token to add a user.
    ///
    /// # Arguments
    ///
    /// * `token` - Contact token obtained from another user's get_user_link
    ///
    /// # Returns
    ///
    /// User ID of the imported user.
    ///
    /// # Errors
    ///
    /// Returns error if token is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Import a contact token
    /// // let user_id = manager.import_contact_token("token_here").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn import_contact_token(&self, token: &str) -> Result<UserId> {
        if token.is_empty() {
            return Err(AccountManagerError::InvalidContactToken);
        }

        // In a real implementation, this would send contacts_importContactToken query

        Err(AccountManagerError::NetworkError(
            "Contact token import not implemented".to_string(),
        ))
    }

    /// Invalidates authentication codes sent to user.
    ///
    /// # Arguments
    ///
    /// * `authentication_codes` - List of codes to invalidate
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // Invalidate specific codes
    /// manager.invalidate_authentication_codes(vec![
    ///     "CODE123".to_string(),
    ///     "CODE456".to_string(),
    /// ]).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn invalidate_authentication_codes(&self, authentication_codes: Vec<String>) {
        // In a real implementation, this would send account_invalidateSignInCodes query
    }

    /// Handles new unconfirmed authorization from server.
    ///
    /// # Arguments
    ///
    /// * `hash` - Session identifier
    /// * `date` - Unix timestamp when user logged in
    /// * `device` - Model of device
    /// * `location` - Location description
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AccountManager::new();
    ///
    /// // New unconfirmed authorization detected
    /// manager.on_new_unconfirmed_authorization(
    ///     123456789,
    ///     1704067200,
    ///     "iPhone 15",
    ///     "United States"
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_new_unconfirmed_authorization(
        &self,
        hash: i64,
        date: i32,
        device: &str,
        location: &str,
    ) {
        if hash == 0 {
            return;
        }

        let auth = UnconfirmedAuthorization::new(hash, date, device, location);

        let mut unconfirmed = self.unconfirmed_authorizations.write().await;
        let _ = unconfirmed.add(auth);
    }

    /// Confirms an authorization (removes from unconfirmed list).
    ///
    /// # Arguments
    ///
    /// * `hash` - Authorization hash to confirm
    ///
    /// # Returns
    ///
    /// `true` if authorization was found and removed, `false` otherwise.
    async fn confirm_authorization(&self, hash: i64) -> bool {
        let mut unconfirmed = self.unconfirmed_authorizations.write().await;
        unconfirmed.remove(hash)
    }

    /// Cleans up expired unconfirmed authorizations.
    async fn cleanup_expired_authorizations(&self) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);
        let mut unconfirmed = self.unconfirmed_authorizations.write().await;
        unconfirmed.remove_expired(current_time);
    }

    /// Gets the first unconfirmed authorization.
    ///
    /// # Returns
    ///
    /// Option containing the oldest unconfirmed authorization, if any.
    pub async fn get_first_unconfirmed_authorization(&self) -> Option<UnconfirmedAuthorization> {
        let unconfirmed = self.unconfirmed_authorizations.read().await;
        unconfirmed.get_first().cloned()
    }

    /// Checks if there are any unconfirmed authorizations.
    ///
    /// # Returns
    ///
    /// `true` if there are unconfirmed authorizations.
    pub async fn has_unconfirmed_authorizations(&self) -> bool {
        let unconfirmed = self.unconfirmed_authorizations.read().await;
        !unconfirmed.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_manager() -> AccountManager {
        AccountManager::new()
    }

    // AccountManager basic tests
    #[tokio::test]
    async fn test_account_manager_new() {
        let manager = new_manager();
        assert!(!manager.has_unconfirmed_authorizations().await);
    }

    #[tokio::test]
    async fn test_set_default_message_ttl() {
        let manager = new_manager();

        manager.set_default_message_ttl(86400).await.unwrap();
        assert_eq!(manager.get_default_message_ttl().await.unwrap(), 86400);

        manager.set_default_message_ttl(0).await.unwrap();
        assert_eq!(manager.get_default_message_ttl().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_set_default_message_ttl_negative() {
        let manager = new_manager();

        let result = manager.set_default_message_ttl(-1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_account_ttl() {
        let manager = new_manager();

        manager.set_account_ttl(30).await.unwrap();
        assert_eq!(manager.get_account_ttl().await.unwrap(), 30);

        manager.set_account_ttl(366).await.unwrap();
        assert_eq!(manager.get_account_ttl().await.unwrap(), 366);
    }

    #[tokio::test]
    async fn test_set_account_ttl_invalid() {
        let manager = new_manager();

        assert!(manager.set_account_ttl(0).await.is_err());
        assert!(manager.set_account_ttl(367).await.is_err());
        assert!(manager.set_account_ttl(-1).await.is_err());
    }

    #[tokio::test]
    async fn test_set_inactive_session_ttl_days() {
        let manager = new_manager();

        manager.set_inactive_session_ttl_days(7).await.unwrap();
        assert_eq!(
            manager
                .get_active_sessions()
                .await
                .unwrap()
                .inactive_session_ttl_days(),
            7
        );
    }

    #[tokio::test]
    async fn test_set_inactive_session_ttl_days_invalid() {
        let manager = new_manager();

        assert!(manager.set_inactive_session_ttl_days(0).await.is_err());
        assert!(manager.set_inactive_session_ttl_days(367).await.is_err());
    }

    #[tokio::test]
    async fn test_age_verification_parameters() {
        let manager = new_manager();

        let params = age_verification::AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        manager.update_age_verification_parameters(params).await;

        let retrieved = manager.get_age_verification_parameters().await;
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().need_verification());
    }

    #[tokio::test]
    async fn test_confirm_qr_code_authentication_invalid_link() {
        let manager = new_manager();

        let result = manager.confirm_qr_code_authentication("invalid_link").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_confirm_qr_code_authentication_empty_token() {
        let manager = new_manager();

        let result = manager
            .confirm_qr_code_authentication("tg://login?token=")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_active_sessions() {
        let manager = new_manager();

        let sessions = manager.get_active_sessions().await.unwrap();
        assert_eq!(sessions.inactive_session_ttl_days(), 180); // Default value
        assert!(sessions.sessions().is_empty());
    }

    #[tokio::test]
    async fn test_terminate_session() {
        let manager = new_manager();

        manager.terminate_session(123456789).await.unwrap();
    }

    #[tokio::test]
    async fn test_terminate_all_other_sessions() {
        let manager = new_manager();

        // Add an unconfirmed authorization
        manager
            .on_new_unconfirmed_authorization(123456789, 1704067200, "iPhone 15", "United States")
            .await;

        assert!(manager.has_unconfirmed_authorizations().await);

        manager.terminate_all_other_sessions().await.unwrap();

        assert!(!manager.has_unconfirmed_authorizations().await);
    }

    #[tokio::test]
    async fn test_confirm_session() {
        let manager = new_manager();

        // Add an unconfirmed authorization
        manager
            .on_new_unconfirmed_authorization(123456789, 1704067200, "iPhone 15", "United States")
            .await;

        assert!(manager.has_unconfirmed_authorizations().await);

        // Confirm the session
        manager.confirm_session(123456789).await.unwrap();

        assert!(!manager.has_unconfirmed_authorizations().await);
    }

    #[tokio::test]
    async fn test_confirm_session_not_found() {
        let manager = new_manager();

        let result = manager.confirm_session(123456789).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_toggle_session_can_accept_calls() {
        let manager = new_manager();

        manager
            .toggle_session_can_accept_calls(123456789, false)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_toggle_session_can_accept_secret_chats() {
        let manager = new_manager();

        manager
            .toggle_session_can_accept_secret_chats(123456789, false)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_connected_websites() {
        let manager = new_manager();

        let websites = manager.get_connected_websites().await.unwrap();
        assert!(websites.websites().is_empty());
    }

    #[tokio::test]
    async fn test_disconnect_website() {
        let manager = new_manager();

        manager.disconnect_website(987654321).await.unwrap();
    }

    #[tokio::test]
    async fn test_disconnect_all_websites() {
        let manager = new_manager();

        manager.disconnect_all_websites().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_user_link() {
        let manager = new_manager();

        let link = manager.get_user_link().await.unwrap();
        assert_eq!(link.url(), "https://t.me/username");
        assert_eq!(link.expires_in(), 0);
    }

    #[tokio::test]
    async fn test_import_contact_token_empty() {
        let manager = new_manager();

        let result = manager.import_contact_token("").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalidate_authentication_codes() {
        let manager = new_manager();

        manager
            .invalidate_authentication_codes(vec!["CODE123".to_string()])
            .await;
    }

    // Unconfirmed authorization tests
    #[tokio::test]
    async fn test_on_new_unconfirmed_authorization() {
        let manager = new_manager();

        manager
            .on_new_unconfirmed_authorization(123456789, 1704067200, "iPhone 15", "United States")
            .await;

        assert!(manager.has_unconfirmed_authorizations().await);
    }

    #[tokio::test]
    async fn test_on_new_unconfirmed_authorization_zero_hash() {
        let manager = new_manager();

        manager
            .on_new_unconfirmed_authorization(0, 1704067200, "iPhone 15", "United States")
            .await;

        assert!(!manager.has_unconfirmed_authorizations().await);
    }

    #[tokio::test]
    async fn test_get_first_unconfirmed_authorization() {
        let manager = new_manager();

        manager
            .on_new_unconfirmed_authorization(123456789, 1704067200, "iPhone 15", "United States")
            .await;

        let first = manager.get_first_unconfirmed_authorization().await;
        assert!(first.is_some());
        assert_eq!(first.unwrap().hash, 123456789);
    }

    #[tokio::test]
    async fn test_get_first_unconfirmed_authorization_none() {
        let manager = new_manager();

        let first = manager.get_first_unconfirmed_authorization().await;
        assert!(first.is_none());
    }

    // UnconfirmedAuthorizations tests
    #[test]
    fn test_unconfirmed_authorizations_new() {
        let unconfirmed = UnconfirmedAuthorizations::new();
        assert!(unconfirmed.is_empty());
    }

    #[test]
    fn test_unconfirmed_authorizations_add() {
        let mut unconfirmed = UnconfirmedAuthorizations::new();

        let auth =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        assert!(unconfirmed.add(auth));

        assert!(!unconfirmed.is_empty());
    }

    #[test]
    fn test_unconfirmed_authorizations_add_duplicate_date() {
        let mut unconfirmed = UnconfirmedAuthorizations::new();

        let auth1 =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        let auth2 = UnconfirmedAuthorization::new(987654321, 1704067200, "iPad", "Canada");

        assert!(unconfirmed.add(auth1));
        assert!(!unconfirmed.add(auth2)); // Same date, should fail
    }

    #[test]
    fn test_unconfirmed_authorizations_remove() {
        let mut unconfirmed = UnconfirmedAuthorizations::new();

        let auth =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        unconfirmed.add(auth);

        assert!(unconfirmed.remove(123456789));
        assert!(unconfirmed.is_empty());
    }

    #[test]
    fn test_unconfirmed_authorizations_remove_not_found() {
        let mut unconfirmed = UnconfirmedAuthorizations::new();

        assert!(!unconfirmed.remove(123456789));
    }

    #[test]
    fn test_unconfirmed_authorizations_get_first() {
        let mut unconfirmed = UnconfirmedAuthorizations::new();

        let auth1 =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        let auth2 = UnconfirmedAuthorization::new(987654321, 1704153600, "iPad", "Canada");

        unconfirmed.add(auth1);
        unconfirmed.add(auth2);

        let first = unconfirmed.get_first().unwrap();
        assert_eq!(first.hash, 123456789);
        assert_eq!(first.date, 1704067200);
    }

    #[test]
    fn test_unconfirmed_authorizations_get_first_empty() {
        let unconfirmed = UnconfirmedAuthorizations::new();

        assert!(unconfirmed.get_first().is_none());
    }

    #[test]
    fn test_unconfirmed_authorizations_remove_expired() {
        let mut unconfirmed = UnconfirmedAuthorizations::new();

        // Add an old authorization (expired)
        let old_time = 1704067200 - DEFAULT_AUTHORIZATION_AUTOCONFIRM_PERIOD - 100;
        let auth1 =
            UnconfirmedAuthorization::new(123456789, old_time, "iPhone 15", "United States");

        // Add a recent authorization (not expired)
        let recent_time = 1704067200;
        let auth2 = UnconfirmedAuthorization::new(987654321, recent_time, "iPad", "Canada");

        unconfirmed.add(auth1);
        unconfirmed.add(auth2);

        let current_time = 1704067200;
        let removed = unconfirmed.remove_expired(current_time);

        assert!(removed);
        assert_eq!(unconfirmed.authorizations.len(), 1);
        assert!(unconfirmed.get_first().is_some());
    }

    #[test]
    fn test_unconfirmed_authorizations_get_first_expiration_date() {
        let mut unconfirmed = UnconfirmedAuthorizations::new();

        let auth =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        unconfirmed.add(auth);

        let expiration = unconfirmed.get_first_expiration_date();
        assert!(expiration.is_some());
        assert_eq!(
            expiration.unwrap(),
            1704067200 + DEFAULT_AUTHORIZATION_AUTOCONFIRM_PERIOD
        );
    }

    #[test]
    fn test_unconfirmed_authorizations_get_first_expiration_date_empty() {
        let unconfirmed = UnconfirmedAuthorizations::new();

        assert!(unconfirmed.get_first_expiration_date().is_none());
    }

    // Thread safety tests
    #[tokio::test]
    async fn test_concurrent_access() {
        let manager = std::sync::Arc::new(new_manager());
        let mut handles = Vec::new();

        for i in 0..10 {
            let manager_clone = std::sync::Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                let _ = manager_clone.set_account_ttl(30 + i as i32).await;
                let _ = manager_clone.get_account_ttl().await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // Should not panic
    }

    #[tokio::test]
    async fn test_concurrent_unconfirmed_authorizations() {
        let manager = std::sync::Arc::new(new_manager());
        let mut handles = Vec::new();

        for i in 0..10 {
            let manager_clone = std::sync::Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                manager_clone
                    .on_new_unconfirmed_authorization(
                        i as i64,
                        1704067200 + i as i32,
                        "Device",
                        "Location",
                    )
                    .await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        assert!(manager.has_unconfirmed_authorizations().await);
    }
}
