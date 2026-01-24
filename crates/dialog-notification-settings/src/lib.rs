//! Dialog notification settings.
//!
//! This module provides the `DialogNotificationSettings` type, which represents
//! notification preferences for a dialog.
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogNotificationSettings` (td/telegram/DialogNotificationSettings.h)
//! - Contains mute_until, sound, show_preview, and various flags
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_notification_settings::DialogNotificationSettings;
//!
//! // Create default notification settings
//! let settings = DialogNotificationSettings::new();
//! assert_eq!(settings.mute_for(), 0);
//! assert!(settings.use_default_mute_until());
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Dialog notification settings.
///
/// Contains all notification preferences for a dialog, including muting,
/// sound settings, and preview options.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_notification_settings::DialogNotificationSettings;
///
/// let settings = DialogNotificationSettings::new();
/// assert!(settings.use_default_mute_until());
/// assert!(settings.show_preview());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogNotificationSettings {
    /// Timestamp until which notifications are muted (0 = not muted).
    mute_until: i32,
    /// Whether to use default mute_until setting.
    use_default_mute_until: bool,
    /// Whether to show message previews.
    show_preview: bool,
    /// Whether to use default show_preview setting.
    use_default_show_preview: bool,
    /// Whether to mute stories.
    mute_stories: bool,
    /// Whether to use default mute_stories setting.
    use_default_mute_stories: bool,
    /// Whether to hide story sender.
    hide_story_sender: bool,
    /// Whether to use default hide_story_sender setting.
    use_default_hide_story_sender: bool,
    /// Whether to send messages silently.
    silent_send_message: bool,
}

impl DialogNotificationSettings {
    /// Creates default notification settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(settings.use_default_mute_until());
    /// assert!(settings.show_preview());
    /// ```
    pub fn new() -> Self {
        Self {
            mute_until: 0,
            use_default_mute_until: true,
            show_preview: true,
            use_default_show_preview: true,
            mute_stories: false,
            use_default_mute_stories: true,
            hide_story_sender: false,
            use_default_hide_story_sender: true,
            silent_send_message: false,
        }
    }

    /// Returns the mute duration in seconds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_mute_for(3600);
    /// assert_eq!(settings.mute_for(), 3600);
    /// ```
    pub fn mute_for(&self) -> i32 {
        self.mute_until
    }

    /// Checks if using default mute_until setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(settings.use_default_mute_until());
    /// ```
    pub fn use_default_mute_until(&self) -> bool {
        self.use_default_mute_until
    }

    /// Checks if message previews should be shown.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(settings.show_preview());
    /// ```
    pub fn show_preview(&self) -> bool {
        self.show_preview
    }

    /// Checks if using default show_preview setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(settings.use_default_show_preview());
    /// ```
    pub fn use_default_show_preview(&self) -> bool {
        self.use_default_show_preview
    }

    /// Checks if stories should be muted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(!settings.mute_stories());
    /// ```
    pub fn mute_stories(&self) -> bool {
        self.mute_stories
    }

    /// Checks if using default mute_stories setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(settings.use_default_mute_stories());
    /// ```
    pub fn use_default_mute_stories(&self) -> bool {
        self.use_default_mute_stories
    }

    /// Checks if story sender should be hidden.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(!settings.hide_story_sender());
    /// ```
    pub fn hide_story_sender(&self) -> bool {
        self.hide_story_sender
    }

    /// Checks if using default hide_story_sender setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(settings.use_default_hide_story_sender());
    /// ```
    pub fn use_default_hide_story_sender(&self) -> bool {
        self.use_default_hide_story_sender
    }

    /// Checks if messages should be sent silently.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let settings = DialogNotificationSettings::new();
    /// assert!(!settings.silent_send_message());
    /// ```
    pub fn silent_send_message(&self) -> bool {
        self.silent_send_message
    }

    /// Sets the mute duration in seconds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_mute_for(7200);
    /// assert_eq!(settings.mute_for(), 7200);
    /// ```
    pub fn set_mute_for(&mut self, seconds: i32) {
        self.mute_until = seconds.max(0);
    }

    /// Sets whether to use default mute_until setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_use_default_mute_until(false);
    /// assert!(!settings.use_default_mute_until());
    /// ```
    pub fn set_use_default_mute_until(&mut self, value: bool) {
        self.use_default_mute_until = value;
    }

    /// Sets whether to show message previews.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_show_preview(false);
    /// assert!(!settings.show_preview());
    /// ```
    pub fn set_show_preview(&mut self, value: bool) {
        self.show_preview = value;
    }

    /// Sets whether to use default show_preview setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_use_default_show_preview(false);
    /// assert!(!settings.use_default_show_preview());
    /// ```
    pub fn set_use_default_show_preview(&mut self, value: bool) {
        self.use_default_show_preview = value;
    }

    /// Sets whether to mute stories.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_mute_stories(true);
    /// assert!(settings.mute_stories());
    /// ```
    pub fn set_mute_stories(&mut self, value: bool) {
        self.mute_stories = value;
    }

    /// Sets whether to use default mute_stories setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_use_default_mute_stories(false);
    /// assert!(!settings.use_default_mute_stories());
    /// ```
    pub fn set_use_default_mute_stories(&mut self, value: bool) {
        self.use_default_mute_stories = value;
    }

    /// Sets whether to hide story sender.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_hide_story_sender(true);
    /// assert!(settings.hide_story_sender());
    /// ```
    pub fn set_hide_story_sender(&mut self, value: bool) {
        self.hide_story_sender = value;
    }

    /// Sets whether to use default hide_story_sender setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_use_default_hide_story_sender(false);
    /// assert!(!settings.use_default_hide_story_sender());
    /// ```
    pub fn set_use_default_hide_story_sender(&mut self, value: bool) {
        self.use_default_hide_story_sender = value;
    }

    /// Sets whether to send messages silently.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_notification_settings::DialogNotificationSettings;
    ///
    /// let mut settings = DialogNotificationSettings::new();
    /// settings.set_silent_send_message(true);
    /// assert!(settings.silent_send_message());
    /// ```
    pub fn set_silent_send_message(&mut self, value: bool) {
        self.silent_send_message = value;
    }
}

