// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Part representation for chunked file transfers.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A file part in a chunked transfer.
///
/// Represents a single part/chunk of a file being uploaded or downloaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Part {
    /// Part ID (index)
    pub id: i32,
    /// Offset in bytes from the start of the file
    pub offset: i64,
    /// Size of this part in bytes
    pub size: usize,
}

impl Part {
    /// Creates a new file part.
    ///
    /// # Arguments
    ///
    /// * `id` - The part ID (index)
    /// * `offset` - The offset in bytes from the start of the file
    /// * `size` - The size of this part in bytes
    #[must_use]
    pub const fn new(id: i32, offset: i64, size: usize) -> Self {
        Self { id, offset, size }
    }

    /// Creates an empty part (sentinel value).
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            id: -1,
            offset: 0,
            size: 0,
        }
    }

    /// Returns `true` if this is an empty part.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.id < 0
    }

    /// Gets the end offset of this part.
    #[must_use]
    pub const fn end_offset(&self) -> i64 {
        self.offset + self.size as i64
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "[empty part]")
        } else {
            write!(
                f,
                "[part id={}, offset={}, size={}]",
                self.id, self.offset, self.size
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let part = Part::new(0, 1000, 512);
        assert_eq!(part.id, 0);
        assert_eq!(part.offset, 1000);
        assert_eq!(part.size, 512);
    }

    #[test]
    fn test_empty() {
        let part = Part::empty();
        assert!(part.is_empty());
        assert_eq!(part.id, -1);
    }

    #[test]
    fn test_is_empty() {
        let part = Part::new(0, 0, 100);
        assert!(!part.is_empty());
    }

    #[test]
    fn test_end_offset() {
        let part = Part::new(0, 1000, 512);
        assert_eq!(part.end_offset(), 1512);
    }

    #[test]
    fn test_display() {
        let part = Part::new(5, 10000, 1024);
        let s = format!("{part}");
        assert!(s.contains("5"));
        assert!(s.contains("10000"));
        assert!(s.contains("1024"));
    }

    #[test]
    fn test_display_empty() {
        let part = Part::empty();
        let s = format!("{part}");
        assert!(s.contains("empty"));
    }
}
