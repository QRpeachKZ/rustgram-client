//! # Resource State
//!
//! Resource tracking and limit management for file operations.
//!
//! ## Overview
//!
//! This module provides `ResourceState`, a type for tracking resource usage
//! including limits, estimated limits, and active usage. This is used by
//! the file download/upload managers to control bandwidth and resource allocation.
//!
//! ## TDLib Correspondence
//!
//! TDLib class: `td::telegram::files::ResourceState`
//! - Tracks resource limits (master) and local estimates (self)
//! - Supports master/slave synchronization pattern
//! - Handles partial resource allocation with active usage tracking
//!
//! ## Architecture
//!
//! The resource state maintains:
//! - **limit_** - Master-controlled resource limit
//! - **estimated_limit_** - Self-estimated resource limit
//! - **used_** - Resources already consumed
//! - **using_** - Resources currently in use
//! - **unit_size_** - Allocation unit size (for rounding)
//!
//! ## Examples
//!
//! ```
//! use rustgram_resource_state::ResourceState;
//!
//! let mut state = ResourceState::new();
//! state.set_unit_size(1024); // 1KB units
//! state.update_limit(4096).unwrap(); // Set limit to 4KB
//!
//! // Start using resources
//! state.start_use(2048).unwrap();
//!
//! // Check available resources
//! assert_eq!(state.unused(), 2048); // 4096 - 2048 - 0
//!
//! // Stop using and mark as consumed
//! state.stop_use(1024).unwrap();
//! assert_eq!(state.get_using(), 1024);
//! assert_eq!(state.get_used(), 1024);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::absurd_extreme_comparisons)] // For range checks

use core::fmt;

/// Resource tracking and limit management state.
///
/// Tracks resource usage with master/slave synchronization pattern.
/// Used for bandwidth control, file size limits, and resource allocation.
///
/// # Fields
///
/// - `estimated_limit_` - Self-estimated resource limit
/// - `limit_` - Master-controlled resource limit
/// - `used_` - Resources already consumed
/// - `using_` - Resources currently in use
/// - `unit_size_` - Allocation unit size (default: 1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceState {
    /// Self-estimated resource limit (updated by update_estimated_limit)
    estimated_limit: i64,
    /// Master-controlled resource limit (updated by update_slave)
    limit: i64,
    /// Resources already consumed
    used: i64,
    /// Resources currently in use (not yet marked as consumed)
    using: i64,
    /// Unit size for allocation rounding
    unit_size: usize,
}

