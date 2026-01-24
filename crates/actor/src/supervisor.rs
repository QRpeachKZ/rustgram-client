// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Supervisor strategies for fault tolerance.

use crate::actor_info::ActorInfo;
use std::fmt;
use std::time::Duration;

/// Supervisor strategy for handling child actor failures.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::SupervisorStrategy;
///
/// let strategy = SupervisorStrategy::OneForOne {
///     max_retries: 3,
///     within: Duration::from_secs(60),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupervisorStrategy {
    /// Restart only the failed child actor.
    OneForOne {
        /// Maximum number of restart attempts.
        max_retries: usize,
        /// Time window for restart attempts.
        within: Duration,
    },
    /// Restart all children when one fails.
    OneForAll {
        /// Maximum number of restart attempts.
        max_retries: usize,
        /// Time window for restart attempts.
        within: Duration,
    },
    /// Escalate the failure to the parent supervisor.
    Escalate,
    /// Stop the child actor without restarting.
    Stop,
}

impl SupervisorStrategy {
    /// Creates a OneForOne strategy with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of restart attempts
    /// * `within` - Time window for restart attempts
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorStrategy;
    /// use std::time::Duration;
    ///
    /// let strategy = SupervisorStrategy::one_for_one(5, Duration::from_secs(60));
    /// ```
    pub fn one_for_one(max_retries: usize, within: Duration) -> Self {
        Self::OneForOne {
            max_retries,
            within,
        }
    }

    /// Creates a OneForAll strategy with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of restart attempts
    /// * `within` - Time window for restart attempts
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorStrategy;
    /// use std::time::Duration;
    ///
    /// let strategy = SupervisorStrategy::one_for_all(3, Duration::from_secs(30));
    /// ```
    pub fn one_for_all(max_retries: usize, within: Duration) -> Self {
        Self::OneForAll {
            max_retries,
            within,
        }
    }

    /// Checks if this is a OneForOne strategy.
    ///
    /// # Returns
    ///
    /// * `true` - If this is OneForOne
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorStrategy;
    /// use std::time::Duration;
    ///
    /// let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
    /// assert!(strategy.is_one_for_one());
    /// ```
    pub fn is_one_for_one(&self) -> bool {
        matches!(self, Self::OneForOne { .. })
    }

    /// Checks if this is a OneForAll strategy.
    ///
    /// # Returns
    ///
    /// * `true` - If this is OneForAll
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorStrategy;
    /// use std::time::Duration;
    ///
    /// let strategy = SupervisorStrategy::one_for_all(3, Duration::from_secs(60));
    /// assert!(strategy.is_one_for_all());
    /// ```
    pub fn is_one_for_all(&self) -> bool {
        matches!(self, Self::OneForAll { .. })
    }

    /// Checks if this is an Escalate strategy.
    ///
    /// # Returns
    ///
    /// * `true` - If this is Escalate
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorStrategy;
    ///
    /// assert!(SupervisorStrategy::Escalate.is_escalate());
    /// ```
    pub fn is_escalate(&self) -> bool {
        matches!(self, Self::Escalate)
    }

    /// Checks if this is a Stop strategy.
    ///
    /// # Returns
    ///
    /// * `true` - If this is Stop
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorStrategy;
    ///
    /// assert!(SupervisorStrategy::Stop.is_stop());
    /// ```
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }
}

/// A child actor under supervision.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::SupervisedChild;
/// use std::time::{Duration, Instant};
///
/// let child = SupervisedChild {
///     id: 123,
///     restart_count: 0,
///     last_restart: None,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SupervisedChild {
    /// The actor ID.
    pub id: u64,
    /// The number of times this child has been restarted.
    pub restart_count: usize,
    /// The timestamp of the last restart.
    pub last_restart: Option<std::time::Instant>,
}

impl SupervisedChild {
    /// Creates a new supervised child.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisedChild;
    ///
    /// let child = SupervisedChild::new(123);
    /// assert_eq!(child.id, 123);
    /// assert_eq!(child.restart_count, 0);
    /// ```
    pub fn new(id: u64) -> Self {
        Self {
            id,
            restart_count: 0,
            last_restart: None,
        }
    }

