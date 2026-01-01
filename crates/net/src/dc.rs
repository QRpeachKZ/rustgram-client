// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Data Center (DC) management types.
//!
//! This module implements TDLib's DC identification and options system.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};

/// Maximum raw DC ID value
pub const MAX_RAW_DC_ID: i32 = 1000;

/// Special DC ID values
const DC_ID_EMPTY: i32 = 0;
const DC_ID_MAIN: i32 = -1;
const DC_ID_INVALID: i32 = -2;

/// Data Center identifier.
///
/// Based on TDLib's DcId class from `td/telegram/net/DcId.h`.
///
/// # Examples
///
/// ```
/// use rustgram_net::DcId;
///
/// // Create DC ID for internal DC
/// let dc = DcId::internal(2);
/// assert_eq!(dc.get_raw_id(), 2);
/// assert!(!dc.is_external());
///
/// // Create DC ID for external (CDN) DC
/// let dc_cdn = DcId::external(2);
/// assert_eq!(dc_cdn.get_raw_id(), 2);
/// assert!(dc_cdn.is_external());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DcId {
    dc_id: i32,
    is_external: bool,
}

impl Default for DcId {
    fn default() -> Self {
        Self::empty()
    }
}

impl DcId {
    /// Creates an empty DC ID.
    #[inline]
    pub fn empty() -> Self {
        Self {
            dc_id: DC_ID_EMPTY,
            is_external: false,
        }
    }

    /// Creates the main DC ID.
    #[inline]
    pub fn main() -> Self {
        Self {
            dc_id: DC_ID_MAIN,
            is_external: false,
        }
    }

    /// Creates an invalid DC ID.
    #[inline]
    pub fn invalid() -> Self {
        Self {
            dc_id: DC_ID_INVALID,
            is_external: false,
        }
    }

    /// Creates an internal DC ID.
    ///
    /// # Panics
    ///
    /// Panics if `id` is not a valid DC ID (1..=MAX_RAW_DC_ID).
    #[inline]
    pub fn internal(id: i32) -> Self {
        assert!(
            Self::is_valid(id),
            "DC ID {} is not valid (must be 1..={MAX_RAW_DC_ID})",
            id
        );
        Self {
            dc_id: id,
            is_external: false,
        }
    }

    /// Creates an external (CDN) DC ID.
    ///
    /// # Panics
    ///
    /// Panics if `id` is not a valid DC ID.
    #[inline]
    pub fn external(id: i32) -> Self {
        assert!(
            Self::is_valid(id),
            "DC ID {} is not valid (must be 1..={MAX_RAW_DC_ID})",
            id
        );
        Self {
            dc_id: id,
            is_external: true,
        }
    }

    /// Creates a DC ID from a value without validation.
    ///
    /// This is used when reading from storage.
    #[inline]
    pub fn from_value(value: i32) -> Self {
        Self {
            dc_id: value,
            is_external: false,
        }
    }

    /// Creates a DC ID safely, returning invalid if the ID is out of range.
    #[inline]
    pub fn create(dc_id_value: i32) -> Self {
        if Self::is_valid(dc_id_value) {
            Self {
                dc_id: dc_id_value,
                is_external: false,
            }
        } else {
            Self::invalid()
        }
    }

    /// Checks if a DC ID value is valid.
    #[inline]
    pub fn is_valid(id: i32) -> bool {
        (1..=MAX_RAW_DC_ID).contains(&id)
    }

    /// Returns `true` if this is an empty DC ID.
    #[inline]
    pub fn is_empty(&self) -> bool {
        !Self::is_valid(self.dc_id)
    }

    /// Returns `true` if this is the main DC ID.
    #[inline]
    pub fn is_main(&self) -> bool {
        self.dc_id == DC_ID_MAIN
    }

    /// Returns the raw numeric DC ID.
    ///
    /// # Panics
    ///
    /// Panics if this is not an exact DC ID (i.e., not empty, main, or invalid).
    #[inline]
    pub fn get_raw_id(&self) -> i32 {
        assert!(
            self.is_exact(),
            "Cannot get raw ID from non-exact DC ID: {:?}",
            self
        );
        self.dc_id
    }

    /// Returns the DC ID value (may be special values).
    #[inline]
    pub fn get_value(&self) -> i32 {
        self.dc_id
    }

    /// Returns `true` if this is an internal (non-CDN) DC.
    #[inline]
    pub fn is_internal(&self) -> bool {
        !self.is_external()
    }

