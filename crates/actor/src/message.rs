// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Core message types for the actor framework.
//!
//! This module defines the message trait and related types for type-safe
//! message passing between actors.

use std::any::{Any, TypeId};
use std::fmt;

/// Base trait for messages that can be sent between actors.
///
/// # Type Parameters
///
/// * `Response` - The type of value returned when handling this message
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::Message;
///
/// struct Ping {
///     from: String,
/// }
///
/// impl Message for Ping {
///     type Response = Pong;
/// }
///
/// struct Pong {
///     to: String,
/// }
/// ```
pub trait Message: Send + 'static {
    /// The response type returned when this message is handled.
    type Response: Send + 'static;
}

/// A message that doesn't expect a response.
///
/// This is a marker trait for messages that are "fire and forget".
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::NoResponse;
///
/// struct LogMessage {
///     level: String,
///     message: String,
/// }
///
/// impl Message for LogMessage {
///     type Response = NoResponse;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoResponse {}

/// Type-erased message wrapper for dynamic dispatch.
///
/// `Envelope` allows messages of different types to be stored and processed
/// through a common interface, while maintaining type safety through
/// downcasting.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::{Envelope, Message, NoResponse};
///
/// struct MyMessage;
///
/// impl Message for MyMessage {
///     type Response = NoResponse;
/// }
///
/// let envelope = Envelope::new(MyMessage);
/// ```
pub struct Envelope {
    /// The type ID of the contained message.
    type_id: TypeId,
    /// The boxed message data.
    data: Box<dyn Any + Send>,
}

impl Envelope {
    /// Creates a new envelope containing the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to wrap
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Envelope;
    ///
    /// struct MyMessage;
    ///
    /// let envelope = Envelope::new(MyMessage);
    /// ```
    pub fn new<M: Message>(message: M) -> Self {
        Self {
            type_id: TypeId::of::<M>(),
            data: Box::new(message),
        }
    }

    /// Attempts to downcast the envelope to a specific message type.
    ///
    /// # Returns
    ///
    /// * `Some(M)` - If the envelope contains a message of type `M`
    /// * `None` - If the envelope contains a different message type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Envelope;
    ///
    /// struct MyMessage;
    ///
    /// let envelope = Envelope::new(MyMessage);
    /// assert!(envelope.is::<MyMessage>());
    /// ```
    pub fn downcast<M: Message>(self) -> Option<M> {
        if self.type_id == TypeId::of::<M>() {
            self.data.downcast::<M>().ok().map(|b| *b)
        } else {
            None
        }
    }

    /// Checks if the envelope contains a specific message type.
    ///
    /// # Returns
    ///
    /// * `true` - If the envelope contains a message of type `M`
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Envelope;
    ///
    /// struct MyMessage;
    /// struct OtherMessage;
    ///
    /// let envelope = Envelope::new(MyMessage);
    /// assert!(envelope.is::<MyMessage>());
    /// assert!(!envelope.is::<OtherMessage>());
    /// ```
    pub fn is<M: Message>(&self) -> bool {
        self.type_id == TypeId::of::<M>()
    }

    /// Returns the type ID of the contained message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Envelope;
    /// use std::any::TypeId;
    ///
    /// struct MyMessage;
    ///
    /// let envelope = Envelope::new(MyMessage);
    /// assert_eq!(envelope.type_id(), TypeId::of::<MyMessage>());
    /// ```
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl fmt::Debug for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Envelope")
            .field("type_id", &self.type_id)
            .finish()
    }
}

/// A response wrapper for async message handling.
///
/// `Response` handles the result of message processing, supporting both
/// synchronous and asynchronous response patterns.
///
/// # Type Parameters
///
/// * `T` - The response value type
#[derive(Debug)]
pub enum Response<T> {
    /// The message was processed successfully.
    Ok(T),
    /// The actor was not found.
    ActorNotFound,
    /// The actor is not running.
    ActorNotRunning,
    /// The operation timed out.
    Timeout,
    /// An internal error occurred.
    Error(String),
}

