// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto transport header types.
//!
//! Based on TDLib's `td/mtproto/Transport.cpp` header structures.

use std::fmt;

/// MTProto encrypted packet header (AES-IGE mode).
///
/// This header is used for encrypted MTProto packets.
/// Format:
/// ```text
/// [0:8]   auth_key_id (u64)
/// [8:24]  message_key (UInt128)
/// [24:32] salt (encrypted, u64)
/// [32:40] session_id (encrypted, u64)
/// [40:]   encrypted data
/// ```
///
/// # References
///
/// - TDLib: `td/mtproto/Transport.cpp` - `CryptoHeader`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct CryptoHeader {
    /// Authentication key ID
    pub auth_key_id: u64,

    /// Message key (128-bit)
    pub message_key: [u8; 16],

    /// Salt (part of encrypted data)
    pub salt: u64,

    /// Session ID (part of encrypted data)
    pub session_id: u64,
}

impl CryptoHeader {
    /// Size of the encrypted header portion (salt + session_id).
    pub const ENCRYPTED_HEADER_SIZE: usize = 16;

    /// Total size of the header.
    pub const SIZE: usize = 40;

    /// Creates a new `CryptoHeader`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            auth_key_id: 0,
            message_key: [0u8; 16],
            salt: 0,
            session_id: 0,
        }
    }

    /// Creates a `CryptoHeader` with the specified values.
    #[must_use]
    pub const fn with_values(
        auth_key_id: u64,
        message_key: [u8; 16],
        salt: u64,
        session_id: u64,
    ) -> Self {
        Self {
            auth_key_id,
            message_key,
            salt,
            session_id,
        }
    }

    /// Returns the byte offset where encryption begins.
    #[must_use]
    pub const fn encrypt_begin_offset() -> usize {
        // Encryption starts after auth_key_id + msg_key (offset 24)
        24
    }

    /// Writes the header to a byte buffer.
    ///
    /// # Panics
    ///
    /// Panics if `buf.len() < 40`.
    pub fn write_to(&self, buf: &mut [u8]) {
        assert!(buf.len() >= Self::SIZE, "Buffer too small for CryptoHeader");

        buf[0..8].copy_from_slice(&self.auth_key_id.to_le_bytes());
        buf[8..24].copy_from_slice(&self.message_key);
        buf[24..32].copy_from_slice(&self.salt.to_le_bytes());
        buf[32..40].copy_from_slice(&self.session_id.to_le_bytes());
    }

    /// Reads a `CryptoHeader` from a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if `buf.len() < 40`.
    pub fn read_from(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::SIZE {
            return None;
        }

        let auth_key_id = u64::from_le_bytes(buf[0..8].try_into().ok()?);
        let mut message_key = [0u8; 16];
        message_key.copy_from_slice(&buf[8..24]);
        let salt = u64::from_le_bytes(buf[24..32].try_into().ok()?);
        let session_id = u64::from_le_bytes(buf[32..40].try_into().ok()?);

        Some(Self {
            auth_key_id,
            message_key,
            salt,
            session_id,
        })
    }
}

/// MTProto encrypted packet prefix (inside encrypted data).
///
/// This prefix follows the header and contains message metadata.
/// Format:
/// ```text
/// [0:8]   msg_id (u64)
/// [8:12]  seq_no (u32)
/// [12:16] message_data_length (u32)
/// ```
///
/// # References
///
/// - TDLib: `td/mtproto/Transport.cpp` - `CryptoPrefix`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct CryptoPrefix {
    /// Message ID
    pub msg_id: u64,

    /// Sequence number
    pub seq_no: u32,

    /// Message data length
    pub message_data_length: u32,
}

impl CryptoPrefix {
    /// Size of the prefix in bytes.
    pub const SIZE: usize = 16;

