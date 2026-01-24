//! High-level operations for the issue tracker.
//!
//! This module provides workflow helpers and high-level operations for
//! managing issues, including epic workflows, task status transitions,
//! and progress reporting.

use crate::database::{IssueDatabase, IssueFilter, IssueUpdate};
use crate::error::Result;
use crate::types::{DependencyType, Issue, IssueId, IssueStatus, IssueType, Priority};
use chrono::Utc;

/// High-level issue tracker with workflow operations.
///
/// Provides convenient methods for common issue tracking workflows
/// including epic creation, task management, and progress reporting.
pub struct IssueTracker {
    /// The underlying database connection.
    db: IssueDatabase,
}

impl IssueTracker {
    /// Creates a new IssueTracker with the default database path.
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be opened.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_issue_tracker::IssueTracker;
    ///
    /// let tracker = IssueTracker::new();
    /// assert!(tracker.is_ok());
    /// ```
    pub fn new() -> Result<Self> {
        let db = IssueDatabase::open()?;
        Ok(Self { db })
    }

    /// Creates a new IssueTracker with a custom database path.
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be opened.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_issue_tracker::IssueTracker;
    ///
    /// let tracker = IssueTracker::with_path("/custom/path/beads.db");
    /// assert!(tracker.is_ok());
    /// ```
    pub fn with_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let db = IssueDatabase::open_with_path(path)?;
        Ok(Self { db })
    }

    /// Creates an in-memory tracker for testing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueTracker;
    ///
    /// let tracker = IssueTracker::in_memory().unwrap();
    /// tracker.init().unwrap();
    /// ```
    pub fn in_memory() -> Result<Self> {
        let db = IssueDatabase::in_memory()?;
        Ok(Self { db })
    }

    /// Initializes the database schema.
    ///
    /// Creates the necessary tables for issues, dependencies, and labels.
    /// This should be called when setting up a new test database.
    ///
    /// # Errors
    ///
    /// Returns an error if table creation fails.
    pub fn init(&self) -> Result<()> {
        self.db.init_schema()
    }

    /// Creates an epic with a standard workflow of sub-tasks.
    ///
    /// This method creates an epic issue and a set of standard workflow tasks:
    /// - Planning (task)
    /// - Implementation (task)
    /// - Testing (task)
    /// - Documentation (task)
    ///
    /// All tasks are created as children of the epic with appropriate dependencies.
    ///
    /// # Errors
    ///
    /// Returns an error if any issue creation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueTracker;
    /// use rustgram_issue_tracker::Priority;
    ///
    /// # let tracker = IssueTracker::in_memory().unwrap();
    /// # tracker.init().unwrap();
    /// let (epic_id, task_ids) = tracker
    ///     .create_epic_workflow(
    ///         "New Feature",
    ///         "Implement a new feature",
    ///         Priority::P1,
    ///         vec!["feature".to_string()],
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(task_ids.len(), 4); // Planning, Implementation, Testing, Documentation
    /// ```
    pub fn create_epic_workflow(
        &self,
        title: &str,
        description: &str,
        priority: Priority,
        labels: Vec<String>,
    ) -> Result<(IssueId, Vec<IssueId>)> {
        let now = Utc::now();
        let epic_id = IssueId::new(format!("epic-{}", now.timestamp()))?;

        // Create the epic
        let epic = Issue::new(
            epic_id.clone(),
            title,
            description,
            IssueStatus::Open,
            priority,
            IssueType::Epic,
            labels.clone(),
            None,
            now,
        )?;
        self.db.create_issue(&epic)?;

        // Create standard workflow tasks
        let mut task_ids = Vec::new();
        let workflow_steps = [
            ("Planning", "Plan the implementation approach"),
            ("Implementation", "Implement the feature"),
            ("Testing", "Write and run tests"),
            ("Documentation", "Update documentation"),
        ];

        for (idx, (step_name, step_desc)) in workflow_steps.iter().enumerate() {
            let task_id = IssueId::new(format!("task-{}-{}", now.timestamp(), idx))?;
            let mut task_labels = labels.clone();
            task_labels.push(format!(
                "step:{}",
                step_name.to_lowercase().replace(' ', "-")
            ));

            let task = Issue::new(
                task_id.clone(),
                format!("{}: {}", title, step_name),
                *step_desc,
                IssueStatus::Open,
                priority,
                IssueType::Task,
                task_labels,
                None,
                now,
            )?;
            self.db.create_issue(&task)?;

            // Add parent-child dependency
            self.db
                .add_dependency(&task_id, &epic_id, DependencyType::ParentChild)?;

            task_ids.push(task_id);
        }

        Ok((epic_id, task_ids))
    }

    /// Gets the next ready task for a given workflow step label.
    ///
    /// A task is "ready" if it:
    /// - Has the specified step label
    /// - Is in Open status
    /// - Is not blocked by any dependencies
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueTracker;
    ///
    /// # let tracker = IssueTracker::in_memory().unwrap();
    /// # tracker.init().unwrap();
    /// let task = tracker.get_ready_task("step:planning").unwrap();
    /// assert!(task.is_none()); // No tasks created yet
    /// ```
    pub fn get_ready_task(&self, step_label: &str) -> Result<Option<Issue>> {
        let filter = IssueFilter::new()
            .with_status(IssueStatus::Open)
            .with_labels(vec![step_label.to_string()]);

        let tasks = self.db.list_issues(&filter)?;

        for task in tasks {
            if !self.db.is_blocked(&task.id)? {
                return Ok(Some(task));
            }
        }

        Ok(None)
    }

    /// Marks a task as in-progress.
    ///
    /// Updates the task status to InProgress and sets the updated_at timestamp.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The task doesn't exist
    /// - The database update fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueTracker, IssueId};
    ///
    /// # let tracker = IssueTracker::in_memory().unwrap();
    /// # tracker.init().unwrap();
    /// let id = IssueId::new("task-1").unwrap();
    ///
    /// # let result = tracker.start_task(&id);
    /// # assert!(result.is_err()); // Task doesn't exist
    /// ```
    pub fn start_task(&self, id: &IssueId) -> Result<()> {
        let mut updates = IssueUpdate::new();
        updates.status = Some(IssueStatus::InProgress);
        self.db.update_issue(id, &updates)
    }

    /// Completes a task with optional notes.
    ///
    /// Updates the task status to Closed and optionally appends notes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The task doesn't exist
    /// - The task has open blocking dependencies
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueTracker, IssueId};
    ///
    /// # let tracker = IssueTracker::in_memory().unwrap();
    /// # tracker.init().unwrap();
    /// let id = IssueId::new("task-1").unwrap();
    ///
    /// # let result = tracker.complete_task(&id, "All tests passed");
    /// # assert!(result.is_err()); // Task doesn't exist
    /// ```
    pub fn complete_task(&self, id: &IssueId, notes: &str) -> Result<()> {
        self.db.close_issue(id, notes)
    }

    /// Blocks a task with a reason.
    ///
    /// Updates the task status to Blocked to indicate it cannot proceed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The task doesn't exist
    /// - The database update fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueTracker, IssueId};
    ///
    /// # let tracker = IssueTracker::in_memory().unwrap();
    /// # tracker.init().unwrap();
    /// let id = IssueId::new("task-1").unwrap();
    ///
    /// # let result = tracker.block_task(&id, "Waiting for API review");
    /// # assert!(result.is_err()); // Task doesn't exist
    /// ```
    pub fn block_task(&self, id: &IssueId, reason: &str) -> Result<()> {
        let mut updates = IssueUpdate::new();
        updates.status = Some(IssueStatus::Blocked);
        updates.notes = Some(reason.to_string());
        self.db.update_issue(id, &updates)
    }

    /// Creates a discovered issue during task execution.
    ///
    /// Creates a new issue as a child of the parent issue with a
    /// DiscoveredFrom dependency relationship. This is useful for
    /// tracking bugs or additional work found during implementation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent issue doesn't exist
    /// - Issue creation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueTracker, IssueId, Priority, Issue, IssueStatus, IssueType};
    /// use chrono::Utc;
    ///
    /// # let tracker = IssueTracker::in_memory().unwrap();
    /// # tracker.init().unwrap();
    /// # let parent_id = IssueId::new("parent-1").unwrap();
    /// # let now = Utc::now();
    /// # let parent = Issue::new(
    /// #     parent_id.clone(), "Parent", "Desc", IssueStatus::Open,
    /// #     Priority::P1, IssueType::Task, vec![], None, now
    /// # ).unwrap();
    /// # tracker.create_issue(&parent).unwrap();
    /// let discovered_id = tracker
    ///     .create_discovered(&parent_id, "Found a bug", "Need to fix this")
    ///     .unwrap();
    /// ```
    pub fn create_discovered(
        &self,
        parent_id: &IssueId,
        title: &str,
        description: &str,
    ) -> Result<IssueId> {
        let now = Utc::now();
        let discovered_id = IssueId::new(format!("discovered-{}", now.timestamp()))?;

        let discovered = Issue::new(
            discovered_id.clone(),
            title,
            description,
            IssueStatus::Open,
            Priority::P2,
            IssueType::Task,
            vec!["discovered".to_string()],
            None,
            now,
        )?;
        self.db.create_issue(&discovered)?;

        // Add discovered-from dependency
        self.db
            .add_dependency(&discovered_id, parent_id, DependencyType::DiscoveredFrom)?;

        Ok(discovered_id)
    }

    /// Generates a progress report for an epic.
    ///
    /// Returns an EpicProgress struct containing counts of tasks in various states.
    ///
    /// # Errors
    ///
    /// Returns an error if the database queries fail.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueTracker, IssueId};
    ///
    /// # let tracker = IssueTracker::in_memory().unwrap();
    /// # tracker.init().unwrap();
    /// let epic_id = IssueId::new("epic-1").unwrap();
    ///
    /// // When epic doesn't exist or has no children, returns progress with 0 tasks
    /// let progress = tracker.epic_progress(&epic_id).unwrap();
    /// assert_eq!(progress.total_tasks, 0);
    /// ```
    pub fn epic_progress(&self, epic_id: &IssueId) -> Result<EpicProgress> {
        let filter = IssueFilter::new().with_parent_id(epic_id.clone());
        let tasks = self.db.list_issues(&filter)?;

        let total_tasks = tasks.len();
        let completed_tasks = tasks
            .iter()
            .filter(|t| t.status == IssueStatus::Closed)
            .count();
        let blocked_tasks = tasks
            .iter()
            .filter(|t| t.status == IssueStatus::Blocked)
            .count();
        let in_progress_tasks = tasks
            .iter()
            .filter(|t| t.status == IssueStatus::InProgress)
            .count();
        let pending_tasks = tasks
            .iter()
            .filter(|t| t.status == IssueStatus::Open)
            .count();

        Ok(EpicProgress {
            epic_id: epic_id.clone(),
            total_tasks,
            completed_tasks,
            blocked_tasks,
            in_progress_tasks,
            pending_tasks,
        })
    }

    /// Gets an issue by ID.
    ///
    /// Returns `None` if the issue doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub fn get_issue(&self, id: &IssueId) -> Result<Option<Issue>> {
        self.db.get_issue(id)
    }

    /// Lists issues with optional filters.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub fn list_issues(&self, filter: &IssueFilter) -> Result<Vec<Issue>> {
        self.db.list_issues(filter)
    }

    /// Creates a new issue.
    ///
    /// # Errors
    ///
    /// Returns an error if issue creation fails.
    pub fn create_issue(&self, issue: &Issue) -> Result<IssueId> {
        self.db.create_issue(issue)
    }

    /// Updates an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the update fails.
    pub fn update_issue(&self, id: &IssueId, updates: &IssueUpdate) -> Result<()> {
        self.db.update_issue(id, updates)
    }

    /// Adds a dependency between issues.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Either issue doesn't exist
    /// - The dependency would create a cycle
    pub fn add_dependency(
        &self,
        issue_id: &IssueId,
        depends_on: &IssueId,
        dep_type: DependencyType,
    ) -> Result<()> {
        self.db.add_dependency(issue_id, depends_on, dep_type)
    }

    /// Gets dependencies for an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub fn get_dependencies(&self, id: &IssueId) -> Result<Vec<crate::types::Dependency>> {
        self.db.get_dependencies(id)
    }

    /// Adds a label to an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the issue doesn't exist.
    pub fn add_label(&self, id: &IssueId, label: &str) -> Result<()> {
        self.db.add_label(id, label)
    }

    /// Removes a label from an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub fn remove_label(&self, id: &IssueId, label: &str) -> Result<()> {
        self.db.remove_label(id, label)
    }

    /// Checks if an issue is blocked by dependencies.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub fn is_blocked(&self, id: &IssueId) -> Result<bool> {
        self.db.is_blocked(id)
    }

    /// Closes an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The issue doesn't exist
    /// - The issue has open dependencies
    pub fn close_issue(&self, id: &IssueId, reason: &str) -> Result<()> {
        self.db.close_issue(id, reason)
    }
}

