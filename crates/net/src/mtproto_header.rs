// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto header generation for transport layer.
//!
//! This module implements TDLib's MtprotoHeader from `td/telegram/net/MtprotoHeader.h`.
//!
//! Generates the protocol headers sent during connection establishment,
//! including device information, language settings, and connection parameters.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

/// Error types for MTProto header operations.
#[derive(Debug, Error)]
pub enum MtprotoHeaderError {
    /// Invalid API ID
    #[error("Invalid API ID: {0}")]
    InvalidApiId(i32),

    /// Invalid language code
    #[error("Invalid language code: {0}")]
    InvalidLanguageCode(String),

    /// Invalid parameters
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    /// Header generation failed
    #[error("Failed to generate header: {0}")]
    GenerationFailed(String),
}

/// MTProto header options.
///
/// Configuration for generating MTProto connection headers.
/// Based on TDLib's MtprotoHeader::Options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtprotoHeaderOptions {
    /// API ID (from my.telegram.org)
    pub api_id: i32,

    /// System language code (e.g., "en")
    pub system_language_code: String,

    /// Device model (e.g., "iPhone", "PC")
    pub device_model: String,

    /// System version (e.g., "iOS 17.0", "Windows 11")
    pub system_version: String,

    /// Application version (e.g., "10.0.0")
    pub application_version: String,

    /// Language pack (e.g., "tdesktop")
    pub language_pack: String,

    /// Language code (e.g., "en", "ru")
    pub language_code: String,

    /// Custom parameters string
    pub parameters: String,

    /// Timezone offset in seconds
    pub tz_offset: i32,

    /// Whether the client is running in an emulator
    pub is_emulator: bool,

    /// Proxy type (if any)
    pub proxy_type: Option<String>,
}

impl Default for MtprotoHeaderOptions {
    fn default() -> Self {
        Self {
            api_id: 0,
            system_language_code: "en".to_string(),
            device_model: "Unknown".to_string(),
            system_version: "Unknown".to_string(),
            application_version: "1.0.0".to_string(),
            language_pack: "".to_string(),
            language_code: "en".to_string(),
            parameters: "".to_string(),
            tz_offset: 0,
            is_emulator: false,
            proxy_type: None,
        }
    }
}

impl MtprotoHeaderOptions {
    /// Creates new options.
    pub fn new(
        api_id: i32,
        device_model: String,
        system_version: String,
        app_version: String,
    ) -> Self {
        Self {
            api_id,
            device_model,
            system_version,
            application_version: app_version,
            ..Self::default()
        }
    }

    /// Validates the options.
    pub fn validate(&self) -> Result<(), MtprotoHeaderError> {
        if self.api_id <= 0 {
            return Err(MtprotoHeaderError::InvalidApiId(self.api_id));
        }

        if self.system_language_code.is_empty() {
            return Err(MtprotoHeaderError::InvalidParameters(
                "system_language_code is empty".into(),
            ));
        }

        if self.device_model.is_empty() {
            return Err(MtprotoHeaderError::InvalidParameters(
                "device_model is empty".into(),
            ));
        }

        if self.application_version.is_empty() {
            return Err(MtprotoHeaderError::InvalidParameters(
                "application_version is empty".into(),
            ));
        }

        Ok(())
    }

    /// Returns the API ID.
    pub fn api_id(&self) -> i32 {
        self.api_id
    }

    /// Sets the API ID.
    pub fn set_api_id(&mut self, api_id: i32) {
        self.api_id = api_id;
    }

    /// Returns the system language code.
    pub fn system_language_code(&self) -> &str {
        &self.system_language_code
    }

    /// Returns the device model.
    pub fn device_model(&self) -> &str {
        &self.device_model
    }

    /// Returns the system version.
    pub fn system_version(&self) -> &str {
        &self.system_version
    }

    /// Returns the application version.
    pub fn application_version(&self) -> &str {
        &self.application_version
    }

    /// Returns the language pack.
    pub fn language_pack(&self) -> &str {
        &self.language_pack
    }

    /// Returns the language code.
    pub fn language_code(&self) -> &str {
        &self.language_code
    }

    /// Returns the custom parameters.
    pub fn parameters(&self) -> &str {
        &self.parameters
    }

