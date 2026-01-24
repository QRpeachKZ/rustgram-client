//! # Rustgram Issue Tracker
//!
//! A native Rust crate providing type-safe interfaces to the Beads (bd) issue tracking system.
//!
//! ## Overview
//!
//! This crate provides:
//! - Type-safe interfaces for Beads database entities (Issue, Dependency, Label)
//! - Direct SQLite access for high-performance operations
//! - High-level workflow operations for epic management
//! - Full compatibility with Beads v0.46.0 database schema
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - **types**: Core data types (IssueId, IssueStatus, Priority, IssueType, etc.)
//! - **error**: Error types and Result alias
//! - **database**: Low-level database operations with SQLite
//! - **operations**: High-level workflow operations and IssueTracker
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use rustgram_issue_tracker::{IssueTracker, Issue, IssueId, IssueStatus, Priority, IssueType};
//! use chrono::Utc;
//!
//! // Open the default Beads database
//! let tracker = IssueTracker::new()?;
//!
//! // Or use a custom path
//! // let tracker = IssueTracker::with_path("/custom/path/beads.db")?;
//!
//! // Create a new issue
//! let id = IssueId::new("my-issue-1")?;
//! let issue = Issue::new(
//!     id.clone(),
//!     "Implement feature X",
//!     "Detailed description of the feature",
//!     IssueStatus::Open,
//!     Priority::P1,
//!     IssueType::Task,
//!     vec!["enhancement".to_string()],
//!     Some("username".to_string()),
//!     Utc::now(),
//! )?;
//!
//! tracker.create_issue(&issue)?;
//!
//! // Create an epic with standard workflow tasks
//! let (epic_id, task_ids) = tracker.create_epic_workflow(
//!     "New Feature",
//!     "Implement a new feature from start to finish",
//!     Priority::P1,
//!     vec!["feature".to_string()],
//! )?;
//!
//! // Get the next ready task
//! if let Some(task) = tracker.get_ready_task("step:planning")? {
//!     tracker.start_task(&task.id)?;
//!     // Work on task...
//!     tracker.complete_task(&task.id, "Planning complete")?;
//! }
//!
//! // Get epic progress
//! let progress = tracker.epic_progress(&epic_id)?;
//! println!("Epic is {:.0}% complete", progress.completion_percent());
//! # Ok::<(), rustgram_issue_tracker::Error>(())
//! ```
//!
//! ## Testing
//!
//! For testing, use the in-memory database:
//!
//! ```rust
//! use rustgram_issue_tracker::IssueTracker;
//!
//! let tracker = IssueTracker::in_memory()?;
//! tracker.init()?;
//! // Now you can use the tracker for testing...
//! # Ok::<(), rustgram_issue_tracker::Error>(())
//! ```
//!
//! ## Thread Safety
//!
//! The `IssueDatabase` and `IssueTracker` use `rusqlite::Connection` which
//! uses interior mutability. Each connection should be used from a single thread.
//! For concurrent access, create a connection per thread.
//!
//! ## Beads Compatibility
//!
//! This crate is compatible with Beads v0.46.0 database schema:
//! - Issues table with status, priority, type fields
//! - Dependencies table with parent-child, blocks relationships
//! - Labels table for flexible tagging

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]
#![allow(clippy::must_use_candidate)]

// Export public API
pub use database::{IssueDatabase, IssueFilter, IssueUpdate};
pub use error::{Error, Result};
pub use operations::{EpicProgress, IssueTracker};
pub use types::{
    Dependency, DependencyType, Issue, IssueId, IssueStatus, IssueType, Priority, MAX_TITLE_LENGTH,
};

mod database;
mod error;
mod operations;
mod types;

