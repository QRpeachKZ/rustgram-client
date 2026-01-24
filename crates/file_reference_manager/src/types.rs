//! Supporting types for file reference manager.

use rustgram_dialog_id::DialogId;
use serde::{Deserialize, Serialize};

/// Full identifier for a story.
///
/// Combines the story ID with the dialog ID where the story was posted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoryFullId {
    /// The story identifier.
    pub story_id: i32,

    /// The dialog where the story was posted.
    pub dialog_id: DialogId,
}

/// Full identifier for a story album.
///
/// Combines the album ID with the dialog ID where the album was posted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoryAlbumFullId {
    /// The dialog where the album was posted.
    pub dialog_id: DialogId,

    /// The album identifier.
    pub album_id: i64,
}