    /// Returns `true` if this is an external (CDN) DC.
    #[inline]
    pub fn is_external(&self) -> bool {
        self.is_external
    }

    /// Returns `true` if this is an exact DC ID (has a numeric value).
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.dc_id > 0
    }
}

impl fmt::Display for DcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self {
                dc_id: DC_ID_INVALID,
                ..
            } => write!(f, "DcId(invalid)"),
            Self {
                dc_id: DC_ID_EMPTY, ..
            } => write!(f, "DcId(empty)"),
            Self {
                dc_id: DC_ID_MAIN, ..
            } => write!(f, "DcId(main)"),
            _ => {
                if self.is_external() {
                    write!(f, "DcId({} external)", self.dc_id)
                } else {
                    write!(f, "DcId({})", self.dc_id)
                }
            }
        }
    }
}

/// Error type for DC operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum DcError {
    /// Invalid DC ID value
    #[error("Invalid DC ID: {0}")]
    InvalidId(i32),

    /// Invalid IP address
    #[error("Invalid IP address: {0}")]
    InvalidIp(String),

    /// Missing DC option for specified DC
    #[error("No DC option found for DC {0:?}")]
    DcNotFound(DcId),
}

/// Flags for DcOption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum DcOptionFlag {
    /// IPv6 address
    IPv6 = 1,
    /// Media-only DC (for large file downloads)
    MediaOnly = 2,
    /// Obfuscated TCP only
    ObfuscatedTcpOnly = 4,
    /// CDN (Content Delivery Network) DC
    Cdn = 8,
    /// Static DC address
    Static = 16,
    /// Has secret for obfuscation
    HasSecret = 32,
}

/// Data Center connection option.
///
/// Represents a way to connect to a specific Telegram data center.
/// Based on TDLib's DcOption from `td/telegram/net/DcOptions.h`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DcOption {
    /// DC identifier
    pub dc_id: DcId,
    /// IP address
    pub ip_address: IpAddr,
    /// Port number
    pub port: u16,
    /// Option flags
    pub flags: u32,
    /// Secret for obfuscated TCP (if present)
    pub secret: Option<Vec<u8>>,
}

impl DcOption {
    /// Creates a new DC option.
    pub fn new(dc_id: DcId, ip_address: IpAddr, port: u16) -> Self {
        Self {
            dc_id,
            ip_address,
            port,
            flags: 0,
            secret: None,
        }
    }

    /// Returns `true` if this option uses IPv6.
    #[inline]
    pub fn is_ipv6(&self) -> bool {
        (self.flags & DcOptionFlag::IPv6 as u32) != 0
    }

    /// Returns `true` if this is a media-only DC (for large files).
    #[inline]
    pub fn is_media_only(&self) -> bool {
        (self.flags & DcOptionFlag::MediaOnly as u32) != 0
    }

    /// Returns `true` if this DC only supports obfuscated TCP.
    #[inline]
    pub fn is_obfuscated_tcp_only(&self) -> bool {
        (self.flags & DcOptionFlag::ObfuscatedTcpOnly as u32) != 0
    }

    /// Returns `true` if this is a CDN DC.
    #[inline]
    pub fn is_cdn(&self) -> bool {
        (self.flags & DcOptionFlag::Cdn as u32) != 0
    }

    /// Returns `true` if this is a static DC.
    #[inline]
    pub fn is_static(&self) -> bool {
        (self.flags & DcOptionFlag::Static as u32) != 0
    }

    /// Returns `true` if this DC has a secret.
    #[inline]
    pub fn has_secret(&self) -> bool {
        (self.flags & DcOptionFlag::HasSecret as u32) != 0
    }

    /// Returns `true` if this DC option is valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.dc_id.is_exact() && (self.ip_address.is_ipv4() || self.ip_address.is_ipv6())
    }

    /// Returns the socket address for this option.
    #[inline]
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip_address, self.port)
    }

    /// Sets a flag on this option.
    #[inline]
    pub fn with_flag(mut self, flag: DcOptionFlag) -> Self {
        self.flags |= flag as u32;
        self
    }

    /// Sets the secret for obfuscated TCP.
    #[inline]
    pub fn with_secret(mut self, secret: Vec<u8>) -> Self {
        self.flags |= DcOptionFlag::HasSecret as u32;
        self.secret = Some(secret);
        self
    }
}

