//! # Rustgram ConnectionState
//!
//! Connection state management for Telegram MTProto client.
//!
//! This crate provides types and utilities for tracking and managing
//! the connection state to Telegram servers. It implements the state
//! machine from TDLib's ConnectionState with callback support.
//!
//! ## Overview
//!
//! - [`ConnectionState`] - Enum representing all possible connection states
//! - [`ConnectionStateManager`] - Manager for tracking state changes
//! - [`StateCallback`] - Trait for receiving state change notifications
//! - [`ClosureCallback`] - Closure-based callback implementation
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
//! Basic state tracking:
//!
//! ```
//! use rustgram_connectionstate::{ConnectionStateManager, ConnectionState};
//!
//! let mut manager = ConnectionStateManager::new();
//! assert_eq!(manager.current_state(), ConnectionState::Empty);
//!
//! manager.set_state(ConnectionState::Connecting).unwrap();
//! assert!(manager.current_state().is_connecting());
//!
//! manager.set_state(ConnectionState::Ready).unwrap();
//! assert!(manager.current_state().is_ready());
//! ```
//!
//! Using callbacks:
//!
//! ```
//! use rustgram_connectionstate::{
//!     ConnectionStateManager, ConnectionState, StateCallback, ClosureCallback
//! };
//!
//! // Create a callback that logs state changes
//! let callback = ClosureCallback::new(|state| {
//!     println!("State changed to: {}", state);
//!     true // Keep callback registered
//! });
//!
//! let mut manager = ConnectionStateManager::new();
//! manager.register_callback(Box::new(callback));
//!
//! // This will trigger the callback
//! manager.set_state(ConnectionState::Ready).unwrap();
//! ```
//!
//! Custom callback implementation:
//!
//! ```
//! use rustgram_connectionstate::{ConnectionState, StateCallback};
//!
//! struct MyCallback {
//!     counter: std::sync::Arc<std::sync::atomic::AtomicUsize>,
//! }
//!
//! impl StateCallback for MyCallback {
//!     fn on_state_changed(&self, state: ConnectionState) -> bool {
//!         self.counter.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
//!         println!("New state: {} (count: {})", state,
//!             self.counter.load(std::sync::atomic::Ordering::Acquire));
//!         true
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

mod callback;
mod error;
mod manager;

// Re-export public API
pub use callback::{ClosureCallback, StateCallback};
pub use error::{ConnectionState, StateError, StateResult};
pub use manager::ConnectionStateManager;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-connectionstate";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-connectionstate");
    }

    #[test]
    fn test_state_transitions() {
        let mut manager = ConnectionStateManager::new();

        // Valid progression
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
    fn test_state_regression() {
        let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);

        // Regression is allowed (e.g., connection lost)
        assert!(manager.set_state(ConnectionState::Connecting).unwrap());
        assert!(manager
            .set_state(ConnectionState::WaitingForNetwork)
            .unwrap());
    }

    #[test]
    fn test_callback_lifecycle() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let count = std::sync::Arc::new(AtomicUsize::new(0));

        let mut manager = ConnectionStateManager::new();

        // Register callback
        let count_clone = count.clone();
        manager.register_callback(Box::new(ClosureCallback::new(move |state| {
            count_clone.fetch_add(1, Ordering::AcqRel);
            // Unregister after reaching Ready
            state != ConnectionState::Ready
        })));

        // Trigger state changes
        for state in [
            ConnectionState::WaitingForNetwork,
            ConnectionState::Connecting,
            ConnectionState::Ready,
        ] {
            manager.set_state(state).unwrap();
        }

        // Callback should have been called 3 times
        assert_eq!(count.load(Ordering::Acquire), 3);

        // Callback should be unregistered now
        assert_eq!(manager.callback_count(), 0);

        // This should not trigger callback
        manager.set_state(ConnectionState::Empty).unwrap();
        assert_eq!(count.load(Ordering::Acquire), 3);
    }

    #[test]
    fn test_multiple_callbacks() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let count1 = std::sync::Arc::new(AtomicUsize::new(0));
        let count2 = std::sync::Arc::new(AtomicUsize::new(0));

        let mut manager = ConnectionStateManager::new();

        let count1_clone = count1.clone();
        manager.register_callback(Box::new(ClosureCallback::new(move |_| {
            count1_clone.fetch_add(1, Ordering::AcqRel);
            true
        })));

        let count2_clone = count2.clone();
        manager.register_callback(Box::new(ClosureCallback::new(move |_| {
            count2_clone.fetch_add(1, Ordering::AcqRel);
            true
        })));

        manager.set_state(ConnectionState::Ready).unwrap();

        // Both callbacks should have been called
        assert_eq!(count1.load(Ordering::Acquire), 1);
        assert_eq!(count2.load(Ordering::Acquire), 1);
    }

    #[test]
    fn test_state_properties() {
        assert!(!ConnectionState::Empty.is_connected());
        assert!(!ConnectionState::WaitingForNetwork.is_connected());
        assert!(!ConnectionState::ConnectingToProxy.is_connected());
        assert!(!ConnectionState::Connecting.is_connected());
        assert!(ConnectionState::Updating.is_connected());
        assert!(ConnectionState::Ready.is_connected());
    }

    #[test]
    fn test_manager_reset() {
        let mut manager = ConnectionStateManager::new();

        manager.set_state(ConnectionState::Ready).unwrap();
        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));

        assert_eq!(manager.current_state(), ConnectionState::Ready);
        assert_eq!(manager.callback_count(), 1);

        manager.reset();

        assert_eq!(manager.current_state(), ConnectionState::Empty);
        assert_eq!(manager.callback_count(), 0);
    }
}
