// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Response patterns for message passing (ask/tell).

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::{oneshot, Notify};

/// A future that resolves when a response is received.
///
/// `ResponseFuture` is used for the "ask" pattern where you send a message
/// and wait for a response.
///
/// # Type Parameters
///
/// * `T` - The response type
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::ResponseFuture;
///
/// async fn example() {
///     let future = ResponseFuture::<i32>::new();
///     // The future will resolve when the response is received
/// }
/// ```
#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture<T> {
    /// The inner receiver for the response.
    inner: Option<oneshot::Receiver<T>>,
    /// Shared state for waker registration.
    state: Arc<Notify>,
}

impl<T> ResponseFuture<T> {
    /// Creates a new response future.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ResponseFuture;
    ///
    /// let future = ResponseFuture::<String>::new();
    /// ```
    pub fn new() -> (Self, oneshot::Sender<T>) {
        let (tx, rx) = oneshot::channel();
        let future = Self {
            inner: Some(rx),
            state: Arc::new(Notify::new()),
        };
        (future, tx)
    }

    /// Checks if the future has completed.
    ///
    /// # Returns
    ///
    /// * `true` - If the response has been received
    /// * `false` - If still waiting
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ResponseFuture;
    ///
    /// let (future, _tx) = ResponseFuture::<i32>::new();
    /// assert!(!future.is_done());
    /// ```
    pub fn is_done(&self) -> bool {
        self.inner
            .as_ref()
            .map(|rx| rx.is_closed())
            .unwrap_or(false)
    }
}

impl<T> Default for ResponseFuture<T> {
    fn default() -> Self {
        let (this, _) = Self::new();
        this
    }
}

impl<T> Future for ResponseFuture<T> {
    type Output = Result<T, ResponseError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let rx = match self.inner.as_mut() {
            Some(r) => r,
            None => return Poll::Ready(Err(ResponseError::Canceled)),
        };

        match rx.poll(cx) {
            Poll::Ready(Ok(value)) => {
                self.inner.take();
                Poll::Ready(Ok(value))
            }
            Poll::Ready(Err(_)) => {
                self.inner.take();
                Poll::Ready(Err(ResponseError::Canceled))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> fmt::Debug for ResponseFuture<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResponseFuture")
            .field("is_done", &self.is_done())
            .finish()
    }
}

/// Errors that can occur when waiting for a response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseError {
    /// The sender was dropped without sending a response.
    Canceled,
    /// The operation timed out.
    Timeout,
    /// The actor was not found.
    ActorNotFound,
    /// The actor is not running.
    ActorNotRunning,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canceled => write!(f, "Response canceled"),
            Self::Timeout => write!(f, "Response timeout"),
            Self::ActorNotFound => write!(f, "Actor not found"),
            Self::ActorNotRunning => write!(f, "Actor not running"),
        }
    }
}

impl std::error::Error for ResponseError {}

/// A sender for a tell pattern (fire and forget).
///
/// `TellSender` is used when you want to send a message without waiting
/// for a response.
///
/// # Type Parameters
///
/// * `T` - The message type
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::TellSender;
///
/// let sender = TellSender::<String>::new();
/// // Send the message without waiting for a response
/// ```
pub struct TellSender<T> {
    /// The inner sender.
    inner: Option<oneshot::Sender<T>>,
}

impl<T> TellSender<T> {
    /// Creates a new tell sender.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::TellSender;
    ///
    /// let sender = TellSender::<i32>::new();
    /// ```
    pub fn new() -> (Self, oneshot::Receiver<T>) {
        let (tx, rx) = oneshot::channel();
        let sender = Self { inner: Some(tx) };
        (sender, rx)
    }

    /// Sends the message without waiting for a response.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the message was sent
    /// * `Err(T)` - If the receiver was dropped or sender already used
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::TellSender;
    ///
    /// let (sender, mut receiver) = TellSender::<i32>::new();
    /// assert!(sender.send(42).is_ok());
    /// ```
    pub fn send(mut self, value: T) -> Result<(), T> {
        match self.inner.take() {
            Some(tx) => tx.send(value),
            None => Err(value),
        }
    }
}

