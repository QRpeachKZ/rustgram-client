//! Storage-related types for StorageManager.

use rustgram_dialog_id::DialogId;
use std::collections::HashMap;

/// File type enumeration.
///
/// Stub implementation - TODO: Integrate with proper FileType from TDLib.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FileType {
    /// Unknown file type.
    Unknown = 0,
    /// Thumbnail.
    Thumbnail = 1,
    /// Profile photo.
    ProfilePhoto = 2,
    /// Photo.
    Photo = 3,
    /// Voice note.
    Voice = 4,
    /// Video.
    Video = 5,
    /// Document.
    Document = 6,
    /// Encrypted file.
    Encrypted = 7,
    /// Temporary file.
    Temp = 8,
    /// Sticker.
    Sticker = 9,
    /// Audio.
    Audio = 10,
    /// Animation.
    Animation = 11,
    /// Encrypted thumbnail.
    EncryptedThumbnail = 12,
    /// Wallpaper.
    Wallpaper = 13,
    /// Video note.
    VideoNote = 14,
    /// Secure raw file.
    SecureRaw = 15,
    /// Secure file.
    Secure = 16,
    /// Background.
    Background = 17,
    /// Document as file.
    DocumentAsFile = 18,
    /// Size (MAX_FILE_TYPE).
    Size = 19,
}

impl FileType {
    /// Creates a FileType from an i32 value.
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Unknown),
            1 => Some(Self::Thumbnail),
            2 => Some(Self::ProfilePhoto),
            3 => Some(Self::Photo),
            4 => Some(Self::Voice),
            5 => Some(Self::Video),
            6 => Some(Self::Document),
            7 => Some(Self::Encrypted),
            8 => Some(Self::Temp),
            9 => Some(Self::Sticker),
            10 => Some(Self::Audio),
            11 => Some(Self::Animation),
            12 => Some(Self::EncryptedThumbnail),
            13 => Some(Self::Wallpaper),
            14 => Some(Self::VideoNote),
            15 => Some(Self::SecureRaw),
            16 => Some(Self::Secure),
            17 => Some(Self::Background),
            18 => Some(Self::DocumentAsFile),
            19 => Some(Self::Size),
            _ => None,
        }
    }

    /// Converts FileType to i32.
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns whether this is a temporary file type.
    #[must_use]
    pub const fn is_temp(self) -> bool {
        matches!(self, Self::Temp)
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::Unknown
    }
}
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Storage-related types for StorageManager.

use rustgram_dialog_id::DialogId;
use rustgram_types::FileType;
use std::collections::HashMap;

/// Maximum file type value.
pub const MAX_FILE_TYPE: usize = 19;

/// Fast storage statistics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStatsFast {
    /// Total file size in bytes.
    pub size: i64,
    /// Total file count.
    pub count: i32,
    /// Database size in bytes.
    pub database_size: i64,
    /// Language pack database size in bytes.
    pub language_pack_database_size: i64,
    /// Log file size in bytes.
    pub log_size: i64,
}

impl FileStatsFast {
    /// Creates new fast storage statistics.
    ///
    /// # Arguments
    ///
    /// * `size` - Total file size in bytes
    /// * `count` - Total file count
    /// * `database_size` - Database size in bytes
    /// * `language_pack_database_size` - Language pack database size in bytes
    /// * `log_size` - Log file size in bytes
    #[must_use]
    pub const fn new(
        size: i64,
        count: i32,
        database_size: i64,
        language_pack_database_size: i64,
        log_size: i64,
    ) -> Self {
        Self {
            size,
            count,
            database_size,
            language_pack_database_size,
            log_size,
        }
    }

    /// Returns the total size including database and logs.
    #[must_use]
    pub const fn total_size(&self) -> i64 {
        self.size + self.database_size + self.language_pack_database_size + self.log_size
    }
}

/// File statistics by type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTypeStat {
    /// Total size in bytes.
    pub size: i64,
    /// Number of files.
    pub count: i32,
}

impl FileTypeStat {
    /// Creates a new empty file type statistic.
    #[must_use]
    pub const fn new() -> Self {
        Self { size: 0, count: 0 }
    }

    /// Adds size and count to this statistic.
    pub fn add(&mut self, size: i64, count: i32) {
        self.size += size;
        self.count += count;
    }

    /// Returns whether this statistic is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.size == 0 && self.count == 0
    }
}

impl Default for FileTypeStat {
    fn default() -> Self {
        Self::new()
    }
}

/// Detailed file statistics.
#[derive(Debug, Clone)]
pub struct FileStats {
    /// Statistics by file type.
    pub by_type: Vec<FileTypeStat>,
    /// Statistics by owner dialog.
    pub by_dialog: HashMap<DialogId, Vec<FileTypeStat>>,
    /// All files (if need_all_files was true).
    pub all_files: Vec<FileInfo>,
    /// Whether to split by dialog.
    split_by_dialog: bool,
}

impl FileStats {
    /// Creates new file statistics.
    ///
    /// # Arguments
    ///
    /// * `need_all_files` - Whether to include all files in the result
    /// * `split_by_dialog` - Whether to split statistics by dialog
    #[must_use]
    pub fn new(need_all_files: bool, split_by_dialog: bool) -> Self {
        Self {
            by_type: vec![FileTypeStat::new(); MAX_FILE_TYPE],
            by_dialog: HashMap::new(),
            all_files: if need_all_files { Vec::new() } else { Vec::new() },
            split_by_dialog,
        }
    }

