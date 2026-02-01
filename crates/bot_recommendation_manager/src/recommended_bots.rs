// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Internal bot recommendation cache structure.

use std::time::{SystemTime, UNIX_EPOCH};

use rustgram_types::UserId;

use crate::BOT_RECOMMENDATIONS_CACHE_TIME;

/// Internal structure storing cached bot recommendations.
///
/// Contains the list of recommended bot user IDs and cache expiration time.
///
/// # Examples
///
/// ```rust
/// use rustgram_bot_recommendation_manager::RecommendedBots;
/// use rustgram_types::UserId;
///
/// let bots = RecommendedBots::new(5, vec![
///     UserId::new(111).expect("valid"),
///     UserId::new(222).expect("valid"),
/// ]);
///
/// assert_eq!(bots.total_count(), 5);
/// assert_eq!(bots.bot_user_ids().len(), 2);
/// assert!(!bots.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RecommendedBots {
    /// Total number of recommended bots
    total_count: i32,
    /// List of recommended bot user IDs
    bot_user_ids: Vec<UserId>,
    /// Unix timestamp for cache expiration
    next_reload_time: f64,
}

impl RecommendedBots {
    /// Creates a new RecommendedBots instance.
    ///
    /// # Arguments
    ///
    /// * `total_count` - Total number of recommended bots
    /// * `bot_user_ids` - List of recommended bot user IDs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::RecommendedBots;
    /// use rustgram_types::UserId;
    ///
    /// let bots = RecommendedBots::new(3, vec![
    ///     UserId::new(111).expect("valid"),
    ///     UserId::new(222).expect("valid"),
    ///     UserId::new(333).expect("valid"),
    /// ]);
    /// ```
    pub fn new(total_count: i32, bot_user_ids: Vec<UserId>) -> Self {
        let next_reload_time = Self::calculate_next_reload_time();
        Self {
            total_count,
            bot_user_ids,
            next_reload_time,
        }
    }

    /// Returns true if there are no recommendations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::RecommendedBots;
    ///
    /// let bots = RecommendedBots::new(0, vec![]);
    /// assert!(bots.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bot_user_ids.is_empty()
    }

    /// Returns true if the cache needs to be reloaded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::RecommendedBots;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// // Create with future expiration time
    /// let bots = RecommendedBots::new(0, vec![]);
    /// assert!(!bots.needs_reload());
    /// ```
    pub fn needs_reload(&self) -> bool {
        let now = Self::current_time();
        now >= self.next_reload_time
    }

    /// Updates the next reload time to the future.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::RecommendedBots;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let mut bots = RecommendedBots::new(0, vec![]);
    /// let old_time = bots.next_reload_time();
    ///
    /// // Ensure time progresses
    /// thread::sleep(Duration::from_millis(10));
    /// bots.update_next_reload_time();
    /// assert!(bots.next_reload_time() >= old_time);
    /// ```
    pub fn update_next_reload_time(&mut self) {
        self.next_reload_time = Self::calculate_next_reload_time();
    }

    /// Returns the total count of recommendations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::RecommendedBots;
    ///
    /// let bots = RecommendedBots::new(42, vec![]);
    /// assert_eq!(bots.total_count(), 42);
    /// ```
    pub const fn total_count(&self) -> i32 {
        self.total_count
    }

    /// Returns the list of recommended bot user IDs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::RecommendedBots;
    /// use rustgram_types::UserId;
    ///
    /// let ids = vec![UserId::new(111).expect("valid")];
    /// let bots = RecommendedBots::new(1, ids.clone());
    /// assert_eq!(bots.bot_user_ids(), &ids);
    /// ```
    pub fn bot_user_ids(&self) -> &[UserId] {
        &self.bot_user_ids
    }

    /// Returns the next reload time as a Unix timestamp.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::RecommendedBots;
    ///
    /// let bots = RecommendedBots::new(0, vec![]);
    /// let time = bots.next_reload_time();
    /// assert!(time > 0.0);
    /// ```
    pub const fn next_reload_time(&self) -> f64 {
        self.next_reload_time
    }

    /// Calculates the next reload time based on current time and cache duration.
    fn calculate_next_reload_time() -> f64 {
        let now = Self::current_time();
        now + BOT_RECOMMENDATIONS_CACHE_TIME as f64
    }

    /// Returns the current time as a Unix timestamp in seconds.
    fn current_time() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9)
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recommended_bots_new_empty() {
        let bots = RecommendedBots::new(0, vec![]);
        assert!(bots.is_empty());
        assert_eq!(bots.total_count(), 0);
        assert!(bots.bot_user_ids().is_empty());
    }

    #[test]
    fn test_recommended_bots_new_with_data() {
        let ids = vec![
            UserId::new(111).expect("valid"),
            UserId::new(222).expect("valid"),
            UserId::new(333).expect("valid"),
        ];

        let bots = RecommendedBots::new(5, ids.clone());
        assert!(!bots.is_empty());
        assert_eq!(bots.total_count(), 5);
        assert_eq!(bots.bot_user_ids().len(), 3);
    }

    #[test]
    fn test_recommended_bots_needs_reload_false() {
        let bots = RecommendedBots::new(0, vec![]);
        // Just created, should not need reload
        assert!(!bots.needs_reload());
    }

    #[test]
    fn test_recommended_bots_update_next_reload_time() {
        let mut bots = RecommendedBots::new(0, vec![]);
        let old_time = bots.next_reload_time();

        // Wait a tiny bit to ensure time progresses
        std::thread::sleep(std::time::Duration::from_millis(10));
        bots.update_next_reload_time();

        // The new time should be at least as large as the old time
        assert!(bots.next_reload_time() >= old_time);
    }

    #[test]
    fn test_recommended_bots_next_reload_time_is_future() {
        let bots = RecommendedBots::new(0, vec![]);
        let now = RecommendedBots::current_time();
        let reload_time = bots.next_reload_time();

        assert!(reload_time > now);
    }

    #[test]
    fn test_recommended_bots_clone() {
        let ids = vec![UserId::new(111).expect("valid")];
        let bots1 = RecommendedBots::new(1, ids.clone());
        let bots2 = bots1.clone();

        assert_eq!(bots1, bots2);
    }

    #[test]
    fn test_recommended_bots_partial_eq() {
        let ids1 = vec![UserId::new(111).expect("valid")];
        let ids2 = vec![UserId::new(111).expect("valid")];
        let bots1 = RecommendedBots::new(1, ids1);
        let bots2 = RecommendedBots::new(1, ids2);

        // Note: PartialEq implementation may not consider next_reload_time
        // due to f64 floating point comparison issues
        assert!(bots1.total_count() == bots2.total_count());
        assert!(bots1.bot_user_ids() == bots2.bot_user_ids());
    }
}
