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

//! # Parts Manager
//!
//! Manages file part tracking for chunked uploads and downloads.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `PartsManager` class from `td/telegram/files/PartsManager.h`.
//!
//! ## Overview
//!
//! The PartsManager tracks which parts of a file have been uploaded/downloaded
//! and manages the state of chunked file transfers. It supports:
//!
//! - **Part tracking**: Track which parts are ready, pending, or empty
//! - **Bitmask management**: Convert to/from bitmask format for serialization
//! - **Streaming support**: Handle partial file downloads with offset/limit
//! - **Size validation**: Validate file sizes against expected values
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_parts_manager::{PartsManager, Part, PartStatus, InitOptions};
//!
//! // Initialize for a 10MB file with 512KB parts
//! let mut manager = PartsManager::new();
//! manager.init(
//!     InitOptions {
//!         size: 10_000_000,
//!         expected_size: 10_000_000,
//!         is_size_final: true,
//!         part_size: 512 * 1024,
//!         use_part_count_limit: true,
//!         is_upload: false,
//!     },
//!     &[],
//! )?;
//!
//! // Get next part to download
//! let part = manager.start_part()?;
//! if !part.is_empty() {
//!     // Download part...
//!     manager.on_part_ok(part.id, 512 * 1024, 512 * 1024)?;
//! }
//!
//! // Check if download is complete
//! if manager.ready() {
//!     manager.finish()?;
//! }
//! # Ok::<(), rustgram_parts_manager::Error>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use rustgram_file_bitmask::FileBitmask;
use std::fmt;

pub use error::Error;
pub use part::Part;

mod error;
mod part;

/// Maximum part count for premium users.
const MAX_PART_COUNT_PREMIUM: i32 = 8000;

/// Maximum part size (512 KB).
const MAX_PART_SIZE: usize = 512 << 10; // 512 * 1024

/// Maximum file size based on max parts * max part size.
const MAX_FILE_SIZE: i64 = (MAX_PART_SIZE as i64) * (MAX_PART_COUNT_PREMIUM as i64);

/// Status of a part in the file transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PartStatus {
    /// Part is empty (not downloaded/uploaded)
    #[default]
    Empty = 0,
    /// Part is pending (in progress)
    Pending = 1,
    /// Part is ready (completed)
    Ready = 2,
}

