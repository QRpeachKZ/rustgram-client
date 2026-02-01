//! # File Load Manager
//!
//! Priority-based file load request queue with resource manager integration.
//!
//! ## Overview
//!
//! This module provides a lightweight coordinator for file load operations,
//! implementing TDLib-style priority scheduling with resource allocation.
//! It manages pending and active load requests, ensuring optimal resource
//! utilization based on operation priority.
//!
//! ## Architecture
//!
//! - **Pending queue**: Priority-ordered requests waiting for resources
//! - **Active set**: Currently processing requests with allocated resources
//! - **State machine**: Pending → Active → Complete
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_file_load_manager::FileLoadManager;
//! use rustgram_resource_manager::{ResourceManager, ResourcePriority};
//!
//! let resources = std::sync::Arc::new(ResourceManager::new());
//! let mut manager = FileLoadManager::new(resources);
//!
//! // Queue load requests with priority
//! manager.queue(1, ResourcePriority::High, Some(10_000_000));
//! manager.queue(2, ResourcePriority::Normal, Some(5_000_000));
//!
//! // Schedule pending requests based on available resources
//! manager.schedule();
//!
//! // Complete a load operation
//! let result = manager.complete(1);
//! ```
//!
//! ## Thread Safety
//!
//! `FileLoadManager` uses `Arc<ResourceManager>` for resource sharing.
//! The manager itself is not thread-safe and should be used within
//! a single thread or synchronized externally.

#![warn(missing_docs)]

mod error;
mod request;
mod state;

pub use error::{Error, Result};
pub use request::LoadRequest;
pub use state::LoadState;

use rustgram_resource_manager::{ResourceManager, ResourcePriority, ResourceType};
use std::sync::Arc;

/// File load manager with priority-based scheduling.
///
/// Coordinates file load operations through a priority queue and
/// integrates with ResourceManager for resource allocation.
///
/// TDLib equivalent: td::telegram::FileLoadManager
#[derive(Debug)]
pub struct FileLoadManager {
    /// Resource manager for allocation control.
    resources: Arc<ResourceManager>,
    /// Priority-sorted pending requests.
    pending: Vec<LoadRequest>,
    /// Currently processing requests.
    active: Vec<LoadRequest>,
}

impl FileLoadManager {
    /// Creates a new file load manager.
    ///
    /// # Arguments
    ///
    /// * `resources` - Resource manager for allocation control
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_load_manager::FileLoadManager;
    /// use rustgram_resource_manager::ResourceManager;
    /// use std::sync::Arc;
    ///
    /// let resources = Arc::new(ResourceManager::new());
    /// let manager = FileLoadManager::new(resources);
    /// ```
    pub fn new(resources: Arc<ResourceManager>) -> Self {
        Self {
            resources,
            pending: Vec::new(),
            active: Vec::new(),
        }
    }

    /// Queues a new load request.
    ///
    /// Adds the request to the pending queue and sorts by priority.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier for the file
    /// * `priority` - Priority for resource allocation
    /// * `size` - Optional size for bandwidth estimation
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_load_manager::FileLoadManager;
    /// use rustgram_resource_manager::{ResourceManager, ResourcePriority};
    /// use std::sync::Arc;
    ///
    /// let resources = Arc::new(ResourceManager::new());
    /// let mut manager = FileLoadManager::new(resources);
    /// manager.queue(1, ResourcePriority::High, Some(10_000));
    /// ```
    pub fn queue(&mut self, file_id: u64, priority: ResourcePriority, size: Option<u64>) {
        let request = LoadRequest::new(file_id, priority, size);
        self.pending.push(request);
        self.pending.sort_by_key(|r| std::cmp::Reverse(r.priority));
    }

    /// Schedules pending requests based on available resources.
    ///
    /// Processes the pending queue in priority order, starting requests
    /// that have available resources. Moves started requests to active list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_load_manager::FileLoadManager;
    /// use rustgram_resource_manager::{ResourceManager, ResourcePriority};
    /// use std::sync::Arc;
    ///
    /// let resources = Arc::new(ResourceManager::new());
    /// let mut manager = FileLoadManager::new(resources);
    /// manager.queue(1, ResourcePriority::High, None);
    /// manager.schedule();
    /// ```
    pub fn schedule(&mut self) {
        let mut started = Vec::new();
        let mut remaining = Vec::new();

        // Process all pending requests
        for mut request in self.pending.drain(..) {
            // Check if we can start this request
            if self.resources.can_start(ResourceType::Download) {
                // Simulate the allocation by tracking active count through ResourceManager
                // We use a dummy request to increment the counter
                let _ = self.resources.request_resource(
                    rustgram_resource_manager::ResourceRequest::new(
                        ResourceType::Download,
                        request.priority,
                        request.file_id,
                        request.size,
                    ),
                );
                request.state = LoadState::Active;
                started.push(request);
            } else {
                remaining.push(request);
            }
        }

        // Update state
        self.pending = remaining;
        self.pending.sort_by_key(|r| std::cmp::Reverse(r.priority));
        self.active.extend(started);
    }