impl<T> Response<T> {
    /// Creates a successful response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Response;
    ///
    /// let response = Response::ok(42);
    /// assert!(response.is_ok());
    /// ```
    pub fn ok(value: T) -> Self {
        Self::Ok(value)
    }

    /// Checks if the response is successful.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Response;
    ///
    /// let response = Response::<i32>::ok(42);
    /// assert!(response.is_ok());
    /// ```
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    /// Checks if the response is an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Response;
    ///
    /// let response = Response::<i32>::ActorNotFound;
    /// assert!(response.is_err());
    /// ```
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Converts the response into a `Result`.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - If the response is successful
    /// * `Err(ResponseError)` - If the response is an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Response;
    ///
    /// let response = Response::ok(42);
    /// assert_eq!(response.into_result(), Ok(42));
    /// ```
    pub fn into_result(self) -> Result<T, ResponseError> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::ActorNotFound => Err(ResponseError::ActorNotFound),
            Self::ActorNotRunning => Err(ResponseError::ActorNotRunning),
            Self::Timeout => Err(ResponseError::Timeout),
            Self::Error(msg) => Err(ResponseError::Other(msg)),
        }
    }
}

impl<T> Clone for Response<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Ok(v) => Self::Ok(v.clone()),
            Self::ActorNotFound => Self::ActorNotFound,
            Self::ActorNotRunning => Self::ActorNotRunning,
            Self::Timeout => Self::Timeout,
            Self::Error(msg) => Self::Error(msg.clone()),
        }
    }
}

/// Errors that can occur when processing a message response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseError {
    /// The actor was not found.
    ActorNotFound,
    /// The actor is not running.
    ActorNotRunning,
    /// The operation timed out.
    Timeout,
    /// Another error occurred.
    Other(String),
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ActorNotFound => write!(f, "Actor not found"),
            Self::ActorNotRunning => write!(f, "Actor not running"),
            Self::Timeout => write!(f, "Operation timed out"),
            Self::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ResponseError {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMessage;
    struct OtherMessage;

    impl Message for TestMessage {
        type Response = NoResponse;
    }

    impl Message for OtherMessage {
        type Response = NoResponse;
    }

    #[test]
    fn test_envelope_new() {
        let envelope = Envelope::new(TestMessage);
        assert!(envelope.is::<TestMessage>());
    }

    #[test]
    fn test_envelope_downcast() {
        let envelope = Envelope::new(TestMessage);
        let result = envelope.downcast::<TestMessage>();
        assert!(result.is_some());
    }

    #[test]
    fn test_envelope_downcast_wrong_type() {
        let envelope = Envelope::new(TestMessage);
        let result = envelope.downcast::<OtherMessage>();
        assert!(result.is_none());
    }

    #[test]
    fn test_envelope_is() {
        let envelope = Envelope::new(TestMessage);
        assert!(envelope.is::<TestMessage>());
        assert!(!envelope.is::<OtherMessage>());
    }

    #[test]
    fn test_envelope_type_id() {
        let envelope = Envelope::new(TestMessage);
        assert_eq!(envelope.type_id(), TypeId::of::<TestMessage>());
    }

    #[test]
    fn test_response_ok() {
        let response = Response::ok(42);
        assert!(response.is_ok());
        assert!(!response.is_err());
    }

    #[test]
    fn test_response_into_result() {
        let response = Response::ok(42);
        let result = response.into_result();
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_response_error() {
        let response = Response::<i32>::ActorNotFound;
        assert!(response.is_err());
        assert_eq!(
            response.into_result(),
            Err(ResponseError::ActorNotFound)
        );
    }

    #[test]
    fn test_response_clone() {
        let response = Response::ok(42);
        let cloned = response.clone();
        assert!(cloned.is_ok());
    }

    #[test]
    fn test_response_error_display() {
        let error = ResponseError::Timeout;
        assert_eq!(format!("{}", error), "Operation timed out");
    }

    #[test]
    fn test_no_response() {
        let no_response = NoResponse;
        // Just ensure it compiles and can be created
        let _ = no_response;
    }
}
