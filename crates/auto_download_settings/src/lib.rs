// Copyright 2025 rustgram-client contributors
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

//! # Auto Download Settings
//!
//! Automatic media download configuration for different network types.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `AutoDownloadSettings` struct from `td/telegram/AutoDownloadSettings.h`.
//!
//! ## Structure
//!
//! - `AutoDownloadSettings`: Configuration for automatic media downloading
//! - `NetworkType`: Network type for which settings apply
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_auto_download_settings::AutoDownloadSettings;
//!
//! let settings = AutoDownloadSettings::new()
//!     .with_max_photo_file_size(1024 * 1024)
//!     .with_max_video_file_size(10 * 1024 * 1024)
//!     .with_enabled(true);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Network type for auto-download settings.
///
/// Corresponds to TDLib `NetType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum NetworkType {
    /// Mobile network (cellular)
    Mobile,
    /// WiFi network
    WiFi,
    /// Roaming network
    Roaming,
    /// Other networks (default)
    #[default]
    Other,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mobile => write!(f, "Mobile"),
            Self::WiFi => write!(f, "WiFi"),
            Self::Roaming => write!(f, "Roaming"),
            Self::Other => write!(f, "Other"),
        }
    }
}

/// Automatic media download settings.
///
/// Corresponds to TDLib `AutoDownloadSettings` struct.
/// Controls which media files are automatically downloaded based on size and type.
///
/// ## TDLib Mapping
///
/// - `max_photo_file_size` → `max_photo_file_size()`
/// - `max_video_file_size` → `max_video_file_size()`
/// - `max_other_file_size` → `max_other_file_size()`
/// - `video_upload_bitrate` → `video_upload_bitrate()`
/// - `is_enabled` → `is_enabled()`
/// - `preload_large_videos` → `preload_large_videos()`
/// - `preload_next_audio` → `preload_next_audio()`
/// - `preload_stories` → `preload_stories()`
/// - `use_less_data_for_calls` → `use_less_data_for_calls()`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoDownloadSettings {
    /// Maximum photo file size in bytes for auto-download
    max_photo_file_size: i32,
    /// Maximum video file size in bytes for auto-download
    max_video_file_size: i64,
    /// Maximum other file size in bytes for auto-download
    max_other_file_size: i64,
    /// Video upload bitrate in bits per second
    video_upload_bitrate: i32,
    /// Whether auto-download is enabled
    is_enabled: bool,
    /// Whether to preload large videos
    preload_large_videos: bool,
    /// Whether to preload next audio track
    preload_next_audio: bool,
    /// Whether to preload stories
    preload_stories: bool,
    /// Whether to use less data for calls
    use_less_data_for_calls: bool,
}

impl Default for AutoDownloadSettings {
    /// Creates default auto-download settings (all disabled).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::default();
    /// assert_eq!(settings.max_photo_file_size(), 0);
    /// assert!(!settings.is_enabled());
    /// ```
    fn default() -> Self {
        Self {
            max_photo_file_size: 0,
            max_video_file_size: 0,
            max_other_file_size: 0,
            video_upload_bitrate: 0,
            is_enabled: false,
            preload_large_videos: false,
            preload_next_audio: false,
            preload_stories: false,
            use_less_data_for_calls: false,
        }
    }
}

impl AutoDownloadSettings {
    /// Creates a new AutoDownloadSettings with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new();
    /// assert_eq!(settings.max_photo_file_size(), 0);
    /// assert!(!settings.is_enabled());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder pattern: sets maximum photo file size.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum photo file size in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_max_photo_file_size(1024 * 1024);
    /// assert_eq!(settings.max_photo_file_size(), 1024 * 1024);
    /// ```
    #[must_use]
    pub fn with_max_photo_file_size(mut self, size: i32) -> Self {
        self.max_photo_file_size = size;
        self
    }

    /// Builder pattern: sets maximum video file size.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum video file size in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_max_video_file_size(10 * 1024 * 1024);
    /// assert_eq!(settings.max_video_file_size(), 10 * 1024 * 1024);
    /// ```
    #[must_use]
    pub fn with_max_video_file_size(mut self, size: i64) -> Self {
        self.max_video_file_size = size;
        self
    }

    /// Builder pattern: sets maximum other file size.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum other file size in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_max_other_file_size(5 * 1024 * 1024);
    /// assert_eq!(settings.max_other_file_size(), 5 * 1024 * 1024);
    /// ```
    #[must_use]
    pub fn with_max_other_file_size(mut self, size: i64) -> Self {
        self.max_other_file_size = size;
        self
    }

