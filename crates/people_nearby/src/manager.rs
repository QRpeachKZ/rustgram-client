// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Manager for people nearby functionality.
//!
//! This module implements the PeopleNearbyManager which handles searching
//! for nearby users and managing location visibility settings.

use crate::{ChatsNearby, PeopleNearbyError, Result, UsersNearbyUpdate};
use rustgram_venue::Location;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, info};

/// Default timeout for nearby results (60 seconds).
///
/// After this time, an `updateUsersNearby` is sent to refresh results.
pub const NEARBY_TIMEOUT_SECS: u64 = 60;

/// Default visibility expire time for location sharing.
///
/// Location visibility expires after 24 hours by default.
pub const DEFAULT_VISIBILITY_EXPIRE_HOURS: u64 = 24;

/// Binlog key for storing location visibility expiration.
pub const LOCATION_VISIBILITY_KEY: &str = "location_visibility_expire_date";

/// Binlog key for storing pending location visibility expiration.
pub const PENDING_LOCATION_VISIBILITY_KEY: &str = "pending_location_visibility_expire_date";

/// Manager for people nearby functionality.
///
/// Handles searching for nearby users and managing location visibility settings.
///
/// Corresponds to TDLib class `td::PeopleNearbyManager` from `PeopleNearbyManager.h/cpp`.
///
/// # Examples
///
/// ```rust
/// use rustgram_people_nearby::PeopleNearbyManager;
///
/// let mut manager = PeopleNearbyManager::new(false);
///
/// // Search for nearby users
/// let location = rustgram_venue::Location::from_components(55.7558, 37.6173, 10.0, 0);
/// let results = manager.search_nearby(&location, None, None);
///
/// assert!(results.is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct PeopleNearbyManager {
    /// Whether this is a bot account (bots can't use people nearby)
    is_bot: bool,
    /// Last search location
    last_location: Option<Location>,
    /// Last search timestamp
    last_search_time: Option<i64>,
    /// Location visibility expiration time (unix timestamp)
    visibility_expire_time: Option<i64>,
}

impl PeopleNearbyManager {
    /// Creates a new PeopleNearbyManager.
    ///
    /// # Arguments
    ///
    /// * `is_bot` - Whether this is a bot account
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    ///
    /// let manager = PeopleNearbyManager::new(false);
    /// assert!(!manager.is_bot());
    /// ```
    pub fn new(is_bot: bool) -> Self {
        // TDLib reference: PeopleNearbyManager.cpp:16-20
        // For non-bot accounts, erase location visibility keys on initialization
        Self {
            is_bot,
            last_location: None,
            last_search_time: None,
            visibility_expire_time: None,
        }
    }

    /// Returns whether this is a bot account.
    ///
    /// Bot accounts cannot use the people nearby feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    ///
    /// let manager = PeopleNearbyManager::new(true);
    /// assert!(manager.is_bot());
    /// ```
    pub fn is_bot(&self) -> bool {
        self.is_bot
    }

    /// Searches for nearby chats and users.
    ///
    /// # Arguments
    ///
    /// * `location` - The location to search from
    /// * `background` - If true, search in background (optional)
    /// * `offset` - Pagination offset for continued search (optional)
    ///
    /// # Returns
    ///
    /// Returns the nearby chats or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - This is a bot account
    /// - The location is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    /// use rustgram_venue::Location;
    ///
    /// let manager = PeopleNearbyManager::new(false);
    /// let location = Location::from_components(55.7558, 37.6173, 10.0, 0);
    ///
    /// // This would normally call the RPC
    /// // let results = manager.search_nearby(&location, Some(false), None);
    /// ```
    pub fn search_nearby(
        &mut self,
        location: &Location,
        _background: Option<bool>,
        _offset: Option<&str>,
    ) -> Result<ChatsNearby> {
        if self.is_bot {
            return Err(PeopleNearbyError::UpdateWithoutSearch);
        }

        if location.is_empty() {
            return Err(PeopleNearbyError::LocationRequired);
        }

        // Store search state for potential follow-up requests
        self.last_location = Some(location.clone());
        self.last_search_time = Some(now_unix());

        debug!(
            "Searching nearby users at location: ({}, {})",
            location.latitude(),
            location.longitude()
        );

        // In a real implementation, this would make an RPC call
        // to the server: contacts.getLocated or similar
        // For now, return empty results as a placeholder
        Ok(ChatsNearby::empty())
    }

    /// Sets location visibility expiration.
    ///
    /// # Arguments
    ///
    /// * `expire_date` - Unix timestamp when visibility should expire
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// let now = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .unwrap()
    ///     .as_secs() as i64;
    ///
    /// manager.set_visibility_expiration(Some(now + 3600));
    /// ```
    pub fn set_visibility_expiration(&mut self, expire_date: Option<i64>) {
        self.visibility_expire_time = expire_date;

        if let Some(expire) = expire_date {
            info!("Location visibility expires at: {}", expire);
        } else {
            info!("Location visibility cleared");
        }
    }

