// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! State tracking structures for MessageQueryManager.

use rustgram_business_connection_id::BusinessConnectionId;
use rustgram_file_upload_id::FileUploadId;
use rustgram_message_extended_media::Photo;
use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// State of a message cover being uploaded.
///
/// Tracks the upload progress of a message cover photo.
///
/// # Example
///
/// ```rust
/// use rustgram_business_connection_id::BusinessConnectionId;
/// use rustgram_message_query_manager::state::BeingUploadedCover;
/// use rustgram_message_extended_media::Photo;
/// use rustgram_types::{ChatId, DialogId};
///
/// let conn_id = BusinessConnectionId::default();
/// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
/// let photo = Photo::new();
/// let cover = BeingUploadedCover::new(conn_id, dialog_id, photo);
/// assert!(!cover.is_complete());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingUploadedCover {
    /// Business connection ID for business chats.
    business_connection_id: BusinessConnectionId,
    /// Dialog where the cover is being uploaded.
    dialog_id: DialogId,
    /// Photo being uploaded.
    photo: Photo,
    /// File upload ID for tracking.
    file_upload_id: Option<FileUploadId>,
    /// Whether the upload is complete.
    is_complete: bool,
}

impl BeingUploadedCover {
    /// Creates a new cover upload state.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Business connection ID
    /// * `dialog_id` - Dialog where the cover is being uploaded
    /// * `photo` - Photo being uploaded
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::state::BeingUploadedCover;
    /// use rustgram_business_connection_id::BusinessConnectionId;
    /// use rustgram_message_extended_media::Photo;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let conn_id = BusinessConnectionId::default();
    /// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
    /// let photo = Photo::new();
    /// let cover = BeingUploadedCover::new(conn_id, dialog_id, photo);
    /// ```
    pub fn new(
        business_connection_id: BusinessConnectionId,
        dialog_id: DialogId,
        photo: Photo,
    ) -> Self {
        Self {
            business_connection_id,
            dialog_id,
            photo,
            file_upload_id: None,
            is_complete: false,
        }
    }

    /// Returns the business connection ID.
    #[must_use]
    pub const fn business_connection_id(&self) -> &BusinessConnectionId {
        &self.business_connection_id
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the photo being uploaded.
    #[must_use]
    pub const fn photo(&self) -> &Photo {
        &self.photo
    }

    /// Returns the file upload ID if set.
    #[must_use]
    pub const fn file_upload_id(&self) -> Option<&FileUploadId> {
        self.file_upload_id.as_ref()
    }

    /// Checks if the upload is complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Sets the file upload ID.
    pub fn with_file_upload_id(mut self, file_upload_id: FileUploadId) -> Self {
        self.file_upload_id = Some(file_upload_id);
        self
    }

    /// Marks the upload as complete.
    pub fn mark_complete(mut self) -> Self {
        self.is_complete = true;
        self
    }
}

impl fmt::Display for BeingUploadedCover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BeingUploadedCover(dialog={}, complete={})",
            self.dialog_id, self.is_complete
        )
    }
}

/// Message reload operation types.
///
/// Tracks which type of extended data is being reloaded for messages.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::state::ReloadType;
///
/// let reload = ReloadType::ExtendedMedia;
/// assert!(matches!(reload, ReloadType::ExtendedMedia));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReloadType {
    /// Extended media is being reloaded.
    ExtendedMedia,
    /// Fact checks are being reloaded.
    FactChecks,
    /// View counts are being reloaded.
    Views,
    /// Reactions are being reloaded.
    Reactions,
}

impl fmt::Display for ReloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtendedMedia => write!(f, "extended media"),
            Self::FactChecks => write!(f, "fact checks"),
            Self::Views => write!(f, "views"),
            Self::Reactions => write!(f, "reactions"),
        }
    }
}

/// Message reload state tracking.
///
/// Tracks messages that are currently being reloaded for various types of data.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::state::{MessageReloadState, ReloadType};
/// use rustgram_types::{ChatId, DialogId, MessageId};
///
/// let mut state = MessageReloadState::new();
/// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
/// let message_id = MessageId::from_server_id(456);
///
/// state.add_reloading_message(dialog_id, message_id, ReloadType::ExtendedMedia);
/// assert!(state.is_reloading(dialog_id, message_id, ReloadType::ExtendedMedia));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessageReloadState {
    /// Messages being reloaded with their reload type.
    reloading_messages: Vec<(DialogId, MessageId, ReloadType)>,
}