    /// Marks a load request as complete.
    ///
    /// Releases resources and removes the request from active list.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID to complete
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the request was found and completed
    /// - `Err(Error::NotFound)` if the file_id was not in active list
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_load_manager::FileLoadManager;
    /// use rustgram_resource_manager::ResourceManager;
    /// use std::sync::Arc;
    ///
    /// let resources = Arc::new(ResourceManager::new());
    /// let mut manager = FileLoadManager::new(resources);
    /// // ... queue and schedule ...
    /// let result = manager.complete(1);
    /// ```
    pub fn complete(&mut self, file_id: u64) -> Result<()> {
        let pos = self
            .active
            .iter()
            .position(|r| r.file_id == file_id)
            .ok_or(Error::NotFound(file_id))?;

        self.active.remove(pos);
        self.resources.release_resource_type(ResourceType::Download);
        Ok(())
    }

    /// Returns the number of pending requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_load_manager::FileLoadManager;
    /// use rustgram_resource_manager::{ResourceManager, ResourcePriority};
    /// use std::sync::Arc;
    ///
    /// let resources = Arc::new(ResourceManager::new());
    /// let mut manager = FileLoadManager::new(resources);
    /// manager.queue(1, ResourcePriority::High, None);
    /// assert_eq!(manager.pending_count(), 1);
    /// ```
    #[must_use]
    pub const fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Returns the number of active requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_load_manager::FileLoadManager;
    /// use rustgram_resource_manager::ResourceManager;
    /// use std::sync::Arc;
    ///
    /// let resources = Arc::new(ResourceManager::new());
    /// let manager = FileLoadManager::new(resources);
    /// assert_eq!(manager.active_count(), 0);
    /// ```
    #[must_use]
    pub const fn active_count(&self) -> usize {
        self.active.len()
    }
}

impl Default for FileLoadManager {
    fn default() -> Self {
        Self::new(Arc::new(ResourceManager::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_resource_manager::ResourcePriority;

    fn create_test_manager() -> FileLoadManager {
        let resources = Arc::new(ResourceManager::new().with_max_concurrent(2));
        FileLoadManager::new(resources)
    }

    // Construction tests (2)

    #[test]
    fn test_new() {
        let resources = Arc::new(ResourceManager::new());
        let manager = FileLoadManager::new(resources.clone());
        assert!(manager.pending.is_empty());
        assert!(manager.active.is_empty());
    }

    #[test]
    fn test_default() {
        let manager = FileLoadManager::default();
        assert_eq!(manager.pending_count(), 0);
        assert_eq!(manager.active_count(), 0);
    }

    // Queue operations tests (3)

    #[test]
    fn test_queue_single() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::Normal, Some(1024));

        assert_eq!(manager.pending_count(), 1);
        assert_eq!(manager.pending[0].file_id, 1);
        assert_eq!(manager.pending[0].priority, ResourcePriority::Normal);
        assert_eq!(manager.pending[0].size, Some(1024));
        assert!(manager.pending[0].is_pending());
    }

    #[test]
    fn test_queue_multiple() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::Low, None);
        manager.queue(2, ResourcePriority::High, Some(2048));
        manager.queue(3, ResourcePriority::Normal, None);

