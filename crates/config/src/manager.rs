// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Config manager module.
//!
//! Centralized configuration management with caching and updates.

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::app::AppConfig;
use crate::dh::DhConfig;
use crate::error::{ConfigError, Result};
use crate::simple::SimpleConfig;
use crate::tl::{GetAppConfig, GetConfig, GetDhConfig};
use rustgram_types::TlSerialize;

use bytes::BytesMut;
use rustgram_net::DcId;

/// Network query callback trait.
///
/// Abstraction for sending network queries and receiving responses.
pub trait NetQueryCallback: Send + Sync {
    /// Sends a query and returns the response.
    fn send_query(&self, query: NetQuery) -> Result<bytes::Bytes>;
}

/// Network query representation.
#[derive(Debug, Clone)]
pub struct NetQuery {
    /// Query data.
    pub data: bytes::Bytes,
    /// Target DC ID.
    pub dc_id: DcId,
    /// Whether authentication is required.
    pub auth_required: bool,
    /// TL constructor ID.
    pub tl_constructor: i32,
}

/// Sender for network queries.
///
/// This is a simplified type alias. In a real implementation,
/// this would be a channel sender to the NetQueryDispatcher.
pub type NetQuerySender = Arc<dyn NetQueryCallback>;

/// Configuration update types.
///
/// Represents different types of configuration updates that can be received.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigUpdate {
    /// Application config update.
    AppConfig(AppConfig),

    /// DC options update.
    DcOptions(rustgram_net::DcOptions),

    /// TTL update.
    Ttl(i32),
}

impl ConfigUpdate {
    /// Creates a new AppConfig update.
    pub fn app_config(config: AppConfig) -> Self {
        Self::AppConfig(config)
    }

    /// Creates a new DC options update.
    pub fn dc_options(options: rustgram_net::DcOptions) -> Self {
        Self::DcOptions(options)
    }

    /// Creates a new TTL update.
    pub fn ttl(ttl: i32) -> Self {
        Self::Ttl(ttl)
    }
}

/// Configuration manager.
///
/// Centralized manager for all Telegram configuration with caching
/// and automatic refresh capabilities.
///
/// # Examples
///
/// ```no_run
/// use rustgram_config::ConfigManager;
/// use std::sync::Arc;
///
/// struct MockCallback;
/// impl rustgram_config::NetQueryCallback for MockCallback {
///     fn send_query(&self, _query: rustgram_config::NetQuery) -> rustgram_config::Result<bytes::Bytes> {
///         Ok(bytes::Bytes::new())
///     }
/// }
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let sender = Arc::new(MockCallback);
/// let manager = ConfigManager::new(sender);
///
/// // Fetch simple config
/// let simple_config = manager.get_simple_config().await?;
///
/// // Fetch app config (cached)
/// let app_config = manager.get_app_config().await?;
/// # Ok(())
/// # }
/// ```
pub struct ConfigManager {
    /// Cached simple config.
    simple_config: Arc<RwLock<Option<SimpleConfig>>>,

    /// Cached app config.
    app_config: Arc<RwLock<Option<AppConfig>>>,

    /// Cached DH config.
    dh_config: Arc<RwLock<Option<DhConfig>>>,

    /// Current DC ID.
    dc_id: Arc<RwLock<DcId>>,

    /// Test mode flag.
    test_mode: Arc<RwLock<bool>>,
}

impl ConfigManager {
    /// Creates a new ConfigManager.
    ///
    /// # Arguments
    ///
    /// * `_net_query_sender` - Sender for network queries (reserved for future use)
    pub fn new(_net_query_sender: NetQuerySender) -> Self {
        Self {
            simple_config: Arc::new(RwLock::new(None)),
            app_config: Arc::new(RwLock::new(None)),
            dh_config: Arc::new(RwLock::new(None)),
            dc_id: Arc::new(RwLock::new(DcId::internal(1))),
            test_mode: Arc::new(RwLock::new(false)),
        }
    }

    /// Fetches the simple configuration.
    ///
    /// This will use the cached value if available and valid.
    pub async fn get_simple_config(&self) -> Result<SimpleConfig> {
        // Check cache first
        {
            let cache = self.simple_config.read().await;
            if let Some(ref config) = *cache {
                debug!("Returning cached simple config");
                return Ok(config.clone());
            }
        }

        // Fetch from network
        info!("Fetching simple config from network");
        let config = self.fetch_simple_config_internal().await?;

        // Cache the result
        {
            let mut cache = self.simple_config.write().await;
            *cache = Some(config.clone());
        }

        Ok(config)
    }

