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

//! # Documents Manager
//!
//! Manager for document metadata and storage.
//!
//! ## TDLib Alignment
//!
//! Simplified version of TDLib's `DocumentsManager` that manages
//! document metadata like file names, mime types, and thumbnails.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_documents_manager::{DocumentsManager, DocumentError};
//! use rustgram_document::{Document, DocumentType};
//! use rustgram_file_id::FileId;
//!
//! let mut manager = DocumentsManager::new();
//!
//! let file_id = FileId::new(123, 0);
//! let doc = Document::new(DocumentType::Video, file_id);
//!
//! manager.add_document(doc.clone()).unwrap();
//! assert!(manager.has_document(file_id));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_document::Document;
use rustgram_file_id::FileId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub mod error;
pub use error::{DocumentError, Result};

/// Manager for document metadata.
///
/// Stores and manages documents by their FileId.
#[derive(Debug, Clone)]
pub struct DocumentsManager {
    /// Map of file_id -> Document
    documents: HashMap<FileId, Document>,
}

impl Default for DocumentsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentsManager {
    /// Creates a new documents manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::DocumentsManager;
    ///
    /// let manager = DocumentsManager::new();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Adds a document to the manager.
    ///
    /// # Errors
    ///
    /// Returns `DocumentError::AlreadyExists` if a document with the same FileId already exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::{DocumentsManager, DocumentError};
    /// use rustgram_document::{Document, DocumentType};
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DocumentsManager::new();
    /// let file_id = FileId::new(123, 0);
    /// let doc = Document::new(DocumentType::Video, file_id);
    ///
    /// manager.add_document(doc).unwrap();
    /// ```
    pub fn add_document(&mut self, document: Document) -> Result<()> {
        let file_id = document.file_id();
        if self.documents.contains_key(&file_id) {
            return Err(DocumentError::AlreadyExists { file_id });
        }
        self.documents.insert(file_id, document);
        Ok(())
    }

    /// Removes a document from the manager.
    ///
    /// # Errors
    ///
    /// Returns `DocumentError::NotFound` if the document doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::{DocumentsManager, DocumentError};
    /// use rustgram_document::{Document, DocumentType};
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DocumentsManager::new();
    /// let file_id = FileId::new(123, 0);
    /// let doc = Document::new(DocumentType::Video, file_id);
    ///
    /// manager.add_document(doc).unwrap();
    /// manager.remove_document(file_id).unwrap();
    /// assert!(!manager.has_document(file_id));
    /// ```
    pub fn remove_document(&mut self, file_id: FileId) -> Result<()> {
        self.documents
            .remove(&file_id)
            .ok_or(DocumentError::NotFound { file_id })?;
        Ok(())
    }

    /// Gets a document by its FileId.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::DocumentsManager;
    /// use rustgram_document::{Document, DocumentType};
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DocumentsManager::new();
    /// let file_id = FileId::new(123, 0);
    /// let doc = Document::new(DocumentType::Audio, file_id);
    ///
    /// manager.add_document(doc).unwrap();
    /// let retrieved = manager.get_document(file_id).unwrap();
    /// assert_eq!(retrieved.doc_type(), DocumentType::Audio);
    /// ```
    pub fn get_document(&self, file_id: FileId) -> Option<&Document> {
        self.documents.get(&file_id)
    }

    /// Checks if a document exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::DocumentsManager;
    /// use rustgram_document::{Document, DocumentType};
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DocumentsManager::new();
    /// let file_id = FileId::new(123, 0);
    /// let doc = Document::new(DocumentType::Video, file_id);
    ///
    /// manager.add_document(doc).unwrap();
    /// assert!(manager.has_document(file_id));
    /// ```
    pub fn has_document(&self, file_id: FileId) -> bool {
        self.documents.contains_key(&file_id)
    }

    /// Gets all documents.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::DocumentsManager;
    /// use rustgram_document::{Document, DocumentType};
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DocumentsManager::new();
    ///
    /// for i in 1..=3 {
    ///     let file_id = FileId::new(i, 0);
    ///     let doc = Document::new(DocumentType::Video, file_id);
    ///     manager.add_document(doc).unwrap();
    /// }
    ///
    /// assert_eq!(manager.get_all().len(), 3);
    /// ```
    pub fn get_all(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    /// Gets the total number of documents.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::DocumentsManager;
    /// use rustgram_document::{Document, DocumentType};
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DocumentsManager::new();
    /// let file_id = FileId::new(123, 0);
    /// let doc = Document::new(DocumentType::Video, file_id);
    ///
    /// manager.add_document(doc).unwrap();
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.documents.len()
    }

