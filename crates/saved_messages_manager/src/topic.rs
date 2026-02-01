//! Saved messages topic implementation.

use crate::error::Result;
use crate::topic_id::SavedMessagesTopicId;
use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Simplified draft message stub.
///
/// This is a minimal implementation that will be expanded when the full DraftMessage type is available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftMessage {
    /// The draft message date.
    pub date: i32,
}

impl DraftMessage {
    /// Creates a new draft message.
    #[inline]
    pub const fn new(date: i32) -> Self {
        Self { date }
    }

    /// Returns the draft message date.
    #[inline]
    pub const fn date(&self) -> i32 {
        self.date
    }
}

/// Ordered messages container (simplified BTreeMap-based implementation).
///
/// This is a temporary replacement for the full OrderedMessages type.
/// It will be replaced when the full type is available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderedMessages {
    /// Messages ordered by message ID.
    messages: BTreeMap<MessageId, OrderedMessage>,
}

/// A single ordered message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderedMessage {
    /// Message ID.
    pub message_id: MessageId,
    /// Message date.
    pub date: i32,
    /// Message content type (placeholder).
    #[serde(skip)]
    pub content_type: String,
}

impl OrderedMessages {
    /// Creates a new empty ordered messages container.
    #[inline]
    pub const fn new() -> Self {
        Self {
            messages: BTreeMap::new(),
        }
    }

    /// Adds a message to the container.
    #[inline]
    pub fn add_message(&mut self, message_id: MessageId, date: i32) {
        let message = OrderedMessage {
            message_id,
            date,
            content_type: String::new(),
        };
        self.messages.insert(message_id, message);
    }

    /// Removes a message from the container.
    #[inline]
    pub fn remove_message(&mut self, message_id: MessageId) -> Option<OrderedMessage> {
        self.messages.remove(&message_id)
    }

    /// Gets the last message ID.
    #[inline]
    pub fn last_message_id(&self) -> Option<MessageId> {
        self.messages.keys().last().copied()
    }

    /// Gets the last message date.
    #[inline]
    pub fn last_message_date(&self) -> Option<i32> {
        self.messages.values().last().map(|m| m.date)
    }

    /// Returns the number of messages.
    #[inline]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Checks if the container is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Checks if a message exists.
    #[inline]
    pub fn contains(&self, message_id: MessageId) -> bool {
        self.messages.contains_key(&message_id)
    }
}

impl Default for OrderedMessages {
    fn default() -> Self {
        Self::new()
    }
}

/// Lightweight info for monoforum topics.
///
/// This structure holds the essential information about a topic without
/// the full message state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedMessagesTopicInfo {
    /// The peer dialog ID.
    pub peer_dialog_id: rustgram_types::DialogId,
    /// The last topic message ID.
    pub last_topic_message_id: MessageId,
    /// The draft message, if any.
    pub draft_message: Option<DraftMessage>,
    /// The read inbox max message ID.
    pub read_inbox_max_message_id: MessageId,
    /// The read outbox max message ID.
    pub read_outbox_max_message_id: MessageId,
    /// The unread message count.
    pub unread_count: i32,
    /// The unread reaction count.
    pub unread_reaction_count: i32,
    /// Whether the topic is marked as unread.
    pub is_marked_as_unread: bool,
    /// Whether nopaid messages are excepted.
    pub nopaid_messages_exception: bool,
    /// Whether the topic is pinned.
    pub is_pinned: bool,
}

impl SavedMessagesTopicInfo {
    /// Creates a new topic info.
    #[inline]
    pub fn new(peer_dialog_id: rustgram_types::DialogId) -> Self {
        Self {
            peer_dialog_id,
            last_topic_message_id: MessageId::default(),
            draft_message: None,
            read_inbox_max_message_id: MessageId::default(),
            read_outbox_max_message_id: MessageId::default(),
            unread_count: 0,
            unread_reaction_count: 0,
            is_marked_as_unread: false,
            nopaid_messages_exception: false,
            is_pinned: false,
        }
    }

    /// Checks if this topic has any unread messages.
    #[inline]
    pub fn has_unread(&self) -> bool {
        self.unread_count > 0 || self.is_marked_as_unread
    }

    /// Checks if this topic has any unread reactions.
    #[inline]
    pub fn has_unread_reactions(&self) -> bool {
        self.unread_reaction_count > 0
    }
}

