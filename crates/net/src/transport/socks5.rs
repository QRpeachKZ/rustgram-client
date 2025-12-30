// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! SOCKS5 proxy transport for MTProto.
//!
//! This module implements SOCKS5 proxy support for Telegram MTProto.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::connection::{ConnectionError, ConnectionState};
use crate::proxy::{Proxy, ProxyError};
use crate::transport::tcp::TcpTransport;

/// SOCKS5 authentication methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Socks5AuthMethod {
    /// No authentication
    None = 0x00,
    /// Username/password
    UserPass = 0x02,
    /// No acceptable methods
    NoAcceptable = 0xFF,
}

impl TryFrom<u8> for Socks5AuthMethod {
    type Error = ProxyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::None),
            0x02 => Ok(Self::UserPass),
            0xFF => Ok(Self::NoAcceptable),
            _ => Err(ProxyError::InvalidType(format!(
                "Unknown auth method: 0x{:02X}",
                value
            ))),
        }
    }
}

/// SOCKS5 address types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Socks5AddrType {
    /// IPv4
    Ipv4 = 0x01,
    /// Domain name
    Domain = 0x03,
    /// IPv6
    Ipv6 = 0x04,
}

impl TryFrom<u8> for Socks5AddrType {
    type Error = ProxyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Ipv4),
            0x03 => Ok(Self::Domain),
            0x04 => Ok(Self::Ipv6),
            _ => Err(ProxyError::InvalidType(format!(
                "Unknown address type: 0x{:02X}",
                value
            ))),
        }
    }
}

/// SOCKS5 command codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Socks5Command {
    /// Connect
    Connect = 0x01,
    /// Bind
    Bind = 0x02,
    /// UDP associate
    UdpAssociate = 0x03,
}

/// SOCKS5 reply codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Socks5Reply {
    Success = 0x00,
    GeneralFailure = 0x01,
    ConnectionNotAllowed = 0x02,
    NetworkUnreachable = 0x03,
    HostUnreachable = 0x04,
    ConnectionRefused = 0x05,
    TtlExpired = 0x06,
    CommandNotSupported = 0x07,
    AddressTypeNotSupported = 0x08,
}

impl Socks5Reply {
    fn as_error(&self) -> ProxyError {
        match self {
            Self::Success => ProxyError::ConnectionFailed("Unexpected success reply".into()),
            Self::GeneralFailure => {
                ProxyError::ConnectionFailed("General SOCKS server failure".into())
            }
            Self::ConnectionNotAllowed => {
                ProxyError::ConnectionFailed("Connection not allowed by ruleset".into())
            }
            Self::NetworkUnreachable => ProxyError::ConnectionFailed("Network unreachable".into()),
            Self::HostUnreachable => ProxyError::ConnectionFailed("Host unreachable".into()),
            Self::ConnectionRefused => ProxyError::ConnectionFailed("Connection refused".into()),
            Self::TtlExpired => ProxyError::ConnectionFailed("TTL expired".into()),
            Self::CommandNotSupported => {
                ProxyError::ConnectionFailed("Command not supported".into())
            }
            Self::AddressTypeNotSupported => {
                ProxyError::ConnectionFailed("Address type not supported".into())
            }
        }
    }
}

impl std::convert::TryFrom<u8> for Socks5Reply {
    type Error = ProxyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Success),
            0x01 => Ok(Self::GeneralFailure),
            0x02 => Ok(Self::ConnectionNotAllowed),
            0x03 => Ok(Self::NetworkUnreachable),
            0x04 => Ok(Self::HostUnreachable),
            0x05 => Ok(Self::ConnectionRefused),
            0x06 => Ok(Self::TtlExpired),
            0x07 => Ok(Self::CommandNotSupported),
            0x08 => Ok(Self::AddressTypeNotSupported),
            _ => Err(ProxyError::ConnectionFailed(format!(
                "Unknown SOCKS5 reply code: {}",
                value
            ))),
        }
    }
}

/// SOCKS5 proxy transport.
pub struct Socks5Transport {
    /// Underlying TCP transport through proxy
    transport: Option<TcpTransport>,

    /// Target address
    target: SocketAddr,

    /// Proxy configuration
    proxy: Proxy,

    /// Connection state
    state: ConnectionState,
}

