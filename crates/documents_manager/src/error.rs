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

//! Error types for documents manager.

use rustgram_file_id::FileId;
use std::fmt;

/// Error type for document operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentError {
    /// Document already exists.
    AlreadyExists {
        /// The file ID of the existing document.
        file_id: FileId,
    },
    /// Document not found.
    NotFound {
        /// The file ID that was not found.
        file_id: FileId,
    },
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { file_id } => {
                write!(f, "Document {} already exists", file_id.get())
            }
            Self::NotFound { file_id } => {
                write!(f, "Document {} not found", file_id.get())
            }
        }
    }
}

impl std::error::Error for DocumentError {}

/// Result type for document operations.
pub type Result<T> = std::result::Result<T, DocumentError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_already_exists() {
        let file_id = FileId::new(123, 0);
        let error = DocumentError::AlreadyExists { file_id };
        let display = format!("{}", error);
        assert!(display.contains("already exists"));
        assert!(display.contains("123"));
    }

    #[test]
    fn test_error_display_not_found() {
        let file_id = FileId::new(456, 0);
        let error = DocumentError::NotFound { file_id };
        let display = format!("{}", error);
        assert!(display.contains("not found"));
        assert!(display.contains("456"));
    }
}
