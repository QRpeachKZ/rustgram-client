// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto transport writing operations.
//!
//! Based on TDLib's `td/mtproto/Transport.cpp` write implementation.

use rand::Rng;

use crate::crypto::{aes_ige_encrypt, kdf, kdf2, sha256, KdfOutput};
use crate::packet::{PacketInfo, PacketType};
use crate::transport::header::{
    CryptoHeader, CryptoPrefix, EndToEndHeader, EndToEndPrefix, NoCryptoHeader,
};

/// Options for writing MTProto packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteOptions {
    /// Packet type
    pub packet_type: PacketType,

    /// MTProto version (1 or 2)
    pub version: i32,

    /// Whether to use random padding (MTProto 2.0)
    pub use_random_padding: bool,

    /// Whether to check that message length is divisible by 4
    pub check_mod4: bool,

    /// For e2e packets: whether this side is the creator
    pub is_creator: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            packet_type: PacketType::Common,
            version: 2,
            use_random_padding: false,
            check_mod4: true,
            is_creator: false,
        }
    }
}

impl WriteOptions {
    /// Creates new write options.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            packet_type: PacketType::Common,
            version: 2,
            use_random_padding: false,
            check_mod4: true,
            is_creator: false,
        }
    }

    /// Creates options for common (non-e2e) packets.
    #[must_use]
    pub const fn common() -> Self {
        Self {
            packet_type: PacketType::Common,
            version: 2,
            use_random_padding: false,
            check_mod4: true,
            is_creator: false,
        }
    }

    /// Creates options for end-to-end encrypted packets.
    #[must_use]
    pub const fn end_to_end() -> Self {
        Self {
            packet_type: PacketType::EndToEnd,
            version: 2,
            use_random_padding: false,
            check_mod4: true,
            is_creator: false,
        }
    }

    /// Sets the MTProto version.
    #[must_use]
    pub const fn with_version(mut self, version: i32) -> Self {
        self.version = version;
        self
    }

    /// Sets whether to use random padding.
    #[must_use]
    pub const fn with_random_padding(mut self, use_random: bool) -> Self {
        self.use_random_padding = use_random;
        self
    }

    /// Sets whether to check mod4 alignment.
    #[must_use]
    pub const fn with_check_mod4(mut self, check: bool) -> Self {
        self.check_mod4 = check;
        self
    }

    /// Sets whether this side is the creator (for e2e).
    #[must_use]
    pub const fn with_creator(mut self, is_creator: bool) -> Self {
        self.is_creator = is_creator;
        self
    }
}

/// Error type for transport write operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportWriteError {
    /// Data is empty
    EmptyData,

    /// Data too large
    DataTooLarge {
        /// Actual size
        actual: usize,
        /// Maximum size
        max: usize,
    },

    /// Invalid data length
    InvalidLength {
        /// Data length
        length: usize,
        /// Reason
        reason: String,
    },

    /// No auth key provided
    NoAuthKey,

    /// Crypto error
    CryptoError(String),
}

impl std::fmt::Display for TransportWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyData => write!(f, "Cannot write empty data"),
            Self::DataTooLarge { actual, max } => {
                write!(f, "Data too large: {} bytes (max {} bytes)", actual, max)
            }
            Self::InvalidLength { length, reason } => {
                write!(f, "Invalid data length {}: {}", length, reason)
            }
            Self::NoAuthKey => write!(f, "No auth key provided for encrypted packet"),
            Self::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
        }
    }
}

impl std::error::Error for TransportWriteError {}

/// MTProto transport writing interface.
pub trait TransportWrite: Send + Sync {
    /// Writes an MTProto packet.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write
    /// * `auth_key` - Optional authentication key for encryption
    /// * `packet_info` - Input/output parameter for packet metadata
    ///
    /// # Returns
    ///
    /// The encoded packet bytes
    ///
    /// # Errors
    ///
    /// Returns a `TransportWriteError` if writing fails
    fn write(
        &self,
        data: &[u8],
        auth_key: Option<&[u8; 256]>,
        packet_info: &mut PacketInfo,
    ) -> Result<Vec<u8>, TransportWriteError>;
}

const fn align_up(size: usize, alignment: usize) -> usize {
    (size + alignment - 1) & !(alignment - 1)
}

/// Default MTProto transport writer implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTransportWriter {
    options: WriteOptions,
}

