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

//! Type definitions for account management.

use crate::error::{AccountManagerError, Result};
use crate::tl::{Authorization, WebAuthorization};
use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Session type detected from device and application information.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::SessionType;
///
/// let session_type = SessionType::detect("Telegram Web", "Chrome", "iOS", "17.0");
/// assert_eq!(session_type, SessionType::Safari);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionType {
    /// Android device.
    Android,
    /// Generic Apple device.
    Apple,
    /// iPhone.
    Iphone,
    /// iPad.
    Ipad,
    /// Mac.
    Mac,
    /// Windows.
    Windows,
    /// Linux.
    Linux,
    /// Ubuntu.
    Ubuntu,
    /// Chrome browser.
    Chrome,
    /// Firefox browser.
    Firefox,
    /// Safari browser.
    Safari,
    /// Edge browser.
    Edge,
    /// Opera browser.
    Opera,
    /// Vivaldi browser.
    Vivaldi,
    /// Brave browser.
    Brave,
    /// Xbox console.
    Xbox,
    /// Unknown session type.
    Unknown,
}

impl SessionType {
    /// Detects session type from device and application information.
    ///
    /// This implements the TDLib session type detection algorithm.
    ///
    /// # Arguments
    ///
    /// * `app_name` - Application name (e.g., "Telegram Web")
    /// * `device_model` - Device model (e.g., "iPhone", "Chrome")
    /// * `platform` - Platform (e.g., "iOS", "Windows")
    /// * `system_version` - System version (e.g., "17.0", "10.0.19043")
    ///
    /// # Returns
    ///
    /// Detected session type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::SessionType;
    ///
    /// // Web browsers
    /// assert_eq!(SessionType::detect("Telegram Web", "Chrome", "Windows", "10.0"), SessionType::Chrome);
    /// assert_eq!(SessionType::detect("Telegram Web", "Safari", "iOS", "17.0"), SessionType::Safari);
    ///
    /// // Mobile platforms
    /// assert_eq!(SessionType::detect("Telegram", "Pixel 5", "Android", "11"), SessionType::Android);
    /// assert_eq!(SessionType::detect("Telegram", "iPhone", "iOS", "17.0"), SessionType::Iphone);
    ///
    /// // Desktop platforms
    /// assert_eq!(SessionType::detect("Telegram Desktop", "", "Windows", "10.0"), SessionType::Windows);
    /// assert_eq!(SessionType::detect("Telegram Desktop", "", "Ubuntu", "22.04"), SessionType::Ubuntu);
    /// ```
    pub fn detect(
        app_name: &str,
        device_model: &str,
        platform: &str,
        system_version: &str,
    ) -> Self {
        let device_lower = device_model.to_lowercase();
        let platform_lower = platform.to_lowercase();
        let system_lower = system_version.to_lowercase();

        // Check for Xbox first
        if device_lower.contains("xbox") {
            return Self::Xbox;
        }

        // Check if it's a web session
        let is_web = Self::is_web_app(app_name);

        if is_web {
            // Detect browser type
            if device_lower.contains("brave") {
                return Self::Brave;
            } else if device_lower.contains("vivaldi") {
                return Self::Vivaldi;
            } else if device_lower.contains("opera") || device_lower.contains("opr") {
                return Self::Opera;
            } else if device_lower.contains("edg") {
                return Self::Edge;
            } else if device_lower.contains("chrome") {
                return Self::Chrome;
            } else if device_lower.contains("firefox") || device_lower.contains("fxios") {
                return Self::Firefox;
            } else if device_lower.contains("safari") {
                return Self::Safari;
            }
        }

        // Detect mobile/desktop platforms
        if platform_lower.starts_with("android") || system_lower.contains("android") {
            Self::Android
        } else if platform_lower.starts_with("windows") || system_lower.contains("windows") {
            Self::Windows
        } else if platform_lower.starts_with("ubuntu") || system_lower.contains("ubuntu") {
            Self::Ubuntu
        } else if platform_lower.starts_with("linux") || system_lower.contains("linux") {
            Self::Linux
        } else {
            let is_ios = platform_lower.starts_with("ios") || system_lower.contains("ios");
            let is_macos = platform_lower.starts_with("macos") || system_lower.contains("macos");

            if is_ios && device_lower.contains("iphone") {
                Self::Iphone
            } else if is_ios && device_lower.contains("ipad") {
                Self::Ipad
            } else if is_macos && device_lower.contains("mac") {
                Self::Mac
            } else if is_ios || is_macos {
                Self::Apple
            } else {
                Self::Unknown
            }
        }
    }

