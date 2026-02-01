// Copyright 2024 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # State Manager
//!
//! Manages connection state for the Telegram client.
//!
//! ## Overview
//!
//! The StateManager tracks the connection state and notifies callbacks of changes.
//! It implements debouncing for state transitions to avoid frequent updates.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_state_manager::{StateManager, Callback};
//! use rustgram_connectionstate::ConnectionState;
//! use rustgram_net::NetType;
//!
//! struct MyCallback;
//!
//! impl Callback for MyCallback {
//!     fn on_state(&mut self, state: ConnectionState) -> bool {
//!         println!("State changed to: {:?}", state);
//!         true
//!     }
//!
//!     fn on_network(&mut self, network_type: NetType, generation: u32) -> bool {
//!         println!("Network: {:?}, generation: {}", network_type, generation);
//!         true
//!     }
//!
//!     fn on_online(&mut self, is_online: bool) -> bool {
//!         println!("Online: {}", is_online);
//!         true
//!     }
//! }
//!
//! let mut manager = StateManager::new();
//! manager.add_callback(Box::new(MyCallback));
//! manager.set_online(true);
//! manager.set_network(NetType::WiFi);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::time::{Duration, Instant};

use rustgram_connectionstate::ConnectionState;
use rustgram_net::NetType;

/// Up delay for state transitions (seconds).
const UP_DELAY: Duration = Duration::from_millis(50);

/// Down delay for state transitions (seconds).
const DOWN_DELAY: Duration = Duration::from_millis(300);

/// Trait for state change callbacks.
///
/// Callbacks are notified of state, network, and online status changes.
/// Returning `false` from a callback method will cause the callback to be removed.
pub trait Callback: Send + Sync {
    /// Called when the connection state changes.
    ///
    /// Returns `false` to remove this callback.
    fn on_state(&mut self, state: ConnectionState) -> bool {
        let _ = state;
        true
    }

    /// Called when the network type changes.
    ///
    /// # Arguments
    ///
    /// * `network_type` - The new network type
    /// * `generation` - The network generation counter
    ///
    /// Returns `false` to remove this callback.
    fn on_network(&mut self, network_type: NetType, generation: u32) -> bool {
        let _ = (network_type, generation);
        true
    }

    /// Called when the online status changes.
    ///
    /// Returns `false` to remove this callback.
    fn on_online(&mut self, is_online: bool) -> bool {
        let _ = is_online;
        true
    }

    /// Called when the logging out status changes.
    ///
    /// Returns `false` to remove this callback.
    fn on_logging_out(&mut self, is_logging_out: bool) -> bool {
        let _ = is_logging_out;
        true
    }
}

/// Internal flags for state change notifications.
#[derive(Debug, Clone, Copy)]
enum Flag {
    Online,
    State,
    Network,
    LoggingOut,
}

/// State manager for tracking connection state.
///
/// The StateManager coordinates state changes and notifies callbacks
/// with debouncing to avoid excessive updates.
///
/// # Examples
///
/// ```
/// use rustgram_state_manager::StateManager;
/// use rustgram_connectionstate::ConnectionState;
///
/// let manager = StateManager::new();
/// assert_eq!(manager.get_state(), ConnectionState::Empty);
/// ```
pub struct StateManager {
    /// Synchronization flag.
    sync_flag: bool,
    /// Network availability flag.
    network_flag: bool,
    /// Current network type.
    network_type: NetType,
    /// Network generation counter (increments on network changes).
    network_generation: u32,
    /// Online flag.
    online_flag: bool,
    /// Using proxy flag.
    use_proxy: bool,
    /// Logging out flag.
    is_logging_out: bool,

    /// Connection count (from ConnectionManager).
    connect_count: u32,
    /// Proxy connection count (from ConnectionManager).
    connect_proxy_count: u32,

    /// Pending state (before debounce delay).
    pending_state: ConnectionState,
    /// Pending state timestamp.
    pending_timestamp: Option<Instant>,
    /// Flushed state (after debounce delay).
    flush_state: ConnectionState,

