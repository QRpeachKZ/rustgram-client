// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Actor metadata and state tracking.

use std::fmt;
use std::time::Instant;

/// The current state of an actor in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorState {
    /// Actor is being initialized.
    Starting,
    /// Actor is running and processing messages.
    Running,
    /// Actor is in the process of stopping.
    Stopping,
    /// Actor is migrating to a different scheduler.
    ///
    /// # Fields
    ///
    /// * `u32` - The destination scheduler ID
    Migrating(u32),
    /// Actor has been stopped and resources freed.
    Dead,
}

impl ActorState {
    /// Checks if the actor is in a runnable state.
    ///
    /// # Returns
    ///
    /// * `true` - If the actor can process messages
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorState;
    ///
    /// assert!(ActorState::Running.is_runnable());
    /// assert!(!ActorState::Dead.is_runnable());
    /// ```
    pub fn is_runnable(self) -> bool {
        matches!(self, Self::Running)
    }

    /// Checks if the actor is alive (not dead).
    ///
    /// # Returns
    ///
    /// * `true` - If the actor is not dead
    /// * `false` - If the actor is dead
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorState;
    ///
    /// assert!(ActorState::Running.is_alive());
    /// assert!(!ActorState::Dead.is_alive());
    /// ```
    pub fn is_alive(self) -> bool {
        !matches!(self, Self::Dead)
    }

    /// Checks if the actor is migrating.
    ///
    /// # Returns
    ///
    /// * `Some(u32)` - The destination scheduler ID if migrating
    /// * `None` - If not migrating
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorState;
    ///
    /// assert_eq!(ActorState::Migrating(5).migrating_to(), Some(5));
    /// assert_eq!(ActorState::Running.migrating_to(), None);
    /// ```
    pub fn migrating_to(self) -> Option<u32> {
        match self {
            Self::Migrating(dest) => Some(dest),
            _ => None,
        }
    }
}

impl fmt::Display for ActorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Starting => write!(f, "Starting"),
            Self::Running => write!(f, "Running"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Migrating(dest) => write!(f, "Migrating to {}", dest),
            Self::Dead => write!(f, "Dead"),
        }
    }
}

/// Metadata about an actor.
///
/// `ActorInfo` contains runtime information about an actor's state,
/// scheduler assignment, and timing information.
#[derive(Debug)]
pub struct ActorInfo {
    /// The unique name of this actor.
    pub name: String,
    /// The current state of the actor.
    pub state: ActorState,
    /// The ID of the scheduler this actor is assigned to.
    pub scheduler_id: u32,
    /// The timestamp when the actor was created.
    pub created_at: Instant,
    /// Optional timeout for the actor.
    pub timeout: Option<Instant>,
}

impl ActorInfo {
    /// Creates new actor info with the given name and scheduler ID.
    ///
    /// # Arguments
    ///
    /// * `name` - The actor's name
    /// * `scheduler_id` - The scheduler ID this actor is assigned to
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorInfo;
    ///
    /// let info = ActorInfo::new("my_actor", 1);
    /// assert_eq!(info.name, "my_actor");
    /// assert_eq!(info.scheduler_id, 1);
    /// ```
    pub fn new(name: String, scheduler_id: u32) -> Self {
        Self {
            name,
            state: ActorState::Starting,
            scheduler_id,
            created_at: Instant::now(),
            timeout: None,
        }
    }

    /// Returns the actor's name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorInfo;
    ///
    /// let info = ActorInfo::new("test_actor", 0);
    /// assert_eq!(info.name(), "test_actor");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the actor's current state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{ActorInfo, ActorState};
    ///
    /// let info = ActorInfo::new("test_actor", 0);
    /// assert_eq!(info.state(), ActorState::Starting);
    /// ```
    pub fn state(&self) -> ActorState {
        self.state
    }

    /// Returns the scheduler ID this actor is assigned to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorInfo;
    ///
    /// let info = ActorInfo::new("test_actor", 5);
    /// assert_eq!(info.scheduler_id(), 5);
    /// ```
    pub fn scheduler_id(&self) -> u32 {
        self.scheduler_id
    }

    /// Returns the timeout if one is set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorInfo;
    /// use std::time::Instant;
    ///
    /// let mut info = ActorInfo::new("test_actor", 0);
    /// assert!(info.timeout().is_none());
    ///
    /// let timeout = Instant::now() + std::time::Duration::from_secs(10);
    /// info.timeout = Some(timeout);
    /// assert!(info.timeout().is_some());
    /// ```
    pub fn timeout(&self) -> Option<Instant> {
        self.timeout
    }

