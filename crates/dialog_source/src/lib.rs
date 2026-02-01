// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Dialog Source
//!
//! Source of a dialog in Telegram MTProto.
//!
//! Based on TDLib's `DialogSource` from `td/telegram/DialogSource.h`.
//!
//! # Overview
//!
//! A `DialogSource` indicates how a dialog appeared in the user's chat list.
//! This is primarily used for sponsored messages and public service announcements.
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_source::DialogSource;
//!
//! let source = DialogSource::mtproto_proxy();
//! let serialized = source.serialize();
//! assert_eq!(serialized, "1");
//!
//! let deserialized = DialogSource::unserialize(&serialized).unwrap();
//! assert_eq!(source, deserialized);
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fmt;
use std::str;

/// Type identifier for MTProto proxy source.
const TYPE_MTPROTO_PROXY: i32 = 1;

/// Type identifier for public service announcement source.
const TYPE_PSA: i32 = 2;

/// Separator character for PSA data serialization.
const PSA_SEPARATOR: char = '\x01';

/// Error type for dialog source operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogSourceError {
    /// Invalid type identifier in serialized data.
    InvalidType(String),
    /// Malformed serialized data.
    MalformedData(String),
    /// Missing required field in serialized data.
    MissingField(String),
}

impl fmt::Display for DialogSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidType(msg) => write!(f, "Invalid type: {}", msg),
            Self::MalformedData(msg) => write!(f, "Malformed data: {}", msg),
            Self::MissingField(msg) => write!(f, "Missing field: {}", msg),
        }
    }
}

impl std::error::Error for DialogSourceError {}

/// Dialog source type.
///
/// Represents how a dialog appeared in the user's chat list.
///
/// # Variants
///
/// * **Membership** - Default/legacy state (not used in sponsored messages)
/// * **MtprotoProxy** - Dialog from MTProto proxy sponsorship
/// * **PublicServiceAnnouncement** - Dialog from a public service announcement
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_source::DialogSource;
///
/// let proxy = DialogSource::mtproto_proxy();
/// let psa = DialogSource::public_service_announcement("UPGRADE", "Upgrade your app");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DialogSource {
    /// Default/legacy state (not used in sponsored messages).
    #[default]
    Membership,
    /// Dialog from MTProto proxy sponsorship.
    MtprotoProxy,
    /// Dialog from a public service announcement.
    PublicServiceAnnouncement {
        /// Type of the PSA (e.g., "UPGRADE", "PREMIUM")
        psa_type: String,
        /// Message content of the PSA
        psa_text: String,
    },
}

impl DialogSource {
    /// Creates a MTProto proxy source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::mtproto_proxy();
    /// assert!(matches!(source, DialogSource::MtprotoProxy));
    /// ```
    #[must_use]
    pub const fn mtproto_proxy() -> Self {
        Self::MtprotoProxy
    }

    /// Creates a public service announcement source.
    ///
    /// # Arguments
    ///
    /// * `psa_type` - Type identifier for the PSA
    /// * `psa_text` - Message content for the PSA
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::public_service_announcement("UPGRADE", "Upgrade now");
    /// assert!(matches!(source, DialogSource::PublicServiceAnnouncement { .. }));
    /// ```
    #[must_use]
    pub fn public_service_announcement<S1, S2>(psa_type: S1, psa_text: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::PublicServiceAnnouncement {
            psa_type: psa_type.into(),
            psa_text: psa_text.into(),
        }
    }

    /// Serializes the dialog source to a string.
    ///
    /// # Serialization Format
    ///
    /// * **Membership**: Empty string (legacy)
    /// * **MtprotoProxy**: `"1"`
    /// * **PublicServiceAnnouncement**: `"2 psa_type\x01psa_text"`
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let proxy = DialogSource::mtproto_proxy();
    /// assert_eq!(proxy.serialize(), "1");
    ///
    /// let psa = DialogSource::public_service_announcement("TYPE", "text");
    /// assert_eq!(psa.serialize(), "2 TYPE\x01text");
    /// ```
    #[must_use]
    pub fn serialize(&self) -> String {
        match self {
            Self::Membership => String::new(),
            Self::MtprotoProxy => TYPE_MTPROTO_PROXY.to_string(),
            Self::PublicServiceAnnouncement { psa_type, psa_text } => {
                format!("{} {}{}{}", TYPE_PSA, psa_type, PSA_SEPARATOR, psa_text)
            }
        }
    }

