//! Caching layer for dialog data.
//!
//! Implements TTL-based caching with automatic invalidation.

use rustgram_types::DialogId;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::tl_types::Dialog;

/// Cache entry with TTL.
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    /// Cached value
    value: T,
    /// Expiration timestamp
    expires_at: Instant,
}

impl<T> CacheEntry<T> {
    /// Creates a new cache entry.
    #[must_use]
    fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: Instant::now().checked_add(ttl).unwrap_or_else(|| {
                // If overflow, use a far future time
                Instant::now() + Duration::from_secs(86400 * 365)
            }),
        }
    }

    /// Returns `true` if the entry has expired.
    #[must_use]
    fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

/// Dialog metadata for caching.
#[derive(Debug, Clone)]
pub struct DialogMetadata {
    /// Dialog title
    pub title: String,
    /// Unread count
    pub unread_count: i32,
    /// Top message ID
    pub top_message: i32,
}

/// Dialog cache with TTL support.
///
/// Thread-safe cache for dialog data with automatic expiration.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::cache::DialogCache;
///
/// let cache = DialogCache::new();
///
/// // Set dialog list
/// cache.set_dialogs(vec![]);
///
/// // Get cached dialogs
/// let dialogs = cache.get_dialogs();
/// ```
#[derive(Debug, Clone)]
pub struct DialogCache {
    /// Dialog list cache
    dialogs: Arc<parking_lot::Mutex<Option<CacheEntry<Vec<Dialog>>>>>,
    /// Individual dialog metadata cache
    metadata: Arc<parking_lot::Mutex<HashMap<DialogId, CacheEntry<DialogMetadata>>>>,
    /// Default TTL for cache entries
    default_ttl: Duration,
    /// Cache statistics
    stats: Arc<parking_lot::Mutex<CacheStats>>,
}

impl DialogCache {
    /// Default TTL for dialog list cache (1 minute).
    pub const DEFAULT_DIALOGS_TTL: Duration = Duration::from_secs(60);
    /// Default TTL for metadata cache (5 minutes).
    pub const DEFAULT_METADATA_TTL: Duration = Duration::from_secs(300);

