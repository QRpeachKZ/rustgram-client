//! Type definitions for AnimationsManager.

use serde::{Deserialize, Serialize};

// Use the actual package names (with underscores for module names)
use rustgram_animation_size as animation_size_module;
use rustgram_dimensions as dimensions_module;
use rustgram_file_id as file_id_module;
use rustgram_photo_size as photo_size_module;
use rustgram_secret_input_media as secret_input_media_module;

// Type aliases for cleaner code
pub use animation_size_module::AnimationSize;
pub use dimensions_module::Dimensions;
pub use file_id_module::FileId;
pub use photo_size_module::PhotoSize;
pub use secret_input_media_module::SecretInputMedia;

/// Animation metadata structure.
///
/// Contains all metadata for a GIF/animation file including duration, dimensions,
/// thumbnails, and associated stickers.
///
/// # TDLib Correspondence
///
/// Corresponds to `AnimationsManager::Animation` class in TDLib.
///
/// # Examples
///
/// ```
/// use rustgram_animations_manager::Animation;
/// use rustgram_file_id::FileId;
/// use rustgram_dimensions::Dimensions;
/// use rustgram_photo_size::PhotoSize;
/// use rustgram_animation_size::AnimationSize;
///
/// let animation = Animation::new(
///     FileId::new(123, 456),
///     "animation.mp4".to_string(),
///     "video/mp4".to_string(),
///     5,
///     Dimensions::from_wh(640, 480),
///     "minithumb".to_string(),
///     PhotoSize::new("s".to_string()),
///     AnimationSize::new(0, 0),
///     false,
///     vec![],
/// );
///
/// assert_eq!(animation.file_id().get(), 123);
/// assert_eq!(animation.duration(), 5);
/// assert!(animation.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    /// Animation file identifier
    file_id: FileId,
    /// Original file name
    file_name: String,
    /// MIME type (e.g., "video/mp4", "image/gif")
    mime_type: String,
    /// Duration in seconds (clamped to >= 0)
    duration: i32,
    /// Width × height in pixels
    dimensions: Dimensions,
    /// JPEG minithumbnail data
    minithumbnail: String,
    /// Static thumbnail preview
    thumbnail: PhotoSize,
    /// Animated MPEG-4 thumbnail
    animated_thumbnail: AnimationSize,
    /// Whether attached stickers exist
    has_stickers: bool,
    /// Associated sticker file IDs
    sticker_file_ids: Vec<FileId>,
}

