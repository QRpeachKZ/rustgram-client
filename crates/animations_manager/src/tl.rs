//! TL stub types for AnimationsManager.
//!
//! These are stub implementations of TL types needed for network queries.
//! TODO: Replace with full TL layer implementation when available.

use serde::{Deserialize, Serialize};
use std::fmt;

/// TL stub for InputDocument.
///
/// Represents a reference to an uploaded document for use in Telegram API calls.
/// Used in messages.saveGif query to reference animations.
///
/// # TDLib Correspondence
///
/// This stub corresponds to the TL type `InputDocument` from Telegram's scheme.
///
/// # Examples
///
/// ```
/// use rustgram_animations_manager::tl::InputDocument;
///
/// let empty = InputDocument::Empty;
/// assert!(matches!(empty, InputDocument::Empty));
///
/// let doc = InputDocument::Document {
///     id: 123,
///     access_hash: 456,
///     file_reference: vec![1, 2, 3],
/// };
/// assert!(matches!(doc, InputDocument::Document { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub enum InputDocument {
    /// TDLib: inputDocumentEmpty
    ///
    /// Represents an empty/invalid document reference.
    #[default]
    Empty,

    /// TDLib: inputDocument
    ///
    /// Represents a valid document reference with ID, access hash, and file reference.
    Document {
        /// Document identifier (i64)
        id: i64,
        /// Access hash for the document
        access_hash: i64,
        /// File reference bytes for file reference repair
        file_reference: Vec<u8>,
    },
}

impl fmt::Display for InputDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "inputDocumentEmpty"),
            Self::Document { id, .. } => write!(f, "inputDocument(id={})", id),
        }
    }
}

/// TL stub for SavedGifs.
///
/// Represents the response from messages.getSavedGifs query.
/// Contains either a hash-only response (if not modified) or full animation list.
///
/// # TDLib Correspondence
///
/// This stub corresponds to the TL type `messages.SavedGifs` from Telegram's scheme.
///
/// # Examples
///
/// ```
/// use rustgram_animations_manager::tl::SavedGifs;
///
/// let not_modified = SavedGifs::NotModified;
/// assert!(matches!(not_modified, SavedGifs::NotModified));
///
/// let saved = SavedGifs::SavedGifs {
///     hash: 12345,
///     gif_ids: vec![1, 2, 3],
/// };
/// assert!(matches!(saved, SavedGifs::SavedGifs { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SavedGifs {
    /// TDLib: messages.savedGifsNotModified
    ///
    /// Server indicates animations haven't changed (hash matches).
    NotModified,

    /// TDLib: messages.savedGifs
    ///
    /// Full response with hash and animation IDs.
    SavedGifs {
        /// Hash for incremental updates
        hash: i64,
        /// Vector of document IDs representing saved animations
        /// TODO: Full implementation would have Vector<Document> here
        gif_ids: Vec<i64>,
    },
}

impl SavedGifs {
    /// Returns the hash from the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::tl::SavedGifs;
    ///
    /// let not_modified = SavedGifs::NotModified;
    /// assert_eq!(not_modified.get_hash(), 0);
    ///
    /// let saved = SavedGifs::SavedGifs {
    ///     hash: 12345,
    ///     gif_ids: vec![],
    /// };
    /// assert_eq!(saved.get_hash(), 12345);
    /// ```
    #[must_use]
    pub const fn get_hash(&self) -> i64 {
        match self {
            Self::NotModified => 0,
            Self::SavedGifs { hash, .. } => *hash,
        }
    }

    /// Returns the animation IDs from the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::tl::SavedGifs;
    ///
    /// let not_modified = SavedGifs::NotModified;
    /// assert!(not_modified.get_gif_ids().is_empty());
    ///
    /// let saved = SavedGifs::SavedGifs {
    ///     hash: 0,
    ///     gif_ids: vec![1, 2, 3],
    /// };
    /// assert_eq!(saved.get_gif_ids(), &[1, 2, 3]);
    /// ```
    #[must_use]
    pub fn get_gif_ids(&self) -> &[i64] {
        match self {
            Self::NotModified => &[],
            Self::SavedGifs { gif_ids, .. } => gif_ids,
        }
    }
}

