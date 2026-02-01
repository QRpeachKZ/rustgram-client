//! Error types for the issue tracker.
//!
//! This module defines all error types that can occur when interacting with
//! the Beads issue tracking system.

use thiserror::Error;

/// Result type alias for issue tracker operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during issue tracker operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Database-related errors.
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// Invalid issue ID format.
    #[error("Invalid issue ID format: {0}")]
    InvalidIssueId(String),

    /// Issue not found in database.
    #[error("Issue not found: {0}")]
    IssueNotFound(String),

    /// Dependency would create a circular reference.
    #[error("Dependency would create cycle: {issue_id} -> {depends_on}")]
    CircularDependency {
        /// The issue that would depend on another.
        issue_id: String,
        /// The issue that would be depended on.
        depends_on: String,
    },

    /// Cannot close issue with open dependencies.
    #[error("Cannot close issue with {count} open dependencies")]
    OpenDependencies {
        /// Number of open dependencies.
        count: usize,
    },

    /// Priority value out of valid range (0-4).
    #[error("Priority out of range (0-4): {0}")]
    InvalidPriority(i32),

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Beads CLI command error.
    #[error("Beads CLI error: {0}")]
    Cli(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid status transition.
    #[error("Invalid status transition: {from:?} -> {to:?}")]
    InvalidStatusTransition {
        /// Current status.
        from: IssueStatusError,
        /// Target status.
        to: IssueStatusError,
    },

    /// Database not found at expected path.
    #[error("Beads database not found at path: {0}")]
    DatabaseNotFound(String),

    /// Empty title provided.
    #[error("Issue title cannot be empty")]
    EmptyTitle,

    /// Title exceeds maximum length.
    #[error("Issue title exceeds maximum length of {max} characters (got {len})")]
    TitleTooLong {
        /// Maximum allowed length.
        max: usize,
        /// Actual length provided.
        len: usize,
    },

    /// Chrono parsing error.
    #[error("DateTime parse error: {0}")]
    ChronoParse(String),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Compare by string representation for foreign error types
            (Error::Database(a), Error::Database(b)) => format!("{}", a) == format!("{}", b),
            (Error::InvalidIssueId(a), Error::InvalidIssueId(b)) => a == b,
            (Error::IssueNotFound(a), Error::IssueNotFound(b)) => a == b,
            (
                Error::CircularDependency {
                    issue_id: ai,
                    depends_on: ad,
                },
                Error::CircularDependency {
                    issue_id: bi,
                    depends_on: bd,
                },
            ) => ai == bi && ad == bd,
            (Error::OpenDependencies { count: a }, Error::OpenDependencies { count: b }) => a == b,
            (Error::InvalidPriority(a), Error::InvalidPriority(b)) => a == b,
            (Error::Serialization(a), Error::Serialization(b)) => {
                format!("{}", a) == format!("{}", b)
            }
            (Error::Cli(a), Error::Cli(b)) => a == b,
            (Error::Io(a), Error::Io(b)) => format!("{}", a) == format!("{}", b),
            (
                Error::InvalidStatusTransition { from: af, to: at },
                Error::InvalidStatusTransition { from: bf, to: bt },
            ) => af == bf && at == bt,
            (Error::DatabaseNotFound(a), Error::DatabaseNotFound(b)) => a == b,
            (Error::EmptyTitle, Error::EmptyTitle) => true,
            (
                Error::TitleTooLong { max: am, len: al },
                Error::TitleTooLong { max: bm, len: bl },
            ) => am == bm && al == bl,
            (Error::ChronoParse(a), Error::ChronoParse(b)) => a == b,
            _ => false,
        }
    }
}

/// Issue status values for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueStatusError {
    /// Open status.
    Open,
    /// In progress status.
    InProgress,
    /// Closed status.
    Closed,
    /// Blocked status.
    Blocked,
}

impl std::fmt::Display for IssueStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Closed => write!(f, "closed"),
            Self::Blocked => write!(f, "blocked"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidIssueId("bad-id".to_string());
        assert_eq!(err.to_string(), "Invalid issue ID format: bad-id");
    }

    #[test]
    fn test_circular_dependency_error() {
        let err = Error::CircularDependency {
            issue_id: "issue-1".to_string(),
            depends_on: "issue-2".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Dependency would create cycle: issue-1 -> issue-2"
        );
    }

    #[test]
    fn test_open_dependencies_error() {
        let err = Error::OpenDependencies { count: 3 };
        assert_eq!(
            err.to_string(),
            "Cannot close issue with 3 open dependencies"
        );
    }

    #[test]
    fn test_invalid_priority_error() {
        let err = Error::InvalidPriority(10);
        assert_eq!(err.to_string(), "Priority out of range (0-4): 10");
    }

    #[test]
    fn test_empty_title_error() {
        let err = Error::EmptyTitle;
        assert_eq!(err.to_string(), "Issue title cannot be empty");
    }

    #[test]
    fn test_title_too_long_error() {
        let err = Error::TitleTooLong { max: 500, len: 600 };
        assert_eq!(
            err.to_string(),
            "Issue title exceeds maximum length of 500 characters (got 600)"
        );
    }

    #[test]
    fn test_issue_status_display() {
        assert_eq!(IssueStatusError::Open.to_string(), "open");
        assert_eq!(IssueStatusError::InProgress.to_string(), "in_progress");
        assert_eq!(IssueStatusError::Closed.to_string(), "closed");
        assert_eq!(IssueStatusError::Blocked.to_string(), "blocked");
    }

    #[test]
    fn test_invalid_status_transition_error() {
        let err = Error::InvalidStatusTransition {
            from: IssueStatusError::Open,
            to: IssueStatusError::Blocked,
        };
        assert_eq!(
            err.to_string(),
            "Invalid status transition: Open -> Blocked"
        );
    }

    #[test]
    fn test_error_partial_eq() {
        // Test that PartialEq works for Error enum
        let err1 = Error::InvalidIssueId("test".to_string());
        let err2 = Error::InvalidIssueId("test".to_string());
        let err3 = Error::InvalidIssueId("other".to_string());
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);

        // Test CircularDependency
        let cd1 = Error::CircularDependency {
            issue_id: "a".to_string(),
            depends_on: "b".to_string(),
        };
        let cd2 = Error::CircularDependency {
            issue_id: "a".to_string(),
            depends_on: "b".to_string(),
        };
        assert_eq!(cd1, cd2);

        // Test EmptyTitle (unit variant)
        assert_eq!(Error::EmptyTitle, Error::EmptyTitle);

        // Test InvalidPriority
        assert_eq!(Error::InvalidPriority(5), Error::InvalidPriority(5));
    }
}