    /// Returns the timezone offset.
    pub fn tz_offset(&self) -> i32 {
        self.tz_offset
    }

    /// Returns whether this is an emulator.
    pub fn is_emulator(&self) -> bool {
        self.is_emulator
    }
}

/// MTProto header.
///
/// Generates and caches protocol headers for MTProto connections.
/// Based on TDLib's MtprotoHeader from `td/telegram/net/MtprotoHeader.h`.
#[derive(Debug)]
pub struct MtprotoHeader {
    /// Header options
    options: Arc<RwLock<MtprotoHeaderOptions>>,

    /// Cached default header (for authenticated connections)
    default_header: Arc<RwLock<String>>,

    /// Cached anonymous header (for unauthenticated connections)
    anonymous_header: Arc<RwLock<String>>,
}

impl MtprotoHeader {
    /// Creates a new MTProto header from options.
    pub fn new(options: MtprotoHeaderOptions) -> Result<Self, MtprotoHeaderError> {
        options.validate()?;

        let default_header = Self::gen_header(&options, false)?;
        let anonymous_header = Self::gen_header(&options, true)?;

        Ok(Self {
            options: Arc::new(RwLock::new(options)),
            default_header: Arc::new(RwLock::new(default_header)),
            anonymous_header: Arc::new(RwLock::new(anonymous_header)),
        })
    }

    /// Creates a header with default options.
    pub fn with_defaults(api_id: i32) -> Result<Self, MtprotoHeaderError> {
        let options = MtprotoHeaderOptions {
            api_id,
            system_language_code: "en".to_string(),
            device_model: "PC".to_string(),
            system_version: "Windows".to_string(),
            application_version: "1.0".to_string(),
            language_pack: "".to_string(),
            language_code: "en".to_string(),
            parameters: "".to_string(),
            tz_offset: 0,
            is_emulator: false,
            proxy_type: None,
        };

        Self::new(options)
    }

    /// Returns the default header for authenticated connections.
    pub fn get_default_header(&self) -> String {
        self.default_header.read().clone()
    }

    /// Returns the anonymous header for unauthenticated connections.
    pub fn get_anonymous_header(&self) -> String {
        self.anonymous_header.read().clone()
    }

    /// Returns the system language code.
    pub fn get_system_language_code(&self) -> String {
        self.options.read().system_language_code.clone()
    }

    /// Sets the proxy type and regenerates headers.
    pub fn set_proxy(&self, proxy_type: String) {
        let mut options = self.options.write();
        options.proxy_type = Some(proxy_type);

        // Regenerate headers
        if let Ok(default) = Self::gen_header(&options, false) {
            *self.default_header.write() = default;
        }

        if let Ok(anonymous) = Self::gen_header(&options, true) {
            *self.anonymous_header.write() = anonymous;
        }
    }

    /// Sets custom parameters and regenerates headers.
    ///
    /// Returns `true` if the value changed.
    pub fn set_parameters(&self, parameters: String) -> Result<bool, MtprotoHeaderError> {
        let mut options = self.options.write();

        if options.parameters == parameters {
            return Ok(false);
        }

        options.parameters = parameters;

        let default = Self::gen_header(&options, false)?;
        *self.default_header.write() = default;

        Ok(true)
    }

    /// Sets the emulator flag and regenerates headers.
    ///
    /// Returns `true` if the value changed.
    pub fn set_is_emulator(&self, is_emulator: bool) -> Result<bool, MtprotoHeaderError> {
        let mut options = self.options.write();

        if options.is_emulator == is_emulator {
            return Ok(false);
        }

        options.is_emulator = is_emulator;

        let default = Self::gen_header(&options, false)?;
        *self.default_header.write() = default;

        Ok(true)
    }

    /// Sets the language pack and regenerates headers.
    ///
    /// Returns `true` if the value changed.
    pub fn set_language_pack(&self, language_pack: String) -> Result<bool, MtprotoHeaderError> {
        let mut options = self.options.write();

        if options.language_pack == language_pack {
            return Ok(false);
        }

        options.language_pack = language_pack;

        let default = Self::gen_header(&options, false)?;
        *self.default_header.write() = default;

        Ok(true)
    }

