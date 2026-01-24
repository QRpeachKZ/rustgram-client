// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0

//! Dialog message storage.

use rustgram_message_types::Message;
use rustgram_types::{DialogId, MessageId};
use std::collections::HashMap;

/// Storage for messages across all dialogs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogMessageState {
    messages: HashMap<DialogId, HashMap<MessageId, Message>>,
}

impl DialogMessageState {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub async fn add_message(
        &mut self,
        dialog_id: DialogId,
        message: Message,
    ) -> crate::Result<()> {
        let storage = self.messages.entry(dialog_id).or_insert_with(HashMap::new);
        storage.insert(message.id(), message);
        Ok(())
    }

    pub async fn get_message(&self, dialog_id: DialogId, message_id: MessageId) -> Option<Message> {
        self.messages.get(&dialog_id)?.get(&message_id).cloned()
    }

    pub async fn remove_message(
        &mut self,
        dialog_id: DialogId,
        message_id: MessageId,
    ) -> crate::Result<()> {
        if let Some(storage) = self.messages.get_mut(&dialog_id) {
            storage.remove(&message_id);
            Ok(())
        } else {
            Err(crate::Error::DialogNotFound)
        }
    }

    pub async fn clear_dialog(&mut self, dialog_id: DialogId) -> crate::Result<()> {
        if let Some(storage) = self.messages.get_mut(&dialog_id) {
            storage.clear();
            Ok(())
        } else {
            Err(crate::Error::DialogNotFound)
        }
    }

    pub async fn get_history(
        &self,
        dialog_id: DialogId,
        _from_message_id: MessageId,
        _offset: i32,
        _limit: i32,
    ) -> crate::Result<Vec<Message>> {
        if let Some(storage) = self.messages.get(&dialog_id) {
            let mut messages: Vec<_> = storage.values().cloned().collect();
            messages.sort_by_key(|m| std::cmp::Reverse(m.id()));
            Ok(messages)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_message_count(&self, dialog_id: DialogId) -> usize {
        self.messages.get(&dialog_id).map(|s| s.len()).unwrap_or(0)
    }
}

impl Default for DialogMessageState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_message_content_type::MessageContentType;
    use rustgram_types::UserId;

    fn test_msg(id: i32) -> Message {
        let uid = UserId::new(123).unwrap();
        let did = DialogId::from_user(uid);
        Message::new(
            MessageId::from_server_id(id),
            did,
            did,
            1234567890,
            MessageContentType::Text,
        )
    }

    #[tokio::test]
    async fn test_add_get() {
        let mut storage = DialogMessageState::new();
        let uid = UserId::new(123).unwrap();
        let did = DialogId::from_user(uid);
        let msg = test_msg(1);

        storage.add_message(did, msg.clone()).await.unwrap();
        let retrieved = storage.get_message(did, msg.id()).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), msg.id());
    }

    #[tokio::test]
    async fn test_remove() {
        let mut storage = DialogMessageState::new();
        let uid = UserId::new(123).unwrap();
        let did = DialogId::from_user(uid);
        let msg = test_msg(1);

        storage.add_message(did, msg.clone()).await.unwrap();
        storage.remove_message(did, msg.id()).await.unwrap();
        assert_eq!(storage.get_message_count(did).await, 0);
    }
}