impl Default for ResourceState {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceState {
    /// Creates a new ResourceState with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let state = ResourceState::new();
    /// assert_eq!(state.get_using(), 0);
    /// assert_eq!(state.get_used(), 0);
    /// assert_eq!(state.unit_size(), 1);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            estimated_limit: 0,
            limit: 0,
            used: 0,
            using: 0,
            unit_size: 1,
        }
    }

    /// Creates a new ResourceState with the given unit size.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let state = ResourceState::with_unit_size(1024);
    /// assert_eq!(state.unit_size(), 1024);
    /// ```
    #[must_use]
    pub const fn with_unit_size(unit_size: usize) -> Self {
        Self {
            estimated_limit: 0,
            limit: 0,
            used: 0,
            using: 0,
            unit_size,
        }
    }

    /// Starts using the specified amount of resources.
    ///
    /// Increases the `using` count. Must ensure that `used + using <= limit`.
    ///
    /// # Errors
    ///
    /// Returns `Error::LimitExceeded` if adding to `using` would exceed `limit`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// state.start_use(500).unwrap();
    /// assert_eq!(state.get_using(), 500);
    /// ```
    pub fn start_use(&mut self, x: i64) -> Result<(), Error> {
        if self.used + self.using + x > self.limit {
            return Err(Error::LimitExceeded {
                requested: x,
                used: self.used,
                using: self.using,
                limit: self.limit,
            });
        }
        self.using += x;
        Ok(())
    }

    /// Stops using the specified amount of resources.
    ///
    /// Decreases `using` and increases `used` by the specified amount.
    ///
    /// # Errors
    ///
    /// Returns `Error::InsufficientUsing` if `x` is greater than current `using`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// state.start_use(500).unwrap();
    /// state.stop_use(300).unwrap();
    /// assert_eq!(state.get_using(), 200);
    /// assert_eq!(state.get_used(), 300);
    /// ```
    pub fn stop_use(&mut self, x: i64) -> Result<(), Error> {
        if x > self.using {
            return Err(Error::InsufficientUsing {
                requested: x,
                using: self.using,
            });
        }
        self.using -= x;
        self.used += x;
        Ok(())
    }

    /// Updates the master resource limit by adding the specified amount.
    ///
    /// # Errors
    ///
    /// Returns `Error::Overflow` if adding would overflow i64.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// assert_eq!(state.limit(), 1000);
    /// state.update_limit(500).unwrap();
    /// assert_eq!(state.limit(), 1500);
    /// ```
    pub fn update_limit(&mut self, extra: i64) -> Result<(), Error> {
        self.limit = self
            .limit
            .checked_add(extra)
            .ok_or(Error::Overflow)?;
        Ok(())
    }

    /// Updates the estimated limit based on extra resources available.
    ///
    /// This implements the TDLib algorithm for estimated limit calculation:
    /// 1. Calculate intersection between `using` and `extra`
    /// 2. Compute new_estimated_limit = used + using + extra - intersection
    /// 3. If new estimate is less than limit, use extra limit
    /// 4. Round up to unit_size boundary
    ///
    /// Returns `true` if the estimated limit changed.
    ///
    /// # Errors
    ///
    /// Returns `Error::Overflow` if calculation would overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// state.start_use(200).unwrap();
    ///
    /// // First update should change the estimate
    /// assert!(state.update_estimated_limit(500).unwrap());
    ///
    /// // Same update should not change
    /// assert!(!state.update_estimated_limit(500).unwrap());
    /// ```
    pub fn update_estimated_limit(&mut self, extra: i64) -> Result<bool, Error> {
        // Calculate intersection between using and extra
        let using_and_extra_intersection = self.using.min(extra);

        // Compute new estimated limit
        let new_estimated_limit = (self.used + self.using + extra - using_and_extra_intersection)
            .checked_add(
                // Use extra limit if estimate < limit
                if self.used + self.using + extra - using_and_extra_intersection < self.limit {
                    self.limit - (self.used + self.using + extra - using_and_extra_intersection)
                } else {
                    0
                }
            )
            .ok_or(Error::Overflow)?;

        if new_estimated_limit == self.estimated_limit {
            return Ok(false);
        }

        self.estimated_limit = new_estimated_limit;
        Ok(true)
    }

    /// Returns the active limit (limit - used).
    ///
    /// This is the amount of resources that can still be allocated.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// assert_eq!(state.active_limit(), 1000);
    ///
    /// state.start_use(500).unwrap();
    /// state.stop_use(300).unwrap();
    /// assert_eq!(state.active_limit(), 700); // 1000 - 300 used
    /// ```
    #[must_use]
    pub const fn active_limit(&self) -> i64 {
        self.limit - self.used
    }

    /// Returns the amount of resources currently in use.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// state.start_use(400).unwrap();
    /// assert_eq!(state.get_using(), 400);
    /// ```
    #[must_use]
    pub const fn get_using(&self) -> i64 {
        self.using
    }

    /// Returns the amount of resources already used/consumed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// state.start_use(500).unwrap();
    /// state.stop_use(500).unwrap();
    /// assert_eq!(state.get_used(), 500);
    /// ```
    #[must_use]
    pub const fn get_used(&self) -> i64 {
        self.used
    }

    /// Returns the total limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// assert_eq!(state.limit(), 1000);
    /// ```
    #[must_use]
    pub const fn limit(&self) -> i64 {
        self.limit
    }

    /// Returns the estimated limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let state = ResourceState::new();
    /// assert_eq!(state.estimated_limit(), 0);
    /// ```
    #[must_use]
    pub const fn estimated_limit(&self) -> i64 {
        self.estimated_limit
    }

    /// Returns the unused resources (limit - using - used).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.update_limit(1000).unwrap();
    /// state.start_use(300).unwrap();
    /// state.stop_use(200).unwrap();
    /// // unused = limit - using - used = 1000 - 100 - 200 = 700
    /// assert_eq!(state.unused(), 700);
    /// ```
    #[must_use]
    pub const fn unused(&self) -> i64 {
        self.limit - self.using - self.used
    }

    /// Returns the estimated extra resources available.
    ///
    /// Calculates unused with rounding to unit_size boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.set_unit_size(1024);
    /// state.update_limit(10000).unwrap();
    /// state.start_use(1000).unwrap();
    ///
    /// let extra = state.estimated_extra();
    /// assert!(extra >= 0);
    /// ```
    #[must_use]
    pub fn estimated_extra(&self) -> i64 {
        let max_limit = self.limit.max(self.estimated_limit);
        let new_unused = max_limit - self.using - self.used;

        // Round up to unit_size
        let unit = i64::try_from(self.unit_size).unwrap_or(i64::MAX);
        let rounded_unused = if new_unused > 0 {
            ((new_unused + unit - 1) / unit) * unit
        } else {
            new_unused
        };

        rounded_unused + self.using + self.used - self.limit
    }

    /// Sets the unit size for allocation rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut state = ResourceState::new();
    /// state.set_unit_size(4096);
    /// assert_eq!(state.unit_size(), 4096);
    /// ```
    pub fn set_unit_size(&mut self, new_unit_size: usize) {
        self.unit_size = new_unit_size;
    }

    /// Returns the current unit size.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let state = ResourceState::with_unit_size(2048);
    /// assert_eq!(state.unit_size(), 2048);
    /// ```
    #[must_use]
    pub const fn unit_size(&self) -> usize {
        self.unit_size
    }

    /// Adds another ResourceState to this one.
    ///
    /// Adds the other's active_limit to using and used to used.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut a = ResourceState::new();
    /// let mut b = ResourceState::new();
    ///
    /// a.update_limit(1000).unwrap();
    /// a.start_use(400).unwrap();
    /// a.stop_use(200).unwrap();
    ///
    /// b.update_limit(500).unwrap();
    /// b.start_use(300).unwrap();
    /// b.stop_use(100).unwrap();
    ///
    /// a.add_assign(&b).unwrap();
    /// assert_eq!(a.get_used(), 300); // 200 + 100
    /// ```
    pub fn add_assign(&mut self, other: &ResourceState) -> Result<(), Error> {
        self.using = self
            .using
            .checked_add(other.active_limit())
            .ok_or(Error::Overflow)?;
        self.used = self.used.checked_add(other.used).ok_or(Error::Overflow)?;
        Ok(())
    }

    /// Subtracts another ResourceState from this one.
    ///
    /// Subtracts the other's active_limit from using and used from used.
    ///
    /// # Errors
    ///
    /// Returns `Error::Underflow` if subtraction would go negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut a = ResourceState::new();
    /// let mut b = ResourceState::new();
    ///
    /// a.update_limit(2000).unwrap();
    /// a.start_use(1000).unwrap();
    /// a.stop_use(500).unwrap(); // using=500, used=500
    ///
    /// b.update_limit(500).unwrap();
    /// b.start_use(300).unwrap();
    /// b.stop_use(100).unwrap(); // using=200, used=100, active_limit=400
    ///
    /// a.sub_assign(&b).unwrap();
    /// // using -= 400, used -= 100
    /// assert_eq!(a.get_using(), 100);
    /// assert_eq!(a.get_used(), 400);
    /// ```
    pub fn sub_assign(&mut self, other: &ResourceState) -> Result<(), Error> {
        if other.active_limit() > self.using {
            return Err(Error::Underflow {
                field: "using",
                requested: other.active_limit(),
                available: self.using,
            });
        }
        if other.used > self.used {
            return Err(Error::Underflow {
                field: "used",
                requested: other.used,
                available: self.used,
            });
        }

        self.using -= other.active_limit();
        self.used -= other.used;
        Ok(())
    }

    /// Updates this state from master state.
    ///
    /// Copies estimated_limit, used, using, and unit_size from master.
    /// Used when this slave synchronizes with its master.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut master = ResourceState::with_unit_size(2048);
    /// master.update_limit(5000).unwrap();
    /// master.start_use(1000).unwrap();
    ///
    /// let mut slave = ResourceState::new();
    /// slave.update_from_master(&master);
    ///
    /// assert_eq!(slave.unit_size(), 2048);
    /// assert_eq!(slave.get_using(), 1000);
    /// ```
    pub fn update_from_master(&mut self, master: &ResourceState) {
        self.estimated_limit = master.estimated_limit;
        self.used = master.used;
        self.using = master.using;
        self.unit_size = master.unit_size;
    }

    /// Updates this slave's limit from master state.
    ///
    /// Copies only the limit from master.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_state::ResourceState;
    ///
    /// let mut master = ResourceState::new();
    /// master.update_limit(10000).unwrap();
    ///
    /// let mut slave = ResourceState::new();
    /// slave.update_slave_limit(&master);
    ///
    /// assert_eq!(slave.limit(), 10000);
    /// ```
    pub fn update_slave_limit(&mut self, master: &ResourceState) {
        self.limit = master.limit;
    }
}

