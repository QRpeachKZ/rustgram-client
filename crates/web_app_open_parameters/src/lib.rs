// rustgram_web_app_open_parameters
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # Web App Open Parameters
//!
//! Parameters for opening a Telegram Web App with theme and mode settings.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_web_app_open_parameters::{WebAppOpenParameters, ThemeParameters, WebAppMode};
//!
//! let theme = ThemeParameters::new();
//! let params = WebAppOpenParameters::new(Some(theme), "MyApp".to_string(), WebAppMode::Compact);
//! assert_eq!(params.application_name(), "MyApp");
//! assert!(params.is_compact());
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Web app open mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WebAppMode {
    /// Compact mode
    Compact,
    /// Full size mode (default)
    #[default]
    FullSize,
    /// Full screen mode
    FullScreen,
}

impl WebAppMode {
    /// Creates a WebAppMode from a TDLib API mode string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::WebAppMode;
    ///
    /// assert_eq!(
    ///     WebAppMode::from_td_api_mode("webAppOpenModeCompact"),
    ///     WebAppMode::Compact
    /// );
    /// assert_eq!(
    ///     WebAppMode::from_td_api_mode("webAppOpenModeFullScreen"),
    ///     WebAppMode::FullScreen
    /// );
    /// assert_eq!(
    ///     WebAppMode::from_td_api_mode("webAppOpenModeFullSize"),
    ///     WebAppMode::FullSize
    /// );
    /// ```
    #[must_use]
    pub fn from_td_api_mode(mode: &str) -> Self {
        match mode {
            "webAppOpenModeCompact" => Self::Compact,
            "webAppOpenModeFullScreen" => Self::FullScreen,
            _ => Self::FullSize,
        }
    }

    /// Returns whether this is compact mode.
    #[must_use]
    pub const fn is_compact(self) -> bool {
        matches!(self, Self::Compact)
    }

    /// Returns whether this is full screen mode.
    #[must_use]
    pub const fn is_full_screen(self) -> bool {
        matches!(self, Self::FullScreen)
    }
}

/// Theme parameters for a web app.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeParameters {
    /// Background color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// Text color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// Hint color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_color: Option<String>,
    /// Link color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_color: Option<String>,
    /// Button color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_color: Option<String>,
    /// Button text color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text_color: Option<String>,
    /// Secondary background color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_bg_color: Option<String>,
}

impl Default for ThemeParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeParameters {
    /// Creates a new empty theme parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::ThemeParameters;
    ///
    /// let theme = ThemeParameters::new();
    /// assert!(theme.background_color.is_none());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            background_color: None,
            text_color: None,
            hint_color: None,
            link_color: None,
            button_color: None,
            button_text_color: None,
            secondary_bg_color: None,
        }
    }

    /// Creates theme parameters from a JSON string.
    ///
    /// Returns `None` if parsing fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::ThemeParameters;
    ///
    /// let json = "{\"background_color\":\"#ffffff\",\"text_color\":\"#000000\"}";
    /// let theme = ThemeParameters::from_json(json);
    /// assert!(theme.is_some());
    /// assert_eq!(theme.unwrap().background_color, Some("#ffffff".to_string()));
    /// ```
    #[must_use]
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }

    /// Converts theme parameters to a JSON string.
    ///
    /// Returns `None` if serialization fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::ThemeParameters;
    ///
    /// let theme = ThemeParameters::new();
    /// let json = theme.to_json_string();
    /// assert!(json.is_ok());
    /// ```
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Returns whether all theme parameters are empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::ThemeParameters;
    ///
    /// let theme = ThemeParameters::new();
    /// assert!(theme.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.background_color.is_none()
            && self.text_color.is_none()
            && self.hint_color.is_none()
            && self.link_color.is_none()
            && self.button_color.is_none()
            && self.button_text_color.is_none()
            && self.secondary_bg_color.is_none()
    }
}

/// Parameters for opening a Telegram Web App.
///
/// Contains theme parameters, application name, and display mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebAppOpenParameters {
    /// Theme parameters (optional)
    theme_parameters: Option<ThemeParameters>,
    /// Application name
    application_name: String,
    /// Compact mode flag
    is_compact: bool,
    /// Full screen mode flag
    is_full_screen: bool,
}

impl Default for WebAppOpenParameters {
    fn default() -> Self {
        Self::new(None, String::new(), WebAppMode::FullSize)
    }
}

impl WebAppOpenParameters {
    /// Creates a new web app open parameters.
    ///
    /// # Arguments
    ///
    /// * `theme_parameters` - Optional theme parameters
    /// * `application_name` - Application name (will be validated)
    /// * `mode` - Web app display mode
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::{WebAppOpenParameters, ThemeParameters, WebAppMode};
    ///
    /// let theme = ThemeParameters::new();
    /// let params = WebAppOpenParameters::new(Some(theme), "MyApp".to_string(), WebAppMode::Compact);
    /// assert_eq!(params.application_name(), "MyApp");
    /// assert!(params.is_compact());
    /// ```
    #[must_use]
    pub fn new(
        theme_parameters: Option<ThemeParameters>,
        mut application_name: String,
        mode: WebAppMode,
    ) -> Self {
        // Validate application name (simple validation: remove control characters)
        application_name.retain(|c| !c.is_control());

        let is_compact = mode.is_compact();
        let is_full_screen = mode.is_full_screen();

        Self {
            theme_parameters,
            application_name,
            is_compact,
            is_full_screen,
        }
    }

