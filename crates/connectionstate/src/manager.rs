//! Connection state manager.

use crate::callback::StateCallback;
use crate::error::{ConnectionState, StateResult};

/// Manager for tracking and notifying connection state changes.
///
/// `ConnectionStateManager` maintains the current connection state and
/// notifies registered callbacks when the state changes. It prevents
/// duplicate notifications for the same state.
///
/// # Examples
///
/// ```
/// use rustgram_connectionstate::{ConnectionStateManager, ConnectionState, StateCallback};
///
/// struct MyCallback;
///
/// impl StateCallback for MyCallback {
///     fn on_state_changed(&self, state: ConnectionState) -> bool {
///         println!("State: {}", state);
///         true
///     }
/// }
///
/// let mut manager = ConnectionStateManager::new();
/// manager.register_callback(Box::new(MyCallback));
/// manager.set_state(ConnectionState::Connecting).unwrap();
/// ```
pub struct ConnectionStateManager {
    /// Current connection state.
    current_state: ConnectionState,
    /// Registered callbacks.
    callbacks: Vec<Box<dyn StateCallback>>,
}

impl ConnectionStateManager {
    /// Creates a new connection state manager with initial `Empty` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::{ConnectionStateManager, ConnectionState};
    ///
    /// let manager = ConnectionStateManager::new();
    /// assert_eq!(manager.current_state(), ConnectionState::Empty);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            current_state: ConnectionState::Empty,
            callbacks: Vec::new(),
        }
    }

    /// Creates a new connection state manager with the specified initial state.
    ///
    /// # Parameters
    ///
    /// * `initial_state` - The initial connection state
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::{ConnectionStateManager, ConnectionState};
    ///
    /// let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
    /// assert_eq!(manager.current_state(), ConnectionState::Ready);
    /// ```
    #[inline]
    pub const fn with_state(initial_state: ConnectionState) -> Self {
        Self {
            current_state: initial_state,
            callbacks: Vec::new(),
        }
    }

    /// Returns the current connection state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::{ConnectionStateManager, ConnectionState};
    ///
    /// let manager = ConnectionStateManager::new();
    /// assert_eq!(manager.current_state(), ConnectionState::Empty);
    /// ```
    #[inline]
    pub const fn current_state(&self) -> ConnectionState {
        self.current_state
    }

    /// Sets a new connection state and notifies callbacks if changed.
    ///
    /// If the new state is different from the current state, all registered
    /// callbacks will be notified. Callbacks that return `false` will be
    /// automatically unregistered.
    ///
    /// # Parameters
    ///
    /// * `new_state` - The new connection state
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if state changed and callbacks were notified
    /// * `Ok(false)` if state was unchanged
    /// * `Err(StateError)` if callbacks failed
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::new();
    ///
    /// // State changes from Empty to Connecting
    /// let changed = manager.set_state(ConnectionState::Connecting).unwrap();
    /// assert!(changed);
    ///
    /// // State unchanged
    /// let changed = manager.set_state(ConnectionState::Connecting).unwrap();
    /// assert!(!changed);
    /// ```
    pub fn set_state(&mut self, new_state: ConnectionState) -> StateResult<bool> {
        if new_state == self.current_state {
            return Ok(false);
        }

        self.current_state = new_state;
        self.notify_callbacks(new_state)?;
        Ok(true)
    }

    /// Registers a callback for state change notifications.
    ///
    /// # Parameters
    ///
    /// * `callback` - The callback to register
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::{ConnectionStateManager, StateCallback, ConnectionState};
    ///
    /// struct MyCallback;
    /// impl StateCallback for MyCallback {
    ///     fn on_state_changed(&self, state: ConnectionState) -> bool { true }
    /// }
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// manager.register_callback(Box::new(MyCallback));
    /// ```
    pub fn register_callback(&mut self, callback: Box<dyn StateCallback>) {
        self.callbacks.push(callback);
    }

    /// Removes all registered callbacks.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionStateManager;
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// manager.register_callback(Box::new(
    ///     rustgram_connectionstate::ClosureCallback::new(|_| true)
    /// ));
    /// assert!(!manager.is_empty());
    ///
    /// manager.clear_callbacks();
    /// assert!(manager.is_empty());
    /// ```
    pub fn clear_callbacks(&mut self) {
        self.callbacks.clear();
    }

    /// Returns the number of registered callbacks.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionStateManager;
    ///
    /// let mut manager = ConnectionStateManager::new();
    /// assert_eq!(manager.callback_count(), 0);
    ///
    /// manager.register_callback(Box::new(
    ///     rustgram_connectionstate::ClosureCallback::new(|_| true)
    /// ));
    /// assert_eq!(manager.callback_count(), 1);
    /// ```
    #[inline]
    pub fn callback_count(&self) -> usize {
        self.callbacks.len()
    }

    /// Returns true if there are no registered callbacks.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionStateManager;
    ///
    /// let manager = ConnectionStateManager::new();
    /// assert!(manager.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.callbacks.is_empty()
    }

    /// Resets the state to `Empty` and clears all callbacks.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::{ConnectionStateManager, ConnectionState};
    ///
    /// let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);
    /// manager.register_callback(Box::new(
    ///     rustgram_connectionstate::ClosureCallback::new(|_| true)
    /// ));
    ///
    /// manager.reset();
    /// assert_eq!(manager.current_state(), ConnectionState::Empty);
    /// assert!(manager.is_empty());
    /// ```
    pub fn reset(&mut self) {
        self.current_state = ConnectionState::Empty;
        self.callbacks.clear();
    }

    /// Notifies all registered callbacks of a state change.
    ///
    /// Callbacks that return `false` are removed. Any callback that
    /// causes an error during execution is logged and removed.
    fn notify_callbacks(&mut self, state: ConnectionState) -> StateResult<()> {
        // Keep callbacks that return true (want to stay registered)
        // Callbacks returning false are removed silently (normal unregister behavior)
        self.callbacks
            .retain(|callback| callback.on_state_changed(state));
        Ok(())
    }
}