    /// Checks if the app name indicates a web application.
    fn is_web_app(app_name: &str) -> bool {
        let web_name = "Web";
        let pos = match app_name.find(web_name) {
            Some(p) => p,
            None => return false,
        };

        // Check if the character after "Web" is not a lowercase letter
        let next_char = app_name.chars().nth(pos + web_name.len());
        match next_char {
            None => true,
            Some(c) => !c.is_ascii_lowercase(),
        }
    }
}

impl Default for SessionType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Android => write!(f, "Android"),
            Self::Apple => write!(f, "Apple"),
            Self::Iphone => write!(f, "iPhone"),
            Self::Ipad => write!(f, "iPad"),
            Self::Mac => write!(f, "Mac"),
            Self::Windows => write!(f, "Windows"),
            Self::Linux => write!(f, "Linux"),
            Self::Ubuntu => write!(f, "Ubuntu"),
            Self::Chrome => write!(f, "Chrome"),
            Self::Firefox => write!(f, "Firefox"),
            Self::Safari => write!(f, "Safari"),
            Self::Edge => write!(f, "Edge"),
            Self::Opera => write!(f, "Opera"),
            Self::Vivaldi => write!(f, "Vivaldi"),
            Self::Brave => write!(f, "Brave"),
            Self::Xbox => write!(f, "Xbox"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Contains information about one session in a Telegram application.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::Session;
///
/// let session = Session::new(
///     123456789,
///     true,
///     false,
///     "iPhone 15",
///     "iOS",
///     "17.0",
///     "Telegram",
///     "10.0.0",
///     1704067200,
///     1704153600
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    /// Session identifier (authorization hash).
    pub id: i64,

    /// True if this is the current session.
    pub is_current: bool,

    /// True if 2FA password is needed to complete authorization.
    pub is_password_pending: bool,

    /// True if session wasn't confirmed from another session.
    pub is_unconfirmed: bool,

    /// True if incoming secret chats can be accepted.
    pub can_accept_secret_chats: bool,

    /// True if incoming calls can be accepted.
    pub can_accept_calls: bool,

    /// Type of session based on system and application version.
    pub session_type: SessionType,

    /// Telegram API identifier provided by application.
    pub api_id: i32,

    /// Name of application provided by application.
    pub application_name: String,

    /// Version of application provided by application.
    pub application_version: String,

    /// True if official application or uses official api_id.
    pub is_official_application: bool,

    /// Model of device application runs on.
    pub device_model: String,

    /// Operating system application runs on.
    pub platform: String,

    /// Version of operating system.
    pub system_version: String,

    /// Unix timestamp when user logged in.
    pub log_in_date: i32,

    /// Unix timestamp when session was last used.
    pub last_active_date: i32,

    /// IP address from which session was created.
    pub ip_address: String,

    /// Human-readable description of country and region.
    pub location: String,
}

impl Session {
    /// Creates a new session.
    ///
    /// # Arguments
    ///
    /// * `id` - Session identifier
    /// * `is_current` - Whether this is the current session
    /// * `is_password_pending` - Whether 2FA password is pending
    /// * `device_model` - Device model
    /// * `platform` - Platform
    /// * `system_version` - System version
    /// * `application_name` - Application name
    /// * `application_version` - Application version
    /// * `log_in_date` - Login date (Unix timestamp)
    /// * `last_active_date` - Last active date (Unix timestamp)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::Session;
    ///
    /// let session = Session::new(
    ///     123456789,
    ///     true,
    ///     false,
    ///     "iPhone 15",
    ///     "iOS",
    ///     "17.0",
    ///     "Telegram",
    ///     "10.0.0",
    ///     1704067200,
    ///     1704153600
    /// );
    /// ```
    pub fn new(
        id: i64,
        is_current: bool,
        is_password_pending: bool,
        device_model: &str,
        platform: &str,
        system_version: &str,
        application_name: &str,
        application_version: &str,
        log_in_date: i32,
        last_active_date: i32,
    ) -> Self {
        let session_type =
            SessionType::detect(application_name, device_model, platform, system_version);

        Self {
            id,
            is_current,
            is_password_pending,
            is_unconfirmed: false,
            can_accept_secret_chats: true,
            can_accept_calls: true,
            session_type,
            api_id: 0,
            application_name: application_name.to_string(),
            application_version: application_version.to_string(),
            is_official_application: false,
            device_model: device_model.to_string(),
            platform: platform.to_string(),
            system_version: system_version.to_string(),
            log_in_date,
            last_active_date,
            ip_address: String::new(),
            location: String::new(),
        }
    }

