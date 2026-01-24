//! # Rustgram VideosManager
//!
//! Videos manager for Telegram MTProto client.
//!
//! ## Overview
//!
//! This module provides functionality for managing regular videos in Telegram.
//! Unlike video notes (round videos), regular videos can have arbitrary dimensions
//! and support additional features like animated thumbnails and stickers.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_videos_manager::{VideosManager, Video};
//! use rustgram_file_id::FileId;
//! use rustgram_dimensions::Dimensions;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let manager = VideosManager::new();
//! let video = Video::new(
//!     FileId::new(1, 0),
//!     30,
//!     Dimensions::from_wh(1920, 1080),
//!     "video/mp4"
//! );
//! manager.add_video(video).await;
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

use rustgram_animation_size::AnimationSize;
use rustgram_dimensions::Dimensions;
use rustgram_file_id::FileId;
use rustgram_photo_size::PhotoSize;

/// Video metadata.
///
/// Represents a regular video in Telegram with all associated metadata
/// including dimensions, duration, thumbnails, and various flags.
#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    file_id: FileId,
    duration: i32,
    dimensions: Dimensions,
    file_name: String,
    mime_type: String,
    minithumbnail: String,
    thumbnail: PhotoSize,
    animated_thumbnail: AnimationSize,
    supports_streaming: bool,
    is_animation: bool,
    has_stickers: bool,
    sticker_file_ids: Vec<FileId>,
    preload_prefix_size: i32,
    start_ts: f64,
    codec: String,
}

impl Video {
    /// Creates a new video with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique file identifier
    /// * `duration` - Video duration in seconds
    /// * `dimensions` - Video dimensions (width x height)
    /// * `mime_type` - MIME type (e.g., "video/mp4")
    #[inline]
    #[must_use]
    pub fn new(file_id: FileId, duration: i32, dimensions: Dimensions, mime_type: &str) -> Self {
        Self {
            file_id,
            duration,
            dimensions,
            file_name: String::new(),
            mime_type: String::from(mime_type),
            minithumbnail: String::new(),
            thumbnail: PhotoSize::new(String::new()),
            animated_thumbnail: AnimationSize::new(),
            supports_streaming: false,
            is_animation: false,
            has_stickers: false,
            sticker_file_ids: Vec::new(),
            preload_prefix_size: 0,
            start_ts: 0.0,
            codec: String::new(),
        }
    }

    /// Returns the file ID.
    #[inline]
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the duration in seconds.
    #[inline]
    #[must_use]
    pub const fn duration(&self) -> i32 {
        self.duration
    }

    /// Returns the video dimensions.
    #[inline]
    #[must_use]
    pub const fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    /// Returns the file name.
    #[inline]
    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns the MIME type.
    #[inline]
    #[must_use]
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    /// Returns the minithumbnail (blurhash).
    #[inline]
    #[must_use]
    pub fn minithumbnail(&self) -> &str {
        &self.minithumbnail
    }

    /// Returns the thumbnail.
    #[inline]
    #[must_use]
    pub const fn thumbnail(&self) -> &PhotoSize {
        &self.thumbnail
    }

    /// Returns the animated thumbnail.
    #[inline]
    #[must_use]
    pub const fn animated_thumbnail(&self) -> &AnimationSize {
        &self.animated_thumbnail
    }

    /// Returns whether the video supports streaming.
    #[inline]
    #[must_use]
    pub const fn supports_streaming(&self) -> bool {
        self.supports_streaming
    }

    /// Returns whether this is an animation (GIF).
    #[inline]
    #[must_use]
    pub const fn is_animation(&self) -> bool {
        self.is_animation
    }

    /// Returns whether the video has associated stickers.
    #[inline]
    #[must_use]
    pub const fn has_stickers(&self) -> bool {
        self.has_stickers
    }

    /// Returns the sticker file IDs associated with this video.
    #[inline]
    #[must_use]
    pub fn sticker_file_ids(&self) -> &[FileId] {
        &self.sticker_file_ids
    }

    /// Returns the preload prefix size in bytes.
    #[inline]
    #[must_use]
    pub const fn preload_prefix_size(&self) -> i32 {
        self.preload_prefix_size
    }

    /// Returns the start timestamp in seconds.
    #[inline]
    #[must_use]
    pub const fn start_ts(&self) -> f64 {
        self.start_ts
    }

    /// Returns the video codec (e.g., "h264").
    #[inline]
    #[must_use]
    pub fn codec(&self) -> &str {
        &self.codec
    }

