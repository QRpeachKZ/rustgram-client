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

//! # Referral Program Manager
//!
//! Manages referral/affiliate programs for Telegram MTProto client.
//!
//! ## Overview
//!
//! The ReferralProgramManager handles:
//! - Suggested referral programs
//! - Connected referral programs
//! - Program search and discovery
//! - Program connection and revocation
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's `ReferralProgramManager` from `td/telegram/ReferralProgramManager.h`.
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_referral_program_manager::{ReferralProgramManager, AffiliateType};
//! use rustgram_types::UserId;
//! use rustgram_referral_program_parameters::ReferralProgramParameters;
//! use rustgram_types::DialogId;
//!
//! let mut manager = ReferralProgramManager::new();
//!
//! // Set a referral program for a dialog
//! let params = ReferralProgramParameters::with_params(100, 12);
//! let dialog_id = DialogId::from_user(UserId::new(123456).unwrap());
//! assert!(manager.set_dialog_referral_program(dialog_id, params).is_ok());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::len_without_is_empty)]

mod error;
mod tl;

use rustgram_referral_program_info::ReferralProgramInfo;
use rustgram_referral_program_parameters::ReferralProgramParameters;
use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
use rustgram_types::{DialogId, UserId};
use std::collections::HashMap;

pub use error::{Error, Result};
pub use tl::{AffiliateType, Chat, ConnectedProgram, FoundProgram};

/// Suggested bot referral program.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `ReferralProgramManager::SuggestedBotStarRef`.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::SuggestedBotStarRef;
/// use rustgram_types::UserId;
/// use rustgram_referral_program_info::ReferralProgramInfo;
/// use rustgram_referral_program_parameters::ReferralProgramParameters;
/// use rustgram_star_amount::StarAmount;
///
/// let user_id = UserId::new(123456).unwrap();
/// let params = ReferralProgramParameters::with_params(100, 12);
/// let daily_amount = StarAmount::from_parts(50, 0).unwrap();
/// let info = ReferralProgramInfo::with_params(params, 0, daily_amount);
///
/// let program = SuggestedBotStarRef::new(user_id, info);
/// assert!(program.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuggestedBotStarRef {
    /// Bot user ID.
    user_id: UserId,
    /// Referral program information.
    info: ReferralProgramInfo,
}

impl SuggestedBotStarRef {
    /// Creates a new suggested bot referral program.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Bot user ID
    /// * `info` - Referral program information
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::SuggestedBotStarRef;
    /// use rustgram_types::UserId;
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    ///
    /// let user_id = UserId::new(123456).unwrap();
    /// let info = ReferralProgramInfo::default();
    /// let program = SuggestedBotStarRef::new(user_id, info);
    /// ```
    #[must_use]
    pub const fn new(user_id: UserId, info: ReferralProgramInfo) -> Self {
        Self { user_id, info }
    }

    /// Returns the bot user ID.
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns the referral program info.
    #[must_use]
    pub const fn info(&self) -> &ReferralProgramInfo {
        &self.info
    }

    /// Checks if this is a valid referral program.
    ///
    /// A valid program has a valid user ID and valid program info.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.user_id.is_valid() && self.info.is_valid()
    }

    /// Checks if this program is currently active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.info.is_active()
    }
}

/// Connected bot referral program.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `ReferralProgramManager::ConnectedBotStarRef`.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::ConnectedBotStarRef;
/// use rustgram_types::UserId;
/// use rustgram_referral_program_parameters::ReferralProgramParameters;
///
/// let user_id = UserId::new(123456).unwrap();
/// let params = ReferralProgramParameters::with_params(100, 12);
///
/// let program = ConnectedBotStarRef::new(
///     "https://t.me/bot?start=ref123".to_string(),
///     1234567890,
///     user_id,
///     params,
///     100,
///     1000,
/// );
/// assert!(program.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectedBotStarRef {
    /// Referral URL.
    url: String,
    /// Connection date as Unix timestamp.
    date: i32,
    /// Bot user ID.
    user_id: UserId,
    /// Program parameters.
    parameters: ReferralProgramParameters,
    /// Number of participants.
    participant_count: i64,
    /// Revenue in star count.
    revenue_star_count: i64,
    /// Whether the program is revoked.
    is_revoked: bool,
}

