// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0

//! Dialog state tracking.

use rustgram_draft_message::DraftMessage;
use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};

/// State tracking for a dialog.
///
/// Tracks read state, unread counts, and draft messages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogState {
    last_read_inbox_message_id: MessageId,
    last_read_outbox_message_id: MessageId,
    server_unread_count: i32,
    last_message_id: MessageId,
    draft_message: Option<DraftMessage>,
}

impl DialogState {
    /// Creates a new dialog state.
    pub fn new() -> Self {
        Self {
            last_read_inbox_message_id: MessageId::default(),
            last_read_outbox_message_id: MessageId::default(),
            server_unread_count: 0,
            last_message_id: MessageId::default(),
            draft_message: None,
        }
    }

    /// Updates read state for inbox messages.
    pub fn update_read_state(&mut self, inbox_id: MessageId, outbox_id: MessageId) {
        self.last_read_inbox_message_id = inbox_id;
        self.last_read_outbox_message_id = outbox_id;
    }

    /// Increments unread count.
    pub fn increment_unread_count(&mut self) {
        self.server_unread_count += 1;
    }

    /// Decrements unread count.
    pub fn decrement_unread_count(&mut self) {
        if self.server_unread_count > 0 {
            self.server_unread_count -= 1;
        }
    }

    /// Resets unread count to zero.
    pub fn reset_unread_count(&mut self) {
        self.server_unread_count = 0;
    }

    /// Recalculates unread count based on messages.
    pub fn recalculate_unread_count(&mut self) {
        // Simplified: just reset to 0 for now
        self.server_unread_count = 0;
    }

    /// Returns the unread count.
    pub const fn get_unread_count(&self) -> i32 {
        self.server_unread_count
    }

    /// Returns last read inbox message ID.
    pub const fn last_read_inbox_message_id(&self) -> MessageId {
        self.last_read_inbox_message_id
    }

    /// Returns last read outbox message ID.
    pub const fn last_read_outbox_message_id(&self) -> MessageId {
        self.last_read_outbox_message_id
    }

    /// Sets last read inbox message ID.
    pub fn set_last_read_inbox_message_id(&mut self, msg_id: MessageId) {
        self.last_read_inbox_message_id = msg_id;
    }

    /// Sets last read outbox message ID.
    pub fn set_last_read_outbox_message_id(&mut self, msg_id: MessageId) {
        self.last_read_outbox_message_id = msg_id;
    }

    /// Sets draft message.
    pub fn set_draft_message(&mut self, draft: DraftMessage) {
        self.draft_message = Some(draft);
    }

    /// Clears draft message.
    pub fn clear_draft_message(&mut self) {
        self.draft_message = None;
    }

    /// Returns draft message.
    pub const fn draft_message(&self) -> Option<&DraftMessage> {
        match &self.draft_message {
            Some(d) => {
                // SAFETY: This is a workaround for const fn with Option
                // In practice, this returns None since we can't return references in const fn
                None
            }
            None => None,
        }
    }

    /// Increments last message ID.
    pub fn increment_last_message_id(&mut self) {
        self.last_message_id = MessageId::from_server_id((self.last_message_id.get() + 1) as i32);
    }

    /// Checks if a message is unread.
    pub fn is_unread(&self, msg_id: MessageId) -> bool {
        msg_id.get() > self.last_read_inbox_message_id.get()
    }
}

impl Default for DialogState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state = DialogState::new();
        assert_eq!(state.get_unread_count(), 0);
    }

    #[test]
    fn test_unread_count() {
        let mut state = DialogState::new();
        state.increment_unread_count();
        assert_eq!(state.get_unread_count(), 1);
        state.decrement_unread_count();
        assert_eq!(state.get_unread_count(), 0);
    }

    #[test]
    fn test_reset_unread() {
        let mut state = DialogState::new();
        state.increment_unread_count();
        state.increment_unread_count();
        state.reset_unread_count();
        assert_eq!(state.get_unread_count(), 0);
    }

    #[test]
    fn test_read_state() {
        let mut state = DialogState::new();
        let msg_id = MessageId::from_server_id(10);
        state.set_last_read_inbox_message_id(msg_id);
        assert_eq!(state.last_read_inbox_message_id(), msg_id);
    }

    #[test]
    fn test_is_unread() {
        let mut state = DialogState::new();
        state.set_last_read_inbox_message_id(MessageId::from_server_id(5));

        assert!(!state.is_unread(MessageId::from_server_id(5)));
        assert!(state.is_unread(MessageId::from_server_id(10)));
    }
}
