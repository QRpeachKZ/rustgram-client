//! Error types for connection state management.

use std::fmt;

/// Errors that can occur in connection state management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    /// Invalid state transition.
    InvalidTransition {
        /// The current state.
        from: ConnectionState,
        /// The target state.
        to: ConnectionState,
    },
    /// Callback registration failed.
    CallbackRegistrationFailed,
    /// Callback invocation failed.
    CallbackFailed,
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { from, to } => {
                write!(f, "Invalid state transition from {:?} to {:?}", from, to)
            }
            Self::CallbackRegistrationFailed => f.write_str("Failed to register callback"),
            Self::CallbackFailed => f.write_str("Callback invocation failed"),
        }
    }
}

impl std::error::Error for StateError {}

/// Result type for state operations.
pub type StateResult<T> = Result<T, StateError>;

/// Connection state for the Telegram client.
///
/// Represents the current state of the connection to Telegram servers.
/// States follow a progression from `Empty` through connection states to `Ready`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    /// Initial empty state (not yet connected).
    Empty = 0,
    /// Waiting for network connectivity.
    WaitingForNetwork = 1,
    /// Connecting to proxy server.
    ConnectingToProxy = 2,
    /// Connecting to Telegram servers.
    Connecting = 3,
    /// Syncing data with servers.
    Updating = 4,
    /// Connection ready and operational.
    Ready = 5,
}

impl ConnectionState {
    /// Returns true if this state represents an active connection.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// assert!(!ConnectionState::Empty.is_connected());
    /// assert!(!ConnectionState::WaitingForNetwork.is_connected());
    /// assert!(!ConnectionState::Connecting.is_connected());
    /// assert!(ConnectionState::Updating.is_connected());
    /// assert!(ConnectionState::Ready.is_connected());
    /// ```
    #[inline]
    pub const fn is_connected(self) -> bool {
        matches!(self, Self::Updating | Self::Ready)
    }

    /// Returns true if this state represents a connecting phase.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// assert!(!ConnectionState::Empty.is_connecting());
    /// assert!(!ConnectionState::WaitingForNetwork.is_connecting());
    /// assert!(ConnectionState::ConnectingToProxy.is_connecting());
    /// assert!(ConnectionState::Connecting.is_connecting());
    /// assert!(!ConnectionState::Updating.is_connecting());
    /// assert!(!ConnectionState::Ready.is_connecting());
    /// ```
    #[inline]
    pub const fn is_connecting(self) -> bool {
        matches!(self, Self::ConnectingToProxy | Self::Connecting)
    }

    /// Returns true if this state represents a ready state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// assert!(!ConnectionState::Empty.is_ready());
    /// assert!(ConnectionState::Ready.is_ready());
    /// ```
    #[inline]
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns the numeric representation of this state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// assert_eq!(ConnectionState::Empty.as_i32(), 0);
    /// assert_eq!(ConnectionState::Ready.as_i32(), 5);
    /// ```
    #[inline]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Creates a connection state from its numeric representation.
    ///
    /// Returns `None` if the value is not a valid state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// assert_eq!(ConnectionState::from_i32(0), Some(ConnectionState::Empty));
    /// assert_eq!(ConnectionState::from_i32(5), Some(ConnectionState::Ready));
    /// assert_eq!(ConnectionState::from_i32(99), None);
    /// ```
    #[inline]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Empty),
            1 => Some(Self::WaitingForNetwork),
            2 => Some(Self::ConnectingToProxy),
            3 => Some(Self::Connecting),
            4 => Some(Self::Updating),
            5 => Some(Self::Ready),
            _ => None,
        }
    }

    /// Returns the name of this state as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_connectionstate::ConnectionState;
    ///
    /// assert_eq!(ConnectionState::Ready.name(), "Ready");
    /// ```
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::WaitingForNetwork => "WaitingForNetwork",
            Self::ConnectingToProxy => "ConnectingToProxy",
            Self::Connecting => "Connecting",
            Self::Updating => "Updating",
            Self::Ready => "Ready",
        }
    }
}

