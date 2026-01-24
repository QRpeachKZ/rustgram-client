//! Animation size stub

#![warn(missing_docs)]

use rustgram_file_id::FileId;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stub for animation size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationSize {
    /// Width.
    pub width: i32,
    /// Height.
    pub height: i32,
    /// File ID.
    pub file_id: FileId,
}

impl AnimationSize {
    /// Creates a new animation size.
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            file_id: FileId::empty(),
        }
    }

    /// Returns the file ID.
    pub fn file_id(&self) -> FileId {
        self.file_id
    }
}

impl Default for AnimationSize {
    fn default() -> Self {
        Self::new(0, 0)
    }
}