impl Animation {
    /// Creates a new Animation with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Animation file identifier
    /// * `file_name` - Original file name
    /// * `mime_type` - MIME type (e.g., "video/mp4", "image/gif")
    /// * `duration` - Duration in seconds (will be clamped to >= 0)
    /// * `dimensions` - Width × height in pixels
    /// * `minithumbnail` - JPEG minithumbnail data
    /// * `thumbnail` - Static thumbnail preview
    /// * `animated_thumbnail` - Animated MPEG-4 thumbnail
    /// * `has_stickers` - Whether attached stickers exist
    /// * `sticker_file_ids` - Associated sticker file IDs
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::Animation;
    /// use rustgram_file_id::FileId;
    /// use rustgram_dimensions::Dimensions;
    /// use rustgram_photo_size::PhotoSize;
    /// use rustgram_animation_size::AnimationSize;
    ///
    /// let animation = Animation::new(
    ///     FileId::new(1, 2),
    ///     "test.mp4".to_string(),
    ///     "video/mp4".to_string(),
    ///     10,
    ///     Dimensions::from_wh(320, 240),
    ///     "thumb".to_string(),
    ///     PhotoSize::new("s".to_string()),
    ///     AnimationSize::new(0, 0),
    ///     true,
    ///     vec![FileId::new(3, 4)],
    /// );
    /// ```
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        file_id: FileId,
        file_name: String,
        mime_type: String,
        duration: i32,
        dimensions: Dimensions,
        minithumbnail: String,
        thumbnail: PhotoSize,
        animated_thumbnail: AnimationSize,
        has_stickers: bool,
        sticker_file_ids: Vec<FileId>,
    ) -> Self {
        Self {
            file_id,
            file_name,
            mime_type,
            duration: duration.max(0),
            dimensions,
            minithumbnail,
            thumbnail,
            animated_thumbnail,
            has_stickers,
            sticker_file_ids,
        }
    }

    /// Creates an empty Animation for testing purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::Animation;
    ///
    /// let empty = Animation::empty();
    /// assert!(!empty.file_id().is_valid());
    /// assert_eq!(empty.duration(), 0);
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self {
            file_id: FileId::empty(),
            file_name: String::new(),
            mime_type: String::new(),
            duration: 0,
            dimensions: Dimensions::from_wh(0, 0),
            minithumbnail: String::new(),
            thumbnail: PhotoSize::new(String::new()),
            animated_thumbnail: AnimationSize::new(0, 0),
            has_stickers: false,
            sticker_file_ids: Vec::new(),
        }
    }

    /// Returns the file ID of the animation.
    #[must_use]
    #[inline]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the file name of the animation.
    #[must_use]
    #[inline]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns the MIME type of the animation.
    #[must_use]
    #[inline]
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    /// Returns the duration in seconds.
    #[must_use]
    #[inline]
    pub const fn duration(&self) -> i32 {
        self.duration
    }

    /// Returns the dimensions of the animation.
    #[must_use]
    #[inline]
    pub const fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    /// Returns the minithumbnail data.
    #[must_use]
    #[inline]
    pub fn minithumbnail(&self) -> &str {
        &self.minithumbnail
    }

    /// Returns the static thumbnail.
    #[must_use]
    #[inline]
    pub const fn thumbnail(&self) -> &PhotoSize {
        &self.thumbnail
    }

    /// Returns the animated thumbnail.
    #[must_use]
    #[inline]
    pub const fn animated_thumbnail(&self) -> &AnimationSize {
        &self.animated_thumbnail
    }

    /// Returns whether the animation has associated stickers.
    #[must_use]
    #[inline]
    pub const fn has_stickers(&self) -> bool {
        self.has_stickers
    }

    /// Returns the sticker file IDs associated with this animation.
    #[must_use]
    #[inline]
    pub fn sticker_file_ids(&self) -> &[FileId] {
        &self.sticker_file_ids
    }

    /// Returns `true` if this animation is valid (has valid file ID).
    #[must_use]
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.file_id.is_valid()
    }

    /// Returns the thumbnail file ID (static thumbnail).
    #[must_use]
    #[inline]
    pub const fn get_thumbnail_file_id(&self) -> FileId {
        // TODO: When PhotoSize has file_id field, access it
        // For now, return empty since stub doesn't have file_id
        FileId::empty()
    }

    /// Returns the animated thumbnail file ID.
    #[must_use]
    #[inline]
    pub fn get_animated_thumbnail_file_id(&self) -> FileId {
        self.animated_thumbnail.file_id
    }
}

/// Saved animations loading state.
///
/// Represents the current state of saved animations loading.
///
/// # TDLib Correspondence
///
/// Corresponds to the state machine in AnimationsManager:
/// - `are_saved_animations_loaded_` -> Loaded
/// - `are_saved_animations_being_loaded_` -> Loading
/// - Neither -> NotLoaded
///
/// # State Transitions
///
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
/// # Examples
///
/// ```
/// use rustgram_animations_manager::SavedAnimationsState;
///
/// let state = SavedAnimationsState::NotLoaded;
/// assert!(matches!(state, SavedAnimationsState::NotLoaded));
/// assert!(!state.is_loaded());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SavedAnimationsState {
    /// Initial state, not yet loaded
    #[default]
    NotLoaded,
    /// Currently loading from database or server
    Loading,
    /// Successfully loaded, ready for access
    Loaded,
}

