//! # Rustgram UserManager
//!
//! User manager for Telegram MTProto client with network integration.
//!
//! ## Overview
//!
//! This module provides functionality for managing user information in a Telegram client.
//! It includes the [`User`] struct which stores user data and the [`UserManager`] which
//! handles storage and retrieval of users.
//!
//! ## Network Integration
//!
//! The manager can fetch users from the network using TL (Type Language) serialization.
//! It supports:
//! - `users.getUsers` - Fetch multiple users by ID
//! - `users.getFullUser` - Fetch full user profile
//! - Caching with LRU (5000 user capacity)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rustgram_user_manager::{User, UserManager};
//! use rustgram_user_id::UserId;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = UserManager::new();
//!     let mut user = User::new();
//!     user.set_id(UserId::from_i32(123));
//!     user.set_first_name("Alice".to_string());
//!     user.set_deleted(false);
//!
//!     manager.add_user(user).await;
//!     assert!(manager.has_user(UserId::from_i32(123)).await);
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

pub mod cache;
pub mod network;
pub mod tl;

use lru::LruCache;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use rustgram_dialog_photo::DialogPhoto;
use rustgram_user_id::UserId;
use rustgram_usernames::Usernames;

pub use cache::{CacheStats, UserCache};
pub use network::{
    GetUserResult, GetUsersResult, MockNetworkClient, NetworkError, RealNetworkClient,
    UserNetworkClient,
};
pub use tl::{GetFullUserRequest, GetFullUserResponse, InputUser, UserFull, UserProfilePhoto};

/// Default LRU cache capacity for users.
const DEFAULT_CACHE_CAPACITY: usize = 5000;

/// Default timeout for network queries.
const DEFAULT_QUERY_TIMEOUT: Duration = Duration::from_secs(10);

/// TL constructor for `users.getUsers`.
#[allow(dead_code)]
const GET_USERS_CONSTRUCTOR: u32 = 0xd91a548;

/// TL constructor for `users.getFullUser`.
#[allow(dead_code)]
const GET_FULL_USER_CONSTRUCTOR: u32 = 0xb60f5918;

/// Basic user information.
///
/// Represents a Telegram user with all relevant profile data including
/// names, usernames, phone number, profile photo, and various status flags.
///
/// This is a simplified version of the TL `User` type. For full TL compatibility,
/// use the serialization methods in the `tl` module.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
    id: UserId,
    first_name: String,
    last_name: String,
    usernames: Usernames,
    phone_number: String,
    photo: DialogPhoto,
    is_verified: bool,
    is_premium: bool,
    is_bot: bool,
    is_deleted: bool,
    is_contact: bool,
    is_mutual_contact: bool,
}

impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}

impl User {
    /// Creates a new empty user with default values.
    ///
    /// The created user will have:
    /// - Empty ID (invalid)
    /// - Empty strings for names and phone number
    /// - Default/empty photo
    /// - All flags set to false except `is_deleted` which is true
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: UserId::default(),
            first_name: String::new(),
            last_name: String::new(),
            usernames: Usernames::default(),
            phone_number: String::new(),
            photo: DialogPhoto::default(),
            is_verified: false,
            is_premium: false,
            is_bot: false,
            is_deleted: true,
            is_contact: false,
            is_mutual_contact: false,
        }
    }

    /// Returns the user's ID.
    #[inline]
    #[must_use]
    pub const fn id(&self) -> UserId {
        self.id
    }

    /// Returns the user's first name.
    #[inline]
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns the user's last name.
    #[inline]
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns the user's usernames.
    #[inline]
    #[must_use]
    pub fn usernames(&self) -> &Usernames {
        &self.usernames
    }

    /// Returns the user's phone number.
    #[inline]
    #[must_use]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Returns the user's profile photo.
    #[inline]
    #[must_use]
    pub const fn photo(&self) -> &DialogPhoto {
        &self.photo
    }

    /// Returns whether the user is verified by Telegram.
    #[inline]
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.is_verified
    }

    /// Returns whether the user has Telegram Premium.
    #[inline]
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        self.is_premium
    }

    /// Returns whether this user is a bot.
    #[inline]
    #[must_use]
    pub const fn is_bot(&self) -> bool {
        self.is_bot
    }

    /// Returns whether the user account is deleted.
    #[inline]
    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    /// Returns whether this user is in the contacts list.
    #[inline]
    #[must_use]
    pub const fn is_contact(&self) -> bool {
        self.is_contact
    }

    /// Returns whether this is a mutual contact.
    #[inline]
    #[must_use]
    pub const fn is_mutual_contact(&self) -> bool {
        self.is_mutual_contact
    }

    /// Sets the user's ID.
    pub fn set_id(&mut self, id: UserId) {
        self.id = id;
    }

    /// Sets the user's first name.
    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    /// Sets the user's last name.
    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    /// Sets the user's usernames.
    pub fn set_usernames(&mut self, usernames: Usernames) {
        self.usernames = usernames;
    }

    /// Sets the user's phone number.
    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number = phone_number;
    }

    /// Sets the user's profile photo.
    pub fn set_photo(&mut self, photo: DialogPhoto) {
        self.photo = photo;
    }

    /// Sets the verified status.
    pub fn set_verified(&mut self, is_verified: bool) {
        self.is_verified = is_verified;
    }

    /// Sets the premium status.
    pub fn set_premium(&mut self, is_premium: bool) {
        self.is_premium = is_premium;
    }

    /// Sets whether this is a bot.
    pub fn set_bot(&mut self, is_bot: bool) {
        self.is_bot = is_bot;
    }

    /// Sets the deleted status.
    pub fn set_deleted(&mut self, is_deleted: bool) {
        self.is_deleted = is_deleted;
    }

    /// Sets whether this is a contact.
    pub fn set_contact(&mut self, is_contact: bool) {
        self.is_contact = is_contact;
    }

    /// Sets the mutual contact status.
    pub fn set_mutual_contact(&mut self, is_mutual_contact: bool) {
        self.is_mutual_contact = is_mutual_contact;
    }

    /// Returns whether this user is valid (has a non-zero ID).
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.id.get() != 0
    }

    /// Returns the user's display name.
    ///
    /// The display name is constructed as:
    /// - "{first_name} {last_name}" if both are present
    /// - "{first_name}" if only first name is present
    /// - "{last_name}" if only last name is present
    /// - The editable username if no names are present
    /// - "Deleted" if no information is available
    #[must_use]
    pub fn display_name(&self) -> String {
        if !self.first_name.is_empty() {
            if !self.last_name.is_empty() {
                format!("{} {}", self.first_name, self.last_name)
            } else {
                self.first_name.clone()
            }
        } else if !self.last_name.is_empty() {
            self.last_name.clone()
        } else {
            self.usernames
                .editable_username()
                .unwrap_or("Deleted")
                .to_string()
        }
    }
}

