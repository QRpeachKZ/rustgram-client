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

//! Error types for StorageManager.

use std::fmt;

/// Result type alias for StorageManager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in StorageManager operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Storage path not found or inaccessible.
    StoragePathNotFound,
    /// Invalid storage parameters.
    InvalidParameters,
    /// GC operation failed.
    GcFailed,
    /// Database error.
    DatabaseError(String),
    /// IO error.
    IoError(String),
    /// Internal error.
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StoragePathNotFound => write!(f, "Storage path not found"),
            Self::InvalidParameters => write!(f, "Invalid storage parameters"),
            Self::GcFailed => write!(f, "Garbage collection failed"),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(format!("{}", Error::StoragePathNotFound), "Storage path not found");
        assert_eq!(format!("{}", Error::InvalidParameters), "Invalid storage parameters");
    }
}
