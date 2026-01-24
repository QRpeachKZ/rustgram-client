//! # Global Privacy Settings
//!
//! Privacy settings for Telegram account.
//!
//! ## Overview
//!
//! `GlobalPrivacySettings` manages privacy settings that apply globally
//! to the Telegram account, including archiving, read dates, and new chat settings.
//!
//! ## Usage
//!
//! ```
//! use rustgram_global_privacy_settings::GlobalPrivacySettings;
//!
//! let settings = GlobalPrivacySettings::new();
//! assert!(!settings.archive_and_mute_new_noncontact_peers());
//! ```

use rustgram_star_gift_settings::StarGiftSettings;
use std::vec::Vec;

/// Global privacy settings for a Telegram account.
///
/// These settings control how the account interacts with new contacts,
/// archived chats, and read receipts.
///
/// # Examples
///
/// ```
/// use rustgram_global_privacy_settings::GlobalPrivacySettings;
///
/// let settings = GlobalPrivacySettings::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalPrivacySettings {
    archive_and_mute_new_noncontact_peers: bool,
    keep_archived_unmuted: bool,
    keep_archived_folders: bool,
    hide_read_marks: bool,
    new_noncontact_peers_require_premium: bool,
    noncontact_peers_paid_star_count: i64,
    gift_settings: StarGiftSettings,
}

impl GlobalPrivacySettings {
    /// Creates a new global privacy settings with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            archive_and_mute_new_noncontact_peers: false,
            keep_archived_unmuted: false,
            keep_archived_folders: false,
            hide_read_marks: false,
            new_noncontact_peers_require_premium: false,
            noncontact_peers_paid_star_count: 0,
            gift_settings: StarGiftSettings::new(),
        }
    }

    /// Returns `true` if new non-contact peers should be archived and muted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// assert!(!settings.archive_and_mute_new_noncontact_peers());
    /// ```
    #[inline]
    pub const fn archive_and_mute_new_noncontact_peers(&self) -> bool {
        self.archive_and_mute_new_noncontact_peers
    }

    /// Returns `true` if archived chats should remain unmuted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// assert!(!settings.keep_archived_unmuted());
    /// ```
    #[inline]
    pub const fn keep_archived_unmuted(&self) -> bool {
        self.keep_archived_unmuted
    }

    /// Returns `true` if archived folders should be kept.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// assert!(!settings.keep_archived_folders());
    /// ```
    #[inline]
    pub const fn keep_archived_folders(&self) -> bool {
        self.keep_archived_folders
    }

    /// Returns `true` if read marks should be hidden.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// assert!(!settings.hide_read_marks());
    /// ```
    #[inline]
    pub const fn hide_read_marks(&self) -> bool {
        self.hide_read_marks
    }

    /// Returns `true` if premium is required for new non-contact peers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// assert!(!settings.new_noncontact_peers_require_premium());
    /// ```
    #[inline]
    pub const fn new_noncontact_peers_require_premium(&self) -> bool {
        self.new_noncontact_peers_require_premium
    }

    /// Returns the star count required for non-contact peers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// assert_eq!(settings.noncontact_peers_paid_star_count(), 0);
    /// ```
    #[inline]
    pub const fn noncontact_peers_paid_star_count(&self) -> i64 {
        self.noncontact_peers_paid_star_count
    }

    /// Returns the star gift settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let settings = GlobalPrivacySettings::new();
    /// let gift_settings = settings.gift_settings();
    /// ```
    #[inline]
    pub const fn gift_settings(&self) -> &StarGiftSettings {
        &self.gift_settings
    }

    /// Sets whether new non-contact peers should be archived and muted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let mut settings = GlobalPrivacySettings::new();
    /// settings.set_archive_and_mute_new_noncontact_peers(true);
    /// assert!(settings.archive_and_mute_new_noncontact_peers());
    /// ```
    #[inline]
    pub fn set_archive_and_mute_new_noncontact_peers(&mut self, value: bool) {
        self.archive_and_mute_new_noncontact_peers = value;
    }

    /// Sets whether archived chats should remain unmuted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let mut settings = GlobalPrivacySettings::new();
    /// settings.set_keep_archived_unmuted(true);
    /// assert!(settings.keep_archived_unmuted());
    /// ```
    #[inline]
    pub fn set_keep_archived_unmuted(&mut self, value: bool) {
        self.keep_archived_unmuted = value;
    }

    /// Sets whether archived folders should be kept.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let mut settings = GlobalPrivacySettings::new();
    /// settings.set_keep_archived_folders(true);
    /// assert!(settings.keep_archived_folders());
    /// ```
    #[inline]
    pub fn set_keep_archived_folders(&mut self, value: bool) {
        self.keep_archived_folders = value;
    }

    /// Sets whether read marks should be hidden.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let mut settings = GlobalPrivacySettings::new();
    /// settings.set_hide_read_marks(true);
    /// assert!(settings.hide_read_marks());
    /// ```
    #[inline]
    pub fn set_hide_read_marks(&mut self, value: bool) {
        self.hide_read_marks = value;
    }

    /// Sets whether premium is required for new non-contact peers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let mut settings = GlobalPrivacySettings::new();
    /// settings.set_new_noncontact_peers_require_premium(true);
    /// assert!(settings.new_noncontact_peers_require_premium());
    /// ```
    #[inline]
    pub fn set_new_noncontact_peers_require_premium(&mut self, value: bool) {
        self.new_noncontact_peers_require_premium = value;
    }

    /// Sets the star count required for non-contact peers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let mut settings = GlobalPrivacySettings::new();
    /// settings.set_noncontact_peers_paid_star_count(100);
    /// assert_eq!(settings.noncontact_peers_paid_star_count(), 100);
    /// ```
    #[inline]
    pub fn set_noncontact_peers_paid_star_count(&mut self, value: i64) {
        self.noncontact_peers_paid_star_count = value;
    }

    /// Applies changes from another settings instance.
    ///
    /// # Arguments
    ///
    /// * `other` - The settings to apply changes from
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global_privacy_settings::GlobalPrivacySettings;
    ///
    /// let mut settings = GlobalPrivacySettings::new();
    /// let other = GlobalPrivacySettings {
    ///     archive_and_mute_new_noncontact_peers: true,
    ///     ..Default::default()
    /// };
    /// settings.apply_changes(&other);
    /// assert!(settings.archive_and_mute_new_noncontact_peers());
    /// ```
    pub fn apply_changes(&mut self, other: &GlobalPrivacySettings) {
        self.archive_and_mute_new_noncontact_peers = other.archive_and_mute_new_noncontact_peers;
        self.keep_archived_unmuted = other.keep_archived_unmuted;
        self.keep_archived_folders = other.keep_archived_folders;
        self.hide_read_marks = other.hide_read_marks;
        self.new_noncontact_peers_require_premium = other.new_noncontact_peers_require_premium;
        self.noncontact_peers_paid_star_count = other.noncontact_peers_paid_star_count;
        self.gift_settings = other.gift_settings.clone();
    }
}

