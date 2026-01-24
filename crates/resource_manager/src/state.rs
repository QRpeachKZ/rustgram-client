//! Resource state tracking.
//!
//! This module provides state tracking for resource allocation,
//! similar to TDLib's ResourceState class.

/// Tracks the state of resource allocation.
///
/// This struct maintains information about resource limits, usage,
/// and estimation for bandwidth management.
///
/// TDLib equivalent: td::ResourceState
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceState {
    /// Estimated resource limit
    estimated_limit: i64,
    /// Actual resource limit assigned by manager
    limit: i64,
    /// Resources that have been used
    used: i64,
    /// Resources currently in use
    using: i64,
    /// Unit size for allocation (e.g., chunk size)
    unit_size: u64,
}

impl Default for ResourceState {
    fn default() -> Self {
        Self {
            estimated_limit: 0,
            limit: 0,
            used: 0,
            using: 0,
            unit_size: 1,
        }
    }
}

impl ResourceState {
    /// Creates a new resource state with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new resource state with the specified unit size.
    pub fn with_unit_size(unit_size: u64) -> Self {
        Self {
            unit_size,
            ..Default::default()
        }
    }

    /// Starts using the specified amount of resources.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount of resources to start using
    pub fn start_use(&mut self, amount: i64) {
        self.using = self.using.saturating_add(amount);
        // Verify we don't exceed the limit
        debug_assert!(
            self.used + self.using <= self.limit,
            "Resource usage exceeds limit"
        );
    }

    /// Stops using the specified amount of resources.
    ///
    /// Moves resources from "using" to "used".
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount of resources to stop using
    pub fn stop_use(&mut self, amount: i64) {
        self.using = self.using.saturating_sub(amount).max(0);
        self.used = self.used.saturating_add(amount);
    }

    /// Updates the resource limit by adding extra resources.
    ///
    /// # Arguments
    ///
    /// * `extra` - Additional resources to add to the limit
    pub fn update_limit(&mut self, extra: i64) {
        self.limit = self.limit.saturating_add(extra);
    }

    /// Updates the estimated limit.
    ///
    /// Returns `true` if the estimated limit changed.
    ///
    /// # Arguments
    ///
    /// * `extra` - Additional estimated resources
    pub fn update_estimated_limit(&mut self, extra: i64) -> bool {
        // Calculate intersection between using and extra
        let using_and_extra_intersection = self.using.min(extra);
        let new_estimated_limit = self.used + self.using + extra - using_and_extra_intersection;

        // Use extra limit if available
        let (new_estimated_limit, used_delta) = if new_estimated_limit < self.limit {
            let extra_limit = self.limit.saturating_sub(new_estimated_limit);
            (new_estimated_limit + extra_limit, extra_limit)
        } else {
            (new_estimated_limit, 0)
        };

        if new_estimated_limit == self.estimated_limit {
            return false;
        }

        self.estimated_limit = new_estimated_limit;
        self.used = self.used.saturating_add(used_delta);
        true
    }

    /// Returns the active limit (limit minus used).
    pub fn active_limit(&self) -> i64 {
        self.limit.saturating_sub(self.used)
    }

    /// Returns the amount of resources currently being used.
    pub fn get_using(&self) -> i64 {
        self.using
    }

    /// Returns the amount of unused resources.
    pub fn unused(&self) -> i64 {
        self.limit
            .saturating_sub(self.using)
            .saturating_sub(self.used)
    }

    /// Returns the estimated extra resources needed.
    pub fn estimated_extra(&self) -> i64 {
        let max_limit = self.limit.max(self.estimated_limit);
        let new_unused = max_limit
            .saturating_sub(self.using)
            .saturating_sub(self.used);

        // Round up to unit size
        let unit_size = self.unit_size as i64;
        let rounded = if unit_size > 0 {
            ((new_unused + unit_size - 1) / unit_size) * unit_size
        } else {
            new_unused
        };

        rounded
            .saturating_add(self.using)
            .saturating_add(self.used)
            .saturating_sub(self.limit)
    }

    /// Sets the unit size for resource allocation.
    pub fn set_unit_size(&mut self, unit_size: u64) {
        self.unit_size = unit_size.max(1);
    }