impl Default for DialogNotificationSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DialogNotificationSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NotificationSettings(mute_until={}, show_preview={}, mute_stories={})",
            self.mute_until, self.show_preview, self.mute_stories
        )
    }
}

impl Serialize for DialogNotificationSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            self.mute_until,
            self.use_default_mute_until,
            self.show_preview,
            self.use_default_show_preview,
            self.mute_stories,
            self.use_default_mute_stories,
            self.hide_story_sender,
            self.use_default_hide_story_sender,
            self.silent_send_message,
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DialogNotificationSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (
            mute_until,
            use_default_mute_until,
            show_preview,
            use_default_show_preview,
            mute_stories,
            use_default_mute_stories,
            hide_story_sender,
            use_default_hide_story_sender,
            silent_send_message,
        ) = <(i32, bool, bool, bool, bool, bool, bool, bool, bool)>::deserialize(deserializer)?;

        Ok(Self {
            mute_until,
            use_default_mute_until,
            show_preview,
            use_default_show_preview,
            mute_stories,
            use_default_mute_stories,
            hide_story_sender,
            use_default_hide_story_sender,
            silent_send_message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug() {
        let settings = DialogNotificationSettings::new();
        assert!(format!("{:?}", settings).contains("DialogNotificationSettings"));
    }

    #[test]
    fn test_clone() {
        let settings = DialogNotificationSettings::new();
        let cloned = settings.clone();
        assert_eq!(settings, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let s1 = DialogNotificationSettings::new();
        let s2 = DialogNotificationSettings::new();
        assert_eq!(s1, s2);

        let mut s3 = DialogNotificationSettings::new();
        s3.set_mute_for(100);
        assert_ne!(s1, s3);
    }

    #[test]
    fn test_default() {
        let settings = DialogNotificationSettings::default();
        assert!(settings.use_default_mute_until());
    }

    #[test]
    fn test_display() {
        let settings = DialogNotificationSettings::new();
        let display = format!("{}", settings);
        assert!(display.contains("NotificationSettings"));
    }

    // Constructor tests (1 * 2 = 2)
    #[test]
    fn test_new() {
        let settings = DialogNotificationSettings::new();
        assert_eq!(settings.mute_for(), 0);
        assert!(settings.use_default_mute_until());
        assert!(settings.show_preview());
        assert!(settings.use_default_show_preview());
        assert!(!settings.mute_stories());
        assert!(settings.use_default_mute_stories());
        assert!(!settings.hide_story_sender());
        assert!(settings.use_default_hide_story_sender());
        assert!(!settings.silent_send_message());
    }

    // Getter tests (9 * 2 = 18)
    #[test]
    fn test_mute_for() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_mute_for(3600);
        assert_eq!(settings.mute_for(), 3600);
    }

    #[test]
    fn test_use_default_mute_until() {
        let settings = DialogNotificationSettings::new();
        assert!(settings.use_default_mute_until());
    }

    #[test]
    fn test_show_preview() {
        let settings = DialogNotificationSettings::new();
        assert!(settings.show_preview());
    }

    #[test]
    fn test_use_default_show_preview() {
        let settings = DialogNotificationSettings::new();
        assert!(settings.use_default_show_preview());
    }

    #[test]
    fn test_mute_stories() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_mute_stories(true);
        assert!(settings.mute_stories());
    }

    #[test]
    fn test_use_default_mute_stories() {
        let settings = DialogNotificationSettings::new();
        assert!(settings.use_default_mute_stories());
    }

    #[test]
    fn test_hide_story_sender() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_hide_story_sender(true);
        assert!(settings.hide_story_sender());
    }

    #[test]
    fn test_use_default_hide_story_sender() {
        let settings = DialogNotificationSettings::new();
        assert!(settings.use_default_hide_story_sender());
    }

    #[test]
    fn test_silent_send_message() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_silent_send_message(true);
        assert!(settings.silent_send_message());
    }

    // Setter tests (9 * 2 = 18)
    #[test]
    fn test_set_mute_for() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_mute_for(7200);
        assert_eq!(settings.mute_for(), 7200);
    }

    #[test]
    fn test_set_mute_for_clamps_negative() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_mute_for(-100);
        assert_eq!(settings.mute_for(), 0);
    }

    #[test]
    fn test_set_use_default_mute_until() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_use_default_mute_until(false);
        assert!(!settings.use_default_mute_until());
    }

    #[test]
    fn test_set_show_preview() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_show_preview(false);
        assert!(!settings.show_preview());
    }

    #[test]
    fn test_set_use_default_show_preview() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_use_default_show_preview(false);
        assert!(!settings.use_default_show_preview());
    }

    #[test]
    fn test_set_mute_stories() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_mute_stories(true);
        assert!(settings.mute_stories());
    }

    #[test]
    fn test_set_use_default_mute_stories() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_use_default_mute_stories(false);
        assert!(!settings.use_default_mute_stories());
    }

    #[test]
    fn test_set_hide_story_sender() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_hide_story_sender(true);
        assert!(settings.hide_story_sender());
    }

    #[test]
    fn test_set_use_default_hide_story_sender() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_use_default_hide_story_sender(false);
        assert!(!settings.use_default_hide_story_sender());
    }

    #[test]
    fn test_set_silent_send_message() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_silent_send_message(true);
        assert!(settings.silent_send_message());
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_mute_for(3600);
        settings.set_show_preview(false);
        settings.set_mute_stories(true);

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: DialogNotificationSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_default() {
        let settings = DialogNotificationSettings::new();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: DialogNotificationSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    // Edge cases (2)
    #[test]
    fn test_large_mute_duration() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_mute_for(i32::MAX);
        assert_eq!(settings.mute_for(), i32::MAX);
    }

    #[test]
    fn test_all_flags_false() {
        let mut settings = DialogNotificationSettings::new();
        settings.set_show_preview(false);
        settings.set_mute_stories(true);
        settings.set_hide_story_sender(true);
        settings.set_silent_send_message(true);
        settings.set_use_default_mute_until(false);
        settings.set_use_default_show_preview(false);
        settings.set_use_default_mute_stories(false);
        settings.set_use_default_hide_story_sender(false);

        assert!(!settings.show_preview());
        assert!(settings.mute_stories());
        assert!(settings.hide_story_sender());
        assert!(settings.silent_send_message());
    }

    // Doc test example (1)
    #[test]
    fn test_doc_example() {
        let settings = DialogNotificationSettings::new();
        assert_eq!(settings.mute_for(), 0);
        assert!(settings.use_default_mute_until());
    }
}
