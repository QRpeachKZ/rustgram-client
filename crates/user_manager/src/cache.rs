//! Caching layer for user data.
//!
//! Implements TTL-based caching with automatic invalidation following the
//! DialogManager cache pattern.

use rustgram_user_id::UserId;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::{User, UserFull};

/// Default TTL for basic user data (5 minutes).
pub const DEFAULT_USER_TTL: Duration = Duration::from_secs(300);

/// Default TTL for full user data (10 minutes).
pub const DEFAULT_FULL_USER_TTL: Duration = Duration::from_secs(600);

/// Cache entry with TTL.
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    /// Cached value
    data: T,
    /// Expiration timestamp
    expires_at: Instant,
}

impl<T> CacheEntry<T> {
    /// Creates a new cache entry with TTL.
    #[must_use]
    fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
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

/// Cache statistics tracking.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Cache hit count
    hits: u64,
    /// Cache miss count
    misses: u64,
}

impl CacheStats {
    /// Creates a new cache stats tracker.
    #[must_use]
    pub const fn new() -> Self {
        Self { hits: 0, misses: 0 }
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

    /// Returns the number of cache hits.
    #[must_use]
    pub const fn hits(&self) -> u64 {
        self.hits
    }

    /// Returns the number of cache misses.
    #[must_use]
    pub const fn misses(&self) -> u64 {
        self.misses
    }
}

/// User cache with TTL support.
///
/// Thread-safe cache for user data with automatic expiration.
/// Follows the DialogManager cache pattern with separate caches for basic
/// and full user data.
///
/// # Examples
///
/// ```
/// use rustgram_user_manager::cache::UserCache;
///
/// let cache = UserCache::new();
///
/// // Set user
/// let user = rustgram_user_manager::User::new();
/// cache.set_user(rustgram_user_id::UserId::from_i32(123), user);
///
/// // Get cached user
/// let cached = cache.get_user(rustgram_user_id::UserId::from_i32(123));
/// ```
#[derive(Clone)]
pub struct UserCache {
    /// Basic user cache with TTL
    users: Arc<parking_lot::Mutex<HashMap<UserId, CacheEntry<User>>>>,
    /// Full user cache with longer TTL
    full_users: Arc<parking_lot::Mutex<HashMap<UserId, CacheEntry<UserFull>>>>,
    /// Cache statistics
    stats: Arc<parking_lot::Mutex<CacheStats>>,
}

impl UserCache {
    /// Creates a new user cache with default TTL values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_manager::cache::UserCache;
    ///
    /// let cache = UserCache::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            users: Arc::new(parking_lot::Mutex::new(HashMap::new())),
            full_users: Arc::new(parking_lot::Mutex::new(HashMap::new())),
            stats: Arc::new(parking_lot::Mutex::new(CacheStats::new())),
        }
    }

    /// Gets basic user from cache, returns None if expired or not found.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch
    ///
    /// # Returns
    ///
    /// `Some(user)` if cached and not expired, `None` otherwise
    #[must_use]
    pub fn get_user(&self, id: UserId) -> Option<User> {
        let mut users = self.users.lock();
        if let Some(entry) = users.get(&id) {
            if !entry.is_expired() {
                self.stats.lock().hits += 1;
                return Some(entry.data.clone());
            } else {
                // Expired, remove it
                users.remove(&id);
            }
        }
        self.stats.lock().misses += 1;
        None
    }

    /// Gets full user from cache, returns None if expired or not found.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch
    ///
    /// # Returns
    ///
    /// `Some(full_user)` if cached and not expired, `None` otherwise
    #[must_use]
    pub fn get_full_user(&self, id: UserId) -> Option<UserFull> {
        let mut full_users = self.full_users.lock();
        if let Some(entry) = full_users.get(&id) {
            if !entry.is_expired() {
                self.stats.lock().hits += 1;
                return Some(entry.data.clone());
            } else {
                // Expired, remove it
                full_users.remove(&id);
            }
        }
        self.stats.lock().misses += 1;
        None
    }

    /// Stores user in cache with default TTL.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    /// * `user` - User to cache
    pub fn set_user(&self, id: UserId, user: User) {
        self.users
            .lock()
            .insert(id, CacheEntry::new(user, DEFAULT_USER_TTL));
    }

    /// Stores full user in cache with extended TTL.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    /// * `full_user` - Full user to cache
    pub fn set_full_user(&self, id: UserId, full_user: UserFull) {
        self.full_users
            .lock()
            .insert(id, CacheEntry::new(full_user, DEFAULT_FULL_USER_TTL));
    }

    /// Removes user from all caches.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to invalidate
    pub fn invalidate_user(&self, id: UserId) {
        self.users.lock().remove(&id);
        self.full_users.lock().remove(&id);
    }

    /// Clears all cached data.
    pub fn invalidate_all(&self) {
        self.users.lock().clear();
        self.full_users.lock().clear();
    }

    /// Returns cache statistics.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_manager::cache::UserCache;
    ///
    /// let cache = UserCache::new();
    /// let stats = cache.stats();
    /// ```
    #[must_use]
    pub fn stats(&self) -> CacheStats {
        let stats = self.stats.lock();
        CacheStats {
            hits: stats.hits,
            misses: stats.misses,
        }
    }

    /// Returns the number of cached basic users.
    #[must_use]
    pub fn user_count(&self) -> usize {
        self.users.lock().len()
    }

    /// Returns the number of cached full users.
    #[must_use]
    pub fn full_user_count(&self) -> usize {
        self.full_users.lock().len()
    }

    /// Clears expired entries from the cache.
    ///
    /// This should be called periodically to free memory.
    pub fn cleanup_expired(&self) {
        // Clean up basic users
        let mut users = self.users.lock();
        users.retain(|_, entry| !entry.is_expired());

        // Clean up full users
        let mut full_users = self.full_users.lock();
        full_users.retain(|_, entry| !entry.is_expired());
    }

    /// Returns `true` if a basic user is cached and not expired.
    #[must_use]
    pub fn has_user(&self, id: UserId) -> bool {
        self.get_user(id).is_some()
    }

    /// Returns `true` if a full user is cached and not expired.
    #[must_use]
    pub fn has_full_user(&self, id: UserId) -> bool {
        self.get_full_user(id).is_some()
    }
}

