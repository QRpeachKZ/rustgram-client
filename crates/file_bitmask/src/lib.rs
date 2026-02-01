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

//! # File Bitmask
//!
//! Bitmask for tracking which parts of a file have been downloaded.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `Bitmask` class from `td/telegram/files/FileBitmask.h`.
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_bitmask::{FileBitmask, BitmaskOnes};
//!
//! // Create a bitmask with 100 bits set
//! let mask = FileBitmask::with_ones(BitmaskOnes, 100);
//! assert!(mask.get(0));
//! assert!(mask.get(99));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Marker type for creating a bitmask from decoded data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitmaskDecode;

/// Marker type for creating a bitmask with N bits set to 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitmaskOnes;

/// File bitmask for tracking downloaded parts.
///
/// Uses a compact byte representation where each bit represents whether a part
/// of the file has been downloaded (1) or not (0).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FileBitmask {
    /// Internal data storage (each byte = 8 bits)
    data: Vec<u8>,
}

impl FileBitmask {
    /// Creates an empty bitmask.
    #[must_use]
    pub const fn empty() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a bitmask by decoding zero-one encoded data.
    ///
    /// # Arguments
    ///
    /// * `_decode` - Marker type indicating decode operation
    /// * `data` - The encoded data to decode
    #[must_use]
    pub fn with_decode(_decode: BitmaskDecode, data: &[u8]) -> Self {
        Self {
            data: zero_one_decode(data),
        }
    }

    /// Creates a bitmask with the first N bits set to 1.
    ///
    /// # Arguments
    ///
    /// * `_ones` - Marker type indicating ones operation
    /// * `count` - Number of bits to set
    #[must_use]
    pub fn with_ones(_ones: BitmaskOnes, count: i64) -> Self {
        let byte_count = ((count + 7) / 8) as usize;
        let mut mask = Self {
            data: vec![0u8; byte_count],
        };
        for i in 0..count {
            mask.set(i);
        }
        mask
    }

    /// Encodes the bitmask to zero-one encoded format.
    ///
    /// # Arguments
    ///
    /// * `prefix_count` - Optional prefix count to limit encoding (-1 for no limit)
    #[must_use]
    pub fn encode(&self, prefix_count: i32) -> Vec<u8> {
        let mut data = self.data.clone();

        // Handle prefix count
        if prefix_count != -1 {
            let truncated_size = ((prefix_count + 7) / 8) as usize;
            if truncated_size < data.len() {
                data.truncate(truncated_size);
            }
            if prefix_count % 8 != 0 && !data.is_empty() {
                let last_idx = truncated_size.saturating_sub(1);
                let mask = 0xFF >> (8 - (prefix_count % 8));
                data[last_idx] &= mask;
            }
        }

        // Remove trailing zeros
        while data.last() == Some(&0) {
            data.pop();
        }

        zero_one_encode(&data)
    }

    /// Gets the ready prefix size starting from an offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - Starting offset in bytes
    /// * `part_size` - Size of each part in bytes
    /// * `file_size` - Total file size (0 if unknown)
    #[must_use]
    pub fn get_ready_prefix_size(&self, offset: i64, part_size: i64, file_size: i64) -> i64 {
        if offset < 0 || part_size <= 0 {
            return 0;
        }

        let offset_part = offset / part_size;
        let ones = self.get_ready_parts(offset_part);

        if ones == 0 {
            return 0;
        }

        let ready_parts_end = (offset_part + ones) * part_size;
        let ready_parts_end = if file_size != 0 && ready_parts_end > file_size {
            file_size.min(offset.max(file_size))
        } else {
            ready_parts_end
        };

        ready_parts_end.saturating_sub(offset)
    }

    /// Gets the total size of all ready parts.
    ///
    /// # Arguments
    ///
    /// * `part_size` - Size of each part in bytes
    /// * `file_size` - Total file size (0 if unknown)
    #[must_use]
    pub fn get_total_size(&self, part_size: i64, file_size: i64) -> i64 {
        let mut res = 0i64;
        for i in 0..self.size() {
            if self.get(i) {
                let from = i * part_size;
                let mut to = from + part_size;
                if file_size != 0 && file_size < to {
                    to = file_size;
                }
                if from < to {
                    res += to - from;
                }
            }
        }
        res
    }

