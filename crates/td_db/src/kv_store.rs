// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Key-value store for settings and preferences.
//!
//! This module provides a simple SQLite-based key-value store for storing
//! application settings, notification settings, privacy settings, theme settings, etc.

use std::path::Path;

use bytes::Bytes;

use crate::TdDbParameters;

/// Error type for key-value store operations.
pub type KvResult<T> = Result<T, KvError>;

/// Errors that can occur in key-value store operations.
#[derive(Debug, thiserror::Error)]
pub enum KvError {
    /// Database connection error.
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    /// I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Value not found for the given key.
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    /// Invalid value type for the operation.
    #[error("Invalid value type: {0}")]
    InvalidType(String),
}

/// Key-value store for application settings.
///
/// This store uses SQLite to persist key-value pairs with BLOB values.
/// It's designed for storing settings like notification preferences,
/// privacy settings, theme settings, and other application preferences.
///
/// # Example
///
/// ```rust,no_run
/// use rustgram_td_db::{KeyValueStore, TdDbParameters};
/// use bytes::Bytes;
///
/// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let params = TdDbParameters::new(
///     "/path/to/db".to_string(),
///     "/path/to/files".to_string(),
///     false,
///     true
/// );
///
/// let store = KeyValueStore::open(&params)?;
///
/// // Store a binary value
/// store.set("theme", Bytes::from("dark"))?;
///
/// // Retrieve a value
/// let theme = store.get("theme")?;
/// assert_eq!(theme, Some(Bytes::from("dark")));
///
/// // Store and retrieve boolean values
/// store.set_bool("notifications_enabled", true)?;
/// let enabled = store.get_bool("notifications_enabled")?;
/// assert!(enabled);
///
/// # Ok(())
/// # }
/// ```
pub struct KeyValueStore {
    /// Database connection.
    conn: rusqlite::Connection,
}

impl KeyValueStore {
    /// Opens a key-value store with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Database parameters specifying the database directory
    ///
    /// # Returns
    ///
    /// Returns `Ok(KeyValueStore)` if the store was opened successfully.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open(params: &TdDbParameters) -> KvResult<Self> {
        let db_path = Path::new(params.database_directory()).join("kv_store.db");

        // Create parent directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = rusqlite::Connection::open(&db_path)?;

        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;

        // Create the key-value table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv_store (
                key TEXT PRIMARY KEY NOT NULL,
                value BLOB
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    /// Gets a value by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Bytes))` if the key exists, `Ok(None)` if it doesn't.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    /// use bytes::Bytes;
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// let value = store.get("my_key")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, key: &str) -> KvResult<Option<Bytes>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM kv_store WHERE key = ?")?;