    /// Creates a new `CryptoPrefix`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            msg_id: 0,
            seq_no: 0,
            message_data_length: 0,
        }
    }

    /// Creates a `CryptoPrefix` with the specified values.
    #[must_use]
    pub const fn with_values(msg_id: u64, seq_no: u32, message_data_length: u32) -> Self {
        Self {
            msg_id,
            seq_no,
            message_data_length,
        }
    }

    /// Returns the total data size (prefix + data).
    #[must_use]
    pub const fn total_data_size(&self) -> usize {
        Self::SIZE + self.message_data_length as usize
    }

    /// Writes the prefix to a byte buffer.
    ///
    /// # Panics
    ///
    /// Panics if `buf.len() < 16`.
    pub fn write_to(&self, buf: &mut [u8]) {
        assert!(buf.len() >= Self::SIZE, "Buffer too small for CryptoPrefix");

        buf[0..8].copy_from_slice(&self.msg_id.to_le_bytes());
        buf[8..12].copy_from_slice(&self.seq_no.to_le_bytes());
        buf[12..16].copy_from_slice(&self.message_data_length.to_le_bytes());
    }

    /// Reads a `CryptoPrefix` from a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns `None` if `buf.len() < 16`.
    pub fn read_from(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::SIZE {
            return None;
        }

        let msg_id = u64::from_le_bytes(buf[0..8].try_into().ok()?);
        let seq_no = u32::from_le_bytes(buf[8..12].try_into().ok()?);
        let message_data_length = u32::from_le_bytes(buf[12..16].try_into().ok()?);

        Some(Self {
            msg_id,
            seq_no,
            message_data_length,
        })
    }
}

/// MTProto end-to-end encrypted packet header.
///
/// Used for end-to-end encrypted messages in secret chats.
/// Format:
/// ```text
/// [0:8]   auth_key_id (u64)
/// [8:24]  message_key (UInt128)
/// [24:]   encrypted data
/// ```
///
/// # References
///
/// - TDLib: `td/mtproto/Transport.cpp` - `EndToEndHeader`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct EndToEndHeader {
    /// Authentication key ID
    pub auth_key_id: u64,

    /// Message key (128-bit)
    pub message_key: [u8; 16],
}

impl EndToEndHeader {
    /// Size of the encrypted header portion (none for e2e).
    pub const ENCRYPTED_HEADER_SIZE: usize = 0;

    /// Total size of the header.
    pub const SIZE: usize = 24;

    /// Creates a new `EndToEndHeader`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            auth_key_id: 0,
            message_key: [0u8; 16],
        }
    }

    /// Creates an `EndToEndHeader` with the specified values.
    #[must_use]
    pub const fn with_values(auth_key_id: u64, message_key: [u8; 16]) -> Self {
        Self {
            auth_key_id,
            message_key,
        }
    }

    /// Returns the byte offset where encryption begins.
    #[must_use]
    pub const fn encrypt_begin_offset() -> usize {
        // Encryption starts after message_key (offset 24)
        Self::SIZE
    }

    /// Writes the header to a byte buffer.
    ///
    /// # Panics
    ///
    /// Panics if `buf.len() < 24`.
    pub fn write_to(&self, buf: &mut [u8]) {
        assert!(
            buf.len() >= Self::SIZE,
            "Buffer too small for EndToEndHeader"
        );

        buf[0..8].copy_from_slice(&self.auth_key_id.to_le_bytes());
        buf[8..24].copy_from_slice(&self.message_key);
    }

    /// Reads an `EndToEndHeader` from a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns `None` if `buf.len() < 24`.
    pub fn read_from(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::SIZE {
            return None;
        }

        let auth_key_id = u64::from_le_bytes(buf[0..8].try_into().ok()?);
        let mut message_key = [0u8; 16];
        message_key.copy_from_slice(&buf[8..24]);

        Some(Self {
            auth_key_id,
            message_key,
        })
    }
}

/// MTProto end-to-end encrypted packet prefix.
///
/// This prefix follows the e2e header.
/// Format:
/// ```text
/// [0:4] message_data_length (u32)
/// ```
///
/// # References
///
/// - TDLib: `td/mtproto/Transport.cpp` - `EndToEndPrefix`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct EndToEndPrefix {
    /// Message data length
    pub message_data_length: u32,
}

impl EndToEndPrefix {
    /// Size of the prefix in bytes.
    pub const SIZE: usize = 4;