    /// Checks if a specific part is ready (bit is set).
    ///
    /// # Arguments
    ///
    /// * `offset_part` - The part index to check
    #[must_use]
    pub fn get(&self, offset_part: i64) -> bool {
        if offset_part < 0 {
            return false;
        }
        let index = (offset_part / 8) as usize;
        if index >= self.data.len() {
            return false;
        }
        (self.data[index] & (1 << (offset_part % 8))) != 0
    }

    /// Counts consecutive ready parts starting from offset_part.
    ///
    /// # Arguments
    ///
    /// * `offset_part` - The starting part index
    #[must_use]
    pub fn get_ready_parts(&self, offset_part: i64) -> i64 {
        let mut res = 0i64;
        while self.get(offset_part + res) {
            res += 1;
        }
        res
    }

    /// Converts the bitmask to a vector of set part indices.
    #[must_use]
    pub fn as_vector(&self) -> Vec<i32> {
        let mut res = Vec::new();
        let size = (self.data.len() * 8) as i32;
        for i in 0..size {
            if self.get(i as i64) {
                res.push(i);
            }
        }
        res
    }

    /// Sets a specific part as ready (sets the bit).
    ///
    /// # Arguments
    ///
    /// * `offset_part` - The part index to set
    pub fn set(&mut self, offset_part: i64) {
        if offset_part < 0 {
            return;
        }
        let need_size = (offset_part / 8 + 1) as usize;
        if need_size > self.data.len() {
            self.data.resize(need_size, 0);
        }
        self.data[need_size - 1] |= 1 << (offset_part % 8);
    }

    /// Returns the total number of parts the bitmask can represent.
    #[must_use]
    pub fn size(&self) -> i64 {
        (self.data.len() * 8) as i64
    }

    /// Compresses the bitmask by a factor of k.
    ///
    /// # Arguments
    ///
    /// * `k` - Compression factor
    #[must_use]
    pub fn compress(&self, k: i32) -> Self {
        let mut res = Self::empty();
        for i in 0..self.size() / k as i64 {
            let mut all_set = true;
            for j in 0..k {
                if !self.get(i * k as i64 + j as i64) {
                    all_set = false;
                    break;
                }
            }
            if all_set {
                res.set(i);
            }
        }
        res
    }

    /// Returns `true` if the bitmask is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl fmt::Display for FileBitmask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut prev = false;
        let mut cnt = 0i32;
        for i in 0..=self.size() {
            let cur = self.get(i);
            if cur != prev {
                if cnt < 5 {
                    for _ in 0..cnt {
                        write!(f, "{}", if prev { '1' } else { '0' })?;
                    }
                } else {
                    write!(f, "{}(x{})", if prev { '1' } else { '0' }, cnt)?;
                }
                cnt = 0;
            }
            prev = cur;
            cnt += 1;
        }
        Ok(())
    }
}

/// Zero-one encoding: compresses runs of 0x00 or 0xFF bytes.
#[must_use]
pub fn zero_one_encode(data: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    let mut i = 0;
    while i < data.len() {
        res.push(data[i]);
        let c = data[i];
        if c == 0 || c == 0xFF {
            let mut cnt = 1u8;
            while (cnt < 250)
                && (i + (cnt as usize) < data.len())
                && (data[i + (cnt as usize)] == c)
            {
                cnt += 1;
            }
            res.push(cnt);
            i += (cnt as usize) - 1;
        }
        i += 1;
    }
    res
}

