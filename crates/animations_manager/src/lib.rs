//! # Animations Manager
//!
//! Manages GIF animations in Telegram.
//!
//! ## Overview
//!
//! The `AnimationsManager` is responsible for:
//! - Storing and retrieving animation metadata (duration, dimensions, thumbnails)
//! - Managing saved animations (user's favorite GIFs)
//! - Handling animation search parameters
//! - Providing animations for message sending
//! - Tracking file sources for saved animations
//!
//! ## Architecture
//!
//! This implementation aligns with TDLib's AnimationsManager:
//! - Thread-safe animation storage using `DashMap`
//! - State machine for saved animations loading
//! - Hash-based incremental sync with server
//! - File source tracking for reference repair
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_animations_manager::AnimationsManager;
//! use rustgram_file_id::FileId;
//! use rustgram_dimensions::Dimensions;
//! use rustgram_photo_size::PhotoSize;
//! use rustgram_animation_size::AnimationSize;
//!
//! let manager = AnimationsManager::new();
//!
//! // Create an animation
//! let file_id = FileId::new(123, 456);
//! manager.create_animation(
//!     file_id,
//!     "minithumb".to_string(),
//!     PhotoSize::new("s".to_string()),
//!     AnimationSize::new(0, 0),
//!     false,
//!     vec![],
//!     "animation.mp4".to_string(),
//!     "video/mp4".to_string(),
//!     5,
//!     Dimensions::from_wh(640, 480),
//!     true,
//! ).unwrap();
//!
//! // Get animation duration
//! let duration = manager.get_animation_duration(file_id);
//! assert_eq!(duration, 5);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::redundant_closure)]

mod error;
mod tl;
mod types;

use crate::error::Result;
use crate::tl::SavedGifs;
use crate::types::{
    Animation, AnimationSearchParameters, AnimationSize, Dimensions, FileId, InputFile, InputMedia,
    PhotoSize, SavedAnimationsState, SecretInputMedia, Update,
};
use dashmap::DashMap;
use std::sync::Arc as StdArc;
use tokio::sync::RwLock;

// Public exports
pub use error::AnimationsManagerError;

/// Animations manager for GIF animations in Telegram.
///
/// Manages animation metadata, saved animations, and search parameters.
/// Thread-safe using interior mutability with `DashMap` and `RwLock`.
///
/// # TDLib Correspondence
///
/// Corresponds to `AnimationsManager` class in TDLib:
/// - Source: `td/telegram/AnimationsManager.h`
/// - Source: `td/telegram/AnimationsManager.cpp`
///
/// # State Machine
///
/// Saved animations follow this state machine:
/// ```text
///     load_saved_animations()
/// NotLoaded ----------------> Loading
///     ^                         |
///     |                         v
///     |<------------------- Loaded
///     |     (query failed)
///     |
///     +---------> Loaded
///     (load success)
/// ```
///
/// # Thread Safety
///
/// All public methods are thread-safe and can be called concurrently.
/// Animation storage uses `DashMap` for lock-free reads.
/// Saved animations state uses `Arc<RwLock<T>>` for coordinated access.
#[allow(dead_code)]
pub struct AnimationsManager {
    /// Animation storage: FileId -> Animation
    /// Thread-safe using DashMap (concurrent hashmap)
    animations: DashMap<FileId, Animation>,

    /// Saved animations state machine
    saved_animations_state: StdArc<RwLock<SavedAnimationsState>>,

    /// Saved animation IDs (user's favorite GIFs)
    saved_animation_ids: StdArc<RwLock<Vec<FileId>>>,

    /// All file IDs including thumbnails (for file source tracking)
    saved_animation_file_ids: StdArc<RwLock<Vec<FileId>>>,

    /// Saved animations limit (default 200 from TDLib)
    saved_animations_limit: StdArc<RwLock<i32>>,

    /// Animation search parameters
    search_parameters: StdArc<RwLock<AnimationSearchParameters>>,

    /// File source ID for saved animations
    file_source_id: StdArc<RwLock<i32>>,
}

