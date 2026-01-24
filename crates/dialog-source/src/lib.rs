//! Dialog source information.
//!
//! This module provides the `DialogSource` type, which describes how a dialog
//! was added to the user's chat list.
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogSource` (td/telegram/DialogSource.h)
//! - Three types: Membership, MtprotoProxy, PublicServiceAnnouncement
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_source::DialogSource;
//!
//! // Default membership source
//! let source = DialogSource::membership();
//! assert!(source.is_membership());
//!
//! // Public service announcement
//! let psa = DialogSource::public_service_announcement("update", "Update your app");
//! assert!(psa.is_psa());
//! assert_eq!(psa.psa_type(), Some("update"));
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Dialog source type.
///
/// Describes how a dialog was added to the user's chat list.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_source::DialogSource;
///
/// let membership = DialogSource::membership();
/// assert!(matches!(membership, DialogSource::Membership));
///
/// let proxy = DialogSource::mtproto_proxy();
/// assert!(matches!(proxy, DialogSource::MtprotoProxy));
///
/// let psa = DialogSource::public_service_announcement("tips", "Use Telegram wisely");
/// assert!(matches!(psa, DialogSource::PublicServiceAnnouncement { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogSource {
    /// Dialog was added through normal membership (user joined or was added).
    Membership,
    /// Dialog was added through an MTProto proxy.
    MtprotoProxy,
    /// Dialog was added as a public service announcement.
    PublicServiceAnnouncement {
        /// Type of the PSA (e.g., "tips", "update").
        psa_type: String,
        /// Text content of the PSA.
        psa_text: String,
    },
}

impl DialogSource {
    /// Creates a membership dialog source.
    ///
    /// This is the default for most dialogs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::membership();
    /// assert!(source.is_membership());
    /// ```
    pub fn membership() -> Self {
        Self::Membership
    }

    /// Creates an MTProto proxy dialog source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::mtproto_proxy();
    /// assert!(source.is_mtproto_proxy());
    /// ```
    pub fn mtproto_proxy() -> Self {
        Self::MtprotoProxy
    }

    /// Creates a public service announcement dialog source.
    ///
    /// # Arguments
    ///
    /// * `psa_type` - Type of the PSA
    /// * `psa_text` - Text content of the PSA
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::public_service_announcement("tips", "Use Telegram");
    /// assert!(source.is_psa());
    /// assert_eq!(source.psa_type(), Some("tips"));
    /// ```
    pub fn public_service_announcement<S: Into<String>>(psa_type: S, psa_text: S) -> Self {
        Self::PublicServiceAnnouncement {
            psa_type: psa_type.into(),
            psa_text: psa_text.into(),
        }
    }

    /// Checks if this is a membership source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::membership();
    /// assert!(source.is_membership());
    /// ```
    pub fn is_membership(&self) -> bool {
        matches!(self, Self::Membership)
    }

    /// Checks if this is an MTProto proxy source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::mtproto_proxy();
    /// assert!(source.is_mtproto_proxy());
    /// ```
    pub fn is_mtproto_proxy(&self) -> bool {
        matches!(self, Self::MtprotoProxy)
    }

    /// Checks if this is a public service announcement source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::public_service_announcement("tips", "text");
    /// assert!(source.is_psa());
    /// ```
    pub fn is_psa(&self) -> bool {
        matches!(self, Self::PublicServiceAnnouncement { .. })
    }

    /// Gets the PSA type if this is a PSA source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::public_service_announcement("update", "Update now");
    /// assert_eq!(source.psa_type(), Some("update"));
    ///
    /// let membership = DialogSource::membership();
    /// assert_eq!(membership.psa_type(), None);
    /// ```
    pub fn psa_type(&self) -> Option<&str> {
        match self {
            Self::PublicServiceAnnouncement { psa_type, .. } => Some(psa_type),
            _ => None,
        }
    }

    /// Gets the PSA text if this is a PSA source.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::public_service_announcement("tips", "Be safe");
    /// assert_eq!(source.psa_text(), Some("Be safe"));
    /// ```
    pub fn psa_text(&self) -> Option<&str> {
        match self {
            Self::PublicServiceAnnouncement { psa_text, .. } => Some(psa_text),
            _ => None,
        }
    }

    /// Serializes the dialog source to a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::membership();
    /// let serialized = source.serialize();
    /// assert!(!serialized.is_empty());
    /// ```
    pub fn serialize(&self) -> String {
        match self {
            Self::Membership => "membership".to_string(),
            Self::MtprotoProxy => "mtproto_proxy".to_string(),
            Self::PublicServiceAnnouncement { psa_type, psa_text } => {
                format!("psa:{}:{}", psa_type, psa_text)
            }
        }
    }

    /// Deserializes a dialog source from a string.
    ///
    /// Returns None if the string format is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_source::DialogSource;
    ///
    /// let source = DialogSource::membership();
    /// let serialized = source.serialize();
    /// let deserialized = DialogSource::unserialize(&serialized).unwrap();
    /// assert_eq!(source, deserialized);
    /// ```
    pub fn unserialize(s: &str) -> Option<Self> {
        if s == "membership" {
            Some(Self::Membership)
        } else if s == "mtproto_proxy" {
            Some(Self::MtprotoProxy)
        } else if s.starts_with("psa:") {
            let rest = &s[4..];
            if let Some(colon_pos) = rest.find(':') {
                let psa_type = rest[..colon_pos].to_string();
                let psa_text = rest[colon_pos + 1..].to_string();
                Some(Self::PublicServiceAnnouncement { psa_type, psa_text })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for DialogSource {
    fn default() -> Self {
        Self::Membership
    }
}

impl fmt::Display for DialogSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Membership => write!(f, "membership"),
            Self::MtprotoProxy => write!(f, "mtproto proxy"),
            Self::PublicServiceAnnouncement { psa_type, .. } => {
                write!(f, "public service announcement ({})", psa_type)
            }
        }
    }
}

