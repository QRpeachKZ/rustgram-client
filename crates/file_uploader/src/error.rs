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

//! Errors for file uploader operations.

use thiserror::Error;

/// Errors that can occur in file uploader operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// Parts manager error
    #[error("parts manager error: {0}")]
    PartsManagerError(String),

    /// Invalid state for the requested operation
    #[error("invalid state for operation")]
    InvalidState,

    /// Upload is not active
    #[error("upload is not active")]
    NotActive,

    /// Invalid part ID
    #[error("invalid part ID")]
    InvalidPartId,

    /// IO error occurred
    #[error("I/O error: {0}")]
    IoError(String),

    /// Network error occurred
    #[error("network error: {0}")]
    NetworkError(String),

    /// File not found
    #[error("file not found: {0}")]
    FileNotFound(String),

    /// File size mismatch
    #[error("file size mismatch: expected {expected}, got {actual}")]
    SizeMismatch {
        /// Expected size
        expected: i64,
        /// Actual size
        actual: i64,
    },

    /// Encryption error
    #[error("encryption error: {0}")]
    EncryptionError(String),

    /// Read error
    #[error("read error: {0}")]
    ReadError(String),

    /// Resource manager error
    #[error("resource manager error: {0}")]
    ResourceError(String),
}

impl From<rustgram_parts_manager::Error> for Error {
    fn from(err: rustgram_parts_manager::Error) -> Self {
        Self::PartsManagerError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::InvalidState),
            "invalid state for operation"
        );
        assert_eq!(
            format!("{}", Error::NotActive),
            "upload is not active"
        );
        assert_eq!(format!("{}", Error::IoError("test".to_string())), "I/O error: test");
    }

    #[test]
    fn test_size_mismatch() {
        let error = Error::SizeMismatch {
            expected: 1000,
            actual: 500,
        };
        assert!(format!("{}", error).contains("expected"));
        assert!(format!("{}", error).contains("actual"));
    }

    #[test]
    fn test_parts_manager_error_conversion() {
        let parts_err = rustgram_parts_manager::Error::InvalidPartId;
        let upload_err: Error = parts_err.into();
        assert!(upload_err.to_string().contains("parts manager error"));
    }
}