    /// Fetches the app configuration.
    ///
    /// This will use the cached value if available and not expired.
    pub async fn get_app_config(&self) -> Result<AppConfig> {
        // Check cache first
        {
            let cache = self.app_config.read().await;
            if let Some(ref config) = *cache {
                if !config.needs_refresh() {
                    debug!("Returning cached app config");
                    return Ok(config.clone());
                }
                debug!("App config needs refresh");
            }
        }

        // Fetch from network
        info!("Fetching app config from network");
        let config = self.fetch_app_config_internal().await?;

        // Cache the result
        {
            let mut cache = self.app_config.write().await;
            *cache = Some(config.clone());
        }

        Ok(config)
    }

    /// Fetches the DH configuration.
    ///
    /// # Arguments
    ///
    /// * `version` - Current DH version (use 0 if unknown)
    pub async fn get_dh_config(&self, version: i32) -> Result<DhConfig> {
        // Check cache first
        {
            let cache = self.dh_config.read().await;
            if let Some(ref config) = *cache {
                if !config.needs_update(version) {
                    debug!("Returning cached DH config");
                    return Ok(config.clone());
                }
                debug!("DH config needs update");
            }
        }

        // Fetch from network
        info!("Fetching DH config from network, version={}", version);
        let config = self.fetch_dh_config_internal(version).await?;

        // Cache the result
        {
            let mut cache = self.dh_config.write().await;
            *cache = Some(config.clone());
        }

        Ok(config)
    }

    /// Invalidates the cached app config.
    ///
    /// Forces a refresh on the next call to `get_app_config`.
    pub async fn invalidate_app_config(&self) {
        let mut cache = self.app_config.write().await;
        *cache = None;
        debug!("App config cache invalidated");
    }

    /// Invalidates all cached configs.
    pub async fn invalidate_all(&self) {
        {
            let mut cache = self.app_config.write().await;
            *cache = None;
        }
        {
            let mut cache = self.simple_config.write().await;
            *cache = None;
        }
        {
            let mut cache = self.dh_config.write().await;
            *cache = None;
        }
        debug!("All config caches invalidated");
    }

    /// Handles a configuration update.
    ///
    /// Called when receiving a config update from Telegram.
    pub fn on_update(&self, update: ConfigUpdate) {
        debug!(
            "Received config update: {:?}",
            std::mem::discriminant(&update)
        );

        match update {
            ConfigUpdate::AppConfig(config) => {
                let cache = Arc::clone(&self.app_config);
                tokio::spawn(async move {
                    let mut cache = cache.write().await;
                    *cache = Some(config);
                    debug!("App config updated");
                });
            }
            ConfigUpdate::DcOptions(options) => {
                let simple_cache = Arc::clone(&self.simple_config);
                let dc_id = Arc::clone(&self.dc_id);

                tokio::spawn(async move {
                    let current_dc_id = *dc_id.read().await;
                    let mut cache = simple_cache.write().await;
                    if let Some(ref mut config) = *cache {
                        config.set_dc_options(options);
                    } else {
                        *cache = Some(SimpleConfig {
                            dc_options: options,
                            dc_id: current_dc_id,
                            test_mode: false,
                        });
                    }
                    debug!("DC options updated");
                });
            }
            ConfigUpdate::Ttl(ttl) => {
                info!("Received TTL update: {}", ttl);
                // TODO: TTL updates should affect future request behavior
                // Currently not implemented
            }
        }
    }

    /// Returns the current DC ID.
    pub async fn dc_id(&self) -> DcId {
        *self.dc_id.read().await
    }

    /// Sets the current DC ID.
    pub async fn set_dc_id(&self, dc_id: DcId) {
        let mut dc = self.dc_id.write().await;
        *dc = dc_id;
        debug!("DC ID set to {}", dc_id);
    }

    /// Returns `true` if in test mode.
    pub async fn is_test_mode(&self) -> bool {
        *self.test_mode.read().await
    }

    /// Sets the test mode flag.
    pub async fn set_test_mode(&self, test_mode: bool) {
        let mut tm = self.test_mode.write().await;
        *tm = test_mode;
        debug!("Test mode set to {}", test_mode);
    }

    /// Fetches simple config from network (internal).
    async fn fetch_simple_config_internal(&self) -> Result<SimpleConfig> {
        let req = GetConfig::new();
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        let _query = NetQuery {
            data: buf.freeze(),
            dc_id: self.dc_id().await,
            auth_required: false,
            tl_constructor: req.constructor_id() as i32,
        };

        // TODO: Implement network query to send and receive config.getConfig response
        // This should:
        // 1. Send the query via NetQuerySender
        // 2. Wait for response
        // 3. Deserialize ConfigDcOptions or ConfigDcOptionsEmpty
        // 4. Return SimpleConfig
        unimplemented!(
            "Network query for getConfig not yet implemented. \
            NetQuery: {:?}",
            _query
        )
    }

