// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Profile tab enumeration for Telegram MTProto client.
//!
//! This module implements TDLib's ProfileTab.
//!
//! # Example
//!
//! ```rust
//! use rustgram_profile_tab::ProfileTab;
//!
//! let tab = ProfileTab::Media;
//! assert!(tab.is_media());
//! assert_eq!(tab.name(), "media");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_channel_type::ChannelType;
use std::fmt::{self, Display, Formatter};

/// Profile tab.
///
/// Based on TDLib's `ProfileTab` enum.
///
/// Represents the different tabs available in a Telegram profile.
///
/// # Example
///
/// ```rust
/// use rustgram_profile_tab::ProfileTab;
///
/// let media = ProfileTab::Media;
/// assert!(media.is_media());
/// assert!(!media.is_posts());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum ProfileTab {
    /// Default/first tab
    Default = 0,

    /// Posts tab
    Posts = 1,

    /// Gifts tab
    Gifts = 2,

    /// Media tab
    Media = 3,

    /// Files tab
    Files = 4,

    /// Music tab
    Music = 5,

    /// Voice tab
    Voice = 6,

    /// Links tab
    Links = 7,

    /// Gifs tab
    Gifs = 8,

    /// Unknown tab
    #[default]
    Unknown = 9,
}

impl ProfileTab {
    /// Creates a new ProfileTab from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(ProfileTab)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert_eq!(ProfileTab::from_i32(0), Some(ProfileTab::Default));
    /// assert_eq!(ProfileTab::from_i32(3), Some(ProfileTab::Media));
    /// assert_eq!(ProfileTab::from_i32(99), Some(ProfileTab::Unknown));
    /// ```
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::Posts),
            2 => Some(Self::Gifts),
            3 => Some(Self::Media),
            4 => Some(Self::Files),
            5 => Some(Self::Music),
            6 => Some(Self::Voice),
            7 => Some(Self::Links),
            8 => Some(Self::Gifs),
            _ => Some(Self::Unknown),
        }
    }

    /// Returns the i32 representation of this profile tab.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert_eq!(ProfileTab::Default.to_i32(), 0);
    /// assert_eq!(ProfileTab::Media.to_i32(), 3);
    /// ```
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns the name of this profile tab.
    ///
    /// # Returns
    ///
    /// Returns a lowercase string representation of the tab.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert_eq!(ProfileTab::Default.name(), "default");
    /// assert_eq!(ProfileTab::Media.name(), "media");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Posts => "posts",
            Self::Gifts => "gifts",
            Self::Media => "media",
            Self::Files => "files",
            Self::Music => "music",
            Self::Voice => "voice",
            Self::Links => "links",
            Self::Gifs => "gifs",
            Self::Unknown => "unknown",
        }
    }

    /// Checks if this is the default tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the default tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Default.is_default());
    /// assert!(!ProfileTab::Media.is_default());
    /// ```
    pub fn is_default(self) -> bool {
        matches!(self, Self::Default)
    }

    /// Checks if this is the posts tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the posts tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Posts.is_posts());
    /// assert!(!ProfileTab::Media.is_posts());
    /// ```
    pub fn is_posts(self) -> bool {
        matches!(self, Self::Posts)
    }

    /// Checks if this is the gifts tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the gifts tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Gifts.is_gifts());
    /// assert!(!ProfileTab::Media.is_gifts());
    /// ```
    pub fn is_gifts(self) -> bool {
        matches!(self, Self::Gifts)
    }

    /// Checks if this is the media tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the media tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Media.is_media());
    /// assert!(!ProfileTab::Posts.is_media());
    /// ```
    pub fn is_media(self) -> bool {
        matches!(self, Self::Media)
    }

    /// Checks if this is the files tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the files tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Files.is_files());
    /// assert!(!ProfileTab::Media.is_files());
    /// ```
    pub fn is_files(self) -> bool {
        matches!(self, Self::Files)
    }

    /// Checks if this is the music tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the music tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Music.is_music());
    /// assert!(!ProfileTab::Media.is_music());
    /// ```
    pub fn is_music(self) -> bool {
        matches!(self, Self::Music)
    }

    /// Checks if this is the voice tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the voice tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Voice.is_voice());
    /// assert!(!ProfileTab::Media.is_voice());
    /// ```
    pub fn is_voice(self) -> bool {
        matches!(self, Self::Voice)
    }

    /// Checks if this is the links tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the links tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Links.is_links());
    /// assert!(!ProfileTab::Media.is_links());
    /// ```
    pub fn is_links(self) -> bool {
        matches!(self, Self::Links)
    }

    /// Checks if this is the gifs tab.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the gifs tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Gifs.is_gifs());
    /// assert!(!ProfileTab::Media.is_gifs());
    /// ```
    pub fn is_gifs(self) -> bool {
        matches!(self, Self::Gifs)
    }

    /// Checks if this tab is unknown.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is an unknown tab, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Unknown.is_unknown());
    /// assert!(!ProfileTab::Media.is_unknown());
    /// ```
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Checks if this tab is supported.
    ///
    /// # Returns
    ///
    /// Returns `true` if the tab is supported, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Media.is_supported());
    /// assert!(!ProfileTab::Unknown.is_supported());
    /// ```
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Checks if this tab is media-related (media, voice, gifs).
    ///
    /// # Returns
    ///
    /// Returns `true` if the tab is media-related, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Media.is_media_related());
    /// assert!(ProfileTab::Voice.is_media_related());
    /// assert!(ProfileTab::Gifs.is_media_related());
    /// assert!(!ProfileTab::Links.is_media_related());
    /// ```
    pub fn is_media_related(self) -> bool {
        matches!(self, Self::Media | Self::Voice | Self::Gifs)
    }

    /// Checks if this tab contains downloadable content (files, music, voice).
    ///
    /// # Returns
    ///
    /// Returns `true` if the tab contains downloadable content, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    ///
    /// assert!(ProfileTab::Files.is_downloadable());
    /// assert!(ProfileTab::Music.is_downloadable());
    /// assert!(!ProfileTab::Links.is_downloadable());
    /// ```
    pub fn is_downloadable(self) -> bool {
        matches!(self, Self::Files | Self::Music | Self::Voice)
    }

    /// Returns the set of tabs available for a given channel type.
    ///
    /// # Arguments
    ///
    /// * `channel_type` - The channel type
    ///
    /// # Returns
    ///
    /// Returns a vector of available tabs for the channel type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_profile_tab::ProfileTab;
    /// use rustgram_channel_type::ChannelType;
    ///
    /// let tabs = ProfileTab::tabs_for_channel_type(ChannelType::Broadcast);
    /// assert!(!tabs.is_empty());
    /// ```
    pub fn tabs_for_channel_type(channel_type: ChannelType) -> Vec<Self> {
        match channel_type {
            ChannelType::Broadcast => vec![Self::Default, Self::Posts, Self::Media, Self::Files],
            ChannelType::Megagroup => vec![
                Self::Default,
                Self::Media,
                Self::Files,
                Self::Music,
                Self::Voice,
                Self::Links,
            ],
            ChannelType::Unknown => vec![Self::Default],
        }
    }
}