    /// Sets the actor's state.
    ///
    /// # Arguments
    ///
    /// * `state` - The new state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{ActorInfo, ActorState};
    ///
    /// let mut info = ActorInfo::new("test_actor", 0);
    /// info.set_state(ActorState::Running);
    /// assert_eq!(info.state(), ActorState::Running);
    /// ```
    pub fn set_state(&mut self, state: ActorState) {
        self.state = state;
    }

    /// Sets a timeout for the actor.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout instant
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorInfo;
    /// use std::time::{Instant, Duration};
    ///
    /// let mut info = ActorInfo::new("test_actor", 0);
    /// let timeout = Instant::now() + Duration::from_secs(5);
    /// info.set_timeout(Some(timeout));
    /// assert!(info.timeout().is_some());
    /// ```
    pub fn set_timeout(&mut self, timeout: Option<Instant>) {
        self.timeout = timeout;
    }

    /// Checks if the actor's timeout has expired.
    ///
    /// # Returns
    ///
    /// * `true` - If a timeout is set and has passed
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorInfo;
    /// use std::time::{Instant, Duration};
    ///
    /// let mut info = ActorInfo::new("test_actor", 0);
    /// assert!(!info.has_timeout_expired());
    ///
    /// let past = Instant::now() - Duration::from_secs(1);
    /// info.set_timeout(Some(past));
    /// assert!(info.has_timeout_expired());
    /// ```
    pub fn has_timeout_expired(&self) -> bool {
        match self.timeout {
            Some(timeout) => timeout <= Instant::now(),
            None => false,
        }
    }
}

impl Clone for ActorInfo {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            state: self.state,
            scheduler_id: self.scheduler_id,
            created_at: self.created_at,
            timeout: self.timeout,
        }
    }
}

impl fmt::Display for ActorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ActorInfo(name={}, state={}, scheduler={})",
            self.name, self.state, self.scheduler_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_actor_state_is_runnable() {
        assert!(ActorState::Running.is_runnable());
        assert!(!ActorState::Starting.is_runnable());
        assert!(!ActorState::Stopping.is_runnable());
        assert!(!ActorState::Dead.is_runnable());
        assert!(!ActorState::Migrating(1).is_runnable());
    }

    #[test]
    fn test_actor_state_is_alive() {
        assert!(ActorState::Starting.is_alive());
        assert!(ActorState::Running.is_alive());
        assert!(ActorState::Stopping.is_alive());
        assert!(ActorState::Migrating(1).is_alive());
        assert!(!ActorState::Dead.is_alive());
    }

    #[test]
    fn test_actor_state_migrating_to() {
        assert_eq!(ActorState::Migrating(5).migrating_to(), Some(5));
        assert_eq!(ActorState::Running.migrating_to(), None);
    }

    #[test]
    fn test_actor_state_display() {
        assert_eq!(format!("{}", ActorState::Running), "Running");
        assert_eq!(format!("{}", ActorState::Migrating(3)), "Migrating to 3");
    }

    #[test]
    fn test_actor_info_new() {
        let info = ActorInfo::new("test".to_string(), 1);
        assert_eq!(info.name(), "test");
        assert_eq!(info.scheduler_id(), 1);
        assert_eq!(info.state(), ActorState::Starting);
    }

    #[test]
    fn test_actor_info_set_state() {
        let mut info = ActorInfo::new("test".to_string(), 0);
        info.set_state(ActorState::Running);
        assert_eq!(info.state(), ActorState::Running);
    }

    #[test]
    fn test_actor_info_timeout() {
        let mut info = ActorInfo::new("test".to_string(), 0);
        assert!(info.timeout().is_none());

        let timeout = Instant::now() + Duration::from_secs(10);
        info.set_timeout(Some(timeout));
        assert_eq!(info.timeout(), Some(timeout));
    }

    #[test]
    fn test_actor_info_timeout_expired() {
        let mut info = ActorInfo::new("test".to_string(), 0);
        assert!(!info.has_timeout_expired());

        let past = Instant::now() - Duration::from_secs(1);
        info.set_timeout(Some(past));
        assert!(info.has_timeout_expired());

        let future = Instant::now() + Duration::from_secs(10);
        info.set_timeout(Some(future));
        assert!(!info.has_timeout_expired());
    }

    #[test]
    fn test_actor_info_display() {
        let info = ActorInfo::new("actor1".to_string(), 2);
        let display = format!("{}", info);
        assert!(display.contains("actor1"));
        assert!(display.contains("Starting"));
        assert!(display.contains("2"));
    }

    #[test]
    fn test_actor_info_clone() {
        let mut info = ActorInfo::new("test".to_string(), 1);
        info.set_state(ActorState::Running);
        let cloned = info.clone();
        assert_eq!(cloned.name(), "test");
        assert_eq!(cloned.state(), ActorState::Running);
        assert_eq!(cloned.scheduler_id(), 1);
    }
}