    /// Applies a dialog limit to the statistics.
    pub fn apply_dialog_limit(&mut self, limit: i32) {
        if !self.split_by_dialog {
            return;
        }
        // Keep only the first `limit` dialogs
        let mut dialog_ids: Vec<DialogId> = self.by_dialog.keys().copied().collect();
        dialog_ids.truncate(limit as usize);
        let mut new_by_dialog = HashMap::new();
        for id in dialog_ids {
            if let Some(stats) = self.by_dialog.remove(&id) {
                new_by_dialog.insert(id, stats);
            }
        }
        self.by_dialog = new_by_dialog;
    }

    /// Returns the total size across all file types.
    #[must_use]
    pub fn get_total_size(&self) -> i64 {
        self.by_type.iter().map(|s| s.size).sum()
    }

    /// Returns the total count across all file types.
    #[must_use]
    pub fn get_total_count(&self) -> i32 {
        self.by_type.iter().map(|s| s.count).sum()
    }
}

/// Individual file information.
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// File type.
    pub file_type: FileType,
    /// File path.
    pub path: String,
    /// Owner dialog ID.
    pub owner_dialog_id: Option<DialogId>,
    /// File size in bytes.
    pub size: i64,
    /// Last access time (nanoseconds).
    pub atime_nsec: u64,
    /// Last modification time (nanoseconds).
    pub mtime_nsec: u64,
}

impl FileInfo {
    /// Creates a new file info.
    #[must_use]
    pub fn new(
        file_type: FileType,
        path: String,
        owner_dialog_id: Option<DialogId>,
        size: i64,
        atime_nsec: u64,
        mtime_nsec: u64,
    ) -> Self {
        Self {
            file_type,
            path,
            owner_dialog_id,
            size,
            atime_nsec,
            mtime_nsec,
        }
    }
}

/// Garbage collection parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileGcParameters {
    /// Dialog IDs to keep files for (None = all dialogs).
    pub dialog_ids: Option<Vec<DialogId>>,
    /// Exclude dialog IDs from GC.
    pub exclude_dialog_ids: Vec<DialogId>,
    /// Keep files from these dialogs even if they are in exclude_dialog_ids.
    pub keep_dialog_ids: Vec<DialogId>,
    /// File types to GC.
    pub file_types: Option<Vec<FileType>>,
    /// Keep files newer than this time (Unix timestamp).
    pub keep_files_newer_than: Option<i64>,
    /// Keep files older than this time (Unix timestamp).
    pub keep_files_older_than: Option<i64>,
    /// Keep all dialogs' files created before this time.
    pub keep_all_dialog_files_created_before: Option<i64>,
    /// Keep files from these specific dialogs even if they match other criteria.
    pub keep_files_from_dialogs: Vec<DialogId>,
    /// Return deleted file statistics.
    pub return_deleted_file_statistics: bool,
    /// Maximum delay before GC starts.
    pub max_delay: i32,
}

impl FileGcParameters {
    /// Creates a new default GC parameters.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dialog_ids: None,
            exclude_dialog_ids: Vec::new(),
            keep_dialog_ids: Vec::new(),
            file_types: None,
            keep_files_newer_than: None,
            keep_files_older_than: None,
            keep_all_dialog_files_created_before: None,
            keep_files_from_dialogs: Vec::new(),
            return_deleted_file_statistics: false,
            max_delay: 0,
        }
    }

    /// Sets the dialog IDs to process.
    #[must_use]
    pub fn with_dialog_ids(mut self, dialog_ids: Vec<DialogId>) -> Self {
        self.dialog_ids = Some(dialog_ids);
        self
    }

    /// Sets the file types to process.
    #[must_use]
    pub fn with_file_types(mut self, file_types: Vec<FileType>) -> Self {
        self.file_types = Some(file_types);
        self
    }
}

impl Default for FileGcParameters {
    fn default() -> Self {
        Self::new()
    }
}

/// Database statistics.
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    /// Debug information string.
    pub debug: String,
}

impl DatabaseStats {
    /// Creates new database statistics.
    #[must_use]
    pub fn new(debug: String) -> Self {
        Self { debug }
    }

    /// Gets the database statistics as a string.
    #[must_use]
    pub fn get_debug(&self) -> &str {
        &self.debug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_stats_fast_new() {
        let stats = FileStatsFast::new(1024, 10, 512, 256, 128);
        assert_eq!(stats.size, 1024);
        assert_eq!(stats.count, 10);
        assert_eq!(stats.total_size(), 1920);
    }

    #[test]
    fn test_file_type_stat_new() {
        let stat = FileTypeStat::new();
        assert!(stat.is_empty());
        assert_eq!(stat.size, 0);
        assert_eq!(stat.count, 0);
    }

    #[test]
    fn test_file_type_stat_add() {
        let mut stat = FileTypeStat::new();
        stat.add(100, 5);
        stat.add(50, 3);
        assert_eq!(stat.size, 150);
        assert_eq!(stat.count, 8);
        assert!(!stat.is_empty());
    }

    #[test]
    fn test_file_stats_new() {
        let stats = FileStats::new(true, true);
        assert_eq!(stats.get_total_size(), 0);
        assert_eq!(stats.get_total_count(), 0);
    }

    #[test]
    fn test_file_gc_parameters_new() {
        let params = FileGcParameters::new();
        assert!(params.dialog_ids.is_none());
        assert!(params.exclude_dialog_ids.is_empty());
    }

    #[test]
    fn test_file_gc_parameters_with_dialog_ids() {
        let dialog_id = DialogId::User(rustgram_types::UserId(123));
        let params = FileGcParameters::new().with_dialog_ids(vec![dialog_id]);
        assert!(params.dialog_ids.is_some());
    }

    #[test]
    fn test_database_stats_new() {
        let stats = DatabaseStats::new("test debug".to_string());
        assert_eq!(stats.get_debug(), "test debug");
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_FILE_TYPE, 19);
    }
}
