// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto proxy transport for MTProto.
//!
//! This module implements MTProto proxy (MTPROTO) support for Telegram MTProto.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::connection::{ConnectionError, ConnectionState};
use crate::crypto::{aes_ige_decrypt, aes_ige_encrypt, sha256};
use crate::proxy::Proxy;
use crate::transport::{ReadResult, WriteOptions};

/// MTProto proxy protocol version.
const PROTOCOL_VERSION: u8 = 1;

/// MTProto proxy transport.
pub struct MtprotoProxyTransport {
    /// TCP stream
    stream: Option<TcpStream>,

    /// Target address
    target: SocketAddr,

    /// Proxy configuration
    proxy: Proxy,

    /// Encryption key
    key: [u8; 32],

    /// Encryption nonce
    nonce: [u8; 16],

    /// Connection state
    state: ConnectionState,

    /// Write options
    write_options: WriteOptions,
}

impl MtprotoProxyTransport {
    /// Creates a new MTProto proxy transport.
    pub fn new(proxy: Proxy, target: SocketAddr) -> Result<Self, crate::proxy::ProxyError> {
        proxy.validate()?;

        if !proxy.use_mtproto_proxy() {
            return Err(crate::proxy::ProxyError::InvalidType(
                "Not an MTProto proxy".into(),
            ));
        }

        let secret = proxy
            .secret
            .as_ref()
            .ok_or_else(|| crate::proxy::ProxyError::InvalidAddress("No secret provided".into()))?;

        if secret.len() < 16 {
            return Err(crate::proxy::ProxyError::InvalidAddress(
                "Secret too short (minimum 16 bytes)".into(),
            ));
        }

        let mut key = [0u8; 32];
        let mut nonce = [0u8; 16];

        // Parse secret: format is "dd[secret]" where dd is two hex digits
        let secret_bytes = if secret.len() >= 17 {
            // Skip first byte (protocol indicator)
            &secret[1..]
        } else {
            secret
        };

        // Extract key and nonce from secret
        let key_len = key.len().min(secret_bytes.len());
        key[..key_len].copy_from_slice(&secret_bytes[..key_len]);

        if secret_bytes.len() > 32 {
            let nonce_len = nonce.len().min(secret_bytes.len() - 32);
            nonce[..nonce_len].copy_from_slice(&secret_bytes[32..32 + nonce_len]);
        }

        Ok(Self {
            stream: None,
            target,
            proxy,
            key,
            nonce,
            state: ConnectionState::Empty,
            write_options: WriteOptions::default(),
        })
    }

    /// Returns the target address.
    pub fn target(&self) -> SocketAddr {
        self.target
    }

    /// Returns the proxy configuration.
    pub fn proxy(&self) -> &Proxy {
        &self.proxy
    }

    /// Returns the connection state.
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Returns `true` if connected.
    pub fn is_connected(&self) -> bool {
        self.stream.is_some() && self.state == ConnectionState::Ready
    }

    /// Sets the write options.
    pub fn set_write_options(&mut self, options: WriteOptions) {
        self.write_options = options;
    }

    /// Connects through the MTProto proxy.
    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        self.state = ConnectionState::Connecting;

        // Parse proxy address
        let proxy_addr = self.parse_proxy_addr().await?;