impl Socks5Transport {
    /// Creates a new SOCKS5 transport.
    pub fn new(proxy: Proxy, target: SocketAddr) -> Result<Self, ProxyError> {
        proxy.validate()?;

        if !proxy.use_socks5_proxy() {
            return Err(ProxyError::InvalidType("Not a SOCKS5 proxy".into()));
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

    /// Connects through the SOCKS5 proxy.
    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        self.state = ConnectionState::Connecting;

        // Parse proxy address
        let proxy_addr = self.parse_proxy_addr().await?;

        // Connect to proxy
        let mut transport = timeout(Duration::from_secs(10), TcpStream::connect(proxy_addr))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(10)))?
            .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        // Perform SOCKS5 handshake
        self.socks5_handshake(&mut transport).await?;

        // Wrap in TcpTransport
        let mut tcp_transport = TcpTransport::new(self.target);
        tcp_transport.stream = Some(transport);
        tcp_transport.state = ConnectionState::Ready;

        self.transport = Some(tcp_transport);
        self.state = ConnectionState::Ready;

        tracing::debug!(
            "SOCKS5 transport connected to {} via {}",
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

    /// Performs SOCKS5 handshake.
    async fn socks5_handshake(&self, stream: &mut TcpStream) -> Result<(), ConnectionError> {
        // Determine auth methods
        let methods = if self.proxy.user.is_some() || self.proxy.password.is_some() {
            vec![Socks5AuthMethod::UserPass, Socks5AuthMethod::None]
        } else {
            vec![Socks5AuthMethod::None]
        };

        // Send greeting
        let mut greeting = vec![0x05u8]; // SOCKS version
        greeting.push(methods.len() as u8);
        for method in &methods {
            greeting.push(*method as u8);
        }

        stream
            .write_all(&greeting)
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        // Read server response
        let mut response = [0u8; 2];
        timeout(Duration::from_secs(5), stream.read_exact(&mut response))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(5)))?
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        if response[0] != 0x05 {
            return Err(ConnectionError::Proxy(format!(
                "Invalid SOCKS version: 0x{:02X}",
                response[0]
            )));
        }

        let selected_method = Socks5AuthMethod::try_from(response[1])
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        // Handle authentication
        match selected_method {
            Socks5AuthMethod::None => {
                // No authentication needed
            }
            Socks5AuthMethod::UserPass => {
                self.socks5_auth(stream).await?;
            }
            Socks5AuthMethod::NoAcceptable => {
                return Err(ConnectionError::Proxy(
                    "No acceptable authentication method".into(),
                ));
            }
        }

        // Send connect request
        self.socks5_connect(stream).await?;

        Ok(())
    }

    /// Performs SOCKS5 username/password authentication.
    async fn socks5_auth(&self, stream: &mut TcpStream) -> Result<(), ConnectionError> {
        let user = self.proxy.user.as_deref().unwrap_or("");
        let pass = self.proxy.password.as_deref().unwrap_or("");

        let mut auth_req = vec![0x01u8]; // Username/password auth version
        auth_req.push(user.len() as u8);
        auth_req.extend_from_slice(user.as_bytes());
        auth_req.push(pass.len() as u8);
        auth_req.extend_from_slice(pass.as_bytes());

        stream
            .write_all(&auth_req)
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        // Read response
        let mut response = [0u8; 2];
        timeout(Duration::from_secs(5), stream.read_exact(&mut response))
            .await
            .map_err(|_| ConnectionError::Timeout(Duration::from_secs(5)))?
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        if response[0] != 0x01 {
            return Err(ConnectionError::Proxy(
                "Invalid auth response version".into(),
            ));
        }

        if response[1] != 0x00 {
            return Err(ConnectionError::Proxy("Authentication failed".into()));
        }

        Ok(())
    }

    /// Sends SOCKS5 CONNECT request.
    async fn socks5_connect(&self, stream: &mut TcpStream) -> Result<(), ConnectionError> {
        let mut request = vec![0x05u8]; // SOCKS version
        request.push(Socks5Command::Connect as u8);
        request.push(0x00); // Reserved

        // Add destination address
        match self.target.ip() {
            IpAddr::V4(addr) => {
                request.push(Socks5AddrType::Ipv4 as u8);
                request.extend_from_slice(&addr.octets());
            }
            IpAddr::V6(addr) => {
                request.push(Socks5AddrType::Ipv6 as u8);
                request.extend_from_slice(&addr.octets());
            }
        }

        request.extend_from_slice(&self.target.port().to_be_bytes());

        stream
            .write_all(&request)
            .await
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        // Read response
        let mut response_header = [0u8; 4];
        timeout(
            Duration::from_secs(10),
            stream.read_exact(&mut response_header),
        )
        .await
        .map_err(|_| ConnectionError::Timeout(Duration::from_secs(10)))?
        .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        if response_header[0] != 0x05 {
            return Err(ConnectionError::Proxy(format!(
                "Invalid response version: 0x{:02X}",
                response_header[0]
            )));
        }

        let reply = Socks5Reply::try_from(response_header[1])
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        if reply != Socks5Reply::Success {
            return Err(ConnectionError::Proxy(reply.as_error().to_string()));
        }

        // Skip the rest of the response (bound address and port)
        let addr_type = Socks5AddrType::try_from(response_header[3])
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;

        match addr_type {
            Socks5AddrType::Ipv4 => {
                let mut buf = [0u8; 4 + 2]; // IPv4 + port
                timeout(Duration::from_secs(5), stream.read_exact(&mut buf))
                    .await
                    .map_err(|_| ConnectionError::Timeout(Duration::from_secs(5)))?
                    .map_err(|e| ConnectionError::Proxy(e.to_string()))?;
            }
            Socks5AddrType::Ipv6 => {
                let mut buf = [0u8; 16 + 2]; // IPv6 + port
                timeout(Duration::from_secs(5), stream.read_exact(&mut buf))
                    .await
                    .map_err(|_| ConnectionError::Timeout(Duration::from_secs(5)))?
                    .map_err(|e| ConnectionError::Proxy(e.to_string()))?;
            }
            Socks5AddrType::Domain => {
                let mut len_buf = [0u8; 1];
                timeout(Duration::from_secs(5), stream.read_exact(&mut len_buf))
                    .await
                    .map_err(|_| ConnectionError::Timeout(Duration::from_secs(5)))?
                    .map_err(|e| ConnectionError::Proxy(e.to_string()))?;
                let domain_len = len_buf[0] as usize;
                let mut buf = vec![0u8; domain_len + 2]; // domain + port
                timeout(Duration::from_secs(5), stream.read_exact(&mut buf))
                    .await
                    .map_err(|_| ConnectionError::Timeout(Duration::from_secs(5)))?
                    .map_err(|e| ConnectionError::Proxy(e.to_string()))?;
            }
        }

        Ok(())
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

        tracing::debug!("SOCKS5 transport closed connection to {}", self.target);

        Ok(())
    }
}

