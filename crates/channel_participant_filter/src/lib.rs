// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Channel Participant Filter
//!
//! Channel participant filter for Telegram.
//!
//! ## Overview
//!
//! Filters for selecting different types of channel participants.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Channel participant filter type.
///
/// Used to filter participants when querying channel members.
/// Based on TDLib's `ChannelParticipantFilter` class.
///
/// # Example
///
/// ```rust
/// use rustgram_channel_participant_filter::ChannelParticipantFilter;
///
/// // Get recent members
/// let recent = ChannelParticipantFilter::Recent;
/// assert!(recent.is_recent());
///
/// // Get administrators
/// let admins = ChannelParticipantFilter::Administrators;
/// assert!(admins.is_administrators());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelParticipantFilter {
    /// Recent participants.
    Recent,
    /// Contacts.
    Contacts(String),
    /// Administrators.
    Administrators,
    /// Search with query.
    Search(String),
    /// Mention (participants that can be mentioned).
    Mention,
    /// Restricted users.
    Restricted(String),
    /// Banned users.
    Banned(String),
    /// Bots.
    Bots,
}

impl Default for ChannelParticipantFilter {
    fn default() -> Self {
        Self::Recent
    }
}

impl ChannelParticipantFilter {
    /// Creates a recent filter.
    #[must_use]
    pub const fn recent() -> Self {
        Self::Recent
    }

    /// Creates a contacts filter.
    #[must_use]
    pub fn contacts(query: String) -> Self {
        Self::Contacts(query)
    }

    /// Creates an administrators filter.
    #[must_use]
    pub const fn administrators() -> Self {
        Self::Administrators
    }

    /// Creates a search filter.
    #[must_use]
    pub fn search(query: String) -> Self {
        Self::Search(query)
    }

    /// Creates a mention filter.
    #[must_use]
    pub const fn mention() -> Self {
        Self::Mention
    }

    /// Creates a restricted filter.
    #[must_use]
    pub fn restricted(query: String) -> Self {
        Self::Restricted(query)
    }

    /// Creates a banned filter.
    #[must_use]
    pub fn banned(query: String) -> Self {
        Self::Banned(query)
    }

    /// Creates a bots filter.
    #[must_use]
    pub const fn bots() -> Self {
        Self::Bots
    }

    /// Checks if this is a recent filter.
    #[must_use]
    pub const fn is_recent(&self) -> bool {
        matches!(self, Self::Recent)
    }

    /// Checks if this is a contacts filter.
    #[must_use]
    pub const fn is_contacts(&self) -> bool {
        matches!(self, Self::Contacts(_))
    }

    /// Checks if this is an administrators filter.
    #[must_use]
    pub const fn is_administrators(&self) -> bool {
        matches!(self, Self::Administrators)
    }

    /// Checks if this is a search filter.
    #[must_use]
    pub const fn is_search(&self) -> bool {
        matches!(self, Self::Search(_))
    }

    /// Checks if this is a mention filter.
    #[must_use]
    pub const fn is_mention(&self) -> bool {
        matches!(self, Self::Mention)
    }

    /// Checks if this is a restricted filter.
    #[must_use]
    pub const fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted(_))
    }

    /// Checks if this is a banned filter.
    #[must_use]
    pub const fn is_banned(&self) -> bool {
        matches!(self, Self::Banned(_))
    }

    /// Checks if this is a bots filter.
    #[must_use]
    pub const fn is_bots(&self) -> bool {
        matches!(self, Self::Bots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let filter = ChannelParticipantFilter::default();
        assert!(filter.is_recent());
    }

    #[test]
    fn test_recent() {
        let filter = ChannelParticipantFilter::recent();
        assert!(filter.is_recent());
        assert!(!filter.is_administrators());
    }

    #[test]
    fn test_administrators() {
        let filter = ChannelParticipantFilter::administrators();
        assert!(filter.is_administrators());
    }

    #[test]
    fn test_bots() {
        let filter = ChannelParticipantFilter::bots();
        assert!(filter.is_bots());
    }

    #[test]
    fn test_contacts() {
        let filter = ChannelParticipantFilter::contacts("query".to_string());
        assert!(filter.is_contacts());
    }

    #[test]
    fn test_search() {
        let filter = ChannelParticipantFilter::search("test".to_string());
        assert!(filter.is_search());
    }

    #[test]
    fn test_restricted() {
        let filter = ChannelParticipantFilter::restricted("q".to_string());
        assert!(filter.is_restricted());
    }

    #[test]
    fn test_banned() {
        let filter = ChannelParticipantFilter::banned("q".to_string());
        assert!(filter.is_banned());
    }

    #[test]
    fn test_mention() {
        let filter = ChannelParticipantFilter::mention();
        assert!(filter.is_mention());
    }

    #[test]
    fn test_equality() {
        let f1 = ChannelParticipantFilter::Recent;
        let f2 = ChannelParticipantFilter::Recent;
        let f3 = ChannelParticipantFilter::Administrators;
        assert_eq!(f1, f2);
        assert_ne!(f1, f3);
    }
}
