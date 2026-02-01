//! Core data types for the issue tracker.
//!
//! This module defines all the core types used throughout the issue tracker,
//! including issue IDs, status, priority, and issue types.

use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum allowed title length for issues.
///
/// Matches the Beads database CHECK constraint.
pub const MAX_TITLE_LENGTH: usize = 500;

/// Issue identifier (e.g., "rustgram-client-abc").
///
/// This is a newtype wrapper around a String to provide type safety
/// and prevent mixing issue IDs with other string values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IssueId(String);

impl IssueId {
    /// Creates a new IssueId from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the ID is empty or contains only whitespace.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueId;
    ///
    /// let id = IssueId::new("rustgram-client-abc");
    /// assert!(id.is_ok());
    ///
    /// let empty_id = IssueId::new("");
    /// assert!(empty_id.is_err());
    /// ```
    pub fn new(id: impl Into<String>) -> Result<Self> {
        let id = id.into();
        let trimmed = id.trim();
        if trimmed.is_empty() {
            return Err(Error::InvalidIssueId(
                "Issue ID cannot be empty".to_string(),
            ));
        }
        Ok(IssueId(trimmed.to_string()))
    }

    /// Returns the issue ID as a string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueId;
    ///
    /// let id = IssueId::new("rustgram-client-abc").unwrap();
    /// assert_eq!(id.as_str(), "rustgram-client-abc");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the IssueId and returns the inner String.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueId;
    ///
    /// let id = IssueId::new("rustgram-client-abc").unwrap();
    /// let s: String = id.into_inner();
    /// assert_eq!(s, "rustgram-client-abc");
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for IssueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for IssueId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl<'a> TryFrom<&'a str> for IssueId {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        Self::new(value)
    }
}

impl From<IssueId> for String {
    fn from(id: IssueId) -> Self {
        id.0
    }
}

impl<'a> From<&'a IssueId> for &'a str {
    fn from(id: &'a IssueId) -> Self {
        id.as_str()
    }
}

/// Issue status matching Beads schema.
///
/// These are the four primary states an issue can be in during its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueStatus {
    /// Issue is open and not yet started.
    Open,
    /// Issue is currently being worked on.
    InProgress,
    /// Issue has been closed/resolved.
    Closed,
    /// Issue is blocked by dependencies.
    Blocked,
}

impl IssueStatus {
    /// Returns all possible status values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueStatus;
    ///
    /// let all = IssueStatus::all();
    /// assert_eq!(all.len(), 4);
    /// ```
    pub const fn all() -> [IssueStatus; 4] {
        [
            IssueStatus::Open,
            IssueStatus::InProgress,
            IssueStatus::Closed,
            IssueStatus::Blocked,
        ]
    }

    /// Returns the string representation of the status.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueStatus;
    ///
    /// assert_eq!(IssueStatus::Open.as_str(), "open");
    /// assert_eq!(IssueStatus::Closed.as_str(), "closed");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::InProgress => "in_progress",
            Self::Closed => "closed",
            Self::Blocked => "blocked",
        }
    }

    /// Parses a status from a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueStatus;
    ///
    /// assert_eq!(IssueStatus::parse_str("open"), Some(IssueStatus::Open));
    /// assert_eq!(IssueStatus::parse_str("closed"), Some(IssueStatus::Closed));
    /// assert_eq!(IssueStatus::parse_str("invalid"), None);
    /// ```
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "open" => Some(Self::Open),
            "in_progress" => Some(Self::InProgress),
            "closed" => Some(Self::Closed),
            "blocked" => Some(Self::Blocked),
            _ => None,
        }
    }
}

impl fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Priority level (0-4, matching Beads constraint).
///
/// Lower numbers indicate higher priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    /// Critical priority (P0).
    P0 = 0,
    /// High priority (P1).
    P1 = 1,
    /// Medium priority (P2).
    P2 = 2,
    /// Low priority (P3).
    P3 = 3,
    /// Lowest priority (P4).
    P4 = 4,
}

impl Priority {
    /// Returns all possible priority values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::Priority;
    ///
    /// let all = Priority::all();
    /// assert_eq!(all.len(), 5);
    /// ```
    pub const fn all() -> [Priority; 5] {
        [
            Priority::P0,
            Priority::P1,
            Priority::P2,
            Priority::P3,
            Priority::P4,
        ]
    }

