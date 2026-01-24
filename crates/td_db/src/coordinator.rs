// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! TdDb coordinator - manages all databases.
//!
//! This module provides the central TdDb coordinator that initializes and manages
//! all SQLite databases used by the Telegram client.

use std::path::Path;

use rustgram_storage::{
    ChatDb, DbConnection, FileDb, MessageDb, StorageError, StorageResult, UserDb,
};

use crate::TdDbParameters;

/// TdDb coordinator - manages all Telegram client databases.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `TdDb` class in `td/telegram/TdDb.h`.
///
/// # Example
///
/// ```rust,no_run
/// use rustgram_td_db::{TdDb, TdDbParameters};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let params = TdDbParameters::new(
///     "/path/to/db".to_string(),
///     "/path/to/files".to_string(),
///     false,
///     true
/// );
///
/// let mut tddb = TdDb::open(params)?;
///
/// // Access databases
/// if let Some(message_db) = tddb.get_message_db() {
///     // Use message database
/// }
///
/// tddb.close()?;
/// # Ok(())
/// # }
/// ```
pub struct TdDb {
    /// Database parameters.
    parameters: TdDbParameters,

    /// Message database (always enabled).
    message_db: Option<MessageDb>,

    /// User database (always enabled).
    user_db: Option<UserDb>,

    /// Chat database (always enabled).
    chat_db: Option<ChatDb>,

    /// File database (optional, based on parameters).
    file_db: Option<FileDb>,

    /// Whether the database is open.
    is_open: bool,
}

impl TdDb {
    /// Opens all databases with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Database configuration parameters
    ///
    /// # Returns
    ///
    /// Returns `Ok(TdDb)` if all databases opened successfully, or an error if:
    /// - Database directory cannot be created
    /// - Database files are corrupted
    /// - Migration fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let tddb = TdDb::open(params)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open(params: TdDbParameters) -> StorageResult<Self> {
        // Create database directory if it doesn't exist
        let db_dir = Path::new(params.database_directory());
        if !db_dir.exists() {
            std::fs::create_dir_all(db_dir)?;
        }

        // Initialize individual databases
        let message_db = Self::open_message_db(&params)?;
        let user_db = Self::open_user_db(&params)?;
        let chat_db = Self::open_chat_db(&params)?;
        let file_db = if params.use_file_database() {
            Some(Self::open_file_db(&params)?)
        } else {
            None
        };

