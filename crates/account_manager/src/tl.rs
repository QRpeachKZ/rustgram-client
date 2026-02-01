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

//! TL type stubs for authorization and web authorization.
//!
//! These are stub implementations that will be replaced when the full TL layer is available.

use serde::{Deserialize, Serialize};

/// Stub for TL authorization type.
///
/// TODO: Replace with full TL implementation when available.
///
/// This stub contains all fields from TDLib's authorization TL type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Authorization {
    /// Authorization hash (session ID).
    pub hash: i64,

    /// Whether this is the current session.
    pub current: bool,

    /// Whether 2FA password is pending.
    pub password_pending: bool,

    /// Whether session is unconfirmed.
    pub unconfirmed: bool,

    /// Whether encrypted requests are disabled.
    pub encrypted_requests_disabled: bool,

    /// Whether call requests are disabled.
    pub call_requests_disabled: bool,

    /// API ID.
    pub api_id: i32,

    /// Application name.
    pub app_name: String,

    /// Application version.
    pub app_version: String,

    /// Whether this is an official application.
    pub official_app: bool,

    /// Device model.
    pub device_model: String,

    /// Platform.
    pub platform: String,

    /// System version.
    pub system_version: String,

    /// Date created (Unix timestamp).
    pub date_created: i32,

    /// Date active (Unix timestamp).
    pub date_active: i32,

    /// IP address.
    pub ip: String,

    /// Country.
    pub country: String,
}

impl Authorization {
    /// Creates a new authorization stub.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::tl::Authorization;
    ///
    /// let auth = Authorization::new(
    ///     123456789,
    ///     true,
    ///     false,
    ///     "Telegram",
    ///     "10.0",
    ///     "iPhone",
    ///     "iOS",
    ///     "17.0"
    /// );
    /// ```
    pub fn new(
        hash: i64,
        current: bool,
        password_pending: bool,
        app_name: &str,
        app_version: &str,
        device_model: &str,
        platform: &str,
        system_version: &str,
    ) -> Self {
        Self {
            hash,
            current,
            password_pending,
            unconfirmed: false,
            encrypted_requests_disabled: false,
            call_requests_disabled: false,
            api_id: 0,
            app_name: app_name.to_string(),
            app_version: app_version.to_string(),
            official_app: false,
            device_model: device_model.to_string(),
            platform: platform.to_string(),
            system_version: system_version.to_string(),
            date_created: 0,
            date_active: 0,
            ip: String::new(),
            country: String::new(),
        }
    }
}

/// Stub for TL webAuthorization type.
///
/// TODO: Replace with full TL implementation when available.
///
/// This stub contains all fields from TDLib's webAuthorization TL type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebAuthorization {
    /// Authorization hash (website ID).
    pub hash: i64,

    /// Domain name.
    pub domain: String,

    /// Bot user ID.
    pub bot_id: i64,

    /// Browser.
    pub browser: String,

    /// Platform.
    pub platform: String,

    /// Date created (Unix timestamp).
    pub date_created: i32,

    /// Date active (Unix timestamp).
    pub date_active: i32,

    /// IP address.
    pub ip: String,

    /// Region.
    pub region: String,
}

impl WebAuthorization {
    /// Creates a new web authorization stub.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::tl::WebAuthorization;
    ///
    /// let web_auth = WebAuthorization::new(
    ///     987654321,
    ///     "example.com",
    ///     123456789,
    ///     "Chrome",
    ///     "Windows"
    /// );
    /// ```
    pub fn new(hash: i64, domain: &str, bot_id: i64, browser: &str, platform: &str) -> Self {
        Self {
            hash,
            domain: domain.to_string(),
            bot_id,
            browser: browser.to_string(),
            platform: platform.to_string(),
            date_created: 0,
            date_active: 0,
            ip: String::new(),
            region: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_new() {
        let auth = Authorization::new(
            123456789, true, false, "Telegram", "10.0", "iPhone", "iOS", "17.0",
        );

        assert_eq!(auth.hash, 123456789);
        assert!(auth.current);
        assert!(!auth.password_pending);
        assert_eq!(auth.app_name, "Telegram");
        assert_eq!(auth.app_version, "10.0");
        assert_eq!(auth.device_model, "iPhone");
        assert_eq!(auth.platform, "iOS");
        assert_eq!(auth.system_version, "17.0");
    }

    #[test]
    fn test_web_authorization_new() {
        let web_auth =
            WebAuthorization::new(987654321, "example.com", 123456789, "Chrome", "Windows");

        assert_eq!(web_auth.hash, 987654321);
        assert_eq!(web_auth.domain, "example.com");
        assert_eq!(web_auth.bot_id, 123456789);
        assert_eq!(web_auth.browser, "Chrome");
        assert_eq!(web_auth.platform, "Windows");
    }

    #[test]
    fn test_authorization_serialization() {
        let auth = Authorization::new(
            123456789, true, false, "Telegram", "10.0", "iPhone", "iOS", "17.0",
        );

        let json = serde_json::to_string(&auth).unwrap();
        let deserialized: Authorization = serde_json::from_str(&json).unwrap();

        assert_eq!(auth, deserialized);
    }

    #[test]
    fn test_web_authorization_serialization() {
        let web_auth =
            WebAuthorization::new(987654321, "example.com", 123456789, "Chrome", "Windows");

        let json = serde_json::to_string(&web_auth).unwrap();
        let deserialized: WebAuthorization = serde_json::from_str(&json).unwrap();

        assert_eq!(web_auth, deserialized);
    }
}
