//! Supporting types for download manager callback.

use rustgram_file_id::FileId;
use serde::{Deserialize, Serialize};

/// Download counter values.
///
/// Represents the current state of all downloads being tracked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counters {
    /// Number of active downloads.
    pub active_count: i32,

    /// Number of paused downloads.
    pub paused_count: i32,

    /// Number of completed downloads.
    pub completed_count: i32,

    /// Total size of all downloads in bytes.
    pub total_size: i64,
}

/// Per-file download counters.
///
/// Tracks progress for individual files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileCounters {
    /// Total number of chunks/parts to download.
    pub total_count: i32,

    /// Number of chunks already downloaded.
    pub downloaded_count: i32,
}

/// View of a file for UI display.
///
/// Contains information needed to display file download status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileView {
    /// Unique identifier of the file.
    pub file_id: FileId,

    /// Total file size in bytes.
    pub size: i64,

    /// Number of bytes already downloaded.
    pub downloaded_size: i64,

    /// Whether the download has completed.
    pub is_downloading_completed: bool,
}

/// Complete file object representation.
///
/// Contains both local and remote file information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileObject {
    /// File identifier.
    pub id: i32,

    /// Actual file size in bytes.
    pub size: i64,

    /// Expected file size in bytes (may differ from actual size).
    pub expected_size: i64,

    /// Local file information.
    pub local: LocalFile,

    /// Remote file information.
    pub remote: RemoteFile,
}

/// Local file information.
///
/// Contains information about the locally stored file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalFile {
    /// Path to the local file.
    pub path: String,

    /// Number of bytes downloaded.
    pub downloaded_size: i64,
}

/// Remote file information.
///
/// Contains information about the remote file location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteFile {
    /// Remote file identifier.
    pub id: String,

    /// Whether the file is currently being uploaded.
    pub is_uploading_active: bool,
}

/// Download object representation.
///
/// Contains information specifically relevant to download operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileDownloadObject {
    /// File identifier.
    pub file_id: i32,

    /// Message identifier (if file is from a message).
    pub message_id: i64,

    /// Unix timestamp when the file was added to downloads.
    pub add_date: i32,

    /// Unix timestamp when the download completed (0 if not completed).
    pub complete_date: i32,

    /// Whether the download is paused.
    pub is_paused: bool,
}