    /// Builder pattern: sets video upload bitrate.
    ///
    /// # Arguments
    ///
    /// * `bitrate` - Video upload bitrate in bits per second
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_video_upload_bitrate(1000000);
    /// assert_eq!(settings.video_upload_bitrate(), 1000000);
    /// ```
    #[must_use]
    pub fn with_video_upload_bitrate(mut self, bitrate: i32) -> Self {
        self.video_upload_bitrate = bitrate;
        self
    }

    /// Builder pattern: sets whether auto-download is enabled.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable auto-download
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_enabled(true);
    /// assert!(settings.is_enabled());
    /// ```
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    /// Builder pattern: sets whether to preload large videos.
    ///
    /// # Arguments
    ///
    /// * `preload` - Whether to preload large videos
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_preload_large_videos(true);
    /// assert!(settings.preload_large_videos());
    /// ```
    #[must_use]
    pub fn with_preload_large_videos(mut self, preload: bool) -> Self {
        self.preload_large_videos = preload;
        self
    }

    /// Builder pattern: sets whether to preload next audio.
    ///
    /// # Arguments
    ///
    /// * `preload` - Whether to preload next audio track
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_preload_next_audio(true);
    /// assert!(settings.preload_next_audio());
    /// ```
    #[must_use]
    pub fn with_preload_next_audio(mut self, preload: bool) -> Self {
        self.preload_next_audio = preload;
        self
    }

    /// Builder pattern: sets whether to preload stories.
    ///
    /// # Arguments
    ///
    /// * `preload` - Whether to preload stories
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_preload_stories(true);
    /// assert!(settings.preload_stories());
    /// ```
    #[must_use]
    pub fn with_preload_stories(mut self, preload: bool) -> Self {
        self.preload_stories = preload;
        self
    }

    /// Builder pattern: sets whether to use less data for calls.
    ///
    /// # Arguments
    ///
    /// * `less_data` - Whether to use less data for calls
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_use_less_data_for_calls(true);
    /// assert!(settings.use_less_data_for_calls());
    /// ```
    #[must_use]
    pub fn with_use_less_data_for_calls(mut self, less_data: bool) -> Self {
        self.use_less_data_for_calls = less_data;
        self
    }

    /// Returns the maximum photo file size in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_max_photo_file_size(1024 * 1024);
    /// assert_eq!(settings.max_photo_file_size(), 1024 * 1024);
    /// ```
    #[must_use]
    pub const fn max_photo_file_size(&self) -> i32 {
        self.max_photo_file_size
    }

    /// Returns the maximum video file size in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_max_video_file_size(10 * 1024 * 1024);
    /// assert_eq!(settings.max_video_file_size(), 10 * 1024 * 1024);
    /// ```
    #[must_use]
    pub const fn max_video_file_size(&self) -> i64 {
        self.max_video_file_size
    }

    /// Returns the maximum other file size in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_max_other_file_size(5 * 1024 * 1024);
    /// assert_eq!(settings.max_other_file_size(), 5 * 1024 * 1024);
    /// ```
    #[must_use]
    pub const fn max_other_file_size(&self) -> i64 {
        self.max_other_file_size
    }

    /// Returns the video upload bitrate in bits per second.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_video_upload_bitrate(1000000);
    /// assert_eq!(settings.video_upload_bitrate(), 1000000);
    /// ```
    #[must_use]
    pub const fn video_upload_bitrate(&self) -> i32 {
        self.video_upload_bitrate
    }

    /// Returns whether auto-download is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_enabled(true);
    /// assert!(settings.is_enabled());
    /// ```
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Returns whether to preload large videos.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_preload_large_videos(true);
    /// assert!(settings.preload_large_videos());
    /// ```
    #[must_use]
    pub const fn preload_large_videos(&self) -> bool {
        self.preload_large_videos
    }

    /// Returns whether to preload next audio.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_preload_next_audio(true);
    /// assert!(settings.preload_next_audio());
    /// ```
    #[must_use]
    pub const fn preload_next_audio(&self) -> bool {
        self.preload_next_audio
    }

    /// Returns whether to preload stories.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_preload_stories(true);
    /// assert!(settings.preload_stories());
    /// ```
    #[must_use]
    pub const fn preload_stories(&self) -> bool {
        self.preload_stories
    }

    /// Returns whether to use less data for calls.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_use_less_data_for_calls(true);
    /// assert!(settings.use_less_data_for_calls());
    /// ```
    #[must_use]
    pub const fn use_less_data_for_calls(&self) -> bool {
        self.use_less_data_for_calls
    }

    /// Checks if auto-download is disabled (all sizes are 0 or disabled).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auto_download_settings::AutoDownloadSettings;
    ///
    /// let settings = AutoDownloadSettings::default();
    /// assert!(settings.is_disabled());
    ///
    /// let settings = AutoDownloadSettings::new()
    ///     .with_enabled(true);
    /// assert!(!settings.is_disabled());
    /// ```
    #[must_use]
    pub fn is_disabled(&self) -> bool {
        !self.is_enabled
            && self.max_photo_file_size == 0
            && self.max_video_file_size == 0
            && self.max_other_file_size == 0
    }
}