impl Serialize for DialogSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as tagged enum
        match self {
            Self::Membership => (0u8, "").serialize(serializer),
            Self::MtprotoProxy => (1u8, "").serialize(serializer),
            Self::PublicServiceAnnouncement { psa_type, psa_text } => {
                (2u8, psa_type, psa_text).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for DialogSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DialogSourceVisitor;

        impl<'de> serde::de::Visitor<'de> for DialogSourceVisitor {
            type Value = DialogSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a dialog source tuple")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let tag: u8 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(match tag {
                    0 => DialogSource::Membership,
                    1 => DialogSource::MtprotoProxy,
                    2 => {
                        let psa_type: String = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        let psa_text: String = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                        DialogSource::PublicServiceAnnouncement { psa_type, psa_text }
                    }
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid dialog source tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(DialogSourceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug_membership() {
        let source = DialogSource::membership();
        assert_eq!(format!("{:?}", source), "Membership");
    }

    #[test]
    fn test_debug_mtproto_proxy() {
        let source = DialogSource::mtproto_proxy();
        assert_eq!(format!("{:?}", source), "MtprotoProxy");
    }

    #[test]
    fn test_debug_psa() {
        let source = DialogSource::public_service_announcement("tips", "text");
        assert!(format!("{:?}", source).contains("PublicServiceAnnouncement"));
    }

    #[test]
    fn test_clone() {
        let source = DialogSource::membership();
        let cloned = source.clone();
        assert_eq!(source, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let s1 = DialogSource::membership();
        let s2 = DialogSource::membership();
        let s3 = DialogSource::mtproto_proxy();
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn test_display_membership() {
        let source = DialogSource::membership();
        assert_eq!(format!("{}", source), "membership");
    }

    #[test]
    fn test_display_mtproto_proxy() {
        let source = DialogSource::mtproto_proxy();
        assert_eq!(format!("{}", source), "mtproto proxy");
    }

    #[test]
    fn test_display_psa() {
        let source = DialogSource::public_service_announcement("tips", "text");
        assert!(format!("{}", source).contains("tips"));
    }

    #[test]
    fn test_default() {
        let source = DialogSource::default();
        assert!(source.is_membership());
    }

    // Constructor tests (3 constructors * 2 tests = 6)
    #[test]
    fn test_membership() {
        let source = DialogSource::membership();
        assert!(source.is_membership());
    }

    #[test]
    fn test_mtproto_proxy() {
        let source = DialogSource::mtproto_proxy();
        assert!(source.is_mtproto_proxy());
    }

    #[test]
    fn test_psa_construction() {
        let source = DialogSource::public_service_announcement("tips", "text");
        assert!(source.is_psa());
        assert_eq!(source.psa_type(), Some("tips"));
        assert_eq!(source.psa_text(), Some("text"));
    }

    #[test]
    fn test_psa_with_str() {
        let source = DialogSource::public_service_announcement("update", "Update now");
        assert!(source.is_psa());
    }

    #[test]
    fn test_psa_with_string() {
        let psa_type = String::from("help");
        let psa_text = String::from("Need help?");
        let source = DialogSource::public_service_announcement(psa_type, psa_text);
        assert_eq!(source.psa_type(), Some("help"));
    }

    // Method tests (8 methods * 3 tests = 24)
    #[test]
    fn test_is_membership_true() {
        let source = DialogSource::membership();
        assert!(source.is_membership());
        assert!(!source.is_mtproto_proxy());
        assert!(!source.is_psa());
    }

    #[test]
    fn test_is_mtproto_proxy_true() {
        let source = DialogSource::mtproto_proxy();
        assert!(source.is_mtproto_proxy());
        assert!(!source.is_membership());
        assert!(!source.is_psa());
    }

    #[test]
    fn test_is_psa_true() {
        let source = DialogSource::public_service_announcement("tips", "text");
        assert!(source.is_psa());
        assert!(!source.is_membership());
        assert!(!source.is_mtproto_proxy());
    }

    #[test]
    fn test_psa_type_some() {
        let source = DialogSource::public_service_announcement("update", "text");
        assert_eq!(source.psa_type(), Some("update"));
    }

    #[test]
    fn test_psa_type_none() {
        let source = DialogSource::membership();
        assert_eq!(source.psa_type(), None);
    }

    #[test]
    fn test_psa_text_some() {
        let source = DialogSource::public_service_announcement("tips", "Be safe");
        assert_eq!(source.psa_text(), Some("Be safe"));
    }

    #[test]
    fn test_psa_text_none() {
        let source = DialogSource::mtproto_proxy();
        assert_eq!(source.psa_text(), None);
    }

    #[test]
    fn test_serialize_membership() {
        let source = DialogSource::membership();
        assert_eq!(source.serialize(), "membership");
    }

    #[test]
    fn test_serialize_mtproto_proxy() {
        let source = DialogSource::mtproto_proxy();
        assert_eq!(source.serialize(), "mtproto_proxy");
    }

    #[test]
    fn test_serialize_psa() {
        let source = DialogSource::public_service_announcement("tips", "text");
        assert_eq!(source.serialize(), "psa:tips:text");
    }

    #[test]
    fn test_unserialize_membership() {
        let source = DialogSource::unserialize("membership").unwrap();
        assert_eq!(source, DialogSource::Membership);
    }

    #[test]
    fn test_unserialize_mtproto_proxy() {
        let source = DialogSource::unserialize("mtproto_proxy").unwrap();
        assert_eq!(source, DialogSource::MtprotoProxy);
    }

    #[test]
    fn test_unserialize_psa() {
        let source = DialogSource::unserialize("psa:tips:text").unwrap();
        match source {
            DialogSource::PublicServiceAnnouncement { psa_type, psa_text } => {
                assert_eq!(psa_type, "tips");
                assert_eq!(psa_text, "text");
            }
            _ => panic!("Expected PSA"),
        }
    }

    #[test]
    fn test_unserialize_invalid() {
        assert!(DialogSource::unserialize("invalid").is_none());
    }

    #[test]
    fn test_unserialize_malformed_psa() {
        assert!(DialogSource::unserialize("psa:only_type").is_none());
    }

    #[test]
    fn test_serialize_roundtrip() {
        let sources = vec![
            DialogSource::membership(),
            DialogSource::mtproto_proxy(),
            DialogSource::public_service_announcement("tips", "text"),
        ];
        for source in sources {
            let serialized = source.serialize();
            let deserialized = DialogSource::unserialize(&serialized).unwrap();
            assert_eq!(source, deserialized);
        }
    }

    // JSON serialization tests (2)
    #[test]
    fn test_json_serialize_deserialize() {
        let source = DialogSource::public_service_announcement("update", "text");
        let json = serde_json::to_string(&source).unwrap();
        let deserialized: DialogSource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, deserialized);
    }

    #[test]
    fn test_json_all_variants() {
        let sources = vec![
            DialogSource::membership(),
            DialogSource::mtproto_proxy(),
            DialogSource::public_service_announcement("tips", "text"),
        ];
        for source in sources {
            let json = serde_json::to_string(&source).unwrap();
            let deserialized: DialogSource = serde_json::from_str(&json).unwrap();
            assert_eq!(source, deserialized);
        }
    }

    // Doc test example (1)
    #[test]
    fn test_doc_example() {
        let source = DialogSource::membership();
        assert!(source.is_membership());

        let psa = DialogSource::public_service_announcement("update", "Update your app");
        assert!(psa.is_psa());
        assert_eq!(psa.psa_type(), Some("update"));
    }
}
