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

//! # Document
//!
//! Represents a document (file) in Telegram messages.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `Document` struct from `td/telegram/Document.h`.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_file_id::FileId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Document type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum DocumentType {
    Unknown = 0,
    Animation = 1,
    Audio = 2,
    General = 3,
    Sticker = 4,
    Video = 5,
    VideoNote = 6,
    VoiceNote = 7,
}

impl Default for DocumentType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl DocumentType {
    pub const fn is_video(self) -> bool {
        matches!(self, Self::Video | Self::VideoNote | Self::Animation)
    }

    pub const fn is_audio(self) -> bool {
        matches!(self, Self::Audio | Self::VoiceNote)
    }
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Animation => write!(f, "Animation"),
            Self::Audio => write!(f, "Audio"),
            Self::General => write!(f, "Document"),
            Self::Sticker => write!(f, "Sticker"),
            Self::Video => write!(f, "Video"),
            Self::VideoNote => write!(f, "VideoNote"),
            Self::VoiceNote => write!(f, "VoiceNote"),
        }
    }
}

/// Document representing a file in Telegram.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    doc_type: DocumentType,
    file_id: FileId,
}

impl Document {
    pub fn new(doc_type: DocumentType, file_id: FileId) -> Self {
        Self { doc_type, file_id }
    }

    pub const fn doc_type(&self) -> DocumentType {
        self.doc_type
    }

    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    pub const fn empty(&self) -> bool {
        matches!(self.doc_type, DocumentType::Unknown)
    }

    pub fn has_valid_file(&self) -> bool {
        self.file_id.is_valid()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            doc_type: DocumentType::default(),
            file_id: FileId::empty(),
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.doc_type, self.file_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let doc = Document::new(DocumentType::Video, FileId::new(123, 0));
        assert!(format!("{:?}", doc).contains("Document"));
    }

    #[test]
    fn test_clone() {
        let doc = Document::new(DocumentType::Video, FileId::new(123, 0));
        let cloned = doc.clone();
        assert_eq!(doc, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let d1 = Document::new(DocumentType::Video, FileId::new(123, 0));
        let d2 = Document::new(DocumentType::Video, FileId::new(123, 0));
        let d3 = Document::new(DocumentType::Audio, FileId::new(123, 0));
        assert_eq!(d1, d2);
        assert_ne!(d1, d3);
    }

    #[test]
    fn test_default() {
        let doc = Document::default();
        assert_eq!(doc.doc_type(), DocumentType::Unknown);
        assert!(doc.file_id().is_empty());
    }

    #[test]
    fn test_display() {
        let doc = Document::new(DocumentType::Video, FileId::new(123, 0));
        let display = format!("{}", doc);
        assert!(display.contains("Video"));
        assert!(display.contains("123"));
    }

    #[test]
    fn test_document_type_display() {
        assert_eq!(format!("{}", DocumentType::Unknown), "Unknown");
        assert_eq!(format!("{}", DocumentType::Animation), "Animation");
        assert_eq!(format!("{}", DocumentType::Audio), "Audio");
        assert_eq!(format!("{}", DocumentType::General), "Document");
        assert_eq!(format!("{}", DocumentType::Sticker), "Sticker");
        assert_eq!(format!("{}", DocumentType::Video), "Video");
        assert_eq!(format!("{}", DocumentType::VideoNote), "VideoNote");
        assert_eq!(format!("{}", DocumentType::VoiceNote), "VoiceNote");
    }

    #[test]
    fn test_document_type_is_video() {
        assert!(DocumentType::Video.is_video());
        assert!(DocumentType::VideoNote.is_video());
        assert!(DocumentType::Animation.is_video());
        assert!(!DocumentType::Audio.is_video());
        assert!(!DocumentType::General.is_video());
    }

    #[test]
    fn test_document_type_is_audio() {
        assert!(DocumentType::Audio.is_audio());
        assert!(DocumentType::VoiceNote.is_audio());
        assert!(!DocumentType::Video.is_audio());
        assert!(!DocumentType::General.is_audio());
    }

    #[test]
    fn test_document_type_default() {
        assert_eq!(DocumentType::default(), DocumentType::Unknown);
    }

    #[test]
    fn test_new_video() {
        let doc = Document::new(DocumentType::Video, FileId::new(123, 0));
        assert_eq!(doc.doc_type(), DocumentType::Video);
        assert_eq!(doc.file_id().get(), 123);
    }

    #[test]
    fn test_new_audio() {
        let doc = Document::new(DocumentType::Audio, FileId::new(456, 0));
        assert_eq!(doc.doc_type(), DocumentType::Audio);
        assert_eq!(doc.file_id().get(), 456);
    }

    #[test]
    fn test_new_sticker() {
        let doc = Document::new(DocumentType::Sticker, FileId::new(789, 0));
        assert_eq!(doc.doc_type(), DocumentType::Sticker);
    }

    #[test]
    fn test_new_general() {
        let doc = Document::new(DocumentType::General, FileId::new(999, 0));
        assert_eq!(doc.doc_type(), DocumentType::General);
    }

    #[test]
    fn test_new_unknown() {
        let doc = Document::new(DocumentType::Unknown, FileId::new(0, 0));
        assert_eq!(doc.doc_type(), DocumentType::Unknown);
    }

    #[test]
    fn test_doc_type() {
        let doc = Document::new(DocumentType::VideoNote, FileId::new(1, 0));
        assert_eq!(doc.doc_type(), DocumentType::VideoNote);
    }

    #[test]
    fn test_file_id() {
        let doc = Document::new(DocumentType::Video, FileId::new(123, 456));
        assert_eq!(doc.file_id().get(), 123);
        assert_eq!(doc.file_id().get_remote(), 456);
    }

    #[test]
    fn test_empty_true() {
        let doc = Document::new(DocumentType::Unknown, FileId::new(123, 0));
        assert!(doc.empty());
    }

    #[test]
    fn test_empty_false() {
        let doc = Document::new(DocumentType::Video, FileId::new(0, 0));
        assert!(!doc.empty());
    }

    #[test]
    fn test_has_valid_file_true() {
        let doc = Document::new(DocumentType::Video, FileId::new(123, 0));
        assert!(doc.has_valid_file());
    }

    #[test]
    fn test_has_valid_file_false_zero_id() {
        let doc = Document::new(DocumentType::Video, FileId::new(0, 0));
        assert!(!doc.has_valid_file());
    }

    #[test]
    fn test_has_valid_file_false_negative_id() {
        let doc = Document::new(DocumentType::Video, FileId::new(-1, 0));
        assert!(!doc.has_valid_file());
    }

    #[test]
    fn test_serialize_document_type() {
        let dt = DocumentType::Video;
        let json = serde_json::to_string(&dt).unwrap();
        let parsed: DocumentType = serde_json::from_str(&json).unwrap();
        assert_eq!(dt, parsed);
    }

    #[test]
    fn test_equality_same_type_and_file() {
        let d1 = Document::new(DocumentType::Video, FileId::new(123, 0));
        let d2 = Document::new(DocumentType::Video, FileId::new(123, 0));
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_equality_different_type() {
        let d1 = Document::new(DocumentType::Video, FileId::new(123, 0));
        let d2 = Document::new(DocumentType::Audio, FileId::new(123, 0));
        assert_ne!(d1, d2);
    }

    #[test]
    fn test_equality_different_file() {
        let d1 = Document::new(DocumentType::Video, FileId::new(123, 0));
        let d2 = Document::new(DocumentType::Video, FileId::new(456, 0));
        assert_ne!(d1, d2);
    }
}
