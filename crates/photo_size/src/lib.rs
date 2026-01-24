//! Photo size stub for Telegram.
//!
//! TODO: Full implementation when TDLib integration is complete.

#![warn(missing_docs)]
#![warn(clippy::all)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Photo size stub.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhotoSize {
    /// Photo type identifier.
    pub type_: String,
}

impl PhotoSize {
    /// Creates a new photo size.
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photo_size_new() {
        let size = PhotoSize::new("test".to_string());
        assert_eq!(size.type_, "test");
    }
}
