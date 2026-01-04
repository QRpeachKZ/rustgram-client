// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Network statistics and types.
//!
//! This module implements TDLib's network statistics tracking from
//! `td/telegram/net/NetType.h` and `td/telegram/net/NetStatsManager.h`.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Network type.
///
/// Based on TDLib's NetType from `td/telegram/net/NetType.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum NetType {
    /// Other network type
    #[default]
    Other = 0,

    /// WiFi network
    WiFi = 1,

    /// Mobile network
    Mobile = 2,

    /// Mobile roaming
    MobileRoaming = 3,

    /// Size (sentinel value)
    Size = 4,

    /// None (no network)
    None = 5,

    /// Unknown network type
    Unknown = 6,
}

impl NetType {
    /// Returns `true` if this is a mobile network type.
    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::Mobile | Self::MobileRoaming)
    }

    /// Returns `true` if this is a roaming connection.
    pub fn is_roaming(&self) -> bool {
        matches!(self, Self::MobileRoaming)
    }

    /// Returns the string representation for database keys.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Other => "other",
            Self::WiFi => "wifi",
            Self::Mobile => "mobile",
            Self::MobileRoaming => "mobile_roaming",
            Self::Size => "size",
            Self::None => "none",
            Self::Unknown => "unknown",
        }
    }
}

impl FromStr for NetType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "other" => Ok(Self::Other),
            "wifi" => Ok(Self::WiFi),
            "mobile" => Ok(Self::Mobile),
            "mobile_roaming" => Ok(Self::MobileRoaming),
            "none" => Ok(Self::None),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown NetType: {}", s)),
        }
    }
}

/// File type for statistics.
///
/// Corresponds to TDLib's FileType enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum FileType {
    /// Unknown file type
    #[default]
    None = -1,

    /// Photo
    Photo = 0,

    /// Avatar
    ProfilePhoto = 1,

    /// Video
    Video = 2,

    /// Audio
    Audio = 3,

    /// Voice message
    Voice = 4,

    /// Video message
    VideoNote = 5,

    /// Document
    Document = 6,

    /// Sticker
    Sticker = 7,

    /// Animated sticker (TGS)
    AnimatedSticker = 8,

    /// Custom emoji
    CustomEmoji = 9,

    /// Background
    Background = 10,

    /// Encrypted file
    Secure = 20,

    /// Secure decrypted file
    SecureDecrypted = 21,
}

impl FileType {
    /// Returns the unique name for this file type.
    pub fn unique_name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Photo => "photo",
            Self::ProfilePhoto => "profilephoto",
            Self::Video => "video",
            Self::Audio => "audio",
            Self::Voice => "voice",
            Self::VideoNote => "videonote",
            Self::Document => "document",
            Self::Sticker => "sticker",
            Self::AnimatedSticker => "animatedsticker",
            Self::CustomEmoji => "customemoji",
            Self::Background => "background",
            Self::Secure => "secure",
            Self::SecureDecrypted => "securederypted",
        }
    }
}

/// Network statistics entry.
///
/// Based on TDLib's NetworkStatsEntry from `td/telegram/net/NetStatsManager.h`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStatsEntry {
    /// File type
    pub file_type: FileType,

    /// Network type
    pub net_type: NetType,

    /// Bytes received
    pub rx: i64,

    /// Bytes sent
    pub tx: i64,

    /// Whether this is a call statistics entry
    pub is_call: bool,

    /// Call count (for call statistics)
    pub count: i64,

    /// Call duration in seconds (for call statistics)
    pub duration: f64,
}

impl Default for NetworkStatsEntry {
    fn default() -> Self {
        Self {
            file_type: FileType::None,
            net_type: NetType::Other,
            rx: 0,
            tx: 0,
            is_call: false,
            count: 0,
            duration: 0.0,
        }
    }
}

impl NetworkStatsEntry {
    /// Creates a new network statistics entry.
    pub fn new(file_type: FileType, net_type: NetType) -> Self {
        Self {
            file_type,
            net_type,
            ..Default::default()
        }
    }

