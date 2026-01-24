// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Client lifecycle management for the TDLib adapter.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use crate::error::AdapterError;
use rustgram_client_actor::ClientActor;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Unique client identifier compatible with TDLib.
///
/// TDLib uses `int32_t` for client IDs. This wrapper provides type safety
/// while maintaining compatibility with the TDLib JSON API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(i32);

impl ClientId {
    /// Creates a new ClientId with the specified value.
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the raw i32 value.
    pub const fn get(&self) -> i32 {
        self.0
    }

    /// Checks if this is a valid client ID.
    ///
    /// TDLib uses positive integers for valid client IDs.
    pub const fn is_valid(&self) -> bool {
        self.0 > 0
    }
}

impl From<i32> for ClientId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<ClientId> for i32 {
    fn from(id: ClientId) -> Self {
        id.0
    }
}

/// Configuration for creating a new client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Telegram API ID
    pub api_id: i32,

    /// Telegram API hash
    pub api_hash: String,

    /// Path to the database directory
    pub database_path: String,

    /// Path to files directory
    pub files_directory: String,

    /// Whether to use test datacenter
    pub use_test_dc: bool,

    /// Default DC ID to use
    pub default_dc_id: i32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_id: 0,
            api_hash: String::new(),
            database_path: "/tmp/tdlib".to_string(),
            files_directory: "/tmp/tdlib_files".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        }
    }
}

/// Registry for managing client instances.
///
/// Maps TDLib-compatible client IDs to their corresponding ClientActor instances.
pub struct ClientRegistry {
    /// Map of client IDs to actors
    clients: Arc<RwLock<HashMap<ClientId, Arc<ClientActor>>>>,

    /// Generator for unique client IDs
    next_id: Arc<AtomicI32>,
}

impl ClientRegistry {
    /// Creates a new client registry.
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicI32::new(1)),
        }
    }

    /// Registers a new client and returns its ID.
    ///
    /// # Arguments
    ///
    /// * `actor` - The ClientActor to register
    ///
    /// # Returns
    ///
    /// The unique ClientId assigned to this client
    pub async fn register(&self, actor: Arc<ClientActor>) -> ClientId {
        let id = ClientId::new(self.next_id.fetch_add(1, Ordering::SeqCst));

        let mut clients = self.clients.write().await;
        clients.insert(id, actor);

        info!("Registered new client with ID {}", id.get());
        id
    }

    /// Unregisters a client by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The client ID to unregister
    ///
    /// # Returns
    ///
    /// Ok(()) if the client was unregistered, Err if not found
    pub async fn unregister(&self, id: ClientId) -> Result<(), AdapterError> {
        let mut clients = self.clients.write().await;

        clients
            .remove(&id)
            .ok_or_else(|| AdapterError::InvalidClientId(id.get()))?;

        info!("Unregistered client {}", id.get());
        Ok(())
    }

    /// Gets a client by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The client ID to look up
    ///
    /// # Returns
    ///
    /// A reference to the ClientActor if found
    pub async fn get(&self, id: ClientId) -> Result<Arc<ClientActor>, AdapterError> {
        let clients = self.clients.read().await;

        clients
            .get(&id)
            .cloned()
            .ok_or_else(|| AdapterError::InvalidClientId(id.get()))
    }

    /// Checks if a client ID is registered.
    pub async fn contains(&self, id: ClientId) -> bool {
        let clients = self.clients.read().await;
        clients.contains_key(&id)
    }

    /// Returns the number of registered clients.
    pub async fn count(&self) -> usize {
        let clients = self.clients.read().await;
        clients.len()
    }

    /// Clears all registered clients.
    pub async fn clear(&self) {
        let mut clients = self.clients.write().await;
        let count = clients.len();
        clients.clear();

        if count > 0 {
            info!("Cleared {} registered clients", count);
        }
    }
}

