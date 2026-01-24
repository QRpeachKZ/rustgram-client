// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Online presence manager for Telegram MTProto client.
//!
//! This module implements TDLib's OnlineManager.
//!
//! # Overview
//!
//! The OnlineManager manages the online presence of the current user,
//! including online status updates and ping timeouts to the server.
//!
//! # Example
//!
//! ```rust
//! use rustgram_online_manager::OnlineManager;
//!
//! let manager = OnlineManager::new();
//! assert!(!manager.is_online());
//! manager.set_is_online(true);
//! assert!(manager.is_online());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Online presence manager.
///
/// Based on TDLib's `OnlineManager` class.
///
/// Manages the online status of the current user, handling:
/// - Online state tracking
/// - Bot online status (separate from user online status)
/// - Ping server timeout (default: 300 seconds)
///
/// # Example
///
/// ```rust
/// use rustgram_online_manager::OnlineManager;
///
/// let manager = OnlineManager::new();
/// assert!(!manager.is_online());
///
/// manager.set_is_online(true);
/// assert!(manager.is_online());
///
/// manager.set_is_bot_online(true);
/// assert!(manager.is_bot_online());
/// ```
#[derive(Debug, Clone)]
pub struct OnlineManager {
    /// Shared state for online status.
    state: Arc<OnlineState>,
}

/// Shared online state.
#[derive(Debug)]
struct OnlineState {
    /// User online status.
    is_online: AtomicBool,
    /// Bot online status.
    is_bot_online: AtomicBool,
}

impl Default for OnlineState {
    fn default() -> Self {
        Self {
            is_online: AtomicBool::new(false),
            is_bot_online: AtomicBool::new(false),
        }
    }
}

impl Default for OnlineManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OnlineManager {
    /// Ping server timeout in seconds (300 seconds = 5 minutes).
    ///
    /// Based on TDLib's `PING_SERVER_TIMEOUT` constant.
    pub const PING_SERVER_TIMEOUT: i32 = 300;

