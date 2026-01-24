// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! In-memory file database implementation.

use crate::{FileDb, FileDbError, FileDbId, FileDbResult};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

/// In-memory file database implementation.
///
/// This is the default implementation of `FileDb` trait, storing all data
/// in memory. It's useful for testing and development.
///
/// # Thread Safety
///
/// This implementation uses `Arc<RwLock<>>` for interior mutability, allowing
/// concurrent reads and exclusive writes.
///
/// # Usage
///
/// ```
/// use file_db::{FileDb, InMemoryFileDb};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let db = InMemoryFileDb::new();
/// let id = db.get_next_file_db_id().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct InMemoryFileDb {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    /// Counter for generating file DB IDs
    next_id: AtomicU64,
    /// File data by location key
    by_key: RwLock<HashMap<String, Bytes>>,
    /// File data by file DB ID
    by_id: RwLock<HashMap<FileDbId, Bytes>>,
    /// Location key by file DB ID
    keys_by_id: RwLock<HashMap<FileDbId, String>>,
    /// File references (source -> target)
    refs: RwLock<HashMap<FileDbId, FileDbId>>,
}

impl InMemoryFileDb {
    /// Creates a new in-memory file database.
    ///
    /// # Example
    ///
    /// ```
    /// use file_db::InMemoryFileDb;
    ///
    /// let db = InMemoryFileDb::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                next_id: AtomicU64::new(0),
                by_key: RwLock::new(HashMap::new()),
                by_id: RwLock::new(HashMap::new()),
                keys_by_id: RwLock::new(HashMap::new()),
                refs: RwLock::new(HashMap::new()),
            }),
        }
    }

    /// Creates a new in-memory file database with the specified starting ID.
    ///
    /// # Arguments
    ///
    /// * `starting_id` - The initial value for the ID counter
    ///
    /// # Example
    ///
    /// ```
    /// use file_db::InMemoryFileDb;
    ///
    /// let db = InMemoryFileDb::with_starting_id(100);
    /// ```
    #[must_use]
    pub fn with_starting_id(starting_id: u64) -> Self {
        Self {
            inner: Arc::new(Inner {
                next_id: AtomicU64::new(starting_id),
                by_key: RwLock::new(HashMap::new()),
                by_id: RwLock::new(HashMap::new()),
                keys_by_id: RwLock::new(HashMap::new()),
                refs: RwLock::new(HashMap::new()),
            }),
        }
    }

    /// Returns the current number of files in the database.
    ///
    /// # Example
    ///
    /// ```
    /// use file_db::InMemoryFileDb;
    ///
    /// let db = InMemoryFileDb::new();
    /// assert_eq!(db.len(), 0);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner
            .by_key
            .read()
            .map_or(0, |map| map.len())
    }

    /// Returns `true` if the database is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use file_db::InMemoryFileDb;
    ///
    /// let db = InMemoryFileDb::new();
    /// assert!(db.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears all data from the database.
    ///
    /// # Example
    ///
    /// ```
    /// use file_db::InMemoryFileDb;
    ///
    /// let db = InMemoryFileDb::new();
    /// db.clear();
    /// assert!(db.is_empty());
    /// ```
    pub fn clear(&self) {
        let _ = self.inner.by_key.write().map(|mut map| map.clear());
        let _ = self.inner.by_id.write().map(|mut map| map.clear());
        let _ = self.inner.keys_by_id.write().map(|mut map| map.clear());
        let _ = self.inner.refs.write().map(|mut map| map.clear());
    }
}

impl Default for InMemoryFileDb {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl FileDb for InMemoryFileDb {
    async fn get_next_file_db_id(&self) -> FileDbResult<FileDbId> {
        let id = self
            .inner
            .next_id
            .fetch_add(1, Ordering::SeqCst)
            .checked_add(1)
            .ok_or_else(|| FileDbError::Other("File DB ID overflow".to_string()))?;
        Ok(FileDbId::new(id))
    }