impl PartStatus {
    /// Returns `true` if the part is ready.
    #[must_use]
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns `true` if the part is pending.
    #[must_use]
    pub const fn is_pending(self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if the part is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

/// Result type for parts manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Manages file parts for chunked uploads and downloads.
///
/// Tracks which parts of a file have been transferred and manages the
/// state of chunked file operations.
#[derive(Debug, Clone, Default)]
pub struct PartsManager {
    /// Whether this is an upload operation
    is_upload: bool,
    /// Whether the file needs verification
    need_check: bool,
    /// Size of verified prefix
    checked_prefix_size: i64,
    /// Whether known prefix flag is set
    known_prefix_flag: bool,
    /// Known prefix size
    known_prefix_size: i64,
    /// Total file size
    size: i64,
    /// Expected file size
    expected_size: i64,
    /// Whether file size is unknown
    unknown_size_flag: bool,
    /// Total ready size
    ready_size: i64,
    /// Streaming ready size
    streaming_ready_size: i64,
    /// Part size in bytes
    part_size: usize,
    /// Number of parts
    part_count: i32,
    /// Number of pending parts
    pending_count: i32,
    /// First empty part index
    first_empty_part: i32,
    /// First not ready part index
    first_not_ready_part: i32,
    /// Streaming offset in bytes
    streaming_offset: i64,
    /// Streaming limit in bytes
    streaming_limit: i64,
    /// First streaming empty part
    first_streaming_empty_part: i32,
    /// First streaming not ready part
    first_streaming_not_ready_part: i32,
    /// Status of each part
    part_status: Vec<PartStatus>,
    /// Bitmask representation
    bitmask: FileBitmask,
    /// Whether to use part count limit
    use_part_count_limit: bool,
}

/// Initialization options for PartsManager.
///
/// Groups configuration parameters for file transfer initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitOptions {
    /// Total file size (0 if unknown)
    pub size: i64,
    /// Expected file size
    pub expected_size: i64,
    /// Whether the size is final (won't change)
    pub is_size_final: bool,
    /// Size of each part
    pub part_size: usize,
    /// Whether to enforce part count limits
    pub use_part_count_limit: bool,
    /// Whether this is an upload operation
    pub is_upload: bool,
}

impl PartsManager {
    /// Creates a new parts manager.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            is_upload: false,
            need_check: false,
            checked_prefix_size: 0,
            known_prefix_flag: false,
            known_prefix_size: 0,
            size: 0,
            expected_size: 0,
            unknown_size_flag: false,
            ready_size: 0,
            streaming_ready_size: 0,
            part_size: 0,
            part_count: 0,
            pending_count: 0,
            first_empty_part: 0,
            first_not_ready_part: 0,
            streaming_offset: 0,
            streaming_limit: 0,
            first_streaming_empty_part: 0,
            first_streaming_not_ready_part: 0,
            part_status: Vec::new(),
            bitmask: FileBitmask::empty(),
            use_part_count_limit: false,
        }
    }

    /// Initializes the parts manager for a file transfer.
    ///
    /// # Arguments
    ///
    /// * `options` - Initialization options including size, part_size, and flags
    /// * `ready_parts` - Indices of already ready parts
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File size exceeds maximum
    /// - Part size is invalid
    /// - Ready parts contain invalid indices
    pub fn init(&mut self, options: InitOptions, ready_parts: &[i32]) -> Result<()> {
        if options.part_size > MAX_PART_SIZE {
            return Err(Error::InvalidPartSize);
        }

        if options.size > MAX_FILE_SIZE {
            return Err(Error::FileTooLarge);
        }

        self.is_upload = options.is_upload;
        self.use_part_count_limit = options.use_part_count_limit;
        self.part_size = options.part_size;
        self.expected_size = options.expected_size;

        if options.size != 0 {
            self.size = options.size;
            self.part_count = ((options.size + (options.part_size as i64) - 1) / (options.part_size as i64)) as i32;
        } else {
            self.unknown_size_flag = true;
            self.part_count = 0;
        }

        self.part_status = vec![PartStatus::Empty; self.part_count as usize];
        self.bitmask = FileBitmask::empty();

        self.init_common(ready_parts)?;
        self.update_first_empty_part();
        self.update_first_not_ready_part();

        if options.is_size_final && options.size != 0 {
            self.known_prefix_flag = true;
            self.known_prefix_size = options.size;
        }

        Ok(())
    }

    /// Returns `true` if the file transfer may finish (all parts accounted for).
    #[must_use]
    pub fn may_finish(&self) -> bool {
        if self.part_count == 0 {
            return false;
        }
        if self.unknown_size_flag {
            return true;
        }
        self.checked_prefix_size >= self.size
    }

    /// Returns `true` if all parts are ready.
    #[must_use]
    pub fn ready(&self) -> bool {
        if !self.may_finish() {
            return false;
        }
        if self.need_check && self.checked_prefix_size < self.known_prefix_size {
            return false;
        }
        self.first_not_ready_part >= self.part_count
    }

    /// Returns `true` if unchecked parts are ready (before verification).
    #[must_use]
    pub fn unchecked_ready(&self) -> bool {
        self.first_not_ready_part >= self.part_count
    }

    /// Marks the transfer as finished.
    ///
    /// # Errors
    ///
    /// Returns an error if not all parts are ready.
    pub fn finish(&mut self) -> Result<()> {
        if !self.ready() {
            return Err(Error::NotReady);
        }
        Ok(())
    }

    /// Gets the next part to process.
    ///
    /// Returns an empty part if there are no more parts to process.
    ///
    /// # Errors
    ///
    /// Returns an error if the parts manager is not initialized.
    pub fn start_part(&mut self) -> Result<Part> {
        if self.part_count == 0 {
            return Ok(Part::empty());
        }

        let part_id = if self.is_streaming_limit_reached() {
            self.get_part(self.first_streaming_empty_part)
        } else {
            self.get_part(self.first_empty_part)
        };

        if part_id.id < 0 {
            return Ok(Part::empty());
        }

        self.on_part_start(part_id.id);

        Ok(part_id)
    }

    /// Marks a part as successfully completed.
    ///
    /// # Arguments
    ///
    /// * `part_id` - The part ID
    /// * `part_size` - The size of this part
    /// * `actual_size` - The actual size transferred
    ///
    /// # Errors
    ///
    /// Returns an error if the part ID is invalid.
    pub fn on_part_ok(
        &mut self,
        part_id: i32,
        _part_size: usize,
        actual_size: usize,
    ) -> Result<()> {
        if part_id < 0 || part_id >= self.part_count {
            return Err(Error::InvalidPartId);
        }

        let status = &mut self.part_status[part_id as usize];
        if *status == PartStatus::Pending {
            self.pending_count -= 1;
        }

        *status = PartStatus::Ready;
        self.bitmask.set(part_id as i64);
        self.ready_size += actual_size as i64;

        if part_id == self.first_empty_part {
            self.update_first_empty_part();
        }
        if part_id == self.first_not_ready_part {
            self.update_first_not_ready_part();
        }

        Ok(())
    }

    /// Marks a part as failed.
    ///
    /// # Arguments
    ///
    /// * `part_id` - The part ID
    pub fn on_part_failed(&mut self, part_id: i32) {
        if part_id < 0 || part_id >= self.part_count {
            return;
        }

        let status = &mut self.part_status[part_id as usize];
        if *status == PartStatus::Pending {
            self.pending_count -= 1;
        }

        *status = PartStatus::Empty;
    }

    /// Sets the known prefix size.
    ///
    /// # Arguments
    ///
    /// * `size` - The known prefix size
    /// * `is_ready` - Whether the prefix is ready
    ///
    /// # Errors
    ///
    /// Returns an error if the size is invalid.
    pub fn set_known_prefix(&mut self, size: i64, is_ready: bool) -> Result<()> {
        if size < 0 {
            return Err(Error::InvalidSize);
        }

        self.known_prefix_flag = true;
        self.known_prefix_size = size;

        if is_ready {
            self.checked_prefix_size = size;
        }

        Ok(())
    }

    /// Sets that the file needs verification.
    pub fn set_need_check(&mut self) {
        self.need_check = true;
    }

    /// Sets the checked prefix size.
    ///
    /// # Arguments
    ///
    /// * `size` - The checked prefix size
    pub fn set_checked_prefix_size(&mut self, size: i64) {
        self.checked_prefix_size = size;
    }

    /// Sets the streaming offset and returns the number of parts to skip.
    ///
    /// # Arguments
    ///
    /// * `offset` - The streaming offset in bytes
    /// * `limit` - The streaming limit in bytes
    ///
    /// # Returns
    ///
    /// The number of parts to skip
    pub fn set_streaming_offset(&mut self, offset: i64, limit: i64) -> i32 {
        self.streaming_offset = offset;
        self.streaming_limit = limit;

        let part_size = self.part_size as i64;
        if part_size == 0 {
            return 0;
        }

        let offset_part = offset / part_size;
        self.first_streaming_empty_part = offset_part as i32;
        self.first_streaming_not_ready_part = offset_part as i32;

        offset_part as i32
    }

    /// Sets the streaming limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - The streaming limit in bytes
    pub fn set_streaming_limit(&mut self, limit: i64) {
        self.streaming_limit = limit;
    }

    /// Gets the checked prefix size.
    #[must_use]
    pub const fn get_checked_prefix_size(&self) -> i64 {
        self.checked_prefix_size
    }

    /// Gets the unchecked ready prefix size.
    #[must_use]
    pub fn get_unchecked_ready_prefix_size(&mut self) -> i64 {
        self.bitmask
            .get_ready_prefix_size(self.streaming_offset, self.part_size as i64, self.size)
    }

    /// Gets the file size.
    #[must_use]
    pub const fn get_size(&self) -> i64 {
        self.size
    }

    /// Gets the size or zero if unknown.
    #[must_use]
    pub const fn get_size_or_zero(&self) -> i64 {
        self.size
    }

    /// Gets the estimated extra size.
    #[must_use]
    pub const fn get_estimated_extra(&self) -> i64 {
        0
    }

    /// Gets the ready size.
    #[must_use]
    pub const fn get_ready_size(&self) -> i64 {
        self.ready_size
    }

    /// Gets the part size.
    #[must_use]
    pub const fn get_part_size(&self) -> usize {
        self.part_size
    }

    /// Gets the part count.
    #[must_use]
    pub const fn get_part_count(&self) -> i32 {
        self.part_count
    }

    /// Gets the unchecked ready prefix count.
    #[must_use]
    pub fn get_unchecked_ready_prefix_count(&mut self) -> i32 {
        let prefix_size = self.get_unchecked_ready_prefix_size();
        (prefix_size / self.part_size as i64) as i32
    }

    /// Gets the ready prefix count.
    #[must_use]
    pub const fn get_ready_prefix_count(&self) -> i32 {
        self.first_not_ready_part
    }

    /// Gets the streaming offset.
    #[must_use]
    pub const fn get_streaming_offset(&self) -> i64 {
        self.streaming_offset
    }

    /// Gets the bitmask.
    #[must_use]
    pub fn get_bitmask(&mut self) -> String {
        // Encode the bitmask as hex string
        let encoded = self.bitmask.encode(-1);
        hex_encode(&encoded)
    }

    // Private methods

    fn init_common(&mut self, ready_parts: &[i32]) -> Result<()> {
        self.ready_size = 0;

        if !ready_parts.is_empty() {
            let part_size = self.part_size as i64;
            for &part_id in ready_parts {
                if part_id < 0 || part_id >= self.part_count {
                    return Err(Error::InvalidPartId);
                }

                if self.part_status[part_id as usize] == PartStatus::Empty {
                    self.part_status[part_id as usize] = PartStatus::Ready;
                    self.bitmask.set(part_id as i64);

                    let mut part_size = part_size;
                    if self.size != 0 {
                        let offset = part_id as i64 * part_size;
                        if offset + part_size > self.size {
                            part_size = self.size - offset;
                        }
                    }
                    self.ready_size += part_size;
                }
            }
        }

        Ok(())
    }

    fn get_part(&self, part_id: i32) -> Part {
        if part_id < 0 || part_id >= self.part_count {
            return Part::empty();
        }

        let offset = part_id as i64 * self.part_size as i64;
        let mut size = self.part_size;

        if self.size != 0 {
            let remaining = self.size - offset;
            if remaining < size as i64 {
                size = remaining as usize;
            }
        }

        Part::new(part_id, offset, size)
    }

    fn on_part_start(&mut self, part_id: i32) {
        if part_id >= 0 && part_id < self.part_count {
            let status = &mut self.part_status[part_id as usize];
            if *status == PartStatus::Empty {
                *status = PartStatus::Pending;
                self.pending_count += 1;
            }
        }
    }

    fn update_first_empty_part(&mut self) {
        self.first_empty_part = self
            .part_status
            .iter()
            .position(|&s| s == PartStatus::Empty)
            .unwrap_or(self.part_count as usize) as i32;
    }

    fn update_first_not_ready_part(&mut self) {
        self.first_not_ready_part = self
            .part_status
            .iter()
            .position(|&s| !s.is_ready())
            .unwrap_or(self.part_count as usize) as i32;
    }

    fn is_streaming_limit_reached(&self) -> bool {
        if self.streaming_limit == 0 {
            return false;
        }
        self.streaming_ready_size >= self.streaming_limit
    }
}