    /// Creates a new cache with default TTL.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::cache::DialogCache;
    ///
    /// let cache = DialogCache::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::with_ttl(Self::DEFAULT_DIALOGS_TTL)
    }

    /// Creates a new cache with custom TTL.
    ///
    /// # Arguments
    ///
    /// * `ttl` - Default time-to-live for cache entries
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::cache::DialogCache;
    /// use std::time::Duration;
    ///
    /// let cache = DialogCache::with_ttl(Duration::from_secs(120));
    /// ```
    #[must_use]
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            dialogs: Arc::new(parking_lot::Mutex::new(None)),
            metadata: Arc::new(parking_lot::Mutex::new(HashMap::new())),
            default_ttl: ttl,
            stats: Arc::new(parking_lot::Mutex::new(CacheStats::default())),
        }
    }

    /// Gets cached dialogs if not expired.
    ///
    /// # Returns
    ///
    /// `Some(dialogs)` if cached and not expired, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::cache::DialogCache;
    ///
    /// let cache = DialogCache::new();
    ///
    /// // Initially empty
    /// assert!(cache.get_dialogs().is_none());
    ///
    /// // Set and get
    /// cache.set_dialogs(vec![]);
    /// assert!(cache.get_dialogs().is_some());
    /// ```
    #[must_use]
    pub fn get_dialogs(&self) -> Option<Vec<Dialog>> {
        let mut dialogs = self.dialogs.lock();

        if let Some(entry) = dialogs.as_ref() {
            if !entry.is_expired() {
                self.stats.lock().hits += 1;
                return Some(entry.value.clone());
            } else {
                // Expired, remove it
                *dialogs = None;
            }
        }

        self.stats.lock().misses += 1;
        None
    }

    /// Sets cached dialogs with TTL.
    ///
    /// # Arguments
    ///
    /// * `dialogs` - Dialogs to cache
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::cache::DialogCache;
    ///
    /// let cache = DialogCache::new();
    /// cache.set_dialogs(vec![]);
    /// ```
    pub fn set_dialogs(&self, dialogs: Vec<Dialog>) {
        let count = dialogs.len();
        *self.dialogs.lock() = Some(CacheEntry::new(dialogs, self.default_ttl));
        self.stats.lock().dialog_count = count;
    }

    /// Gets cached metadata if not expired.
    ///
    /// # Arguments
    ///
    /// * `id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Some(metadata)` if cached and not expired, `None` otherwise
    #[must_use]
    pub fn get_metadata(&self, id: DialogId) -> Option<DialogMetadata> {
        let mut metadata = self.metadata.lock();

        if let Some(entry) = metadata.get(&id) {
            if !entry.is_expired() {
                self.stats.lock().hits += 1;
                return Some(entry.value.clone());
            } else {
                // Expired, remove it
                metadata.remove(&id);
            }
        }

        self.stats.lock().misses += 1;
        None
    }

    /// Sets cached metadata with TTL.
    ///
    /// # Arguments
    ///
    /// * `id` - Dialog ID
    /// * `metadata` - Metadata to cache
    pub fn set_metadata(&self, id: DialogId, metadata: DialogMetadata) {
        let ttl = Duration::from_secs(300); // 5 minutes for metadata
        self.metadata
            .lock()
            .insert(id, CacheEntry::new(metadata, ttl));
    }

    /// Invalidates all cache entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::cache::DialogCache;
    ///
    /// let cache = DialogCache::new();
    /// cache.set_dialogs(vec![]);
    /// cache.invalidate_all();
    /// assert!(cache.get_dialogs().is_none());
    /// ```
    pub fn invalidate_all(&self) {
        self.dialogs.lock().take();
        self.metadata.lock().clear();
    }

    /// Invalidates a specific dialog.
    ///
    /// # Arguments
    ///
    /// * `id` - Dialog ID to invalidate
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustgram_dialog_manager::cache::{DialogCache, DialogMetadata};
    /// # use rustgram_types::{ChatId, DialogId};
    /// #
    /// # let cache = DialogCache::new();
    /// # let dialog_id = DialogId::from_chat(ChatId::new(1).unwrap());
    /// #
    /// # cache.set_metadata(dialog_id, DialogMetadata {
    /// #     title: "Test".to_string(),
    /// #     unread_count: 0,
    /// #     top_message: 0,
    /// # });
    ///
    /// cache.invalidate_dialog(dialog_id);
    /// assert!(cache.get_metadata(dialog_id).is_none());
    /// ```
    pub fn invalidate_dialog(&self, id: DialogId) {
        self.metadata.lock().remove(&id);
    }

    /// Returns cache statistics.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::cache::DialogCache;
    ///
    /// let cache = DialogCache::new();
    /// let stats = cache.stats();
    /// ```
    #[must_use]
    pub fn stats(&self) -> CacheStats {
        let stats = self.stats.lock();
        let metadata_count = self.metadata.lock().len();

        CacheStats {
            dialog_count: stats.dialog_count,
            metadata_count,
            hits: stats.hits,
            misses: stats.misses,
        }
    }

    /// Clears expired entries from the cache.
    ///
    /// This should be called periodically to free memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::cache::DialogCache;
    ///
    /// let cache = DialogCache::new();
    /// cache.cleanup_expired();
    /// ```
    pub fn cleanup_expired(&self) {
        // Clean up dialogs
        let mut dialogs = self.dialogs.lock();
        if let Some(entry) = dialogs.as_ref() {
            if entry.is_expired() {
                dialogs.take();
            }
        }

        // Clean up metadata
        let mut metadata = self.metadata.lock();
        metadata.retain(|_, entry| !entry.is_expired());
    }

    /// Returns the number of cached metadata entries.
    #[must_use]
    pub fn metadata_count(&self) -> usize {
        self.metadata.lock().len()
    }

    /// Returns `true` if dialogs are cached and not expired.
    #[must_use]
    pub fn has_dialogs(&self) -> bool {
        self.get_dialogs().is_some()
    }

    /// Returns `true` if metadata is cached and not expired.
    #[must_use]
    pub fn has_metadata(&self, id: DialogId) -> bool {
        self.get_metadata(id).is_some()
    }
}

impl Default for DialogCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics.
#[derive(Debug, Clone, PartialEq)]
pub struct CacheStats {
    /// Number of cached dialogs
    pub dialog_count: usize,
    /// Number of cached metadata entries
    pub metadata_count: usize,
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
}

