// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! HTTP CONNECT proxy transport for MTProto.
//!
//! This module implements HTTP CONNECT proxy support for Telegram MTProto.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::connection::{ConnectionError, ConnectionState};
use crate::proxy::Proxy;
use crate::transport::tcp::TcpTransport;

/// HTTP CONNECT proxy transport.
pub struct HttpProxyTransport {
    /// Underlying TCP transport through proxy
    transport: Option<TcpTransport>,

    /// Target address
    target: SocketAddr,

    /// Proxy configuration
    proxy: Proxy,

    /// Connection state
    state: ConnectionState,
}

impl HttpProxyTransport {
    /// Creates a new HTTP CONNECT proxy transport.
    pub fn new(proxy: Proxy, target: SocketAddr) -> Result<Self, crate::proxy::ProxyError> {
        proxy.validate()?;

        if !proxy.use_http_tcp_proxy() {
            return Err(crate::proxy::ProxyError::InvalidType(
                "Not an HTTP proxy".into(),
            ));
        }

        Ok(Self {
            transport: None,
            target,
            proxy,
            state: ConnectionState::Empty,
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
        self.transport
            .as_ref()
            .map(|t| t.is_connected())
            .unwrap_or(false)
    }

    /// Connects through the HTTP CONNECT proxy.
    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        self.state = ConnectionState::Connecting;

        // Parse proxy address
        let proxy_addr = self.parse_proxy_addr().await?;

        // Connect to proxy
        let stream = timeout(Duration::from_secs(10), TcpStream::connect(proxy_addr))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(10)))?
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        // Perform HTTP CONNECT handshake
        let stream = self.http_connect_handshake(stream).await?;

        // Wrap in TcpTransport
        let mut tcp_transport = TcpTransport::new(self.target);
        tcp_transport.stream = Some(stream);
        tcp_transport.state = ConnectionState::Ready;

        self.transport = Some(tcp_transport);
        self.state = ConnectionState::Ready;

        tracing::debug!(
            "HTTP CONNECT proxy transport connected to {} via {}",
            self.target,
            proxy_addr
        );

        Ok(())
    }

    /// Parses the proxy server address.
    async fn parse_proxy_addr(&self) -> Result<SocketAddr, ConnectionError> {
        let addr_str = format!("{}:{}", self.proxy.server, self.proxy.port);

        // Try to parse as SocketAddr
        if let Ok(addr) = addr_str.parse::<SocketAddr>() {
            return Ok(addr);
        }

        // Try to resolve hostname
        tokio::net::lookup_host(addr_str)
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?
            .next()
            .ok_or_else(|| ConnectionError::Proxy("No addresses found".into()))
    }

    /// Performs HTTP CONNECT handshake.
    async fn http_connect_handshake(
        &self,
        mut stream: TcpStream,
    ) -> Result<TcpStream, ConnectionError> {
        // Build CONNECT request
        let target_host = format!("{}", self.target.ip());
        let target_port = self.target.port();

        let mut request = format!(
            "CONNECT {}:{} HTTP/1.1\r\n\
             Host: {}:{}\r\n\
             User-Agent: MTProxy/1.0\r\n\
             Proxy-Connection: keep-alive\r\n",
            target_host, target_port, target_host, target_port
        );

        // Add proxy authentication if provided
        if let (Some(user), Some(pass)) = (&self.proxy.user, &self.proxy.password) {
            let credentials = format!("{}:{}", user, pass);
            let encoded = base64::encode(&credentials);
            request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", encoded));
        }

        request.push_str("\r\n");

        // Send request
        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        stream
            .flush()
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        // Read response
        let mut reader = BufReader::new(&mut stream);

        let mut status_line = String::new();
        timeout(Duration::from_secs(15), reader.read_line(&mut status_line))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(15)))?
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        // Parse status line
        if !status_line.starts_with("HTTP/1.") {
            return Err(ConnectionError::Proxy(format!(
                "Invalid HTTP response: {}",
                status_line.trim()
            )));
        }

        let parts: Vec<&str> = status_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(ConnectionError::Proxy(format!(
                "Invalid status line: {}",
                status_line.trim()
            )));
        }

        let status_code: u16 = parts[1]
            .parse()
            .map_err(|_| ConnectionError::Proxy("Invalid status code".into()))?;

        if status_code != 200 {
            return Err(ConnectionError::Proxy(format!(
                "CONNECT failed with status {}",
                status_code
            )));
        }

        // Skip remaining headers until empty line
        loop {
            let mut line = String::new();
            timeout(Duration::from_secs(5), reader.read_line(&mut line))
                .await
                .map_err(|_| ConnectionError::Timeout(Duration::from_secs(5)))?
                .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

            if line == "\r\n" || line == "\n" {
                break;
            }
        }

        Ok(stream)
    }

    /// Writes data through the proxy.
    pub async fn write(
        &mut self,
        data: &[u8],
        auth_key: Option<&[u8; 256]>,
    ) -> Result<(), ConnectionError> {
        let transport = self
            .transport
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        transport.write(data, auth_key).await
    }

    /// Reads data from the proxy.
    pub async fn read(
        &mut self,
        auth_key: Option<&[u8; 256]>,
    ) -> Result<crate::transport::ReadResult, ConnectionError> {
        let transport = self
            .transport
            .as_mut()
            .ok_or_else(|| ConnectionError::Failed("Not connected".into()))?;

        transport.read(auth_key).await
    }

    /// Closes the connection.
    pub async fn close(&mut self) -> Result<(), ConnectionError> {
        if let Some(mut transport) = self.transport.take() {
            transport.close().await?;
        }

        self.state = ConnectionState::Closed;

        tracing::debug!(
            "HTTP CONNECT proxy transport closed connection to {}",
            self.target
        );

        Ok(())
    }
}