        // Connect to proxy
        let mut stream = timeout(Duration::from_secs(10), TcpStream::connect(proxy_addr))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(10)))?
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        stream
            .set_nodelay(true)
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // Perform MTProto proxy handshake
        self.mtproto_handshake(&mut stream).await?;

        self.stream = Some(stream);
        self.state = ConnectionState::Ready;

        tracing::debug!(
            "MTProto proxy transport connected to {} via {}",
            self.target,
            proxy_addr
        );

        Ok(())
    }

    /// Parses the proxy server address.
    async fn parse_proxy_addr(&self) -> Result<SocketAddr, ConnectionError> {
        let addr_str = format!("{}:{}", self.proxy.server, self.proxy.port);

        if let Ok(addr) = addr_str.parse::<SocketAddr>() {
            return Ok(addr);
        }

        tokio::net::lookup_host(addr_str)
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?
            .next()
            .ok_or_else(|| ConnectionError::Proxy("No addresses found".into()))
    }

    /// Performs MTProto proxy handshake.
    async fn mtproto_handshake(&self, stream: &mut TcpStream) -> Result<(), ConnectionError> {
        // Build handshake packet
        let mut packet = BytesMut::with_capacity(64);

        // Timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);

        packet.put_u32_le(timestamp);

        // Protocol version
        packet.put_u8(PROTOCOL_VERSION);

        // DC ID (extracted from target)
        let dc_id = self.extract_dc_id()?;
        packet.put_u32_le(dc_id);

        // IP address
        match self.target.ip() {
            IpAddr::V4(addr) => {
                packet.put_u8(4); // IPv4
                packet.put_slice(&addr.octets());
            }
            IpAddr::V6(addr) => {
                packet.put_u8(6); // IPv6
                packet.put_slice(&addr.octets());
            }
        }

        // Port
        packet.put_u16_le(self.target.port());

        // Random padding
        let padding_len = 15; // Total header should be 64 bytes
        let current_len = packet.len();
        for _ in 0..(padding_len - (current_len % padding_len)) {
            packet.put_u8(rand::random());
        }

        // Encrypt handshake
        let encrypted = self.encrypt_packet(packet.freeze())?;

        // Send handshake
        stream
            .write_all(&encrypted)
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        stream
            .flush()
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        Ok(())
    }

    /// Extracts DC ID from target address (for MTProto proxy protocol).
    fn extract_dc_id(&self) -> Result<u32, ConnectionError> {
        // For MTProto proxy, DC ID is typically encoded in the target
        // For now, use a default value
        Ok(2)
    }

    /// Encrypts a packet with MTProto proxy encryption.
    fn encrypt_packet(&self, data: Bytes) -> Result<Vec<u8>, ConnectionError> {
        // Derive AES key and IV using KDF
        let hash_input = {
            let mut input = Vec::with_capacity(48);
            input.extend_from_slice(&self.key);
            input.extend_from_slice(&self.nonce);
            input
        };

        let hash = sha256(&hash_input);

        let mut aes_key = [0u8; 32];
        let mut aes_iv = [0u8; 32];

        aes_key.copy_from_slice(&hash[..32]);
        aes_iv.copy_from_slice(&hash[16..48]);

        // Encrypt using AES-IGE
        let mut encrypted = data.to_vec();

        // Pad to block size
        while encrypted.len() % 16 != 0 {
            encrypted.push(0);
        }

        aes_ige_encrypt(&aes_key, &mut aes_iv, &mut encrypted)
            .map_err(|e| ConnectionError::Ssl(e.to_string()))?;

        Ok(encrypted)
    }

    /// Decrypts a packet with MTProto proxy decryption.
    fn decrypt_packet(&self, data: &mut [u8]) -> Result<(), ConnectionError> {
        // Same key derivation as encryption
        let hash_input = {
            let mut input = Vec::with_capacity(48);
            input.extend_from_slice(&self.key);
            input.extend_from_slice(&self.nonce);
            input
        };

        let hash = sha256(&hash_input);

        let mut aes_key = [0u8; 32];
        let mut aes_iv = [0u8; 32];

        aes_key.copy_from_slice(&hash[..32]);
        aes_iv.copy_from_slice(&hash[16..48]);

        aes_ige_decrypt(&aes_key, &mut aes_iv, data)
            .map_err(|e| ConnectionError::Ssl(e.to_string()))?;

        Ok(())
    }

    /// Writes data through the proxy.
    pub async fn write(
        &mut self,
        data: &[u8],
        _auth_key: Option<&[u8; 256]>,
    ) -> Result<(), ConnectionError> {
        // Encrypt with MTProto proxy encryption
        let encrypted = self.encrypt_packet(Bytes::copy_from_slice(data))?;

        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        stream
            .write_all(&encrypted)
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        stream
            .flush()
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        tracing::trace!("MTProto proxy transport wrote {} bytes", data.len());

        Ok(())
    }

    /// Reads data from the proxy.
    pub async fn read(
        &mut self,
        _auth_key: Option<&[u8; 256]>,
    ) -> Result<ReadResult, ConnectionError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        // Read packet length (4 bytes)
        let mut len_buf = [0u8; 4];
        timeout(Duration::from_secs(15), stream.read_exact(&mut len_buf))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(15)))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // In MTProto proxy, length is in the packet, not a prefix
        // For now, assume fixed buffer size
        let mut buffer = vec![0u8; 4096];
        let n = timeout(Duration::from_secs(15), stream.read(&mut buffer))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(15)))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        if n == 0 {
            return Ok(ReadResult::nop());
        }

        buffer.truncate(n);

        // Decrypt
        self.decrypt_packet(&mut buffer)?;

        // Return as packet (skip MTProto proxy headers if any)
        Ok(ReadResult::packet(buffer))
    }

    /// Closes the connection.
    pub async fn close(&mut self) -> Result<(), ConnectionError> {
        if let Some(mut stream) = self.stream.take() {
            stream
                .shutdown()
                .await
                .map_err(|e| ConnectionError::Socket(e.to_string()))?;
        }

        self.state = ConnectionState::Closed;

        tracing::debug!(
            "MTProto proxy transport closed connection to {}",
            self.target
        );

        Ok(())
    }
}

/// MTProto proxy transport factory.
pub struct MtprotoProxyTransportFactory;

impl MtprotoProxyTransportFactory {
    /// Creates a new MTProto proxy transport and connects it.
    pub async fn connect(
        proxy: Proxy,
        target: SocketAddr,
    ) -> Result<MtprotoProxyTransport, ConnectionError> {
        let mut transport = MtprotoProxyTransport::new(proxy, target)
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;
        transport.connect().await?;
        Ok(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtproto_proxy_transport_new() {
        let secret = vec![
            0xDDu8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let proxy = Proxy::mtproto("127.0.0.1".into(), 1080, secret);
        let target = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 443);

        let transport = MtprotoProxyTransport::new(proxy, target).unwrap();

        assert_eq!(transport.target(), target);
        assert!(!transport.is_connected());
    }

    #[test]
    fn test_mtproto_proxy_transport_new_invalid_proxy() {
        let proxy = Proxy::none();
        let target = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 443);

        let result = MtprotoProxyTransport::new(proxy, target);
        assert!(result.is_err());
    }

    #[test]
    fn test_protocol_version() {
        assert_eq!(PROTOCOL_VERSION, 1);
    }
}
