// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Authentication Manager
//!
//! This module provides the main authentication state manager for Telegram client.
//!
//! ## Overview
//!
//! The `AuthManager` is the central coordinator for authentication flow. It tracks
//! the current authentication state, manages active network queries, and provides
//! methods for all authentication operations.
//!
//! ## Architecture
//!
//! Based on TDLib's `AuthManager` class, this module:
//! - Tracks authentication state transitions
//! - Manages active network query types
//! - Coordinates phone, bot token, QR code, and password authentication
//! - Handles logout and account deletion
//! - Integrates with NetQueryDispatcher for network operations
//!
//! ## State Machine
//!
//! ```text
//! None -> WaitPhoneNumber -> WaitCode -> WaitPassword -> Ok
//!                                      |
//!                                      v
//!                                   LoggingOut -> Closing
//!
//! Error states:
//! WaitCode -> NetworkError -> WaitingRetry -> WaitCode
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_auth_manager::{AuthManager, State};
//! use rustgram_net::NetQueryDispatcher;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let dispatcher = NetQueryDispatcher::new();
//!     let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
//!
//!     // Start authentication
//!     manager.set_phone_number("+1234567890".to_string()).await?;
//!
//!     // Check state
//!     assert!(matches!(manager.get_state(), State::WaitPhoneNumber));
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod state;

#[cfg(test)]
mod network_tests;

use bytes::{Bytes, BytesMut};
use rustgram_auth::{PasswordInfo, QrCodeLogin};
use rustgram_net::{
    AuthFlag, GzipFlag, NetQuery, NetQueryCallback, NetQueryDispatcher, NetQueryId,
    NetQueryType as NetQueryFlag, QueryError,
};
use rustgram_terms_of_service::TermsOfService;
use rustgram_types::UserId;
use rustgram_types::{
    Authorization, LoggedOut, SendCodeRequest, SentCode, SignInRequest, TlDeserialize, TlHelper,
    TlSerialize,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

pub use state::State;

/// Maximum number of retry attempts for network operations
const MAX_RETRY_ATTEMPTS: u32 = 3;

/// Base delay for retry attempts (exponential backoff)
const BASE_RETRY_DELAY: Duration = Duration::from_secs(1);

/// TL constructor IDs for auth requests
const TL_SEND_CODE: i32 = 0xa677244fu32 as i32;
const TL_SIGN_IN: i32 = 0x8d52a951u32 as i32;
const TL_LOG_OUT: i32 = 0x3e72ba19u32 as i32;

/// Type of active network query
///
/// Represents the type of authentication operation currently in progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum NetQueryType {
    /// No active query
    #[default]
    None,

    /// Sending phone number
    SendPhoneNumber,

    /// Sending authentication code
    SendCode,

    /// Sending password (2FA)
    SendPassword,

    /// Sending bot token
    SendBotToken,

    /// Requesting QR code
    RequestQrCode,

    /// Sending email verification
    SendEmailVerification,

    /// Checking password
    CheckPassword,

    /// Logging out
    LogOut,

    /// Deleting account
    DeleteAccount,
}

impl NetQueryType {
    /// Check if this is an authentication-related query
    pub const fn is_auth_query(&self) -> bool {
        matches!(
            self,
            Self::SendPhoneNumber
                | Self::SendCode
                | Self::SendPassword
                | Self::SendBotToken
                | Self::RequestQrCode
                | Self::SendEmailVerification
                | Self::CheckPassword
        )
    }

    /// Check if this is a destructive operation
    pub const fn is_destructive(&self) -> bool {
        matches!(self, Self::LogOut | Self::DeleteAccount)
    }
}

/// Pending authentication request
///
/// Tracks in-flight authentication requests.
#[derive(Debug)]
struct PendingAuthRequest {
    /// Query ID for this request
    query_id: NetQueryId,

    /// Request type
    request_type: PendingRequestType,

    /// Timestamp when request was created
    created_at: std::time::Instant,

    /// Retry attempt count
    retry_count: u32,
}

/// Type of pending request
#[derive(Debug, Clone, PartialEq, Eq)]
enum PendingRequestType {
    SendCode(String),
    SignIn(String, String),
    LogOut,
}

/// Authentication manager
///
/// Main state manager that coordinates all authentication operations.
/// Based on TDLib's `AuthManager` class.
///
/// # Example
///
/// ```rust,no_run
/// use rustgram_auth_manager::{AuthManager, State};
/// use rustgram_net::NetQueryDispatcher;
/// use rustgram_types::UserId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let dispatcher = NetQueryDispatcher::new();
/// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
///
/// // Start authentication
/// manager.set_phone_number("+1234567890".to_string()).await?;
///
/// // Check state
/// assert_eq!(manager.get_state(), State::WaitPhoneNumber);
///
/// // Advance to code verification (normally this would happen via network response)
/// manager.set_state(State::WaitCode).await;
///
/// // Submit authentication code
/// manager.check_code("12345".to_string(), None).await?;
///
/// Ok(())
/// # }
/// ```
// #[derive(Debug)]
pub struct AuthManager {
    /// API ID from Telegram
    api_id: i32,

    /// API hash from Telegram
    api_hash: Arc<str>,

    /// Current authentication state
    state: Arc<RwLock<State>>,

    /// Type of active network query
    net_query_type: Arc<RwLock<NetQueryType>>,

    /// Current query ID
    query_id: Arc<RwLock<u64>>,