    /// Creates a file statistics entry.
    pub fn file(file_type: FileType, net_type: NetType, rx: i64, tx: i64) -> Self {
        Self {
            file_type,
            net_type,
            rx,
            tx,
            is_call: false,
            count: 0,
            duration: 0.0,
        }
    }

    /// Creates a call statistics entry.
    pub fn call(net_type: NetType, tx: i64, rx: i64, count: i64, duration: f64) -> Self {
        Self {
            file_type: FileType::None,
            net_type,
            rx,
            tx,
            is_call: true,
            count,
            duration,
        }
    }

    /// Adds bytes to this entry.
    pub fn add_bytes(&mut self, rx: i64, tx: i64) {
        self.rx = self.rx.saturating_add(rx);
        self.tx = self.tx.saturating_add(tx);
    }

    /// Returns the total bytes (rx + tx).
    pub fn total_bytes(&self) -> i64 {
        self.rx.saturating_add(self.tx)
    }

    /// Returns `true` if there's any data recorded.
    pub fn has_data(&self) -> bool {
        self.rx > 0 || self.tx > 0 || (self.is_call && self.count > 0)
    }

    /// Resets this entry.
    pub fn reset(&mut self) {
        self.rx = 0;
        self.tx = 0;
        self.count = 0;
        self.duration = 0.0;
    }
}

/// Network statistics collection.
///
/// Based on TDLib's NetworkStats from `td/telegram/net/NetStatsManager.h`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkStats {
    /// Unix timestamp when stats collection started
    pub since: i32,

    /// Statistics entries
    pub entries: Vec<NetworkStatsEntry>,
}

impl NetworkStats {
    /// Creates new network statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if there are no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Adds a statistics entry.
    pub fn add_entry(&mut self, entry: NetworkStatsEntry) {
        self.entries.push(entry);
    }

    /// Finds or creates an entry for the given file type and network type.
    pub fn entry(&mut self, file_type: FileType, net_type: NetType) -> &mut NetworkStatsEntry {
        // Find existing entry
        if let Some(idx) = self
            .entries
            .iter()
            .position(|e| e.file_type == file_type && e.net_type == net_type)
        {
            return &mut self.entries[idx];
        }

        // Create new entry
        self.entries
            .push(NetworkStatsEntry::new(file_type, net_type));
        self.entries
            .last_mut()
            .expect("entry was just pushed, so it exists")
    }

    /// Adds bytes to an entry.
    pub fn add_bytes(&mut self, file_type: FileType, net_type: NetType, rx: i64, tx: i64) {
        let entry = self.entry(file_type, net_type);
        entry.add_bytes(rx, tx);
    }

    /// Resets all statistics.
    pub fn reset(&mut self) {
        self.entries.clear();
        self.since = 0;
    }

    /// Filters out entries without data.
    pub fn filter_active(&self) -> Vec<NetworkStatsEntry> {
        self.entries
            .iter()
            .filter(|e| e.has_data() && e.file_type != FileType::SecureDecrypted)
            .cloned()
            .collect()
    }

    /// Returns total bytes across all entries.
    pub fn total_bytes(&self) -> i64 {
        self.entries.iter().map(|e| e.total_bytes()).sum()
    }
}

/// Network statistics manager.
///
/// Based on TDLib's NetStatsManager from `td/telegram/net/NetStatsManager.h`.
#[derive(Debug, Clone)]
pub struct NetStatsManager {
    /// Current network type
    net_type: NetType,

    /// Statistics by category
    common_stats: NetworkStatsEntry,
    media_stats: NetworkStatsEntry,
    file_stats: Vec<NetworkStatsEntry>,
    call_stats: NetworkStatsEntry,
}