        let result = stmt.query_row([key], |row| {
            let value: Vec<u8> = row.get(0)?;
            Ok(Bytes::from(value))
        });

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(KvError::from(e)),
        }
    }

    /// Sets a value for a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The value to store
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the value was set successfully.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    /// use bytes::Bytes;
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// store.set("my_key", Bytes::from("my_value"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set(&self, key: &str, value: Bytes) -> KvResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO kv_store (key, value) VALUES (?, ?)",
            (key, value.as_ref()),
        )?;
        Ok(())
    }

    /// Deletes a value by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the key was deleted, `Ok(false)` if it didn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// store.delete("my_key")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, key: &str) -> KvResult<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM kv_store WHERE key = ?", [key])?;
        Ok(rows_affected > 0)
    }

    /// Gets a boolean value by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns `Ok(bool)` if the key exists and contains a valid boolean.
    /// Returns `Err(KvError::KeyNotFound)` if the key doesn't exist.
    /// Returns `Err(KvError::InvalidType)` if the value is not a valid boolean.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// let enabled = store.get_bool("notifications_enabled")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_bool(&self, key: &str) -> KvResult<bool> {
        let value = self
            .get(key)?
            .ok_or_else(|| KvError::KeyNotFound(key.to_string()))?;

        if value.len() != 1 {
            return Err(KvError::InvalidType(
                "Boolean value must be exactly 1 byte".to_string(),
            ));
        }

        match value[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(KvError::InvalidType(
                "Boolean value must be 0 or 1".to_string(),
            )),
        }
    }

    /// Sets a boolean value for a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The boolean value to store
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the value was set successfully.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// store.set_bool("notifications_enabled", true)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_bool(&self, key: &str, value: bool) -> KvResult<()> {
        let bytes = if value { [1u8] } else { [0u8] };
        self.set(key, Bytes::copy_from_slice(&bytes))
    }

    /// Gets an integer value by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns `Ok(i64)` if the key exists and contains a valid integer.
    /// Returns `Err(KvError::KeyNotFound)` if the key doesn't exist.
    /// Returns `Err(KvError::InvalidType)` if the value is not a valid integer.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// let count = store.get_i64("message_count")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_i64(&self, key: &str) -> KvResult<i64> {
        let value = self
            .get(key)?
            .ok_or_else(|| KvError::KeyNotFound(key.to_string()))?;

        if value.len() != 8 {
            return Err(KvError::InvalidType(
                "Integer value must be exactly 8 bytes".to_string(),
            ));
        }

        let arr: [u8; 8] = value[..8]
            .try_into()
            .map_err(|_| KvError::InvalidType("Failed to convert to array".to_string()))?;

        Ok(i64::from_le_bytes(arr))
    }

    /// Sets an integer value for a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The integer value to store
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the value was set successfully.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// store.set_i64("message_count", 42)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_i64(&self, key: &str, value: i64) -> KvResult<()> {
        self.set(key, Bytes::copy_from_slice(&value.to_le_bytes()))
    }

    /// Gets a string value by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` if the key exists and contains valid UTF-8.
    /// Returns `Err(KvError::KeyNotFound)` if the key doesn't exist.
    /// Returns `Err(KvError::InvalidType)` if the value is not valid UTF-8.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// let theme = store.get_string("theme")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_string(&self, key: &str) -> KvResult<String> {
        let value = self
            .get(key)?
            .ok_or_else(|| KvError::KeyNotFound(key.to_string()))?;

        String::from_utf8(value.to_vec())
            .map_err(|_| KvError::InvalidType("Value is not valid UTF-8".to_string()))
    }

    /// Sets a string value for a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The string value to store
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the value was set successfully.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// store.set_string("theme", "dark")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_string(&self, key: &str, value: &str) -> KvResult<()> {
        self.set(key, Bytes::copy_from_slice(value.as_bytes()))
    }

    /// Lists all keys in the store.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<String>)` containing all keys in the store.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// let keys = store.list_keys()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_keys(&self) -> KvResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT key FROM kv_store ORDER BY key")?;

        let keys = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(keys)
    }

    /// Clears all keys from the store.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all keys were cleared successfully.
    ///
    /// # Warning
    ///
    /// This operation permanently deletes all data in the store.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustgram_td_db::{KeyValueStore, TdDbParameters};
    ///
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let params = TdDbParameters::new(
    ///     "/path/to/db".to_string(),
    ///     "/path/to/files".to_string(),
    ///     false,
    ///     true
    /// );
    ///
    /// let store = KeyValueStore::open(&params)?;
    /// store.clear()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear(&self) -> KvResult<()> {
        self.conn.execute("DELETE FROM kv_store", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_store() -> (KeyValueStore, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );
        let store = KeyValueStore::open(&params).unwrap();
        (store, dir)
    }

    #[test]
    fn test_kv_store_open() {
        let (store, _dir) = create_test_store();
        assert!(store.list_keys().unwrap().is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let (store, _dir) = create_test_store();

        store.set("test_key", Bytes::from("test_value")).unwrap();

        let value = store.get("test_key").unwrap();
        assert_eq!(value, Some(Bytes::from("test_value")));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let (store, _dir) = create_test_store();

        let value = store.get("nonexistent").unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn test_delete_existing_key() {
        let (store, _dir) = create_test_store();

        store.set("test_key", Bytes::from("test_value")).unwrap();
        let deleted = store.delete("test_key").unwrap();
        assert!(deleted);

        let value = store.get("test_key").unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let (store, _dir) = create_test_store();

        let deleted = store.delete("nonexistent").unwrap();
        assert!(!deleted);
    }

    #[test]
    fn test_update_existing_key() {
        let (store, _dir) = create_test_store();

        store.set("test_key", Bytes::from("old_value")).unwrap();
        store.set("test_key", Bytes::from("new_value")).unwrap();

        let value = store.get("test_key").unwrap();
        assert_eq!(value, Some(Bytes::from("new_value")));
    }

    #[test]
    fn test_set_and_get_bool() {
        let (store, _dir) = create_test_store();

        store.set_bool("enabled", true).unwrap();
        assert!(store.get_bool("enabled").unwrap());

        store.set_bool("enabled", false).unwrap();
        assert!(!store.get_bool("enabled").unwrap());
    }

    #[test]
    fn test_get_bool_nonexistent() {
        let (store, _dir) = create_test_store();

        let result = store.get_bool("nonexistent");
        assert!(matches!(result, Err(KvError::KeyNotFound(_))));
    }

    #[test]
    fn test_set_and_get_i64() {
        let (store, _dir) = create_test_store();

        store.set_i64("count", 42).unwrap();
        assert_eq!(store.get_i64("count").unwrap(), 42);

        store.set_i64("count", -1000).unwrap();
        assert_eq!(store.get_i64("count").unwrap(), -1000);
    }

    #[test]
    fn test_get_i64_nonexistent() {
        let (store, _dir) = create_test_store();

        let result = store.get_i64("nonexistent");
        assert!(matches!(result, Err(KvError::KeyNotFound(_))));
    }

    #[test]
    fn test_set_and_get_string() {
        let (store, _dir) = create_test_store();

        store.set_string("theme", "dark").unwrap();
        assert_eq!(store.get_string("theme").unwrap(), "dark");

        store.set_string("theme", "light").unwrap();
        assert_eq!(store.get_string("theme").unwrap(), "light");
    }

    #[test]
    fn test_get_string_nonexistent() {
        let (store, _dir) = create_test_store();

        let result = store.get_string("nonexistent");
        assert!(matches!(result, Err(KvError::KeyNotFound(_))));
    }

    #[test]
    fn test_list_keys() {
        let (store, _dir) = create_test_store();

        store.set_string("key1", "value1").unwrap();
        store.set_string("key2", "value2").unwrap();
        store.set_string("key3", "value3").unwrap();

        let keys = store.list_keys().unwrap();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        assert!(keys.contains(&"key3".to_string()));
    }

    #[test]
    fn test_list_keys_sorted() {
        let (store, _dir) = create_test_store();

        store.set_string("zebra", "value1").unwrap();
        store.set_string("apple", "value2").unwrap();
        store.set_string("banana", "value3").unwrap();

        let keys = store.list_keys().unwrap();
        assert_eq!(keys, vec!["apple", "banana", "zebra"]);
    }

    #[test]
    fn test_clear() {
        let (store, _dir) = create_test_store();

        store.set_string("key1", "value1").unwrap();
        store.set_string("key2", "value2").unwrap();

        store.clear().unwrap();

        assert!(store.list_keys().unwrap().is_empty());
    }

    #[test]
    fn test_empty_blob() {
        let (store, _dir) = create_test_store();

        store.set("empty", Bytes::new()).unwrap();
        let value = store.get("empty").unwrap();
        assert_eq!(value, Some(Bytes::new()));
    }

    #[test]
    fn test_large_blob() {
        let (store, _dir) = create_test_store();

        let large_data = vec![0u8; 10_000];
        store.set("large", Bytes::from(large_data.clone())).unwrap();

        let value = store.get("large").unwrap();
        assert_eq!(value.unwrap().to_vec(), large_data);
    }

    #[test]
    fn test_persistence() {
        let dir = tempdir().unwrap();
        let params = TdDbParameters::new(
            dir.path().to_str().unwrap().to_string(),
            "/files".to_string(),
            false,
            true,
        );

        // Create and populate store
        {
            let store = KeyValueStore::open(&params).unwrap();
            store.set_string("persistent", "value").unwrap();
        }

        // Reopen store and verify data persists
        {
            let store = KeyValueStore::open(&params).unwrap();
            let value = store.get_string("persistent").unwrap();
            assert_eq!(value, "value");
        }
    }
}
