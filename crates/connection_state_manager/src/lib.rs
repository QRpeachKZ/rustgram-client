//! # Rustgram ConnectionStateManager
//!
//! Connection state management for Telegram MTProto client.
//!
//! This crate provides a manager for tracking and managing connection state
//! transitions. It wraps the [`ConnectionState`] enum from the `connectionstate`
//! crate and provides convenient methods for state management.
//!
//! ## Overview
//!
//! - [`ConnectionStateManager`] - Manages connection state transitions
//!
//! ## Connection States
//!
//! The connection state progresses through these stages:
//!
//! - `Empty` - Initial state (not yet connected)
//! - `WaitingForNetwork` - No network connectivity
//! - `ConnectingToProxy` - Connecting through proxy
//! - `Connecting` - Connecting to Telegram servers
//! - `Updating` - Syncing data with servers
//! - `Ready` - Connection ready and operational
//!
//! ## Examples
//!
//! Basic state management:
//!
//! ```
//! use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
//!
//! let mut manager = ConnectionStateManager::new();
//! assert_eq!(manager.current_state(), ConnectionState::Empty);
//! assert!(!manager.is_ready());
//!
//! manager.set_state(ConnectionState::Ready).unwrap();
//! assert!(manager.is_ready());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;

// Re-export ConnectionState and StateError for convenience
pub use rustgram_connectionstate::{ConnectionState, StateError};

/// Connection state manager.
///
/// Manages the current connection state and provides convenience methods
/// for checking state conditions.
///
/// # Examples
///
/// ```
/// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
///
/// let manager = ConnectionStateManager::new();
/// assert_eq!(manager.current_state(), ConnectionState::Empty);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConnectionStateManager {
    /// Current connection state
    state: ConnectionState,
}

impl Default for ConnectionStateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionStateManager {
    /// Creates a new state manager with Empty state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let manager = ConnectionStateManager::new();
    /// assert_eq!(manager.current_state(), ConnectionState::Empty);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Empty,
        }
    }

    /// Creates a state manager with the specified initial state.
    ///
    /// # Arguments
    ///
    /// * `state` - The initial connection state
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
    /// assert!(manager.is_ready());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_state(state: ConnectionState) -> Self {
        Self { state }
    }

    /// Returns the current connection state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let manager = ConnectionStateManager::new();
    /// assert_eq!(manager.current_state(), ConnectionState::Empty);
    /// ```
    #[inline]
    #[must_use]
    pub const fn current_state(&self) -> ConnectionState {
        self.state
    }

    /// Sets a new connection state.
    ///
    /// This method allows any state transition, including regression
    /// (e.g., Ready -> Connecting when connection is lost).
    ///
    /// # Arguments
    ///
    /// * `state` - The new connection state
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the state was different and was changed
    /// `Ok(false)` if the state was the same and no change occurred
    ///
    /// # Errors
    ///
    /// This function currently does not return errors but may in the future
    /// if validation is added.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// assert!(manager.set_state(ConnectionState::Connecting).unwrap());
    /// assert!(!manager.set_state(ConnectionState::Connecting).unwrap());
    /// ```
    #[inline]
    pub fn set_state(&mut self, state: ConnectionState) -> Result<bool, StateError> {
        if self.state == state {
            return Ok(false);
        }
        self.state = state;
        Ok(true)
    }

    /// Checks if the current state is Ready.
    ///
    /// # Returns
    ///
    /// `true` if the connection is ready and operational
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// assert!(!manager.is_ready());
    ///
    /// manager.set_state(ConnectionState::Ready).unwrap();
    /// assert!(manager.is_ready());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.state == ConnectionState::Ready
    }

    /// Checks if the current state is connecting (any connecting state).
    ///
    /// This includes `ConnectingToProxy`, `Connecting`, and `Updating`.
    ///
    /// # Returns
    ///
    /// `true` if currently connecting to servers
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// manager.set_state(ConnectionState::Connecting).unwrap();
    /// assert!(manager.is_connecting());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_connecting(&self) -> bool {
        matches!(
            self.state,
            ConnectionState::ConnectingToProxy
                | ConnectionState::Connecting
                | ConnectionState::Updating
        )
    }

    /// Checks if currently waiting for network.
    ///
    /// # Returns
    ///
    /// `true` if waiting for network connectivity
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// manager.set_state(ConnectionState::WaitingForNetwork).unwrap();
    /// assert!(manager.is_waiting_for_network());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_waiting_for_network(&self) -> bool {
        self.state == ConnectionState::WaitingForNetwork
    }

    /// Checks if there's an active connection (connected to servers).
    ///
    /// # Returns
    ///
    /// `true` if connected (Updating or Ready)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// manager.set_state(ConnectionState::Updating).unwrap();
    /// assert!(manager.is_connected());
    ///
    /// manager.set_state(ConnectionState::Ready).unwrap();
    /// assert!(manager.is_connected());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_connected(&self) -> bool {
        matches!(
            self.state,
            ConnectionState::Updating | ConnectionState::Ready
        )
    }

    /// Resets the state manager back to Empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// manager.set_state(ConnectionState::Ready).unwrap();
    /// assert!(manager.is_ready());
    ///
    /// manager.reset();
    /// assert_eq!(manager.current_state(), ConnectionState::Empty);
    /// ```
    pub fn reset(&mut self) {
        self.state = ConnectionState::Empty;
    }

    /// Gets a description of the current state.
    ///
    /// # Returns
    ///
    /// A human-readable description of the current state
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state_manager::{ConnectionStateManager, ConnectionState};
    ///
    /// let manager = ConnectionStateManager::new();
    /// assert_eq!(manager.state_description(), "Empty");
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// manager.set_state(ConnectionState::Ready).unwrap();
    /// assert_eq!(manager.state_description(), "Ready");
    /// ```
    #[must_use]
    pub fn state_description(&self) -> &str {
        match self.state {
            ConnectionState::Empty => "Empty",
            ConnectionState::WaitingForNetwork => "WaitingForNetwork",
            ConnectionState::ConnectingToProxy => "ConnectingToProxy",
            ConnectionState::Connecting => "Connecting",
            ConnectionState::Updating => "Updating",
            ConnectionState::Ready => "Ready",
        }
    }
}