    /// Creates a session from TL authorization.
    ///
    /// # Arguments
    ///
    /// * `auth` - TL authorization object
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::{Session, tl::Authorization};
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
    /// let session = Session::from_authorization(auth);
    /// ```
    pub fn from_authorization(auth: Authorization) -> Self {
        let session_type = SessionType::detect(
            &auth.app_name,
            &auth.device_model,
            &auth.platform,
            &auth.system_version,
        );

        Self {
            id: auth.hash,
            is_current: auth.current,
            is_password_pending: auth.password_pending,
            is_unconfirmed: auth.unconfirmed,
            can_accept_secret_chats: !auth.encrypted_requests_disabled,
            can_accept_calls: !auth.call_requests_disabled,
            session_type,
            api_id: auth.api_id,
            application_name: auth.app_name,
            application_version: auth.app_version,
            is_official_application: auth.official_app,
            device_model: auth.device_model,
            platform: auth.platform,
            system_version: auth.system_version,
            log_in_date: auth.date_created,
            last_active_date: auth.date_active,
            ip_address: auth.ip,
            location: auth.country,
        }
    }

    /// Validates session data.
    ///
    /// # Returns
    ///
    /// Ok if session is valid, Err otherwise.
    pub fn validate(&self) -> Result<()> {
        if self.id == 0 {
            return Err(AccountManagerError::InvalidSessionId(0));
        }

        if self.log_in_date < 0 || self.last_active_date < 0 {
            return Err(AccountManagerError::InvalidAuthorizationParameters);
        }

        Ok(())
    }
}

/// Contains information about one website the user is logged into.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::ConnectedWebsite;
/// use rustgram_types::UserId;
///
/// let bot_id = UserId::new(123456789).unwrap();
/// let website = ConnectedWebsite::new(
///     987654321,
///     "example.com",
///     bot_id,
///     "Chrome",
///     "Windows",
///     1704067200,
///     1704153600
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedWebsite {
    /// Website identifier (authorization hash).
    pub id: i64,

    /// Domain name of the website.
    pub domain_name: String,

    /// User identifier of bot linked with website.
    pub bot_user_id: UserId,

    /// Version of browser used to log in.
    pub browser: String,

    /// Operating system browser is running on.
    pub platform: String,

    /// Unix timestamp when user was logged in.
    pub log_in_date: i32,

    /// Unix timestamp when authorization was last used.
    pub last_active_date: i32,

    /// IP address from which user logged in.
    pub ip_address: String,

    /// Human-readable country and region description.
    pub location: String,
}

impl ConnectedWebsite {
    /// Creates a new connected website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website identifier
    /// * `domain_name` - Domain name
    /// * `bot_user_id` - Bot user ID
    /// * `browser` - Browser
    /// * `platform` - Platform
    /// * `log_in_date` - Login date (Unix timestamp)
    /// * `last_active_date` - Last active date (Unix timestamp)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::ConnectedWebsite;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(123456789).unwrap();
    /// let website = ConnectedWebsite::new(
    ///     987654321,
    ///     "example.com",
    ///     bot_id,
    ///     "Chrome",
    ///     "Windows",
    ///     1704067200,
    ///     1704153600
    /// );
    /// ```
    pub fn new(
        id: i64,
        domain_name: &str,
        bot_user_id: UserId,
        browser: &str,
        platform: &str,
        log_in_date: i32,
        last_active_date: i32,
    ) -> Self {
        Self {
            id,
            domain_name: domain_name.to_string(),
            bot_user_id,
            browser: browser.to_string(),
            platform: platform.to_string(),
            log_in_date,
            last_active_date,
            ip_address: String::new(),
            location: String::new(),
        }
    }

