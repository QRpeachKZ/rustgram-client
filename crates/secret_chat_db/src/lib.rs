// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Secret Chat Database
//!
//! Key-value storage interface for secret chat data persistence.
//!
//! ## Overview
//!
//! This module provides a simple database interface for storing secret chat
//! related data. It mirrors TDLib's `SecretChatDb` functionality, providing
//! typed key-value storage with serialization support.
//!
//! ## Architecture
//!
//! - [`SecretChatDb`] - Main database interface for secret chat operations
//! - [`KeyValueSyncInterface`] - Trait for key-value storage backends
//! - [`SecretChatValue`] - Trait for types that can be stored as secret chat values
//!
//! ## Example
//!
//! ```rust
//! use rustgram_secret_chat_db::{SecretChatDb, SecretChatValue};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! struct MySecretValue {
//!     data: String,
//! }
//!
//! impl SecretChatValue for MySecretValue {
//!     fn key() -> &'static str {
//!         "my_value"
//!     }
//! }
//!
//! // Use SecretChatDb with any KeyValueSyncInterface implementation
//! // let db = SecretChatDb::new(storage, chat_id);
//! // db.set_value(&value)?;
//! // let retrieved: MySecretValue = db.get_value()?;
//! ```

mod error;
mod interface;

pub use error::{SecretChatDbError, SecretChatDbResult};
pub use interface::KeyValueSyncInterface;

use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// Trait for values that can be stored in secret chat database.
///
/// Types implementing this trait can be stored and retrieved from
/// [`SecretChatDb`] using the [`set_value`](SecretChatDb::set_value) and
/// [`get_value`](SecretChatDb::get_value) methods.
pub trait SecretChatValue: Serialize + DeserializeOwned + Send + Sync {
    /// Returns the key suffix for this value type.
    ///
    /// The full key will be formatted as `"secret{chat_id}{key()}"`.
    fn key() -> &'static str;
}

/// Database interface for secret chat key-value storage.
///
/// Provides typed storage operations for secret chat related data.
/// Keys are formatted as `"secret{chat_id}{value_key}"` where `value_key`
/// comes from the [`SecretChatValue`] trait.
///
/// # Type Parameters
///
/// * `K` - The key-value storage backend implementing [`KeyValueSyncInterface`]
///
/// # Example
///
/// ```rust
/// use rustgram_secret_chat_db::{SecretChatDb, SecretChatValue};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// struct Layer {
///     version: i32,
/// }
///
/// impl SecretChatValue for Layer {
///     fn key() -> &'static str {
///         "layer"
///     }
/// }
///
/// // Mock storage would implement KeyValueSyncInterface
/// // let db = SecretChatDb::new(mock_storage, 12345);
/// // db.set_value(&Layer { version: 143 })?;
/// // let layer = db.get_value::<Layer>()?;
/// ```
#[derive(Debug, Clone)]
pub struct SecretChatDb<K: KeyValueSyncInterface> {
    /// The underlying key-value storage interface
    storage: Arc<K>,

    /// The secret chat ID for this database instance
    chat_id: i32,
}

impl<K: KeyValueSyncInterface> SecretChatDb<K> {
    /// Creates a new `SecretChatDb` instance.
    ///
    /// # Arguments
    ///
    /// * `storage` - The key-value storage backend
    /// * `chat_id` - The secret chat ID for this database
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_db::SecretChatDb;
    /// # struct MyStorage;
    /// # impl rustgram_secret_chat_db::KeyValueSyncInterface for MyStorage {
    /// #   fn set(&self, key: String, value: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// #   fn get(&self, key: String) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> { Ok(None) }
    /// #   fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// # }
    ///
    /// let storage = MyStorage;
    /// let db = SecretChatDb::new(storage, 12345);
    /// ```
    pub fn new(storage: K, chat_id: i32) -> Self {
        Self {
            storage: Arc::new(storage),
            chat_id,
        }
    }

    /// Returns the secret chat ID for this database.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_db::SecretChatDb;
    /// # struct MyStorage;
    /// # impl rustgram_secret_chat_db::KeyValueSyncInterface for MyStorage {
    /// #   fn set(&self, key: String, value: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// #   fn get(&self, key: String) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> { Ok(None) }
    /// #   fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// # }
    ///
    /// let storage = MyStorage;
    /// let db = SecretChatDb::new(storage, 12345);
    /// assert_eq!(db.chat_id(), 12345);
    /// ```
    pub fn chat_id(&self) -> i32 {
        self.chat_id
    }