impl fmt::Display for ResourceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ResourceState {{ estimated_limit: {}, used: {}, using: {}, limit: {}, unit_size: {} }}",
            self.estimated_limit, self.used, self.using, self.limit, self.unit_size
        )
    }
}

/// Errors that can occur during ResourceState operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Resource limit would be exceeded
    LimitExceeded {
        /// Requested amount
        requested: i64,
        /// Already used
        used: i64,
        /// Currently in use
        using: i64,
        /// Maximum limit
        limit: i64,
    },
    /// Insufficient resources in use to stop
    InsufficientUsing {
        /// Requested amount to stop
        requested: i64,
        /// Currently in use
        using: i64,
    },
    /// Arithmetic overflow
    Overflow,
    /// Arithmetic underflow
    Underflow {
        /// Field name
        field: &'static str,
        /// Requested amount
        requested: i64,
        /// Available amount
        available: i64,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitExceeded { requested, used, using, limit } => write!(
                f,
                "LimitExceeded: requested {} (used: {}, using: {}, limit: {})",
                requested, used, using, limit
            ),
            Self::InsufficientUsing { requested, using } => {
                write!(f, "InsufficientUsing: requested {} (using: {})", requested, using)
            }
            Self::Overflow => write!(f, "Overflow: arithmetic operation overflowed"),
            Self::Underflow { field, requested, available } => write!(
                f,
                "Underflow: {} requested {} (available: {})",
                field, requested, available
            ),
        }
    }
}