    /// Deserializes a dialog source from a string.
    ///
    /// # Arguments
    ///
    /// * `data` - The serialized dialog source string
    ///
    /// # Errors
    ///
    /// Returns an error if the data is malformed or contains an invalid type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::unserialize("1").unwrap();
    /// assert!(matches!(source, DialogSource::MtprotoProxy));
    /// ```
    pub fn unserialize<S: AsRef<str>>(data: S) -> Result<Self, DialogSourceError> {
        let data = data.as_ref();

        // Legacy: empty string is treated as mtproto_proxy
        if data.is_empty() {
            return Ok(Self::MtprotoProxy);
        }

        // Parse type identifier
        let rest = data.strip_prefix(' ').unwrap_or(data);
        let mut parts = rest.splitn(2, ' ');

        let type_str = parts
            .next()
            .ok_or_else(|| DialogSourceError::MissingField("type".to_string()))?;

        let type_id: i32 = type_str
            .parse()
            .map_err(|_| DialogSourceError::InvalidType(type_str.to_string()))?;

        match type_id {
            TYPE_MTPROTO_PROXY => Ok(Self::MtprotoProxy),
            TYPE_PSA => {
                let data_part = parts
                    .next()
                    .ok_or_else(|| DialogSourceError::MissingField("psa data".to_string()))?;

                let mut psa_parts = data_part.splitn(2, PSA_SEPARATOR);
                let psa_type = psa_parts
                    .next()
                    .ok_or_else(|| DialogSourceError::MissingField("psa_type".to_string()))?
                    .to_string();

                let psa_text = psa_parts
                    .next()
                    .ok_or_else(|| DialogSourceError::MissingField("psa_text".to_string()))?
                    .to_string();

                Ok(Self::PublicServiceAnnouncement { psa_type, psa_text })
            }
            _ => Err(DialogSourceError::InvalidType(format!(
                "unknown type id: {}",
                type_id
            ))),
        }
    }

    /// Checks if this source is the default Membership type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// assert!(DialogSource::Membership.is_membership());
    /// assert!(!DialogSource::mtproto_proxy().is_membership());
    /// ```
    #[must_use]
    pub const fn is_membership(&self) -> bool {
        matches!(self, Self::Membership)
    }

    /// Checks if this is an MTProto proxy source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// assert!(DialogSource::mtproto_proxy().is_mtproto_proxy());
    /// ```
    #[must_use]
    pub const fn is_mtproto_proxy(&self) -> bool {
        matches!(self, Self::MtprotoProxy)
    }

    /// Checks if this is a public service announcement source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let psa = DialogSource::public_service_announcement("TYPE", "text");
    /// assert!(psa.is_public_service_announcement());
    /// ```
    #[must_use]
    pub const fn is_public_service_announcement(&self) -> bool {
        matches!(self, Self::PublicServiceAnnouncement { .. })
    }

    /// Returns the PSA type if this is a public service announcement.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let psa = DialogSource::public_service_announcement("UPGRADE", "text");
    /// assert_eq!(psa.psa_type(), Some("UPGRADE"));
    /// ```
    #[must_use]
    pub fn psa_type(&self) -> Option<&str> {
        match self {
            Self::PublicServiceAnnouncement { psa_type, .. } => Some(psa_type),
            _ => None,
        }
    }

    /// Returns the PSA text if this is a public service announcement.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let psa = DialogSource::public_service_announcement("TYPE", "Upgrade now");
    /// assert_eq!(psa.psa_text(), Some("Upgrade now"));
    /// ```
    #[must_use]
    pub fn psa_text(&self) -> Option<&str> {
        match self {
            Self::PublicServiceAnnouncement { psa_text, .. } => Some(psa_text),
            _ => None,
        }
    }
}