    /// Sets the file name.
    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }

    /// Sets the MIME type.
    pub fn set_mime_type(&mut self, mime_type: String) {
        self.mime_type = mime_type;
    }

    /// Sets the minithumbnail (blurhash).
    pub fn set_minithumbnail(&mut self, minithumbnail: String) {
        self.minithumbnail = minithumbnail;
    }

    /// Sets the thumbnail.
    pub fn set_thumbnail(&mut self, thumbnail: PhotoSize) {
        self.thumbnail = thumbnail;
    }

    /// Sets the animated thumbnail.
    pub fn set_animated_thumbnail(&mut self, animated_thumbnail: AnimationSize) {
        self.animated_thumbnail = animated_thumbnail;
    }

    /// Sets whether the video supports streaming.
    pub fn set_supports_streaming(&mut self, supports_streaming: bool) {
        self.supports_streaming = supports_streaming;
    }

    /// Sets whether this is an animation (GIF).
    pub fn set_animation(&mut self, is_animation: bool) {
        self.is_animation = is_animation;
    }

    /// Sets the sticker file IDs.
    pub fn set_sticker_file_ids(&mut self, sticker_file_ids: Vec<FileId>) {
        self.has_stickers = !sticker_file_ids.is_empty();
        self.sticker_file_ids = sticker_file_ids;
    }

    /// Sets the preload prefix size in bytes.
    pub fn set_preload_prefix_size(&mut self, size: i32) {
        self.preload_prefix_size = size;
    }

    /// Sets the start timestamp in seconds.
    pub fn set_start_ts(&mut self, start_ts: f64) {
        self.start_ts = start_ts;
    }

    /// Sets the video codec.
    pub fn set_codec(&mut self, codec: String) {
        self.codec = codec;
    }

    /// Returns whether this video is valid.
    ///
    /// A valid video has:
    /// - Positive duration
    /// - Non-empty file ID
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.duration > 0 && !self.file_id.is_empty()
    }

    /// Returns the search text for this video.
    ///
    /// Used for searching videos by file name or other metadata.
    #[must_use]
    pub fn search_text(&self) -> String {
        if self.file_name.is_empty() {
            String::new()
        } else {
            self.file_name.to_lowercase()
        }
    }
}

/// Videos manager.
///
/// Provides thread-safe storage and retrieval of videos.
/// Uses `Arc<RwLock<T>>` for concurrent access.
///
/// # Example
///
/// ```rust
/// use rustgram_videos_manager::VideosManager;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = VideosManager::new();
/// assert_eq!(manager.video_count().await, 0);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct VideosManager {
    videos: Arc<RwLock<HashMap<FileId, Video>>>,
}

impl Default for VideosManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VideosManager {
    /// Creates a new empty videos manager.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            videos: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds a video to the manager.
    ///
    /// Returns `true` if the video was added (didn't previously exist),
    /// `false` if a video with this file ID already existed.
    pub async fn add_video(&self, video: Video) -> bool {
        let file_id = video.file_id();
        let mut videos = self.videos.write().await;
        videos.insert(file_id, video).is_none()
    }

    /// Gets a video by file ID.
    ///
    /// Returns `None` if the video doesn't exist.
    pub async fn get_video(&self, file_id: FileId) -> Option<Video> {
        let videos = self.videos.read().await;
        videos.get(&file_id).cloned()
    }

    /// Removes a video by file ID.
    ///
    /// Returns the removed video if it existed, `None` otherwise.
    pub async fn remove_video(&self, file_id: FileId) -> Option<Video> {
        let mut videos = self.videos.write().await;
        videos.remove(&file_id)
    }

    /// Returns the number of videos stored.
    pub async fn video_count(&self) -> usize {
        let videos = self.videos.read().await;
        videos.len()
    }

    /// Returns whether a video with the given file ID exists.
    pub async fn has_video(&self, file_id: FileId) -> bool {
        let videos = self.videos.read().await;
        videos.contains_key(&file_id)
    }

    /// Gets the duration of a video.
    ///
    /// Returns `0` if the video doesn't exist.
    pub async fn get_duration(&self, file_id: FileId) -> i32 {
        let videos = self.videos.read().await;
        videos.get(&file_id).map(|v| v.duration()).unwrap_or(0)
    }

    /// Gets the MIME type of a video.
    ///
    /// Returns an empty string if the video doesn't exist.
    pub async fn get_mime_type(&self, file_id: FileId) -> String {
        let videos = self.videos.read().await;
        videos
            .get(&file_id)
            .map(|v| v.mime_type().to_string())
            .unwrap_or_default()
    }

    /// Gets the search text for a video.
    ///
    /// Returns an empty string if the video doesn't exist.
    pub async fn get_search_text(&self, file_id: FileId) -> String {
        let videos = self.videos.read().await;
        videos
            .get(&file_id)
            .map(|v| v.search_text())
            .unwrap_or_default()
    }

    /// Gets the thumbnail file ID for a video.
    ///
    /// Returns an empty file ID if the video or thumbnail doesn't exist.
    pub async fn get_thumbnail_file_id(&self, file_id: FileId) -> FileId {
        let videos = self.videos.read().await;
        videos
            .get(&file_id)
            .map(|_| FileId::empty())
            .unwrap_or_default()
    }