    /// Checks if the child can be restarted based on the strategy.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The supervisor strategy
    ///
    /// # Returns
    ///
    /// * `true` - If the child can be restarted
    /// * `false` - If the restart limit has been exceeded
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{SupervisedChild, SupervisorStrategy};
    /// use std::time::Duration;
    ///
    /// let child = SupervisedChild::new(123);
    /// let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
    /// assert!(child.can_restart(&strategy));
    /// ```
    pub fn can_restart(&self, strategy: &SupervisorStrategy) -> bool {
        match strategy {
            SupervisorStrategy::OneForOne { max_retries, within } => {
                if self.restart_count >= *max_retries {
                    return false;
                }
                if let Some(last) = self.last_restart {
                    let elapsed = last.elapsed();
                    if elapsed > *within {
                        return true; // Time window expired, restart count resets
                    }
                }
                true
            }
            SupervisorStrategy::OneForAll { max_retries, within } => {
                if self.restart_count >= *max_retries {
                    return false;
                }
                if let Some(last) = self.last_restart {
                    let elapsed = last.elapsed();
                    if elapsed > *within {
                        return true;
                    }
                }
                true
            }
            _ => false,
        }
    }

    /// Records a restart for this child.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisedChild;
    ///
    /// let mut child = SupervisedChild::new(123);
    /// assert_eq!(child.restart_count, 0);
    ///
    /// child.record_restart();
    /// assert_eq!(child.restart_count, 1);
    /// assert!(child.last_restart.is_some());
    /// ```
    pub fn record_restart(&mut self) {
        self.restart_count += 1;
        self.last_restart = Some(std::time::Instant::now());
    }

    /// Resets the restart count (e.g., after time window expires).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisedChild;
    ///
    /// let mut child = SupervisedChild::new(123);
    /// child.record_restart();
    /// assert_eq!(child.restart_count, 1);
    ///
    /// child.reset_restart_count();
    /// assert_eq!(child.restart_count, 0);
    /// ```
    pub fn reset_restart_count(&mut self) {
        self.restart_count = 0;
        self.last_restart = None;
    }
}

/// Supervisor directive for handling a failure.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::SupervisorDirective;
///
/// let directive = SupervisorDirective::Restart(123);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupervisorDirective {
    /// Restart the specified actor.
    Restart(u64),
    /// Restart all children.
    RestartAll,
    /// Stop the specified actor.
    Stop(u64),
    /// Stop all children.
    StopAll,
    /// Escalate to parent supervisor.
    Escalate,
    /// Resume without action.
    Resume,
}

impl SupervisorDirective {
    /// Checks if this is a restart directive.
    ///
    /// # Returns
    ///
    /// * `true` - If this is Restart or RestartAll
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorDirective;
    ///
    /// assert!(SupervisorDirective::Restart(123).is_restart());
    /// assert!(SupervisorDirective::RestartAll.is_restart());
    /// ```
    pub fn is_restart(&self) -> bool {
        matches!(self, Self::Restart(_) | Self::RestartAll)
    }

    /// Checks if this is a stop directive.
    ///
    /// # Returns
    ///
    /// * `true` - If this is Stop or StopAll
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorDirective;
    ///
    /// assert!(SupervisorDirective::Stop(123).is_stop());
    /// assert!(SupervisorDirective::StopAll.is_stop());
    /// ```
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop(_) | Self::StopAll)
    }

    /// Checks if this is an escalate directive.
    ///
    /// # Returns
    ///
    /// * `true` - If this is Escalate
    /// * `false` - Otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::SupervisorDirective;
    ///
    /// assert!(SupervisorDirective::Escalate.is_escalate());
    /// ```
    pub fn is_escalate(&self) -> bool {
        matches!(self, Self::Escalate)
    }
}

/// A supervisor that manages child actors.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::{Supervisor, SupervisorStrategy};
/// use std::time::Duration;
///
/// let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
/// let supervisor = Supervisor::new(strategy);
/// ```
#[derive(Debug, Clone)]
pub struct Supervisor {
    /// The supervisor strategy.
    pub strategy: SupervisorStrategy,
    /// The supervised children.
    pub children: Vec<SupervisedChild>,
}