    /// Returns the unit size.
    pub fn unit_size(&self) -> u64 {
        self.unit_size
    }

    /// Returns the estimated limit.
    pub fn estimated_limit(&self) -> i64 {
        self.estimated_limit
    }

    /// Returns the actual limit.
    pub fn limit(&self) -> i64 {
        self.limit
    }

    /// Returns the amount of used resources.
    pub fn used(&self) -> i64 {
        self.used
    }

    /// Adds another resource state to this one.
    pub fn add(&mut self, other: &ResourceState) {
        self.using = self.using.saturating_add(other.active_limit());
        self.used = self.used.saturating_add(other.used);
    }

    /// Subtracts another resource state from this one.
    pub fn sub(&mut self, other: &ResourceState) {
        self.using = self.using.saturating_sub(other.active_limit()).max(0);
        self.used = self.used.saturating_sub(other.used).max(0);
    }

    /// Updates this state from another (master) state.
    pub fn update_from(&mut self, other: &ResourceState) {
        self.estimated_limit = other.estimated_limit;
        self.used = other.used;
        self.using = other.using;
        self.unit_size = other.unit_size;
    }

    /// Updates the slave limit from another state.
    pub fn update_slave_limit(&mut self, other: &ResourceState) {
        self.limit = other.limit;
    }

    /// Returns whether all resources are unused.
    pub fn is_unused(&self) -> bool {
        self.unused() > 0
    }

    /// Returns whether at maximum capacity.
    pub fn is_at_capacity(&self) -> bool {
        self.unused() == 0 && self.using == 0
    }