impl ConnectedBotStarRef {
    /// Creates a new connected bot referral program.
    ///
    /// # Arguments
    ///
    /// * `url` - Referral URL
    /// * `date` - Connection date as Unix timestamp
    /// * `user_id` - Bot user ID
    /// * `parameters` - Program parameters
    /// * `participant_count` - Number of participants
    /// * `revenue_star_count` - Revenue in stars
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::ConnectedBotStarRef;
    /// use rustgram_types::UserId;
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let user_id = UserId::new(123456).unwrap();
    /// let params = ReferralProgramParameters::with_params(100, 12);
    ///
    /// let program = ConnectedBotStarRef::new(
    ///     "https://t.me/bot?start=ref123".to_string(),
    ///     1234567890,
    ///     user_id,
    ///     params,
    ///     100,
    ///     1000,
    /// );
    /// ```
    #[must_use]
    pub fn new(
        url: String,
        date: i32,
        user_id: UserId,
        parameters: ReferralProgramParameters,
        participant_count: i64,
        revenue_star_count: i64,
    ) -> Self {
        Self {
            url,
            date,
            user_id,
            parameters,
            participant_count,
            revenue_star_count,
            is_revoked: false,
        }
    }

    /// Returns the referral URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the connection date.
    #[must_use]
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns the bot user ID.
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns the program parameters.
    #[must_use]
    pub const fn parameters(&self) -> &ReferralProgramParameters {
        &self.parameters
    }

    /// Returns the participant count.
    #[must_use]
    pub const fn participant_count(&self) -> i64 {
        self.participant_count
    }

    /// Returns the revenue star count.
    #[must_use]
    pub const fn revenue_star_count(&self) -> i64 {
        self.revenue_star_count
    }

    /// Checks if this program is revoked.
    #[must_use]
    pub const fn is_revoked(&self) -> bool {
        self.is_revoked
    }

    /// Marks this program as revoked.
    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }

    /// Checks if this is a valid referral program.
    ///
    /// A valid program has a non-empty URL, positive date, valid user ID,
    /// valid parameters, and non-negative counts.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.url.is_empty()
            && self.date > 0
            && self.user_id.is_valid()
            && self.parameters.is_valid()
            && self.participant_count >= 0
            && self.revenue_star_count >= 0
    }
}

/// Referral program manager.
///
/// Manages referral/affiliate programs for Telegram.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `ReferralProgramManager` from `td/telegram/ReferralProgramManager.h`.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::ReferralProgramManager;
/// use rustgram_types::{UserId, DialogId};
/// use rustgram_referral_program_parameters::ReferralProgramParameters;
///
/// let mut manager = ReferralProgramManager::new();
///
/// // Set a referral program for a dialog
/// let params = ReferralProgramParameters::with_params(100, 12);
/// let dialog_id = DialogId::from_user(UserId::new(123456).unwrap());
/// assert!(manager.set_dialog_referral_program(dialog_id, params).is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct ReferralProgramManager {
    /// Suggested referral programs.
    suggested_programs: Vec<SuggestedBotStarRef>,
    /// Connected referral programs.
    connected_programs: Vec<ConnectedBotStarRef>,
    /// Programs indexed by user ID for quick lookup.
    user_programs: HashMap<UserId, ConnectedBotStarRef>,
}

