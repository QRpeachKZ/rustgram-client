//! Asynchronous dialog database interface.

use bytes::Bytes;
use rustgram_types::DialogId;

use crate::connection::DbConnection;
use crate::dialog::sync::{DialogDbSync, DialogsResult};
use crate::error::{StorageError, StorageResult};

/// Asynchronous dialog database interface.
pub struct DialogDbAsync {
    db: DbConnection,
}

impl DialogDbAsync {
    /// Creates a new asynchronous dialog database interface.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Adds or updates a dialog asynchronously.
    pub async fn add_dialog(
        &self,
        dialog_id: DialogId,
        folder_id: Option<i32>,
        order: i64,
        data: Bytes,
    ) -> StorageResult<()> {
        let mut db = DialogDbSync::new(self.db.clone());
        tokio::task::spawn_blocking(move || {
            db.add_dialog(dialog_id, folder_id, order, data)
        })
        .await
        .map_err(|_| StorageError::TransactionError("Task join failed".to_string()))?
    }

    /// Gets a dialog by its ID asynchronously.
    pub async fn get_dialog(&self, dialog_id: DialogId) -> StorageResult<Bytes> {
        let mut db = DialogDbSync::new(self.db.clone());
        tokio::task::spawn_blocking(move || db.get_dialog(dialog_id))
            .await
            .map_err(|_| StorageError::TransactionError("Task join failed".to_string()))?
    }

    /// Gets multiple dialogs from a folder asynchronously.
    pub async fn get_dialogs(
        &self,
        folder_id: Option<i32>,
        order: i64,
        dialog_id: DialogId,
        limit: i32,
    ) -> StorageResult<DialogsResult> {
        let mut db = DialogDbSync::new(self.db.clone());
        tokio::task::spawn_blocking(move || {
            db.get_dialogs(folder_id, order, dialog_id, limit)
        })
        .await
        .map_err(|_| StorageError::TransactionError("Task join failed".to_string()))?
    }

    /// Deletes a dialog asynchronously.
    pub async fn delete_dialog(&self, dialog_id: DialogId) -> StorageResult<()> {
        let mut db = DialogDbSync::new(self.db.clone());
        tokio::task::spawn_blocking(move || db.delete_dialog(dialog_id))
            .await
            .map_err(|_| StorageError::TransactionError("Task join failed".to_string()))?
    }

    /// Gets the count of dialogs in a folder asynchronously.
    pub async fn get_dialog_count(&self, folder_id: Option<i32>) -> StorageResult<i32> {
        let mut db = DialogDbSync::new(self.db.clone());
        tokio::task::spawn_blocking(move || db.get_dialog_count(folder_id))
            .await
            .map_err(|_| StorageError::TransactionError("Task join failed".to_string()))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::schema::get_dialog_migrations;
    use crate::migrations::MigrationManager;
    use tempfile::tempdir;

    async fn setup_test_db() -> DbConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        // Run migrations
        let mut manager = MigrationManager::new();
        for migration in get_dialog_migrations() {
            manager = manager.add_migration(migration);
        }
        manager.run(&db).unwrap();

        db
    }

    #[tokio::test]
    async fn test_async_add_and_get_dialog() {
        let db = setup_test_db().await;
        let async_db = DialogDbAsync::new(db);

        let user_id = rustgram_types::UserId::new(12345).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let data = Bytes::from("test dialog data");

        // Add dialog
        async_db
            .add_dialog(dialog_id, Some(0), 100, data.clone())
            .await
            .unwrap();

        // Get dialog
        let retrieved = async_db.get_dialog(dialog_id).await.unwrap();
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_async_get_dialogs_paginated() {
        let db = setup_test_db().await;
        let async_db = DialogDbAsync::new(db);

        // Add multiple dialogs
        for i in 1..=10 {
            let user_id = rustgram_types::UserId::new(i).unwrap();
            let dialog_id = DialogId::from_user(user_id);
            async_db
                .add_dialog(
                    dialog_id,
                    Some(0),
                    i as i64 * 10,
                    Bytes::from(format!("dialog {}", i)),
                )
                .await
                .unwrap();
        }

        // Get first page
        let dummy_id = DialogId::from_user(rustgram_types::UserId::new(0).unwrap());
        let result = async_db
            .get_dialogs(Some(0), 1000, dummy_id, 5)
            .await
            .unwrap();
        assert_eq!(result.dialogs.len(), 5);
    }

    #[tokio::test]
    async fn test_async_delete_dialog() {
        let db = setup_test_db().await;
        let async_db = DialogDbAsync::new(db);

        let user_id = rustgram_types::UserId::new(12345).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        async_db
            .add_dialog(dialog_id, Some(0), 100, Bytes::from("test"))
            .await
            .unwrap();

        // Verify exists
        assert!(async_db.get_dialog(dialog_id).await.is_ok());

        // Delete
        async_db.delete_dialog(dialog_id).await.unwrap();

        // Verify gone
        assert!(async_db.get_dialog(dialog_id).await.is_err());
    }

    #[tokio::test]
    async fn test_async_get_dialog_count() {
        let db = setup_test_db().await;
        let async_db = DialogDbAsync::new(db);

        // Add dialogs to folder 0
        for i in 1..=5 {
            let user_id = rustgram_types::UserId::new(i).unwrap();
            let dialog_id = DialogId::from_user(user_id);
            async_db
                .add_dialog(dialog_id, Some(0), i as i64, Bytes::new())
                .await
                .unwrap();
        }

        let count = async_db.get_dialog_count(Some(0)).await.unwrap();
        assert_eq!(count, 5);
    }
}
