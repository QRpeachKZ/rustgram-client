//! # Call Actor
//!
//! Actor-based implementation for managing Telegram calls.
//!
//! ## Overview
//!
//! This module implements the call actor, which manages the state and
//! operations of voice/video calls in Telegram using an actor pattern.
//!
//! ## TDLib Correspondence
//!
//! TDLib class: `CallActor`
//!
//! ## Call States
//!
//! The call actor progresses through these states:
//! - `Empty` - Not initialized
//! - `SendRequestQuery` - Sending call request
//! - `WaitRequestResult` - Waiting for request response
//! - `SendAcceptQuery` - Sending accept request
//! - `WaitAcceptResult` - Waiting for accept response
//! - `SendConfirmQuery` - Sending confirm request
//! - `WaitConfirmResult` - Waiting for confirm response
//! - `Ready` - Call is ready
//! - `SendDiscardQuery` - Sending discard request
//! - `WaitDiscardResult` - Waiting for discard response
//! - `Discarded` - Call is discarded
//!
//! ## Examples
//!
//! ```
//! use rustgram_call_actor::{CallActor, CallState};
//! use rustgram_call_id::CallId;
//! use rustgram_types::UserId;
//!
//! // Create a new call actor
//! let call_id = CallId::new(123);
//! let user_id = UserId::from_i32(456);
//! let actor = CallActor::new(call_id, user_id, false);
//!
//! // Check initial state
//! assert_eq!(actor.state(), CallState::Empty);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use core::fmt;
use rustgram_call_discard_reason::CallDiscardReason;
use rustgram_call_id::CallId;
use rustgram_types::UserId;

/// Protocol settings for a Telegram call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallProtocol {
    /// Enable UDP P2P connections.
    pub udp_p2p: bool,
    /// Enable UDP reflector connections.
    pub udp_reflector: bool,
    /// Minimum protocol layer version.
    pub min_layer: i32,
    /// Maximum protocol layer version.
    pub max_layer: i32,
    /// Supported library versions.
    pub library_versions: Vec<String>,
}

impl Default for CallProtocol {
    fn default() -> Self {
        Self {
            udp_p2p: true,
            udp_reflector: true,
            min_layer: 65,
            max_layer: 65,
            library_versions: Vec::new(),
        }
    }
}

impl CallProtocol {
    /// Creates a new CallProtocol with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallProtocol;
    ///
    /// let protocol = CallProtocol::new();
    /// assert!(protocol.udp_p2p);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the UDP P2P flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallProtocol;
    ///
    /// let protocol = CallProtocol::new().with_udp_p2p(false);
    /// assert!(!protocol.udp_p2p);
    /// ```
    #[must_use]
    pub const fn with_udp_p2p(mut self, udp_p2p: bool) -> Self {
        self.udp_p2p = udp_p2p;
        self
    }

    /// Sets the UDP reflector flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallProtocol;
    ///
    /// let protocol = CallProtocol::new().with_udp_reflector(false);
    /// assert!(!protocol.udp_reflector);
    /// ```
    #[must_use]
    pub const fn with_udp_reflector(mut self, udp_reflector: bool) -> Self {
        self.udp_reflector = udp_reflector;
        self
    }

    /// Sets the layer range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallProtocol;
    ///
    /// let protocol = CallProtocol::new().with_layers(65, 70);
    /// assert_eq!(protocol.min_layer, 65);
    /// assert_eq!(protocol.max_layer, 70);
    /// ```
    #[must_use]
    pub const fn with_layers(mut self, min_layer: i32, max_layer: i32) -> Self {
        self.min_layer = min_layer;
        self.max_layer = max_layer;
        self
    }
}