    /// Sets the language code and regenerates headers.
    ///
    /// Returns `true` if the value changed.
    pub fn set_language_code(&self, language_code: String) -> Result<bool, MtprotoHeaderError> {
        let mut options = self.options.write();

        if options.language_code == language_code {
            return Ok(false);
        }

        options.language_code = language_code;

        let default = Self::gen_header(&options, false)?;
        *self.default_header.write() = default;

        Ok(true)
    }

    /// Sets the timezone offset and regenerates headers.
    ///
    /// Returns `true` if the value changed.
    pub fn set_tz_offset(&self, tz_offset: i32) -> Result<bool, MtprotoHeaderError> {
        let mut options = self.options.write();

        if options.tz_offset == tz_offset {
            return Ok(false);
        }

        options.tz_offset = tz_offset;

        let default = Self::gen_header(&options, false)?;
        *self.default_header.write() = default;

        Ok(true)
    }

    /// Returns a copy of the current options.
    pub fn get_options(&self) -> MtprotoHeaderOptions {
        self.options.read().clone()
    }

    /// Generates an MTProto header string.
    ///
    /// This is the core function that creates the protocol header.
    /// In a real implementation, this would create a binary TL-encoded structure.
    /// For now, we create a JSON-like string representation.
    fn gen_header(
        options: &MtprotoHeaderOptions,
        is_anonymous: bool,
    ) -> Result<String, MtprotoHeaderError> {
        // Validate options
        options.validate()?;

        // Build the header string
        // In a real implementation, this would be TL-encoded binary data
        // For now, we use a structured string representation

        let mut parts = Vec::new();

        // Basic info
        parts.push(format!("api_id={}", options.api_id));
        parts.push(format!("device_model={}", options.device_model));
        parts.push(format!("system_version={}", options.system_version));
        parts.push(format!("app_version={}", options.application_version));

        // Language settings
        parts.push(format!("lang_code={}", options.system_language_code));
        if !options.language_pack.is_empty() {
            parts.push(format!("lang_pack={}", options.language_pack));
        }
        if !options.language_code.is_empty() {
            parts.push(format!("lang={}", options.language_code));
        }

        // Timezone
        if options.tz_offset != 0 {
            parts.push(format!("tz_offset={}", options.tz_offset));
        }

        // Emulator flag
        if options.is_emulator {
            parts.push("emulator=true".to_string());
        }

        // Parameters
        if !options.parameters.is_empty() {
            parts.push(format!("params={}", options.parameters));
        }

        // Proxy info
        if let Some(ref proxy) = options.proxy_type {
            parts.push(format!("proxy={}", proxy));
        }

        // Anonymous flag
        if is_anonymous {
            parts.push("anonymous=true".to_string());
        }

        Ok(parts.join(";"))
    }

    /// Creates a header for a specific platform.
    pub fn for_platform(
        api_id: i32,
        platform: Platform,
        app_version: String,
    ) -> Result<Self, MtprotoHeaderError> {
        let options = match platform {
            Platform::Windows => MtprotoHeaderOptions {
                api_id,
                system_language_code: "en".to_string(),
                device_model: "PC".to_string(),
                system_version: "Windows".to_string(),
                application_version: app_version,
                language_pack: "windows".to_string(),
                language_code: "en".to_string(),
                ..Default::default()
            },
            Platform::MacOS => MtprotoHeaderOptions {
                api_id,
                system_language_code: "en".to_string(),
                device_model: "Mac".to_string(),
                system_version: "macOS".to_string(),
                application_version: app_version,
                language_pack: "macos".to_string(),
                language_code: "en".to_string(),
                ..Default::default()
            },
            Platform::Linux => MtprotoHeaderOptions {
                api_id,
                system_language_code: "en".to_string(),
                device_model: "PC".to_string(),
                system_version: "Linux".to_string(),
                application_version: app_version,
                language_pack: "linux".to_string(),
                language_code: "en".to_string(),
                ..Default::default()
            },
            Platform::Android => MtprotoHeaderOptions {
                api_id,
                system_language_code: "en".to_string(),
                device_model: "Android".to_string(),
                system_version: "Android 13".to_string(),
                application_version: app_version,
                language_pack: "android".to_string(),
                language_code: "en".to_string(),
                ..Default::default()
            },
            Platform::IOS => MtprotoHeaderOptions {
                api_id,
                system_language_code: "en".to_string(),
                device_model: "iPhone".to_string(),
                system_version: "iOS 17.0".to_string(),
                application_version: app_version,
                language_pack: "ios".to_string(),
                language_code: "en".to_string(),
                ..Default::default()
            },
        };

        Self::new(options)
    }
}