impl SavedAnimationsState {
    /// Returns `true` if animations are currently loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::SavedAnimationsState;
    ///
    /// assert!(!SavedAnimationsState::NotLoaded.is_loaded());
    /// assert!(!SavedAnimationsState::Loading.is_loaded());
    /// assert!(SavedAnimationsState::Loaded.is_loaded());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_loaded(&self) -> bool {
        matches!(self, Self::Loaded)
    }

    /// Returns `true` if animations are currently loading.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::SavedAnimationsState;
    ///
    /// assert!(!SavedAnimationsState::NotLoaded.is_loading());
    /// assert!(SavedAnimationsState::Loading.is_loading());
    /// assert!(!SavedAnimationsState::Loaded.is_loading());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }
}

/// Animation search parameters.
///
/// Configuration for animation search hints and provider.
///
/// # TDLib Correspondence
///
/// Corresponds to `animation_search_emojis_` and `animation_search_provider_`
/// fields in AnimationsManager.
///
/// # Examples
///
/// ```
/// use rustgram_animations_manager::AnimationSearchParameters;
///
/// let params = AnimationSearchParameters::new();
/// assert!(!params.is_emojis_inited());
/// assert!(!params.is_provider_inited());
///
/// let with_emojis = params.with_emojis(vec!["👍".to_string(), "❤️".to_string()]);
/// assert!(with_emojis.is_emojis_inited());
///
/// let with_provider = with_emojis.with_provider("giphy".to_string());
/// assert!(with_provider.is_provider_inited());
/// assert_eq!(with_provider.provider(), Some("giphy"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationSearchParameters {
    /// Emoji list for search hints
    emojis: Vec<String>,
    /// Search provider name
    provider: String,
    /// Whether emoji list is initialized
    is_emojis_inited: bool,
    /// Whether provider is initialized
    is_provider_inited: bool,
}

impl AnimationSearchParameters {
    /// Creates a new empty AnimationSearchParameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_animations_manager::AnimationSearchParameters;
    ///
    /// let params = AnimationSearchParameters::new();
    /// assert!(params.emojis().is_empty());
    /// assert_eq!(params.provider(), None);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            emojis: Vec::new(),
            provider: String::new(),
            is_emojis_inited: false,
            is_provider_inited: false,
        }
    }

    /// Returns the emoji list for search hints.
    #[must_use]
    #[inline]
    pub fn emojis(&self) -> &[String] {
        &self.emojis
    }

    /// Returns the provider name if initialized.
    #[must_use]
    #[inline]
    pub fn provider(&self) -> Option<&str> {
        if self.is_provider_inited {
            Some(&self.provider)
        } else {
            None
        }
    }

    /// Returns `true` if the emoji list is initialized.
    #[must_use]
    #[inline]
    pub const fn is_emojis_inited(&self) -> bool {
        self.is_emojis_inited
    }

    /// Returns `true` if the provider is initialized.
    #[must_use]
    #[inline]
    pub const fn is_provider_inited(&self) -> bool {
        self.is_provider_inited
    }

    /// Sets the emoji list and marks as initialized.
    #[must_use]
    pub fn with_emojis(mut self, emojis: Vec<String>) -> Self {
        self.emojis = emojis;
        self.is_emojis_inited = true;
        self
    }

    /// Sets the provider and marks as initialized.
    #[must_use]
    pub fn with_provider(mut self, provider: String) -> Self {
        self.provider = provider;
        self.is_provider_inited = true;
        self
    }

    /// Returns `true` if both emojis and provider are initialized.
    #[must_use]
    #[inline]
    pub const fn is_fully_initialized(&self) -> bool {
        self.is_emojis_inited && self.is_provider_inited
    }
}

impl Default for AnimationSearchParameters {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub type for InputFile (for animation add/remove operations).
///
/// TODO: Replace with full implementation when InputFile type is available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputFile {
    /// Inner placeholder data
    _inner: (),
}

impl InputFile {
    /// Creates a new placeholder InputFile.
    #[must_use]
    pub const fn new() -> Self {
        Self { _inner: () }
    }
}

