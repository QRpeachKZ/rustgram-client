//! Authentication state machine
//!
//! This module defines the state machine for Telegram authentication flow.
//! Based on TDLib's `AuthManager::State` enum.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Current state of the authentication flow
///
/// Represents the state machine from TDLib's `AuthManager::State` enum.
/// Each state corresponds to a specific phase in the authentication process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AuthState {
    /// No authentication started (State::None)
    #[default]
    Idle,

    /// Waiting for phone number input (State::WaitPhoneNumber)
    WaitingForPhone,

    /// Waiting for premium purchase confirmation (State::WaitPremiumPurchase)
    WaitingForPremiumPurchase,

    /// Waiting for authentication code (State::WaitCode)
    WaitingForCode,

    /// Waiting for QR code confirmation (State::WaitQrCodeConfirmation)
    WaitingForQrCode,

    /// Waiting for password (2FA) (State::WaitPassword)
    WaitingForPassword,

    /// Waiting for user registration (State::WaitRegistration)
    WaitingForRegistration,

    /// Waiting for email address (State::WaitEmailAddress)
    WaitingForEmailAddress,

    /// Waiting for email code verification (State::WaitEmailCode)
    WaitingForEmailCode,

    /// Successfully authenticated (State::Ok)
    Authenticated,

    /// Logging out (State::LoggingOut)
    LoggingOut,

    /// Destroying authentication keys (State::DestroyingKeys)
    DestroyingKeys,

    /// Closing connection (State::Closing)
    Closing,

    /// Authentication failed
    Failed,
}

impl fmt::Display for AuthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::WaitingForPhone => write!(f, "WaitingForPhone"),
            Self::WaitingForPremiumPurchase => write!(f, "WaitingForPremiumPurchase"),
            Self::WaitingForCode => write!(f, "WaitingForCode"),
            Self::WaitingForQrCode => write!(f, "WaitingForQrCode"),
            Self::WaitingForPassword => write!(f, "WaitingForPassword"),
            Self::WaitingForRegistration => write!(f, "WaitingForRegistration"),
            Self::WaitingForEmailAddress => write!(f, "WaitingForEmailAddress"),
            Self::WaitingForEmailCode => write!(f, "WaitingForEmailCode"),
            Self::Authenticated => write!(f, "Authenticated"),
            Self::LoggingOut => write!(f, "LoggingOut"),
            Self::DestroyingKeys => write!(f, "DestroyingKeys"),
            Self::Closing => write!(f, "Closing"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}

impl AuthState {
    /// Check if authentication is complete
    pub const fn is_authenticated(&self) -> bool {
        matches!(self, Self::Authenticated)
    }

    /// Check if waiting for user input
    pub const fn is_waiting_input(&self) -> bool {
        matches!(
            self,
            Self::WaitingForPhone
                | Self::WaitingForCode
                | Self::WaitingForPassword
                | Self::WaitingForRegistration
                | Self::WaitingForEmailAddress
                | Self::WaitingForEmailCode
                | Self::WaitingForPremiumPurchase
        )
    }

    /// Check if state is terminal (no further transitions possible)
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Authenticated | Self::Failed | Self::DestroyingKeys | Self::Closing
        )
    }

    /// Check if state is a waiting state (transitional)
    pub const fn is_waiting(&self) -> bool {
        matches!(
            self,
            Self::WaitingForPhone
                | Self::WaitingForCode
                | Self::WaitingForPassword
                | Self::WaitingForRegistration
                | Self::WaitingForEmailAddress
                | Self::WaitingForEmailCode
                | Self::WaitingForPremiumPurchase
                | Self::WaitingForQrCode
        )
    }

    /// Check if state allows transitions
    pub const fn is_active(&self) -> bool {
        !self.is_terminal() && !matches!(self, Self::LoggingOut)
    }

    /// Get timeout for this state in seconds
    ///
    /// Returns the timeout after which the state expires.
    /// Based on TDLib's `DbState` timeout logic.
    pub const fn timeout(&self) -> Option<u32> {
        match self {
            Self::WaitingForPassword
            | Self::WaitingForPremiumPurchase
            | Self::WaitingForRegistration => Some(86400), // 24 hours
            Self::WaitingForEmailAddress
            | Self::WaitingForEmailCode
            | Self::WaitingForCode
            | Self::WaitingForQrCode => Some(300), // 5 minutes
            _ => None,
        }
    }
}

