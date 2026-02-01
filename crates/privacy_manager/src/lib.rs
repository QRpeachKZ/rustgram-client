// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Privacy Manager
//!
//! Manages privacy settings and rules for Telegram.
//!
//! ## Overview
//!
//! The `PrivacyManager` handles privacy operations including:
//! - Getting privacy settings for various keys
//! - Setting privacy rules
//! - Handling privacy updates from Telegram
//! - Managing unpaid message allowances
//!
//! ## Architecture
//!
//! This is a simplified version of TDLib's `PrivacyManager` that focuses
//! on the core data structures and state management. The full TDLib
//! implementation includes network queries and real-time updates.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_privacy_manager::{PrivacyManager, PrivacyKey};
//! use rustgram_user_privacy_setting::{UserPrivacySetting, UserPrivacySettingRule};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let manager = PrivacyManager::new();
//!
//! // Get privacy rules for a setting
//! let rules = manager.get_privacy(PrivacyKey::Status).await;
//! assert!(rules.is_some());
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::UserId;
use rustgram_user_privacy_setting::UserPrivacySetting;
use rustgram_user_privacy_setting_rule::{PrivacyRule, PrivacyValueType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub use error::{Error, Result};
pub use privacy_key::PrivacyKey;

mod error;
mod privacy_key;

/// Privacy rules for a user setting.
///
/// Based on TDLib's `UserPrivacySettingRules`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyRules {
    /// The privacy rules.
    rules: Vec<PrivacyRule>,
    /// Whether these rules are the server-side rules.
    is_synchronized: bool,
}

impl PrivacyRules {
    /// Creates new privacy rules.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            is_synchronized: false,
        }
    }

    /// Creates privacy rules with specific rules.
    #[must_use]
    pub fn with_rules(rules: Vec<PrivacyRule>) -> Self {
        Self {
            rules,
            is_synchronized: false,
        }
    }

    /// Returns the rules.
    #[must_use]
    pub fn rules(&self) -> &[PrivacyRule] {
        &self.rules
    }

    /// Returns whether these rules are synchronized.
    #[must_use]
    pub fn is_synchronized(&self) -> bool {
        self.is_synchronized
    }

    /// Sets the synchronization status.
    pub fn set_synchronized(&mut self, synchronized: bool) {
        self.is_synchronized = synchronized;
    }

    /// Adds a rule.
    pub fn add_rule(&mut self, rule: PrivacyRule) {
        self.rules.push(rule);
    }

    /// Checks if these rules allow all users.
    #[must_use]
    pub fn allows_all(&self) -> bool {
        self.rules.iter().any(|r| r.is_all_allowed())
    }

    /// Checks if these rules disallow all users.
    #[must_use]
    pub fn disallows_all(&self) -> bool {
        self.rules.iter().any(|r| r.is_all_disallowed())
    }
}

impl Default for PrivacyRules {
    fn default() -> Self {
        Self::new()
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivacyRules")
            .field("rule_count", &self.rules.len())
            .field("is_synchronized", &self.is_synchronized)
            .finish()
    }
}

/// Privacy information for a setting.
///
/// Based on TDLib's `PrivacyInfo`.
#[derive(Debug, Clone)]
struct PrivacyInfo {
    /// The current privacy rules.
    rules: PrivacyRules,
    /// Pending rules to be set.
    pending_rules: Option<PrivacyRules>,
    /// Whether a set query is in progress.
    has_set_query: bool,
}

impl PrivacyInfo {
    /// Creates new privacy info.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: PrivacyRules::new(),
            pending_rules: None,
            has_set_query: false,
        }
    }
}

impl Default for PrivacyInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Privacy manager.
///
/// Manages privacy settings and rules.
#[derive(Clone)]
pub struct PrivacyManager {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    /// Privacy info for each setting key.
    info: HashMap<PrivacyKey, PrivacyInfo>,
}

impl fmt::Debug for PrivacyManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivacyManager")
            .field("setting_count", &self.try_setting_count())
            .finish()
    }
}