/// HTTP CONNECT proxy transport factory.
pub struct HttpProxyTransportFactory;

impl HttpProxyTransportFactory {
    /// Creates a new HTTP CONNECT proxy transport and connects it.
    pub async fn connect(
        proxy: Proxy,
        target: SocketAddr,
    ) -> Result<HttpProxyTransport, ConnectionError> {
        let mut transport = HttpProxyTransport::new(proxy, target)
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;
        transport.connect().await?;
        Ok(transport)
    }
}

// Simple base64 encoding for proxy auth
mod base64 {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut result = String::new();

        for chunk in bytes.chunks(3) {
            let mut buffer = [0u8; 3];
            buffer[..chunk.len()].copy_from_slice(chunk);

            let b0 = buffer[0];
            let b1 = if chunk.len() > 1 { buffer[1] } else { 0 };
            let b2 = if chunk.len() > 2 { buffer[2] } else { 0 };

            result.push(TABLE[(b0 >> 2) as usize] as char);
            result.push(TABLE[((b0 << 4 | b1 >> 4) & 0x3F) as usize] as char);

            if chunk.len() > 1 {
                result.push(TABLE[((b1 << 2 | b2 >> 6) & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }

            if chunk.len() > 2 {
                result.push(TABLE[(b2 & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_proxy_transport_new() {
        let proxy = Proxy::http_tcp("127.0.0.1".into(), 8080, None, None);
        let target = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 443);

        let transport = HttpProxyTransport::new(proxy, target).unwrap();

        assert_eq!(transport.target(), target);
        assert!(!transport.is_connected());
    }

    #[test]
    fn test_http_proxy_transport_new_invalid_proxy() {
        let proxy = Proxy::none();
        let target = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 443);

        let result = HttpProxyTransport::new(proxy, target);
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64::encode("user:pass"), "dXNlcjpwYXNz");
        assert_eq!(base64::encode("test"), "dGVzdA==");
        assert_eq!(base64::encode("a"), "YQ==");
        assert_eq!(base64::encode("ab"), "YWI=");
        assert_eq!(base64::encode("abc"), "YWJj");
    }
}