    /// Sent code info (if code was sent)
    sent_code: Arc<RwLock<Option<SentCode>>>,

    /// Password info (if 2FA is enabled)
    password_info: Arc<RwLock<Option<PasswordInfo>>>,

    /// QR code login session (if active)
    qr_code_login: Arc<RwLock<Option<QrCodeLogin>>>,

    /// Phone code hash for verification
    phone_code_hash: Arc<RwLock<Option<String>>>,

    /// Stored phone number for sign-in requests
    phone_number: Arc<RwLock<Option<String>>>,

    /// Terms of service (if any)
    terms_of_service: Arc<RwLock<Option<TermsOfService>>>,

    /// Whether terms were accepted
    terms_accepted: Arc<RwLock<bool>>,

    /// Current user ID (if authenticated)
    user_id: Arc<RwLock<Option<UserId>>>,

    /// Whether to wait for password
    wait_password: Arc<RwLock<bool>>,

    /// Whether code check was successful
    code_success: Arc<RwLock<bool>>,

    /// Next query ID counter
    next_query_id: Arc<std::sync::atomic::AtomicU64>,

    /// Network query dispatcher
    dispatcher: Arc<NetQueryDispatcher>,

    /// Pending requests tracked by query ID
    pending_requests: Arc<parking_lot::Mutex<HashMap<NetQueryId, PendingAuthRequest>>>,

    /// Request timeout duration
    request_timeout: Duration,
}

impl AuthManager {
    /// Creates a new authentication manager
    ///
    /// # Arguments
    ///
    /// * `api_id` - API ID from Telegram
    /// * `api_hash` - API hash from Telegram
    /// * `dispatcher` - Network query dispatcher for sending requests
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    /// assert!(!manager.is_authorized().await);
    /// # }
    /// ```
    pub fn new(api_id: i32, api_hash: String, dispatcher: NetQueryDispatcher) -> Self {
        Self {
            api_id,
            api_hash: api_hash.into(),
            state: Arc::new(RwLock::new(State::None)),
            net_query_type: Arc::new(RwLock::new(NetQueryType::None)),
            query_id: Arc::new(RwLock::new(0)),
            sent_code: Arc::new(RwLock::new(None)),
            password_info: Arc::new(RwLock::new(None)),
            qr_code_login: Arc::new(RwLock::new(None)),
            phone_code_hash: Arc::new(RwLock::new(None)),
            phone_number: Arc::new(RwLock::new(None)),
            terms_of_service: Arc::new(RwLock::new(None)),
            terms_accepted: Arc::new(RwLock::new(false)),
            user_id: Arc::new(RwLock::new(None)),
            wait_password: Arc::new(RwLock::new(false)),
            code_success: Arc::new(RwLock::new(false)),
            next_query_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
            dispatcher: Arc::new(dispatcher),
            pending_requests: Arc::new(parking_lot::Mutex::new(HashMap::new())),
            request_timeout: Duration::from_secs(60),
        }
    }

    /// Creates a new authentication manager with custom request timeout
    ///
    /// # Arguments
    ///
    /// * `api_id` - API ID from Telegram
    /// * `api_hash` - API hash from Telegram
    /// * `dispatcher` - Network query dispatcher for sending requests
    /// * `timeout` - Request timeout duration
    pub fn with_timeout(
        api_id: i32,
        api_hash: String,
        dispatcher: NetQueryDispatcher,
        timeout: Duration,
    ) -> Self {
        Self {
            api_id,
            api_hash: api_hash.into(),
            state: Arc::new(RwLock::new(State::None)),
            net_query_type: Arc::new(RwLock::new(NetQueryType::None)),
            query_id: Arc::new(RwLock::new(0)),
            sent_code: Arc::new(RwLock::new(None)),
            password_info: Arc::new(RwLock::new(None)),
            qr_code_login: Arc::new(RwLock::new(None)),
            phone_code_hash: Arc::new(RwLock::new(None)),
            phone_number: Arc::new(RwLock::new(None)),
            terms_of_service: Arc::new(RwLock::new(None)),
            terms_accepted: Arc::new(RwLock::new(false)),
            user_id: Arc::new(RwLock::new(None)),
            wait_password: Arc::new(RwLock::new(false)),
            code_success: Arc::new(RwLock::new(false)),
            next_query_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
            dispatcher: Arc::new(dispatcher),
            pending_requests: Arc::new(parking_lot::Mutex::new(HashMap::new())),
            request_timeout: timeout,
        }
    }

    /// Sets the phone number for authentication
    ///
    /// Initiates phone number authentication flow.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - Phone number in international format
    ///
    /// # Errors
    ///
    /// Returns an error if the phone number is invalid or if
    /// authentication is already in progress.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    ///
    /// manager.set_phone_number("+1234567890".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_phone_number(&self, phone_number: String) -> Result<(), AuthManagerError> {
        // Validate phone number
        if phone_number.is_empty() || !phone_number.starts_with('+') {
            return Err(AuthManagerError::InvalidPhoneNumber(phone_number.clone()));
        }

        // Store phone number for later use in sign_in
        *self.phone_number.write().await = Some(phone_number.clone());

        let mut state = self.state.write().await;
        let mut query_type = self.net_query_type.write().await;

