//! # Rustgram BusinessInfo
//!
//! Business info handling for Telegram MTProto client.
//!
//! This crate provides types for managing business account information
//! including location, work hours, and messages.
//!
//! ## Overview
//!
//! - [`BusinessInfo`] - Business account information
//! - [`BusinessLocation`] - Business location information
//! - [`BusinessWorkHours`] - Business working hours
//!
//! ## Examples
//!
//! ```
//! use rustgram_business_info::BusinessInfo;
//!
//! let info = BusinessInfo::new();
//! assert!(info.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Business location information.
///
/// Represents the physical location of a business.
///
/// # Examples
///
/// ```
/// use rustgram_business_info::BusinessLocation;
///
/// let location = BusinessLocation::new();
/// assert!(location.is_empty());
///
/// let location = BusinessLocation::with_data(
///     "123 Main St".to_string(),
///     "New York".to_string(),
///     40.7128,
///     -74.0060,
/// );
/// assert!(!location.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessLocation {
    /// Street address
    address: String,
    /// City name
    city: String,
    /// Building ID (for multi-building locations)
    building_id: String,
}

impl Default for BusinessLocation {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessLocation {
    /// Creates a new empty location.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessLocation;
    ///
    /// let location = BusinessLocation::new();
    /// assert!(location.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            address: String::new(),
            city: String::new(),
            building_id: String::new(),
        }
    }

    /// Creates a location with address and city.
    ///
    /// # Arguments
    ///
    /// * `address` - Street address
    /// * `city` - City name
    /// * `latitude` - Latitude (unused placeholder)
    /// * `longitude` - Longitude (unused placeholder)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessLocation;
    ///
    /// let location = BusinessLocation::with_data(
    ///     "123 Main St".to_string(),
    ///     "New York".to_string(),
    ///     40.7128,
    ///     -74.0060,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(address: String, city: String, _latitude: f64, _longitude: f64) -> Self {
        Self {
            address,
            city,
            building_id: String::new(),
        }
    }

    /// Checks if the location is empty.
    ///
    /// # Returns
    ///
    /// `true` if address and city are empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessLocation;
    ///
    /// assert!(BusinessLocation::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.address.is_empty() && self.city.is_empty()
    }

    /// Returns the address.
    #[inline]
    #[must_use]
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns the city.
    #[inline]
    #[must_use]
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Sets the address.
    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    /// Sets the city.
    pub fn set_city(&mut self, city: String) {
        self.city = city;
    }
}

/// Business work hours.
///
/// Defines when a business is open.
///
/// # Examples
///
/// ```
/// use rustgram_business_info::BusinessWorkHours;
///
/// let hours = BusinessWorkHours::new();
/// assert!(hours.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessWorkHours {
    /// Time zone identifier (e.g., "America/New_York")
    timezone_id: String,
    /// Opening hours for each day (simplified)
    // In full implementation, this would be a complex structure
    // with opening/closing times for each day of the week
    is_configured: bool,
}

impl Default for BusinessWorkHours {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessWorkHours {
    /// Creates a new empty work hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessWorkHours;
    ///
    /// let hours = BusinessWorkHours::new();
    /// assert!(hours.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            timezone_id: String::new(),
            is_configured: false,
        }
    }

    /// Creates work hours with a timezone.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessWorkHours;
    ///
    /// let hours = BusinessWorkHours::with_timezone("America/New_York".to_string());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_timezone(timezone_id: String) -> Self {
        Self {
            timezone_id,
            is_configured: true,
        }
    }

    /// Checks if work hours are configured.
    ///
    /// # Returns
    ///
    /// `true` if work hours are set
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessWorkHours;
    ///
    /// assert!(BusinessWorkHours::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.is_configured
    }

    /// Returns the timezone ID.
    #[inline]
    #[must_use]
    pub fn timezone_id(&self) -> &str {
        &self.timezone_id
    }
}

/// Business information.
///
/// Aggregates all business account settings.
///
/// # Examples
///
/// ```
/// use rustgram_business_info::BusinessInfo;
///
/// let info = BusinessInfo::new();
/// assert!(info.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessInfo {
    /// Business location
    location: BusinessLocation,
    /// Work hours
    work_hours: BusinessWorkHours,
}