impl fmt::Display for AutoDownloadSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AutoDownloadSettings[enabled={}, max_photo={}, max_video={}, max_other={}]",
            self.is_enabled,
            self.max_photo_file_size,
            self.max_video_file_size,
            self.max_other_file_size
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests
    #[test]
    fn test_default() {
        let settings = AutoDownloadSettings::default();
        assert_eq!(settings.max_photo_file_size(), 0);
        assert_eq!(settings.max_video_file_size(), 0);
        assert_eq!(settings.max_other_file_size(), 0);
        assert_eq!(settings.video_upload_bitrate(), 0);
        assert!(!settings.is_enabled());
        assert!(!settings.preload_large_videos());
        assert!(!settings.preload_next_audio());
        assert!(!settings.preload_stories());
        assert!(!settings.use_less_data_for_calls());
    }

    #[test]
    fn test_clone() {
        let settings = AutoDownloadSettings::new().with_enabled(true);
        let cloned = settings.clone();
        assert_eq!(settings, cloned);
    }

    #[test]
    fn test_equality() {
        let settings1 = AutoDownloadSettings::new().with_enabled(true);
        let settings2 = AutoDownloadSettings::new().with_enabled(true);
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_inequality() {
        let settings1 = AutoDownloadSettings::new().with_enabled(true);
        let settings2 = AutoDownloadSettings::new().with_enabled(false);
        assert_ne!(settings1, settings2);
    }

    // NetworkType tests
    #[test]
    fn test_network_type_default() {
        let net_type = NetworkType::default();
        assert_eq!(net_type, NetworkType::Other);
    }

    #[test]
    fn test_network_type_display() {
        assert_eq!(format!("{}", NetworkType::Mobile), "Mobile");
        assert_eq!(format!("{}", NetworkType::WiFi), "WiFi");
        assert_eq!(format!("{}", NetworkType::Roaming), "Roaming");
        assert_eq!(format!("{}", NetworkType::Other), "Other");
    }

    // Builder pattern tests
    #[test]
    fn test_builder_max_photo_file_size() {
        let settings = AutoDownloadSettings::new().with_max_photo_file_size(1024 * 1024);
        assert_eq!(settings.max_photo_file_size(), 1024 * 1024);
    }

    #[test]
    fn test_builder_max_video_file_size() {
        let settings = AutoDownloadSettings::new().with_max_video_file_size(10 * 1024 * 1024);
        assert_eq!(settings.max_video_file_size(), 10 * 1024 * 1024);
    }

    #[test]
    fn test_builder_max_other_file_size() {
        let settings = AutoDownloadSettings::new().with_max_other_file_size(5 * 1024 * 1024);
        assert_eq!(settings.max_other_file_size(), 5 * 1024 * 1024);
    }

    #[test]
    fn test_builder_video_upload_bitrate() {
        let settings = AutoDownloadSettings::new().with_video_upload_bitrate(1000000);
        assert_eq!(settings.video_upload_bitrate(), 1000000);
    }

    #[test]
    fn test_builder_enabled() {
        let settings = AutoDownloadSettings::new().with_enabled(true);
        assert!(settings.is_enabled());
    }

    #[test]
    fn test_builder_preload_large_videos() {
        let settings = AutoDownloadSettings::new().with_preload_large_videos(true);
        assert!(settings.preload_large_videos());
    }

    #[test]
    fn test_builder_preload_next_audio() {
        let settings = AutoDownloadSettings::new().with_preload_next_audio(true);
        assert!(settings.preload_next_audio());
    }

    #[test]
    fn test_builder_preload_stories() {
        let settings = AutoDownloadSettings::new().with_preload_stories(true);
        assert!(settings.preload_stories());
    }

    #[test]
    fn test_builder_use_less_data_for_calls() {
        let settings = AutoDownloadSettings::new().with_use_less_data_for_calls(true);
        assert!(settings.use_less_data_for_calls());
    }

    // Chained builder tests
    #[test]
    fn test_builder_chain() {
        let settings = AutoDownloadSettings::new()
            .with_max_photo_file_size(1024 * 1024)
            .with_max_video_file_size(10 * 1024 * 1024)
            .with_max_other_file_size(5 * 1024 * 1024)
            .with_enabled(true);

        assert_eq!(settings.max_photo_file_size(), 1024 * 1024);
        assert_eq!(settings.max_video_file_size(), 10 * 1024 * 1024);
        assert_eq!(settings.max_other_file_size(), 5 * 1024 * 1024);
        assert!(settings.is_enabled());
    }

    // is_disabled tests
    #[test]
    fn test_is_disabled_default() {
        let settings = AutoDownloadSettings::default();
        assert!(settings.is_disabled());
    }

    #[test]
    fn test_is_disabled_when_enabled() {
        let settings = AutoDownloadSettings::new().with_enabled(true);
        assert!(!settings.is_disabled());
    }

    #[test]
    fn test_is_disabled_with_sizes() {
        let settings = AutoDownloadSettings::new()
            .with_max_photo_file_size(1024)
            .with_enabled(false);
        assert!(!settings.is_disabled());
    }

    // Display tests
    #[test]
    fn test_display() {
        let settings = AutoDownloadSettings::new()
            .with_enabled(true)
            .with_max_photo_file_size(1024);
        let display = format!("{}", settings);
        assert!(display.contains("enabled=true"));
        assert!(display.contains("max_photo=1024"));
    }

    // Serialization tests
    #[test]
    fn test_serialize_deserialize() {
        let settings = AutoDownloadSettings::new()
            .with_enabled(true)
            .with_max_photo_file_size(1024 * 1024)
            .with_max_video_file_size(10 * 1024 * 1024);
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: AutoDownloadSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    // Edge cases
    #[test]
    fn test_negative_sizes() {
        let settings = AutoDownloadSettings::new()
            .with_max_photo_file_size(-1)
            .with_max_video_file_size(-1)
            .with_max_other_file_size(-1);
        assert_eq!(settings.max_photo_file_size(), -1);
        assert_eq!(settings.max_video_file_size(), -1);
        assert_eq!(settings.max_other_file_size(), -1);
    }

    #[test]
    fn test_max_values() {
        let settings = AutoDownloadSettings::new()
            .with_max_photo_file_size(i32::MAX)
            .with_max_video_file_size(i64::MAX)
            .with_max_other_file_size(i64::MAX);
        assert_eq!(settings.max_photo_file_size(), i32::MAX);
        assert_eq!(settings.max_video_file_size(), i64::MAX);
        assert_eq!(settings.max_other_file_size(), i64::MAX);
    }

    // Boolean flag tests
    #[test]
    fn test_all_flags_true() {
        let settings = AutoDownloadSettings::new()
            .with_enabled(true)
            .with_preload_large_videos(true)
            .with_preload_next_audio(true)
            .with_preload_stories(true)
            .with_use_less_data_for_calls(true);

        assert!(settings.is_enabled());
        assert!(settings.preload_large_videos());
        assert!(settings.preload_next_audio());
        assert!(settings.preload_stories());
        assert!(settings.use_less_data_for_calls());
    }

    #[test]
    fn test_all_flags_false() {
        let settings = AutoDownloadSettings::new()
            .with_enabled(false)
            .with_preload_large_videos(false)
            .with_preload_next_audio(false)
            .with_preload_stories(false)
            .with_use_less_data_for_calls(false);

        assert!(!settings.is_enabled());
        assert!(!settings.preload_large_videos());
        assert!(!settings.preload_next_audio());
        assert!(!settings.preload_stories());
        assert!(!settings.use_less_data_for_calls());
    }

    // Preset scenarios
    #[test]
    fn test_mobile_preset() {
        // Mobile: conservative settings to save data
        let settings = AutoDownloadSettings::new()
            .with_max_photo_file_size(512 * 1024) // 512 KB
            .with_max_video_file_size(5 * 1024 * 1024) // 5 MB
            .with_max_other_file_size(2 * 1024 * 1024) // 2 MB
            .with_enabled(true)
            .with_use_less_data_for_calls(true);

        assert!(settings.is_enabled());
        assert!(settings.use_less_data_for_calls());
        assert_eq!(settings.max_photo_file_size(), 512 * 1024);
    }

    #[test]
    fn test_wifi_preset() {
        // WiFi: liberal settings
        let settings = AutoDownloadSettings::new()
            .with_max_photo_file_size(10 * 1024 * 1024) // 10 MB
            .with_max_video_file_size(100 * 1024 * 1024) // 100 MB
            .with_max_other_file_size(50 * 1024 * 1024) // 50 MB
            .with_enabled(true)
            .with_preload_large_videos(true)
            .with_preload_next_audio(true);

        assert!(settings.is_enabled());
        assert!(settings.preload_large_videos());
        assert!(settings.preload_next_audio());
    }

    #[test]
    fn test_roaming_preset() {
        // Roaming: most conservative settings
        let settings = AutoDownloadSettings::new()
            .with_max_photo_file_size(0) // Disabled
            .with_max_video_file_size(0)
            .with_max_other_file_size(0)
            .with_enabled(false);

        assert!(settings.is_disabled());
    }
}
