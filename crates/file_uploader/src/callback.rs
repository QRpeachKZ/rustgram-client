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

//! Callback trait for file upload events.

use rustgram_file_id::FileId;

/// Callback for file upload events.
///
/// Implement this trait to receive notifications about upload progress.
pub trait FileUploaderCallback: Send + Sync {
    /// Called when upload starts.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID
    fn on_start_upload(&self, file_id: FileId);

    /// Called when upload completes successfully.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID
    /// * `size` - The final file size
    fn on_ok(&self, file_id: FileId, size: i64);

    /// Called when upload fails.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID
    /// * `error` - The error message
    fn on_error(&self, file_id: FileId, error: String);

    /// Called on upload progress.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID
    /// * `progress` - Progress percentage (0-100)
    fn on_progress(&self, file_id: FileId, progress: f64);
}

/// Default no-op callback implementation.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DefaultFileUploaderCallback;

impl FileUploaderCallback for DefaultFileUploaderCallback {
    fn on_start_upload(&self, _file_id: FileId) {
        // No-op
    }

    fn on_ok(&self, _file_id: FileId, _size: i64) {
        // No-op
    }

    fn on_error(&self, _file_id: FileId, _error: String) {
        // No-op
    }

    fn on_progress(&self, _file_id: FileId, _progress: f64) {
        // No-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_callback() {
        let callback = DefaultFileUploaderCallback::default();
        let file_id = FileId::new(123, 0);
        callback.on_start_upload(file_id);
        callback.on_progress(file_id, 50.0);
        // Should not panic
    }
}