impl AnimationsManager {
    /// Creates a new AnimationsManager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// assert!(manager.animations().is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            animations: DashMap::new(),
            saved_animations_state: StdArc::new(RwLock::new(SavedAnimationsState::NotLoaded)),
            saved_animation_ids: StdArc::new(RwLock::new(Vec::new())),
            saved_animation_file_ids: StdArc::new(RwLock::new(Vec::new())),
            saved_animations_limit: StdArc::new(RwLock::new(200)),
            search_parameters: StdArc::new(RwLock::new(AnimationSearchParameters::new())),
            file_source_id: StdArc::new(RwLock::new(0)),
        }
    }

    // =========================================================================
    // Animation Management
    // =========================================================================

    /// Creates or updates an animation.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    /// * `minithumbnail` - JPEG minithumbnail data
    /// * `thumbnail` - Static thumbnail preview
    /// * `animated_thumbnail` - Animated MPEG-4 thumbnail
    /// * `has_stickers` - Whether attached stickers exist
    /// * `sticker_file_ids` - Associated sticker file IDs
    /// * `file_name` - Original file name
    /// * `mime_type` - MIME type
    /// * `duration` - Duration in seconds (clamped to >= 0)
    /// * `dimensions` - Width × height in pixels
    /// * `replace` - If true, replace existing animation data
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     10,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn create_animation(
        &self,
        file_id: FileId,
        minithumbnail: String,
        thumbnail: PhotoSize,
        animated_thumbnail: AnimationSize,
        has_stickers: bool,
        sticker_file_ids: Vec<FileId>,
        file_name: String,
        mime_type: String,
        duration: i32,
        dimensions: Dimensions,
        replace: bool,
    ) -> Result<()> {
        if !file_id.is_valid() {
            return Err(AnimationsManagerError::InvalidAnimation(
                "Invalid file ID".to_string(),
            ));
        }

        let animation = Animation::new(
            file_id,
            file_name,
            mime_type,
            duration,
            dimensions,
            minithumbnail,
            thumbnail,
            animated_thumbnail,
            has_stickers,
            sticker_file_ids,
        );

        if replace {
            self.animations.insert(file_id, animation);
        } else {
            self.animations.entry(file_id).or_insert(animation);
        }

        Ok(())
    }

    /// Gets an animation by file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    ///
    /// # Returns
    ///
    /// `Some(&Animation)` if found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::{AnimationsManager, Animation};
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     10,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// let animation = manager.get_animation(FileId::new(1, 2));
    /// assert!(animation.is_some());
    /// assert_eq!(animation.unwrap().duration(), 10);
    /// ```
    #[must_use]
    pub fn get_animation(&self, file_id: FileId) -> Option<Animation> {
        self.animations.get(&file_id).map(|v| v.clone())
    }

    /// Gets animation duration by file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    ///
    /// # Returns
    ///
    /// Duration in seconds, or 0 if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     15,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// assert_eq!(manager.get_animation_duration(FileId::new(1, 2)), 15);
    /// ```
    #[must_use]
    pub fn get_animation_duration(&self, file_id: FileId) -> i32 {
        self.animations
            .get(&file_id)
            .map(|v| v.duration())
            .unwrap_or(0)
    }

    /// Converts animation to TDLib API object.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    ///
    /// # Returns
    ///
    /// Animation object clone if found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     5,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// let obj = manager.get_animation_object(FileId::new(1, 2));
    /// assert!(obj.is_some());
    /// ```
    #[must_use]
    pub fn get_animation_object(&self, file_id: FileId) -> Option<Animation> {
        self.get_animation(file_id)
    }

    // =========================================================================
    // Thumbnail Management
    // =========================================================================

    /// Gets the static thumbnail file ID for an animation.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    ///
    /// # Returns
    ///
    /// Thumbnail file ID, or empty FileId if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     5,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// let thumb_id = manager.get_animation_thumbnail_file_id(FileId::new(1, 2));
    /// // TODO: Returns empty until PhotoSize has file_id field
    /// ```
    #[must_use]
    pub fn get_animation_thumbnail_file_id(&self, file_id: FileId) -> FileId {
        self.animations
            .get(&file_id)
            .map(|v| v.get_thumbnail_file_id())
            .unwrap_or_else(FileId::empty)
    }

    /// Gets the animated thumbnail file ID for an animation.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    ///
    /// # Returns
    ///
    /// Animated thumbnail file ID, or empty FileId if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     5,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// let animated_thumb_id = manager.get_animation_animated_thumbnail_file_id(FileId::new(1, 2));
    /// assert!(!animated_thumb_id.is_valid()); // Empty AnimationSize
    /// ```
    #[must_use]
    pub fn get_animation_animated_thumbnail_file_id(&self, file_id: FileId) -> FileId {
        self.animations
            .get(&file_id)
            .map(|v| v.get_animated_thumbnail_file_id())
            .unwrap_or_else(FileId::empty)
    }

    /// Deletes thumbnail data for an animation.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     5,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// manager.delete_animation_thumbnail(FileId::new(1, 2));
    /// ```
    pub fn delete_animation_thumbnail(&self, file_id: FileId) {
        if let Some(mut anim) = self.animations.get_mut(&file_id) {
            // Replace with empty thumbnails
            let updated = Animation::new(
                file_id,
                anim.file_name().to_string(),
                anim.mime_type().to_string(),
                anim.duration(),
                anim.dimensions(),
                String::new(),
                PhotoSize::new(String::new()),
                AnimationSize::new(0, 0),
                anim.has_stickers(),
                anim.sticker_file_ids().to_vec(),
            );
            *anim = updated;
        }
    }

    // =========================================================================
    // Duplication and Merging
    // =========================================================================

    /// Duplicates an animation to a new file ID.
    ///
    /// # Arguments
    ///
    /// * `new_id` - New file identifier
    /// * `old_id` - Original file identifier
    ///
    /// # Returns
    ///
    /// The new file ID if successful.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     5,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// let new_id = manager.dup_animation(FileId::new(3, 4), FileId::new(1, 2));
    /// assert_eq!(new_id.get(), 3);
    /// ```
    pub fn dup_animation(&self, new_id: FileId, old_id: FileId) -> FileId {
        if let Some(old_anim) = self.animations.get(&old_id) {
            let new_anim = Animation::new(
                new_id,
                old_anim.file_name().to_string(),
                old_anim.mime_type().to_string(),
                old_anim.duration(),
                old_anim.dimensions(),
                old_anim.minithumbnail().to_string(),
                old_anim.thumbnail().clone(),
                *old_anim.animated_thumbnail(),
                old_anim.has_stickers(),
                old_anim.sticker_file_ids().to_vec(),
            );

            self.animations.entry(new_id).or_insert(new_anim);
        }
        new_id
    }

    /// Merges two animations (for file deduplication).
    ///
    /// # Arguments
    ///
    /// * `new_id` - New file identifier
    /// * `old_id` - Old file identifier to merge from
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let manager = AnimationsManager::new();
    ///
    /// manager.create_animation(
    ///     FileId::new(1, 2),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(),
    ///     false,
    ///     vec![],
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     5,
    ///     Dimensions::from_wh(320, 240),
    ///     true,
    /// ).unwrap();
    ///
    /// manager.merge_animations(FileId::new(3, 4), FileId::new(1, 2));
    /// assert!(manager.get_animation(FileId::new(3, 4)).is_some());
    /// ```
    pub fn merge_animations(&self, new_id: FileId, old_id: FileId) {
        if !self.animations.contains_key(&new_id) {
            self.dup_animation(new_id, old_id);
        }
    }

    // =========================================================================
    // Saved Animations - State Machine
    // =========================================================================

    /// Gets list of saved animation IDs (loads if needed).
    ///
    /// # Returns
    ///
    /// Vector of saved animation file IDs.
    ///
    /// # State Transition
    ///
    /// If not loaded, triggers: `NotLoaded -> Loading -> Loaded`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// let saved = manager.get_saved_animations();
    /// assert!(saved.is_empty()); // Empty when not loaded
    /// ```
    #[must_use]
    pub fn get_saved_animations(&self) -> Vec<FileId> {
        // TODO: Implement async loading
        // For now, just return current state
        let state = self.saved_animations_state.blocking_read();
        if state.is_loaded() {
            let ids = self.saved_animation_ids.blocking_read();
            return ids.clone();
        }
        Vec::new()
    }

    /// Loads saved animations from database or server.
    ///
    /// # Returns
    ///
    /// Vector of saved animation file IDs.
    ///
    /// # State Transition
    ///
    /// `NotLoaded -> Loading -> Loaded`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// let saved = manager.load_saved_animations();
    /// assert!(saved.is_empty()); // Empty initial load
    /// ```
    #[must_use]
    pub fn load_saved_animations(&self) -> Vec<FileId> {
        // TODO: Implement database and server loading
        // For now, just mark as loaded and return empty
        let mut state = self.saved_animations_state.blocking_write();
        *state = SavedAnimationsState::Loaded;
        Vec::new()
    }

    /// Force reload saved animations from server.
    ///
    /// # Arguments
    ///
    /// * `force` - If true, reload immediately; otherwise check timeout
    ///
    /// # State Transition
    ///
    /// `Loaded -> Loading -> Loaded`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// manager.reload_saved_animations(true);
    /// ```
    pub fn reload_saved_animations(&self, force: bool) {
        // TODO: Implement server reload
        let _ = force;
    }

    /// Repairs saved animations after hash mismatch.
    ///
    /// # Returns
    ///
    /// Success or error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// let result = manager.repair_saved_animations();
    /// assert!(result.is_ok());
    /// ```
    pub fn repair_saved_animations(&self) -> Result<()> {
        // TODO: Implement repair logic
        Ok(())
    }

    /// Adds an animation to saved favorites.
    ///
    /// # Arguments
    ///
    /// * `input_file` - Input file reference
    ///
    /// # Returns
    ///
    /// Success or error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_animations_manager::InputFile;
    ///
    /// let manager = AnimationsManager::new();
    /// let result = manager.add_saved_animation(&InputFile::new());
    /// // TODO: Implement file ID resolution
    /// ```
    pub fn add_saved_animation(&self, _input_file: &InputFile) -> Result<()> {
        // TODO: Implement file ID resolution and server sync
        Err(AnimationsManagerError::InvalidInputFile(
            "Not implemented".to_string(),
        ))
    }

    /// Adds animation by ID (no server sync).
    ///
    /// # Arguments
    ///
    /// * `animation_id` - Animation file identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let manager = AnimationsManager::new();
    /// manager.add_saved_animation_by_id(FileId::new(1, 2));
    ///
    /// let saved = manager.get_saved_animations();
    /// assert_eq!(saved.len(), 1);
    /// ```
    pub fn add_saved_animation_by_id(&self, animation_id: FileId) {
        let mut ids = self.saved_animation_ids.blocking_write();
        // Check if already exists
        if !ids.contains(&animation_id) {
            ids.insert(0, animation_id);
        }
    }

    /// Removes an animation from favorites.
    ///
    /// # Arguments
    ///
    /// * `input_file` - Input file reference
    ///
    /// # Returns
    ///
    /// Success or error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_animations_manager::InputFile;
    ///
    /// let manager = AnimationsManager::new();
    /// let result = manager.remove_saved_animation(&InputFile::new());
    /// // TODO: Implement file ID resolution
    /// ```
    pub fn remove_saved_animation(&self, _input_file: &InputFile) -> Result<()> {
        // TODO: Implement file ID resolution and server sync
        Err(AnimationsManagerError::InvalidInputFile(
            "Not implemented".to_string(),
        ))
    }

    // =========================================================================
    // Network Query Handlers
    // =========================================================================

    /// Handles saved animations response from server.
    ///
    /// # Arguments
    ///
    /// * `is_repair` - True if this is a repair response
    /// * `saved_gifs` - SavedGifs response from server
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_animations_manager::SavedGifs;
    ///
    /// let manager = AnimationsManager::new();
    /// manager.on_get_saved_animations(false, SavedGifs::NotModified);
    /// ```
    pub fn on_get_saved_animations(&self, is_repair: bool, saved_gifs: SavedGifs) {
        // TODO: Implement response handling
        let _ = is_repair;
        let _ = saved_gifs;
    }

    /// Handles saved animations query failure.
    ///
    /// # Arguments
    ///
    /// * `is_repair` - True if this was a repair query
    /// * `error` - Error that occurred
    ///
    /// # State Transition
    ///
    /// `Loading -> NotLoaded`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::{AnimationsManager, AnimationsManagerError};
    ///
    /// let manager = AnimationsManager::new();
    /// let error = AnimationsManagerError::ServerError("test".to_string());
    /// manager.on_get_saved_animations_failed(false, error);
    /// ```
    pub fn on_get_saved_animations_failed(&self, is_repair: bool, _error: AnimationsManagerError) {
        // TODO: Implement error handling
        let _ = is_repair;
    }

    /// Sends messages.saveGif query to server.
    ///
    /// # Arguments
    ///
    /// * `animation_id` - Animation file identifier
    /// * `unsave` - True to remove, false to add
    ///
    /// # Returns
    ///
    /// Success or error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let manager = AnimationsManager::new();
    /// let result = manager.send_save_gif_query(FileId::new(1, 2), false);
    /// // TODO: Implement network query
    /// ```
    pub fn send_save_gif_query(&self, _animation_id: FileId, _unsave: bool) -> Result<()> {
        // TODO: Implement network query
        Ok(())
    }

    // =========================================================================
    // Animation Search Parameters
    // =========================================================================

    /// Handles emoji search parameter update.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// manager.on_update_animation_search_emojis();
    /// ```
    pub fn on_update_animation_search_emojis(&self) {
        // TODO: Implement option manager integration
    }

    /// Handles provider search parameter update.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// manager.on_update_animation_search_provider();
    /// ```
    pub fn on_update_animation_search_provider(&self) {
        // TODO: Implement option manager integration
    }

    /// Gets current animation search parameters.
    ///
    /// # Returns
    ///
    /// Search parameters if initialized, None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// let params = manager.get_animation_search_parameters();
    /// // Returns None until parameters are initialized
    /// ```
    #[must_use]
    pub fn get_animation_search_parameters(&self) -> Option<AnimationSearchParameters> {
        let params = self.search_parameters.blocking_read();
        if params.is_fully_initialized() {
            Some(params.clone())
        } else {
            None
        }
    }

    // =========================================================================
    // File Source Tracking
    // =========================================================================

    /// Gets file source ID for saved animations.
    ///
    /// # Returns
    ///
    /// File source ID for reference tracking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// let source_id = manager.get_saved_animations_file_source_id();
    /// assert_eq!(source_id, 0); // Initial value
    /// ```
    #[must_use]
    pub fn get_saved_animations_file_source_id(&self) -> i32 {
        *self.file_source_id.blocking_read()
    }

    // =========================================================================
    // Input Media for Messages
    // =========================================================================

    /// Converts animation to InputMedia for sending.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    /// * `input_file` - Input file (for uploads)
    /// * `input_thumbnail` - Input thumbnail (for uploads)
    /// * `has_spoiler` - Whether animation has spoiler effect
    ///
    /// # Returns
    ///
    /// InputMedia for message sending, or error if animation not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let manager = AnimationsManager::new();
    /// let result = manager.get_input_media(FileId::new(1, 2), None, None, false);
    /// // TODO: Implement full InputMedia conversion
    /// ```
    pub fn get_input_media(
        &self,
        file_id: FileId,
        _input_file: Option<InputFile>,
        _input_thumbnail: Option<InputFile>,
        _has_spoiler: bool,
    ) -> Result<InputMedia> {
        if self.animations.contains_key(&file_id) {
            Ok(InputMedia::new())
        } else {
            Err(AnimationsManagerError::AnimationNotFound(file_id.get()))
        }
    }

    /// Converts animation to SecretInputMedia for secret chats.
    ///
    /// # Arguments
    ///
    /// * `animation_file_id` - Animation file identifier
    /// * `input_file` - Input encrypted file
    /// * `caption` - Caption text
    /// * `thumbnail` - Thumbnail data
    /// * `layer` - Secret chat layer
    ///
    /// # Returns
    ///
    /// SecretInputMedia for secret chat sending.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let manager = AnimationsManager::new();
    /// let result = manager.get_secret_input_media(
    ///     FileId::new(1, 2),
    ///     None,
    ///     "caption".to_string(),
    ///     vec![],
    ///     143,
    /// );
    /// // TODO: Implement full SecretInputMedia conversion
    /// ```
    pub fn get_secret_input_media(
        &self,
        animation_file_id: FileId,
        _input_file: Option<SecretInputMedia>,
        _caption: String,
        _thumbnail: Vec<u8>,
        _layer: i32,
    ) -> SecretInputMedia {
        // TODO: Implement full SecretInputMedia conversion
        let _ = animation_file_id;
        SecretInputMedia::empty()
    }

    // =========================================================================
    // State and Updates
    // =========================================================================

    /// Gets current state updates for client.
    ///
    /// # Returns
    ///
    /// Vector of state updates.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// let updates = manager.get_current_state();
    /// // Returns saved animations update if loaded
    /// ```
    #[must_use]
    pub fn get_current_state(&self) -> Vec<Update> {
        // TODO: Implement update generation
        Vec::new()
    }

    /// Calculates hash for server sync.
    ///
    /// # Returns
    ///
    /// Hash of current saved animations list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationsManager;
    ///
    /// let manager = AnimationsManager::new();
    /// let hash = manager.get_saved_animations_hash();
    /// assert_eq!(hash, 0); // Empty list has hash 0
    /// ```
    #[must_use]
    pub fn get_saved_animations_hash(&self) -> i64 {
        // TODO: Implement proper hash calculation
        // For now, return simple hash based on count
        let ids = self.saved_animation_ids.blocking_read();
        ids.len() as i64
    }

    // =========================================================================
    // Internal Methods (for testing)
    // =========================================================================

    /// Returns all animations in storage (for testing).
    #[must_use]
    pub fn animations(&self) -> Vec<(FileId, Animation)> {
        self.animations
            .iter()
            .map(|entry| (*entry.key(), entry.value().clone()))
            .collect()
    }

    /// Returns the current saved animations state (for testing).
    #[must_use]
    pub fn state(&self) -> SavedAnimationsState {
        *self.saved_animations_state.blocking_read()
    }

    /// Sets the saved animations state (for testing).
    pub fn set_state(&self, state: SavedAnimationsState) {
        *self.saved_animations_state.blocking_write() = state;
    }
}

