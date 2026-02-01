// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # PTS Manager
//!
//! Manages PTS (Pending Timestamp) state for Telegram MTProto.
//!
//! Based on TDLib's `PtsManager` from `td/telegram/PtsManager.h`.
//!
//! # Overview
//!
//! The `PtsManager` is responsible for tracking and managing the PTS state,
//! which represents the processed message sequence. It uses a `ChangesProcessor`
//! internally to ensure that updates are applied in the correct order even if
//! they complete out of order.
//!
//! **Note:** This is not about handling gaps. It's about finding the current
//! processed PTS. All checks must be done before using this manager.
//!
//! # Example
//!
//! ```rust
//! use rustgram_pts_manager::PtsManager;
//!
//! let mut manager = PtsManager::new();
//!
//! // Initialize with a PTS value
//! manager.init(100);
//!
//! // Add pending PTS updates
//! let id1 = manager.add_pts(101);
//! let id2 = manager.add_pts(102);
//!
//! // Finish updates - they'll be processed in order
//! let db_pts = manager.finish(id2);
//! assert_eq!(db_pts, 100); // id1 not finished yet
//!
//! let db_pts = manager.finish(id1);
//! assert_eq!(db_pts, 102); // Both processed
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_changes_processor::{ChangesProcessor, ChangesProcessorId};
use serde::{Deserialize, Serialize};

/// PTS manager for tracking message sequence state.
///
/// Manages the database PTS (persisted) and memory PTS (current) values,
/// using a ChangesProcessor to ensure updates are applied in order.
///
/// # Example
///
/// ```rust
/// use rustgram_pts_manager::PtsManager;
///
/// let mut manager = PtsManager::new();
///
/// // Initialize with PTS from database
/// manager.init(50);
///
/// let id = manager.add_pts(51);
/// manager.finish(id);
///
/// assert_eq!(manager.db_pts(), 51);
/// assert_eq!(manager.mem_pts(), 51);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtsManager {
    /// Database PTS value (persisted).
    db_pts: i32,
    /// Memory PTS value (current).
    mem_pts: i32,
    /// Whether the manager has been initialized.
    #[serde(default = "default_initialized")]
    initialized: bool,
    /// Changes processor for ordered updates.
    #[serde(skip)]
    state_helper: ChangesProcessor<i32>,
}

/// Default value for the `initialized` field in serde.
fn default_initialized() -> bool {
    false
}

