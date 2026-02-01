// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Time Zone Manager
//!
//! Time zone management for Telegram.
//!
//! ## Overview
//!
//! Manages timezone information with caching and updates.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_time_zone_manager::TimeZoneManager;
//!
//! let manager = TimeZoneManager::new();
//! let offset = manager.get_time_zone_offset("America/New_York");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Time zone information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeZone {
    /// Time zone identifier (e.g., "America/New_York")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// UTC offset in seconds
    pub utc_offset: i32,
}

impl TimeZone {
    /// Creates a new time zone
    #[must_use]
    pub fn new(id: String, name: String, utc_offset: i32) -> Self {
        Self {
            id,
            name,
            utc_offset,
        }
    }

    /// Returns the time zone ID
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the time zone name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the UTC offset in seconds
    #[must_use]
    pub const fn utc_offset(&self) -> i32 {
        self.utc_offset
    }

    /// Returns the UTC offset in hours
    #[must_use]
    pub fn utc_offset_hours(&self) -> f64 {
        self.utc_offset as f64 / 3600.0
    }

    /// Returns the UTC offset as a string (+HH:MM or -HH:MM)
    #[must_use]
    pub fn utc_offset_string(&self) -> String {
        let abs_offset = self.utc_offset.abs();
        let hours = abs_offset / 3600;
        let minutes = (abs_offset % 3600) / 60;
        let sign = if self.utc_offset >= 0 { '+' } else { '-' };
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.utc_offset_string())
    }
}

/// Collection of time zones
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeZoneList {
    /// List of time zones
    pub time_zones: Vec<TimeZone>,
    /// Hash for cache validation
    pub hash: i32,
    /// Whether the list has been loaded
    pub is_loaded: bool,
}

impl Default for TimeZoneList {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeZoneList {
    /// Creates a new empty time zone list
    #[must_use]
    pub const fn new() -> Self {
        Self {
            time_zones: Vec::new(),
            hash: 0,
            is_loaded: false,
        }
    }

    /// Returns the number of time zones
    #[must_use]
    pub fn len(&self) -> usize {
        self.time_zones.len()
    }

    /// Returns true if there are no time zones
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.time_zones.is_empty()
    }

    /// Adds a time zone to the list
    pub fn add(&mut self, time_zone: TimeZone) {
        self.time_zones.push(time_zone);
    }

    /// Finds a time zone by ID
    #[must_use]
    pub fn find_by_id(&self, id: &str) -> Option<&TimeZone> {
        self.time_zones.iter().find(|tz| tz.id == id)
    }

    /// Clears the list
    pub fn clear(&mut self) {
        self.time_zones.clear();
    }

    /// Marks as loaded
    pub fn mark_loaded(&mut self) {
        self.is_loaded = true;
    }
}

/// Time zone manager
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeZoneManager {
    /// Time zones list
    time_zones: TimeZoneList,
    /// Default UTC offset when timezone not found
    default_utc_offset: i32,
}

impl Default for TimeZoneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeZoneManager {
    /// Creates a new time zone manager
    #[must_use]
    pub const fn new() -> Self {
        Self {
            time_zones: TimeZoneList::new(),
            default_utc_offset: 0,
        }
    }

    /// Creates a new time zone manager with default offset
    #[must_use]
    pub const fn with_default_offset(default_utc_offset: i32) -> Self {
        Self {
            time_zones: TimeZoneList::new(),
            default_utc_offset,
        }
    }

    /// Returns the time zones list
    #[must_use]
    pub const fn time_zones(&self) -> &TimeZoneList {
        &self.time_zones
    }

    /// Returns true if time zones are loaded
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        self.time_zones.is_loaded
    }

    /// Gets the UTC offset for a time zone ID
    #[must_use]
    pub fn get_time_zone_offset(&self, time_zone_id: &str) -> i32 {
        if let Some(tz) = self.time_zones.find_by_id(time_zone_id) {
            return tz.utc_offset();
        }
        self.default_utc_offset
    }

    /// Gets the UTC offset in hours for a time zone ID
    #[must_use]
    pub fn get_time_zone_offset_hours(&self, time_zone_id: &str) -> f64 {
        self.get_time_zone_offset(time_zone_id) as f64 / 3600.0
    }

    /// Gets a time zone by ID
    #[must_use]
    pub fn get_time_zone(&self, time_zone_id: &str) -> Option<&TimeZone> {
        self.time_zones.find_by_id(time_zone_id)
    }

    /// Returns all time zones
    #[must_use]
    pub fn all_time_zones(&self) -> &[TimeZone] {
        &self.time_zones.time_zones
    }

    /// Updates the time zones list
    pub fn update_time_zones(&mut self, time_zones: Vec<TimeZone>, hash: i32) {
        self.time_zones.time_zones = time_zones;
        self.time_zones.hash = hash;
        self.time_zones.is_loaded = true;
    }

    /// Clears the time zones
    pub fn clear(&mut self) {
        self.time_zones.clear();
        self.time_zones.is_loaded = false;
    }

    /// Sets the default UTC offset
    pub fn set_default_utc_offset(&mut self, offset: i32) {
        self.default_utc_offset = offset;
    }

    /// Returns the default UTC offset
    #[must_use]
    pub const fn default_utc_offset(&self) -> i32 {
        self.default_utc_offset
    }
}