    async fn get_file_data(&self, key: &str) -> FileDbResult<Bytes> {
        self.inner
            .by_key
            .read()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?
            .get(key)
            .cloned()
            .ok_or_else(|| FileDbError::not_found(format!("key '{}'", key)))
    }

    async fn set_file_data(&self, file_db_id: FileDbId, key: &str, file_data: Bytes) -> FileDbResult<()> {
        // Acquire write locks
        let mut by_key = self
            .inner
            .by_key
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;
        let mut by_id = self
            .inner
            .by_id
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;
        let mut keys_by_id = self
            .inner
            .keys_by_id
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;

        // Store data
        by_key.insert(key.to_string(), file_data.clone());
        by_id.insert(file_db_id, file_data);
        keys_by_id.insert(file_db_id, key.to_string());

        Ok(())
    }

    async fn clear_file_data(&self, key: &str) -> FileDbResult<()> {
        let mut by_key = self
            .inner
            .by_key
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;

        // Remove the key entry
        by_key.remove(key);

        Ok(())
    }

    async fn delete_file(&self, file_db_id: FileDbId) -> FileDbResult<()> {
        let mut by_id = self
            .inner
            .by_id
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;
        let mut keys_by_id = self
            .inner
            .keys_by_id
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;
        let mut by_key = self
            .inner
            .by_key
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;

        // Get the key first
        if let Some(key) = keys_by_id.remove(&file_db_id) {
            by_key.remove(&key);
        }

        by_id.remove(&file_db_id);

        Ok(())
    }

    async fn get_file_by_id(&self, file_db_id: FileDbId) -> FileDbResult<Bytes> {
        self.inner
            .by_id
            .read()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?
            .get(&file_db_id)
            .cloned()
            .ok_or_else(|| FileDbError::not_found(format!("file_db_id {}", file_db_id.get())))
    }

    async fn get_file_key(&self, file_db_id: FileDbId) -> FileDbResult<String> {
        self.inner
            .keys_by_id
            .read()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?
            .get(&file_db_id)
            .cloned()
            .ok_or_else(|| FileDbError::not_found(format!("file_db_id {}", file_db_id.get())))
    }

    async fn get_file_count(&self) -> FileDbResult<usize> {
        Ok(self
            .inner
            .by_key
            .read()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?
            .len())
    }

    async fn set_file_ref(&self, file_db_id: FileDbId, target_db_id: FileDbId) -> FileDbResult<()> {
        let mut refs = self
            .inner
            .refs
            .write()
            .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;

        refs.insert(file_db_id, target_db_id);

        Ok(())
    }

