// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Channel type enumeration.
//!
//! This module implements TDLib's ChannelType from `td/telegram/ChannelType.h`.
//!
//! # Example
//!
//! ```rust
//! use rustgram_channel_type::ChannelType;
//!
//! let channel_type = ChannelType::Broadcast;
//! assert_eq!(channel_type.name(), "channel");
//!
//! let megagroup = ChannelType::Megagroup;
//! assert_eq!(megagroup.name(), "supergroup");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use std::fmt::{self, Display, Formatter};

/// Channel type for Telegram channels and supergroups.
///
/// Based on TDLib's `ChannelType` enum from `td/telegram/ChannelType.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_channel_type::ChannelType;
///
/// let broadcast = ChannelType::Broadcast;
/// assert!(broadcast.is_broadcast());
/// assert!(!broadcast.is_megagroup());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ChannelType {
    /// Broadcast channel (one-to-many)
    Broadcast = 0,

    /// Megagroup (supergroup, many-to-many)
    Megagroup = 1,

    /// Unknown channel type
    #[default]
    Unknown = 2,
}

impl ChannelType {
    /// Creates a new ChannelType from a u8 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(ChannelType)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_type::ChannelType;
    ///
    /// assert_eq!(ChannelType::from_u8(0), Some(ChannelType::Broadcast));
    /// assert_eq!(ChannelType::from_u8(1), Some(ChannelType::Megagroup));
    /// assert_eq!(ChannelType::from_u8(99), None);
    /// ```
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Broadcast),
            1 => Some(Self::Megagroup),
            2 => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Returns the u8 representation of this channel type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_type::ChannelType;
    ///
    /// assert_eq!(ChannelType::Broadcast.to_u8(), 0);
    /// assert_eq!(ChannelType::Megagroup.to_u8(), 1);
    /// ```
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// Returns the name of this channel type as used in TDLib.
    ///
    /// # Returns
    ///
    /// * `"channel"` for broadcast channels
    /// * `"supergroup"` for megagroups
    /// * `"unknown"` for unknown types
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_type::ChannelType;
    ///
    /// assert_eq!(ChannelType::Broadcast.name(), "channel");
    /// assert_eq!(ChannelType::Megagroup.name(), "supergroup");
    /// assert_eq!(ChannelType::Unknown.name(), "unknown");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            Self::Broadcast => "channel",
            Self::Megagroup => "supergroup",
            Self::Unknown => "unknown",
        }
    }

    /// Checks if this is a broadcast channel.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a broadcast channel, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_type::ChannelType;
    ///
    /// assert!(ChannelType::Broadcast.is_broadcast());
    /// assert!(!ChannelType::Megagroup.is_broadcast());
    /// ```
    pub fn is_broadcast(self) -> bool {
        matches!(self, Self::Broadcast)
    }

    /// Checks if this is a megagroup (supergroup).
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a megagroup, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_type::ChannelType;
    ///
    /// assert!(ChannelType::Megagroup.is_megagroup());
    /// assert!(!ChannelType::Broadcast.is_megagroup());
    /// ```
    pub fn is_megagroup(self) -> bool {
        matches!(self, Self::Megagroup)
    }

    /// Checks if this channel type is unknown.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is an unknown type, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_type::ChannelType;
    ///
    /// assert!(ChannelType::Unknown.is_unknown());
    /// assert!(!ChannelType::Broadcast.is_unknown());
    /// ```
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(ChannelType::from_u8(0), Some(ChannelType::Broadcast));
        assert_eq!(ChannelType::from_u8(1), Some(ChannelType::Megagroup));
        assert_eq!(ChannelType::from_u8(2), Some(ChannelType::Unknown));
        assert_eq!(ChannelType::from_u8(3), None);
        assert_eq!(ChannelType::from_u8(99), None);
    }

    #[test]
    fn test_to_u8() {
        assert_eq!(ChannelType::Broadcast.to_u8(), 0);
        assert_eq!(ChannelType::Megagroup.to_u8(), 1);
        assert_eq!(ChannelType::Unknown.to_u8(), 2);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for value in 0u8..=2 {
            let channel_type = ChannelType::from_u8(value);
            assert_eq!(channel_type.map(|ct| ct.to_u8()), Some(value));
        }
    }

    #[test]
    fn test_name() {
        assert_eq!(ChannelType::Broadcast.name(), "channel");
        assert_eq!(ChannelType::Megagroup.name(), "supergroup");
        assert_eq!(ChannelType::Unknown.name(), "unknown");
    }

    #[test]
    fn test_is_broadcast() {
        assert!(ChannelType::Broadcast.is_broadcast());
        assert!(!ChannelType::Megagroup.is_broadcast());
        assert!(!ChannelType::Unknown.is_broadcast());
    }

    #[test]
    fn test_is_megagroup() {
        assert!(!ChannelType::Broadcast.is_megagroup());
        assert!(ChannelType::Megagroup.is_megagroup());
        assert!(!ChannelType::Unknown.is_megagroup());
    }

    #[test]
    fn test_is_unknown() {
        assert!(!ChannelType::Broadcast.is_unknown());
        assert!(!ChannelType::Megagroup.is_unknown());
        assert!(ChannelType::Unknown.is_unknown());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ChannelType::Broadcast), "channel");
        assert_eq!(format!("{}", ChannelType::Megagroup), "supergroup");
        assert_eq!(format!("{}", ChannelType::Unknown), "unknown");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", ChannelType::Broadcast), "Broadcast");
        assert_eq!(format!("{:?}", ChannelType::Megagroup), "Megagroup");
        assert_eq!(format!("{:?}", ChannelType::Unknown), "Unknown");
    }

    #[test]
    fn test_default() {
        assert_eq!(ChannelType::default(), ChannelType::Unknown);
    }

    #[test]
    fn test_equality() {
        assert_eq!(ChannelType::Broadcast, ChannelType::Broadcast);
        assert_eq!(ChannelType::Megagroup, ChannelType::Megagroup);
        assert_ne!(ChannelType::Broadcast, ChannelType::Megagroup);
        assert_ne!(ChannelType::Broadcast, ChannelType::Unknown);
    }

    #[test]
    fn test_copy() {
        let a = ChannelType::Broadcast;
        let b = a;
        assert_eq!(a, ChannelType::Broadcast);
        assert_eq!(b, ChannelType::Broadcast);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ChannelType::Broadcast);
        set.insert(ChannelType::Megagroup);
        set.insert(ChannelType::Unknown);
        assert_eq!(set.len(), 3);
    }
}
