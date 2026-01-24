// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Event system for actor message passing.

use std::any::Any;
use std::fmt;

/// Events that can be sent to actors.
///
/// Events represent the internal messages that drive the actor system,
/// including lifecycle events, timeouts, and message passing.
#[derive(Debug)]
pub enum Event {
    /// Start the actor (begin execution).
    Start,
    /// Stop the actor (graceful shutdown).
    Stop,
    /// Yield execution back to the scheduler.
    Yield,
    /// A timeout has expired.
    Timeout,
    /// The actor has been hung up on (owner dropped).
    Hangup,
    /// Execute a closure on the actor.
    Closure(Box<dyn FnOnce(&mut dyn ActorTrait) + Send>),
    /// Raw event with data.
    Raw(u64, Box<dyn Any + Send>),
}

/// Trait object for actor operations in events.
pub trait ActorTrait: Send {
    /// Get a reference to the actor as `Any` for downcasting.
    fn as_any(&self) -> &dyn std::any::Any;
    /// Get a mutable reference to the actor as `Any` for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T> ActorTrait for T
where
    T: Actor + 'static,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Forward declaration to avoid circular dependency.
pub trait Actor: Send + 'static {
    /// Called when the actor is started.
    fn start_up(&mut self);

    /// Called when the actor receives a wakeup signal.
    fn wakeup(&mut self);

    /// Called when the actor is hung up on.
    fn hangup(&mut self);

    /// Called when the actor should stop.
    fn tear_down(&mut self);

    /// Main event loop for the actor.
    fn loop_exec(&mut self);

    /// Handle timeout expiration.
    fn timeout_expired(&mut self);
}

impl Event {
    /// Creates a new closure event.
    ///
    /// # Arguments
    ///
    /// * `f` - The closure to execute
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Event;
    ///
    /// let event = Event::closure(|_actor| {
    ///     println!("Executing closure");
    /// });
    /// ```
    pub fn closure<F>(f: F) -> Self
    where
        F: FnOnce(&mut dyn ActorTrait) + Send + 'static,
    {
        Self::Closure(Box::new(f))
    }

    /// Creates a new raw event with data.
    ///
    /// # Arguments
    ///
    /// * `id` - The event ID
    /// * `data` - The event data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Event;
    ///
    /// let event = Event::raw(42, Box::new("test data"));
    /// ```
    pub fn raw<T>(id: u64, data: T) -> Self
    where
        T: Send + 'static,
    {
        Self::Raw(id, Box::new(data))
    }

    /// Checks if this event is a start event.
    pub fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    /// Checks if this event is a stop event.
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }

    /// Checks if this event is a yield event.
    pub fn is_yield(&self) -> bool {
        matches!(self, Self::Yield)
    }

    /// Checks if this event is a timeout event.
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }

    /// Checks if this event is a hangup event.
    pub fn is_hangup(&self) -> bool {
        matches!(self, Self::Hangup)
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "Start"),
            Self::Stop => write!(f, "Stop"),
            Self::Yield => write!(f, "Yield"),
            Self::Timeout => write!(f, "Timeout"),
            Self::Hangup => write!(f, "Hangup"),
            Self::Closure(_) => write!(f, "Closure"),
            Self::Raw(id, _) => write!(f, "Raw({})", id),
        }
    }
}

/// A full event with source and destination information.
///
/// `EventFull` contains additional metadata about where an event
/// originated and where it's going.
#[derive(Debug)]
pub struct EventFull {
    /// The actual event.
    pub event: Event,
    /// The source actor ID (0 if system-generated).
    pub source_id: u64,
    /// The destination actor ID.
    pub dest_id: u64,
    /// The destination scheduler ID.
    pub dest_scheduler: u32,
}