impl MessageReloadState {
    /// Creates a new empty reload state.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::state::MessageReloadState;
    ///
    /// let state = MessageReloadState::new();
    /// assert!(state.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            reloading_messages: Vec::new(),
        }
    }

    /// Adds a message to the reloading list.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog containing the message
    /// * `message_id` - Message being reloaded
    /// * `reload_type` - Type of reload operation
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::state::{MessageReloadState, ReloadType};
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// let mut state = MessageReloadState::new();
    /// state.add_reloading_message(DialogId::from_chat(ChatId::new(123).unwrap()), MessageId::from_server_id(456), ReloadType::Views);
    /// ```
    pub fn add_reloading_message(
        &mut self,
        dialog_id: DialogId,
        message_id: MessageId,
        reload_type: ReloadType,
    ) {
        self.reloading_messages
            .push((dialog_id, message_id, reload_type));
    }

    /// Removes a message from the reloading list.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog containing the message
    /// * `message_id` - Message to remove
    /// * `reload_type` - Type of reload operation
    pub fn remove_reloading_message(
        &mut self,
        dialog_id: DialogId,
        message_id: MessageId,
        reload_type: ReloadType,
    ) {
        self.reloading_messages
            .retain(|(d, m, t)| *d != dialog_id || *m != message_id || *t != reload_type);
    }

    /// Checks if a message is being reloaded.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog containing the message
    /// * `message_id` - Message to check
    /// * `reload_type` - Type of reload operation
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::state::{MessageReloadState, ReloadType};
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// let mut state = MessageReloadState::new();
    /// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
    /// let message_id = MessageId::from_server_id(456);
    ///
    /// assert!(!state.is_reloading(dialog_id, message_id, ReloadType::ExtendedMedia));
    ///
    /// state.add_reloading_message(dialog_id, message_id, ReloadType::ExtendedMedia);
    /// assert!(state.is_reloading(dialog_id, message_id, ReloadType::ExtendedMedia));
    /// ```
    #[must_use]
    pub fn is_reloading(
        &self,
        dialog_id: DialogId,
        message_id: MessageId,
        reload_type: ReloadType,
    ) -> bool {
        self.reloading_messages
            .iter()
            .any(|(d, m, t)| *d == dialog_id && *m == message_id && *t == reload_type)
    }

    /// Returns the number of messages being reloaded.
    #[must_use]
    pub fn len(&self) -> usize {
        self.reloading_messages.len()
    }

    /// Checks if no messages are being reloaded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.reloading_messages.is_empty()
    }

    /// Clears all reloading messages.
    pub fn clear(&mut self) {
        self.reloading_messages.clear();
    }

    /// Returns an iterator over reloading messages.
    pub fn iter(&self) -> impl Iterator<Item = &(DialogId, MessageId, ReloadType)> {
        self.reloading_messages.iter()
    }
}

impl fmt::Display for MessageReloadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MessageReloadState(reloading={})",
            self.reloading_messages.len()
        )
    }
}

/// Reaction reload state per dialog.
///
/// Tracks reactions being reloaded for a specific dialog.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::state::ReactionsToReload;
/// use rustgram_types::MessageId;
///
/// let mut reactions = ReactionsToReload::new();
/// reactions.add_message(MessageId::from_server_id(123));
/// assert!(reactions.has_pending());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReactionsToReload {
    /// Message IDs that need reaction reload.
    message_ids: Vec<MessageId>,
    /// Whether a request has been sent.
    request_sent: bool,
}

impl ReactionsToReload {
    /// Creates a new empty reactions reload state.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::state::ReactionsToReload;
    ///
    /// let reactions = ReactionsToReload::new();
    /// assert!(!reactions.has_pending());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            message_ids: Vec::new(),
            request_sent: false,
        }
    }

    /// Adds a message ID to the reload list.
    ///
    /// # Arguments
    ///
    /// * `message_id` - Message ID to add
    pub fn add_message(&mut self, message_id: MessageId) {
        if !self.message_ids.contains(&message_id) {
            self.message_ids.push(message_id);
        }
    }

    /// Removes a message ID from the reload list.
    ///
    /// # Arguments
    ///
    /// * `message_id` - Message ID to remove
    pub fn remove_message(&mut self, message_id: MessageId) {
        self.message_ids.retain(|&id| id != message_id);
    }

    /// Returns the message IDs being reloaded.
    #[must_use]
    pub fn message_ids(&self) -> &[MessageId] {
        &self.message_ids
    }

    /// Checks if a request has been sent.
    #[must_use]
    pub const fn request_sent(&self) -> bool {
        self.request_sent
    }

    /// Sets whether a request has been sent.
    pub fn set_request_sent(mut self, sent: bool) -> Self {
        self.request_sent = sent;
        self
    }

    /// Checks if there are pending reactions to reload.
    #[must_use]
    pub fn has_pending(&self) -> bool {
        !self.message_ids.is_empty()
    }

    /// Returns the number of pending message IDs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.message_ids.len()
    }

    /// Checks if there are no pending reactions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.message_ids.is_empty()
    }

    /// Clears all pending message IDs.
    pub fn clear(&mut self) {
        self.message_ids.clear();
        self.request_sent = false;
    }
}