    /// Returns the numeric value of the priority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::Priority;
    ///
    /// assert_eq!(Priority::P0.as_i32(), 0);
    /// assert_eq!(Priority::P4.as_i32(), 4);
    /// ```
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Creates a priority from a numeric value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not in the range 0-4.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::Priority;
    ///
    /// assert_eq!(Priority::from_i32(0).unwrap(), Priority::P0);
    /// assert_eq!(Priority::from_i32(4).unwrap(), Priority::P4);
    /// assert!(Priority::from_i32(10).is_err());
    /// ```
    pub fn from_i32(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Priority::P0),
            1 => Ok(Priority::P1),
            2 => Ok(Priority::P2),
            3 => Ok(Priority::P3),
            4 => Ok(Priority::P4),
            _ => Err(Error::InvalidPriority(value)),
        }
    }

    /// Returns the string representation of the priority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::Priority;
    ///
    /// assert_eq!(Priority::P0.as_str(), "P0");
    /// assert_eq!(Priority::P4.as_str(), "P4");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::P0 => "P0",
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
            Self::P4 => "P4",
        }
    }

    /// Parses a priority from a string (e.g., "P0", "P1").
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::Priority;
    ///
    /// assert_eq!(Priority::parse_str("P0"), Some(Priority::P0));
    /// assert_eq!(Priority::parse_str("P4"), Some(Priority::P4));
    /// assert_eq!(Priority::parse_str("invalid"), None);
    /// ```
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "P0" => Some(Self::P0),
            "P1" => Some(Self::P1),
            "P2" => Some(Self::P2),
            "P3" => Some(Self::P3),
            "P4" => Some(Self::P4),
            _ => None,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<i32> for Priority {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
        Self::from_i32(value)
    }
}

/// Issue type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueType {
    /// Standard task.
    Task,
    /// Epic (large work item with sub-tasks).
    Epic,
    /// Gate/verification checkpoint.
    Gate,
}

impl IssueType {
    /// Returns all possible issue type values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueType;
    ///
    /// let all = IssueType::all();
    /// assert_eq!(all.len(), 3);
    /// ```
    pub const fn all() -> [IssueType; 3] {
        [IssueType::Task, IssueType::Epic, IssueType::Gate]
    }

    /// Returns the string representation of the issue type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueType;
    ///
    /// assert_eq!(IssueType::Task.as_str(), "task");
    /// assert_eq!(IssueType::Epic.as_str(), "epic");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Epic => "epic",
            Self::Gate => "gate",
        }
    }

    /// Parses an issue type from a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueType;
    ///
    /// assert_eq!(IssueType::parse_str("task"), Some(IssueType::Task));
    /// assert_eq!(IssueType::parse_str("epic"), Some(IssueType::Epic));
    /// assert_eq!(IssueType::parse_str("invalid"), None);
    /// ```
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "task" => Some(Self::Task),
            "epic" => Some(Self::Epic),
            "gate" => Some(Self::Gate),
            _ => None,
        }
    }
}

impl fmt::Display for IssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Dependency relationship type between issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    /// Parent-child relationship (epic contains tasks).
    ParentChild,
    /// Blocking relationship (issue blocks another).
    Blocks,
    /// Discovered during execution.
    DiscoveredFrom,
    /// Related issues (non-blocking).
    Related,
}

impl DependencyType {
    /// Returns all possible dependency type values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::DependencyType;
    ///
    /// let all = DependencyType::all();
    /// assert_eq!(all.len(), 4);
    /// ```
    pub const fn all() -> [DependencyType; 4] {
        [
            DependencyType::ParentChild,
            DependencyType::Blocks,
            DependencyType::DiscoveredFrom,
            DependencyType::Related,
        ]
    }

    /// Returns the string representation of the dependency type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::DependencyType;
    ///
    /// assert_eq!(DependencyType::ParentChild.as_str(), "parent-child");
    /// assert_eq!(DependencyType::Blocks.as_str(), "blocks");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ParentChild => "parent-child",
            Self::Blocks => "blocks",
            Self::DiscoveredFrom => "discovered-from",
            Self::Related => "related",
        }
    }

    /// Parses a dependency type from a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::DependencyType;
    ///
    /// assert_eq!(DependencyType::parse_str("parent-child"), Some(DependencyType::ParentChild));
    /// assert_eq!(DependencyType::parse_str("blocks"), Some(DependencyType::Blocks));
    /// assert_eq!(DependencyType::parse_str("invalid"), None);
    /// ```
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "parent-child" => Some(Self::ParentChild),
            "blocks" => Some(Self::Blocks),
            "discovered-from" => Some(Self::DiscoveredFrom),
            "related" => Some(Self::Related),
            _ => None,
        }
    }
}

