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

//! Error types for file database operations.

use std::fmt;

/// File database error type.
///
/// Represents errors that can occur during file database operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileDbError {
    /// File not found in the database.
    NotFound(String),

    /// Invalid file reference (circular or broken reference).
    InvalidRef(String),

    /// I/O error during database operation.
    Io(String),

    /// Serialization/deserialization error.
    Serialization(String),

    /// Database-specific error.
    Database(String),

    /// Generic error with message.
    Other(String),
}

impl fmt::Display for FileDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "File not found: {}", msg),
            Self::InvalidRef(msg) => write!(f, "Invalid file reference: {}", msg),
            Self::Io(msg) => write!(f, "I/O error: {}", msg),
            Self::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for FileDbError {}

impl FileDbError {
    /// Returns `true` if this error indicates that a file was not found.
    #[must_use]
    pub const fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound(_))
    }

    /// Creates a new `NotFound` error with the given message.
    #[must_use]
    pub const fn not_found(msg: String) -> Self {
        Self::NotFound(msg)
    }

    /// Creates a new `InvalidRef` error with the given message.
    #[must_use]
    pub const fn invalid_ref(msg: String) -> Self {
        Self::InvalidRef(msg)
    }

    /// Creates a new `Io` error from an `std::io::Error`.
    #[must_use]
    pub fn io(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }

    /// Creates a new `Serialization` error with the given message.
    #[must_use]
    pub const fn serialization(msg: String) -> Self {
        Self::Serialization(msg)
    }

    /// Creates a new `Database` error with the given message.
    #[must_use]
    pub const fn database(msg: String) -> Self {
        Self::Database(msg)
    }
}

impl From<std::io::Error> for FileDbError {
    fn from(err: std::io::Error) -> Self {
        Self::io(err)
    }
}

/// Result type for file database operations.
pub type FileDbResult<T> = Result<T, FileDbError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = FileDbError::NotFound("test_key".to_string());
        assert_eq!(err.to_string(), "File not found: test_key");

        let err = FileDbError::InvalidRef("circular ref".to_string());
        assert_eq!(err.to_string(), "Invalid file reference: circular ref");

        let err = FileDbError::Serialization("parse error".to_string());
        assert_eq!(err.to_string(), "Serialization error: parse error");
    }

    #[test]
    fn test_error_is_not_found() {
        let err = FileDbError::NotFound("key".to_string());
        assert!(err.is_not_found());

        let err = FileDbError::InvalidRef("ref".to_string());
        assert!(!err.is_not_found());

        let err = FileDbError::Io("error".to_string());
        assert!(!err.is_not_found());
    }

    #[test]
    fn test_error_constructors() {
        let err = FileDbError::not_found("test".to_string());
        assert!(matches!(err, FileDbError::NotFound(_)));

        let err = FileDbError::invalid_ref("test".to_string());
        assert!(matches!(err, FileDbError::InvalidRef(_)));

        let err = FileDbError::serialization("test".to_string());
        assert!(matches!(err, FileDbError::Serialization(_)));

        let err = FileDbError::database("test".to_string());
        assert!(matches!(err, FileDbError::Database(_)));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let db_err = FileDbError::from(io_err);
        assert!(matches!(db_err, FileDbError::Io(_)));
        assert!(db_err.to_string().contains("file not found"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = FileDbError::NotFound("key".to_string());
        let err2 = FileDbError::NotFound("key".to_string());
        let err3 = FileDbError::NotFound("other".to_string());

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