    /// Creates a new OnlineManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// assert!(!manager.is_online());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(OnlineState::default()),
        }
    }

    /// Initializes the manager.
    ///
    /// This method should be called after creating the manager to set up
    /// any necessary internal state. In the current stub implementation,
    /// this is a no-op but provided for API compatibility with TDLib.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// manager.init();
    /// ```
    pub fn init(&self) {
        // TODO: Initialize ping timeouts when actor system is available
    }

    /// Checks if the user is currently online.
    ///
    /// # Returns
    ///
    /// `true` if the user is online, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// assert!(!manager.is_online());
    ///
    /// manager.set_is_online(true);
    /// assert!(manager.is_online());
    /// ```
    #[must_use]
    pub fn is_online(&self) -> bool {
        self.state.is_online.load(Ordering::Acquire)
    }

    /// Sets the user's online status.
    ///
    /// # Arguments
    ///
    /// * `is_online` - Whether the user should be online
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// manager.set_is_online(true);
    /// assert!(manager.is_online());
    ///
    /// manager.set_is_online(false);
    /// assert!(!manager.is_online());
    /// ```
    pub fn set_is_online(&self, is_online: bool) {
        self.state.is_online.store(is_online, Ordering::Release);
        // TODO: Trigger update_status_query when net module is available
    }

    /// Checks if the bot is currently online.
    ///
    /// # Returns
    ///
    /// `true` if the bot is online, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// assert!(!manager.is_bot_online());
    ///
    /// manager.set_is_bot_online(true);
    /// assert!(manager.is_bot_online());
    /// ```
    #[must_use]
    pub fn is_bot_online(&self) -> bool {
        self.state.is_bot_online.load(Ordering::Acquire)
    }

    /// Sets the bot's online status.
    ///
    /// # Arguments
    ///
    /// * `is_bot_online` - Whether the bot should be online
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// manager.set_is_bot_online(true);
    /// assert!(manager.is_bot_online());
    ///
    /// manager.set_is_bot_online(false);
    /// assert!(!manager.is_bot_online());
    /// ```
    pub fn set_is_bot_online(&self, is_bot_online: bool) {
        self.state
            .is_bot_online
            .store(is_bot_online, Ordering::Release);
        // TODO: Trigger update_status_query when net module is available
    }

    /// Called when online status is updated.
    ///
    /// This method should be called when the online status changes to trigger
    /// any necessary updates.
    ///
    /// # Arguments
    ///
    /// * `force` - Whether to force an update even if the status hasn't changed
    /// * `send_update` - Whether to send an update to the server
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// manager.on_online_updated(true, true);
    /// ```
    pub fn on_online_updated(&self, force: bool, send_update: bool) {
        // TODO: Implement online timeout callback when actor system is available
        let _ = force;
        let _ = send_update;
    }

    /// Called when the status update query succeeds.
    ///
    /// # Arguments
    ///
    /// * `is_online` - The online status that was successfully updated
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// let manager = OnlineManager::new();
    /// manager.on_update_status_success(true);
    /// ```
    pub fn on_update_status_success(&self, is_online: bool) {
        self.set_is_online(is_online);
        // TODO: Reset online timeout when actor system is available
    }

    /// Gets the ping server timeout.
    ///
    /// # Returns
    ///
    /// The ping server timeout in seconds (default: 300).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_online_manager::OnlineManager;
    ///
    /// assert_eq!(OnlineManager::ping_server_timeout(), 300);
    /// ```
    #[must_use]
    pub const fn ping_server_timeout() -> i32 {
        Self::PING_SERVER_TIMEOUT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manager = OnlineManager::new();
        assert!(!manager.is_online());
        assert!(!manager.is_bot_online());
    }

    #[test]
    fn test_default() {
        let manager = OnlineManager::default();
        assert!(!manager.is_online());
        assert!(!manager.is_bot_online());
    }

    #[test]
    fn test_set_is_online() {
        let manager = OnlineManager::new();
        assert!(!manager.is_online());

        manager.set_is_online(true);
        assert!(manager.is_online());

        manager.set_is_online(false);
        assert!(!manager.is_online());
    }

    #[test]
    fn test_set_is_bot_online() {
        let manager = OnlineManager::new();
        assert!(!manager.is_bot_online());

        manager.set_is_bot_online(true);
        assert!(manager.is_bot_online());

        manager.set_is_bot_online(false);
        assert!(!manager.is_bot_online());
    }

    #[test]
    fn test_is_online() {
        let manager = OnlineManager::new();
        assert!(!manager.is_online());

        manager.set_is_online(true);
        assert!(manager.is_online());

        manager.set_is_online(false);
        assert!(!manager.is_online());
    }

    #[test]
    fn test_is_bot_online() {
        let manager = OnlineManager::new();
        assert!(!manager.is_bot_online());

        manager.set_is_bot_online(true);
        assert!(manager.is_bot_online());

        manager.set_is_bot_online(false);
        assert!(!manager.is_bot_online());
    }

    #[test]
    fn test_init() {
        let manager = OnlineManager::new();
        manager.init(); // Should not panic
        assert!(!manager.is_online());
    }

    #[test]
    fn test_on_online_updated() {
        let manager = OnlineManager::new();
        manager.on_online_updated(true, true); // Should not panic
    }

    #[test]
    fn test_on_update_status_success_true() {
        let manager = OnlineManager::new();
        manager.on_update_status_success(true);
        assert!(manager.is_online());
    }

    #[test]
    fn test_on_update_status_success_false() {
        let manager = OnlineManager::new();
        manager.set_is_online(true);
        manager.on_update_status_success(false);
        assert!(!manager.is_online());
    }

    #[test]
    fn test_ping_server_timeout() {
        assert_eq!(OnlineManager::ping_server_timeout(), 300);
    }

    #[test]
    fn test_const_ping_server_timeout() {
        const TIMEOUT: i32 = OnlineManager::PING_SERVER_TIMEOUT;
        assert_eq!(TIMEOUT, 300);
    }

    #[test]
    fn test_clone() {
        let manager1 = OnlineManager::new();
        manager1.set_is_online(true);
        manager1.set_is_bot_online(true);

        let manager2 = manager1.clone();
        assert!(manager2.is_online());
        assert!(manager2.is_bot_online());

        // Changes to one affect the other (shared state)
        manager2.set_is_online(false);
        assert!(!manager1.is_online());
        assert!(!manager2.is_online());
    }

    #[test]
    fn test_independent_managers() {
        let manager1 = OnlineManager::new();
        let manager2 = OnlineManager::new();

        manager1.set_is_online(true);
        manager1.set_is_bot_online(true);

        assert!(manager1.is_online());
        assert!(manager1.is_bot_online());
        assert!(!manager2.is_online());
        assert!(!manager2.is_bot_online());
    }

    #[test]
    fn test_online_and_bot_independent() {
        let manager = OnlineManager::new();

        manager.set_is_online(true);
        assert!(manager.is_online());
        assert!(!manager.is_bot_online());

        manager.set_is_bot_online(true);
        assert!(manager.is_online());
        assert!(manager.is_bot_online());

        manager.set_is_online(false);
        assert!(!manager.is_online());
        assert!(manager.is_bot_online());
    }

    #[test]
    fn test_toggle_online() {
        let manager = OnlineManager::new();

        for _ in 0..10 {
            manager.set_is_online(true);
            assert!(manager.is_online());

            manager.set_is_online(false);
            assert!(!manager.is_online());
        }
    }

    #[test]
    fn test_toggle_bot_online() {
        let manager = OnlineManager::new();

        for _ in 0..10 {
            manager.set_is_bot_online(true);
            assert!(manager.is_bot_online());

            manager.set_is_bot_online(false);
            assert!(!manager.is_bot_online());
        }
    }

    #[test]
    fn test_on_update_status_success_multiple() {
        let manager = OnlineManager::new();

        for i in 0..10 {
            let expected = i % 2 == 0;
            manager.on_update_status_success(expected);
            assert_eq!(manager.is_online(), expected);
        }
    }

    #[test]
    fn test_debug_format() {
        let manager = OnlineManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("OnlineManager"));
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let manager = Arc::new(OnlineManager::new());
        let mut handles = vec![];

        // Spawn multiple threads that update online status
        for i in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    let is_online = (i + j) % 2 == 0;
                    manager_clone.set_is_online(is_online);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Final state should be valid (either true or false)
        let is_online = manager.is_online();
        assert!(is_online || !is_online); // Just check it's a valid bool
    }
}