    /// Fetches app config from network (internal).
    async fn fetch_app_config_internal(&self) -> Result<AppConfig> {
        let cache_hash = {
            let cache = self.app_config.read().await;
            cache.as_ref().map(|c| c.cache_hash()).unwrap_or(0)
        };

        let req = GetAppConfig::new(cache_hash);
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        let _query = NetQuery {
            data: buf.freeze(),
            dc_id: self.dc_id().await,
            auth_required: true,
            tl_constructor: crate::tl::GET_APP_CONFIG_MAGIC as i32,
        };

        // TODO: Implement network query to send and receive help.getAppConfig response
        // This should:
        // 1. Send the query via NetQuerySender
        // 2. Wait for response
        // 3. Deserialize AppConfigTl
        // 4. Return AppConfig
        unimplemented!(
            "Network query for getAppConfig not yet implemented. \
            NetQuery: {:?}",
            _query
        )
    }

    /// Fetches DH config from network (internal).
    async fn fetch_dh_config_internal(&self, version: i32) -> Result<DhConfig> {
        let req = GetDhConfig::new(version, 256);
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        let _query = NetQuery {
            data: buf.freeze(),
            dc_id: self.dc_id().await,
            auth_required: true,
            tl_constructor: crate::tl::GET_DH_CONFIG_MAGIC as i32,
        };

        // TODO: Implement network query to send and receive messages.getDhConfig response
        // This should:
        // 1. Send the query via NetQuerySender
        // 2. Wait for response (DhConfigTl or DhConfigNotModified)
        // 3. Deserialize and handle both response types
        // 4. Return DhConfig
        unimplemented!(
            "Network query for getDhConfig not yet implemented. \
            NetQuery: {:?}",
            _query
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    struct MockCallback;

    impl NetQueryCallback for MockCallback {
        fn send_query(&self, _query: NetQuery) -> Result<bytes::Bytes> {
            Ok(bytes::Bytes::new())
        }
    }

    fn make_test_manager() -> ConfigManager {
        let sender = Arc::new(MockCallback);
        ConfigManager::new(sender)
    }

    #[tokio::test]
    async fn test_config_manager_creation() {
        let manager = make_test_manager();

        assert_eq!(manager.dc_id().await, DcId::internal(1));
        assert!(!manager.is_test_mode().await);
    }

    #[tokio::test]
    async fn test_config_manager_set_dc_id() {
        let manager = make_test_manager();

        manager.set_dc_id(DcId::internal(4)).await;
        assert_eq!(manager.dc_id().await, DcId::internal(4));
    }

    #[tokio::test]
    async fn test_config_manager_set_test_mode() {
        let manager = make_test_manager();

        manager.set_test_mode(true).await;
        assert!(manager.is_test_mode().await);
    }

    #[tokio::test]
    async fn test_config_update_app_config() {
        let manager = make_test_manager();

        let config = AppConfig::default();
        manager.on_update(ConfigUpdate::app_config(config.clone()));

        // Give time for async update
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Should now have cached config
        let cache = manager.app_config.read().await;
        assert!(cache.is_some());
    }

    #[tokio::test]
    async fn test_config_update_ttl() {
        let manager = make_test_manager();

        // Should not panic
        manager.on_update(ConfigUpdate::ttl(300));
    }

    #[tokio::test]
    async fn test_invalidate_app_config() {
        let manager = make_test_manager();

        // Set a cached config
        {
            let mut cache = manager.app_config.write().await;
            *cache = Some(AppConfig::default());
        }

        // Invalidate
        manager.invalidate_app_config().await;

        let cache = manager.app_config.read().await;
        assert!(cache.is_none());
    }

    #[tokio::test]
    async fn test_invalidate_all() {
        let manager = make_test_manager();

        // Set cached configs
        {
            let mut simple = manager.simple_config.write().await;
            *simple = Some(SimpleConfig::default());
        }
        {
            let mut app = manager.app_config.write().await;
            *app = Some(AppConfig::default());
        }

        // Invalidate all
        manager.invalidate_all().await;

        assert!(manager.simple_config.read().await.is_none());
        assert!(manager.app_config.read().await.is_none());
    }

    #[test]
    fn test_config_update_discriminants() {
        use std::mem::discriminant;

        let update1 = ConfigUpdate::AppConfig(AppConfig::default());
        let update2 = ConfigUpdate::AppConfig(AppConfig::default());
        let update3 = ConfigUpdate::Ttl(300);

        assert_eq!(discriminant(&update1), discriminant(&update2));
        assert_ne!(discriminant(&update1), discriminant(&update3));
    }
}
