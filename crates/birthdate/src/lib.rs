//! # Rustgram Birthdate
//!
//! Birthdate type for Telegram MTProto client.
//!
//! This crate provides a compact, bit-packed representation of birthdates
//! that matches TDLib's storage format. It includes support for:
//!
//! - Day, month, and optional year storage in a single i32
//! - Conversion to/from Telegram API (MTProto) format
//! - Conversion to/from TD API format
//! - Full validation of date values
//!
//! ## Storage Format
//!
//! The birthdate is stored in a single i32 value using bit packing:
//!
//! ```text
//! | Bits 31-9 | Bits 8-5 | Bits 4-0 |
//! |-----------|----------|----------|
//! | Year      | Month    | Day      |
//! | 23 bits   | 4 bits   | 5 bits   |
//! ```
//!
//! - Day: 5 bits (values 1-31, 0 for empty)
//! - Month: 4 bits (values 1-12, 0 for empty)
//! - Year: 23 bits (values 1800-3000, or 0 for unknown)
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_birthdate::Birthdate;
//!
//! // Create a birthdate with year
//! let bd = Birthdate::new(15, 6, 1990).unwrap();
//! assert_eq!(bd.day(), 15);
//! assert_eq!(bd.month(), 6);
//! assert_eq!(bd.year(), Some(1990));
//!
//! // Create a birthdate without year (year is unknown)
//! let bd_no_year = Birthdate::new(15, 6, 0).unwrap();
//! assert_eq!(bd_no_year.day(), 15);
//! assert_eq!(bd_no_year.month(), 6);
//! assert_eq!(bd_no_year.year(), None);
//!
//! // Empty birthdate
//! assert!(Birthdate::new(0, 0, 0).unwrap().is_empty());
//!
//! // Display format
//! assert_eq!(bd.to_string(), "1990-06-15");
//! assert_eq!(bd_no_year.to_string(), "----06-15");
//! ```
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's `Birthdate` class:
//! - Source: `references/td/td/telegram/Birthdate.h`
//! - Bit packing format matches exactly
//! - Validation rules match TDLib's behavior
//! - Year 0 indicates "unknown" year
//!
//! ## Modules
//!
//! - [`birthdate`] - Core Birthdate type with bit-packed storage
//! - [`error`] - Error types for birthdate operations
//! - [`tl`] - TL serialization support

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

pub mod birthdate;
pub mod error;
pub mod tl;

// Re-export public API at crate root
pub use birthdate::{Birthdate, BIRTHDAY_MAGIC, YEAR_FLAG_MASK};
pub use error::{BirthdateError, Result};
pub use tl::TelegramApiBirthday;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-birthdate";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-birthdate");
    }

    #[test]
    fn test_constants() {
        assert_eq!(BIRTHDAY_MAGIC, 0x6c8e1e06);
        assert_eq!(YEAR_FLAG_MASK, 0x1);
    }

    #[test]
    fn test_empty_birthdate() {
        let bd = Birthdate::default();
        assert!(bd.is_empty());
        assert_eq!(bd.to_string(), "");
    }

    #[test]
    fn test_complete_workflow() {
        // Create from components
        let bd = Birthdate::new(15, 6, 1990).unwrap();

        // Access components
        assert_eq!(bd.day(), 15);
        assert_eq!(bd.month(), 6);
        assert_eq!(bd.year(), Some(1990));

        // Convert to TD API
        let (day, month, year) = bd.to_td_api();
        assert_eq!((day, month, year), (15, 6, 1990));

        // Convert from TD API
        let bd2 = Birthdate::from_td_api(day, month, year).unwrap();
        assert_eq!(bd, bd2);

        // Convert to Telegram API
        let telegram_bd = bd.to_telegram_api();
        assert_eq!(telegram_bd.flags, YEAR_FLAG_MASK);
        assert_eq!(telegram_bd.day, 15);
        assert_eq!(telegram_bd.month, 6);
        assert_eq!(telegram_bd.year, Some(1990));

        // Convert from Telegram API
        let bd3 = Birthdate::from_telegram_api(&telegram_bd).unwrap();
        assert_eq!(bd, bd3);

        // Display
        assert_eq!(bd.to_string(), "1990-06-15");
        assert_eq!(format!("{:?}", bd), "Birthdate(1990-06-15)");
    }
}
