//! # Rustgram GroupCallVideoPayload
//!
//! Group call video payload handling for Telegram MTProto client.
//!
//! This crate provides types for representing video payload information in
//! Telegram group calls (voice chats/video chats). It handles video source
//! groups, endpoints, and pause states for WebRTC-based calls.
//!
//! ## Overview
//!
//! - [`GroupCallVideoSourceGroup`] - Represents a group of video sources with semantics
//! - [`GroupCallVideoPayload`] - Complete video payload with source groups and endpoint
//!
//! ## Video Source Groups
//!
//! Video sources are organized into groups with specific semantics:
//!
//! - **Semantics**: Describes how video sources should be displayed (e.g., "simulcast")
//! - **Source IDs**: List of video source identifiers in the group
//!
//! ## Examples
//!
//! Basic video payload:
//!
//! ```
//! use rustgram_group_call_video_payload::{GroupCallVideoPayload, GroupCallVideoSourceGroup};
//!
//! let payload = GroupCallVideoPayload::new();
//! assert!(payload.is_empty());
//!
//! let source_group = GroupCallVideoSourceGroup::with_data("simulcast".to_string(), vec![1, 2, 3]);
//! let payload_with_sources = GroupCallVideoPayload::with_data(
//!     vec![source_group],
//!     "endpoint".to_string(),
//!     false,
//! );
//! assert!(!payload_with_sources.is_empty());
//! ```
//!
//! Creating a paused video payload:
//!
//! ```
//! use rustgram_group_call_video_payload::GroupCallVideoPayload;
//!
//! let paused = GroupCallVideoPayload::with_data(
//!     vec![],
//!     "wss://endpoint.example.com".to_string(),
//!     true,
//! );
//! assert!(paused.is_paused());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Video source group for group calls.
///
/// Represents a group of video sources with specific semantics that define
/// how the sources should be handled and displayed.
///
/// # Examples
///
/// ```
/// use rustgram_group_call_video_payload::GroupCallVideoSourceGroup;
///
/// let group = GroupCallVideoSourceGroup::with_data("simulcast".to_string(), vec![1, 2, 3]);
/// assert_eq!(group.semantics(), "simulcast");
/// assert_eq!(group.source_ids(), &[1, 2, 3]);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GroupCallVideoSourceGroup {
    /// Describes the semantics of this video source group
    semantics: String,
    /// List of video source IDs in this group
    source_ids: Vec<i32>,
}

impl Default for GroupCallVideoSourceGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl GroupCallVideoSourceGroup {
    /// Creates a new empty video source group.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoSourceGroup;
    ///
    /// let group = GroupCallVideoSourceGroup::new();
    /// assert!(group.semantics().is_empty());
    /// assert!(group.source_ids().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            semantics: String::new(),
            source_ids: Vec::new(),
        }
    }

    /// Creates a new video source group with the given data.
    ///
    /// # Arguments
    ///
    /// * `semantics` - The semantics describing this source group
    /// * `source_ids` - List of video source IDs
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoSourceGroup;
    ///
    /// let group = GroupCallVideoSourceGroup::with_data("simulcast".to_string(), vec![1, 2, 3]);
    /// assert_eq!(group.semantics(), "simulcast");
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(semantics: String, source_ids: Vec<i32>) -> Self {
        Self {
            semantics,
            source_ids,
        }
    }

    /// Returns the semantics of this source group.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoSourceGroup;
    ///
    /// let group = GroupCallVideoSourceGroup::with_data("simulcast".to_string(), vec![1]);
    /// assert_eq!(group.semantics(), "simulcast");
    /// ```
    #[inline]
    #[must_use]
    pub fn semantics(&self) -> &str {
        &self.semantics
    }

    /// Returns the source IDs in this group.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoSourceGroup;
    ///
    /// let group = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![1, 2, 3]);
    /// assert_eq!(group.source_ids(), &[1, 2, 3]);
    /// ```
    #[inline]
    #[must_use]
    pub fn source_ids(&self) -> &[i32] {
        &self.source_ids
    }

    /// Checks if this source group is empty.
    ///
    /// # Returns
    ///
    /// `true` if semantics is empty and there are no source IDs
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoSourceGroup;
    ///
    /// assert!(GroupCallVideoSourceGroup::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.semantics.is_empty() && self.source_ids.is_empty()
    }

    /// Returns the number of source IDs in this group.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoSourceGroup;
    ///
    /// let group = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![1, 2, 3]);
    /// assert_eq!(group.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.source_ids.len()
    }
}

