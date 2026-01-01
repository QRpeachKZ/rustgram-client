// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! HTTP transport for MTProto.
//!
//! This module implements HTTP-based transport for Telegram MTProto.

use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::connection::{ConnectionError, ConnectionState};
use crate::packet::PacketInfo;
use crate::transport::{ReadResult, TransportRead, TransportWrite, WriteOptions};

/// HTTP content type for MTProto.
const CONTENT_TYPE: &str = "application/x-tlantic";

/// HTTP transport.
///
/// Handles HTTP connections for MTProto (used in some regions/proxies).
pub struct HttpTransport {
    /// Underlying TCP stream
    stream: Option<TcpStream>,

    /// Remote address
    addr: SocketAddr,

    /// Connection state
    state: ConnectionState,

    /// Transport reader
    reader: Box<dyn TransportRead>,

    /// Transport writer
    writer: Box<dyn TransportWrite>,

    /// Write options
    write_options: WriteOptions,

    /// HTTP host header
    host: String,

    /// Whether to use HTTPS
    use_https: bool,
}

impl HttpTransport {
    /// Creates a new HTTP transport (not connected).
    pub fn new(addr: SocketAddr, host: String) -> Self {
        Self {
            stream: None,
            addr,
            state: ConnectionState::Empty,
            reader: Box::new(crate::transport::read::DefaultTransportReader::new()),
            writer: Box::new(crate::transport::write::DefaultTransportWriter::new()),
            write_options: WriteOptions::default(),
            host,
            use_https: false,
        }
    }

    /// Creates a new HTTPS transport.
    pub fn https(addr: SocketAddr, host: String) -> Self {
        Self {
            stream: None,
            addr,
            state: ConnectionState::Empty,
            reader: Box::new(crate::transport::read::DefaultTransportReader::new()),
            writer: Box::new(crate::transport::write::DefaultTransportWriter::new()),
            write_options: WriteOptions::default(),
            host,
            use_https: true,
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

    /// Returns `true` if using HTTPS.
    pub fn is_https(&self) -> bool {
        self.use_https
    }

    /// Returns the host header.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Sets the write options.
    pub fn set_write_options(&mut self, options: WriteOptions) {
        self.write_options = options;
    }

    /// Connects to the remote address.
    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        self.state = ConnectionState::Connecting;

        let stream = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            TcpStream::connect(self.addr),
        )
        .await
        .map_err(|_| ConnectionError::Timeout(std::time::Duration::from_secs(10)))?
        .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        stream
            .set_nodelay(true)
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        self.stream = Some(stream);
        self.state = ConnectionState::Ready;

        tracing::debug!("HTTP transport connected to {}", self.addr);

        Ok(())
    }

    /// Writes an HTTP request with MTProto data.
    pub async fn write(
        &mut self,
        data: &[u8],
        auth_key: Option<&[u8; 256]>,
    ) -> Result<(), ConnectionError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        // Encode MTProto packet
        let mut packet_info = PacketInfo::new()
            .with_no_crypto(auth_key.is_none())
            .with_packet_type(self.write_options.packet_type);

        let packet = self
            .writer
            .write(data, auth_key, &mut packet_info)
            .map_err(|e| ConnectionError::Ssl(e.to_string()))?;

        // Build HTTP request
        let scheme = if self.use_https { "https" } else { "http" };
        let url = format!("{}://{}{}", scheme, self.host, "/api");

        let request = format!(
            "POST {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: keep-alive\r\n\
             \r\n",
            url,
            self.host,
            CONTENT_TYPE,
            packet.len()
        );

        // Send request headers and body
        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        stream
            .write_all(&packet)
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        stream
            .flush()
            .await
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        tracing::trace!("HTTP transport wrote {} bytes", packet.len());