/// Progress report for an epic and its sub-tasks.
#[derive(Debug, Clone)]
pub struct EpicProgress {
    /// The epic issue ID.
    pub epic_id: IssueId,
    /// Total number of sub-tasks.
    pub total_tasks: usize,
    /// Number of completed tasks.
    pub completed_tasks: usize,
    /// Number of blocked tasks.
    pub blocked_tasks: usize,
    /// Number of in-progress tasks.
    pub in_progress_tasks: usize,
    /// Number of pending tasks.
    pub pending_tasks: usize,
}

impl EpicProgress {
    /// Calculates the completion percentage.
    ///
    /// Returns a value from 0.0 to 100.0 representing the percentage
    /// of completed tasks.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::EpicProgress;
    /// use rustgram_issue_tracker::IssueId;
    ///
    /// # let epic_id = IssueId::new("epic-1").unwrap();
    /// let progress = EpicProgress {
    ///     epic_id,
    ///     total_tasks: 10,
    ///     completed_tasks: 5,
    ///     blocked_tasks: 1,
    ///     in_progress_tasks: 2,
    ///     pending_tasks: 2,
    /// };
    ///
    /// assert_eq!(progress.completion_percent(), 50.0);
    /// ```
    pub fn completion_percent(&self) -> f64 {
        if self.total_tasks == 0 {
            return 0.0;
        }
        (self.completed_tasks as f64 / self.total_tasks as f64) * 100.0
    }

