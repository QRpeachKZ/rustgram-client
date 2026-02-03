// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto transport reading operations.
//!
//! Based on TDLib's `td/mtproto/Transport.cpp` read implementation.

use bytes::Bytes;

use crate::crypto::{aes_ige_decrypt, kdf, kdf2, KdfOutput};
use crate::packet::{MessageId, PacketInfo, PacketType};
use crate::transport::header::{
    CryptoHeader, CryptoPrefix, EndToEndHeader, EndToEndPrefix, NoCryptoHeader, NoCryptoPrefix,
};

/// Result of reading from the transport.
///
/// Represents the different possible outcomes when reading MTProto packets.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ReadResult {
    /// No operation - no packet available
    #[default]
    Nop,

    /// A complete packet was received
    Packet(Bytes),

    /// An error occurred
    Error(i32),

    /// A quick acknowledgment was received
    QuickAck(u32),
}

impl ReadResult {
    /// Creates a `Nop` result.
    #[must_use]
    pub const fn nop() -> Self {
        Self::Nop
    }

    /// Creates a `Packet` result.
    #[must_use]
    pub fn packet(data: Vec<u8>) -> Self {
        Self::Packet(Bytes::from(data))
    }

    /// Creates a `Packet` result from `Bytes`.
    #[must_use]
    pub const fn packet_bytes(data: Bytes) -> Self {
        Self::Packet(data)
    }

    /// Creates an `Error` result.
    #[must_use]
    pub const fn error(code: i32) -> Self {
        Self::Error(code)
    }

    /// Creates a `QuickAck` result.
    #[must_use]
    pub const fn quick_ack(ack: u32) -> Self {
        Self::QuickAck(ack)
    }

    /// Returns `true` if this is a `Nop` result.
    #[must_use]
    pub const fn is_nop(&self) -> bool {
        matches!(self, Self::Nop)
    }

    /// Returns `true` if this is a `Packet` result.
    #[must_use]
    pub const fn is_packet(&self) -> bool {
        matches!(self, Self::Packet(..))
    }

    /// Returns `true` if this is an `Error` result.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error(..))
    }

    /// Returns `true` if this is a `QuickAck` result.
    #[must_use]
    pub const fn is_quick_ack(&self) -> bool {
        matches!(self, Self::QuickAck(..))
    }

    /// Returns the packet data if this is a `Packet` result.
    #[must_use]
    pub fn packet_data(&self) -> Option<&Bytes> {
        match self {
            Self::Packet(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the error code if this is an `Error` result.
    #[must_use]
    pub const fn error_code(&self) -> Option<i32> {
        match self {
            Self::Error(code) => Some(*code),
            _ => None,
        }
    }

    /// Returns the quick ack value if this is a `QuickAck` result.
    #[must_use]
    pub const fn quick_ack_value(&self) -> Option<u32> {
        match self {
            Self::QuickAck(ack) => Some(*ack),
            _ => None,
        }
    }
}

/// Error type for transport read operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportReadError {
    /// Message too small
    MessageTooSmall {
        /// Actual size
        actual: usize,
        /// Expected minimum size
        expected: usize,
    },

    /// Auth key ID mismatch
    AuthKeyIdMismatch {
        /// Found auth key ID
        found: u64,
        /// Expected auth key ID
        expected: u64,
    },

    /// Message key mismatch
    MessageKeyMismatch,

    /// Invalid message length
    InvalidMessageLength {
        /// Message data length
        length: u32,
        /// Reason
        reason: String,
    },

    /// Invalid padding
    InvalidPadding {
        /// Padding size
        pad_size: usize,
    },

    /// Decryption failed
    DecryptionFailed,

    /// Crypto error
    CryptoError(String),
}

impl std::fmt::Display for TransportReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MessageTooSmall { actual, expected } => write!(
                f,
                "Message too small: got {} bytes, expected at least {} bytes",
                actual, expected
            ),
            Self::AuthKeyIdMismatch { found, expected } => write!(
                f,
                "Auth key ID mismatch: found {:016x}, expected {:016x}",
                found, expected
            ),
            Self::MessageKeyMismatch => write!(f, "Message key mismatch"),
            Self::InvalidMessageLength { length, reason } => {
                write!(f, "Invalid message length {}: {}", length, reason)
            }
            Self::InvalidPadding { pad_size } => {
                write!(f, "Invalid padding size: {}", pad_size)
            }
            Self::DecryptionFailed => write!(f, "Decryption failed"),
            Self::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
        }
    }
}

impl std::error::Error for TransportReadError {}