impl fmt::Display for DependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Complete issue representation.
///
/// This struct contains all the fields that can be stored in the Beads database
/// for a single issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Unique issue identifier.
    pub id: IssueId,
    /// Issue title (max 500 characters).
    pub title: String,
    /// Detailed description of the issue.
    pub description: String,
    /// Current status of the issue.
    pub status: IssueStatus,
    /// Priority level (0-4, lower is higher priority).
    pub priority: Priority,
    /// Type of issue (task, epic, or gate).
    pub issue_type: IssueType,
    /// Labels/tags associated with the issue.
    pub labels: Vec<String>,
    /// Assignee username or ID.
    pub assignee: Option<String>,
    /// Timestamp when the issue was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp when the issue was last updated.
    pub updated_at: DateTime<Utc>,
    /// Timestamp when the issue was closed (if closed).
    pub closed_at: Option<DateTime<Utc>>,
}

impl Issue {
    /// Creates a new issue with the given ID, title, and other properties.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The title is empty
    /// - The title exceeds MAX_TITLE_LENGTH
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{Issue, IssueId, IssueStatus, Priority, IssueType};
    /// use chrono::Utc;
    ///
    /// let id = IssueId::new("test-1").unwrap();
    /// let now = Utc::now();
    ///
    /// let issue = Issue::new(
    ///     id.clone(),
    ///     "Test Issue",
    ///     "Description",
    ///     IssueStatus::Open,
    ///     Priority::P1,
    ///     IssueType::Task,
    ///     vec!["bug".to_string()],
    ///     None,
    ///     now,
    /// ).unwrap();
    ///
    /// assert_eq!(issue.id.as_str(), "test-1");
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: IssueId,
        title: impl Into<String>,
        description: impl Into<String>,
        status: IssueStatus,
        priority: Priority,
        issue_type: IssueType,
        labels: Vec<String>,
        assignee: Option<String>,
        created_at: DateTime<Utc>,
    ) -> Result<Self> {
        let title = title.into();
        Self::validate_title(&title)?;
        let description = description.into();
        let updated_at = created_at;

        Ok(Self {
            id,
            title,
            description,
            status,
            priority,
            issue_type,
            labels,
            assignee,
            created_at,
            updated_at,
            closed_at: None,
        })
    }

    /// Validates that a title meets requirements.
    ///
    /// # Errors
    ///
    /// Returns an error if the title is empty or too long.
    pub fn validate_title(title: &str) -> Result<()> {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return Err(Error::EmptyTitle);
        }
        if trimmed.len() > MAX_TITLE_LENGTH {
            return Err(Error::TitleTooLong {
                max: MAX_TITLE_LENGTH,
                len: trimmed.len(),
            });
        }
        Ok(())
    }

    /// Returns true if the issue is closed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{Issue, IssueId, IssueStatus, Priority, IssueType};
    /// use chrono::Utc;
    ///
    /// # let id = IssueId::new("test-1").unwrap();
    /// # let now = Utc::now();
    /// # let mut issue = Issue::new(id, "Test", "Desc", IssueStatus::Open, Priority::P1, IssueType::Task, vec![], None, now).unwrap();
    /// assert!(!issue.is_closed());
    /// issue.status = IssueStatus::Closed;
    /// assert!(issue.is_closed());
    /// ```
    pub fn is_closed(&self) -> bool {
        self.status == IssueStatus::Closed
    }

    /// Returns true if the issue is blocked.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{Issue, IssueId, IssueStatus, Priority, IssueType};
    /// use chrono::Utc;
    ///
    /// # let id = IssueId::new("test-1").unwrap();
    /// # let now = Utc::now();
    /// # let mut issue = Issue::new(id, "Test", "Desc", IssueStatus::Open, Priority::P1, IssueType::Task, vec![], None, now).unwrap();
    /// assert!(!issue.is_blocked());
    /// issue.status = IssueStatus::Blocked;
    /// assert!(issue.is_blocked());
    /// ```
    pub fn is_blocked(&self) -> bool {
        self.status == IssueStatus::Blocked
    }
}