impl Default for InputFile {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub type for InputMedia (for message sending).
///
/// TODO: Replace with full implementation when InputMedia type is available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputMedia {
    /// Inner placeholder data
    _inner: (),
}

impl InputMedia {
    /// Creates a new placeholder InputMedia.
    #[must_use]
    pub const fn new() -> Self {
        Self { _inner: () }
    }
}

impl Default for InputMedia {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub type for Update (for client updates).
///
/// TODO: Replace with full implementation when Update type is available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    /// Inner placeholder data
    _inner: (),
}

impl Update {
    /// Creates a new placeholder Update.
    #[must_use]
    pub const fn new() -> Self {
        Self { _inner: () }
    }
}

impl Default for Update {
    fn default() -> Self {
        Self::new()
    }
}

/// Re-export of SecretInputMedia from rustgram-secret-input-media.

#[cfg(test)]
mod tests {
    use super::*;

    // === Animation tests ===

    #[test]
    fn test_animation_new_valid() {
        let animation = Animation::new(
            FileId::new(123, 456),
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            10,
            Dimensions::from_wh(640, 480),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
        );
        assert_eq!(animation.file_id().get(), 123);
        assert_eq!(animation.file_name(), "test.mp4");
        assert_eq!(animation.mime_type(), "video/mp4");
        assert_eq!(animation.duration(), 10);
        assert_eq!(animation.dimensions().width(), 640);
        assert_eq!(animation.dimensions().height(), 480);
        assert!(animation.is_valid());
    }

    #[test]
    fn test_animation_new_clamped_duration() {
        let animation = Animation::new(
            FileId::new(1, 2),
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            -5,
            Dimensions::from_wh(100, 100),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
        );
        assert_eq!(animation.duration(), 0); // Clamped to 0
    }

    #[test]
    fn test_animation_empty() {
        let animation = Animation::empty();
        assert!(!animation.file_id().is_valid());
        assert_eq!(animation.file_name(), "");
        assert_eq!(animation.mime_type(), "");
        assert_eq!(animation.duration(), 0);
        assert!(!animation.is_valid());
    }

    #[test]
    fn test_animation_clone() {
        let animation = Animation::new(
            FileId::new(1, 2),
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            5,
            Dimensions::from_wh(200, 200),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            true,
            vec![FileId::new(3, 4)],
        );
        let cloned = animation.clone();
        assert_eq!(animation, cloned);
    }

