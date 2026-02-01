// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Request Actor
//!
//! Generic actor for handling async requests with retry logic.
//!
//! This module provides the RequestActor which handles async requests with
//! automatic retry functionality. Based on TDLib's RequestActor.h.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_actor::{Actor, ActorShared};
use rustgram_promise::{FutureActor, Promise};
use rustgram_td::Td;
use std::fmt::{self, Debug, Formatter};

/// Actor for handling async requests with retry logic.
///
/// Manages request lifecycle, retries on failure, and result delivery.
///
/// # Type Parameters
///
/// * `T` - The result type of the request
///
/// # TODO
///
/// This is a simplified implementation. Full TDLib RequestActor has:
/// - More sophisticated retry logic
/// - Timeout handling
/// - Error categorization
/// - Request cancellation
pub struct RequestActor<T> {
    /// Shared actor reference to Td
    td_id: ActorShared<Td>,

    /// Request ID for tracking
    request_id: u64,

    /// Number of retry attempts remaining
    tries_left: i32,

    /// Pending future for the request
    future: Option<FutureActor<T>>,
}

impl<T> RequestActor<T> {
    /// Default number of retry attempts.
    pub const DEFAULT_TRIES: i32 = 2;

    /// Creates a new RequestActor.
    ///
    /// # Arguments
    ///
    /// * `td_id` - Shared actor reference to Td
    /// * `request_id` - Request ID for tracking
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// struct MyActor;
    ///
    /// let td_id = ActorShared::<MyActor>::default();
    /// let actor = RequestActor::<String>::new(td_id, 123);
    /// ```
    pub fn new(td_id: ActorShared<Td>, request_id: u64) -> Self {
        Self {
            td_id,
            request_id,
            tries_left: Self::DEFAULT_TRIES,
            future: None,
        }
    }

    /// Returns the request ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let actor = RequestActor::<String>::new(td_id, 456);
    /// assert_eq!(actor.request_id(), 456);
    /// ```
    pub fn request_id(&self) -> u64 {
        self.request_id
    }

    /// Returns the remaining retry count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let actor = RequestActor::<String>::new(td_id, 1);
    /// assert_eq!(actor.get_tries(), 2);
    /// ```
    pub fn get_tries(&self) -> i32 {
        self.tries_left
    }

    /// Sets the retry count.
    ///
    /// # Arguments
    ///
    /// * `tries` - Number of retry attempts
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let mut actor = RequestActor::<String>::new(td_id, 1);
    /// actor.set_tries(5);
    /// assert_eq!(actor.get_tries(), 5);
    /// ```
    pub fn set_tries(&mut self, tries: i32) {
        self.tries_left = tries;
    }

    /// Returns the Td actor shared reference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let actor = RequestActor::<String>::new(td_id, 1);
    /// assert!(actor.td_id().id().is_zero());
    /// ```
    pub fn td_id(&self) -> &ActorShared<Td> {
        &self.td_id
    }

    /// Returns the pending future if available.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let actor = RequestActor::<String>::new(td_id, 1);
    /// assert!(actor.get_future().is_none());
    /// ```
    pub fn get_future(&self) -> Option<&FutureActor<T>> {
        self.future.as_ref()
    }

    /// Sets the future for this request.
    ///
    /// # Arguments
    ///
    /// * `future` - The future to set
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    /// use rustgram_promise::FutureActor;
    ///
    /// let td_id = ActorShared::default();
    /// let mut actor = RequestActor::<String>::new(td_id, 1);
    /// let future = FutureActor::new();
    /// actor.set_future(future);
    /// assert!(actor.get_future().is_some());
    /// ```
    pub fn set_future(&mut self, future: FutureActor<T>) {
        self.future = Some(future);
    }

    /// Decrements the retry count.
    ///
    /// # Returns
    ///
    /// Returns the new retry count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let mut actor = RequestActor::<String>::new(td_id, 1);
    /// let remaining = actor.decrement_tries();
    /// assert_eq!(remaining, 1);
    /// ```
    pub fn decrement_tries(&mut self) -> i32 {
        self.tries_left = self.tries_left.saturating_sub(1);
        self.tries_left
    }

    /// Checks if there are retry attempts remaining.
    ///
    /// # Returns
    ///
    /// Returns `true` if retries are available, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let mut actor = RequestActor::<String>::new(td_id, 1);
    /// assert!(actor.has_tries());
    /// actor.set_tries(0);
    /// assert!(!actor.has_tries());
    /// ```
    pub fn has_tries(&self) -> bool {
        self.tries_left > 0
    }

