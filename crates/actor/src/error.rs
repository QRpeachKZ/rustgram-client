// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the actor framework.

use std::fmt;

/// Errors that can occur in the actor system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorError {
    /// Actor not found in registry.
    ActorNotFound(u64),
    /// Actor exists but is not in running state.
    ActorNotRunning(u64),
    /// Actor has been stopped and is dead.
    ActorDead(u64),
    /// Scheduler has been stopped.
    SchedulerStopped,
    /// Actor mailbox is full.
    MailboxFull,
    /// Actor migration failed.
    MigrationFailed,
    /// Timeout expired.
    TimeoutExpired,
    /// Invalid actor reference.
    InvalidActorRef,
    /// Operation not permitted in current actor state.
    InvalidState(String),
}

impl fmt::Display for ActorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ActorNotFound(id) => write!(f, "Actor {} not found", id),
            Self::ActorNotRunning(id) => write!(f, "Actor {} is not running", id),
            Self::ActorDead(id) => write!(f, "Actor {} is dead", id),
            Self::SchedulerStopped => write!(f, "Scheduler has been stopped"),
            Self::MailboxFull => write!(f, "Actor mailbox is full"),
            Self::MigrationFailed => write!(f, "Actor migration failed"),
            Self::TimeoutExpired => write!(f, "Timeout expired"),
            Self::InvalidActorRef => write!(f, "Invalid actor reference"),
            Self::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
        }
    }
}

impl std::error::Error for ActorError {}

/// Result type for actor operations.
pub type Result<T> = std::result::Result<T, ActorError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_error_display() {
        assert_eq!(
            format!("{}", ActorError::ActorNotFound(123)),
            "Actor 123 not found"
        );
        assert_eq!(
            format!("{}", ActorError::SchedulerStopped),
            "Scheduler has been stopped"
        );
    }

    #[test]
    fn test_actor_error_equality() {
        assert_eq!(ActorError::MailboxFull, ActorError::MailboxFull);
        assert_ne!(
            ActorError::ActorNotFound(1),
            ActorError::ActorNotFound(2)
        );
    }

    #[test]
    fn test_result_type() {
        let result: Result<()> = Ok(());
        assert!(result.is_ok());

        let result: Result<()> = Err(ActorError::InvalidActorRef);
        assert!(result.is_err());
    }
}
