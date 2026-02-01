//! # Connection State
//!
//! Represents the current connection state of the Telegram client.
//!
//! ## Overview
//!
//! This module defines the `ConnectionState` enum, which represents the
//! current state of the network connection to Telegram servers.
//!
//! ## TDLib Correspondence
//!
//! TDLib enum: `ConnectionState`
//! - `ConnectionState::WaitingForNetwork` → TDLib `ConnectionState::WaitingForNetwork`
//! - `ConnectionState::ConnectingToProxy` → TDLib `ConnectionState::ConnectingToProxy`
//! - `ConnectionState::Connecting` → TDLib `ConnectionState::Connecting`
//! - `ConnectionState::Updating` → TDLib `ConnectionState::Updating`
//! - `ConnectionState::Ready` → TDLib `ConnectionState::Ready`
//! - `ConnectionState::Empty` → TDLib `ConnectionState::Empty`
//!
//! ## Examples
//!
//! ```
//! use rustgram_connection_state::ConnectionState;
//!
//! // Create connection state
//! let state = ConnectionState::Connecting;
//!
//! // Check if connected
//! if state.is_connected() {
//!     println!("Connected to Telegram");
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use core::fmt;

/// Represents the current connection state of the Telegram client.
///
/// The connection state follows a progression from network waiting through
/// connecting to ready, with possible transitions to updating states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConnectionState {
    /// Waiting for network connection
    WaitingForNetwork = 0,
    /// Connecting to proxy
    ConnectingToProxy = 1,
    /// Connecting to Telegram servers
    Connecting = 2,
    /// Updating connection data
    Updating = 3,
    /// Connection is ready and stable
    Ready = 4,
    /// Connection state is empty/not initialized
    Empty = 5,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Empty
    }
}

impl ConnectionState {
    /// Creates ConnectionState from an i32 value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not in the range 0..=5.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state::ConnectionState;
    ///
    /// assert_eq!(ConnectionState::from_i32(0), Ok(ConnectionState::WaitingForNetwork));
    /// assert_eq!(ConnectionState::from_i32(4), Ok(ConnectionState::Ready));
    /// assert!(ConnectionState::from_i32(99).is_err());
    /// ```
    pub const fn from_i32(value: i32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::WaitingForNetwork),
            1 => Ok(Self::ConnectingToProxy),
            2 => Ok(Self::Connecting),
            3 => Ok(Self::Updating),
            4 => Ok(Self::Ready),
            5 => Ok(Self::Empty),
            _ => Err(Error::InvalidValue(value)),
        }
    }

    /// Returns the i32 representation of this ConnectionState.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state::ConnectionState;
    ///
    /// assert_eq!(ConnectionState::WaitingForNetwork.as_i32(), 0);
    /// assert_eq!(ConnectionState::Ready.as_i32(), 4);
    /// ```
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Checks if the client is currently connecting.
    ///
    /// Returns true for ConnectingToProxy, Connecting, and Updating states.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state::ConnectionState;
    ///
    /// assert!(ConnectionState::Connecting.is_connecting());
    /// assert!(ConnectionState::ConnectingToProxy.is_connecting());
    /// assert!(!ConnectionState::Ready.is_connecting());
    /// ```
    #[must_use]
    pub const fn is_connecting(self) -> bool {
        matches!(
            self,
            Self::ConnectingToProxy | Self::Connecting | Self::Updating
        )
    }

    /// Checks if the client is connected and ready.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state::ConnectionState;
    ///
    /// assert!(ConnectionState::Ready.is_connected());
    /// assert!(!ConnectionState::Connecting.is_connected());
    /// ```
    #[must_use]
    pub const fn is_connected(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Checks if the connection state is empty/uninitialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state::ConnectionState;
    ///
    /// assert!(ConnectionState::Empty.is_empty());
    /// assert!(!ConnectionState::Ready.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Checks if the client is waiting for network.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state::ConnectionState;
    ///
    /// assert!(ConnectionState::WaitingForNetwork.is_waiting_for_network());
    /// assert!(!ConnectionState::Connecting.is_waiting_for_network());
    /// ```
    #[must_use]
    pub const fn is_waiting_for_network(self) -> bool {
        matches!(self, Self::WaitingForNetwork)
    }

    /// Returns all connection state variants.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connection_state::ConnectionState;
    ///
    /// let all = ConnectionState::all();
    /// assert_eq!(all.len(), 6);
    /// ```
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::WaitingForNetwork,
            Self::ConnectingToProxy,
            Self::Connecting,
            Self::Updating,
            Self::Ready,
            Self::Empty,
        ]
    }
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WaitingForNetwork => write!(f, "WaitingForNetwork"),
            Self::ConnectingToProxy => write!(f, "ConnectingToProxy"),
            Self::Connecting => write!(f, "Connecting"),
            Self::Updating => write!(f, "Updating"),
            Self::Ready => write!(f, "Ready"),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