/// Query type for authentication requests
///
/// Corresponds to TDLib's `AuthManager::NetQueryType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AuthQueryType {
    /// No active query
    #[default]
    None,

    /// Sign in request
    SignIn,

    /// Sign up request
    SignUp,

    /// Send code request
    SendCode,

    /// Check premium purchase
    CheckPremiumPurchase,

    /// Set premium purchase transaction
    SetPremiumPurchaseTransaction,

    /// Send email code
    SendEmailCode,

    /// Verify email address
    VerifyEmailAddress,

    /// Reset email address
    ResetEmailAddress,

    /// Request QR code
    RequestQrCode,

    /// Import QR code
    ImportQrCode,

    /// Get password info
    GetPassword,

    /// Check password
    CheckPassword,

    /// Request password recovery
    RequestPasswordRecovery,

    /// Check password recovery code
    CheckPasswordRecoveryCode,

    /// Recover password
    RecoverPassword,

    /// Request Firebase SMS
    RequestFirebaseSms,

    /// Bot authentication
    BotAuthentication,

    /// General authentication
    Authentication,

    /// Log out
    LogOut,

    /// Delete account
    DeleteAccount,
}

/// Authentication state transition info
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateTransition {
    /// Previous state
    pub from: AuthState,
    /// New state
    pub to: AuthState,
    /// Query type that caused transition
    pub query_type: AuthQueryType,
}

impl StateTransition {
    /// Create a new state transition
    pub const fn new(from: AuthState, to: AuthState, query_type: AuthQueryType) -> Self {
        Self {
            from,
            to,
            query_type,
        }
    }

    /// Check if transition is valid
    pub fn is_valid(&self) -> bool {
        // Define valid transitions based on TDLib's state machine
        matches!(
            (self.from, self.to),
            (AuthState::Idle, AuthState::WaitingForPhone)
                | (AuthState::Idle, AuthState::WaitingForQrCode)
                | (AuthState::Idle, AuthState::Authenticated) // Bot auth
                | (AuthState::WaitingForPhone, AuthState::WaitingForCode)
                | (AuthState::WaitingForPhone, AuthState::WaitingForPremiumPurchase)
                | (AuthState::WaitingForCode, AuthState::WaitingForPassword)
                | (AuthState::WaitingForCode, AuthState::WaitingForRegistration)
                | (AuthState::WaitingForCode, AuthState::Authenticated)
                | (AuthState::WaitingForPassword, AuthState::Authenticated)
                | (AuthState::WaitingForRegistration, AuthState::Authenticated)
                | (AuthState::WaitingForEmailAddress, AuthState::WaitingForEmailCode)
                | (AuthState::WaitingForEmailCode, AuthState::WaitingForCode)
                | (AuthState::WaitingForQrCode, AuthState::Authenticated)
                | (AuthState::Authenticated, AuthState::LoggingOut)
                | (AuthState::LoggingOut, AuthState::DestroyingKeys)
                | (_, AuthState::Failed)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        assert_eq!(AuthState::default(), AuthState::Idle);
    }

    #[test]
    fn test_state_checks() {
        assert!(!AuthState::Idle.is_authenticated());
        assert!(AuthState::Authenticated.is_authenticated());
        assert!(AuthState::WaitingForCode.is_waiting_input());
        assert!(AuthState::Authenticated.is_terminal());
        assert!(AuthState::Idle.is_active());
        assert!(!AuthState::LoggingOut.is_active());
    }

    #[test]
    fn test_state_timeouts() {
        assert_eq!(AuthState::WaitingForPassword.timeout(), Some(86400));
        assert_eq!(AuthState::WaitingForCode.timeout(), Some(300));
        assert_eq!(AuthState::Idle.timeout(), None);
        assert_eq!(AuthState::Authenticated.timeout(), None);
    }

    #[test]
    fn test_valid_transitions() {
        let transition = StateTransition::new(
            AuthState::WaitingForPhone,
            AuthState::WaitingForCode,
            AuthQueryType::SendCode,
        );
        assert!(transition.is_valid());

        let invalid = StateTransition::new(
            AuthState::Authenticated,
            AuthState::Idle,
            AuthQueryType::None,
        );
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_display() {
        assert_eq!(AuthState::Idle.to_string(), "Idle");
        assert_eq!(AuthState::WaitingForCode.to_string(), "WaitingForCode");
        assert_eq!(AuthState::Authenticated.to_string(), "Authenticated");
    }
}