    /// Returns the application name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::WebAppOpenParameters;
    ///
    /// let params = WebAppOpenParameters::default();
    /// assert_eq!(params.application_name(), "");
    /// ```
    #[must_use]
    pub fn application_name(&self) -> &str {
        &self.application_name
    }

    /// Returns whether compact mode is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::{WebAppOpenParameters, WebAppMode, ThemeParameters};
    ///
    /// let params = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::Compact);
    /// assert!(params.is_compact());
    /// ```
    #[must_use]
    pub const fn is_compact(&self) -> bool {
        self.is_compact
    }

    /// Returns whether full screen mode is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::{WebAppOpenParameters, WebAppMode};
    ///
    /// let params = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::FullScreen);
    /// assert!(params.is_full_screen());
    /// ```
    #[must_use]
    pub const fn is_full_screen(&self) -> bool {
        self.is_full_screen
    }

    /// Returns a reference to the theme parameters if present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::{WebAppOpenParameters, ThemeParameters};
    ///
    /// let theme = ThemeParameters::new();
    /// let params = WebAppOpenParameters::new(Some(theme), "App".to_string(), Default::default());
    /// assert!(params.theme_parameters().is_some());
    /// ```
    #[must_use]
    pub fn theme_parameters(&self) -> Option<&ThemeParameters> {
        self.theme_parameters.as_ref()
    }

    /// Converts theme parameters to a JSON data string.
    ///
    /// Returns `None` if theme parameters are not set or serialization fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::{WebAppOpenParameters, ThemeParameters};
    ///
    /// let params = WebAppOpenParameters::default();
    /// assert!(params.to_input_theme_parameters().is_none());
    /// ```
    #[must_use]
    pub fn to_input_theme_parameters(&self) -> Option<String> {
        self.theme_parameters.as_ref()?.to_json_string().ok()
    }

    /// Returns the web app mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_web_app_open_parameters::{WebAppOpenParameters, WebAppMode};
    ///
    /// let params = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::Compact);
    /// assert_eq!(params.mode(), WebAppMode::Compact);
    /// ```
    #[must_use]
    pub const fn mode(&self) -> WebAppMode {
        if self.is_compact {
            WebAppMode::Compact
        } else if self.is_full_screen {
            WebAppMode::FullScreen
        } else {
            WebAppMode::FullSize
        }
    }
}

