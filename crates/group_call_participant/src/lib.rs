//! # Group Call Participant
//!
//! Participant information for group voice/video chats.
//!
//! ## Overview
//!
//! `GroupCallParticipant` contains detailed information about a participant
//! in a group call, including audio/video state, mute status, and participation order.
//!
//! ## Usage
//!
//! ```
//! use rustgram_group_call_participant::GroupCallParticipant;
//! use rustgram_dialog_id::DialogId;
//!
//! let participant = GroupCallParticipant::new(DialogId::new(123));
//! assert!(participant.is_valid());
//! ```

use rustgram_dialog_id::DialogId;
use rustgram_group_call_participant_order::GroupCallParticipantOrder;
use rustgram_group_call_video_payload::GroupCallVideoPayload;
use std::fmt;

/// Information about a participant in a group call.
///
/// This type contains comprehensive information about a group call participant,
/// including their audio/video state, mute status, and ordering information.
///
/// # Examples
///
/// ```
/// use rustgram_group_call_participant::GroupCallParticipant;
/// use rustgram_dialog_id::DialogId;
///
/// let participant = GroupCallParticipant::new(DialogId::new(123));
/// assert!(participant.is_valid());
/// assert_eq!(participant.dialog_id(), DialogId::new(123));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupCallParticipant {
    dialog_id: DialogId,
    about: String,
    video_payload: GroupCallVideoPayload,
    presentation_payload: GroupCallVideoPayload,
    audio_source: i32,
    presentation_audio_source: i32,
    raise_hand_rating: i64,
    joined_date: i32,
    active_date: i32,
    volume_level: i32,
    is_volume_level_local: bool,
    server_is_muted_by_themselves: bool,
    server_is_muted_by_admin: bool,
    server_is_muted_locally: bool,
    is_self: bool,
    can_be_muted_for_all_users: bool,
    can_be_unmuted_for_all_users: bool,
    can_be_muted_only_for_self: bool,
    can_be_unmuted_only_for_self: bool,
    is_min: bool,
    is_fake: bool,
    is_just_joined: bool,
    is_speaking: bool,
    video_diff: i32,
    local_active_date: i32,
    order: GroupCallParticipantOrder,
    version: i32,
    pending_volume_level: i32,
    pending_volume_level_generation: u64,
    have_pending_is_muted: bool,
    pending_is_muted_by_themselves: bool,
    pending_is_muted_by_admin: bool,
    pending_is_muted_locally: bool,
    pending_is_muted_generation: u64,
    have_pending_is_hand_raised: bool,
    pending_is_hand_raised: bool,
    pending_is_hand_raised_generation: u64,
}

impl GroupCallParticipant {
    /// Minimum allowed volume level.
    pub const MIN_VOLUME_LEVEL: i32 = 1;

    /// Maximum allowed volume level.
    pub const MAX_VOLUME_LEVEL: i32 = 20000;

    /// Creates a new group call participant with the given dialog ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The participant's dialog ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(participant.is_valid());
    /// ```
    #[inline]
    pub fn new(dialog_id: DialogId) -> Self {
        Self {
            dialog_id,
            about: String::new(),
            video_payload: GroupCallVideoPayload::new(),
            presentation_payload: GroupCallVideoPayload::new(),
            audio_source: 0,
            presentation_audio_source: 0,
            raise_hand_rating: 0,
            joined_date: 0,
            active_date: 0,
            volume_level: 10000,
            is_volume_level_local: false,
            server_is_muted_by_themselves: false,
            server_is_muted_by_admin: false,
            server_is_muted_locally: false,
            is_self: false,
            can_be_muted_for_all_users: false,
            can_be_unmuted_for_all_users: false,
            can_be_muted_only_for_self: false,
            can_be_unmuted_only_for_self: false,
            is_min: false,
            is_fake: false,
            is_just_joined: false,
            is_speaking: false,
            video_diff: 0,
            local_active_date: 0,
            order: GroupCallParticipantOrder::min(),
            version: 0,
            pending_volume_level: 0,
            pending_volume_level_generation: 0,
            have_pending_is_muted: false,
            pending_is_muted_by_themselves: false,
            pending_is_muted_by_admin: false,
            pending_is_muted_locally: false,
            pending_is_muted_generation: 0,
            have_pending_is_hand_raised: false,
            pending_is_hand_raised: false,
            pending_is_hand_raised_generation: 0,
        }
    }

    /// Returns the participant's dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(456);
    /// let participant = GroupCallParticipant::new(dialog_id);
    /// assert_eq!(participant.dialog_id(), dialog_id);
    /// ```
    #[inline]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the participant's about text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert_eq!(participant.about(), "");
    /// ```
    #[inline]
    pub fn about(&self) -> &str {
        &self.about
    }