    /// Clears all documents.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_documents_manager::DocumentsManager;
    /// use rustgram_document::{Document, DocumentType};
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DocumentsManager::new();
    /// let file_id = FileId::new(123, 0);
    /// let doc = Document::new(DocumentType::Video, file_id);
    ///
    /// manager.add_document(doc).unwrap();
    /// manager.clear();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.documents.clear();
    }
}

impl Serialize for DocumentsManager {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.documents.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DocumentsManager {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let documents = HashMap::deserialize(deserializer)?;
        Ok(Self { documents })
    }
}

impl fmt::Display for DocumentsManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DocumentsManager(count={})", self.count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_document::DocumentType;

    // Creation tests (2)
    #[test]
    fn test_new() {
        let manager = DocumentsManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_default() {
        let manager = DocumentsManager::default();
        assert_eq!(manager.count(), 0);
    }

    // Add document tests (3)
    #[test]
    fn test_add_document() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Video, file_id);

        let result = manager.add_document(doc);
        assert!(result.is_ok());
        assert!(manager.has_document(file_id));
    }

    #[test]
    fn test_add_document_duplicate() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Video, file_id);

        manager.add_document(doc.clone()).unwrap();
        let result = manager.add_document(doc);
        assert!(matches!(result, Err(DocumentError::AlreadyExists { .. })));
    }

    #[test]
    fn test_add_multiple_documents() {
        let mut manager = DocumentsManager::new();

        for i in 1..=5 {
            let file_id = FileId::new(i, 0);
            let doc = Document::new(DocumentType::Video, file_id);
            manager.add_document(doc).unwrap();
        }

        assert_eq!(manager.count(), 5);
    }

    // Remove document tests (2)
    #[test]
    fn test_remove_document() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Video, file_id);

        manager.add_document(doc).unwrap();
        let result = manager.remove_document(file_id);
        assert!(result.is_ok());
        assert!(!manager.has_document(file_id));
    }

    #[test]
    fn test_remove_document_not_found() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);

        let result = manager.remove_document(file_id);
        assert!(matches!(result, Err(DocumentError::NotFound { .. })));
    }

    // Get document tests (2)
    #[test]
    fn test_get_document() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Audio, file_id);

        manager.add_document(doc).unwrap();
        let retrieved = manager.get_document(file_id).unwrap();
        assert_eq!(retrieved.doc_type(), DocumentType::Audio);
    }

    #[test]
    fn test_get_document_not_found() {
        let manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);

        assert!(manager.get_document(file_id).is_none());
    }

    // Has document tests (2)
    #[test]
    fn test_has_document_true() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Video, file_id);

        manager.add_document(doc).unwrap();
        assert!(manager.has_document(file_id));
    }

    #[test]
    fn test_has_document_false() {
        let manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);

        assert!(!manager.has_document(file_id));
    }

    // Get all tests (2)
    #[test]
    fn test_get_all_empty() {
        let manager = DocumentsManager::new();
        assert!(manager.get_all().is_empty());
    }

    #[test]
    fn test_get_all() {
        let mut manager = DocumentsManager::new();

        for i in 1..=3 {
            let file_id = FileId::new(i, 0);
            let doc = Document::new(DocumentType::Video, file_id);
            manager.add_document(doc).unwrap();
        }

        assert_eq!(manager.get_all().len(), 3);
    }

    // Count tests (2)
    #[test]
    fn test_count_empty() {
        let manager = DocumentsManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_count() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Video, file_id);

        manager.add_document(doc).unwrap();
        assert_eq!(manager.count(), 1);
    }

    // Clear tests (1)
    #[test]
    fn test_clear() {
        let mut manager = DocumentsManager::new();

        for i in 1..=5 {
            let file_id = FileId::new(i, 0);
            let doc = Document::new(DocumentType::Video, file_id);
            manager.add_document(doc).unwrap();
        }

        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    // Display tests (1)
    #[test]
    fn test_display() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Video, file_id);

        manager.add_document(doc).unwrap();
        let display = format!("{}", manager);
        assert!(display.contains("DocumentsManager"));
        assert!(display.contains("count=1"));
    }

    // Clone tests (1)
    #[test]
    fn test_clone() {
        let mut manager = DocumentsManager::new();
        let file_id = FileId::new(123, 0);
        let doc = Document::new(DocumentType::Video, file_id);

        manager.add_document(doc).unwrap();
        let cloned = manager.clone();
        assert!(cloned.has_document(file_id));
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize() {
        let mut manager = DocumentsManager::new();

        for i in 1..=3 {
            let file_id = FileId::new(i, 0);
            let doc = Document::new(DocumentType::Video, file_id);
            manager.add_document(doc).unwrap();
        }

        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: DocumentsManager = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.count(), 3);
    }

    #[test]
    fn test_serialize_empty() {
        let manager = DocumentsManager::new();

        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: DocumentsManager = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.count(), 0);
    }
}
