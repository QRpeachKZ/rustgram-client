// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Group Call Join Parameters
//!
//! Parameters for joining a group call in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`GroupCallJoinParameters`] struct, which represents
//! the parameters needed to join a group call. It includes the payload,
//! audio source, and mute/video settings.
//! It mirrors TDLib's `GroupCallJoinParameters` struct.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_group_call_join_parameters::GroupCallJoinParameters;
//!
//! // Create join parameters
//! let params = GroupCallJoinParameters::new(
//!     vec![1, 2, 3, 4],
//!     12345,
//!     false,
//!     true
//! ).expect("valid parameters");
//!
//! assert_eq!(params.audio_source(), 12345);
//! assert!(!params.is_muted());
//! assert!(params.is_my_video_enabled());
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error type for group call join parameters validation.
///
/// # Example
///
/// ```rust
/// use rustgram_group_call_join_parameters::{GroupCallJoinParameters, GroupCallJoinParametersError};
///
/// let result = GroupCallJoinParameters::new(vec![1, 2, 3], -1, false, false);
/// assert!(matches!(result, Err(GroupCallJoinParametersError::InvalidAudioSource)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GroupCallJoinParametersError {
    /// Invalid audio source value (must be non-negative).
    InvalidAudioSource,
}

impl fmt::Display for GroupCallJoinParametersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAudioSource => write!(f, "Invalid audio source: must be non-negative"),
        }
    }
}

impl std::error::Error for GroupCallJoinParametersError {}

/// Parameters for joining a group call.
///
/// This type contains all the parameters needed when joining a group call
/// in Telegram, including the payload data and audio/video settings.
///
/// # Fields
///
/// - `payload` - The payload data for the join request
/// - `audio_source` - The audio source ID (must be non-negative)
/// - `is_muted` - Whether the user is muted
/// - `is_my_video_enabled` - Whether the user's video is enabled
///
/// # Example
///
/// ```rust
/// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
///
/// // Create valid join parameters
/// let params = GroupCallJoinParameters::new(
///     vec![1, 2, 3, 4],
///     12345,
///     false,
///     true
/// ).expect("valid parameters");
///
/// assert_eq!(params.audio_source(), 12345);
/// assert!(!params.is_muted());
/// assert!(params.is_my_video_enabled());
/// assert!(!params.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroupCallJoinParameters {
    /// The payload data for the join request.
    payload: Vec<u8>,

    /// The audio source ID (must be non-negative).
    audio_source: i32,

    /// Whether the user is muted.
    is_muted: bool,

    /// Whether the user's video is enabled.
    is_my_video_enabled: bool,
}

impl GroupCallJoinParameters {
    /// Creates new group call join parameters.
    ///
    /// # Arguments
    ///
    /// * `payload` - The payload data for the join request
    /// * `audio_source` - The audio source ID (must be non-negative)
    /// * `is_muted` - Whether the user is muted
    /// * `is_my_video_enabled` - Whether the user's video is enabled
    ///
    /// # Errors
    ///
    /// Returns an error if `audio_source` is negative.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// // Valid parameters
    /// let params = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3, 4],
    ///     12345,
    ///     false,
    ///     true
    /// ).expect("valid parameters");
    ///
    /// // Invalid audio source
    /// let result = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3],
    ///     -1,
    ///     false,
    ///     false
    /// );
    /// assert!(result.is_err());
    /// ```
    pub fn new(
        payload: Vec<u8>,
        audio_source: i32,
        is_muted: bool,
        is_my_video_enabled: bool,
    ) -> Result<Self, GroupCallJoinParametersError> {
        if audio_source < 0 {
            return Err(GroupCallJoinParametersError::InvalidAudioSource);
        }

        Ok(Self {
            payload,
            audio_source,
            is_muted,
            is_my_video_enabled,
        })
    }

    /// Returns the payload data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// let params = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3, 4],
    ///     12345,
    ///     false,
    ///     true
    /// ).unwrap();
    ///
    /// assert_eq!(params.payload(), &[1, 2, 3, 4]);
    /// ```
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Returns the audio source ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// let params = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3],
    ///     99999,
    ///     false,
    ///     true
    /// ).unwrap();
    ///
    /// assert_eq!(params.audio_source(), 99999);
    /// ```
    #[must_use]
    pub const fn audio_source(&self) -> i32 {
        self.audio_source
    }