    /// Returns the location visibility expiration time.
    ///
    /// Returns `None` if visibility is not set.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    ///
    /// let manager = PeopleNearbyManager::new(false);
    /// assert!(manager.visibility_expiration().is_none());
    /// ```
    pub fn visibility_expiration(&self) -> Option<i64> {
        self.visibility_expire_time
    }

    /// Checks if location visibility is currently active.
    ///
    /// Returns `true` if visibility is set and not expired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// let now = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .unwrap()
    ///     .as_secs() as i64;
    ///
    /// manager.set_visibility_expiration(Some(now + 3600));
    /// assert!(manager.is_visibility_active());
    /// ```
    pub fn is_visibility_active(&self) -> bool {
        match self.visibility_expire_time {
            Some(expire_time) => {
                let now = now_unix();
                now < expire_time
            }
            None => false,
        }
    }

    /// Returns the time since the last search.
    ///
    /// Returns `None` if no search has been performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    /// use rustgram_venue::Location;
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// let location = Location::from_components(55.7558, 37.6173, 10.0, 0);
    /// let _ = manager.search_nearby(&location, None, None);
    ///
    /// let elapsed = manager.time_since_last_search();
    /// assert!(elapsed.is_some());
    /// ```
    pub fn time_since_last_search(&self) -> Option<Duration> {
        self.last_search_time.map(|last| {
            let now = now_unix();
            let secs = if now > last { now - last } else { 0 };
            Duration::from_secs(secs as u64)
        })
    }

    /// Checks if an update should be sent (60 seconds after search).
    ///
    /// Returns `true` if enough time has passed since the last search.
    ///
    /// TDLib reference: Update sent 60 seconds after successful search
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    /// use rustgram_venue::Location;
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// let location = Location::from_components(55.7558, 37.6173, 10.0, 0);
    /// let _ = manager.search_nearby(&location, None, None);
    ///
    /// // Just searched, so no update needed yet
    /// assert!(!manager.should_send_update());
    /// ```
    pub fn should_send_update(&self) -> bool {
        match self.time_since_last_search() {
            Some(elapsed) => elapsed.as_secs() >= NEARBY_TIMEOUT_SECS,
            None => false,
        }
    }

    /// Creates an update for nearby users.
    ///
    /// Should be called when `should_send_update()` returns true.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    ///
    /// let manager = PeopleNearbyManager::new(false);
    /// let update = manager.create_update();
    ///
    /// assert_eq!(update.constructor_id(), 0xb090efb9);
    /// ```
    pub fn create_update(&self) -> UsersNearbyUpdate {
        UsersNearbyUpdate::new()
    }

    /// Clears the last search state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    /// use rustgram_venue::Location;
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// let location = Location::from_components(55.7558, 37.6173, 10.0, 0);
    /// let _ = manager.search_nearby(&location, None, None);
    ///
    /// manager.clear_search_state();
    /// assert!(manager.last_search_location().is_none());
    /// ```
    pub fn clear_search_state(&mut self) {
        self.last_location = None;
        self.last_search_time = None;
    }

    /// Returns the last search location.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    /// use rustgram_venue::Location;
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// let location = Location::from_components(55.7558, 37.6173, 10.0, 0);
    /// let _ = manager.search_nearby(&location, None, None);
    ///
    /// assert!(manager.last_search_location().is_some());
    /// ```
    pub fn last_search_location(&self) -> Option<&Location> {
        self.last_location.as_ref()
    }

    /// Enables location visibility with a default expiration time.
    ///
    /// Visibility expires after 24 hours by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// manager.enable_visibility();
    ///
    /// assert!(manager.is_visibility_active());
    /// ```
    pub fn enable_visibility(&mut self) {
        let now = now_unix();
        let expire = now + (DEFAULT_VISIBILITY_EXPIRE_HOURS as i64 * 3600);
        self.set_visibility_expiration(Some(expire));
    }

    /// Disables location visibility.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_people_nearby::PeopleNearbyManager;
    ///
    /// let mut manager = PeopleNearbyManager::new(false);
    /// manager.enable_visibility();
    /// assert!(manager.is_visibility_active());
    ///
    /// manager.disable_visibility();
    /// assert!(!manager.is_visibility_active());
    /// ```
    pub fn disable_visibility(&mut self) {
        self.set_visibility_expiration(None);
    }
}