impl DefaultTransportWriter {
    /// Creates a new writer with default options.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            options: WriteOptions::new(),
        }
    }

    /// Creates a new writer with the specified options.
    #[must_use]
    pub const fn with_options(options: WriteOptions) -> Self {
        Self { options }
    }

    /// Returns the current options.
    #[must_use]
    pub const fn options(&self) -> WriteOptions {
        self.options
    }

    /// Sets the options.
    pub fn set_options(&mut self, options: WriteOptions) {
        self.options = options;
    }

    /// Calculates the encrypted packet size for MTProto 1.0.
    fn calc_crypto_size_v1(
        data_size: usize,
        enc_header_size: usize,
        raw_header_size: usize,
    ) -> usize {
        // Align encrypted data to block size (16 bytes)
        let encrypted_size = align_up(enc_header_size + data_size, 16);
        raw_header_size + encrypted_size
    }

    /// Calculates the encrypted packet size for MTProto 2.0.
    fn calc_crypto_size_v2(
        data_size: usize,
        enc_header_size: usize,
        raw_header_size: usize,
        use_random_padding: bool,
    ) -> usize {
        if use_random_padding {
            // Random padding size (0-255 bytes)
            let random_pad = rand::thread_rng().gen::<u8>() as usize;
            let encrypted_size = align_up(enc_header_size + data_size + random_pad + 12, 16);
            raw_header_size + encrypted_size
        } else {
            // Basic padding
            let sizes = [64, 128, 192, 256, 384, 512, 768, 1024, 1280];

            let base_size = enc_header_size + data_size + 12;

            for size in sizes {
                let aligned = (size + 15) & !15;
                if base_size <= aligned {
                    return raw_header_size + aligned;
                }
            }

            // For larger packets
            let encrypted_size = align_up(base_size - 1280 + 447, 448) + 1280;
            raw_header_size + encrypted_size
        }
    }

    /// Calculates the message key (MTProto 2.0).
    fn calc_message_key_v2(auth_key: &[u8; 256], x: usize, to_encrypt: &[u8]) -> ([u8; 16], u32) {
        // msg_key_large = SHA256(substr(auth_key, 88+x, 32) + plaintext)
        let mut hash_input = Vec::with_capacity(32 + to_encrypt.len());
        hash_input.extend_from_slice(&auth_key[88 + x..88 + x + 32]);
        hash_input.extend_from_slice(to_encrypt);

        let msg_key_large = sha256(&hash_input);

        // msg_key = substr(msg_key_large, 8, 16)
        let mut msg_key = [0u8; 16];
        msg_key.copy_from_slice(&msg_key_large[8..24]);

        // message_ack = substr(msg_key_large, 0, 4) | (1 << 31)
        let mut ack_bytes = [0u8; 4];
        ack_bytes.copy_from_slice(&msg_key_large[0..4]);
        let message_ack = u32::from_le_bytes(ack_bytes) | (1 << 31);

        (msg_key, message_ack)
    }

    /// Writes an unencrypted packet.
    fn write_no_crypto(
        &self,
        data: &[u8],
        _packet_info: &PacketInfo,
    ) -> Result<Vec<u8>, TransportWriteError> {
        if data.is_empty() {
            return Err(TransportWriteError::EmptyData);
        }

        let mut packet = vec![0u8; NoCryptoHeader::SIZE + data.len()];

        let header = NoCryptoHeader::new();
        header.write_to(&mut packet);

        packet[NoCryptoHeader::SIZE..].copy_from_slice(data);

        Ok(packet)
    }

    /// Writes an encrypted packet (MTProto).
    fn write_crypto(
        &self,
        data: &[u8],
        auth_key: &[u8; 256],
        packet_info: &mut PacketInfo,
    ) -> Result<Vec<u8>, TransportWriteError> {
        if data.is_empty() {
            return Err(TransportWriteError::EmptyData);
        }

        if data.len() > 1 << 24 {
            return Err(TransportWriteError::DataTooLarge {
                actual: data.len(),
                max: 1 << 24,
            });
        }

        // Validate length
        if self.options.check_mod4 && data.len() % 4 != 0 {
            return Err(TransportWriteError::InvalidLength {
                length: data.len(),
                reason: "data length not divisible by 4".to_string(),
            });
        }

        let auth_key_id = compute_auth_key_id(auth_key);

        // Calculate packet size
        let raw_header_size = CryptoHeader::SIZE - CryptoHeader::ENCRYPTED_HEADER_SIZE;
        let packet_size = if self.options.version == 2 {
            Self::calc_crypto_size_v2(
                data.len(),
                CryptoHeader::ENCRYPTED_HEADER_SIZE,
                raw_header_size,
                self.options.use_random_padding,
            )
        } else {
            Self::calc_crypto_size_v1(
                data.len(),
                CryptoHeader::ENCRYPTED_HEADER_SIZE,
                raw_header_size,
            )
        };

        let mut packet = vec![0u8; packet_size];

        // Write header
        let header = CryptoHeader {
            auth_key_id,
            salt: packet_info.salt,
            session_id: packet_info.session_id,
            message_key: [0u8; 16],
        };

        packet[0..8].copy_from_slice(&header.auth_key_id.to_le_bytes());
        // message_key will be filled after encryption

        // Prepare encrypted data
        let encrypt_start = CryptoHeader::encrypt_begin_offset();
        let prefix = CryptoPrefix {
            msg_id: packet_info.message_id.as_u64(),
            seq_no: packet_info.seq_no as u32,
            message_data_length: data.len() as u32,
        };

        // Write prefix to the encrypted section
        prefix.write_to(&mut packet[encrypt_start..]);

        // Write data
        let data_offset = encrypt_start + CryptoPrefix::SIZE;
        packet[data_offset..data_offset + data.len()].copy_from_slice(data);

        // Calculate padding
        let total_data = CryptoHeader::ENCRYPTED_HEADER_SIZE + CryptoPrefix::SIZE + data.len();
        let padding_size = packet_size - raw_header_size - total_data;

        // Fill padding with random bytes
        let padding_offset = data_offset + data.len();
        if padding_size > 0 {
            let padding = &mut packet[padding_offset..padding_offset + padding_size];
            rand::thread_rng().fill(padding);
        }

        // Calculate message key and encrypt
        let x = 0; // Client to server

        let (msg_key, message_ack) = {
            let to_encrypt = &packet[encrypt_start..packet_size];
            if self.options.version == 2 {
                Self::calc_message_key_v2(auth_key, x, to_encrypt)
            } else {
                // MTProto 1.0 uses SHA1
                return Err(TransportWriteError::CryptoError(
                    "MTProto 1.0 not fully implemented".to_string(),
                ));
            }
        };

        // Write message key
        packet[8..24].copy_from_slice(&msg_key);
        packet_info.message_ack = message_ack;

        // Derive AES key/IV and encrypt
        let KdfOutput { aes_key, aes_iv } = if self.options.version == 2 {
            kdf2(auth_key, &msg_key, x)
        } else {
            kdf(auth_key, &msg_key, x)
        };

        // Now do the actual encryption on the encrypted portion
        {
            let to_encrypt = &mut packet[encrypt_start..packet_size];
            let mut iv = aes_iv;
            aes_ige_encrypt(&aes_key, &mut iv, to_encrypt)
                .map_err(|e| TransportWriteError::CryptoError(e.to_string()))?;
        }

        Ok(packet)
    }

    /// Writes an end-to-end encrypted packet.
    fn write_e2e_crypto(
        &self,
        data: &[u8],
        auth_key: &[u8; 256],
        packet_info: &mut PacketInfo,
    ) -> Result<Vec<u8>, TransportWriteError> {
        if data.is_empty() {
            return Err(TransportWriteError::EmptyData);
        }

        if data.len() > 1 << 24 {
            return Err(TransportWriteError::DataTooLarge {
                actual: data.len(),
                max: 1 << 24,
            });
        }

        let auth_key_id = compute_auth_key_id(auth_key);

        // Calculate packet size
        let raw_header_size = EndToEndHeader::SIZE;
        let packet_size = if self.options.version == 2 {
            Self::calc_crypto_size_v2(
                data.len(),
                EndToEndHeader::ENCRYPTED_HEADER_SIZE,
                raw_header_size,
                self.options.use_random_padding,
            )
        } else {
            Self::calc_crypto_size_v1(
                data.len(),
                EndToEndHeader::ENCRYPTED_HEADER_SIZE,
                raw_header_size,
            )
        };

        let mut packet = vec![0u8; packet_size];

        // Write header
        packet[0..8].copy_from_slice(&auth_key_id.to_le_bytes());
        // message_key will be filled after encryption

        // Prepare encrypted data
        let encrypt_start = EndToEndHeader::encrypt_begin_offset();
        let prefix = EndToEndPrefix {
            message_data_length: data.len() as u32,
        };

        // Write prefix
        prefix.write_to(&mut packet[encrypt_start..]);

        // Write data
        let data_offset = encrypt_start + EndToEndPrefix::SIZE;
        packet[data_offset..data_offset + data.len()].copy_from_slice(data);

        // Calculate padding
        let total_data = EndToEndHeader::ENCRYPTED_HEADER_SIZE + EndToEndPrefix::SIZE + data.len();
        let padding_size = packet_size - raw_header_size - total_data;

        // Fill padding with random bytes
        let padding_offset = data_offset + data.len();
        if padding_size > 0 {
            let padding = &mut packet[padding_offset..padding_offset + padding_size];
            rand::thread_rng().fill(padding);
        }

        // Calculate message key and encrypt
        let x = if self.options.is_creator && self.options.version != 1 {
            8
        } else {
            0
        };

        let (msg_key, message_ack) = {
            let to_encrypt = &packet[encrypt_start..packet_size];
            if self.options.version == 2 {
                Self::calc_message_key_v2(auth_key, x, to_encrypt)
            } else {
                return Err(TransportWriteError::CryptoError(
                    "MTProto 1.0 e2e not fully implemented".to_string(),
                ));
            }
        };

        // Write message key
        packet[8..24].copy_from_slice(&msg_key);
        packet_info.message_ack = message_ack;

        // Derive AES key/IV and encrypt
        let KdfOutput { aes_key, aes_iv } = if self.options.version == 2 {
            kdf2(auth_key, &msg_key, x)
        } else {
            kdf(auth_key, &msg_key, x)
        };

        // Now do the actual encryption on the encrypted portion
        {
            let to_encrypt = &mut packet[encrypt_start..packet_size];
            let mut iv = aes_iv;
            aes_ige_encrypt(&aes_key, &mut iv, to_encrypt)
                .map_err(|e| TransportWriteError::CryptoError(e.to_string()))?;
        }

        Ok(packet)
    }
}

