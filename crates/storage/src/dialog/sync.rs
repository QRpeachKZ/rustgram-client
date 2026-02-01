//! Synchronous dialog database interface.

use bytes::Bytes;
use rustgram_types::DialogId;

use crate::connection::DbConnection;
use crate::error::{StorageError, StorageResult};

/// Result from get_dialogs query.
#[derive(Debug, Clone)]
pub struct DialogsResult {
    /// List of dialogs with their serialized data.
    pub dialogs: Vec<(DialogId, Bytes)>,
    /// Order value for next page.
    pub next_order: i64,
    /// Dialog ID for next page.
    pub next_dialog_id: Option<DialogId>,
}

/// Synchronous dialog database interface.
pub struct DialogDbSync {
    db: DbConnection,
}

impl DialogDbSync {
    /// Creates a new synchronous dialog database interface.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Adds or updates a dialog in the database.
    pub fn add_dialog(
        &mut self,
        dialog_id: DialogId,
        folder_id: Option<i32>,
        order: i64,
        data: Bytes,
    ) -> StorageResult<()> {
        let conn = self.db.connect()?;

        // DialogId is stored as i64, using the encoded value
        let encoded_id = dialog_id.to_encoded();

        conn.execute(
            "INSERT OR REPLACE INTO dialogs (dialog_id, dialog_order, data, folder_id)
             VALUES (?1, ?2, ?3, ?4)",
            [
                &encoded_id as &dyn rusqlite::ToSql,
                &order as &dyn rusqlite::ToSql,
                &data.as_ref() as &dyn rusqlite::ToSql,
                &folder_id as &dyn rusqlite::ToSql,
            ],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets a dialog by its ID.
    pub fn get_dialog(&mut self, dialog_id: DialogId) -> StorageResult<Bytes> {
        let conn = self.db.connect()?;
        let encoded_id = dialog_id.to_encoded();

        conn.query_row(
            "SELECT data FROM dialogs WHERE dialog_id = ?1",
            [&encoded_id],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(Bytes::from(data))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("Dialog {}", encoded_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Gets multiple dialogs from a folder.
    pub fn get_dialogs(
        &mut self,
        folder_id: Option<i32>,
        order: i64,
        _dialog_id: DialogId, // TODO: use for pagination
        limit: i32,
    ) -> StorageResult<DialogsResult> {
        let conn = self.db.connect()?;

        let (sql, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(fid) = folder_id {
            (
                "SELECT dialog_id, dialog_order, data FROM dialogs
                 WHERE folder_id = ?1 AND dialog_order < ?2
                 ORDER BY dialog_order DESC, dialog_id DESC
                 LIMIT ?3"
                    .to_string(),
                vec![
                    Box::new(fid) as Box<dyn rusqlite::ToSql>,
                    Box::new(order),
                    Box::new(limit),
                ],
            )
        } else {
            (
                "SELECT dialog_id, dialog_order, data FROM dialogs
                 WHERE folder_id IS NULL AND dialog_order < ?1
                 ORDER BY dialog_order DESC, dialog_id DESC
                 LIMIT ?2"
                    .to_string(),
                vec![Box::new(order), Box::new(limit)],
            )
        };

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&sql)?;

        let mut dialogs = Vec::new();
        let mut rows = stmt.query(&params_refs[..])?;

        while let Some(row) = rows.next()? {
            let dialog_id: i64 = row.get(0)?;
            let dialog_order: i64 = row.get(1)?;
            let data: Vec<u8> = row.get(2)?;

            // Decode DialogId from i64
            let decoded_id = DialogId::from_encoded(dialog_id)
                .map_err(|e| StorageError::InvalidParameter(e.to_string()))?;

            dialogs.push((decoded_id, Bytes::from(data)));

            // Track next page info
            if dialogs.len() as i32 == limit {
                return Ok(DialogsResult {
                    dialogs,
                    next_order: dialog_order,
                    next_dialog_id: Some(
                        DialogId::from_encoded(dialog_id)
                            .map_err(|e| StorageError::InvalidParameter(e.to_string()))?,
                    ),
                });
            }
        }

        Ok(DialogsResult {
            dialogs,
            next_order: 0,
            next_dialog_id: None,
        })
    }

    /// Deletes a dialog from the database.
    pub fn delete_dialog(&mut self, dialog_id: DialogId) -> StorageResult<()> {
        let conn = self.db.connect()?;
        let encoded_id = dialog_id.to_encoded();

        conn.execute("DELETE FROM dialogs WHERE dialog_id = ?1", [&encoded_id])
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets the count of dialogs in a folder.
    pub fn get_dialog_count(&mut self, folder_id: Option<i32>) -> StorageResult<i32> {
        let conn = self.db.connect()?;

        let (sql, params) = if let Some(fid) = folder_id {
            (
                "SELECT COUNT(*) FROM dialogs WHERE folder_id = ?1",
                vec![Box::new(fid) as Box<dyn rusqlite::ToSql>],
            )
        } else {
            (
                "SELECT COUNT(*) FROM dialogs WHERE folder_id IS NULL",
                vec![],
            )
        };

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        conn.query_row(sql, &params_refs[..], |row| row.get(0))
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
    use crate::dialog::schema::get_dialog_migrations;
    use crate::migrations::MigrationManager;
    use tempfile::tempdir;

    fn setup_test_db() -> DbConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        // Run migrations
        let mut manager = MigrationManager::new();
        for migration in get_dialog_migrations() {
            manager = manager.add_migration(migration);
        }
        manager.run(&db).unwrap();

        // Keep dir alive by intentionally leaking it
        std::mem::forget(dir);
        db
    }

    #[test]
    fn test_add_and_get_dialog() {
        let db = setup_test_db();
        let mut dialog_db = DialogDbSync::new(db);

        // Use a valid DialogId (user type with ID 12345)
        let user_id = rustgram_types::UserId::new(12345).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let data = Bytes::from("test dialog data");

        // Add dialog
        dialog_db
            .add_dialog(dialog_id, Some(0), 100, data.clone())
            .unwrap();

        // Get dialog
        let retrieved = dialog_db.get_dialog(dialog_id).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_get_nonexistent_dialog() {
        let db = setup_test_db();
        let mut dialog_db = DialogDbSync::new(db);

        let user_id = rustgram_types::UserId::new(99999).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let result = dialog_db.get_dialog(dialog_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_get_dialogs_paginated() {
        let db = setup_test_db();
        let mut dialog_db = DialogDbSync::new(db);

        // Add multiple dialogs
        for i in 1..=10 {
            let user_id = rustgram_types::UserId::new(i).unwrap();
            let dialog_id = DialogId::from_user(user_id);
            dialog_db
                .add_dialog(
                    dialog_id,
                    Some(0),
                    i as i64 * 10,
                    Bytes::from(format!("dialog {}", i)),
                )
                .unwrap();
        }

        // Get first page (using a dummy DialogId for ordering)
        let dummy_id = DialogId::from_user(rustgram_types::UserId::new(999999).unwrap());
        let result = dialog_db.get_dialogs(Some(0), 1000, dummy_id, 5).unwrap();
        assert_eq!(result.dialogs.len(), 5);
    }

    #[test]
    fn test_delete_dialog() {
        let db = setup_test_db();
        let mut dialog_db = DialogDbSync::new(db);

        let user_id = rustgram_types::UserId::new(12345).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        dialog_db
            .add_dialog(dialog_id, Some(0), 100, Bytes::from("test"))
            .unwrap();

        // Verify exists
        assert!(dialog_db.get_dialog(dialog_id).is_ok());

        // Delete
        dialog_db.delete_dialog(dialog_id).unwrap();

        // Verify gone
        assert!(dialog_db.get_dialog(dialog_id).is_err());
    }

    #[test]
    fn test_get_dialog_count() {
        let db = setup_test_db();
        let mut dialog_db = DialogDbSync::new(db);

        // Add dialogs to folder 0
        for i in 1..=5 {
            let user_id = rustgram_types::UserId::new(i).unwrap();
            let dialog_id = DialogId::from_user(user_id);
            dialog_db
                .add_dialog(dialog_id, Some(0), i as i64, Bytes::new())
                .unwrap();
        }

        // Add dialogs to folder 1
        for i in 6..=8 {
            let user_id = rustgram_types::UserId::new(i).unwrap();
            let dialog_id = DialogId::from_user(user_id);
            dialog_db
                .add_dialog(dialog_id, Some(1), i as i64, Bytes::new())
                .unwrap();
        }

        assert_eq!(dialog_db.get_dialog_count(Some(0)).unwrap(), 5);
        assert_eq!(dialog_db.get_dialog_count(Some(1)).unwrap(), 3);
    }

    #[test]
    fn test_transaction() {
        let db = setup_test_db();
        let dialog_db = DialogDbSync::new(db);

        let mut conn = dialog_db.db().connect().unwrap();

        // Create a test table
        conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT)", [])
            .unwrap();

        {
            use crate::connection::Transaction;
            let mut tx = Transaction::new(&mut conn).unwrap();

            // Insert within transaction
            tx.tx_mut()
                .unwrap()
                .execute(
                    "INSERT INTO test (id, value) VALUES (?1, ?2)",
                    &[
                        &1i64 as &dyn rusqlite::ToSql,
                        &"test" as &dyn rusqlite::ToSql,
                    ],
                )
                .unwrap();

            tx.commit().unwrap();
        }

        // Verify data was committed
        let count = conn
            .query_row("SELECT COUNT(*) FROM test", [], |row| row.get::<_, i32>(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}