/// Dependency relationship between two issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// The issue that has the dependency.
    pub issue_id: IssueId,
    /// The issue that is depended on.
    pub depends_on_id: IssueId,
    /// Type of dependency relationship.
    pub dependency_type: DependencyType,
    /// Timestamp when the dependency was created.
    pub created_at: DateTime<Utc>,
}

impl Dependency {
    /// Creates a new dependency relationship.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{Dependency, DependencyType, IssueId};
    /// use chrono::Utc;
    ///
    /// let issue_id = IssueId::new("issue-1").unwrap();
    /// let depends_on = IssueId::new("issue-2").unwrap();
    /// let now = Utc::now();
    ///
    /// let dep = Dependency::new(issue_id.clone(), depends_on.clone(), DependencyType::Blocks, now);
    /// assert_eq!(dep.issue_id.as_str(), "issue-1");
    /// assert_eq!(dep.depends_on_id.as_str(), "issue-2");
    /// ```
    pub fn new(
        issue_id: IssueId,
        depends_on_id: IssueId,
        dependency_type: DependencyType,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            issue_id,
            depends_on_id,
            dependency_type,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_id_new_valid() {
        let id = IssueId::new("rustgram-client-abc").unwrap();
        assert_eq!(id.as_str(), "rustgram-client-abc");
    }

    #[test]
    fn test_issue_id_new_empty() {
        let result = IssueId::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_issue_id_new_whitespace_only() {
        let result = IssueId::new("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_issue_id_new_trims_whitespace() {
        let id = IssueId::new("  test-id  ").unwrap();
        assert_eq!(id.as_str(), "test-id");
    }

    #[test]
    fn test_issue_id_into_inner() {
        let id = IssueId::new("test-id").unwrap();
        let s: String = id.into_inner();
        assert_eq!(s, "test-id");
    }

    #[test]
    fn test_issue_id_display() {
        let id = IssueId::new("test-id").unwrap();
        assert_eq!(format!("{}", id), "test-id");
    }

    #[test]
    fn test_issue_id_try_from_string() {
        let id = IssueId::try_from("test-id".to_string()).unwrap();
        assert_eq!(id.as_str(), "test-id");
    }

    #[test]
    fn test_issue_id_try_from_str() {
        let id = IssueId::try_from("test-id").unwrap();
        assert_eq!(id.as_str(), "test-id");
    }

    #[test]
    fn test_issue_id_from_into_string() {
        let id = IssueId::new("test-id").unwrap();
        let s: String = id.into();
        assert_eq!(s, "test-id");
    }

    #[test]
    fn test_issue_id_hash() {
        use std::collections::HashSet;
        let id1 = IssueId::new("test-id").unwrap();
        let id2 = IssueId::new("test-id").unwrap();
        let id3 = IssueId::new("other-id").unwrap();

        let mut set = HashSet::new();
        set.insert(id1.clone());
        set.insert(id2);
        set.insert(id3.clone());

        assert_eq!(set.len(), 2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id3));
    }

    #[test]
    fn test_issue_status_all() {
        let all = IssueStatus::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&IssueStatus::Open));
        assert!(all.contains(&IssueStatus::InProgress));
        assert!(all.contains(&IssueStatus::Closed));
        assert!(all.contains(&IssueStatus::Blocked));
    }

    #[test]
    fn test_issue_status_as_str() {
        assert_eq!(IssueStatus::Open.as_str(), "open");
        assert_eq!(IssueStatus::InProgress.as_str(), "in_progress");
        assert_eq!(IssueStatus::Closed.as_str(), "closed");
        assert_eq!(IssueStatus::Blocked.as_str(), "blocked");
    }

    #[test]
    fn test_issue_status_parse_str() {
        assert_eq!(IssueStatus::parse_str("open"), Some(IssueStatus::Open));
        assert_eq!(
            IssueStatus::parse_str("in_progress"),
            Some(IssueStatus::InProgress)
        );
        assert_eq!(IssueStatus::parse_str("closed"), Some(IssueStatus::Closed));
        assert_eq!(
            IssueStatus::parse_str("blocked"),
            Some(IssueStatus::Blocked)
        );
        assert_eq!(IssueStatus::parse_str("invalid"), None);
    }

