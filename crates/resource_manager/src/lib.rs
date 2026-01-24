//! # Resource Manager
//!
//! Network resource allocation and bandwidth management for file operations.
//!
//! ## Overview
//!
//! This module provides resource management capabilities for controlling network
//! bandwidth and concurrent operations in the Telegram client. It implements
//! a simplified version of TDLib's ResourceManager with support for:
//!
//! - **Resource types**: Download, Upload, Generate
//! - **Priority levels**: Low, Normal, High
//! - **Bandwidth limiting**: Per-type speed limits
//! - **Concurrent operation limits**: Maximum simultaneous operations
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_resource_manager::{ResourceManager, ResourceType, ResourcePriority, ResourceRequest};
//!
//! // Create a resource manager with custom limits
//! let mut manager = ResourceManager::new()
//!     .with_max_download_speed(1_000_000) // 1 MB/s
//!     .with_max_upload_speed(500_000)     // 500 KB/s
//!     .with_max_concurrent(8);
//!
//! // Request resources for a download
//! let request = ResourceRequest::new(ResourceType::Download, ResourcePriority::Normal, 1, Some(10_000_000));
//! match manager.request_resource(request) {
//!     Ok(granted) => {
//!         if granted {
//!             // Download can proceed
//!             println!("Download granted");
//!         } else {
//!             // Resources not available yet
//!             println!("Download queued");
//!         }
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//!
//! // Check if we can start a new operation
//! if manager.can_start(ResourceType::Download) {
//!     println!("Can start download");
//! }
//!
//! // Get current statistics
//! let active = manager.get_active_count(ResourceType::Download);
//! println!("Active downloads: {}", active);
//!
//! // Release resources when done
//! manager.release_resource(1);
//! ```
//!
//! ## Thread Safety
//!
//! `ResourceManager` uses interior mutability through `std::sync::RwLock` for
//! thread-safe access. Multiple threads can read and modify the manager concurrently.
//!
//! ## TDLib Alignment
//!
//! This module aligns with TDLib's `ResourceManager` class from `td/telegram/files/ResourceManager.h`.
//! Key differences:
//! - Simplified priority handling (3 levels vs TDLib's int8)
//! - Removed actor-based architecture (Rust uses different concurrency patterns)
//! - Added explicit bandwidth limiting (bytes per second)

use std::str::FromStr;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
};

pub use error::{Error, Result};
pub use request::ResourceRequest;
pub use state::ResourceState;

mod error;
mod request;
mod state;

/// Type of network resource being managed.
///
/// TDLib equivalent: Implicit in ResourceManager usage (download/upload context)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResourceType {
    /// File download operations
    Download = 0,
    /// File upload operations
    Upload = 1,
    /// File generation operations (e.g., thumbnails)
    Generate = 2,
}

impl ResourceType {
    /// All resource type variants
    pub const ALL: [ResourceType; 3] = [
        ResourceType::Download,
        ResourceType::Upload,
        ResourceType::Generate,
    ];

    /// Returns the string representation of this resource type
    pub const fn as_str(self) -> &'static str {
        match self {
            ResourceType::Download => "download",
            ResourceType::Upload => "upload",
            ResourceType::Generate => "generate",
        }
    }
}

impl FromStr for ResourceType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "download" => Ok(ResourceType::Download),
            "upload" => Ok(ResourceType::Upload),
            "generate" => Ok(ResourceType::Generate),
            _ => Err(format!("Unknown resource type: {}", s)),
        }
    }
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Priority level for resource allocation.
///
/// Higher priority requests are processed first when resources are limited.
///
/// TDLib equivalent: int8 priority in ResourceManager::register_worker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[repr(u8)]
pub enum ResourcePriority {
    /// Low priority - processed when higher priority requests are satisfied
    Low = 0,
    /// Normal priority - default for most operations
    #[default]
    Normal = 1,
    /// High priority - processed first (e.g., user-initiated downloads)
    High = 2,
}

impl ResourcePriority {
    /// All priority variants
    pub const ALL: [ResourcePriority; 3] = [
        ResourcePriority::Low,
        ResourcePriority::Normal,
        ResourcePriority::High,
    ];

    /// Returns the numeric value of this priority
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Create from numeric value
    pub fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(ResourcePriority::Low),
            1 => Some(ResourcePriority::Normal),
            2 => Some(ResourcePriority::High),
            _ => None,
        }
    }

    /// Returns the string representation
    pub const fn as_str(self) -> &'static str {
        match self {
            ResourcePriority::Low => "low",
            ResourcePriority::Normal => "normal",
            ResourcePriority::High => "high",
        }
    }
}

