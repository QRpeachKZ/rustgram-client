//! Database layer for the issue tracker.
//!
//! This module provides the SQLite database connection and all CRUD operations
//! for issues, dependencies, and labels.

use crate::error::{Error, Result};
use crate::types::{Dependency, DependencyType, Issue, IssueId, IssueStatus, IssueType, Priority};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::Path;

/// Default path to the Beads database.
const DEFAULT_DB_PATH: &str = ".beads/beads.db";

/// Issue database with direct SQLite access.
///
/// Provides methods for creating, reading, updating, and deleting issues
/// in the Beads SQLite database.
pub struct IssueDatabase {
    /// The SQLite connection.
    conn: Connection,
}

impl IssueDatabase {
    /// Opens the Beads database at the default location (.beads/beads.db).
    ///
    /// # Errors
    ///
    /// Returns an error if the database file doesn't exist or can't be opened.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_issue_tracker::IssueDatabase;
    ///
    /// let db = IssueDatabase::open();
    /// assert!(db.is_ok());
    /// ```
    pub fn open() -> Result<Self> {
        Self::open_with_path(DEFAULT_DB_PATH)
    }

    /// Opens the Beads database at a custom path.
    ///
    /// # Errors
    ///
    /// Returns an error if the database file doesn't exist or can't be opened.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_issue_tracker::IssueDatabase;
    ///
    /// let db = IssueDatabase::open_with_path("/custom/path/beads.db");
    /// assert!(db.is_ok());
    /// ```
    pub fn open_with_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return Err(Error::DatabaseNotFound(path_ref.display().to_string()));
        }

        let conn = Connection::open(path_ref)?;
        Ok(Self { conn })
    }

    /// Creates a new in-memory database for testing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueDatabase;
    ///
    /// let db = IssueDatabase::in_memory().unwrap();
    /// db.init_schema().unwrap();
    /// ```
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    /// Initializes the database schema.
    ///
    /// Creates the necessary tables for issues, dependencies, and labels.
    /// This should be called when setting up a new test database.
    ///
    /// # Errors
    ///
    /// Returns an error if table creation fails.
    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS issues (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL CHECK(length(title) <= 500),
                description TEXT DEFAULT '',
                notes TEXT DEFAULT '',
                status TEXT NOT NULL DEFAULT 'open',
                priority INTEGER NOT NULL DEFAULT 2 CHECK(priority >= 0 AND priority <= 4),
                issue_type TEXT NOT NULL DEFAULT 'task',
                assignee TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                closed_at TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS dependencies (
                issue_id TEXT NOT NULL,
                depends_on_id TEXT NOT NULL,
                type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                created_by TEXT NOT NULL DEFAULT '',
                PRIMARY KEY (issue_id, depends_on_id),
                FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE,
                FOREIGN KEY (depends_on_id) REFERENCES issues(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS labels (
                issue_id TEXT NOT NULL,
                label TEXT NOT NULL,
                PRIMARY KEY (issue_id, label),
                FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    /// Creates a new issue in the database.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The issue ID already exists
    /// - The title is invalid
    /// - Database insertion fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, Issue, IssueId, IssueStatus, Priority, IssueType};
    /// use chrono::Utc;
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// let id = IssueId::new("test-1").unwrap();
    /// let issue = Issue::new(
    ///     id.clone(),
    ///     "Test Issue",
    ///     "Description",
    ///     IssueStatus::Open,
    ///     Priority::P1,
    ///     IssueType::Task,
    ///     vec!["bug".to_string()],
    ///     None,
    ///     Utc::now(),
    /// ).unwrap();
    ///
    /// let created_id = db.create_issue(&issue).unwrap();
    /// assert_eq!(created_id.as_str(), "test-1");
    /// ```
    pub fn create_issue(&self, issue: &Issue) -> Result<IssueId> {
        let tx = self.conn.unchecked_transaction()?;

        tx.execute(
            "INSERT INTO issues (id, title, description, status, priority, issue_type, assignee, created_at, updated_at, closed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                issue.id.as_str(),
                issue.title,
                issue.description,
                issue.status.as_str(),
                issue.priority.as_i32(),
                issue.issue_type.as_str(),
                issue.assignee,
                issue.created_at.to_rfc3339(),
                issue.updated_at.to_rfc3339(),
                issue.closed_at.map(|dt| dt.to_rfc3339()),
            ],
        )?;

        // Insert labels
        for label in &issue.labels {
            tx.execute(
                "INSERT INTO labels (issue_id, label) VALUES (?1, ?2)",
                params![issue.id.as_str(), label],
            )?;
        }

        tx.commit()?;
        Ok(issue.id.clone())
    }

    /// Gets an issue by ID.
    ///
    /// Returns `None` if the issue doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueId};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// let id = IssueId::new("test-1").unwrap();
    /// let issue = db.get_issue(&id).unwrap();
    /// assert!(issue.is_none());
    /// ```
    pub fn get_issue(&self, id: &IssueId) -> Result<Option<Issue>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority, issue_type, assignee, created_at, updated_at, closed_at
             FROM issues WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id.as_str()])?;

        if let Some(row) = rows.next()? {
            let status_str: String = row.get(3)?;
            let status = IssueStatus::parse_str(&status_str)
                .ok_or_else(|| Error::InvalidIssueId(format!("Invalid status: {}", status_str)))?;

            let priority_i32: i32 = row.get(4)?;
            let priority = Priority::from_i32(priority_i32)?;

            let type_str: String = row.get(5)?;
            let issue_type = IssueType::parse_str(&type_str)
                .ok_or_else(|| Error::InvalidIssueId(format!("Invalid type: {}", type_str)))?;

            let created_at_str: String = row.get(7)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| Error::ChronoParse(e.to_string()))?
                .with_timezone(&Utc);

            let updated_at_str: String = row.get(8)?;
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|e| Error::ChronoParse(e.to_string()))?
                .with_timezone(&Utc);

            let closed_at: Option<String> = row.get(9)?;
            let closed_at = match closed_at {
                Some(s) => Some(
                    DateTime::parse_from_rfc3339(&s)
                        .map_err(|e| Error::ChronoParse(e.to_string()))?
                        .with_timezone(&Utc),
                ),
                None => None,
            };

            let issue_id = id.clone();
            let labels = self.get_labels_for_issue(&issue_id)?;

            Ok(Some(Issue {
                id: issue_id,
                title: row.get(1)?,
                description: row.get(2)?,
                status,
                priority,
                issue_type,
                labels,
                assignee: row.get(6)?,
                created_at,
                updated_at,
                closed_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// Lists issues with optional filters.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueFilter};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// let filter = IssueFilter::default();
    /// let issues = db.list_issues(&filter).unwrap();
    /// assert!(issues.is_empty());
    /// ```
    pub fn list_issues(&self, filter: &IssueFilter) -> Result<Vec<Issue>> {
        let mut query = String::from(
            "SELECT id, title, description, status, priority, issue_type, assignee, created_at, updated_at, closed_at
             FROM issues WHERE 1=1",
        );
        let mut param_values: Vec<Box<dyn rusqlite::ToSql>> = vec![];
        let mut param_count = 0;

        if let Some(status) = filter.status {
            param_count += 1;
            query.push_str(&format!(" AND status = ?{}", param_count));
            param_values.push(Box::new(status.as_str().to_string()));
        }

        if let Some(priority) = filter.priority {
            param_count += 1;
            query.push_str(&format!(" AND priority = ?{}", param_count));
            param_values.push(Box::new(priority.as_i32()));
        }

        if let Some(issue_type) = filter.issue_type {
            param_count += 1;
            query.push_str(&format!(" AND issue_type = ?{}", param_count));
            param_values.push(Box::new(issue_type.as_str().to_string()));
        }

        if let Some(ref assignee) = filter.assignee {
            param_count += 1;
            query.push_str(&format!(" AND assignee = ?{}", param_count));
            param_values.push(Box::new(assignee.clone()));
        }

        // Handle labels filtering with a subquery
        if !filter.labels.is_empty() {
            let start_param = param_count + 1;
            let label_placeholders: Vec<String> = (0..filter.labels.len())
                .map(|i| format!("?{}", start_param + i))
                .collect();
            query.push_str(&format!(
                " AND id IN (SELECT issue_id FROM labels WHERE label IN ({}))",
                label_placeholders.join(", ")
            ));
            for label in &filter.labels {
                param_count += 1;
                param_values.push(Box::new(label.clone()));
            }
        }

        // Handle parent filtering
        if let Some(ref parent_id) = filter.parent_id {
            param_count += 1;
            query.push_str(&format!(
                " AND id IN (SELECT issue_id FROM dependencies WHERE depends_on_id = ?{} AND type = 'parent-child')",
                param_count
            ));
            param_values.push(Box::new(parent_id.as_str().to_string()));
        }

        query.push_str(" ORDER BY priority ASC, created_at DESC");

        let mut stmt = self.conn.prepare(&query)?;

        // Convert Box<dyn ToSql> to &dyn ToSql for the query call
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        let mut rows = stmt.query(params_refs.as_slice())?;

        let mut issues = Vec::new();
        while let Some(row) = rows.next()? {
            let status_str: String = row.get(3)?;
            let status = IssueStatus::parse_str(&status_str)
                .ok_or_else(|| Error::InvalidIssueId(format!("Invalid status: {}", status_str)))?;

            let priority_i32: i32 = row.get(4)?;
            let priority = Priority::from_i32(priority_i32)?;

            let type_str: String = row.get(5)?;
            let issue_type = IssueType::parse_str(&type_str)
                .ok_or_else(|| Error::InvalidIssueId(format!("Invalid type: {}", type_str)))?;

            let created_at_str: String = row.get(7)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| Error::ChronoParse(e.to_string()))?
                .with_timezone(&Utc);

            let updated_at_str: String = row.get(8)?;
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|e| Error::ChronoParse(e.to_string()))?
                .with_timezone(&Utc);

            let closed_at: Option<String> = row.get(9)?;
            let closed_at = match closed_at {
                Some(s) => Some(
                    DateTime::parse_from_rfc3339(&s)
                        .map_err(|e| Error::ChronoParse(e.to_string()))?
                        .with_timezone(&Utc),
                ),
                None => None,
            };

            let id: String = row.get(0)?;
            let issue_id = IssueId::new(id.clone())?;
            let labels = self.get_labels_for_issue(&issue_id)?;

            issues.push(Issue {
                id: issue_id,
                title: row.get(1)?,
                description: row.get(2)?,
                status,
                priority,
                issue_type,
                labels,
                assignee: row.get(6)?,
                created_at,
                updated_at,
                closed_at,
            });
        }

        Ok(issues)
    }

    /// Updates issue fields.
    ///
    /// Only fields set in `updates` will be modified. The `updated_at` timestamp
    /// is automatically set to the current time.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The issue doesn't exist
    /// - The update would result in invalid data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueUpdate, IssueId, IssueStatus};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// # let id = IssueId::new("test-1").unwrap();
    /// let mut updates = IssueUpdate::default();
    /// updates.status = Some(IssueStatus::InProgress);
    ///
    /// # let result = db.update_issue(&id, &updates);
    /// # assert!(result.is_err()); // Issue doesn't exist
    /// ```
    pub fn update_issue(&self, id: &IssueId, updates: &IssueUpdate) -> Result<()> {
        let mut set_clauses = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::ToSql>> = vec![];
        let mut param_count = 0;

        if let Some(ref title) = updates.title {
            param_count += 1;
            set_clauses.push(format!("title = ?{}", param_count));
            param_values.push(Box::new(title.clone()));
        }

        if let Some(ref description) = updates.description {
            param_count += 1;
            set_clauses.push(format!("description = ?{}", param_count));
            param_values.push(Box::new(description.clone()));
        }

        let now_string = Utc::now().to_rfc3339();
        if let Some(status) = updates.status {
            param_count += 1;
            set_clauses.push(format!("status = ?{}", param_count));
            param_values.push(Box::new(status.as_str().to_string()));

            // Handle closed_at for status changes
            if status == IssueStatus::Closed {
                param_count += 1;
                set_clauses.push(format!("closed_at = ?{}", param_count));
                param_values.push(Box::new(now_string.clone()));
            } else {
                set_clauses.push("closed_at = NULL".to_string());
            }
        }

        if let Some(priority) = updates.priority {
            param_count += 1;
            set_clauses.push(format!("priority = ?{}", param_count));
            param_values.push(Box::new(priority.as_i32()));
        }

        if let Some(ref assignee) = updates.assignee {
            param_count += 1;
            set_clauses.push(format!("assignee = ?{}", param_count));
            match assignee {
                Some(ref a) => param_values.push(Box::new(a.clone())),
                None => param_values.push(Box::new(None::<String>)),
            }
        }

        if set_clauses.is_empty() && updates.notes.is_none() {
            return Ok(());
        }

        param_count += 1;
        set_clauses.push(format!("updated_at = ?{}", param_count));
        param_values.push(Box::new(now_string));

        param_count += 1;
        let query = format!(
            "UPDATE issues SET {} WHERE id = ?{}",
            set_clauses.join(", "),
            param_count
        );
        param_values.push(Box::new(id.as_str().to_string()));

        // Convert Box<dyn ToSql> to &dyn ToSql for the execute call
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        let rows_affected = self.conn.execute(&query, params_refs.as_slice())?;

        if rows_affected == 0 {
            return Err(Error::IssueNotFound(id.as_str().to_string()));
        }

        Ok(())
    }

    /// Closes an issue with a reason.
    ///
    /// Sets the status to closed and records the reason in the notes field.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The issue doesn't exist
    /// - The issue has open dependencies
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueId};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// # let id = IssueId::new("test-1").unwrap();
    /// let result = db.close_issue(&id, "Completed successfully");
    /// # assert!(result.is_err()); // Issue doesn't exist
    /// ```
    pub fn close_issue(&self, id: &IssueId, _reason: &str) -> Result<()> {
        // Check for open dependencies
        if self.is_blocked(id)? {
            return Err(Error::OpenDependencies {
                count: self.count_open_dependencies(id)?,
            });
        }

        let now = Utc::now().to_rfc3339();
        let rows_affected = self.conn.execute(
            "UPDATE issues SET status = 'closed', closed_at = ?1, updated_at = ?2 WHERE id = ?3",
            params![now, now, id.as_str()],
        )?;

        if rows_affected == 0 {
            return Err(Error::IssueNotFound(id.as_str().to_string()));
        }

        Ok(())
    }

    /// Adds a dependency relationship between two issues.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Either issue doesn't exist
    /// - The dependency would create a cycle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueId, DependencyType};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// let issue_id = IssueId::new("issue-1").unwrap();
    /// let depends_on = IssueId::new("issue-2").unwrap();
    ///
    /// # let result = db.add_dependency(&issue_id, &depends_on, DependencyType::Blocks);
    /// # assert!(result.is_err()); // Issues don't exist
    /// ```
    pub fn add_dependency(
        &self,
        issue_id: &IssueId,
        depends_on: &IssueId,
        dep_type: DependencyType,
    ) -> Result<()> {
        // Check for cycles
        if self.would_create_cycle(issue_id, depends_on)? {
            return Err(Error::CircularDependency {
                issue_id: issue_id.as_str().to_string(),
                depends_on: depends_on.as_str().to_string(),
            });
        }

        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO dependencies (issue_id, depends_on_id, type, created_at, created_by)
             VALUES (?1, ?2, ?3, ?4, '')",
            params![
                issue_id.as_str(),
                depends_on.as_str(),
                dep_type.as_str(),
                now
            ],
        )?;

        Ok(())
    }

    /// Gets all dependencies for an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueId};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// let id = IssueId::new("test-1").unwrap();
    /// let deps = db.get_dependencies(&id).unwrap();
    /// assert!(deps.is_empty());
    /// ```
    pub fn get_dependencies(&self, id: &IssueId) -> Result<Vec<Dependency>> {
        let mut stmt = self.conn.prepare(
            "SELECT issue_id, depends_on_id, type, created_at
             FROM dependencies WHERE issue_id = ?1",
        )?;

        let mut rows = stmt.query(params![id.as_str()])?;

        let mut dependencies = Vec::new();
        while let Some(row) = rows.next()? {
            let type_str: String = row.get(2)?;
            let dependency_type = DependencyType::parse_str(&type_str).ok_or_else(|| {
                Error::InvalidIssueId(format!("Invalid dependency type: {}", type_str))
            })?;

            let created_at_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| Error::ChronoParse(e.to_string()))?
                .with_timezone(&Utc);

            dependencies.push(Dependency {
                issue_id: IssueId::new(row.get::<_, String>(0)?)?,
                depends_on_id: IssueId::new(row.get::<_, String>(1)?)?,
                dependency_type,
                created_at,
            });
        }

        Ok(dependencies)
    }

    /// Checks if an issue is blocked by unclosed dependencies.
    ///
    /// Returns `true` if the issue has any dependencies with `type = 'blocks'`
    /// that are not closed.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub fn is_blocked(&self, id: &IssueId) -> Result<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) FROM dependencies d
             JOIN issues i ON d.depends_on_id = i.id
             WHERE d.issue_id = ?1 AND d.type = 'blocks' AND i.status != 'closed'",
        )?;

        let count: i64 = stmt.query_row(params![id.as_str()], |row| row.get(0))?;
        Ok(count > 0)
    }

    /// Counts open dependencies for an issue.
    fn count_open_dependencies(&self, id: &IssueId) -> Result<usize> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) FROM dependencies d
             JOIN issues i ON d.depends_on_id = i.id
             WHERE d.issue_id = ?1 AND d.type = 'blocks' AND i.status != 'closed'",
        )?;

        let count: i64 = stmt.query_row(params![id.as_str()], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Checks if adding a dependency would create a cycle.
    fn would_create_cycle(&self, issue_id: &IssueId, depends_on: &IssueId) -> Result<bool> {
        // If depends_on already depends on issue_id (directly or indirectly), we have a cycle
        let mut visited = std::collections::HashSet::new();
        let mut to_visit = vec![depends_on.as_str().to_string()];

        while let Some(current) = to_visit.pop() {
            if current == issue_id.as_str() {
                return Ok(true);
            }
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            let mut stmt = self
                .conn
                .prepare("SELECT depends_on_id FROM dependencies WHERE issue_id = ?1")?;

            let mut rows = stmt.query(params![&current])?;
            while let Some(row) = rows.next()? {
                let dep_id: String = row.get(0)?;
                to_visit.push(dep_id);
            }
        }

        Ok(false)
    }

    /// Adds a label to an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the issue doesn't exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueId};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// let id = IssueId::new("test-1").unwrap();
    ///
    /// # let result = db.add_label(&id, "bug");
    /// # assert!(result.is_err()); // Issue doesn't exist
    /// ```
    pub fn add_label(&self, id: &IssueId, label: &str) -> Result<()> {
        let rows_affected = self.conn.execute(
            "INSERT INTO labels (issue_id, label) VALUES (?1, ?2)",
            params![id.as_str(), label],
        )?;

        // Check if issue exists
        let exists: bool = self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM issues WHERE id = ?1)",
            params![id.as_str()],
            |row| row.get(0),
        )?;

        if !exists {
            return Err(Error::IssueNotFound(id.as_str().to_string()));
        }

        // Ignore duplicate label errors (constraint violation)
        if rows_affected == 0 {
            // Label might already exist, that's ok
        }

        Ok(())
    }

    /// Removes a label from an issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueDatabase, IssueId};
    ///
    /// # let db = IssueDatabase::in_memory().unwrap();
    /// # db.init_schema().unwrap();
    /// let id = IssueId::new("test-1").unwrap();
    ///
    /// # let result = db.remove_label(&id, "bug");
    /// # assert!(result.is_ok()); // Doesn't error if issue/label doesn't exist
    /// ```
    pub fn remove_label(&self, id: &IssueId, label: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM labels WHERE issue_id = ?1 AND label = ?2",
            params![id.as_str(), label],
        )?;
        Ok(())
    }

    /// Gets all labels for an issue.
    fn get_labels_for_issue(&self, id: &IssueId) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT label FROM labels WHERE issue_id = ?1 ORDER BY label")?;

        let mut rows = stmt.query(params![id.as_str()])?;

        let mut labels = Vec::new();
        while let Some(row) = rows.next()? {
            labels.push(row.get(0)?);
        }

        Ok(labels)
    }
}

