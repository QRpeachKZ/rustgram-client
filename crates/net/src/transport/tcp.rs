// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! TCP transport for MTProto.
//!
//! This module implements TCP-based transport for Telegram MTProto.

use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::connection::{ConnectionError, ConnectionState};
use crate::packet::PacketInfo;
use crate::transport::{ReadResult, TransportRead, TransportWrite, WriteOptions};

/// Maximum packet size for TCP transport.
pub const MAX_PACKET_SIZE: usize = 16 * 1024 * 1024; // 16 MB

/// Default connection timeout.
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Default read timeout.
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(15);

use std::sync::Arc;

/// MTProto TCP transport.
///
/// Handles TCP connections with MTProto packet framing.
pub struct TcpTransport {
    /// TCP stream
    pub stream: Option<TcpStream>,

    /// Remote address
    pub addr: SocketAddr,

    /// Connection state
    pub state: ConnectionState,

    /// Transport reader
    pub reader: Arc<dyn TransportRead>,

    /// Transport writer
    pub writer: Arc<dyn TransportWrite>,

    /// Write options
    pub write_options: WriteOptions,
}

impl TcpTransport {
    /// Creates a new TCP transport (not connected).
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            stream: None,
            addr,
            state: ConnectionState::Empty,
            reader: Arc::new(crate::transport::read::DefaultTransportReader::new()),
            writer: Arc::new(crate::transport::write::DefaultTransportWriter::new()),
            write_options: WriteOptions::default(),
        }
    }

    /// Creates a new TCP transport with custom transport implementations.
    pub fn with_transport(
        addr: SocketAddr,
        reader: Arc<dyn TransportRead>,
        writer: Arc<dyn TransportWrite>,
    ) -> Self {
        Self {
            stream: None,
            addr,
            state: ConnectionState::Empty,
            reader,
            writer,
            write_options: WriteOptions::default(),
        }
    }

    /// Returns the remote address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
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

    /// Connects to the remote address.
    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        self.state = ConnectionState::Connecting;

        let stream = timeout(DEFAULT_CONNECT_TIMEOUT, TcpStream::connect(self.addr))
            .await
            .map_err(|_| ConnectionError::Timeout(DEFAULT_CONNECT_TIMEOUT))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // Set TCP options
        stream
            .set_nodelay(true)
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        self.stream = Some(stream);
        self.state = ConnectionState::Ready;

        tracing::debug!("TCP transport connected to {}", self.addr);

        Ok(())
    }

    /// Writes data to the TCP stream.
    pub async fn write(
        &mut self,
        data: &[u8],
        auth_key: Option<&[u8; 256]>,
    ) -> Result<(), ConnectionError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        // Encode packet using transport
        let mut packet_info = PacketInfo::new()
            .with_no_crypto(auth_key.is_none())
            .with_packet_type(self.write_options.packet_type);

        let packet = self
            .writer
            .write(data, auth_key, &mut packet_info)
            .map_err(|e| ConnectionError::Ssl(e.to_string()))?;

        // Write to stream
        stream
            .write_all(&packet)
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        stream
            .flush()
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        tracing::trace!("TCP transport wrote {} bytes", packet.len());

        Ok(())
    }

    /// Reads data from the TCP stream.
    pub async fn read(
        &mut self,
        auth_key: Option<&[u8; 256]>,
    ) -> Result<ReadResult, ConnectionError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        // Read packet header first (at least 4 bytes for length)
        let mut header = [0u8; 4];
        timeout(DEFAULT_READ_TIMEOUT, stream.read_exact(&mut header))
            .await
            .map_err(|_| ConnectionError::Timeout(DEFAULT_READ_TIMEOUT))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // Determine packet length
        let length = if header[0] < 0x7f {
            // Abridged mode: 1-byte length
            header[0] as usize
        } else {
            // Intermediate/Full mode: 4-byte length
            let mut rest = [0u8; 3];
            timeout(DEFAULT_READ_TIMEOUT, stream.read_exact(&mut rest))
                .await
                .map_err(|_| ConnectionError::Timeout(DEFAULT_READ_TIMEOUT))?
                .map_err(|e| ConnectionError::Socket(e.to_string()))?;

            u32::from_le_bytes([header[1], header[2], header[3], rest[2]]) as usize
        };

        if length > MAX_PACKET_SIZE {
            return Err(ConnectionError::Failed(format!(
                "Packet too large: {} bytes",
                length
            )));
        }

        // Read packet body
        let mut buffer = vec![0u8; length];
        timeout(DEFAULT_READ_TIMEOUT, stream.read_exact(&mut buffer))
            .await
            .map_err(|_| ConnectionError::Timeout(DEFAULT_READ_TIMEOUT))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // Decode packet using transport
        let mut packet_info = PacketInfo::new()
            .with_no_crypto(auth_key.is_none())
            .with_packet_type(self.write_options.packet_type);

        let result = self
            .reader
            .read(&buffer, auth_key, &mut packet_info)
            .map_err(|e| ConnectionError::Failed(e.to_string()))?;

        tracing::trace!("TCP transport read packet: {:?}", result);

        Ok(result)
    }

    /// Closes the TCP connection.
    pub async fn close(&mut self) -> Result<(), ConnectionError> {
        if let Some(mut stream) = self.stream.take() {
            stream
                .shutdown()
                .await
                .map_err(|e| ConnectionError::Socket(e.to_string()))?;
        }

        self.state = ConnectionState::Closed;

        tracing::debug!("TCP transport closed connection to {}", self.addr);

        Ok(())
    }

    /// Splits the transport into read and write halves.
    pub fn split(&mut self) -> Option<(TcpReadHalf, TcpWriteHalf)> {
        let stream = self.stream.take()?;

        let (read, write) = tokio::io::split(stream);

        Some((
            TcpReadHalf {
                reader: self.reader.clone_box(),
                stream: read,
                addr: self.addr,
            },
            TcpWriteHalf {
                writer: self.writer.clone_box(),
                stream: write,
                addr: self.addr,
                write_options: self.write_options,
            },
        ))
    }
}