/// Group call video payload.
///
/// Contains all video payload information for a group call participant,
/// including source groups, endpoint, and pause state.
///
/// # Examples
///
/// ```
/// use rustgram_group_call_video_payload::GroupCallVideoPayload;
///
/// let payload = GroupCallVideoPayload::new();
/// assert!(payload.is_empty());
/// assert!(!payload.is_paused());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GroupCallVideoPayload {
    /// List of video source groups
    source_groups: Vec<GroupCallVideoSourceGroup>,
    /// WebRTC endpoint for this video stream
    endpoint: String,
    /// Whether the video is paused
    is_paused: bool,
}

impl Default for GroupCallVideoPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl GroupCallVideoPayload {
    /// Creates a new empty video payload.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let payload = GroupCallVideoPayload::new();
    /// assert!(payload.is_empty());
    /// assert_eq!(payload.endpoint(), "");
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            source_groups: Vec::new(),
            endpoint: String::new(),
            is_paused: false,
        }
    }

    /// Creates a new video payload with the given data.
    ///
    /// # Arguments
    ///
    /// * `source_groups` - List of video source groups
    /// * `endpoint` - WebRTC endpoint URL
    /// * `is_paused` - Whether video is paused
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::{GroupCallVideoPayload, GroupCallVideoSourceGroup};
    ///
    /// let groups = vec![
    ///     GroupCallVideoSourceGroup::with_data("simulcast".to_string(), vec![1, 2]),
    /// ];
    /// let payload = GroupCallVideoPayload::with_data(
    ///     groups,
    ///     "wss://endpoint.com".to_string(),
    ///     false,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(
        source_groups: Vec<GroupCallVideoSourceGroup>,
        endpoint: String,
        is_paused: bool,
    ) -> Self {
        Self {
            source_groups,
            endpoint,
            is_paused,
        }
    }

    /// Checks if the video payload is empty.
    ///
    /// # Returns
    ///
    /// `true` if there are no source groups
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// assert!(GroupCallVideoPayload::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.source_groups.is_empty()
    }

    /// Returns the source groups.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let payload = GroupCallVideoPayload::new();
    /// assert!(payload.source_groups().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn source_groups(&self) -> &[GroupCallVideoSourceGroup] {
        &self.source_groups
    }

    /// Returns the endpoint.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let payload = GroupCallVideoPayload::with_data(
    ///     vec![],
    ///     "wss://endpoint.com".to_string(),
    ///     false,
    /// );
    /// assert_eq!(payload.endpoint(), "wss://endpoint.com");
    /// ```
    #[inline]
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Checks if the video is paused.
    ///
    /// # Returns
    ///
    /// `true` if video is paused
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let paused = GroupCallVideoPayload::with_data(vec![], "".to_string(), true);
    /// assert!(paused.is_paused());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Returns the number of source groups.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::{GroupCallVideoPayload, GroupCallVideoSourceGroup};
    ///
    /// let payload = GroupCallVideoPayload::with_data(
    ///     vec![
    ///         GroupCallVideoSourceGroup::new(),
    ///         GroupCallVideoSourceGroup::new(),
    ///     ],
    ///     "".to_string(),
    ///     false,
    /// );
    /// assert_eq!(payload.len(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.source_groups.len()
    }

    /// Adds a source group to the payload.
    ///
    /// # Arguments
    ///
    /// * `group` - The source group to add
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::{GroupCallVideoPayload, GroupCallVideoSourceGroup};
    ///
    /// let mut payload = GroupCallVideoPayload::new();
    /// assert_eq!(payload.len(), 0);
    ///
    /// let group = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![1]);
    /// payload.add_source_group(group);
    /// assert_eq!(payload.len(), 1);
    /// ```
    pub fn add_source_group(&mut self, group: GroupCallVideoSourceGroup) {
        self.source_groups.push(group);
    }

    /// Sets the endpoint for this payload.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The new endpoint URL
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let mut payload = GroupCallVideoPayload::new();
    /// payload.set_endpoint("wss://new.com".to_string());
    /// assert_eq!(payload.endpoint(), "wss://new.com");
    /// ```
    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
    }

    /// Sets the pause state for this payload.
    ///
    /// # Arguments
    ///
    /// * `is_paused` - Whether video should be paused
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let mut payload = GroupCallVideoPayload::new();
    /// payload.set_paused(true);
    /// assert!(payload.is_paused());
    /// ```
    pub fn set_paused(&mut self, is_paused: bool) {
        self.is_paused = is_paused;
    }

    /// Clears all source groups.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::{GroupCallVideoPayload, GroupCallVideoSourceGroup};
    ///
    /// let mut payload = GroupCallVideoPayload::with_data(
    ///     vec![GroupCallVideoSourceGroup::new()],
    ///     "".to_string(),
    ///     false,
    /// );
    /// assert!(!payload.is_empty());
    ///
    /// payload.clear_source_groups();
    /// assert!(payload.is_empty());
    /// ```
    pub fn clear_source_groups(&mut self) {
        self.source_groups.clear();
    }

    /// Converts this payload to a TD API representation.
    ///
    /// This would typically convert to `td_api::groupCallParticipantVideoInfo`.
    ///
    /// # Returns
    ///
    /// A tuple of (endpoint, is_paused, source_groups_data) for TD API
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let payload = GroupCallVideoPayload::with_data(
    ///     vec![],
    ///     "wss://endpoint.com".to_string(),
    ///     true,
    /// );
    /// let (endpoint, paused, _) = payload.to_td_api();
    /// assert_eq!(endpoint, "wss://endpoint.com");
    /// assert!(paused);
    /// ```
    #[must_use]
    pub fn to_td_api(&self) -> (String, bool, Vec<(String, Vec<i32>)>) {
        let source_data: Vec<(String, Vec<i32>)> = self
            .source_groups
            .iter()
            .map(|g| (g.semantics.clone(), g.source_ids.clone()))
            .collect();

        (self.endpoint.clone(), self.is_paused, source_data)
    }

    /// Creates a payload from TD API representation.
    ///
    /// This would typically create from `telegram_api::groupCallParticipantVideo`.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - WebRTC endpoint URL
    /// * `is_paused` - Whether video is paused
    /// * `source_data` - List of (semantics, source_ids) tuples
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_video_payload::GroupCallVideoPayload;
    ///
    /// let payload = GroupCallVideoPayload::from_td_api(
    ///     "wss://endpoint.com".to_string(),
    ///     false,
    ///     vec![("simulcast".to_string(), vec![1, 2, 3])],
    /// );
    /// assert_eq!(payload.endpoint(), "wss://endpoint.com");
    /// assert_eq!(payload.len(), 1);
    /// ```
    #[must_use]
    pub fn from_td_api(
        endpoint: String,
        is_paused: bool,
        source_data: Vec<(String, Vec<i32>)>,
    ) -> Self {
        let source_groups = source_data
            .into_iter()
            .map(|(semantics, source_ids)| {
                GroupCallVideoSourceGroup::with_data(semantics, source_ids)
            })
            .collect();

        Self {
            source_groups,
            endpoint,
            is_paused,
        }
    }
}