/// User manager.
///
/// Provides thread-safe storage and retrieval of user information with network integration.
/// Uses `Arc<RwLock<T>>` for concurrent access, LRU caching for performance,
/// and a separate TTL-based cache for network-fetched data.
///
/// # Example
///
/// ```rust
/// use rustgram_user_manager::UserManager;
/// use rustgram_user_id::UserId;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = UserManager::new();
/// assert_eq!(manager.user_count().await, 0);
///
/// // Set "my" user ID
/// manager.set_my_id(UserId::from_i32(123)).await;
/// assert_eq!(manager.get_my_id().await, Some(UserId::from_i32(123)));
/// # }
/// ```
#[derive(Clone)]
pub struct UserManager {
    /// LRU cache for users (primary storage).
    cache: Arc<RwLock<LruCache<UserId, User>>>,
    /// Current user's ID ("my" ID).
    my_id: Arc<RwLock<Option<UserId>>>,
    /// Network client for fetching users (optional).
    network_client: Arc<RwLock<Option<Arc<dyn UserNetworkClient>>>>,
    /// Real network client for direct TL queries (optional).
    real_network_client: Arc<RwLock<Option<RealNetworkClient>>>,
    /// TTL-based cache for network-fetched data.
    user_cache: UserCache,
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UserManager {
    /// Creates a new empty user manager with default cache capacity (5000 users).
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        // SAFETY: DEFAULT_CACHE_CAPACITY is 5000, which is > 0
        #[allow(clippy::expect_used)]
        let capacity = std::num::NonZeroUsize::new(DEFAULT_CACHE_CAPACITY)
            .expect("DEFAULT_CACHE_CAPACITY must be > 0");

        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity))),
            my_id: Arc::new(RwLock::new(None)),
            network_client: Arc::new(RwLock::new(None)),
            real_network_client: Arc::new(RwLock::new(None)),
            user_cache: UserCache::new(),
        }
    }

    /// Creates a new user manager with custom cache capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of users to cache (must be > 0)
    ///
    /// # Errors
    ///
    /// Returns an error if capacity is 0.
    pub fn with_capacity(capacity: usize) -> Result<Self, String> {
        let non_zero = std::num::NonZeroUsize::new(capacity)
            .ok_or_else(|| "capacity must be greater than 0".to_string())?;

        Ok(Self {
            cache: Arc::new(RwLock::new(LruCache::new(non_zero))),
            my_id: Arc::new(RwLock::new(None)),
            network_client: Arc::new(RwLock::new(None)),
            real_network_client: Arc::new(RwLock::new(None)),
            user_cache: UserCache::new(),
        })
    }

    /// Sets the network client for fetching users.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client implementation
    pub async fn set_network_client(&self, client: impl UserNetworkClient + 'static) {
        let mut network = self.network_client.write().await;
        *network = Some(Arc::new(client));
    }

    /// Removes the network client.
    pub async fn clear_network_client(&self) {
        let mut network = self.network_client.write().await;
        *network = None;
    }

    /// Adds a user to the manager.
    ///
    /// Returns `true` if the user was added (didn't previously exist),
    /// `false` if a user with this ID already existed (and was updated).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_user_manager::{User, UserManager};
    /// use rustgram_user_id::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = UserManager::new();
    /// let mut user = User::new();
    /// user.set_id(UserId::from_i32(123));
    /// assert!(manager.add_user(user.clone()).await);
    /// assert!(!manager.add_user(user).await);
    /// # }
    /// ```
    pub async fn add_user(&self, user: User) -> bool {
        let id = user.id();
        let mut cache = self.cache.write().await;
        if cache.contains(&id) {
            cache.put(id, user);
            false
        } else {
            cache.put(id, user);
            true
        }
    }

    /// Gets a user by ID from cache.
    ///
    /// Returns `None` if the user doesn't exist in cache.
    pub async fn get_user(&self, id: UserId) -> Option<User> {
        let mut cache = self.cache.write().await;
        cache.get(&id).cloned()
    }

    /// Fetches multiple users from the network.
    ///
    /// # Arguments
    ///
    /// * `ids` - User IDs to fetch
    ///
    /// # Returns
    ///
    /// A result containing successful fetches and errors.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::NoClient` if no network client is configured.
    pub async fn fetch_users(&self, ids: Vec<UserId>) -> Result<GetUsersResult, NetworkError> {
        let network = self.network_client.read().await;
        let client = network.as_ref().ok_or(NetworkError::NoClient)?;

        let input_users: Vec<InputUser> = ids.iter().map(|&id| InputUser::user(id)).collect();

        let result = client.get_users(input_users, DEFAULT_QUERY_TIMEOUT).await?;

        // Add successful users to cache
        for user in &result.users {
            self.add_user(user.clone()).await;
        }

        Ok(result)
    }

    /// Fetches the current user ("me") from the network.
    ///
    /// # Returns
    ///
    /// The current user if found.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::NoClient` if no network client is configured.
    pub async fn fetch_me(&self) -> Result<Option<User>, NetworkError> {
        let network = self.network_client.read().await;
        let client = network.as_ref().ok_or(NetworkError::NoClient)?;

        let user_opt: Option<User> = client
            .get_user(InputUser::self_(), DEFAULT_QUERY_TIMEOUT)
            .await?;

        if let Some(ref user) = user_opt {
            self.set_my_id(user.id()).await;
            self.add_user(user.clone()).await;
        }

        Ok(user_opt)
    }

    /// Removes a user by ID.
    ///
    /// Returns the removed user if it existed, `None` otherwise.
    pub async fn remove_user(&self, id: UserId) -> Option<User> {
        let mut cache = self.cache.write().await;
        cache.pop(&id)
    }

    /// Returns the number of users currently in cache.
    pub async fn user_count(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    /// Returns whether a user with the given ID exists in cache.
    pub async fn has_user(&self, id: UserId) -> bool {
        let cache = self.cache.read().await;
        cache.contains(&id)
    }

    /// Returns the current user's ID ("my" ID).
    pub async fn get_my_id(&self) -> Option<UserId> {
        let my_id = self.my_id.read().await;
        *my_id
    }

    /// Sets the current user's ID ("my" ID).
    pub async fn set_my_id(&self, id: UserId) {
        let mut my_id = self.my_id.write().await;
        *my_id = Some(id);
    }

    /// Gets the current user ("me") from cache.
    ///
    /// Returns `None` if:
    /// - "My" ID hasn't been set
    /// - The user with "my" ID doesn't exist in cache
    pub async fn get_me(&self) -> Option<User> {
        let my_id = self.my_id.read().await;
        if let Some(id) = *my_id {
            let mut cache = self.cache.write().await;
            cache.get(&id).cloned()
        } else {
            None
        }
    }

    /// Returns whether a user is a bot.
    ///
    /// Returns `false` if the user doesn't exist in cache.
    pub async fn is_bot(&self, id: UserId) -> bool {
        let mut cache = self.cache.write().await;
        cache.get(&id).map(|u| u.is_bot()).unwrap_or(false)
    }

    /// Returns whether a user has Premium.
    ///
    /// Returns `false` if the user doesn't exist in cache.
    pub async fn is_premium(&self, id: UserId) -> bool {
        let mut cache = self.cache.write().await;
        cache.get(&id).map(|u| u.is_premium()).unwrap_or(false)
    }

    /// Returns whether a user is deleted.
    ///
    /// Returns `true` if the user doesn't exist (deleted by default).
    pub async fn is_deleted(&self, id: UserId) -> bool {
        let mut cache = self.cache.write().await;
        cache.get(&id).map(|u| u.is_deleted()).unwrap_or(true)
    }

    /// Gets the display name for a user.
    ///
    /// Returns `None` if the user doesn't exist in cache.
    pub async fn get_display_name(&self, id: UserId) -> Option<String> {
        let mut cache = self.cache.write().await;
        cache.get(&id).map(|u| u.display_name())
    }

    /// Clears all users from cache.
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Returns all user IDs currently in cache.
    pub async fn all_user_ids(&self) -> Vec<UserId> {
        let cache = self.cache.read().await;
        cache.iter().map(|(&id, _)| id).collect()
    }

    // ========================================================================
    // Network Integration Methods
    // ========================================================================

    /// Sets the real network client for fetching users.
    ///
    /// This is separate from the mock network client and is used for
    /// direct TL queries to Telegram servers.
    ///
    /// # Arguments
    ///
    /// * `client` - Real network client implementation
    pub async fn set_real_network_client(&self, client: RealNetworkClient) {
        let mut network = self.real_network_client.write().await;
        *network = Some(client);
    }

    /// Fetches a user from cache or network.
    ///
    /// This method first checks the TTL-based cache, then falls back to the LRU cache,
    /// and finally fetches from the network if not found in either cache.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch
    ///
    /// # Returns
    ///
    /// - `Ok(Some(user))` - User found (from cache or network)
    /// - `Ok(None)` - User not found
    /// - `Err(NetworkError)` - Network error or no network client configured
    pub async fn fetch_user(&self, id: UserId) -> Result<Option<User>, NetworkError> {
        // Check TTL cache first
        if let Some(user) = self.user_cache.get_user(id) {
            return Ok(Some(user));
        }

        // Check LRU cache
        if let Some(user) = self.get_user(id).await {
            return Ok(Some(user));
        }

        // Fetch from network
        let network = self.network_client.read().await;
        let client = network.as_ref().ok_or(NetworkError::NoClient)?;

        let input_user = InputUser::user(id);
        let user = client.get_user(input_user, DEFAULT_QUERY_TIMEOUT).await?;

        if let Some(ref user) = user {
            // Cache in both caches
            self.user_cache.set_user(id, user.clone());
            self.add_user(user.clone()).await;
        }

        Ok(user)
    }

    /// Fetches full user profile from cache or network.
    ///
    /// This method first checks the TTL-based cache for full user data,
    /// then fetches from the network if not found.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch
    ///
    /// # Returns
    ///
    /// - `Ok(Some(full_user))` - Full user profile found
    /// - `Ok(None)` - User not found
    /// - `Err(NetworkError)` - Network error or no network client configured
    pub async fn fetch_full_user(&self, id: UserId) -> Result<Option<UserFull>, NetworkError> {
        // Check TTL cache first
        if let Some(full_user) = self.user_cache.get_full_user(id) {
            return Ok(Some(full_user));
        }

        // Fetch from network
        let network = self.network_client.read().await;
        let client = network.as_ref().ok_or(NetworkError::NoClient)?;

        let input_user = InputUser::user(id);
        let full_user_opt = client
            .get_full_user(input_user, DEFAULT_QUERY_TIMEOUT)
            .await?;

        // Update basic user info in LRU cache
        if let Some(ref full_user) = full_user_opt {
            // Cache full user in TTL cache
            self.user_cache.set_full_user(id, full_user.clone());

            // Update basic user in LRU cache
            if let Some(ref user) = full_user.user {
                self.add_user(user.clone()).await;
            }
        }

        Ok(full_user_opt)
    }

    /// Handles user update from server.
    ///
    /// This method should be called when a user update is received from the server.
    /// It invalidates the cached user data to force a refresh on next access.
    ///
    /// # Arguments
    ///
    /// * `update` - User update data (placeholder for future UpdateProcessor integration)
    pub fn on_user_update(&self, id: UserId) {
        // Invalidate cached user data
        self.user_cache.invalidate_user(id);
    }

    /// Invalidates cached user data for a specific user.
    ///
    /// This removes the user from both the TTL-based cache and the LRU cache.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to invalidate
    pub async fn invalidate_user(&self, id: UserId) {
        self.user_cache.invalidate_user(id);
        let mut cache = self.cache.write().await;
        cache.pop(&id);
    }

    /// Returns cache statistics.
    ///
    /// # Returns
    ///
    /// Statistics about the TTL-based cache including hit rate and request counts.
    #[must_use]
    pub fn cache_stats(&self) -> CacheStats {
        self.user_cache.stats()
    }

    /// Returns the user cache for direct access.
    #[must_use]
    pub fn user_cache(&self) -> &UserCache {
        &self.user_cache
    }

    // ========================================================================
    // UserFull Profile Getters (Subtask 1gk.1)
    // ========================================================================

    /// Gets the display name for a user from UserFull profile.
    ///
    /// This method fetches the full user profile (from cache or network)
    /// and returns the display name constructed from the user's first and last name.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch display name for
    ///
    /// # Returns
    ///
    /// - `Ok(Some(display_name))` - Display name found
    /// - `Ok(None)` - User not found
    /// - `Err(NetworkError)` - Network error or no network client configured
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_user_manager::UserManager;
    /// # use rustgram_user_id::UserId;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = UserManager::new();
    /// let user_id = UserId::from_i32(123);
    ///
    /// // Fetch display name (will try network if not in cache)
    /// match manager.get_display_name_full(user_id).await? {
    ///     Some(name) => println!("User: {}", name),
    ///     None => println!("User not found"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_display_name_full(&self, id: UserId) -> Result<Option<String>, NetworkError> {
        let full_user = self.fetch_full_user(id).await?;
        Ok(full_user.and_then(|u| u.user).map(|u| u.display_name()))
    }

    /// Gets the user status from UserFull profile.
    ///
    /// Returns the user's status as a string representation.
    /// The status can be: "Empty", "Online", "Offline", "Recently",
    /// "LastWeek", or "LastMonth" depending on the user's online state.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch status for
    ///
    /// # Returns
    ///
    /// - `Ok(Some(status))` - Status string found
    /// - `Ok(None)` - User not found
    /// - `Err(NetworkError)` - Network error or no network client configured
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_user_manager::UserManager;
    /// # use rustgram_user_id::UserId;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = UserManager::new();
    /// let user_id = UserId::from_i32(123);
    ///
    /// // Fetch user status
    /// match manager.get_status(user_id).await? {
    ///     Some(status) => println!("Status: {}", status),
    ///     None => println!("User not found"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_status(&self, id: UserId) -> Result<Option<String>, NetworkError> {
        let full_user = self.fetch_full_user(id).await?;
        Ok(full_user.map(|u| u.status().display_name().to_string()))
    }

    /// Gets the user's bio/about text from UserFull profile.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch bio for
    ///
    /// # Returns
    ///
    /// - `Ok(Some(bio))` - Bio text found
    /// - `Ok(None)` - User not found or no bio set
    /// - `Err(NetworkError)` - Network error or no network client configured
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_user_manager::UserManager;
    /// # use rustgram_user_id::UserId;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = UserManager::new();
    /// let user_id = UserId::from_i32(123);
    ///
    /// // Fetch user bio
    /// match manager.get_bio(user_id).await? {
    ///     Some(bio) => println!("Bio: {}", bio),
    ///     None => println!("No bio set"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bio(&self, id: UserId) -> Result<Option<String>, NetworkError> {
        let full_user = self.fetch_full_user(id).await?;
        Ok(full_user.and_then(|u| u.about).map(|s| s.to_string()))
    }

    /// Gets the user's profile photo from UserFull profile.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to fetch profile photo for
    ///
    /// # Returns
    ///
    /// - `Ok(Some(photo))` - Profile photo found
    /// - `Ok(None)` - User not found or no photo set
    /// - `Err(NetworkError)` - Network error or no network client configured
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_user_manager::UserManager;
    /// # use rustgram_user_id::UserId;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = UserManager::new();
    /// let user_id = UserId::from_i32(123);
    ///
    /// // Fetch profile photo
    /// match manager.get_profile_photo(user_id).await? {
    ///     Some(photo) => println!("Photo ID: {}", photo.photo_id()),
    ///     None => println!("No photo set"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_profile_photo(
        &self,
        id: UserId,
    ) -> Result<Option<tl::UserProfilePhoto>, NetworkError> {
        let full_user = self.fetch_full_user(id).await?;
        Ok(full_user.and_then(|u| u.profile_photo))
    }
}