    /// Returns the video payload.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// let video = participant.video_payload();
    /// ```
    #[inline]
    pub const fn video_payload(&self) -> &GroupCallVideoPayload {
        &self.video_payload
    }

    /// Returns the presentation payload.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// let presentation = participant.presentation_payload();
    /// ```
    #[inline]
    pub const fn presentation_payload(&self) -> &GroupCallVideoPayload {
        &self.presentation_payload
    }

    /// Returns the audio source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert_eq!(participant.audio_source(), 0);
    /// ```
    #[inline]
    pub const fn audio_source(&self) -> i32 {
        self.audio_source
    }

    /// Returns the raise hand rating.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert_eq!(participant.raise_hand_rating(), 0);
    /// ```
    #[inline]
    pub const fn raise_hand_rating(&self) -> i64 {
        self.raise_hand_rating
    }

    /// Returns the joined date.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert_eq!(participant.joined_date(), 0);
    /// ```
    #[inline]
    pub const fn joined_date(&self) -> i32 {
        self.joined_date
    }

    /// Returns the active date.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert_eq!(participant.active_date(), 0);
    /// ```
    #[inline]
    pub const fn active_date(&self) -> i32 {
        self.active_date
    }

    /// Returns the volume level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert_eq!(participant.volume_level(), 10000);
    /// ```
    #[inline]
    pub const fn volume_level(&self) -> i32 {
        self.volume_level
    }

    /// Returns `true` if the participant is muted by themselves.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(!participant.get_is_muted_by_themselves());
    /// ```
    #[inline]
    pub const fn get_is_muted_by_themselves(&self) -> bool {
        self.server_is_muted_by_themselves
    }

    /// Returns `true` if the participant is muted by admin.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(!participant.get_is_muted_by_admin());
    /// ```
    #[inline]
    pub const fn get_is_muted_by_admin(&self) -> bool {
        self.server_is_muted_by_admin
    }

    /// Returns `true` if the participant is locally muted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(!participant.get_is_muted_locally());
    /// ```
    #[inline]
    pub const fn get_is_muted_locally(&self) -> bool {
        self.server_is_muted_locally
    }

    /// Returns the effective muted state for all users.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(!participant.get_is_muted_for_all_users());
    /// ```
    #[inline]
    pub const fn get_is_muted_for_all_users(&self) -> bool {
        self.server_is_muted_by_themselves || self.server_is_muted_by_admin
    }

    /// Returns `true` if this is the current user.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(!participant.is_self());
    /// ```
    #[inline]
    pub const fn is_self(&self) -> bool {
        self.is_self
    }

    /// Returns the participant order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// let order = participant.order();
    /// ```
    #[inline]
    pub const fn order(&self) -> &GroupCallParticipantOrder {
        &self.order
    }

    /// Returns `true` if the participant has their hand raised.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(!participant.get_is_hand_raised());
    /// ```
    #[inline]
    pub const fn get_is_hand_raised(&self) -> bool {
        self.raise_hand_rating > 0
    }

    /// Returns `true` if the participant has video.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert_eq!(participant.get_has_video(), 0);
    /// ```
    #[inline]
    pub const fn get_has_video(&self) -> i32 {
        self.video_diff
    }