/// MTProto transport reading interface.
pub trait TransportRead: Send + Sync {
    /// Reads an MTProto packet from the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - The raw message bytes from the socket
    /// * `auth_key` - The authentication key for decryption
    /// * `packet_info` - Output parameter for packet metadata
    ///
    /// # Returns
    ///
    /// A `ReadResult` indicating what was read
    ///
    /// # Errors
    ///
    /// Returns a `TransportReadError` if reading fails
    fn read(
        &self,
        message: &[u8],
        auth_key: Option<&[u8; 256]>,
        packet_info: &mut PacketInfo,
    ) -> Result<ReadResult, TransportReadError>;
}

/// Default MTProto transport reader implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTransportReader;

impl DefaultTransportReader {
    /// Creates a new reader.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Checks if message is a quick ack or error.
    fn check_special_messages(message: &[u8]) -> Option<ReadResult> {
        if message.len() < 4 {
            return None;
        }

        // Read the first 4 bytes as i32
        let code = i32::from_le_bytes(message[0..4].try_into().ok()?);

        if code == 0 {
            return Some(ReadResult::nop());
        }

        if message.len() >= 8 && code == -1 {
            let ack = u32::from_le_bytes(message[4..8].try_into().ok()?);
            return Some(ReadResult::quick_ack(ack));
        }

        if code < 0 {
            return Some(ReadResult::error(code));
        }

        None
    }

    /// Reads auth key ID from message.
    fn read_auth_key_id(message: &[u8]) -> Result<u64, TransportReadError> {
        if message.len() < 8 {
            return Err(TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: 8,
            });
        }

