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

//! Callback trait for file download events.

use rustgram_file_location::{FullLocalFileLocation, PartialLocalFileLocation};

/// Callback for file download events.
///
/// Implement this trait to receive notifications about download progress.
pub trait FileDownloaderCallback: Send + Sync {
    /// Called when download starts.
    fn on_start_download(&self);

    /// Called when a partial download completes.
    ///
    /// # Arguments
    ///
    /// * `partial` - The partial local file location
    /// * `size` - The downloaded size
    fn on_partial_download(&self, partial: PartialLocalFileLocation, size: i64);

    /// Called when download completes successfully.
    ///
    /// # Arguments
    ///
    /// * `full_local` - The full local file location
    /// * `size` - The final file size
    /// * `is_new` - Whether this is a new file
    fn on_ok(&self, full_local: FullLocalFileLocation, size: i64, is_new: bool);

    /// Called when download fails.
    ///
    /// # Arguments
    ///
    /// * `error` - The error message
    fn on_error(&self, error: String);
}

/// Default no-op callback implementation.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DefaultFileDownloaderCallback;

impl FileDownloaderCallback for DefaultFileDownloaderCallback {
    fn on_start_download(&self) {
        // No-op
    }

    fn on_partial_download(&self, _partial: PartialLocalFileLocation, _size: i64) {
        // No-op
    }

    fn on_ok(&self, _full_local: FullLocalFileLocation, _size: i64, _is_new: bool) {
        // No-op
    }

    fn on_error(&self, _error: String) {
        // No-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_callback() {
        let callback = DefaultFileDownloaderCallback::default();
        callback.on_start_download();
        // Should not panic
    }
}