    /// Returns whether the user is muted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// let params = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3],
    ///     12345,
    ///     true,
    ///     false
    /// ).unwrap();
    ///
    /// assert!(params.is_muted());
    /// ```
    #[must_use]
    pub const fn is_muted(&self) -> bool {
        self.is_muted
    }

    /// Returns whether the user's video is enabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// let params = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3],
    ///     12345,
    ///     false,
    ///     true
    /// ).unwrap();
    ///
    /// assert!(params.is_my_video_enabled());
    /// ```
    #[must_use]
    pub const fn is_my_video_enabled(&self) -> bool {
        self.is_my_video_enabled
    }

    /// Returns all values as a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// let params = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3, 4],
    ///     12345,
    ///     false,
    ///     true
    /// ).unwrap();
    ///
    /// let (payload, audio, muted, video) = params.get();
    /// assert_eq!(payload, &[1, 2, 3, 4]);
    /// assert_eq!(audio, 12345);
    /// assert_eq!(muted, false);
    /// assert_eq!(video, true);
    /// ```
    #[must_use]
    pub fn get(&self) -> (&[u8], i32, bool, bool) {
        (
            &self.payload,
            self.audio_source,
            self.is_muted,
            self.is_my_video_enabled,
        )
    }

    /// Checks if the payload is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// let with_payload = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3],
    ///     12345,
    ///     false,
    ///     true
    /// ).unwrap();
    /// assert!(!with_payload.is_empty());
    ///
    /// let without_payload = GroupCallJoinParameters::new(
    ///     vec![],
    ///     12345,
    ///     false,
    ///     true
    /// ).unwrap();
    /// assert!(without_payload.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

impl fmt::Display for GroupCallJoinParameters {
    /// Formats the group call join parameters for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    ///
    /// let params = GroupCallJoinParameters::new(
    ///     vec![1, 2, 3],
    ///     12345,
    ///     true,
    ///     false
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     format!("{}", params),
    ///     "GroupCallJoinParameters(audio_source: 12345, muted: true, video: false)"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GroupCallJoinParameters(audio_source: {}, muted: {}, video: {})",
            self.audio_source, self.is_muted, self.is_my_video_enabled
        )
    }
}

impl TryFrom<(Vec<u8>, i32, bool, bool)> for GroupCallJoinParameters {
    type Error = GroupCallJoinParametersError;