impl Default for BusinessInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessInfo {
    /// Creates a new empty business info.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessInfo;
    ///
    /// let info = BusinessInfo::new();
    /// assert!(info.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            location: BusinessLocation::new(),
            work_hours: BusinessWorkHours::new(),
        }
    }

    /// Creates business info with the given data.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::{BusinessInfo, BusinessLocation, BusinessWorkHours};
    ///
    /// let location = BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
    /// let work_hours = BusinessWorkHours::with_timezone("UTC".to_string());
    ///
    /// let info = BusinessInfo::with_data(location, work_hours);
    /// assert!(!info.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(location: BusinessLocation, work_hours: BusinessWorkHours) -> Self {
        Self {
            location,
            work_hours,
        }
    }

    /// Checks if business info is empty.
    ///
    /// # Returns
    ///
    /// `true` if all components are empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessInfo;
    ///
    /// assert!(BusinessInfo::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.location.is_empty() && self.work_hours.is_empty()
    }

    /// Returns the business location.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessInfo;
    ///
    /// let info = BusinessInfo::new();
    /// assert!(info.location().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn location(&self) -> &BusinessLocation {
        &self.location
    }

    /// Returns the work hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::BusinessInfo;
    ///
    /// let info = BusinessInfo::new();
    /// assert!(info.work_hours().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn work_hours(&self) -> &BusinessWorkHours {
        &self.work_hours
    }

    /// Sets the business location.
    ///
    /// # Returns
    ///
    /// `true` if the location was changed
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::{BusinessInfo, BusinessLocation};
    ///
    /// let mut info = BusinessInfo::new();
    /// let location = BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
    ///
    /// let changed = info.set_location(location.clone());
    /// assert!(changed);
    /// assert_eq!(info.location().address(), "123 Main St");
    /// ```
    pub fn set_location(&mut self, location: BusinessLocation) -> bool {
        if self.location == location {
            return false;
        }
        self.location = location;
        true
    }

    /// Sets the work hours.
    ///
    /// # Returns
    ///
    /// `true` if the work hours were changed
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::{BusinessInfo, BusinessWorkHours};
    ///
    /// let mut info = BusinessInfo::new();
    /// let work_hours = BusinessWorkHours::with_timezone("UTC".to_string());
    ///
    /// let changed = info.set_work_hours(work_hours.clone());
    /// assert!(changed);
    /// assert!(!info.work_hours().is_empty());
    /// ```
    pub fn set_work_hours(&mut self, work_hours: BusinessWorkHours) -> bool {
        if self.work_hours == work_hours {
            return false;
        }
        self.work_hours = work_hours;
        true
    }

    /// Clears all business info.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_info::{BusinessInfo, BusinessLocation};
    ///
    /// let mut info = BusinessInfo::new();
    /// let location = BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
    /// info.set_location(location);
    ///
    /// info.clear();
    /// assert!(info.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl fmt::Display for BusinessLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessLocation {{ address: {:?}, city: {:?} }}",
            self.address, self.city
        )
    }
}

impl fmt::Display for BusinessWorkHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessWorkHours {{ timezone: {:?}, configured: {} }}",
            self.timezone_id, self.is_configured
        )
    }
}

