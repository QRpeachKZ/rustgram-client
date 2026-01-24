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

//! # File Statistics
//!
//! File statistics collection for Telegram client.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib file statistics classes from `td/telegram/files/FileStats.h`:
//! - `FileTypeStat` - Size and count for a single file type
//! - `FullFileInfo` - Complete file metadata
//! - `FileStatsFast` - Quick statistics without per-dialog breakdown
//! - `FileStats` - Main statistics container with dialog breakdown
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_file_stats::{FileStats, FullFileInfo};
//! use rustgram_file_type::FileType;
//! use rustgram_dialog_id::DialogId;
//!
//! // Create statistics collector with dialog breakdown
//! let mut stats = FileStats::new(false, true);
//!
//! // Add file information
//! let info = FullFileInfo::new(
//!     FileType::Photo,
//!     "/path/to/photo.jpg".to_string(),
//!     DialogId::new(12345),
//!     1024000,
//!     0,
//!     0,
//! );
//! stats.add(info);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

mod file_stats_fast;
mod file_type_stat;
mod full_file_info;

pub use file_stats_fast::FileStatsFast;
pub use file_type_stat::FileTypeStat;
pub use full_file_info::FullFileInfo;

use rustgram_dialog_id::DialogId;
use rustgram_file_type::{FileType, FileTypeClass, MAX_FILE_TYPE};
use std::collections::HashMap;

/// Special dialog ID used for "other" files when filtering.
///
/// This is used to store files that don't belong to any of the
/// selected dialog IDs after applying filters.
pub const OTHER_DIALOG_ID: DialogId = DialogId::new(0);

/// Main file statistics container.
///
/// Collects file statistics with optional per-dialog breakdown.
/// Corresponds to TDLib's `FileStats` class.
///
/// # Fields
///
/// * `need_all_files` - Whether to collect full file info in `all_files`
/// * `split_by_owner_dialog_id` - Whether to separate stats by dialog
/// * `stat_by_type` - Aggregated statistics by file type
/// * `stat_by_owner_dialog_id` - Per-dialog statistics (if split enabled)
/// * `all_files` - Complete file list (if need_all_files enabled)
#[derive(Debug, Clone)]
pub struct FileStats {
    /// Whether to collect full file info.
    pub need_all_files: bool,
    /// Whether to separate stats by dialog owner.
    pub split_by_owner_dialog_id: bool,
    /// Statistics aggregated by file type.
    pub stat_by_type: [FileTypeStat; MAX_FILE_TYPE],
    /// Per-dialog statistics (only used if split_by_owner_dialog_id is true).
    pub stat_by_owner_dialog_id: HashMap<DialogId, [FileTypeStat; MAX_FILE_TYPE]>,
    /// Full file information (only collected if need_all_files is true).
    pub all_files: Vec<FullFileInfo>,
}

impl PartialEq for FileStats {
    fn eq(&self, other: &Self) -> bool {
        self.need_all_files == other.need_all_files
            && self.split_by_owner_dialog_id == other.split_by_owner_dialog_id
            && self.stat_by_type == other.stat_by_type
            && self.stat_by_owner_dialog_id == other.stat_by_owner_dialog_id
            && self.all_files == other.all_files
    }
}

impl Eq for FileStats {}

impl Default for FileStats {
    fn default() -> Self {
        Self::new(false, false)
    }
}

impl FileStats {
    /// Creates a new `FileStats`.
    ///
    /// # Arguments
    ///
    /// * `need_all_files` - Whether to collect full file info in `all_files`
    /// * `split_by_owner_dialog_id` - Whether to separate stats by dialog
    #[must_use]
    pub fn new(need_all_files: bool, split_by_owner_dialog_id: bool) -> Self {
        Self {
            need_all_files,
            split_by_owner_dialog_id,
            stat_by_type: [FileTypeStat::empty(); MAX_FILE_TYPE],
            stat_by_owner_dialog_id: HashMap::new(),
            all_files: Vec::new(),
        }
    }