// Re-export chrono for convenience
pub use chrono;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_version_info() {
        // Basic sanity check that the crate is working
        assert!(!env!("CARGO_PKG_VERSION").is_empty());
    }

    #[test]
    fn test_max_title_length_constant() {
        // Ensure the constant matches Beads schema
        assert_eq!(MAX_TITLE_LENGTH, 500);
    }

    #[test]
    fn test_issue_id_new_valid() {
        let id = IssueId::new("test-123").unwrap();
        assert_eq!(id.as_str(), "test-123");
    }

    #[test]
    fn test_issue_status_all() {
        let all = IssueStatus::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_priority_all() {
        let all = Priority::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_issue_type_all() {
        let all = IssueType::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_dependency_type_all() {
        let all = DependencyType::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_tracker_in_memory() {
        let tracker = IssueTracker::in_memory();
        assert!(tracker.is_ok());
    }

    #[test]
    fn test_tracker_init() {
        let tracker = IssueTracker::in_memory().unwrap();
        let result = tracker.init();
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_display() {
        let err = Error::InvalidIssueId("test-id".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid issue ID format"));
        assert!(msg.contains("test-id"));
    }

    #[test]
    fn test_filter_default() {
        let filter = IssueFilter::default();
        assert!(filter.status.is_none());
        assert!(filter.priority.is_none());
        assert!(filter.issue_type.is_none());
        assert!(filter.assignee.is_none());
        assert!(filter.labels.is_empty());
        assert!(filter.parent_id.is_none());
    }

    #[test]
    fn test_update_default() {
        let update = IssueUpdate::default();
        assert!(update.title.is_none());
        assert!(update.description.is_none());
        assert!(update.status.is_none());
        assert!(update.priority.is_none());
        assert!(update.assignee.is_none());
    }

    #[test]
    fn test_issue_status_display() {
        assert_eq!(format!("{}", IssueStatus::Open), "open");
        assert_eq!(format!("{}", IssueStatus::InProgress), "in_progress");
        assert_eq!(format!("{}", IssueStatus::Closed), "closed");
        assert_eq!(format!("{}", IssueStatus::Blocked), "blocked");
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", Priority::P0), "P0");
        assert_eq!(format!("{}", Priority::P1), "P1");
        assert_eq!(format!("{}", Priority::P2), "P2");
        assert_eq!(format!("{}", Priority::P3), "P3");
        assert_eq!(format!("{}", Priority::P4), "P4");
    }

    #[test]
    fn test_issue_type_display() {
        assert_eq!(format!("{}", IssueType::Task), "task");
        assert_eq!(format!("{}", IssueType::Epic), "epic");
        assert_eq!(format!("{}", IssueType::Gate), "gate");
    }

    #[test]
    fn test_dependency_type_display() {
        assert_eq!(format!("{}", DependencyType::ParentChild), "parent-child");
        assert_eq!(format!("{}", DependencyType::Blocks), "blocks");
        assert_eq!(
            format!("{}", DependencyType::DiscoveredFrom),
            "discovered-from"
        );
        assert_eq!(format!("{}", DependencyType::Related), "related");
    }

    #[test]
    fn test_issue_id_display() {
        let id = IssueId::new("test-id").unwrap();
        assert_eq!(format!("{}", id), "test-id");
    }

    #[test]
    fn test_issue_new_valid() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let issue = Issue::new(
            id.clone(),
            "Test Issue",
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Task,
            vec![],
            None,
            now,
        );
        assert!(issue.is_ok());
    }

    #[test]
    fn test_issue_new_title_too_long() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let long_title = "a".repeat(501);
        let issue = Issue::new(
            id,
            long_title,
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Task,
            vec![],
            None,
            now,
        );
        assert!(issue.is_err());
    }

    #[test]
    fn test_epic_progress_completion() {
        let epic_id = IssueId::new("epic-1").unwrap();
        let progress = EpicProgress {
            epic_id,
            total_tasks: 10,
            completed_tasks: 5,
            blocked_tasks: 1,
            in_progress_tasks: 2,
            pending_tasks: 2,
        };
        assert_eq!(progress.completion_percent(), 50.0);
        assert!(!progress.is_complete());
        assert!(progress.has_blockers());
    }

    #[test]
    fn test_epic_progress_complete() {
        let epic_id = IssueId::new("epic-1").unwrap();
        let progress = EpicProgress {
            epic_id,
            total_tasks: 5,
            completed_tasks: 5,
            blocked_tasks: 0,
            in_progress_tasks: 0,
            pending_tasks: 0,
        };
        assert_eq!(progress.completion_percent(), 100.0);
        assert!(progress.is_complete());
        assert!(!progress.has_blockers());
    }

    #[test]
    fn test_issue_serialization() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let issue = Issue::new(
            id,
            "Test Issue",
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Task,
            vec!["bug".to_string()],
            Some("user".to_string()),
            now,
        )
        .unwrap();

        let json = serde_json::to_string(&issue).unwrap();
        let _deserialized: Issue = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_database_in_memory() {
        let db = IssueDatabase::in_memory();
        assert!(db.is_ok());
    }

    #[test]
    fn test_database_init_schema() {
        let db = IssueDatabase::in_memory().unwrap();
        let result = db.init_schema();
        assert!(result.is_ok());
    }
}