    /// Returns true if the epic is complete (all tasks closed).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::EpicProgress;
    /// use rustgram_issue_tracker::IssueId;
    ///
    /// # let epic_id = IssueId::new("epic-1").unwrap();
    /// let progress = EpicProgress {
    ///     epic_id,
    ///     total_tasks: 10,
    ///     completed_tasks: 10,
    ///     blocked_tasks: 0,
    ///     in_progress_tasks: 0,
    ///     pending_tasks: 0,
    /// };
    ///
    /// assert!(progress.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.total_tasks > 0 && self.completed_tasks == self.total_tasks
    }

    /// Returns true if the epic has any blocked tasks.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::EpicProgress;
    /// use rustgram_issue_tracker::IssueId;
    ///
    /// # let epic_id = IssueId::new("epic-1").unwrap();
    /// let progress = EpicProgress {
    ///     epic_id,
    ///     total_tasks: 10,
    ///     completed_tasks: 5,
    ///     blocked_tasks: 1,
    ///     in_progress_tasks: 2,
    ///     pending_tasks: 2,
    /// };
    ///
    /// assert!(progress.has_blockers());
    /// ```
    pub fn has_blockers(&self) -> bool {
        self.blocked_tasks > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Issue;

    fn create_test_tracker() -> IssueTracker {
        let tracker = IssueTracker::in_memory().unwrap();
        tracker.init().unwrap();
        tracker
    }

    fn create_test_issue(id: &str, title: &str) -> Issue {
        let issue_id = IssueId::new(id).unwrap();
        let now = Utc::now();
        Issue::new(
            issue_id,
            title,
            "Description",
            IssueStatus::Open,
            Priority::P2,
            IssueType::Task,
            vec![],
            None,
            now,
        )
        .unwrap()
    }

    #[test]
    fn test_tracker_new() {
        let result = IssueTracker::in_memory();
        assert!(result.is_ok());
    }

    #[test]
    fn test_tracker_init() {
        let _tracker = create_test_tracker();
        // init() already called in create_test_tracker
        // If we got here, init() succeeded
    }

    #[test]
    fn test_create_epic_workflow() {
        let tracker = create_test_tracker();
        let (epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        assert_eq!(task_ids.len(), 4);

        // Verify epic was created
        let epic = tracker.get_issue(&epic_id).unwrap().unwrap();
        assert_eq!(epic.title, "New Feature");
        assert_eq!(epic.issue_type, IssueType::Epic);
    }

    #[test]
    fn test_create_epic_workflow_tasks_exist() {
        let tracker = create_test_tracker();
        let (epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        // Verify all tasks exist
        for task_id in &task_ids {
            let task = tracker.get_issue(task_id).unwrap().unwrap();
            assert!(task.title.contains("New Feature"));
        }

        // Verify tasks are children of epic
        let filter = IssueFilter::new().with_parent_id(epic_id);
        let children = tracker.list_issues(&filter).unwrap();
        assert_eq!(children.len(), 4);
    }

    #[test]
    fn test_get_ready_task_none() {
        let tracker = create_test_tracker();
        let task = tracker.get_ready_task("step:planning").unwrap();
        assert!(task.is_none());
    }

    #[test]
    fn test_get_ready_task_exists() {
        let tracker = create_test_tracker();
        let (_epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        let task = tracker.get_ready_task("step:planning").unwrap();
        assert!(task.is_some());
        assert_eq!(task.unwrap().id, task_ids[0]);
    }

    #[test]
    fn test_get_ready_task_blocked() {
        let tracker = create_test_tracker();
        let (_epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        // Create a blocking dependency
        let blocker = create_test_issue("blocker", "Blocker");
        tracker.create_issue(&blocker).unwrap();
        tracker
            .add_dependency(&task_ids[0], &blocker.id, DependencyType::Blocks)
            .unwrap();

        let task = tracker.get_ready_task("step:planning").unwrap();
        assert!(task.is_none()); // Task is blocked
    }

    #[test]
    fn test_start_task() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("task-1", "Test Task");
        tracker.create_issue(&issue).unwrap();

        tracker.start_task(&issue.id).unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.status, IssueStatus::InProgress);
    }

    #[test]
    fn test_start_task_not_exists() {
        let tracker = create_test_tracker();
        let id = IssueId::new("nonexistent").unwrap();

        let result = tracker.start_task(&id);
        assert!(result.is_err());
    }

    #[test]
    fn test_complete_task() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("task-1", "Test Task");
        tracker.create_issue(&issue).unwrap();

        tracker.complete_task(&issue.id, "Done").unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.status, IssueStatus::Closed);
        assert!(retrieved.closed_at.is_some());
    }

    #[test]
    fn test_complete_task_not_exists() {
        let tracker = create_test_tracker();
        let id = IssueId::new("nonexistent").unwrap();

        let result = tracker.complete_task(&id, "Done");
        assert!(result.is_err());
    }

    #[test]
    fn test_block_task() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("task-1", "Test Task");
        tracker.create_issue(&issue).unwrap();

        tracker.block_task(&issue.id, "Waiting for review").unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.status, IssueStatus::Blocked);
    }

    #[test]
    fn test_block_task_not_exists() {
        let tracker = create_test_tracker();
        let id = IssueId::new("nonexistent").unwrap();

        let result = tracker.block_task(&id, "Blocked");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_discovered() {
        let tracker = create_test_tracker();
        let parent = create_test_issue("parent-1", "Parent Task");
        tracker.create_issue(&parent).unwrap();

        let discovered_id = tracker
            .create_discovered(&parent.id, "Found bug", "Need to fix")
            .unwrap();

        let discovered = tracker.get_issue(&discovered_id).unwrap().unwrap();
        assert_eq!(discovered.title, "Found bug");
        assert!(discovered.labels.contains(&"discovered".to_string()));

        // Check dependency
        let deps = tracker.get_dependencies(&discovered_id).unwrap();
        assert!(deps
            .iter()
            .any(|d| d.depends_on_id == parent.id
                && d.dependency_type == DependencyType::DiscoveredFrom));
    }

    #[test]
    fn test_create_discovered_parent_not_exists() {
        let tracker = create_test_tracker();
        let parent_id = IssueId::new("nonexistent").unwrap();

        let result = tracker.create_discovered(&parent_id, "Found bug", "Need to fix");
        assert!(result.is_err());
    }

    #[test]
    fn test_epic_progress_empty() {
        let tracker = create_test_tracker();
        let epic_id = IssueId::new("epic-1").unwrap();

        // Create epic without tasks
        let epic = Issue::new(
            epic_id.clone(),
            "Epic",
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Epic,
            vec![],
            None,
            Utc::now(),
        )
        .unwrap();
        tracker.create_issue(&epic).unwrap();

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert_eq!(progress.total_tasks, 0);
        assert_eq!(progress.completed_tasks, 0);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_epic_progress_partial() {
        let tracker = create_test_tracker();
        let (epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        // Complete first task
        tracker.complete_task(&task_ids[0], "Done").unwrap();

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert_eq!(progress.total_tasks, 4);
        assert_eq!(progress.completed_tasks, 1);
        assert_eq!(progress.pending_tasks, 3);
        assert_eq!(progress.completion_percent(), 25.0);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_epic_progress_complete() {
        let tracker = create_test_tracker();
        let (epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        // Complete all tasks
        for task_id in &task_ids {
            tracker.complete_task(task_id, "Done").unwrap();
        }

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert_eq!(progress.total_tasks, 4);
        assert_eq!(progress.completed_tasks, 4);
        assert_eq!(progress.completion_percent(), 100.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_epic_progress_with_blocked() {
        let tracker = create_test_tracker();
        let (epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        // Block a task
        tracker.block_task(&task_ids[0], "Blocked").unwrap();

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert_eq!(progress.total_tasks, 4);
        assert_eq!(progress.blocked_tasks, 1);
        assert!(progress.has_blockers());
    }

    #[test]
    fn test_epic_progress_with_in_progress() {
        let tracker = create_test_tracker();
        let (epic_id, task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        // Start a task
        tracker.start_task(&task_ids[0]).unwrap();

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert_eq!(progress.total_tasks, 4);
        assert_eq!(progress.in_progress_tasks, 1);
        assert_eq!(progress.pending_tasks, 3);
    }

    #[test]
    fn test_get_issue() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("test-1", "Test Issue");
        tracker.create_issue(&issue).unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.title, "Test Issue");
    }

    #[test]
    fn test_get_issue_not_exists() {
        let tracker = create_test_tracker();
        let id = IssueId::new("nonexistent").unwrap();

        let retrieved = tracker.get_issue(&id).unwrap();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_list_issues() {
        let tracker = create_test_tracker();
        let issue1 = create_test_issue("test-1", "First");
        let issue2 = create_test_issue("test-2", "Second");

        tracker.create_issue(&issue1).unwrap();
        tracker.create_issue(&issue2).unwrap();

        let filter = IssueFilter::default();
        let issues = tracker.list_issues(&filter).unwrap();
        assert_eq!(issues.len(), 2);
    }

    #[test]
    fn test_create_issue() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("test-1", "Test Issue");

        let created_id = tracker.create_issue(&issue).unwrap();
        assert_eq!(created_id.as_str(), "test-1");
    }

    #[test]
    fn test_update_issue() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("test-1", "Original");
        tracker.create_issue(&issue).unwrap();

        let mut updates = IssueUpdate::new();
        updates.title = Some("Updated".to_string());

        tracker.update_issue(&issue.id, &updates).unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.title, "Updated");
    }

    #[test]
    fn test_add_dependency() {
        let tracker = create_test_tracker();
        let issue1 = create_test_issue("test-1", "First");
        let issue2 = create_test_issue("test-2", "Second");

        tracker.create_issue(&issue1).unwrap();
        tracker.create_issue(&issue2).unwrap();

        tracker
            .add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        let deps = tracker.get_dependencies(&issue1.id).unwrap();
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn test_get_dependencies() {
        let tracker = create_test_tracker();
        let issue1 = create_test_issue("test-1", "First");
        let issue2 = create_test_issue("test-2", "Second");

        tracker.create_issue(&issue1).unwrap();
        tracker.create_issue(&issue2).unwrap();

        tracker
            .add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        let deps = tracker.get_dependencies(&issue1.id).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].depends_on_id.as_str(), "test-2");
    }

    #[test]
    fn test_add_label() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("test-1", "Test");
        tracker.create_issue(&issue).unwrap();

        tracker.add_label(&issue.id, "bug").unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert!(retrieved.labels.contains(&"bug".to_string()));
    }

    #[test]
    fn test_remove_label() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("test-1", "Test");
        tracker.create_issue(&issue).unwrap();

        tracker.add_label(&issue.id, "bug").unwrap();
        tracker.remove_label(&issue.id, "bug").unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert!(!retrieved.labels.contains(&"bug".to_string()));
    }

    #[test]
    fn test_is_blocked() {
        let tracker = create_test_tracker();
        let issue1 = create_test_issue("test-1", "First");
        let issue2 = create_test_issue("test-2", "Second");

        tracker.create_issue(&issue1).unwrap();
        tracker.create_issue(&issue2).unwrap();

        tracker
            .add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        assert!(tracker.is_blocked(&issue1.id).unwrap());
        assert!(!tracker.is_blocked(&issue2.id).unwrap());
    }

    #[test]
    fn test_close_issue() {
        let tracker = create_test_tracker();
        let issue = create_test_issue("test-1", "Test");
        tracker.create_issue(&issue).unwrap();

        tracker.close_issue(&issue.id, "Done").unwrap();

        let retrieved = tracker.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.status, IssueStatus::Closed);
    }

    #[test]
    fn test_epic_progress_completion_percent_zero_tasks() {
        let tracker = create_test_tracker();
        let epic_id = IssueId::new("epic-1").unwrap();

        let epic = Issue::new(
            epic_id.clone(),
            "Epic",
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Epic,
            vec![],
            None,
            Utc::now(),
        )
        .unwrap();
        tracker.create_issue(&epic).unwrap();

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert_eq!(progress.completion_percent(), 0.0);
    }

    #[test]
    fn test_epic_progress_is_complete_false() {
        let tracker = create_test_tracker();
        let epic_id = IssueId::new("epic-1").unwrap();

        let epic = Issue::new(
            epic_id.clone(),
            "Epic",
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Epic,
            vec![],
            None,
            Utc::now(),
        )
        .unwrap();
        tracker.create_issue(&epic).unwrap();

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_epic_progress_has_blockers_false() {
        let tracker = create_test_tracker();
        let (epic_id, _task_ids) = tracker
            .create_epic_workflow(
                "New Feature",
                "Implement a new feature",
                Priority::P1,
                vec!["feature".to_string()],
            )
            .unwrap();

        let progress = tracker.epic_progress(&epic_id).unwrap();
        assert!(!progress.has_blockers());
    }

    #[test]
    fn test_complete_task_with_open_blocking_deps_fails() {
        let tracker = create_test_tracker();
        let issue1 = create_test_issue("test-1", "First");
        let issue2 = create_test_issue("test-2", "Second");

        tracker.create_issue(&issue1).unwrap();
        tracker.create_issue(&issue2).unwrap();

        tracker
            .add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        let result = tracker.complete_task(&issue1.id, "Done");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_discovered_has_correct_labels() {
        let tracker = create_test_tracker();
        let parent = create_test_issue("parent-1", "Parent Task");
        tracker.create_issue(&parent).unwrap();

        let discovered_id = tracker
            .create_discovered(&parent.id, "Found bug", "Need to fix")
            .unwrap();

        let discovered = tracker.get_issue(&discovered_id).unwrap().unwrap();
        assert!(discovered.labels.contains(&"discovered".to_string()));
    }
}