impl fmt::Display for ReactionsToReload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ReactionsToReload(pending={}, request_sent={})",
            self.message_ids.len(),
            self.request_sent
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_business_connection_id::BusinessConnectionId;
    use rustgram_types::ChatId;

    // BeingUploadedCover tests
    #[test]
    fn test_being_uploaded_cover_new() {
        let conn_id = BusinessConnectionId::default();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let photo = Photo::new();
        let cover = BeingUploadedCover::new(conn_id, dialog_id, photo);

        assert!(!cover.is_complete());
        assert_eq!(
            cover.dialog_id(),
            DialogId::from_chat(ChatId::new(123).unwrap())
        );
        assert!(cover.file_upload_id().is_none());
    }

    #[test]
    fn test_being_uploaded_cover_with_file_upload_id() {
        use rustgram_file_id::FileId;

        let conn_id = BusinessConnectionId::default();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let photo = Photo::new();
        let file_id = FileId::new(42, 0);
        let upload_id = rustgram_file_upload_id::FileUploadId::new(file_id, 1);

        let cover =
            BeingUploadedCover::new(conn_id, dialog_id, photo).with_file_upload_id(upload_id);

        assert!(cover.file_upload_id().is_some());
        assert_eq!(cover.file_upload_id().unwrap().internal_upload_id(), 1);
    }

    #[test]
    fn test_being_uploaded_cover_mark_complete() {
        let conn_id = BusinessConnectionId::default();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let photo = Photo::new();
        let cover = BeingUploadedCover::new(conn_id, dialog_id, photo).mark_complete();

        assert!(cover.is_complete());
    }

    #[test]
    fn test_being_uploaded_cover_display() {
        let conn_id = BusinessConnectionId::default();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let photo = Photo::new();
        let cover = BeingUploadedCover::new(conn_id, dialog_id, photo);

        let display = format!("{cover}");
        // DialogId Display format just shows the number, so the output should be:
        // "BeingUploadedCover(dialog=123, complete=false)"
        assert!(display.contains("123"));
        assert!(display.contains("complete=false"));
    }

    #[test]
    fn test_being_uploaded_cover_clone() {
        let conn_id = BusinessConnectionId::default();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let photo = Photo::new();
        let cover1 = BeingUploadedCover::new(conn_id, dialog_id, photo);
        let cover2 = cover1.clone();

        assert_eq!(cover1, cover2);
    }

    // ReloadType tests
    #[test]
    fn test_reload_type_extended_media() {
        let reload = ReloadType::ExtendedMedia;
        assert_eq!(format!("{reload}"), "extended media");
    }

    #[test]
    fn test_reload_type_fact_checks() {
        let reload = ReloadType::FactChecks;
        assert_eq!(format!("{reload}"), "fact checks");
    }

    #[test]
    fn test_reload_type_views() {
        let reload = ReloadType::Views;
        assert_eq!(format!("{reload}"), "views");
    }

    #[test]
    fn test_reload_type_reactions() {
        let reload = ReloadType::Reactions;
        assert_eq!(format!("{reload}"), "reactions");
    }

    // MessageReloadState tests
    #[test]
    fn test_message_reload_state_new() {
        let state = MessageReloadState::new();
        assert!(state.is_empty());
        assert_eq!(state.len(), 0);
    }

    #[test]
    fn test_message_reload_state_default() {
        let state = MessageReloadState::default();
        assert!(state.is_empty());
    }

    #[test]
    fn test_message_reload_state_add() {
        let mut state = MessageReloadState::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let message_id = MessageId::from_server_id(456);

        state.add_reloading_message(dialog_id, message_id, ReloadType::ExtendedMedia);
        assert_eq!(state.len(), 1);
        assert!(state.is_reloading(dialog_id, message_id, ReloadType::ExtendedMedia));
    }

    #[test]
    fn test_message_reload_state_remove() {
        let mut state = MessageReloadState::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let message_id = MessageId::from_server_id(456);

        state.add_reloading_message(dialog_id, message_id, ReloadType::Views);
        state.remove_reloading_message(dialog_id, message_id, ReloadType::Views);

        assert!(!state.is_reloading(dialog_id, message_id, ReloadType::Views));
        assert!(state.is_empty());
    }

    #[test]
    fn test_message_reload_state_is_reloading_different_type() {
        let mut state = MessageReloadState::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let message_id = MessageId::from_server_id(456);

        state.add_reloading_message(dialog_id, message_id, ReloadType::ExtendedMedia);

        assert!(!state.is_reloading(dialog_id, message_id, ReloadType::Views));
        assert!(state.is_reloading(dialog_id, message_id, ReloadType::ExtendedMedia));
    }

    #[test]
    fn test_message_reload_state_clear() {
        let mut state = MessageReloadState::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());

        state.add_reloading_message(dialog_id, MessageId::from_server_id(1), ReloadType::Views);
        state.add_reloading_message(
            dialog_id,
            MessageId::from_server_id(2),
            ReloadType::Reactions,
        );

        assert_eq!(state.len(), 2);
        state.clear();
        assert!(state.is_empty());
    }

    #[test]
    fn test_message_reload_state_iter() {
        let mut state = MessageReloadState::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());

        state.add_reloading_message(dialog_id, MessageId::from_server_id(1), ReloadType::Views);
        state.add_reloading_message(
            dialog_id,
            MessageId::from_server_id(2),
            ReloadType::Reactions,
        );

        let count = state.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_message_reload_state_display() {
        let mut state = MessageReloadState::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());

        state.add_reloading_message(dialog_id, MessageId::from_server_id(1), ReloadType::Views);

        let display = format!("{state}");
        assert!(display.contains("reloading=1"));
    }

    #[test]
    fn test_message_reload_state_clone() {
        let mut state = MessageReloadState::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());

        state.add_reloading_message(dialog_id, MessageId::from_server_id(1), ReloadType::Views);

        let state2 = state.clone();
        assert_eq!(state, state2);
    }

    // ReactionsToReload tests
    #[test]
    fn test_reactions_to_reload_new() {
        let reactions = ReactionsToReload::new();
        assert!(reactions.is_empty());
        assert!(!reactions.has_pending());
        assert!(!reactions.request_sent());
    }

    #[test]
    fn test_reactions_to_reload_default() {
        let reactions = ReactionsToReload::default();
        assert!(reactions.is_empty());
    }

    #[test]
    fn test_reactions_to_reload_add_message() {
        let mut reactions = ReactionsToReload::new();
        reactions.add_message(MessageId::from_server_id(123));

        assert!(reactions.has_pending());
        assert_eq!(reactions.len(), 1);
    }

    #[test]
    fn test_reactions_to_reload_add_duplicate() {
        let mut reactions = ReactionsToReload::new();
        let msg_id = MessageId::from_server_id(123);

        reactions.add_message(msg_id);
        reactions.add_message(msg_id);

        assert_eq!(reactions.len(), 1);
    }

    #[test]
    fn test_reactions_to_reload_remove_message() {
        let mut reactions = ReactionsToReload::new();
        let msg_id = MessageId::from_server_id(123);

        reactions.add_message(msg_id);
        reactions.remove_message(msg_id);

        assert!(!reactions.has_pending());
        assert!(reactions.is_empty());
    }

    #[test]
    fn test_reactions_to_reload_message_ids() {
        let mut reactions = ReactionsToReload::new();

        reactions.add_message(MessageId::from_server_id(123));
        reactions.add_message(MessageId::from_server_id(456));

        let ids = reactions.message_ids();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_reactions_to_reload_set_request_sent() {
        let reactions = ReactionsToReload::new().set_request_sent(true);

        assert!(reactions.request_sent());
    }

    #[test]
    fn test_reactions_to_reload_clear() {
        let mut reactions = ReactionsToReload::new();

        reactions.add_message(MessageId::from_server_id(123));
        reactions.add_message(MessageId::from_server_id(456));

        assert_eq!(reactions.len(), 2);
        reactions.clear();

        assert!(reactions.is_empty());
        assert!(!reactions.request_sent());
    }

    #[test]
    fn test_reactions_to_reload_display() {
        let mut reactions = ReactionsToReload::new();
        reactions.add_message(MessageId::from_server_id(123));

        let display = format!("{reactions}");
        assert!(display.contains("pending=1"));
    }

    #[test]
    fn test_reactions_to_reload_clone() {
        let mut reactions = ReactionsToReload::new();
        reactions.add_message(MessageId::from_server_id(123));

        let reactions2 = reactions.clone();
        assert_eq!(reactions, reactions2);
    }

    #[test]
    fn test_reactions_to_reload_equality() {
        let msg_id = MessageId::from_server_id(123);

        let mut reactions1 = ReactionsToReload::new();
        reactions1.add_message(msg_id);

        let mut reactions2 = ReactionsToReload::new();
        reactions2.add_message(msg_id);

        assert_eq!(reactions1, reactions2);
    }

    #[test]
    fn test_reactions_to_reload_inequality() {
        let mut reactions1 = ReactionsToReload::new();
        reactions1.add_message(MessageId::from_server_id(123));

        let mut reactions2 = ReactionsToReload::new();
        reactions2.add_message(MessageId::from_server_id(456));

        assert_ne!(reactions1, reactions2);
    }
}