impl TransportWrite for DefaultTransportWriter {
    fn write(
        &self,
        data: &[u8],
        auth_key: Option<&[u8; 256]>,
        packet_info: &mut PacketInfo,
    ) -> Result<Vec<u8>, TransportWriteError> {
        if packet_info.no_crypto_flag {
            self.write_no_crypto(data, packet_info)
        } else if packet_info.packet_type == PacketType::EndToEnd {
            let key = auth_key.ok_or(TransportWriteError::NoAuthKey)?;
            self.write_e2e_crypto(data, key, packet_info)
        } else {
            let key = auth_key.ok_or(TransportWriteError::NoAuthKey)?;
            self.write_crypto(data, key, packet_info)
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
    use crate::packet::MessageId;

    #[test]
    fn test_write_options_default() {
        let opts = WriteOptions::default();
        assert_eq!(opts.packet_type, PacketType::Common);
        assert_eq!(opts.version, 2);
        assert!(!opts.use_random_padding);
        assert!(opts.check_mod4);
        assert!(!opts.is_creator);
    }

    #[test]
    fn test_write_options_common() {
        let opts = WriteOptions::common();
        assert_eq!(opts.packet_type, PacketType::Common);
    }

    #[test]
    fn test_write_options_end_to_end() {
        let opts = WriteOptions::end_to_end();
        assert_eq!(opts.packet_type, PacketType::EndToEnd);
    }

    #[test]
    fn test_write_options_builder() {
        let opts = WriteOptions::new()
            .with_version(1)
            .with_random_padding(true)
            .with_creator(true);

        assert_eq!(opts.version, 1);
        assert!(opts.use_random_padding);
        assert!(opts.is_creator);
    }

    #[test]
    fn test_transport_write_error_display() {
        let err = TransportWriteError::EmptyData;
        let s = format!("{err}");
        assert!(s.contains("empty"));

        let err = TransportWriteError::DataTooLarge {
            actual: 20000,
            max: 10000,
        };
        let s = format!("{err}");
        assert!(s.contains("too large"));

        let err = TransportWriteError::InvalidLength {
            length: 5,
            reason: "test".to_string(),
        };
        let s = format!("{err}");
        assert!(s.contains("5"));
        assert!(s.contains("test"));

        let err = TransportWriteError::NoAuthKey;
        let s = format!("{err}");
        assert!(s.contains("auth key"));
    }

    #[test]
    fn test_default_transport_writer_new() {
        let writer = DefaultTransportWriter::new();
        assert_eq!(writer.options(), WriteOptions::default());
    }

    #[test]
    fn test_default_transport_writer_with_options() {
        let opts = WriteOptions::new().with_version(1);
        let writer = DefaultTransportWriter::with_options(opts);
        assert_eq!(writer.options().version, 1);
    }

    #[test]
    fn test_write_no_crypto() {
        let writer = DefaultTransportWriter::new();
        let data = vec![1u8, 2, 3, 4];
        let packet_info = PacketInfo::new().with_no_crypto(true);

        let result = writer.write_no_crypto(&data, &packet_info);

        assert!(result.is_ok());
        let packet = result.unwrap();
        assert!(packet.len() > data.len());
        assert_eq!(&packet[8..], &data[..]);
    }

    #[test]
    fn test_write_no_crypto_empty() {
        let writer = DefaultTransportWriter::new();
        let data = vec![];
        let packet_info = PacketInfo::new();

        let result = writer.write_no_crypto(&data, &packet_info);

        assert!(matches!(result, Err(TransportWriteError::EmptyData)));
    }

    #[test]
    fn test_calc_crypto_size_v1() {
        // CryptoHeader raw size = auth_key_id (8) + message_key (16) = 24
        // encrypted_header_size = salt (8) + session_id (8) = 16
        let size = DefaultTransportWriter::calc_crypto_size_v1(100, 16, 24);
        // encrypted_size = align_up(16 + 100, 16) = align_up(116, 16) = 128
        // total = 24 + 128 = 152
        assert_eq!(size, 152);
    }

    #[test]
    fn test_calc_crypto_size_v2() {
        let size = DefaultTransportWriter::calc_crypto_size_v2(100, 16, 8, false);
        // Should align to one of the predefined sizes
        assert!(size >= 100 + 16 + 8);
    }

    #[test]
    fn test_compute_auth_key_id() {
        let key = [42u8; 256];
        let id = compute_auth_key_id(&key);
        assert_ne!(id, 0);

        let id2 = compute_auth_key_id(&key);
        assert_eq!(id, id2);
    }

    #[test]
    fn test_calc_message_key_v2() {
        let auth_key = [1u8; 256];
        let to_encrypt = vec![2u8; 64];

        let (msg_key, ack) = DefaultTransportWriter::calc_message_key_v2(&auth_key, 0, &to_encrypt);

        assert_ne!(msg_key, [0u8; 16]);
        assert!(ack & (1 << 31) != 0); // Bit 31 should be set
    }

    #[test]
    fn test_write_crypto_invalid_length() {
        let writer =
            DefaultTransportWriter::with_options(WriteOptions::new().with_check_mod4(true));
        let data = vec![1u8, 2, 3]; // Not divisible by 4
        let auth_key = [0u8; 256];
        let mut packet_info = PacketInfo::new()
            .with_salt(0)
            .with_session_id(0)
            .with_message_id(MessageId::from_u64(1))
            .with_seq_no(0);

        let result = writer.write_crypto(&data, &auth_key, &mut packet_info);

        assert!(matches!(
            result,
            Err(TransportWriteError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_write_crypto_empty() {
        let writer = DefaultTransportWriter::new();
        let data = vec![];
        let auth_key = [0u8; 256];
        let mut packet_info = PacketInfo::new();

        let result = writer.write_crypto(&data, &auth_key, &mut packet_info);

        assert!(matches!(result, Err(TransportWriteError::EmptyData)));
    }

    #[test]
    fn test_write_e2e_crypto_empty() {
        let writer = DefaultTransportWriter::new();
        let data = vec![];
        let auth_key = [0u8; 256];
        let mut packet_info = PacketInfo::end_to_end();

        let result = writer.write_e2e_crypto(&data, &auth_key, &mut packet_info);

        assert!(matches!(result, Err(TransportWriteError::EmptyData)));
    }
}