impl Default for NetStatsManager {
    fn default() -> Self {
        Self {
            net_type: NetType::Other,
            common_stats: NetworkStatsEntry::new(FileType::None, NetType::Other),
            media_stats: NetworkStatsEntry::new(FileType::None, NetType::Other),
            file_stats: Vec::new(),
            call_stats: NetworkStatsEntry::new(FileType::None, NetType::Other),
        }
    }
}

impl NetStatsManager {
    /// Creates a new network statistics manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current network type.
    pub fn net_type(&self) -> NetType {
        self.net_type
    }

    /// Sets the current network type.
    pub fn set_net_type(&mut self, net_type: NetType) {
        self.net_type = net_type;
    }

    /// Returns the common statistics callback data.
    pub fn common_stats(&self) -> NetworkStatsEntry {
        self.common_stats.clone()
    }

    /// Returns the media statistics callback data.
    pub fn media_stats(&self) -> NetworkStatsEntry {
        self.media_stats.clone()
    }

    /// Returns statistics for a specific file type.
    pub fn file_stats(&self, file_type: FileType) -> Option<NetworkStatsEntry> {
        self.file_stats
            .iter()
            .find(|e| e.file_type == file_type)
            .cloned()
    }

    /// Returns the call statistics.
    pub fn call_stats(&self) -> NetworkStatsEntry {
        self.call_stats.clone()
    }

    /// Adds network statistics.
    pub fn add_stats(&mut self, entry: &NetworkStatsEntry) {
        match entry.file_type {
            FileType::None => {
                if entry.is_call {
                    self.call_stats.add_bytes(entry.rx, entry.tx);
                } else {
                    self.common_stats.add_bytes(entry.rx, entry.tx);
                }
            }
            FileType::Photo
            | FileType::ProfilePhoto
            | FileType::Video
            | FileType::Voice
            | FileType::VideoNote => {
                self.media_stats.add_bytes(entry.rx, entry.tx);
            }
            _ => {
                // Find or create file stats entry
                if let Some(stats) = self
                    .file_stats
                    .iter_mut()
                    .find(|e| e.file_type == entry.file_type)
                {
                    stats.add_bytes(entry.rx, entry.tx);
                } else {
                    let mut stats = entry.clone();
                    stats.rx = 0;
                    stats.tx = 0;
                    stats.add_bytes(entry.rx, entry.tx);
                    self.file_stats.push(stats);
                }
            }
        }
    }

    /// Generates network statistics snapshot.
    pub fn get_network_stats(&self) -> NetworkStats {
        let mut stats = NetworkStats::new();

        if self.common_stats.has_data() {
            stats.entries.push(self.common_stats.clone());
        }

        if self.media_stats.has_data() {
            stats.entries.push(self.media_stats.clone());
        }

        for file_stats in &self.file_stats {
            if file_stats.has_data() {
                stats.entries.push(file_stats.clone());
            }
        }

        if self.call_stats.has_data() {
            stats.entries.push(self.call_stats.clone());
        }

        stats
    }

    /// Resets all statistics.
    pub fn reset(&mut self) {
        self.common_stats.reset();
        self.media_stats.reset();
        self.file_stats.clear();
        self.call_stats.reset();
    }
}

/// Callback for tracking network statistics.
///
/// Based on TDLib's NetStatsCallback from `td/net/NetStats.h`.
#[derive(Debug, Clone)]
pub struct NetStatsCallback {
    /// Statistics entry
    entry: NetworkStatsEntry,

    /// Callback ID
    id: usize,
}

impl NetStatsCallback {
    /// Creates a new stats callback.
    pub fn new(file_type: FileType, net_type: NetType, id: usize) -> Self {
        Self {
            entry: NetworkStatsEntry::new(file_type, net_type),
            id,
        }
    }

    /// Returns the callback ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Adds bytes to the statistics.
    pub fn add_bytes(&mut self, rx: i64, tx: i64) {
        self.entry.add_bytes(rx, tx);
    }

    /// Returns the current statistics.
    pub fn entry(&self) -> NetworkStatsEntry {
        self.entry.clone()
    }

