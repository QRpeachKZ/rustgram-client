// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Update bridge for glib <-> tokio communication.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use crate::client::ClientId;
use crate::error::AdapterError;
use rustgram_client_actor::Update as ClientActorUpdate;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Update type compatible with TDLib JSON format.
///
/// This represents a TDLib update that can be sent to the paper-plane GUI.
pub type Update = String;

/// Errors that can occur in the update bridge.
#[derive(Debug, Error)]
pub enum BridgeError {
    /// Client not found
    #[error("Client {0} not found")]
    ClientNotFound(i32),

    /// Channel full
    #[error("Update channel full for client {0}")]
    ChannelFull(i32),

    /// Channel closed
    #[error("Update channel closed for client {0}")]
    ChannelClosed(i32),

    /// Serialization error
    #[error("Failed to serialize update: {0}")]
    SerializationError(String),
}

/// Converter for transforming rustgram-client updates to TDLib JSON format.
pub struct UpdateConverter;

impl UpdateConverter {
    /// Creates a new update converter.
    pub fn new() -> Self {
        Self
    }

    /// Converts a ClientActor update to TDLib JSON format.
    ///
    /// # Arguments
    ///
    /// * `update` - The ClientActor update to convert
    ///
    /// # Returns
    ///
    /// TDLib-compatible JSON update string
    pub fn convert_update(update: &ClientActorUpdate) -> Update {
        match update {
            ClientActorUpdate::AuthorizationState(value) => {
                // Just forward the existing JSON
                serde_json::to_string(value).unwrap_or_default()
            }
            ClientActorUpdate::NewMessage(value) => {
                let wrapper = json!({
                    "@type": "updateNewMessage",
                    "message": value
                });
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::EditMessage(value) => {
                let wrapper = json!({
                    "@type": "updateEditMessage",
                    "message": value
                });
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::DeleteMessages(value) => {
                let wrapper = json!({
                    "@type": "updateDeleteMessages",
                });
                if let Some(arr) = value.get("message_ids") {
                    let mut wrapper = wrapper;
                    wrapper["message_ids"] = arr.clone();
                    serde_json::to_string(&wrapper).unwrap_or_default()
                } else {
                    serde_json::to_string(&wrapper).unwrap_or_default()
                }
            }
            ClientActorUpdate::UserStatus(value) => {
                let wrapper = json!({
                    "@type": "updateUserStatus",
                });
                let mut wrapper = wrapper;
                if let Some(user_id) = value.get("user_id") {
                    wrapper["user_id"] = user_id.clone();
                }
                if let Some(status) = value.get("status") {
                    wrapper["status"] = status.clone();
                }
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::NewChat(value) => {
                let wrapper = json!({
                    "@type": "updateNewChat",
                    "chat": value
                });
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::Generic(value) => serde_json::to_string(value).unwrap_or_default(),
        }
    }
}

impl Default for UpdateConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Bridge for delivering updates from tokio runtime to glib main loop.
///
/// Uses async_channel to provide thread-safe, non-blocking update delivery.
pub struct UpdateBridge {
    /// Per-client update receivers
    receivers: Arc<RwLock<HashMap<ClientId, async_channel::Receiver<Update>>>>,

    /// Per-client update senders
    senders: Arc<RwLock<HashMap<ClientId, async_channel::Sender<Update>>>>,

    /// Channel capacity for update queues
    capacity: usize,
}

impl UpdateBridge {
    /// Creates a new update bridge with the specified channel capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            receivers: Arc::new(RwLock::new(HashMap::new())),
            senders: Arc::new(RwLock::new(HashMap::new())),
            capacity,
        }
    }

    /// Creates a new update bridge with default capacity (500).
    pub fn with_default_capacity() -> Self {
        Self::new(500)
    }

    /// Registers a client for update delivery.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to register
    ///
    /// # Returns
    ///
    /// Ok(()) if registration succeeded, Err if client already registered
    pub async fn register_client(&self, client_id: ClientId) -> Result<(), AdapterError> {
        let (sender, receiver) = async_channel::bounded(self.capacity);

        let mut senders = self.senders.write().await;
        let mut receivers = self.receivers.write().await;

        if senders.contains_key(&client_id) {
            return Err(AdapterError::InvalidClientId(client_id.get()));
        }

        senders.insert(client_id, sender);
        receivers.insert(client_id, receiver);

        debug!(
            "Registered update channel for client {} (capacity: {})",
            client_id.get(),
            self.capacity
        );

        Ok(())
    }

    /// Unregisters a client from update delivery.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to unregister
    ///
    /// # Returns
    ///
    /// Ok(()) if unregistration succeeded, Err if client not found
    pub async fn unregister_client(&self, client_id: ClientId) -> Result<(), AdapterError> {
        let mut senders = self.senders.write().await;
        let mut receivers = self.receivers.write().await;

        senders
            .remove(&client_id)
            .ok_or_else(|| AdapterError::InvalidClientId(client_id.get()))?;

        receivers.remove(&client_id);

        debug!("Unregistered update channel for client {}", client_id.get());

        Ok(())
    }

    /// Sends an update to a specific client.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The target client ID
    /// * `update` - The update JSON string
    ///
    /// # Returns
    ///
    /// Ok(()) if sent, Err if send failed (channel full or closed)
    pub async fn send_update(
        &self,
        client_id: ClientId,
        update: Update,
    ) -> Result<(), AdapterError> {
        let senders = self.senders.read().await;

        let sender = senders
            .get(&client_id)
            .ok_or_else(|| AdapterError::InvalidClientId(client_id.get()))?;

        // Try to send without blocking
        match sender.try_send(update) {
            Ok(()) => {
                debug!("Sent update to client {}", client_id.get());
                Ok(())
            }
            Err(async_channel::TrySendError::Full(_)) => {
                warn!("Update channel full for client {}", client_id.get());
                Err(AdapterError::RateLimitExceeded)
            }
            Err(async_channel::TrySendError::Closed(_)) => {
                warn!("Update channel closed for client {}", client_id.get());
                Err(AdapterError::InvalidClientId(client_id.get()))
            }
        }
    }

    /// Receives an update for a specific client (non-blocking).
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to receive updates for
    ///
    /// # Returns
    ///
    /// Some(update) if available, None if no updates pending
    pub fn try_receive(&self, client_id: ClientId) -> Option<Update> {
        // Try to get the receiver without blocking
        // Use try_read for RwLock
        let receivers = self.receivers.try_read().ok()?;

        let receiver = receivers.get(&client_id)?;

        match receiver.try_recv() {
            Ok(update) => Some(update),
            Err(async_channel::TryRecvError::Empty) => None,
            Err(async_channel::TryRecvError::Closed) => None,
        }
    }

    /// Converts a ClientActor update to TDLib JSON format.
    ///
    /// # Arguments
    ///
    /// * `update` - The ClientActor update to convert
    ///
    /// # Returns
    ///
    /// TDLib-compatible JSON update string
    pub fn convert_update(&self, update: &ClientActorUpdate) -> Update {
        match update {
            ClientActorUpdate::AuthorizationState(value) => {
                // Just forward the existing JSON
                serde_json::to_string(value).unwrap_or_default()
            }
            ClientActorUpdate::NewMessage(value) => {
                let wrapper = json!({
                    "@type": "updateNewMessage",
                    "message": value
                });
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::EditMessage(value) => {
                let wrapper = json!({
                    "@type": "updateEditMessage",
                    "message": value
                });
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::DeleteMessages(value) => {
                let wrapper = json!({
                    "@type": "updateDeleteMessages",
                });
                if let Some(arr) = value.get("message_ids") {
                    let mut wrapper = wrapper;
                    wrapper["message_ids"] = arr.clone();
                    serde_json::to_string(&wrapper).unwrap_or_default()
                } else {
                    serde_json::to_string(&wrapper).unwrap_or_default()
                }
            }
            ClientActorUpdate::UserStatus(value) => {
                let wrapper = json!({
                    "@type": "updateUserStatus",
                });
                let mut wrapper = wrapper;
                if let Some(user_id) = value.get("user_id") {
                    wrapper["user_id"] = user_id.clone();
                }
                if let Some(status) = value.get("status") {
                    wrapper["status"] = status.clone();
                }
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::NewChat(value) => {
                let wrapper = json!({
                    "@type": "updateNewChat",
                    "chat": value
                });
                serde_json::to_string(&wrapper).unwrap_or_default()
            }
            ClientActorUpdate::Generic(value) => serde_json::to_string(value).unwrap_or_default(),
        }
    }

    /// Returns the number of pending updates for a client.
    pub async fn pending_count(&self, client_id: ClientId) -> usize {
        let receivers = self.receivers.read().await;

        receivers.get(&client_id).map(|r| r.len()).unwrap_or(0)
    }

    /// Clears all pending updates for a client.
    pub async fn clear_updates(&self, client_id: ClientId) -> Result<(), AdapterError> {
        let receivers = self.receivers.read().await;

        let receiver = receivers
            .get(&client_id)
            .ok_or_else(|| AdapterError::InvalidClientId(client_id.get()))?;

        let mut count = 0;
        while receiver.try_recv().is_ok() {
            count += 1;
        }

        if count > 0 {
            debug!(
                "Cleared {} pending updates for client {}",
                count,
                client_id.get()
            );
        }

        Ok(())
    }
}

impl Default for UpdateBridge {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bridge_new() {
        let bridge = UpdateBridge::new(100);
        assert_eq!(bridge.capacity, 100);
    }

    #[tokio::test]
    async fn test_bridge_default() {
        let bridge = UpdateBridge::default();
        assert_eq!(bridge.capacity, 500);
    }

    #[tokio::test]
    async fn test_bridge_register_client() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(1);

        let result = bridge.register_client(client_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_bridge_register_duplicate() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(1);

        bridge.register_client(client_id).await.unwrap();
        let result = bridge.register_client(client_id).await;

        assert!(matches!(result, Err(AdapterError::InvalidClientId(_))));
    }

    #[tokio::test]
    async fn test_bridge_unregister_client() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(1);

        bridge.register_client(client_id).await.unwrap();
        let result = bridge.unregister_client(client_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_bridge_unregister_nonexistent() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(999);

        let result = bridge.unregister_client(client_id).await;

        assert!(matches!(result, Err(AdapterError::InvalidClientId(_))));
    }

    #[tokio::test]
    async fn test_bridge_send_and_receive() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(1);

        bridge.register_client(client_id).await.unwrap();

        let update = r#"{"@type":"updateNewMessage"}"#.to_string();
        let send_result = bridge.send_update(client_id, update.clone()).await;
        assert!(send_result.is_ok());

        let received = bridge.try_receive(client_id);
        assert_eq!(received, Some(update));
    }

    #[tokio::test]
    async fn test_bridge_try_receive_empty() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(1);

        bridge.register_client(client_id).await;

        let received = bridge.try_receive(client_id);
        assert!(received.is_none());
    }

    #[tokio::test]
    async fn test_bridge_send_invalid_client() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(999);

        let update = r#"{"@type":"update"}"#.to_string();
        let result = bridge.send_update(client_id, update).await;

        assert!(matches!(result, Err(AdapterError::InvalidClientId(_))));
    }

    #[tokio::test]
    async fn test_bridge_pending_count() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(1);

        bridge.register_client(client_id).await.unwrap();

        assert_eq!(bridge.pending_count(client_id).await, 0);

        let update = r#"{"@type":"update"}"#;
        bridge
            .send_update(client_id, update.to_string())
            .await
            .unwrap();
        assert_eq!(bridge.pending_count(client_id).await, 1);
    }

    #[tokio::test]
    async fn test_bridge_clear_updates() {
        let bridge = UpdateBridge::default();
        let client_id = ClientId::new(1);

        bridge.register_client(client_id).await.unwrap();

        for i in 0..5 {
            let update = format!(r#"{{"@type":"update","id":{}}}"#, i);
            bridge.send_update(client_id, update).await.unwrap();
        }

        assert_eq!(bridge.pending_count(client_id).await, 5);

        let result = bridge.clear_updates(client_id).await;
        assert!(result.is_ok());
        assert_eq!(bridge.pending_count(client_id).await, 0);
    }

    #[tokio::test]
    async fn test_bridge_convert_update_new_message() {
        let bridge = UpdateBridge::default();

        let msg = json!({
            "id": 1,
            "text": "Hello"
        });
        let update = ClientActorUpdate::NewMessage(msg.clone());

        let converted = bridge.convert_update(&update);
        let value: serde_json::Value = serde_json::from_str(&converted).unwrap();

        assert_eq!(value["@type"], "updateNewMessage");
        assert_eq!(value["message"], msg);
    }

    #[tokio::test]
    async fn test_bridge_convert_update_authorization_state() {
        let bridge = UpdateBridge::default();

        let state = json!({
            "@type": "authorizationStateWaitPhoneNumber"
        });
        let update = ClientActorUpdate::AuthorizationState(state.clone());

        let converted = bridge.convert_update(&update);
        let value: serde_json::Value = serde_json::from_str(&converted).unwrap();

        assert_eq!(value["@type"], "authorizationStateWaitPhoneNumber");
    }

    #[tokio::test]
    async fn test_bridge_convert_update_delete_messages() {
        let bridge = UpdateBridge::default();

        let msg_ids = json!({"message_ids": [1, 2, 3]});
        let update = ClientActorUpdate::DeleteMessages(msg_ids.clone());

        let converted = bridge.convert_update(&update);
        let value: serde_json::Value = serde_json::from_str(&converted).unwrap();

        assert_eq!(value["@type"], "updateDeleteMessages");
        assert_eq!(value["message_ids"], msg_ids["message_ids"]);
    }

    #[tokio::test]
    async fn test_bridge_convert_update_generic() {
        let bridge = UpdateBridge::default();

        let generic = json!({
            "@type": "updateCustom",
            "data": "test"
        });
        let update = ClientActorUpdate::Generic(generic.clone());

        let converted = bridge.convert_update(&update);
        let value: serde_json::Value = serde_json::from_str(&converted).unwrap();

        assert_eq!(value, generic);
    }

    #[tokio::test]
    async fn test_bridge_multiple_clients() {
        let bridge = UpdateBridge::default();

        let id1 = ClientId::new(1);
        let id2 = ClientId::new(2);

        bridge.register_client(id1).await.unwrap();
        bridge.register_client(id2).await.unwrap();

        let update1 = r#"{"@type":"update","client":1}"#;
        let update2 = r#"{"@type":"update","client":2}"#;

        bridge.send_update(id1, update1.to_string()).await.unwrap();
        bridge.send_update(id2, update2.to_string()).await.unwrap();

        let recv1 = bridge.try_receive(id1);
        let recv2 = bridge.try_receive(id2);

        assert_eq!(recv1, Some(update1.to_string()));
        assert_eq!(recv2, Some(update2.to_string()));
    }

    #[tokio::test]
    async fn test_bridge_channel_capacity_limit() {
        let bridge = UpdateBridge::new(2);
        let client_id = ClientId::new(1);

        bridge.register_client(client_id).await.unwrap();

        let update = r#"{"@type":"update"}"#;

        // Fill the channel
        let result1 = bridge.send_update(client_id, update.to_string()).await;
        assert!(result1.is_ok());

        let result2 = bridge.send_update(client_id, update.to_string()).await;
        assert!(result2.is_ok());

        // This should fail - channel full
        let result3 = bridge.send_update(client_id, update.to_string()).await;
        assert!(matches!(result3, Err(AdapterError::RateLimitExceeded)));
    }
}