impl Default for GlobalPrivacySettings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let settings = GlobalPrivacySettings::new();
        assert!(!settings.archive_and_mute_new_noncontact_peers());
        assert!(!settings.keep_archived_unmuted());
        assert!(!settings.keep_archived_folders());
        assert!(!settings.hide_read_marks());
        assert!(!settings.new_noncontact_peers_require_premium());
        assert_eq!(settings.noncontact_peers_paid_star_count(), 0);
    }

    #[test]
    fn test_archive_and_mute_new_noncontact_peers() {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_archive_and_mute_new_noncontact_peers(true);
        assert!(settings.archive_and_mute_new_noncontact_peers());
    }

    #[test]
    fn test_keep_archived_unmuted() {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_keep_archived_unmuted(true);
        assert!(settings.keep_archived_unmuted());
    }

    #[test]
    fn test_keep_archived_folders() {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_keep_archived_folders(true);
        assert!(settings.keep_archived_folders());
    }

    #[test]
    fn test_hide_read_marks() {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_hide_read_marks(true);
        assert!(settings.hide_read_marks());
    }

    #[test]
    fn test_new_noncontact_peers_require_premium() {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_new_noncontact_peers_require_premium(true);
        assert!(settings.new_noncontact_peers_require_premium());
    }

    #[test]
    fn test_noncontact_peers_paid_star_count() {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_noncontact_peers_paid_star_count(100);
        assert_eq!(settings.noncontact_peers_paid_star_count(), 100);
    }

    #[test]
    fn test_gift_settings() {
        let settings = GlobalPrivacySettings::new();
        let gift_settings = settings.gift_settings();
        // Just verify we can access it
        let _ = gift_settings;
    }

    #[test]
    fn test_apply_changes() {
        let mut settings = GlobalPrivacySettings::new();
        let mut other = GlobalPrivacySettings::new();
        other.set_archive_and_mute_new_noncontact_peers(true);
        other.set_hide_read_marks(true);
        other.set_noncontact_peers_paid_star_count(500);

        settings.apply_changes(&other);

        assert!(settings.archive_and_mute_new_noncontact_peers());
        assert!(settings.hide_read_marks());
        assert_eq!(settings.noncontact_peers_paid_star_count(), 500);
    }

    #[test]
    fn test_default() {
        let settings = GlobalPrivacySettings::default();
        assert!(!settings.archive_and_mute_new_noncontact_peers());
    }

    #[test]
    fn test_clone() {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_archive_and_mute_new_noncontact_peers(true);
        let cloned = settings.clone();
        assert_eq!(settings, cloned);
    }

    #[test]
    fn test_equality() {
        let settings1 = GlobalPrivacySettings::new();
        let settings2 = GlobalPrivacySettings::new();
        assert_eq!(settings1, settings2);

        let mut settings3 = GlobalPrivacySettings::new();
        settings3.set_archive_and_mute_new_noncontact_peers(true);
        assert_ne!(settings1, settings3);
    }

    #[rstest]
    #[case(true, false, false, false, false, 0)]
    #[case(false, true, false, false, false, 0)]
    #[case(false, false, true, false, false, 0)]
    #[case(false, false, false, true, false, 0)]
    #[case(false, false, false, false, true, 0)]
    #[case(false, false, false, false, false, 100)]
    fn test_settings_combinations(
        #[case] archive: bool,
        #[case] unmuted: bool,
        #[case] folders: bool,
        #[case] hide: bool,
        #[case] premium: bool,
        #[case] stars: i64,
    ) {
        let mut settings = GlobalPrivacySettings::new();
        settings.set_archive_and_mute_new_noncontact_peers(archive);
        settings.set_keep_archived_unmuted(unmuted);
        settings.set_keep_archived_folders(folders);
        settings.set_hide_read_marks(hide);
        settings.set_new_noncontact_peers_require_premium(premium);
        settings.set_noncontact_peers_paid_star_count(stars);

        assert_eq!(settings.archive_and_mute_new_noncontact_peers(), archive);
        assert_eq!(settings.keep_archived_unmuted(), unmuted);
        assert_eq!(settings.keep_archived_folders(), folders);
        assert_eq!(settings.hide_read_marks(), hide);
        assert_eq!(settings.new_noncontact_peers_require_premium(), premium);
        assert_eq!(settings.noncontact_peers_paid_star_count(), stars);
    }
}