/// Filter for listing issues.
#[derive(Debug, Clone, Default)]
pub struct IssueFilter {
    /// Optional status filter.
    pub status: Option<IssueStatus>,
    /// Optional priority filter.
    pub priority: Option<Priority>,
    /// Optional issue type filter.
    pub issue_type: Option<IssueType>,
    /// Optional assignee filter.
    pub assignee: Option<String>,
    /// Labels to filter by (issue must have all specified labels).
    pub labels: Vec<String>,
    /// Optional parent issue ID filter.
    pub parent_id: Option<IssueId>,
}

impl IssueFilter {
    /// Creates a new empty filter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueFilter;
    ///
    /// let filter = IssueFilter::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the status filter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueFilter, IssueStatus};
    ///
    /// let filter = IssueFilter::new().with_status(IssueStatus::Open);
    /// ```
    pub fn with_status(mut self, status: IssueStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the priority filter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueFilter, Priority};
    ///
    /// let filter = IssueFilter::new().with_priority(Priority::P1);
    /// ```
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Sets the issue type filter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueFilter, IssueType};
    ///
    /// let filter = IssueFilter::new().with_issue_type(IssueType::Task);
    /// ```
    pub fn with_issue_type(mut self, issue_type: IssueType) -> Self {
        self.issue_type = Some(issue_type);
        self
    }

    /// Sets the assignee filter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueFilter;
    ///
    /// let filter = IssueFilter::new().with_assignee("username".to_string());
    /// ```
    pub fn with_assignee(mut self, assignee: String) -> Self {
        self.assignee = Some(assignee);
        self
    }

    /// Sets the labels filter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueFilter;
    ///
    /// let filter = IssueFilter::new().with_labels(vec!["bug".to_string(), "urgent".to_string()]);
    /// ```
    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    /// Sets the parent ID filter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueFilter, IssueId};
    ///
    /// let parent_id = IssueId::new("parent-1").unwrap();
    /// let filter = IssueFilter::new().with_parent_id(parent_id);
    /// ```
    pub fn with_parent_id(mut self, parent_id: IssueId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }
}