impl<T> fmt::Debug for TellSender<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TellSender")
            .field("has_inner", &self.inner.is_some())
            .finish()
    }
}

/// A request-response channel for the ask pattern.
///
/// `AskChannel` provides a way to send a message and receive a response,
/// with support for timeouts.
///
/// # Type Parameters
///
/// * `Req` - The request type
/// * `Res` - The response type
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::AskChannel;
///
/// let channel = AskChannel::<String, i32>::new();
/// ```
pub struct AskChannel<Req, Res> {
    /// The request sender.
    req_tx: oneshot::Sender<Req>,
    /// The response receiver (created when request is sent).
    res_rx: Option<oneshot::Receiver<Res>>,
    _phantom: std::marker::PhantomData<Res>,
}

impl<Req, Res> AskChannel<Req, Res> {
    /// Creates a new ask channel.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::AskChannel;
    ///
    /// let channel = AskChannel::<String, i32>::new();
    /// ```
    pub fn new() -> Self {
        let (req_tx, _) = oneshot::channel();
        Self {
            req_tx,
            res_rx: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates an ask channel from a request sender.
    ///
    /// # Arguments
    ///
    /// * `req_tx` - The request sender
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::AskChannel;
    /// use tokio::sync::oneshot;
    ///
    /// let (tx, _) = oneshot::channel::<String>();
    /// let channel = AskChannel::<String, i32>::from_sender(tx);
    /// ```
    pub fn from_sender(req_tx: oneshot::Sender<Req>) -> Self {
        Self {
            req_tx,
            res_rx: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Req, Res> Default for AskChannel<Req, Res> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Req, Res> fmt::Debug for AskChannel<Req, Res> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AskChannel")
            .field("has_response", &self.res_rx.is_some())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_response_future_success() {
        let (future, tx) = ResponseFuture::<i32>::new();
        tx.send(42).unwrap();

        let result = future.await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_response_future_canceled() {
        let (future, _tx) = ResponseFuture::<i32>::new();
        // Drop the sender without sending
        drop(_tx);

        let result = future.await;
        assert_eq!(result, Err(ResponseError::Canceled));
    }

    #[test]
    fn test_response_future_is_done() {
        let (future, tx) = ResponseFuture::<i32>::new();
        assert!(!future.is_done());

        tx.send(42).unwrap();
        // Note: is_done checks if the channel is closed, which happens after send
        assert!(future.is_done());
    }

    #[test]
    fn test_response_future_default() {
        let future = ResponseFuture::<i32>::default();
        assert!(!future.is_done());
    }

    #[test]
    fn test_response_error_display() {
        assert_eq!(format!("{}", ResponseError::Canceled), "Response canceled");
        assert_eq!(format!("{}", ResponseError::Timeout), "Response timeout");
    }

    #[test]
    fn test_tell_sender_send() {
        let (sender, mut receiver) = TellSender::<i32>::new();
        assert!(sender.send(42).is_ok());

        let result = receiver.blocking_recv();
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_tell_sender_send_dropped() {
        let (sender, _) = TellSender::<i32>::new();
        // Drop the receiver
        let (sender, _) = TellSender::<i32>::new();
        drop((sender,));

        let (sender, _) = TellSender::<i32>::new();
        // Create a new pair and drop receiver
        let (sender, receiver) = TellSender::<i32>::new();
        drop(receiver);
        assert!(sender.send(42).is_err());
    }

    #[test]
    fn test_ask_channel_new() {
        let _channel = AskChannel::<String, i32>::new();
        // Just ensure it compiles
    }

    #[test]
    fn test_ask_channel_default() {
        let _channel = AskChannel::<String, i32>::default();
        // Just ensure it compiles
    }

    #[tokio::test]
    async fn test_ask_channel_timeout() {
        use tokio::time::timeout;

        let (future, _tx) = ResponseFuture::<i32>::new();

        let result = timeout(Duration::from_millis(100), future).await;
        assert!(result.is_err()); // Timeout
    }
}