/// Zero-one decoding: decompresses runs of 0x00 or 0xFF bytes.
#[must_use]
pub fn zero_one_decode(data: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let c = data[i];
        if (c == 0 || c == 0xFF) && i + 1 < data.len() {
            let cnt = data[i + 1];
            for _ in 0..cnt {
                res.push(c);
            }
            i += 2;
            continue;
        }
        res.push(c);
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === Basic constructor tests ===

    #[test]
    fn test_empty() {
        let mask = FileBitmask::empty();
        assert!(mask.is_empty());
        assert!(!mask.get(0));
    }

    #[test]
    fn test_default() {
        let mask = FileBitmask::default();
        assert!(mask.is_empty());
    }

    // === with_ones tests ===

    #[test]
    fn test_with_ones() {
        let mask = FileBitmask::with_ones(BitmaskOnes, 10);
        assert_eq!(mask.size(), 16); // Rounded up to 8 bits
        for i in 0..10 {
            assert!(mask.get(i), "Bit {} should be set", i);
        }
        assert!(!mask.get(10));
    }

    #[test]
    fn test_with_ones_empty() {
        let mask = FileBitmask::with_ones(BitmaskOnes, 0);
        assert!(mask.is_empty());
    }

    // === get/set tests ===

    #[test]
    fn test_get_set() {
        let mut mask = FileBitmask::empty();
        assert!(!mask.get(0));

        mask.set(0);
        assert!(mask.get(0));
        assert!(!mask.get(1));

        mask.set(10);
        assert!(mask.get(10));

        mask.set(15);
        assert!(mask.get(15));
    }

    #[test]
    fn test_set_expands() {
        let mut mask = FileBitmask::empty();
        mask.set(20);
        assert_eq!(mask.size(), 24); // Expanded to hold bit 20
        assert!(mask.get(20));
    }

    #[rstest]
    #[case(-1, false)]
    #[case(0, false)]
    #[case(1, true)]
    fn test_get_bounds(#[case] offset: i64, #[case] expected: bool) {
        let mut mask = FileBitmask::empty();
        mask.set(1);
        assert_eq!(mask.get(offset), expected);
    }

    // === get_ready_parts tests ===

    #[test]
    fn test_get_ready_parts() {
        let mut mask = FileBitmask::empty();
        mask.set(0);
        mask.set(1);
        mask.set(2);
        mask.set(4); // Gap at 3

        assert_eq!(mask.get_ready_parts(0), 3);
        assert_eq!(mask.get_ready_parts(1), 2);
        assert_eq!(mask.get_ready_parts(3), 0);
        assert_eq!(mask.get_ready_parts(4), 1);
    }

    // === get_ready_prefix_size tests ===

    #[test]
    fn test_get_ready_prefix_size() {
        let mut mask = FileBitmask::empty();
        mask.set(0);
        mask.set(1);
        mask.set(2);

        // 3 parts ready, each 100 bytes = 300 bytes
        assert_eq!(mask.get_ready_prefix_size(0, 100, 0), 300);
        assert_eq!(mask.get_ready_prefix_size(50, 100, 0), 250); // From offset 50
        assert_eq!(mask.get_ready_prefix_size(200, 100, 0), 100); // From offset 200
    }

    #[test]
    fn test_get_ready_prefix_size_with_file_limit() {
        let mut mask = FileBitmask::empty();
        mask.set(0);
        mask.set(1);
        mask.set(2);

        // 3 parts ready, but file is only 250 bytes
        assert_eq!(mask.get_ready_prefix_size(0, 100, 250), 250);
    }

    // === get_total_size tests ===

    #[test]
    fn test_get_total_size() {
        let mut mask = FileBitmask::empty();
        mask.set(0);
        mask.set(2);
        mask.set(4); // Non-contiguous

        // 3 parts x 100 bytes each
        assert_eq!(mask.get_total_size(100, 0), 300);
    }

    #[test]
    fn test_get_total_size_with_file_limit() {
        let mut mask = FileBitmask::empty();
        mask.set(0);
        mask.set(1);

        // 2 parts, but file is only 150 bytes
        assert_eq!(mask.get_total_size(100, 150), 150);
    }

    // === as_vector tests ===

    #[test]
    fn test_as_vector() {
        let mut mask = FileBitmask::empty();
        mask.set(0);
        mask.set(2);
        mask.set(5);

        assert_eq!(mask.as_vector(), vec![0, 2, 5]);
    }

    #[test]
    fn test_as_vector_empty() {
        let mask = FileBitmask::empty();
        assert!(mask.as_vector().is_empty());
    }

    // === compress tests ===

    #[test]
    fn test_compress() {
        let mut mask = FileBitmask::empty();
        for i in 0..16 {
            mask.set(i);
        }

        let compressed = mask.compress(4);
        // All 16 bits set, compressed by 4 = 4 bits set
        assert_eq!(compressed.as_vector(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_compress_with_gaps() {
        let mut mask = FileBitmask::empty();
        for i in 0..12 {
            mask.set(i);
        }
        // Skip 12-15
        for i in 16..20 {
            mask.set(i);
        }

        let compressed = mask.compress(4);
        // First 3 groups of 4 set, 4th group not set, 5th group set
        assert_eq!(compressed.as_vector(), vec![0, 1, 2, 4]);
    }

    // === encode/decode tests ===

    #[test]
    fn test_encode_decode() {
        let original = FileBitmask::with_ones(BitmaskOnes, 100);
        let encoded = original.encode(-1);
        let decoded = FileBitmask::with_decode(BitmaskDecode, &encoded);

        for i in 0..100 {
            assert_eq!(decoded.get(i), original.get(i), "Bit {} mismatch", i);
        }
    }

    #[test]
    fn test_encode_with_prefix() {
        let mask = FileBitmask::with_ones(BitmaskOnes, 100);
        // Encode with only first 50 bits
        let encoded = mask.encode(50);
        let decoded = FileBitmask::with_decode(BitmaskDecode, &encoded);

        for i in 0..50 {
            assert!(decoded.get(i), "Bit {} should be set", i);
        }
    }

    // === zero_one_encode/decode tests ===

    #[test]
    fn test_zero_one_encode_simple() {
        assert_eq!(zero_one_encode(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_zero_one_encode_zeros() {
        assert_eq!(zero_one_encode(&[0, 0, 0, 0]), vec![0, 4]);
    }

    #[test]
    fn test_zero_one_encode_255() {
        assert_eq!(zero_one_encode(&[0xFF, 0xFF, 0xFF]), vec![0xFF, 3]);
    }

    #[test]
    fn test_zero_one_encode_mixed() {
        assert_eq!(zero_one_encode(&[1, 0, 0, 2]), vec![1, 0, 2, 2]);
    }

    #[test]
    fn test_zero_one_encode_limit() {
        // Should split at 250
        let data = vec![0u8; 300];
        let encoded = zero_one_encode(&data);
        // First 250 zeros, then remaining 50
        assert_eq!(encoded, vec![0, 250, 0, 50]);
    }

    #[test]
    fn test_zero_one_decode_zeros() {
        assert_eq!(zero_one_decode(&[0, 3]), vec![0, 0, 0]);
    }

    #[test]
    fn test_zero_one_decode_255() {
        assert_eq!(zero_one_decode(&[0xFF, 2]), vec![0xFF, 0xFF]);
    }

    #[test]
    fn test_zero_decode_roundtrip() {
        let original = vec![1u8, 2, 3, 0, 0, 0, 4, 0xFF, 0xFF, 5];
        let encoded = zero_one_encode(&original);
        let decoded = zero_one_decode(&encoded);
        assert_eq!(decoded, original);
    }

    // === Display tests ===

    #[test]
    fn test_display_empty() {
        let mask = FileBitmask::empty();
        // Empty mask has no bits set, so output is empty
        assert_eq!(format!("{mask}"), "");
    }

    #[test]
    fn test_display_simple() {
        let mut mask = FileBitmask::empty();
        mask.set(0);
        mask.set(1);
        let s = format!("{mask}");
        assert!(s.contains('1'));
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let mask = FileBitmask::with_ones(BitmaskOnes, 10);
        let json = serde_json::to_string(&mask).unwrap();
        assert!(json.contains("data"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"data":[255,255]}"#;
        let mask: FileBitmask = serde_json::from_str(json).unwrap();
        assert_eq!(mask.size(), 16);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = FileBitmask::with_ones(BitmaskOnes, 50);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FileBitmask = serde_json::from_str(&json).unwrap();
        for i in 0..original.size() {
            assert_eq!(deserialized.get(i), original.get(i));
        }
    }

    // === Size tests ===

    #[test]
    fn test_size() {
        let mut mask = FileBitmask::empty();
        assert_eq!(mask.size(), 0);

        mask.set(0);
        assert_eq!(mask.size(), 8);

        mask.set(15);
        assert_eq!(mask.size(), 16);
    }
}