        Ok(Self {
            parameters: params,
            message_db: Some(message_db),
            user_db: Some(user_db),
            chat_db: Some(chat_db),
            file_db,
            is_open: true,
        })
    }

    /// Opens the message database.
    fn open_message_db(params: &TdDbParameters) -> StorageResult<MessageDb> {
        let db_path = Path::new(params.database_directory()).join("messages.db");
        let conn = DbConnection::new(db_path)?;

        // Check for corruption
        Self::check_integrity(&conn)?;

        let db = MessageDb::new(conn);
        db.init()?;
        Ok(db)
    }

    /// Opens the user database.
    fn open_user_db(params: &TdDbParameters) -> StorageResult<UserDb> {
        let db_path = Path::new(params.database_directory()).join("users.db");
        let conn = DbConnection::new(db_path)?;

        // Check for corruption
        Self::check_integrity(&conn)?;

        let db = UserDb::new(conn);
        db.init()?;
        Ok(db)
    }

    /// Opens the chat database.
    fn open_chat_db(params: &TdDbParameters) -> StorageResult<ChatDb> {
        let db_path = Path::new(params.database_directory()).join("chats.db");
        let conn = DbConnection::new(db_path)?;

        // Check for corruption
        Self::check_integrity(&conn)?;

        let db = ChatDb::new(conn);
        db.init()?;
        Ok(db)
    }

    /// Opens the file database.
    fn open_file_db(params: &TdDbParameters) -> StorageResult<FileDb> {
        let db_path = Path::new(params.database_directory()).join("files.db");
        let conn = DbConnection::new(db_path)?;

        // Check for corruption
        Self::check_integrity(&conn)?;

        let db = FileDb::new(conn);
        db.init()?;
        Ok(db)
    }

    /// Checks database integrity and attempts recovery if corrupted.
    ///
    /// # Arguments
    ///
    /// * `conn` - Database connection to check
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if database is valid or recovery succeeded.
    /// Returns `Err(StorageError::DatabaseCorrupted)` if recovery fails.
    fn check_integrity(conn: &DbConnection) -> StorageResult<()> {
        let mut db_conn = conn.connect()?;

        // Run PRAGMA integrity_check
        let result: String = db_conn
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .unwrap_or_else(|_| "corrupted".to_string());

        if result != "ok" {
            // Attempt WAL checkpoint recovery
            let recovery_result = Self::attempt_wal_recovery(&mut db_conn);
            if recovery_result.is_err() {
                return Err(StorageError::DatabaseCorrupted);
            }

            // Check integrity again after recovery attempt
            let result_after: String = db_conn
                .query_row("PRAGMA integrity_check", [], |row| row.get(0))
                .unwrap_or_else(|_| "corrupted".to_string());

            if result_after != "ok" {
                return Err(StorageError::DatabaseCorrupted);
            }
        }

        Ok(())
    }

    /// Attempts WAL checkpoint recovery for corrupted databases.
    ///
    /// # Arguments
    ///
    /// * `conn` - Database connection to recover
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if recovery succeeded or database is already fine.
    fn attempt_wal_recovery(conn: &mut rusqlite::Connection) -> StorageResult<()> {
        // Attempt to truncate the WAL file
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)", [])
            .map_err(|_| StorageError::DatabaseCorrupted)?;
        Ok(())
    }

    /// Closes all databases and flushes pending writes.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all databases closed successfully.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let mut tddb = TdDb::open(params)?;
    /// // ... use databases ...
    /// tddb.close()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn close(&mut self) -> StorageResult<()> {
        if !self.is_open {
            return Ok(());
        }

        // Close each database
        self.message_db = None;
        self.user_db = None;
        self.chat_db = None;
        self.file_db = None;

        self.is_open = false;
        Ok(())
    }

    /// Returns the message database if available.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let tddb = TdDb::open(params)?;
    /// if let Some(message_db) = tddb.get_message_db() {
    ///     let sync = message_db.sync();
    ///     // Use message database
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_message_db(&self) -> Option<&MessageDb> {
        self.message_db.as_ref()
    }

    /// Returns the user database if available.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let tddb = TdDb::open(params)?;
    /// if let Some(user_db) = tddb.get_user_db() {
    ///     let sync = user_db.sync();
    ///     // Use user database
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_user_db(&self) -> Option<&UserDb> {
        self.user_db.as_ref()
    }

    /// Returns the chat database if available.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let tddb = TdDb::open(params)?;
    /// if let Some(chat_db) = tddb.get_chat_db() {
    ///     let sync = chat_db.sync();
    ///     // Use chat database
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_chat_db(&self) -> Option<&ChatDb> {
        self.chat_db.as_ref()
    }

    /// Returns the file database if available.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let tddb = TdDb::open(params)?;
    /// if let Some(file_db) = tddb.get_file_db() {
    ///     let sync = file_db.sync();
    ///     // Use file database
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_file_db(&self) -> Option<&FileDb> {
        self.file_db.as_ref()
    }

    /// Returns the database parameters.
    #[must_use]
    pub const fn parameters(&self) -> &TdDbParameters {
        &self.parameters
    }

    /// Returns whether the database is currently open.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.is_open
    }

    /// Destroys all database files for the given parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Database parameters specifying which databases to destroy
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all database files were successfully deleted,
    /// or an error if deletion failed.
    ///
    /// # Warning
    ///
    /// This operation permanently deletes all data. Use with caution.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// // Destroy all databases
    /// TdDb::destroy(&params)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn destroy(params: &TdDbParameters) -> StorageResult<()> {
        let db_dir = Path::new(params.database_directory());

        // List of database files to delete
        let db_files = [
            "messages.db",
            "messages.db-wal",
            "messages.db-shm",
            "users.db",
            "users.db-wal",
            "users.db-shm",
            "chats.db",
            "chats.db-wal",
            "chats.db-shm",
        ];

        // Delete message, user, and chat databases
        for file in &db_files {
            let path = db_dir.join(file);
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
        }

        // Delete file database if enabled
        if params.use_file_database() {
            let file_db_files = ["files.db", "files.db-wal", "files.db-shm"];
            for file in &file_db_files {
                let path = db_dir.join(file);
                if path.exists() {
                    std::fs::remove_file(&path)?;
                }
            }
        }

        Ok(())
    }

    /// Rebuilds all databases from scratch.
    ///
    /// # Arguments
    ///
    /// * `params` - Database parameters specifying which databases to rebuild
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all databases were successfully rebuilt,
    /// or an error if rebuild failed.
    ///
    /// # Warning
    ///
    /// This operation permanently deletes all existing data and creates
    /// fresh empty databases.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{TdDb, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// // Rebuild all databases (clears all data)
    /// TdDb::rebuild(&params)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rebuild(params: &TdDbParameters) -> StorageResult<()> {
        // First destroy existing databases
        Self::destroy(params)?;

        // Then create fresh databases
        let _tddb = Self::open(params.clone())?;

        Ok(())
    }
}

impl Drop for TdDb {
    /// Automatically closes databases when TdDb is dropped.
    fn drop(&mut self) {
        let _ = self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_td_db_open() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let tddb = TdDb::open(params);
        assert!(tddb.is_ok());

        let tddb = tddb.unwrap();
        assert!(tddb.is_open());
        assert!(tddb.get_message_db().is_some());
        assert!(tddb.get_user_db().is_some());
        assert!(tddb.get_chat_db().is_some());
        assert!(tddb.get_file_db().is_some());
    }