        Ok(u64::from_le_bytes(
            message[0..8]
                .try_into()
                .expect("slice has verified length of 8"),
        ))
    }

    /// Reads a no-crypto packet.
    ///
    /// Format per MTProto 2.0 specification (plaintext handshake):
    /// ```text
    /// auth_key_id(8) = 0
    /// msg_id(8)
    /// message_data_length(4)
    /// message_data(N)
    /// ```
    ///
    /// NOTE: Unlike encrypted packets, plaintext packets do NOT have a seq_no field.
    /// Total header size is 20 bytes (8 + 8 + 4), not 24.
    fn read_no_crypto(&self, message: &[u8]) -> Result<ReadResult, TransportReadError> {
        let min_size = NoCryptoHeader::SIZE + NoCryptoPrefix::SIZE;

        tracing::debug!(
            "NoCrypto packet read: received_bytes={}, min_required={}, header_size={}, prefix_size={}",
            message.len(),
            min_size,
            NoCryptoHeader::SIZE,
            NoCryptoPrefix::SIZE,
        );

        if message.len() < min_size {
            return Err(TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: min_size,
            });
        }

        let header = NoCryptoHeader::read_from(message).ok_or({
            TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: min_size,
            }
        })?;

        if !header.is_no_crypto() {
            tracing::error!(
                "NoCrypto packet: invalid auth_key_id={:016x}, expected 0",
                header.auth_key_id
            );
            return Err(TransportReadError::AuthKeyIdMismatch {
                found: header.auth_key_id,
                expected: 0,
            });
        }

        // Read the NoCryptoPrefix (msg_id + message_data_length, NO padding)
        let prefix = NoCryptoPrefix::read_from(&message[NoCryptoHeader::SIZE..]).ok_or({
            TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: min_size,
            }
        })?;

        // Detailed logging
        tracing::debug!(
            "NoCrypto packet header: auth_key_id={:016x}, msg_id={:016x}, msg_id_mod4={:08x}, data_len={}",
            header.auth_key_id,
            prefix.msg_id,
            prefix.msg_id % 4,
            prefix.message_data_length,
        );

        // Validate message length
        let total_size = NoCryptoHeader::SIZE + NoCryptoPrefix::SIZE + prefix.message_data_length as usize;
        if message.len() < total_size {
            return Err(TransportReadError::InvalidMessageLength {
                length: prefix.message_data_length,
                reason: format!("not enough data: {} < {}", message.len(), total_size),
            });
        }

        // Extract data
        let data_offset = NoCryptoHeader::SIZE + NoCryptoPrefix::SIZE;
        let data = message[data_offset..total_size].to_vec();

        tracing::trace!(
            "NoCrypto packet data (first 32 bytes): {:02x?}",
            data.iter().take(std::cmp::min(32, data.len())).collect::<Vec<_>>()
        );

        Ok(ReadResult::packet(data))
    }

    /// Reads an encrypted packet.
    fn read_crypto(
        &self,
        message: &[u8],
        auth_key: &[u8; 256],
        packet_info: &mut PacketInfo,
    ) -> Result<ReadResult, TransportReadError> {
        let min_size =
            CryptoHeader::encrypt_begin_offset() + CryptoHeader::ENCRYPTED_HEADER_SIZE + CryptoPrefix::SIZE;
        if message.len() < min_size {
            return Err(TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: min_size,
            });
        }

        let header = CryptoHeader::read_from(message).ok_or({
            TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: CryptoHeader::SIZE,
            }
        })?;

        // Check auth key ID
        let auth_key_id = compute_auth_key_id(auth_key);
        if header.auth_key_id != auth_key_id {
            return Err(TransportReadError::AuthKeyIdMismatch {
                found: header.auth_key_id,
                expected: auth_key_id,
            });
        }

        // Derive AES key and IV
        let x = if packet_info.packet_type == PacketType::Common {
            8 // Server to client
        } else {
            0 // Client to server or e2e
        };

        let KdfOutput { aes_key, aes_iv } = if packet_info.version == 2 {
            kdf2(auth_key, &header.message_key, x)
        } else {
            kdf(auth_key, &header.message_key, x)
        };

        // Decrypt the message
        let encrypted_start = CryptoHeader::encrypt_begin_offset();
        let mut to_decrypt = message[encrypted_start..].to_vec();

        // Align to block size
        while to_decrypt.len() % 16 != 0 {
            to_decrypt.push(0);
        }

        let mut iv = aes_iv;
        aes_ige_decrypt(&aes_key, &mut iv, &mut to_decrypt)
            .map_err(|e| TransportReadError::CryptoError(e.to_string()))?;

        // Read the encrypted header (salt + session_id) and prefix
        if to_decrypt.len() < CryptoHeader::ENCRYPTED_HEADER_SIZE + CryptoPrefix::SIZE {
            return Err(TransportReadError::MessageTooSmall {
                actual: to_decrypt.len(),
                expected: CryptoHeader::ENCRYPTED_HEADER_SIZE + CryptoPrefix::SIZE,
            });
        }
        let salt = u64::from_le_bytes(to_decrypt[0..8].try_into().map_err(|_| {
            TransportReadError::MessageTooSmall {
                actual: to_decrypt.len(),
                expected: CryptoHeader::ENCRYPTED_HEADER_SIZE,
            }
        })?);
        let session_id = u64::from_le_bytes(to_decrypt[8..16].try_into().map_err(|_| {
            TransportReadError::MessageTooSmall {
                actual: to_decrypt.len(),
                expected: CryptoHeader::ENCRYPTED_HEADER_SIZE,
            }
        })?);

        let prefix_offset = CryptoHeader::ENCRYPTED_HEADER_SIZE;
        let prefix = CryptoPrefix::read_from(&to_decrypt[prefix_offset..]).ok_or({
            TransportReadError::MessageTooSmall {
                actual: to_decrypt.len(),
                expected: CryptoHeader::ENCRYPTED_HEADER_SIZE + CryptoPrefix::SIZE,
            }
        })?;

        // Validate message length
        let data_offset = CryptoHeader::ENCRYPTED_HEADER_SIZE + CryptoPrefix::SIZE;
        let total_size = data_offset + prefix.message_data_length as usize;
        if to_decrypt.len() < total_size {
            return Err(TransportReadError::InvalidMessageLength {
                length: prefix.message_data_length,
                reason: format!("not enough data: {} < {}", to_decrypt.len(), total_size),
            });
        }

        // Check padding
        let pad_size = to_decrypt.len() - total_size;
        if !(12..=1024).contains(&pad_size) {
            return Err(TransportReadError::InvalidPadding { pad_size });
        }

        // Update packet info
        packet_info.salt = salt;
        packet_info.session_id = session_id;
        packet_info.message_id = MessageId::from_u64(prefix.msg_id);
        packet_info.seq_no = prefix.seq_no as i32;

        // Extract data
        let data = to_decrypt[data_offset..total_size].to_vec();
        Ok(ReadResult::packet(data))
    }

    /// Reads an end-to-end encrypted packet.
    fn read_e2e_crypto(
        &self,
        message: &[u8],
        auth_key: &[u8; 256],
        packet_info: &mut PacketInfo,
    ) -> Result<ReadResult, TransportReadError> {
        if message.len() < EndToEndHeader::SIZE {
            return Err(TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: EndToEndHeader::SIZE,
            });
        }

        let header = EndToEndHeader::read_from(message).ok_or({
            TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: EndToEndHeader::SIZE,
            }
        })?;

        // Similar to read_crypto but for e2e
        let x = if packet_info.packet_type == PacketType::EndToEnd
            && packet_info.is_creator
            && packet_info.version != 1
        {
            8
        } else {
            0
        };

        // Derive keys and decrypt
        let KdfOutput { aes_key, aes_iv } = if packet_info.version == 2 {
            kdf2(auth_key, &header.message_key, x)
        } else {
            kdf(auth_key, &header.message_key, x)
        };

        let encrypted_start = EndToEndHeader::encrypt_begin_offset();
        let mut to_decrypt = message[encrypted_start..].to_vec();

        // Align to block size
        while to_decrypt.len() % 16 != 0 {
            to_decrypt.push(0);
        }

        let mut iv = aes_iv;
        aes_ige_decrypt(&aes_key, &mut iv, &mut to_decrypt)
            .map_err(|e| TransportReadError::CryptoError(e.to_string()))?;

        // Read the prefix
        if to_decrypt.len() < EndToEndPrefix::SIZE {
            return Err(TransportReadError::MessageTooSmall {
                actual: to_decrypt.len(),
                expected: EndToEndPrefix::SIZE,
            });
        }

        let prefix = EndToEndPrefix::read_from(&to_decrypt).ok_or({
            TransportReadError::MessageTooSmall {
                actual: to_decrypt.len(),
                expected: EndToEndPrefix::SIZE,
            }
        })?;

        // Validate
        let total_size = EndToEndPrefix::SIZE + prefix.message_data_length as usize;
        if to_decrypt.len() < total_size {
            return Err(TransportReadError::InvalidMessageLength {
                length: prefix.message_data_length,
                reason: format!("not enough data: {} < {}", to_decrypt.len(), total_size),
            });
        }

        // Extract data
        let data = to_decrypt[EndToEndPrefix::SIZE..total_size].to_vec();
        Ok(ReadResult::packet(data))
    }
}

