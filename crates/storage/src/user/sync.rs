//! Synchronous user database interface.

use bytes::Bytes;
use rusqlite::params;

use crate::connection::DbConnection;
use crate::error::{StorageError, StorageResult};

/// Synchronous user database interface.
pub struct UserDbSync {
    db: DbConnection,
}

impl UserDbSync {
    /// Creates a new synchronous user database interface.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Adds or updates a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    /// * `data` - Serialized user data
    /// * `profile_photo` - Serialized profile photo data (optional)
    /// * `bio` - User bio text (optional)
    pub fn add_user(
        &mut self,
        user_id: i32,
        data: Bytes,
        profile_photo: Option<Bytes>,
        bio: Option<String>,
    ) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "INSERT OR REPLACE INTO users (user_id, data, profile_photo, bio)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                user_id,
                data.as_ref(),
                profile_photo.as_ref().map(|b| b.as_ref()),
                bio,
            ],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    pub fn get_user(&mut self, user_id: i32) -> StorageResult<Bytes> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT data FROM users WHERE user_id = ?1",
            params![user_id],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(Bytes::from(data))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("User {}", user_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Gets multiple users by their IDs.
    ///
    /// # Arguments
    ///
    /// * `user_ids` - List of user identifiers
    pub fn get_users(&mut self, user_ids: Vec<i32>) -> StorageResult<Vec<Bytes>> {
        if user_ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.db.connect()?;

        // Build IN clause
        let in_clause = user_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "SELECT data FROM users WHERE user_id IN ({})",
            in_clause
        );

        let mut stmt = conn.prepare(&sql)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            user_ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

        let mut users = Vec::new();
        let mut rows = stmt.query(&params_refs[..])?;

        while let Some(row) = rows.next()? {
            let data: Vec<u8> = row.get(0)?;
            users.push(Bytes::from(data));
        }

        Ok(users)
    }

    /// Gets a user's profile photo.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    pub fn get_user_profile_photo(&mut self, user_id: i32) -> StorageResult<Option<Bytes>> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT profile_photo FROM users WHERE user_id = ?1",
            params![user_id],
            |row| {
                let photo: Option<Vec<u8>> = row.get(0)?;
                Ok(photo.map(Bytes::from))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("User {}", user_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Gets a user's bio.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    pub fn get_user_bio(&mut self, user_id: i32) -> StorageResult<Option<String>> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT bio FROM users WHERE user_id = ?1",
            params![user_id],
            |row| row.get(0),
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("User {}", user_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Deletes a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    pub fn delete_user(&mut self, user_id: i32) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute("DELETE FROM users WHERE user_id = ?1", params![user_id])
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Adds or updates full user info.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    /// * `full_info` - Serialized full user info
    pub fn add_user_full(&mut self, user_id: i32, full_info: Bytes) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "INSERT OR REPLACE INTO users_full (user_id, full_info)
             VALUES (?1, ?2)",
            params![user_id, full_info.as_ref()],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets full user info.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    pub fn get_user_full(&mut self, user_id: i32) -> StorageResult<Bytes> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT full_info FROM users_full WHERE user_id = ?1",
            params![user_id],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(Bytes::from(data))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("UserFullInfo for user {}", user_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Deletes full user info.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier
    pub fn delete_user_full(&mut self, user_id: i32) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "DELETE FROM users_full WHERE user_id = ?1",
            params![user_id],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets the count of users in the database.
    pub fn get_user_count(&mut self) -> StorageResult<i32> {
        let conn = self.db.connect()?;

        conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
            .map_err(|e| StorageError::QueryError(e.to_string()))
    }

    /// Returns a reference to the underlying connection.
    pub fn db(&self) -> &DbConnection {
        &self.db
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::schema::get_user_migrations;
    use crate::migrations::MigrationManager;
    use tempfile::tempdir;

    fn setup_test_db() -> DbConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        // Run migrations
        let mut manager = MigrationManager::new();
        for migration in get_user_migrations() {
            manager = manager.add_migration(migration);
        }
        manager.run(&db).unwrap();

        // Keep dir alive by intentionally leaking it
        std::mem::forget(dir);
        db
    }

    #[test]
    fn test_add_and_get_user() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;
        let data = Bytes::from("test user data");

        user_db
            .add_user(user_id, data.clone(), None, None)
            .unwrap();

        let retrieved = user_db.get_user(user_id).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_get_nonexistent_user() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let result = user_db.get_user(99999);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_delete_user() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;
        let data = Bytes::from("test user data");

        user_db.add_user(user_id, data, None, None).unwrap();

        // Verify exists
        assert!(user_db.get_user(user_id).is_ok());

        // Delete
        user_db.delete_user(user_id).unwrap();

        // Verify gone
        assert!(user_db.get_user(user_id).is_err());
    }

    #[test]
    fn test_get_users_batch() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        // Add multiple users
        let user_ids = vec![1, 2, 3, 4, 5];
        for &user_id in &user_ids {
            user_db
                .add_user(user_id, Bytes::from(format!("user {}", user_id)), None, None)
                .unwrap();
        }

        // Get all users
        let users = user_db.get_users(user_ids.clone()).unwrap();
        assert_eq!(users.len(), 5);

        // Get subset
        let users = user_db.get_users(vec![1, 3, 5]).unwrap();
        assert_eq!(users.len(), 3);
    }

    #[test]
    fn test_get_users_empty() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let users = user_db.get_users(vec![]).unwrap();
        assert_eq!(users.len(), 0);
    }

    #[test]
    fn test_user_profile_photo() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;
        let photo = Bytes::from("profile photo data");

        user_db
            .add_user(user_id, Bytes::from("user data"), Some(photo.clone()), None)
            .unwrap();

        let retrieved = user_db.get_user_profile_photo(user_id).unwrap();
        assert_eq!(retrieved, Some(photo));
    }

    #[test]
    fn test_user_bio() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;
        let bio = "Software developer from Rust".to_string();

        user_db
            .add_user(
                user_id,
                Bytes::from("user data"),
                None,
                Some(bio.clone()),
            )
            .unwrap();

        let retrieved = user_db.get_user_bio(user_id).unwrap();
        assert_eq!(retrieved, Some(bio));
    }

    #[test]
    fn test_user_full_info() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;
        let full_info = Bytes::from("full user info data");

        user_db.add_user_full(user_id, full_info.clone()).unwrap();

        let retrieved = user_db.get_user_full(user_id).unwrap();
        assert_eq!(retrieved, full_info);
    }

    #[test]
    fn test_delete_user_full() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;

        user_db
            .add_user_full(user_id, Bytes::from("full info"))
            .unwrap();

        // Verify exists
        assert!(user_db.get_user_full(user_id).is_ok());

        // Delete
        user_db.delete_user_full(user_id).unwrap();

        // Verify gone
        assert!(user_db.get_user_full(user_id).is_err());
    }

    #[test]
    fn test_get_user_count() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        // Add users
        for i in 1..=5 {
            user_db
                .add_user(i, Bytes::from(format!("user {}", i)), None, None)
                .unwrap();
        }

        assert_eq!(user_db.get_user_count().unwrap(), 5);
    }

    #[test]
    fn test_update_user() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;

        // Add initial user
        user_db
            .add_user(user_id, Bytes::from("original"), None, None)
            .unwrap();

        // Update with new data
        user_db
            .add_user(
                user_id,
                Bytes::from("updated"),
                Some(Bytes::from("photo")),
                Some("bio".to_string()),
            )
            .unwrap();

        let retrieved = user_db.get_user(user_id).unwrap();
        assert_eq!(retrieved, Bytes::from("updated"));

        let photo = user_db.get_user_profile_photo(user_id).unwrap();
        assert_eq!(photo, Some(Bytes::from("photo")));

        let bio = user_db.get_user_bio(user_id).unwrap();
        assert_eq!(bio, Some("bio".to_string()));
    }

    #[test]
    fn test_user_and_full_info_separate() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;

        // Add user
        user_db
            .add_user(user_id, Bytes::from("user data"), None, None)
            .unwrap();

        // Add full info
        user_db
            .add_user_full(user_id, Bytes::from("full info"))
            .unwrap();

        // Both should exist
        assert!(user_db.get_user(user_id).is_ok());
        assert!(user_db.get_user_full(user_id).is_ok());

        // Delete user
        user_db.delete_user(user_id).unwrap();

        // User should be gone, but full info should remain
        assert!(user_db.get_user(user_id).is_err());
        assert!(user_db.get_user_full(user_id).is_ok());
    }

    #[test]
    fn test_get_nonexistent_profile_photo() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;

        // Add user without photo
        user_db
            .add_user(user_id, Bytes::from("user data"), None, None)
            .unwrap();

        let photo = user_db.get_user_profile_photo(user_id).unwrap();
        assert_eq!(photo, None);
    }

    #[test]
    fn test_get_nonexistent_bio() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        let user_id = 12345i32;

        // Add user without bio
        user_db
            .add_user(user_id, Bytes::from("user data"), None, None)
            .unwrap();

        let bio = user_db.get_user_bio(user_id).unwrap();
        assert_eq!(bio, None);
    }

    #[test]
    fn test_get_users_nonexistent_ids() {
        let db = setup_test_db();
        let mut user_db = UserDbSync::new(db);

        // Add one user
        user_db
            .add_user(1, Bytes::from("user 1"), None, None)
            .unwrap();

        // Request multiple users, some that don't exist
        let users = user_db.get_users(vec![1, 999, 1000]).unwrap();
        assert_eq!(users.len(), 1);
    }
}