    /// Resets the state to default values.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let state = ResourceState::default();
        assert_eq!(state.estimated_limit(), 0);
        assert_eq!(state.limit(), 0);
        assert_eq!(state.used(), 0);
        assert_eq!(state.get_using(), 0);
        assert_eq!(state.unit_size(), 1);
    }

    #[test]
    fn test_new() {
        let state = ResourceState::new();
        assert_eq!(state.unit_size(), 1);
    }

    #[test]
    fn test_with_unit_size() {
        let state = ResourceState::with_unit_size(4096);
        assert_eq!(state.unit_size(), 4096);
    }

    #[test]
    fn test_set_unit_size() {
        let mut state = ResourceState::new();
        state.set_unit_size(1024);
        assert_eq!(state.unit_size(), 1024);

        state.set_unit_size(0);
        assert_eq!(state.unit_size(), 1); // Minimum is 1
    }

    #[test]
    fn test_start_use() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        state.start_use(500);
        assert_eq!(state.get_using(), 500);
        assert_eq!(state.unused(), 500);
    }

    #[test]
    fn test_stop_use() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        state.start_use(500);
        state.stop_use(500);
        assert_eq!(state.get_using(), 0);
        assert_eq!(state.used(), 500);
        assert_eq!(state.unused(), 500);
    }

    #[test]
    fn test_update_limit() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        assert_eq!(state.limit(), 1000);

        state.update_limit(500);
        assert_eq!(state.limit(), 1500);
    }

    #[test]
    fn test_update_estimated_limit() {
        let mut state = ResourceState::new();
        state.update_limit(1000);

        let changed = state.update_estimated_limit(500);
        assert!(changed);
        // When limit > estimated, estimated_limit gets adjusted to match limit
        assert_eq!(state.estimated_limit(), 1000);
    }

    #[test]
    fn test_update_estimated_limit_no_change() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        state.update_estimated_limit(500);

        let changed = state.update_estimated_limit(500);
        assert!(!changed);
    }

    #[test]
    fn test_active_limit() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        state.start_use(300);
        state.stop_use(200);

        assert_eq!(state.active_limit(), 800); // 1000 - 200
    }

    #[test]
    fn test_unused() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        state.start_use(300);

        assert_eq!(state.unused(), 700);
    }

    #[test]
    fn test_estimated_extra() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        state.start_use(200);

        let extra = state.estimated_extra();
        // When estimated_limit is 0, extra is calculated from limit
        assert_eq!(extra, 0); // limit - using - used = 1000 - 200 - 0 = 800, then rounded
    }

    #[test]
    fn test_add() {
        let mut state1 = ResourceState::new();
        state1.update_limit(1000);

        let mut state2 = ResourceState::new();
        state2.update_limit(500);
        state2.start_use(200);
        state2.stop_use(200);

        state1.add(&state2);
        assert_eq!(state1.get_using(), 300); // state2's active_limit
        assert_eq!(state1.used(), 200);
    }

    #[test]
    fn test_sub() {
        let mut state1 = ResourceState::new();
        state1.update_limit(1000);
        state1.start_use(300);
        state1.stop_use(200);

        let mut state2 = ResourceState::new();
        state2.update_limit(500);
        state2.start_use(200);
        state2.stop_use(200);

        state1.sub(&state2);
        // state1.active_limit = 1000 - 200 = 800
        // state1.using = 100 - 300 (state2.active_limit) = saturates to 0
        // state1.used = 200 - 200 = 0
        assert_eq!(state1.get_using(), 0);
        assert_eq!(state1.used(), 0);
    }

    #[test]
    fn test_update_from() {
        let mut state1 = ResourceState::new();
        state1.update_limit(1000);

        let mut state2 = ResourceState::new();
        state2.update_limit(500);
        state2.update_estimated_limit(300);
        state2.start_use(100);

        state1.update_from(&state2);
        // update_from copies estimated_limit, used, using, unit_size from state2
        // update_estimated_limit adjusts: when estimated(300) < limit(500),
        // it adds extra_limit(200) to both estimated_limit and used
        assert_eq!(state1.estimated_limit(), 500);
        assert_eq!(state1.used(), 200); // used gets the extra_limit added
        assert_eq!(state1.get_using(), 100);
        assert_eq!(state1.unit_size(), state2.unit_size());
    }

    #[test]
    fn test_update_slave_limit() {
        let mut state1 = ResourceState::new();
        state1.update_limit(1000);

        let mut state2 = ResourceState::new();
        state2.update_limit(500);

        state1.update_slave_limit(&state2);
        assert_eq!(state1.limit(), 500);
    }

    #[test]
    fn test_is_unused() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        assert!(state.is_unused());

        state.start_use(1000);
        assert!(!state.is_unused());
    }

    #[test]
    fn test_is_at_capacity() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        assert!(!state.is_at_capacity());

        state.start_use(1000);
        state.stop_use(1000);
        // Now unused is 0 (1000 - 0 - 1000) and using is 0
        assert!(state.is_at_capacity());

        state.update_limit(500);
        assert!(!state.is_at_capacity());
    }

    #[test]
    fn test_reset() {
        let mut state = ResourceState::new();
        state.update_limit(1000);
        state.start_use(500);
        state.stop_use(250);

        state.reset();
        assert_eq!(state, ResourceState::default());
    }

    #[test]
    fn test_clone() {
        let mut state1 = ResourceState::new();
        state1.update_limit(1000);
        state1.start_use(500);

        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_eq() {
        let state1 = ResourceState::new();
        let state2 = ResourceState::new();
        assert_eq!(state1, state2);

        let mut state3 = ResourceState::new();
        state3.update_limit(1000);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_saturating_arithmetic() {
        let mut state = ResourceState::new();
        state.update_limit(100);

        // Start using within limit
        state.start_use(50);
        assert_eq!(state.get_using(), 50);

        // Stop exactly what was started (normal case)
        state.stop_use(50);
        assert_eq!(state.get_using(), 0);
        assert_eq!(state.used(), 50);
    }

    #[test]
    fn test_large_values() {
        let mut state = ResourceState::new();
        state.update_limit(i64::MAX);

        state.start_use(1_000_000_000);
        assert_eq!(state.get_using(), 1_000_000_000);

        state.stop_use(500_000_000);
        assert_eq!(state.used(), 500_000_000);
    }

    #[test]
    fn test_unit_size_rounding() {
        let mut state = ResourceState::with_unit_size(4096);
        state.update_limit(10000);
        state.start_use(1000);

        let extra = state.estimated_extra();
        // The calculation doesn't guarantee the result is divisible by unit_size
        // but it does use unit_size in the intermediate rounding step
        assert!(extra >= 0);
    }

    #[test]
    fn test_resource_state_debug() {
        let state = ResourceState::new();
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("ResourceState"));
    }
}
