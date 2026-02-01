// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Actor Framework for rustgram-client
//!
//! This crate provides a TDLib-compatible actor framework for building
//! concurrent, message-based applications in Rust.
//!
//! ## Overview
//!
//! The actor framework provides:
//! - Type-safe message passing between actors
//! - Actor lifecycle management
//! - Supervisor patterns for fault tolerance
//! - Hierarchical actor registry
//! - Multi-scheduler support for parallel execution
//!
//! ## Core Concepts
//!
//! ### Actors
//!
//! An actor is an independent entity that processes messages sequentially.
//! Each actor has:
//! - A unique ID ([`ActorId`])
//! - A mailbox for message queuing ([`Mailbox`])
//! - State and behavior defined by implementing the [`Actor`] trait
//!
//! ### Message Passing
//!
//! Actors communicate through asynchronous message passing:
//! - **Tell pattern**: Send a message without waiting for a response ([`TellSender`])
//! - **Ask pattern**: Send a message and await a response ([`ResponseFuture`])
//!
//! ### Supervision
//!
//! Actors can be supervised to handle failures:
//! - **OneForOne**: Restart only the failed child
//! - **OneForAll**: Restart all children when one fails
//! - **Escalate**: Pass failure up to parent supervisor
//! - **Stop**: Stop the failed child
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_actor::{Actor, ActorId, ActorInfo, ActorState};
//!
//! struct MyActor {
//!     count: i32,
//! }
//!
//! impl Actor for MyActor {
//!     fn start_up(&mut self) {
//!         println!("Actor starting");
//!     }
//!
//!     fn wakeup(&mut self) {
//!         println!("Actor woke up");
//!     }
//!
//!     fn hangup(&mut self) {
//!         println!("Actor hung up");
//!     }
//!
//!     fn tear_down(&mut self) {
//!         println!("Actor tearing down");
//!     }
//!
//!     fn loop_exec(&mut self) {
//!         // Main event loop
//!     }
//!
//!     fn timeout_expired(&mut self) {
//!         println!("Timeout expired");
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

// Core modules
pub mod actor;
pub mod actor_id;
pub mod actor_info;
pub mod error;
pub mod event;
pub mod mailbox;
pub mod message;
pub mod registry;
pub mod response;
pub mod supervisor;

// Re-exports for convenience
pub use actor::{Actor, ActorExt, ActorHandle};
pub use actor_id::ActorId;
pub use actor_info::{ActorInfo, ActorState};
pub use error::{ActorError, Result};
pub use event::{Event, ActorTrait as EventActorTrait, EventFull};
pub use mailbox::Mailbox;
pub use message::{Envelope, Message, NoResponse, Response, ResponseError as MessageResponseError};
pub use registry::{Registry, SharedRegistry};
pub use response::{AskChannel, ResponseError, ResponseFuture, TellSender};
pub use supervisor::{
    Supervisor, SupervisorDirective, SupervisorStrategy, SupervisedChild,
};

// ActorShared and ActorOwn for backward compatibility

/// Shared reference to an actor.
///
/// Provides shared access to an actor for sending messages.
///
/// # Type Parameters
///
/// * `T` - The actor type
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::{ActorShared, ActorId};
///
/// struct MyActor;
///
/// let id = ActorId::<MyActor>::new(123, 0, 0);
/// let shared = ActorShared::new(id);
/// ```
#[derive(Debug, Clone)]
pub struct ActorShared<T> {
    /// The actor ID being shared
    id: ActorId<T>,
}

impl<T> ActorShared<T> {
    /// Creates a new ActorShared from an ActorId.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID to wrap
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{ActorShared, ActorId};
    ///
    /// struct MyActor;
    ///
    /// let id = ActorId::<MyActor>::new(123, 0, 0);
    /// let shared = ActorShared::new(id);
    /// assert_eq!(shared.id().as_u64(), 123);
    /// ```
    pub fn new(id: ActorId<T>) -> Self {
        Self { id }
    }

    /// Returns the actor ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{ActorShared, ActorId};
    ///
    /// struct MyActor;
    ///
    /// let id = ActorId::<MyActor>::new(456, 0, 0);
    /// let shared = ActorShared::new(id);
    /// assert_eq!(shared.id().as_u64(), 456);
    /// ```
    pub fn id(&self) -> ActorId<T> {
        self.id
    }
}

/// Owning reference to an actor (sends hangup on drop).
///
/// # Type Parameters
///
/// * `T` - The actor type
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::{ActorOwn, ActorId};
///
/// struct MyActor;
///
/// let id = ActorId::<MyActor>::new(123, 0, 0);
/// let own = ActorOwn::new(id);
/// ```
#[derive(Debug, Clone)]
pub struct ActorOwn<T> {
    /// The actor ID being owned
    id: ActorId<T>,
}