impl Supervisor {
    /// Creates a new supervisor with the given strategy.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The supervisor strategy
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Supervisor, SupervisorStrategy};
    /// use std::time::Duration;
    ///
    /// let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
    /// let supervisor = Supervisor::new(strategy);
    /// ```
    pub fn new(strategy: SupervisorStrategy) -> Self {
        Self {
            strategy,
            children: Vec::new(),
        }
    }

    /// Adds a child actor to supervision.
    ///
    /// # Arguments
    ///
    /// * `child_id` - The actor ID of the child
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Supervisor, SupervisorStrategy};
    /// use std::time::Duration;
    ///
    /// let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
    /// supervisor.add_child(123);
    /// assert_eq!(supervisor.children.len(), 1);
    /// ```
    pub fn add_child(&mut self, child_id: u64) {
        let child = SupervisedChild::new(child_id);
        self.children.push(child);
    }

    /// Removes a child actor from supervision.
    ///
    /// # Arguments
    ///
    /// * `child_id` - The actor ID of the child
    ///
    /// # Returns
    ///
    /// * `Some(SupervisedChild)` - If the child was found
    /// * `None` - If the child was not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Supervisor, SupervisorStrategy};
    /// use std::time::Duration;
    ///
    /// let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
    /// supervisor.add_child(123);
    /// assert!(supervisor.remove_child(123).is_some());
    /// ```
    pub fn remove_child(&mut self, child_id: u64) -> Option<SupervisedChild> {
        let pos = self.children.iter().position(|c| c.id == child_id)?;
        Some(self.children.remove(pos))
    }

    /// Handles a child failure and returns the appropriate directive.
    ///
    /// # Arguments
    ///
    /// * `child_id` - The actor ID of the failed child
    ///
    /// # Returns
    ///
    /// The supervisor directive for handling the failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Supervisor, SupervisorStrategy};
    /// use std::time::Duration;
    ///
    /// let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
    /// supervisor.add_child(123);
    ///
    /// let directive = supervisor.handle_failure(123);
    /// assert!(directive.is_restart());
    /// ```
    pub fn handle_failure(&mut self, child_id: u64) -> SupervisorDirective {
        match self.strategy {
            SupervisorStrategy::OneForOne { .. } => {
                if let Some(child) = self.children.iter_mut().find(|c| c.id == child_id) {
                    if child.can_restart(&self.strategy) {
                        child.record_restart();
                        SupervisorDirective::Restart(child_id)
                    } else {
                        SupervisorDirective::Stop(child_id)
                    }
                } else {
                    SupervisorDirective::Stop(child_id)
                }
            }
            SupervisorStrategy::OneForAll { .. } => {
                // Check if any child has exceeded restart limit
                let can_restart = self.children.iter().all(|c| c.can_restart(&self.strategy));
                if can_restart {
                    for child in &mut self.children {
                        child.record_restart();
                    }
                    SupervisorDirective::RestartAll
                } else {
                    SupervisorDirective::StopAll
                }
            }
            SupervisorStrategy::Escalate => SupervisorDirective::Escalate,
            SupervisorStrategy::Stop => SupervisorDirective::Stop(child_id),
        }
    }

    /// Gets a child by ID.
    ///
    /// # Arguments
    ///
    /// * `child_id` - The actor ID of the child
    ///
    /// # Returns
    ///
    /// * `Some(&SupervisedChild)` - If the child was found
    /// * `None` - If the child was not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Supervisor, SupervisorStrategy};
    /// use std::time::Duration;
    ///
    /// let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
    /// supervisor.add_child(123);
    /// assert!(supervisor.get_child(123).is_some());
    /// ```
    pub fn get_child(&self, child_id: u64) -> Option<&SupervisedChild> {
        self.children.iter().find(|c| c.id == child_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_supervisor_strategy_one_for_one() {
        let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
        assert!(strategy.is_one_for_one());
        assert!(!strategy.is_one_for_all());
    }

    #[test]
    fn test_supervisor_strategy_one_for_all() {
        let strategy = SupervisorStrategy::one_for_all(3, Duration::from_secs(60));
        assert!(strategy.is_one_for_all());
        assert!(!strategy.is_one_for_one());
    }

    #[test]
    fn test_supervisor_strategy_escalate() {
        assert!(SupervisorStrategy::Escalate.is_escalate());
        assert!(!SupervisorStrategy::Escalate.is_stop());
    }

    #[test]
    fn test_supervisor_strategy_stop() {
        assert!(SupervisorStrategy::Stop.is_stop());
        assert!(!SupervisorStrategy::Stop.is_escalate());
    }

    #[test]
    fn test_supervised_child_new() {
        let child = SupervisedChild::new(123);
        assert_eq!(child.id, 123);
        assert_eq!(child.restart_count, 0);
        assert!(child.last_restart.is_none());
    }

    #[test]
    fn test_supervised_child_can_restart() {
        let child = SupervisedChild::new(123);
        let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
        assert!(child.can_restart(&strategy));
    }

    #[test]
    fn test_supervised_child_can_restart_limit() {
        let mut child = SupervisedChild::new(123);
        let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));

        for _ in 0..3 {
            child.record_restart();
        }
        assert!(!child.can_restart(&strategy));
    }

    #[test]
    fn test_supervised_child_record_restart() {
        let mut child = SupervisedChild::new(123);
        child.record_restart();
        assert_eq!(child.restart_count, 1);
        assert!(child.last_restart.is_some());
    }

    #[test]
    fn test_supervised_child_reset_restart_count() {
        let mut child = SupervisedChild::new(123);
        child.record_restart();
        child.reset_restart_count();
        assert_eq!(child.restart_count, 0);
        assert!(child.last_restart.is_none());
    }

    #[test]
    fn test_supervisor_directive_is_restart() {
        assert!(SupervisorDirective::Restart(123).is_restart());
        assert!(SupervisorDirective::RestartAll.is_restart());
        assert!(!SupervisorDirective::Stop(123).is_restart());
    }

    #[test]
    fn test_supervisor_directive_is_stop() {
        assert!(SupervisorDirective::Stop(123).is_stop());
        assert!(SupervisorDirective::StopAll.is_stop());
        assert!(!SupervisorDirective::Restart(123).is_stop());
    }

    #[test]
    fn test_supervisor_directive_is_escalate() {
        assert!(SupervisorDirective::Escalate.is_escalate());
        assert!(!SupervisorDirective::Restart(123).is_escalate());
    }

    #[test]
    fn test_supervisor_new() {
        let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
        let supervisor = Supervisor::new(strategy);
        assert!(supervisor.children.is_empty());
    }

    #[test]
    fn test_supervisor_add_child() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
        supervisor.add_child(123);
        assert_eq!(supervisor.children.len(), 1);
    }

    #[test]
    fn test_supervisor_remove_child() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
        supervisor.add_child(123);
        assert!(supervisor.remove_child(123).is_some());
        assert!(supervisor.remove_child(123).is_none());
    }

    #[test]
    fn test_supervisor_get_child() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
        supervisor.add_child(123);
        assert!(supervisor.get_child(123).is_some());
        assert!(supervisor.get_child(456).is_none());
    }

    #[test]
    fn test_supervisor_handle_failure_one_for_one() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
        supervisor.add_child(123);

        let directive = supervisor.handle_failure(123);
        assert!(directive.is_restart());

        let child = supervisor.get_child(123).unwrap();
        assert_eq!(child.restart_count, 1);
    }

    #[test]
    fn test_supervisor_handle_failure_one_for_one_limit() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(3, Duration::from_secs(60)));
        supervisor.add_child(123);

        // Exceed the restart limit
        for _ in 0..3 {
            supervisor.handle_failure(123);
        }

        let directive = supervisor.handle_failure(123);
        assert!(directive.is_stop());
    }

    #[test]
    fn test_supervisor_handle_failure_one_for_all() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_all(3, Duration::from_secs(60)));
        supervisor.add_child(123);
        supervisor.add_child(456);

        let directive = supervisor.handle_failure(123);
        assert_eq!(directive, SupervisorDirective::RestartAll);

        let child1 = supervisor.get_child(123).unwrap();
        let child2 = supervisor.get_child(456).unwrap();
        assert_eq!(child1.restart_count, 1);
        assert_eq!(child2.restart_count, 1);
    }

    #[test]
    fn test_supervisor_handle_failure_escalate() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::Escalate);
        supervisor.add_child(123);

        let directive = supervisor.handle_failure(123);
        assert_eq!(directive, SupervisorDirective::Escalate);
    }

    #[test]
    fn test_supervisor_handle_failure_stop() {
        let mut supervisor = Supervisor::new(SupervisorStrategy::Stop);
        supervisor.add_child(123);

        let directive = supervisor.handle_failure(123);
        assert!(directive.is_stop());
    }
}
