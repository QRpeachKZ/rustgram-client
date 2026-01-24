// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! TDLib authorization state translation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_auth_manager::State as RustgramState;
use serde::{Deserialize, Serialize};

/// TDLib authorization state enum.
///
/// Represents the current state of the authentication flow in TDLib format.
/// This maps directly to TDLib's `authorizationState` types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum TdAuthState {
    /// Waiting for phone number
    #[serde(rename = "authorizationStateWaitPhoneNumber")]
    WaitPhoneNumber,

    /// Waiting for authentication code
    #[serde(rename = "authorizationStateWaitCode")]
    WaitCode,

    /// Waiting for password (2FA)
    #[serde(rename = "authorizationStateWaitPassword")]
    WaitPassword,

    /// Ready (authenticated)
    #[serde(rename = "authorizationStateReady")]
    Ready,

    /// Closing
    #[serde(rename = "authorizationStateClosing")]
    Closing,

    /// Network error
    #[serde(rename = "authorizationStateWaitTdlib")]
    WaitTdlib,
}

impl TdAuthState {
    /// Converts from rustgram AuthManager state.
    pub fn from_rustgram(state: &RustgramState) -> Self {
        match state {
            RustgramState::None => Self::WaitPhoneNumber,
            RustgramState::WaitPhoneNumber => Self::WaitPhoneNumber,
            RustgramState::WaitCode => Self::WaitCode,
            RustgramState::WaitPassword => Self::WaitPassword,
            RustgramState::WaitEmailCode => Self::WaitCode, // Treat email code same as regular code
            RustgramState::Ok => Self::Ready,
            RustgramState::LoggingOut => Self::Closing,
            RustgramState::Closing => Self::Closing,
            RustgramState::NetworkError(_) => Self::WaitTdlib,
            RustgramState::WaitingRetry { .. } => Self::WaitTdlib,
        }
    }

    /// Converts to TDLib JSON format.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|_| {
            serde_json::json!({
                "@type": "authorizationStateWaitTdlib"
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_td_auth_state_serialization() {
        let state = TdAuthState::WaitPhoneNumber;
        let json = serde_json::to_string(&state).unwrap();

        assert!(json.contains("authorizationStateWaitPhoneNumber"));
    }

    #[test]
    fn test_td_auth_state_deserialization() {
        let json = r#"{"@type":"authorizationStateWaitCode"}"#;
        let state: TdAuthState = serde_json::from_str(json).unwrap();

        assert_eq!(state, TdAuthState::WaitCode);
    }

    #[test]
    fn test_from_rustgram_none() {
        let rg_state = RustgramState::None;
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::WaitPhoneNumber);
    }

    #[test]
    fn test_from_rustgram_wait_phone_number() {
        let rg_state = RustgramState::WaitPhoneNumber;
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::WaitPhoneNumber);
    }

    #[test]
    fn test_from_rustgram_wait_code() {
        let rg_state = RustgramState::WaitCode;
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::WaitCode);
    }

    #[test]
    fn test_from_rustgram_wait_password() {
        let rg_state = RustgramState::WaitPassword;
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::WaitPassword);
    }

    #[test]
    fn test_from_rustgram_ok() {
        let rg_state = RustgramState::Ok;
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::Ready);
    }

    #[test]
    fn test_from_rustgram_logging_out() {
        let rg_state = RustgramState::LoggingOut;
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::Closing);
    }

    #[test]
    fn test_from_rustgram_closing() {
        let rg_state = RustgramState::Closing;
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::Closing);
    }

    #[test]
    fn test_from_rustgram_network_error() {
        let rg_state = RustgramState::NetworkError("test error".to_string());
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::WaitTdlib);
    }

    #[test]
    fn test_from_rustgram_waiting_retry() {
        let rg_state = RustgramState::WaitingRetry {
            attempts: 2,
            delay: Duration::from_secs(5),
        };
        let td_state = TdAuthState::from_rustgram(&rg_state);

        assert_eq!(td_state, TdAuthState::WaitTdlib);
    }

    #[test]
    fn test_to_json() {
        let state = TdAuthState::Ready;
        let json = state.to_json();

        assert_eq!(json["@type"], "authorizationStateReady");
    }

    #[test]
    fn test_td_auth_state_equality() {
        assert_eq!(TdAuthState::WaitPhoneNumber, TdAuthState::WaitPhoneNumber);
        assert_ne!(TdAuthState::WaitCode, TdAuthState::WaitPassword);
    }

    #[test]
    fn test_all_states_roundtrip() {
        let states = vec![
            TdAuthState::WaitPhoneNumber,
            TdAuthState::WaitCode,
            TdAuthState::WaitPassword,
            TdAuthState::Ready,
            TdAuthState::Closing,
            TdAuthState::WaitTdlib,
        ];

        for state in states {
            let json = serde_json::to_string(&state).unwrap();
            let deserialized: TdAuthState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, deserialized);
        }
    }
}
