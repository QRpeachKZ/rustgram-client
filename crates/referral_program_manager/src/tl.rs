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

//! Stub types for referral program manager.
//!
//! This module contains stub implementations for TL types that are not yet
//! fully implemented in the codebase.

use rustgram_referral_program_info::ReferralProgramInfo;
use rustgram_referral_program_parameters::ReferralProgramParameters;
use rustgram_types::UserId;
use std::fmt;

/// Affiliate type for referral programs.
///
/// # TL Correspondence
///
/// This is a stub for the TL type `AffiliateType` which will be
/// fully implemented when the TL layer is complete.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::AffiliateType;
///
/// let bot_type = AffiliateType::Bot;
/// let channel_type = AffiliateType::Channel;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AffiliateType {
    /// Bot affiliate program.
    Bot,
    /// Channel affiliate program.
    Channel,
}

impl fmt::Display for AffiliateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bot => write!(f, "Bot"),
            Self::Channel => write!(f, "Channel"),
        }
    }
}

/// Found referral program.
///
/// # TL Correspondence
///
/// This is a stub for the TL type `FoundProgram` which will be
/// fully implemented when the TL layer is complete.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::FoundProgram;
/// use rustgram_types::UserId;
/// use rustgram_referral_program_info::ReferralProgramInfo;
///
/// let program = FoundProgram {
///     user_id: UserId::new(123456).unwrap(),
///     info: ReferralProgramInfo::default(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundProgram {
    /// Bot user ID.
    pub user_id: UserId,
    /// Referral program information.
    pub info: ReferralProgramInfo,
}

/// Connected referral program.
///
/// # TL Correspondence
///
/// This is a stub for the TL type `ConnectedProgram` which will be
/// fully implemented when the TL layer is complete.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::ConnectedProgram;
/// use rustgram_types::UserId;
/// use rustgram_referral_program_parameters::ReferralProgramParameters;
///
/// let program = ConnectedProgram {
///     url: "https://t.me/bot".to_string(),
///     date: 1234567890,
///     user_id: UserId::new(123456).unwrap(),
///     parameters: ReferralProgramParameters::default(),
///     participant_count: 100,
///     revenue_star_count: 1000,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectedProgram {
    /// Referral URL.
    pub url: String,
    /// Connection date as Unix timestamp.
    pub date: i32,
    /// Bot user ID.
    pub user_id: UserId,
    /// Program parameters.
    pub parameters: ReferralProgramParameters,
    /// Number of participants.
    pub participant_count: i64,
    /// Revenue in star count.
    pub revenue_star_count: i64,
}

/// Chat stub for referral program search results.
///
/// # TL Correspondence
///
/// This is a stub for the TL type `Chat` which will be
/// fully implemented when the TL layer is complete.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::Chat;
///
/// let chat = Chat {
///     id: 123456,
///     title: "Test Chat".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chat {
    /// Chat ID.
    pub id: i64,
    /// Chat title.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affiliate_type_display() {
        assert_eq!(format!("{}", AffiliateType::Bot), "Bot");
        assert_eq!(format!("{}", AffiliateType::Channel), "Channel");
    }

    #[test]
    fn test_affiliate_type_copy() {
        let bot_type = AffiliateType::Bot;
        let copy = bot_type;
        assert_eq!(bot_type, copy);
    }

    #[test]
    fn test_affiliate_type_clone() {
        let bot_type = AffiliateType::Bot;
        let cloned = bot_type;
        assert_eq!(bot_type, cloned);
    }

    #[test]
    fn test_found_program_new() {
        let program = FoundProgram {
            user_id: UserId::new(123456).unwrap(),
            info: ReferralProgramInfo::default(),
        };
        assert_eq!(program.user_id.get(), 123456);
    }

    #[test]
    fn test_connected_program_new() {
        let program = ConnectedProgram {
            url: "https://t.me/bot".to_string(),
            date: 1234567890,
            user_id: UserId::new(123456).unwrap(),
            parameters: ReferralProgramParameters::default(),
            participant_count: 100,
            revenue_star_count: 1000,
        };
        assert_eq!(program.url, "https://t.me/bot");
    }

    #[test]
    fn test_chat_new() {
        let chat = Chat {
            id: 123456,
            title: "Test Chat".to_string(),
        };
        assert_eq!(chat.id, 123456);
        assert_eq!(chat.title, "Test Chat");
    }
}