/// Returns the current unix timestamp in seconds.
fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_new() {
        let manager = PeopleNearbyManager::new(false);
        assert!(!manager.is_bot());
        assert!(manager.last_search_location().is_none());
        assert!(manager.visibility_expiration().is_none());
    }

    #[test]
    fn test_manager_new_bot() {
        let manager = PeopleNearbyManager::new(true);
        assert!(manager.is_bot());
    }

    #[test]
    fn test_search_nearby_with_empty_location() {
        let mut manager = PeopleNearbyManager::new(false);
        let empty = Location::empty();

        let result = manager.search_nearby(&empty, None, None);

        assert!(matches!(result, Err(PeopleNearbyError::LocationRequired)));
    }

    #[test]
    fn test_search_nearby_as_bot() {
        let mut manager = PeopleNearbyManager::new(true);
        let location = Location::from_components(55.7558, 37.6173, 10.0, 0);

        let result = manager.search_nearby(&location, None, None);

        assert!(matches!(
            result,
            Err(PeopleNearbyError::UpdateWithoutSearch)
        ));
    }

    #[test]
    fn test_search_nearby_valid_location() {
        let mut manager = PeopleNearbyManager::new(false);
        let location = Location::from_components(55.7558, 37.6173, 10.0, 0);

        let result = manager.search_nearby(&location, None, None);

        assert!(result.is_ok());
        assert!(manager.last_search_location().is_some());
        assert!(manager.time_since_last_search().is_some());
    }

    #[test]
    fn test_set_visibility_expiration() {
        let mut manager = PeopleNearbyManager::new(false);
        let expire_time = 1735795200i64;

        manager.set_visibility_expiration(Some(expire_time));

        assert_eq!(manager.visibility_expiration(), Some(expire_time));
    }

    #[test]
    fn test_clear_visibility_expiration() {
        let mut manager = PeopleNearbyManager::new(false);
        manager.set_visibility_expiration(Some(12345));

        manager.set_visibility_expiration(None);

        assert!(manager.visibility_expiration().is_none());
    }

    #[test]
    fn test_is_visibility_active() {
        let mut manager = PeopleNearbyManager::new(false);
        let now = now_unix();

        // No expiration set
        assert!(!manager.is_visibility_active());

        // Future expiration
        manager.set_visibility_expiration(Some(now + 3600));
        assert!(manager.is_visibility_active());

        // Past expiration
        manager.set_visibility_expiration(Some(now - 3600));
        assert!(!manager.is_visibility_active());
    }

    #[test]
    fn test_time_since_last_search() {
        let mut manager = PeopleNearbyManager::new(false);
        let location = Location::from_components(55.7558, 37.6173, 10.0, 0);

        assert!(manager.time_since_last_search().is_none());

        let _ = manager.search_nearby(&location, None, None);

        let elapsed = manager.time_since_last_search();
        assert!(elapsed.is_some());
        assert!(elapsed.unwrap().as_secs() < 1);
    }

    #[test]
    fn test_should_send_update_immediate() {
        let mut manager = PeopleNearbyManager::new(false);
        let location = Location::from_components(55.7558, 37.6173, 10.0, 0);

        let _ = manager.search_nearby(&location, None, None);

        // Just searched, so no update needed
        assert!(!manager.should_send_update());
    }

    #[test]
    fn test_create_update() {
        let manager = PeopleNearbyManager::new(false);
        let update = manager.create_update();

        assert_eq!(update.constructor_id(), 0xb090efb9);
    }

    #[test]
    fn test_clear_search_state() {
        let mut manager = PeopleNearbyManager::new(false);
        let location = Location::from_components(55.7558, 37.6173, 10.0, 0);

        let _ = manager.search_nearby(&location, None, None);
        assert!(manager.last_search_location().is_some());

        manager.clear_search_state();
        assert!(manager.last_search_location().is_none());
        assert!(manager.time_since_last_search().is_none());
    }

    #[test]
    fn test_enable_visibility() {
        let mut manager = PeopleNearbyManager::new(false);

        manager.enable_visibility();

        assert!(manager.is_visibility_active());

        let expire = manager.visibility_expiration().unwrap();
        let now = now_unix();
        let expected_expire = now + (DEFAULT_VISIBILITY_EXPIRE_HOURS as i64 * 3600);

        // Allow some tolerance for execution time
        assert!((expire - expected_expire).abs() < 5);
    }

    #[test]
    fn test_disable_visibility() {
        let mut manager = PeopleNearbyManager::new(false);

        manager.enable_visibility();
        assert!(manager.is_visibility_active());

        manager.disable_visibility();
        assert!(!manager.is_visibility_active());
    }

    #[test]
    fn test_last_search_location() {
        let mut manager = PeopleNearbyManager::new(false);
        let location = Location::from_components(55.7558, 37.6173, 10.0, 0);

        assert!(manager.last_search_location().is_none());

        let _ = manager.search_nearby(&location, None, None);

        let last = manager.last_search_location().unwrap();
        assert_eq!(last.latitude(), 55.7558);
        assert_eq!(last.longitude(), 37.6173);
    }

    #[test]
    fn test_visibility_expiration_none() {
        let manager = PeopleNearbyManager::new(false);
        assert!(manager.visibility_expiration().is_none());
    }

    #[test]
    fn test_now_unix() {
        let now = now_unix();
        assert!(now > 0);

        let now2 = now_unix();
        assert!(now2 >= now);
        assert!(now2 - now < 2); // Should be very close
    }
}