    /// Creates a connected website from TL web authorization.
    ///
    /// # Arguments
    ///
    /// * `web_auth` - TL web authorization object
    ///
    /// # Returns
    ///
    /// Result containing the connected website or an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::{ConnectedWebsite, tl::WebAuthorization};
    ///
    /// let web_auth = WebAuthorization::new(
    ///     987654321,
    ///     "example.com",
    ///     123456789,
    ///     "Chrome",
    ///     "Windows"
    /// );
    /// let website = ConnectedWebsite::from_web_authorization(web_auth);
    /// ```
    pub fn from_web_authorization(web_auth: WebAuthorization) -> Result<Self> {
        let bot_user_id = UserId::new(web_auth.bot_id)
            .map_err(|_| AccountManagerError::InvalidWebsiteId(web_auth.hash))?;

        Ok(Self {
            id: web_auth.hash,
            domain_name: web_auth.domain,
            bot_user_id,
            browser: web_auth.browser,
            platform: web_auth.platform,
            log_in_date: web_auth.date_created,
            last_active_date: web_auth.date_active,
            ip_address: web_auth.ip,
            location: web_auth.region,
        })
    }

    /// Validates website data.
    ///
    /// # Returns
    ///
    /// Ok if website is valid, Err otherwise.
    pub fn validate(&self) -> Result<()> {
        if self.id == 0 {
            return Err(AccountManagerError::InvalidWebsiteId(0));
        }

        if self.domain_name.is_empty() {
            return Err(AccountManagerError::InvalidAuthorizationParameters);
        }

        if !self.bot_user_id.is_valid() {
            return Err(AccountManagerError::InvalidAuthorizationParameters);
        }

        Ok(())
    }
}

/// Contains information about an unconfirmed session.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::UnconfirmedAuthorization;
///
/// let unconfirmed = UnconfirmedAuthorization::new(
///     123456789,
///     1704067200,
///     "iPhone 15",
///     "United States"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnconfirmedAuthorization {
    /// Session identifier.
    pub hash: i64,

    /// Unix timestamp when user logged in.
    pub date: i32,

    /// Model of device used for session creation.
    pub device: String,

    /// Human-readable description of location.
    pub location: String,
}

impl UnconfirmedAuthorization {
    /// Creates a new unconfirmed authorization.
    ///
    /// # Arguments
    ///
    /// * `hash` - Session identifier
    /// * `date` - Unix timestamp when user logged in
    /// * `device` - Device model
    /// * `location` - Location description
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::UnconfirmedAuthorization;
    ///
    /// let unconfirmed = UnconfirmedAuthorization::new(
    ///     123456789,
    ///     1704067200,
    ///     "iPhone 15",
    ///     "United States"
    /// );
    /// ```
    pub fn new(hash: i64, date: i32, device: &str, location: &str) -> Self {
        Self {
            hash,
            date,
            device: device.to_string(),
            location: location.to_string(),
        }
    }

    /// Returns the session hash.
    pub fn get_hash(&self) -> i64 {
        self.hash
    }

    /// Returns the login date.
    pub fn get_date(&self) -> i32 {
        self.date
    }

    /// Validates unconfirmed authorization data.
    ///
    /// # Returns
    ///
    /// Ok if valid, Err otherwise.
    pub fn validate(&self) -> Result<()> {
        if self.hash == 0 {
            return Err(AccountManagerError::AuthorizationNotConfirmed(0));
        }

        if self.date < 0 {
            return Err(AccountManagerError::InvalidAuthorizationParameters);
        }

        Ok(())
    }
}

/// Account time-to-live settings.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::AccountTTL;
///
/// let ttl = AccountTTL::with_days(30).unwrap();
/// assert_eq!(ttl.days(), 30);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountTTL {
    /// Number of days before account deletion (1-366).
    days: i32,
}

impl AccountTTL {
    /// Minimum valid TTL value.
    pub const MIN_DAYS: i32 = 1;

    /// Maximum valid TTL value.
    pub const MAX_DAYS: i32 = 366;

    /// Creates account TTL with specified days.
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days (1-366)
    ///
    /// # Returns
    ///
    /// Some(AccountTTL) if valid, None otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountTTL;
    ///
    /// let ttl = AccountTTL::with_days(30).unwrap();
    /// assert_eq!(ttl.days(), 30);
    ///
    /// assert!(AccountTTL::with_days(0).is_none());
    /// assert!(AccountTTL::with_days(400).is_none());
    /// ```
    pub fn with_days(days: i32) -> Option<Self> {
        if days >= Self::MIN_DAYS && days <= Self::MAX_DAYS {
            Some(Self { days })
        } else {
            None
        }
    }