    /// Formats a full storage key for a given value key suffix.
    ///
    /// Keys are formatted as `"secret{chat_id}{value_key}"`.
    fn format_key(&self, value_key: &str) -> String {
        format!("secret{}{}", self.chat_id, value_key)
    }

    /// Stores a value in the database.
    ///
    /// The value will be serialized using bincode and stored with a key
    /// formatted as `"secret{chat_id}{value_key}"`.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The value type implementing [`SecretChatValue`]
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails or the storage operation fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rustgram_secret_chat_db::{SecretChatDb, SecretChatValue};
    /// # use serde::{Deserialize, Serialize};
    /// # struct MyStorage;
    /// # impl rustgram_secret_chat_db::KeyValueSyncInterface for MyStorage {
    /// #   fn set(&self, key: String, value: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// #   fn get(&self, key: String) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> { Ok(None) }
    /// #   fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// # struct Layer { version: i32 }
    /// # impl SecretChatValue for Layer { fn key() -> &'static str { "layer" } }
    /// #
    /// # let storage = MyStorage;
    /// # let db = SecretChatDb::new(storage, 12345);
    /// # db.set_value(&Layer { version: 143 }).ok();
    /// ```
    pub fn set_value<V: SecretChatValue>(&self, value: &V) -> SecretChatDbResult<()> {
        let key = self.format_key(V::key());
        let serialized = bincode::serialize(value)
            .map_err(|e| SecretChatDbError::SerializationError(e.to_string()))?;
        self.storage
            .set(key, Bytes::from(serialized))
            .map_err(|e| SecretChatDbError::StorageError(e.to_string()))?;
        Ok(())
    }

    /// Removes a value from the database.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The value type implementing [`SecretChatValue`]
    ///
    /// # Errors
    ///
    /// Returns an error if the storage operation fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rustgram_secret_chat_db::{SecretChatDb, SecretChatValue};
    /// # use serde::{Deserialize, Serialize};
    /// # struct MyStorage;
    /// # impl rustgram_secret_chat_db::KeyValueSyncInterface for MyStorage {
    /// #   fn set(&self, key: String, value: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// #   fn get(&self, key: String) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> { Ok(None) }
    /// #   fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize)]
    /// # struct Layer { version: i32 }
    /// # impl SecretChatValue for Layer { fn key() -> &'static str { "layer" } }
    /// #
    /// # let storage = MyStorage;
    /// # let db = SecretChatDb::new(storage, 12345);
    /// # db.erase_value::<Layer>()?;
    /// # Ok::<(), rustgram_secret_chat_db::SecretChatDbError>(())
    /// ```
    pub fn erase_value<V: SecretChatValue>(&self) -> SecretChatDbResult<()> {
        let key = self.format_key(V::key());
        self.storage
            .erase(key)
            .map_err(|e| SecretChatDbError::StorageError(e.to_string()))?;
        Ok(())
    }