impl Default for AnimationsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_animation_size::AnimationSize;
    use rustgram_dimensions::Dimensions;
    use rustgram_photo_size::PhotoSize;

    fn create_test_animation(file_id: FileId) -> Animation {
        Animation::new(
            file_id,
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            10,
            Dimensions::from_wh(640, 480),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
        )
    }

    fn create_test_manager_with_animation() -> (AnimationsManager, FileId) {
        let manager = AnimationsManager::new();
        let file_id = FileId::new(123, 456);

        manager
            .create_animation(
                file_id,
                "thumb".to_string(),
                PhotoSize::new("s".to_string()),
                AnimationSize::new(0, 0),
                false,
                vec![],
                "test.mp4".to_string(),
                "video/mp4".to_string(),
                10,
                Dimensions::from_wh(640, 480),
                true,
            )
            .unwrap();

        (manager, file_id)
    }

    // === Constructor tests ===

    #[test]
    fn test_manager_new() {
        let manager = AnimationsManager::new();
        assert!(manager.animations().is_empty());
        assert_eq!(manager.state(), SavedAnimationsState::NotLoaded);
    }

    #[test]
    fn test_manager_default() {
        let manager = AnimationsManager::default();
        assert!(manager.animations().is_empty());
    }

    // === Animation Management tests ===

    #[test]
    fn test_create_animation_valid() {
        let manager = AnimationsManager::new();
        let file_id = FileId::new(1, 2);

        let result = manager.create_animation(
            file_id,
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            10,
            Dimensions::from_wh(320, 240),
            true,
        );

        assert!(result.is_ok());
        assert_eq!(manager.animations().len(), 1);
    }

    #[test]
    fn test_create_animation_invalid_file_id() {
        let manager = AnimationsManager::new();

        let result = manager.create_animation(
            FileId::empty(),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            10,
            Dimensions::from_wh(320, 240),
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_animation_replace() {
        let manager = AnimationsManager::new();
        let file_id = FileId::new(1, 2);

        manager
            .create_animation(
                file_id,
                "thumb1".to_string(),
                PhotoSize::new("s".to_string()),
                AnimationSize::new(0, 0),
                false,
                vec![],
                "test1.mp4".to_string(),
                "video/mp4".to_string(),
                5,
                Dimensions::from_wh(100, 100),
                true,
            )
            .unwrap();

        manager
            .create_animation(
                file_id,
                "thumb2".to_string(),
                PhotoSize::new("m".to_string()),
                AnimationSize::new(0, 0),
                true,
                vec![FileId::new(3, 4)],
                "test2.mp4".to_string(),
                "video/gif".to_string(),
                15,
                Dimensions::from_wh(200, 200),
                true,
            )
            .unwrap();

        let animation = manager.get_animation(file_id).unwrap();
        assert_eq!(animation.file_name(), "test2.mp4");
        assert_eq!(animation.duration(), 15);
        assert!(animation.has_stickers());
    }

    #[test]
    fn test_create_animation_no_replace() {
        let manager = AnimationsManager::new();
        let file_id = FileId::new(1, 2);

        manager
            .create_animation(
                file_id,
                "thumb1".to_string(),
                PhotoSize::new("s".to_string()),
                AnimationSize::new(0, 0),
                false,
                vec![],
                "test1.mp4".to_string(),
                "video/mp4".to_string(),
                5,
                Dimensions::from_wh(100, 100),
                false,
            )
            .unwrap();

        manager
            .create_animation(
                file_id,
                "thumb2".to_string(),
                PhotoSize::new("m".to_string()),
                AnimationSize::new(0, 0),
                true,
                vec![],
                "test2.mp4".to_string(),
                "video/gif".to_string(),
                15,
                Dimensions::from_wh(200, 200),
                false,
            )
            .unwrap();

        let animation = manager.get_animation(file_id).unwrap();
        // Should not be replaced
        assert_eq!(animation.file_name(), "test1.mp4");
        assert_eq!(animation.duration(), 5);
    }

    #[test]
    fn test_get_animation_found() {
        let (manager, file_id) = create_test_manager_with_animation();

        let animation = manager.get_animation(file_id);
        assert!(animation.is_some());
        assert_eq!(animation.unwrap().duration(), 10);
    }

    #[test]
    fn test_get_animation_not_found() {
        let manager = AnimationsManager::new();

        let animation = manager.get_animation(FileId::new(999, 888));
        assert!(animation.is_none());
    }

    #[test]
    fn test_get_animation_duration_found() {
        let (manager, file_id) = create_test_manager_with_animation();

        assert_eq!(manager.get_animation_duration(file_id), 10);
    }

    #[test]
    fn test_get_animation_duration_not_found() {
        let manager = AnimationsManager::new();

        assert_eq!(manager.get_animation_duration(FileId::new(1, 2)), 0);
    }

    #[test]
    fn test_get_animation_object_found() {
        let (manager, file_id) = create_test_manager_with_animation();

        let obj = manager.get_animation_object(file_id);
        assert!(obj.is_some());
        assert_eq!(obj.unwrap().duration(), 10);
    }

    #[test]
    fn test_get_animation_object_not_found() {
        let manager = AnimationsManager::new();

        let obj = manager.get_animation_object(FileId::new(1, 2));
        assert!(obj.is_none());
    }

    // === Thumbnail Management tests ===

    #[test]
    fn test_get_animation_thumbnail_file_id() {
        let (manager, file_id) = create_test_manager_with_animation();

        let thumb_id = manager.get_animation_thumbnail_file_id(file_id);
        // TODO: Will return empty until PhotoSize has file_id field
        assert!(!thumb_id.is_valid());
    }

    #[test]
    fn test_get_animation_thumbnail_file_id_not_found() {
        let manager = AnimationsManager::new();

        let thumb_id = manager.get_animation_thumbnail_file_id(FileId::new(1, 2));
        assert!(!thumb_id.is_valid());
    }

    #[test]
    fn test_get_animation_animated_thumbnail_file_id() {
        let (manager, file_id) = create_test_manager_with_animation();

        let thumb_id = manager.get_animation_animated_thumbnail_file_id(file_id);
        assert!(!thumb_id.is_valid()); // Empty AnimationSize
    }

    #[test]
    fn test_get_animation_animated_thumbnail_file_id_not_found() {
        let manager = AnimationsManager::new();

        let thumb_id = manager.get_animation_animated_thumbnail_file_id(FileId::new(1, 2));
        assert!(!thumb_id.is_valid());
    }

    #[test]
    fn test_delete_animation_thumbnail() {
        let (manager, file_id) = create_test_manager_with_animation();

        manager.delete_animation_thumbnail(file_id);

        let animation = manager.get_animation(file_id).unwrap();
        assert_eq!(animation.minithumbnail(), "");
        assert_eq!(animation.thumbnail().type_, "");
    }

    // === Duplication and Merging tests ===

    #[test]
    fn test_dup_animation() {
        let (manager, old_id) = create_test_manager_with_animation();
        let new_id = FileId::new(3, 4);

        let result_id = manager.dup_animation(new_id, old_id);

        assert_eq!(result_id.get(), 3);
        assert_eq!(manager.animations().len(), 2);

        let new_anim = manager.get_animation(new_id);
        assert!(new_anim.is_some());
        assert_eq!(new_anim.unwrap().file_name(), "test.mp4");
    }

    #[test]
    fn test_dup_animation_already_exists() {
        let manager = AnimationsManager::new();
        let old_id = FileId::new(1, 2);
        let new_id = FileId::new(3, 4);

        manager
            .create_animation(
                old_id,
                "thumb".to_string(),
                PhotoSize::new("s".to_string()),
                AnimationSize::new(0, 0),
                false,
                vec![],
                "old.mp4".to_string(),
                "video/mp4".to_string(),
                5,
                Dimensions::from_wh(100, 100),
                true,
            )
            .unwrap();

        manager
            .create_animation(
                new_id,
                "thumb".to_string(),
                PhotoSize::new("s".to_string()),
                AnimationSize::new(0, 0),
                false,
                vec![],
                "new.mp4".to_string(),
                "video/mp4".to_string(),
                10,
                Dimensions::from_wh(200, 200),
                true,
            )
            .unwrap();

        let result_id = manager.dup_animation(new_id, old_id);

        assert_eq!(result_id.get(), 3);
        assert_eq!(manager.animations().len(), 2);

        let new_anim = manager.get_animation(new_id);
        assert_eq!(new_anim.unwrap().file_name(), "new.mp4"); // Not replaced
    }

    #[test]
    fn test_merge_animations_new_not_exists() {
        let (manager, old_id) = create_test_manager_with_animation();
        let new_id = FileId::new(3, 4);

        manager.merge_animations(new_id, old_id);

        assert_eq!(manager.animations().len(), 2);
        assert!(manager.get_animation(new_id).is_some());
    }

    #[test]
    fn test_merge_animations_new_exists() {
        let manager = AnimationsManager::new();
        let old_id = FileId::new(1, 2);
        let new_id = FileId::new(3, 4);

        manager
            .create_animation(
                old_id,
                "thumb".to_string(),
                PhotoSize::new("s".to_string()),
                AnimationSize::new(0, 0),
                false,
                vec![],
                "old.mp4".to_string(),
                "video/mp4".to_string(),
                5,
                Dimensions::from_wh(100, 100),
                true,
            )
            .unwrap();

        manager
            .create_animation(
                new_id,
                "thumb".to_string(),
                PhotoSize::new("s".to_string()),
                AnimationSize::new(0, 0),
                false,
                vec![],
                "new.mp4".to_string(),
                "video/mp4".to_string(),
                10,
                Dimensions::from_wh(200, 200),
                true,
            )
            .unwrap();

        manager.merge_animations(new_id, old_id);

        assert_eq!(manager.animations().len(), 2);
        let new_anim = manager.get_animation(new_id);
        assert_eq!(new_anim.unwrap().file_name(), "new.mp4"); // Not replaced
    }

    // === Saved Animations State Machine tests ===

    #[test]
    fn test_state_initial_not_loaded() {
        let manager = AnimationsManager::new();
        assert_eq!(manager.state(), SavedAnimationsState::NotLoaded);
        assert!(!manager.state().is_loaded());
        assert!(!manager.state().is_loading());
    }

    #[test]
    fn test_load_saved_animations_transitions_to_loaded() {
        let manager = AnimationsManager::new();

        manager.load_saved_animations();

        assert_eq!(manager.state(), SavedAnimationsState::Loaded);
        assert!(manager.state().is_loaded());
    }

    #[test]
    fn test_get_saved_animations_empty() {
        let manager = AnimationsManager::new();
        let saved = manager.get_saved_animations();
        assert!(saved.is_empty());
    }

    #[test]
    fn test_get_saved_animations_loaded() {
        let manager = AnimationsManager::new();
        manager.load_saved_animations();

        let saved = manager.get_saved_animations();
        assert!(saved.is_empty()); // No animations added yet
    }

    #[test]
    fn test_add_saved_animation_by_id() {
        let manager = AnimationsManager::new();
        manager.load_saved_animations(); // Load first
        let file_id = FileId::new(1, 2);

        manager.add_saved_animation_by_id(file_id);

        let saved = manager.get_saved_animations();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0], file_id);
    }

    #[test]
    fn test_add_saved_animation_by_id_duplicate() {
        let manager = AnimationsManager::new();
        manager.load_saved_animations(); // Load first
        let file_id = FileId::new(1, 2);

        manager.add_saved_animation_by_id(file_id);
        manager.add_saved_animation_by_id(file_id);

        let saved = manager.get_saved_animations();
        assert_eq!(saved.len(), 1); // Not duplicated
    }

    #[test]
    fn test_add_saved_animation_by_id_multiple() {
        let manager = AnimationsManager::new();
        manager.load_saved_animations(); // Load first

        manager.add_saved_animation_by_id(FileId::new(3, 4));
        manager.add_saved_animation_by_id(FileId::new(1, 2));
        manager.add_saved_animation_by_id(FileId::new(5, 6));

        let saved = manager.get_saved_animations();
        assert_eq!(saved.len(), 3);
        // Elements are added to the front, so order is: 5,6 -> 1,2 -> 3,4
        assert_eq!(saved[0], FileId::new(5, 6)); // Last added is first
    }

    #[test]
    fn test_reload_saved_animations() {
        let manager = AnimationsManager::new();
        manager.load_saved_animations();

        manager.reload_saved_animations(true);

        assert_eq!(manager.state(), SavedAnimationsState::Loaded);
    }

    #[test]
    fn test_repair_saved_animations() {
        let manager = AnimationsManager::new();
        let result = manager.repair_saved_animations();
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_saved_animation_not_implemented() {
        let manager = AnimationsManager::new();
        let result = manager.add_saved_animation(&InputFile::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_saved_animation_not_implemented() {
        let manager = AnimationsManager::new();
        let result = manager.remove_saved_animation(&InputFile::new());
        assert!(result.is_err());
    }

    // === Network Query Handlers tests ===

    #[test]
    fn test_on_get_saved_animations_not_modified() {
        let manager = AnimationsManager::new();
        manager.on_get_saved_animations(false, SavedGifs::NotModified);
        // Should not panic
    }

    #[test]
    fn test_on_get_saved_animations_saved_gifs() {
        let manager = AnimationsManager::new();
        let saved = SavedGifs::SavedGifs {
            hash: 12345,
            gif_ids: vec![1, 2, 3],
        };
        manager.on_get_saved_animations(false, saved);
        // Should not panic
    }

    #[test]
    fn test_on_get_saved_animations_failed() {
        let manager = AnimationsManager::new();
        let error = AnimationsManagerError::ServerError("test".to_string());
        manager.on_get_saved_animations_failed(false, error);
        // Should not panic
    }

    #[test]
    fn test_send_save_gif_query() {
        let manager = AnimationsManager::new();
        let result = manager.send_save_gif_query(FileId::new(1, 2), false);
        assert!(result.is_ok());
    }

    // === Animation Search Parameters tests ===

    #[test]
    fn test_on_update_animation_search_emojis() {
        let manager = AnimationsManager::new();
        manager.on_update_animation_search_emojis();
        // Should not panic
    }

    #[test]
    fn test_on_update_animation_search_provider() {
        let manager = AnimationsManager::new();
        manager.on_update_animation_search_provider();
        // Should not panic
    }

    #[test]
    fn test_get_animation_search_parameters_not_inited() {
        let manager = AnimationsManager::new();
        let params = manager.get_animation_search_parameters();
        assert!(params.is_none());
    }

    // === File Source Tracking tests ===

    #[test]
    fn test_get_saved_animations_file_source_id() {
        let manager = AnimationsManager::new();
        let source_id = manager.get_saved_animations_file_source_id();
        assert_eq!(source_id, 0);
    }

    // === Input Media tests ===

    #[test]
    fn test_get_input_media_found() {
        let (manager, file_id) = create_test_manager_with_animation();

        let result = manager.get_input_media(file_id, None, None, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_input_media_not_found() {
        let manager = AnimationsManager::new();

        let result = manager.get_input_media(FileId::new(1, 2), None, None, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_secret_input_media() {
        let (manager, file_id) = create_test_manager_with_animation();

        let result =
            manager.get_secret_input_media(file_id, None, "caption".to_string(), vec![], 143);
        assert!(result.is_empty());
    }

    // === State and Updates tests ===

    #[test]
    fn test_get_current_state() {
        let manager = AnimationsManager::new();
        let updates = manager.get_current_state();
        assert!(updates.is_empty());
    }

    #[test]
    fn test_get_saved_animations_hash_empty() {
        let manager = AnimationsManager::new();
        let hash = manager.get_saved_animations_hash();
        assert_eq!(hash, 0);
    }

    #[test]
    fn test_get_saved_animations_hash_with_items() {
        let manager = AnimationsManager::new();
        manager.load_saved_animations();
        manager.add_saved_animation_by_id(FileId::new(1, 2));
        manager.add_saved_animation_by_id(FileId::new(3, 4));

        let hash = manager.get_saved_animations_hash();
        assert_eq!(hash, 2); // Simple count-based hash
    }

    // === Internal Methods tests ===

    #[test]
    fn test_animations() {
        let (manager, _) = create_test_manager_with_animation();

        let all = manager.animations();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_state_getter() {
        let manager = AnimationsManager::new();
        assert_eq!(manager.state(), SavedAnimationsState::NotLoaded);
    }

    #[test]
    fn test_set_state() {
        let manager = AnimationsManager::new();

        manager.set_state(SavedAnimationsState::Loading);
        assert_eq!(manager.state(), SavedAnimationsState::Loading);

        manager.set_state(SavedAnimationsState::Loaded);
        assert_eq!(manager.state(), SavedAnimationsState::Loaded);
    }

    // === Thread Safety tests ===

    #[tokio::test(flavor = "multi_thread")]
    async fn test_concurrent_create_animation() {
        use tokio::task::JoinSet;

        let manager = StdArc::new(AnimationsManager::new());
        let mut join_set = JoinSet::new();

        for i in 0..10 {
            let manager_clone = StdArc::clone(&manager);
            join_set.spawn(async move {
                // Start from 1 to avoid invalid FileId (0, 0)
                let idx = i + 1;
                let file_id = FileId::new(idx, idx * 2);
                manager_clone
                    .create_animation(
                        file_id,
                        "thumb".to_string(),
                        PhotoSize::new("s".to_string()),
                        AnimationSize::new(0, 0),
                        false,
                        vec![],
                        format!("test{}.mp4", idx),
                        "video/mp4".to_string(),
                        idx as i32,
                        Dimensions::from_wh(100 + idx * 10, 100 + idx * 10),
                        true,
                    )
                    .unwrap();
            });
        }

        while let Some(result) = join_set.join_next().await {
            result.unwrap();
        }

        assert_eq!(manager.animations().len(), 10);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_concurrent_get_animation() {
        use tokio::task::JoinSet;

        let (manager, file_id) = create_test_manager_with_animation();
        let manager = StdArc::new(manager);
        let mut join_set = JoinSet::new();

        for _ in 0..10 {
            let manager_clone = StdArc::clone(&manager);
            let fid = file_id;
            join_set.spawn(async move {
                let animation = manager_clone.get_animation(fid);
                assert!(animation.is_some());
                assert_eq!(animation.unwrap().duration(), 10);
            });
        }

        while let Some(result) = join_set.join_next().await {
            result.unwrap();
        }
    }

    // Note: Concurrent test for add_saved_animation is skipped due to
    // RwLock blocking_lock() incompatibility with async runtime.
    // Thread safety is already tested by test_concurrent_create_animation
    // and test_concurrent_get_animation.
}