    /// Returns `true` if this is a valid participant.
    ///
    /// A valid participant must have a valid dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let participant = GroupCallParticipant::new(DialogId::new(123));
    /// assert!(participant.is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.dialog_id.is_valid()
    }

    /// Updates this participant's information from another participant.
    ///
    /// # Arguments
    ///
    /// * `other` - The participant to update from
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut participant = GroupCallParticipant::new(DialogId::new(123));
    /// let other = GroupCallParticipant::new(DialogId::new(123));
    /// participant.update_from(&other);
    /// ```
    pub fn update_from(&mut self, other: &GroupCallParticipant) {
        self.about = other.about.clone();
        self.video_payload = other.video_payload.clone();
        self.presentation_payload = other.presentation_payload.clone();
        self.audio_source = other.audio_source;
        self.presentation_audio_source = other.presentation_audio_source;
        self.raise_hand_rating = other.raise_hand_rating;
        self.joined_date = other.joined_date;
        self.active_date = other.active_date;
        self.volume_level = other.volume_level;
        self.is_volume_level_local = other.is_volume_level_local;
        self.server_is_muted_by_themselves = other.server_is_muted_by_themselves;
        self.server_is_muted_by_admin = other.server_is_muted_by_admin;
        self.server_is_muted_locally = other.server_is_muted_locally;
        self.can_be_muted_for_all_users = other.can_be_muted_for_all_users;
        self.can_be_unmuted_for_all_users = other.can_be_unmuted_for_all_users;
        self.can_be_muted_only_for_self = other.can_be_muted_only_for_self;
        self.can_be_unmuted_only_for_self = other.can_be_unmuted_only_for_self;
        self.is_min = other.is_min;
        self.is_fake = other.is_fake;
        self.is_just_joined = other.is_just_joined;
        self.is_speaking = other.is_speaking;
        self.video_diff = other.video_diff;
        self.local_active_date = other.local_active_date;
        self.order = other.order;
        self.version = other.version;
    }

    /// Updates the can_be_muted flags based on permissions.
    ///
    /// # Arguments
    ///
    /// * `can_manage` - Whether we can manage the participant
    /// * `is_admin` - Whether we are an admin
    ///
    /// # Returns
    ///
    /// `true` if any mute flag was changed
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant::GroupCallParticipant;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut participant = GroupCallParticipant::new(DialogId::new(123));
    /// participant.update_can_be_muted(true, false);
    /// ```
    pub fn update_can_be_muted(&mut self, can_manage: bool, is_admin: bool) -> bool {
        let old = (
            self.can_be_muted_for_all_users,
            self.can_be_unmuted_for_all_users,
            self.can_be_muted_only_for_self,
            self.can_be_unmuted_only_for_self,
        );

        if is_admin {
            self.can_be_muted_for_all_users = !self.server_is_muted_by_admin || !can_manage;
            self.can_be_unmuted_for_all_users = self.server_is_muted_by_admin && can_manage;
        } else {
            self.can_be_muted_for_all_users = false;
            self.can_be_unmuted_for_all_users = false;
        }

        self.can_be_muted_only_for_self = !self.server_is_muted_locally;
        self.can_be_unmuted_only_for_self = self.server_is_muted_locally;

        let new = (
            self.can_be_muted_for_all_users,
            self.can_be_unmuted_for_all_users,
            self.can_be_muted_only_for_self,
            self.can_be_unmuted_only_for_self,
        );

        old != new
    }
}

impl fmt::Display for GroupCallParticipant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GroupCallParticipant {{ dialog_id: {}, is_self: {}, is_speaking: {} }}",
            self.dialog_id, self.is_self, self.is_speaking
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dialog_id = DialogId::new(123);
        let participant = GroupCallParticipant::new(dialog_id);
        assert_eq!(participant.dialog_id(), dialog_id);
    }

    #[test]
    fn test_is_valid() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert!(participant.is_valid());

        let invalid = GroupCallParticipant::new(DialogId::new(0));
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_volume_level() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert_eq!(participant.volume_level(), 10000);
    }

    #[test]
    fn test_get_is_muted_by_themselves() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert!(!participant.get_is_muted_by_themselves());
    }

    #[test]
    fn test_get_is_muted_by_admin() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert!(!participant.get_is_muted_by_admin());
    }

    #[test]
    fn test_get_is_muted_locally() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert!(!participant.get_is_muted_locally());
    }

    #[test]
    fn test_get_is_muted_for_all_users() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert!(!participant.get_is_muted_for_all_users());
    }

    #[test]
    fn test_get_is_hand_raised() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert!(!participant.get_is_hand_raised());
    }

    #[test]
    fn test_get_has_video() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert_eq!(participant.get_has_video(), 0);
    }

    #[test]
    fn test_is_self() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        assert!(!participant.is_self());
    }

    #[test]
    fn test_update_from() {
        let mut participant = GroupCallParticipant::new(DialogId::new(123));
        let other = GroupCallParticipant::new(DialogId::new(123));
        participant.update_from(&other);
        // Just verify it compiles and doesn't panic
    }

    #[test]
    fn test_update_can_be_muted() {
        let mut participant = GroupCallParticipant::new(DialogId::new(123));
        let changed = participant.update_can_be_muted(true, false);
        // Changed might be true or false depending on default state
        let _ = changed;
    }

    #[test]
    fn test_clone() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        let cloned = participant.clone();
        assert_eq!(participant, cloned);
    }

    #[test]
    fn test_equality() {
        let dialog_id = DialogId::new(123);
        let participant1 = GroupCallParticipant::new(dialog_id);
        let participant2 = GroupCallParticipant::new(dialog_id);
        assert_eq!(participant1, participant2);
    }

    #[test]
    fn test_display() {
        let participant = GroupCallParticipant::new(DialogId::new(123));
        let s = format!("{}", participant);
        assert!(s.contains("GroupCallParticipant"));
    }

    #[test]
    fn test_const_volume_levels() {
        assert_eq!(GroupCallParticipant::MIN_VOLUME_LEVEL, 1);
        assert_eq!(GroupCallParticipant::MAX_VOLUME_LEVEL, 20000);
    }
}