    #[test]
    fn test_issue_status_display() {
        assert_eq!(format!("{}", IssueStatus::Open), "open");
        assert_eq!(format!("{}", IssueStatus::Closed), "closed");
    }

    #[test]
    fn test_priority_all() {
        let all = Priority::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_priority_as_i32() {
        assert_eq!(Priority::P0.as_i32(), 0);
        assert_eq!(Priority::P1.as_i32(), 1);
        assert_eq!(Priority::P2.as_i32(), 2);
        assert_eq!(Priority::P3.as_i32(), 3);
        assert_eq!(Priority::P4.as_i32(), 4);
    }

    #[test]
    fn test_priority_from_i32_valid() {
        assert_eq!(Priority::from_i32(0).unwrap(), Priority::P0);
        assert_eq!(Priority::from_i32(1).unwrap(), Priority::P1);
        assert_eq!(Priority::from_i32(2).unwrap(), Priority::P2);
        assert_eq!(Priority::from_i32(3).unwrap(), Priority::P3);
        assert_eq!(Priority::from_i32(4).unwrap(), Priority::P4);
    }

    #[test]
    fn test_priority_from_i32_invalid() {
        assert!(Priority::from_i32(-1).is_err());
        assert!(Priority::from_i32(5).is_err());
        assert!(Priority::from_i32(10).is_err());
    }

    #[test]
    fn test_priority_as_str() {
        assert_eq!(Priority::P0.as_str(), "P0");
        assert_eq!(Priority::P1.as_str(), "P1");
        assert_eq!(Priority::P2.as_str(), "P2");
        assert_eq!(Priority::P3.as_str(), "P3");
        assert_eq!(Priority::P4.as_str(), "P4");
    }

    #[test]
    fn test_priority_parse_str() {
        assert_eq!(Priority::parse_str("P0"), Some(Priority::P0));
        assert_eq!(Priority::parse_str("P4"), Some(Priority::P4));
        assert_eq!(Priority::parse_str("invalid"), None);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::P0 < Priority::P1);
        assert!(Priority::P1 < Priority::P2);
        assert!(Priority::P2 < Priority::P3);
        assert!(Priority::P3 < Priority::P4);
    }

    #[test]
    fn test_priority_try_from_i32() {
        let p: Priority = 0i32.try_into().unwrap();
        assert_eq!(p, Priority::P0);
        assert!(Priority::try_from(10i32).is_err());
    }