    /// Returns the number of days.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::AccountTTL;
    ///
    /// let ttl = AccountTTL::with_days(30).unwrap();
    /// assert_eq!(ttl.days(), 30);
    /// ```
    pub const fn days(self) -> i32 {
        self.days
    }
}

/// Contains list of all active sessions.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::Sessions;
///
/// let sessions = Sessions::new(7);
/// assert_eq!(sessions.inactive_session_ttl_days(), 7);
/// assert!(sessions.sessions().is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sessions {
    /// List of active sessions.
    sessions: Vec<Session>,

    /// Days of inactivity before auto-termination (1-366).
    inactive_session_ttl_days: i32,
}

impl Sessions {
    /// Creates a new sessions collection.
    ///
    /// # Arguments
    ///
    /// * `inactive_session_ttl_days` - Days before auto-termination
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::Sessions;
    ///
    /// let sessions = Sessions::new(7);
    /// ```
    pub fn new(inactive_session_ttl_days: i32) -> Self {
        Self {
            sessions: Vec::new(),
            inactive_session_ttl_days,
        }
    }

    /// Returns the list of sessions.
    pub fn sessions(&self) -> &[Session] {
        &self.sessions
    }

    /// Returns the inactive session TTL in days.
    pub fn inactive_session_ttl_days(&self) -> i32 {
        self.inactive_session_ttl_days
    }

    /// Adds a session to the collection.
    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }
}

/// Contains list of connected websites.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::ConnectedWebsites;
///
/// let websites = ConnectedWebsites::new();
/// assert!(websites.websites().is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedWebsites {
    /// List of connected websites.
    websites: Vec<ConnectedWebsite>,
}

impl ConnectedWebsites {
    /// Creates a new connected websites collection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::ConnectedWebsites;
    ///
    /// let websites = ConnectedWebsites::new();
    /// ```
    pub fn new() -> Self {
        Self {
            websites: Vec::new(),
        }
    }

    /// Returns the list of connected websites.
    pub fn websites(&self) -> &[ConnectedWebsite] {
        &self.websites
    }

    /// Adds a website to the collection.
    pub fn add_website(&mut self, website: ConnectedWebsite) {
        self.websites.push(website);
    }
}

/// Contains HTTPS URL for user information.
///
/// # Example
///
/// ```rust
/// use rustgram_account_manager::UserLink;
///
/// let link = UserLink::new("https://t.me/username", 86400);
/// assert_eq!(link.url(), "https://t.me/username");
/// assert_eq!(link.expires_in(), 86400);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserLink {
    /// The URL.
    url: String,

    /// Seconds until link expires (0 for permanent username link).
    expires_in: i32,
}

impl UserLink {
    /// Creates a new user link.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL
    /// * `expires_in` - Seconds until expiration (0 for permanent)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_account_manager::UserLink;
    ///
    /// let link = UserLink::new("https://t.me/username", 86400);
    /// ```
    pub fn new(url: &str, expires_in: i32) -> Self {
        Self {
            url: url.to_string(),
            expires_in,
        }
    }

    /// Returns the URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns seconds until expiration.
    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SessionType tests
    #[test]
    fn test_session_type_detect_android() {
        assert_eq!(
            SessionType::detect("Telegram", "Pixel 5", "Android", "11"),
            SessionType::Android
        );
    }

    #[test]
    fn test_session_type_detect_ios_iphone() {
        assert_eq!(
            SessionType::detect("Telegram", "iPhone", "iOS", "17.0"),
            SessionType::Iphone
        );
    }

    #[test]
    fn test_session_type_detect_ios_ipad() {
        assert_eq!(
            SessionType::detect("Telegram", "iPad", "iOS", "17.0"),
            SessionType::Ipad
        );
    }

    #[test]
    fn test_session_type_detect_mac() {
        assert_eq!(
            SessionType::detect("Telegram", "Mac", "macOS", "14.0"),
            SessionType::Mac
        );
    }

    #[test]
    fn test_session_type_detect_apple_generic() {
        assert_eq!(
            SessionType::detect("Telegram", "", "iOS", "17.0"),
            SessionType::Apple
        );
    }

    #[test]
    fn test_session_type_detect_windows() {
        assert_eq!(
            SessionType::detect("Telegram Desktop", "", "Windows", "10.0"),
            SessionType::Windows
        );
    }