impl Default for SavedMessagesTopicInfo {
    fn default() -> Self {
        Self::new(rustgram_types::DialogId::default())
    }
}

/// Main topic struct with message state.
///
/// This structure holds the complete state of a saved messages topic,
/// including all messages and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedMessagesTopic {
    /// The dialog ID.
    pub dialog_id: rustgram_types::DialogId,
    /// The saved messages topic ID.
    pub topic_id: SavedMessagesTopicId,
    /// Ordered messages.
    ordered_messages: OrderedMessages,
    /// Last message ID.
    last_message_id: MessageId,
    /// Read inbox max message ID.
    read_inbox_max_message_id: MessageId,
    /// Read outbox max message ID.
    read_outbox_max_message_id: MessageId,
    /// Draft message.
    draft_message: Option<DraftMessage>,
    /// Local message count.
    local_message_count: i32,
    /// Server message count.
    server_message_count: i32,
    /// Sent message count.
    sent_message_count: i32,
    /// Unread message count.
    unread_count: i32,
    /// Unread reaction count.
    unread_reaction_count: i32,
    /// Last message date.
    last_message_date: i32,
    /// Draft message date.
    draft_message_date: i32,
    /// Pinned order.
    pinned_order: i64,
    /// Private order.
    private_order: i64,
    /// Whether server message count is initialized.
    is_server_message_count_inited: bool,
    /// Whether marked as unread.
    is_marked_as_unread: bool,
    /// Whether nopaid messages are excepted.
    nopaid_messages_exception: bool,
    /// Whether received from server.
    is_received_from_server: bool,
    /// Whether unread count needs repair.
    need_repair_unread_count: bool,
    /// Whether the topic has changed.
    is_changed: bool,
}

impl SavedMessagesTopic {
    /// Creates a new saved messages topic.
    #[inline]
    pub fn new(dialog_id: rustgram_types::DialogId, topic_id: SavedMessagesTopicId) -> Self {
        Self {
            dialog_id,
            topic_id,
            ordered_messages: OrderedMessages::new(),
            last_message_id: MessageId::default(),
            read_inbox_max_message_id: MessageId::default(),
            read_outbox_max_message_id: MessageId::default(),
            draft_message: None,
            local_message_count: 0,
            server_message_count: 0,
            sent_message_count: -1,
            unread_count: 0,
            unread_reaction_count: 0,
            last_message_date: 0,
            draft_message_date: 0,
            pinned_order: 0,
            private_order: 0,
            is_server_message_count_inited: false,
            is_marked_as_unread: false,
            nopaid_messages_exception: false,
            is_received_from_server: false,
            need_repair_unread_count: false,
            is_changed: false,
        }
    }

    /// Returns the last message ID.
    #[inline]
    pub const fn last_message_id(&self) -> MessageId {
        self.last_message_id
    }

    /// Returns the read inbox max message ID.
    #[inline]
    pub const fn read_inbox_max_message_id(&self) -> MessageId {
        self.read_inbox_max_message_id
    }

    /// Returns the read outbox max message ID.
    #[inline]
    pub const fn read_outbox_max_message_id(&self) -> MessageId {
        self.read_outbox_max_message_id
    }

    /// Returns the unread count.
    #[inline]
    pub const fn unread_count(&self) -> i32 {
        self.unread_count
    }

    /// Returns the unread reaction count.
    #[inline]
    pub const fn unread_reaction_count(&self) -> i32 {
        self.unread_reaction_count
    }

    /// Returns the last message date.
    #[inline]
    pub const fn last_message_date(&self) -> i32 {
        self.last_message_date
    }

    /// Returns the pinned order.
    #[inline]
    pub const fn pinned_order(&self) -> i64 {
        self.pinned_order
    }

    /// Checks if the topic is pinned.
    #[inline]
    pub const fn is_pinned(&self) -> bool {
        self.pinned_order > 0
    }

    /// Checks if the topic is marked as unread.
    #[inline]
    pub const fn is_marked_as_unread(&self) -> bool {
        self.is_marked_as_unread
    }

    /// Checks if nopaid messages are excepted.
    #[inline]
    pub const fn nopaid_messages_exception(&self) -> bool {
        self.nopaid_messages_exception
    }

    /// Checks if the topic has any unread messages.
    #[inline]
    pub fn has_unread(&self) -> bool {
        self.unread_count > 0 || self.is_marked_as_unread
    }

    /// Checks if the topic has any unread reactions.
    #[inline]
    pub fn has_unread_reactions(&self) -> bool {
        self.unread_reaction_count > 0
    }