    #[test]
    fn test_animation_equality() {
        let anim1 = Animation::new(
            FileId::new(1, 2),
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            5,
            Dimensions::from_wh(200, 200),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
        );
        let anim2 = Animation::new(
            FileId::new(1, 2),
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            5,
            Dimensions::from_wh(200, 200),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
        );
        assert_eq!(anim1, anim2);

        let anim3 = Animation::new(
            FileId::new(3, 4),
            "other.mp4".to_string(),
            "video/mp4".to_string(),
            10,
            Dimensions::from_wh(400, 400),
            "thumb".to_string(),
            PhotoSize::new("m".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
        );
        assert_ne!(anim1, anim3);
    }

    #[test]
    fn test_animation_getters() {
        let sticker_ids = vec![FileId::new(10, 20), FileId::new(30, 40)];
        let animation = Animation::new(
            FileId::new(1, 2),
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            15,
            Dimensions::from_wh(800, 600),
            "minithumb".to_string(),
            PhotoSize::new("x".to_string()),
            AnimationSize::new(0, 0),
            true,
            sticker_ids.clone(),
        );

        assert_eq!(animation.file_name(), "test.mp4");
        assert_eq!(animation.mime_type(), "video/mp4");
        assert_eq!(animation.duration(), 15);
        assert_eq!(animation.dimensions().width(), 800);
        assert_eq!(animation.dimensions().height(), 600);
        assert_eq!(animation.minithumbnail(), "minithumb");
        assert!(animation.has_stickers());
        assert_eq!(animation.sticker_file_ids(), &sticker_ids);
    }

    #[test]
    fn test_animation_thumbnails() {
        let animation = Animation::new(
            FileId::new(1, 2),
            "test.mp4".to_string(),
            "video/mp4".to_string(),
            5,
            Dimensions::from_wh(100, 100),
            "thumb".to_string(),
            PhotoSize::new("s".to_string()),
            AnimationSize::new(0, 0),
            false,
            vec![],
        );

        assert_eq!(animation.thumbnail().type_, "s");
        assert!(!animation.get_animated_thumbnail_file_id().is_valid());
    }

    // === SavedAnimationsState tests ===

    #[test]
    fn test_state_initial_not_loaded() {
        let state = SavedAnimationsState::NotLoaded;
        assert!(matches!(state, SavedAnimationsState::NotLoaded));
        assert!(!state.is_loaded());
        assert!(!state.is_loading());
    }

    #[test]
    fn test_state_loading() {
        let state = SavedAnimationsState::Loading;
        assert!(matches!(state, SavedAnimationsState::Loading));
        assert!(!state.is_loaded());
        assert!(state.is_loading());
    }

    #[test]
    fn test_state_loaded() {
        let state = SavedAnimationsState::Loaded;
        assert!(matches!(state, SavedAnimationsState::Loaded));
        assert!(state.is_loaded());
        assert!(!state.is_loading());
    }

    #[test]
    fn test_state_default() {
        let state = SavedAnimationsState::default();
        assert!(matches!(state, SavedAnimationsState::NotLoaded));
    }

    #[test]
    fn test_state_copy() {
        let state = SavedAnimationsState::Loaded;
        let copied = state;
        assert_eq!(state, copied);
    }

    // === AnimationSearchParameters tests ===

    #[test]
    fn test_search_parameters_new() {
        let params = AnimationSearchParameters::new();
        assert!(params.emojis().is_empty());
        assert_eq!(params.provider(), None);
        assert!(!params.is_emojis_inited());
        assert!(!params.is_provider_inited());
        assert!(!params.is_fully_initialized());
    }

    #[test]
    fn test_search_parameters_with_emojis() {
        let params = AnimationSearchParameters::new().with_emojis(vec![
            "👍".to_string(),
            "❤️".to_string(),
            "😂".to_string(),
        ]);
        assert_eq!(params.emojis(), &["👍", "❤️", "😂"]);
        assert!(params.is_emojis_inited());
        assert!(!params.is_provider_inited());
    }

    #[test]
    fn test_search_parameters_with_provider() {
        let params = AnimationSearchParameters::new().with_provider("giphy".to_string());
        assert!(params.emojis().is_empty());
        assert!(!params.is_emojis_inited());
        assert_eq!(params.provider(), Some("giphy"));
        assert!(params.is_provider_inited());
    }

    #[test]
    fn test_search_parameters_fully_initialized() {
        let params = AnimationSearchParameters::new()
            .with_emojis(vec!["👍".to_string()])
            .with_provider("giphy".to_string());
        assert!(params.is_fully_initialized());
        assert_eq!(params.emojis().len(), 1);
        assert_eq!(params.provider(), Some("giphy"));
    }

    #[test]
    fn test_search_parameters_default() {
        let params = AnimationSearchParameters::default();
        assert!(params.emojis().is_empty());
        assert!(!params.is_emojis_inited());
    }

    #[test]
    fn test_search_parameters_clone() {
        let params = AnimationSearchParameters::new()
            .with_emojis(vec!["test".to_string()])
            .with_provider("test".to_string());
        let cloned = params.clone();
        assert_eq!(params, cloned);
    }

    // === Stub type tests ===

    #[test]
    fn test_input_file_stub() {
        let file = InputFile::new();
        let default = InputFile::default();
        assert_eq!(file, default);
    }

    #[test]
    fn test_input_media_stub() {
        let media = InputMedia::new();
        let default = InputMedia::default();
        assert_eq!(media, default);
    }

    #[test]
    fn test_update_stub() {
        let update = Update::new();
        let default = Update::default();
        assert_eq!(update, default);
    }
}
