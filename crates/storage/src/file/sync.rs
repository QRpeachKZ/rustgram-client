//! Synchronous file database interface.

use bytes::Bytes;
use rusqlite::params;

use crate::connection::DbConnection;
use crate::error::{StorageError, StorageResult};

/// Synchronous file database interface.
pub struct FileDbSync {
    db: DbConnection,
}

impl FileDbSync {
    /// Creates a new synchronous file database interface.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Gets the next file database ID.
    ///
    /// This increments the internal sequence and returns the new ID.
    pub fn get_next_file_db_id(&mut self) -> StorageResult<i32> {
        let mut conn = self.db.connect()?;

        // Start transaction
        let tx = conn
            .transaction()
            .map_err(|e| StorageError::TransactionError(e.to_string()))?;

        // Get current sequence value
        let current: i32 = tx
            .query_row("SELECT seq FROM file_db_id_seq", [], |row| row.get(0))
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        let next_id = current + 1;

        // Update sequence
        tx.execute(
            "UPDATE file_db_id_seq SET seq = ?1",
            params![next_id],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        // Commit transaction
        tx.commit()
            .map_err(|e| StorageError::TransactionError(e.to_string()))?;

        Ok(next_id)
    }

    /// Gets file data by location key.
    ///
    /// # Arguments
    ///
    /// * `key` - Serialized location key
    pub fn get_file_data(&mut self, key: &str) -> StorageResult<Bytes> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT file_data FROM files WHERE key = ?1",
            params![key],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(Bytes::from(data))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("File with key {}", key))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Sets file data (merges with existing if any).
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - File database ID
    /// * `key` - Serialized location key
    /// * `file_data` - Serialized file data
    pub fn set_file_data(
        &mut self,
        file_db_id: i32,
        key: &str,
        file_data: Bytes,
    ) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "INSERT OR REPLACE INTO files (file_db_id, key, file_data)
             VALUES (?1, ?2, ?3)",
            params![file_db_id, key, file_data.as_ref()],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Clears file data for a specific location.
    ///
    /// # Arguments
    ///
    /// * `key` - Serialized location key
    pub fn clear_file_data(&mut self, key: &str) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute("DELETE FROM files WHERE key = ?1", params![key])
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Deletes file data by file_db_id.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - File database ID
    pub fn delete_file(&mut self, file_db_id: i32) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "DELETE FROM files WHERE file_db_id = ?1",
            params![file_db_id],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets file data by file_db_id.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - File database ID
    pub fn get_file_by_id(&mut self, file_db_id: i32) -> StorageResult<Bytes> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT file_data FROM files WHERE file_db_id = ?1",
            params![file_db_id],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(Bytes::from(data))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("File with file_db_id {}", file_db_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Gets the location key for a file by its ID.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - File database ID
    pub fn get_file_key(&mut self, file_db_id: i32) -> StorageResult<String> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT key FROM files WHERE file_db_id = ?1",
            params![file_db_id],
            |row| row.get(0),
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("File with file_db_id {}", file_db_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Gets the count of files in the database.
    pub fn get_file_count(&mut self) -> StorageResult<i32> {
        let conn = self.db.connect()?;

        conn.query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))
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
    use crate::file::schema::get_file_migrations;
    use crate::migrations::MigrationManager;
    use tempfile::tempdir;

    fn setup_test_db() -> DbConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        // Run migrations
        let mut manager = MigrationManager::new();
        for migration in get_file_migrations() {
            manager = manager.add_migration(migration);
        }
        manager.run(&db).unwrap();

        // Keep dir alive by intentionally leaking it
        std::mem::forget(dir);
        db
    }

    #[test]
    fn test_get_next_file_db_id() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        // First ID should be 1
        let id1 = file_db.get_next_file_db_id().unwrap();
        assert_eq!(id1, 1);

        // Second ID should be 2
        let id2 = file_db.get_next_file_db_id().unwrap();
        assert_eq!(id2, 2);

        // Third ID should be 3
        let id3 = file_db.get_next_file_db_id().unwrap();
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_set_and_get_file_data() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let file_db_id = 1i32;
        let key = "remote_location_1";
        let data = Bytes::from("test file data");

        file_db
            .set_file_data(file_db_id, key, data.clone())
            .unwrap();