    /// Returns the total message count.
    #[inline]
    pub fn total_message_count(&self) -> i32 {
        self.local_message_count + self.server_message_count
    }

    /// Adds a message to the topic.
    #[inline]
    pub fn add_message(&mut self, message_id: MessageId, date: i32) -> Result<()> {
        if message_id.is_server() {
            self.server_message_count += 1;
        } else {
            self.local_message_count += 1;
        }

        self.ordered_messages.add_message(message_id, date);
        self.last_message_id = message_id;
        self.last_message_date = date;
        self.is_changed = true;
        Ok(())
    }

    /// Removes a message from the topic.
    #[inline]
    pub fn remove_message(&mut self, message_id: MessageId) -> Result<()> {
        if message_id.is_server() && self.server_message_count > 0 {
            self.server_message_count -= 1;
        } else if self.local_message_count > 0 {
            self.local_message_count -= 1;
        }

        self.ordered_messages.remove_message(message_id);
        self.last_message_id = self.ordered_messages.last_message_id().unwrap_or_default();
        self.last_message_date = self.ordered_messages.last_message_date().unwrap_or(0);
        self.is_changed = true;
        Ok(())
    }

    /// Sets the last message ID.
    #[inline]
    pub fn set_last_message_id(&mut self, message_id: MessageId, date: i32) {
        self.last_message_id = message_id;
        self.last_message_date = date;
        self.is_changed = true;
    }

    /// Sets the read inbox max message ID.
    #[inline]
    pub fn set_read_inbox_max_message_id(&mut self, message_id: MessageId) {
        self.read_inbox_max_message_id = message_id;
        self.is_changed = true;
    }

    /// Sets the read outbox max message ID.
    #[inline]
    pub fn set_read_outbox_max_message_id(&mut self, message_id: MessageId) {
        self.read_outbox_max_message_id = message_id;
        self.is_changed = true;
    }

    /// Sets the unread count.
    #[inline]
    pub fn set_unread_count(&mut self, count: i32) {
        self.unread_count = count;
        self.is_changed = true;
    }

    /// Sets the unread reaction count.
    #[inline]
    pub fn set_unread_reaction_count(&mut self, count: i32) {
        self.unread_reaction_count = count;
        self.is_changed = true;
    }

    /// Sets the pinned order.
    #[inline]
    pub fn set_pinned_order(&mut self, order: i64) {
        self.pinned_order = order;
        self.is_changed = true;
    }

    /// Sets the private order.
    #[inline]
    pub fn set_private_order(&mut self, order: i64) {
        self.private_order = order;
        self.is_changed = true;
    }

    /// Sets whether marked as unread.
    #[inline]
    pub fn set_marked_as_unread(&mut self, marked: bool) {
        self.is_marked_as_unread = marked;
        self.is_changed = true;
    }

    /// Sets whether nopaid messages are excepted.
    #[inline]
    pub fn set_nopaid_messages_exception(&mut self, exception: bool) {
        self.nopaid_messages_exception = exception;
        self.is_changed = true;
    }

    /// Sets the draft message.
    #[inline]
    pub fn set_draft_message(&mut self, draft: Option<DraftMessage>) {
        self.draft_message_date = draft.as_ref().map(|d| d.date).unwrap_or(0);
        self.draft_message = draft;
        self.is_changed = true;
    }

    /// Returns the topic info.
    #[inline]
    pub fn to_info(&self) -> SavedMessagesTopicInfo {
        SavedMessagesTopicInfo {
            peer_dialog_id: self.dialog_id,
            last_topic_message_id: self.last_message_id,
            draft_message: self.draft_message.clone(),
            read_inbox_max_message_id: self.read_inbox_max_message_id,
            read_outbox_max_message_id: self.read_outbox_max_message_id,
            unread_count: self.unread_count,
            unread_reaction_count: self.unread_reaction_count,
            is_marked_as_unread: self.is_marked_as_unread,
            nopaid_messages_exception: self.nopaid_messages_exception,
            is_pinned: self.is_pinned(),
        }
    }

    /// Resets the changed flag.
    #[inline]
    pub fn reset_changed(&mut self) {
        self.is_changed = false;
    }

    /// Checks if the topic has changed.
    #[inline]
    pub const fn is_changed(&self) -> bool {
        self.is_changed
    }