impl Default for ConnectionState {
    #[inline]
    fn default() -> Self {
        Self::Empty
    }
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_ordering() {
        assert!(ConnectionState::Empty < ConnectionState::WaitingForNetwork);
        assert!(ConnectionState::Connecting < ConnectionState::Ready);
    }

    #[test]
    fn test_is_connected() {
        assert!(!ConnectionState::Empty.is_connected());
        assert!(!ConnectionState::WaitingForNetwork.is_connected());
        assert!(!ConnectionState::ConnectingToProxy.is_connected());
        assert!(!ConnectionState::Connecting.is_connected());
        assert!(ConnectionState::Updating.is_connected());
        assert!(ConnectionState::Ready.is_connected());
    }

    #[test]
    fn test_is_connecting() {
        assert!(!ConnectionState::Empty.is_connecting());
        assert!(!ConnectionState::WaitingForNetwork.is_connecting());
        assert!(ConnectionState::ConnectingToProxy.is_connecting());
        assert!(ConnectionState::Connecting.is_connecting());
        assert!(!ConnectionState::Updating.is_connecting());
        assert!(!ConnectionState::Ready.is_connecting());
    }

    #[test]
    fn test_is_ready() {
        assert!(!ConnectionState::Empty.is_ready());
        assert!(!ConnectionState::WaitingForNetwork.is_ready());
        assert!(!ConnectionState::ConnectingToProxy.is_ready());
        assert!(!ConnectionState::Connecting.is_ready());
        assert!(!ConnectionState::Updating.is_ready());
        assert!(ConnectionState::Ready.is_ready());
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(ConnectionState::Empty.as_i32(), 0);
        assert_eq!(ConnectionState::WaitingForNetwork.as_i32(), 1);
        assert_eq!(ConnectionState::ConnectingToProxy.as_i32(), 2);
        assert_eq!(ConnectionState::Connecting.as_i32(), 3);
        assert_eq!(ConnectionState::Updating.as_i32(), 4);
        assert_eq!(ConnectionState::Ready.as_i32(), 5);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(ConnectionState::from_i32(0), Some(ConnectionState::Empty));
        assert_eq!(
            ConnectionState::from_i32(1),
            Some(ConnectionState::WaitingForNetwork)
        );
        assert_eq!(
            ConnectionState::from_i32(2),
            Some(ConnectionState::ConnectingToProxy)
        );
        assert_eq!(
            ConnectionState::from_i32(3),
            Some(ConnectionState::Connecting)
        );
        assert_eq!(
            ConnectionState::from_i32(4),
            Some(ConnectionState::Updating)
        );
        assert_eq!(ConnectionState::from_i32(5), Some(ConnectionState::Ready));
        assert_eq!(ConnectionState::from_i32(-1), None);
        assert_eq!(ConnectionState::from_i32(99), None);
    }

    #[test]
    fn test_name() {
        assert_eq!(ConnectionState::Empty.name(), "Empty");
        assert_eq!(
            ConnectionState::WaitingForNetwork.name(),
            "WaitingForNetwork"
        );
        assert_eq!(
            ConnectionState::ConnectingToProxy.name(),
            "ConnectingToProxy"
        );
        assert_eq!(ConnectionState::Connecting.name(), "Connecting");
        assert_eq!(ConnectionState::Updating.name(), "Updating");
        assert_eq!(ConnectionState::Ready.name(), "Ready");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ConnectionState::Ready), "Ready");
        assert_eq!(format!("{}", ConnectionState::Connecting), "Connecting");
    }

    #[test]
    fn test_default() {
        assert_eq!(ConnectionState::default(), ConnectionState::Empty);
    }

    #[test]
    fn test_error_display() {
        let err = StateError::InvalidTransition {
            from: ConnectionState::Empty,
            to: ConnectionState::Ready,
        };
        assert!(err.to_string().contains("Invalid state transition"));

        let err = StateError::CallbackRegistrationFailed;
        assert_eq!(err.to_string(), "Failed to register callback");

        let err = StateError::CallbackFailed;
        assert_eq!(err.to_string(), "Callback invocation failed");
    }
}