impl Display for ProfileTab {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(ProfileTab::from_i32(0), Some(ProfileTab::Default));
        assert_eq!(ProfileTab::from_i32(1), Some(ProfileTab::Posts));
        assert_eq!(ProfileTab::from_i32(2), Some(ProfileTab::Gifts));
        assert_eq!(ProfileTab::from_i32(3), Some(ProfileTab::Media));
        assert_eq!(ProfileTab::from_i32(4), Some(ProfileTab::Files));
        assert_eq!(ProfileTab::from_i32(5), Some(ProfileTab::Music));
        assert_eq!(ProfileTab::from_i32(6), Some(ProfileTab::Voice));
        assert_eq!(ProfileTab::from_i32(7), Some(ProfileTab::Links));
        assert_eq!(ProfileTab::from_i32(8), Some(ProfileTab::Gifs));
        assert_eq!(ProfileTab::from_i32(99), Some(ProfileTab::Unknown));
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(ProfileTab::Default.to_i32(), 0);
        assert_eq!(ProfileTab::Posts.to_i32(), 1);
        assert_eq!(ProfileTab::Gifts.to_i32(), 2);
        assert_eq!(ProfileTab::Media.to_i32(), 3);
        assert_eq!(ProfileTab::Files.to_i32(), 4);
        assert_eq!(ProfileTab::Music.to_i32(), 5);
        assert_eq!(ProfileTab::Voice.to_i32(), 6);
        assert_eq!(ProfileTab::Links.to_i32(), 7);
        assert_eq!(ProfileTab::Gifs.to_i32(), 8);
        assert_eq!(ProfileTab::Unknown.to_i32(), 9);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for value in 0i32..=8 {
            let tab = ProfileTab::from_i32(value);
            assert_eq!(tab.map(|t| t.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_name() {
        assert_eq!(ProfileTab::Default.name(), "default");
        assert_eq!(ProfileTab::Posts.name(), "posts");
        assert_eq!(ProfileTab::Gifts.name(), "gifts");
        assert_eq!(ProfileTab::Media.name(), "media");
        assert_eq!(ProfileTab::Files.name(), "files");
        assert_eq!(ProfileTab::Music.name(), "music");
        assert_eq!(ProfileTab::Voice.name(), "voice");
        assert_eq!(ProfileTab::Links.name(), "links");
        assert_eq!(ProfileTab::Gifs.name(), "gifs");
        assert_eq!(ProfileTab::Unknown.name(), "unknown");
    }

    #[test]
    fn test_is_default() {
        assert!(ProfileTab::Default.is_default());
        assert!(!ProfileTab::Posts.is_default());
        assert!(!ProfileTab::Media.is_default());
    }

    #[test]
    fn test_is_posts() {
        assert!(!ProfileTab::Default.is_posts());
        assert!(ProfileTab::Posts.is_posts());
        assert!(!ProfileTab::Media.is_posts());
    }

    #[test]
    fn test_is_gifts() {
        assert!(!ProfileTab::Default.is_gifts());
        assert!(ProfileTab::Gifts.is_gifts());
        assert!(!ProfileTab::Media.is_gifts());
    }

    #[test]
    fn test_is_media() {
        assert!(!ProfileTab::Default.is_media());
        assert!(!ProfileTab::Posts.is_media());
        assert!(ProfileTab::Media.is_media());
    }

    #[test]
    fn test_is_files() {
        assert!(!ProfileTab::Default.is_files());
        assert!(ProfileTab::Files.is_files());
        assert!(!ProfileTab::Media.is_files());
    }

    #[test]
    fn test_is_music() {
        assert!(!ProfileTab::Default.is_music());
        assert!(ProfileTab::Music.is_music());
        assert!(!ProfileTab::Media.is_music());
    }

    #[test]
    fn test_is_voice() {
        assert!(!ProfileTab::Default.is_voice());
        assert!(ProfileTab::Voice.is_voice());
        assert!(!ProfileTab::Media.is_voice());
    }

    #[test]
    fn test_is_links() {
        assert!(!ProfileTab::Default.is_links());
        assert!(ProfileTab::Links.is_links());
        assert!(!ProfileTab::Media.is_links());
    }

    #[test]
    fn test_is_gifs() {
        assert!(!ProfileTab::Default.is_gifs());
        assert!(ProfileTab::Gifs.is_gifs());
        assert!(!ProfileTab::Media.is_gifs());
    }

    #[test]
    fn test_is_unknown() {
        assert!(!ProfileTab::Default.is_unknown());
        assert!(!ProfileTab::Media.is_unknown());
        assert!(ProfileTab::Unknown.is_unknown());
    }

    #[test]
    fn test_is_supported() {
        assert!(ProfileTab::Default.is_supported());
        assert!(ProfileTab::Posts.is_supported());
        assert!(ProfileTab::Media.is_supported());
        assert!(!ProfileTab::Unknown.is_supported());
    }

    #[test]
    fn test_is_media_related() {
        assert!(ProfileTab::Media.is_media_related());
        assert!(ProfileTab::Voice.is_media_related());
        assert!(ProfileTab::Gifs.is_media_related());
        assert!(!ProfileTab::Links.is_media_related());
        assert!(!ProfileTab::Files.is_media_related());
    }

    #[test]
    fn test_is_downloadable() {
        assert!(ProfileTab::Files.is_downloadable());
        assert!(ProfileTab::Music.is_downloadable());
        assert!(ProfileTab::Voice.is_downloadable());
        assert!(!ProfileTab::Media.is_downloadable());
        assert!(!ProfileTab::Links.is_downloadable());
    }

    #[test]
    fn test_tabs_for_channel_type_broadcast() {
        let tabs = ProfileTab::tabs_for_channel_type(ChannelType::Broadcast);
        assert!(tabs.contains(&ProfileTab::Default));
        assert!(tabs.contains(&ProfileTab::Posts));
        assert!(tabs.contains(&ProfileTab::Media));
        assert!(tabs.contains(&ProfileTab::Files));
    }

    #[test]
    fn test_tabs_for_channel_type_megagroup() {
        let tabs = ProfileTab::tabs_for_channel_type(ChannelType::Megagroup);
        assert!(tabs.contains(&ProfileTab::Default));
        assert!(tabs.contains(&ProfileTab::Media));
        assert!(tabs.contains(&ProfileTab::Files));
        assert!(tabs.contains(&ProfileTab::Music));
        assert!(tabs.contains(&ProfileTab::Voice));
        assert!(tabs.contains(&ProfileTab::Links));
    }

    #[test]
    fn test_tabs_for_channel_type_unknown() {
        let tabs = ProfileTab::tabs_for_channel_type(ChannelType::Unknown);
        assert_eq!(tabs, vec![ProfileTab::Default]);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ProfileTab::Default), "default");
        assert_eq!(format!("{}", ProfileTab::Media), "media");
        assert_eq!(format!("{}", ProfileTab::Music), "music");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", ProfileTab::Default), "Default");
        assert_eq!(format!("{:?}", ProfileTab::Media), "Media");
        assert_eq!(format!("{:?}", ProfileTab::Unknown), "Unknown");
    }

    #[test]
    fn test_default() {
        assert_eq!(ProfileTab::default(), ProfileTab::Unknown);
    }

    #[test]
    fn test_equality() {
        assert_eq!(ProfileTab::Media, ProfileTab::Media);
        assert_eq!(ProfileTab::Files, ProfileTab::Files);
        assert_ne!(ProfileTab::Media, ProfileTab::Files);
    }

    #[test]
    fn test_copy() {
        let a = ProfileTab::Media;
        let b = a;
        assert_eq!(a, ProfileTab::Media);
        assert_eq!(b, ProfileTab::Media);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ProfileTab::Default);
        set.insert(ProfileTab::Posts);
        set.insert(ProfileTab::Media);
        set.insert(ProfileTab::Files);
        assert!(set.len() >= 4);
    }