/// Platform type for header generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum Platform {
    /// Windows desktop
    Windows = 0,

    /// macOS desktop
    MacOS = 1,

    /// Linux desktop
    #[default]
    Linux = 2,

    /// Android mobile
    Android = 3,

    /// iOS mobile
    IOS = 4,
}

impl Platform {
    /// Returns the platform name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::MacOS => "macOS",
            Self::Linux => "Linux",
            Self::Android => "Android",
            Self::IOS => "iOS",
        }
    }

    /// Returns `true` if this is a desktop platform.
    pub fn is_desktop(&self) -> bool {
        matches!(self, Self::Windows | Self::MacOS | Self::Linux)
    }

    /// Returns `true` if this is a mobile platform.
    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::Android | Self::IOS)
    }
}

/// Factory for creating MTProto headers.
pub struct MtprotoHeaderFactory;

impl MtprotoHeaderFactory {
    /// Creates a header for desktop applications.
    pub fn create_desktop(
        api_id: i32,
        app_version: String,
    ) -> Result<MtprotoHeader, MtprotoHeaderError> {
        MtprotoHeader::for_platform(api_id, Platform::Linux, app_version)
    }

    /// Creates a header for mobile applications.
    pub fn create_mobile(
        api_id: i32,
        platform: Platform,
        app_version: String,
    ) -> Result<MtprotoHeader, MtprotoHeaderError> {
        if !platform.is_mobile() {
            return Err(MtprotoHeaderError::InvalidParameters(format!(
                "{:?} is not a mobile platform",
                platform
            )));
        }

        MtprotoHeader::for_platform(api_id, platform, app_version)
    }