impl fmt::Display for TimeZoneManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TimeZoneManager({} time zones, loaded: {})",
            self.time_zones.len(),
            self.time_zones.is_loaded
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_zone_new() {
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        assert_eq!(tz.id(), "America/New_York");
        assert_eq!(tz.utc_offset(), -18000);
    }

    #[test]
    fn test_time_zone_utc_offset_hours() {
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        assert_eq!(tz.utc_offset_hours(), -5.0);
    }

    #[test]
    fn test_time_zone_utc_offset_string() {
        let tz1 = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        assert_eq!(tz1.utc_offset_string(), "-05:00");

        let tz2 = TimeZone::new(
            "Europe/Berlin".to_string(),
            "Central European Time".to_string(),
            3600,
        );
        assert_eq!(tz2.utc_offset_string(), "+01:00");

        let tz3 = TimeZone::new("Asia/Kolkata".to_string(), "India Time".to_string(), 19800);
        assert_eq!(tz3.utc_offset_string(), "+05:30");
    }

    #[test]
    fn test_time_zone_display() {
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        let display = format!("{}", tz);
        assert!(display.contains("Eastern Time"));
        assert!(display.contains("-05:00"));
    }

    #[test]
    fn test_time_zone_list_new() {
        let list = TimeZoneList::new();
        assert!(list.is_empty());
        assert!(!list.is_loaded);
    }

    #[test]
    fn test_time_zone_list_add() {
        let mut list = TimeZoneList::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        list.add(tz);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_time_zone_list_find_by_id() {
        let mut list = TimeZoneList::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        list.add(tz);

        assert!(list.find_by_id("America/New_York").is_some());
        assert!(list.find_by_id("Europe/London").is_none());
    }

    #[test]
    fn test_time_zone_list_clear() {
        let mut list = TimeZoneList::new();
        list.add(TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        ));
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_time_zone_manager_new() {
        let manager = TimeZoneManager::new();
        assert_eq!(manager.default_utc_offset(), 0);
        assert!(!manager.is_loaded());
    }

    #[test]
    fn test_time_zone_manager_with_default_offset() {
        let manager = TimeZoneManager::with_default_offset(-18000);
        assert_eq!(manager.default_utc_offset(), -18000);
    }

    #[test]
    fn test_get_time_zone_offset() {
        let mut manager = TimeZoneManager::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        manager.update_time_zones(vec![tz], 1);

        assert_eq!(manager.get_time_zone_offset("America/New_York"), -18000);
    }

    #[test]
    fn test_get_time_zone_offset_not_found() {
        let manager = TimeZoneManager::with_default_offset(3600);
        assert_eq!(manager.get_time_zone_offset("Unknown/Zone"), 3600);
    }

    #[test]
    fn test_get_time_zone_offset_hours() {
        let mut manager = TimeZoneManager::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        manager.update_time_zones(vec![tz], 1);

        assert_eq!(manager.get_time_zone_offset_hours("America/New_York"), -5.0);
    }

    #[test]
    fn test_get_time_zone() {
        let mut manager = TimeZoneManager::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        manager.update_time_zones(vec![tz], 1);

        assert!(manager.get_time_zone("America/New_York").is_some());
        assert!(manager.get_time_zone("Unknown/Zone").is_none());
    }

    #[test]
    fn test_all_time_zones() {
        let mut manager = TimeZoneManager::new();
        let tz1 = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        let tz2 = TimeZone::new("Europe/London".to_string(), "GMT".to_string(), 0);
        manager.update_time_zones(vec![tz1, tz2], 1);

        assert_eq!(manager.all_time_zones().len(), 2);
    }

    #[test]
    fn test_update_time_zones() {
        let mut manager = TimeZoneManager::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        manager.update_time_zones(vec![tz], 123);

        assert!(manager.is_loaded());
        assert_eq!(manager.time_zones().hash, 123);
    }

    #[test]
    fn test_clear() {
        let mut manager = TimeZoneManager::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        manager.update_time_zones(vec![tz], 1);
        manager.clear();

        assert!(!manager.is_loaded());
        assert!(manager.all_time_zones().is_empty());
    }

    #[test]
    fn test_set_default_utc_offset() {
        let mut manager = TimeZoneManager::new();
        manager.set_default_utc_offset(7200);
        assert_eq!(manager.default_utc_offset(), 7200);
    }

    #[test]
    fn test_manager_display() {
        let manager = TimeZoneManager::new();
        let display = format!("{}", manager);
        assert!(display.contains("TimeZoneManager"));
        assert!(display.contains("0 time zones"));
    }

    #[test]
    fn test_manager_display_with_time_zones() {
        let mut manager = TimeZoneManager::new();
        let tz = TimeZone::new(
            "America/New_York".to_string(),
            "Eastern Time".to_string(),
            -18000,
        );
        manager.update_time_zones(vec![tz], 1);

        let display = format!("{}", manager);
        assert!(display.contains("1 time zones"));
        assert!(display.contains("loaded: true"));
    }
}
