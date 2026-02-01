//! Database connection management.

use std::path::Path;
use std::sync::Arc;

use crate::error::{StorageError, StorageResult};

/// SQLite database connection manager.
#[derive(Clone)]
pub struct DbConnection {
    /// Path to the database file.
    db_path: Arc<std::path::PathBuf>,
}

impl DbConnection {
    /// Creates a new database connection manager.
    pub fn new<P: AsRef<Path>>(db_path: P) -> StorageResult<Self> {
        let db_path = db_path.as_ref();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Open database to create file if it doesn't exist
        let conn = rusqlite::Connection::open(db_path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA foreign_keys=ON;",
        )?;
        drop(conn);

        Ok(Self {
            db_path: Arc::new(db_path.to_path_buf()),
        })
    }

    /// Opens a new connection to the database.
    pub fn connect(&self) -> StorageResult<rusqlite::Connection> {
        rusqlite::Connection::open(&*self.db_path).map_err(StorageError::from)
    }

    /// Returns the path to the database file.
    pub fn path(&self) -> &Path {
        &self.db_path
    }
}

/// A database transaction.
pub struct Transaction<'a> {
    tx: Option<rusqlite::Transaction<'a>>,
}

impl<'a> Transaction<'a> {
    /// Creates a new transaction from a connection.
    pub fn new(conn: &'a mut rusqlite::Connection) -> StorageResult<Self> {
        let tx = conn
            .transaction()
            .map_err(|e| StorageError::TransactionError(e.to_string()))?;
        Ok(Self { tx: Some(tx) })
    }

    /// Commits the transaction.
    pub fn commit(mut self) -> StorageResult<()> {
        match self.tx.take() {
            Some(tx) => tx
                .commit()
                .map_err(|e| StorageError::TransactionError(e.to_string())),
            None => Err(StorageError::TransactionError(
                "Transaction already consumed".to_string(),
            )),
        }
    }

    /// Rolls back the transaction.
    pub fn rollback(mut self) -> StorageResult<()> {
        match self.tx.take() {
            Some(tx) => tx
                .rollback()
                .map_err(|e| StorageError::TransactionError(e.to_string())),
            None => Err(StorageError::TransactionError(
                "Transaction already consumed".to_string(),
            )),
        }
    }

    /// Returns a reference to the underlying transaction.
    pub fn tx(&self) -> StorageResult<&rusqlite::Transaction<'a>> {
        self.tx.as_ref().ok_or_else(|| {
            StorageError::TransactionError("Transaction already consumed".to_string())
        })
    }

    /// Returns a mutable reference to the underlying transaction.
    pub fn tx_mut(&mut self) -> StorageResult<&mut rusqlite::Transaction<'a>> {
        self.tx.as_mut().ok_or_else(|| {
            StorageError::TransactionError("Transaction already consumed".to_string())
        })
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        // Rollback on drop if not committed
        if let Some(tx) = self.tx.take() {
            let _ = tx.rollback();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_connection_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = DbConnection::new(&db_path);
        assert!(conn.is_ok());
        assert!(db_path.exists());
    }

    #[test]
    fn test_connect() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let conn = db.connect().unwrap();

        // Simple query to test connection
        let result: String = conn
            .query_row("SELECT 'hello'", [], |row| row.get(0))
            .unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_transaction_commit() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let mut conn = db.connect().unwrap();

        // Create a test table
        conn.execute("CREATE TABLE test (id INTEGER, value TEXT)", [])
            .unwrap();

        {
            let mut tx = Transaction::new(&mut conn).unwrap();
            tx.tx_mut()
                .unwrap()
                .execute("INSERT INTO test (id, value) VALUES (1, 'test')", [])
                .unwrap();
            tx.commit().unwrap();
        }

        // Verify data was committed
        let value: String = conn
            .query_row("SELECT value FROM test WHERE id = 1", [], |row| row.get(0))
            .unwrap();
        assert_eq!(value, "test");
    }

    #[test]
    fn test_transaction_rollback() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let mut conn = db.connect().unwrap();

        // Create a test table
        conn.execute("CREATE TABLE test (id INTEGER, value TEXT)", [])
            .unwrap();

        {
            let mut tx = Transaction::new(&mut conn).unwrap();
            tx.tx_mut()
                .unwrap()
                .execute("INSERT INTO test (id, value) VALUES (1, 'test')", [])
                .unwrap();
            // Explicit rollback
            tx.rollback().unwrap();
        }

        // Verify data was not inserted
        let result = conn.query_row("SELECT value FROM test WHERE id = 1", [], |_| {
            Ok::<_, rusqlite::Error>("found")
        });
        assert!(matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)));
    }
}