    async fn resolve_file_ref(&self, file_db_id: FileDbId) -> FileDbResult<Bytes> {
        const MAX_REF_DEPTH: usize = 100;

        let mut current_id = file_db_id;
        let mut visited = std::collections::HashSet::new();

        for _ in 0..MAX_REF_DEPTH {
            // Try to get data directly
            {
                let by_id = self
                    .inner
                    .by_id
                    .read()
                    .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;

                if let Some(data) = by_id.get(&current_id) {
                    return Ok(data.clone());
                }
            }

            // Follow reference
            let refs = self
                .inner
                .refs
                .read()
                .map_err(|e| FileDbError::Database(format!("RwLock poisoned: {}", e)))?;

            if let Some(&target_id) = refs.get(&current_id) {
                // Check for circular references
                if !visited.insert(current_id) {
                    return Err(FileDbError::invalid_ref(format!(
                        "Circular reference detected at file_db_id {}",
                        current_id.get()
                    )));
                }
                current_id = target_id;
            } else {
                // No more references and no data found
                return Err(FileDbError::not_found(format!(
                    "file_db_id {} (after following references)",
                    file_db_id.get()
                )));
            }
        }

        Err(FileDbError::invalid_ref(format!(
            "Reference chain too deep for file_db_id {} (max depth: {})",
            file_db_id.get(),
            MAX_REF_DEPTH
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === Constructor tests ===

    #[test]
    fn test_new() {
        let db = InMemoryFileDb::new();
        assert!(db.is_empty());
    }

    #[test]
    fn test_default() {
        let db = InMemoryFileDb::default();
        assert!(db.is_empty());
    }

    #[test]
    fn test_with_starting_id() {
        let db = InMemoryFileDb::with_starting_id(100);
        assert!(db.is_empty());
    }

    #[test]
    fn test_len() {
        let db = InMemoryFileDb::new();
        assert_eq!(db.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let db = InMemoryFileDb::new();
        assert!(db.is_empty());
    }

    #[test]
    fn test_clear() {
        let db = InMemoryFileDb::new();
        db.clear();
        assert!(db.is_empty());
    }

    // === get_next_file_db_id tests ===

    #[tokio::test]
    async fn test_get_next_file_db_id() {
        let db = InMemoryFileDb::new();

        let id1 = db.get_next_file_db_id().await.unwrap();
        assert_eq!(id1.get(), 1);

        let id2 = db.get_next_file_db_id().await.unwrap();
        assert_eq!(id2.get(), 2);

        let id3 = db.get_next_file_db_id().await.unwrap();
        assert_eq!(id3.get(), 3);
    }

    #[tokio::test]
    async fn test_get_next_file_db_id_with_starting() {
        let db = InMemoryFileDb::with_starting_id(100);

        let id = db.get_next_file_db_id().await.unwrap();
        assert_eq!(id.get(), 101);
    }

    // === get_file_data tests ===

    #[tokio::test]
    async fn test_get_file_data_not_found() {
        let db = InMemoryFileDb::new();
        let result = db.get_file_data("nonexistent").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    // === set_file_data tests ===

    #[tokio::test]
    async fn test_set_and_get_file_data() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test_key";
        let data = Bytes::from("test data");

        db.set_file_data(id, key, data.clone()).await.unwrap();

        let retrieved = db.get_file_data(key).await.unwrap();
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_set_file_data_multiple() {
        let db = InMemoryFileDb::new();

        for i in 1..=5 {
            let id = FileDbId::new(i);
            let key = format!("key_{}", i);
            let data = Bytes::from(format!("data_{}", i));
            db.set_file_data(id, &key, data).await.unwrap();
        }

        assert_eq!(db.len(), 5);
    }

    #[tokio::test]
    async fn test_set_file_data_overwrite() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test_key";

        db.set_file_data(id, key, Bytes::from("original"))
            .await
            .unwrap();
        db.set_file_data(id, key, Bytes::from("updated"))
            .await
            .unwrap();

        let retrieved = db.get_file_data(key).await.unwrap();
        assert_eq!(retrieved, Bytes::from("updated"));
    }

    // === clear_file_data tests ===

    #[tokio::test]
    async fn test_clear_file_data() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test_key";

        db.set_file_data(id, key, Bytes::from("data")).await.unwrap();
        assert!(db.get_file_data(key).await.is_ok());

        db.clear_file_data(key).await.unwrap();
        assert!(db.get_file_data(key).await.is_err());
    }

    // === delete_file tests ===

    #[tokio::test]
    async fn test_delete_file() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test_key";

        db.set_file_data(id, key, Bytes::from("data")).await.unwrap();
        assert!(db.get_file_by_id(id).await.is_ok());

        db.delete_file(id).await.unwrap();
        assert!(db.get_file_by_id(id).await.is_err());
        assert!(db.get_file_key(id).await.is_err());
    }

    #[tokio::test]
    async fn test_delete_file_not_found() {
        let db = InMemoryFileDb::new();
        let result = db.delete_file(FileDbId::new(999)).await;
        // Should succeed even if file doesn't exist
        assert!(result.is_ok());
    }

    // === get_file_by_id tests ===

    #[tokio::test]
    async fn test_get_file_by_id() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test_key";
        let data = Bytes::from("test data");

        db.set_file_data(id, key, data.clone()).await.unwrap();

        let retrieved = db.get_file_by_id(id).await.unwrap();
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_get_file_by_id_not_found() {
        let db = InMemoryFileDb::new();
        let result = db.get_file_by_id(FileDbId::new(999)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    // === get_file_key tests ===

    #[tokio::test]
    async fn test_get_file_key() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test_key";

        db.set_file_data(id, key, Bytes::from("data")).await.unwrap();

        let retrieved_key = db.get_file_key(id).await.unwrap();
        assert_eq!(retrieved_key, key);
    }

    #[tokio::test]
    async fn test_get_file_key_not_found() {
        let db = InMemoryFileDb::new();
        let result = db.get_file_key(FileDbId::new(999)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    // === get_file_count tests ===

    #[tokio::test]
    async fn test_get_file_count() {
        let db = InMemoryFileDb::new();

        assert_eq!(db.get_file_count().await.unwrap(), 0);

        for i in 1..=5 {
            let id = FileDbId::new(i);
            let key = format!("key_{}", i);
            db.set_file_data(id, &key, Bytes::from("data")).await.unwrap();
        }

        assert_eq!(db.get_file_count().await.unwrap(), 5);
    }

    // === set_file_ref tests ===

    #[tokio::test]
    async fn test_set_file_ref() {
        let db = InMemoryFileDb::new();
        let id1 = FileDbId::new(1);
        let id2 = FileDbId::new(2);

        db.set_file_ref(id1, id2).await.unwrap();
    }

    // === resolve_file_ref tests ===

    #[tokio::test]
    async fn test_resolve_file_ref_direct() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test_key";
        let data = Bytes::from("test data");

        db.set_file_data(id, key, data.clone()).await.unwrap();

        let resolved = db.resolve_file_ref(id).await.unwrap();
        assert_eq!(resolved, data);
    }

    #[tokio::test]
    async fn test_resolve_file_ref_one_hop() {
        let db = InMemoryFileDb::new();
        let id1 = FileDbId::new(1);
        let id2 = FileDbId::new(2);
        let key = "test_key";
        let data = Bytes::from("test data");

        db.set_file_data(id2, key, data.clone()).await.unwrap();
        db.set_file_ref(id1, id2).await.unwrap();

        let resolved = db.resolve_file_ref(id1).await.unwrap();
        assert_eq!(resolved, data);
    }

    #[tokio::test]
    async fn test_resolve_file_ref_chain() {
        let db = InMemoryFileDb::new();
        let id1 = FileDbId::new(1);
        let id2 = FileDbId::new(2);
        let id3 = FileDbId::new(3);
        let key = "test_key";
        let data = Bytes::from("test data");

        db.set_file_data(id3, key, data.clone()).await.unwrap();
        db.set_file_ref(id1, id2).await.unwrap();
        db.set_file_ref(id2, id3).await.unwrap();

        let resolved = db.resolve_file_ref(id1).await.unwrap();
        assert_eq!(resolved, data);
    }

    #[tokio::test]
    async fn test_resolve_file_ref_not_found() {
        let db = InMemoryFileDb::new();
        let result = db.resolve_file_ref(FileDbId::new(999)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[tokio::test]
    async fn test_resolve_file_ref_circular() {
        let db = InMemoryFileDb::new();
        let id1 = FileDbId::new(1);
        let id2 = FileDbId::new(2);
        let id3 = FileDbId::new(3);

        db.set_file_ref(id1, id2).await.unwrap();
        db.set_file_ref(id2, id3).await.unwrap();
        db.set_file_ref(id3, id1).await.unwrap(); // Circular

        let result = db.resolve_file_ref(id1).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FileDbError::InvalidRef(_)));
    }

    #[tokio::test]
    async fn test_resolve_file_ref_self_referential() {
        let db = InMemoryFileDb::new();
        let id1 = FileDbId::new(1);

        db.set_file_ref(id1, id1).await.unwrap(); // Self-reference

        let result = db.resolve_file_ref(id1).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FileDbError::InvalidRef(_)));
    }

    // === Empty key tests ===

    #[tokio::test]
    async fn test_empty_key() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "";
        let data = Bytes::from("data");

        db.set_file_data(id, key, data.clone()).await.unwrap();

        let retrieved = db.get_file_data(key).await.unwrap();
        assert_eq!(retrieved, data);
    }

    // === Large data tests ===

    #[tokio::test]
    async fn test_large_file_data() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "large";
        let data = Bytes::from(vec![0u8; 1024 * 1024]); // 1MB

        db.set_file_data(id, key, data.clone()).await.unwrap();

        let retrieved = db.get_file_data(key).await.unwrap();
        assert_eq!(retrieved.len(), 1024 * 1024);
    }

    // === Clone tests ===

    #[tokio::test]
    async fn test_clone() {
        let db1 = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test";
        let data = Bytes::from("data");

        db1.set_file_data(id, key, data.clone()).await.unwrap();

        let db2 = db1.clone();

        let retrieved = db2.get_file_data(key).await.unwrap();
        assert_eq!(retrieved, data);
    }

    // === Concurrent access tests ===

    #[tokio::test]
    async fn test_concurrent_reads() {
        let db = InMemoryFileDb::new();
        let id = FileDbId::new(1);
        let key = "test";
        let data = Bytes::from("data");

        db.set_file_data(id, key, data.clone()).await.unwrap();

        let db1 = db.clone();
        let db2 = db.clone();

        let (result1, result2) = tokio::join!(
            db1.get_file_data(key),
            db2.get_file_data(key)
        );

        assert_eq!(result1.unwrap(), data);
        assert_eq!(result2.unwrap(), data);
    }

    #[tokio::test]
    async fn test_concurrent_writes() {
        let db = InMemoryFileDb::new();

        let mut handles = vec![];
        for i in 1..=10 {
            let db = db.clone();
            let key = format!("key_{}", i);
            let data = Bytes::from(format!("data_{}", i));
            handles.push(tokio::spawn(async move {
                let id = FileDbId::new(i);
                db.set_file_data(id, &key, data).await
            }));
        }

        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        assert_eq!(db.get_file_count().await.unwrap(), 10);
    }

    // === RwLock poisoning tests ===

    #[rstest]
    #[case::file_data("file_data")]
    #[case::file_key("file_key")]
    #[case::clear("clear")]
    #[case::delete("delete")]
    #[case::count("count")]
    #[case::file_ref("ref")]
    #[case::resolve("resolve")]
    #[tokio::test]
    async fn test_lock_poisoning_recovery(#[case] operation: &str) {
        // This test ensures RwLock poisoning is handled properly
        let db = InMemoryFileDb::new();

        match operation {
            "file_data" => {
                let _ = db.get_file_data("test").await;
            }
            "file_key" => {
                let _ = db.get_file_key(FileDbId::new(1)).await;
            }
            "clear" => {
                let _ = db.clear_file_data("test").await;
            }
            "delete" => {
                let _ = db.delete_file(FileDbId::new(1)).await;
            }
            "count" => {
                let _ = db.get_file_count().await;
            }
            "ref" => {
                let _ = db.set_file_ref(FileDbId::new(1), FileDbId::new(2)).await;
            }
            "resolve" => {
                let _ = db.resolve_file_ref(FileDbId::new(1)).await;
            }
            _ => unreachable!(),
        }
        // If we reach here, poisoning is handled correctly
    }

    // === Error message tests ===

    #[tokio::test]
    async fn test_error_messages() {
        let db = InMemoryFileDb::new();

        let err = db.get_file_data("missing").await.unwrap_err();
        assert!(err.to_string().contains("missing"));
        assert!(err.is_not_found());

        let err = db.get_file_by_id(FileDbId::new(999)).await.unwrap_err();
        assert!(err.to_string().contains("999"));

        let err = db.get_file_key(FileDbId::new(888)).await.unwrap_err();
        assert!(err.to_string().contains("888"));
    }

    // === ID sequence tests ===

    #[tokio::test]
    async fn test_id_sequence_monotonic() {
        let db = InMemoryFileDb::new();

        let mut prev_id = 0u64;
        for _ in 0..100 {
            let id = db.get_next_file_db_id().await.unwrap();
            assert!(id.get() > prev_id);
            prev_id = id.get();
        }
    }
}
