//! TL types for authentication protocol
//!
//! This module defines TL (Type Language) types used in MTProto authentication.
//! Based on TDLib's telegram_api auth types.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Authorization
///
/// Represents successful authentication result.
/// Corresponds to `auth_Authorization` TL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    /// Indicates if this is a signup (not login)
    pub is_signup: bool,

    /// When the authorization was created
    pub created_at: i64,

    /// User ID (if available)
    pub user_id: Option<i64>,

    /// Bot flag (for bot authentication)
    pub is_bot: bool,

    /// Terms of service (if needs acceptance)
    pub terms_of_service: Option<TermsOfService>,
}

impl Authorization {
    /// Create a new authorization
    pub fn new(is_signup: bool, user_id: Option<i64>, is_bot: bool) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Self {
            is_signup,
            created_at: now,
            user_id,
            is_bot,
            terms_of_service: None,
        }
    }

    /// Add terms of service
    pub fn with_terms_of_service(mut self, tos: TermsOfService) -> Self {
        self.terms_of_service = Some(tos);
        self
    }

    /// Check if this is a signup
    pub const fn is_signup(&self) -> bool {
        self.is_signup
    }

    /// Check if this is a bot
    pub const fn is_bot(&self) -> bool {
        self.is_bot
    }

    /// Get user ID
    pub const fn user_id(&self) -> Option<i64> {
        self.user_id
    }

    /// Get terms of service
    pub fn terms_of_service(&self) -> Option<&TermsOfService> {
        self.terms_of_service.as_ref()
    }
}

/// Login token
///
/// Token for QR code login.
/// Corresponds to `auth_LoginToken` TL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginToken {
    /// Login token
    pub token: Vec<u8>,

    /// Expiration time
    pub expires: i64,

    /// List of user IDs
    pub user_ids: Vec<i64>,
}

impl LoginToken {
    /// Create new login token
    pub fn new(token: Vec<u8>, expires_in_seconds: i64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Self {
            token,
            expires: now + expires_in_seconds,
            user_ids: Vec::new(),
        }
    }

    /// Check if token has expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        now > self.expires
    }

    /// Get token bytes
    pub fn token(&self) -> &[u8] {
        &self.token
    }

    /// Add user ID
    pub fn add_user_id(mut self, user_id: i64) -> Self {
        self.user_ids.push(user_id);
        self
    }
}

/// Sent code
///
/// Information about sent authentication code.
/// Corresponds to `auth_SentCode` TL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentCodeTl {
    /// Phone code hash
    pub phone_code_hash: String,

    /// Type of code sent
    pub code_type: SentCodeTypeTl,

    /// Next code type (if timeout)
    pub next_type: Option<SentCodeTypeTl>,

    /// Timeout before next code (seconds)
    pub timeout: i32,
}

/// Sent code type (TL)
///
/// TL-level representation of code types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentCodeTypeTl {
    /// App internal code
    SentCodeTypeApp,

    /// SMS code
    SentCodeTypeSms { length: i32 },

    /// Call code
    SentCodeTypeCall { length: i32 },

    /// Flash call
    SentCodeTypeFlashCall,

    /// Missed call
    SentCodeTypeMissedCall { prefix: String, length: i32 },

    /// Email code
    SentCodeTypeEmailCode { email_pattern: String, length: i32 },

    /// Email Apple ID
    SentCodeTypeEmailApple { email_pattern: String, length: i32 },

    /// Email Google ID
    SentCodeTypeEmailGoogle { email_pattern: String, length: i32 },

    /// Setup email required
    SentCodeTypeSetUpEmailRequired { app_privacy_url: String },

    /// Firebase SMS
    SentCodeTypeFirebaseSms {
        play_store_app_config: String,
        app_store_app_config: String,
        length: i32,
    },

    /// iOS push SMS
    SentCodeTypeSmsIosPush {
        play_store_app_config: String,
        app_store_app_config: String,
        length: i32,
    },
}

/// Terms of service
///
/// Terms of service that must be accepted.
/// Corresponds to `help_TermsOfService` TL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsOfService {
    /// Terms of service ID
    pub id: String,

    /// Text content
    pub text: String,

    /// Entities (formatting)
    pub entities: Vec<MessageEntity>,

    /// Minimum user age
    pub min_age_confirm: Option<i32>,

    /// Popup flag
    pub popup: bool,
}

