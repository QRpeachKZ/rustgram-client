// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Proxy support for Telegram network connections.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Proxy type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProxyType {
    /// No proxy
    #[default]
    None = 0,
    /// SOCKS5 proxy
    Socks5 = 1,
    /// MTProto proxy
    Mtproto = 2,
    /// HTTP TCP proxy
    HttpTcp = 3,
    /// HTTP caching proxy
    HttpCaching = 4,
}

impl fmt::Display for ProxyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Socks5 => write!(f, "SOCKS5"),
            Self::Mtproto => write!(f, "MTProto"),
            Self::HttpTcp => write!(f, "HTTP/TCP"),
            Self::HttpCaching => write!(f, "HTTP/Caching"),
        }
    }
}

/// Proxy error.
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ProxyError {
    /// Invalid proxy type
    #[error("Invalid proxy type: {0}")]
    InvalidType(String),

    /// Invalid proxy address
    #[error("Invalid proxy address: {0}")]
    InvalidAddress(String),

    /// Connection failed
    #[error("Proxy connection failed: {0}")]
    ConnectionFailed(String),

    /// Authentication failed
    #[error("Proxy authentication failed")]
    AuthenticationFailed,

    /// Unsupported proxy type for operation
    #[error("Unsupported proxy type: {0:?}")]
    UnsupportedType(ProxyType),
}

/// Proxy configuration.
///
/// Based on TDLib's Proxy from `td/telegram/net/Proxy.h`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proxy {
    /// Proxy type
    pub proxy_type: ProxyType,

    /// Server address
    pub server: String,

    /// Port number
    pub port: u16,

    /// Username (for SOCKS5/HTTP)
    pub user: Option<String>,

    /// Password (for SOCKS5/HTTP)
    pub password: Option<String>,

    /// Secret (for MTProto proxy)
    pub secret: Option<Vec<u8>>,
}

impl Default for Proxy {
    fn default() -> Self {
        Self::none()
    }
}

impl Proxy {
    /// Creates a proxy with no proxy (direct connection).
    pub fn none() -> Self {
        Self {
            proxy_type: ProxyType::None,
            server: String::new(),
            port: 0,
            user: None,
            password: None,
            secret: None,
        }
    }

    /// Creates a SOCKS5 proxy.
    pub fn socks5(
        server: String,
        port: u16,
        user: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            proxy_type: ProxyType::Socks5,
            server,
            port,
            user,
            password,
            secret: None,
        }
    }

    /// Creates an MTProto proxy.
    pub fn mtproto(server: String, port: u16, secret: Vec<u8>) -> Self {
        Self {
            proxy_type: ProxyType::Mtproto,
            server,
            port,
            user: None,
            password: None,
            secret: Some(secret),
        }
    }

    /// Creates an HTTP TCP proxy.
    pub fn http_tcp(
        server: String,
        port: u16,
        user: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            proxy_type: ProxyType::HttpTcp,
            server,
            port,
            user,
            password,
            secret: None,
        }
    }

    /// Creates an HTTP caching proxy.
    pub fn http_caching(
        server: String,
        port: u16,
        user: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            proxy_type: ProxyType::HttpCaching,
            server,
            port,
            user,
            password,
            secret: None,
        }
    }

    /// Returns `true` if using any proxy.
    pub fn use_proxy(&self) -> bool {
        self.proxy_type != ProxyType::None
    }

    /// Returns `true` if using SOCKS5 proxy.
    pub fn use_socks5_proxy(&self) -> bool {
        self.proxy_type == ProxyType::Socks5
    }

    /// Returns `true` if using MTProto proxy.
    pub fn use_mtproto_proxy(&self) -> bool {
        self.proxy_type == ProxyType::Mtproto
    }

    /// Returns `true` if using HTTP TCP proxy.
    pub fn use_http_tcp_proxy(&self) -> bool {
        self.proxy_type == ProxyType::HttpTcp
    }

    /// Returns `true` if using HTTP caching proxy.
    pub fn use_http_caching_proxy(&self) -> bool {
        self.proxy_type == ProxyType::HttpCaching
    }

    /// Validates the proxy configuration.
    pub fn validate(&self) -> Result<(), ProxyError> {
        if !self.use_proxy() {
            return Ok(());
        }

        if self.server.is_empty() {
            return Err(ProxyError::InvalidAddress("Server address is empty".into()));
        }

        if self.port == 0 {
            return Err(ProxyError::InvalidAddress("Port is zero".into()));
        }

        match self.proxy_type {
            ProxyType::Socks5 | ProxyType::HttpTcp | ProxyType::HttpCaching => {
                // Username/password optional, no validation needed
            }
            ProxyType::Mtproto => {
                if self.secret.is_none()
                    || self.secret.as_ref().map(|s| s.is_empty()).unwrap_or(true)
                {
                    return Err(ProxyError::InvalidAddress(
                        "MTProto proxy requires a secret".into(),
                    ));
                }
            }
            ProxyType::None => {}
        }

        Ok(())
    }

    /// Returns the proxy server address.
    pub fn server(&self) -> &str {
        &self.server
    }

    /// Returns the proxy port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the proxy secret (if available).
    pub fn secret(&self) -> Option<&[u8]> {
        self.secret.as_deref()
    }

    /// Returns the proxy type.
    pub fn proxy_type(&self) -> ProxyType {
        self.proxy_type
    }
}

impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.proxy_type {
            ProxyType::None => write!(f, "Proxy: None"),
            ProxyType::Socks5 => {
                write!(f, "SOCKS5({}:{})", self.server, self.port)?;
                if let Some(user) = &self.user {
                    write!(f, " user={}", user)?;
                }
                Ok(())
            }
            ProxyType::Mtproto => {
                write!(
                    f,
                    "MTProto({}:{}, secret={})",
                    self.server,
                    self.port,
                    self.secret.as_ref().map(|s| s.len()).unwrap_or(0)
                )
            }
            ProxyType::HttpTcp => {
                write!(f, "HTTP/TCP({}:{})", self.server, self.port)?;
                if let Some(user) = &self.user {
                    write!(f, " user={}", user)?;
                }
                Ok(())
            }
            ProxyType::HttpCaching => {
                write!(f, "HTTP/Caching({}:{})", self.server, self.port)?;
                if let Some(user) = &self.user {
                    write!(f, " user={}", user)?;
                }
                Ok(())
            }
        }
    }
}

/// Secret for MTProto proxy obfuscation.
///
/// Based on TDLib's ProxySecret from `td/mtproto/ProxySecret.h`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxySecret {
    /// Secret data
    pub secret: Vec<u8>,
}

impl ProxySecret {
    /// Creates a new proxy secret.
    pub fn new(secret: Vec<u8>) -> Self {
        Self { secret }
    }

    /// Returns the raw secret bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.secret
    }

    /// Returns the length of the secret.
    pub fn len(&self) -> usize {
        self.secret.len()
    }

    /// Returns `true` if the secret is empty.
    pub fn is_empty(&self) -> bool {
        self.secret.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_none() {
        let proxy = Proxy::none();
        assert!(!proxy.use_proxy());
        assert!(!proxy.use_socks5_proxy());
        assert!(!proxy.use_mtproto_proxy());
    }

    #[test]
    fn test_proxy_socks5() {
        let proxy = Proxy::socks5(
            "127.0.0.1".into(),
            1080,
            Some("user".into()),
            Some("pass".into()),
        );

        assert!(proxy.use_proxy());
        assert!(proxy.use_socks5_proxy());
        assert_eq!(proxy.server(), "127.0.0.1");
        assert_eq!(proxy.port(), 1080);
        assert_eq!(proxy.user, Some("user".into()));
        assert_eq!(proxy.password, Some("pass".into()));
    }

    #[test]
    fn test_proxy_mtproto() {
        let secret = vec![0x01, 0x02, 0x03, 0x04];
        let proxy = Proxy::mtproto("example.com".into(), 443, secret.clone());

        assert!(proxy.use_proxy());
        assert!(proxy.use_mtproto_proxy());
        assert_eq!(proxy.server(), "example.com");
        assert_eq!(proxy.port(), 443);
        assert_eq!(proxy.secret(), Some(secret.as_slice()));
    }

    #[test]
    fn test_proxy_validate() {
        let proxy = Proxy::none();
        assert!(proxy.validate().is_ok());

        let proxy = Proxy::socks5("".into(), 0, None, None);
        assert!(proxy.validate().is_err());

        let proxy = Proxy::socks5("127.0.0.1".into(), 1080, None, None);
        assert!(proxy.validate().is_ok());

        let proxy = Proxy::mtproto("example.com".into(), 443, vec![]);
        assert!(proxy.validate().is_err());

        let proxy = Proxy::mtproto("example.com".into(), 443, vec![1, 2, 3]);
        assert!(proxy.validate().is_ok());
    }

    #[test]
    fn test_proxy_type_display() {
        assert_eq!(format!("{}", ProxyType::None), "None");
        assert_eq!(format!("{}", ProxyType::Socks5), "SOCKS5");
        assert_eq!(format!("{}", ProxyType::Mtproto), "MTProto");
        assert_eq!(format!("{}", ProxyType::HttpTcp), "HTTP/TCP");
        assert_eq!(format!("{}", ProxyType::HttpCaching), "HTTP/Caching");
    }

    #[test]
    fn test_proxy_secret() {
        let secret = ProxySecret::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(secret.len(), 5);
        assert!(!secret.is_empty());
        assert_eq!(secret.as_bytes(), &[1, 2, 3, 4, 5]);
    }
}
