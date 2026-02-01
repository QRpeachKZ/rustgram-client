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

//! # File GC Parameters
//!
//! Garbage collection parameters for file cleanup.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileGcParameters` struct from `td/telegram/files/FileGcParameters.h`.
//!
//! ## Structure
//!
//! - `FileGcParameters`: Configuration for file garbage collection
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_file_gc_parameters::FileGcParameters;
//!
//! let params = FileGcParameters::new()
//!     .with_max_files_size(1024 * 1024 * 1024)
//!     .with_max_time_from_last_access(30 * 86400)
//!     .with_max_file_count(1000);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use rustgram_dialog_id::DialogId;
use rustgram_file_type::FileType;
use serde::{Deserialize, Serialize};
use std::fmt;

/// File garbage collection parameters.
///
/// Corresponds to TDLib `FileGcParameters` struct.
/// Controls which files should be garbage collected based on various criteria.
///
/// ## TDLib Mapping
///
/// - `max_files_size_` → `max_files_size()`
/// - `max_time_from_last_access_` → `max_time_from_last_access()`
/// - `max_file_count_` → `max_file_count()`
/// - `immunity_delay_` → `immunity_delay()`
/// - `file_types_` → `file_types()`
/// - `owner_dialog_ids_` → `owner_dialog_ids()`
/// - `exclude_owner_dialog_ids_` → `exclude_owner_dialog_ids()`
/// - `dialog_limit_` → `dialog_limit()`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileGcParameters {
    /// Maximum total size of files in bytes (-1 for no limit)
    max_files_size: i64,
    /// Maximum time since last access in seconds (-1 for no limit)
    max_time_from_last_access: i32,
    /// Maximum number of files (-1 for no limit)
    max_file_count: i32,
    /// Immunity delay in seconds (files newer than this are protected)
    immunity_delay: i32,
    /// File types to include in GC (empty = all types)
    file_types: Vec<FileType>,
    /// Dialog IDs whose files should be GC'd (empty = all dialogs)
    owner_dialog_ids: Vec<DialogId>,
    /// Dialog IDs whose files should be excluded from GC
    exclude_owner_dialog_ids: Vec<DialogId>,
    /// Maximum number of dialogs to process (0 = unlimited)
    dialog_limit: i32,
}

impl Default for FileGcParameters {
    /// Creates default GC parameters (all unlimited/no restrictions).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert_eq!(params.max_files_size(), -1);
    /// assert_eq!(params.max_time_from_last_access(), -1);
    /// assert_eq!(params.max_file_count(), -1);
    /// ```
    fn default() -> Self {
        Self {
            max_files_size: -1,
            max_time_from_last_access: -1,
            max_file_count: -1,
            immunity_delay: -1,
            file_types: Vec::new(),
            owner_dialog_ids: Vec::new(),
            exclude_owner_dialog_ids: Vec::new(),
            dialog_limit: 0,
        }
    }
}

impl FileGcParameters {
    /// Creates a new FileGcParameters with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new();
    /// assert_eq!(params.max_files_size(), -1);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder pattern: sets maximum total files size.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum total size in bytes (-1 for no limit)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_files_size(1024 * 1024 * 1024);
    /// assert_eq!(params.max_files_size(), 1024 * 1024 * 1024);
    /// ```
    #[must_use]
    pub fn with_max_files_size(mut self, size: i64) -> Self {
        self.max_files_size = size;
        self
    }

    /// Builder pattern: sets maximum time since last access.
    ///
    /// # Arguments
    ///
    /// * `seconds` - Maximum time in seconds (-1 for no limit)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_time_from_last_access(30 * 86400);
    /// assert_eq!(params.max_time_from_last_access(), 30 * 86400);
    /// ```
    #[must_use]
    pub fn with_max_time_from_last_access(mut self, seconds: i32) -> Self {
        self.max_time_from_last_access = seconds;
        self
    }

    /// Builder pattern: sets maximum file count.
    ///
    /// # Arguments
    ///
    /// * `count` - Maximum number of files (-1 for no limit)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_file_count(1000);
    /// assert_eq!(params.max_file_count(), 1000);
    /// ```
    #[must_use]
    pub fn with_max_file_count(mut self, count: i32) -> Self {
        self.max_file_count = count;
        self
    }

    /// Builder pattern: sets immunity delay.
    ///
    /// # Arguments
    ///
    /// * `seconds` - Immunity delay in seconds (-1 for no immunity)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_immunity_delay(7 * 86400);
    /// assert_eq!(params.immunity_delay(), 7 * 86400);
    /// ```
    #[must_use]
    pub fn with_immunity_delay(mut self, seconds: i32) -> Self {
        self.immunity_delay = seconds;
        self
    }

