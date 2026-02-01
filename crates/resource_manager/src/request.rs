//! Resource request types.

use crate::{generate_request_id, ResourcePriority, ResourceType};

/// A resource allocation request.
///
/// Represents a request for network resources with specific priority and size.
///
/// # Examples
///
/// ```
/// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
///
/// let request = ResourceRequest::new(
///     ResourceType::Download,
///     ResourcePriority::Normal,
///     1,
///     Some(10_000_000)
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceRequest {
    /// The type of resource being requested
    pub resource_type: ResourceType,
    /// Priority level for this request
    pub priority: ResourcePriority,
    /// Unique identifier for this request
    pub id: u64,
    /// Optional size in bytes (for bandwidth estimation)
    pub size: Option<u64>,
}

impl ResourceRequest {
    /// Creates a new resource request.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - The type of resource (Download, Upload, Generate)
    /// * `priority` - Priority level (Low, Normal, High)
    /// * `id` - Unique request identifier
    /// * `size` - Optional size in bytes for bandwidth planning
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::new(
    ///     ResourceType::Download,
    ///     ResourcePriority::High,
    ///     42,
    ///     Some(5_000_000)
    /// );
    /// ```
    pub fn new(
        resource_type: ResourceType,
        priority: ResourcePriority,
        id: u64,
        size: Option<u64>,
    ) -> Self {
        Self {
            resource_type,
            priority,
            id,
            size,
        }
    }

    /// Creates a new resource request with an auto-generated ID.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - The type of resource
    /// * `priority` - Priority level
    /// * `size` - Optional size in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::with_generated_id(
    ///     ResourceType::Download,
    ///     ResourcePriority::Normal,
    ///     Some(1_000_000)
    /// );
    /// assert!(request.id > 0);
    /// ```
    pub fn with_generated_id(
        resource_type: ResourceType,
        priority: ResourcePriority,
        size: Option<u64>,
    ) -> Self {
        Self {
            resource_type,
            priority,
            id: generate_request_id(),
            size,
        }
    }

    /// Returns the resource type of this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::new(
    ///     ResourceType::Upload,
    ///     ResourcePriority::Normal,
    ///     1,
    ///     None
    /// );
    /// assert_eq!(request.resource_type(), ResourceType::Upload);
    /// ```
    pub const fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Returns the priority of this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::new(
    ///     ResourceType::Download,
    ///     ResourcePriority::High,
    ///     1,
    ///     None
    /// );
    /// assert_eq!(request.priority(), ResourcePriority::High);
    /// ```
    pub const fn priority(&self) -> ResourcePriority {
        self.priority
    }

    /// Returns the request ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::new(
    ///     ResourceType::Download,
    ///     ResourcePriority::Normal,
    ///     42,
    ///     None
    /// );
    /// assert_eq!(request.id(), 42);
    /// ```
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Returns the optional size in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::new(
    ///     ResourceType::Download,
    ///     ResourcePriority::Normal,
    ///     1,
    ///     Some(1_000_000)
    /// );
    /// assert_eq!(request.size(), Some(1_000_000));
    /// ```
    pub const fn size(&self) -> Option<u64> {
        self.size
    }

    /// Returns whether this request has a known size.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::new(
    ///     ResourceType::Download,
    ///     ResourcePriority::Normal,
    ///     1,
    ///     Some(1_000_000)
    /// );
    /// assert!(request.has_size());
    /// ```
    pub const fn has_size(&self) -> bool {
        self.size.is_some()
    }

    /// Creates a builder for this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
    ///
    /// let request = ResourceRequest::builder()
    ///     .resource_type(ResourceType::Download)
    ///     .priority(ResourcePriority::High)
    ///     .id(1)
    ///     .size(Some(5_000_000))
    ///     .build();
    /// ```
    pub fn builder() -> ResourceRequestBuilder {
        ResourceRequestBuilder::new()
    }
}

impl Default for ResourceRequest {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::Download,
            priority: ResourcePriority::Normal,
            id: generate_request_id(),
            size: None,
        }
    }
}

/// Builder for creating [`ResourceRequest`] instances.
///
/// # Examples
///
/// ```
/// use rustgram_resource_manager::{ResourceRequest, ResourceType, ResourcePriority};
///
/// let request = ResourceRequest::builder()
///     .resource_type(ResourceType::Upload)
///     .priority(ResourcePriority::High)
///     .id(100)
///     .size(Some(10_000_000))
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct ResourceRequestBuilder {
    resource_type: Option<ResourceType>,
    priority: Option<ResourcePriority>,
    id: Option<u64>,
    size: Option<u64>,
}

impl ResourceRequestBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the resource type.
    pub fn resource_type(mut self, resource_type: ResourceType) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Sets the priority.
    pub fn priority(mut self, priority: ResourcePriority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Sets the request ID.
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the size.
    pub fn size(mut self, size: Option<u64>) -> Self {
        self.size = size;
        self
    }

    /// Builds the [`ResourceRequest`].
    ///
    /// Returns `None` if required fields are not set.
    pub fn build(self) -> ResourceRequest {
        ResourceRequest {
            resource_type: self.resource_type.unwrap_or(ResourceType::Download),
            priority: self.priority.unwrap_or(ResourcePriority::Normal),
            id: self.id.unwrap_or_else(generate_request_id),
            size: self.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_request_new() {
        let request = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            1,
            Some(1_000_000),
        );

        assert_eq!(request.resource_type, ResourceType::Download);
        assert_eq!(request.priority, ResourcePriority::Normal);
        assert_eq!(request.id, 1);
        assert_eq!(request.size, Some(1_000_000));
    }

    #[test]
    fn test_resource_request_with_generated_id() {
        let request1 = ResourceRequest::with_generated_id(
            ResourceType::Download,
            ResourcePriority::Normal,
            None,
        );
        let request2 =
            ResourceRequest::with_generated_id(ResourceType::Upload, ResourcePriority::High, None);

        assert!(request2.id > request1.id);
    }

    #[test]
    fn test_resource_request_getters() {
        let request = ResourceRequest::new(
            ResourceType::Upload,
            ResourcePriority::High,
            42,
            Some(5_000_000),
        );

        assert_eq!(request.resource_type(), ResourceType::Upload);
        assert_eq!(request.priority(), ResourcePriority::High);
        assert_eq!(request.id(), 42);
        assert_eq!(request.size(), Some(5_000_000));
    }

    #[test]
    fn test_resource_request_has_size() {
        let with_size = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            1,
            Some(1_000_000),
        );
        let without_size =
            ResourceRequest::new(ResourceType::Download, ResourcePriority::Normal, 2, None);

        assert!(with_size.has_size());
        assert!(!without_size.has_size());
    }

    #[test]
    fn test_resource_request_default() {
        let request = ResourceRequest::default();
        assert_eq!(request.resource_type, ResourceType::Download);
        assert_eq!(request.priority, ResourcePriority::Normal);
        assert!(!request.has_size());
    }

    #[test]
    fn test_resource_request_clone() {
        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            1,
            Some(1_000_000),
        );
        let request2 = request1.clone();
        assert_eq!(request1, request2);
    }