impl fmt::Display for BusinessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessInfo {{ location: {}, work_hours: {} }}",
            self.location, self.work_hours
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-info";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== BusinessLocation Tests ==========

    #[test]
    fn test_location_new_is_empty() {
        let location = BusinessLocation::new();
        assert!(location.is_empty());
        assert_eq!(location.address(), "");
        assert_eq!(location.city(), "");
    }

    #[test]
    fn test_location_with_data() {
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 40.7, -74.0);
        assert!(!location.is_empty());
        assert_eq!(location.address(), "123 Main St");
        assert_eq!(location.city(), "NYC");
    }

    #[test]
    fn test_location_setters() {
        let mut location = BusinessLocation::new();
        location.set_address("456 Oak Ave".to_string());
        location.set_city("LA".to_string());

        assert_eq!(location.address(), "456 Oak Ave");
        assert_eq!(location.city(), "LA");
    }

    // ========== BusinessWorkHours Tests ==========

    #[test]
    fn test_work_hours_new_is_empty() {
        let hours = BusinessWorkHours::new();
        assert!(hours.is_empty());
    }

    #[test]
    fn test_work_hours_with_timezone() {
        let hours = BusinessWorkHours::with_timezone("America/New_York".to_string());
        assert!(!hours.is_empty());
        assert_eq!(hours.timezone_id(), "America/New_York");
    }

    // ========== BusinessInfo Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let info = BusinessInfo::new();
        assert!(info.is_empty());
        assert!(info.location().is_empty());
        assert!(info.work_hours().is_empty());
    }

    #[test]
    fn test_default_creates_empty() {
        let info = BusinessInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_with_data() {
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        let work_hours = BusinessWorkHours::with_timezone("UTC".to_string());

        let info = BusinessInfo::with_data(location.clone(), work_hours.clone());
        assert!(!info.is_empty());
        assert_eq!(info.location().address(), "123 Main St");
        assert_eq!(info.work_hours().timezone_id(), "UTC");
    }

    // ========== BusinessInfo is_empty Tests ==========

    #[test]
    fn test_is_empty_when_all_empty() {
        let info = BusinessInfo::new();
        assert!(info.is_empty());
    }

    #[test]
    fn test_is_empty_with_location() {
        let mut info = BusinessInfo::new();
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        info.set_location(location);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_empty_with_work_hours() {
        let mut info = BusinessInfo::new();
        let work_hours = BusinessWorkHours::with_timezone("UTC".to_string());
        info.set_work_hours(work_hours);
        assert!(!info.is_empty());
    }

    // ========== BusinessInfo Mutator Tests ==========

    #[test]
    fn test_set_location_changes() {
        let mut info = BusinessInfo::new();
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);

        let changed = info.set_location(location.clone());
        assert!(changed);
        assert_eq!(info.location(), &location);
    }

    #[test]
    fn test_set_location_same_value() {
        let mut info = BusinessInfo::new();
        let location = BusinessLocation::new();
        info.set_location(location.clone());

        let changed = info.set_location(location);
        assert!(!changed);
    }

    #[test]
    fn test_set_work_hours_changes() {
        let mut info = BusinessInfo::new();
        let work_hours = BusinessWorkHours::with_timezone("UTC".to_string());

        let changed = info.set_work_hours(work_hours.clone());
        assert!(changed);
        assert_eq!(info.work_hours(), &work_hours);
    }

    #[test]
    fn test_set_work_hours_same_value() {
        let mut info = BusinessInfo::new();
        let work_hours = BusinessWorkHours::new();
        info.set_work_hours(work_hours.clone());

        let changed = info.set_work_hours(work_hours);
        assert!(!changed);
    }

    #[test]
    fn test_clear() {
        let mut info = BusinessInfo::new();
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        info.set_location(location);
        assert!(!info.is_empty());

        info.clear();
        assert!(info.is_empty());
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_values() {
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        let work_hours = BusinessWorkHours::with_timezone("UTC".to_string());

        let info1 = BusinessInfo::with_data(location.clone(), work_hours.clone());
        let info2 = BusinessInfo::with_data(location, work_hours);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_inequality_different_location() {
        let loc1 =
            BusinessLocation::with_data("Address 1".to_string(), "City".to_string(), 0.0, 0.0);
        let loc2 =
            BusinessLocation::with_data("Address 2".to_string(), "City".to_string(), 0.0, 0.0);

        let info1 = BusinessInfo::with_data(loc1, BusinessWorkHours::new());
        let info2 = BusinessInfo::with_data(loc2, BusinessWorkHours::new());
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_location_equality() {
        let loc1 =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        let loc2 =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        assert_eq!(loc1, loc2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone_info() {
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        let info1 = BusinessInfo::with_data(location, BusinessWorkHours::new());
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_clone_location() {
        let loc1 =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        let loc2 = loc1.clone();
        assert_eq!(loc1, loc2);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_location() {
        let location =
            BusinessLocation::with_data("123 Main St".to_string(), "NYC".to_string(), 0.0, 0.0);
        let s = format!("{}", location);
        assert!(s.contains("123 Main St"));
        assert!(s.contains("NYC"));
    }

    #[test]
    fn test_display_info() {
        let info = BusinessInfo::new();
        let s = format!("{}", info);
        assert!(s.contains("BusinessInfo"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-business-info");
    }
}