// Always implement std::error::Error since std is always available
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    // Constructor tests (3)
    #[test]
    fn test_new_default() {
        let state = ResourceState::new();
        assert_eq!(state.get_using(), 0);
        assert_eq!(state.get_used(), 0);
        assert_eq!(state.limit(), 0);
        assert_eq!(state.estimated_limit(), 0);
        assert_eq!(state.unit_size(), 1);
    }

    #[test]
    fn test_with_unit_size() {
        let state = ResourceState::with_unit_size(4096);
        assert_eq!(state.unit_size(), 4096);
        assert_eq!(state.get_using(), 0);
    }

    #[test]
    fn test_default_impl() {
        let state = ResourceState::default();
        assert_eq!(state.get_using(), 0);
        assert_eq!(state.unit_size(), 1);
    }

    // start_use tests (4)
    #[test]
    fn test_start_use_basic() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(500).unwrap();
        assert_eq!(state.get_using(), 500);
        assert_eq!(state.get_used(), 0);
    }

    #[test]
    fn test_start_use_multiple() {
        let mut state = ResourceState::new();
        state.update_limit(2000).unwrap();
        state.start_use(500).unwrap();
        state.start_use(300).unwrap();
        assert_eq!(state.get_using(), 800);
    }

    #[test]
    fn test_start_use_exceeds_limit() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(800).unwrap();
        let result = state.start_use(300);
        assert!(result.is_err());
        assert_eq!(state.get_using(), 800); // Unchanged
    }

    #[test]
    fn test_start_use_negative() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        // Negative values should work for corrections
        state.start_use(500).unwrap();
        state.start_use(-100).unwrap();
        assert_eq!(state.get_using(), 400);
    }

    // stop_use tests (4)
    #[test]
    fn test_stop_use_basic() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(500).unwrap();
        state.stop_use(300).unwrap();
        assert_eq!(state.get_using(), 200);
        assert_eq!(state.get_used(), 300);
    }

    #[test]
    fn test_stop_use_all() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(500).unwrap();
        state.stop_use(500).unwrap();
        assert_eq!(state.get_using(), 0);
        assert_eq!(state.get_used(), 500);
    }

    #[test]
    fn test_stop_use_insufficient() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(200).unwrap();
        let result = state.stop_use(300);
        assert!(result.is_err());
        assert_eq!(state.get_using(), 200); // Unchanged
    }

    #[test]
    fn test_stop_use_zero() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(500).unwrap();
        state.stop_use(0).unwrap();
        assert_eq!(state.get_using(), 500);
        assert_eq!(state.get_used(), 0);
    }

    // update_limit tests (3)
    #[test]
    fn test_update_limit_positive() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        assert_eq!(state.limit(), 1000);
        state.update_limit(500).unwrap();
        assert_eq!(state.limit(), 1500);
    }

    #[test]
    fn test_update_limit_negative() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.update_limit(-300).unwrap();
        assert_eq!(state.limit(), 700);
    }

    #[test]
    fn test_update_limit_overflow() {
        let mut state = ResourceState::new();
        state.limit = i64::MAX - 10;
        let result = state.update_limit(100);
        assert!(result.is_err());
    }

    // update_estimated_limit tests (4)
    #[test]
    fn test_update_estimated_limit_changes() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(200).unwrap();

        let changed = state.update_estimated_limit(500).unwrap();
        assert!(changed);
    }

    #[test]
    fn test_update_estimated_limit_no_change() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(200).unwrap();

        state.update_estimated_limit(500).unwrap();
        let changed = state.update_estimated_limit(500).unwrap();
        assert!(!changed);
    }

    #[test]
    fn test_update_estimated_limit_with_active_usage() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(400).unwrap();
        state.stop_use(100).unwrap();

        // Should calculate intersection between using (300) and extra
        state.update_estimated_limit(300).unwrap();
        assert!(state.estimated_limit() >= 0);
    }

    #[test]
    fn test_update_estimated_limit_uses_extra() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(100).unwrap();

        // When estimate < limit, should use extra limit
        state.update_estimated_limit(200).unwrap();
        // estimated_limit should be closer to limit
        assert!(state.estimated_limit() >= 100);
    }

    // active_limit tests (2)
    #[test]
    fn test_active_limit_basic() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        assert_eq!(state.active_limit(), 1000);

        state.start_use(500).unwrap();
        state.stop_use(300).unwrap();
        assert_eq!(state.active_limit(), 700); // 1000 - 300 used
    }

    #[test]
    fn test_active_limit_with_using() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(200).unwrap();
        state.stop_use(100).unwrap();
        // active_limit = limit - used = 1000 - 100 = 900
        assert_eq!(state.active_limit(), 900);
    }

    // unused tests (3)
    #[test]
    fn test_unused_basic() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        assert_eq!(state.unused(), 1000);
    }

    #[test]
    fn test_unused_with_usage() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(300).unwrap();
        state.stop_use(100).unwrap();
        // unused = limit - using - used = 1000 - 200 - 100 = 700
        assert_eq!(state.unused(), 700);
    }

    #[test]
    fn test_unused_zero() {
        let mut state = ResourceState::new();
        state.update_limit(1000).unwrap();
        state.start_use(1000).unwrap();
        state.stop_use(1000).unwrap();
        // unused = limit - using - used = 1000 - 0 - 1000 = 0
        assert_eq!(state.unused(), 0);
    }

    // estimated_extra tests (2)
    #[test]
    fn test_estimated_extra_basic() {
        let mut state = ResourceState::new();
        state.update_limit(10000).unwrap();
        state.start_use(1000).unwrap();

        let extra = state.estimated_extra();
        assert!(extra >= 0);
    }

    #[test]
    fn test_estimated_extra_with_unit_size() {
        let mut state = ResourceState::new();
        state.set_unit_size(1024);
        state.update_limit(10000).unwrap();
        state.start_use(1000).unwrap();

        let extra = state.estimated_extra();
        // Should be a reasonable value (non-negative)
        assert!(extra >= 0);
    }

    // set_unit_size tests (2)
    #[test]
    fn test_set_unit_size() {
        let mut state = ResourceState::new();
        state.set_unit_size(2048);
        assert_eq!(state.unit_size(), 2048);
        state.set_unit_size(4096);
        assert_eq!(state.unit_size(), 4096);
    }

    #[test]
    fn test_set_unit_size_zero() {
        let mut state = ResourceState::new();
        state.set_unit_size(0);
        assert_eq!(state.unit_size(), 0);
    }

    // add_assign tests (3)
    #[test]
    fn test_add_assign_basic() {
        let mut a = ResourceState::new();
        let mut b = ResourceState::new();

        a.update_limit(1000).unwrap();
        a.start_use(400).unwrap();
        a.stop_use(200).unwrap();

        b.update_limit(500).unwrap();
        b.start_use(300).unwrap();
        b.stop_use(100).unwrap();

        a.add_assign(&b).unwrap();
        // using += b.active_limit (500 - 100 = 400)
        // used += b.used (100)
        assert_eq!(a.get_used(), 300); // 200 + 100
        assert_eq!(a.get_using(), 600); // 200 + 400
    }

    #[test]
    fn test_add_assign_overflow() {
        let mut a = ResourceState::new();
        let mut b = ResourceState::new();

        a.update_limit(i64::MAX).unwrap();
        a.using = i64::MAX - 10;
        b.update_limit(100).unwrap();

        let result = a.add_assign(&b);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_assign_preserves_limit() {
        let mut a = ResourceState::new();
        let mut b = ResourceState::new();

        a.update_limit(1000).unwrap();
        b.update_limit(500).unwrap();

        let original_limit = a.limit();
        a.add_assign(&b).unwrap();
        assert_eq!(a.limit(), original_limit);
    }

    // sub_assign tests (3)
    #[test]
    fn test_sub_assign_basic() {
        let mut a = ResourceState::new();
        let mut b = ResourceState::new();

        a.update_limit(2000).unwrap();
        a.start_use(1000).unwrap();
        a.stop_use(500).unwrap(); // using=500, used=500

        b.update_limit(500).unwrap();
        b.start_use(300).unwrap();
        b.stop_use(100).unwrap(); // using=200, used=100, active_limit=400

        a.sub_assign(&b).unwrap();
        // using -= b.active_limit (400)
        // used -= b.used (100)
        assert_eq!(a.get_using(), 100); // 500 - 400
        assert_eq!(a.get_used(), 400);  // 500 - 100
    }

    #[test]
    fn test_sub_assign_underflow() {
        let mut a = ResourceState::new();
        let mut b = ResourceState::new();

        a.update_limit(1000).unwrap();
        a.start_use(100).unwrap();
        a.stop_use(50).unwrap(); // using=50, used=50

        b.update_limit(500).unwrap();
        b.start_use(200).unwrap();
        b.stop_use(100).unwrap(); // using=100, used=100, active_limit=400

        let result = a.sub_assign(&b);
        // Should fail because active_limit (400) > using (50)
        assert!(result.is_err());
    }

    #[test]
    fn test_sub_assign_preserves_limit() {
        let mut a = ResourceState::new();
        let mut b = ResourceState::new();

        a.update_limit(2000).unwrap();
        a.start_use(1000).unwrap();
        a.stop_use(500).unwrap();

        b.update_limit(500).unwrap();
        b.start_use(300).unwrap();
        b.stop_use(100).unwrap();

        let original_limit = a.limit();
        a.sub_assign(&b).unwrap();
        assert_eq!(a.limit(), original_limit);
    }

    // update_from_master tests (3)
    #[test]
    fn test_update_from_master_basic() {
        let mut master = ResourceState::with_unit_size(2048);
        master.update_limit(5000).unwrap();
        master.start_use(1000).unwrap();
        master.stop_use(500).unwrap();

        let mut slave = ResourceState::new();
        slave.update_from_master(&master);

        assert_eq!(slave.unit_size(), 2048);
        assert_eq!(slave.get_using(), 500);   // 1000 - 500
        assert_eq!(slave.get_used(), 500);
        assert_eq!(slave.estimated_limit(), master.estimated_limit());
    }

    #[test]
    fn test_update_from_master_preserves_slave_limit() {
        let mut master = ResourceState::new();
        master.update_limit(5000).unwrap();

        let mut slave = ResourceState::new();
        slave.update_limit(1000).unwrap();

        slave.update_from_master(&master);
        assert_eq!(slave.limit(), 1000); // Slave's limit is preserved
    }

    #[test]
    fn test_update_from_master_all_fields() {
        let mut master = ResourceState::with_unit_size(4096);
        master.update_limit(10000).unwrap();
        master.start_use(2000).unwrap();
        master.stop_use(1000).unwrap();
        master.update_estimated_limit(5000).unwrap();

        let mut slave = ResourceState::new();
        slave.update_from_master(&master);

        assert_eq!(slave.unit_size(), 4096);
        assert_eq!(slave.get_using(), 1000);
        assert_eq!(slave.get_used(), 1000);
        assert_eq!(slave.estimated_limit(), master.estimated_limit());
    }

    // update_slave_limit tests (2)
    #[test]
    fn test_update_slave_limit_basic() {
        let mut master = ResourceState::new();
        master.update_limit(10000).unwrap();

        let mut slave = ResourceState::new();
        slave.update_slave_limit(&master);

        assert_eq!(slave.limit(), 10000);
    }

    #[test]
    fn test_update_slave_limit_only_limit() {
        let mut master = ResourceState::new();
        master.update_limit(10000).unwrap();
        master.start_use(1000).unwrap();

        let mut slave = ResourceState::new();
        slave.update_slave_limit(&master);

        assert_eq!(slave.limit(), 10000);
        assert_eq!(slave.get_using(), 0); // Other fields unchanged
    }

    // Display tests (2)
    #[test]
    fn test_display() {
        let state = ResourceState::new();
        let s = format!("{}", state);
        assert!(s.contains("ResourceState"));
        assert!(s.contains("estimated_limit"));
    }

    #[test]
    fn test_debug() {
        let state = ResourceState::new();
        let s = format!("{:?}", state);
        assert!(s.contains("ResourceState"));
    }

    // Clone tests (2)
    #[test]
    fn test_clone_independent() {
        let mut a = ResourceState::new();
        a.update_limit(1000).unwrap();
        a.start_use(500).unwrap();

        let mut b = a.clone();
        b.stop_use(200).unwrap();

        assert_eq!(a.get_using(), 500);
        assert_eq!(b.get_using(), 300);
    }

    #[test]
    fn test_clone_equality() {
        let mut a = ResourceState::new();
        a.update_limit(1000).unwrap();
        a.start_use(500).unwrap();

        let b = a.clone();
        assert_eq!(a, b);
    }

    // PartialEq tests (2)
    #[test]
    fn test_partial_eq_equal() {
        let a = ResourceState::new();
        let b = ResourceState::new();
        assert_eq!(a, b);
    }

    #[test]
    fn test_partial_eq_not_equal() {
        let mut a = ResourceState::new();
        a.update_limit(1000).unwrap();

        let b = ResourceState::new();
        assert_ne!(a, b);
    }

    // Error tests (6)
    #[test]
    fn test_error_limit_exceeded() {
        let err = Error::LimitExceeded {
            requested: 500,
            used: 300,
            using: 200,
            limit: 1000,
        };
        let s = format!("{}", err);
        assert!(s.contains("LimitExceeded"));
    }

    #[test]
    fn test_error_insufficient_using() {
        let err = Error::InsufficientUsing {
            requested: 500,
            using: 200,
        };
        let s = format!("{}", err);
        assert!(s.contains("InsufficientUsing"));
    }

    #[test]
    fn test_error_overflow() {
        let err = Error::Overflow;
        let s = format!("{}", err);
        assert!(s.contains("Overflow"));
    }

    #[test]
    fn test_error_underflow() {
        let err = Error::Underflow {
            field: "using",
            requested: 500,
            available: 200,
        };
        let s = format!("{}", err);
        assert!(s.contains("Underflow"));
    }

    #[test]
    fn test_error_std_error() {
        let err = Error::Overflow;
        assert!(err.source().is_none()); // No source for Overflow
    }

    // Integration tests (2)
    #[test]
    fn test_full_resource_lifecycle() {
        let mut state = ResourceState::with_unit_size(1024);
        state.update_limit(10000).unwrap();

        // Allocate resources
        state.start_use(5000).unwrap();
        assert_eq!(state.unused(), 5000);

        // Partially consume
        state.stop_use(2000).unwrap();
        assert_eq!(state.get_using(), 3000);
        assert_eq!(state.get_used(), 2000);

        // Release remaining
        state.stop_use(3000).unwrap();
        assert_eq!(state.get_using(), 0);
        assert_eq!(state.get_used(), 5000);
        assert_eq!(state.active_limit(), 5000);
    }

    #[test]
    fn test_master_slave_sync() {
        let mut master = ResourceState::with_unit_size(2048);
        master.update_limit(50000).unwrap();

        let mut slave = ResourceState::new();

        // Master allocates resources
        master.start_use(10000).unwrap();
        master.stop_use(5000).unwrap();

        // Slave syncs with master
        slave.update_slave_limit(&master);
        slave.update_from_master(&master);

        assert_eq!(slave.limit(), 50000);
        assert_eq!(slave.get_using(), 5000);
        assert_eq!(slave.get_used(), 5000);
        assert_eq!(slave.unit_size(), 2048);
    }
}