impl ReferralProgramManager {
    /// Creates a new referral program manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::ReferralProgramManager;
    ///
    /// let manager = ReferralProgramManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            suggested_programs: Vec::new(),
            connected_programs: Vec::new(),
            user_programs: HashMap::new(),
        }
    }

    /// Sets a referral program for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `parameters` - Program parameters
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(Error)` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::ReferralProgramManager;
    /// use rustgram_types::{UserId, DialogId};
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let mut manager = ReferralProgramManager::new();
    /// let params = ReferralProgramParameters::with_params(100, 12);
    /// let dialog_id = DialogId::from_user(UserId::new(123456).unwrap());
    /// assert!(manager.set_dialog_referral_program(dialog_id, params).is_ok());
    /// ```
    pub fn set_dialog_referral_program(
        &mut self,
        _dialog_id: DialogId,
        parameters: ReferralProgramParameters,
    ) -> Result<()> {
        if !parameters.is_valid() {
            return Err(Error::InvalidParameters);
        }

        // In a real implementation, this would make an API call to set the program
        // For now, we just validate the parameters
        Ok(())
    }

    /// Searches for a dialog referral program.
    ///
    /// # Arguments
    ///
    /// * `username` - Username to search
    /// * `referral` - Referral code
    ///
    /// # Returns
    ///
    /// `Ok(chat)` if found, `Err(Error)` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::ReferralProgramManager;
    ///
    /// let manager = ReferralProgramManager::new();
    /// // This would normally make an API call
    /// let result = manager.search_dialog_referral_program("bot", "ref123");
    /// // Result would be Err(Error::InvalidUrl) in this stub implementation
    /// ```
    pub fn search_dialog_referral_program(&self, _username: &str, _referral: &str) -> Result<Chat> {
        // Stub implementation - in real code this would make an API call
        Err(Error::InvalidUrl)
    }

    /// Searches for referral programs.
    ///
    /// # Arguments
    ///
    /// * `affiliate` - Affiliate type
    /// * `sort_order` - Sort order
    /// * `offset` - Offset string for pagination
    /// * `limit` - Maximum results
    ///
    /// # Returns
    ///
    /// `Ok(programs)` if successful, `Err(Error)` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::{ReferralProgramManager, AffiliateType};
    /// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
    ///
    /// let manager = ReferralProgramManager::new();
    /// let result = manager.search_referral_programs(
    ///     AffiliateType::Bot,
    ///     ReferralProgramSortOrder::Profitability,
    ///     "",
    ///     10,
    /// );
    /// ```
    pub fn search_referral_programs(
        &self,
        _affiliate: AffiliateType,
        _sort_order: ReferralProgramSortOrder,
        _offset: &str,
        limit: i32,
    ) -> Result<Vec<FoundProgram>> {
        if limit <= 0 {
            return Ok(Vec::new());
        }

        // Filter suggested programs by affiliate type
        let programs: Vec<FoundProgram> = self
            .suggested_programs
            .iter()
            .filter(|p| p.is_valid())
            .take(limit as usize)
            .map(|p| FoundProgram {
                user_id: p.user_id(),
                info: p.info().clone(),
            })
            .collect();

        Ok(programs)
    }

    /// Connects to a referral program.
    ///
    /// # Arguments
    ///
    /// * `affiliate` - Affiliate type
    /// * `bot_user_id` - Bot user ID
    ///
    /// # Returns
    ///
    /// `Ok(program)` if successful, `Err(Error)` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::{ReferralProgramManager, AffiliateType};
    /// use rustgram_types::UserId;
    ///
    /// let mut manager = ReferralProgramManager::new();
    /// let bot_user_id = UserId::new(123456).unwrap();
    /// let result = manager.connect_referral_program(AffiliateType::Bot, bot_user_id);
    /// ```
    pub fn connect_referral_program(
        &mut self,
        _affiliate: AffiliateType,
        bot_user_id: UserId,
    ) -> Result<ConnectedProgram> {
        if !bot_user_id.is_valid() {
            return Err(Error::ProgramNotFound(bot_user_id));
        }

        // Check if already connected
        if let Some(existing) = self.user_programs.get(&bot_user_id) {
            if existing.is_revoked() {
                return Err(Error::NotConnected);
            }
            return Err(Error::AlreadyConnected);
        }

        // Find in suggested programs
        let suggested = self
            .suggested_programs
            .iter()
            .find(|p| p.user_id() == bot_user_id)
            .ok_or(Error::ProgramNotFound(bot_user_id))?;

        // Create connected program
        let program = ConnectedBotStarRef::new(
            format!("https://t.me/bot?start={}", bot_user_id.get()),
            chrono::Utc::now().timestamp() as i32,
            bot_user_id,
            suggested.info().parameters().clone(),
            0,
            0,
        );

        let connected = ConnectedProgram {
            url: program.url().to_string(),
            date: program.date(),
            user_id: program.user_id(),
            parameters: program.parameters().clone(),
            participant_count: program.participant_count(),
            revenue_star_count: program.revenue_star_count(),
        };

        // Store the program
        self.connected_programs.push(program.clone());
        self.user_programs.insert(bot_user_id, program);

        Ok(connected)
    }

    /// Revokes a referral program.
    ///
    /// # Arguments
    ///
    /// * `affiliate` - Affiliate type
    /// * `url` - Referral URL to revoke
    ///
    /// # Returns
    ///
    /// `Ok(program)` if successful, `Err(Error)` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::{ReferralProgramManager, AffiliateType};
    ///
    /// let mut manager = ReferralProgramManager::new();
    /// let result = manager.revoke_referral_program(AffiliateType::Bot, "https://t.me/bot?start=123");
    /// ```
    pub fn revoke_referral_program(
        &mut self,
        _affiliate: AffiliateType,
        url: &str,
    ) -> Result<ConnectedProgram> {
        if url.is_empty() {
            return Err(Error::InvalidUrl);
        }

        // Find the program by URL
        let index = self
            .connected_programs
            .iter()
            .position(|p| p.url() == url)
            .ok_or(Error::NotConnected)?;

        let program = &mut self.connected_programs[index];
        program.revoke();

        let connected = ConnectedProgram {
            url: program.url().to_string(),
            date: program.date(),
            user_id: program.user_id(),
            parameters: program.parameters().clone(),
            participant_count: program.participant_count(),
            revenue_star_count: program.revenue_star_count(),
        };

        Ok(connected)
    }

    /// Gets a connected referral program by bot user ID.
    ///
    /// # Arguments
    ///
    /// * `affiliate` - Affiliate type
    /// * `bot_user_id` - Bot user ID
    ///
    /// # Returns
    ///
    /// `Some(program)` if found, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_referral_program_manager::{ReferralProgramManager, AffiliateType};
    /// use rustgram_types::UserId;
    ///
    /// let manager = ReferralProgramManager::new();
    /// let bot_user_id = UserId::new(123456).unwrap();
    /// let program = manager.get_connected_referral_program(AffiliateType::Bot, bot_user_id);
    /// assert!(program.is_none());
    /// ```
    #[must_use]
    pub fn get_connected_referral_program(
        &self,
        _affiliate: AffiliateType,
        bot_user_id: UserId,
    ) -> Option<ConnectedProgram> {
        self.user_programs
            .get(&bot_user_id)
            .map(|p| ConnectedProgram {
                url: p.url().to_string(),
                date: p.date(),
                user_id: p.user_id(),
                parameters: p.parameters().clone(),
                participant_count: p.participant_count(),
                revenue_star_count: p.revenue_star_count(),
            })
    }

    /// Returns all suggested programs.
    #[must_use]
    pub fn suggested_programs(&self) -> &[SuggestedBotStarRef] {
        &self.suggested_programs
    }

    /// Returns all connected programs.
    #[must_use]
    pub fn connected_programs(&self) -> &[ConnectedBotStarRef] {
        &self.connected_programs
    }

    /// Adds a suggested program.
    pub fn add_suggested_program(&mut self, program: SuggestedBotStarRef) {
        self.suggested_programs.push(program);
    }
}