    /// Retrieves a value from the database.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The value type implementing [`SecretChatValue`]
    ///
    /// # Returns
    ///
    /// Returns the deserialized value if found.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The value is not found
    /// - Deserialization fails
    /// - The storage operation fails
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rustgram_secret_chat_db::{SecretChatDb, SecretChatValue};
    /// # use serde::{Deserialize, Serialize};
    /// # struct MyStorage;
    /// # impl rustgram_secret_chat_db::KeyValueSyncInterface for MyStorage {
    /// #   fn set(&self, key: String, value: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// #   fn get(&self, key: String) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
    /// #     Ok(Some(bytes::Bytes::from(vec![143, 0, 0, 0]))) // Mock serialized data
    /// #   }
    /// #   fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    /// # struct Layer { version: i32 }
    /// # impl SecretChatValue for Layer { fn key() -> &'static str { "layer" } }
    /// #
    /// # let storage = MyStorage;
    /// # let db = SecretChatDb::new(storage, 12345);
    /// # db.set_value(&Layer { version: 143 }).ok();
    /// # let layer = db.get_value::<Layer>()?;
    /// # Ok::<(), rustgram_secret_chat_db::SecretChatDbError>(())
    /// ```
    pub fn get_value<V: SecretChatValue>(&self) -> SecretChatDbResult<V> {
        let key = self.format_key(V::key());
        let data = self
            .storage
            .get(key.clone())
            .map_err(|e| SecretChatDbError::StorageError(e.to_string()))?
            .ok_or(SecretChatDbError::NotFound(key))?;
        let value = bincode::deserialize(&data)
            .map_err(|e| SecretChatDbError::DeserializationError(e.to_string()))?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock storage for testing
    #[derive(Debug, Clone)]
    struct MockStorage {
        data: Arc<Mutex<HashMap<String, Bytes>>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                data: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    impl KeyValueSyncInterface for MockStorage {
        fn set(&self, key: String, value: Bytes) -> Result<(), Box<dyn std::error::Error>> {
            let mut data = self.data.lock().unwrap();
            data.insert(key, value);
            Ok(())
        }

        fn get(&self, key: String) -> Result<Option<Bytes>, Box<dyn std::error::Error>> {
            let data = self.data.lock().unwrap();
            Ok(data.get(&key).cloned())
        }

        fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>> {
            let mut data = self.data.lock().unwrap();
            data.remove(&key);
            Ok(())
        }
    }

    // Test value types
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Layer {
        version: i32,
    }

    impl SecretChatValue for Layer {
        fn key() -> &'static str {
            "layer"
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Config {
        use_pfs: bool,
    }

    impl SecretChatValue for Config {
        fn key() -> &'static str {
            "config"
        }
    }

    #[test]
    fn test_secret_chat_db_creation() {
        let storage = MockStorage::new();
        let db = SecretChatDb::new(storage.clone(), 12345);
        assert_eq!(db.chat_id(), 12345);
    }

    #[test]
    fn test_format_key() {
        let storage = MockStorage::new();
        let db = SecretChatDb::new(storage, 12345);
        assert_eq!(db.format_key("layer"), "secret12345layer");
        assert_eq!(db.format_key("config"), "secret12345config");
    }

    #[test]
    fn test_set_and_get_value() {
        let storage = MockStorage::new();
        let db = SecretChatDb::new(storage, 12345);

        let layer = Layer { version: 143 };
        db.set_value(&layer).unwrap();

        let retrieved = db.get_value::<Layer>().unwrap();
        assert_eq!(retrieved, layer);
    }

    #[test]
    fn test_erase_value() {
        let storage = MockStorage::new();
        let db = SecretChatDb::new(storage, 12345);

        let layer = Layer { version: 143 };
        db.set_value(&layer).unwrap();
        db.erase_value::<Layer>().unwrap();

        let result = db.get_value::<Layer>();
        assert!(matches!(result, Err(SecretChatDbError::NotFound(_))));
    }

    #[test]
    fn test_multiple_values() {
        let storage = MockStorage::new();
        let db = SecretChatDb::new(storage, 12345);

        let layer = Layer { version: 143 };
        let config = Config { use_pfs: true };

        db.set_value(&layer).unwrap();
        db.set_value(&config).unwrap();

        let retrieved_layer = db.get_value::<Layer>().unwrap();
        let retrieved_config = db.get_value::<Config>().unwrap();

        assert_eq!(retrieved_layer, layer);
        assert_eq!(retrieved_config, config);
    }

    #[test]
    fn test_multiple_chat_ids() {
        let storage = MockStorage::new();
        let db1 = SecretChatDb::new(storage.clone(), 111);
        let db2 = SecretChatDb::new(storage, 222);

        let layer1 = Layer { version: 143 };
        let layer2 = Layer { version: 144 };

        db1.set_value(&layer1).unwrap();
        db2.set_value(&layer2).unwrap();

        let retrieved1 = db1.get_value::<Layer>().unwrap();
        let retrieved2 = db2.get_value::<Layer>().unwrap();

        assert_eq!(retrieved1, layer1);
        assert_eq!(retrieved2, layer2);
    }

    #[test]
    fn test_get_nonexistent_value() {
        let storage = MockStorage::new();
        let db = SecretChatDb::new(storage, 12345);

        let result = db.get_value::<Layer>();
        assert!(matches!(result, Err(SecretChatDbError::NotFound(_))));
    }
}