    /// Creates group call join parameters from a tuple.
    ///
    /// Returns an error if audio_source is negative.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_join_parameters::GroupCallJoinParameters;
    /// use std::convert::TryFrom;
    ///
    /// let params: Result<_, _> = GroupCallJoinParameters::try_from((vec![1, 2, 3], 12345, false, true));
    /// assert!(params.is_ok());
    ///
    /// let invalid: Result<_, _> = GroupCallJoinParameters::try_from((vec![1, 2, 3], -1, false, false));
    /// assert!(invalid.is_err());
    /// ```
    fn try_from(
        (payload, audio_source, is_muted, is_my_video_enabled): (Vec<u8>, i32, bool, bool),
    ) -> Result<Self, Self::Error> {
        Self::new(payload, audio_source, is_muted, is_my_video_enabled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3, 4], 12345, false, true).unwrap();
        assert_eq!(params.payload(), &[1, 2, 3, 4]);
        assert_eq!(params.audio_source(), 12345);
        assert!(!params.is_muted());
        assert!(params.is_my_video_enabled());
    }

    #[test]
    fn test_new_invalid_audio_source() {
        let result = GroupCallJoinParameters::new(vec![1, 2, 3], -1, false, false);
        assert!(matches!(
            result,
            Err(GroupCallJoinParametersError::InvalidAudioSource)
        ));
    }

    #[test]
    fn test_payload() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3, 4, 5], 12345, false, true).unwrap();
        assert_eq!(params.payload(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_audio_source() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 99999, false, true).unwrap();
        assert_eq!(params.audio_source(), 99999);
    }

    #[test]
    fn test_is_muted_true() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, true, false).unwrap();
        assert!(params.is_muted());
    }

    #[test]
    fn test_is_muted_false() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        assert!(!params.is_muted());
    }

    #[test]
    fn test_is_my_video_enabled_true() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        assert!(params.is_my_video_enabled());
    }

    #[test]
    fn test_is_my_video_enabled_false() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, false).unwrap();
        assert!(!params.is_my_video_enabled());
    }

    #[test]
    fn test_get() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3, 4], 12345, true, false).unwrap();
        let (payload, audio, muted, video) = params.get();
        assert_eq!(payload, &[1, 2, 3, 4]);
        assert_eq!(audio, 12345);
        assert!(muted);
        assert!(!video);
    }

    #[test]
    fn test_is_empty_true() {
        let params = GroupCallJoinParameters::new(vec![], 12345, false, true).unwrap();
        assert!(params.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        assert!(!params.is_empty());
    }

    #[test]
    fn test_equality() {
        let params1 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        let params2 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_inequality_different_payload() {
        let params1 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        let params2 = GroupCallJoinParameters::new(vec![1, 2, 3, 4], 12345, false, true).unwrap();
        assert_ne!(params1, params2);
    }

    #[test]
    fn test_inequality_different_audio_source() {
        let params1 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        let params2 = GroupCallJoinParameters::new(vec![1, 2, 3], 99999, false, true).unwrap();
        assert_ne!(params1, params2);
    }

    #[test]
    fn test_inequality_different_muted() {
        let params1 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, true, false).unwrap();
        let params2 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, false).unwrap();
        assert_ne!(params1, params2);
    }

    #[test]
    fn test_inequality_different_video() {
        let params1 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        let params2 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, false).unwrap();
        assert_ne!(params1, params2);
    }

    #[test]
    fn test_clone_semantics() {
        let params1 = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        let params2 = params1.clone();
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_display_format() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, true, false).unwrap();
        assert_eq!(
            format!("{}", params),
            "GroupCallJoinParameters(audio_source: 12345, muted: true, video: false)"
        );
    }

    #[test]
    fn test_try_from_tuple_valid() {
        use std::convert::TryFrom;

        let params: Result<GroupCallJoinParameters, _> =
            GroupCallJoinParameters::try_from((vec![1, 2, 3], 12345, false, true));
        assert!(params.is_ok());
    }

    #[test]
    fn test_try_from_tuple_invalid() {
        use std::convert::TryFrom;

        let params: Result<GroupCallJoinParameters, _> =
            GroupCallJoinParameters::try_from((vec![1, 2, 3], -1, false, false));
        assert!(params.is_err());
    }

    #[test]
    fn test_debug_format() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 12345, false, true).unwrap();
        let debug_str = format!("{:?}", params);
        assert!(debug_str.contains("GroupCallJoinParameters"));
    }

    #[test]
    fn test_zero_audio_source() {
        let params = GroupCallJoinParameters::new(vec![1, 2, 3], 0, false, true).unwrap();
        assert_eq!(params.audio_source(), 0);
    }

    #[test]
    fn test_error_display() {
        let error = GroupCallJoinParametersError::InvalidAudioSource;
        assert_eq!(
            format!("{}", error),
            "Invalid audio source: must be non-negative"
        );
    }

    #[test]
    fn test_all_combinations_of_bools() {
        for muted in [true, false] {
            for video in [true, false] {
                let params =
                    GroupCallJoinParameters::new(vec![1, 2, 3], 12345, muted, video).unwrap();
                assert_eq!(params.is_muted(), muted);
                assert_eq!(params.is_my_video_enabled(), video);
            }
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = GroupCallJoinParameters::new(vec![1, 2, 3, 4], 12345, false, true).unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"payload":[1,2,3,4],"audio_source":12345,"is_muted":false,"is_my_video_enabled":true}"#
        );

        let deserialized: GroupCallJoinParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test binary serialization
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: GroupCallJoinParameters = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_empty_payload() {
        let original = GroupCallJoinParameters::new(vec![], 12345, true, false).unwrap();

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"payload":[],"audio_source":12345,"is_muted":true,"is_my_video_enabled":false}"#
        );

        let deserialized: GroupCallJoinParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_error_serialize() {
        let error = GroupCallJoinParametersError::InvalidAudioSource;
        let json = serde_json::to_string(&error).unwrap();
        assert_eq!(json, r#""InvalidAudioSource""#);
    }
}