        Ok(())
    }

    /// Reads an HTTP response with MTProto data.
    pub async fn read(
        &mut self,
        auth_key: Option<&[u8; 256]>,
    ) -> Result<ReadResult, ConnectionError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        // Read HTTP response line
        let mut response_line = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            tokio::time::timeout(
                std::time::Duration::from_secs(15),
                stream.read_exact(&mut byte),
            )
            .await
            .map_err(|_| ConnectionError::Timeout(std::time::Duration::from_secs(15)))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

            response_line.push(byte[0]);

            if response_line.len() >= 4 && &response_line[response_line.len() - 4..] == b"\r\n\r\n"
            {
                break;
            }

            if response_line.len() > 8192 {
                return Err(ConnectionError::Failed("HTTP header too large".into()));
            }
        }

        // Parse response line
        let response_str = String::from_utf8_lossy(&response_line);
        if !response_str.starts_with("HTTP/1.") {
            return Err(ConnectionError::Failed("Invalid HTTP response".into()));
        }

        // Parse content length
        let content_length = response_str
            .lines()
            .find(|line| line.to_lowercase().starts_with("content-length:"))
            .and_then(|line| {
                line.split(':')
                    .nth(1)
                    .map(|s| s.trim().parse::<usize>().ok())
            })
            .flatten()
            .ok_or_else(|| ConnectionError::Failed("No Content-Length header".into()))?;

        if content_length > 16 * 1024 * 1024 {
            return Err(ConnectionError::Failed(format!(
                "Response too large: {} bytes",
                content_length
            )));
        }

        // Read response body
        let mut buffer = vec![0u8; content_length];
        tokio::time::timeout(
            std::time::Duration::from_secs(15),
            stream.read_exact(&mut buffer),
        )
        .await
        .map_err(|_| ConnectionError::Timeout(std::time::Duration::from_secs(15)))?
        .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // Decode MTProto packet
        let mut packet_info = PacketInfo::new()
            .with_no_crypto(auth_key.is_none())
            .with_packet_type(self.write_options.packet_type);

        let result = self
            .reader
            .read(&buffer, auth_key, &mut packet_info)
            .map_err(|e| ConnectionError::Failed(e.to_string()))?;

        tracing::trace!("HTTP transport read packet: {:?}", result);

        Ok(result)
    }

    /// Closes the HTTP connection.
    pub async fn close(&mut self) -> Result<(), ConnectionError> {
        if let Some(mut stream) = self.stream.take() {
            stream
                .shutdown()
                .await
                .map_err(|e| ConnectionError::Socket(e.to_string()))?;
        }

        self.state = ConnectionState::Closed;

        tracing::debug!("HTTP transport closed connection to {}", self.addr);

        Ok(())
    }
}

/// HTTP transport factory.
pub struct HttpTransportFactory;

impl HttpTransportFactory {
    /// Creates a new HTTP transport and connects it.
    pub async fn connect(addr: SocketAddr, host: String) -> Result<HttpTransport, ConnectionError> {
        let mut transport = HttpTransport::new(addr, host);
        transport.connect().await?;
        Ok(transport)
    }

    /// Creates a new HTTPS transport and connects it.
    pub async fn connect_https(
        addr: SocketAddr,
        host: String,
    ) -> Result<HttpTransport, ConnectionError> {
        let mut transport = HttpTransport::https(addr, host);
        transport.connect().await?;
        Ok(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_http_transport_new() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);
        let transport = HttpTransport::new(addr, "example.com".into());

        assert_eq!(transport.addr(), addr);
        assert!(!transport.is_connected());
        assert!(!transport.is_https());
        assert_eq!(transport.host(), "example.com");
    }

    #[test]
    fn test_http_transport_https() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 443);
        let transport = HttpTransport::https(addr, "example.com".into());

        assert!(transport.is_https());
    }

    #[test]
    fn test_content_type() {
        assert_eq!(CONTENT_TYPE, "application/x-tlantic");
    }

    #[tokio::test]
    async fn test_http_transport_close_when_not_connected() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);
        let mut transport = HttpTransport::new(addr, "example.com".into());

        let result = transport.close().await;
        assert!(result.is_ok());
        assert_eq!(transport.state(), ConnectionState::Closed);
    }
}