    /// Builder pattern: sets file types to include in GC.
    ///
    /// # Arguments
    ///
    /// * `types` - File types to include (empty = all types)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    /// use rustgram_file_type::FileType;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_file_types(vec![FileType::Photo, FileType::Video]);
    /// assert_eq!(params.file_types().len(), 2);
    /// ```
    #[must_use]
    pub fn with_file_types(mut self, types: Vec<FileType>) -> Self {
        self.file_types = types;
        self
    }

    /// Builder pattern: sets owner dialog IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - Dialog IDs whose files should be GC'd (empty = all)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_owner_dialog_ids(vec![DialogId::new(123), DialogId::new(456)]);
    /// assert_eq!(params.owner_dialog_ids().len(), 2);
    /// ```
    #[must_use]
    pub fn with_owner_dialog_ids(mut self, ids: Vec<DialogId>) -> Self {
        self.owner_dialog_ids = ids;
        self
    }

    /// Builder pattern: sets excluded owner dialog IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - Dialog IDs to exclude from GC
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_exclude_owner_dialog_ids(vec![DialogId::new(789)]);
    /// assert_eq!(params.exclude_owner_dialog_ids().len(), 1);
    /// ```
    #[must_use]
    pub fn with_exclude_owner_dialog_ids(mut self, ids: Vec<DialogId>) -> Self {
        self.exclude_owner_dialog_ids = ids;
        self
    }

    /// Builder pattern: sets dialog limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of dialogs to process (0 = unlimited)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_dialog_limit(100);
    /// assert_eq!(params.dialog_limit(), 100);
    /// ```
    #[must_use]
    pub fn with_dialog_limit(mut self, limit: i32) -> Self {
        self.dialog_limit = limit;
        self
    }

    /// Returns the maximum total files size in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_files_size(1024 * 1024 * 1024);
    /// assert_eq!(params.max_files_size(), 1024 * 1024 * 1024);
    /// ```
    #[must_use]
    pub const fn max_files_size(&self) -> i64 {
        self.max_files_size
    }

    /// Returns the maximum time since last access in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_time_from_last_access(30 * 86400);
    /// assert_eq!(params.max_time_from_last_access(), 30 * 86400);
    /// ```
    #[must_use]
    pub const fn max_time_from_last_access(&self) -> i32 {
        self.max_time_from_last_access
    }

    /// Returns the maximum file count.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_file_count(1000);
    /// assert_eq!(params.max_file_count(), 1000);
    /// ```
    #[must_use]
    pub const fn max_file_count(&self) -> i32 {
        self.max_file_count
    }

    /// Returns the immunity delay in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_immunity_delay(7 * 86400);
    /// assert_eq!(params.immunity_delay(), 7 * 86400);
    /// ```
    #[must_use]
    pub const fn immunity_delay(&self) -> i32 {
        self.immunity_delay
    }

    /// Returns the file types to include in GC.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    /// use rustgram_file_type::FileType;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_file_types(vec![FileType::Photo, FileType::Video]);
    /// assert_eq!(params.file_types().len(), 2);
    /// ```
    #[must_use]
    pub fn file_types(&self) -> &[FileType] {
        &self.file_types
    }

    /// Returns the owner dialog IDs.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_owner_dialog_ids(vec![DialogId::new(123)]);
    /// assert_eq!(params.owner_dialog_ids().len(), 1);
    /// ```
    #[must_use]
    pub fn owner_dialog_ids(&self) -> &[DialogId] {
        &self.owner_dialog_ids
    }

    /// Returns the excluded owner dialog IDs.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_exclude_owner_dialog_ids(vec![DialogId::new(789)]);
    /// assert_eq!(params.exclude_owner_dialog_ids().len(), 1);
    /// ```
    #[must_use]
    pub fn exclude_owner_dialog_ids(&self) -> &[DialogId] {
        &self.exclude_owner_dialog_ids
    }

    /// Returns the dialog limit.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::new()
    ///     .with_dialog_limit(100);
    /// assert_eq!(params.dialog_limit(), 100);
    /// ```
    #[must_use]
    pub const fn dialog_limit(&self) -> i32 {
        self.dialog_limit
    }

    /// Checks if size limit is disabled (set to -1).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert!(params.is_size_limit_disabled());
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_files_size(1024);
    /// assert!(!params.is_size_limit_disabled());
    /// ```
    #[must_use]
    pub fn is_size_limit_disabled(&self) -> bool {
        self.max_files_size < 0
    }

