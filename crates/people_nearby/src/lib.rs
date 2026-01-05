// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Rustgram People Nearby
//!
//! People nearby functionality for Telegram MTProto client.
//!
//! ## Overview
//!
//! This module provides functionality for searching and managing nearby
//! users and chats in the Telegram MTProto protocol. It implements types
//! for representing nearby chats and a manager for handling search operations
//! and location visibility settings.
//!
//! ## Features
//!
//! - Search for nearby users based on location
//! - Manage location visibility settings
//! - Pagination support for large result sets
//! - Automatic updates (60 seconds after search)
//!
//! ## Examples
//!
//! ### Searching for Nearby Users
//!
//! ```rust
//! use rustgram_people_nearby::PeopleNearbyManager;
//! use rustgram_venue::Location;
//!
//! let mut manager = PeopleNearbyManager::new(false);
//!
//! // Create a location (Moscow coordinates)
//! let location = Location::from_components(55.7558, 37.6173, 10.0, 0);
//!
//! // Search for nearby users
//! match manager.search_nearby(&location, None, None) {
//!     Ok(results) => {
//!         println!("Found {} nearby users", results.len());
//!         for nearby in results.users_nearby() {
//!             println!("Chat {}: {} meters away",
//!                 nearby.chat_id().get(),
//!                 nearby.distance()
//!             );
//!         }
//!     }
//!     Err(e) => {
//!         eprintln!("Error searching nearby: {}", e);
//!     }
//! }
//! ```
//!
//! ### Managing Location Visibility
//!
//! ```rust
//! use rustgram_people_nearby::PeopleNearbyManager;
//!
//! let mut manager = PeopleNearbyManager::new(false);
//!
//! // Enable visibility (expires in 24 hours)
//! manager.enable_visibility();
//! assert!(manager.is_visibility_active());
//!
//! // Disable visibility
//! manager.disable_visibility();
//! assert!(!manager.is_visibility_active());
//!
//! // Set custom expiration
//! use std::time::{SystemTime, UNIX_EPOCH};
//! let now = SystemTime::now()
//!     .duration_since(UNIX_EPOCH)
//!     .unwrap()
//!     .as_secs() as i64;
//! manager.set_visibility_expiration(Some(now + 3600)); // 1 hour
//! ```
//!
//! ### Working with Results
//!
//! ```rust
//! use rustgram_people_nearby::{ChatNearby, ChatsNearby};
//! use rustgram_types::ChatId;
//!
//! // Create nearby chats
//! let chat1 = ChatNearby::new(ChatId::new(123).unwrap(), 150).unwrap();
//! let chat2 = ChatNearby::new(ChatId::new(456).unwrap(), 300).unwrap();
//!
//! let mut results = ChatsNearby::new(vec![chat1, chat2], "");
//!
//! // Sort by distance
//! results.sort_by_distance();
//!
//! // Filter by maximum distance
//! results.filter_by_distance(200);
//! assert_eq!(results.len(), 1);
//!
//! // Check pagination
//! if results.has_more() {
//!     println!("More results available: {}", results.next_offset().unwrap());
//! }
//! ```
//!
//! ## TDLib Compatibility
//!
//! - **Reference**: `references/td/td/telegram/PeopleNearbyManager.{h,cpp}`
//! - **TL Type**: `chatNearby`, `chatsNearby`
//! - **TL Function**: `searchChatsNearby`
//! - **TL Update**: `updateUsersNearby`
//!
//! ## TL Correspondence
//!
//! ### TD API
//!
//! ```text
//! chatNearby#bde26775 chat_id:int53 distance:int = ChatNearby;
//! chatsNearby#e482a098 users_nearby:vector<chatNearby> next_offset:int = ChatsNearby;
//! updateUsersNearby#b090efb9 = Update;
//!
//! searchChatsNearby#e0155ce6 location:geoPoint = ChatsNearby;
//! setLocationVisibility#a75abb6f visibility_period:int = Bool;
//! ```
//!
//! ## Limitations
//!
//! - Bot accounts cannot use the people nearby feature
//! - Maximum distance is limited to 1,000 km
//! - Location visibility expires after a configurable time period
//!
//! ## Design Decisions
//!
//! 1. **Distance in Meters**: Stored as integer meters for precision and
//!    compatibility with TDLib.
//!
//! 2. **Pagination**: Uses string-based offsets for flexible pagination
//!    compatible with Telegram's implementation.
//!
//! 3. **Update Timing**: Updates are sent 60 seconds after a successful search
//!    to refresh nearby user data.
//!
//! 4. **Visibility Management**: Location visibility can be set with custom
//!    expiration times or default to 24 hours.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod error;
pub mod manager;
pub mod types;

// Re-exports
pub use error::{PeopleNearbyError, Result};
pub use manager::{PeopleNearbyManager, DEFAULT_VISIBILITY_EXPIRE_HOURS, LOCATION_VISIBILITY_KEY};
pub use types::{ChatNearby, ChatsNearby, UsersNearbyUpdate, MAX_DISTANCE_METERS};

/// Timeout for nearby updates (60 seconds).
pub const UPDATE_TIMEOUT_SECS: u64 = 60;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-people-nearby";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-people-nearby");
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_DISTANCE_METERS, 1_000_000);
        assert_eq!(DEFAULT_VISIBILITY_EXPIRE_HOURS, 24);
        assert_eq!(UPDATE_TIMEOUT_SECS, 60);
    }

    #[test]
    fn test_binlog_keys() {
        assert_eq!(LOCATION_VISIBILITY_KEY, "location_visibility_expire_date");
    }
}