    /// Gets the animated thumbnail file ID for a video.
    ///
    /// Returns an empty file ID if the video or animated thumbnail doesn't exist.
    pub async fn get_animated_thumbnail_file_id(&self, file_id: FileId) -> FileId {
        let videos = self.videos.read().await;
        videos
            .get(&file_id)
            .map(|v| v.animated_thumbnail().file_id())
            .unwrap_or_default()
    }

    /// Deletes the thumbnail from a video.
    pub async fn delete_thumbnail(&self, file_id: FileId) {
        let mut videos = self.videos.write().await;
        if let Some(video) = videos.get_mut(&file_id) {
            video.set_thumbnail(PhotoSize::new(String::new()));
        }
    }

    /// Clears all videos from storage.
    pub async fn clear(&self) {
        let mut videos = self.videos.write().await;
        videos.clear();
    }

    /// Returns all file IDs currently stored.
    pub async fn all_file_ids(&self) -> Vec<FileId> {
        let videos = self.videos.read().await;
        videos.keys().copied().collect()
    }
}

impl fmt::Display for VideosManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VideosManager")
    }
}

/// Crate version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name for identification.
pub const CRATE_NAME: &str = "rustgram_videos_manager";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_new() {
        let video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        );
        assert_eq!(video.duration(), 30);
        assert!(video.is_valid());
        assert_eq!(video.mime_type(), "video/mp4");
        assert!(!video.supports_streaming());
    }

    #[test]
    fn test_video_with_stickers() {
        let mut video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        );
        assert!(!video.has_stickers());

        video.set_sticker_file_ids(vec![FileId::new(10, 0), FileId::new(11, 0)]);
        assert!(video.has_stickers());
        assert_eq!(video.sticker_file_ids().len(), 2);
    }

    #[test]
    fn test_video_search_text() {
        let mut video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        );
        assert!(video.search_text().is_empty());

        video.set_file_name("My_Video.MP4".to_string());
        assert_eq!(video.search_text(), "my_video.mp4");
    }

    #[tokio::test]
    async fn test_manager_add_get() {
        let mgr = VideosManager::new();
        let video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        );
        mgr.add_video(video).await;
        assert_eq!(mgr.video_count().await, 1);
        assert!(mgr.has_video(FileId::new(1, 0)).await);
    }

    #[tokio::test]
    async fn test_manager_remove() {
        let mgr = VideosManager::new();
        let video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        );
        mgr.add_video(video).await;
        assert_eq!(mgr.video_count().await, 1);

        let removed = mgr.remove_video(FileId::new(1, 0)).await;
        assert!(removed.is_some());
        assert_eq!(mgr.video_count().await, 0);
    }

    #[tokio::test]
    async fn test_get_duration() {
        let mgr = VideosManager::new();
        let video = Video::new(
            FileId::new(1, 0),
            60,
            Dimensions::from_wh(1280, 720),
            "video/mp4",
        );
        mgr.add_video(video).await;
        assert_eq!(mgr.get_duration(FileId::new(1, 0)).await, 60);
        assert_eq!(mgr.get_duration(FileId::new(99, 0)).await, 0);
    }

    #[tokio::test]
    async fn test_get_mime_type() {
        let mgr = VideosManager::new();
        let video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/webm",
        );
        mgr.add_video(video).await;
        assert_eq!(mgr.get_mime_type(FileId::new(1, 0)).await, "video/webm");
        assert!(mgr.get_mime_type(FileId::new(99, 0)).await.is_empty());
    }

    #[tokio::test]
    async fn test_get_search_text() {
        let mgr = VideosManager::new();
        let mut video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        );
        video.set_file_name("Test Video.mp4".to_string());
        mgr.add_video(video).await;

        assert_eq!(
            mgr.get_search_text(FileId::new(1, 0)).await,
            "test video.mp4"
        );
    }

    #[tokio::test]
    async fn test_delete_thumbnail() {
        let mgr = VideosManager::new();
        let mut video = Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        );
        video.set_thumbnail(PhotoSize::new("thumbnail_1".to_string()));
        mgr.add_video(video).await;

        assert_eq!(
            mgr.get_video(FileId::new(1, 0))
                .await
                .unwrap()
                .thumbnail()
                .type_,
            "thumbnail_1"
        );

        mgr.delete_thumbnail(FileId::new(1, 0)).await;

        assert!(mgr
            .get_video(FileId::new(1, 0))
            .await
            .unwrap()
            .thumbnail()
            .type_
            .is_empty());
    }

    #[tokio::test]
    async fn test_manager_clear() {
        let mgr = VideosManager::new();
        mgr.add_video(Video::new(
            FileId::new(1, 0),
            30,
            Dimensions::from_wh(1920, 1080),
            "video/mp4",
        ))
        .await;
        mgr.add_video(Video::new(
            FileId::new(2, 0),
            45,
            Dimensions::from_wh(1280, 720),
            "video/webm",
        ))
        .await;
        assert_eq!(mgr.video_count().await, 2);

        mgr.clear().await;
        assert_eq!(mgr.video_count().await, 0);
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram_videos_manager");
    }
}