impl<T> ActorOwn<T> {
    /// Creates a new ActorOwn from an ActorId.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID to wrap
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{ActorOwn, ActorId};
    ///
    /// struct MyActor;
    ///
    /// let id = ActorId::<MyActor>::new(123, 0, 0);
    /// let own = ActorOwn::new(id);
    /// assert_eq!(own.id().as_u64(), 123);
    /// ```
    pub fn new(id: ActorId<T>) -> Self {
        Self { id }
    }

    /// Returns the actor ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{ActorOwn, ActorId};
    ///
    /// struct MyActor;
    ///
    /// let id = ActorId::<MyActor>::new(456, 0, 0);
    /// let own = ActorOwn::new(id);
    /// assert_eq!(own.id().as_u64(), 456);
    /// ```
    pub fn id(&self) -> ActorId<T> {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestActor;

    impl Actor for TestActor {
        fn start_up(&mut self) {}
        fn wakeup(&mut self) {}
        fn hangup(&mut self) {}
        fn tear_down(&mut self) {}
        fn loop_exec(&mut self) {}
        fn timeout_expired(&mut self) {}
    }

    #[test]
    fn test_actor_shared_new() {
        let id = ActorId::<TestActor>::new(123, 0, 0);
        let shared = ActorShared::new(id);
        assert_eq!(shared.id().as_u64(), 123);
    }

    #[test]
    fn test_actor_shared_clone() {
        let id = ActorId::<TestActor>::new(123, 0, 0);
        let shared1 = ActorShared::new(id);
        let shared2 = shared1.clone();
        assert_eq!(shared1.id().as_u64(), 123);
        assert_eq!(shared2.id().as_u64(), 123);
    }

    #[test]
    fn test_actor_own_new() {
        let id = ActorId::<TestActor>::new(456, 0, 0);
        let own = ActorOwn::new(id);
        assert_eq!(own.id().as_u64(), 456);
    }

    #[test]
    fn test_actor_own_clone() {
        let id = ActorId::<TestActor>::new(789, 0, 0);
        let own1 = ActorOwn::new(id);
        let own2 = own1.clone();
        assert_eq!(own1.id().as_u64(), 789);
        assert_eq!(own2.id().as_u64(), 789);
    }

    // Edge case tests
    #[test]
    fn test_actor_shared_zero_id() {
        let id = ActorId::<TestActor>::zero();
        let shared = ActorShared::new(id);
        assert!(shared.id().is_zero());
    }

    #[test]
    fn test_actor_shared_max_id() {
        let id = ActorId::<TestActor>::new(u64::MAX, u32::MAX, u32::MAX);
        let shared = ActorShared::new(id);
        assert_eq!(shared.id().as_u64(), u64::MAX);
        assert_eq!(shared.id().scheduler_id(), u32::MAX);
        assert_eq!(shared.id().generation(), u32::MAX);
    }

    #[test]
    fn test_actor_own_zero_id() {
        let id = ActorId::<TestActor>::zero();
        let own = ActorOwn::new(id);
        assert!(own.id().is_zero());
    }

    #[test]
    fn test_actor_own_max_id() {
        let id = ActorId::<TestActor>::new(u64::MAX, u32::MAX, u32::MAX);
        let own = ActorOwn::new(id);
        assert_eq!(own.id().as_u64(), u64::MAX);
    }

    #[test]
    fn test_actor_shared_multiple_clones() {
        let id = ActorId::<TestActor>::new(123, 1, 2);
        let shared1 = ActorShared::new(id);
        let shared2 = shared1.clone();
        let shared3 = shared2.clone();

        assert_eq!(shared1.id(), shared2.id());
        assert_eq!(shared2.id(), shared3.id());
    }

    #[test]
    fn test_actor_own_multiple_clones() {
        let id = ActorId::<TestActor>::new(456, 3, 4);
        let own1 = ActorOwn::new(id);
        let own2 = own1.clone();
        let own3 = own2.clone();

        assert_eq!(own1.id(), own2.id());
        assert_eq!(own2.id(), own3.id());
    }

    #[test]
    fn test_different_actor_types_can_share_ids() {
        struct ActorA;
        struct ActorB;

        impl Actor for ActorA {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        impl Actor for ActorB {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let id_a = ActorId::<ActorA>::new(100, 1, 0);
        let id_b = ActorId::<ActorB>::new(100, 1, 0);

        let shared_a = ActorShared::new(id_a);
        let shared_b = ActorShared::new(id_b);

        assert_eq!(shared_a.id().as_u64(), shared_b.id().as_u64());
    }
}