impl Default for PtsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PtsManager {
    /// Creates a new `PtsManager` with default values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let manager = PtsManager::new();
    /// assert_eq!(manager.db_pts(), -1);
    /// assert_eq!(manager.mem_pts(), -1);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            db_pts: -1,
            mem_pts: -1,
            initialized: false,
            state_helper: ChangesProcessor::new(),
        }
    }

    /// Initializes the manager with a PTS value.
    ///
    /// This should be called once with the initial PTS value, typically
    /// loaded from persistent storage.
    ///
    /// # Arguments
    ///
    /// * `pts` - The initial PTS value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let mut manager = PtsManager::new();
    /// manager.init(100);
    ///
    /// assert_eq!(manager.db_pts(), 100);
    /// assert_eq!(manager.mem_pts(), 100);
    /// ```
    pub fn init(&mut self, pts: i32) {
        self.db_pts = pts;
        self.mem_pts = pts;
        self.initialized = true;
        self.state_helper.clear();
    }

    /// Adds a pending PTS update.
    ///
    /// Returns an ID that can be used later to finish this update.
    /// If `pts` is positive, it also updates the memory PTS.
    ///
    /// # Arguments
    ///
    /// * `pts` - The PTS value for this update
    ///
    /// # Returns
    ///
    /// An ID that can be used with `finish()` to complete this update
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let mut manager = PtsManager::new();
    /// manager.init(100);
    ///
    /// let id = manager.add_pts(101);
    /// // id can be used later with finish()
    /// ```
    pub fn add_pts(&mut self, pts: i32) -> ChangesProcessorId {
        if pts > 0 {
            self.mem_pts = pts;
        }
        self.state_helper.add(pts)
    }

    /// Finishes a PTS update and processes any now-ready entries.
    ///
    /// When an entry is finished and becomes ready to process, it updates
    /// the database PTS if the value is non-zero.
    ///
    /// # Arguments
    ///
    /// * `pts_id` - The ID returned from `add_pts()`
    ///
    /// # Returns
    ///
    /// The current database PTS value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let mut manager = PtsManager::new();
    /// manager.init(100);
    ///
    /// let id1 = manager.add_pts(101);
    /// let id2 = manager.add_pts(102);
    ///
    /// // Finish out of order - db_pts only updates when consecutive
    /// manager.finish(id2);
    /// assert_eq!(manager.db_pts(), 100);
    ///
    /// manager.finish(id1);
    /// assert_eq!(manager.db_pts(), 102);
    /// ```
    pub fn finish(&mut self, pts_id: ChangesProcessorId) -> i32 {
        self.state_helper.finish(pts_id, |pts| {
            if pts != 0 {
                self.db_pts = pts;
            }
        });
        self.db_pts
    }

    /// Returns the current database PTS value.
    ///
    /// This is the persisted PTS value that has been confirmed and processed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let manager = PtsManager::new();
    /// assert_eq!(manager.db_pts(), -1);
    /// ```
    #[must_use]
    pub const fn db_pts(&self) -> i32 {
        self.db_pts
    }

    /// Returns the current memory PTS value.
    ///
    /// This is the latest PTS value seen, which may be ahead of `db_pts()`
    /// if there are pending updates.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let mut manager = PtsManager::new();
    /// manager.init(100);
    ///
    /// manager.add_pts(105);
    /// assert_eq!(manager.mem_pts(), 105); // Updated immediately
    /// ```
    #[must_use]
    pub const fn mem_pts(&self) -> i32 {
        self.mem_pts
    }

    /// Checks if the manager has been initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let manager = PtsManager::new();
    /// assert!(!manager.is_initialized());
    ///
    /// let mut manager = PtsManager::new();
    /// manager.init(0);
    /// assert!(manager.is_initialized());
    /// ```
    #[must_use]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Resets the manager to uninitialized state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_pts_manager::PtsManager;
    ///
    /// let mut manager = PtsManager::new();
    /// manager.init(100);
    /// assert!(manager.is_initialized());
    ///
    /// manager.reset();
    /// assert!(!manager.is_initialized());
    /// ```
    pub fn reset(&mut self) {
        self.db_pts = -1;
        self.mem_pts = -1;
        self.initialized = false;
        self.state_helper.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manager = PtsManager::new();
        assert_eq!(manager.db_pts(), -1);
        assert_eq!(manager.mem_pts(), -1);
        assert!(!manager.is_initialized());
    }

    #[test]
    fn test_default() {
        let manager = PtsManager::default();
        assert_eq!(manager.db_pts(), -1);
        assert_eq!(manager.mem_pts(), -1);
    }

    #[test]
    fn test_init() {
        let mut manager = PtsManager::new();
        manager.init(100);

        assert_eq!(manager.db_pts(), 100);
        assert_eq!(manager.mem_pts(), 100);
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_init_zero() {
        let mut manager = PtsManager::new();
        manager.init(0);

        assert_eq!(manager.db_pts(), 0);
        assert_eq!(manager.mem_pts(), 0);
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_init_negative() {
        let mut manager = PtsManager::new();
        manager.init(-1);

        assert_eq!(manager.db_pts(), -1);
        assert_eq!(manager.mem_pts(), -1);
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_add_pts_positive() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let _id = manager.add_pts(101);
        assert_eq!(manager.mem_pts(), 101);
    }

    #[test]
    fn test_add_pts_zero() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let _id = manager.add_pts(0);
        assert_eq!(manager.mem_pts(), 100); // Unchanged
    }

    #[test]
    fn test_add_pts_negative() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let _id = manager.add_pts(-1);
        assert_eq!(manager.mem_pts(), 100); // Unchanged
    }

    #[test]
    fn test_finish_in_order() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let id1 = manager.add_pts(101);
        let id2 = manager.add_pts(102);
        let id3 = manager.add_pts(103);

        let db_pts = manager.finish(id1);
        assert_eq!(db_pts, 101);

        let db_pts = manager.finish(id2);
        assert_eq!(db_pts, 102);

        let db_pts = manager.finish(id3);
        assert_eq!(db_pts, 103);
    }

    #[test]
    fn test_finish_out_of_order() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let id1 = manager.add_pts(101);
        let id2 = manager.add_pts(102);
        let id3 = manager.add_pts(103);

        // Finish in reverse order
        let db_pts = manager.finish(id3);
        assert_eq!(db_pts, 100); // Nothing ready yet

        let db_pts = manager.finish(id2);
        assert_eq!(db_pts, 100); // id1 still blocking

        let db_pts = manager.finish(id1);
        assert_eq!(db_pts, 103); // All processed
    }

    #[test]
    fn test_finish_middle_first() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let id1 = manager.add_pts(101);
        let id2 = manager.add_pts(102);
        let id3 = manager.add_pts(103);

        let db_pts = manager.finish(id2);
        assert_eq!(db_pts, 100);

        let db_pts = manager.finish(id3);
        assert_eq!(db_pts, 100); // id1 still blocking

        let db_pts = manager.finish(id1);
        assert_eq!(db_pts, 103);
    }

    #[test]
    fn test_finish_with_zero_pts() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let id1 = manager.add_pts(101);
        let id2 = manager.add_pts(0); // Zero PTS should not update db_pts
        let id3 = manager.add_pts(102);

        manager.finish(id1);
        manager.finish(id2);
        let db_pts = manager.finish(id3);

        assert_eq!(db_pts, 102); // Zero was skipped
    }

    #[test]
    fn test_is_initialized() {
        let manager = PtsManager::new();
        assert!(!manager.is_initialized());

        let mut manager = PtsManager::new();
        manager.init(0);
        assert!(manager.is_initialized());

        let mut manager = PtsManager::new();
        manager.init(-1);
        assert!(manager.is_initialized());

        let mut manager = PtsManager::new();
        manager.init(100);
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_reset() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let _id = manager.add_pts(101);
        assert!(manager.is_initialized());
        assert_eq!(manager.mem_pts(), 101);

        manager.reset();
        assert!(!manager.is_initialized());
        assert_eq!(manager.db_pts(), -1);
        assert_eq!(manager.mem_pts(), -1);
    }

    #[test]
    fn test_multiple_finish_same_id() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let id = manager.add_pts(101);

        manager.finish(id);
        let db_pts = manager.finish(id); // Finishing same ID again
        assert_eq!(db_pts, 101);
    }

    #[test]
    fn test_large_pts_values() {
        let mut manager = PtsManager::new();
        manager.init(1_000_000);

        let id = manager.add_pts(1_000_001);
        let db_pts = manager.finish(id);

        assert_eq!(db_pts, 1_000_001);
        assert_eq!(manager.mem_pts(), 1_000_001);
    }

    #[test]
    fn test_mem_pts_updates_immediately() {
        let mut manager = PtsManager::new();
        manager.init(100);

        manager.add_pts(105);
        assert_eq!(manager.mem_pts(), 105);

        manager.add_pts(110);
        assert_eq!(manager.mem_pts(), 110);

        manager.add_pts(115);
        assert_eq!(manager.mem_pts(), 115);
    }

    #[test]
    fn test_db_pts_updates_only_on_finish() {
        let mut manager = PtsManager::new();
        manager.init(100);

        let id = manager.add_pts(200);
        assert_eq!(manager.db_pts(), 100); // Not updated yet
        assert_eq!(manager.mem_pts(), 200); // Updated immediately

        manager.finish(id);
        assert_eq!(manager.db_pts(), 200); // Now updated
    }

    #[test]
    fn test_serialization() {
        let mut manager = PtsManager::new();
        manager.init(100);

        // Note: ChangesProcessor is skipped during serialization
        let json = serde_json::to_string(&manager).expect("Failed to serialize");
        assert!(json.contains("100"));

        let deserialized: PtsManager = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.db_pts(), 100);
        assert_eq!(deserialized.mem_pts(), 100);
    }

    #[test]
    fn test_clone() {
        let mut manager1 = PtsManager::new();
        manager1.init(100);

        let id = manager1.add_pts(101);

        let manager2 = manager1.clone();

        // Both have same state
        assert_eq!(manager1.db_pts(), manager2.db_pts());
        assert_eq!(manager1.mem_pts(), manager2.mem_pts());

        // But independent ChangesProcessor state
        manager1.finish(id);
        assert_eq!(manager1.db_pts(), 101);
    }
}