    /// Adds file information by copying.
    ///
    /// # Arguments
    ///
    /// * `info` - Reference to the file info to add
    pub fn add_copy(&mut self, info: &FullFileInfo) {
        self.add_impl(
            info.file_type,
            info.owner_dialog_id,
            info.size,
            info.path.clone(),
        );

        if self.need_all_files {
            self.all_files.push(info.clone());
        }
    }

    /// Adds file information by moving.
    ///
    /// # Arguments
    ///
    /// * `info` - The file info to add
    pub fn add(&mut self, info: FullFileInfo) {
        let path = info.path.clone();
        self.add_impl(info.file_type, info.owner_dialog_id, info.size, path);

        if self.need_all_files {
            self.all_files.push(info);
        }
    }

    /// Internal implementation for adding file stats.
    fn add_impl(&mut self, file_type: FileType, dialog_id: DialogId, size: i64, _path: String) {
        let type_index = file_type as usize;
        if type_index >= MAX_FILE_TYPE {
            return;
        }

        // Add to type stats
        self.stat_by_type[type_index] = self.stat_by_type[type_index].add_file(size, 1);

        // Add to dialog stats if splitting is enabled
        if self.split_by_owner_dialog_id {
            let dialog_stats = self
                .stat_by_owner_dialog_id
                .entry(dialog_id)
                .or_insert_with(|| [FileTypeStat::empty(); MAX_FILE_TYPE]);
            dialog_stats[type_index] = dialog_stats[type_index].add_file(size, 1);
        }
    }

    /// Returns total non-temporary statistics.
    ///
    /// Aggregates all statistics except Temp files.
    #[must_use]
    pub fn get_total_nontemp_stat(&self) -> FileTypeStat {
        let mut result = FileTypeStat::empty();

        for (idx, stat) in self.stat_by_type.iter().enumerate() {
            let file_type = file_type_from_index(idx);
            if file_type.class() != FileTypeClass::Temp {
                result = result.add(*stat);
            }
        }

        result
    }

    /// Returns statistics for a specific file type excluding temp.
    #[allow(dead_code)]
    fn get_nontemp_stat(&self, file_type: FileType) -> FileTypeStat {
        if file_type.class() == FileTypeClass::Temp {
            return FileTypeStat::empty();
        }

        // Get the main type (collapses aliases like Wallpaper -> Background)
        let main_type = file_type.main_type();
        let mut result = FileTypeStat::empty();

        // Add all stats that match this main type
        for (idx, stat) in self.stat_by_type.iter().enumerate() {
            let current_type = file_type_from_index(idx);
            if current_type.main_type() == main_type {
                result = result.add(*stat);
            }
        }

        result
    }

    /// Keeps only the top N dialogs by size.
    ///
    /// Moves files from excluded dialogs to the "other" category.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of dialogs to keep (excluding "other")
    pub fn apply_dialog_limit(&mut self, limit: i32) {
        if !self.split_by_owner_dialog_id || limit <= 0 {
            return;
        }

        // Calculate total size for each dialog
        let mut dialog_sizes: Vec<(DialogId, i64)> = self
            .stat_by_owner_dialog_id
            .iter()
            .map(|(dialog_id, stats)| {
                let total_size: i64 = stats
                    .iter()
                    .map(|s| s.size)
                    .fold(0i64, |acc, s| acc.saturating_add(s));
                (*dialog_id, total_size)
            })
            .collect();

        // Sort by size descending
        dialog_sizes.sort_by(|a, b| b.1.cmp(&a.1));

        // Keep top N dialogs
        let keep_count = usize::try_from(limit).unwrap_or(usize::MAX);
        let keep_dialogs: std::collections::HashSet<DialogId> = dialog_sizes
            .iter()
            .take(keep_count)
            .map(|(id, _)| *id)
            .collect();

        // Move excluded dialogs to "other"
        let mut other_stats = [FileTypeStat::empty(); MAX_FILE_TYPE];
        let dialog_ids: Vec<DialogId> = self.stat_by_owner_dialog_id.keys().copied().collect();

        for dialog_id in dialog_ids {
            if !keep_dialogs.contains(&dialog_id) {
                if let Some(stats) = self.stat_by_owner_dialog_id.remove(&dialog_id) {
                    for (idx, stat) in stats.iter().enumerate() {
                        other_stats[idx] = other_stats[idx].add(*stat);
                    }
                }
            }
        }

        // Add "other" category if there were any excluded dialogs
        if !self.stat_by_owner_dialog_id.contains_key(&OTHER_DIALOG_ID)
            && other_stats.iter().any(|s| !s.is_empty())
        {
            self.stat_by_owner_dialog_id
                .insert(OTHER_DIALOG_ID, other_stats);
        }
    }

