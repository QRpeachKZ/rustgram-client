// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Call Manager
//!
//! Manager for 1-on-1 voice and video calls in Telegram.
//!
//! ## Overview
//!
//! The `CallManager` handles creation, management, and termination of
//! individual voice and video calls. It provides methods for:
//!
//! - Creating outgoing calls
//! - Accepting incoming calls
//! - Sending WebRTC signaling data
//! - Discarding/ending calls
//! - Rating call quality
//!
//! ## Architecture
//!
//! Based on TDLib's `CallManager` class, this module:
//! - Tracks active calls by ID
//! - Manages call state transitions
//! - Handles signaling data exchange
//! - Provides call rating and feedback
//!
//! ## Call States
//!
//! ```text
//! Pending -> Active -> Discarded
//!                     -> Rated
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_call_manager::CallManager;
//! use rustgram_types::UserId;
//! use rustgram_call_id::CallId;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut manager = CallManager::new();
//!
//!     // Create an outgoing call
//!     let user_id = UserId::new(1234567890).unwrap();
//!     let call_id = manager.create_call(user_id, false).await?;
//!
//!     // Accept an incoming call
//!     manager.accept_call(call_id, false).await?;
//!
//!     // Send signaling data
//!     manager.send_signaling_data(call_id, b"signaling_data".to_vec()).await?;
//!
//!     // Discard the call
//!     manager.discard_call(call_id, false).await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;

use rustgram_call_discard_reason::CallDiscardReason;
use rustgram_call_id::CallId;
use rustgram_types::UserId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use error::{Error, Result};

/// Call protocol configuration
///
/// Contains WebRTC protocol parameters for a call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallProtocol {
    /// Protocol version
    version: i32,
    /// Whether UDP is enabled
    udp_enabled: bool,
    /// Minimum UDP port
    min_udp_port: i32,
    /// Maximum UDP port
    max_udp_port: i32,
}

impl Default for CallProtocol {
    fn default() -> Self {
        Self {
            version: 7,
            udp_enabled: true,
            min_udp_port: 0,
            max_udp_port: 0,
        }
    }
}

impl CallProtocol {
    /// Creates a new call protocol
    #[must_use]
    pub const fn new(
        version: i32,
        udp_enabled: bool,
        min_udp_port: i32,
        max_udp_port: i32,
    ) -> Self {
        Self {
            version,
            udp_enabled,
            min_udp_port,
            max_udp_port,
        }
    }

    /// Returns the protocol version
    #[must_use]
    pub const fn version(&self) -> i32 {
        self.version
    }

    /// Returns whether UDP is enabled
    #[must_use]
    pub const fn udp_enabled(&self) -> bool {
        self.udp_enabled
    }

    /// Returns the minimum UDP port
    #[must_use]
    pub const fn min_udp_port(&self) -> i32 {
        self.min_udp_port
    }

    /// Returns the maximum UDP port
    #[must_use]
    pub const fn max_udp_port(&self) -> i32 {
        self.max_udp_port
    }
}

/// Call state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallState {
    /// Call is pending
    Pending,
    /// Call is active
    Active,
    /// Call was discarded
    Discarded(CallDiscardReason),
}

/// Active call information
#[derive(Debug, Clone)]
struct CallInfo {
    /// Unique call identifier
    call_id: CallId,
    /// User ID of the other participant
    user_id: UserId,
    /// Whether this is a video call
    is_video: bool,
    /// Current call state
    state: CallState,
    /// Call protocol
    protocol: CallProtocol,
    /// Server call ID
    server_call_id: Option<i64>,
    /// Pending signaling updates
    pending_updates: Vec<String>,
}

/// Manager for 1-on-1 voice and video calls
///
/// Handles creation, management, and termination of individual calls.
/// Based on TDLib's `CallManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_call_manager::CallManager;
/// use rustgram_types::UserId;
/// use rustgram_call_id::CallId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut manager = CallManager::new();
///
/// // Create an outgoing call
/// let user_id = UserId::new(1234567890).unwrap();
/// let call_id = manager.create_call(user_id, false).await?;
/// assert!(call_id.is_valid());
///
/// // Accept the call
/// manager.accept_call(call_id, false).await?;
///
/// // Discard the call
/// manager.discard_call(call_id, false).await?;
///
/// Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CallManager {
    /// Next call ID to assign
    next_call_id: Arc<AtomicI32>,
    /// Next connection ID
    next_connection_id: Arc<AtomicU64>,
    /// Active calls by call ID
    calls: Arc<RwLock<HashMap<CallId, CallInfo>>>,
    /// Calls by user ID
    user_calls: Arc<RwLock<HashMap<UserId, CallId>>>,
}