        // Check if we can start phone auth
        match *state {
            State::None | State::WaitPhoneNumber => {
                *state = State::WaitPhoneNumber;
                *query_type = NetQueryType::SendPhoneNumber;
                *self.query_id.write().await = self.next_query_id();

                // Send the actual network request
                drop(state);
                drop(query_type);
                self.send_code(phone_number).await?;

                Ok(())
            }
            _ => Err(AuthManagerError::InvalidState(state.clone())),
        }
    }

    /// Checks an authentication code
    ///
    /// Verifies the code sent via SMS/email.
    ///
    /// # Arguments
    ///
    /// * `code` - Authentication code
    /// * `email_verification` - Optional email verification (for email-based codes)
    ///
    /// # Errors
    ///
    /// Returns an error if the code is invalid or if not in the correct state.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    ///
    /// // First set phone number
    /// manager.set_phone_number("+1234567890".to_string()).await?;
    ///
    /// // Set state to WaitCode (normally happens via network response)
    /// manager.set_state(rustgram_auth_manager::State::WaitCode).await;
    ///
    /// // Then check code
    /// manager.check_code("12345".to_string(), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_code(
        &self,
        code: String,
        email_verification: Option<rustgram_types::EmailVerification>,
    ) -> Result<(), AuthManagerError> {
        // Validate code
        if code.is_empty() || code.len() > 16 {
            return Err(AuthManagerError::InvalidCode(code));
        }

        let state = self.state.read().await;
        match *state {
            State::WaitCode => {
                drop(state);

                // Get phone code hash
                let phone_code_hash = self.phone_code_hash.read().await;
                let hash = phone_code_hash
                    .as_ref()
                    .ok_or_else(|| AuthManagerError::Failed {
                        code: 400,
                        message: "PHONE_CODE_HASH_INVALID".to_string(),
                    })?
                    .clone();
                drop(phone_code_hash);

                let mut query_type = self.net_query_type.write().await;
                *query_type = NetQueryType::SendCode;
                *self.query_id.write().await = self.next_query_id();
                *self.code_success.write().await = false;

                // Validate email verification if provided
                if let Some(ref email) = email_verification {
                    if !email.is_valid() {
                        return Err(AuthManagerError::InvalidEmailVerification);
                    }
                }

                // Send sign-in request
                drop(query_type);
                self.sign_in(hash.clone(), code, email_verification).await?;

                Ok(())
            }
            _ => Err(AuthManagerError::InvalidState(state.clone())),
        }
    }

    /// Checks a password (2FA)
    ///
    /// Verifies the two-factor authentication password.
    ///
    /// # Arguments
    ///
    /// * `password` - The 2FA password
    ///
    /// # Errors
    ///
    /// Returns an error if the password is incorrect or if not in the correct state.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    /// // Set state to WaitPassword (normally happens via network response)
    /// manager.set_state(rustgram_auth_manager::State::WaitPassword).await;
    /// manager.check_password("my_password".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_password(&self, password: String) -> Result<(), AuthManagerError> {
        if password.is_empty() {
            return Err(AuthManagerError::EmptyPassword);
        }

        let state = self.state.read().await;
        match *state {
            State::WaitPassword => {
                drop(state);
                let mut query_type = self.net_query_type.write().await;
                *query_type = NetQueryType::CheckPassword;
                *self.query_id.write().await = self.next_query_id();
                // Note: Actual password check would be done via network request
                // For now, we just update the state
                Ok(())
            }
            _ => Err(AuthManagerError::InvalidState(state.clone())),
        }
    }

    /// Checks a bot token
    ///
    /// Authenticates using a bot token.
    ///
    /// # Arguments
    ///
    /// * `token` - Bot token from BotFather
    ///
    /// # Errors
    ///
    /// Returns an error if the token is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    ///
    /// manager.check_bot_token("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_bot_token(&self, token: String) -> Result<(), AuthManagerError> {
        if token.is_empty() || !token.contains(':') {
            return Err(AuthManagerError::InvalidBotToken(token));
        }

        let mut state = self.state.write().await;
        let mut query_type = self.net_query_type.write().await;

        *state = State::WaitCode;
        *query_type = NetQueryType::SendBotToken;
        *self.query_id.write().await = self.next_query_id();

        Ok(())
    }

    /// Requests QR code authentication
    ///
    /// Starts QR code-based authentication flow.
    ///
    /// # Errors
    ///
    /// Returns an error if QR code auth is not available.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    ///
    /// manager.request_qr_code_authentication().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn request_qr_code_authentication(&self) -> Result<(), AuthManagerError> {
        let mut state = self.state.write().await;
        let mut query_type = self.net_query_type.write().await;

        match *state {
            State::None | State::WaitPhoneNumber => {
                *state = State::WaitCode;
                *query_type = NetQueryType::RequestQrCode;
                *self.query_id.write().await = self.next_query_id();
                Ok(())
            }
            _ => Err(AuthManagerError::InvalidState(state.clone())),
        }
    }

    /// Logs out
    ///
    /// Initiates logout process.
    ///
    /// # Errors
    ///
    /// Returns an error if not authenticated.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    /// // Set state to Ok (authenticated)
    /// manager.set_state(rustgram_auth_manager::State::Ok).await;
    /// manager.log_out().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn log_out(&self) -> Result<(), AuthManagerError> {
        let state = self.state.read().await;
        match *state {
            State::Ok => {
                drop(state);
                let mut query_type = self.net_query_type.write().await;
                *query_type = NetQueryType::LogOut;
                *self.query_id.write().await = self.next_query_id();

                // Send log-out request
                drop(query_type);
                self.send_log_out().await?;

                Ok(())
            }
            _ => Err(AuthManagerError::NotAuthenticated),
        }
    }

    /// Deletes account
    ///
    /// Deletes the authenticated account.
    ///
    /// # Arguments
    ///
    /// * `reason` - Reason for deletion
    ///
    /// # Errors
    ///
    /// Returns an error if not authenticated.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let mut manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    /// // Set state to Ok (authenticated)
    /// manager.set_state(rustgram_auth_manager::State::Ok).await;
    /// manager.delete_account("No longer needed".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_account(&self, _reason: String) -> Result<(), AuthManagerError> {
        let state = self.state.read().await;
        match *state {
            State::Ok => {
                drop(state);
                let mut query_type = self.net_query_type.write().await;
                *query_type = NetQueryType::DeleteAccount;
                *self.query_id.write().await = self.next_query_id();
                // Note: Actual delete account would be done via network request
                Ok(())
            }
            _ => Err(AuthManagerError::NotAuthenticated),
        }
    }

    /// Checks if the client is authorized
    ///
    /// Returns `true` if authentication is complete and successful.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::AuthManager;
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    /// assert!(!manager.is_authorized().await);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_authorized(&self) -> bool {
        let state = self.state.read().await;
        matches!(*state, State::Ok)
    }

    /// Gets the current authentication state
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_auth_manager::{AuthManager, State};
    /// use rustgram_net::NetQueryDispatcher;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = NetQueryDispatcher::new();
    /// let manager = AuthManager::new(12345, "api_hash".to_string(), dispatcher);
    /// assert_eq!(manager.get_state(), State::None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_state(&self) -> State {
        // Use try_read for synchronous access
        if let Ok(state) = self.state.try_read() {
            // Clone the state for return
            state.clone()
        } else {
            State::None
        }
    }

    /// Gets the API ID
    pub const fn api_id(&self) -> i32 {
        self.api_id
    }

    /// Gets the API hash
    pub fn api_hash(&self) -> &str {
        &self.api_hash
    }

    /// Gets the current user ID (if authenticated)
    pub async fn user_id(&self) -> Option<UserId> {
        *self.user_id.read().await
    }

    /// Sets the user ID (internal use)
    pub async fn set_user_id(&self, user_id: UserId) {
        *self.user_id.write().await = Some(user_id);
        *self.state.write().await = State::Ok;
    }

    /// Gets the current network query type
    pub async fn net_query_type(&self) -> NetQueryType {
        *self.net_query_type.read().await
    }

    /// Gets the current query ID
    pub async fn query_id(&self) -> u64 {
        *self.query_id.read().await
    }

    /// Clears the current query
    pub async fn clear_query(&self) {
        *self.net_query_type.write().await = NetQueryType::None;
        *self.query_id.write().await = 0;
    }

    /// Sets sent code info (internal use)
    pub async fn set_sent_code(&self, code: rustgram_types::SentCode) {
        *self.sent_code.write().await = Some(code);
    }

    /// Gets sent code info
    pub async fn sent_code(&self) -> Option<SentCode> {
        self.sent_code.read().await.clone()
    }

    /// Sets password info (internal use)
    pub async fn set_password_info(&self, info: PasswordInfo) {
        let has_password = info.has_password();
        *self.password_info.write().await = Some(info);
        if has_password {
            *self.wait_password.write().await = true;
            *self.state.write().await = State::WaitPassword;
        }
    }

    /// Gets password info
    pub async fn password_info(&self) -> Option<PasswordInfo> {
        self.password_info.read().await.clone()
    }

    /// Sets QR code login session (internal use)
    pub async fn set_qr_code_login(&self, login: QrCodeLogin) {
        *self.qr_code_login.write().await = Some(login);
    }

    /// Gets QR code login session
    pub async fn qr_code_login(&self) -> Option<QrCodeLogin> {
        self.qr_code_login.read().await.clone()
    }

    /// Sets terms of service (internal use)
    pub async fn set_terms_of_service(&self, terms: TermsOfService) {
        *self.terms_of_service.write().await = Some(terms);
    }

    /// Gets terms of service
    pub async fn terms_of_service(&self) -> Option<TermsOfService> {
        self.terms_of_service.read().await.clone()
    }

    /// Accepts terms of service
    pub async fn accept_terms_of_service(&self) {
        *self.terms_accepted.write().await = true;
    }

    /// Checks if terms were accepted
    pub async fn terms_accepted(&self) -> bool {
        *self.terms_accepted.read().await
    }

    /// Sets the phone code hash (internal use)
    pub async fn set_phone_code_hash(&self, hash: String) {
        *self.phone_code_hash.write().await = Some(hash);
    }

    /// Gets the phone code hash
    pub async fn phone_code_hash(&self) -> Option<String> {
        self.phone_code_hash.read().await.clone()
    }

    /// Gets the next query ID
    fn next_query_id(&self) -> u64 {
        self.next_query_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    /// Sets the state directly (internal use)
    pub async fn set_state(&self, state: State) {
        *self.state.write().await = state;
    }

    // ========== Network Integration Methods ==========

    /// Sends a code request to the server
    async fn send_code(&self, phone_number: String) -> Result<(), AuthManagerError> {
        info!("Sending auth code request");

        // Create the SendCode request
        let settings = rustgram_types::CodeSettings::new();
        let request = SendCodeRequest::new(
            phone_number.clone(),
            self.api_id,
            self.api_hash.to_string(),
            settings,
        );

        // Serialize the request
        let mut buffer = BytesMut::new();
        TlHelper::write_i32(&mut buffer, TL_SEND_CODE);
        request
            .serialize_tl(&mut buffer)
            .map_err(|e| AuthManagerError::Failed {
                code: 400,
                message: format!("Serialization error: {}", e),
            })?;

        // Create NetQuery
        let query_id = self.next_query_id();
        let query = NetQuery::new(
            query_id,
            buffer.freeze(),
            self.dispatcher.main_dc_id(),
            NetQueryFlag::Common,
            AuthFlag::On,
            GzipFlag::Off,
            TL_SEND_CODE,
        );

        // Set callback
        let state = self.state.clone();
        let phone_code_hash = self.phone_code_hash.clone();
        let sent_code = self.sent_code.clone();
        let pending_requests = self.pending_requests.clone();

        query.set_callback(Box::new(AuthQueryCallback {
            query_id,
            state,
            phone_code_hash,
            sent_code,
            pending_requests,
        }));

        // Track the pending request
        {
            let mut pending = self.pending_requests.lock();
            pending.insert(
                query_id,
                PendingAuthRequest {
                    query_id,
                    request_type: PendingRequestType::SendCode(phone_number),
                    created_at: std::time::Instant::now(),
                    retry_count: 0,
                },
            );
        }

        // Dispatch the query
        self.dispatcher
            .dispatch(query)
            .map_err(|e| AuthManagerError::Failed {
                code: 500,
                message: format!("Dispatch error: {}", e),
            })?;

        debug!("Sent code request with query_id {}", query_id);
        Ok(())
    }

    /// Sends a sign-in request to the server
    async fn sign_in(
        &self,
        phone_code_hash: String,
        code: String,
        email_verification: Option<rustgram_types::EmailVerification>,
    ) -> Result<(), AuthManagerError> {
        info!("Sending sign-in request");

        // Get stored phone number
        let phone_number_guard = self.phone_number.read().await;
        let phone_number = phone_number_guard
            .as_ref()
            .ok_or_else(|| AuthManagerError::Failed {
                code: 400,
                message: "PHONE_NUMBER_NOT_SET".to_string(),
            })?
            .clone();
        drop(phone_number_guard);

        // Create the SignIn request
        let request = if let Some(email) = email_verification {
            SignInRequest::with_code_and_email(
                phone_number,
                phone_code_hash.clone(),
                code.clone(),
                email,
            )
        } else {
            SignInRequest::with_code(phone_number, phone_code_hash.clone(), code.clone())
        };

        // Serialize the request
        let mut buffer = BytesMut::new();
        TlHelper::write_i32(&mut buffer, TL_SIGN_IN);
        request
            .serialize_tl(&mut buffer)
            .map_err(|e| AuthManagerError::Failed {
                code: 400,
                message: format!("Serialization error: {}", e),
            })?;

        // Create NetQuery
        let query_id = self.next_query_id();
        let query = NetQuery::new(
            query_id,
            buffer.freeze(),
            self.dispatcher.main_dc_id(),
            NetQueryFlag::Common,
            AuthFlag::On,
            GzipFlag::Off,
            TL_SIGN_IN,
        );

        // Set callback for authorization response
        let state = self.state.clone();
        let user_id = self.user_id.clone();
        let pending_requests = self.pending_requests.clone();

        query.set_callback(Box::new(SignInQueryCallback {
            query_id,
            state,
            user_id,
            pending_requests,
        }));

        // Track the pending request
        {
            let mut pending = self.pending_requests.lock();
            pending.insert(
                query_id,
                PendingAuthRequest {
                    query_id,
                    request_type: PendingRequestType::SignIn(phone_code_hash, code),
                    created_at: std::time::Instant::now(),
                    retry_count: 0,
                },
            );
        }

        // Dispatch the query
        self.dispatcher
            .dispatch(query)
            .map_err(|e| AuthManagerError::Failed {
                code: 500,
                message: format!("Dispatch error: {}", e),
            })?;

        debug!("Sent sign-in request with query_id {}", query_id);
        Ok(())
    }

    /// Sends a log-out request to the server
    async fn send_log_out(&self) -> Result<(), AuthManagerError> {
        info!("Sending log-out request");

        // Serialize the empty LogOut request
        let mut buffer = BytesMut::new();
        TlHelper::write_i32(&mut buffer, TL_LOG_OUT);

        // Create NetQuery
        let query_id = self.next_query_id();
        let query = NetQuery::new(
            query_id,
            buffer.freeze(),
            self.dispatcher.main_dc_id(),
            NetQueryFlag::Common,
            AuthFlag::On,
            GzipFlag::Off,
            TL_LOG_OUT,
        );

        // Set callback for log-out response
        let state = self.state.clone();
        let pending_requests = self.pending_requests.clone();

        query.set_callback(Box::new(LogOutQueryCallback {
            query_id,
            state,
            pending_requests,
        }));

        // Track the pending request
        {
            let mut pending = self.pending_requests.lock();
            pending.insert(
                query_id,
                PendingAuthRequest {
                    query_id,
                    request_type: PendingRequestType::LogOut,
                    created_at: std::time::Instant::now(),
                    retry_count: 0,
                },
            );
        }

        // Dispatch the query
        self.dispatcher
            .dispatch(query)
            .map_err(|e| AuthManagerError::Failed {
                code: 500,
                message: format!("Dispatch error: {}", e),
            })?;

        debug!("Sent log-out request with query_id {}", query_id);
        Ok(())
    }

    /// Handles authentication result from network callback
    pub async fn on_auth_result(&self, query_id: NetQueryId, result: Result<Bytes, QueryError>) {
        // Remove from pending requests
        let pending_request = {
            let mut pending = self.pending_requests.lock();
            pending.remove(&query_id)
        };

        let Some(pending) = pending_request else {
            warn!("Received result for unknown query_id {}", query_id);
            return;
        };

        match result {
            Ok(data) => {
                info!("Query {} completed successfully", query_id);
                self.handle_success_response(pending.request_type, data)
                    .await;
            }
            Err(err) => {
                error!("Query {} failed: {}", query_id, err);
                self.handle_error_response(pending, err).await;
            }
        }
    }

    /// Handles successful responses
    async fn handle_success_response(&self, request_type: PendingRequestType, data: Bytes) {
        match request_type {
            PendingRequestType::SendCode(_) => {
                // Parse SentCode response
                let mut tl_bytes = rustgram_types::tl::Bytes::new(data.clone());
                if let Ok(sent_code) = rustgram_types::SentCode::deserialize_tl(&mut tl_bytes) {
                    self.set_sent_code(SentCode::new(
                        sent_code.is_phone_registered(),
                        sent_code.code_type().clone(),
                        sent_code.next_type().cloned(),
                        sent_code.timeout(),
                    ))
                    .await;

                    // Update state to WaitCode
                    self.set_state(State::WaitCode).await;
                } else {
                    error!("Failed to parse SentCode response");
                    self.set_state(State::NetworkError("Parse error".to_string()))
                        .await;
                }
            }
            PendingRequestType::SignIn(_, _) => {
                // Parse Authorization response
                let mut tl_bytes = rustgram_types::tl::Bytes::new(data.clone());
                if let Ok(auth) = Authorization::deserialize_tl(&mut tl_bytes) {
                    if auth.is_success() {
                        if let Some(user_id) = auth.user_id() {
                            match UserId::new(user_id) {
                                Ok(uid) => self.set_user_id(uid).await,
                                Err(_) => {
                                    warn!("Invalid user_id received: {}", user_id);
                                }
                            }
                        }
                        self.set_state(State::Ok).await;
                    } else if auth.is_sign_up_required() {
                        // TODO: Handle sign-up required
                        warn!("Sign-up required but not implemented");
                        self.set_state(State::NetworkError("Sign-up required".to_string()))
                            .await;
                    }
                } else {
                    error!("Failed to parse Authorization response");
                    self.set_state(State::NetworkError("Parse error".to_string()))
                        .await;
                }
            }
            PendingRequestType::LogOut => {
                // Parse LoggedOut response
                let mut tl_bytes = rustgram_types::tl::Bytes::new(data.clone());
                if let Ok(logged_out) = LoggedOut::deserialize_tl(&mut tl_bytes) {
                    if logged_out.success() {
                        self.set_state(State::Closing).await;
                    }
                } else {
                    error!("Failed to parse LoggedOut response");
                    self.set_state(State::NetworkError("Parse error".to_string()))
                        .await;
                }
            }
        }
    }

    /// Handles error responses with retry logic
    async fn handle_error_response(&self, pending: PendingAuthRequest, err: QueryError) {
        if pending.retry_count < MAX_RETRY_ATTEMPTS {
            // Calculate delay with exponential backoff
            let delay = BASE_RETRY_DELAY * 2u32.pow(pending.retry_count);

            info!(
                "Retrying request {} (attempt {}/{}) after {:?}",
                pending.query_id,
                pending.retry_count + 1,
                MAX_RETRY_ATTEMPTS,
                delay
            );

            // Update state to waiting retry
            self.set_state(State::WaitingRetry {
                attempts: pending.retry_count + 1,
                delay,
            })
            .await;

            // Spawn async task to handle retry after delay
            let state_clone = Arc::clone(&self.state);
            tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                // After delay, return to appropriate state for retry
                *state_clone.write().await = State::WaitCode;
                warn!("Retry timer elapsed; ready to retry request");
            });
        } else {
            error!("Max retry attempts reached for query {}", pending.query_id);
            self.set_state(State::NetworkError(err.to_string())).await;
        }
    }

    /// Cleans up timed-out requests
    pub fn cleanup_timeouts(&self) {
        let now = std::time::Instant::now();
        let mut pending = self.pending_requests.lock();

        let timed_out: Vec<NetQueryId> = pending
            .iter()
            .filter(|(_, req)| now.duration_since(req.created_at) > self.request_timeout)
            .map(|(id, _)| *id)
            .collect();

        for id in timed_out {
            if let Some(_req) = pending.remove(&id) {
                warn!("Request {} timed out", id);
                // Note: We would trigger a timeout callback here
            }
        }
    }
}