    #[test]
    fn test_all_tabs_distinct() {
        let tabs = [
            ProfileTab::Default,
            ProfileTab::Posts,
            ProfileTab::Gifts,
            ProfileTab::Media,
            ProfileTab::Files,
            ProfileTab::Music,
            ProfileTab::Voice,
            ProfileTab::Links,
            ProfileTab::Gifs,
            ProfileTab::Unknown,
        ];

        for i in 0..tabs.len() {
            for j in (i + 1)..tabs.len() {
                assert_ne!(tabs[i], tabs[j]);
            }
        }
    }

    #[test]
    fn test_media_related_tabs() {
        let media_related = [ProfileTab::Media, ProfileTab::Voice, ProfileTab::Gifs];

        for tab in media_related {
            assert!(tab.is_media_related());
        }
    }

    #[test]
    fn test_downloadable_tabs() {
        let downloadable = [ProfileTab::Files, ProfileTab::Music, ProfileTab::Voice];

        for tab in downloadable {
            assert!(tab.is_downloadable());
        }
    }

    #[test]
    fn test_all_names_unique() {
        let names = [
            ProfileTab::Default.name(),
            ProfileTab::Posts.name(),
            ProfileTab::Gifts.name(),
            ProfileTab::Media.name(),
            ProfileTab::Files.name(),
            ProfileTab::Music.name(),
            ProfileTab::Voice.name(),
            ProfileTab::Links.name(),
            ProfileTab::Gifs.name(),
            ProfileTab::Unknown.name(),
        ];

        let mut unique_names = std::collections::HashSet::new();
        for name in names {
            unique_names.insert(name);
        }

        assert_eq!(unique_names.len(), 10);
    }

    #[test]
    fn test_type_count() {
        let tabs = [
            ProfileTab::Default,
            ProfileTab::Posts,
            ProfileTab::Gifts,
            ProfileTab::Media,
            ProfileTab::Files,
            ProfileTab::Music,
            ProfileTab::Voice,
            ProfileTab::Links,
            ProfileTab::Gifs,
            ProfileTab::Unknown,
        ];
        assert_eq!(tabs.len(), 10);
    }
}
