//! # Download Manager Callback
//!
//! Trait for download manager callbacks in TDLib.
//!
//! This module defines the callback interface that the download manager uses
//! to communicate with its clients. It provides methods for updating download
//! counters, file status changes, and controlling download operations.
//!
//! ## Overview
//!
//! The `DownloadManagerCallback` trait is an adapter interface that allows
//! different components to react to download events without knowing the
//! internal implementation of the download manager.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_download_manager_callback::{
//!     DownloadManagerCallback, DownloadManager, Counters, FileCounters, FileView,
//!     FileObject, LocalFile, RemoteFile, FileDownloadObject,
//! };
//! use rustgram_file_id::FileId;
//! use rustgram_file_source_id::FileSourceId;
//! use std::sync::Arc;
//!
//! struct MyCallback {
//!     internal_id: i64,
//! }
//!
//! impl DownloadManagerCallback for MyCallback {
//!     fn update_counters(&mut self, counters: Counters) {
//!         println!("Active: {}, Paused: {}, Completed: {}",
//!             counters.active_count,
//!             counters.paused_count,
//!             counters.completed_count
//!         );
//!     }
//!
//!     fn get_internal_download_id(&self) -> i64 {
//!         self.internal_id
//!     }
//!
//!     fn update_file_added(&mut self, _file_id: FileId, _file_source_id: FileSourceId,
//!                          _add_date: i32, _complete_date: i32, _is_paused: bool,
//!                          _counters: FileCounters) {
//!         // Handle file added
//!     }
//!
//!     fn update_file_changed(&mut self, _file_id: FileId, _complete_date: i32,
//!                            _is_paused: bool, _counters: FileCounters) {
//!         // Handle file changed
//!     }
//!
//!     fn update_file_removed(&mut self, _file_id: FileId, _counters: FileCounters) {
//!         // Handle file removed
//!     }
//!
//!     fn start_file(&mut self, _file_id: FileId, _internal_download_id: i64,
//!                   _priority: i8, _download_manager: Option<Arc<dyn DownloadManager>>) {
//!         // Handle start file
//!     }
//!
//!     fn pause_file(&mut self, _file_id: FileId, _internal_download_id: i64) {
//!         // Handle pause file
//!     }
//!
//!     fn delete_file(&mut self, _file_id: FileId) {
//!         // Handle delete file
//!     }
//!
//!     fn get_file_search_text(&self, _file_id: FileId, _file_source_id: FileSourceId)
//!         -> Option<String>
//!     {
//!         None
//!     }
//!
//!     fn get_file_view(&self, _file_id: FileId) -> FileView {
//!         FileView {
//!             file_id: FileId::empty(),
//!             size: 0,
//!             downloaded_size: 0,
//!             is_downloading_completed: false,
//!         }
//!     }
//!
//!     fn get_file_object(&self, _file_id: FileId) -> FileObject {
//!         FileObject {
//!             id: 0,
//!             size: 0,
//!             expected_size: 0,
//!             local: LocalFile {
//!                 path: String::new(),
//!                 downloaded_size: 0,
//!             },
//!             remote: RemoteFile {
//!                 id: String::new(),
//!                 is_uploading_active: false,
//!             },
//!         }
//!     }
//!
//!     fn get_file_download_object(&self, _file_id: FileId, _file_source_id: FileSourceId,
//!                                  _add_date: i32, _complete_date: i32, _is_paused: bool)
//!         -> FileDownloadObject
//!     {
//!         FileDownloadObject {
//!             file_id: 0,
//!             message_id: 0,
//!             add_date: 0,
//!             complete_date: 0,
//!             is_paused: false,
//!         }
//!     }
//! }
//! ```

pub use types::{
    Counters, FileCounters, FileDownloadObject, FileObject, FileView, LocalFile, RemoteFile,
};

use rustgram_file_id::FileId;
use rustgram_file_source_id::FileSourceId;
use std::sync::Arc;

mod types;