impl Default for CallManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CallManager {
    /// Creates a new call manager
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    ///
    /// let manager = CallManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_call_id: Arc::new(AtomicI32::new(1)),
            next_connection_id: Arc::new(AtomicU64::new(1)),
            calls: Arc::new(RwLock::new(HashMap::new())),
            user_calls: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new outgoing call
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID of the recipient
    /// * `is_video` - Whether this is a video call
    ///
    /// # Returns
    ///
    /// The call ID of the created call
    ///
    /// # Errors
    ///
    /// Returns an error if a call with this user already exists
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    ///
    /// let call_id = manager.create_call(user_id, false).await?;
    /// assert!(call_id.is_valid());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_call(&self, user_id: UserId, is_video: bool) -> Result<CallId> {
        // Check if there's already an active call with this user
        let user_calls = self.user_calls.read().await;
        if let Some(&existing_call_id) = user_calls.get(&user_id) {
            return Err(Error::CallAlreadyExists(existing_call_id));
        }
        drop(user_calls);

        let call_id = CallId::new(self.next_call_id.fetch_add(1, Ordering::SeqCst));
        let call_info = CallInfo {
            call_id,
            user_id,
            is_video,
            state: CallState::Pending,
            protocol: CallProtocol::default(),
            server_call_id: None,
            pending_updates: Vec::new(),
        };

        let mut calls = self.calls.write().await;
        let mut user_calls = self.user_calls.write().await;

        calls.insert(call_id, call_info);
        user_calls.insert(user_id, call_id);