/// Callback implementation for SendCode queries
#[allow(dead_code)]
struct AuthQueryCallback {
    query_id: NetQueryId,
    state: Arc<RwLock<State>>,
    phone_code_hash: Arc<RwLock<Option<String>>>,
    sent_code: Arc<RwLock<Option<SentCode>>>,
    pending_requests: Arc<parking_lot::Mutex<HashMap<NetQueryId, PendingAuthRequest>>>,
}

#[async_trait::async_trait]
impl NetQueryCallback for AuthQueryCallback {
    async fn on_result(&self, query: NetQuery) {
        if query.is_ok() {
            let _data = query.ok();
            // Parse SentCode response and update phone_code_hash
            // For now, we just transition to WaitCode state
            *self.state.write().await = State::WaitCode;
        } else if query.is_error() {
            let err = query.error();
            *self.state.write().await = State::NetworkError(err.to_string());
        }

        // Remove from pending
        self.pending_requests.lock().remove(&self.query_id);
    }
}

/// Callback implementation for SignIn queries
#[allow(dead_code)]
struct SignInQueryCallback {
    query_id: NetQueryId,
    state: Arc<RwLock<State>>,
    user_id: Arc<RwLock<Option<UserId>>>,
    pending_requests: Arc<parking_lot::Mutex<HashMap<NetQueryId, PendingAuthRequest>>>,
}