/// Error type for ConnectionState operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid i32 value for ConnectionState
    InvalidValue(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(v) => write!(f, "Invalid ConnectionState value: {}", v),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (6)
    #[test]
    fn test_default() {
        assert_eq!(ConnectionState::default(), ConnectionState::Empty);
    }

    #[test]
    fn test_copy() {
        let state = ConnectionState::Connecting;
        let copy = state;
        assert_eq!(state, ConnectionState::Connecting);
        assert_eq!(copy, ConnectionState::Connecting);
    }

    #[test]
    fn test_clone() {
        let state = ConnectionState::Ready;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(ConnectionState::Ready, ConnectionState::Ready);
        assert_ne!(ConnectionState::Ready, ConnectionState::Connecting);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ConnectionState::Ready);
        set.insert(ConnectionState::Ready);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<ConnectionState>();
        assert_sync::<ConnectionState>();
    }

    // from_i32 tests (3)
    #[test]
    fn test_from_i32_valid() {
        assert_eq!(
            ConnectionState::from_i32(0),
            Ok(ConnectionState::WaitingForNetwork)
        );
        assert_eq!(
            ConnectionState::from_i32(1),
            Ok(ConnectionState::ConnectingToProxy)
        );
        assert_eq!(
            ConnectionState::from_i32(2),
            Ok(ConnectionState::Connecting)
        );
        assert_eq!(ConnectionState::from_i32(3), Ok(ConnectionState::Updating));
        assert_eq!(ConnectionState::from_i32(4), Ok(ConnectionState::Ready));
        assert_eq!(ConnectionState::from_i32(5), Ok(ConnectionState::Empty));
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(ConnectionState::from_i32(-1), Err(Error::InvalidValue(-1)));
        assert_eq!(ConnectionState::from_i32(6), Err(Error::InvalidValue(6)));
        assert_eq!(ConnectionState::from_i32(99), Err(Error::InvalidValue(99)));
    }

    #[test]
    fn test_from_i32_roundtrip() {
        for state in ConnectionState::all() {
            assert_eq!(ConnectionState::from_i32(state.as_i32()), Ok(*state));
        }
    }

    // as_i32 tests (2)
    #[test]
    fn test_as_i32() {
        assert_eq!(ConnectionState::WaitingForNetwork.as_i32(), 0);
        assert_eq!(ConnectionState::ConnectingToProxy.as_i32(), 1);
        assert_eq!(ConnectionState::Connecting.as_i32(), 2);
        assert_eq!(ConnectionState::Updating.as_i32(), 3);
        assert_eq!(ConnectionState::Ready.as_i32(), 4);
        assert_eq!(ConnectionState::Empty.as_i32(), 5);
    }

    // State check tests (4)
    #[test]
    fn test_is_connecting() {
        assert!(!ConnectionState::WaitingForNetwork.is_connecting());
        assert!(ConnectionState::ConnectingToProxy.is_connecting());
        assert!(ConnectionState::Connecting.is_connecting());
        assert!(ConnectionState::Updating.is_connecting());
        assert!(!ConnectionState::Ready.is_connecting());
        assert!(!ConnectionState::Empty.is_connecting());
    }

    #[test]
    fn test_is_connected() {
        assert!(!ConnectionState::WaitingForNetwork.is_connected());
        assert!(!ConnectionState::Connecting.is_connected());
        assert!(ConnectionState::Ready.is_connected());
    }

    #[test]
    fn test_is_empty() {
        assert!(ConnectionState::Empty.is_empty());
        assert!(!ConnectionState::Ready.is_empty());
        assert!(!ConnectionState::Connecting.is_empty());
    }

    #[test]
    fn test_is_waiting_for_network() {
        assert!(ConnectionState::WaitingForNetwork.is_waiting_for_network());
        assert!(!ConnectionState::Connecting.is_waiting_for_network());
    }

    // all() tests (2)
    #[test]
    fn test_all_count() {
        assert_eq!(ConnectionState::all().len(), 6);
    }

    #[test]
    fn test_all_contains_all() {
        let all = ConnectionState::all();
        assert!(all.contains(&ConnectionState::WaitingForNetwork));
        assert!(all.contains(&ConnectionState::ConnectingToProxy));
        assert!(all.contains(&ConnectionState::Connecting));
        assert!(all.contains(&ConnectionState::Updating));
        assert!(all.contains(&ConnectionState::Ready));
        assert!(all.contains(&ConnectionState::Empty));
    }

    // Display tests (2)
    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", ConnectionState::WaitingForNetwork),
            "WaitingForNetwork"
        );
        assert_eq!(format!("{}", ConnectionState::Ready), "Ready");
    }

    // Error tests (2)
    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::InvalidValue(99)),
            "Invalid ConnectionState value: 99"
        );
    }

    #[test]
    fn test_error_partial_eq() {
        assert_eq!(Error::InvalidValue(1), Error::InvalidValue(1));
        assert_ne!(Error::InvalidValue(1), Error::InvalidValue(2));
    }

    // Const eval tests (2)
    #[test]
    fn test_const_as_i32() {
        const VALUE: i32 = ConnectionState::Ready.as_i32();
        assert_eq!(VALUE, 4);
    }

    #[test]
    fn test_const_is_connected() {
        const IS_READY: bool = ConnectionState::Ready.is_connected();
        assert!(IS_READY);
    }
}
