// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Core actor trait and implementation.

use crate::actor_info::ActorInfo;
use crate::event::Actor;
use std::any::Any;
use std::fmt;

/// A handle to an actor that allows sending messages.
///
/// `ActorHandle` provides a way to interact with an actor without
/// directly accessing its internals.
///
/// # Type Parameters
///
/// * `T` - The actor type
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::ActorHandle;
///
/// struct MyActor;
///
/// let handle = ActorHandle::<MyActor>::new(123, 1);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ActorHandle<T> {
    /// The actor ID.
    pub id: u64,
    /// The scheduler ID.
    pub scheduler_id: u32,
    /// Phantom data for the actor type.
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ActorHandle<T> {
    /// Creates a new actor handle.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID
    /// * `scheduler_id` - The scheduler ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorHandle;
    ///
    /// struct MyActor;
    /// let handle = ActorHandle::<MyActor>::new(123, 1);
    /// assert_eq!(handle.id, 123);
    /// ```
    pub fn new(id: u64, scheduler_id: u32) -> Self {
        Self {
            id,
            scheduler_id,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns the actor ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorHandle;
    ///
    /// struct MyActor;
    /// let handle = ActorHandle::<MyActor>::new(456, 1);
    /// assert_eq!(handle.actor_id(), 456);
    /// ```
    pub fn actor_id(&self) -> u64 {
        self.id
    }

    /// Returns the scheduler ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorHandle;
    ///
    /// struct MyActor;
    /// let handle = ActorHandle::<MyActor>::new(123, 2);
    /// assert_eq!(handle.scheduler(), 2);
    /// ```
    pub fn scheduler(&self) -> u32 {
        self.scheduler_id
    }
}

impl<T> Default for ActorHandle<T> {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl<T> fmt::Display for ActorHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ActorHandle(id={}, scheduler={})", self.id, self.scheduler_id)
    }
}

/// Extension trait for actors with additional functionality.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::ActorExt;
///
/// struct MyActor;
///
/// impl ActorExt for MyActor {
///     fn actor_info(&self) -> Option<&ActorInfo> {
///         None
///     }
/// }
/// ```
pub trait ActorExt: Actor {
    /// Returns the actor info if available.
    ///
    /// # Returns
    ///
    /// * `Some(&ActorInfo)` - If the actor has info
    /// * `None` - If the actor info is not available
    fn actor_info(&self) -> Option<&ActorInfo> {
        None
    }

    /// Returns a mutable reference to the actor info if available.
    ///
    /// # Returns
    ///
    /// * `Some(&mut ActorInfo)` - If the actor has info
    /// * `None` - If the actor info is not available
    fn actor_info_mut(&mut self) -> Option<&mut ActorInfo> {
        None
    }

    /// Called before the actor starts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorExt;
    ///
    /// struct MyActor;
    ///
    /// impl ActorExt for MyActor {
    ///     fn pre_start(&mut self) {
    ///         println!("Actor about to start");
    ///     }
    /// }
    /// ```
    fn pre_start(&mut self) {
        // Default: do nothing
    }

    /// Called after the actor stops.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorExt;
    ///
    /// struct MyActor;
    ///
    /// impl ActorExt for MyActor {
    ///     fn post_stop(&mut self) {
    ///         println!("Actor stopped");
    ///     }
    /// }
    fn post_stop(&mut self) {
        // Default: do nothing
    }
}

/// Blanket implementation for all actors.
impl<T> ActorExt for T where T: Actor {}

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
    fn test_actor_handle_new() {
        let handle = ActorHandle::<TestActor>::new(123, 1);
        assert_eq!(handle.id, 123);
        assert_eq!(handle.scheduler_id, 1);
    }

    #[test]
    fn test_actor_handle_actor_id() {
        let handle = ActorHandle::<TestActor>::new(456, 1);
        assert_eq!(handle.actor_id(), 456);
    }

    #[test]
    fn test_actor_handle_scheduler() {
        let handle = ActorHandle::<TestActor>::new(123, 2);
        assert_eq!(handle.scheduler(), 2);
    }

    #[test]
    fn test_actor_handle_copy() {
        let handle1 = ActorHandle::<TestActor>::new(123, 1);
        let handle2 = handle1;
        assert_eq!(handle1.id, 123); // Copy, so both valid
        assert_eq!(handle2.id, 123);
    }

    #[test]
    fn test_actor_handle_clone() {
        let handle1 = ActorHandle::<TestActor>::new(123, 1);
        let handle2 = handle1.clone();
        assert_eq!(handle1.id, 123);
        assert_eq!(handle2.id, 123);
    }

    #[test]
    fn test_actor_handle_default() {
        let handle = ActorHandle::<TestActor>::default();
        assert_eq!(handle.id, 0);
        assert_eq!(handle.scheduler_id, 0);
    }

    #[test]
    fn test_actor_handle_display() {
        let handle = ActorHandle::<TestActor>::new(123, 2);
        let display = format!("{}", handle);
        assert!(display.contains("123"));
        assert!(display.contains("2"));
    }

    #[test]
    fn test_actor_ext_blanket_impl() {
        let actor = TestActor;
        // ActorExt is implemented for all Actor types
        let _info = actor.actor_info();
        let _info_mut = actor.actor_info_mut();
    }
}