impl fmt::Display for UserManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UserManager")
    }
}

/// Crate version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name for identification.
pub const CRATE_NAME: &str = "rustgram_user_manager";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_user_status::UserStatus;

    #[test]
    fn test_user_new() {
        let user = User::new();
        assert!(!user.is_valid());
        assert!(user.is_deleted());
    }

    #[test]
    fn test_user_valid() {
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("John".to_string());
        user.set_deleted(false);
        assert!(user.is_valid());
    }

    #[test]
    fn test_display_name() {
        let mut user = User::new();
        user.set_first_name("John".to_string());
        user.set_last_name("Doe".to_string());
        assert_eq!(user.display_name(), "John Doe");

        user.set_last_name(String::new());
        assert_eq!(user.display_name(), "John");
    }

    #[tokio::test]
    async fn test_manager_add_get() {
        let mgr = UserManager::new();
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Alice".to_string());
        user.set_deleted(false);
        mgr.add_user(user).await;
        assert_eq!(mgr.user_count().await, 1);
        assert!(mgr.has_user(UserId::from_i32(123)).await);
    }

    #[tokio::test]
    async fn test_get_display_name() {
        let mgr = UserManager::new();
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Bob".to_string());
        user.set_deleted(false);
        mgr.add_user(user).await;
        assert_eq!(
            mgr.get_display_name(UserId::from_i32(123)).await.as_deref(),
            Some("Bob")
        );
    }

    #[tokio::test]
    async fn test_my_id() {
        let mgr = UserManager::new();
        assert!(mgr.get_my_id().await.is_none());

        mgr.set_my_id(UserId::from_i32(999)).await;
        assert_eq!(mgr.get_my_id().await, Some(UserId::from_i32(999)));
    }

    #[tokio::test]
    async fn test_get_me() {
        let mgr = UserManager::new();
        mgr.set_my_id(UserId::from_i32(999)).await;

        let mut me = User::new();
        me.set_id(UserId::from_i32(999));
        me.set_first_name("Me".to_string());
        me.set_deleted(false);
        mgr.add_user(me).await;

        let found = mgr.get_me().await;
        assert!(found.is_some());
        assert_eq!(found.unwrap().first_name(), "Me");
    }

    #[tokio::test]
    async fn test_is_bot_premium_deleted() {
        let mgr = UserManager::new();

        let mut bot = User::new();
        bot.set_id(UserId::from_i32(1));
        bot.set_bot(true);
        bot.set_deleted(false);
        mgr.add_user(bot).await;
        assert!(mgr.is_bot(UserId::from_i32(1)).await);
        assert!(!mgr.is_premium(UserId::from_i32(1)).await);
        assert!(!mgr.is_deleted(UserId::from_i32(1)).await);

        let mut premium = User::new();
        premium.set_id(UserId::from_i32(2));
        premium.set_premium(true);
        premium.set_deleted(false);
        mgr.add_user(premium).await;
        assert!(mgr.is_premium(UserId::from_i32(2)).await);
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram_user_manager");
    }

    #[tokio::test]
    async fn test_with_capacity() {
        let mgr = UserManager::with_capacity(100).unwrap();
        assert_eq!(mgr.user_count().await, 0);

        let mut user = User::new();
        user.set_id(UserId::from_i32(1));
        user.set_deleted(false);
        mgr.add_user(user).await;
        assert_eq!(mgr.user_count().await, 1);
    }

    #[tokio::test]
    async fn test_remove_user() {
        let mgr = UserManager::new();
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_deleted(false);
        mgr.add_user(user.clone()).await;

        let removed = mgr.remove_user(UserId::from_i32(123)).await;
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id(), UserId::from_i32(123));
        assert!(!mgr.has_user(UserId::from_i32(123)).await);
    }

    #[tokio::test]
    async fn test_clear() {
        let mgr = UserManager::new();
        for i in 1..=10 {
            let mut user = User::new();
            user.set_id(UserId::from_i32(i));
            user.set_deleted(false);
            mgr.add_user(user).await;
        }
        assert_eq!(mgr.user_count().await, 10);

        mgr.clear().await;
        assert_eq!(mgr.user_count().await, 0);
    }

    #[tokio::test]
    async fn test_all_user_ids() {
        let mgr = UserManager::new();
        let ids = vec![1, 2, 3, 4, 5];

        for &id in &ids {
            let mut user = User::new();
            user.set_id(UserId::from_i32(id));
            user.set_deleted(false);
            mgr.add_user(user).await;
        }

        let result = mgr.all_user_ids().await;
        assert_eq!(result.len(), 5);
        assert!(result.contains(&UserId::from_i32(1)));
        assert!(result.contains(&UserId::from_i32(5)));
    }

    #[tokio::test]
    async fn test_no_network_client() {
        let mgr = UserManager::new();
        let result = mgr.fetch_user(UserId::from_i32(123)).await;

        assert!(matches!(result, Err(NetworkError::NoClient)));
    }

    #[tokio::test]
    async fn test_lru_cache_eviction() {
        let mgr = UserManager::with_capacity(3).unwrap();

        // Add 3 users
        for i in 1..=3 {
            let mut user = User::new();
            user.set_id(UserId::from_i32(i));
            user.set_first_name(format!("User{}", i));
            user.set_deleted(false);
            mgr.add_user(user).await;
        }

        assert_eq!(mgr.user_count().await, 3);

        // Add 4th user, should evict first
        let mut user4 = User::new();
        user4.set_id(UserId::from_i32(4));
        user4.set_first_name("User4".to_string());
        user4.set_deleted(false);
        mgr.add_user(user4).await;

        assert_eq!(mgr.user_count().await, 3);
        assert!(!mgr.has_user(UserId::from_i32(1)).await);
        assert!(mgr.has_user(UserId::from_i32(2)).await);
        assert!(mgr.has_user(UserId::from_i32(4)).await);
    }

    // =========================================================================
    // User struct tests (additional)
    // =========================================================================

    #[test]
    fn test_user_setters_getters() {
        let mut user = User::new();

        // Test all setters
        user.set_id(UserId::from_i32(123));
        user.set_first_name("John".to_string());
        user.set_last_name("Doe".to_string());
        user.set_phone_number("+1234567890".to_string());
        user.set_verified(true);
        user.set_premium(true);
        user.set_bot(false);
        user.set_deleted(false);
        user.set_contact(true);
        user.set_mutual_contact(true);

        assert_eq!(user.id(), UserId::from_i32(123));
        assert_eq!(user.first_name(), "John");
        assert_eq!(user.last_name(), "Doe");
        assert_eq!(user.phone_number(), "+1234567890");
        assert!(user.is_verified());
        assert!(user.is_premium());
        assert!(!user.is_bot());
        assert!(!user.is_deleted());
        assert!(user.is_contact());
        assert!(user.is_mutual_contact());
    }

    #[test]
    fn test_user_display_name_fallback() {
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_deleted(false);

        // No names, no username -> "Deleted"
        assert_eq!(user.display_name(), "Deleted");

        // Only last name
        user.set_last_name("Smith".to_string());
        assert_eq!(user.display_name(), "Smith");

        // First name only
        user.set_last_name(String::new());
        user.set_first_name("John".to_string());
        assert_eq!(user.display_name(), "John");

        // Both names
        user.set_last_name("Doe".to_string());
        assert_eq!(user.display_name(), "John Doe");
    }

    #[test]
    fn test_user_clone_equality() {
        let mut user1 = User::new();
        user1.set_id(UserId::from_i32(123));
        user1.set_first_name("Alice".to_string());
        user1.set_deleted(false);

        let user2 = user1.clone();
        assert_eq!(user1, user2);
        assert_eq!(user1.id(), user2.id());
        assert_eq!(user1.first_name(), user2.first_name());
    }

    #[test]
    fn test_user_default() {
        let user = User::default();
        assert!(!user.is_valid());
        assert!(user.is_deleted());
        assert_eq!(user.first_name(), "");
        assert_eq!(user.last_name(), "");
    }

    #[test]
    fn test_user_with_usernames() {
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));

        let usernames = Usernames::default();
        user.set_usernames(usernames.clone());

        assert_eq!(user.usernames(), &usernames);
    }

    #[test]
    fn test_user_with_photo() {
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));

        let photo = DialogPhoto::default();
        user.set_photo(photo.clone());

        assert_eq!(user.photo(), &photo);
    }

    // =========================================================================
    // UserManager capacity tests
    // =========================================================================

    #[test]
    fn test_manager_with_capacity_zero() {
        let result = UserManager::with_capacity(0);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.contains("capacity must be greater than 0"));
        }
    }

    #[tokio::test]
    async fn test_manager_with_capacity_large() {
        let mgr = UserManager::with_capacity(100000).unwrap();
        assert_eq!(mgr.user_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_add_duplicate_updates() {
        let mgr = UserManager::new();

        let mut user1 = User::new();
        user1.set_id(UserId::from_i32(123));
        user1.set_first_name("Alice".to_string());
        user1.set_deleted(false);

        let mut user2 = User::new();
        user2.set_id(UserId::from_i32(123));
        user2.set_first_name("Alice Updated".to_string());
        user2.set_last_name("Smith".to_string());
        user2.set_deleted(false);

        // First add should return true
        assert!(mgr.add_user(user1.clone()).await);
        assert_eq!(mgr.user_count().await, 1);

        // Second add should return false (update)
        assert!(!mgr.add_user(user2.clone()).await);
        assert_eq!(mgr.user_count().await, 1);

        // Verify updated data
        let fetched = mgr.get_user(UserId::from_i32(123)).await;
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.first_name(), "Alice Updated");
        assert_eq!(fetched.last_name(), "Smith");
    }

    // =========================================================================
    // UserManager retrieval tests
    // =========================================================================

    #[tokio::test]
    async fn test_get_user_not_found() {
        let mgr = UserManager::new();
        let result = mgr.get_user(UserId::from_i32(999)).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_has_user_false() {
        let mgr = UserManager::new();
        assert!(!mgr.has_user(UserId::from_i32(123)).await);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_user() {
        let mgr = UserManager::new();
        let result = mgr.remove_user(UserId::from_i32(999)).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_display_name_not_found() {
        let mgr = UserManager::new();
        let result = mgr.get_display_name(UserId::from_i32(999)).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_is_bot_default_false() {
        let mgr = UserManager::new();
        assert!(!mgr.is_bot(UserId::from_i32(123)).await);
    }

    #[tokio::test]
    async fn test_is_premium_default_false() {
        let mgr = UserManager::new();
        assert!(!mgr.is_premium(UserId::from_i32(123)).await);
    }

    #[tokio::test]
    async fn test_is_deleted_default_true() {
        let mgr = UserManager::new();
        assert!(mgr.is_deleted(UserId::from_i32(123)).await);
    }

    // =========================================================================
    // Network client tests
    // =========================================================================

    #[tokio::test]
    async fn test_set_network_client() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        mgr.set_network_client(client).await;

        // Should not error now
        let result = mgr.fetch_user(UserId::from_i32(123)).await;
        // Will be Ok(None) since client is empty
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_clear_network_client() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        mgr.set_network_client(client).await;
        mgr.clear_network_client().await;

        // Should error again
        let result = mgr.fetch_user(UserId::from_i32(123)).await;
        assert!(matches!(result, Err(NetworkError::NoClient)));
    }

    #[tokio::test]
    async fn test_fetch_user_cache_hit() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to cache
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Cached".to_string());
        user.set_deleted(false);
        mgr.add_user(user.clone()).await;

        // Set network client (won't be called due to cache hit)
        mgr.set_network_client(client).await;

        // Fetch should get from cache
        let result = mgr.fetch_user(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let fetched = result.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().first_name(), "Cached");
    }

    #[tokio::test]
    async fn test_fetch_user_from_network() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to mock network
        let mut network_user = User::new();
        network_user.set_id(UserId::from_i32(123));
        network_user.set_first_name("Network".to_string());
        network_user.set_deleted(false);
        client.add_user(network_user.clone()).await;

        mgr.set_network_client(client).await;

        // Fetch should get from network
        let result = mgr.fetch_user(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let fetched = result.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().first_name(), "Network");

        // Should now be in cache
        assert!(mgr.has_user(UserId::from_i32(123)).await);
    }

    #[tokio::test]
    async fn test_fetch_user_not_found() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        mgr.set_network_client(client).await;

        // Fetch non-existent user
        let result = mgr.fetch_user(UserId::from_i32(999)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_fetch_users_multiple() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add users to mock network
        for i in 1..=3 {
            let mut user = User::new();
            user.set_id(UserId::from_i32(i));
            user.set_first_name(format!("User{}", i));
            user.set_deleted(false);
            client.add_user(user).await;
        }

        mgr.set_network_client(client).await;

        // Fetch multiple users
        let ids = vec![
            UserId::from_i32(1),
            UserId::from_i32(2),
            UserId::from_i32(3),
            UserId::from_i32(999), // Doesn't exist
        ];

        let result = mgr.fetch_users(ids).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.success_count(), 3);
        assert_eq!(result.failure_count(), 1);
        assert!(result.is_complete_success() == false);
    }

    #[tokio::test]
    async fn test_fetch_full_user() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to mock network
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Full".to_string());
        user.set_deleted(false);
        client.add_user(user).await;

        mgr.set_network_client(client).await;

        // Fetch full user
        let result = mgr.fetch_full_user(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let full = result.unwrap();
        assert!(full.is_some());
        let full = full.unwrap();
        assert!(full.user.is_some());
        assert_eq!(full.user.unwrap().first_name(), "Full");
    }

    #[tokio::test]
    async fn test_fetch_me() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to mock network (will be returned as "me")
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Me".to_string());
        user.set_deleted(false);
        client.add_user(user).await;

        mgr.set_network_client(client).await;

        // Fetch me
        let result = mgr.fetch_me().await;
        assert!(result.is_ok());
        let me = result.unwrap();
        assert!(me.is_some());

        // My ID should be set
        assert_eq!(mgr.get_my_id().await, Some(UserId::from_i32(123)));
    }

    #[tokio::test]
    async fn test_fetch_me_no_users() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        mgr.set_network_client(client).await;

        // Fetch me with empty mock
        let result = mgr.fetch_me().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    // =========================================================================
    // My ID tests
    // =========================================================================

    #[tokio::test]
    async fn test_get_me_without_my_id() {
        let mgr = UserManager::new();
        assert!(mgr.get_me().await.is_none());
    }

    #[tokio::test]
    async fn test_get_me_my_id_not_in_cache() {
        let mgr = UserManager::new();
        mgr.set_my_id(UserId::from_i32(999)).await;

        // My ID is set but user not in cache
        assert!(mgr.get_me().await.is_none());
    }

    #[tokio::test]
    async fn test_set_my_id_override() {
        let mgr = UserManager::new();

        mgr.set_my_id(UserId::from_i32(111)).await;
        assert_eq!(mgr.get_my_id().await, Some(UserId::from_i32(111)));

        mgr.set_my_id(UserId::from_i32(222)).await;
        assert_eq!(mgr.get_my_id().await, Some(UserId::from_i32(222)));
    }

    // =========================================================================
    // Concurrent access tests
    // =========================================================================

    #[tokio::test]
    async fn test_concurrent_add_users() {
        let mgr = Arc::new(UserManager::new());
        let mut handles = Vec::new();

        for i in 1..=10 {
            let mgr_clone = Arc::clone(&mgr);
            let handle = tokio::spawn(async move {
                let mut user = User::new();
                user.set_id(UserId::from_i32(i));
                user.set_first_name(format!("User{}", i));
                user.set_deleted(false);
                mgr_clone.add_user(user).await
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(mgr.user_count().await, 10);
    }

    #[tokio::test]
    async fn test_concurrent_get_users() {
        let mgr = Arc::new(UserManager::new());

        // Add a user
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Concurrent".to_string());
        user.set_deleted(false);
        mgr.add_user(user).await;

        let mut handles = Vec::new();

        for _ in 1..=10 {
            let mgr_clone = Arc::clone(&mgr);
            let handle =
                tokio::spawn(async move { mgr_clone.get_user(UserId::from_i32(123)).await });
            handles.push(handle);
        }

        // All should succeed
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_some());
            assert_eq!(result.unwrap().first_name(), "Concurrent");
        }
    }

    #[tokio::test]
    async fn test_concurrent_add_and_get() {
        let mgr = Arc::new(UserManager::new());
        let mut add_handles = Vec::new();
        let mut get_handles = Vec::new();

        // Spawn add tasks
        for i in 1..=5 {
            let mgr_clone = Arc::clone(&mgr);
            let handle = tokio::spawn(async move {
                let mut user = User::new();
                user.set_id(UserId::from_i32(i));
                user.set_first_name(format!("User{}", i));
                user.set_deleted(false);
                mgr_clone.add_user(user).await
            });
            add_handles.push(handle);
        }

        // Spawn get tasks
        for i in 1..=5 {
            let mgr_clone = Arc::clone(&mgr);
            let handle = tokio::spawn(async move { mgr_clone.get_user(UserId::from_i32(i)).await });
            get_handles.push(handle);
        }

        // Wait for all adds
        for handle in add_handles {
            handle.await.unwrap();
        }

        // Wait for all gets
        for handle in get_handles {
            handle.await.unwrap();
        }

        assert!(mgr.user_count().await <= 5);
    }

    // =========================================================================
    // LRU cache edge cases
    // =========================================================================

    #[tokio::test]
    async fn test_lru_capacity_1() {
        let mgr = UserManager::with_capacity(1).unwrap();

        let mut user1 = User::new();
        user1.set_id(UserId::from_i32(1));
        user1.set_deleted(false);
        mgr.add_user(user1).await;

        let mut user2 = User::new();
        user2.set_id(UserId::from_i32(2));
        user2.set_deleted(false);
        mgr.add_user(user2).await;

        assert_eq!(mgr.user_count().await, 1);
        assert!(!mgr.has_user(UserId::from_i32(1)).await);
        assert!(mgr.has_user(UserId::from_i32(2)).await);
    }

    #[tokio::test]
    async fn test_lru_large_capacity() {
        let mgr = UserManager::with_capacity(1000).unwrap();

        for i in 1..=100 {
            let mut user = User::new();
            user.set_id(UserId::from_i32(i));
            user.set_deleted(false);
            mgr.add_user(user).await;
        }

        assert_eq!(mgr.user_count().await, 100);
    }

    #[tokio::test]
    async fn test_lru_get_refreshes() {
        let mgr = UserManager::with_capacity(3).unwrap();

        // Add 3 users
        for i in 1..=3 {
            let mut user = User::new();
            user.set_id(UserId::from_i32(i));
            user.set_deleted(false);
            mgr.add_user(user).await;
        }

        // Access user 1 to refresh it
        mgr.get_user(UserId::from_i32(1)).await;

        // Add 4th and 5th users - user 1 should still be there
        let mut user4 = User::new();
        user4.set_id(UserId::from_i32(4));
        user4.set_deleted(false);
        mgr.add_user(user4).await;

        let mut user5 = User::new();
        user5.set_id(UserId::from_i32(5));
        user5.set_deleted(false);
        mgr.add_user(user5).await;

        // User 1 should still exist (was accessed), user 2 evicted
        assert!(mgr.has_user(UserId::from_i32(1)).await);
        assert!(!mgr.has_user(UserId::from_i32(2)).await);
        assert!(mgr.has_user(UserId::from_i32(4)).await);
        assert!(mgr.has_user(UserId::from_i32(5)).await);
    }

    // =========================================================================
    // Display tests
    // =========================================================================

    #[test]
    fn test_user_manager_display() {
        let mgr = UserManager::new();
        assert_eq!(format!("{}", mgr), "UserManager");
    }

    #[test]
    fn test_version_constants() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
        assert_eq!(CRATE_NAME, "rustgram_user_manager");
    }

    // =========================================================================
    // Network Integration Tests
    // =========================================================================

    #[tokio::test]
    async fn test_set_real_network_client() {
        let mgr = UserManager::new();
        let dispatcher = Arc::new(rustgram_net::NetQueryDispatcher::new());
        let client = RealNetworkClient::new(dispatcher);

        // Should not panic
        mgr.set_real_network_client(client).await;
    }

    #[tokio::test]
    async fn test_fetch_user_no_network_client() {
        let mgr = UserManager::new();
        let result = mgr.fetch_user(UserId::from_i32(123)).await;

        assert!(matches!(result, Err(NetworkError::NoClient)));
    }

    #[tokio::test]
    async fn test_fetch_user_from_ttl_cache() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to TTL cache directly
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Cached".to_string());
        user.set_deleted(false);
        mgr.user_cache.set_user(UserId::from_i32(123), user.clone());

        // Set network client (won't be called due to cache hit)
        mgr.set_network_client(client).await;

        // Fetch should get from TTL cache
        let result = mgr.fetch_user(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let fetched = result.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().first_name(), "Cached");
    }

    #[tokio::test]
    async fn test_fetch_full_user_no_network_client() {
        let mgr = UserManager::new();
        let result = mgr.fetch_full_user(UserId::from_i32(123)).await;

        assert!(matches!(result, Err(NetworkError::NoClient)));
    }

    #[tokio::test]
    async fn test_invalidate_user() {
        let mgr = UserManager::new();

        // Add user to both caches
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Test".to_string());
        user.set_deleted(false);
        mgr.add_user(user.clone()).await;
        mgr.user_cache.set_user(UserId::from_i32(123), user);

        assert!(mgr.has_user(UserId::from_i32(123)).await);
        assert!(mgr.user_cache.has_user(UserId::from_i32(123)));

        // Invalidate
        mgr.invalidate_user(UserId::from_i32(123)).await;

        assert!(!mgr.has_user(UserId::from_i32(123)).await);
        assert!(!mgr.user_cache.has_user(UserId::from_i32(123)));
    }

    #[tokio::test]
    async fn test_on_user_update() {
        let mgr = UserManager::new();

        // Add user to TTL cache
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Before".to_string());
        user.set_deleted(false);
        mgr.user_cache.set_user(UserId::from_i32(123), user);

        assert!(mgr.user_cache.has_user(UserId::from_i32(123)));

        // Trigger update
        mgr.on_user_update(UserId::from_i32(123));

        // Should be invalidated
        assert!(!mgr.user_cache.has_user(UserId::from_i32(123)));
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let mgr = UserManager::new();

        let stats = mgr.cache_stats();
        assert_eq!(stats.hits(), 0);
        assert_eq!(stats.misses(), 0);
        assert_eq!(stats.total_requests(), 0);
    }

    #[tokio::test]
    async fn test_user_cache_access() {
        let mgr = UserManager::new();
        let cache = mgr.user_cache();

        assert_eq!(cache.user_count(), 0);
        assert_eq!(cache.full_user_count(), 0);
    }

    #[tokio::test]
    async fn test_fetch_user_from_lru_cache() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to LRU cache
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("LRU".to_string());
        user.set_deleted(false);
        mgr.add_user(user).await;

        // Set network client (won't be called due to LRU cache hit)
        mgr.set_network_client(client).await;

        // Fetch should get from LRU cache
        let result = mgr.fetch_user(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let fetched = result.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().first_name(), "LRU");
    }

    #[tokio::test]
    async fn test_fetch_user_caches_in_both() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to mock network
        let mut network_user = User::new();
        network_user.set_id(UserId::from_i32(123));
        network_user.set_first_name("Network".to_string());
        network_user.set_deleted(false);
        client.add_user(network_user.clone()).await;

        mgr.set_network_client(client).await;

        // Fetch should get from network and cache in both
        let result = mgr.fetch_user(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let fetched = result.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().first_name(), "Network");

        // Should now be in both caches
        assert!(mgr.has_user(UserId::from_i32(123)).await);
        assert!(mgr.user_cache.has_user(UserId::from_i32(123)));
    }

    #[tokio::test]
    async fn test_cache_hit_rate_tracking() {
        let mgr = UserManager::new();

        // Add user to TTL cache
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Test".to_string());
        user.set_deleted(false);
        mgr.user_cache.set_user(UserId::from_i32(123), user);

        // Fetch through fetch_user (should hit TTL cache)
        let _ = mgr.fetch_user(UserId::from_i32(123)).await;

        let stats = mgr.cache_stats();
        assert!(stats.hits() > 0);
    }

    // =========================================================================
    // User Profile Display Tests (Subtask 1gk.1)
    // =========================================================================

    #[tokio::test]
    async fn test_get_display_name_full() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        // Add user to mock network
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Alice".to_string());
        user.set_last_name("Smith".to_string());
        user.set_deleted(false);
        client.add_user(user).await;

        mgr.set_network_client(client).await;

        // Fetch display name
        let result = mgr.get_display_name_full(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let name = result.unwrap();
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "Alice Smith");
    }

    #[tokio::test]
    async fn test_get_display_name_full_not_found() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        mgr.set_network_client(client).await;

        // Fetch non-existent user
        let result = mgr.get_display_name_full(UserId::from_i32(999)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_status_online() {
        let mgr = UserManager::new();

        // Create full user with online status
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Online".to_string());
        user.set_deleted(false);

        let mut full_user = UserFull::new();
        full_user.user = Some(user);
        full_user.status = UserStatus::Online { expires: i32::MAX };

        // Add to cache
        mgr.user_cache
            .set_full_user(UserId::from_i32(123), full_user);

        // Get status
        let result = mgr.get_status(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap(), "Online");
    }

    #[tokio::test]
    async fn test_get_status_offline() {
        let mgr = UserManager::new();

        // Create full user with offline status
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Offline".to_string());
        user.set_deleted(false);

        let mut full_user = UserFull::new();
        full_user.user = Some(user);
        full_user.status = UserStatus::Offline {
            was_online: 1704100400,
        };

        // Add to cache
        mgr.user_cache
            .set_full_user(UserId::from_i32(123), full_user);

        // Get status
        let result = mgr.get_status(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap(), "Offline");
    }

    #[tokio::test]
    async fn test_get_status_recently() {
        let mgr = UserManager::new();

        // Create full user with recently status
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Recently".to_string());
        user.set_deleted(false);

        let mut full_user = UserFull::new();
        full_user.user = Some(user);
        full_user.status = UserStatus::Recently {
            by_my_privacy_settings: false,
        };

        // Add to cache
        mgr.user_cache
            .set_full_user(UserId::from_i32(123), full_user);

        // Get status
        let result = mgr.get_status(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap(), "Recently");
    }

    #[tokio::test]
    async fn test_get_status_not_found() {
        let mgr = UserManager::new();
        let client = MockNetworkClient::new();

        mgr.set_network_client(client).await;

        // Fetch status for non-existent user
        let result = mgr.get_status(UserId::from_i32(999)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_bio_some() {
        let mgr = UserManager::new();

        // Create full user with bio
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Bio".to_string());
        user.set_deleted(false);

        let mut full_user = UserFull::new();
        full_user.user = Some(user);
        full_user.about = Some("This is my bio".to_string());

        // Add to cache
        mgr.user_cache
            .set_full_user(UserId::from_i32(123), full_user);

        // Get bio
        let result = mgr.get_bio(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let bio = result.unwrap();
        assert!(bio.is_some());
        assert_eq!(bio.unwrap(), "This is my bio");
    }

    #[tokio::test]
    async fn test_get_bio_none() {
        let mgr = UserManager::new();

        // Create full user without bio
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("NoBio".to_string());
        user.set_deleted(false);

        let mut full_user = UserFull::new();
        full_user.user = Some(user);
        full_user.about = None;

        // Add to cache
        mgr.user_cache
            .set_full_user(UserId::from_i32(123), full_user);

        // Get bio
        let result = mgr.get_bio(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let bio = result.unwrap();
        assert!(bio.is_none());
    }

    #[tokio::test]
    async fn test_get_profile_photo_some() {
        let mgr = UserManager::new();

        // Create full user with profile photo
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Photo".to_string());
        user.set_deleted(false);

        let mut full_user = UserFull::new();
        full_user.user = Some(user);

        let mut profile_photo = UserProfilePhoto::new();
        profile_photo.photo_id = 12345;
        full_user.profile_photo = Some(profile_photo);

        // Add to cache
        mgr.user_cache
            .set_full_user(UserId::from_i32(123), full_user);

        // Get profile photo
        let result = mgr.get_profile_photo(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let photo = result.unwrap();
        assert!(photo.is_some());
        assert_eq!(photo.unwrap().photo_id(), 12345);
    }

    #[tokio::test]
    async fn test_get_profile_photo_none() {
        let mgr = UserManager::new();

        // Create full user without profile photo
        let mut user = User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("NoPhoto".to_string());
        user.set_deleted(false);

        let mut full_user = UserFull::new();
        full_user.user = Some(user);
        full_user.profile_photo = None;

        // Add to cache
        mgr.user_cache
            .set_full_user(UserId::from_i32(123), full_user);

        // Get profile photo
        let result = mgr.get_profile_photo(UserId::from_i32(123)).await;
        assert!(result.is_ok());
        let photo = result.unwrap();
        assert!(photo.is_none());
    }

    #[tokio::test]
    async fn test_get_status_all_types() {
        let mgr = UserManager::new();

        let statuses = vec![
            (UserStatus::Empty, "Empty"),
            (UserStatus::Online { expires: 1000 }, "Online"),
            (UserStatus::Offline { was_online: 500 }, "Offline"),
            (
                UserStatus::Recently {
                    by_my_privacy_settings: false,
                },
                "Recently",
            ),
            (
                UserStatus::LastWeek {
                    by_my_privacy_settings: true,
                },
                "LastWeek",
            ),
            (
                UserStatus::LastMonth {
                    by_my_privacy_settings: false,
                },
                "LastMonth",
            ),
        ];

        for (i, (status, expected_name)) in statuses.into_iter().enumerate() {
            let user_id = UserId::from_i32(100 + i as i32);

            // Create full user with status
            let mut user = User::new();
            user.set_id(user_id);
            user.set_first_name(format!("User{}", i));
            user.set_deleted(false);

            let mut full_user = UserFull::new();
            full_user.user = Some(user);
            full_user.status = status;

            // Add to cache
            mgr.user_cache.set_full_user(user_id, full_user);

            // Get status
            let result = mgr.get_status(user_id).await;
            assert!(result.is_ok());
            let status_str = result.unwrap();
            assert!(status_str.is_some());
            assert_eq!(status_str.unwrap(), expected_name);
        }
    }
}