    /// Checks if this is the last message in the topic.
    #[inline]
    pub fn is_last_message(&self, message_id: MessageId) -> bool {
        self.last_message_id == message_id
    }
}

impl PartialEq for SavedMessagesTopic {
    fn eq(&self, other: &Self) -> bool {
        self.topic_id == other.topic_id
            && self.dialog_id == other.dialog_id
            && self.last_message_id == other.last_message_id
    }
}

impl Eq for SavedMessagesTopic {}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[test]
    fn test_draft_message() {
        let draft = DraftMessage::new(1234567890);
        assert_eq!(draft.date(), 1234567890);
    }

    #[test]
    fn test_ordered_messages() {
        let mut messages = OrderedMessages::new();
        assert!(messages.is_empty());

        let msg1 = MessageId::from_server_id(100);
        let msg2 = MessageId::from_server_id(200);

        messages.add_message(msg1, 100);
        messages.add_message(msg2, 200);

        assert_eq!(messages.len(), 2);
        assert_eq!(messages.last_message_id(), Some(msg2));
        assert_eq!(messages.last_message_date(), Some(200));
        assert!(messages.contains(msg1));
        assert!(messages.contains(msg2));

        messages.remove_message(msg1);
        assert_eq!(messages.len(), 1);
        assert!(!messages.contains(msg1));
        assert!(messages.contains(msg2));
    }

    #[test]
    fn test_topic_info() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let mut info = SavedMessagesTopicInfo::new(dialog_id);

        assert!(!info.has_unread());
        assert!(!info.has_unread_reactions());
        assert_eq!(info.unread_count, 0);

        info.unread_count = 5;
        assert!(info.has_unread());
    }

    #[test]
    fn test_topic_creation() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let topic = SavedMessagesTopic::new(dialog_id, topic_id);

        assert!(!topic.is_pinned());
        assert!(!topic.is_marked_as_unread());
        assert!(!topic.nopaid_messages_exception());
        assert_eq!(topic.total_message_count(), 0);
        assert!(!topic.is_changed());
    }

    #[test]
    fn test_topic_add_message() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut topic = SavedMessagesTopic::new(dialog_id, topic_id);

        let msg1 = MessageId::from_server_id(100);
        topic.add_message(msg1, 100).unwrap();

        assert_eq!(topic.server_message_count, 1);
        assert_eq!(topic.total_message_count(), 1);
        assert_eq!(topic.last_message_id(), msg1);
        assert!(topic.is_changed());
    }

    #[test]
    fn test_topic_remove_message() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut topic = SavedMessagesTopic::new(dialog_id, topic_id);

        let msg1 = MessageId::from_server_id(100);
        topic.add_message(msg1, 100).unwrap();
        topic.remove_message(msg1).unwrap();

        assert_eq!(topic.server_message_count, 0);
        assert_eq!(topic.total_message_count(), 0);
        assert!(topic.is_changed());
    }

    #[test]
    fn test_topic_unread() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut topic = SavedMessagesTopic::new(dialog_id, topic_id);

        assert!(!topic.has_unread());

        topic.set_unread_count(5);
        assert!(topic.has_unread());
        assert_eq!(topic.unread_count(), 5);
    }

    #[test]
    fn test_topic_pinned() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut topic = SavedMessagesTopic::new(dialog_id, topic_id);

        assert!(!topic.is_pinned());

        topic.set_pinned_order(1234567890);
        assert!(topic.is_pinned());
        assert_eq!(topic.pinned_order(), 1234567890);
    }

    #[test]
    fn test_topic_to_info() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut topic = SavedMessagesTopic::new(dialog_id, topic_id);

        topic.set_unread_count(5);
        topic.set_marked_as_unread(true);

        let info = topic.to_info();
        assert_eq!(info.peer_dialog_id, dialog_id);
        assert_eq!(info.unread_count, 5);
        assert!(info.is_marked_as_unread);
    }

    #[test]
    fn test_topic_is_last_message() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut topic = SavedMessagesTopic::new(dialog_id, topic_id);

        let msg1 = MessageId::from_server_id(100);
        let msg2 = MessageId::from_server_id(200);

        topic.add_message(msg1, 100).unwrap();
        assert!(topic.is_last_message(msg1));
        assert!(!topic.is_last_message(msg2));

        topic.add_message(msg2, 200).unwrap();
        assert!(!topic.is_last_message(msg1));
        assert!(topic.is_last_message(msg2));
    }
}