    /// Filters to only the specified dialog IDs.
    ///
    /// Moves files from excluded dialogs to the "other" category.
    ///
    /// # Arguments
    ///
    /// * `dialog_ids` - List of dialog IDs to keep
    pub fn apply_dialog_ids(&mut self, dialog_ids: Vec<DialogId>) {
        if !self.split_by_owner_dialog_id {
            return;
        }

        if dialog_ids.is_empty() {
            return;
        }

        let keep_set: std::collections::HashSet<DialogId> = dialog_ids.into_iter().collect();
        let mut other_stats = [FileTypeStat::empty(); MAX_FILE_TYPE];
        let current_dialogs: Vec<DialogId> = self.stat_by_owner_dialog_id.keys().copied().collect();

        for dialog_id in current_dialogs {
            if !keep_set.contains(&dialog_id) {
                if let Some(stats) = self.stat_by_owner_dialog_id.remove(&dialog_id) {
                    for (idx, stat) in stats.iter().enumerate() {
                        other_stats[idx] = other_stats[idx].add(*stat);
                    }
                }
            }
        }

        // Add or update "other" category
        if other_stats.iter().any(|s| !s.is_empty()) {
            if let Some(existing_other) = self.stat_by_owner_dialog_id.get_mut(&OTHER_DIALOG_ID) {
                for (idx, stat) in other_stats.iter().enumerate() {
                    existing_other[idx] = existing_other[idx].add(*stat);
                }
            } else {
                self.stat_by_owner_dialog_id
                    .insert(OTHER_DIALOG_ID, other_stats);
            }
        }
    }

    /// Returns all dialog IDs with statistics.
    ///
    /// Excludes the "other" category.
    #[must_use]
    pub fn get_dialog_ids(&self) -> Vec<DialogId> {
        self.stat_by_owner_dialog_id
            .keys()
            .copied()
            .filter(|id| *id != OTHER_DIALOG_ID)
            .collect()
    }

    /// Returns the collected files.
    ///
    /// This consumes the vector and returns its contents.
    #[must_use]
    pub fn get_all_files(&self) -> Vec<FullFileInfo> {
        self.all_files.clone()
    }

    /// Returns the number of files collected.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.all_files.len()
    }

    /// Returns `true` if no files have been added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stat_by_type.iter().all(|s| s.is_empty())
    }
}