impl TermsOfService {
    /// Create new terms of service
    pub fn new(id: String, text: String, popup: bool) -> Self {
        Self {
            id,
            text,
            entities: Vec::new(),
            min_age_confirm: None,
            popup,
        }
    }

    /// Add entity
    pub fn add_entity(mut self, entity: MessageEntity) -> Self {
        self.entities.push(entity);
        self
    }

    /// Set minimum age
    pub fn with_min_age(mut self, age: i32) -> Self {
        self.min_age_confirm = Some(age);
        self
    }

    /// Check if needs confirmation
    pub const fn needs_confirmation(&self) -> bool {
        self.min_age_confirm.is_some()
    }

    /// Get minimum age
    pub const fn min_age(&self) -> Option<i32> {
        self.min_age_confirm
    }
}

/// Message entity
///
/// Formatting entity for text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Entity type
    pub entity_type: String,

    /// Offset in text
    pub offset: i32,

    /// Length
    pub length: i32,

    /// Additional data
    pub data: Option<String>,
}

impl MessageEntity {
    /// Create new message entity
    pub fn new(entity_type: String, offset: i32, length: i32) -> Self {
        Self {
            entity_type,
            offset,
            length,
            data: None,
        }
    }

    /// Add data
    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }
}

/// Authorization state
///
/// Current authorization state for TD API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationStateTl {
    /// Waiting for phone number
    WaitPhoneNumber,

    /// Waiting for code
    WaitCode {
        phone_number: String,
        code_info: SentCodeTl,
        code_type: SentCodeTypeTl,
        next_type: Option<SentCodeTypeTl>,
    },

    /// Waiting for password
    WaitPassword { password_info: PasswordInfoTl },

    /// Waiting for registration
    WaitRegistration { terms_of_service: TermsOfService },

    /// Authorized
    Ok { user_id: i64 },

    /// Logging out
    LoggingOut,

    /// Closing
    Closing,

    /// Unknown
    Unknown,
}

/// Password info (TL)
///
/// TL representation of password info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordInfoTl {
    /// Has password flag
    pub has_password: bool,

    /// Password hint
    pub hint: String,

    /// SRP parameters
    pub srp_id: i64,
    pub srp_g: i32,
    pub srp_p: Vec<u8>,
    pub current_algo: Option<String>,
    pub srp_b: Vec<u8>,

    /// Recovery info
    pub has_recovery: bool,
    pub email_unconfirmed_pattern: Option<String>,
}

impl PasswordInfoTl {
    /// Create password info (no password)
    pub fn no_password() -> Self {
        Self {
            has_password: false,
            hint: String::new(),
            srp_id: 0,
            srp_g: 0,
            srp_p: Vec::new(),
            current_algo: None,
            srp_b: Vec::new(),
            has_recovery: false,
            email_unconfirmed_pattern: None,
        }
    }

    /// Check if has password
    pub const fn has_password(&self) -> bool {
        self.has_password
    }
}

impl Default for PasswordInfoTl {
    fn default() -> Self {
        Self::no_password()
    }
}

/// Export login token
///
/// Result of export login token query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportLoginToken {
    /// Login token ID
    pub id: i64,

    /// Token bytes
    pub token: Vec<u8>,

    /// Expiration time
    pub expires: i64,
}

impl ExportLoginToken {
    /// Create export login token
    pub fn new(id: i64, token: Vec<u8>, expires_in_seconds: i64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Self {
            id,
            token,
            expires: now + expires_in_seconds,
        }
    }

    /// Check if expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        now > self.expires
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization() {
        let auth = Authorization::new(false, Some(123456), false);
        assert!(!auth.is_signup());
        assert!(!auth.is_bot());
        assert_eq!(auth.user_id(), Some(123456));
    }

    #[test]
    fn test_login_token() {
        let token = LoginToken::new(vec![1, 2, 3], 300);
        assert!(!token.is_expired());
        assert_eq!(token.token(), &[1, 2, 3]);
    }

    #[test]
    fn test_terms_of_service() {
        let tos = TermsOfService::new("test_id".to_string(), "Test terms".to_string(), true)
            .with_min_age(16);

        assert!(tos.needs_confirmation());
        assert_eq!(tos.min_age(), Some(16));
        assert!(tos.popup);
    }

    #[test]
    fn test_password_info_tl() {
        let info = PasswordInfoTl::no_password();
        assert!(!info.has_password());
    }
}