    #[test]
    fn test_td_db_open_without_file_db() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            false,
        );

        let tddb = TdDb::open(params);
        assert!(tddb.is_ok());

        let tddb = tddb.unwrap();
        assert!(tddb.get_file_db().is_none());
    }

    #[test]
    fn test_td_db_close() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let mut tddb = TdDb::open(params).unwrap();
        assert!(tddb.is_open());

        let close_result = tddb.close();
        assert!(close_result.is_ok());
        assert!(!tddb.is_open());
        assert!(tddb.get_message_db().is_none());
        assert!(tddb.get_user_db().is_none());
        assert!(tddb.get_chat_db().is_none());
    }

    #[test]
    fn test_td_db_double_close() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let mut tddb = TdDb::open(params).unwrap();
        assert!(tddb.close().is_ok());
        assert!(tddb.close().is_ok()); // Second close should also succeed
    }

    #[test]
    fn test_td_db_destroy() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        // Create databases
        let _tddb = TdDb::open(params.clone()).unwrap();
        drop(_tddb);

        // Destroy databases
        let destroy_result = TdDb::destroy(&params);
        assert!(destroy_result.is_ok());

        // Verify databases don't exist
        let db_dir = dir.path();
        assert!(!db_dir.join("messages.db").exists());
        assert!(!db_dir.join("users.db").exists());
        assert!(!db_dir.join("chats.db").exists());
        assert!(!db_dir.join("files.db").exists());
    }

    #[test]
    fn test_td_db_rebuild() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        // Create databases
        let _tddb = TdDb::open(params.clone()).unwrap();
        drop(_tddb);

        // Rebuild databases
        let rebuild_result = TdDb::rebuild(&params);
        assert!(rebuild_result.is_ok());

        // Verify databases exist and are usable
        let tddb = TdDb::open(params).unwrap();
        assert!(tddb.is_open());
    }

    #[test]
    fn test_td_db_parameters() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let tddb = TdDb::open(params.clone()).unwrap();
        assert_eq!(
            tddb.parameters().database_directory(),
            params.database_directory()
        );
        assert_eq!(
            tddb.parameters().files_directory(),
            params.files_directory()
        );
    }

    #[test]
    fn test_td_db_drop() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let tddb = TdDb::open(params).unwrap();
        assert!(tddb.is_open());

        // Drop should close the database
        drop(tddb);

        // Creating a new TdDb with same path should work
        let params2 = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );
        let tddb2 = TdDb::open(params2);
        assert!(tddb2.is_ok());
    }

    #[test]
    fn test_integrity_check_valid_db() {
        let dir = tempdir().unwrap();
        let _params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let db_path = dir.path().join("test.db");
        let conn = DbConnection::new(&db_path).unwrap();

        // Valid database should pass integrity check
        let result = TdDb::check_integrity(&conn);
        assert!(result.is_ok());
    }

    #[test]
    fn test_message_db_accessible() {
        let dir = tempdir().unwrap();
        let _params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let tddb = TdDb::open(_params.clone()).unwrap();
        let message_db = tddb.get_message_db();
        assert!(message_db.is_some());

        let mut sync = message_db.unwrap().sync();
        // Verify we can access the sync interface
        let count = sync.get_message_count(1);
        assert!(count.is_ok());
        assert_eq!(count.unwrap(), 0);
    }

    #[test]
    fn test_user_db_accessible() {
        let dir = tempdir().unwrap();
        let _params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let tddb = TdDb::open(_params.clone()).unwrap();
        let user_db = tddb.get_user_db();
        assert!(user_db.is_some());

        let mut sync = user_db.unwrap().sync();
        // Verify we can access the sync interface
        let result = sync.get_user(1);
        assert!(result.is_err()); // No users yet, should return NotFound error
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_chat_db_accessible() {
        let dir = tempdir().unwrap();
        let _params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let tddb = TdDb::open(_params.clone()).unwrap();
        let chat_db = tddb.get_chat_db();
        assert!(chat_db.is_some());

        let mut sync = chat_db.unwrap().sync();
        // Verify we can access the sync interface
        let result = sync.get_chat(1);
        assert!(result.is_err()); // No chats yet, should return NotFound error
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_file_db_accessible() {
        let dir = tempdir().unwrap();
        let _params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        let tddb = TdDb::open(_params.clone()).unwrap();
        let file_db = tddb.get_file_db();
        assert!(file_db.is_some());

        let mut sync = file_db.unwrap().sync();
        // Verify we can access the sync interface
        let result = sync.get_next_file_db_id();
        assert!(result.is_ok());
    }
}