impl CacheStats {
    /// Creates empty cache statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            dialog_count: 0,
            metadata_count: 0,
            hits: 0,
            misses: 0,
        }
    }

    /// Returns the total number of requests.
    #[must_use]
    pub const fn total_requests(&self) -> u64 {
        self.hits + self.misses
    }

    /// Returns the cache hit rate (0.0 to 1.0).
    ///
    /// Returns 0.0 if there were no requests.
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_requests();
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{ChatId, DialogId};

    #[test]
    fn test_cache_new() {
        let cache = DialogCache::new();

        assert_eq!(cache.default_ttl, DialogCache::DEFAULT_DIALOGS_TTL);
        assert!(!cache.has_dialogs());
    }

    #[test]
    fn test_cache_with_ttl() {
        let ttl = Duration::from_secs(120);
        let cache = DialogCache::with_ttl(ttl);

        assert_eq!(cache.default_ttl, ttl);
    }

    #[test]
    fn test_cache_default() {
        let cache = DialogCache::default();

        assert_eq!(cache.default_ttl, DialogCache::DEFAULT_DIALOGS_TTL);
    }

    #[test]
    fn test_get_set_dialogs() {
        let cache = DialogCache::new();

        // Initially empty
        assert!(cache.get_dialogs().is_none());

        // Set and get
        let dialogs = vec![Dialog {
            id: DialogId::Chat(ChatId::new(1).unwrap()),
            peer: crate::tl_types::Peer::Chat {
                chat_id: ChatId::new(1).unwrap(),
            },
            top_message: 123,
            unread_count: 5,
            read_inbox_max_id: 120,
            read_outbox_max_id: 115,
        }];

        cache.set_dialogs(dialogs.clone());
        assert!(cache.has_dialogs());

        let cached = cache.get_dialogs();
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);
    }

    #[test]
    fn test_get_set_metadata() {
        let cache = DialogCache::new();
        let dialog_id = DialogId::Chat(ChatId::new(1).unwrap());

        // Initially empty
        assert!(!cache.has_metadata(dialog_id));

        // Set and get
        let metadata = DialogMetadata {
            title: "Test".to_string(),
            unread_count: 5,
            top_message: 123,
        };

        cache.set_metadata(dialog_id, metadata.clone());
        assert!(cache.has_metadata(dialog_id));

        let cached = cache.get_metadata(dialog_id);
        assert!(cached.is_some());
        let cached_metadata = cached.unwrap();
        assert_eq!(cached_metadata.title, "Test");
        assert_eq!(cached_metadata.unread_count, 5);
    }

    #[test]
    fn test_invalidate_all() {
        let cache = DialogCache::new();

        cache.set_dialogs(vec![]);
        let dialog_id = DialogId::Chat(ChatId::new(1).unwrap());
        cache.set_metadata(
            dialog_id,
            DialogMetadata {
                title: "Test".to_string(),
                unread_count: 0,
                top_message: 0,
            },
        );

        assert!(cache.has_dialogs());
        assert!(cache.has_metadata(dialog_id));

        cache.invalidate_all();

        assert!(!cache.has_dialogs());
        assert!(!cache.has_metadata(dialog_id));
    }

    #[test]
    fn test_invalidate_dialog() {
        let cache = DialogCache::new();
        let dialog_id = DialogId::Chat(ChatId::new(1).unwrap());

        cache.set_metadata(
            dialog_id,
            DialogMetadata {
                title: "Test".to_string(),
                unread_count: 0,
                top_message: 0,
            },
        );

        assert!(cache.has_metadata(dialog_id));

        cache.invalidate_dialog(dialog_id);

        assert!(!cache.has_metadata(dialog_id));
    }

    #[test]
    fn test_cache_stats() {
        let cache = DialogCache::new();

        // Initial stats
        let stats = cache.stats();
        assert_eq!(stats.dialog_count, 0);
        assert_eq!(stats.metadata_count, 0);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);

        // Add dialogs
        cache.set_dialogs(vec![]);
        let stats = cache.stats();
        assert_eq!(stats.dialog_count, 0); // Empty vec

        // Check hit/miss tracking
        let _ = cache.get_dialogs(); // Hit
        let _ = cache.get_dialogs(); // Hit

        let stats = cache.stats();
        assert_eq!(stats.hits, 2);

        // Invalidate and check miss
        cache.invalidate_all();
        let _ = cache.get_dialogs(); // Miss

        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn test_hit_rate() {
        let stats = CacheStats::new();

        assert_eq!(stats.hit_rate(), 0.0);

        let stats = CacheStats {
            dialog_count: 0,
            metadata_count: 0,
            hits: 80,
            misses: 20,
        };

        assert!((stats.hit_rate() - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test".to_string(), Duration::from_millis(10));

        assert!(!entry.is_expired());

        std::thread::sleep(Duration::from_millis(20));

        assert!(entry.is_expired());
    }

    #[test]
    fn test_metadata_count() {
        let cache = DialogCache::new();

        assert_eq!(cache.metadata_count(), 0);

        let dialog_id1 = DialogId::Chat(ChatId::new(1).unwrap());
        let dialog_id2 = DialogId::Chat(ChatId::new(2).unwrap());

        cache.set_metadata(
            dialog_id1,
            DialogMetadata {
                title: "Test 1".to_string(),
                unread_count: 0,
                top_message: 0,
            },
        );

        assert_eq!(cache.metadata_count(), 1);

        cache.set_metadata(
            dialog_id2,
            DialogMetadata {
                title: "Test 2".to_string(),
                unread_count: 0,
                top_message: 0,
            },
        );

        assert_eq!(cache.metadata_count(), 2);
    }

    #[test]
    fn test_constants() {
        assert_eq!(DialogCache::DEFAULT_DIALOGS_TTL, Duration::from_secs(60));
        assert_eq!(DialogCache::DEFAULT_METADATA_TTL, Duration::from_secs(300));
    }
}