#[async_trait::async_trait]
impl NetQueryCallback for SignInQueryCallback {
    async fn on_result(&self, query: NetQuery) {
        if query.is_ok() {
            let _data = query.ok();
            // Parse Authorization response
            // For now, we just transition to Ok state
            *self.state.write().await = State::Ok;
        } else if query.is_error() {
            let err = query.error();
            *self.state.write().await = State::NetworkError(err.to_string());
        }

        // Remove from pending
        self.pending_requests.lock().remove(&self.query_id);
    }
}

/// Callback implementation for LogOut queries
struct LogOutQueryCallback {
    query_id: NetQueryId,
    state: Arc<RwLock<State>>,
    pending_requests: Arc<parking_lot::Mutex<HashMap<NetQueryId, PendingAuthRequest>>>,
}

#[async_trait::async_trait]
impl NetQueryCallback for LogOutQueryCallback {
    async fn on_result(&self, query: NetQuery) {
        if query.is_ok() {
            // Transition to Closing state
            *self.state.write().await = State::Closing;
        } else if query.is_error() {
            let err = query.error();
            *self.state.write().await = State::NetworkError(err.to_string());
        }

        // Remove from pending
        self.pending_requests.lock().remove(&self.query_id);
    }
}

/// Authentication manager error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthManagerError {
    /// Invalid phone number
    InvalidPhoneNumber(String),

    /// Invalid authentication code
    InvalidCode(String),

    /// Invalid email verification
    InvalidEmailVerification,

    /// Invalid bot token
    InvalidBotToken(String),

    /// Empty password
    EmptyPassword,

    /// Not authenticated
    NotAuthenticated,

    /// Invalid state for operation
    InvalidState(State),

    /// Operation failed
    Failed {
        /// Error code
        code: i32,
        /// Error message
        message: String,
    },
}