    /// Clears the pending future.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    /// use rustgram_promise::FutureActor;
    ///
    /// let td_id = ActorShared::default();
    /// let mut actor = RequestActor::<String>::new(td_id, 1);
    /// actor.set_future(FutureActor::new());
    /// actor.clear_future();
    /// assert!(actor.get_future().is_none());
    /// ```
    pub fn clear_future(&mut self) {
        self.future = None;
    }

    /// Checks if the request has a pending future.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let actor = RequestActor::<String>::new(td_id, 1);
    /// assert!(!actor.has_future());
    /// ```
    pub fn has_future(&self) -> bool {
        self.future.is_some()
    }

    /// Sends a result to the promise.
    ///
    /// # TODO
    ///
    /// Implement actual result delivery to promise.
    ///
    /// # Arguments
    ///
    /// * `result` - The result to send
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let mut actor = RequestActor::<String>::new(td_id, 1);
    /// actor.send_result("success".to_string());
    /// ```
    pub fn send_result(&mut self, _result: T) {
        // Stub: Would send result to promise
        self.clear_future();
    }

    /// Sends an error to the promise.
    ///
    /// # TODO
    ///
    /// Implement actual error delivery to promise.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to send
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let mut actor = RequestActor::<String>::new(td_id, 1);
    /// actor.send_error("request failed");
    /// ```
    pub fn send_error<E>(&mut self, _error: E)
    where
        E: fmt::Display,
    {
        // Stub: Would send error to promise
        self.clear_future();
    }
}

impl<T> Actor for RequestActor<T> {}

impl<T> Debug for RequestActor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestActor")
            .field("request_id", &self.request_id)
            .field("tries_left", &self.tries_left)
            .field("has_future", &self.has_future())
            .finish()
    }
}

impl<T> Clone for RequestActor<T> {
    fn clone(&self) -> Self {
        Self {
            td_id: self.td_id.clone(),
            request_id: self.request_id,
            tries_left: self.tries_left,
            future: self.future.clone(),
        }
    }
}

/// Actor that runs only once without retry.
///
/// Simplified version of RequestActor for idempotent operations.
///
/// # Type Parameters
///
/// * `T` - The result type
///
/// # Example
///
/// ```rust
/// use rustgram_request_actor::RequestOnceActor;
/// use rustgram_actor::ActorShared;
///
/// let td_id = ActorShared::default();
/// let actor = RequestOnceActor::<String>::new(td_id, 1);
/// assert_eq!(actor.get_tries(), 1); // Only one try
/// ```
#[derive(Debug, Clone)]
pub struct RequestOnceActor<T> {
    /// Inner request actor with single try
    inner: RequestActor<T>,
}

impl<T> RequestOnceActor<T> {
    /// Creates a new RequestOnceActor.
    ///
    /// # Arguments
    ///
    /// * `td_id` - Shared actor reference to Td
    /// * `request_id` - Request ID for tracking
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_request_actor::RequestOnceActor;
    /// use rustgram_actor::ActorShared;
    ///
    /// let td_id = ActorShared::default();
    /// let actor = RequestOnceActor::<String>::new(td_id, 789);
    /// assert_eq!(actor.request_id(), 789);
    /// ```
    pub fn new(td_id: ActorShared<Td>, request_id: u64) -> Self {
        let mut inner = RequestActor::new(td_id, request_id);
        inner.set_tries(1); // Only one try
        Self { inner }
    }

    /// Returns the request ID.
    pub fn request_id(&self) -> u64 {
        self.inner.request_id()
    }

    /// Returns the remaining retry count (always 1).
    pub fn get_tries(&self) -> i32 {
        self.inner.get_tries()
    }

    /// Returns the Td actor shared reference.
    pub fn td_id(&self) -> &ActorShared<Td> {
        self.inner.td_id()
    }

    /// Returns the pending future if available.
    pub fn get_future(&self) -> Option<&FutureActor<T>> {
        self.inner.get_future()
    }

    /// Sets the future for this request.
    pub fn set_future(&mut self, future: FutureActor<T>) {
        self.inner.set_future(future);
    }

    /// Clears the pending future.
    pub fn clear_future(&mut self) {
        self.inner.clear_future();
    }

