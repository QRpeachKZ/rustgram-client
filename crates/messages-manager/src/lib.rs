// Copyright 2025 rustgram-client contributors

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_draft_message::DraftMessage;
use rustgram_message_content_type::MessageContentType;
use rustgram_message_types::{Message, MessageSendOptions};
use rustgram_types::{DialogId, MessageId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod dialog_state;
mod storage;

pub use dialog_state::DialogState;
pub use storage::DialogMessageState;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("message not found")]
    MessageNotFound,
    #[error("dialog not found")]
    DialogNotFound,
    #[error("invalid message ID: {0}")]
    InvalidMessageId(String),
    #[error("message is not editable")]
    NotEditable,
    #[error("send failed: {0}")]
    SendFailed(String),
    #[error("edit failed: {0}")]
    EditFailed(String),
    #[error("delete failed: {0}")]
    DeleteFailed(String),
    #[error("validation error: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct MessagesManager {
    storage: Arc<RwLock<DialogMessageState>>,
    dialog_states: Arc<RwLock<HashMap<DialogId, DialogState>>>,
}

impl MessagesManager {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(DialogMessageState::new())),
            dialog_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn send_message(
        &self,
        dialog_id: DialogId,
        sender_id: DialogId,
        content_type: MessageContentType,
        _options: Option<MessageSendOptions>,
    ) -> Result<MessageId> {
        self.ensure_dialog_state(dialog_id).await;
        let message_id = MessageId::from_server_id(current_timestamp());
        let message = Message::new(
            message_id,
            dialog_id,
            sender_id,
            current_timestamp(),
            content_type,
        );
        {
            let mut storage = self.storage.write().await;
            storage.add_message(dialog_id, message).await?;
        }
        Ok(message_id)
    }

    pub async fn get_message(&self, dialog_id: DialogId, message_id: MessageId) -> Result<Message> {
        let storage = self.storage.read().await;
        storage
            .get_message(dialog_id, message_id)
            .await
            .ok_or(Error::MessageNotFound)
    }

    pub async fn delete_messages(
        &self,
        dialog_id: DialogId,
        message_ids: Vec<MessageId>,
        _revoke: bool,
    ) -> Result<()> {
        let mut storage = self.storage.write().await;
        for msg_id in &message_ids {
            storage.remove_message(dialog_id, *msg_id).await?;
        }
        Ok(())
    }

    pub async fn edit_message_text(
        &self,
        dialog_id: DialogId,
        message_id: MessageId,
        _new_text: String,
    ) -> Result<()> {
        let mut storage = self.storage.write().await;
        if let Some(mut msg) = storage.get_message(dialog_id, message_id).await {
            msg.set_edit_date(current_timestamp());
            storage.add_message(dialog_id, msg).await?;
            Ok(())
        } else {
            Err(Error::MessageNotFound)
        }
    }

    pub async fn get_dialog_state(&self, dialog_id: DialogId) -> Option<DialogState> {
        let states = self.dialog_states.read().await;
        states.get(&dialog_id).cloned()
    }

    pub async fn get_message_count(&self, dialog_id: DialogId) -> usize {
        let storage = self.storage.read().await;
        storage.get_message_count(dialog_id).await
    }

    async fn ensure_dialog_state(&self, dialog_id: DialogId) {
        let mut states = self.dialog_states.write().await;
        if !states.contains_key(&dialog_id) {
            states.insert(dialog_id, DialogState::new());
        }
    }
}

impl Default for MessagesManager {
    fn default() -> Self {
        Self::new()
    }
}

fn current_timestamp() -> i32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i32)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[tokio::test]
    async fn test_send_and_get() {
        let manager = MessagesManager::new();
        let uid = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(uid);

        let msg_id = manager
            .send_message(dialog_id, dialog_id, MessageContentType::Text, None)
            .await
            .unwrap();
        let msg = manager.get_message(dialog_id, msg_id).await.unwrap();
        assert_eq!(msg.id(), msg_id);
    }

    #[tokio::test]
    async fn test_delete_messages() {
        let manager = MessagesManager::new();
        let uid = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(uid);

        let msg_id = manager
            .send_message(dialog_id, dialog_id, MessageContentType::Text, None)
            .await
            .unwrap();
        manager
            .delete_messages(dialog_id, vec![msg_id], false)
            .await
            .unwrap();
        assert_eq!(manager.get_message_count(dialog_id).await, 0);
    }
}
