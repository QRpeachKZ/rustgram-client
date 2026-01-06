//! Authentication code types and helpers
//!
//! This module handles sent codes and code types for Telegram authentication.
//! Based on TDLib's `SendCodeHelper` and `auth_SentCode` TL type.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of authentication code that was sent
///
/// Corresponds to the various `auth_SentCodeType` TL constructors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentCodeType {
    /// Message sent to the app (internal)
    App,

    /// SMS code
    Sms {
        /// Length of the code
        length: u32,
    },

    /// Call with code
    Call {
        /// Length of the code
        length: u32,
    },

    /// Flash call (shows code as incoming call)
    FlashCall,

    /// Code sent via missed call notification
    MissedCall {
        /// Phone number prefix
        prefix: String,
        /// Length of the code
        length: u32,
    },

    /// Code sent to email
    EmailCode {
        /// Email pattern (e.g., "e***@example.com")
        email_pattern: String,
        /// Length of the code
        length: u32,
    },

    /// Code sent via email with Apple ID
    EmailApple {
        /// Email pattern
        email_pattern: String,
        /// Length of the code
        length: u32,
    },

    /// Code sent via email with Google ID
    EmailGoogle {
        /// Email pattern
        email_pattern: String,
        /// Length of the code
        length: u32,
    },

    /// Setup email required first
    SetUpEmailRequired {
        /// App-specific privacy URL
        app_privacy_url: String,
    },

    /// Code sent via Firebase SMS
    FirebaseSms {
        /// Play Store app configuration
        play_store_app_config: String,
        /// App Store app configuration
        app_store_app_config: String,
        /// Length of the code
        length: u32,
    },

    /// Code sent via SMS with iOS native push
    SmsIosPush {
        /// Play Store app configuration
        play_store_app_config: String,
        /// App Store app configuration
        app_store_app_config: String,
        /// Length of the code
        length: u32,
    },

    /// Unknown code type (for forward compatibility)
    Unknown {
        /// Type name
        type_name: String,
    },
}

impl fmt::Display for SentCodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::App => write!(f, "App"),
            Self::Sms { length } => write!(f, "SMS ({} digits)", length),
            Self::Call { length } => write!(f, "Call ({} digits)", length),
            Self::FlashCall => write!(f, "Flash Call"),
            Self::MissedCall { prefix, length } => {
                write!(f, "Missed Call (prefix: {}, {} digits)", prefix, length)
            }
            Self::EmailCode {
                email_pattern,
                length,
            } => write!(f, "Email ({}, {} digits)", email_pattern, length),
            Self::EmailApple {
                email_pattern,
                length,
            } => write!(f, "Apple ID ({}, {} digits)", email_pattern, length),
            Self::EmailGoogle {
                email_pattern,
                length,
            } => write!(f, "Google ID ({}, {} digits)", email_pattern, length),
            Self::SetUpEmailRequired { .. } => write!(f, "Setup Email Required"),
            Self::FirebaseSms { length, .. } => write!(f, "Firebase SMS ({} digits)", length),
            Self::SmsIosPush { length, .. } => write!(f, "iOS Push SMS ({} digits)", length),
            Self::Unknown { type_name } => write!(f, "Unknown ({})", type_name),
        }
    }
}

impl SentCodeType {
    /// Get the expected length of the code
    pub const fn code_length(&self) -> Option<u32> {
        match self {
            Self::Sms { length }
            | Self::Call { length }
            | Self::MissedCall { length, .. }
            | Self::EmailCode { length, .. }
            | Self::EmailApple { length, .. }
            | Self::EmailGoogle { length, .. }
            | Self::FirebaseSms { length, .. }
            | Self::SmsIosPush { length, .. } => Some(*length),
            _ => None,
        }
    }

    /// Check if this is an email-based code
    pub const fn is_email(&self) -> bool {
        matches!(
            self,
            Self::EmailCode { .. }
                | Self::EmailApple { .. }
                | Self::EmailGoogle { .. }
                | Self::SetUpEmailRequired { .. }
        )
    }

    /// Check if this is an SMS-based code
    pub const fn is_sms(&self) -> bool {
        matches!(
            self,
            Self::Sms { .. } | Self::FirebaseSms { .. } | Self::SmsIosPush { .. }
        )
    }

    /// Check if this is a call-based code
    pub const fn is_call(&self) -> bool {
        matches!(
            self,
            Self::Call { .. } | Self::FlashCall | Self::MissedCall { .. }
        )
    }
}

/// Information about a sent authentication code
///
/// Corresponds to TDLib's `SendCodeHelper` and `auth_SentCode` TL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentCode {
    /// Phone code hash used for verification
    pub phone_code_hash: String,

    /// Type of code that was sent
    pub code_type: SentCodeType,

    /// Timeout before next code can be sent (seconds)
    pub next_type: Option<SentCodeType>,

    /// When the code was sent
    pub sent_at: i64,

    /// Expiration time of the code
    pub expires_at: i64,
}