impl fmt::Display for SavedGifs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotModified => write!(f, "messages.savedGifsNotModified"),
            Self::SavedGifs { hash, gif_ids } => {
                write!(
                    f,
                    "messages.savedGifs(hash={}, count={})",
                    hash,
                    gif_ids.len()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === InputDocument tests ===

    #[test]
    fn test_input_document_empty() {
        let empty = InputDocument::Empty;
        assert!(matches!(empty, InputDocument::Empty));
        assert_eq!(format!("{}", empty), "inputDocumentEmpty");
    }

    #[test]
    fn test_input_document_document() {
        let doc = InputDocument::Document {
            id: 123,
            access_hash: 456,
            file_reference: vec![1, 2, 3],
        };
        assert!(matches!(doc, InputDocument::Document { .. }));
        assert_eq!(format!("{}", doc), "inputDocument(id=123)");
    }

    #[test]
    fn test_input_document_default() {
        let empty: InputDocument = Default::default();
        assert!(matches!(empty, InputDocument::Empty));
    }

    #[test]
    fn test_input_document_clone() {
        let doc = InputDocument::Document {
            id: 123,
            access_hash: 456,
            file_reference: vec![1, 2, 3],
        };
        let cloned = doc.clone();
        assert_eq!(doc, cloned);
    }

    #[test]
    fn test_input_document_equality() {
        let doc1 = InputDocument::Document {
            id: 123,
            access_hash: 456,
            file_reference: vec![1, 2, 3],
        };
        let doc2 = InputDocument::Document {
            id: 123,
            access_hash: 456,
            file_reference: vec![1, 2, 3],
        };
        assert_eq!(doc1, doc2);

        let doc3 = InputDocument::Document {
            id: 456,
            access_hash: 789,
            file_reference: vec![4, 5, 6],
        };
        assert_ne!(doc1, doc3);
    }

    // === SavedGifs tests ===

    #[test]
    fn test_saved_gifs_not_modified() {
        let not_modified = SavedGifs::NotModified;
        assert!(matches!(not_modified, SavedGifs::NotModified));
        assert_eq!(format!("{}", not_modified), "messages.savedGifsNotModified");
    }

    #[test]
    fn test_saved_gifs_saved_gifs() {
        let saved = SavedGifs::SavedGifs {
            hash: 12345,
            gif_ids: vec![1, 2, 3],
        };
        assert!(matches!(saved, SavedGifs::SavedGifs { .. }));
        assert_eq!(
            format!("{}", saved),
            "messages.savedGifs(hash=12345, count=3)"
        );
    }

    #[test]
    fn test_saved_gifs_get_hash() {
        assert_eq!(SavedGifs::NotModified.get_hash(), 0);
        assert_eq!(
            SavedGifs::SavedGifs {
                hash: 12345,
                gif_ids: vec![]
            }
            .get_hash(),
            12345
        );
    }

    #[test]
    fn test_saved_gifs_get_gif_ids() {
        assert!(SavedGifs::NotModified.get_gif_ids().is_empty());
        assert_eq!(
            SavedGifs::SavedGifs {
                hash: 0,
                gif_ids: vec![1, 2, 3]
            }
            .get_gif_ids(),
            &[1, 2, 3]
        );
    }

    #[test]
    fn test_saved_gifs_clone() {
        let saved = SavedGifs::SavedGifs {
            hash: 12345,
            gif_ids: vec![1, 2, 3],
        };
        let cloned = saved.clone();
        assert_eq!(saved, cloned);
    }

    #[test]
    fn test_saved_gifs_equality() {
        let saved1 = SavedGifs::SavedGifs {
            hash: 12345,
            gif_ids: vec![1, 2, 3],
        };
        let saved2 = SavedGifs::SavedGifs {
            hash: 12345,
            gif_ids: vec![1, 2, 3],
        };
        assert_eq!(saved1, saved2);

        let saved3 = SavedGifs::SavedGifs {
            hash: 54321,
            gif_ids: vec![4, 5, 6],
        };
        assert_ne!(saved1, saved3);
    }
}