/// Internal state of the call actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CallState {
    /// Call is not initialized.
    Empty = 0,
    /// Sending request to server.
    SendRequestQuery = 1,
    /// Waiting for request result.
    WaitRequestResult = 2,
    /// Sending accept to server.
    SendAcceptQuery = 3,
    /// Waiting for accept result.
    WaitAcceptResult = 4,
    /// Sending confirm to server.
    SendConfirmQuery = 5,
    /// Waiting for confirm result.
    WaitConfirmResult = 6,
    /// Call is ready and active.
    Ready = 7,
    /// Sending discard to server.
    SendDiscardQuery = 8,
    /// Waiting for discard result.
    WaitDiscardResult = 9,
    /// Call is discarded.
    Discarded = 10,
}

impl Default for CallState {
    fn default() -> Self {
        Self::Empty
    }
}

impl CallState {
    /// Creates CallState from an i32 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallState;
    ///
    /// assert_eq!(CallState::from_i32(0), Ok(CallState::Empty));
    /// assert_eq!(CallState::from_i32(7), Ok(CallState::Ready));
    /// assert!(CallState::from_i32(99).is_err());
    /// ```
    pub const fn from_i32(value: i32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::SendRequestQuery),
            2 => Ok(Self::WaitRequestResult),
            3 => Ok(Self::SendAcceptQuery),
            4 => Ok(Self::WaitAcceptResult),
            5 => Ok(Self::SendConfirmQuery),
            6 => Ok(Self::WaitConfirmResult),
            7 => Ok(Self::Ready),
            8 => Ok(Self::SendDiscardQuery),
            9 => Ok(Self::WaitDiscardResult),
            10 => Ok(Self::Discarded),
            _ => Err(Error::InvalidState(value)),
        }
    }

    /// Returns the i32 representation of this CallState.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallState;
    ///
    /// assert_eq!(CallState::Empty.as_i32(), 0);
    /// assert_eq!(CallState::Ready.as_i32(), 7);
    /// ```
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Checks if this is a query state (sending request).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallState;
    ///
    /// assert!(CallState::SendRequestQuery.is_query());
    /// assert!(CallState::SendAcceptQuery.is_query());
    /// assert!(!CallState::Ready.is_query());
    /// ```
    #[must_use]
    pub const fn is_query(self) -> bool {
        matches!(
            self,
            Self::SendRequestQuery
                | Self::SendAcceptQuery
                | Self::SendConfirmQuery
                | Self::SendDiscardQuery
        )
    }

    /// Checks if this is a wait state (waiting for response).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallState;
    ///
    /// assert!(CallState::WaitRequestResult.is_waiting());
    /// assert!(CallState::WaitAcceptResult.is_waiting());
    /// assert!(!CallState::Ready.is_waiting());
    /// ```
    #[must_use]
    pub const fn is_waiting(self) -> bool {
        matches!(
            self,
            Self::WaitRequestResult
                | Self::WaitAcceptResult
                | Self::WaitConfirmResult
                | Self::WaitDiscardResult
        )
    }

    /// Checks if the call is active (ready or in progress).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallState;
    ///
    /// assert!(CallState::Ready.is_active());
    /// assert!(CallState::SendConfirmQuery.is_active());
    /// assert!(!CallState::Empty.is_active());
    /// ```
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(
            self,
            Self::SendRequestQuery
                | Self::WaitRequestResult
                | Self::SendAcceptQuery
                | Self::WaitAcceptResult
                | Self::SendConfirmQuery
                | Self::WaitConfirmResult
                | Self::Ready
        )
    }

    /// Checks if the call is terminal (ended).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallState;
    ///
    /// assert!(CallState::Discarded.is_terminal());
    /// assert!(!CallState::Ready.is_terminal());
    /// ```
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Discarded)
    }

    /// Returns all call state variants.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallState;
    ///
    /// let all = CallState::all();
    /// assert_eq!(all.len(), 11);
    /// ```
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::Empty,
            Self::SendRequestQuery,
            Self::WaitRequestResult,
            Self::SendAcceptQuery,
            Self::WaitAcceptResult,
            Self::SendConfirmQuery,
            Self::WaitConfirmResult,
            Self::Ready,
            Self::SendDiscardQuery,
            Self::WaitDiscardResult,
            Self::Discarded,
        ]
    }
}