    #[test]
    fn test_issue_type_all() {
        let all = IssueType::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_issue_type_as_str() {
        assert_eq!(IssueType::Task.as_str(), "task");
        assert_eq!(IssueType::Epic.as_str(), "epic");
        assert_eq!(IssueType::Gate.as_str(), "gate");
    }

    #[test]
    fn test_issue_type_parse_str() {
        assert_eq!(IssueType::parse_str("task"), Some(IssueType::Task));
        assert_eq!(IssueType::parse_str("epic"), Some(IssueType::Epic));
        assert_eq!(IssueType::parse_str("gate"), Some(IssueType::Gate));
        assert_eq!(IssueType::parse_str("invalid"), None);
    }

    #[test]
    fn test_dependency_type_all() {
        let all = DependencyType::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_dependency_type_as_str() {
        assert_eq!(DependencyType::ParentChild.as_str(), "parent-child");
        assert_eq!(DependencyType::Blocks.as_str(), "blocks");
        assert_eq!(DependencyType::DiscoveredFrom.as_str(), "discovered-from");
        assert_eq!(DependencyType::Related.as_str(), "related");
    }

    #[test]
    fn test_dependency_type_parse_str() {
        assert_eq!(
            DependencyType::parse_str("parent-child"),
            Some(DependencyType::ParentChild)
        );
        assert_eq!(
            DependencyType::parse_str("blocks"),
            Some(DependencyType::Blocks)
        );
        assert_eq!(
            DependencyType::parse_str("discovered-from"),
            Some(DependencyType::DiscoveredFrom)
        );
        assert_eq!(
            DependencyType::parse_str("related"),
            Some(DependencyType::Related)
        );
        assert_eq!(DependencyType::parse_str("invalid"), None);
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
            vec!["bug".to_string()],
            None,
            now,
        )
        .unwrap();

        assert_eq!(issue.id.as_str(), "test-1");
        assert_eq!(issue.title, "Test Issue");
        assert_eq!(issue.description, "Description");
        assert_eq!(issue.status, IssueStatus::Open);
        assert_eq!(issue.priority, Priority::P1);
        assert_eq!(issue.issue_type, IssueType::Task);
        assert_eq!(issue.labels, vec!["bug"]);
        assert!(issue.assignee.is_none());
        assert_eq!(issue.created_at, now);
        assert_eq!(issue.updated_at, now);
        assert!(issue.closed_at.is_none());
    }

    #[test]
    fn test_issue_new_empty_title() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let result = Issue::new(
            id,
            "",
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Task,
            vec![],
            None,
            now,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_issue_new_whitespace_only_title() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let result = Issue::new(
            id,
            "   ",
            "Description",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Task,
            vec![],
            None,
            now,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_issue_new_title_too_long() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let long_title = "a".repeat(501);
        let result = Issue::new(
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
        assert!(result.is_err());
    }

    #[test]
    fn test_issue_validate_title() {
        assert!(Issue::validate_title("Valid title").is_ok());
        assert!(Issue::validate_title("  Valid title  ").is_ok());
        assert!(Issue::validate_title("").is_err());
        assert!(Issue::validate_title("   ").is_err());
    }

    #[test]
    fn test_issue_is_closed() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let mut issue = Issue::new(
            id,
            "Test",
            "Desc",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Task,
            vec![],
            None,
            now,
        )
        .unwrap();

        assert!(!issue.is_closed());
        issue.status = IssueStatus::Closed;
        assert!(issue.is_closed());
    }

    #[test]
    fn test_issue_is_blocked() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let mut issue = Issue::new(
            id,
            "Test",
            "Desc",
            IssueStatus::Open,
            Priority::P1,
            IssueType::Task,
            vec![],
            None,
            now,
        )
        .unwrap();

        assert!(!issue.is_blocked());
        issue.status = IssueStatus::Blocked;
        assert!(issue.is_blocked());
    }

    #[test]
    fn test_dependency_new() {
        let issue_id = IssueId::new("issue-1").unwrap();
        let depends_on = IssueId::new("issue-2").unwrap();
        let now = Utc::now();

        let dep = Dependency::new(
            issue_id.clone(),
            depends_on.clone(),
            DependencyType::Blocks,
            now,
        );

        assert_eq!(dep.issue_id.as_str(), "issue-1");
        assert_eq!(dep.depends_on_id.as_str(), "issue-2");
        assert_eq!(dep.dependency_type, DependencyType::Blocks);
        assert_eq!(dep.created_at, now);
    }

    #[test]
    fn test_issue_serialization() {
        let id = IssueId::new("test-1").unwrap();
        let now = Utc::now();
        let issue = Issue::new(
            id.clone(),
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
        let deserialized: Issue = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id.as_str(), "test-1");
        assert_eq!(deserialized.title, "Test Issue");
        assert_eq!(deserialized.assignee, Some("user".to_string()));
    }

    #[test]
    fn test_dependency_serialization() {
        let issue_id = IssueId::new("issue-1").unwrap();
        let depends_on = IssueId::new("issue-2").unwrap();
        let now = Utc::now();

        let dep = Dependency::new(
            issue_id.clone(),
            depends_on.clone(),
            DependencyType::Blocks,
            now,
        );

        let json = serde_json::to_string(&dep).unwrap();
        let deserialized: Dependency = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.issue_id.as_str(), "issue-1");
        assert_eq!(deserialized.depends_on_id.as_str(), "issue-2");
    }

    #[test]
    fn test_priority_serialization() {
        let p = Priority::P1;
        let json = serde_json::to_string(&p).unwrap();
        let deserialized: Priority = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Priority::P1);
    }

    #[test]
    fn test_issue_status_serialization() {
        let s = IssueStatus::InProgress;
        let json = serde_json::to_string(&s).unwrap();
        let deserialized: IssueStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, IssueStatus::InProgress);
    }

    #[test]
    fn test_issue_type_serialization() {
        let t = IssueType::Epic;
        let json = serde_json::to_string(&t).unwrap();
        let deserialized: IssueType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, IssueType::Epic);
    }

    #[test]
    fn test_dependency_type_serialization() {
        let d = DependencyType::Blocks;
        let json = serde_json::to_string(&d).unwrap();
        let deserialized: DependencyType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, DependencyType::Blocks);
    }
}
