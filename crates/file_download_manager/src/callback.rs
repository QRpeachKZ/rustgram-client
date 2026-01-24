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

//! Callback trait for download manager events.

use rustgram_file_id::FileId;

/// Callback for download manager events.
///
/// Implement this trait to receive notifications about download progress and state changes.
pub trait FileDownloadManagerCallback: Send + Sync {
    /// Called when a new download is added to the queue.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    fn on_download_added(&self, download_id: u64, file_id: FileId);

    /// Called when a download starts.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    fn on_download_started(&self, download_id: u64, file_id: FileId);

    /// Called when download progress is updated.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    /// * `downloaded` - Number of bytes downloaded
    /// * `total` - Total file size in bytes
    /// * `progress` - Progress percentage (0.0 to 100.0)
    fn on_download_progress(
        &self,
        download_id: u64,
        file_id: FileId,
        downloaded: i64,
        total: i64,
        progress: f64,
    );

    /// Called when a download is paused.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    fn on_download_paused(&self, download_id: u64, file_id: FileId);

    /// Called when a paused download is resumed.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    fn on_download_resumed(&self, download_id: u64, file_id: FileId);

    /// Called when a download completes successfully.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    fn on_download_completed(&self, download_id: u64, file_id: FileId);

    /// Called when a download fails.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    /// * `error` - The error message
    fn on_download_failed(&self, download_id: u64, file_id: FileId, error: String);

    /// Called when a download is cancelled.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    fn on_download_cancelled(&self, download_id: u64, file_id: FileId);

    /// Called when a download is removed from the manager.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The unique download identifier
    /// * `file_id` - The file ID being downloaded
    fn on_download_removed(&self, download_id: u64, file_id: FileId);
}

/// Default no-op callback implementation.
#[derive(Debug, Default)]
pub struct DefaultFileDownloadManagerCallback;

impl FileDownloadManagerCallback for DefaultFileDownloadManagerCallback {
    fn on_download_added(&self, _download_id: u64, _file_id: FileId) {
        // No-op
    }

    fn on_download_started(&self, _download_id: u64, _file_id: FileId) {
        // No-op
    }

    fn on_download_progress(
        &self,
        _download_id: u64,
        _file_id: FileId,
        _downloaded: i64,
        _total: i64,
        _progress: f64,
    ) {
        // No-op
    }

    fn on_download_paused(&self, _download_id: u64, _file_id: FileId) {
        // No-op
    }

    fn on_download_resumed(&self, _download_id: u64, _file_id: FileId) {
        // No-op
    }

    fn on_download_completed(&self, _download_id: u64, _file_id: FileId) {
        // No-op
    }

    fn on_download_failed(&self, _download_id: u64, _file_id: FileId, _error: String) {
        // No-op
    }

    fn on_download_cancelled(&self, _download_id: u64, _file_id: FileId) {
        // No-op
    }

    fn on_download_removed(&self, _download_id: u64, _file_id: FileId) {
        // No-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_callback() {
        let callback = DefaultFileDownloadManagerCallback::default();
        let file_id = FileId::new(1, 0);

        // Should not panic
        callback.on_download_added(1, file_id);
        callback.on_download_started(1, file_id);
        callback.on_download_progress(1, file_id, 500, 1000, 50.0);
        callback.on_download_paused(1, file_id);
        callback.on_download_resumed(1, file_id);
        callback.on_download_completed(1, file_id);
        callback.on_download_failed(1, file_id, "error".to_string());
        callback.on_download_cancelled(1, file_id);
        callback.on_download_removed(1, file_id);
    }
}