impl Default for UserCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(id: i32, first_name: &str) -> User {
        let mut user = User::new();
        user.set_id(UserId::from_i32(id));
        user.set_first_name(first_name.to_string());
        user.set_deleted(false);
        user
    }

    #[test]
    fn test_cache_new() {
        let cache = UserCache::new();
        assert_eq!(cache.user_count(), 0);
        assert_eq!(cache.full_user_count(), 0);
        assert!(!cache.has_user(UserId::from_i32(1)));
    }

    #[test]
    fn test_cache_default() {
        let cache = UserCache::default();
        assert_eq!(cache.user_count(), 0);
        assert_eq!(cache.full_user_count(), 0);
    }

    #[test]
    fn test_get_set_user() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);
        let user = create_test_user(123, "Alice");

        // Initially not cached
        assert!(cache.get_user(id).is_none());
        assert!(!cache.has_user(id));

        // Set and get
        cache.set_user(id, user.clone());
        assert!(cache.has_user(id));

        let cached = cache.get_user(id);
        assert!(cached.is_some());
        let cached_user = cached.unwrap();
        assert_eq!(cached_user.first_name(), "Alice");
        assert_eq!(cached_user.id(), id);
    }

    #[test]
    fn test_get_set_full_user() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);
        let mut full_user = UserFull::new();
        full_user.user = Some(create_test_user(123, "Bob"));

        // Initially not cached
        assert!(cache.get_full_user(id).is_none());
        assert!(!cache.has_full_user(id));

        // Set and get
        cache.set_full_user(id, full_user.clone());
        assert!(cache.has_full_user(id));

        let cached = cache.get_full_user(id);
        assert!(cached.is_some());
        let cached_full = cached.unwrap();
        assert!(cached_full.user.is_some());
        assert_eq!(cached_full.user.unwrap().first_name(), "Bob");
    }

    #[test]
    fn test_invalidate_user() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);
        let user = create_test_user(123, "Charlie");
        let mut full_user = UserFull::new();
        full_user.user = Some(user.clone());

        // Set both basic and full user
        cache.set_user(id, user);
        cache.set_full_user(id, full_user);

        assert!(cache.has_user(id));
        assert!(cache.has_full_user(id));

        // Invalidate should remove both
        cache.invalidate_user(id);

        assert!(!cache.has_user(id));
        assert!(!cache.has_full_user(id));
    }

    #[test]
    fn test_invalidate_all() {
        let cache = UserCache::new();

        // Add multiple users
        for i in 1..=5 {
            let id = UserId::from_i32(i);
            cache.set_user(id, create_test_user(i, &format!("User{}", i)));
        }

        assert_eq!(cache.user_count(), 5);

        cache.invalidate_all();

        assert_eq!(cache.user_count(), 0);
        assert_eq!(cache.full_user_count(), 0);
    }

    #[test]
    fn test_cache_stats() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);
        let user = create_test_user(123, "Diana");

        // Initial stats
        let stats = cache.stats();
        assert_eq!(stats.hits(), 0);
        assert_eq!(stats.misses(), 0);
        assert_eq!(stats.total_requests(), 0);
        assert_eq!(stats.hit_rate(), 0.0);

        // Cache miss
        let _ = cache.get_user(id);
        let stats = cache.stats();
        assert_eq!(stats.hits(), 0);
        assert_eq!(stats.misses(), 1);

        // Cache hit
        cache.set_user(id, user);
        let _ = cache.get_user(id);
        let stats = cache.stats();
        assert_eq!(stats.hits(), 1);
        assert_eq!(stats.misses(), 1);

        // Hit rate calculation
        let expected_rate = 1.0 / 2.0; // 1 hit out of 2 requests
        assert!((stats.hit_rate() - expected_rate).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cache_hit_rate() {
        let stats = CacheStats::new();
        assert_eq!(stats.hit_rate(), 0.0);
        assert_eq!(stats.total_requests(), 0);
    }

    #[test]
    fn test_user_count() {
        let cache = UserCache::new();

        assert_eq!(cache.user_count(), 0);

        for i in 1..=5 {
            cache.set_user(UserId::from_i32(i), create_test_user(i, "User"));
        }

        assert_eq!(cache.user_count(), 5);
    }

    #[test]
    fn test_full_user_count() {
        let cache = UserCache::new();

        assert_eq!(cache.full_user_count(), 0);

        for i in 1..=3 {
            let mut full_user = UserFull::new();
            full_user.user = Some(create_test_user(i, "User"));
            cache.set_full_user(UserId::from_i32(i), full_user);
        }

        assert_eq!(cache.full_user_count(), 3);
    }

    #[test]
    fn test_cleanup_expired() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);

        // Set user (will expire after DEFAULT_USER_TTL)
        cache.set_user(id, create_test_user(123, "Eve"));
        assert_eq!(cache.user_count(), 1);

        // Cleanup should not remove non-expired entries
        cache.cleanup_expired();
        assert_eq!(cache.user_count(), 1);

        // Note: We can't easily test actual expiration without manipulating time
        // or using very short TTLs in a test-specific constructor
    }

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test", Duration::from_millis(10));
        assert!(!entry.is_expired());

        std::thread::sleep(Duration::from_millis(20));
        assert!(entry.is_expired());
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_USER_TTL, Duration::from_secs(300));
        assert_eq!(DEFAULT_FULL_USER_TTL, Duration::from_secs(600));
    }

    #[test]
    fn test_has_user() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);

        assert!(!cache.has_user(id));

        cache.set_user(id, create_test_user(123, "Frank"));
        assert!(cache.has_user(id));
    }

    #[test]
    fn test_has_full_user() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);

        assert!(!cache.has_full_user(id));

        let mut full_user = UserFull::new();
        full_user.user = Some(create_test_user(123, "Grace"));
        cache.set_full_user(id, full_user);
        assert!(cache.has_full_user(id));
    }

    #[test]
    fn test_separate_caches() {
        let cache = UserCache::new();
        let id = UserId::from_i32(123);

        // Set only basic user
        cache.set_user(id, create_test_user(123, "Henry"));

        assert!(cache.has_user(id));
        assert!(!cache.has_full_user(id));

        // Now set full user
        let mut full_user = UserFull::new();
        full_user.user = Some(create_test_user(123, "Henry"));
        cache.set_full_user(id, full_user);

        // Both should be cached independently
        assert!(cache.has_user(id));
        assert!(cache.has_full_user(id));
        assert_eq!(cache.user_count(), 1);
        assert_eq!(cache.full_user_count(), 1);
    }

    #[test]
    fn test_cache_stats_new() {
        let stats = CacheStats::new();
        assert_eq!(stats.hits(), 0);
        assert_eq!(stats.misses(), 0);
        assert_eq!(stats.total_requests(), 0);
        assert_eq!(stats.hit_rate(), 0.0);
    }

    #[test]
    fn test_cache_total_requests() {
        let stats = CacheStats {
            hits: 80,
            misses: 20,
        };
        assert_eq!(stats.total_requests(), 100);
        assert!((stats.hit_rate() - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cache_hit_rate_edge_cases() {
        let stats = CacheStats { hits: 0, misses: 0 };
        assert_eq!(stats.hit_rate(), 0.0);

        let stats = CacheStats {
            hits: 100,
            misses: 0,
        };
        assert_eq!(stats.hit_rate(), 1.0);

        let stats = CacheStats {
            hits: 0,
            misses: 100,
        };
        assert_eq!(stats.hit_rate(), 0.0);
    }

    #[test]
    fn test_multiple_users_invalidation() {
        let cache = UserCache::new();

        // Add multiple users
        for i in 1..=10 {
            cache.set_user(UserId::from_i32(i), create_test_user(i, "User"));
        }

        assert_eq!(cache.user_count(), 10);

        // Invalidate specific user
        cache.invalidate_user(UserId::from_i32(5));
        assert_eq!(cache.user_count(), 9);
        assert!(!cache.has_user(UserId::from_i32(5)));
        assert!(cache.has_user(UserId::from_i32(1)));
    }
}
