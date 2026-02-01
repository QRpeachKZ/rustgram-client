use crate::state::LoadState;
use rustgram_resource_manager::ResourcePriority;

/// Request wrapper for file load operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadRequest {
    /// Unique identifier for the file.
    pub file_id: u64,
    /// Priority for resource allocation.
    pub priority: ResourcePriority,
    /// Current state of the load operation.
    pub state: LoadState,
    /// Optional size for bandwidth estimation.
    pub size: Option<u64>,
}

impl LoadRequest {
    /// Create a new load request.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier for the file.
    /// * `priority` - Priority for resource allocation.
    /// * `size` - Optional size for bandwidth estimation.
    #[must_use]
    pub const fn new(file_id: u64, priority: ResourcePriority, size: Option<u64>) -> Self {
        Self {
            file_id,
            priority,
            state: LoadState::Pending,
            size,
        }
    }

    /// Check if the request is in pending state.
    #[must_use]
    pub fn is_pending(&self) -> bool {
        self.state == LoadState::Pending
    }

    /// Check if the request is in active state.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.state == LoadState::Active
    }

    /// Check if the request is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.state == LoadState::Complete
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let request = LoadRequest::new(123, ResourcePriority::High, Some(1024));
        assert_eq!(request.file_id, 123);
        assert_eq!(request.priority, ResourcePriority::High);
        assert_eq!(request.size, Some(1024));
        assert_eq!(request.state, LoadState::Pending);
    }

    #[test]
    fn test_new_without_size() {
        let request = LoadRequest::new(456, ResourcePriority::Low, None);
        assert_eq!(request.file_id, 456);
        assert_eq!(request.priority, ResourcePriority::Low);
        assert_eq!(request.size, None);
        assert_eq!(request.state, LoadState::Pending);
    }

    #[test]
    fn test_is_pending() {
        let request = LoadRequest::new(1, ResourcePriority::Normal, None);
        assert!(request.is_pending());
        assert!(!request.is_active());
        assert!(!request.is_complete());
    }

    #[test]
    fn test_is_active() {
        let mut request = LoadRequest::new(1, ResourcePriority::Normal, None);
        request.state = LoadState::Active;
        assert!(!request.is_pending());
        assert!(request.is_active());
        assert!(!request.is_complete());
    }

    #[test]
    fn test_is_complete() {
        let mut request = LoadRequest::new(1, ResourcePriority::Normal, None);
        request.state = LoadState::Complete;
        assert!(!request.is_pending());
        assert!(!request.is_active());
        assert!(request.is_complete());
    }

    #[test]
    fn test_equality() {
        let req1 = LoadRequest::new(123, ResourcePriority::High, Some(1024));
        let req2 = LoadRequest::new(123, ResourcePriority::High, Some(1024));
        assert_eq!(req1, req2);
    }

    #[test]
    fn test_inequality() {
        let req1 = LoadRequest::new(123, ResourcePriority::High, Some(1024));
        let req2 = LoadRequest::new(456, ResourcePriority::High, Some(1024));
        assert_ne!(req1, req2);
    }

    #[test]
    fn test_clone() {
        let req1 = LoadRequest::new(123, ResourcePriority::High, Some(1024));
        let req2 = req1.clone();
        assert_eq!(req1, req2);
    }
}