    #[test]
    fn test_session_type_detect_linux() {
        assert_eq!(
            SessionType::detect("Telegram Desktop", "", "Linux", "6.0"),
            SessionType::Linux
        );
    }

    #[test]
    fn test_session_type_detect_ubuntu() {
        assert_eq!(
            SessionType::detect("Telegram Desktop", "", "Ubuntu", "22.04"),
            SessionType::Ubuntu
        );
    }

    #[test]
    fn test_session_type_detect_chrome_web() {
        assert_eq!(
            SessionType::detect("Telegram Web", "Chrome", "Windows", "10.0"),
            SessionType::Chrome
        );
    }

    #[test]
    fn test_session_type_detect_firefox_web() {
        assert_eq!(
            SessionType::detect("Telegram Web", "Firefox", "Windows", "10.0"),
            SessionType::Firefox
        );
    }

    #[test]
    fn test_session_type_detect_safari_web() {
        assert_eq!(
            SessionType::detect("Telegram Web", "Safari", "iOS", "17.0"),
            SessionType::Safari
        );
    }

    #[test]
    fn test_session_type_detect_edge_web() {
        assert_eq!(
            SessionType::detect("Telegram Web", "Edg", "Windows", "10.0"),
            SessionType::Edge
        );
    }

    #[test]
    fn test_session_type_detect_opera_web() {
        assert_eq!(
            SessionType::detect("Telegram Web", "Opera", "Windows", "10.0"),
            SessionType::Opera
        );
    }

    #[test]
    fn test_session_type_detect_vivaldi_web() {
        assert_eq!(
            SessionType::detect("Telegram Web", "Vivaldi", "Windows", "10.0"),
            SessionType::Vivaldi
        );
    }

    #[test]
    fn test_session_type_detect_brave_web() {
        assert_eq!(
            SessionType::detect("Telegram Web", "Brave", "Windows", "10.0"),
            SessionType::Brave
        );
    }

    #[test]
    fn test_session_type_detect_xbox() {
        assert_eq!(
            SessionType::detect("Telegram", "Xbox", "Windows", "10.0"),
            SessionType::Xbox
        );
    }

    #[test]
    fn test_session_type_detect_unknown() {
        assert_eq!(
            SessionType::detect("Unknown App", "Unknown Device", "Unknown", "1.0"),
            SessionType::Unknown
        );
    }

    // Session tests
    #[test]
    fn test_session_new() {
        let session = Session::new(
            123456789,
            true,
            false,
            "iPhone 15",
            "iOS",
            "17.0",
            "Telegram",
            "10.0.0",
            1704067200,
            1704153600,
        );

        assert_eq!(session.id, 123456789);
        assert!(session.is_current);
        assert!(!session.is_password_pending);
        assert_eq!(session.device_model, "iPhone 15");
        assert_eq!(session.platform, "iOS");
        assert_eq!(session.system_version, "17.0");
        assert_eq!(session.application_name, "Telegram");
        assert_eq!(session.application_version, "10.0.0");
        assert_eq!(session.log_in_date, 1704067200);
        assert_eq!(session.last_active_date, 1704153600);
    }

    #[test]
    fn test_session_from_authorization() {
        let auth = Authorization::new(
            123456789, true, false, "Telegram", "10.0", "iPhone", "iOS", "17.0",
        );

        let session = Session::from_authorization(auth);
        assert_eq!(session.id, 123456789);
        assert!(session.is_current);
        assert!(!session.is_password_pending);
    }

    #[test]
    fn test_session_validate() {
        let session = Session::new(
            123456789,
            true,
            false,
            "iPhone 15",
            "iOS",
            "17.0",
            "Telegram",
            "10.0.0",
            1704067200,
            1704153600,
        );

        assert!(session.validate().is_ok());

        let mut invalid_session = session.clone();
        invalid_session.id = 0;
        assert!(invalid_session.validate().is_err());
    }

    // ConnectedWebsite tests
    #[test]
    fn test_connected_website_new() {
        let bot_id = UserId::new(123456789).unwrap();
        let website = ConnectedWebsite::new(
            987654321,
            "example.com",
            bot_id,
            "Chrome",
            "Windows",
            1704067200,
            1704153600,
        );

        assert_eq!(website.id, 987654321);
        assert_eq!(website.domain_name, "example.com");
        assert_eq!(website.bot_user_id, bot_id);
        assert_eq!(website.browser, "Chrome");
        assert_eq!(website.platform, "Windows");
    }