impl fmt::Display for CallState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::SendRequestQuery => write!(f, "SendRequestQuery"),
            Self::WaitRequestResult => write!(f, "WaitRequestResult"),
            Self::SendAcceptQuery => write!(f, "SendAcceptQuery"),
            Self::WaitAcceptResult => write!(f, "WaitAcceptResult"),
            Self::SendConfirmQuery => write!(f, "SendConfirmQuery"),
            Self::WaitConfirmResult => write!(f, "WaitConfirmResult"),
            Self::Ready => write!(f, "Ready"),
            Self::SendDiscardQuery => write!(f, "SendDiscardQuery"),
            Self::WaitDiscardResult => write!(f, "WaitDiscardResult"),
            Self::Discarded => write!(f, "Discarded"),
        }
    }
}

/// Error type for CallActor operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid state transition
    InvalidTransition(CallState, CallState),
    /// Invalid state value
    InvalidState(i32),
    /// Invalid protocol settings
    InvalidProtocol(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition(from, to) => {
                write!(f, "Invalid state transition from {} to {}", from, to)
            }
            Self::InvalidState(value) => write!(f, "Invalid CallState value: {}", value),
            Self::InvalidProtocol(msg) => write!(f, "Invalid protocol: {}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Actor for managing Telegram calls.
///
/// The call actor manages the lifecycle of voice/video calls using
/// an actor pattern with explicit state transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallActor {
    /// The call ID.
    call_id: CallId,
    /// The user ID of the remote participant.
    user_id: UserId,
    /// Whether this is a video call.
    is_video: bool,
    /// Whether this is an outgoing call.
    is_outgoing: bool,
    /// Current state of the call.
    state: CallState,
    /// Call protocol settings.
    protocol: CallProtocol,
    /// Discard reason if call was discarded.
    discard_reason: Option<CallDiscardReason>,
    /// Duration of the call in seconds.
    duration: i32,
    /// Whether the call was accepted.
    is_accepted: bool,
}

impl Default for CallActor {
    fn default() -> Self {
        Self {
            call_id: CallId::new(0),
            user_id: UserId::from_i32(0),
            is_video: false,
            is_outgoing: false,
            state: CallState::Empty,
            protocol: CallProtocol::default(),
            discard_reason: None,
            duration: 0,
            is_accepted: false,
        }
    }
}

impl CallActor {
    /// Creates a new CallActor.
    ///
    /// # Arguments
    ///
    /// * `call_id` - The call ID
    /// * `user_id` - The user ID of the remote participant
    /// * `is_video` - Whether this is a video call
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let call_id = CallId::new(123);
    /// let user_id = UserId::from_i32(456);
    /// let actor = CallActor::new(call_id, user_id, true);
    /// ```
    #[must_use]
    pub fn new(call_id: CallId, user_id: UserId, is_video: bool) -> Self {
        Self {
            call_id,
            user_id,
            is_video,
            is_outgoing: false,
            state: CallState::Empty,
            protocol: CallProtocol::default(),
            discard_reason: None,
            duration: 0,
            is_accepted: false,
        }
    }

    /// Returns the call ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
    /// assert_eq!(actor.call_id().get(), 123);
    /// ```
    #[must_use]
    pub const fn call_id(&self) -> CallId {
        self.call_id
    }

    /// Returns the user ID of the remote participant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
    /// assert_eq!(actor.user_id().get(), 456);
    /// ```
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns whether this is a video call.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), true);
    /// assert!(actor.is_video());
    /// ```
    #[must_use]
    pub const fn is_video(&self) -> bool {
        self.is_video
    }

    /// Returns whether this is an outgoing call.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let mut actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
    /// actor = actor.with_outgoing(true);
    /// assert!(actor.is_outgoing());
    /// ```
    #[must_use]
    pub const fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    /// Returns the current state of the call.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
    /// assert_eq!(actor.state(), rustgram_call_actor::CallState::Empty);
    /// ```
    #[must_use]
    pub const fn state(&self) -> CallState {
        self.state
    }

    /// Returns the call protocol settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
    /// assert!(actor.protocol().udp_p2p);
    /// ```
    #[must_use]
    pub const fn protocol(&self) -> &CallProtocol {
        &self.protocol
    }

    /// Returns the discard reason if the call was discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::{CallActor, CallState};
    /// use rustgram_call_discard_reason::CallDiscardReason;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_discard_reason(CallDiscardReason::Missed);
    /// assert_eq!(actor.discard_reason(), Some(&CallDiscardReason::Missed));
    /// ```
    #[must_use]
    pub const fn discard_reason(&self) -> Option<&CallDiscardReason> {
        self.discard_reason.as_ref()
    }

    /// Returns the duration of the call in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_duration(60);
    /// assert_eq!(actor.duration(), 60);
    /// ```
    #[must_use]
    pub const fn duration(&self) -> i32 {
        self.duration
    }

    /// Returns whether the call was accepted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_accepted(true);
    /// assert!(actor.is_accepted());
    /// ```
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        self.is_accepted
    }

    /// Sets whether this is an outgoing call.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_outgoing(true);
    /// assert!(actor.is_outgoing());
    /// ```
    #[must_use]
    pub const fn with_outgoing(mut self, is_outgoing: bool) -> Self {
        self.is_outgoing = is_outgoing;
        self
    }

    /// Sets the call protocol settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::{CallActor, CallProtocol};
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let protocol = CallProtocol::new().with_udp_p2p(false);
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_protocol(protocol);
    /// assert!(!actor.protocol().udp_p2p);
    /// ```
    #[must_use]
    pub fn with_protocol(mut self, protocol: CallProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Sets the discard reason.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::{CallActor, CallState};
    /// use rustgram_call_discard_reason::CallDiscardReason;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_discard_reason(CallDiscardReason::Missed)
    ///     .with_state(CallState::Discarded);
    /// assert_eq!(actor.state(), CallState::Discarded);
    /// ```
    #[must_use]
    pub fn with_discard_reason(mut self, reason: CallDiscardReason) -> Self {
        self.discard_reason = Some(reason);
        self
    }

    /// Sets the duration of the call in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_duration(120);
    /// assert_eq!(actor.duration(), 120);
    /// ```
    #[must_use]
    pub const fn with_duration(mut self, duration: i32) -> Self {
        self.duration = duration;
        self
    }

    /// Sets whether the call was accepted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::CallActor;
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_accepted(true);
    /// assert!(actor.is_accepted());
    /// ```
    #[must_use]
    pub const fn with_accepted(mut self, accepted: bool) -> Self {
        self.is_accepted = accepted;
        self
    }

    /// Sets the state of the call.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::{CallActor, CallState};
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
    ///     .with_state(CallState::Ready);
    /// assert_eq!(actor.state(), CallState::Ready);
    /// ```
    #[must_use]
    pub const fn with_state(mut self, state: CallState) -> Self {
        self.state = state;
        self
    }

    /// Checks if the call is in a terminal state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::{CallActor, CallState};
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
    /// assert!(!actor.is_terminal());
    ///
    /// let actor = actor.with_state(CallState::Discarded);
    /// assert!(actor.is_terminal());
    /// ```
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Checks if the call is active.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_actor::{CallActor, CallState};
    /// use rustgram_call_id::CallId;
    /// use rustgram_types::UserId;
    ///
    /// let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
    /// assert!(!actor.is_active());
    ///
    /// let actor = actor.with_state(CallState::Ready);
    /// assert!(actor.is_active());
    /// ```
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.state.is_active()
    }
}

