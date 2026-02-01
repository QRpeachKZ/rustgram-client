//! Callback trait for connection state changes.

use crate::error::ConnectionState;

/// Callback trait for receiving connection state change notifications.
///
/// Implementations of this trait can be registered with
/// `ConnectionStateManager` to receive notifications when the
/// connection state changes.
///
/// # Examples
///
/// ```
/// use rustgram_connectionstate::{ConnectionState, StateCallback};
///
/// struct MyCallback;
///
/// impl StateCallback for MyCallback {
///     fn on_state_changed(&self, state: ConnectionState) -> bool {
///         println!("State changed to: {}", state);
///         true
///     }
/// }
/// ```
pub trait StateCallback: Send + Sync {
    /// Called when the connection state changes.
    ///
    /// # Parameters
    ///
    /// * `state` - The new connection state
    ///
    /// # Returns
    ///
    /// * `true` if the callback should remain registered
    /// * `false` if the callback should be unregistered
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::{ConnectionState, StateCallback};
    ///
    /// struct LoggingCallback;
    ///
    /// impl StateCallback for LoggingCallback {
    ///     fn on_state_changed(&self, state: ConnectionState) -> bool {
    ///         println!("New state: {}", state);
    ///         true
    ///     }
    /// }
    /// ```
    fn on_state_changed(&self, state: ConnectionState) -> bool;
}

/// Simple closure-based callback implementation.
///
/// # Examples
///
/// ```
/// use rustgram_connectionstate::{ConnectionState, ClosureCallback};
///
/// let callback = ClosureCallback::new(|state| {
///     println!("State: {}", state);
///     true // Keep callback registered
/// });
/// ```
pub struct ClosureCallback<F>
where
    F: Fn(ConnectionState) -> bool + Send + Sync,
{
    /// The closure to call when state changes.
    f: F,
}

impl<F> ClosureCallback<F>
where
    F: Fn(ConnectionState) -> bool + Send + Sync,
{
    /// Creates a new closure-based callback.
    ///
    /// # Parameters
    ///
    /// * `f` - Closure that receives the new state and returns whether to keep the callback
    #[inline]
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> StateCallback for ClosureCallback<F>
where
    F: Fn(ConnectionState) -> bool + Send + Sync,
{
    #[inline]
    fn on_state_changed(&self, state: ConnectionState) -> bool {
        (self.f)(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

    #[derive(Clone)]
    struct TestCallback {
        state: std::sync::Arc<AtomicU8>,
        should_keep: std::sync::Arc<AtomicBool>,
    }

    impl TestCallback {
        fn new(keep: bool) -> Self {
            Self {
                state: std::sync::Arc::new(AtomicU8::new(0)),
                should_keep: std::sync::Arc::new(AtomicBool::new(keep)),
            }
        }

        fn get_state(&self) -> u8 {
            self.state.load(Ordering::Acquire)
        }
    }

    impl StateCallback for TestCallback {
        fn on_state_changed(&self, state: ConnectionState) -> bool {
            self.state.store(state.as_i32() as u8, Ordering::Release);
            self.should_keep.load(Ordering::Acquire)
        }
    }

    #[test]
    fn test_callback_invocation() {
        let callback = TestCallback::new(true);
        callback.on_state_changed(ConnectionState::Ready);
        assert_eq!(callback.get_state(), ConnectionState::Ready.as_i32() as u8);
    }

    #[test]
    fn test_callback_keep_registered() {
        let callback = TestCallback::new(true);
        assert!(callback.on_state_changed(ConnectionState::Connecting));
    }

    #[test]
    fn test_callback_unregister() {
        let callback = TestCallback::new(false);
        assert!(!callback.on_state_changed(ConnectionState::Ready));
    }

    #[test]
    fn test_closure_callback() {
        let called = std::sync::Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let callback = ClosureCallback::new(move |state| {
            called_clone.store(true, Ordering::Release);
            assert_eq!(state, ConnectionState::Updating);
            true
        });

        assert!(callback.on_state_changed(ConnectionState::Updating));
        assert!(called.load(Ordering::Acquire));
    }

    #[test]
    fn test_closure_callback_unregister() {
        let callback = ClosureCallback::new(|_| false);
        assert!(!callback.on_state_changed(ConnectionState::Ready));
    }

    #[test]
    fn test_closure_callback_state_tracking() {
        let last_state = std::sync::Arc::new(std::sync::Mutex::new(ConnectionState::Empty));
        let last_state_clone = last_state.clone();

        let callback = ClosureCallback::new(move |state| {
            *last_state_clone.lock().unwrap() = state;
            true
        });

        callback.on_state_changed(ConnectionState::Connecting);
        assert_eq!(*last_state.lock().unwrap(), ConnectionState::Connecting);

        callback.on_state_changed(ConnectionState::Ready);
        assert_eq!(*last_state.lock().unwrap(), ConnectionState::Ready);
    }
}