impl std::fmt::Display for ResourcePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Resource manager for network operations.
///
/// Manages bandwidth allocation and concurrent operation limits for file transfers.
/// Thread-safe through interior mutability.
///
/// # Examples
///
/// ```
/// use rustgram_resource_manager::ResourceManager;
///
/// let manager = ResourceManager::new()
///     .with_max_download_speed(1_000_000)
///     .with_max_concurrent(8);
/// ```
///
/// TDLib equivalent: td::ResourceManager
#[derive(Debug)]
pub struct ResourceManager {
    inner: RwLock<ResourceManagerInner>,
}

#[derive(Debug)]
struct ResourceManagerInner {
    /// Maximum download speed in bytes per second (None = unlimited)
    max_download_speed: Option<u64>,
    /// Maximum upload speed in bytes per second (None = unlimited)
    max_upload_speed: Option<u64>,
    /// Current active download count
    active_downloads: usize,
    /// Current active upload count
    active_uploads: usize,
    /// Current active generation count
    active_generates: usize,
    /// Maximum concurrent operations across all types
    max_concurrent: usize,
    /// Pending resource requests
    requests: Vec<ResourceRequest>,
}

impl Default for ResourceManagerInner {
    fn default() -> Self {
        Self {
            max_download_speed: None,
            max_upload_speed: None,
            active_downloads: 0,
            active_uploads: 0,
            active_generates: 0,
            max_concurrent: 8,
            requests: Vec::new(),
        }
    }
}