impl fmt::Display for WebAppOpenParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebApp[")?;
        if !self.application_name.is_empty() {
            write!(f, "app: {}", self.application_name)?;
        }
        if self.is_compact {
            write!(f, ", compact")?;
        } else if self.is_full_screen {
            write!(f, ", fullscreen")?;
        }
        if self.theme_parameters.is_some() {
            write!(f, ", themed")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let params = WebAppOpenParameters::default();
        assert_eq!(params.application_name(), "");
        assert!(!params.is_compact());
        assert!(!params.is_full_screen());
        assert!(params.theme_parameters().is_none());
    }

    #[test]
    fn test_new() {
        let theme = ThemeParameters::new();
        let params =
            WebAppOpenParameters::new(Some(theme), "TestApp".to_string(), WebAppMode::Compact);
        assert_eq!(params.application_name(), "TestApp");
        assert!(params.is_compact());
        assert!(!params.is_full_screen());
    }

    #[test]
    fn test_modes() {
        let compact = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::Compact);
        assert!(compact.is_compact());
        assert!(!compact.is_full_screen());
        assert_eq!(compact.mode(), WebAppMode::Compact);

        let fullscreen = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::FullScreen);
        assert!(!fullscreen.is_compact());
        assert!(fullscreen.is_full_screen());
        assert_eq!(fullscreen.mode(), WebAppMode::FullScreen);

        let fullsize = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::FullSize);
        assert!(!fullsize.is_compact());
        assert!(!fullsize.is_full_screen());
        assert_eq!(fullsize.mode(), WebAppMode::FullSize);
    }

    #[test]
    fn test_application_name_sanitization() {
        let params = WebAppOpenParameters::new(
            None,
            "App\u{0}With\u{1}Control\u{2}Chars".to_string(),
            WebAppMode::FullSize,
        );
        assert_eq!(params.application_name(), "AppWithControlChars");
    }

    #[test]
    fn test_theme_parameters_new() {
        let theme = ThemeParameters::new();
        assert!(theme.is_empty());
        assert!(theme.background_color.is_none());
    }

    #[test]
    fn test_theme_parameters_from_json() {
        let json = "{\"background_color\":\"#ffffff\",\"text_color\":\"#000000\"}";
        let theme = ThemeParameters::from_json(json);
        assert!(theme.is_some());
        let theme = theme.unwrap();
        assert_eq!(theme.background_color, Some("#ffffff".to_string()));
        assert_eq!(theme.text_color, Some("#000000".to_string()));
    }

    #[test]
    fn test_theme_parameters_to_json_string() {
        let mut theme = ThemeParameters::new();
        theme.background_color = Some("#ffffff".to_string());
        let json = theme.to_json_string();
        assert!(json.is_ok());
        assert!(json.unwrap().contains("#ffffff"));
    }

    #[test]
    fn test_theme_parameters_is_empty() {
        let theme = ThemeParameters::new();
        assert!(theme.is_empty());

        let mut theme2 = ThemeParameters::new();
        theme2.background_color = Some("#ffffff".to_string());
        assert!(!theme2.is_empty());
    }

    #[test]
    fn test_web_app_mode_from_td_api() {
        assert_eq!(
            WebAppMode::from_td_api_mode("webAppOpenModeCompact"),
            WebAppMode::Compact
        );
        assert_eq!(
            WebAppMode::from_td_api_mode("webAppOpenModeFullScreen"),
            WebAppMode::FullScreen
        );
        assert_eq!(
            WebAppMode::from_td_api_mode("webAppOpenModeFullSize"),
            WebAppMode::FullSize
        );
        assert_eq!(
            WebAppMode::from_td_api_mode("unknown"),
            WebAppMode::FullSize
        );
    }

    #[test]
    fn test_theme_parameters_none() {
        let params = WebAppOpenParameters::default();
        assert!(params.theme_parameters().is_none());
        assert!(params.to_input_theme_parameters().is_none());
    }

    #[test]
    fn test_theme_parameters_some() {
        let mut theme = ThemeParameters::new();
        theme.background_color = Some("#ffffff".to_string());
        let params =
            WebAppOpenParameters::new(Some(theme), "App".to_string(), WebAppMode::FullSize);
        assert!(params.theme_parameters().is_some());
        assert!(params.to_input_theme_parameters().is_some());
    }

    #[test]
    fn test_display_no_theme() {
        let params = WebAppOpenParameters::new(None, "MyApp".to_string(), WebAppMode::Compact);
        assert_eq!(format!("{}", params), "WebApp[app: MyApp, compact]");
    }

    #[test]
    fn test_display_with_theme() {
        let theme = ThemeParameters::new();
        let params =
            WebAppOpenParameters::new(Some(theme), "App".to_string(), WebAppMode::FullScreen);
        assert_eq!(
            format!("{}", params),
            "WebApp[app: App, fullscreen, themed]"
        );
    }

    #[test]
    fn test_display_empty() {
        let params = WebAppOpenParameters::default();
        assert_eq!(format!("{}", params), "WebApp[]");
    }

    #[test]
    fn test_equality() {
        let theme1 = ThemeParameters::new();
        let params1 =
            WebAppOpenParameters::new(Some(theme1), "App".to_string(), WebAppMode::Compact);

        let theme2 = ThemeParameters::new();
        let params2 =
            WebAppOpenParameters::new(Some(theme2), "App".to_string(), WebAppMode::Compact);

        assert_eq!(params1, params2);
    }

    #[test]
    fn test_inequality() {
        let params1 = WebAppOpenParameters::new(None, "App1".to_string(), WebAppMode::Compact);
        let params2 = WebAppOpenParameters::new(None, "App2".to_string(), WebAppMode::Compact);
        assert_ne!(params1, params2);
    }

    #[test]
    fn test_cloning() {
        let theme = ThemeParameters::new();
        let params1 =
            WebAppOpenParameters::new(Some(theme), "App".to_string(), WebAppMode::Compact);
        let params2 = params1.clone();
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_mode_mutually_exclusive() {
        // Only one mode should be true at a time
        let compact = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::Compact);
        assert!(compact.is_compact() && !compact.is_full_screen());

        let fullscreen = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::FullScreen);
        assert!(!fullscreen.is_compact() && fullscreen.is_full_screen());

        let fullsize = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::FullSize);
        assert!(!fullsize.is_compact() && !fullsize.is_full_screen());
    }

    #[test]
    fn test_theme_parameters_serialization() {
        let mut theme = ThemeParameters::new();
        theme.background_color = Some("#ffffff".to_string());
        theme.text_color = Some("#000000".to_string());

        let json = theme.to_json_string().unwrap();
        let parsed = ThemeParameters::from_json(&json).unwrap();
        assert_eq!(theme, parsed);
    }

    #[test]
    fn test_web_app_open_parameters_mode() {
        let compact = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::Compact);
        assert_eq!(compact.mode(), WebAppMode::Compact);

        let fullscreen = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::FullScreen);
        assert_eq!(fullscreen.mode(), WebAppMode::FullScreen);

        let fullsize = WebAppOpenParameters::new(None, "App".to_string(), WebAppMode::FullSize);
        assert_eq!(fullsize.mode(), WebAppMode::FullSize);
    }
}