    #[test]
    fn test_connected_website_from_web_authorization() {
        let web_auth =
            WebAuthorization::new(987654321, "example.com", 123456789, "Chrome", "Windows");
        let website = ConnectedWebsite::from_web_authorization(web_auth);

        assert!(website.is_ok());
        let website = website.unwrap();
        assert_eq!(website.id, 987654321);
        assert_eq!(website.domain_name, "example.com");
    }

    #[test]
    fn test_connected_website_validate() {
        let bot_id = UserId::new(123456789).unwrap();
        let website = ConnectedWebsite::new(
            987654321,
            "example.com",
            bot_id,
            "Chrome",
            "Windows",
            1704067200,
            1704153600,
        );

        assert!(website.validate().is_ok());
    }

    // UnconfirmedAuthorization tests
    #[test]
    fn test_unconfirmed_authorization_new() {
        let unconfirmed =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");

        assert_eq!(unconfirmed.hash, 123456789);
        assert_eq!(unconfirmed.date, 1704067200);
        assert_eq!(unconfirmed.device, "iPhone 15");
        assert_eq!(unconfirmed.location, "United States");
    }

    #[test]
    fn test_unconfirmed_authorization_get_hash() {
        let unconfirmed =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        assert_eq!(unconfirmed.get_hash(), 123456789);
    }

    #[test]
    fn test_unconfirmed_authorization_get_date() {
        let unconfirmed =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        assert_eq!(unconfirmed.get_date(), 1704067200);
    }

    #[test]
    fn test_unconfirmed_authorization_validate() {
        let unconfirmed =
            UnconfirmedAuthorization::new(123456789, 1704067200, "iPhone 15", "United States");
        assert!(unconfirmed.validate().is_ok());

        let mut invalid = unconfirmed.clone();
        invalid.hash = 0;
        assert!(invalid.validate().is_err());
    }

    // AccountTTL tests
    #[test]
    fn test_account_ttl_valid() {
        assert!(AccountTTL::with_days(1).is_some());
        assert!(AccountTTL::with_days(30).is_some());
        assert!(AccountTTL::with_days(366).is_some());
    }

    #[test]
    fn test_account_ttl_invalid() {
        assert!(AccountTTL::with_days(0).is_none());
        assert!(AccountTTL::with_days(-1).is_none());
        assert!(AccountTTL::with_days(367).is_none());
        assert!(AccountTTL::with_days(500).is_none());
    }

    #[test]
    fn test_account_ttl_days() {
        let ttl = AccountTTL::with_days(30).unwrap();
        assert_eq!(ttl.days(), 30);
    }

    // Sessions tests
    #[test]
    fn test_sessions_new() {
        let sessions = Sessions::new(7);
        assert_eq!(sessions.inactive_session_ttl_days(), 7);
        assert!(sessions.sessions().is_empty());
    }

    #[test]
    fn test_sessions_add_session() {
        let mut sessions = Sessions::new(7);
        let session = Session::new(
            123456789,
            true,
            false,
            "iPhone 15",
            "iOS",
            "17.0",
            "Telegram",
            "10.0.0",
            1704067200,
            1704153600,
        );

        sessions.add_session(session);
        assert_eq!(sessions.sessions().len(), 1);
    }

    // ConnectedWebsites tests
    #[test]
    fn test_connected_websites_new() {
        let websites = ConnectedWebsites::new();
        assert!(websites.websites().is_empty());
    }

    #[test]
    fn test_connected_websites_add_website() {
        let mut websites = ConnectedWebsites::new();
        let bot_id = UserId::new(123456789).unwrap();
        let website = ConnectedWebsite::new(
            987654321,
            "example.com",
            bot_id,
            "Chrome",
            "Windows",
            1704067200,
            1704153600,
        );

        websites.add_website(website);
        assert_eq!(websites.websites().len(), 1);
    }

    // UserLink tests
    #[test]
    fn test_user_link_new() {
        let link = UserLink::new("https://t.me/username", 86400);
        assert_eq!(link.url(), "https://t.me/username");
        assert_eq!(link.expires_in(), 86400);
    }

    #[test]
    fn test_user_link_permanent() {
        let link = UserLink::new("https://t.me/username", 0);
        assert_eq!(link.expires_in(), 0);
    }
}