    /// Checks if time limit is disabled (set to -1).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert!(params.is_time_limit_disabled());
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_time_from_last_access(100);
    /// assert!(!params.is_time_limit_disabled());
    /// ```
    #[must_use]
    pub fn is_time_limit_disabled(&self) -> bool {
        self.max_time_from_last_access < 0
    }

    /// Checks if count limit is disabled (set to -1).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert!(params.is_count_limit_disabled());
    ///
    /// let params = FileGcParameters::new()
    ///     .with_max_file_count(100);
    /// assert!(!params.is_count_limit_disabled());
    /// ```
    #[must_use]
    pub fn is_count_limit_disabled(&self) -> bool {
        self.max_file_count < 0
    }

    /// Checks if immunity is disabled (set to -1).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert!(params.is_immunity_disabled());
    ///
    /// let params = FileGcParameters::new()
    ///     .with_immunity_delay(100);
    /// assert!(!params.is_immunity_disabled());
    /// ```
    #[must_use]
    pub fn is_immunity_disabled(&self) -> bool {
        self.immunity_delay < 0
    }

    /// Checks if dialog limit is unlimited (set to 0).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert!(params.is_dialog_limit_unlimited());
    ///
    /// let params = FileGcParameters::new()
    ///     .with_dialog_limit(100);
    /// assert!(!params.is_dialog_limit_unlimited());
    /// ```
    #[must_use]
    pub fn is_dialog_limit_unlimited(&self) -> bool {
        self.dialog_limit == 0
    }

    /// Checks if all file types are included (empty list).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert!(params.is_all_file_types());
    /// ```
    #[must_use]
    pub fn is_all_file_types(&self) -> bool {
        self.file_types.is_empty()
    }

    /// Checks if all dialogs are included (empty list).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_gc_parameters::FileGcParameters;
    ///
    /// let params = FileGcParameters::default();
    /// assert!(params.is_all_dialogs());
    /// ```
    #[must_use]
    pub fn is_all_dialogs(&self) -> bool {
        self.owner_dialog_ids.is_empty()
    }
}