        assert_eq!(manager.pending_count(), 3);
    }

    #[test]
    fn test_priority_sorting() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::Low, None);
        manager.queue(2, ResourcePriority::High, None);
        manager.queue(3, ResourcePriority::Normal, None);

        // Should be sorted: High, Normal, Low
        assert_eq!(manager.pending[0].priority, ResourcePriority::High);
        assert_eq!(manager.pending[1].priority, ResourcePriority::Normal);
        assert_eq!(manager.pending[2].priority, ResourcePriority::Low);
    }

    // Scheduling tests (5)

    #[test]
    fn test_schedule_with_resources() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::High, None);

        manager.schedule();

        // Should be moved to active
        assert_eq!(manager.pending_count(), 0);
        assert_eq!(manager.active_count(), 1);
        assert!(manager.active[0].is_active());
    }

    #[test]
    fn test_schedule_no_resources() {
        // Note: ResourceManager forces max_concurrent >= 1, so we use limit 1
        // and queue 2 requests to test queuing behavior
        let resources = Arc::new(ResourceManager::new().with_max_concurrent(1));
        let mut manager = FileLoadManager::new(resources);
        manager.queue(1, ResourcePriority::High, None);
        manager.queue(2, ResourcePriority::Normal, None);

        manager.schedule();

        // Should have 1 active (at limit), 1 pending
        assert_eq!(manager.pending_count(), 1);
        assert_eq!(manager.active_count(), 1);
        assert!(manager.pending[0].is_pending());
    }

    #[test]
    fn test_schedule_high_priority_first() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::Low, None);
        manager.queue(2, ResourcePriority::High, None);
        manager.queue(3, ResourcePriority::Normal, None);

        manager.schedule();

        // High priority should be active, others pending (due to limit of 2)
        assert!(manager.active.iter().any(|r| r.file_id == 2));
        assert!(manager.pending.iter().all(|r| r.file_id != 2));
    }

    #[test]
    fn test_schedule_multiple() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::High, None);
        manager.queue(2, ResourcePriority::High, None);
        manager.queue(3, ResourcePriority::Low, None);

        manager.schedule();

        // Should have 2 active (limit), 1 pending
        assert_eq!(manager.active_count(), 2);
        assert_eq!(manager.pending_count(), 1);
    }

    #[test]
    fn test_schedule_empty_queue() {
        let mut manager = create_test_manager();
        manager.schedule();

        assert_eq!(manager.pending_count(), 0);
        assert_eq!(manager.active_count(), 0);
    }

    // State transition tests (4)

    #[test]
    fn test_start_updates_state() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::High, None);
        assert!(manager.pending[0].is_pending());

        manager.schedule();

        assert!(manager.active[0].is_active());
        assert!(!manager.active[0].is_pending());
    }

    #[test]
    fn test_complete_removes_from_active() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::High, None);
        manager.schedule();
        assert_eq!(manager.active_count(), 1);

        let result = manager.complete(1);

        assert!(result.is_ok());
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_complete_releases_resources() {
        let resources = Arc::new(ResourceManager::new().with_max_concurrent(1));
        let mut manager = FileLoadManager::new(resources.clone());

        manager.queue(1, ResourcePriority::High, None);
        manager.schedule();
        assert!(!resources.can_start(ResourceType::Download));

        manager.complete(1).unwrap();

        // Resources should be available again
        assert!(resources.can_start(ResourceType::Download));
    }

    #[test]
    fn test_state_transitions() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::High, None);

        // Pending state
        assert!(manager.pending[0].is_pending());

        // Active state
        manager.schedule();
        assert!(manager.active[0].is_active());

        // Complete state (removed from manager)
        manager.complete(1).unwrap();
        assert!(!manager.active.iter().any(|r| r.file_id == 1));
    }

    // Count method tests (3)

    #[test]
    fn test_pending_count() {
        let mut manager = create_test_manager();
        assert_eq!(manager.pending_count(), 0);

        manager.queue(1, ResourcePriority::High, None);
        assert_eq!(manager.pending_count(), 1);

        manager.queue(2, ResourcePriority::Normal, None);
        assert_eq!(manager.pending_count(), 2);
    }

    #[test]
    fn test_active_count() {
        let mut manager = create_test_manager();
        assert_eq!(manager.active_count(), 0);

        manager.queue(1, ResourcePriority::High, None);
        manager.queue(2, ResourcePriority::High, None);
        manager.schedule();

        assert_eq!(manager.active_count(), 2);
    }

    #[test]
    fn test_counts_after_operations() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::High, None);
        manager.queue(2, ResourcePriority::High, None);
        manager.queue(3, ResourcePriority::Low, None);

        assert_eq!(manager.pending_count(), 3);
        assert_eq!(manager.active_count(), 0);

        manager.schedule();

        assert_eq!(manager.active_count(), 2);
        assert_eq!(manager.pending_count(), 1);

        manager.complete(1).unwrap();

        assert_eq!(manager.active_count(), 1);
    }

    // Error handling tests (2)

    #[test]
    fn test_complete_nonexistent() {
        let mut manager = create_test_manager();
        let result = manager.complete(999);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NotFound(999));
    }

    #[test]
    fn test_start_nonexistent() {
        let mut manager = create_test_manager();
        manager.queue(1, ResourcePriority::High, None);

        // Schedule processes all pending, so after this
        // file_id 2 should not exist
        manager.schedule();
        let result = manager.complete(2);

        assert!(result.is_err());
    }

    // Integration test (1)

    #[test]
    fn test_resource_manager_integration() {
        let resources = Arc::new(ResourceManager::new().with_max_concurrent(1));
        let mut manager = FileLoadManager::new(resources.clone());

        // Queue two requests
        manager.queue(1, ResourcePriority::High, None);
        manager.queue(2, ResourcePriority::Normal, None);

        // Only one should start
        manager.schedule();
        assert_eq!(manager.active_count(), 1);
        assert!(!resources.can_start(ResourceType::Download));

        // Complete first, second can start
        manager.complete(1).unwrap();
        assert!(resources.can_start(ResourceType::Download));

        manager.schedule();
        assert_eq!(manager.active_count(), 1);
    }
}