impl fmt::Display for DcOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DcOption(dc={}, {}:{}",
            self.dc_id, self.ip_address, self.port
        )?;

        let mut flags = Vec::new();
        if self.is_ipv6() {
            flags.push("IPv6");
        }
        if self.is_media_only() {
            flags.push("MediaOnly");
        }
        if self.is_obfuscated_tcp_only() {
            flags.push("ObfuscatedTcpOnly");
        }
        if self.is_cdn() {
            flags.push("CDN");
        }
        if self.is_static() {
            flags.push("Static");
        }
        if self.has_secret() {
            flags.push("HasSecret");
        }

        if !flags.is_empty() {
            write!(f, " [{}]", flags.join(", "))?;
        }

        Ok(())
    }
}

/// Collection of DC options.
///
/// Based on TDLib's DcOptions from `td/telegram/net/DcOptions.h`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DcOptions {
    /// List of DC options
    pub dc_options: Vec<DcOption>,
}

impl DcOptions {
    /// Creates empty DC options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a DC option to the collection.
    pub fn add(&mut self, option: DcOption) {
        if option.is_valid() {
            self.dc_options.push(option);
        }
    }

    /// Returns options for a specific DC ID.
    pub fn get_options(&self, dc_id: DcId) -> Vec<DcOption> {
        self.dc_options
            .iter()
            .filter(|opt| opt.dc_id == dc_id)
            .cloned()
            .collect()
    }

    /// Returns all DC IDs present in this collection.
    pub fn get_dc_ids(&self) -> Vec<DcId> {
        use std::collections::HashSet;
        self.dc_options
            .iter()
            .map(|opt| opt.dc_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Returns the number of options.
    pub fn len(&self) -> usize {
        self.dc_options.len()
    }

    /// Returns `true` if there are no options.
    pub fn is_empty(&self) -> bool {
        self.dc_options.is_empty()
    }
}

/// Statistics for DC options, used for connection selection.
///
/// Based on TDLib's DcOptionsSet from `td/telegram/net/DcOptionsSet.h`.
#[derive(Debug, Clone, Default)]
pub struct DcOptionsSet {
    /// All DC options
    options: DcOptions,
    /// Statistics for each option
    stats: std::collections::HashMap<usize, DcOptionStats>,
}

/// Connection statistics for a DC option.
#[derive(Debug, Clone, Default)]
pub struct DcOptionStats {
    /// Number of successful connections
    pub success_count: u32,
    /// Number of failed connections
    pub failure_count: u32,
    /// Average round-trip time in seconds
    pub avg_rtt: f64,
    /// Last successful connection timestamp
    pub last_success: Option<std::time::Instant>,
}

impl DcOptionStats {
    /// Records a successful connection.
    pub fn record_success(&mut self, rtt: f64) {
        self.success_count += 1;
        self.avg_rtt =
            (self.avg_rtt * (self.success_count - 1) as f64 + rtt) / self.success_count as f64;
        self.last_success = Some(std::time::Instant::now());
    }

    /// Records a failed connection.
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
    }

    /// Returns the success rate (0.0 to 1.0).
    pub fn success_rate(&self) -> f64 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            0.0
        } else {
            self.success_count as f64 / total as f64
        }
    }
}

impl DcOptionsSet {
    /// Creates an empty DC options set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds DC options to the set.
    pub fn add_options(&mut self, options: DcOptions) {
        let _start_idx = self.options.len();
        for option in options.dc_options {
            if option.is_valid() {
                self.options.add(option);
            }
        }
    }

    /// Returns all options.
    pub fn get_options(&self) -> &DcOptions {
        &self.options
    }

    /// Returns options for a specific DC.
    pub fn get_options_for_dc(&self, dc_id: DcId) -> Vec<DcOption> {
        self.options.get_options(dc_id)
    }