impl fmt::Display for FileGcParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileGcParameters[max_size={}, max_time={}, max_count={}, immunity={}]",
            self.max_files_size,
            self.max_time_from_last_access,
            self.max_file_count,
            self.immunity_delay
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_file_type::FileType;

    // Basic trait tests
    #[test]
    fn test_default() {
        let params = FileGcParameters::default();
        assert_eq!(params.max_files_size(), -1);
        assert_eq!(params.max_time_from_last_access(), -1);
        assert_eq!(params.max_file_count(), -1);
        assert_eq!(params.immunity_delay(), -1);
        assert!(params.file_types().is_empty());
        assert!(params.owner_dialog_ids().is_empty());
        assert!(params.exclude_owner_dialog_ids().is_empty());
        assert_eq!(params.dialog_limit(), 0);
    }

    #[test]
    fn test_clone() {
        let params = FileGcParameters::new().with_max_files_size(1024);
        let cloned = params.clone();
        assert_eq!(params, cloned);
    }

    #[test]
    fn test_equality() {
        let params1 = FileGcParameters::new().with_max_files_size(1024);
        let params2 = FileGcParameters::new().with_max_files_size(1024);
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_inequality() {
        let params1 = FileGcParameters::new().with_max_files_size(1024);
        let params2 = FileGcParameters::new().with_max_files_size(2048);
        assert_ne!(params1, params2);
    }

    // Builder pattern tests
    #[test]
    fn test_builder_max_files_size() {
        let params = FileGcParameters::new().with_max_files_size(1024 * 1024 * 1024);
        assert_eq!(params.max_files_size(), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_builder_max_time_from_last_access() {
        let params = FileGcParameters::new().with_max_time_from_last_access(30 * 86400);
        assert_eq!(params.max_time_from_last_access(), 30 * 86400);
    }

    #[test]
    fn test_builder_max_file_count() {
        let params = FileGcParameters::new().with_max_file_count(1000);
        assert_eq!(params.max_file_count(), 1000);
    }

    #[test]
    fn test_builder_immunity_delay() {
        let params = FileGcParameters::new().with_immunity_delay(7 * 86400);
        assert_eq!(params.immunity_delay(), 7 * 86400);
    }

    #[test]
    fn test_builder_file_types() {
        let params =
            FileGcParameters::new().with_file_types(vec![FileType::Photo, FileType::Video]);
        assert_eq!(params.file_types().len(), 2);
    }

    #[test]
    fn test_builder_owner_dialog_ids() {
        let params = FileGcParameters::new()
            .with_owner_dialog_ids(vec![DialogId::new(123), DialogId::new(456)]);
        assert_eq!(params.owner_dialog_ids().len(), 2);
    }

    #[test]
    fn test_builder_exclude_owner_dialog_ids() {
        let params =
            FileGcParameters::new().with_exclude_owner_dialog_ids(vec![DialogId::new(789)]);
        assert_eq!(params.exclude_owner_dialog_ids().len(), 1);
    }

    #[test]
    fn test_builder_dialog_limit() {
        let params = FileGcParameters::new().with_dialog_limit(100);
        assert_eq!(params.dialog_limit(), 100);
    }

    // Chained builder tests
    #[test]
    fn test_builder_chain() {
        let params = FileGcParameters::new()
            .with_max_files_size(1024 * 1024 * 1024)
            .with_max_time_from_last_access(30 * 86400)
            .with_max_file_count(1000)
            .with_immunity_delay(7 * 86400);

        assert_eq!(params.max_files_size(), 1024 * 1024 * 1024);
        assert_eq!(params.max_time_from_last_access(), 30 * 86400);
        assert_eq!(params.max_file_count(), 1000);
        assert_eq!(params.immunity_delay(), 7 * 86400);
    }

    // Query method tests
    #[test]
    fn test_is_size_limit_disabled() {
        let params = FileGcParameters::default();
        assert!(params.is_size_limit_disabled());

        let params = FileGcParameters::new().with_max_files_size(1024);
        assert!(!params.is_size_limit_disabled());

        let params = FileGcParameters::new().with_max_files_size(-1);
        assert!(params.is_size_limit_disabled());
    }

    #[test]
    fn test_is_time_limit_disabled() {
        let params = FileGcParameters::default();
        assert!(params.is_time_limit_disabled());

        let params = FileGcParameters::new().with_max_time_from_last_access(100);
        assert!(!params.is_time_limit_disabled());

        let params = FileGcParameters::new().with_max_time_from_last_access(-1);
        assert!(params.is_time_limit_disabled());
    }

    #[test]
    fn test_is_count_limit_disabled() {
        let params = FileGcParameters::default();
        assert!(params.is_count_limit_disabled());

        let params = FileGcParameters::new().with_max_file_count(100);
        assert!(!params.is_count_limit_disabled());

        let params = FileGcParameters::new().with_max_file_count(-1);
        assert!(params.is_count_limit_disabled());
    }

    #[test]
    fn test_is_immunity_disabled() {
        let params = FileGcParameters::default();
        assert!(params.is_immunity_disabled());

        let params = FileGcParameters::new().with_immunity_delay(100);
        assert!(!params.is_immunity_disabled());

        let params = FileGcParameters::new().with_immunity_delay(-1);
        assert!(params.is_immunity_disabled());
    }

    #[test]
    fn test_is_dialog_limit_unlimited() {
        let params = FileGcParameters::default();
        assert!(params.is_dialog_limit_unlimited());

        let params = FileGcParameters::new().with_dialog_limit(100);
        assert!(!params.is_dialog_limit_unlimited());

        let params = FileGcParameters::new().with_dialog_limit(0);
        assert!(params.is_dialog_limit_unlimited());
    }

    #[test]
    fn test_is_all_file_types() {
        let params = FileGcParameters::default();
        assert!(params.is_all_file_types());

        let params = FileGcParameters::new().with_file_types(vec![FileType::Photo]);
        assert!(!params.is_all_file_types());
    }

    #[test]
    fn test_is_all_dialogs() {
        let params = FileGcParameters::default();
        assert!(params.is_all_dialogs());

        let params = FileGcParameters::new().with_owner_dialog_ids(vec![DialogId::new(123)]);
        assert!(!params.is_all_dialogs());
    }

    // Display tests
    #[test]
    fn test_display() {
        let params = FileGcParameters::new().with_max_files_size(1024);
        let display = format!("{}", params);
        assert!(display.contains("1024"));
        assert!(display.contains("FileGcParameters"));
    }

    // Serialization tests
    #[test]
    fn test_serialize_deserialize() {
        let params = FileGcParameters::new()
            .with_max_files_size(1024 * 1024 * 1024)
            .with_max_time_from_last_access(30 * 86400)
            .with_max_file_count(1000);

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: FileGcParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(params, deserialized);
    }

    #[test]
    fn test_serialize_with_dialog_ids() {
        let params = FileGcParameters::new()
            .with_owner_dialog_ids(vec![DialogId::new(123), DialogId::new(456)]);

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: FileGcParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(params, deserialized);
    }

    #[test]
    fn test_serialize_with_file_types() {
        let params = FileGcParameters::new().with_file_types(vec![
            FileType::Photo,
            FileType::Video,
            FileType::Document,
        ]);

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: FileGcParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(params, deserialized);
    }

    // Edge cases
    #[test]
    fn test_zero_values() {
        let params = FileGcParameters::new()
            .with_max_files_size(0)
            .with_max_time_from_last_access(0)
            .with_max_file_count(0)
            .with_immunity_delay(0)
            .with_dialog_limit(0);

        assert_eq!(params.max_files_size(), 0);
        assert_eq!(params.max_time_from_last_access(), 0);
        assert_eq!(params.max_file_count(), 0);
        assert_eq!(params.immunity_delay(), 0);
        assert_eq!(params.dialog_limit(), 0);
    }

    #[test]
    fn test_negative_values() {
        let params = FileGcParameters::new()
            .with_max_files_size(-1)
            .with_max_time_from_last_access(-1)
            .with_max_file_count(-1)
            .with_immunity_delay(-1);

        assert_eq!(params.max_files_size(), -1);
        assert_eq!(params.max_time_from_last_access(), -1);
        assert_eq!(params.max_file_count(), -1);
        assert_eq!(params.immunity_delay(), -1);
    }

    #[test]
    fn test_max_values() {
        let params = FileGcParameters::new()
            .with_max_files_size(i64::MAX)
            .with_max_time_from_last_access(i32::MAX)
            .with_max_file_count(i32::MAX)
            .with_immunity_delay(i32::MAX)
            .with_dialog_limit(i32::MAX);

        assert_eq!(params.max_files_size(), i64::MAX);
        assert_eq!(params.max_time_from_last_access(), i32::MAX);
        assert_eq!(params.max_file_count(), i32::MAX);
        assert_eq!(params.immunity_delay(), i32::MAX);
        assert_eq!(params.dialog_limit(), i32::MAX);
    }

    // GC scenario tests
    #[test]
    fn test_aggressive_gc() {
        // Aggressive GC: clean up everything old
        let params = FileGcParameters::new()
            .with_max_files_size(100 * 1024 * 1024) // 100 MB
            .with_max_time_from_last_access(7 * 86400) // 7 days
            .with_max_file_count(500)
            .with_immunity_delay(1 * 86400); // 1 day immunity

        assert!(!params.is_size_limit_disabled());
        assert!(!params.is_time_limit_disabled());
        assert!(!params.is_count_limit_disabled());
        assert!(!params.is_immunity_disabled());
    }

    #[test]
    fn test_conservative_gc() {
        // Conservative GC: only clean up very old/large
        let params = FileGcParameters::new()
            .with_max_files_size(1024 * 1024 * 1024) // 1 GB
            .with_max_time_from_last_access(90 * 86400) // 90 days
            .with_max_file_count(10000)
            .with_immunity_delay(30 * 86400); // 30 day immunity

        assert_eq!(params.max_files_size(), 1024 * 1024 * 1024);
        assert_eq!(params.max_time_from_last_access(), 90 * 86400);
    }

    #[test]
    fn test_no_gc() {
        // No GC: keep everything
        let params = FileGcParameters::new()
            .with_max_files_size(-1)
            .with_max_time_from_last_access(-1)
            .with_max_file_count(-1);

        assert!(params.is_size_limit_disabled());
        assert!(params.is_time_limit_disabled());
        assert!(params.is_count_limit_disabled());
    }

    #[test]
    fn test_specific_dialog_gc() {
        // GC only specific dialogs
        let params = FileGcParameters::new()
            .with_owner_dialog_ids(vec![DialogId::new(123), DialogId::new(456)])
            .with_exclude_owner_dialog_ids(vec![DialogId::new(789)]);

        assert!(!params.is_all_dialogs());
        assert_eq!(params.owner_dialog_ids().len(), 2);
        assert_eq!(params.exclude_owner_dialog_ids().len(), 1);
    }

    #[test]
    fn test_specific_type_gc() {
        // GC only specific file types
        let params = FileGcParameters::new().with_file_types(vec![
            FileType::Video,
            FileType::VoiceNote,
            FileType::VideoNote,
        ]);

        assert!(!params.is_all_file_types());
        assert_eq!(params.file_types().len(), 3);
    }
}