impl TransportRead for DefaultTransportReader {
    fn read(
        &self,
        message: &[u8],
        auth_key: Option<&[u8; 256]>,
        packet_info: &mut PacketInfo,
    ) -> Result<ReadResult, TransportReadError> {
        // Check for special messages (< 16 bytes)
        if message.len() < 16 {
            if let Some(result) = Self::check_special_messages(message) {
                return Ok(result);
            }

            return Err(TransportReadError::MessageTooSmall {
                actual: message.len(),
                expected: 16,
            });
        }

        // Read auth key ID to determine mode
        let auth_key_id = Self::read_auth_key_id(message)?;

        // Determine if using encryption
        let no_crypto = auth_key_id == 0;

        if packet_info.packet_type == PacketType::EndToEnd {
            // End-to-end encryption
            let key = auth_key.ok_or(TransportReadError::DecryptionFailed)?;
            self.read_e2e_crypto(message, key, packet_info)
        } else if no_crypto {
            // No encryption
            packet_info.no_crypto_flag = true;
            self.read_no_crypto(message)
        } else {
            // Normal encryption
            let key = auth_key.ok_or(TransportReadError::DecryptionFailed)?;
            self.read_crypto(message, key, packet_info)
        }
    }
}

/// Computes auth key ID from auth key.
///
/// The auth key ID is the SHA1 hash of the auth key, lower 64 bits.
#[must_use]
pub fn compute_auth_key_id(auth_key: &[u8; 256]) -> u64 {
    use crate::crypto::sha1;

    let hash = sha1(auth_key);
    // Take lower 64 bits of SHA1 hash
    u64::from_le_bytes(
        hash[12..20]
            .try_into()
            .expect("SHA1 hash is always 20 bytes"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_result_nop() {
        let result = ReadResult::nop();
        assert!(result.is_nop());
        assert!(!result.is_packet());
        assert!(!result.is_error());
        assert!(!result.is_quick_ack());
    }

    #[test]
    fn test_read_result_packet() {
        let data = vec![1, 2, 3, 4];
        let result = ReadResult::packet(data.clone());
        assert!(result.is_packet());
        assert!(!result.is_nop());
        assert_eq!(result.packet_data(), Some(&Bytes::from(data)));
    }

    #[test]
    fn test_read_result_error() {
        let result = ReadResult::error(-404);
        assert!(result.is_error());
        assert_eq!(result.error_code(), Some(-404));
    }

    #[test]
    fn test_read_result_quick_ack() {
        let result = ReadResult::quick_ack(12345);
        assert!(result.is_quick_ack());
        assert_eq!(result.quick_ack_value(), Some(12345));
    }

    #[test]
    fn test_read_result_default() {
        let result = ReadResult::default();
        assert!(result.is_nop());
    }

    #[test]
    fn test_transport_read_error_display() {
        let err = TransportReadError::MessageTooSmall {
            actual: 4,
            expected: 16,
        };
        let s = format!("{err}");
        assert!(s.contains("too small"));
        assert!(s.contains("4"));
        assert!(s.contains("16"));

        let err = TransportReadError::AuthKeyIdMismatch {
            found: 123,
            expected: 456,
        };
        let s = format!("{err}");
        assert!(s.contains("mismatch"));

        let err = TransportReadError::InvalidPadding { pad_size: 5 };
        let s = format!("{err}");
        assert!(s.contains("padding"));
    }

    #[test]
    fn test_compute_auth_key_id() {
        let key = [42u8; 256];
        let id1 = compute_auth_key_id(&key);
        let id2 = compute_auth_key_id(&key);
        assert_eq!(id1, id2);

        let different_key = [43u8; 256];
        let id3 = compute_auth_key_id(&different_key);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_check_special_messages_nop() {
        let message = 0i32.to_le_bytes().to_vec();
        let result = DefaultTransportReader::check_special_messages(&message);
        assert_eq!(result, Some(ReadResult::nop()));
    }

    #[test]
    fn test_check_special_messages_error() {
        let mut message = (-100i32).to_le_bytes().to_vec();
        message.extend_from_slice(&[0u8; 4]);
        let result = DefaultTransportReader::check_special_messages(&message);
        assert_eq!(result, Some(ReadResult::error(-100)));
    }

    #[test]
    fn test_check_special_messages_quick_ack() {
        let mut message = (-1i32).to_le_bytes().to_vec();
        message.extend_from_slice(&12345u32.to_le_bytes());
        let result = DefaultTransportReader::check_special_messages(&message);
        assert_eq!(result, Some(ReadResult::quick_ack(12345)));
    }

    #[test]
    fn test_check_special_messages_normal_packet() {
        // Normal packet starts with positive 4 bytes
        let message = 0x12345678u32.to_le_bytes().to_vec();
        let result = DefaultTransportReader::check_special_messages(&message);
        assert!(result.is_none());
    }

    #[test]
    fn test_read_auth_key_id() {
        let key_id = 0x1234567890ABCDEFu64;
        let mut message = key_id.to_le_bytes().to_vec();
        message.extend_from_slice(&[0u8; 32]);

        let result = DefaultTransportReader::read_auth_key_id(&message);
        match result {
            Ok(k) => assert_eq!(k, key_id),
            Err(_) => panic!("Expected Ok key_id"),
        }
    }

    #[test]
    fn test_read_auth_key_id_too_small() {
        let message = [0u8; 4];
        let result = DefaultTransportReader::read_auth_key_id(&message);
        assert!(matches!(
            result,
            Err(TransportReadError::MessageTooSmall { .. })
        ));
    }

    #[test]
    fn test_read_no_crypto() {
        let data = vec![1u8, 2, 3, 4, 5];

        // Build packet: header (8) + prefix (12) + data
        let mut packet = vec![0u8; NoCryptoHeader::SIZE + NoCryptoPrefix::SIZE + data.len()];

        // Write header
        let header = NoCryptoHeader::new();
        header.write_to(&mut packet);

        // Write prefix with message_data_length
        let prefix = NoCryptoPrefix::with_values(0x62000000_00000001, data.len() as u32);
        prefix.write_to(&mut packet[NoCryptoHeader::SIZE..]);

        // Write data
        packet[NoCryptoHeader::SIZE + NoCryptoPrefix::SIZE..].copy_from_slice(&data);

        let reader = DefaultTransportReader::new();
        let result = reader.read_no_crypto(&packet);

        assert!(result.is_ok());
        let read_result = match result {
            Ok(r) => r,
            Err(_) => panic!("Expected Ok result"),
        };
        assert!(read_result.is_packet());
        match read_result.packet_data() {
            Some(d) => assert_eq!(d.as_ref(), &data),
            None => panic!("Expected Some packet_data"),
        }
    }

    #[test]
    fn test_read_no_crypto_wrong_key_id() {
        let mut packet = vec![0u8; NoCryptoHeader::SIZE + NoCryptoPrefix::SIZE + 8];

        // Write non-zero auth_key_id
        let mut header = NoCryptoHeader::new();
        header.auth_key_id = 123; // Not zero
        header.write_to(&mut packet);

        // Write prefix
        let prefix = NoCryptoPrefix::new();
        prefix.write_to(&mut packet[NoCryptoHeader::SIZE..]);

        let reader = DefaultTransportReader::new();
        let result = reader.read_no_crypto(&packet);

        assert!(matches!(
            result,
            Err(TransportReadError::AuthKeyIdMismatch { .. })
        ));
    }
}