    /// Checks if the request has a pending future.
    pub fn has_future(&self) -> bool {
        self.inner.has_future()
    }

    /// Sends a result to the promise.
    pub fn send_result(&mut self, result: T) {
        self.inner.send_result(result);
    }

    /// Sends an error to the promise.
    pub fn send_error<E>(&mut self, error: E)
    where
        E: fmt::Display,
    {
        self.inner.send_error(error);
    }
}

impl<T> Actor for RequestOnceActor<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_promise::FutureActor;

    fn create_test_actor<T>() -> RequestActor<T> {
        let td_id = ActorShared::default();
        RequestActor::new(td_id, 123)
    }

    fn create_test_once_actor<T>() -> RequestOnceActor<T> {
        let td_id = ActorShared::default();
        RequestOnceActor::new(td_id, 456)
    }

    #[test]
    fn test_request_actor_new() {
        let actor = create_test_actor::<String>();
        assert_eq!(actor.request_id(), 123);
        assert_eq!(actor.get_tries(), 2);
        assert!(actor.has_tries());
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_actor_request_id() {
        let actor = create_test_actor::<i32>();
        assert_eq!(actor.request_id(), 123);
    }

    #[test]
    fn test_request_actor_get_tries() {
        let actor = create_test_actor::<()>();
        assert_eq!(actor.get_tries(), 2);
    }

    #[test]
    fn test_request_actor_set_tries() {
        let mut actor = create_test_actor::<()>();
        actor.set_tries(5);
        assert_eq!(actor.get_tries(), 5);

        actor.set_tries(0);
        assert_eq!(actor.get_tries(), 0);
    }

    #[test]
    fn test_request_actor_td_id() {
        let actor = create_test_actor::<()>();
        assert!(actor.td_id().id().is_zero());
    }

    #[test]
    fn test_request_actor_get_future() {
        let actor = create_test_actor::<String>();
        assert!(actor.get_future().is_none());
    }

    #[test]
    fn test_request_actor_set_future() {
        let mut actor = create_test_actor::<String>();
        let future = FutureActor::new();
        actor.set_future(future);
        assert!(actor.get_future().is_some());
    }

    #[test]
    fn test_request_actor_decrement_tries() {
        let mut actor = create_test_actor::<()>();
        assert_eq!(actor.decrement_tries(), 1);
        assert_eq!(actor.decrement_tries(), 0);
        assert_eq!(actor.decrement_tries(), 0); // Stays at 0
    }

    #[test]
    fn test_request_actor_has_tries() {
        let mut actor = create_test_actor::<()>();
        assert!(actor.has_tries());

        actor.decrement_tries();
        assert!(actor.has_tries());

        actor.decrement_tries();
        assert!(!actor.has_tries());
    }

    #[test]
    fn test_request_actor_clear_future() {
        let mut actor = create_test_actor::<String>();
        actor.set_future(FutureActor::new());
        assert!(actor.has_future());

        actor.clear_future();
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_actor_has_future() {
        let mut actor = create_test_actor::<String>();
        assert!(!actor.has_future());

        actor.set_future(FutureActor::new());
        assert!(actor.has_future());

        actor.clear_future();
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_actor_send_result() {
        let mut actor = create_test_actor::<String>();
        actor.set_future(FutureActor::new());
        actor.send_result("result".to_string());
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_actor_send_error() {
        let mut actor = create_test_actor::<String>();
        actor.set_future(FutureActor::new());
        actor.send_error("error");
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_actor_clone() {
        let mut actor1 = create_test_actor::<String>();
        actor1.set_tries(5);
        actor1.set_future(FutureActor::new());

        let actor2 = actor1.clone();
        assert_eq!(actor2.request_id(), 123);
        assert_eq!(actor2.get_tries(), 5);
        assert!(actor2.has_future());
    }

    #[test]
    fn test_request_actor_debug() {
        let actor = create_test_actor::<String>();
        let debug_str = format!("{:?}", actor);
        assert!(debug_str.contains("RequestActor"));
        assert!(debug_str.contains("123"));
    }

    #[test]
    fn test_request_once_actor_new() {
        let actor = create_test_once_actor::<String>();
        assert_eq!(actor.request_id(), 456);
        assert_eq!(actor.get_tries(), 1); // Only one try
    }

    #[test]
    fn test_request_once_actor_single_try() {
        let actor = create_test_once_actor::<()>();
        assert_eq!(actor.get_tries(), 1);
    }

    #[test]
    fn test_request_once_actor_request_id() {
        let actor = create_test_once_actor::<i32>();
        assert_eq!(actor.request_id(), 456);
    }

    #[test]
    fn test_request_once_actor_td_id() {
        let actor = create_test_once_actor::<()>();
        assert!(actor.td_id().id().is_zero());
    }

    #[test]
    fn test_request_once_actor_future_operations() {
        let mut actor = create_test_once_actor::<String>();

        assert!(!actor.has_future());
        assert!(actor.get_future().is_none());

        actor.set_future(FutureActor::new());
        assert!(actor.has_future());
        assert!(actor.get_future().is_some());

        actor.clear_future();
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_once_actor_send_result() {
        let mut actor = create_test_once_actor::<String>();
        actor.set_future(FutureActor::new());
        actor.send_result("done".to_string());
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_once_actor_send_error() {
        let mut actor = create_test_once_actor::<String>();
        actor.set_future(FutureActor::new());
        actor.send_error("failed");
        assert!(!actor.has_future());
    }

    #[test]
    fn test_request_once_actor_clone() {
        let actor1 = create_test_once_actor::<String>();
        let actor2 = actor1.clone();
        assert_eq!(actor2.request_id(), 456);
        assert_eq!(actor2.get_tries(), 1);
    }

    #[test]
    fn test_request_once_actor_debug() {
        let actor = create_test_once_actor::<String>();
        let debug_str = format!("{:?}", actor);
        assert!(debug_str.contains("RequestOnceActor"));
    }

    #[test]
    fn test_request_actor_default_tries() {
        assert_eq!(RequestActor::<()>::DEFAULT_TRIES, 2);
    }

    #[test]
    fn test_request_actor_with_different_result_types() {
        let actor1 = create_test_actor::<String>();
        let actor2 = create_test_actor::<i32>();
        let actor3 = create_test_actor::<Vec<u8>>();

        assert_eq!(actor1.request_id(), 123);
        assert_eq!(actor2.request_id(), 123);
        assert_eq!(actor3.request_id(), 123);
    }

    #[test]
    fn test_request_actor_multiple_decrements() {
        let mut actor = create_test_actor::<()>();
        actor.set_tries(10);

        for _ in 0..9 {
            assert!(actor.has_tries());
            actor.decrement_tries();
        }

        assert!(actor.has_tries());
        actor.decrement_tries();
        assert!(!actor.has_tries());
    }

    #[test]
    fn test_request_actor_zero_tries() {
        let mut actor = create_test_actor::<()>();
        actor.set_tries(0);
        assert!(!actor.has_tries());
        assert_eq!(actor.decrement_tries(), 0);
    }

    #[test]
    fn test_request_actor_retry_simulation() {
        let mut actor = create_test_actor::<String>();
        actor.set_future(FutureActor::new());

        // Simulate first try failing
        actor.send_error("error 1");
        assert!(actor.has_tries());

        // Retry
        actor.set_future(FutureActor::new());
        actor.send_error("error 2");
        assert!(!actor.has_tries()); // No more retries

        // Final attempt would fail with error 500
        actor.set_future(FutureActor::new());
        actor.send_result("final result");
    }

    #[test]
    fn test_request_once_actor_no_retry() {
        let mut actor = create_test_once_actor::<String>();
        actor.set_future(FutureActor::new());

        // Only one try allowed
        actor.send_error("error");
        assert!(!actor.has_future());
        assert_eq!(actor.get_tries(), 1); // But tries_left stays at 1 for RequestOnceActor
    }

    #[test]
    fn test_request_actor_various_request_ids() {
        let td_id = ActorShared::default();

        let actor1 = RequestActor::<()>::new(td_id.clone(), 1);
        let actor2 = RequestActor::<()>::new(td_id.clone(), 999);
        let actor3 = RequestActor::<()>::new(td_id, 0);

        assert_eq!(actor1.request_id(), 1);
        assert_eq!(actor2.request_id(), 999);
        assert_eq!(actor3.request_id(), 0);
    }

    #[test]
    fn test_request_actor_future_replacement() {
        let mut actor = create_test_actor::<String>();

        actor.set_future(FutureActor::new());
        assert!(actor.has_future());

        // Replace with new future
        actor.set_future(FutureActor::new());
        assert!(actor.has_future());

        actor.clear_future();
        assert!(!actor.has_future());
    }
}