    /// Creates a minimal header for testing.
    pub fn create_test() -> Result<MtprotoHeader, MtprotoHeaderError> {
        let options = MtprotoHeaderOptions {
            api_id: 1, // Test API ID
            system_language_code: "en".to_string(),
            device_model: "Test".to_string(),
            system_version: "TestOS".to_string(),
            application_version: "0.0.0".to_string(),
            ..Default::default()
        };

        MtprotoHeader::new(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_default() {
        let options = MtprotoHeaderOptions::default();

        assert_eq!(options.api_id, 0);
        assert_eq!(options.system_language_code, "en");
        assert_eq!(options.device_model, "Unknown");
        assert_eq!(options.language_code, "en");
        assert!(!options.is_emulator);
    }

    #[test]
    fn test_options_new() {
        let options =
            MtprotoHeaderOptions::new(12345, "iPhone".into(), "iOS 17".into(), "10.0".into());

        assert_eq!(options.api_id, 12345);
        assert_eq!(options.device_model, "iPhone");
        assert_eq!(options.system_version, "iOS 17");
        assert_eq!(options.application_version, "10.0");
    }

    #[test]
    fn test_options_validate() {
        let mut options = MtprotoHeaderOptions::default();

        // Invalid API ID
        let result = options.validate();
        assert!(result.is_err());

        options.api_id = 12345;
        assert!(options.validate().is_ok());

        // Empty device model
        options.device_model = "".to_string();
        let result = options.validate();
        assert!(result.is_err());

        options.device_model = "Test".to_string();
        assert!(options.validate().is_ok());
    }

    #[test]
    fn test_header_creation() {
        let options = MtprotoHeaderOptions {
            api_id: 12345,
            system_language_code: "en".to_string(),
            device_model: "PC".to_string(),
            system_version: "Windows".to_string(),
            application_version: "1.0".to_string(),
            language_pack: "windows".to_string(),
            language_code: "en".to_string(),
            parameters: "".to_string(),
            tz_offset: 0,
            is_emulator: false,
            proxy_type: None,
        };

        let header = match MtprotoHeader::new(options) {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok header"),
        };

        let default = header.get_default_header();
        let anonymous = header.get_anonymous_header();

        assert!(default.contains("api_id=12345"));
        assert!(default.contains("device_model=PC"));
        assert!(anonymous.contains("anonymous=true"));
    }

    #[test]
    fn test_header_with_defaults() {
        let header = match MtprotoHeader::with_defaults(12345) {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok header"),
        };

        let default = header.get_default_header();
        assert!(default.contains("api_id=12345"));
        assert!(default.contains("device_model=PC"));
    }

    #[test]
    fn test_header_setters() {
        let header = match MtprotoHeader::with_defaults(12345) {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok header"),
        };

        // Test set_parameters
        let changed = match header.set_parameters("test_params".into()) {
            Ok(c) => c,
            Err(_) => panic!("Expected Ok changed"),
        };
        assert!(changed);

        let default = header.get_default_header();
        assert!(default.contains("params=test_params"));

        let changed = match header.set_parameters("test_params".into()) {
            Ok(c) => c,
            Err(_) => panic!("Expected Ok changed"),
        };
        assert!(!changed);

        // Test set_is_emulator
        let changed = match header.set_is_emulator(true) {
            Ok(c) => c,
            Err(_) => panic!("Expected Ok changed"),
        };
        assert!(changed);

        let default = header.get_default_header();
        assert!(default.contains("emulator=true"));

        // Test set_language_code
        let changed = match header.set_language_code("ru".into()) {
            Ok(c) => c,
            Err(_) => panic!("Expected Ok changed"),
        };
        assert!(changed);

        let default = header.get_default_header();
        assert!(default.contains("lang=ru"));

        // Test set_tz_offset
        let changed = match header.set_tz_offset(10800) {
            Ok(c) => c,
            Err(_) => panic!("Expected Ok changed"),
        };
        assert!(changed);

        let default = header.get_default_header();
        assert!(default.contains("tz_offset=10800"));
    }

    #[test]
    fn test_header_for_platform() {
        let header = match MtprotoHeader::for_platform(12345, Platform::Windows, "2.0".into()) {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok header"),
        };

        let default = header.get_default_header();
        assert!(default.contains("device_model=PC"));
        assert!(default.contains("system_version=Windows"));
        assert!(default.contains("app_version=2.0"));

        let options = header.get_options();
        assert_eq!(options.language_pack, "windows");
    }

    #[test]
    fn test_platform() {
        assert_eq!(Platform::Windows.as_str(), "Windows");
        assert_eq!(Platform::Android.as_str(), "Android");

        assert!(Platform::Windows.is_desktop());
        assert!(Platform::Android.is_mobile());
        assert!(!Platform::Android.is_desktop());
    }

    #[test]
    fn test_factory() {
        let desktop = match MtprotoHeaderFactory::create_desktop(12345, "1.0".into()) {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok desktop"),
        };
        assert!(desktop
            .get_default_header()
            .contains("system_version=Linux"));

        let mobile = match MtprotoHeaderFactory::create_mobile(12345, Platform::IOS, "10.0".into())
        {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok mobile"),
        };
        assert!(mobile.get_default_header().contains("device_model=iPhone"));

        let result = MtprotoHeaderFactory::create_mobile(12345, Platform::Windows, "1.0".into());
        assert!(result.is_err());

        let test = match MtprotoHeaderFactory::create_test() {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok test"),
        };
        assert!(test.get_default_header().contains("api_id=1"));
    }

    #[test]
    fn test_header_system_language() {
        let options = MtprotoHeaderOptions {
            api_id: 12345,
            system_language_code: "ru".to_string(),
            device_model: "PC".to_string(),
            system_version: "Windows".to_string(),
            application_version: "1.0".to_string(),
            ..Default::default()
        };

        let header = match MtprotoHeader::new(options) {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok header"),
        };
        assert_eq!(header.get_system_language_code(), "ru");
    }

    #[test]
    fn test_header_proxy() {
        let header = match MtprotoHeader::with_defaults(12345) {
            Ok(h) => h,
            Err(_) => panic!("Expected Ok header"),
        };

        header.set_proxy("socks5".into());

        let default = header.get_default_header();
        assert!(default.contains("proxy=socks5"));
    }
}