impl EventFull {
    /// Creates a new full event.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to wrap
    /// * `source_id` - The source actor ID
    /// * `dest_id` - The destination actor ID
    /// * `dest_scheduler` - The destination scheduler ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Event, EventFull};
    ///
    /// let full = EventFull::new(Event::Start, 0, 123, 1);
    /// assert_eq!(full.dest_id, 123);
    /// ```
    pub fn new(event: Event, source_id: u64, dest_id: u64, dest_scheduler: u32) -> Self {
        Self {
            event,
            source_id,
            dest_id,
            dest_scheduler,
        }
    }

    /// Creates a system-generated event (no source actor).
    ///
    /// # Arguments
    ///
    /// * `event` - The event to wrap
    /// * `dest_id` - The destination actor ID
    /// * `dest_scheduler` - The destination scheduler ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Event, EventFull};
    ///
    /// let full = EventFull::system(Event::Start, 123, 1);
    /// assert_eq!(full.source_id, 0);
    /// ```
    pub fn system(event: Event, dest_id: u64, dest_scheduler: u32) -> Self {
        Self::new(event, 0, dest_id, dest_scheduler)
    }

    /// Checks if this is a system-generated event.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Event, EventFull};
    ///
    /// let full = EventFull::system(Event::Start, 123, 1);
    /// assert!(full.is_system());
    /// ```
    pub fn is_system(&self) -> bool {
        self.source_id == 0
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
    fn test_event_is_start() {
        assert!(Event::Start.is_start());
        assert!(!Event::Stop.is_start());
    }

    #[test]
    fn test_event_is_stop() {
        assert!(Event::Stop.is_stop());
        assert!(!Event::Start.is_stop());
    }

    #[test]
    fn test_event_is_yield() {
        assert!(Event::Yield.is_yield());
        assert!(!Event::Start.is_yield());
    }

    #[test]
    fn test_event_is_timeout() {
        assert!(Event::Timeout.is_timeout());
        assert!(!Event::Start.is_timeout());
    }

    #[test]
    fn test_event_is_hangup() {
        assert!(Event::Hangup.is_hangup());
        assert!(!Event::Start.is_hangup());
    }

    #[test]
    fn test_event_closure() {
        let mut called = false;
        let event = Event::closure(|_actor| {
            called = true;
        });
        if let Event::Closure(f) = event {
            let mut actor = TestActor;
            f(&mut actor);
            assert!(called);
        } else {
            panic!("Expected closure event");
        }
    }

    #[test]
    fn test_event_raw() {
        let event = Event::raw(42, "test data");
        if let Event::Raw(id, data) = event {
            assert_eq!(id, 42);
            let data = data.downcast_ref::<&str>();
            assert_eq!(data, Some(&"test data"));
        } else {
            panic!("Expected raw event");
        }
    }

    #[test]
    fn test_event_display() {
        assert_eq!(format!("{}", Event::Start), "Start");
        assert_eq!(format!("{}", Event::Stop), "Stop");
        assert_eq!(format!("{}", Event::Yield), "Yield");
        assert_eq!(format!("{}", Event::Timeout), "Timeout");
        assert_eq!(format!("{}", Event::Hangup), "Hangup");
        assert_eq!(format!("{}", Event::Closure(Box::new(|_: &mut dyn ActorTrait| {}))), "Closure");
        assert_eq!(format!("{}", Event::Raw(42, Box::new(()))), "Raw(42)");
    }

    #[test]
    fn test_event_full_new() {
        let full = EventFull::new(Event::Start, 10, 20, 1);
        assert_eq!(full.source_id, 10);
        assert_eq!(full.dest_id, 20);
        assert_eq!(full.dest_scheduler, 1);
    }

    #[test]
    fn test_event_full_system() {
        let full = EventFull::system(Event::Start, 123, 2);
        assert_eq!(full.source_id, 0);
        assert_eq!(full.dest_id, 123);
        assert_eq!(full.dest_scheduler, 2);
    }

    #[test]
    fn test_event_full_is_system() {
        let system = EventFull::system(Event::Start, 123, 1);
        assert!(system.is_system());

        let user = EventFull::new(Event::Start, 10, 20, 1);
        assert!(!user.is_system());
    }
}
