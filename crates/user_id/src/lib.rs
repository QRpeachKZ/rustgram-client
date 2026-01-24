// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # User ID
//!
//! User identifier for Telegram users.
//!
//! ## Overview
//!
//! Re-exports UserId from the types crate for convenience.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_user_id::UserId;
//!
//! let user_id = UserId::new(1234567890).unwrap();
//! assert_eq!(user_id.get(), 1234567890);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub use rustgram_types::UserId;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_new_valid() {
        let id = UserId::new(1234567890).unwrap();
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_user_id_new_invalid() {
        assert!(UserId::new(0).is_err());
        assert!(UserId::new(-1).is_err());
    }

    #[test]
    fn test_user_id_is_valid() {
        let id = UserId::new(123).unwrap();
        assert!(id.is_valid());
    }

    #[test]
    fn test_user_id_display() {
        let id = UserId::new(123).unwrap();
        assert_eq!(format!("{}", id), "user 123");
    }
}