impl fmt::Display for GroupCallVideoPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GroupCallVideoPayload {{ endpoint: {}, paused: {}, groups: {} }}",
            self.endpoint,
            self.is_paused,
            self.source_groups.len()
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-group-call-video-payload";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== GroupCallVideoSourceGroup Constructor Tests ==========

    #[test]
    fn test_source_group_new_creates_empty() {
        let group = GroupCallVideoSourceGroup::new();
        assert!(group.semantics().is_empty());
        assert!(group.source_ids().is_empty());
        assert!(group.is_empty());
    }

    #[test]
    fn test_source_group_default_creates_empty() {
        let group = GroupCallVideoSourceGroup::default();
        assert!(group.is_empty());
    }

    #[test]
    fn test_source_group_with_data_sets_values() {
        let group = GroupCallVideoSourceGroup::with_data("simulcast".to_string(), vec![1, 2, 3]);
        assert_eq!(group.semantics(), "simulcast");
        assert_eq!(group.source_ids(), &[1, 2, 3]);
    }

    // ========== GroupCallVideoSourceGroup Accessor Tests ==========

    #[test]
    fn test_source_group_semantics() {
        let group = GroupCallVideoSourceGroup::with_data("test".to_string(), vec![]);
        assert_eq!(group.semantics(), "test");
    }

    #[test]
    fn test_source_group_source_ids() {
        let ids = vec![10, 20, 30];
        let group = GroupCallVideoSourceGroup::with_data("s".to_string(), ids.clone());
        assert_eq!(group.source_ids(), &ids);
    }

    #[test]
    fn test_source_group_len() {
        let group = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![1, 2, 3, 4, 5]);
        assert_eq!(group.len(), 5);
    }

    #[test]
    fn test_source_group_len_zero() {
        let group = GroupCallVideoSourceGroup::new();
        assert_eq!(group.len(), 0);
    }

    // ========== GroupCallVideoSourceGroup is_empty Tests ==========

    #[test]
    fn test_source_group_is_empty_both_empty() {
        let group = GroupCallVideoSourceGroup::new();
        assert!(group.is_empty());
    }

    #[test]
    fn test_source_group_is_empty_with_semantics() {
        let group = GroupCallVideoSourceGroup::with_data("simulcast".to_string(), vec![]);
        assert!(!group.is_empty());
    }

    #[test]
    fn test_source_group_is_empty_with_sources() {
        let group = GroupCallVideoSourceGroup::with_data("".to_string(), vec![1]);
        assert!(!group.is_empty());
    }

    // ========== GroupCallVideoPayload Constructor Tests ==========

    #[test]
    fn test_payload_new_creates_empty() {
        let payload = GroupCallVideoPayload::new();
        assert!(payload.is_empty());
        assert!(!payload.is_paused());
        assert_eq!(payload.endpoint(), "");
    }

    #[test]
    fn test_payload_default_creates_empty() {
        let payload = GroupCallVideoPayload::default();
        assert!(payload.is_empty());
    }

    #[test]
    fn test_payload_with_data_sets_values() {
        let groups = vec![GroupCallVideoSourceGroup::with_data(
            "s".to_string(),
            vec![1],
        )];
        let payload = GroupCallVideoPayload::with_data(groups, "ep".to_string(), true);
        assert!(!payload.is_empty());
        assert!(payload.is_paused());
        assert_eq!(payload.endpoint(), "ep");
    }

    // ========== GroupCallVideoPayload is_empty Tests ==========

    #[test]
    fn test_payload_is_empty_when_no_groups() {
        let payload = GroupCallVideoPayload::new();
        assert!(payload.is_empty());
    }

    #[test]
    fn test_payload_is_empty_with_groups() {
        let groups = vec![GroupCallVideoSourceGroup::new()];
        let payload = GroupCallVideoPayload::with_data(groups, "".to_string(), false);
        assert!(!payload.is_empty());
    }

    // ========== GroupCallVideoPayload Accessor Tests ==========

    #[test]
    fn test_payload_endpoint() {
        let payload =
            GroupCallVideoPayload::with_data(vec![], "wss://example.com".to_string(), false);
        assert_eq!(payload.endpoint(), "wss://example.com");
    }

    #[test]
    fn test_payload_is_paused_true() {
        let payload = GroupCallVideoPayload::with_data(vec![], "".to_string(), true);
        assert!(payload.is_paused());
    }

    #[test]
    fn test_payload_is_paused_false() {
        let payload = GroupCallVideoPayload::new();
        assert!(!payload.is_paused());
    }

    #[test]
    fn test_payload_source_groups() {
        let groups = vec![
            GroupCallVideoSourceGroup::with_data("s1".to_string(), vec![1]),
            GroupCallVideoSourceGroup::with_data("s2".to_string(), vec![2]),
        ];
        let payload = GroupCallVideoPayload::with_data(groups.clone(), "".to_string(), false);
        assert_eq!(payload.source_groups(), &groups);
    }

    #[test]
    fn test_payload_len() {
        let groups = vec![
            GroupCallVideoSourceGroup::new(),
            GroupCallVideoSourceGroup::new(),
            GroupCallVideoSourceGroup::new(),
        ];
        let payload = GroupCallVideoPayload::with_data(groups, "".to_string(), false);
        assert_eq!(payload.len(), 3);
    }

    // ========== GroupCallVideoPayload Mutator Tests ==========

    #[test]
    fn test_payload_add_source_group() {
        let mut payload = GroupCallVideoPayload::new();
        assert_eq!(payload.len(), 0);

        payload.add_source_group(GroupCallVideoSourceGroup::new());
        assert_eq!(payload.len(), 1);

        payload.add_source_group(GroupCallVideoSourceGroup::new());
        assert_eq!(payload.len(), 2);
    }

    #[test]
    fn test_payload_set_endpoint() {
        let mut payload = GroupCallVideoPayload::new();
        payload.set_endpoint("new_endpoint".to_string());
        assert_eq!(payload.endpoint(), "new_endpoint");
    }

    #[test]
    fn test_payload_set_paused() {
        let mut payload = GroupCallVideoPayload::new();
        assert!(!payload.is_paused());

        payload.set_paused(true);
        assert!(payload.is_paused());

        payload.set_paused(false);
        assert!(!payload.is_paused());
    }

    #[test]
    fn test_payload_clear_source_groups() {
        let groups = vec![GroupCallVideoSourceGroup::new()];
        let mut payload = GroupCallVideoPayload::with_data(groups, "".to_string(), false);
        assert!(!payload.is_empty());

        payload.clear_source_groups();
        assert!(payload.is_empty());
    }

    // ========== TD API Conversion Tests ==========

    #[test]
    fn test_payload_to_td_api() {
        let groups = vec![GroupCallVideoSourceGroup::with_data(
            "sim".to_string(),
            vec![1, 2],
        )];
        let payload = GroupCallVideoPayload::with_data(groups, "ep".to_string(), true);

        let (endpoint, paused, source_data) = payload.to_td_api();
        assert_eq!(endpoint, "ep");
        assert!(paused);
        assert_eq!(source_data.len(), 1);
        assert_eq!(source_data[0].0, "sim");
        assert_eq!(source_data[0].1, vec![1, 2]);
    }

    #[test]
    fn test_payload_from_td_api() {
        let source_data = vec![
            ("simulcast".to_string(), vec![1, 2, 3]),
            ("fallback".to_string(), vec![4]),
        ];
        let payload = GroupCallVideoPayload::from_td_api(
            "wss://endpoint.com".to_string(),
            false,
            source_data,
        );

        assert_eq!(payload.endpoint(), "wss://endpoint.com");
        assert!(!payload.is_paused());
        assert_eq!(payload.len(), 2);
        assert_eq!(payload.source_groups()[0].semantics(), "simulcast");
        assert_eq!(payload.source_groups()[1].semantics(), "fallback");
    }

    #[test]
    fn test_payload_td_api_round_trip() {
        let original = GroupCallVideoPayload::with_data(
            vec![GroupCallVideoSourceGroup::with_data(
                "s".to_string(),
                vec![10, 20],
            )],
            "endpoint".to_string(),
            true,
        );

        let (endpoint, paused, source_data) = original.to_td_api();
        let restored = GroupCallVideoPayload::from_td_api(endpoint, paused, source_data);

        assert_eq!(original, restored);
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_source_group_equality() {
        let group1 = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![1, 2]);
        let group2 = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![1, 2]);
        assert_eq!(group1, group2);
    }

    #[test]
    fn test_source_group_inequality() {
        let group1 = GroupCallVideoSourceGroup::with_data("s1".to_string(), vec![1]);
        let group2 = GroupCallVideoSourceGroup::with_data("s2".to_string(), vec![1]);
        assert_ne!(group1, group2);
    }

    #[test]
    fn test_payload_equality() {
        let groups = vec![GroupCallVideoSourceGroup::with_data(
            "s".to_string(),
            vec![1],
        )];
        let payload1 = GroupCallVideoPayload::with_data(groups.clone(), "ep".to_string(), true);
        let payload2 = GroupCallVideoPayload::with_data(groups, "ep".to_string(), true);
        assert_eq!(payload1, payload2);
    }

    #[test]
    fn test_payload_inequality() {
        let payload1 = GroupCallVideoPayload::with_data(vec![], "ep1".to_string(), false);
        let payload2 = GroupCallVideoPayload::with_data(vec![], "ep2".to_string(), false);
        assert_ne!(payload1, payload2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_source_group_clone() {
        let group = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![1, 2, 3]);
        let cloned = group.clone();
        assert_eq!(group, cloned);
    }

    #[test]
    fn test_payload_clone() {
        let groups = vec![GroupCallVideoSourceGroup::with_data(
            "s".to_string(),
            vec![1],
        )];
        let payload = GroupCallVideoPayload::with_data(groups, "ep".to_string(), true);
        let cloned = payload.clone();
        assert_eq!(payload, cloned);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_payload_display_format() {
        let payload = GroupCallVideoPayload::with_data(
            vec![GroupCallVideoSourceGroup::new()],
            "endpoint".to_string(),
            true,
        );
        let display = format!("{}", payload);
        assert!(display.contains("endpoint"));
        assert!(display.contains("true"));
        assert!(display.contains("1"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-group-call-video-payload");
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_source_group_with_empty_sources() {
        let group = GroupCallVideoSourceGroup::with_data("semantics".to_string(), vec![]);
        assert_eq!(group.len(), 0);
        assert!(!group.is_empty()); // Has semantics
    }

    #[test]
    fn test_payload_with_multiple_source_groups() {
        let groups = vec![
            GroupCallVideoSourceGroup::with_data("s1".to_string(), vec![1, 2]),
            GroupCallVideoSourceGroup::with_data("s2".to_string(), vec![3, 4]),
            GroupCallVideoSourceGroup::with_data("s3".to_string(), vec![5, 6]),
        ];
        let payload = GroupCallVideoPayload::with_data(groups, "ep".to_string(), false);
        assert_eq!(payload.len(), 3);
    }

    #[test]
    fn test_payload_empty_source_data_from_td_api() {
        let payload = GroupCallVideoPayload::from_td_api("ep".to_string(), false, vec![]);
        assert!(payload.is_empty());
        assert_eq!(payload.endpoint(), "ep");
    }

    #[test]
    fn test_source_group_with_negative_source_ids() {
        let group = GroupCallVideoSourceGroup::with_data("s".to_string(), vec![-1, -2]);
        assert_eq!(group.source_ids(), &[-1, -2]);
    }

    #[test]
    fn test_payload_set_empty_endpoint() {
        let mut payload = GroupCallVideoPayload::with_data(vec![], "original".to_string(), false);
        payload.set_endpoint(String::new());
        assert_eq!(payload.endpoint(), "");
    }
}