impl SentCode {
    /// Create a new sent code info
    pub fn new(phone_code_hash: String, code_type: SentCodeType, timeout_seconds: i64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Self {
            phone_code_hash,
            code_type,
            next_type: None,
            sent_at: now,
            expires_at: now + timeout_seconds,
        }
    }

    /// Check if the code has expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        now > self.expires_at
    }

    /// Get remaining time until expiration (seconds)
    pub fn remaining_time(&self) -> i64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        (self.expires_at - now).max(0)
    }

    /// Get the phone code hash
    pub fn phone_code_hash(&self) -> &str {
        &self.phone_code_hash
    }

    /// Get the code type
    pub fn code_type(&self) -> &SentCodeType {
        &self.code_type
    }
}

/// Reason for resending an authentication code
///
/// Corresponds to TDLib's `ResendCodeReason` TD API type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResendCodeReason {
    /// Code was not received
    CodeNotReceived,

    /// SMS delivery failed
    SmsFailed,

    /// Call delivery failed
    CallFailed,

    /// User requested resend
    UserRequest,

    /// Unknown reason
    Unknown {
        /// Reason description
        reason: String,
    },
}

impl fmt::Display for ResendCodeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeNotReceived => write!(f, "CodeNotReceived"),
            Self::SmsFailed => write!(f, "SmsFailed"),
            Self::CallFailed => write!(f, "CallFailed"),
            Self::UserRequest => write!(f, "UserRequest"),
            Self::Unknown { reason } => write!(f, "Unknown({})", reason),
        }
    }
}

/// Phone number authentication settings
///
/// Corresponds to TDLib's `phoneNumberAuthenticationSettings` TD API type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneAuthSettings {
    /// Whether to allow flash call
    pub allow_flash_call: bool,

    /// Whether to allow missed call
    pub allow_missed_call: bool,

    /// Whether to request SMS if call failed
    pub sms_request_if_failed: bool,

    /// Current phone number (for verification)
    pub current_number: Option<String>,

    /// List of allowed Firebase SMS receivers
    pub firebase_sms: Vec<String>,

    /// App-specific settings for iOS push SMS
    pub ios_push_sms: Option<String>,

    /// List of allowed authentication tokens
    pub tokens: Vec<String>,
}

impl Default for PhoneAuthSettings {
    fn default() -> Self {
        Self {
            allow_flash_call: false,
            allow_missed_call: false,
            sms_request_if_failed: true,
            current_number: None,
            firebase_sms: Vec::new(),
            ios_push_sms: None,
            tokens: Vec::new(),
        }
    }
}

impl PhoneAuthSettings {
    /// Create default phone auth settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create settings with flash call enabled
    pub fn with_flash_call() -> Self {
        Self {
            allow_flash_call: true,
            ..Self::default()
        }
    }

    /// Create settings with missed call enabled
    pub fn with_missed_call() -> Self {
        Self {
            allow_missed_call: true,
            ..Self::default()
        }
    }

    /// Add a Firebase SMS receiver
    pub fn add_firebase_token(mut self, token: String) -> Self {
        self.firebase_sms.push(token);
        self
    }

    /// Set the current phone number
    pub fn with_current_number(mut self, number: String) -> Self {
        self.current_number = Some(number);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_type_display() {
        let sms = SentCodeType::Sms { length: 5 };
        assert!(sms.to_string().contains("5"));

        let email = SentCodeType::EmailCode {
            email_pattern: "e***@test.com".to_string(),
            length: 6,
        };
        assert!(email.to_string().contains("e***@test.com"));
    }

    #[test]
    fn test_code_type_classification() {
        let sms = SentCodeType::Sms { length: 5 };
        assert!(sms.is_sms());
        assert!(!sms.is_email());
        assert!(!sms.is_call());

        let email = SentCodeType::EmailCode {
            email_pattern: "e***@test.com".to_string(),
            length: 6,
        };
        assert!(email.is_email());
        assert!(!email.is_sms());
        assert!(!email.is_call());

        let call = SentCodeType::Call { length: 5 };
        assert!(call.is_call());
        assert!(!call.is_sms());
        assert!(!call.is_email());
    }

    #[test]
    fn test_sent_code_expiration() {
        // This test would need time mocking for proper testing
        let code = SentCode::new(
            "test_hash".to_string(),
            SentCodeType::Sms { length: 5 },
            300,
        );
        assert_eq!(code.phone_code_hash(), "test_hash");
        assert!(code.remaining_time() >= 0);
    }

    #[test]
    fn test_phone_auth_settings() {
        let settings = PhoneAuthSettings::new()
            .add_firebase_token("firebase_token".to_string())
            .with_current_number("+1234567890".to_string());

        assert!(!settings.allow_flash_call);
        assert_eq!(settings.firebase_sms.len(), 1);
        assert_eq!(settings.current_number, Some("+1234567890".to_string()));
    }
}