    /// Registered callbacks.
    callbacks: Vec<Box<dyn Callback>>,
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StateManager {
    /// Creates a new StateManager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_state_manager::StateManager;
    ///
    /// let manager = StateManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            sync_flag: false,
            network_flag: false,
            network_type: NetType::Unknown,
            network_generation: 1,
            online_flag: false,
            use_proxy: false,
            is_logging_out: false,
            connect_count: 0,
            connect_proxy_count: 0,
            pending_state: ConnectionState::Empty,
            pending_timestamp: None,
            flush_state: ConnectionState::Empty,
            callbacks: Vec::new(),
        }
    }

    /// Sets the synchronization flag.
    ///
    /// Called when the client completes or loses synchronization with the server.
    ///
    /// # Arguments
    ///
    /// * `is_synchronized` - Whether the client is synchronized
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_state_manager::StateManager;
    ///
    /// let mut manager = StateManager::new();
    /// manager.set_synchronized(true);
    /// ```
    pub fn set_synchronized(&mut self, is_synchronized: bool) {
        self.sync_flag = is_synchronized;
        self.loop_state();
    }

    /// Called when network is updated.
    ///
    /// Re-evaluates the network state with generation increment.
    pub fn on_network_updated(&mut self) {
        self.do_on_network(self.network_type, true);
    }

    /// Sets the network type.
    ///
    /// # Arguments
    ///
    /// * `network_type` - The new network type
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_state_manager::StateManager;
    /// use rustgram_net::NetType;
    ///
    /// let mut manager = StateManager::new();
    /// manager.set_network(NetType::WiFi);
    /// ```
    pub fn set_network(&mut self, network_type: NetType) {
        self.do_on_network(network_type, true);
    }

    /// Sets the online status.
    ///
    /// # Arguments
    ///
    /// * `is_online` - Whether the client is online
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_state_manager::StateManager;
    ///
    /// let mut manager = StateManager::new();
    /// manager.set_online(true);
    /// ```
    pub fn set_online(&mut self, is_online: bool) {
        self.online_flag = is_online;
        self.notify_flag(Flag::Online);
    }

    /// Sets the proxy usage flag.
    ///
    /// # Arguments
    ///
    /// * `use_proxy` - Whether a proxy is being used
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_state_manager::StateManager;
    ///
    /// let mut manager = StateManager::new();
    /// manager.set_proxy(true);
    /// ```
    pub fn set_proxy(&mut self, use_proxy: bool) {
        self.use_proxy = use_proxy;
        self.set_network(self.network_type);
        self.loop_state();
    }

    /// Sets the logging out flag.
    ///
    /// # Arguments
    ///
    /// * `is_logging_out` - Whether the user is logging out
    pub fn set_logging_out(&mut self, is_logging_out: bool) {
        self.is_logging_out = is_logging_out;
        self.notify_flag(Flag::LoggingOut);
    }

    /// Adds a callback for state change notifications.
    ///
    /// The callback will be notified immediately with current state values.
    /// If the callback returns false during initial notification, it won't be added.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback to add
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_state_manager::{StateManager, Callback};
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// struct MyCallback;
    ///
    /// impl Callback for MyCallback {}
    ///
    /// let mut manager = StateManager::new();
    /// manager.add_callback(Box::new(MyCallback));
    /// ```
    pub fn add_callback(&mut self, mut callback: Box<dyn Callback>) {
        let state = self.get_real_state();
        if callback.on_network(self.network_type, self.network_generation)
            && callback.on_online(self.online_flag)
            && callback.on_state(state)
            && callback.on_logging_out(self.is_logging_out)
        {
            self.callbacks.push(callback);
        }
    }

    /// Gets the current connection state.
    ///
    /// # Returns
    ///
    /// The current connection state
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_state_manager::StateManager;
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// let manager = StateManager::new();
    /// assert_eq!(manager.get_state(), ConnectionState::Empty);
    /// ```
    pub fn get_state(&self) -> ConnectionState {
        self.flush_state
    }

    /// Gets the real (non-debounced) connection state.
    ///
    /// # Returns
    ///
    /// The actual current connection state
    pub fn get_real_state(&self) -> ConnectionState {
        if !self.network_flag {
            return ConnectionState::WaitingForNetwork;
        }
        if self.connect_count == 0 {
            if self.use_proxy && self.connect_proxy_count == 0 {
                return ConnectionState::ConnectingToProxy;
            }
            return ConnectionState::Connecting;
        }
        if !self.sync_flag {
            return ConnectionState::Updating;
        }
        ConnectionState::Ready
    }

    /// Gets the current network type.
    ///
    /// # Returns
    ///
    /// The current network type
    pub fn get_network_type(&self) -> NetType {
        self.network_type
    }

    /// Gets the network generation counter.
    ///
    /// # Returns
    ///
    /// The current network generation
    pub fn get_network_generation(&self) -> u32 {
        self.network_generation
    }

    /// Gets the online status.
    ///
    /// # Returns
    ///
    /// Whether the client is online
    pub fn is_online(&self) -> bool {
        self.online_flag
    }

    /// Gets the logging out status.
    ///
    /// # Returns
    ///
    /// Whether the user is logging out
    pub fn is_logging_out(&self) -> bool {
        self.is_logging_out
    }

    /// Sets the connection count (from ConnectionManager).
    ///
    /// # Arguments
    ///
    /// * `count` - The connection count
    pub fn set_connect_count(&mut self, count: u32) {
        self.connect_count = count;
        self.loop_state();
    }

    /// Sets the proxy connection count (from ConnectionManager).
    ///
    /// # Arguments
    ///
    /// * `count` - The proxy connection count
    pub fn set_connect_proxy_count(&mut self, count: u32) {
        self.connect_proxy_count = count;
        self.loop_state();
    }

    /// Processes any pending state changes.
    ///
    /// Should be called periodically to flush debounced state changes.
    ///
    /// # Returns
    ///
    /// The duration until the next state change should be processed, if any
    pub fn tick(&mut self) -> Option<Duration> {
        let now = Instant::now();
        let state = self.get_real_state();

        if state != self.pending_state {
            self.pending_state = state;
            if self.pending_timestamp.is_none() {
                self.pending_timestamp = Some(now);
            }
        }

        if self.pending_state != self.flush_state {
            let delay = if self.flush_state != ConnectionState::Empty {
                let pending_ord = self.pending_state as i32;
                let flush_ord = self.flush_state as i32;
                if pending_ord > flush_ord {
                    UP_DELAY
                } else {
                    DOWN_DELAY
                }
            } else {
                Duration::ZERO
            };

            if self.network_type == NetType::Unknown {
                // No delay for unknown network
                self.flush_state();
            } else if let Some(timestamp) = self.pending_timestamp {
                if now >= timestamp + delay {
                    self.flush_state();
                } else {
                    return Some((timestamp + delay) - now);
                }
            }
        } else {
            self.pending_timestamp = None;
        }

        None
    }

    /// Internal method to handle network changes.
    fn do_on_network(&mut self, new_network_type: NetType, inc_generation: bool) {
        let new_network_flag = new_network_type != NetType::None;
        if self.network_flag != new_network_flag {
            self.network_flag = new_network_flag;
            self.loop_state();
        }
        self.network_type = new_network_type;
        if inc_generation {
            self.network_generation += 1;
        }
        self.notify_flag(Flag::Network);
    }

    /// Main state evaluation loop.
    fn loop_state(&mut self) {
        self.tick();
    }

    /// Flushes the pending state to callbacks.
    fn flush_state(&mut self) {
        self.pending_timestamp = None;
        self.flush_state = self.pending_state;
        self.notify_flag(Flag::State);
    }

    /// Notifies callbacks of a flag change.
    fn notify_flag(&mut self, flag: Flag) {
        let mut i = 0;
        while i < self.callbacks.len() {
            let keep = match flag {
                Flag::Online => self.callbacks[i].on_online(self.online_flag),
                Flag::State => self.callbacks[i].on_state(self.flush_state),
                Flag::Network => {
                    self.callbacks[i].on_network(self.network_type, self.network_generation)
                }
                Flag::LoggingOut => self.callbacks[i].on_logging_out(self.is_logging_out),
            };
            if keep {
                i += 1;
            } else {
                self.callbacks.remove(i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct TestCallback {
        states: Arc<Mutex<Vec<ConnectionState>>>,
        online_calls: Arc<Mutex<Vec<bool>>>,
        network_calls: Arc<Mutex<Vec<(NetType, u32)>>>,
        should_fail: bool,
    }

    impl TestCallback {
        fn new(should_fail: bool) -> Self {
            Self {
                states: Arc::new(Mutex::new(Vec::new())),
                online_calls: Arc::new(Mutex::new(Vec::new())),
                network_calls: Arc::new(Mutex::new(Vec::new())),
                should_fail,
            }
        }
    }

    impl Callback for TestCallback {
        fn on_state(&mut self, state: ConnectionState) -> bool {
            self.states.lock().unwrap().push(state);
            !self.should_fail
        }

        fn on_network(&mut self, network_type: NetType, generation: u32) -> bool {
            self.network_calls
                .lock()
                .unwrap()
                .push((network_type, generation));
            !self.should_fail
        }

        fn on_online(&mut self, is_online: bool) -> bool {
            self.online_calls.lock().unwrap().push(is_online);
            !self.should_fail
        }
    }

    #[test]
    fn test_default() {
        let manager = StateManager::default();
        assert!(!manager.sync_flag);
        assert!(!manager.network_flag);
        assert_eq!(manager.network_type, NetType::Unknown);
        assert!(!manager.online_flag);
    }

    #[test]
    fn test_new() {
        let manager = StateManager::new();
        assert_eq!(manager.get_state(), ConnectionState::Empty);
        assert_eq!(manager.get_network_type(), NetType::Unknown);
        assert!(!manager.is_online());
        assert!(!manager.is_logging_out());
    }

    #[test]
    fn test_initial_state() {
        let manager = StateManager::new();
        assert_eq!(manager.get_state(), ConnectionState::Empty);
    }

    #[test]
    fn test_waiting_for_network() {
        let mut manager = StateManager::new();
        manager.tick();
        // After tick, should be WaitingForNetwork since network_flag is false
        assert_eq!(manager.get_real_state(), ConnectionState::WaitingForNetwork);
    }

    #[test]
    fn test_set_network() {
        let mut manager = StateManager::new();
        manager.set_network(NetType::WiFi);
        assert_eq!(manager.get_network_type(), NetType::WiFi);
        assert_eq!(manager.get_network_generation(), 2); // Starts at 1, increments to 2
    }

    #[test]
    fn test_set_online() {
        let mut manager = StateManager::new();
        manager.set_online(true);
        assert!(manager.is_online());
    }

    #[test]
    fn test_set_proxy() {
        let mut manager = StateManager::new();
        manager.set_proxy(true);
        // Proxy should affect the real state
        assert_eq!(manager.get_real_state(), ConnectionState::ConnectingToProxy);
    }

    #[test]
    fn test_set_synchronized() {
        let mut manager = StateManager::new();
        manager.set_synchronized(true);
        assert!(manager.sync_flag);
    }

    #[test]
    fn test_set_logging_out() {
        let mut manager = StateManager::new();
        manager.set_logging_out(true);
        assert!(manager.is_logging_out());
    }

    #[test]
    fn test_set_connect_count() {
        let mut manager = StateManager::new();
        manager.set_synchronized(true);
        manager.set_network(NetType::Other);
        manager.set_connect_count(1);
        // With network, sync, and connection, should be Ready
        assert_eq!(manager.get_real_state(), ConnectionState::Ready);
    }

    #[test]
    fn test_add_callback() {
        let mut manager = StateManager::new();
        manager.add_callback(Box::new(TestCallback::new(false)));
        assert_eq!(manager.callbacks.len(), 1);
    }

    #[test]
    fn test_callback_rejected() {
        let mut manager = StateManager::new();
        manager.add_callback(Box::new(TestCallback::new(true)));
        // Callback that returns false shouldn't be added
        assert_eq!(manager.callbacks.len(), 0);
    }

    #[test]
    fn test_online_callback() {
        let mut manager = StateManager::new();
        let callback = TestCallback::new(false);
        let online_calls = Arc::clone(&callback.online_calls);
        manager.add_callback(Box::new(callback));

        manager.set_online(true);
        // Initial call + set_online call
        assert_eq!(online_calls.lock().unwrap().len(), 2);
    }

    #[test]
    fn test_network_callback() {
        let mut manager = StateManager::new();
        let callback = TestCallback::new(false);
        let network_calls = Arc::clone(&callback.network_calls);
        manager.add_callback(Box::new(callback));

        manager.set_network(NetType::WiFi);
        // Initial call + set_network call
        assert_eq!(network_calls.lock().unwrap().len(), 2);
    }

    #[test]
    fn test_state_callback() {
        let mut manager = StateManager::new();
        let callback = TestCallback::new(false);
        let states = Arc::clone(&callback.states);
        manager.add_callback(Box::new(callback));

        manager.set_synchronized(true);
        manager.set_network(NetType::Other);
        manager.set_connect_count(1);

        // Force a flush
        let _ = manager.tick();

        // Should have at least initial state
        assert!(!states.lock().unwrap().is_empty());
    }

    #[test]
    fn test_network_generation() {
        let mut manager = StateManager::new();
        assert_eq!(manager.get_network_generation(), 1);

        manager.set_network(NetType::WiFi);
        assert_eq!(manager.get_network_generation(), 2);

        manager.set_network(NetType::Mobile);
        assert_eq!(manager.get_network_generation(), 3);
    }

    #[test]
    fn test_state_progression() {
        let mut manager = StateManager::new();

        // Initial state
        assert_eq!(manager.get_real_state(), ConnectionState::WaitingForNetwork);

        // With network but no connection
        manager.set_network(NetType::Other);
        assert_eq!(manager.get_real_state(), ConnectionState::Connecting);

        // With connection but no sync
        manager.set_connect_count(1);
        assert_eq!(manager.get_real_state(), ConnectionState::Updating);

        // With sync
        manager.set_synchronized(true);
        assert_eq!(manager.get_real_state(), ConnectionState::Ready);
    }

    #[test]
    fn test_proxy_state() {
        let mut manager = StateManager::new();
        manager.set_proxy(true);
        manager.set_network(NetType::Other);

        // With proxy but no connection
        assert_eq!(manager.get_real_state(), ConnectionState::ConnectingToProxy);

        // With proxy connection
        manager.set_connect_proxy_count(1);
        assert_eq!(manager.get_real_state(), ConnectionState::Connecting);
    }

    #[test]
    fn test_state_ordering() {
        // ConnectionState enum values should be ordered
        assert!(ConnectionState::Empty < ConnectionState::Ready);
        assert!(ConnectionState::WaitingForNetwork < ConnectionState::Connecting);
        assert!(ConnectionState::Updating < ConnectionState::Ready);
    }

    #[test]
    fn test_callback_count() {
        let mut manager = StateManager::new();
        assert_eq!(manager.callbacks.len(), 0);

        manager.add_callback(Box::new(TestCallback::new(false)));
        assert_eq!(manager.callbacks.len(), 1);
    }

    #[test]
    fn test_callback_removal_on_failure() {
        let mut manager = StateManager::new();
        manager.add_callback(Box::new(TestCallback::new(true))); // This callback fails immediately

        // Callback that returns false during initial notification shouldn't be added
        assert_eq!(manager.callbacks.len(), 0);
    }

    #[test]
    fn test_updating_to_ready_transition() {
        let mut manager = StateManager::new();
        manager.set_network(NetType::Other);
        manager.set_connect_count(1);

        // Should be Updating without sync
        assert_eq!(manager.get_real_state(), ConnectionState::Updating);

        // With sync, should be Ready
        manager.set_synchronized(true);
        assert_eq!(manager.get_real_state(), ConnectionState::Ready);
    }

    #[test]
    fn test_debounce_flush_immediate() {
        let mut manager = StateManager::new();
        manager.set_network(NetType::Unknown); // Unknown network has no delay
        manager.set_connect_count(1);

        // Should flush immediately for Unknown network
        let delay = manager.tick();
        assert!(delay.is_none() || delay.unwrap() == Duration::ZERO);
    }

    #[test]
    fn test_net_type_variants() {
        let manager = StateManager::new();
        assert_eq!(manager.get_network_type(), NetType::Unknown);

        let mut manager = StateManager::new();
        manager.set_network(NetType::None);
        assert_eq!(manager.get_network_type(), NetType::None);

        manager.set_network(NetType::WiFi);
        assert_eq!(manager.get_network_type(), NetType::WiFi);

        manager.set_network(NetType::Mobile);
        assert_eq!(manager.get_network_type(), NetType::Mobile);

        manager.set_network(NetType::Other);
        assert_eq!(manager.get_network_type(), NetType::Other);
    }
}