impl fmt::Display for ConnectionStateManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConnectionStateManager({})", self.state_description())
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-connection-state-manager";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_new_creates_empty_state() {
        let manager = ConnectionStateManager::new();
        assert_eq!(manager.current_state(), ConnectionState::Empty);
    }

    #[test]
    fn test_default_creates_empty_state() {
        let manager = ConnectionStateManager::default();
        assert_eq!(manager.current_state(), ConnectionState::Empty);
    }

    #[test]
    fn test_with_state_sets_initial_state() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        assert_eq!(manager.current_state(), ConnectionState::Ready);
    }

    #[test]
    fn test_with_state_all_states() {
        assert_eq!(
            ConnectionStateManager::with_state(ConnectionState::Empty).current_state(),
            ConnectionState::Empty
        );
        assert_eq!(
            ConnectionStateManager::with_state(ConnectionState::WaitingForNetwork).current_state(),
            ConnectionState::WaitingForNetwork
        );
        assert_eq!(
            ConnectionStateManager::with_state(ConnectionState::ConnectingToProxy).current_state(),
            ConnectionState::ConnectingToProxy
        );
        assert_eq!(
            ConnectionStateManager::with_state(ConnectionState::Connecting).current_state(),
            ConnectionState::Connecting
        );
        assert_eq!(
            ConnectionStateManager::with_state(ConnectionState::Updating).current_state(),
            ConnectionState::Updating
        );
        assert_eq!(
            ConnectionStateManager::with_state(ConnectionState::Ready).current_state(),
            ConnectionState::Ready
        );
    }

    // ========== current_state Tests ==========

    #[test]
    fn test_current_state_returns_state() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Connecting);
        assert_eq!(manager.current_state(), ConnectionState::Connecting);
    }

    // ========== set_state Tests ==========

    #[test]
    fn test_set_state_changes_state() {
        let mut manager = ConnectionStateManager::new();
        assert!(manager.set_state(ConnectionState::Connecting).unwrap());
        assert_eq!(manager.current_state(), ConnectionState::Connecting);
    }

    #[test]
    fn test_set_state_returns_true_on_change() {
        let mut manager = ConnectionStateManager::new();
        assert!(manager.set_state(ConnectionState::Ready).unwrap());
    }

    #[test]
    fn test_set_state_returns_false_on_same() {
        let mut manager = ConnectionStateManager::new();
        manager.set_state(ConnectionState::Ready).unwrap();
        assert!(!manager.set_state(ConnectionState::Ready).unwrap());
    }

    #[test]
    fn test_set_state_progression() {
        let mut manager = ConnectionStateManager::new();

        assert!(manager
            .set_state(ConnectionState::WaitingForNetwork)
            .unwrap());
        assert!(manager
            .set_state(ConnectionState::ConnectingToProxy)
            .unwrap());
        assert!(manager.set_state(ConnectionState::Connecting).unwrap());
        assert!(manager.set_state(ConnectionState::Updating).unwrap());
        assert!(manager.set_state(ConnectionState::Ready).unwrap());
    }

    #[test]
    fn test_set_state_regression() {
        let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);

        // Regression is allowed (connection lost)
        assert!(manager.set_state(ConnectionState::Connecting).unwrap());
        assert!(manager
            .set_state(ConnectionState::WaitingForNetwork)
            .unwrap());
    }

    #[test]
    fn test_set_state_to_empty() {
        let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        assert!(manager.set_state(ConnectionState::Empty).unwrap());
        assert_eq!(manager.current_state(), ConnectionState::Empty);
    }

    // ========== is_ready Tests ==========

    #[test]
    fn test_is_ready_when_ready() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        assert!(manager.is_ready());
    }

    #[test]
    fn test_is_ready_when_not_ready() {
        let states = [
            ConnectionState::Empty,
            ConnectionState::WaitingForNetwork,
            ConnectionState::ConnectingToProxy,
            ConnectionState::Connecting,
            ConnectionState::Updating,
        ];

        for state in states {
            let manager = ConnectionStateManager::with_state(state);
            assert!(
                !manager.is_ready(),
                "is_ready should be false for {:?}",
                state
            );
        }
    }

    // ========== is_connecting Tests ==========

    #[test]
    fn test_is_connecting_when_connecting_to_proxy() {
        let manager = ConnectionStateManager::with_state(ConnectionState::ConnectingToProxy);
        assert!(manager.is_connecting());
    }

    #[test]
    fn test_is_connecting_when_connecting() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Connecting);
        assert!(manager.is_connecting());
    }

    #[test]
    fn test_is_connecting_when_updating() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Updating);
        assert!(manager.is_connecting());
    }

    #[test]
    fn test_is_connecting_when_not_connecting() {
        let states = [
            ConnectionState::Empty,
            ConnectionState::WaitingForNetwork,
            ConnectionState::Ready,
        ];

        for state in states {
            let manager = ConnectionStateManager::with_state(state);
            assert!(
                !manager.is_connecting(),
                "is_connecting should be false for {:?}",
                state
            );
        }
    }

    // ========== is_waiting_for_network Tests ==========

    #[test]
    fn test_is_waiting_for_network_when_waiting() {
        let manager = ConnectionStateManager::with_state(ConnectionState::WaitingForNetwork);
        assert!(manager.is_waiting_for_network());
    }

    #[test]
    fn test_is_waiting_for_network_when_not_waiting() {
        let states = [
            ConnectionState::Empty,
            ConnectionState::ConnectingToProxy,
            ConnectionState::Connecting,
            ConnectionState::Updating,
            ConnectionState::Ready,
        ];

        for state in states {
            let manager = ConnectionStateManager::with_state(state);
            assert!(
                !manager.is_waiting_for_network(),
                "should not be waiting for {:?}",
                state
            );
        }
    }

    // ========== is_connected Tests ==========

    #[test]
    fn test_is_connected_when_updating() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Updating);
        assert!(manager.is_connected());
    }

    #[test]
    fn test_is_connected_when_ready() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        assert!(manager.is_connected());
    }

    #[test]
    fn test_is_connected_when_not_connected() {
        let states = [
            ConnectionState::Empty,
            ConnectionState::WaitingForNetwork,
            ConnectionState::ConnectingToProxy,
            ConnectionState::Connecting,
        ];

        for state in states {
            let manager = ConnectionStateManager::with_state(state);
            assert!(
                !manager.is_connected(),
                "is_connected should be false for {:?}",
                state
            );
        }
    }

    // ========== reset Tests ==========

    #[test]
    fn test_reset_from_ready() {
        let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        manager.reset();
        assert_eq!(manager.current_state(), ConnectionState::Empty);
    }

    #[test]
    fn test_reset_from_connecting() {
        let mut manager = ConnectionStateManager::with_state(ConnectionState::Connecting);
        manager.reset();
        assert_eq!(manager.current_state(), ConnectionState::Empty);
    }

    #[test]
    fn test_reset_from_empty() {
        let mut manager = ConnectionStateManager::new();
        manager.reset();
        assert_eq!(manager.current_state(), ConnectionState::Empty);
    }

    // ========== state_description Tests ==========

    #[test]
    fn test_state_description_empty() {
        let manager = ConnectionStateManager::new();
        assert_eq!(manager.state_description(), "Empty");
    }

    #[test]
    fn test_state_description_waiting_for_network() {
        let manager = ConnectionStateManager::with_state(ConnectionState::WaitingForNetwork);
        assert_eq!(manager.state_description(), "WaitingForNetwork");
    }

    #[test]
    fn test_state_description_connecting_to_proxy() {
        let manager = ConnectionStateManager::with_state(ConnectionState::ConnectingToProxy);
        assert_eq!(manager.state_description(), "ConnectingToProxy");
    }

    #[test]
    fn test_state_description_connecting() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Connecting);
        assert_eq!(manager.state_description(), "Connecting");
    }

    #[test]
    fn test_state_description_updating() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Updating);
        assert_eq!(manager.state_description(), "Updating");
    }

    #[test]
    fn test_state_description_ready() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        assert_eq!(manager.state_description(), "Ready");
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_state() {
        let manager1 = ConnectionStateManager::with_state(ConnectionState::Ready);
        let manager2 = ConnectionStateManager::with_state(ConnectionState::Ready);
        assert_eq!(manager1, manager2);
    }

    #[test]
    fn test_equality_different_state() {
        let manager1 = ConnectionStateManager::with_state(ConnectionState::Ready);
        let manager2 = ConnectionStateManager::with_state(ConnectionState::Connecting);
        assert_ne!(manager1, manager2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        let cloned = manager.clone();
        assert_eq!(manager, cloned);
        assert_eq!(cloned.current_state(), ConnectionState::Ready);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        let display = format!("{}", manager);
        assert!(display.contains("ConnectionStateManager"));
        assert!(display.contains("Ready"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-connection-state-manager");
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_full_connection_lifecycle() {
        let mut manager = ConnectionStateManager::new();

        // Initial state
        assert!(!manager.is_connected());
        assert!(!manager.is_ready());

        // Start connecting
        manager
            .set_state(ConnectionState::WaitingForNetwork)
            .unwrap();
        assert!(!manager.is_connected());

        manager
            .set_state(ConnectionState::ConnectingToProxy)
            .unwrap();
        assert!(!manager.is_connected());

        manager.set_state(ConnectionState::Connecting).unwrap();
        assert!(!manager.is_connected());

        // Connected but updating
        manager.set_state(ConnectionState::Updating).unwrap();
        assert!(manager.is_connected());
        assert!(!manager.is_ready());

        // Fully ready
        manager.set_state(ConnectionState::Ready).unwrap();
        assert!(manager.is_connected());
        assert!(manager.is_ready());
    }

    #[test]
    fn test_connection_loss_recovery() {
        let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);

        // Connection lost
        manager
            .set_state(ConnectionState::WaitingForNetwork)
            .unwrap();
        assert!(!manager.is_connected());

        // Recover
        manager.set_state(ConnectionState::Connecting).unwrap();
        manager.set_state(ConnectionState::Updating).unwrap();
        manager.set_state(ConnectionState::Ready).unwrap();
        assert!(manager.is_ready());
    }

    #[test]
    fn test_state_change_detection() {
        let mut manager = ConnectionStateManager::new();

        // First change
        assert!(manager.set_state(ConnectionState::Connecting).unwrap());

        // No change (same state)
        assert!(!manager.set_state(ConnectionState::Connecting).unwrap());

        // Change again
        assert!(manager.set_state(ConnectionState::Ready).unwrap());

        // No change (same state)
        assert!(!manager.set_state(ConnectionState::Ready).unwrap());
    }
}