/// Helper to encode bytes as hex string.
#[must_use]
fn hex_encode(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .concat()
}

impl fmt::Display for PartsManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PartsManager(size={}, ready={}/{}, pending={})",
            self.size, self.first_not_ready_part, self.part_count, self.pending_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Constructor tests ===

    #[test]
    fn test_new() {
        let manager = PartsManager::new();
        assert_eq!(manager.get_size(), 0);
        assert_eq!(manager.get_part_count(), 0);
        assert!(!manager.ready());
    }

    #[test]
    fn test_default() {
        let manager = PartsManager::default();
        assert_eq!(manager.get_size(), 0);
    }

    // === Init tests ===

    #[test]
    fn test_init_basic() {
        let mut manager = PartsManager::new();
        let result = manager.init(
            InitOptions {
                size: 10_000_000,
                expected_size: 10_000_000,
                is_size_final: true,
                part_size: 512 * 1024,
                use_part_count_limit: true,
                is_upload: false,
            },
            &[],
        );
        assert!(result.is_ok());
        assert_eq!(manager.get_size(), 10_000_000);
        assert_eq!(manager.get_part_size(), 512 * 1024);
        assert!(!manager.is_upload);
    }

    #[test]
    fn test_init_upload() {
        let mut manager = PartsManager::new();
        let result = manager.init(
            InitOptions {
                size: 10_000_000,
                expected_size: 10_000_000,
                is_size_final: true,
                part_size: 512 * 1024,
                use_part_count_limit: true,
                is_upload: true,
            },
            &[],
        );
        assert!(result.is_ok());
        assert!(manager.is_upload);
    }

    #[test]
    fn test_init_with_ready_parts() {
        let mut manager = PartsManager::new();
        let ready_parts = vec![0, 1, 2];
        let result = manager.init(
            InitOptions {
                size: 1_500_000,
                expected_size: 1_500_000,
                is_size_final: true,
                part_size: 512 * 1024,
                use_part_count_limit: true,
                is_upload: false,
            },
            &ready_parts,
        );
        assert!(result.is_ok());
        assert_eq!(manager.get_ready_size(), 1_500_000); // 3 parts of 512KB
    }

    #[test]
    fn test_init_invalid_part_size() {
        let mut manager = PartsManager::new();
        let result = manager.init(
            InitOptions {
                size: 10_000_000,
                expected_size: 10_000_000,
                is_size_final: true,
                part_size: (512 << 10) + 1,
                use_part_count_limit: true,
                is_upload: false,
            },
            &[],
        );
        assert!(matches!(result, Err(Error::InvalidPartSize)));
    }

    #[test]
    fn test_init_file_too_large() {
        let mut manager = PartsManager::new();
        let result = manager.init(
            InitOptions {
                size: MAX_FILE_SIZE + 1,
                expected_size: MAX_FILE_SIZE + 1,
                is_size_final: true,
                part_size: 512 * 1024,
                use_part_count_limit: true,
                is_upload: false,
            },
            &[],
        );
        assert!(matches!(result, Err(Error::FileTooLarge)));
    }

    // === Part processing tests ===

    #[test]
    fn test_start_part() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        let part = manager.start_part().unwrap();
        assert_eq!(part.id, 0);
        assert_eq!(part.offset, 0);
        assert_eq!(part.size, 100 * 1024);
    }

    #[test]
    fn test_start_part_empty() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 100 * 1024,
                    expected_size: 100 * 1024,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        // Mark the only part as ready
        manager.on_part_ok(0, 100 * 1024, 100 * 1024).unwrap();

        let part = manager.start_part().unwrap();
        assert!(part.is_empty());
    }

    #[test]
    fn test_on_part_ok() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        manager.on_part_ok(0, 100 * 1024, 100 * 1024).unwrap();
        assert_eq!(manager.get_ready_size(), 100 * 1024);
        assert!(manager.bitmask.get(0));
    }

    #[test]
    fn test_on_part_ok_invalid_id() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        let result = manager.on_part_ok(999, 100 * 1024, 100 * 1024);
        assert!(matches!(result, Err(Error::InvalidPartId)));
    }

    #[test]
    fn test_on_part_failed() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        // Start a part (marks as pending)
        let _part = manager.start_part().unwrap();

        // Fail it
        manager.on_part_failed(0);
        assert_eq!(manager.part_status[0], PartStatus::Empty);
    }

    // === Ready/finish tests ===

    #[test]
    fn test_ready_empty() {
        let manager = PartsManager::new();
        assert!(!manager.ready());
        assert!(!manager.may_finish());
    }

    #[test]
    fn test_ready_partial() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        manager.on_part_ok(0, 100 * 1024, 100 * 1024).unwrap();
        assert!(!manager.ready());
    }

    #[test]
    fn test_ready_complete() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 100 * 1024,
                    expected_size: 100 * 1024,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        manager.on_part_ok(0, 100 * 1024, 100 * 1024).unwrap();
        assert!(manager.ready());
        assert!(manager.may_finish());
    }

    #[test]
    fn test_finish_success() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 100 * 1024,
                    expected_size: 100 * 1024,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        manager.on_part_ok(0, 100 * 1024, 100 * 1024).unwrap();
        assert!(manager.finish().is_ok());
    }

    #[test]
    fn test_finish_not_ready() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        let result = manager.finish();
        assert!(matches!(result, Err(Error::NotReady)));
    }

    // === Streaming tests ===

    #[test]
    fn test_set_streaming_offset() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        let skip_count = manager.set_streaming_offset(200 * 1024, 1_000_000);
        assert_eq!(skip_count, 2); // Skip first 2 parts
        assert_eq!(manager.get_streaming_offset(), 200 * 1024);
    }

    #[test]
    fn test_set_streaming_limit() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        manager.set_streaming_limit(300 * 1024);
        assert_eq!(manager.streaming_limit, 300 * 1024);
    }

    // === Known prefix tests ===

    #[test]
    fn test_set_known_prefix() {
        let mut manager = PartsManager::new();
        manager.set_known_prefix(500 * 1024, true).unwrap();
        assert_eq!(manager.known_prefix_size, 500 * 1024);
        assert_eq!(manager.checked_prefix_size, 500 * 1024);
    }

    #[test]
    fn test_set_known_prefix_invalid() {
        let mut manager = PartsManager::new();
        let result = manager.set_known_prefix(-100, true);
        assert!(matches!(result, Err(Error::InvalidSize)));
    }

    // === Bitmask tests ===

    #[test]
    fn test_get_bitmask() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        manager.on_part_ok(0, 100 * 1024, 100 * 1024).unwrap();
        manager.on_part_ok(2, 100 * 1024, 100 * 1024).unwrap();

        let bitmask = manager.get_bitmask();
        // Should be a hex string
        assert!(!bitmask.is_empty());
    }

    // === Getter tests ===

    #[test]
    fn test_getters() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 10_000_000,
                    expected_size: 10_000_000,
                    is_size_final: true,
                    part_size: 512 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        assert_eq!(manager.get_size(), 10_000_000);
        assert_eq!(manager.get_size_or_zero(), 10_000_000);
        assert_eq!(manager.get_part_size(), 512 * 1024);
        assert_eq!(manager.get_part_count(), 20); // 10MB / 512KB
        assert_eq!(manager.get_estimated_extra(), 0);
    }

    // === Display tests ===

    #[test]
    fn test_display() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        let s = format!("{manager}");
        assert!(s.contains("PartsManager"));
        assert!(s.contains("1000000"));
    }

    // === Upload mode tests ===

    #[test]
    fn test_upload_mode() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 10_000_000,
                    expected_size: 10_000_000,
                    is_size_final: true,
                    part_size: 512 * 1024,
                    use_part_count_limit: true,
                    is_upload: true,
                },
                &[],
            )
            .unwrap();

        assert!(manager.is_upload);
        manager.on_part_ok(0, 512 * 1024, 512 * 1024).unwrap();
        assert_eq!(manager.get_ready_size(), 512 * 1024);
    }

    // === Multiple parts tests ===

    #[test]
    fn test_multiple_parts() {
        let mut manager = PartsManager::new();
        manager
            .init(
                InitOptions {
                    size: 1_000_000,
                    expected_size: 1_000_000,
                    is_size_final: true,
                    part_size: 100 * 1024,
                    use_part_count_limit: true,
                    is_upload: false,
                },
                &[],
            )
            .unwrap();

        // Complete all 10 parts
        for i in 0..10 {
            manager.on_part_ok(i, 100 * 1024, 100 * 1024).unwrap();
        }

        assert!(manager.ready());
        assert_eq!(manager.get_ready_size(), 1_000_000);
    }

    // === Need check tests ===

    #[test]
    fn test_set_need_check() {
        let mut manager = PartsManager::new();
        manager.set_need_check();
        assert!(manager.need_check);
    }

    #[test]
    fn test_set_checked_prefix_size() {
        let mut manager = PartsManager::new();
        manager.set_checked_prefix_size(500 * 1024);
        assert_eq!(manager.checked_prefix_size, 500 * 1024);
    }
}