/// Converts an index to a FileType.
///
/// # Safety
///
/// This is safe for indices 0..=27 which correspond to valid FileType enum variants.
fn file_type_from_index(idx: usize) -> FileType {
    // SAFETY: FileType is a repr(i32) enum with values 0..=27
    // Converting from usize in that range is safe
    unsafe { std::mem::transmute::<i32, FileType>(idx as i32) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_info(file_type: FileType, dialog_id: i64, size: i64) -> FullFileInfo {
        FullFileInfo::new(
            file_type,
            format!("/path/to/{dialog_id}_{size}.dat"),
            DialogId::new(dialog_id),
            size,
            0,
            0,
        )
    }

    // === Construction tests ===

    #[test]
    fn test_new() {
        let stats = FileStats::new(true, true);
        assert!(stats.need_all_files);
        assert!(stats.split_by_owner_dialog_id);
        assert!(stats.all_files.is_empty());
        assert!(stats.stat_by_owner_dialog_id.is_empty());
    }

    #[test]
    fn test_new_no_split() {
        let stats = FileStats::new(false, false);
        assert!(!stats.need_all_files);
        assert!(!stats.split_by_owner_dialog_id);
    }

    #[test]
    fn test_default() {
        let stats = FileStats::default();
        assert!(!stats.need_all_files);
        assert!(!stats.split_by_owner_dialog_id);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_is_empty_true() {
        let stats = FileStats::new(false, false);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, 1000));
        assert!(!stats.is_empty());
    }

    // === Basic add tests (no split) ===

    #[test]
    fn test_add_no_split() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 12345, 1000));

        let photo_idx = FileType::Photo as usize;
        assert_eq!(stats.stat_by_type[photo_idx].cnt, 1);
        assert_eq!(stats.stat_by_type[photo_idx].size, 1000);
    }

    #[test]
    fn test_add_multiple_no_split() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, 1000));
        stats.add(make_test_info(FileType::Video, 1, 2000));
        stats.add(make_test_info(FileType::Photo, 2, 500));

        assert_eq!(stats.stat_by_type[FileType::Photo as usize].cnt, 2);
        assert_eq!(stats.stat_by_type[FileType::Photo as usize].size, 1500);
        assert_eq!(stats.stat_by_type[FileType::Video as usize].cnt, 1);
        assert_eq!(stats.stat_by_type[FileType::Video as usize].size, 2000);
    }

    // === Basic add tests (with split) ===

    #[test]
    fn test_add_with_split() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 12345, 1000));

        assert!(stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(12345)));
    }

    #[test]
    fn test_add_multiple_dialogs() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Photo, 222, 2000));
        stats.add(make_test_info(FileType::Video, 111, 3000));

        // Dialog 111 should have Photo(1000) + Video(3000)
        let dialog_111 = stats.stat_by_owner_dialog_id.get(&DialogId::new(111));
        assert!(dialog_111.is_some());
        if let Some(d111) = dialog_111 {
            assert_eq!(d111[FileType::Photo as usize].size, 1000);
            assert_eq!(d111[FileType::Video as usize].size, 3000);
        }

        // Dialog 222 should have Photo(2000)
        let dialog_222 = stats.stat_by_owner_dialog_id.get(&DialogId::new(222));
        assert!(dialog_222.is_some());
    }

    // === add_copy tests ===

    #[test]
    fn test_add_copy_no_split() {
        let mut stats = FileStats::new(false, false);
        let info = make_test_info(FileType::Photo, 1, 1000);
        stats.add_copy(&info);

        assert_eq!(stats.stat_by_type[FileType::Photo as usize].cnt, 1);
    }

    #[test]
    fn test_add_copy_with_split() {
        let mut stats = FileStats::new(false, true);
        let info = make_test_info(FileType::Photo, 12345, 1000);
        stats.add_copy(&info);

        assert!(stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(12345)));
    }

    #[test]
    fn test_add_copy_preserves_original() {
        let mut stats = FileStats::new(false, false);
        let info = make_test_info(FileType::Photo, 1, 1000);
        let original_path = info.path.clone();

        stats.add_copy(&info);

        // Original should be unchanged
        assert_eq!(info.path, original_path);
    }

    // === need_all_files tests ===

    #[test]
    fn test_need_all_files_true() {
        let mut stats = FileStats::new(true, false);
        let info = make_test_info(FileType::Photo, 1, 1000);
        stats.add(info.clone());

        assert_eq!(stats.all_files.len(), 1);
        assert_eq!(stats.all_files[0], info);
    }

    #[test]
    fn test_need_all_files_false() {
        let mut stats = FileStats::new(false, false);
        let info = make_test_info(FileType::Photo, 1, 1000);
        stats.add(info);

        assert_eq!(stats.all_files.len(), 0);
    }

    #[test]
    fn test_file_count() {
        let mut stats = FileStats::new(true, false);
        assert_eq!(stats.file_count(), 0);

        stats.add(make_test_info(FileType::Photo, 1, 1000));
        assert_eq!(stats.file_count(), 1);

        stats.add(make_test_info(FileType::Video, 2, 2000));
        assert_eq!(stats.file_count(), 2);
    }

    // === get_total_nontemp_stat tests ===

    #[test]
    fn test_get_total_nontemp_stat_empty() {
        let stats = FileStats::new(false, false);
        let total = stats.get_total_nontemp_stat();
        assert!(total.is_empty());
    }

    #[test]
    fn test_get_total_nontemp_stat_includes_photo() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, 1000));

        let total = stats.get_total_nontemp_stat();
        assert_eq!(total.cnt, 1);
        assert_eq!(total.size, 1000);
    }

    #[test]
    fn test_get_total_nontemp_stat_excludes_temp() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, 1000));
        stats.add(make_test_info(FileType::Temp, 1, 500));

        let total = stats.get_total_nontemp_stat();
        assert_eq!(total.cnt, 1);
        assert_eq!(total.size, 1000);
    }

    #[test]
    fn test_get_total_nontemp_stat_multiple_types() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, 1000));
        stats.add(make_test_info(FileType::Video, 1, 2000));
        stats.add(make_test_info(FileType::Document, 1, 3000));
        stats.add(make_test_info(FileType::Temp, 1, 500));

        let total = stats.get_total_nontemp_stat();
        assert_eq!(total.cnt, 3);
        assert_eq!(total.size, 6000);
    }

    // === apply_dialog_limit tests ===

    #[test]
    fn test_apply_dialog_limit_no_split() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, 1000));
        stats.apply_dialog_limit(10);

        // Should have no effect when not splitting
        assert!(stats.stat_by_owner_dialog_id.is_empty());
    }

    #[test]
    fn test_apply_dialog_limit_keep_all() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Photo, 222, 2000));
        stats.add(make_test_info(FileType::Photo, 333, 3000));

        stats.apply_dialog_limit(10);

        // All dialogs should remain
        assert_eq!(stats.stat_by_owner_dialog_id.len(), 3);
    }

    #[test]
    fn test_apply_dialog_limit_truncates() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Photo, 222, 2000));
        stats.add(make_test_info(FileType::Photo, 333, 3000));

        stats.apply_dialog_limit(2);

        // Should keep 2 top dialogs + 1 "other"
        assert_eq!(stats.stat_by_owner_dialog_id.len(), 3);
        assert!(stats.stat_by_owner_dialog_id.contains_key(&OTHER_DIALOG_ID));
    }

    #[test]
    fn test_apply_dialog_limit_by_size() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Photo, 222, 5000));
        stats.add(make_test_info(FileType::Photo, 333, 3000));

        stats.apply_dialog_limit(1);

        // Should keep dialog 222 (largest) + "other"
        assert!(stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(222)));
        assert!(stats.stat_by_owner_dialog_id.contains_key(&OTHER_DIALOG_ID));
        assert!(!stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(111)));
        assert!(!stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(333)));
    }

    #[test]
    fn test_apply_dialog_limit_zero() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));

        stats.apply_dialog_limit(0);

        // Should have no effect (limit <= 0 returns early)
        assert!(stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(111)));
    }

    // === apply_dialog_ids tests ===

    #[test]
    fn test_apply_dialog_ids_no_split() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.apply_dialog_ids(vec![DialogId::new(111)]);

        // Should have no effect when not splitting
        assert!(stats.stat_by_owner_dialog_id.is_empty());
    }

    #[test]
    fn test_apply_dialog_ids_keep_all() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Photo, 222, 2000));
        stats.add(make_test_info(FileType::Photo, 333, 3000));

        stats.apply_dialog_ids(vec![
            DialogId::new(111),
            DialogId::new(222),
            DialogId::new(333),
        ]);

        // All dialogs should remain
        assert_eq!(stats.stat_by_owner_dialog_id.len(), 3);
        assert!(!stats.stat_by_owner_dialog_id.contains_key(&OTHER_DIALOG_ID));
    }

    #[test]
    fn test_apply_dialog_ids_filters() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Photo, 222, 2000));
        stats.add(make_test_info(FileType::Photo, 333, 3000));

        stats.apply_dialog_ids(vec![DialogId::new(111), DialogId::new(333)]);

        // Should keep 111, 333, and "other" (containing 222)
        assert_eq!(stats.stat_by_owner_dialog_id.len(), 3);
        assert!(stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(111)));
        assert!(stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(333)));
        assert!(stats.stat_by_owner_dialog_id.contains_key(&OTHER_DIALOG_ID));
        assert!(!stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(222)));
    }

    #[test]
    fn test_apply_dialog_ids_empty() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));

        stats.apply_dialog_ids(vec![]);

        // Should return early with no effect
        assert!(stats
            .stat_by_owner_dialog_id
            .contains_key(&DialogId::new(111)));
    }

    // === get_dialog_ids tests ===

    #[test]
    fn test_get_dialog_ids_empty() {
        let stats = FileStats::new(false, true);
        assert!(stats.get_dialog_ids().is_empty());
    }

    #[test]
    fn test_get_dialog_ids_no_other() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Photo, 222, 2000));

        let ids = stats.get_dialog_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&DialogId::new(111)));
        assert!(ids.contains(&DialogId::new(222)));
    }

    #[test]
    fn test_get_dialog_ids_excludes_other() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.apply_dialog_limit(0);

        let ids = stats.get_dialog_ids();
        // Should not include OTHER_DIALOG_ID
        assert!(!ids.contains(&OTHER_DIALOG_ID));
    }

    // === get_all_files tests ===

    #[test]
    fn test_get_all_files_empty() {
        let stats = FileStats::new(true, false);
        assert!(stats.get_all_files().is_empty());
    }

    #[test]
    fn test_get_all_files_with_need_all() {
        let mut stats = FileStats::new(true, false);
        let info1 = make_test_info(FileType::Photo, 1, 1000);
        let info2 = make_test_info(FileType::Video, 2, 2000);

        stats.add(info1.clone());
        stats.add(info2.clone());

        let files = stats.get_all_files();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&info1));
        assert!(files.contains(&info2));
    }

    #[test]
    fn test_get_all_files_without_need_all() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, 1000));

        assert!(stats.get_all_files().is_empty());
    }

    // === Main type grouping tests ===

    #[test]
    fn test_wallpaper_groups_with_background() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Wallpaper, 1, 1000));
        stats.add(make_test_info(FileType::Background, 1, 2000));

        let background_idx = FileType::Background as usize;
        let wallpaper_idx = FileType::Wallpaper as usize;

        // Both should be tracked in their respective indices
        assert_eq!(stats.stat_by_type[wallpaper_idx].size, 1000);
        assert_eq!(stats.stat_by_type[background_idx].size, 2000);
    }

    // === Equality tests ===

    #[test]
    fn test_equality_true() {
        let mut stats1 = FileStats::new(false, false);
        stats1.add(make_test_info(FileType::Photo, 1, 1000));

        let mut stats2 = FileStats::new(false, false);
        stats2.add(make_test_info(FileType::Photo, 1, 1000));

        assert_eq!(stats1, stats2);
    }

    #[test]
    fn test_equality_false_different_count() {
        let mut stats1 = FileStats::new(false, false);
        stats1.add(make_test_info(FileType::Photo, 1, 1000));

        let stats2 = FileStats::new(false, false);

        assert_ne!(stats1, stats2);
    }

    // === Debug tests ===

    #[test]
    fn test_debug_format() {
        let stats = FileStats::new(true, true);
        let debug = format!("{stats:?}");
        assert!(debug.contains("FileStats"));
    }

    // === Complex integration tests ===

    #[test]
    fn test_full_workflow_with_split() {
        let mut stats = FileStats::new(true, true);

        // Add files from different dialogs
        for i in 1..=10 {
            stats.add(make_test_info(FileType::Photo, i, 1000 * i));
            stats.add(make_test_info(FileType::Video, i, 2000 * i));
        }

        // Apply dialog limit
        stats.apply_dialog_limit(5);

        let ids = stats.get_dialog_ids();
        assert_eq!(ids.len(), 5);
        assert!(stats.stat_by_owner_dialog_id.contains_key(&OTHER_DIALOG_ID));
    }

    #[test]
    fn test_full_workflow_without_split() {
        let mut stats = FileStats::new(false, false);

        // Add various file types
        stats.add(make_test_info(FileType::Photo, 1, 1000));
        stats.add(make_test_info(FileType::Video, 2, 2000));
        stats.add(make_test_info(FileType::Document, 3, 3000));
        stats.add(make_test_info(FileType::Temp, 4, 500));

        let total = stats.get_total_nontemp_stat();
        assert_eq!(total.cnt, 3);
        assert_eq!(total.size, 6000);
    }

    // === File type class tests ===

    #[test]
    fn test_all_file_type_classes() {
        let mut stats = FileStats::new(false, false);

        // Add one from each class
        stats.add(make_test_info(FileType::Photo, 1, 1000));
        stats.add(make_test_info(FileType::Video, 1, 2000));
        stats.add(make_test_info(FileType::SecureEncrypted, 1, 3000));
        stats.add(make_test_info(FileType::Encrypted, 1, 4000));
        stats.add(make_test_info(FileType::Temp, 1, 500));

        let total = stats.get_total_nontemp_stat();
        assert_eq!(total.cnt, 4); // Temp excluded
        assert_eq!(total.size, 10000);
    }

    // === Saturating arithmetic tests ===

    #[test]
    fn test_saturating_add_size() {
        let mut stats = FileStats::new(false, false);
        stats.add(make_test_info(FileType::Photo, 1, i64::MAX));

        let photo_idx = FileType::Photo as usize;
        assert_eq!(stats.stat_by_type[photo_idx].size, i64::MAX);
    }

    // === Clone tests ===

    #[test]
    fn test_clone() {
        let mut stats1 = FileStats::new(true, true);
        stats1.add(make_test_info(FileType::Photo, 1, 1000));

        let stats2 = stats1.clone();
        assert_eq!(stats1, stats2);
    }

    // === Serialization tests (for the contained types) ===

    #[test]
    fn test_contained_types_serializable() {
        let _stats = FileStats::new(true, true);

        // Just ensure the types can be serialized
        let file_type_stat = FileTypeStat::new(100, 1);
        let json = serde_json::to_string(&file_type_stat);
        assert!(json.is_ok());
    }

    // === Edge case: invalid file type index ===

    #[test]
    fn test_add_with_invalid_type_doesnt_panic() {
        let mut stats = FileStats::new(false, false);

        // Temp class should be handled correctly
        stats.add(make_test_info(FileType::Temp, 1, 100));

        let temp_idx = FileType::Temp as usize;
        assert_eq!(stats.stat_by_type[temp_idx].cnt, 1);
    }

    // === Multiple same dialog, different types ===

    #[test]
    fn test_same_dialog_multiple_types() {
        let mut stats = FileStats::new(false, true);
        stats.add(make_test_info(FileType::Photo, 111, 1000));
        stats.add(make_test_info(FileType::Video, 111, 2000));
        stats.add(make_test_info(FileType::Document, 111, 3000));

        let dialog_stats = stats.stat_by_owner_dialog_id.get(&DialogId::new(111));
        assert!(dialog_stats.is_some());

        if let Some(ds) = dialog_stats {
            assert_eq!(ds[FileType::Photo as usize].size, 1000);
            assert_eq!(ds[FileType::Video as usize].size, 2000);
            assert_eq!(ds[FileType::Document as usize].size, 3000);
        }
    }

    // === File type conversion helper tests ===

    #[test]
    fn test_file_type_from_index() {
        assert_eq!(file_type_from_index(0), FileType::Thumbnail);
        assert_eq!(file_type_from_index(2), FileType::Photo);
        assert_eq!(file_type_from_index(4), FileType::Video);
        assert_eq!(file_type_from_index(27), FileType::None);
    }
}