    #[test]
    fn test_resource_request_eq() {
        let request1 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            1,
            Some(1_000_000),
        );
        let request2 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            1,
            Some(1_000_000),
        );
        let request3 = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            2,
            Some(1_000_000),
        );

        assert_eq!(request1, request2);
        assert_ne!(request1, request3);
    }

    #[test]
    fn test_resource_request_builder() {
        let request = ResourceRequest::builder()
            .resource_type(ResourceType::Upload)
            .priority(ResourcePriority::High)
            .id(100)
            .size(Some(10_000_000))
            .build();

        assert_eq!(request.resource_type, ResourceType::Upload);
        assert_eq!(request.priority, ResourcePriority::High);
        assert_eq!(request.id, 100);
        assert_eq!(request.size, Some(10_000_000));
    }

    #[test]
    fn test_resource_request_builder_default() {
        let request = ResourceRequest::builder().build();

        assert_eq!(request.resource_type, ResourceType::Download);
        assert_eq!(request.priority, ResourcePriority::Normal);
        assert!(!request.has_size());
    }

    #[test]
    fn test_resource_request_builder_partial() {
        let request = ResourceRequest::builder()
            .resource_type(ResourceType::Generate)
            .build();

        assert_eq!(request.resource_type, ResourceType::Generate);
        assert_eq!(request.priority, ResourcePriority::Normal);
    }

    #[test]
    fn test_resource_request_debug() {
        let request = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            1,
            Some(1_000_000),
        );

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("Download"));
        assert!(debug_str.contains("Normal"));
    }

    #[test]
    fn test_all_resource_types() {
        for resource_type in ResourceType::ALL {
            let request = ResourceRequest::new(
                resource_type,
                ResourcePriority::Normal,
                generate_request_id(),
                None,
            );
            assert_eq!(request.resource_type, resource_type);
        }
    }

    #[test]
    fn test_all_priorities() {
        for priority in ResourcePriority::ALL {
            let request = ResourceRequest::new(
                ResourceType::Download,
                priority,
                generate_request_id(),
                None,
            );
            assert_eq!(request.priority, priority);
        }
    }

    #[test]
    fn test_request_size_zero() {
        let request =
            ResourceRequest::new(ResourceType::Download, ResourcePriority::Normal, 1, Some(0));

        assert_eq!(request.size, Some(0));
        assert!(request.has_size());
    }

    #[test]
    fn test_request_large_size() {
        let size = u64::MAX;
        let request = ResourceRequest::new(
            ResourceType::Download,
            ResourcePriority::Normal,
            1,
            Some(size),
        );

        assert_eq!(request.size, Some(size));
    }
}
