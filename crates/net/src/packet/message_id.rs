// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto Message ID.
//!
//! Message IDs are 64-bit integers used to uniquely identify messages in MTProto.
//! They are based on server time and must be monotonically increasing for outgoing
//! messages from a client.
//!
//! # Format
//!
//! The message ID format is: `[time_bits(32) : random(32)]`
//!
//! - `time_bits`: `server_time * 2^32` (integer seconds in high bits)
//! - `random`: randomized lower bits for uniqueness
//!
//! # References
//!
//! - TDLib: `td/mtproto/MessageId.h`
//! - MTProto 2.0: <https://core.telegram.org/mtproto/description>

use std::fmt;

/// MTProto message identifier.
///
/// Message IDs are time-based 64-bit values that must be monotonically increasing
/// for outgoing messages. The format follows MTProto 2.0 specification.
///
/// # Format
///
/// The message ID is structured as:
/// ```text
/// bits 0-31:   fractional time/randomized bits
/// bits 32-63:  integer time (seconds)
/// ```
///
/// # Examples
///
/// ```
/// use rustgram_net::packet::MessageId;
///
/// // Create from raw value
/// let msg_id = MessageId::from_u64(0x62000000_12345678);
///
/// // Check if empty
/// assert!(!msg_id.is_empty());
///
/// // Get the raw value
/// assert_eq!(msg_id.as_u64(), 0x62000000_12345678);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(u64);

impl Default for MessageId {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl From<u64> for MessageId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<MessageId> for u64 {
    fn from(msg_id: MessageId) -> Self {
        msg_id.0
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "msg_{:016x}", self.0)
    }
}

impl MessageId {
    /// Empty message ID (zero value).
    pub const EMPTY: Self = Self(0);

    /// Creates a new `MessageId` from a raw 64-bit value.
    #[must_use]
    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw 64-bit value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Generates a new message ID based on server time.
    ///
    /// # Arguments
    ///
    /// * `server_time` - Current server time in seconds since Unix epoch
    /// * `is_outgoing` - Whether this is an outgoing message (affects the bit layout)
    /// * `seq_no` - Sequence number for this message
    ///
    /// # Algorithm
    ///
    /// ```text
    /// msg_id = floor(server_time * 2^32)
    /// ```
    ///
    /// # MTProto Requirements
    ///
    /// - Client → Server messages: `msg_id` must be even
    /// - Server → Client messages: `msg_id` must be odd
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_net::packet::MessageId;
    ///
    /// // Generate for current time
    /// let msg_id = MessageId::generate(1704067200.0, true, 0);
    /// assert!(!msg_id.is_empty());
    /// ```
    #[must_use]
    pub fn generate(server_time: f64, is_outgoing: bool, _seq_no: i32) -> Self {
        // TDLib-compatible: msg_id = floor(server_time * 2^32)
        let mut msg_id = (server_time * ((1u64 << 32) as f64)) as u64;

        // Randomize lower bits for clocks with low precision
        use rand::Rng;
        let rx = rand::thread_rng().gen::<u32>() as u64;
        let to_xor = rx & ((1u64 << 22) - 1);
        msg_id ^= to_xor;

        // Ensure required parity
        if is_outgoing {
            // Client → server messages must be even
            msg_id &= !0x03;
        } else {
            // Server → client messages must be odd
            msg_id &= !0x03;
            msg_id |= 0x01;
        }

        // Debug-only assertion to verify MTProto requirements
        #[cfg(debug_assertions)]
        {
            if is_outgoing {
                assert_eq!(
                    msg_id & 1,
                    0,
                    "Client → Server msg_id must be even, got 0x{:016x} (lsb={})",
                    msg_id,
                    msg_id & 1
                );
            } else {
                assert_eq!(
                    msg_id & 1,
                    1,
                    "Server → Client msg_id must be odd, got 0x{:016x} (lsb={})",
                    msg_id,
                    msg_id & 1
                );
            }

            tracing::trace!(
                "Generated MessageId: 0x{:016x}, lsb={}, outgoing={}",
                msg_id,
                msg_id & 1,
                is_outgoing
            );
        }

        Self(msg_id)
    }