    /// Resets the statistics.
    pub fn reset(&mut self) {
        self.entry.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_type() {
        assert_eq!(NetType::Other.as_str(), "other");
        assert_eq!(NetType::WiFi.as_str(), "wifi");
        assert_eq!(NetType::Mobile.as_str(), "mobile");

        assert!(NetType::Mobile.is_mobile());
        assert!(NetType::MobileRoaming.is_mobile());
        assert!(!NetType::WiFi.is_mobile());

        assert!(NetType::MobileRoaming.is_roaming());
        assert!(!NetType::Mobile.is_roaming());
    }

    #[test]
    fn test_net_type_from_str() {
        assert_eq!(NetType::from_str("wifi"), Ok(NetType::WiFi));
        assert_eq!(NetType::from_str("mobile"), Ok(NetType::Mobile));
        assert!(NetType::from_str("invalid").is_err());
    }

    #[test]
    fn test_file_type() {
        assert_eq!(FileType::Photo.unique_name(), "photo");
        assert_eq!(FileType::Video.unique_name(), "video");
        assert_eq!(FileType::Audio.unique_name(), "audio");
    }

    #[test]
    fn test_network_stats_entry() {
        let mut entry = NetworkStatsEntry::file(FileType::Photo, NetType::WiFi, 1000, 500);

        assert_eq!(entry.rx, 1000);
        assert_eq!(entry.tx, 500);
        assert_eq!(entry.total_bytes(), 1500);
        assert!(entry.has_data());

        entry.add_bytes(500, 250);
        assert_eq!(entry.rx, 1500);
        assert_eq!(entry.tx, 750);

        entry.reset();
        assert_eq!(entry.rx, 0);
        assert_eq!(entry.tx, 0);
        assert!(!entry.has_data());
    }

    #[test]
    fn test_network_stats() {
        let mut stats = NetworkStats::new();
        assert!(stats.is_empty());
        assert_eq!(stats.len(), 0);

        stats.add_bytes(FileType::Photo, NetType::WiFi, 1000, 500);
        stats.add_bytes(FileType::Video, NetType::WiFi, 2000, 1000);

        assert!(!stats.is_empty());
        assert_eq!(stats.len(), 2);
        assert_eq!(stats.total_bytes(), 4500);

        let active = stats.filter_active();
        assert_eq!(active.len(), 2);

        stats.reset();
        assert!(stats.is_empty());
    }

    #[test]
    fn test_net_stats_manager() {
        let mut manager = NetStatsManager::new();

        manager.add_stats(&NetworkStatsEntry::file(
            FileType::Photo,
            NetType::WiFi,
            1000,
            500,
        ));

        // Photo goes to media_stats (Photo, Video, Voice, VideoNote are media)
        let media = manager.media_stats();
        assert_eq!(media.rx, 1000);
        assert_eq!(media.tx, 500);

        let snapshot = manager.get_network_stats();
        assert_eq!(snapshot.len(), 1);

        manager.reset();
        let snapshot = manager.get_network_stats();
        assert!(snapshot.is_empty());
    }

    #[test]
    fn test_net_stats_callback() {
        let mut callback = NetStatsCallback::new(FileType::Video, NetType::Mobile, 1);

        callback.add_bytes(100, 50);
        let entry = callback.entry();
        assert_eq!(entry.rx, 100);
        assert_eq!(entry.tx, 50);

        assert_eq!(callback.id(), 1);

        callback.reset();
        let entry = callback.entry();
        assert!(!entry.has_data());
    }

    #[test]
    fn test_call_stats() {
        let entry = NetworkStatsEntry::call(NetType::WiFi, 1000, 2000, 5, 300.0);

        assert!(entry.is_call);
        assert_eq!(entry.count, 5);
        assert_eq!(entry.duration, 300.0);
        assert_eq!(entry.rx, 2000);
        assert_eq!(entry.tx, 1000);
    }
}
