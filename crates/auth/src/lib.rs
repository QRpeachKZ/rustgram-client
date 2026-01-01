// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Telegram Authentication Module
//!
//! This module handles Telegram authentication flow according to MTProto protocol.
//!
//! ## Reference Implementation
//! - Official TDLib: `references/td/td/telegram/AuthManager.{h,cpp}`
//! - MTProto spec: https://core.telegram.org/mtproto
//!
//! ## Overview
//!
//! The auth module provides authentication state machine that handles:
//! - Phone number authentication
//! - Bot token authentication
//! - QR code authentication
//! - Two-factor authentication (2FA)
//! - Email verification
//! - Password recovery
//!
//! ## State Machine
//!
//! The authentication flow is represented by [`AuthState`]:
//!
//! ```text
//! Idle → WaitingForPhone → WaitingForCode → WaitingForPassword → Authenticated
//!        ↓                    ↓
//!     BotAuth              QRCodeAuth
//! ```
//!
//! ## Examples
//!
//! ```no_run
//! use rustgram_auth::{AuthManager, AuthState};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut manager = AuthManager::new(12345, "api_hash".to_string());
//!
//!     // Start phone authentication
//!     manager.set_phone_number("+1234567890".to_string()).await?;
//!
//!     // Check code was sent
//!     assert!(matches!(manager.state().await, AuthState::WaitingForCode));
//!
//!     // Submit code
//!     manager.check_code("12345".to_string()).await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

/// Authentication state machine
pub mod state;

/// Authentication errors
pub mod error;

/// Code types and helpers
pub mod code;

/// Password authentication (2FA/SRP)
pub mod password;

/// Email verification
pub mod email;

/// QR code authentication
pub mod qr;

/// Authentication manager
pub mod manager;

/// TL types for authentication protocol
pub mod tl;

// Re-exports
pub use code::{SentCode, SentCodeType};
pub use email::{EmailCodeInfo, EmailVerification};
pub use error::{AuthError, AuthResult};
pub use manager::AuthManager;
pub use password::{PasswordInfo, PasswordKdfAlgo};
pub use qr::{QrCodeLogin, QrCodeStatus};
pub use state::AuthState;

/// Current version of authentication protocol
pub const AUTH_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum length for first/last name (server-side limit)
pub const MAX_NAME_LENGTH: usize = 64;

/// Default code timeout in seconds
pub const DEFAULT_CODE_TIMEOUT: u32 = 60;

/// Default password verification timeout in seconds
pub const DEFAULT_PASSWORD_TIMEOUT: u32 = 86400; // 24 hours