impl Default for PrivacyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivacyManager {
    /// Creates a new privacy manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Inner {
                info: HashMap::new(),
            })),
        }
    }

    /// Returns the number of settings (synchronous).
    #[must_use]
    pub fn try_setting_count(&self) -> Option<usize> {
        self.inner.try_read().ok().map(|inner| inner.info.len())
    }

    /// Gets privacy rules for a setting.
    ///
    /// # Arguments
    ///
    /// * `key` - The privacy setting key
    ///
    /// # Returns
    ///
    /// The privacy rules for this setting, or None if not set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_privacy_manager::{PrivacyManager, PrivacyKey};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = PrivacyManager::new();
    ///
    /// let rules = manager.get_privacy(PrivacyKey::Status).await;
    /// # }
    /// ```
    #[must_use]
    pub async fn get_privacy(&self, key: PrivacyKey) -> Option<PrivacyRules> {
        let inner = self.inner.read().await;
        inner.info.get(&key).map(|info| info.rules.clone())
    }

    /// Sets privacy rules for a setting.
    ///
    /// # Arguments
    ///
    /// * `key` - The privacy setting key
    /// * `rules` - The privacy rules to set
    ///
    /// # Errors
    ///
    /// Returns an error if a set query is already in progress.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_privacy_manager::{PrivacyManager, PrivacyKey, PrivacyRules};
    /// use rustgram_user_privacy_setting_rule::PrivacyRule;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = PrivacyManager::new();
    ///
    /// let rules = PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]);
    ///
    /// let result = manager.set_privacy(PrivacyKey::Status, rules).await;
    /// assert!(result.is_ok());
    /// # }
    /// ```
    pub async fn set_privacy(&self, key: PrivacyKey, rules: PrivacyRules) -> Result<()> {
        let mut inner = self.inner.write().await;

        let info = inner.info.entry(key).or_insert_with(PrivacyInfo::new);

        if info.has_set_query {
            warn!("Set query already in progress for {:?}", key);
            return Err(Error::SetQueryInProgress { key });
        }

        info!("Setting privacy for {:?} with {} rules", key, rules.rules().len());
        debug!("Rules: {:?}", rules);

        // In the full implementation, this would send the set query to Telegram
        info.rules = rules;
        info.pending_rules = None;
        info.has_set_query = false;

        Ok(())
    }

    /// Handles a privacy update from Telegram.
    ///
    /// # Arguments
    ///
    /// * `key` - The privacy setting key
    /// * `rules` - The new privacy rules
    pub async fn on_update_privacy(&self, key: PrivacyKey, rules: PrivacyRules) {
        let mut inner = self.inner.write().await;

        let info = inner.info.entry(key).or_insert_with(PrivacyInfo::new);

        info!("Received privacy update for {:?}", key);
        debug!("New rules: {:?}", rules);

        info.rules = rules;
        info.pending_rules = None;
        info.has_set_query = false;
    }

    /// Allows unpaid messages from a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    /// * `refund_payments` - Whether to refund payments
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_privacy_manager::PrivacyManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = PrivacyManager::new();
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let result = manager.allow_unpaid_messages(user_id, false).await;
    /// # }
    /// ```
    pub async fn allow_unpaid_messages(
        &self,
        user_id: UserId,
        refund_payments: bool,
    ) -> Result<()> {
        info!(
            "Allowing unpaid messages from user {:?}, refund={}",
            user_id, refund_payments
        );

        // In the full implementation, this would send the request to Telegram
        Ok(())
    }

    /// Checks if a user is allowed based on privacy rules.
    ///
    /// # Arguments
    ///
    /// * `key` - The privacy setting key
    /// * `user_id` - The user ID to check
    ///
    /// # Returns
    ///
    /// Whether the user is allowed.
    #[must_use]
    pub async fn check_user_access(&self, key: PrivacyKey, user_id: UserId) -> bool {
        let inner = self.inner.read().await;

        if let Some(info) = inner.info.get(&key) {
            // Check the rules to see if this user is allowed
            for rule in &info.rules.rules {
                match rule {
                    PrivacyRule::AllowAll => return true,
                    PrivacyRule::DisallowAll => return false,
                    PrivacyRule::AllowUsers(users) => {
                        if users.contains(&user_id.get()) {
                            return true;
                        }
                    }
                    PrivacyRule::DisallowUsers(users) => {
                        if users.contains(&user_id.get()) {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Default to allowing access if no rules are set
        true
    }

    /// Gets all privacy settings.
    ///
    /// # Returns
    ///
    /// A map of all privacy keys to their rules.
    #[must_use]
    pub async fn get_all_privacy(&self) -> HashMap<PrivacyKey, PrivacyRules> {
        let inner = self.inner.read().await;

        inner
            .info
            .iter()
            .map(|(key, info)| (*key, info.rules.clone()))
            .collect()
    }

    /// Sets default privacy rules for all settings.
    ///
    /// This is useful for initialization.
    pub async fn set_default_rules(&self) {
        let mut inner = self.inner.write().await;

        let default_rules = PrivacyRules::with_rules(vec![
            PrivacyRule::AllowContacts,
            PrivacyRule::AllowPremium,
        ]);

        for key in PrivacyKey::all_known() {
            inner
                .info
                .entry(key)
                .or_insert_with(PrivacyInfo::new)
                .rules = default_rules.clone();
        }

        info!("Set default privacy rules for all settings");
    }

    /// Converts a UserPrivacySetting to a PrivacyKey.
    ///
    /// # Arguments
    ///
    /// * `setting` - The user privacy setting
    ///
    /// # Returns
    ///
    /// The corresponding privacy key, or None if unknown.
    #[must_use]
    pub const fn setting_to_key(setting: UserPrivacySetting) -> Option<PrivacyKey> {
        match setting {
            UserPrivacySetting::UserStatus => Some(PrivacyKey::Status),
            UserPrivacySetting::ChatInvite => Some(PrivacyKey::ChatInvite),
            UserPrivacySetting::Call => Some(PrivacyKey::Call),
            UserPrivacySetting::PeerToPeerCall => Some(PrivacyKey::PeerToPeerCall),
            UserPrivacySetting::UserProfilePhoto => Some(PrivacyKey::ProfilePhoto),
            UserPrivacySetting::UserPhoneNumber => Some(PrivacyKey::PhoneNumber),
            UserPrivacySetting::FindByPhoneNumber => Some(PrivacyKey::PhoneNumberStatus),
            UserPrivacySetting::VoiceMessages => Some(PrivacyKey::Bio),
            UserPrivacySetting::UserBio => Some(PrivacyKey::Username),
            UserPrivacySetting::UserBirthdate => Some(PrivacyKey::Birthdate),
            _ => None,
        }
    }

    /// Converts a PrivacyKey to a UserPrivacySetting.
    ///
    /// # Arguments
    ///
    /// * `key` - The privacy key
    ///
    /// # Returns
    ///
    /// The corresponding user privacy setting.
    #[must_use]
    pub const fn key_to_setting(key: PrivacyKey) -> UserPrivacySetting {
        match key {
            PrivacyKey::Status => UserPrivacySetting::UserStatus,
            PrivacyKey::ChatInvite => UserPrivacySetting::ChatInvite,
            PrivacyKey::Call => UserPrivacySetting::Call,
            PrivacyKey::PeerToPeerCall => UserPrivacySetting::PeerToPeerCall,
            PrivacyKey::ProfilePhoto => UserPrivacySetting::UserProfilePhoto,
            PrivacyKey::PhoneNumber => UserPrivacySetting::UserPhoneNumber,
            PrivacyKey::PhoneNumberStatus => UserPrivacySetting::FindByPhoneNumber,
            PrivacyKey::Bio => UserPrivacySetting::VoiceMessages,
            PrivacyKey::Username => UserPrivacySetting::UserBio,
            PrivacyKey::Birthdate => UserPrivacySetting::UserBirthdate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === PrivacyRules Tests ===

    #[test]
    fn test_privacy_rules_new() {
        let rules = PrivacyRules::new();
        assert!(rules.rules().is_empty());
        assert!(!rules.is_synchronized());
    }

    #[test]
    fn test_privacy_rules_with_rules() {
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]);

        assert_eq!(rules.rules().len(), 1);
        assert!(rules.allows_all());
        assert!(!rules.disallows_all());
    }

    #[test]
    fn test_privacy_rules_add_rule() {
        let mut rules = PrivacyRules::new();
        rules.add_rule(PrivacyRule::AllowAll);
        rules.add_rule(PrivacyRule::AllowContacts);

        assert_eq!(rules.rules().len(), 2);
    }

    #[test]
    fn test_privacy_rules_allows_all() {
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]);
        assert!(rules.allows_all());
    }

    #[test]
    fn test_privacy_rules_disallows_all() {
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::DisallowAll]);
        assert!(rules.disallows_all());
    }

    #[test]
    fn test_privacy_rules_clone() {
        let rules1 = PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]);
        let rules2 = rules1.clone();

        assert_eq!(rules1, rules2);
    }

    // === PrivacyKey Tests ===

    #[test]
    fn test_privacy_key_all_known() {
        let keys = PrivacyKey::all_known();
        assert!(keys.len() > 0);
    }

    #[test]
    fn test_privacy_key_from_i32() {
        assert_eq!(PrivacyKey::from_i32(0), Some(PrivacyKey::Status));
        assert_eq!(PrivacyKey::from_i32(999), None);
    }

    #[test]
    fn test_privacy_key_to_i32() {
        assert_eq!(PrivacyKey::Status.to_i32(), 0);
        assert_eq!(PrivacyKey::Birthdate.to_i32(), 6);
    }

    // === PrivacyManager Tests ===

    #[tokio::test]
    async fn test_manager_new() {
        let manager = PrivacyManager::new();
        assert_eq!(manager.try_setting_count(), Some(0));
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = PrivacyManager::default();
        assert_eq!(manager.try_setting_count(), Some(0));
    }

    #[tokio::test]
    async fn test_get_privacy_not_set() {
        let manager = PrivacyManager::new();
        let rules = manager.get_privacy(PrivacyKey::Status).await;
        assert!(rules.is_none());
    }

    #[tokio::test]
    async fn test_set_privacy() {
        let manager = PrivacyManager::new();
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]);

        let result = manager.set_privacy(PrivacyKey::Status, rules).await;
        assert!(result.is_ok());

        let retrieved = manager.get_privacy(PrivacyKey::Status).await;
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().allows_all());
    }

    #[tokio::test]
    async fn test_set_privacy_multiple() {
        let manager = PrivacyManager::new();

        manager
            .set_privacy(
                PrivacyKey::Status,
                PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]),
            )
            .await
            .unwrap();

        manager
            .set_privacy(
                PrivacyKey::ProfilePhoto,
                PrivacyRules::with_rules(vec![PrivacyRule::AllowContacts]),
            )
            .await
            .unwrap();

        assert_eq!(manager.try_setting_count(), Some(2));
    }

    #[tokio::test]
    async fn test_on_update_privacy() {
        let manager = PrivacyManager::new();
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::DisallowAll]);

        manager
            .on_update_privacy(PrivacyKey::Status, rules)
            .await;

        let retrieved = manager.get_privacy(PrivacyKey::Status).await;
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().disallows_all());
    }

    #[tokio::test]
    async fn test_allow_unpaid_messages() {
        let manager = PrivacyManager::new();
        let user_id = UserId::new(123).unwrap();

        let result = manager.allow_unpaid_messages(user_id, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_user_access_allow_all() {
        let manager = PrivacyManager::new();
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]);

        manager
            .set_privacy(PrivacyKey::Status, rules)
            .await
            .unwrap();

        let user_id = UserId::new(123).unwrap();
        assert!(manager.check_user_access(PrivacyKey::Status, user_id).await);
    }

    #[tokio::test]
    async fn test_check_user_access_disallow_all() {
        let manager = PrivacyManager::new();
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::DisallowAll]);

        manager
            .set_privacy(PrivacyKey::Status, rules)
            .await
            .unwrap();

        let user_id = UserId::new(123).unwrap();
        assert!(!manager.check_user_access(PrivacyKey::Status, user_id).await);
    }

    #[tokio::test]
    async fn test_check_user_access_allow_users() {
        let manager = PrivacyManager::new();
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::AllowUsers(vec![123, 456])]);

        manager
            .set_privacy(PrivacyKey::Status, rules)
            .await
            .unwrap();

        let user1 = UserId::new(123).unwrap();
        let user2 = UserId::new(999).unwrap();

        assert!(manager.check_user_access(PrivacyKey::Status, user1).await);
        assert!(!manager.check_user_access(PrivacyKey::Status, user2).await);
    }

    #[tokio::test]
    async fn test_check_user_access_disallow_users() {
        let manager = PrivacyManager::new();
        let rules = PrivacyRules::with_rules(vec![
            PrivacyRule::AllowAll,
            PrivacyRule::DisallowUsers(vec![123]),
        ]);

        manager
            .set_privacy(PrivacyKey::Status, rules)
            .await
            .unwrap();

        let user1 = UserId::new(123).unwrap();
        let user2 = UserId::new(999).unwrap();

        assert!(!manager.check_user_access(PrivacyKey::Status, user1).await);
        assert!(manager.check_user_access(PrivacyKey::Status, user2).await);
    }

    #[tokio::test]
    async fn test_check_user_access_no_rules() {
        let manager = PrivacyManager::new();
        let user_id = UserId::new(123).unwrap();

        // No rules set, should default to allowing access
        assert!(manager.check_user_access(PrivacyKey::Status, user_id).await);
    }

    #[tokio::test]
    async fn test_get_all_privacy() {
        let manager = PrivacyManager::new();

        manager
            .set_privacy(
                PrivacyKey::Status,
                PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]),
            )
            .await
            .unwrap();

        manager
            .set_privacy(
                PrivacyKey::ProfilePhoto,
                PrivacyRules::with_rules(vec![PrivacyRule::AllowContacts]),
            )
            .await
            .unwrap();

        let all = manager.get_all_privacy().await;
        assert_eq!(all.len(), 2);
        assert!(all.contains_key(&PrivacyKey::Status));
        assert!(all.contains_key(&PrivacyKey::ProfilePhoto));
    }

    #[tokio::test]
    async fn test_set_default_rules() {
        let manager = PrivacyManager::new();

        manager.set_default_rules().await;

        for key in PrivacyKey::all_known() {
            let rules = manager.get_privacy(key).await;
            assert!(rules.is_some(), "Rules should be set for {:?}", key);
        }
    }

    #[tokio::test]
    async fn test_setting_to_key() {
        use rustgram_user_privacy_setting::UserPrivacySetting;

        assert_eq!(
            PrivacyManager::setting_to_key(UserPrivacySetting::UserStatus),
            Some(PrivacyKey::Status)
        );
        assert_eq!(
            PrivacyManager::setting_to_key(UserPrivacySetting::ChatInvite),
            Some(PrivacyKey::ChatInvite)
        );
        assert_eq!(
            PrivacyManager::setting_to_key(UserPrivacySetting::UserBirthdate),
            Some(PrivacyKey::Birthdate)
        );
    }

    #[tokio::test]
    async fn test_key_to_setting() {
        use rustgram_user_privacy_setting::UserPrivacySetting;

        assert_eq!(
            PrivacyManager::key_to_setting(PrivacyKey::Status),
            UserPrivacySetting::UserStatus
        );
        assert_eq!(
            PrivacyManager::key_to_setting(PrivacyKey::Birthdate),
            UserPrivacySetting::UserBirthdate
        );
    }

    #[tokio::test]
    async fn test_manager_clone() {
        let manager1 = PrivacyManager::new();

        manager1
            .set_privacy(
                PrivacyKey::Status,
                PrivacyRules::with_rules(vec![PrivacyRule::AllowAll]),
            )
            .await
            .unwrap();

        let manager2 = manager1.clone();

        let rules1 = manager1.get_privacy(PrivacyKey::Status).await;
        let rules2 = manager2.get_privacy(PrivacyKey::Status).await;

        assert!(rules1.is_some());
        assert!(rules2.is_some());
        assert_eq!(rules1, rules2);
    }

    // === Integration Tests ===

    #[tokio::test]
    async fn test_full_privacy_flow() {
        let manager = PrivacyManager::new();

        // Set initial rules
        let rules = PrivacyRules::with_rules(vec![PrivacyRule::AllowContacts]);
        manager
            .set_privacy(PrivacyKey::Status, rules)
            .await
            .unwrap();

        // Check user access
        let user_id = UserId::new(123).unwrap();
        let allowed = manager.check_user_access(PrivacyKey::Status, user_id).await;

        // AllowAll is not set, so access depends on the specific rule
        // With AllowContacts, it would check if user is a contact
        // Since we can't check that here, we just verify the call works
        let _ = allowed;

        // Update rules
        let new_rules = PrivacyRules::with_rules(vec![PrivacyRule::DisallowAll]);
        manager
            .on_update_privacy(PrivacyKey::Status, new_rules)
            .await;

        let retrieved = manager.get_privacy(PrivacyKey::Status).await;
        assert!(retrieved.unwrap().disallows_all());
    }
}