    /// Creates a new `EndToEndPrefix`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            message_data_length: 0,
        }
    }

    /// Creates an `EndToEndPrefix` with the specified length.
    #[must_use]
    pub const fn with_length(message_data_length: u32) -> Self {
        Self {
            message_data_length,
        }
    }

    /// Returns the total data size (prefix + data).
    #[must_use]
    pub const fn total_data_size(&self) -> usize {
        Self::SIZE + self.message_data_length as usize
    }

    /// Writes the prefix to a byte buffer.
    ///
    /// # Panics
    ///
    /// Panics if `buf.len() < 4`.
    pub fn write_to(&self, buf: &mut [u8]) {
        assert!(
            buf.len() >= Self::SIZE,
            "Buffer too small for EndToEndPrefix"
        );
        buf[0..4].copy_from_slice(&self.message_data_length.to_le_bytes());
    }

    /// Reads an `EndToEndPrefix` from a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns `None` if `buf.len() < 4`.
    pub fn read_from(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::SIZE {
            return None;
        }

        let message_data_length = u32::from_le_bytes(buf[0..4].try_into().ok()?);
        Some(Self {
            message_data_length,
        })
    }
}

/// MTProto unencrypted packet prefix.
///
/// This prefix follows `NoCryptoHeader` for unencrypted packets.
/// Format per MTProto 2.0 specification:
/// ```text
/// [0:8]   msg_id (u64)
/// [8:12]  message_data_length (u32)
/// ```
///
/// NOTE: Unlike encrypted packets (which have seq_no), unencrypted packets
/// do NOT have any padding field. The total size is 12 bytes, not 16.
///
/// # References
///
/// - MTProto 2.0 Specification: <https://core.telegram.org/mtproto/description>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct NoCryptoPrefix {
    /// Message ID (time-based, similar to encrypted packets)
    pub msg_id: u64,

    /// Message data length in bytes
    pub message_data_length: u32,
}

impl NoCryptoPrefix {
    /// Size of the prefix in bytes (msg_id + message_data_length, no padding).
    pub const SIZE: usize = 12;

    /// Creates a new `NoCryptoPrefix`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            msg_id: 0,
            message_data_length: 0,
        }
    }

    /// Creates a `NoCryptoPrefix` with the specified values.
    #[must_use]
    pub const fn with_values(msg_id: u64, message_data_length: u32) -> Self {
        Self {
            msg_id,
            message_data_length,
        }
    }

    /// Returns the total data size (prefix + data).
    #[must_use]
    pub const fn total_data_size(&self) -> usize {
        Self::SIZE + self.message_data_length as usize
    }

    /// Writes the prefix to a byte buffer.
    ///
    /// # Panics
    ///
    /// Panics if `buf.len() < 12`.
    pub fn write_to(&self, buf: &mut [u8]) {
        assert!(buf.len() >= Self::SIZE, "Buffer too small for NoCryptoPrefix");

        buf[0..8].copy_from_slice(&self.msg_id.to_le_bytes());
        buf[8..12].copy_from_slice(&self.message_data_length.to_le_bytes());
    }

    /// Reads a `NoCryptoPrefix` from a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns `None` if `buf.len() < 12`.
    pub fn read_from(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::SIZE {
            return None;
        }

        let msg_id = u64::from_le_bytes(buf[0..8].try_into().ok()?);
        let message_data_length = u32::from_le_bytes(buf[8..12].try_into().ok()?);

        Some(Self {
            msg_id,
            message_data_length,
        })
    }
}

impl fmt::Display for NoCryptoPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NoCryptoPrefix {{ msg_id: {:016x}, length: {} }}",
            self.msg_id, self.message_data_length
        )
    }
}

/// MTProto unencrypted packet header.
///
/// Used for unencrypted packets during initial handshake.
/// Format:
/// ```text
/// [0:8]   auth_key_id (always 0 for unencrypted)
/// [8:16]  msg_id (from NoCryptoPrefix)
/// [16:20] message_data_length (from NoCryptoPrefix)
/// [20:]   data
/// ```
///
/// # References
///
/// - TDLib: `td/mtproto/Transport.cpp` - `NoCryptoHeader`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct NoCryptoHeader {
    /// Authentication key ID (always 0)
    pub auth_key_id: u64,
}

impl NoCryptoHeader {
    /// Size of the header in bytes.
    pub const SIZE: usize = 8;

    /// Creates a new `NoCryptoHeader`.
    #[must_use]
    pub const fn new() -> Self {
        Self { auth_key_id: 0 }
    }

    /// Writes the header to a byte buffer.
    ///
    /// # Panics
    ///
    /// Panics if `buf.len() < 8`.
    pub fn write_to(&self, buf: &mut [u8]) {
        assert!(
            buf.len() >= Self::SIZE,
            "Buffer too small for NoCryptoHeader"
        );
        buf[0..8].copy_from_slice(&self.auth_key_id.to_le_bytes());
    }