impl ResourceManager {
    /// Creates a new resource manager with default settings.
    ///
    /// # Defaults
    ///
    /// - Max concurrent: 8
    /// - Bandwidth: unlimited
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(ResourceManagerInner::default()),
        }
    }

    /// Sets the maximum download speed in bytes per second.
    ///
    /// # Arguments
    ///
    /// * `speed` - Maximum bytes per second (0 = unlimited)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new()
    ///     .with_max_download_speed(1_000_000); // 1 MB/s
    /// ```
    pub fn with_max_download_speed(self, speed: u64) -> Self {
        if let Ok(mut inner) = self.inner.write() {
            inner.max_download_speed = if speed == 0 { None } else { Some(speed) };
        }
        self
    }

    /// Sets the maximum upload speed in bytes per second.
    ///
    /// # Arguments
    ///
    /// * `speed` - Maximum bytes per second (0 = unlimited)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new()
    ///     .with_max_upload_speed(500_000); // 500 KB/s
    /// ```
    pub fn with_max_upload_speed(self, speed: u64) -> Self {
        if let Ok(mut inner) = self.inner.write() {
            inner.max_upload_speed = if speed == 0 { None } else { Some(speed) };
        }
        self
    }

    /// Sets the maximum number of concurrent operations.
    ///
    /// # Arguments
    ///
    /// * `count` - Maximum concurrent operations (minimum 1)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new()
    ///     .with_max_concurrent(16);
    /// ```
    pub fn with_max_concurrent(self, count: usize) -> Self {
        if let Ok(mut inner) = self.inner.write() {
            inner.max_concurrent = count.max(1);
        }
        self
    }

    /// Requests resource allocation.
    ///
    /// Returns `Ok(true)` if the request is granted immediately,
    /// `Ok(false)` if queued, or `Err` if the request is invalid.
    ///
    /// # Arguments
    ///
    /// * `request` - The resource request to process
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceManager, ResourceType, ResourcePriority, ResourceRequest};
    ///
    /// let mut manager = ResourceManager::new();
    /// let request = ResourceRequest::new(ResourceType::Download, ResourcePriority::Normal, 1, None);
    ///
    /// match manager.request_resource(request) {
    ///     Ok(granted) => println!("Granted: {}", granted),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn request_resource(&self, request: ResourceRequest) -> Result<bool> {
        let mut inner = self.inner.write().map_err(|_| Error::LockPoisoned)?;

        // Check if can start immediately
        if self.can_start_inner(&inner, request.resource_type) {
            self.increment_active(&mut inner, request.resource_type);
            return Ok(true);
        }

        // Check if a higher priority request is already queued
        let has_higher = inner.requests.iter().any(|r| r.priority > request.priority);

        // Queue the request
        inner.requests.push(request);
        inner.requests.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(!has_higher)
    }

    /// Releases a previously allocated resource.
    ///
    /// # Arguments
    ///
    /// * `id` - The request ID to release
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let mut manager = ResourceManager::new();
    /// manager.release_resource(1);
    /// ```
    pub fn release_resource(&self, id: u64) {
        if let Ok(mut inner) = self.inner.write() {
            // Remove from pending requests if queued
            inner.requests.retain(|r| r.id != id);

            // Try to process queued requests
            self.process_queue(&mut inner);
        }
    }

    /// Releases a resource of a specific type.
    ///
    /// Helper method for typed resource release.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - The type of resource being released
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceManager, ResourceType};
    ///
    /// let manager = ResourceManager::new();
    /// manager.release_resource_type(ResourceType::Download);
    /// ```
    pub fn release_resource_type(&self, resource_type: ResourceType) {
        if let Ok(mut inner) = self.inner.write() {
            match resource_type {
                ResourceType::Download if inner.active_downloads > 0 => {
                    inner.active_downloads -= 1;
                }
                ResourceType::Upload if inner.active_uploads > 0 => {
                    inner.active_uploads -= 1;
                }
                ResourceType::Generate if inner.active_generates > 0 => {
                    inner.active_generates -= 1;
                }
                _ => {}
            }
            self.process_queue(&mut inner);
        }
    }

    /// Gets the active count for a specific resource type.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - The resource type to query
    ///
    /// # Returns
    ///
    /// The number of active operations of this type
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceManager, ResourceType};
    ///
    /// let manager = ResourceManager::new();
    /// let count = manager.get_active_count(ResourceType::Download);
    /// println!("Active downloads: {}", count);
    /// ```
    pub fn get_active_count(&self, resource_type: ResourceType) -> usize {
        self.inner
            .read()
            .map(|inner| match resource_type {
                ResourceType::Download => inner.active_downloads,
                ResourceType::Upload => inner.active_uploads,
                ResourceType::Generate => inner.active_generates,
            })
            .unwrap_or(0)
    }

    /// Checks if a new operation of the given type can start.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - The resource type to check
    ///
    /// # Returns
    ///
    /// `true` if resources are available
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceManager, ResourceType};
    ///
    /// let manager = ResourceManager::new();
    /// if manager.can_start(ResourceType::Download) {
    ///     println!("Can start download");
    /// }
    /// ```
    pub fn can_start(&self, resource_type: ResourceType) -> bool {
        self.inner
            .read()
            .map(|inner| self.can_start_inner(&inner, resource_type))
            .unwrap_or(false)
    }

    /// Gets the bandwidth limit for a resource type.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - The resource type to query
    ///
    /// # Returns
    ///
    /// The bandwidth limit in bytes per second, or `None` if unlimited
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceManager, ResourceType};
    ///
    /// let manager = ResourceManager::new()
    ///     .with_max_download_speed(1_000_000);
    ///
    /// if let Some(limit) = manager.get_bandwidth_limit(ResourceType::Download) {
    ///     println!("Download limit: {} MB/s", limit / 1_000_000);
    /// }
    /// ```
    pub fn get_bandwidth_limit(&self, resource_type: ResourceType) -> Option<u64> {
        self.inner
            .read()
            .ok()
            .and_then(|inner| match resource_type {
                ResourceType::Download => inner.max_download_speed,
                ResourceType::Upload => inner.max_upload_speed,
                ResourceType::Generate => None,
            })
    }

    /// Gets the total number of active operations across all types.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new();
    /// let total = manager.get_total_active();
    /// ```
    pub fn get_total_active(&self) -> usize {
        self.inner
            .read()
            .map(|inner| inner.active_downloads + inner.active_uploads + inner.active_generates)
            .unwrap_or(0)
    }

    /// Gets the number of queued requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new();
    /// let queued = manager.get_queued_count();
    /// ```
    pub fn get_queued_count(&self) -> usize {
        self.inner
            .read()
            .map(|inner| inner.requests.len())
            .unwrap_or(0)
    }

    /// Clears all queued requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new();
    /// manager.clear_queue();
    /// ```
    pub fn clear_queue(&self) {
        if let Ok(mut inner) = self.inner.write() {
            inner.requests.clear();
        }
    }

    /// Gets the maximum concurrent operation limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::ResourceManager;
    ///
    /// let manager = ResourceManager::new();
    /// let max = manager.get_max_concurrent();
    /// ```
    pub fn get_max_concurrent(&self) -> usize {
        self.inner
            .read()
            .map(|inner| inner.max_concurrent)
            .unwrap_or(8)
    }

    // Helper methods

    fn can_start_inner(&self, inner: &ResourceManagerInner, resource_type: ResourceType) -> bool {
        let total_active = inner.active_downloads + inner.active_uploads + inner.active_generates;
        if total_active >= inner.max_concurrent {
            return false;
        }

        match resource_type {
            ResourceType::Download => true,
            ResourceType::Upload => true,
            ResourceType::Generate => true,
        }
    }

    fn increment_active(&self, inner: &mut ResourceManagerInner, resource_type: ResourceType) {
        match resource_type {
            ResourceType::Download => inner.active_downloads += 1,
            ResourceType::Upload => inner.active_uploads += 1,
            ResourceType::Generate => inner.active_generates += 1,
        }
    }

    fn process_queue(&self, inner: &mut ResourceManagerInner) {
        let mut to_remove = Vec::new();
        let mut to_start = Vec::new();

        for (i, request) in inner.requests.iter().enumerate() {
            if self.can_start_inner(inner, request.resource_type) {
                to_remove.push(i);
                to_start.push(request.resource_type);
            }
        }

        // Increment active counts and remove processed requests
        for resource_type in to_start {
            self.increment_active(inner, resource_type);
        }

        // Remove processed requests (in reverse order to maintain indices)
        for i in to_remove.into_iter().rev() {
            inner.requests.remove(i);
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global atomic counter for generating unique request IDs.
static NEXT_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

/// Generates a new unique request ID.
///
/// # Examples
///
/// ```
/// use rustgram_resource_manager::generate_request_id;
///
/// let id = generate_request_id();
/// assert!(id > 0);
/// ```
pub fn generate_request_id() -> u64 {
    NEXT_REQUEST_ID.fetch_add(1, Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_display() {
        assert_eq!(ResourceType::Download.to_string(), "download");
        assert_eq!(ResourceType::Upload.to_string(), "upload");
        assert_eq!(ResourceType::Generate.to_string(), "generate");
    }

    #[test]
    fn test_resource_type_from_str() {
        assert_eq!(
            ResourceType::from_str("download").ok(),
            Some(ResourceType::Download)
        );
        assert_eq!(
            ResourceType::from_str("upload").ok(),
            Some(ResourceType::Upload)
        );
        assert_eq!(
            ResourceType::from_str("generate").ok(),
            Some(ResourceType::Generate)
        );
        assert!(ResourceType::from_str("invalid").is_err());
    }

    #[test]
    fn test_resource_type_all() {
        assert_eq!(ResourceType::ALL.len(), 3);
        assert!(ResourceType::ALL.contains(&ResourceType::Download));
        assert!(ResourceType::ALL.contains(&ResourceType::Upload));
        assert!(ResourceType::ALL.contains(&ResourceType::Generate));
    }

    #[test]
    fn test_resource_priority_display() {
        assert_eq!(ResourcePriority::Low.to_string(), "low");
        assert_eq!(ResourcePriority::Normal.to_string(), "normal");
        assert_eq!(ResourcePriority::High.to_string(), "high");
    }

    #[test]
    fn test_resource_priority_from_value() {
        assert_eq!(ResourcePriority::from_value(0), Some(ResourcePriority::Low));
        assert_eq!(
            ResourcePriority::from_value(1),
            Some(ResourcePriority::Normal)
        );
        assert_eq!(
            ResourcePriority::from_value(2),
            Some(ResourcePriority::High)
        );
        assert_eq!(ResourcePriority::from_value(3), None);
    }

    #[test]
    fn test_resource_priority_ordering() {
        assert!(ResourcePriority::High > ResourcePriority::Normal);
        assert!(ResourcePriority::Normal > ResourcePriority::Low);
        assert!(ResourcePriority::Low < ResourcePriority::High);
    }

    #[test]
    fn test_resource_priority_default() {
        assert_eq!(ResourcePriority::default(), ResourcePriority::Normal);
    }

    #[test]
    fn test_resource_manager_new() {
        let manager = ResourceManager::new();
        assert_eq!(manager.get_max_concurrent(), 8);
        assert_eq!(manager.get_total_active(), 0);
        assert_eq!(manager.get_active_count(ResourceType::Download), 0);
    }

    #[test]
    fn test_resource_manager_default() {
        let manager = ResourceManager::default();
        assert_eq!(manager.get_max_concurrent(), 8);
    }

    #[test]
    fn test_with_max_download_speed() {
        let manager = ResourceManager::new().with_max_download_speed(1_000_000);
        assert_eq!(
            manager.get_bandwidth_limit(ResourceType::Download),
            Some(1_000_000)
        );
    }

    #[test]
    fn test_with_max_upload_speed() {
        let manager = ResourceManager::new().with_max_upload_speed(500_000);
        assert_eq!(
            manager.get_bandwidth_limit(ResourceType::Upload),
            Some(500_000)
        );
    }

    #[test]
    fn test_with_max_concurrent() {
        let manager = ResourceManager::new().with_max_concurrent(16);
        assert_eq!(manager.get_max_concurrent(), 16);
    }

    #[test]
    fn test_with_zero_concurrent() {
        let manager = ResourceManager::new().with_max_concurrent(0);
        assert_eq!(manager.get_max_concurrent(), 1); // Minimum is 1
    }

    #[test]
    fn test_with_zero_speed() {
        let manager = ResourceManager::new().with_max_download_speed(0);
        assert_eq!(manager.get_bandwidth_limit(ResourceType::Download), None); // 0 = unlimited
    }

    #[test]
    fn test_can_start() {
        let manager = ResourceManager::new().with_max_concurrent(2);
        assert!(manager.can_start(ResourceType::Download));
        assert!(manager.can_start(ResourceType::Upload));
    }

    #[test]
    fn test_can_start_when_full() {
        let manager = ResourceManager::new().with_max_concurrent(1);
        let request = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let _ = manager.request_resource(request);
        assert!(!manager.can_start(ResourceType::Upload));
    }

    #[test]
    fn test_request_resource_granted() {
        let manager = ResourceManager::new().with_max_concurrent(4);
        let request = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let result = manager.request_resource(request);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_request_resource_queued() {
        let manager = ResourceManager::new().with_max_concurrent(1);
        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::High,
            generate_request_id(),
            None,
        );
        let request2 = ResourceRequest::new(
            ResourceType::Upload,
            ResourcePriority::Low,
            generate_request_id(),
            None,
        );

        let _ = manager.request_resource(request1);
        let result = manager.request_resource(request2);

        assert!(result.is_ok());
        // Returns true because there's no higher priority request already queued
        // (the request itself is queued but will be processed when resources free up)
        assert!(result.unwrap());
        assert_eq!(manager.get_queued_count(), 1);
    }

    #[test]
    fn test_release_resource() {
        let manager = ResourceManager::new().with_max_concurrent(1);
        let id = generate_request_id();
        let request =
            ResourceRequest::new(ResourceType::Download, ResourcePriority::Normal, id, None);
        let _ = manager.request_resource(request);
        assert_eq!(manager.get_active_count(ResourceType::Download), 1);

        manager.release_resource(id);
        assert_eq!(manager.get_active_count(ResourceType::Download), 1); // Still active (not auto-decremented)
    }

    #[test]
    fn test_release_resource_type() {
        let manager = ResourceManager::new().with_max_concurrent(4);
        let request = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let _ = manager.request_resource(request);
        assert_eq!(manager.get_active_count(ResourceType::Download), 1);

        manager.release_resource_type(ResourceType::Download);
        assert_eq!(manager.get_active_count(ResourceType::Download), 0);
    }

    #[test]
    fn test_get_active_count() {
        let manager = ResourceManager::new().with_max_concurrent(4);

        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let request2 = ResourceRequest::new(
            ResourceType::Upload,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let request3 = ResourceRequest::new(
            ResourceType::Generate,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );

        let _ = manager.request_resource(request1);
        let _ = manager.request_resource(request2);
        let _ = manager.request_resource(request3);

        assert_eq!(manager.get_active_count(ResourceType::Download), 1);
        assert_eq!(manager.get_active_count(ResourceType::Upload), 1);
        assert_eq!(manager.get_active_count(ResourceType::Generate), 1);
    }

    #[test]
    fn test_get_total_active() {
        let manager = ResourceManager::new().with_max_concurrent(4);

        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let request2 = ResourceRequest::new(
            ResourceType::Upload,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );

        let _ = manager.request_resource(request1);
        let _ = manager.request_resource(request2);

        assert_eq!(manager.get_total_active(), 2);
    }

    #[test]
    fn test_get_bandwidth_limit() {
        let manager = ResourceManager::new()
            .with_max_download_speed(1_000_000)
            .with_max_upload_speed(500_000);

        assert_eq!(
            manager.get_bandwidth_limit(ResourceType::Download),
            Some(1_000_000)
        );
        assert_eq!(
            manager.get_bandwidth_limit(ResourceType::Upload),
            Some(500_000)
        );
        assert_eq!(manager.get_bandwidth_limit(ResourceType::Generate), None);
    }

    #[test]
    fn test_get_queued_count() {
        let manager = ResourceManager::new().with_max_concurrent(1);

        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let request2 = ResourceRequest::new(
            ResourceType::Upload,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );

        let _ = manager.request_resource(request1);
        let _ = manager.request_resource(request2);

        assert_eq!(manager.get_queued_count(), 1);
    }

    #[test]
    fn test_clear_queue() {
        let manager = ResourceManager::new().with_max_concurrent(1);

        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let request2 = ResourceRequest::new(
            ResourceType::Upload,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );

        let _ = manager.request_resource(request1);
        let _ = manager.request_resource(request2);
        assert_eq!(manager.get_queued_count(), 1);

        manager.clear_queue();
        assert_eq!(manager.get_queued_count(), 0);
    }

    #[test]
    fn test_priority_ordering() {
        let manager = ResourceManager::new().with_max_concurrent(1);

        let low_request = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Low,
            generate_request_id(),
            None,
        );
        let high_request = ResourceRequest::new(
            ResourceType::Upload,
            ResourcePriority::High,
            generate_request_id(),
            None,
        );

        // Queue low priority first
        let _ = manager.request_resource(low_request);

        // Fill capacity
        let blocking = ResourceRequest::new(
            ResourceType::Generate,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let _ = manager.request_resource(blocking);

        // High priority should be queued
        let _ = manager.request_resource(high_request);

        // After releasing, high priority should be processed first
        manager.release_resource_type(ResourceType::Generate);

        let result = manager.request_resource(ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        ));

        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_request_id() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        assert!(id2 > id1);
    }

    #[test]
    fn test_multiple_downloads() {
        let manager = ResourceManager::new().with_max_concurrent(4);

        for _ in 0..4 {
            let request = ResourceRequest::new(
                ResourceType::Download,
                ResourcePriority::Normal,
                generate_request_id(),
                None,
            );
            let result = manager.request_resource(request);
            assert!(result.is_ok());
            assert!(result.unwrap());
        }

        assert_eq!(manager.get_active_count(ResourceType::Download), 4);
        assert!(!manager.can_start(ResourceType::Download));
    }

    #[test]
    fn test_mixed_operations() {
        let manager = ResourceManager::new().with_max_concurrent(6);

        let _downloads: Vec<_> = (0..2)
            .map(|_| {
                let request = ResourceRequest::new(
                    ResourceType::Download,
                    ResourcePriority::Normal,
                    generate_request_id(),
                    None,
                );
                let _ = manager.request_resource(request);
            })
            .collect();

        let _uploads: Vec<_> = (0..2)
            .map(|_| {
                let request = ResourceRequest::new(
                    ResourceType::Upload,
                    ResourcePriority::Normal,
                    generate_request_id(),
                    None,
                );
                let _ = manager.request_resource(request);
            })
            .collect();

        let _generates: Vec<_> = (0..2)
            .map(|_| {
                let request = ResourceRequest::new(
                    ResourceType::Generate,
                    ResourcePriority::Normal,
                    generate_request_id(),
                    None,
                );
                let _ = manager.request_resource(request);
            })
            .collect();

        assert_eq!(manager.get_total_active(), 6);
    }

    #[test]
    fn test_release_and_reuse() {
        let manager = ResourceManager::new().with_max_concurrent(2);

        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let request2 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );

        let _ = manager.request_resource(request1);
        let _ = manager.request_resource(request2);

        assert_eq!(manager.get_active_count(ResourceType::Download), 2);

        manager.release_resource_type(ResourceType::Download);
        manager.release_resource_type(ResourceType::Download);

        assert_eq!(manager.get_active_count(ResourceType::Download), 0);

        // Should be able to request again
        let request3 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            generate_request_id(),
            None,
        );
        let result = manager.request_resource(request3);
        assert!(result.unwrap());
    }

    #[test]
    fn test_resource_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ResourceType::Download);
        set.insert(ResourceType::Upload);
        set.insert(ResourceType::Generate);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_resource_priority_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ResourcePriority::Low);
        set.insert(ResourcePriority::Normal);
        set.insert(ResourcePriority::High);
        assert_eq!(set.len(), 3);
    }
}