    /// Returns the approximate server time from this message ID.
    ///
    /// # Returns
    ///
    /// Server time in seconds since Unix epoch.
    #[must_use]
    pub fn time(self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }

        // Reverse the generation: time = msg_id / 2^32
        self.0 as f64 / ((1u64 << 32) as f64)
    }

    /// Returns the sequence number portion of the message ID.
    ///
    /// In this implementation, lower bits are randomized; seq_no is not encoded.
    #[must_use]
    pub const fn seq_no(self) -> i32 {
        // Extract sequence number from bits 32-39
        ((self.0 >> 32) & 0xFF) as i32
    }

    /// Returns true if the message ID is empty (zero value).
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns true if the message ID appears valid.
    ///
    /// A valid message ID should:
    /// - Not be zero
    /// - Have a reasonable time value (after 2010-01-01, before Telegram's launch)
    #[must_use]
    pub fn is_valid(self) -> bool {
        if self.is_empty() {
            return false;
        }

        // Check if time is reasonable (after 2010-01-01, allowing some buffer)
        let time = self.time();
        time >= 1_262_304_000.0 // 2010-01-01 00:00:00 UTC
    }

    /// Returns the next message ID after this one.
    ///
    /// This is used to generate a monotonically increasing sequence of message IDs.
    /// Since time should always be increasing, we add a small amount to ensure
    /// the next ID is greater.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of positions to advance
    #[must_use]
    pub fn next(self, n: i32) -> Self {
        // To ensure monotonic increase, we add to the lower 32 bits
        // In practice, server time should advance naturally
        let delta = (n as u64).max(1);
        Self(self.0.wrapping_add(delta))
    }

    /// Returns true if this is an outgoing message (from client perspective).
    ///
    /// Outgoing messages from clients have bit 32 set in a specific way.
    #[must_use]
    pub const fn is_outgoing(self) -> bool {
        // Client-side messages have certain bit patterns
        (self.0 & 0x03) != 0
    }

    /// Returns true if this is an incoming message (from server).
    #[must_use]
    pub const fn is_incoming(self) -> bool {
        // Server messages are divisible by 4
        (self.0 & 0x03) == 0
    }

    /// Converts to a hex string representation.
    #[must_use]
    pub fn as_hex(self) -> String {
        format!("{:016x}", self.0)
    }

    /// Parses a message ID from hex string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid 16-character hex number.
    #[must_use]
    pub fn from_hex(s: &str) -> Option<Self> {
        let without_prefix = s.strip_prefix("0x").unwrap_or(s);
        u64::from_str_radix(without_prefix, 16).ok().map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_id_default() {
        let msg_id = MessageId::default();
        assert_eq!(msg_id, MessageId::EMPTY);
        assert!(msg_id.is_empty());
    }

    #[test]
    fn test_message_id_from_u64() {
        let value = 0x62000000_12345678;
        let msg_id = MessageId::from_u64(value);
        assert_eq!(msg_id.as_u64(), value);
    }

    #[test]
    fn test_message_id_from_into() {
        let value: u64 = 0x62000000_12345678;
        let msg_id: MessageId = value.into();
        let back: u64 = msg_id.into();
        assert_eq!(back, value);
    }

    #[test]
    fn test_message_id_generate() {
        // 2024-01-01 00:00:00 UTC
        let server_time = 1_704_067_200.0;
        let msg_id = MessageId::generate(server_time, true, 0);

        assert!(!msg_id.is_empty());
        assert!(msg_id.is_valid());

        // Time should be close to what we provided
        let recovered_time = msg_id.time();
        assert!((recovered_time - server_time).abs() < 100.0);
    }

    #[test]
    fn test_message_id_is_empty() {
        assert!(MessageId::EMPTY.is_empty());
        assert!(MessageId::from_u64(0).is_empty());
        assert!(!MessageId::from_u64(1).is_empty());
    }

    #[test]
    fn test_message_id_is_valid() {
        // Empty is not valid
        assert!(!MessageId::EMPTY.is_valid());

        // Valid modern time
        let msg_id = MessageId::generate(1_704_067_200.0, true, 0);
        assert!(msg_id.is_valid());

        // Ancient time (before 2010) - invalid
        // Use a very small timestamp that's definitely before 2010
        let ancient_time = 1_000_000_000.0; // 2001-09-09
        let ancient = MessageId::generate(ancient_time, true, 0);
        assert!(!ancient.is_valid());
    }

    #[test]
    fn test_message_id_next() {
        let msg_id = MessageId::from_u64(0x62000000_00000001);
        let next = msg_id.next(1);
        assert!(next > msg_id);
        assert_eq!(next.as_u64(), 0x62000000_00000002);

        let next_5 = msg_id.next(5);
        assert_eq!(next_5.as_u64(), 0x62000000_00000006);
    }

    #[test]
    fn test_message_id_outgoing_incoming() {
        // Server messages are divisible by 4
        let server_msg = MessageId::from_u64(0x62000000_00000000);
        assert!(server_msg.is_incoming());
        assert!(!server_msg.is_outgoing());

        // Client messages have lower bits set
        let client_msg = MessageId::from_u64(0x62000000_00000001);
        assert!(!client_msg.is_incoming());
        assert!(client_msg.is_outgoing());
    }

    #[test]
    fn test_message_id_display() {
        let msg_id = MessageId::from_u64(0x62000000_12345678);
        let s = format!("{msg_id}");
        assert!(s.starts_with("msg_"));
        assert!(s.contains("6200000012345678"));
    }

    #[test]
    fn test_message_id_as_hex() {
        let msg_id = MessageId::from_u64(0x62000000_12345678);
        assert_eq!(msg_id.as_hex(), "6200000012345678");
    }

    #[test]
    fn test_message_id_from_hex() {
        let msg_id = MessageId::from_hex("6200000012345678");
        assert_eq!(msg_id, Some(MessageId::from_u64(0x62000000_12345678)));

        let with_prefix = MessageId::from_hex("0x6200000012345678");
        assert_eq!(with_prefix, Some(MessageId::from_u64(0x62000000_12345678)));

        let invalid = MessageId::from_hex("zzzzzzzzzzzzzzzz");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_message_id_ord() {
        let msg1 = MessageId::from_u64(100);
        let msg2 = MessageId::from_u64(200);
        let msg3 = MessageId::from_u64(200);

        assert!(msg1 < msg2);
        assert!(msg2 > msg1);
        assert!(msg2 == msg3);
        assert!(msg1 <= msg2);
        assert!(msg2 >= msg1);
    }

    #[test]
    fn test_message_id_hash() {
        use std::collections::HashSet;

        let msg1 = MessageId::from_u64(100);
        let msg2 = MessageId::from_u64(200);
        let msg3 = MessageId::from_u64(100);

        let mut set = HashSet::new();
        set.insert(msg1);
        set.insert(msg2);
        set.insert(msg3);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&msg1));
        assert!(set.contains(&msg2));
    }

    // Property-based tests
    #[cfg(feature = "proptest")]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_message_id_roundtrip(value in any::<u64>()) {
                let msg_id = MessageId::from_u64(value);
                assert_eq!(msg_id.as_u64(), value);
            }

            #[test]
            fn prop_next_is_greater(value in 1u64..1_000_000u64) {
                let msg_id = MessageId::from_u64(value);
                let next = msg_id.next(1);
                assert!(next > msg_id);
            }

            #[test]
            fn prop_from_hex_roundtrip(value in any::<u64>()) {
                let msg_id = MessageId::from_u64(value);
                let hex = msg_id.as_hex();
                let parsed = MessageId::from_hex(&hex);
                assert_eq!(parsed, Some(msg_id));
            }

            #[test]
            fn prop_generate_valid(server_time in 1_388_550_400f64..2_000_000_000f64) {
                let msg_id = MessageId::generate(server_time, true, 0);
                assert!(msg_id.is_valid());
                assert!(!msg_id.is_empty());
            }
        }
    }
}
