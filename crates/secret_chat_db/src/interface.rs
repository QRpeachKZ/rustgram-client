// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Key-value storage interface for secret chat database.

use bytes::Bytes;

/// Synchronous key-value storage interface.
///
/// This trait defines the operations required for a backend storage
/// implementation that can be used with [`SecretChatDb`](crate::SecretChatDb).
///
/// # Example
///
/// ```rust
/// use rustgram_secret_chat_db::KeyValueSyncInterface;
/// use bytes::Bytes;
/// use std::collections::HashMap;
/// use std::sync::Mutex;
///
/// struct InMemoryStorage {
///     data: Mutex<HashMap<String, Bytes>>,
/// }
///
/// impl KeyValueSyncInterface for InMemoryStorage {
///     fn set(&self, key: String, value: Bytes) -> Result<(), Box<dyn std::error::Error>> {
///         let mut data = self.data.lock().unwrap();
///         data.insert(key, value);
///         Ok(())
///     }
///
///     fn get(&self, key: String) -> Result<Option<Bytes>, Box<dyn std::error::Error>> {
///         let data = self.data.lock().unwrap();
///         Ok(data.get(&key).cloned())
///     }
///
///     fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>> {
///         let mut data = self.data.lock().unwrap();
///         data.remove(&key);
///         Ok(())
///     }
/// }
/// ```
pub trait KeyValueSyncInterface: Send + Sync {
    /// Sets a key-value pair in storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The value to store
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn set(&self, key: String, value: Bytes) -> Result<(), Box<dyn std::error::Error>>;

    /// Gets a value from storage by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` if the key exists, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get(&self, key: String) -> Result<Option<Bytes>, Box<dyn std::error::Error>>;

    /// Removes a key-value pair from storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn erase(&self, key: String) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct TestStorage {
        data: Mutex<HashMap<String, Bytes>>,
    }

    impl KeyValueSyncInterface for TestStorage {
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

    #[test]
    fn test_set_and_get() {
        let storage = TestStorage {
            data: Mutex::new(HashMap::new()),
        };

        let value = Bytes::from("test_data");
        storage.set("key1".to_string(), value.clone()).unwrap();

        let retrieved = storage.get("key1".to_string()).unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[test]
    fn test_get_nonexistent() {
        let storage = TestStorage {
            data: Mutex::new(HashMap::new()),
        };

        let result = storage.get("nonexistent".to_string()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_erase() {
        let storage = TestStorage {
            data: Mutex::new(HashMap::new()),
        };

        let value = Bytes::from("test_data");
        storage.set("key1".to_string(), value).unwrap();
        storage.erase("key1".to_string()).unwrap();

        let result = storage.get("key1".to_string()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_overwrite() {
        let storage = TestStorage {
            data: Mutex::new(HashMap::new()),
        };

        storage
            .set("key1".to_string(), Bytes::from("value1"))
            .unwrap();
        storage
            .set("key1".to_string(), Bytes::from("value2"))
            .unwrap();

        let result = storage.get("key1".to_string()).unwrap();
        assert_eq!(result, Some(Bytes::from("value2")));
    }
}