        let retrieved = file_db.get_file_data(key).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_get_nonexistent_file() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let result = file_db.get_file_data("nonexistent_key");
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_clear_file_data() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let key = "remote_location_1";
        let data = Bytes::from("test file data");

        file_db.set_file_data(1, key, data).unwrap();

        // Verify exists
        assert!(file_db.get_file_data(key).is_ok());

        // Clear
        file_db.clear_file_data(key).unwrap();

        // Verify gone
        assert!(file_db.get_file_data(key).is_err());
    }

    #[test]
    fn test_delete_file() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let file_db_id = 1i32;
        let key = "remote_location_1";
        let data = Bytes::from("test file data");

        file_db.set_file_data(file_db_id, key, data).unwrap();

        // Verify exists
        assert!(file_db.get_file_by_id(file_db_id).is_ok());

        // Delete
        file_db.delete_file(file_db_id).unwrap();

        // Verify gone
        assert!(file_db.get_file_by_id(file_db_id).is_err());
    }

    #[test]
    fn test_get_file_by_id() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let file_db_id = 5i32;
        let key = "remote_location_5";
        let data = Bytes::from("test file data 5");

        file_db.set_file_data(file_db_id, key, data.clone()).unwrap();

        let retrieved = file_db.get_file_by_id(file_db_id).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_get_file_key() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let file_db_id = 10i32;
        let key = "remote_location_10";
        let data = Bytes::from("test file data 10");

        file_db.set_file_data(file_db_id, key, data).unwrap();

        let retrieved_key = file_db.get_file_key(file_db_id).unwrap();
        assert_eq!(retrieved_key, key);
    }

    #[test]
    fn test_get_file_count() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        // Add files
        for i in 1..=5 {
            file_db
                .set_file_data(i, &format!("key_{}", i), Bytes::from(format!("data {}", i)))
                .unwrap();
        }

        assert_eq!(file_db.get_file_count().unwrap(), 5);
    }

    #[test]
    fn test_update_file_data() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let file_db_id = 1i32;
        let key = "location_1";

        // Add initial data
        file_db
            .set_file_data(file_db_id, key, Bytes::from("original"))
            .unwrap();

        // Update with new data
        file_db
            .set_file_data(file_db_id, key, Bytes::from("updated"))
            .unwrap();

        let retrieved = file_db.get_file_data(key).unwrap();
        assert_eq!(retrieved, Bytes::from("updated"));
    }

    #[test]
    fn test_unique_key_constraint() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let key = "unique_key";

        // Add file with key
        file_db
            .set_file_data(1, key, Bytes::from("data1"))
            .unwrap();

        // Try to add another file with same key (should replace)
        file_db
            .set_file_data(2, key, Bytes::from("data2"))
            .unwrap();

        // Should have the updated data
        let retrieved = file_db.get_file_data(key).unwrap();
        assert_eq!(retrieved, Bytes::from("data2"));
    }

    #[test]
    fn test_multiple_locations_same_file() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        // Different file_db_id for each location (key is unique)
        let key1 = "remote_location_1";
        let key2 = "local_location_1";

        // Same file stored with different locations (different keys)
        file_db
            .set_file_data(1, key1, Bytes::from("remote_data"))
            .unwrap();

        file_db
            .set_file_data(2, key2, Bytes::from("local_data"))
            .unwrap();

        // Both should exist independently
        let remote_data = file_db.get_file_data(key1).unwrap();
        assert_eq!(remote_data, Bytes::from("remote_data"));

        let local_data = file_db.get_file_data(key2).unwrap();
        assert_eq!(local_data, Bytes::from("local_data"));
    }

    #[test]
    fn test_empty_key() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let key = "";
        let data = Bytes::from("empty key data");

        file_db.set_file_data(1, key, data.clone()).unwrap();

        let retrieved = file_db.get_file_data(key).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_large_file_data() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        // Create 1MB of data
        let large_data = vec![0u8; 1024 * 1024];
        let bytes = Bytes::from(large_data);

        file_db.set_file_data(1, "large_file", bytes.clone()).unwrap();

        let retrieved = file_db.get_file_data("large_file").unwrap();
        assert_eq!(retrieved.len(), 1024 * 1024);
    }

    #[test]
    fn test_get_nonexistent_by_id() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let result = file_db.get_file_by_id(99999);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_get_key_for_nonexistent() {
        let db = setup_test_db();
        let mut file_db = FileDbSync::new(db);

        let result = file_db.get_file_key(99999);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }
}