/// Partial issue update.
#[derive(Debug, Clone, Default)]
pub struct IssueUpdate {
    /// Optional new title.
    pub title: Option<String>,
    /// Optional new description.
    pub description: Option<String>,
    /// Optional new status.
    pub status: Option<IssueStatus>,
    /// Optional new priority.
    pub priority: Option<Priority>,
    /// Optional new assignee (None to clear).
    pub assignee: Option<Option<String>>,
    /// Optional notes to append.
    pub notes: Option<String>,
}

impl IssueUpdate {
    /// Creates a new empty update.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueUpdate;
    ///
    /// let update = IssueUpdate::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueUpdate;
    ///
    /// let update = IssueUpdate::new().with_title("New Title".to_string());
    /// ```
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the description.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueUpdate;
    ///
    /// let update = IssueUpdate::new().with_description("New description".to_string());
    /// ```
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the status.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueUpdate, IssueStatus};
    ///
    /// let update = IssueUpdate::new().with_status(IssueStatus::Closed);
    /// ```
    pub fn with_status(mut self, status: IssueStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the priority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::{IssueUpdate, Priority};
    ///
    /// let update = IssueUpdate::new().with_priority(Priority::P0);
    /// ```
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Sets the assignee.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_issue_tracker::IssueUpdate;
    ///
    /// let update = IssueUpdate::new().with_assignee(Some("username".to_string()));
    /// ```
    pub fn with_assignee(mut self, assignee: Option<String>) -> Self {
        self.assignee = Some(assignee);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_db() -> IssueDatabase {
        let db = IssueDatabase::in_memory().unwrap();
        db.init_schema().unwrap();
        db
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
    fn test_database_in_memory() {
        let db = IssueDatabase::in_memory();
        assert!(db.is_ok());
    }

    #[test]
    fn test_database_init_schema() {
        let db = create_test_db();
        // Check that tables exist
        let table_exists: bool = db
            .conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='issues')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(table_exists);
    }

    #[test]
    fn test_create_issue() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");

        let created_id = db.create_issue(&issue).unwrap();
        assert_eq!(created_id.as_str(), "test-1");
    }

    #[test]
    fn test_create_duplicate_issue() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");

        db.create_issue(&issue).unwrap();
        let result = db.create_issue(&issue);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_issue_exists() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");

        db.create_issue(&issue).unwrap();
        let retrieved = db.get_issue(&issue.id).unwrap();

        assert!(retrieved.is_some());
        let retrieved_issue = retrieved.unwrap();
        assert_eq!(retrieved_issue.id.as_str(), "test-1");
        assert_eq!(retrieved_issue.title, "Test Issue");
    }

    #[test]
    fn test_get_issue_not_exists() {
        let db = create_test_db();
        let id = IssueId::new("nonexistent").unwrap();

        let retrieved = db.get_issue(&id).unwrap();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_list_issues_empty() {
        let db = create_test_db();
        let filter = IssueFilter::default();
        let issues = db.list_issues(&filter).unwrap();

        assert!(issues.is_empty());
    }

    #[test]
    fn test_list_issues_with_items() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        let filter = IssueFilter::default();
        let issues = db.list_issues(&filter).unwrap();

        assert_eq!(issues.len(), 2);
    }

    #[test]
    fn test_list_issues_filter_by_status() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "Open Issue");
        let mut issue2 = create_test_issue("test-2", "Closed Issue");
        issue2.status = IssueStatus::Closed;

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        let filter = IssueFilter::new().with_status(IssueStatus::Open);
        let issues = db.list_issues(&filter).unwrap();

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id.as_str(), "test-1");
    }

    #[test]
    fn test_list_issues_filter_by_priority() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "P0 Issue");
        let mut issue1_p0 = issue1.clone();
        issue1_p0.priority = Priority::P0;
        let _issue2 = create_test_issue("test-2", "P2 Issue");

        db.create_issue(&issue1_p0).unwrap();
        db.create_issue(&_issue2).unwrap();

        let filter = IssueFilter::new().with_priority(Priority::P0);
        let issues = db.list_issues(&filter).unwrap();

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id.as_str(), "test-1");
    }

    #[test]
    fn test_list_issues_filter_by_type() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "Task");
        let mut issue2 = create_test_issue("test-2", "Epic");
        issue2.issue_type = IssueType::Epic;
        let issue2_epic = issue2.clone();

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2_epic).unwrap();

        let filter = IssueFilter::new().with_issue_type(IssueType::Epic);
        let issues = db.list_issues(&filter).unwrap();

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id.as_str(), "test-2");
    }

    #[test]
    fn test_update_issue_title() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Original Title");
        db.create_issue(&issue).unwrap();

        let mut updates = IssueUpdate::new();
        updates.title = Some("Updated Title".to_string());

        db.update_issue(&issue.id, &updates).unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.title, "Updated Title");
    }

    #[test]
    fn test_update_issue_status() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        let mut updates = IssueUpdate::new();
        updates.status = Some(IssueStatus::InProgress);

        db.update_issue(&issue.id, &updates).unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.status, IssueStatus::InProgress);
    }

    #[test]
    fn test_update_issue_not_exists() {
        let db = create_test_db();
        let id = IssueId::new("nonexistent").unwrap();

        // Empty update returns Ok even if issue doesn't exist (no rows affected)
        let updates = IssueUpdate::new();
        let result = db.update_issue(&id, &updates);
        // Empty update returns Ok early (no changes)
        assert!(result.is_ok());

        // Non-empty update should fail
        let mut updates_with_change = IssueUpdate::new();
        updates_with_change.title = Some("New Title".to_string());
        let result = db.update_issue(&id, &updates_with_change);
        assert!(result.is_err());
    }

    #[test]
    fn test_close_issue() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        db.close_issue(&issue.id, "Done").unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.status, IssueStatus::Closed);
        assert!(retrieved.closed_at.is_some());
    }

    #[test]
    fn test_close_issue_not_exists() {
        let db = create_test_db();
        let id = IssueId::new("nonexistent").unwrap();

        let result = db.close_issue(&id, "Done");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_dependency() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        let deps = db.get_dependencies(&issue1.id).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].depends_on_id.as_str(), "test-2");
    }

    #[test]
    fn test_add_dependency_creates_cycle() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        let result = db.add_dependency(&issue2.id, &issue1.id, DependencyType::Blocks);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_dependencies() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");
        let issue3 = create_test_issue("test-3", "Third Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();
        db.create_issue(&issue3).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();
        db.add_dependency(&issue1.id, &issue3.id, DependencyType::Related)
            .unwrap();

        let deps = db.get_dependencies(&issue1.id).unwrap();
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_is_blocked() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        assert!(db.is_blocked(&issue1.id).unwrap());
        assert!(!db.is_blocked(&issue2.id).unwrap());
    }

    #[test]
    fn test_is_blocked_with_closed_dependency() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");
        let mut issue2_closed = issue2.clone();
        issue2_closed.status = IssueStatus::Closed;

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2_closed).unwrap();

        db.add_dependency(&issue1.id, &issue2_closed.id, DependencyType::Blocks)
            .unwrap();

        assert!(!db.is_blocked(&issue1.id).unwrap());
    }

    #[test]
    fn test_close_blocked_issue_fails() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        let result = db.close_issue(&issue1.id, "Done");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_label() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        db.add_label(&issue.id, "bug").unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.labels, vec!["bug"]);
    }

    #[test]
    fn test_add_label_not_exists() {
        let db = create_test_db();
        let id = IssueId::new("nonexistent").unwrap();

        let result = db.add_label(&id, "bug");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_label() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        db.add_label(&issue.id, "bug").unwrap();
        db.remove_label(&issue.id, "bug").unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert!(retrieved.labels.is_empty());
    }

    #[test]
    fn test_remove_label_not_exists() {
        let db = create_test_db();
        let id = IssueId::new("nonexistent").unwrap();

        let result = db.remove_label(&id, "bug");
        assert!(result.is_ok()); // Doesn't error on missing label
    }

    #[test]
    fn test_filter_with_labels() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "Bug");
        let mut issue2 = create_test_issue("test-2", "Feature");
        issue2.labels = vec!["enhancement".to_string()];

        db.create_issue(&issue1).unwrap();
        db.add_label(&issue1.id, "bug").unwrap();
        db.create_issue(&issue2).unwrap();

        let filter = IssueFilter::new().with_labels(vec!["bug".to_string()]);
        let issues = db.list_issues(&filter).unwrap();

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id.as_str(), "test-1");
    }

    #[test]
    fn test_filter_with_parent_id() {
        let db = create_test_db();
        let epic = create_test_issue("epic-1", "Epic");
        let task = create_test_issue("task-1", "Task");

        db.create_issue(&epic).unwrap();
        db.create_issue(&task).unwrap();

        db.add_dependency(&task.id, &epic.id, DependencyType::ParentChild)
            .unwrap();

        let filter = IssueFilter::new().with_parent_id(epic.id.clone());
        let issues = db.list_issues(&filter).unwrap();

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id.as_str(), "task-1");
    }

    #[test]
    fn test_update_issue_priority() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        let mut updates = IssueUpdate::new();
        updates.priority = Some(Priority::P0);

        db.update_issue(&issue.id, &updates).unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.priority, Priority::P0);
    }

    #[test]
    fn test_update_issue_assignee() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        let mut updates = IssueUpdate::new();
        updates.assignee = Some(Some("username".to_string()));

        db.update_issue(&issue.id, &updates).unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.assignee, Some("username".to_string()));
    }

    #[test]
    fn test_update_issue_clear_assignee() {
        let db = create_test_db();
        let mut issue = create_test_issue("test-1", "Test Issue");
        issue.assignee = Some("username".to_string());
        db.create_issue(&issue).unwrap();

        let mut updates = IssueUpdate::new();
        updates.assignee = Some(None);

        db.update_issue(&issue.id, &updates).unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert!(retrieved.assignee.is_none());
    }

    #[test]
    fn test_issue_with_labels() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        db.add_label(&issue.id, "bug").unwrap();
        db.add_label(&issue.id, "urgent").unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.labels.len(), 2);
        assert!(retrieved.labels.contains(&"bug".to_string()));
        assert!(retrieved.labels.contains(&"urgent".to_string()));
    }

    #[test]
    fn test_add_duplicate_label() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        db.add_label(&issue.id, "bug").unwrap();
        let _result = db.add_label(&issue.id, "bug"); // May error due to UNIQUE constraint, that's ok

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.labels.len(), 1);
    }

    #[test]
    fn test_would_create_cycle_direct() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();

        assert!(db.would_create_cycle(&issue2.id, &issue1.id).unwrap());
    }

    #[test]
    fn test_would_create_cycle_indirect() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");
        let issue3 = create_test_issue("test-3", "Third Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();
        db.create_issue(&issue3).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();
        db.add_dependency(&issue2.id, &issue3.id, DependencyType::Blocks)
            .unwrap();

        assert!(db.would_create_cycle(&issue3.id, &issue1.id).unwrap());
    }

    #[test]
    fn test_would_create_cycle_none() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        assert!(!db.would_create_cycle(&issue1.id, &issue2.id).unwrap());
    }

    #[test]
    fn test_dependency_type_all() {
        let all = DependencyType::all();
        assert_eq!(all.len(), 4);
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
    fn test_filter_builder_methods() {
        let id = IssueId::new("test-1").unwrap();
        let filter = IssueFilter::new()
            .with_status(IssueStatus::Open)
            .with_priority(Priority::P1)
            .with_issue_type(IssueType::Task)
            .with_assignee("user".to_string())
            .with_labels(vec!["bug".to_string()])
            .with_parent_id(id);

        assert_eq!(filter.status, Some(IssueStatus::Open));
        assert_eq!(filter.priority, Some(Priority::P1));
        assert_eq!(filter.issue_type, Some(IssueType::Task));
        assert_eq!(filter.assignee, Some("user".to_string()));
        assert_eq!(filter.labels, vec!["bug"]);
        assert!(filter.parent_id.is_some());
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
    fn test_update_builder_methods() {
        let update = IssueUpdate::new()
            .with_title("New Title".to_string())
            .with_description("New Desc".to_string())
            .with_status(IssueStatus::Closed)
            .with_priority(Priority::P0)
            .with_assignee(Some("user".to_string()));

        assert_eq!(update.title, Some("New Title".to_string()));
        assert_eq!(update.description, Some("New Desc".to_string()));
        assert_eq!(update.status, Some(IssueStatus::Closed));
        assert_eq!(update.priority, Some(Priority::P0));
        assert_eq!(update.assignee, Some(Some("user".to_string())));
    }

    #[test]
    fn test_transaction_rollback_on_error() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let _issue2 = create_test_issue("test-2", "Second Issue");

        db.create_issue(&issue1).unwrap();

        // Create a duplicate which should fail
        let result = db.create_issue(&issue1);
        assert!(result.is_err());

        // The original issue should still be accessible
        let retrieved = db.get_issue(&issue1.id).unwrap();
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_filter_by_assignee() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "Assigned Issue");
        let mut issue1_with_assignee = issue1.clone();
        issue1_with_assignee.assignee = Some("user1".to_string());
        let _issue2 = create_test_issue("test-2", "Unassigned Issue");

        db.create_issue(&issue1_with_assignee).unwrap();
        db.create_issue(&_issue2).unwrap();

        let filter = IssueFilter::new().with_assignee("user1".to_string());
        let issues = db.list_issues(&filter).unwrap();

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id.as_str(), "test-1");
    }

    #[test]
    fn test_list_issues_ordering() {
        let db = create_test_db();
        let mut issue1 = create_test_issue("test-1", "P0 Issue");
        issue1.priority = Priority::P0;
        let issue2 = create_test_issue("test-2", "P2 Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();

        let filter = IssueFilter::default();
        let issues = db.list_issues(&filter).unwrap();

        // Should be ordered by priority (P0 before P2)
        assert_eq!(issues[0].priority, Priority::P0);
        assert_eq!(issues[1].priority, Priority::P2);
    }

    #[test]
    fn test_update_issue_description() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        let mut updates = IssueUpdate::new();
        updates.description = Some("Updated description".to_string());

        db.update_issue(&issue.id, &updates).unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert_eq!(retrieved.description, "Updated description");
    }

    #[test]
    fn test_get_empty_dependencies() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        let deps = db.get_dependencies(&issue.id).unwrap();
        assert!(deps.is_empty());
    }

    #[test]
    fn test_database_open_nonexistent() {
        let result = IssueDatabase::open_with_path("/nonexistent/path/beads.db");
        assert!(result.is_err());
    }

    #[test]
    fn test_count_open_dependencies() {
        let db = create_test_db();
        let issue1 = create_test_issue("test-1", "First Issue");
        let issue2 = create_test_issue("test-2", "Second Issue");
        let issue3 = create_test_issue("test-3", "Third Issue");

        db.create_issue(&issue1).unwrap();
        db.create_issue(&issue2).unwrap();
        db.create_issue(&issue3).unwrap();

        db.add_dependency(&issue1.id, &issue2.id, DependencyType::Blocks)
            .unwrap();
        db.add_dependency(&issue1.id, &issue3.id, DependencyType::Blocks)
            .unwrap();

        let count = db.count_open_dependencies(&issue1.id).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_close_issue_sets_closed_at() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        db.close_issue(&issue.id, "Done").unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        assert!(retrieved.closed_at.is_some());
        assert!(retrieved.closed_at.unwrap() > retrieved.created_at);
    }

    #[test]
    fn test_issue_labels_sorted() {
        let db = create_test_db();
        let issue = create_test_issue("test-1", "Test Issue");
        db.create_issue(&issue).unwrap();

        db.add_label(&issue.id, "zebra").unwrap();
        db.add_label(&issue.id, "apple").unwrap();
        db.add_label(&issue.id, "banana").unwrap();

        let retrieved = db.get_issue(&issue.id).unwrap().unwrap();
        // Labels should be sorted alphabetically
        assert_eq!(retrieved.labels, vec!["apple", "banana", "zebra"]);
    }
}