impl Default for ConnectionStateManager {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ConnectionStateManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionStateManager")
            .field("current_state", &self.current_state)
            .field("callback_count", &self.callbacks.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::callback::ClosureCallback;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_new_manager() {
        let manager = ConnectionStateManager::new();
        assert_eq!(manager.current_state(), ConnectionState::Empty);
        assert!(manager.is_empty());
        assert_eq!(manager.callback_count(), 0);
    }

    #[test]
    fn test_with_state() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        assert_eq!(manager.current_state(), ConnectionState::Ready);
    }

    #[test]
    fn test_set_state_changes() {
        let mut manager = ConnectionStateManager::new();

        // Empty -> Connecting
        let changed = manager.set_state(ConnectionState::Connecting).unwrap();
        assert!(changed);
        assert_eq!(manager.current_state(), ConnectionState::Connecting);

        // Connecting -> Ready
        let changed = manager.set_state(ConnectionState::Ready).unwrap();
        assert!(changed);
        assert_eq!(manager.current_state(), ConnectionState::Ready);
    }

    #[test]
    fn test_set_state_unchanged() {
        let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);

        let changed = manager.set_state(ConnectionState::Ready).unwrap();
        assert!(!changed);
        assert_eq!(manager.current_state(), ConnectionState::Ready);
    }

    #[test]
    fn test_register_callback() {
        let mut manager = ConnectionStateManager::new();
        assert!(manager.is_empty());

        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));
        assert_eq!(manager.callback_count(), 1);
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_multiple_callbacks() {
        let mut manager = ConnectionStateManager::new();

        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));
        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));
        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));

        assert_eq!(manager.callback_count(), 3);
    }

    #[test]
    fn test_callback_invocation() {
        let called = std::sync::Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let mut manager = ConnectionStateManager::new();
        manager.register_callback(Box::new(ClosureCallback::new(move |state| {
            called_clone.store(true, Ordering::Release);
            assert_eq!(state, ConnectionState::Connecting);
            true
        })));

        manager.set_state(ConnectionState::Connecting).unwrap();
        assert!(called.load(Ordering::Acquire));
    }

    #[test]
    fn test_callback_not_invoked_on_unchanged() {
        let called = std::sync::Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let mut manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        manager.register_callback(Box::new(ClosureCallback::new(move |state| {
            called_clone.store(true, Ordering::Release);
            true
        })));

        // Set same state
        manager.set_state(ConnectionState::Ready).unwrap();
        assert!(!called.load(Ordering::Acquire));
    }

    #[test]
    fn test_callback_auto_unregister() {
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let mut manager = ConnectionStateManager::new();
        manager.register_callback(Box::new(ClosureCallback::new(move |state| {
            call_count_clone.fetch_add(1, Ordering::AcqRel);
            // Unregister after first call
            state != ConnectionState::Connecting
        })));

        // First call - callback returns false, should unregister
        manager.set_state(ConnectionState::Connecting).unwrap();
        assert_eq!(call_count.load(Ordering::Acquire), 1);
        assert_eq!(manager.callback_count(), 0);

        // Second call - no callbacks registered
        manager.set_state(ConnectionState::Ready).unwrap();
        assert_eq!(call_count.load(Ordering::Acquire), 1); // Still 1
    }

    #[test]
    fn test_clear_callbacks() {
        let mut manager = ConnectionStateManager::new();

        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));
        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));

        assert_eq!(manager.callback_count(), 2);

        manager.clear_callbacks();
        assert!(manager.is_empty());
        assert_eq!(manager.callback_count(), 0);
    }

    #[test]
    fn test_reset() {
        let mut manager = ConnectionStateManager::new();

        manager.set_state(ConnectionState::Ready).unwrap();
        manager.register_callback(Box::new(ClosureCallback::new(|_| true)));

        assert_eq!(manager.current_state(), ConnectionState::Ready);
        assert_eq!(manager.callback_count(), 1);

        manager.reset();

        assert_eq!(manager.current_state(), ConnectionState::Empty);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_multiple_callbacks_all_invoked() {
        let count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut manager = ConnectionStateManager::new();

        for _ in 0..5 {
            let count_clone = count.clone();
            manager.register_callback(Box::new(ClosureCallback::new(move |_| {
                count_clone.fetch_add(1, Ordering::AcqRel);
                true
            })));
        }

        manager.set_state(ConnectionState::Updating).unwrap();
        assert_eq!(count.load(Ordering::Acquire), 5);
    }

    #[test]
    fn test_debug_format() {
        let manager = ConnectionStateManager::with_state(ConnectionState::Ready);
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("ConnectionStateManager"));
        assert!(debug_str.contains("Ready"));
    }
}