        Ok(call_id)
    }

    /// Accepts an incoming call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID to accept
    /// * `is_video` - Whether to accept as video call
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist or is not in pending state
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    /// use rustgram_call_id::CallId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, false).await?;
    ///
    /// manager.accept_call(call_id, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn accept_call(&self, call_id: CallId, is_video: bool) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        if !matches!(call_info.state, CallState::Pending) {
            return Err(Error::InvalidState);
        }

        call_info.state = CallState::Active;
        call_info.is_video = is_video;

        Ok(())
    }

    /// Sends signaling data for a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID
    /// * `data` - Signaling data to send
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist or is not active
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, false).await?;
    /// manager.accept_call(call_id, false).await?;
    ///
    /// manager.send_signaling_data(call_id, b"signaling_data".to_vec()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_signaling_data(&self, call_id: CallId, data: Vec<u8>) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        if !matches!(call_info.state, CallState::Active) {
            return Err(Error::InvalidState);
        }

        // Store signaling data as base64 string
        let data_str = base64_simulator(&data);
        call_info.pending_updates.push(data_str);

        Ok(())
    }

    /// Receives signaling data for a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID
    /// * `data` - Received signaling data
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist or is not active
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, false).await?;
    /// manager.accept_call(call_id, false).await?;
    ///
    /// manager.update_signaling_data(call_id, b"remote_signaling".to_vec()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_signaling_data(&self, call_id: CallId, data: Vec<u8>) -> Result<()> {
        let calls = self.calls.read().await;

        let call_info = calls.get(&call_id).ok_or(Error::CallNotFound(call_id))?;

        if !matches!(call_info.state, CallState::Active) {
            return Err(Error::InvalidState);
        }

        // Process received signaling data
        let _data_str = base64_simulator(&data);

        Ok(())
    }

    /// Discards a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID to discard
    /// * `is_disconnected` - Whether the call was disconnected
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, false).await?;
    ///
    /// manager.discard_call(call_id, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discard_call(&self, call_id: CallId, is_disconnected: bool) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        let reason = if is_disconnected {
            CallDiscardReason::Disconnected
        } else {
            CallDiscardReason::HungUp
        };

        call_info.state = CallState::Discarded(reason);

        // Remove from user calls
        let mut user_calls = self.user_calls.write().await;
        user_calls.remove(&call_info.user_id);

        Ok(())
    }

    /// Rates a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID to rate
    /// * `rating` - Rating from 1 (worst) to 5 (best)
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist or is not discarded
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, false).await?;
    /// manager.discard_call(call_id, false).await?;
    ///
    /// manager.rate_call(call_id, 5).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rate_call(&self, call_id: CallId, rating: i32) -> Result<()> {
        let calls = self.calls.read().await;

        let call_info = calls.get(&call_id).ok_or(Error::CallNotFound(call_id))?;

        if !matches!(call_info.state, CallState::Discarded(_)) {
            return Err(Error::InvalidState);
        }

        if !(1..=5).contains(&rating) {
            return Err(Error::InvalidRating);
        }

        // In a real implementation, this would send the rating to the server
        Ok(())
    }

    /// Gets the state of a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID
    ///
    /// # Returns
    ///
    /// The current call state
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, false).await?;
    ///
    /// let state = manager.get_call_state(call_id).await?;
    /// assert!(matches!(state, rustgram_call_manager::CallState::Pending));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_call_state(&self, call_id: CallId) -> Result<CallState> {
        let calls = self.calls.read().await;

        let call_info = calls.get(&call_id).ok_or(Error::CallNotFound(call_id))?;

        Ok(call_info.state.clone())
    }

    /// Gets the call ID for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    ///
    /// # Returns
    ///
    /// The call ID if there's an active call with this user
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    ///
    /// assert!(manager.get_call_by_user(user_id).await?.is_none());
    ///
    /// let call_id = manager.create_call(user_id, false).await?;
    /// assert_eq!(manager.get_call_by_user(user_id).await?, Some(call_id));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_call_by_user(&self, user_id: UserId) -> Result<Option<CallId>> {
        let user_calls = self.user_calls.read().await;
        Ok(user_calls.get(&user_id).copied())
    }

    /// Gets the user ID for a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID
    ///
    /// # Returns
    ///
    /// The user ID of the other participant
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, false).await?;
    ///
    /// assert_eq!(manager.get_call_user(call_id).await?, user_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_call_user(&self, call_id: CallId) -> Result<UserId> {
        let calls = self.calls.read().await;

        let call_info = calls.get(&call_id).ok_or(Error::CallNotFound(call_id))?;

        Ok(call_info.user_id)
    }

    /// Checks if a call is a video call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Call ID
    ///
    /// # Returns
    ///
    /// Whether this is a video call
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let call_id = manager.create_call(user_id, true).await?;
    ///
    /// assert!(manager.is_video_call(call_id).await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_video_call(&self, call_id: CallId) -> Result<bool> {
        let calls = self.calls.read().await;

        let call_info = calls.get(&call_id).ok_or(Error::CallNotFound(call_id))?;

        Ok(call_info.is_video)
    }

    /// Returns the number of active calls
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = CallManager::new();
    /// assert_eq!(manager.active_call_count().await, 0);
    ///
    /// let user_id = UserId::new(1234567890).unwrap();
    /// manager.create_call(user_id, false).await?;
    /// assert_eq!(manager.active_call_count().await, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn active_call_count(&self) -> usize {
        self.calls.read().await.len()
    }

    /// Generates a new connection ID for WebRTC
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_call_manager::CallManager;
    ///
    /// let manager = CallManager::new();
    /// let conn_id = manager.next_connection_id();
    /// assert!(conn_id > 0);
    /// ```
    #[must_use]
    pub fn next_connection_id(&self) -> i64 {
        self.next_connection_id.fetch_add(1, Ordering::SeqCst) as i64
    }
}

