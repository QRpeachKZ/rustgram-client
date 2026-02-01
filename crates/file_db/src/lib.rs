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

//! # File Database
//!
//! Abstract file database backend with pluggable storage implementations.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileDbInterface` from `td/telegram/files/FileDb.h`.
//!
//! ## Architecture
//!
//! The `FileDb` trait provides an abstract interface for file metadata storage.
//! Implementations can use different backends:
//!
//! - **InMemoryFileDb**: Default in-memory implementation (for testing)
//! - **SqliteFileDb**: SQLite-based persistent storage (future)
//! - **SledFileDb**: Sled embedded database (future)
//!
//! ## Usage
//!
//! ```no_run
//! use file_db::{FileDb, InMemoryFileDb};
//! use rustgram_file_db_id::FileDbId;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create in-memory database
//! let db = InMemoryFileDb::new();
//!
//! // Get next file DB ID
//! let id = db.get_next_file_db_id().await?;
//! println!("Got file DB ID: {}", id.get());
//!
//! // Store file data
//! let key = "remote_location_123";
//! let data = vec![1u8, 2, 3, 4];
//! db.set_file_data(id, key, data.clone().into()).await?;
//!
//! // Retrieve file data
//! let retrieved = db.get_file_data(key).await?;
//! assert_eq!(retrieved.as_ref(), data.as_slice());
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

mod error;
mod in_memory;

pub use error::{FileDbError, FileDbResult};
pub use in_memory::InMemoryFileDb;

// Re-export FileDbId for convenience
pub use rustgram_file_db_id::FileDbId;

use bytes::Bytes;
use std::fmt;

/// Abstract file database backend trait.
///
/// This trait defines the interface for file metadata storage operations.
/// Implementations can use different storage backends (memory, SQLite, sled, etc.).
///
/// # TDLib Correspondence
///
/// Corresponds to `FileDbInterface` in `td/telegram/files/FileDb.h`.
///
/// # Thread Safety
///
/// Implementations must be thread-safe for concurrent access.
/// The default `InMemoryFileDb` uses `Arc<RwLock<HashMap>>` for interior mutability.
#[async_trait::async_trait]
pub trait FileDb: Send + Sync {
    /// Gets the next file database ID.
    ///
    /// This method should increment an internal counter and return the new ID.
    /// IDs must be unique and monotonically increasing within a database instance.
    ///
    /// # Returns
    ///
    /// The next available file database ID.
    ///
    /// # TDLib Correspondence
    ///
    /// Corresponds to `FileDbInterface::get_next_file_db_id()`.
    async fn get_next_file_db_id(&self) -> FileDbResult<FileDbId>;

    /// Gets file data by location key.
    ///
    /// # Arguments
    ///
    /// * `key` - The location key (serialized location object)
    ///
    /// # Returns
    ///
    /// The file data as bytes, or `FileDbError::NotFound` if not found.
    ///
    /// # TDLib Correspondence
    ///
    /// Corresponds to `FileDbInterface::get_file_data_sync_impl()`.
    async fn get_file_data(&self, key: &str) -> FileDbResult<Bytes>;

    /// Sets file data for a given file DB ID and location key.
    ///
    /// This method stores the file data and associates it with both:
    /// - The file DB ID (for direct lookup)
    /// - The location key (for reverse lookup)
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - The file database ID
    /// * `key` - The location key (serialized location object)
    /// * `file_data` - The serialized file data
    ///
    /// # TDLib Correspondence
    ///
    /// Corresponds to `FileDbInterface::set_file_data()`.
    async fn set_file_data(&self, file_db_id: FileDbId, key: &str, file_data: Bytes) -> FileDbResult<()>;

    /// Clears file data for a specific location key.
    ///
    /// # Arguments
    ///
    /// * `key` - The location key to clear
    ///
    /// # TDLib Correspondence
    ///
    /// Corresponds to `FileDbInterface::clear_file_data()`.
    async fn clear_file_data(&self, key: &str) -> FileDbResult<()>;

    /// Deletes file data by file DB ID.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - The file database ID to delete
    ///
    /// # TDLib Correspondence
    ///
    /// Corresponds to deleting the file entry from the database.
    async fn delete_file(&self, file_db_id: FileDbId) -> FileDbResult<()>;

    /// Gets file data by file DB ID.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - The file database ID
    ///
    /// # Returns
    ///
    /// The file data as bytes, or `FileDbError::NotFound` if not found.
    async fn get_file_by_id(&self, file_db_id: FileDbId) -> FileDbResult<Bytes>;

    /// Gets the location key for a file by its DB ID.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - The file database ID
    ///
    /// # Returns
    ///
    /// The location key string, or `FileDbError::NotFound` if not found.
    async fn get_file_key(&self, file_db_id: FileDbId) -> FileDbResult<String>;

    /// Gets the count of files in the database.
    ///
    /// # Returns
    ///
    /// The number of files stored in the database.
    async fn get_file_count(&self) -> FileDbResult<usize>;

    /// Creates a reference from one file DB ID to another.
    ///
    /// This is used for file deduplication - multiple file IDs can point
    /// to the same physical file data.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - The source file DB ID
    /// * `target_db_id` - The target file DB ID to reference
    ///
    /// # TDLib Correspondence
    ///
    /// Corresponds to `FileDbInterface::set_file_data_ref()`.
    async fn set_file_ref(&self, file_db_id: FileDbId, target_db_id: FileDbId) -> FileDbResult<()>;

    /// Resolves a file reference to get the actual file data.
    ///
    /// This follows file references until it finds the actual file data.
    ///
    /// # Arguments
    ///
    /// * `file_db_id` - The file database ID (may be a reference)
    ///
    /// # Returns
    ///
    /// The actual file data, or `FileDbError::NotFound` if not found.
    async fn resolve_file_ref(&self, file_db_id: FileDbId) -> FileDbResult<Bytes>;
}

impl fmt::Debug for dyn FileDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileDb")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_db_id_reexport() {
        let id = FileDbId::new(42);
        assert_eq!(id.get(), 42);
    }
}