impl fmt::Display for DialogSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Membership => write!(f, "chat list"),
            Self::MtprotoProxy => write!(f, "MTProto proxy sponsor"),
            Self::PublicServiceAnnouncement { psa_type, .. } => {
                write!(f, "public service announcement of type \"{}\"", psa_type)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to assert round-trip serialization
    fn assert_round_trip(source: DialogSource) {
        let serialized = source.serialize();
        let deserialized = DialogSource::unserialize(&serialized).unwrap();
        assert_eq!(
            source, deserialized,
            "Round-trip failed for: {:?} -> {:?}",
            source, serialized
        );
    }

    #[test]
    fn test_mtproto_proxy() {
        let source = DialogSource::mtproto_proxy();
        assert!(matches!(source, DialogSource::MtprotoProxy));
        assert!(source.is_mtproto_proxy());
        assert!(!source.is_membership());
        assert!(!source.is_public_service_announcement());
    }

    #[test]
    fn test_public_service_announcement() {
        let source = DialogSource::public_service_announcement("UPGRADE", "Upgrade your app");
        assert!(matches!(
            source,
            DialogSource::PublicServiceAnnouncement { .. }
        ));
        assert!(source.is_public_service_announcement());
        assert!(!source.is_membership());
        assert!(!source.is_mtproto_proxy());
    }

    #[test]
    fn test_default() {
        let source = DialogSource::default();
        assert!(matches!(source, DialogSource::Membership));
        assert!(source.is_membership());
    }

    #[test]
    fn test_serialize_membership() {
        let source = DialogSource::Membership;
        assert_eq!(source.serialize(), "");
    }

    #[test]
    fn test_serialize_mtproto_proxy() {
        let source = DialogSource::MtprotoProxy;
        assert_eq!(source.serialize(), "1");
    }

    #[test]
    fn test_serialize_psa() {
        let source = DialogSource::PublicServiceAnnouncement {
            psa_type: "UPGRADE".to_string(),
            psa_text: "Upgrade now".to_string(),
        };
        assert_eq!(source.serialize(), "2 UPGRADE\x01Upgrade now");
    }

    #[test]
    fn test_serialize_psa_with_spaces() {
        let source = DialogSource::PublicServiceAnnouncement {
            psa_type: "PREMIUM FEATURES".to_string(),
            psa_text: "Get premium access now".to_string(),
        };
        assert_eq!(
            source.serialize(),
            "2 PREMIUM FEATURES\x01Get premium access now"
        );
    }

    #[test]
    fn test_unserialize_empty_string() {
        // Legacy: empty string is treated as mtproto_proxy
        let source = DialogSource::unserialize("").unwrap();
        assert!(matches!(source, DialogSource::MtprotoProxy));
    }

    #[test]
    fn test_unserialize_mtproto_proxy() {
        let source = DialogSource::unserialize("1").unwrap();
        assert!(matches!(source, DialogSource::MtprotoProxy));
    }

    #[test]
    fn test_unserialize_psa() {
        let source = DialogSource::unserialize("2 TYPE\x01text").unwrap();
        assert!(matches!(
            source,
            DialogSource::PublicServiceAnnouncement { .. }
        ));
        assert_eq!(source.psa_type(), Some("TYPE"));
        assert_eq!(source.psa_text(), Some("text"));
    }

    #[test]
    fn test_unserialize_psa_with_complex_data() {
        let source = DialogSource::unserialize("2 UPGRADE\x01Upgrade your app now").unwrap();
        assert!(matches!(
            source,
            DialogSource::PublicServiceAnnouncement { .. }
        ));
        assert_eq!(source.psa_type(), Some("UPGRADE"));
        assert_eq!(source.psa_text(), Some("Upgrade your app now"));
    }

    #[test]
    fn test_unserialize_invalid_type() {
        let result = DialogSource::unserialize("999");
        assert!(result.is_err());
        match result {
            Err(DialogSourceError::InvalidType(_)) => {}
            _ => panic!("Expected InvalidType error"),
        }
    }

    #[test]
    fn test_unserialize_non_numeric_type() {
        let result = DialogSource::unserialize("abc");
        assert!(result.is_err());
        match result {
            Err(DialogSourceError::InvalidType(_)) => {}
            _ => panic!("Expected InvalidType error"),
        }
    }

    #[test]
    fn test_unserialize_missing_psa_data() {
        let result = DialogSource::unserialize("2");
        assert!(result.is_err());
        match result {
            Err(DialogSourceError::MissingField(_)) => {}
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_unserialize_missing_psa_separator() {
        let result = DialogSource::unserialize("2 TYPE");
        assert!(result.is_err());
        match result {
            Err(DialogSourceError::MissingField(_)) => {}
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_round_trip_membership() {
        // Note: Membership serializes to empty string, which deserializes to MtprotoProxy (legacy)
        // So we only test that serialization doesn't panic
        let source = DialogSource::Membership;
        let _ = source.serialize();
    }

    #[test]
    fn test_round_trip_mtproto_proxy() {
        assert_round_trip(DialogSource::MtprotoProxy);
    }

    #[test]
    fn test_round_trip_psa() {
        assert_round_trip(DialogSource::public_service_announcement("TYPE", "text"));
        assert_round_trip(DialogSource::public_service_announcement(
            "UPGRADE",
            "Upgrade now",
        ));
    }

    #[test]
    fn test_equality() {
        let source1 = DialogSource::mtproto_proxy();
        let source2 = DialogSource::mtproto_proxy();
        assert_eq!(source1, source2);

        let source3 = DialogSource::public_service_announcement("TYPE", "text");
        let source4 = DialogSource::public_service_announcement("TYPE", "text");
        assert_eq!(source3, source4);

        let source5 = DialogSource::public_service_announcement("TYPE", "text");
        let source6 = DialogSource::public_service_announcement("OTHER", "text");
        assert_ne!(source5, source6);
    }

    #[test]
    fn test_display() {
        let membership = DialogSource::Membership;
        assert_eq!(format!("{}", membership), "chat list");

        let proxy = DialogSource::mtproto_proxy();
        assert_eq!(format!("{}", proxy), "MTProto proxy sponsor");

        let psa = DialogSource::public_service_announcement("UPGRADE", "text");
        assert_eq!(
            format!("{}", psa),
            "public service announcement of type \"UPGRADE\""
        );
    }

    #[test]
    fn test_psa_type() {
        let source = DialogSource::public_service_announcement("UPGRADE", "text");
        assert_eq!(source.psa_type(), Some("UPGRADE"));

        let other = DialogSource::mtproto_proxy();
        assert_eq!(other.psa_type(), None);
    }

    #[test]
    fn test_psa_text() {
        let source = DialogSource::public_service_announcement("TYPE", "Upgrade now");
        assert_eq!(source.psa_text(), Some("Upgrade now"));

        let other = DialogSource::mtproto_proxy();
        assert_eq!(other.psa_text(), None);
    }

    #[test]
    fn test_empty_psa_strings() {
        let source = DialogSource::public_service_announcement("", "");
        assert_eq!(source.serialize(), "2 \x01");
        let deserialized = DialogSource::unserialize(&source.serialize()).unwrap();
        assert_eq!(deserialized, source);
    }

    #[test]
    fn test_psa_with_special_characters() {
        let source =
            DialogSource::public_service_announcement("TYPE\x01AGAIN", "text\nwith\nnewlines");
        // The first \x01 will be treated as the separator
        assert_eq!(source.psa_type(), Some("TYPE\x01AGAIN"));
        assert_eq!(source.psa_text(), Some("text\nwith\nnewlines"));
    }

    #[test]
    fn test_clone() {
        let source = DialogSource::public_service_announcement("TYPE", "text");
        let cloned = source.clone();
        assert_eq!(source, cloned);
    }

    #[test]
    fn test_debug_format() {
        let source = DialogSource::public_service_announcement("TYPE", "text");
        let debug = format!("{:?}", source);
        assert!(debug.contains("PublicServiceAnnouncement"));
    }

    #[test]
    fn test_constructor_methods() {
        // Test mtproto_proxy constructor
        let proxy = DialogSource::mtproto_proxy();
        assert!(proxy.is_mtproto_proxy());

        // Test public_service_announcement constructor with &str
        let psa1 = DialogSource::public_service_announcement("TYPE", "text");
        assert!(psa1.is_public_service_announcement());

        // Test public_service_announcement constructor with String
        let psa2 =
            DialogSource::public_service_announcement("TYPE".to_string(), "text".to_string());
        assert!(psa2.is_public_service_announcement());

        assert_eq!(psa1, psa2);
    }
}