/// Clone helper for TransportRead trait.
trait TransportReadClone: Send + Sync {
    fn clone_box(&self) -> Arc<dyn TransportRead>;
}

impl<T: TransportRead + Clone + 'static> TransportReadClone for T {
    fn clone_box(&self) -> Arc<dyn TransportRead> {
        Arc::new(self.clone())
    }
}

impl TransportReadClone for Arc<dyn TransportRead> {
    fn clone_box(&self) -> Arc<dyn TransportRead> {
        Arc::clone(self)
    }
}

/// Clone helper for TransportWrite trait.
trait TransportWriteClone: Send + Sync {
    fn clone_box(&self) -> Arc<dyn TransportWrite>;
}

impl<T: TransportWrite + Clone + 'static> TransportWriteClone for T {
    fn clone_box(&self) -> Arc<dyn TransportWrite> {
        Arc::new(self.clone())
    }
}

impl TransportWriteClone for Arc<dyn TransportWrite> {
    fn clone_box(&self) -> Arc<dyn TransportWrite> {
        Arc::clone(self)
    }
}

/// Read half of TCP transport.
pub struct TcpReadHalf {
    pub reader: Arc<dyn TransportRead>,
    pub stream: ReadHalf<TcpStream>,
    pub addr: SocketAddr,
}

impl TcpReadHalf {
    /// Returns the remote address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Reads a packet from the stream.
    pub async fn read_packet(
        &mut self,
        auth_key: Option<&[u8; 256]>,
        packet_type: crate::packet::PacketType,
    ) -> Result<ReadResult, ConnectionError> {
        // Read packet length
        let mut length_buf = [0u8; 4];
        timeout(
            DEFAULT_READ_TIMEOUT,
            self.stream.read_exact(&mut length_buf),
        )
        .await
        .map_err(|_| ConnectionError::Timeout(DEFAULT_READ_TIMEOUT))?
        .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        let length = if length_buf[0] < 0x7f {
            length_buf[0] as usize
        } else {
            u32::from_le_bytes(length_buf) as usize & 0x7FFFFF
        };

        if length > MAX_PACKET_SIZE {
            return Err(ConnectionError::Failed(format!(
                "Packet too large: {} bytes",
                length
            )));
        }

        // Read packet data
        let mut buffer = vec![0u8; length];
        timeout(DEFAULT_READ_TIMEOUT, self.stream.read_exact(&mut buffer))
            .await
            .map_err(|_| ConnectionError::Timeout(DEFAULT_READ_TIMEOUT))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // Decode
        let mut packet_info = PacketInfo::new().with_packet_type(packet_type);
        self.reader
            .read(&buffer, auth_key, &mut packet_info)
            .map_err(|e| ConnectionError::Failed(e.to_string()))
    }
}

/// Write half of TCP transport.
pub struct TcpWriteHalf {
    pub writer: Arc<dyn TransportWrite>,
    pub stream: WriteHalf<TcpStream>,
    pub addr: SocketAddr,
    pub write_options: WriteOptions,
}

impl TcpWriteHalf {
    /// Returns the remote address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Sets the write options.
    pub fn set_write_options(&mut self, options: WriteOptions) {
        self.write_options = options;
    }

    /// Writes a packet to the stream.
    pub async fn write_packet(
        &mut self,
        data: &[u8],
        auth_key: Option<&[u8; 256]>,
    ) -> Result<(), ConnectionError> {
        let mut packet_info = PacketInfo::new()
            .with_no_crypto(auth_key.is_none())
            .with_packet_type(self.write_options.packet_type);

        let packet = self
            .writer
            .write(data, auth_key, &mut packet_info)
            .map_err(|e| ConnectionError::Ssl(e.to_string()))?;

        self.stream
            .write_all(&packet)
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        self.stream
            .flush()
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        Ok(())
    }
}

/// TCP transport factory.
pub struct TcpTransportFactory;

impl TcpTransportFactory {
    /// Creates a new TCP transport and connects it.
    pub async fn connect(addr: SocketAddr) -> Result<TcpTransport, ConnectionError> {
        let mut transport = TcpTransport::new(addr);
        transport.connect().await?;
        Ok(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_tcp_transport_new() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let transport = TcpTransport::new(addr);

        assert_eq!(transport.addr(), addr);
        assert!(!transport.is_connected());
        assert_eq!(transport.state(), ConnectionState::Empty);
    }

    #[test]
    fn test_max_packet_size() {
        assert_eq!(MAX_PACKET_SIZE, 16 * 1024 * 1024);
    }

    #[tokio::test]
    async fn test_tcp_transport_close_when_not_connected() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut transport = TcpTransport::new(addr);

        // Should not error when closing unconnected transport
        let result = transport.close().await;
        assert!(result.is_ok());
        assert_eq!(transport.state(), ConnectionState::Closed);
    }

    #[test]
    fn test_tcp_transport_factory_type() {
        // Just verify the factory exists
        let _ = TcpTransportFactory;
    }
}