    /// Reads a `NoCryptoHeader` from a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns `None` if `buf.len() < 8`.
    pub fn read_from(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::SIZE {
            return None;
        }

        let auth_key_id = u64::from_le_bytes(buf[0..8].try_into().ok()?);
        Some(Self { auth_key_id })
    }

    /// Returns `true` if this header indicates no encryption mode.
    #[must_use]
    pub const fn is_no_crypto(&self) -> bool {
        self.auth_key_id == 0
    }
}

impl fmt::Display for CryptoHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CryptoHeader {{ auth_key_id: {:016x}, salt: {:016x}, session_id: {:016x} }}",
            self.auth_key_id, self.salt, self.session_id
        )
    }
}

impl fmt::Display for CryptoPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CryptoPrefix {{ msg_id: {:016x}, seq_no: {}, length: {} }}",
            self.msg_id, self.seq_no, self.message_data_length
        )
    }
}

impl fmt::Display for EndToEndHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EndToEndHeader {{ auth_key_id: {:016x} }}",
            self.auth_key_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_header_default() {
        let header = CryptoHeader::default();
        assert_eq!(header.auth_key_id, 0);
        assert_eq!(header.salt, 0);
        assert_eq!(header.session_id, 0);
        assert_eq!(header.message_key, [0u8; 16]);
    }

    #[test]
    fn test_crypto_header_new() {
        let header = CryptoHeader::new();
        assert_eq!(header.auth_key_id, 0);
    }

    #[test]
    fn test_crypto_header_with_values() {
        let message_key = [1u8; 16];
        let header = CryptoHeader::with_values(123, message_key, 456, 789);
        assert_eq!(header.auth_key_id, 123);
        assert_eq!(header.salt, 456);
        assert_eq!(header.session_id, 789);
        assert_eq!(header.message_key, message_key);
    }

    #[test]
    fn test_crypto_header_write_read() {
        let header1 = CryptoHeader::with_values(123456, [1u8; 16], 789012, 345678);
        let mut buf = [0u8; 64];
        header1.write_to(&mut buf);

        let header2 = CryptoHeader::read_from(&buf).expect("Failed to read header");
        assert_eq!(header1, header2);
    }

    #[test]
    fn test_crypto_header_read_from_small_buffer() {
        let buf = [0u8; 10];
        assert!(CryptoHeader::read_from(&buf).is_none());
    }

    #[test]
    fn test_crypto_prefix_default() {
        let prefix = CryptoPrefix::default();
        assert_eq!(prefix.msg_id, 0);
        assert_eq!(prefix.seq_no, 0);
        assert_eq!(prefix.message_data_length, 0);
    }

    #[test]
    fn test_crypto_prefix_with_values() {
        let prefix = CryptoPrefix::with_values(123456, 789, 1024);
        assert_eq!(prefix.msg_id, 123456);
        assert_eq!(prefix.seq_no, 789);
        assert_eq!(prefix.message_data_length, 1024);
    }

    #[test]
    fn test_crypto_prefix_total_data_size() {
        let prefix = CryptoPrefix::with_values(0, 0, 100);
        assert_eq!(prefix.total_data_size(), 116); // 16 + 100
    }

    #[test]
    fn test_crypto_prefix_write_read() {
        let prefix1 = CryptoPrefix::with_values(123456, 789, 1024);
        let mut buf = [0u8; 32];
        prefix1.write_to(&mut buf);

        let prefix2 = CryptoPrefix::read_from(&buf).expect("Failed to read prefix");
        assert_eq!(prefix1, prefix2);
    }

    #[test]
    fn test_end_to_end_header_default() {
        let header = EndToEndHeader::default();
        assert_eq!(header.auth_key_id, 0);
        assert_eq!(header.message_key, [0u8; 16]);
    }

    #[test]
    fn test_end_to_end_header_with_values() {
        let message_key = [5u8; 16];
        let header = EndToEndHeader::with_values(999, message_key);
        assert_eq!(header.auth_key_id, 999);
        assert_eq!(header.message_key, message_key);
    }

    #[test]
    fn test_end_to_end_header_write_read() {
        let header1 = EndToEndHeader::with_values(111222, [3u8; 16]);
        let mut buf = [0u8; 64];
        header1.write_to(&mut buf);

        let header2 = EndToEndHeader::read_from(&buf).expect("Failed to read header");
        assert_eq!(header1, header2);
    }

    #[test]
    fn test_end_to_end_prefix_default() {
        let prefix = EndToEndPrefix::default();
        assert_eq!(prefix.message_data_length, 0);
    }

    #[test]
    fn test_end_to_end_prefix_with_length() {
        let prefix = EndToEndPrefix::with_length(512);
        assert_eq!(prefix.message_data_length, 512);
        assert_eq!(prefix.total_data_size(), 516); // 4 + 512
    }

    #[test]
    fn test_end_to_end_prefix_write_read() {
        let prefix1 = EndToEndPrefix::with_length(2048);
        let mut buf = [0u8; 16];
        prefix1.write_to(&mut buf);

        let prefix2 = EndToEndPrefix::read_from(&buf).expect("Failed to read prefix");
        assert_eq!(prefix1, prefix2);
    }

    #[test]
    fn test_no_crypto_header_default() {
        let header = NoCryptoHeader::default();
        assert_eq!(header.auth_key_id, 0);
        assert!(header.is_no_crypto());
    }

    #[test]
    fn test_no_crypto_header_write_read() {
        let header1 = NoCryptoHeader::new();
        let mut buf = [0u8; 16];
        header1.write_to(&mut buf);

        let header2 = NoCryptoHeader::read_from(&buf).expect("Failed to read header");
        assert_eq!(header1, header2);
    }

    #[test]
    fn test_no_crypto_header_is_no_crypto() {
        let header = NoCryptoHeader::new();
        assert!(header.is_no_crypto());
    }

    #[test]
    fn test_crypto_header_display() {
        let header = CryptoHeader::with_values(0x123, [1u8; 16], 0x456, 0x789);
        let s = format!("{header}");
        assert!(s.contains("123"));
        assert!(s.contains("456"));
        assert!(s.contains("789"));
    }

    #[test]
    fn test_crypto_prefix_display() {
        let prefix = CryptoPrefix::with_values(0xABCD, 42, 1024);
        let s = format!("{prefix}");
        assert!(s.contains("abcd"));
        assert!(s.contains("42"));
        assert!(s.contains("1024"));
    }

    #[test]
    fn test_end_to_end_header_display() {
        let header = EndToEndHeader::with_values(0xFEDCBA, [0u8; 16]);
        let s = format!("{header}");
        assert!(s.contains("fedcba"));
    }

    #[test]
    fn test_no_crypto_prefix_default() {
        let prefix = NoCryptoPrefix::default();
        assert_eq!(prefix.msg_id, 0);
        assert_eq!(prefix.message_data_length, 0);
    }

    #[test]
    fn test_no_crypto_prefix_with_values() {
        let prefix = NoCryptoPrefix::with_values(0x62000000_00000001, 20);
        assert_eq!(prefix.msg_id, 0x62000000_00000001);
        assert_eq!(prefix.message_data_length, 20);
    }

    #[test]
    fn test_no_crypto_prefix_size() {
        assert_eq!(NoCryptoPrefix::SIZE, 12); // 8 (msg_id) + 4 (message_data_length)
    }

    #[test]
    fn test_no_crypto_prefix_total_data_size() {
        let prefix = NoCryptoPrefix::with_values(0, 100);
        assert_eq!(prefix.total_data_size(), 112); // 12 + 100 (SIZE was changed from 16 to 12)
    }

    #[test]
    fn test_no_crypto_prefix_write_read() {
        let prefix1 = NoCryptoPrefix::with_values(0x62000000_12345678, 512);
        let mut buf = [0u8; 32];
        prefix1.write_to(&mut buf);

        let prefix2 = NoCryptoPrefix::read_from(&buf).expect("Failed to read prefix");
        assert_eq!(prefix1.msg_id, prefix2.msg_id);
        assert_eq!(prefix1.message_data_length, prefix2.message_data_length);
    }

    #[test]
    fn test_no_crypto_prefix_display() {
        let prefix = NoCryptoPrefix::with_values(0xABCD1234_5678, 1024);
        let s = format!("{prefix}");
        assert!(s.contains("abcd1234"));
        assert!(s.contains("1024"));
    }
}
