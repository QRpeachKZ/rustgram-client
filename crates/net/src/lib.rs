// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rust implementation of Telegram's network layer based on TDLib.
//!
//! This module provides the core networking infrastructure for Telegram client,
//! including connection management, query dispatching, and DC (Data Center) management.
//!
//! ## Modules
//!
//! - [`crypto`] - Cryptography primitives (AES-IGE, KDF, SHA1/SHA256)
//! - [`packet`] - MTProto packet types (MessageId, PacketInfo, MtprotoQuery)
//! - [`auth`] - Authentication data handling
//! - [`connection`] - Connection management
//! - [`dc`] - Data Center types and options
//! - [`proxy`] - Proxy types (SOCKS5, HTTP, MTProto)
//! - [`query`] - Query dispatching and lifecycle
//! - [`stats`] - Network statistics management

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

pub mod auth;
pub mod connection;
pub mod crypto;
pub mod dc;
pub mod dc_auth;
pub mod dispatch;
pub mod mtproto_header;
pub mod packet;
pub mod proxy;
pub mod query;
pub mod rsa_key_shared;
pub mod session;
pub mod session_multi_proxy;
pub mod stats;
pub mod transport;

// Re-export existing types
pub use auth::{AuthDataShared, AuthKeyState};
pub use connection::{ConnectionCreator, ConnectionMode, ConnectionState, Session, SessionProxy};
pub use dc::{DcId, DcOption, DcOptions, DcOptionsSet};
pub use proxy::{Proxy, ProxyType};
pub use query::{
    AuthFlag, GzipFlag, NetQuery, NetQueryCallback, NetQueryDispatcher, NetQueryId, NetQueryState,
    NetQueryType, QueryError,
};
pub use stats::{NetStatsManager, NetType, NetworkStats, NetworkStatsEntry};

// Re-export crypto types
pub use crypto::compute_auth_key_id;
pub use crypto::{aes_ige_decrypt, aes_ige_encrypt, AesIge};
pub use crypto::{kdf, kdf2, sha1, sha256, tmp_kdf, KdfOutput};
pub use crypto::{AuthKeyError, AuthKeyHelper, CryptoAuthKey, DefaultAuthKeyHelper};

// Re-export packet types
pub use packet::{MessageId, MtprotoQuery, PacketInfo, PacketType};

// Re-export transport types
pub use transport::{
    HttpProxyTransport, HttpProxyTransportFactory, HttpTransport, HttpTransportFactory,
};
pub use transport::{MtprotoProxyTransport, MtprotoProxyTransportFactory};
pub use transport::{
    ReadResult, Transport, TransportMode, TransportRead, TransportWrite, WriteOptions,
};
pub use transport::{Socks5Transport, Socks5TransportFactory};
pub use transport::{
    TcpReadHalf, TcpTransport, TcpTransportFactory, TcpWriteHalf, MAX_PACKET_SIZE,
};

// Re-export session types
pub use session::{ContainerDecoder, MessageContainer, ServicePacket};
pub use session::{PacketHandler, PacketHandlerResult, ServicePacketHandler};
pub use session::{PingConfig, PingManager};
pub use session::{QueryLifecycle, QueryState};
pub use session::{
    SessionConnection, SessionConnectionConfig, SessionEvent, SessionState, SessionStatistics,
};

// Re-export dispatch types
pub use dispatch::{DelayConfig, NetQueryDelayer};
pub use dispatch::{DispatchConfig, EnhancedDispatcher};
pub use dispatch::{FloodControl, FloodControlConfig, FloodControlResult};
pub use dispatch::{SequenceConfig, SequenceDispatcher};

// Re-export RSA key types
pub use rsa_key_shared::{PublicRsaKeyInterface, PublicRsaKeySharedCdn, PublicRsaKeySharedMain, RsaKey, RsaKeyError, RsaKeyManager, RsaKeyListener, PublicRsaKeyWatchdog};

// Re-export DC auth types
pub use dc_auth::{DcAuthError, DcAuthInfo, DcAuthManager, DcAuthKeyStorage, DcState, RegisteredAuthKey, StoredAuthKey, TempAuthKeyWatchdog};

// Re-export session multiproxy types
pub use session_multi_proxy::{SessionMultiProxy, SessionMultiProxyConfig, SessionMultiProxyFactory, SessionInfo, SessionProxyError, SessionStats, SessionType};
pub use session_multi_proxy::{SessionProxy as SessionProxyTrait};

// Re-export MTProto header types
pub use mtproto_header::{MtprotoHeader, MtprotoHeaderError, MtprotoHeaderFactory, MtprotoHeaderOptions, Platform};

/// Network module error types
pub mod error {
    pub use super::connection::ConnectionError;
    pub use super::crypto::CryptoError;
    pub use super::dc::DcError;
    pub use super::dc_auth::DcAuthError;
    pub use super::mtproto_header::MtprotoHeaderError;
    pub use super::proxy::ProxyError;
    pub use super::rsa_key_shared::RsaKeyError;
    pub use super::session_multi_proxy::SessionProxyError;
}

/// Prelude for common imports
pub mod prelude {
    pub use super::auth::*;
    pub use super::connection::*;
    pub use super::crypto::prelude::*;
    pub use super::dc::*;
    pub use super::packet::prelude::*;
    pub use super::proxy::*;
    pub use super::query::*;
    pub use super::stats::*;
    pub use super::transport::*;
}