    /// Finds the best option for a DC based on statistics.
    pub fn find_best_option(&self, dc_id: DcId, allow_media_only: bool) -> Option<DcOption> {
        let options: Vec<_> = self
            .options
            .dc_options
            .iter()
            .enumerate()
            .filter(|(_, opt)| {
                opt.dc_id == dc_id && (!opt.is_media_only() || allow_media_only) && opt.is_valid()
            })
            .collect();

        if options.is_empty() {
            return None;
        }

        // Sort by success rate and RTT
        let mut options: Vec<_> = options.into_iter().collect();
        options.sort_by(|(idx_a, _opt_a), (idx_b, _opt_b)| {
            let stats_a = self.stats.get(idx_a);
            let stats_b = self.stats.get(idx_b);

            match (stats_a, stats_b) {
                (Some(a), Some(b)) => {
                    // Compare by success rate (higher is better), then by RTT (lower is better)
                    let rate_cmp = b
                        .success_rate()
                        .partial_cmp(&a.success_rate())
                        .unwrap_or(std::cmp::Ordering::Equal);
                    if rate_cmp != std::cmp::Ordering::Equal {
                        rate_cmp
                    } else {
                        a.avg_rtt
                            .partial_cmp(&b.avg_rtt)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    }
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });

        options.first().map(|(_, opt)| (*opt).clone())
    }

    /// Records connection statistics for an option.
    pub fn record_success(&mut self, option_idx: usize, rtt: f64) {
        self.stats
            .entry(option_idx)
            .or_insert_with(DcOptionStats::default)
            .record_success(rtt);
    }

    /// Records a connection failure.
    pub fn record_failure(&mut self, option_idx: usize) {
        self.stats
            .entry(option_idx)
            .or_insert_with(DcOptionStats::default)
            .record_failure();
    }

    /// Returns statistics for an option.
    pub fn get_stats(&self, option_idx: usize) -> Option<&DcOptionStats> {
        self.stats.get(&option_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dc_id_creation() {
        let dc = DcId::internal(2);
        assert_eq!(dc.get_raw_id(), 2);
        assert!(!dc.is_external());
        assert!(dc.is_exact());
        assert!(!dc.is_empty());
        assert!(!dc.is_main());

        let dc_ext = DcId::external(2);
        assert_eq!(dc_ext.get_raw_id(), 2);
        assert!(dc_ext.is_external());
    }

    #[test]
    fn test_dc_id_special() {
        assert!(DcId::empty().is_empty());
        assert!(DcId::main().is_main());
        assert!(DcId::invalid().is_empty());
    }

    #[test]
    fn test_dc_id_validation() {
        assert!(DcId::is_valid(1));
        assert!(DcId::is_valid(1000));
        assert!(!DcId::is_valid(0));
        assert!(!DcId::is_valid(-1));
        assert!(!DcId::is_valid(1001));
    }

    #[test]
    fn test_dc_option() {
        let dc_id = DcId::internal(2);
        let option = DcOption::new(dc_id, IpAddr::V4(Ipv4Addr::new(149, 154, 167, 51)), 443)
            .with_flag(DcOptionFlag::Static);

        assert_eq!(option.dc_id, dc_id);
        assert!(option.is_static());
        assert!(option.is_valid());
        assert!(!option.is_ipv6());
        assert!(!option.is_media_only());
    }

    #[test]
    fn test_dc_options() {
        let mut options = DcOptions::new();
        options.add(DcOption::new(
            DcId::internal(1),
            IpAddr::V4(Ipv4Addr::new(149, 154, 167, 51)),
            443,
        ));
        options.add(DcOption::new(
            DcId::internal(2),
            IpAddr::V4(Ipv4Addr::new(149, 154, 167, 52)),
            443,
        ));

        assert_eq!(options.len(), 2);
        assert!(!options.is_empty());

        let dc1_options = options.get_options(DcId::internal(1));
        assert_eq!(dc1_options.len(), 1);
    }

    #[test]
    fn test_dc_options_set_best_option() {
        let mut set = DcOptionsSet::new();

        let mut options = DcOptions::new();
        options.add(DcOption::new(
            DcId::internal(2),
            IpAddr::V4(Ipv4Addr::new(149, 154, 167, 51)),
            443,
        ));
        options.add(DcOption::new(
            DcId::internal(2),
            IpAddr::V4(Ipv4Addr::new(149, 154, 167, 52)),
            443,
        ));

        set.add_options(options);

        // Before any stats, first option should be returned
        let best = set.find_best_option(DcId::internal(2), false);
        assert!(best.is_some());

        // Record success for second option
        set.record_success(1, 0.5);

        let best = set.find_best_option(DcId::internal(2), false);
        assert!(best.is_some());
    }

    #[test]
    fn test_dc_option_stats() {
        let mut stats = DcOptionStats::default();

        stats.record_success(0.5);
        stats.record_success(1.0);
        stats.record_failure();

        assert_eq!(stats.success_count, 2);
        assert_eq!(stats.failure_count, 1);
        assert!((stats.success_rate() - 0.666).abs() < 0.01);
        assert!((stats.avg_rtt - 0.75).abs() < 0.01);
    }
}
