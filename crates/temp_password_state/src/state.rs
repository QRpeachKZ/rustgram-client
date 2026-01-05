// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! TempPasswordState struct and implementation.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Temporary password state for Telegram 2FA.
///
/// Represents a time-limited password used for secure operations like
/// payment confirmation without exposing the main account password.
///
/// # Fields
///
/// - `has_temp_password`: Flag indicating if a temp password is currently active
/// - `temp_password`: The temporary password string (from server)
/// - `valid_until`: Unix timestamp when password expires
///
/// # Example
///
/// ```
/// use rustgram_temp_password_state::TempPasswordState;
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let future = SystemTime::now()
///     .duration_since(UNIX_EPOCH)
///     .map(|d| d.as_secs() as i32 + 3600)
///     .unwrap_or(0);
///
/// let state = TempPasswordState::new("secret123", future);
/// assert!(state.has_temp_password());
/// assert_eq!(state.temp_password(), "secret123");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TempPasswordState {
    /// Flag indicating if temp password is present
    has_temp_password: bool,

    /// The temporary password string
    temp_password: String,

    /// Unix timestamp when password expires (seconds since epoch)
    valid_until: i32,
}

impl TempPasswordState {
    /// Creates a new temporary password state.
    ///
    /// # Arguments
    ///
    /// * `temp_password` - The temporary password from server
    /// * `valid_until` - Unix timestamp when password expires
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let future = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 + 3600)
    ///     .unwrap_or(0);
    ///
    /// let state = TempPasswordState::new("secret", future);
    /// assert!(state.has_temp_password());
    /// ```
    pub fn new(temp_password: impl Into<String>, valid_until: i32) -> Self {
        Self {
            has_temp_password: true,
            temp_password: temp_password.into(),
            valid_until,
        }
    }

    /// Returns whether a temporary password is currently active.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    ///
    /// let state = TempPasswordState::default();
    /// assert!(!state.has_temp_password());
    /// ```
    #[must_use]
    pub const fn has_temp_password(&self) -> bool {
        self.has_temp_password
    }

    /// Returns the temporary password string.
    ///
    /// Returns empty string if no password is set.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let future = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 + 3600)
    ///     .unwrap_or(0);
    ///
    /// let state = TempPasswordState::new("secret123", future);
    /// assert_eq!(state.temp_password(), "secret123");
    /// ```
    #[must_use]
    pub fn temp_password(&self) -> &str {
        &self.temp_password
    }

    /// Returns the Unix timestamp when password expires.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let future = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 + 3600)
    ///     .unwrap_or(0);
    ///
    /// let state = TempPasswordState::new("secret", future);
    /// assert_eq!(state.valid_until(), future);
    /// ```
    #[must_use]
    pub const fn valid_until(&self) -> i32 {
        self.valid_until
    }

    /// Checks if the temporary password is still valid at current time.
    ///
    /// Returns `false` if no password is set or if expired.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// // Future timestamp = valid
    /// let future = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 + 86400)
    ///     .unwrap_or(0);
    /// let state = TempPasswordState::new("secret", future);
    /// assert!(state.is_valid());
    ///
    /// // Past timestamp = expired
    /// let past = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 - 86400)
    ///     .unwrap_or(0);
    /// let state = TempPasswordState::new("secret", past);
    /// assert!(!state.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        if !self.has_temp_password {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i32;

        self.valid_until > now
    }

    /// Returns the remaining time until password expires.
    ///
    /// Returns `None` if no password is set or expired.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let future = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 + 3600)
    ///     .unwrap_or(0);
    /// let state = TempPasswordState::new("secret", future);
    ///
    /// let remaining = state.remaining_time();
    /// assert!(remaining.is_some());
    /// assert!(remaining.map_or(0, |d| d.as_secs()) <= 3600);
    /// ```
    #[must_use]
    pub fn remaining_time(&self) -> Option<Duration> {
        if !self.is_valid() {
            return None;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i32;

        let remaining_secs = (self.valid_until - now).max(0) as u64;
        Some(Duration::from_secs(remaining_secs))
    }

    /// Converts to TDLib API object format.
    ///
    /// Returns `(has_password, valid_for)` tuple matching td_api::temporaryPasswordState.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let future = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 + 3600)
    ///     .unwrap_or(0);
    /// let state = TempPasswordState::new("secret", future);
    /// let (has, valid_for) = state.to_api_object();
    ///
    /// assert!(has);
    /// assert!(valid_for > 0 && valid_for <= 3600);
    /// ```
    #[must_use]
    pub fn to_api_object(&self) -> (bool, i32) {
        if !self.is_valid() {
            return (false, 0);
        }

        let remaining = self
            .remaining_time()
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        (true, remaining)
    }

    /// Clears the temporary password state.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let future = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .map(|d| d.as_secs() as i32 + 3600)
    ///     .unwrap_or(0);
    /// let mut state = TempPasswordState::new("secret", future);
    /// assert!(state.has_temp_password());
    ///
    /// state.clear();
    /// assert!(!state.has_temp_password());
    /// ```
    pub fn clear(&mut self) {
        self.has_temp_password = false;
        self.temp_password.clear();
        self.valid_until = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to get current unix timestamp
    fn now() -> i32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32
    }

    #[test]
    fn test_default() {
        let state = TempPasswordState::default();
        assert!(!state.has_temp_password());
        assert_eq!(state.temp_password(), "");
        assert_eq!(state.valid_until(), 0);
    }

    #[test]
    fn test_new() {
        let state = TempPasswordState::new("test123", 1735795200);
        assert!(state.has_temp_password());
        assert_eq!(state.temp_password(), "test123");
        assert_eq!(state.valid_until(), 1735795200);
    }

    #[test]
    fn test_new_from_str() {
        let state = TempPasswordState::new("test123", 1735795200);
        assert!(state.has_temp_password());
        assert_eq!(state.temp_password(), "test123");
    }

    #[test]
    fn test_new_from_string() {
        let state = TempPasswordState::new(String::from("test123"), 1735795200);
        assert!(state.has_temp_password());
        assert_eq!(state.temp_password(), "test123");
    }

    #[test]
    fn test_clear() {
        let mut state = TempPasswordState::new("secret", 1735795200);
        state.clear();
        assert_eq!(state, TempPasswordState::default());
    }

    #[test]
    fn test_is_valid_future() {
        let future = now() + 86400;
        let state = TempPasswordState::new("secret", future);
        assert!(state.is_valid());
    }

    #[test]
    fn test_is_valid_past() {
        let past = now() - 86400;
        let state = TempPasswordState::new("secret", past);
        assert!(!state.is_valid());
    }

    #[test]
    fn test_is_valid_no_password() {
        let state = TempPasswordState::default();
        assert!(!state.is_valid());
    }

    #[test]
    fn test_remaining_time_valid() {
        let future = now() + 3600;
        let state = TempPasswordState::new("secret", future);

        let remaining = state.remaining_time();
        assert!(remaining.is_some());
        assert!(remaining.unwrap().as_secs() <= 3600);
    }

    #[test]
    fn test_remaining_time_expired() {
        let past = now() - 86400;
        let state = TempPasswordState::new("secret", past);

        let remaining = state.remaining_time();
        assert!(remaining.is_none());
    }

    #[test]
    fn test_remaining_time_no_password() {
        let state = TempPasswordState::default();
        assert!(state.remaining_time().is_none());
    }

    #[test]
    fn test_to_api_object_valid() {
        let future = now() + 3600;
        let state = TempPasswordState::new("secret", future);

        let (has, valid_for) = state.to_api_object();
        assert!(has);
        assert!(valid_for > 0 && valid_for <= 3600);
    }

    #[test]
    fn test_to_api_object_expired() {
        let past = now() - 86400;
        let state = TempPasswordState::new("secret", past);

        let (has, valid_for) = state.to_api_object();
        assert!(!has);
        assert_eq!(valid_for, 0);
    }

    #[test]
    fn test_to_api_object_no_password() {
        let state = TempPasswordState::default();

        let (has, valid_for) = state.to_api_object();
        assert!(!has);
        assert_eq!(valid_for, 0);
    }

    #[test]
    fn test_equality() {
        let state1 = TempPasswordState::new("secret", 1735795200);
        let state2 = TempPasswordState::new("secret", 1735795200);
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_inequality_password() {
        let state1 = TempPasswordState::new("secret1", 1735795200);
        let state2 = TempPasswordState::new("secret2", 1735795200);
        assert_ne!(state1, state2);
    }

    #[test]
    fn test_inequality_timestamp() {
        let state1 = TempPasswordState::new("secret", 1735795200);
        let state2 = TempPasswordState::new("secret", 1735795201);
        assert_ne!(state1, state2);
    }

    #[test]
    fn test_clone() {
        let state1 = TempPasswordState::new("secret", 1735795200);
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }
}