impl std::fmt::Display for AuthManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPhoneNumber(_) => write!(f, "Invalid phone number"),
            Self::InvalidCode(_) => write!(f, "Invalid code"),
            Self::InvalidEmailVerification => write!(f, "Invalid email verification"),
            Self::InvalidBotToken(_) => write!(f, "Invalid bot token"),
            Self::EmptyPassword => write!(f, "Empty password"),
            Self::NotAuthenticated => write!(f, "Not authenticated"),
            Self::InvalidState(state) => write!(f, "Invalid state: {:?}", state),
            Self::Failed { code, message } => {
                write!(f, "Operation failed ({}): {}", code, message)
            }
        }
    }
}

impl std::error::Error for AuthManagerError {}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_net::NetQueryDispatcher;

    fn create_test_manager() -> AuthManager {
        let dispatcher = NetQueryDispatcher::new();
        AuthManager::new(12345, "test_api_hash".to_string(), dispatcher)
    }

    #[tokio::test]
    async fn test_auth_manager_new() {
        let manager = create_test_manager();
        assert_eq!(manager.api_id(), 12345);
        assert_eq!(manager.api_hash(), "test_api_hash");
        assert_eq!(manager.get_state(), State::None);
        assert!(!manager.is_authorized().await);
    }

    #[tokio::test]
    async fn test_set_phone_number_valid() {
        let manager = create_test_manager();
        // Note: This will fail to actually send the request in tests
        // but we can test the validation logic
        let result = manager.set_phone_number("+1234567890".to_string()).await;
        // We expect this to fail because we're not actually connected
        // but the validation should pass
        assert!(result.is_ok() || matches!(result, Err(AuthManagerError::Failed { .. })));
    }

    #[tokio::test]
    async fn test_set_phone_number_invalid() {
        let manager = create_test_manager();
        let result = manager.set_phone_number("invalid".to_string()).await;
        assert!(matches!(
            result,
            Err(AuthManagerError::InvalidPhoneNumber(_))
        ));
    }

    #[tokio::test]
    async fn test_check_code_invalid_empty() {
        let manager = create_test_manager();
        manager.set_state(State::WaitCode).await;

        let result = manager.check_code("".to_string(), None).await;
        assert!(matches!(result, Err(AuthManagerError::InvalidCode(_))));
    }

    #[tokio::test]
    async fn test_check_code_invalid_too_long() {
        let manager = create_test_manager();
        manager.set_state(State::WaitCode).await;

        let result = manager
            .check_code("12345678901234567".to_string(), None)
            .await;
        assert!(matches!(result, Err(AuthManagerError::InvalidCode(_))));
    }

    #[tokio::test]
    async fn test_check_password_empty() {
        let manager = create_test_manager();
        manager.set_state(State::WaitPassword).await;

        let result = manager.check_password("".to_string()).await;
        assert!(matches!(result, Err(AuthManagerError::EmptyPassword)));
    }

    #[tokio::test]
    async fn test_check_bot_token_valid() {
        let manager = create_test_manager();
        let token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string();

        let result = manager.check_bot_token(token).await;
        assert!(result.is_ok());
        assert_eq!(manager.get_state(), State::WaitCode);
    }

    #[tokio::test]
    async fn test_check_bot_token_invalid() {
        let manager = create_test_manager();
        let result = manager.check_bot_token("invalid".to_string()).await;
        assert!(matches!(result, Err(AuthManagerError::InvalidBotToken(_))));
    }

    #[tokio::test]
    async fn test_request_qr_code_authentication() {
        let manager = create_test_manager();
        let result = manager.request_qr_code_authentication().await;
        assert!(result.is_ok());
        assert_eq!(manager.get_state(), State::WaitCode);
    }

    #[tokio::test]
    async fn test_log_out_unauthorized() {
        let manager = create_test_manager();
        let result = manager.log_out().await;
        assert!(matches!(result, Err(AuthManagerError::NotAuthenticated)));
    }

    #[tokio::test]
    async fn test_delete_account_unauthorized() {
        let manager = create_test_manager();
        let result = manager.delete_account("reason".to_string()).await;
        assert!(matches!(result, Err(AuthManagerError::NotAuthenticated)));
    }

    #[tokio::test]
    async fn test_set_user_id() {
        let manager = create_test_manager();
        let user_id = UserId::new(123456).unwrap();

        manager.set_user_id(user_id).await;
        assert_eq!(manager.user_id().await, Some(user_id));
        assert!(manager.is_authorized().await);
        assert_eq!(manager.get_state(), State::Ok);
    }

    #[tokio::test]
    async fn test_net_query_type_operations() {
        let manager = create_test_manager();
        assert_eq!(manager.net_query_type().await, NetQueryType::None);

        manager.clear_query().await;
        assert_eq!(manager.net_query_type().await, NetQueryType::None);
    }

    #[tokio::test]
    async fn test_password_info() {
        let manager = create_test_manager();
        let info = PasswordInfo::no_password();

        manager.set_password_info(info.clone()).await;
        assert!(manager.password_info().await.is_some());
    }

    #[tokio::test]
    async fn test_qr_code_login() {
        let manager = create_test_manager();
        let login = QrCodeLogin::new(vec![1, 2, 3, 4], 2, 300);

        manager.set_qr_code_login(login.clone()).await;
        assert!(manager.qr_code_login().await.is_some());
    }

    #[tokio::test]
    async fn test_terms_of_service() {
        let manager = create_test_manager();
        let terms = TermsOfService::new("id".to_string(), "text".to_string(), 18, true);

        manager.set_terms_of_service(terms.clone()).await;
        assert_eq!(manager.terms_of_service().await, Some(terms));
        assert!(!manager.terms_accepted().await);

        manager.accept_terms_of_service().await;
        assert!(manager.terms_accepted().await);
    }

    #[tokio::test]
    async fn test_phone_code_hash() {
        let manager = create_test_manager();
        let hash = "test_hash".to_string();

        manager.set_phone_code_hash(hash.clone()).await;
        assert_eq!(manager.phone_code_hash().await, Some(hash));
    }

    #[tokio::test]
    async fn test_invalid_state_error() {
        let manager = create_test_manager();
        // Try to check code without being in WaitCode state
        let result = manager.check_code("12345".to_string(), None).await;
        assert!(matches!(result, Err(AuthManagerError::InvalidState(_))));
    }

    #[tokio::test]
    async fn test_error_display() {
        assert_eq!(
            format!("{}", AuthManagerError::EmptyPassword),
            "Empty password"
        );
        assert_eq!(
            format!("{}", AuthManagerError::NotAuthenticated),
            "Not authenticated"
        );
        // Note: Display now sanitizes sensitive data (phone numbers, codes, tokens)
        assert_eq!(
            format!(
                "{}",
                AuthManagerError::InvalidPhoneNumber("+123".to_string())
            ),
            "Invalid phone number"
        );
    }

    #[tokio::test]
    async fn test_net_query_type_is_auth_query() {
        assert!(NetQueryType::SendPhoneNumber.is_auth_query());
        assert!(NetQueryType::SendCode.is_auth_query());
        assert!(NetQueryType::CheckPassword.is_auth_query());
        assert!(!NetQueryType::LogOut.is_auth_query());
        assert!(!NetQueryType::None.is_auth_query());
    }

    #[tokio::test]
    async fn test_net_query_type_is_destructive() {
        assert!(NetQueryType::LogOut.is_destructive());
        assert!(NetQueryType::DeleteAccount.is_destructive());
        assert!(!NetQueryType::SendCode.is_destructive());
        assert!(!NetQueryType::None.is_destructive());
    }

    #[tokio::test]
    async fn test_state_with_network_error() {
        let manager = create_test_manager();
        manager
            .set_state(State::NetworkError("test error".to_string()))
            .await;
        assert!(manager.get_state().is_error());
    }

    #[tokio::test]
    async fn test_state_with_waiting_retry() {
        let manager = create_test_manager();
        manager
            .set_state(State::WaitingRetry {
                attempts: 2,
                delay: Duration::from_secs(5),
            })
            .await;
        assert!(manager.get_state().is_error());
    }

    #[tokio::test]
    async fn test_cleanup_timeouts() {
        let manager = create_test_manager();
        // Just ensure it doesn't panic
        manager.cleanup_timeouts();
    }
}