/// Stub trait for DownloadManager.
///
/// TODO: Full implementation when download manager module is available.
///
/// This trait represents the download manager that controls file downloads.
/// It is used by the callback to interact with the actual download manager.
pub trait DownloadManager: Send + Sync {
    /// Placeholder method for download manager operations.
    fn do_download(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Callback trait for download manager events.
///
/// This trait defines the interface that components must implement to receive
/// callbacks from the download manager. It includes methods for:
///
/// - **Counter updates**: Notifications about download statistics
/// - **File lifecycle**: Add, change, and remove events
/// - **Download control**: Start, pause, and delete operations
/// - **File queries**: Retrieve file information and metadata
///
/// # Thread Safety
///
/// Callback implementations may be called from multiple threads concurrently.
/// Implementations must ensure thread safety when accessing shared state.
pub trait DownloadManagerCallback: Send + Sync {
    /// Update download counters.
    ///
    /// Called when the overall download statistics change.
    ///
    /// # Arguments
    ///
    /// * `counters` - Updated counter values for all download states
    fn update_counters(&mut self, counters: Counters);

    /// Notify that a file was added to the download queue.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file
    /// * `file_source_id` - Source identifier for the file
    /// * `add_date` - Unix timestamp when the file was added
    /// * `complete_date` - Unix timestamp when download completed (0 if not completed)
    /// * `is_paused` - Whether the download is paused
    /// * `counters` - Updated counter values for this file
    fn update_file_added(
        &mut self,
        file_id: FileId,
        file_source_id: FileSourceId,
        add_date: i32,
        complete_date: i32,
        is_paused: bool,
        counters: FileCounters,
    );

    /// Notify that a file's download status changed.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file
    /// * `complete_date` - Unix timestamp when download completed (0 if not completed)
    /// * `is_paused` - Whether the download is paused
    /// * `counters` - Updated counter values for this file
    fn update_file_changed(
        &mut self,
        file_id: FileId,
        complete_date: i32,
        is_paused: bool,
        counters: FileCounters,
    );

    /// Notify that a file was removed from the download queue.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file
    /// * `counters` - Updated counter values after removal
    fn update_file_removed(&mut self, file_id: FileId, counters: FileCounters);

    /// Get the internal download ID for this callback.
    ///
    /// Returns an identifier that uniquely identifies this callback instance
    /// within the download manager.
    fn get_internal_download_id(&self) -> i64;

    /// Start downloading a file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file to download
    /// * `internal_download_id` - Internal identifier for tracking this download
    /// * `priority` - Download priority (higher = more important, range: -32 to 31)
    /// * `download_manager` - Optional reference to the download manager
    fn start_file(
        &mut self,
        file_id: FileId,
        internal_download_id: i64,
        priority: i8,
        download_manager: Option<Arc<dyn DownloadManager>>,
    );

    /// Pause downloading a file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file to pause
    /// * `internal_download_id` - Internal identifier for tracking this download
    fn pause_file(&mut self, file_id: FileId, internal_download_id: i64);

    /// Delete a file from the download queue.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file to delete
    fn delete_file(&mut self, file_id: FileId);

    /// Get search text for a file.
    ///
    /// Returns a string that can be used to search for this file,
    /// or `None` if no search text is available.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file
    /// * `file_source_id` - Source identifier for the file
    fn get_file_search_text(&self, file_id: FileId, file_source_id: FileSourceId)
        -> Option<String>;

    /// Get a view of a file for display purposes.
    ///
    /// Returns a simplified view containing information needed to display
    /// the file in the UI.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file
    fn get_file_view(&self, file_id: FileId) -> FileView;

    /// Get the full object representation of a file.
    ///
    /// Returns complete file information including local and remote locations.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file
    fn get_file_object(&self, file_id: FileId) -> FileObject;

    /// Get the download object representation of a file.
    ///
    /// Returns information specifically relevant to download operations.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique identifier of the file
    /// * `file_source_id` - Source identifier for the file
    /// * `add_date` - Unix timestamp when the file was added
    /// * `complete_date` - Unix timestamp when download completed (0 if not completed)
    /// * `is_paused` - Whether the download is paused
    fn get_file_download_object(
        &self,
        file_id: FileId,
        file_source_id: FileSourceId,
        add_date: i32,
        complete_date: i32,
        is_paused: bool,
    ) -> FileDownloadObject;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCallback {
        internal_id: i64,
    }

    impl DownloadManagerCallback for TestCallback {
        fn update_counters(&mut self, _counters: Counters) {}

        fn update_file_added(
            &mut self,
            _file_id: FileId,
            _file_source_id: FileSourceId,
            _add_date: i32,
            _complete_date: i32,
            _is_paused: bool,
            _counters: FileCounters,
        ) {
        }

        fn update_file_changed(
            &mut self,
            _file_id: FileId,
            _complete_date: i32,
            _is_paused: bool,
            _counters: FileCounters,
        ) {
        }

        fn update_file_removed(&mut self, _file_id: FileId, _counters: FileCounters) {}

        fn get_internal_download_id(&self) -> i64 {
            self.internal_id
        }

        fn start_file(
            &mut self,
            _file_id: FileId,
            _internal_download_id: i64,
            _priority: i8,
            _download_manager: Option<Arc<dyn DownloadManager>>,
        ) {
        }

        fn pause_file(&mut self, _file_id: FileId, _internal_download_id: i64) {}

        fn delete_file(&mut self, _file_id: FileId) {}

        fn get_file_search_text(
            &self,
            _file_id: FileId,
            _file_source_id: FileSourceId,
        ) -> Option<String> {
            None
        }

        fn get_file_view(&self, _file_id: FileId) -> FileView {
            FileView {
                file_id: FileId::empty(),
                size: 0,
                downloaded_size: 0,
                is_downloading_completed: false,
            }
        }

        fn get_file_object(&self, _file_id: FileId) -> FileObject {
            FileObject {
                id: 0,
                size: 0,
                expected_size: 0,
                local: LocalFile {
                    path: String::new(),
                    downloaded_size: 0,
                },
                remote: RemoteFile {
                    id: String::new(),
                    is_uploading_active: false,
                },
            }
        }

        fn get_file_download_object(
            &self,
            _file_id: FileId,
            _file_source_id: FileSourceId,
            _add_date: i32,
            _complete_date: i32,
            _is_paused: bool,
        ) -> FileDownloadObject {
            FileDownloadObject {
                file_id: 0,
                message_id: 0,
                add_date: 0,
                complete_date: 0,
                is_paused: false,
            }
        }
    }

    #[test]
    fn test_callback_get_internal_id() {
        let callback = TestCallback { internal_id: 42 };
        assert_eq!(callback.get_internal_download_id(), 42);
    }

    #[test]
    fn test_callback_get_internal_id_zero() {
        let callback = TestCallback { internal_id: 0 };
        assert_eq!(callback.get_internal_download_id(), 0);
    }

    #[test]
    fn test_callback_get_internal_id_negative() {
        let callback = TestCallback { internal_id: -1 };
        assert_eq!(callback.get_internal_download_id(), -1);
    }

    #[test]
    fn test_counters_new() {
        let counters = Counters {
            active_count: 5,
            paused_count: 2,
            completed_count: 10,
            total_size: 1_000_000,
        };
        assert_eq!(counters.active_count, 5);
        assert_eq!(counters.paused_count, 2);
        assert_eq!(counters.completed_count, 10);
        assert_eq!(counters.total_size, 1_000_000);
    }

    #[test]
    fn test_counters_all_zero() {
        let counters = Counters {
            active_count: 0,
            paused_count: 0,
            completed_count: 0,
            total_size: 0,
        };
        assert_eq!(counters.active_count, 0);
        assert_eq!(counters.paused_count, 0);
        assert_eq!(counters.completed_count, 0);
        assert_eq!(counters.total_size, 0);
    }

    #[test]
    fn test_file_counters_new() {
        let counters = FileCounters {
            total_count: 100,
            downloaded_count: 75,
        };
        assert_eq!(counters.total_count, 100);
        assert_eq!(counters.downloaded_count, 75);
    }

    #[test]
    fn test_file_counters_zero() {
        let counters = FileCounters {
            total_count: 0,
            downloaded_count: 0,
        };
        assert_eq!(counters.total_count, 0);
        assert_eq!(counters.downloaded_count, 0);
    }

    #[test]
    fn test_file_view_completed() {
        let view = FileView {
            file_id: FileId::empty(),
            size: 1024,
            downloaded_size: 1024,
            is_downloading_completed: true,
        };
        assert!(view.is_downloading_completed);
        assert_eq!(view.size, view.downloaded_size);
    }

    #[test]
    fn test_file_view_partial() {
        let view = FileView {
            file_id: FileId::empty(),
            size: 1024,
            downloaded_size: 512,
            is_downloading_completed: false,
        };
        assert!(!view.is_downloading_completed);
        assert!(view.downloaded_size < view.size);
    }

    #[test]
    fn test_file_object_new() {
        let obj = FileObject {
            id: 123,
            size: 2048,
            expected_size: 2048,
            local: LocalFile {
                path: "/path/to/file".to_string(),
                downloaded_size: 2048,
            },
            remote: RemoteFile {
                id: "remote_id_123".to_string(),
                is_uploading_active: false,
            },
        };
        assert_eq!(obj.id, 123);
        assert_eq!(obj.size, 2048);
        assert_eq!(obj.local.path, "/path/to/file");
        assert_eq!(obj.remote.id, "remote_id_123");
    }

    #[test]
    fn test_local_file_empty_path() {
        let local = LocalFile {
            path: String::new(),
            downloaded_size: 0,
        };
        assert!(local.path.is_empty());
        assert_eq!(local.downloaded_size, 0);
    }

    #[test]
    fn test_remote_file_uploading() {
        let remote = RemoteFile {
            id: "file_id".to_string(),
            is_uploading_active: true,
        };
        assert!(remote.is_uploading_active);
    }

    #[test]
    fn test_file_download_object_new() {
        let obj = FileDownloadObject {
            file_id: 1,
            message_id: 100,
            add_date: 1640000000,
            complete_date: 1640000100,
            is_paused: false,
        };
        assert_eq!(obj.file_id, 1);
        assert_eq!(obj.message_id, 100);
        assert!(!obj.is_paused);
    }

    #[test]
    fn test_file_download_object_paused() {
        let obj = FileDownloadObject {
            file_id: 2,
            message_id: 200,
            add_date: 1640000000,
            complete_date: 0,
            is_paused: true,
        };
        assert!(obj.is_paused);
        assert_eq!(obj.complete_date, 0);
    }

    #[test]
    fn test_counters_clone() {
        let counters = Counters {
            active_count: 1,
            paused_count: 2,
            completed_count: 3,
            total_size: 1000,
        };
        let cloned = counters.clone();
        assert_eq!(cloned.active_count, 1);
        assert_eq!(cloned.paused_count, 2);
        assert_eq!(cloned.completed_count, 3);
        assert_eq!(cloned.total_size, 1000);
    }

    #[test]
    fn test_file_view_progress() {
        let view = FileView {
            file_id: FileId::empty(),
            size: 1000,
            downloaded_size: 250,
            is_downloading_completed: false,
        };
        let progress = (view.downloaded_size as f64 / view.size as f64) * 100.0;
        assert_eq!(progress, 25.0);
    }

    #[test]
    fn test_file_object_sizes_match() {
        let obj = FileObject {
            id: 1,
            size: 500,
            expected_size: 500,
            local: LocalFile {
                path: "/path".to_string(),
                downloaded_size: 500,
            },
            remote: RemoteFile {
                id: "remote".to_string(),
                is_uploading_active: false,
            },
        };
        assert_eq!(obj.size, obj.expected_size);
        assert_eq!(obj.local.downloaded_size, obj.size);
    }

    #[test]
    fn test_file_object_sizes_differ() {
        let obj = FileObject {
            id: 1,
            size: 500,
            expected_size: 1000,
            local: LocalFile {
                path: "/path".to_string(),
                downloaded_size: 500,
            },
            remote: RemoteFile {
                id: "remote".to_string(),
                is_uploading_active: false,
            },
        };
        assert!(obj.size != obj.expected_size);
    }

    #[test]
    fn test_file_download_object_incomplete() {
        let obj = FileDownloadObject {
            file_id: 1,
            message_id: 100,
            add_date: 1640000000,
            complete_date: 0,
            is_paused: false,
        };
        assert_eq!(obj.complete_date, 0);
        assert!(!obj.is_paused);
    }

    #[test]
    fn test_remote_file_not_uploading() {
        let remote = RemoteFile {
            id: "file_id".to_string(),
            is_uploading_active: false,
        };
        assert!(!remote.is_uploading_active);
    }

    #[test]
    fn test_file_download_object_complete() {
        let obj = FileDownloadObject {
            file_id: 1,
            message_id: 100,
            add_date: 1640000000,
            complete_date: 1640000100,
            is_paused: false,
        };
        assert!(obj.complete_date > obj.add_date);
    }

    #[test]
    fn test_file_counters_partial() {
        let counters = FileCounters {
            total_count: 10,
            downloaded_count: 5,
        };
        assert!(counters.downloaded_count < counters.total_count);
    }

    #[test]
    fn test_file_counters_complete() {
        let counters = FileCounters {
            total_count: 10,
            downloaded_count: 10,
        };
        assert_eq!(counters.downloaded_count, counters.total_count);
    }

    #[test]
    fn test_counters_total_size_large() {
        let counters = Counters {
            active_count: 1,
            paused_count: 0,
            completed_count: 0,
            total_size: i64::MAX / 2,
        };
        assert!(counters.total_size > 0);
    }

    #[test]
    fn test_file_view_zero_size() {
        let view = FileView {
            file_id: FileId::empty(),
            size: 0,
            downloaded_size: 0,
            is_downloading_completed: true,
        };
        assert_eq!(view.size, 0);
        assert_eq!(view.downloaded_size, 0);
    }

    #[test]
    fn test_file_object_empty_remote() {
        let obj = FileObject {
            id: 1,
            size: 0,
            expected_size: 0,
            local: LocalFile {
                path: String::new(),
                downloaded_size: 0,
            },
            remote: RemoteFile {
                id: String::new(),
                is_uploading_active: false,
            },
        };
        assert!(obj.remote.id.is_empty());
    }

    #[test]
    fn test_local_file_large_size() {
        let local = LocalFile {
            path: "/path".to_string(),
            downloaded_size: i64::MAX / 2,
        };
        assert!(local.downloaded_size > 0);
    }

    #[rstest::rstest]
    #[case(0, 0, 0, 0)]
    #[case(1, 2, 3, 100)]
    #[case(10, 20, 30, 1000)]
    #[case(100, 200, 300, 10000)]
    fn test_counters_various_values(
        #[case] active: i32,
        #[case] paused: i32,
        #[case] completed: i32,
        #[case] total: i64,
    ) {
        let counters = Counters {
            active_count: active,
            paused_count: paused,
            completed_count: completed,
            total_size: total,
        };
        assert_eq!(counters.active_count, active);
        assert_eq!(counters.paused_count, paused);
        assert_eq!(counters.completed_count, completed);
        assert_eq!(counters.total_size, total);
    }

    #[rstest::rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(10, 5)]
    #[case(100, 100)]
    fn test_file_counters_various_values(#[case] total: i32, #[case] downloaded: i32) {
        let counters = FileCounters {
            total_count: total,
            downloaded_count: downloaded,
        };
        assert_eq!(counters.total_count, total);
        assert_eq!(counters.downloaded_count, downloaded);
    }
}