impl Default for ClientRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_id_new() {
        let id = ClientId::new(123);
        assert_eq!(id.get(), 123);
        assert!(id.is_valid());
    }

    #[test]
    fn test_client_id_invalid() {
        let id = ClientId::new(0);
        assert!(!id.is_valid());

        let id = ClientId::new(-1);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_client_id_from_i32() {
        let id: ClientId = 456.into();
        assert_eq!(id.get(), 456);
    }

    #[test]
    fn test_client_id_into_i32() {
        let id = ClientId::new(789);
        let raw: i32 = id.into();
        assert_eq!(raw, 789);
    }

    #[test]
    fn test_client_id_copy() {
        let id1 = ClientId::new(100);
        let id2 = id1;
        assert_eq!(id1.get(), id2.get());
    }

    #[test]
    fn test_client_id_equality() {
        let id1 = ClientId::new(200);
        let id2 = ClientId::new(200);
        let id3 = ClientId::new(201);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.api_id, 0);
        assert!(config.api_hash.is_empty());
        assert_eq!(config.database_path, "/tmp/tdlib");
        assert_eq!(config.files_directory, "/tmp/tdlib_files");
        assert!(!config.use_test_dc);
        assert_eq!(config.default_dc_id, 2);
    }

    #[tokio::test]
    async fn test_registry_new() {
        let registry = ClientRegistry::new();
        assert_eq!(registry.count().await, 0);
    }

    #[tokio::test]
    async fn test_registry_register() {
        let registry = ClientRegistry::new();
        let actor = Arc::new(ClientActor::new_with_defaults());

        let id = registry.register(actor).await;
        assert!(id.is_valid());
        assert_eq!(registry.count().await, 1);
    }

    #[tokio::test]
    async fn test_registry_get() {
        let registry = ClientRegistry::new();
        let actor = Arc::new(ClientActor::new_with_defaults());

        let id = registry.register(actor.clone()).await;
        let retrieved = registry.get(id).await;

        assert!(retrieved.is_ok());
    }

    #[tokio::test]
    async fn test_registry_get_invalid() {
        let registry = ClientRegistry::new();
        let result = registry.get(ClientId::new(999)).await;

        assert!(matches!(result, Err(AdapterError::InvalidClientId(999))));
    }

    #[tokio::test]
    async fn test_registry_unregister() {
        let registry = ClientRegistry::new();
        let actor = Arc::new(ClientActor::new_with_defaults());

        let id = registry.register(actor).await;
        assert_eq!(registry.count().await, 1);

        let result = registry.unregister(id).await;
        assert!(result.is_ok());
        assert_eq!(registry.count().await, 0);
    }

    #[tokio::test]
    async fn test_registry_unregister_invalid() {
        let registry = ClientRegistry::new();
        let result = registry.unregister(ClientId::new(999)).await;

        assert!(matches!(result, Err(AdapterError::InvalidClientId(999))));
    }

    #[tokio::test]
    async fn test_registry_contains() {
        let registry = ClientRegistry::new();
        let actor = Arc::new(ClientActor::new_with_defaults());

        let id = registry.register(actor).await;
        assert!(registry.contains(id).await);
        assert!(!registry.contains(ClientId::new(999)).await);
    }

    #[tokio::test]
    async fn test_registry_multiple_clients() {
        let registry = ClientRegistry::new();

        let actor1 = Arc::new(ClientActor::new_with_defaults());
        let actor2 = Arc::new(ClientActor::new_with_defaults());
        let actor3 = Arc::new(ClientActor::new_with_defaults());

        let id1 = registry.register(actor1).await;
        let id2 = registry.register(actor2).await;
        let id3 = registry.register(actor3).await;

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
        assert_eq!(registry.count().await, 3);
    }

    #[tokio::test]
    async fn test_registry_clear() {
        let registry = ClientRegistry::new();

        for _ in 0..5 {
            let actor = Arc::new(ClientActor::new_with_defaults());
            registry.register(actor).await;
        }

        assert_eq!(registry.count().await, 5);
        registry.clear().await;
        assert_eq!(registry.count().await, 0);
    }

    #[tokio::test]
    async fn test_registry_default() {
        let registry = ClientRegistry::default();
        assert_eq!(registry.count().await, 0);
    }
}