/// SOCKS5 transport factory.
pub struct Socks5TransportFactory;

impl Socks5TransportFactory {
    /// Creates a new SOCKS5 transport and connects it.
    pub async fn connect(
        proxy: Proxy,
        target: SocketAddr,
    ) -> Result<Socks5Transport, ConnectionError> {
        let mut transport = Socks5Transport::new(proxy, target)
            .map_err(|e| ConnectionError::Proxy(e.to_string()))?;
        transport.connect().await?;
        Ok(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socks5_auth_method_try_from() {
        assert_eq!(
            Socks5AuthMethod::try_from(0x00).unwrap(),
            Socks5AuthMethod::None
        );
        assert_eq!(
            Socks5AuthMethod::try_from(0x02).unwrap(),
            Socks5AuthMethod::UserPass
        );
        assert_eq!(
            Socks5AuthMethod::try_from(0xFF).unwrap(),
            Socks5AuthMethod::NoAcceptable
        );
        assert!(Socks5AuthMethod::try_from(0x99).is_err());
    }

    #[test]
    fn test_socks5_addr_type_try_from() {
        assert_eq!(
            Socks5AddrType::try_from(0x01).unwrap(),
            Socks5AddrType::Ipv4
        );
        assert_eq!(
            Socks5AddrType::try_from(0x03).unwrap(),
            Socks5AddrType::Domain
        );
        assert_eq!(
            Socks5AddrType::try_from(0x04).unwrap(),
            Socks5AddrType::Ipv6
        );
        assert!(Socks5AddrType::try_from(0x99).is_err());
    }

    #[test]
    fn test_socks5_reply_as_error() {
        let reply = Socks5Reply::Success;
        assert!(matches!(reply.as_error(), ProxyError::ConnectionFailed(_)));

        let reply = Socks5Reply::ConnectionRefused;
        let err = reply.as_error();
        assert!(matches!(err, ProxyError::ConnectionFailed(_)));
        assert!(err.to_string().contains("refused"));
    }

    #[test]
    fn test_socks5_transport_new() {
        let proxy = Proxy::socks5("127.0.0.1".into(), 1080, None, None);
        let target = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 443);

        let transport = Socks5Transport::new(proxy, target).unwrap();

        assert_eq!(transport.target(), target);
        assert!(!transport.is_connected());
    }

    #[test]
    fn test_socks5_transport_new_invalid_proxy() {
        let proxy = Proxy::none();
        let target = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 443);

        let result = Socks5Transport::new(proxy, target);
        assert!(result.is_err());
    }
}