impl Default for ReferralProgramManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_referral_program_info::ReferralProgramInfo;
    use rustgram_star_amount::StarAmount;

    // === SuggestedBotStarRef Tests ===

    #[test]
    fn test_suggested_bot_star_ref_new() {
        let user_id = UserId::new(123456).unwrap();
        let info = ReferralProgramInfo::default();
        let program = SuggestedBotStarRef::new(user_id, info);

        assert_eq!(program.user_id(), user_id);
        assert!(!program.is_valid()); // Default info is not valid
    }

    #[test]
    fn test_suggested_bot_star_ref_is_valid() {
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);

        let program = SuggestedBotStarRef::new(user_id, info);
        assert!(program.is_valid());
    }

    #[test]
    fn test_suggested_bot_star_ref_is_active() {
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);

        let program = SuggestedBotStarRef::new(user_id, info);
        assert!(program.is_active());
    }

    // === ConnectedBotStarRef Tests ===

    #[test]
    fn test_connected_bot_star_ref_new() {
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);

        let program = ConnectedBotStarRef::new(
            "https://t.me/bot".to_string(),
            1234567890,
            user_id,
            params,
            100,
            1000,
        );

        assert_eq!(program.url(), "https://t.me/bot");
        assert_eq!(program.date(), 1234567890);
        assert_eq!(program.user_id(), user_id);
        assert_eq!(program.participant_count(), 100);
        assert_eq!(program.revenue_star_count(), 1000);
        assert!(!program.is_revoked());
    }

    #[test]
    fn test_connected_bot_star_ref_is_valid() {
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);

        let program = ConnectedBotStarRef::new(
            "https://t.me/bot".to_string(),
            1234567890,
            user_id,
            params,
            100,
            1000,
        );

        assert!(program.is_valid());
    }

    #[test]
    fn test_connected_bot_star_ref_invalid_empty_url() {
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);

        let program =
            ConnectedBotStarRef::new(String::new(), 1234567890, user_id, params, 100, 1000);

        assert!(!program.is_valid());
    }

    #[test]
    fn test_connected_bot_star_ref_revoke() {
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);

        let mut program = ConnectedBotStarRef::new(
            "https://t.me/bot".to_string(),
            1234567890,
            user_id,
            params,
            100,
            1000,
        );

        assert!(!program.is_revoked());
        program.revoke();
        assert!(program.is_revoked());
    }

    // === ReferralProgramManager Tests ===

    #[test]
    fn test_referral_program_manager_new() {
        let manager = ReferralProgramManager::new();
        assert!(manager.suggested_programs().is_empty());
        assert!(manager.connected_programs().is_empty());
    }

    #[test]
    fn test_set_dialog_referral_program_valid() {
        let mut manager = ReferralProgramManager::new();
        let params = ReferralProgramParameters::with_params(100, 12);
        let dialog_id = DialogId::from_user(UserId::new(123456).unwrap());

        assert!(manager
            .set_dialog_referral_program(dialog_id, params)
            .is_ok());
    }

    #[test]
    fn test_set_dialog_referral_program_invalid() {
        let mut manager = ReferralProgramManager::new();
        let params = ReferralProgramParameters::default(); // Invalid
        let dialog_id = DialogId::from_user(UserId::new(123456).unwrap());

        assert!(manager
            .set_dialog_referral_program(dialog_id, params)
            .is_err());
    }

    #[test]
    fn test_search_dialog_referral_program() {
        let manager = ReferralProgramManager::new();
        let result = manager.search_dialog_referral_program("bot", "ref123");
        assert!(result.is_err());
    }

    #[test]
    fn test_search_referral_programs_empty() {
        let manager = ReferralProgramManager::new();
        let result = manager.search_referral_programs(
            AffiliateType::Bot,
            ReferralProgramSortOrder::Profitability,
            "",
            10,
        );

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_search_referral_programs_with_suggested() {
        let mut manager = ReferralProgramManager::new();

        // Add a suggested program
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params.clone(), 0, daily);
        manager.add_suggested_program(SuggestedBotStarRef::new(user_id, info));

        let result = manager.search_referral_programs(
            AffiliateType::Bot,
            ReferralProgramSortOrder::Profitability,
            "",
            10,
        );

        assert!(result.is_ok());
        let programs = result.unwrap();
        assert_eq!(programs.len(), 1);
        assert_eq!(programs[0].user_id, user_id);
    }

    #[test]
    fn test_connect_referral_program_not_found() {
        let mut manager = ReferralProgramManager::new();
        let bot_user_id = UserId::new(123456).unwrap();

        let result = manager.connect_referral_program(AffiliateType::Bot, bot_user_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_connect_referral_program_success() {
        let mut manager = ReferralProgramManager::new();

        // Add a suggested program first
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);
        manager.add_suggested_program(SuggestedBotStarRef::new(user_id, info));

        let result = manager.connect_referral_program(AffiliateType::Bot, user_id);
        assert!(result.is_ok());

        let connected = result.unwrap();
        assert_eq!(connected.user_id, user_id);
        assert!(manager
            .get_connected_referral_program(AffiliateType::Bot, user_id)
            .is_some());
    }

    #[test]
    fn test_connect_referral_program_already_connected() {
        let mut manager = ReferralProgramManager::new();

        // Add a suggested program first
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);
        manager.add_suggested_program(SuggestedBotStarRef::new(user_id, info));

        // Connect once
        assert!(manager
            .connect_referral_program(AffiliateType::Bot, user_id)
            .is_ok());

        // Try to connect again
        let result = manager.connect_referral_program(AffiliateType::Bot, user_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_revoke_referral_program_empty_url() {
        let mut manager = ReferralProgramManager::new();
        let result = manager.revoke_referral_program(AffiliateType::Bot, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_revoke_referral_program_not_connected() {
        let mut manager = ReferralProgramManager::new();
        let result = manager.revoke_referral_program(AffiliateType::Bot, "https://t.me/bot");
        assert!(result.is_err());
    }

    #[test]
    fn test_revoke_referral_program_success() {
        let mut manager = ReferralProgramManager::new();

        // Add and connect a program
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);
        manager.add_suggested_program(SuggestedBotStarRef::new(user_id, info));

        manager
            .connect_referral_program(AffiliateType::Bot, user_id)
            .unwrap();

        let url = manager.connected_programs()[0].url().to_string();
        let result = manager.revoke_referral_program(AffiliateType::Bot, &url);
        assert!(result.is_ok());
        assert!(manager.connected_programs()[0].is_revoked());
    }

    #[test]
    fn test_get_connected_referral_program_none() {
        let manager = ReferralProgramManager::new();
        let bot_user_id = UserId::new(123456).unwrap();

        let program = manager.get_connected_referral_program(AffiliateType::Bot, bot_user_id);
        assert!(program.is_none());
    }

    #[test]
    fn test_get_connected_referral_program_some() {
        let mut manager = ReferralProgramManager::new();

        // Add and connect a program
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);
        manager.add_suggested_program(SuggestedBotStarRef::new(user_id, info));

        manager
            .connect_referral_program(AffiliateType::Bot, user_id)
            .unwrap();

        let program = manager.get_connected_referral_program(AffiliateType::Bot, user_id);
        assert!(program.is_some());
        assert_eq!(program.unwrap().user_id, user_id);
    }

    #[test]
    fn test_add_suggested_program() {
        let mut manager = ReferralProgramManager::new();
        let user_id = UserId::new(123456).unwrap();
        let info = ReferralProgramInfo::default();

        manager.add_suggested_program(SuggestedBotStarRef::new(user_id, info));
        assert_eq!(manager.suggested_programs().len(), 1);
    }

    #[test]
    fn test_default() {
        let manager = ReferralProgramManager::default();
        assert!(manager.suggested_programs().is_empty());
        assert!(manager.connected_programs().is_empty());
    }

    #[test]
    fn test_clone() {
        let manager = ReferralProgramManager::new();
        let cloned = manager.clone();
        assert_eq!(
            cloned.suggested_programs().len(),
            manager.suggested_programs().len()
        );
    }

    #[test]
    fn test_affiliate_type_bot() {
        let affiliate = AffiliateType::Bot;
        // Just ensure the type exists
        assert!(matches!(affiliate, AffiliateType::Bot));
    }

    #[test]
    fn test_affiliate_type_channel() {
        let affiliate = AffiliateType::Channel;
        // Just ensure the type exists
        assert!(matches!(affiliate, AffiliateType::Channel));
    }

    #[test]
    fn test_connected_bot_star_ref_clone() {
        let user_id = UserId::new(123456).unwrap();
        let params = ReferralProgramParameters::with_params(100, 12);

        let program = ConnectedBotStarRef::new(
            "https://t.me/bot".to_string(),
            1234567890,
            user_id,
            params,
            100,
            1000,
        );

        let cloned = program.clone();
        assert_eq!(program, cloned);
    }

    #[test]
    fn test_suggested_bot_star_ref_clone() {
        let user_id = UserId::new(123456).unwrap();
        let info = ReferralProgramInfo::default();
        let program = SuggestedBotStarRef::new(user_id, info);

        let cloned = program.clone();
        assert_eq!(program, cloned);
    }
}