/// Simple base64 simulator for tests
fn base64_simulator(data: &[u8]) -> String {
    format!("base64:{}", hex::encode(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_call_manager_new() {
        let manager = CallManager::new();
        assert_eq!(manager.next_call_id.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_call_manager_default() {
        let manager = CallManager::default();
        assert_eq!(manager.next_call_id.load(Ordering::SeqCst), 1);
    }

    // ========== CallProtocol Tests ==========

    #[test]
    fn test_call_protocol_new() {
        let protocol = CallProtocol::new(7, true, 0, 0);
        assert_eq!(protocol.version(), 7);
        assert!(protocol.udp_enabled());
        assert_eq!(protocol.min_udp_port(), 0);
        assert_eq!(protocol.max_udp_port(), 0);
    }

    #[test]
    fn test_call_protocol_default() {
        let protocol = CallProtocol::default();
        assert_eq!(protocol.version(), 7);
        assert!(protocol.udp_enabled());
    }

    // ========== Create Call Tests ==========

    #[tokio::test]
    async fn test_create_call_returns_valid_id() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();

        let call_id = manager.create_call(user_id, false).await.unwrap();
        assert!(call_id.is_valid());
        assert_eq!(call_id.get(), 1);
    }

    #[tokio::test]
    async fn test_create_call_increments_id() {
        let manager = CallManager::new();
        let user_id1 = UserId::new(1).unwrap();
        let user_id2 = UserId::new(2).unwrap();

        let call_id1 = manager.create_call(user_id1, false).await.unwrap();
        let call_id2 = manager.create_call(user_id2, false).await.unwrap();

        assert_eq!(call_id1.get(), 1);
        assert_eq!(call_id2.get(), 2);
    }

    #[tokio::test]
    async fn test_create_call_video() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();

        let call_id = manager.create_call(user_id, true).await.unwrap();
        assert!(manager.is_video_call(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_create_call_already_exists() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();

        manager.create_call(user_id, false).await.unwrap();
        let result = manager.create_call(user_id, false).await;

        assert!(matches!(result, Err(Error::CallAlreadyExists(_))));
    }

    // ========== Accept Call Tests ==========

    #[tokio::test]
    async fn test_accept_call_success() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.accept_call(call_id, false).await.unwrap();

        let state = manager.get_call_state(call_id).await.unwrap();
        assert!(matches!(state, CallState::Active));
    }

    #[tokio::test]
    async fn test_accept_call_not_found() {
        let manager = CallManager::new();
        let call_id = CallId::new(999);

        let result = manager.accept_call(call_id, false).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    #[tokio::test]
    async fn test_accept_call_invalid_state() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.discard_call(call_id, false).await.unwrap();

        let result = manager.accept_call(call_id, false).await;
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    #[tokio::test]
    async fn test_accept_call_as_video() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.accept_call(call_id, true).await.unwrap();
        assert!(manager.is_video_call(call_id).await.unwrap());
    }

    // ========== Send Signaling Data Tests ==========

    #[tokio::test]
    async fn test_send_signaling_data_success() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.accept_call(call_id, false).await.unwrap();
        manager
            .send_signaling_data(call_id, b"test_data".to_vec())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_send_signaling_data_not_active() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        let result = manager
            .send_signaling_data(call_id, b"test_data".to_vec())
            .await;
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    // ========== Update Signaling Data Tests ==========

    #[tokio::test]
    async fn test_update_signaling_data_success() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.accept_call(call_id, false).await.unwrap();
        manager
            .update_signaling_data(call_id, b"remote_data".to_vec())
            .await
            .unwrap();
    }

    // ========== Discard Call Tests ==========

    #[tokio::test]
    async fn test_discard_call_success() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.discard_call(call_id, false).await.unwrap();

        let state = manager.get_call_state(call_id).await.unwrap();
        assert!(matches!(
            state,
            CallState::Discarded(CallDiscardReason::HungUp)
        ));
    }

    #[tokio::test]
    async fn test_discard_call_disconnected() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.discard_call(call_id, true).await.unwrap();

        let state = manager.get_call_state(call_id).await.unwrap();
        assert!(matches!(
            state,
            CallState::Discarded(CallDiscardReason::Disconnected)
        ));
    }

    #[tokio::test]
    async fn test_discard_call_not_found() {
        let manager = CallManager::new();
        let call_id = CallId::new(999);

        let result = manager.discard_call(call_id, false).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    #[tokio::test]
    async fn test_discard_call_removes_from_user_calls() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        assert_eq!(
            manager.get_call_by_user(user_id).await.unwrap(),
            Some(call_id)
        );

        manager.discard_call(call_id, false).await.unwrap();

        assert_eq!(manager.get_call_by_user(user_id).await.unwrap(), None);
    }

    // ========== Rate Call Tests ==========

    #[tokio::test]
    async fn test_rate_call_success() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.discard_call(call_id, false).await.unwrap();
        manager.rate_call(call_id, 5).await.unwrap();
    }

    #[tokio::test]
    async fn test_rate_call_invalid_rating_low() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.discard_call(call_id, false).await.unwrap();

        let result = manager.rate_call(call_id, 0).await;
        assert!(matches!(result, Err(Error::InvalidRating)));
    }

    #[tokio::test]
    async fn test_rate_call_invalid_rating_high() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        manager.discard_call(call_id, false).await.unwrap();

        let result = manager.rate_call(call_id, 6).await;
        assert!(matches!(result, Err(Error::InvalidRating)));
    }

    #[tokio::test]
    async fn test_rate_call_invalid_state() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        let result = manager.rate_call(call_id, 5).await;
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    // ========== Get Call State Tests ==========

    #[tokio::test]
    async fn test_get_call_state_pending() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        let state = manager.get_call_state(call_id).await.unwrap();
        assert!(matches!(state, CallState::Pending));
    }

    #[tokio::test]
    async fn test_get_call_state_not_found() {
        let manager = CallManager::new();
        let call_id = CallId::new(999);

        let result = manager.get_call_state(call_id).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    // ========== Get Call By User Tests ==========

    #[tokio::test]
    async fn test_get_call_by_user_none() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();

        let result = manager.get_call_by_user(user_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_call_by_user_some() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        let result = manager.get_call_by_user(user_id).await.unwrap();
        assert_eq!(result, Some(call_id));
    }

    // ========== Get Call User Tests ==========

    #[tokio::test]
    async fn test_get_call_user_success() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        let result = manager.get_call_user(call_id).await.unwrap();
        assert_eq!(result, user_id);
    }

    #[tokio::test]
    async fn test_get_call_user_not_found() {
        let manager = CallManager::new();
        let call_id = CallId::new(999);

        let result = manager.get_call_user(call_id).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    // ========== Is Video Call Tests ==========

    #[tokio::test]
    async fn test_is_video_call_true() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, true).await.unwrap();

        assert!(manager.is_video_call(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_is_video_call_false() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, false).await.unwrap();

        assert!(!manager.is_video_call(call_id).await.unwrap());
    }

    // ========== Active Call Count Tests ==========

    #[tokio::test]
    async fn test_active_call_count_zero() {
        let manager = CallManager::new();
        assert_eq!(manager.active_call_count().await, 0);
    }

    #[tokio::test]
    async fn test_active_call_count_multiple() {
        let manager = CallManager::new();
        let user_id1 = UserId::new(1).unwrap();
        let user_id2 = UserId::new(2).unwrap();
        let user_id3 = UserId::new(3).unwrap();

        manager.create_call(user_id1, false).await.unwrap();
        manager.create_call(user_id2, false).await.unwrap();
        manager.create_call(user_id3, false).await.unwrap();

        assert_eq!(manager.active_call_count().await, 3);
    }

    // ========== Connection ID Tests ==========

    #[test]
    fn test_next_connection_id_increments() {
        let manager = CallManager::new();

        let id1 = manager.next_connection_id();
        let id2 = manager.next_connection_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    // ========== CallState Equality Tests ==========

    #[test]
    fn test_call_state_equality() {
        assert_eq!(CallState::Pending, CallState::Pending);
        assert_eq!(CallState::Active, CallState::Active);
        assert_eq!(
            CallState::Discarded(CallDiscardReason::HungUp),
            CallState::Discarded(CallDiscardReason::HungUp)
        );
    }

    #[test]
    fn test_call_state_inequality() {
        assert_ne!(CallState::Pending, CallState::Active);
        assert_ne!(
            CallState::Discarded(CallDiscardReason::HungUp),
            CallState::Discarded(CallDiscardReason::Missed)
        );
    }

    // ========== Multi-Call Workflow Tests ==========

    #[tokio::test]
    async fn test_multiple_calls_different_users() {
        let manager = CallManager::new();
        let user1 = UserId::new(1).unwrap();
        let user2 = UserId::new(2).unwrap();

        let call1 = manager.create_call(user1, false).await.unwrap();
        let call2 = manager.create_call(user2, true).await.unwrap();

        assert_ne!(call1, call2);
        assert!(!manager.is_video_call(call1).await.unwrap());
        assert!(manager.is_video_call(call2).await.unwrap());
    }

    #[tokio::test]
    async fn test_call_lifecycle() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();

        // Create
        let call_id = manager.create_call(user_id, false).await.unwrap();
        assert!(matches!(
            manager.get_call_state(call_id).await.unwrap(),
            CallState::Pending
        ));

        // Accept
        manager.accept_call(call_id, false).await.unwrap();
        assert!(matches!(
            manager.get_call_state(call_id).await.unwrap(),
            CallState::Active
        ));

        // Send signaling
        manager
            .send_signaling_data(call_id, b"signaling".to_vec())
            .await
            .unwrap();

        // Discard
        manager.discard_call(call_id, false).await.unwrap();
        assert!(matches!(
            manager.get_call_state(call_id).await.unwrap(),
            CallState::Discarded(_)
        ));

        // Rate
        manager.rate_call(call_id, 5).await.unwrap();
    }

    // ========== Edge Cases ==========

    #[tokio::test]
    async fn test_create_call_after_discard() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();

        let call1 = manager.create_call(user_id, false).await.unwrap();
        manager.discard_call(call1, false).await.unwrap();

        // Should be able to create a new call after discarding
        let call2 = manager.create_call(user_id, false).await.unwrap();
        assert_ne!(call1, call2);
    }

    #[tokio::test]
    async fn test_video_call_all_ratings() {
        let manager = CallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let call_id = manager.create_call(user_id, true).await.unwrap();

        manager.discard_call(call_id, false).await.unwrap();

        for rating in 1..=5 {
            assert!(manager.rate_call(call_id, rating).await.is_ok());
        }
    }
}