impl fmt::Display for CallActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CallActor(call_id={}, user_id={}, state={}, video={})",
            self.call_id.get(),
            self.user_id.get(),
            self.state,
            self.is_video
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // CallProtocol tests
    mod protocol_tests {
        use super::*;

        #[test]
        fn test_protocol_default() {
            let protocol = CallProtocol::default();
            assert!(protocol.udp_p2p);
            assert!(protocol.udp_reflector);
            assert_eq!(protocol.min_layer, 65);
            assert_eq!(protocol.max_layer, 65);
            assert!(protocol.library_versions.is_empty());
        }

        #[test]
        fn test_protocol_new() {
            let protocol = CallProtocol::new();
            assert!(protocol.udp_p2p);
        }

        #[test]
        fn test_protocol_with_udp_p2p() {
            let protocol = CallProtocol::new().with_udp_p2p(false);
            assert!(!protocol.udp_p2p);
        }

        #[test]
        fn test_protocol_with_udp_reflector() {
            let protocol = CallProtocol::new().with_udp_reflector(false);
            assert!(!protocol.udp_reflector);
        }

        #[test]
        fn test_protocol_with_layers() {
            let protocol = CallProtocol::new().with_layers(60, 70);
            assert_eq!(protocol.min_layer, 60);
            assert_eq!(protocol.max_layer, 70);
        }

        #[test]
        fn test_protocol_chain() {
            let protocol = CallProtocol::new()
                .with_udp_p2p(false)
                .with_udp_reflector(false)
                .with_layers(60, 70);
            assert!(!protocol.udp_p2p);
            assert!(!protocol.udp_reflector);
            assert_eq!(protocol.min_layer, 60);
        }
    }

    // CallState tests
    mod state_tests {
        use super::*;

        #[test]
        fn test_state_default() {
            assert_eq!(CallState::default(), CallState::Empty);
        }

        #[test]
        fn test_state_from_i32_valid() {
            assert_eq!(CallState::from_i32(0), Ok(CallState::Empty));
            assert_eq!(CallState::from_i32(7), Ok(CallState::Ready));
            assert_eq!(CallState::from_i32(10), Ok(CallState::Discarded));
        }

        #[test]
        fn test_state_from_i32_invalid() {
            assert!(CallState::from_i32(11).is_err());
            assert!(CallState::from_i32(-1).is_err());
        }

        #[test]
        fn test_state_as_i32() {
            assert_eq!(CallState::Empty.as_i32(), 0);
            assert_eq!(CallState::Ready.as_i32(), 7);
            assert_eq!(CallState::Discarded.as_i32(), 10);
        }

        #[test]
        fn test_state_is_query() {
            assert!(CallState::SendRequestQuery.is_query());
            assert!(CallState::SendAcceptQuery.is_query());
            assert!(CallState::SendConfirmQuery.is_query());
            assert!(CallState::SendDiscardQuery.is_query());
            assert!(!CallState::Ready.is_query());
        }

        #[test]
        fn test_state_is_waiting() {
            assert!(CallState::WaitRequestResult.is_waiting());
            assert!(CallState::WaitAcceptResult.is_waiting());
            assert!(CallState::WaitConfirmResult.is_waiting());
            assert!(CallState::WaitDiscardResult.is_waiting());
            assert!(!CallState::Ready.is_waiting());
        }

        #[test]
        fn test_state_is_active() {
            assert!(CallState::Ready.is_active());
            assert!(CallState::SendRequestQuery.is_active());
            assert!(!CallState::Empty.is_active());
            assert!(!CallState::Discarded.is_active());
        }

        #[test]
        fn test_state_is_terminal() {
            assert!(CallState::Discarded.is_terminal());
            assert!(!CallState::Ready.is_terminal());
        }

        #[test]
        fn test_state_all() {
            let all = CallState::all();
            assert_eq!(all.len(), 11);
        }

        #[test]
        fn test_state_display() {
            assert_eq!(format!("{}", CallState::Ready), "Ready");
            assert_eq!(format!("{}", CallState::Discarded), "Discarded");
        }
    }

    // CallActor tests
    mod actor_tests {
        use super::*;

        #[test]
        fn test_actor_new() {
            let call_id = CallId::new(123);
            let user_id = UserId::from_i32(456);
            let actor = CallActor::new(call_id, user_id, true);

            assert_eq!(actor.call_id().get(), 123);
            assert_eq!(actor.user_id().get(), 456);
            assert!(actor.is_video());
            assert!(!actor.is_outgoing());
            assert_eq!(actor.state(), CallState::Empty);
        }

        #[test]
        fn test_actor_default() {
            let actor = CallActor::default();
            assert_eq!(actor.call_id().get(), 0);
            assert_eq!(actor.state(), CallState::Empty);
        }

        #[test]
        fn test_actor_with_outgoing() {
            let actor =
                CallActor::new(CallId::new(123), UserId::from_i32(456), false).with_outgoing(true);
            assert!(actor.is_outgoing());
        }

        #[test]
        fn test_actor_with_protocol() {
            let protocol = CallProtocol::new().with_udp_p2p(false);
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
                .with_protocol(protocol);
            assert!(!actor.protocol().udp_p2p);
        }

        #[test]
        fn test_actor_with_state() {
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
                .with_state(CallState::Ready);
            assert_eq!(actor.state(), CallState::Ready);
        }

        #[test]
        fn test_actor_with_duration() {
            let actor =
                CallActor::new(CallId::new(123), UserId::from_i32(456), false).with_duration(120);
            assert_eq!(actor.duration(), 120);
        }

        #[test]
        fn test_actor_with_accepted() {
            let actor =
                CallActor::new(CallId::new(123), UserId::from_i32(456), false).with_accepted(true);
            assert!(actor.is_accepted());
        }

        #[test]
        fn test_actor_with_discard_reason() {
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false)
                .with_discard_reason(CallDiscardReason::Missed);
            assert_eq!(actor.discard_reason(), Some(&CallDiscardReason::Missed));
        }

        #[test]
        fn test_actor_is_terminal() {
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
            assert!(!actor.is_terminal());

            let actor = actor.with_state(CallState::Discarded);
            assert!(actor.is_terminal());
        }

        #[test]
        fn test_actor_is_active() {
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
            assert!(!actor.is_active());

            let actor = actor.with_state(CallState::Ready);
            assert!(actor.is_active());
        }

        #[test]
        fn test_actor_display() {
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), true);
            let display = format!("{}", actor);
            assert!(display.contains("CallActor"));
            assert!(display.contains("123"));
            assert!(display.contains("456"));
        }

        #[test]
        fn test_actor_clone() {
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
            let cloned = actor.clone();
            assert_eq!(actor, cloned);
        }

        #[test]
        fn test_actor_partial_eq() {
            let actor1 = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
            let actor2 = CallActor::new(CallId::new(123), UserId::from_i32(456), false);
            assert_eq!(actor1, actor2);

            let actor3 = CallActor::new(CallId::new(789), UserId::from_i32(456), false);
            assert_ne!(actor1, actor3);
        }

        #[test]
        fn test_actor_chain() {
            let actor = CallActor::new(CallId::new(123), UserId::from_i32(456), true)
                .with_outgoing(true)
                .with_duration(60)
                .with_accepted(true);
            assert!(actor.is_outgoing());
            assert!(actor.is_accepted());
            assert_eq!(actor.duration(), 60);
        }

        #[test]
        fn test_actor_send_sync() {
            fn assert_send<T: Send>() {}
            fn assert_sync<T: Sync>() {}
            assert_send::<CallActor>();
            assert_sync::<CallActor>();
        }
    }

    // Error tests
    mod error_tests {
        use super::*;

        #[test]
        fn test_error_display_invalid_transition() {
            let error = Error::InvalidTransition(CallState::Empty, CallState::Discarded);
            let display = format!("{}", error);
            assert!(display.contains("Invalid state transition"));
        }

        #[test]
        fn test_error_display_invalid_state() {
            let error = Error::InvalidState(99);
            assert_eq!(format!("{}", error), "Invalid CallState value: 99");
        }

        #[test]
        fn test_error_display_invalid_protocol() {
            let error = Error::InvalidProtocol("test error".to_string());
            assert_eq!(format!("{}", error), "Invalid protocol: test error");
        }
    }
}
