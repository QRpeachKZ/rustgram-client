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
//! - [`crypto`] - Cryptography primitives (AES-IGE, KDF, SHA1/SHA256, RSA, HMAC)
//! - [`packet`] - MTProto packet types (MessageId, PacketInfo, MtprotoQuery)
//! - [`auth`] - Authentication data handling
//! - [`connection`] - Connection management
//! - [`dc`] - Data Center types and options
//! - [`proxy`] - Proxy types (SOCKS5, HTTP, MTProto)
//! - [`query`] - Query dispatching and lifecycle
//! - [`pool`] - Connection pooling for multiple DCs
//! - [`failover`] - Multi-DC failover for high availability
//! - [`stats`] - Network statistics management

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

pub mod auth;
pub mod circuit_breaker;
pub mod connection;
pub mod crypto;
pub mod dc;
pub mod dc_auth;
pub mod dispatch;
pub mod failover;
pub mod handshake;
pub mod health_check;
pub mod mtproto_header;
pub mod net_actor;
pub mod packet;
pub mod pool;
pub mod proxy;
pub mod query;
pub mod query_creator;
pub mod query_verifier;
pub mod rsa_key_shared;
pub mod session;
pub mod session_multi_proxy;
pub mod stats;
pub mod test_config;
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
pub use crypto::{decrypt_signature, RsaError, RsaPrivateKeyWrapper, RsaPublicKeyWrapper, RsaResult};
pub use crypto::{hmac_sha256, hmac_sha512, pbkdf2_hmac_sha256, pbkdf2_hmac_sha512};
pub use crypto::{kdf, kdf2, pq_factorize, pq_factorize_big, sha1, sha256, tmp_kdf, KdfOutput};
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
pub use rsa_key_shared::{
    PublicRsaKeyInterface, PublicRsaKeySharedCdn, PublicRsaKeySharedMain, PublicRsaKeyWatchdog,
    RsaKey, RsaKeyError, RsaKeyListener, RsaKeyManager,
};

// Re-export DC auth types
pub use dc_auth::{
    DcAuthError, DcAuthInfo, DcAuthKeyStorage, DcAuthManager, DcState, RegisteredAuthKey,
    StoredAuthKey, TempAuthKeyWatchdog,
};

// Re-export handshake types
pub use handshake::{
    HandshakeAction, HandshakeError, HandshakeMode, HandshakeState, MtprotoHandshake,
};

// Re-export session multiproxy types
pub use session_multi_proxy::SessionProxy as SessionProxyTrait;
pub use session_multi_proxy::{
    SessionInfo, SessionMultiProxy, SessionMultiProxyConfig, SessionMultiProxyFactory,
    SessionProxyError, SessionStats, SessionType,
};

// Re-export test config types
pub use test_config::{get_dc_options, get_rsa_keys, is_test_dc, set_test_mode};

// Re-export MTProto header types
pub use mtproto_header::{
    MtprotoHeader, MtprotoHeaderError, MtprotoHeaderFactory, MtprotoHeaderOptions, Platform,
};

// Re-export query creator types
pub use query_creator::{NetQueryCreator, NetQueryStats};

// Re-export query verifier types
pub use query_verifier::{
    NetQueryVerifier, VerificationError, VerificationQuery, VerificationResult, VerificationType,
};

// Re-export net actor types
pub use net_actor::{ActorError, ActorQueryCallback, ActorResult, NetActor, TestActor};

// Re-export connection pool types
pub use pool::{ConnectionPool, ConnectionPurpose, PooledConnection, PoolConfig, PoolError};

// Re-export circuit breaker types
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};

// Re-export health check types
pub use health_check::{HealthChecker, HealthCheckConfig, HealthStatus};

// Re-export failover types
pub use failover::{DcHealth, FailoverError, FailoverManager, FailoverPolicy, RequestType};

/// Network module error types
pub mod error {
    pub use super::connection::ConnectionError;
    pub use super::crypto::CryptoError;
    pub use super::dc::DcError;
    pub use super::dc_auth::DcAuthError;
    pub use super::failover::FailoverError;
    pub use super::handshake::HandshakeError;
    pub use super::mtproto_header::MtprotoHeaderError;
    pub use super::net_actor::ActorError;
    pub use super::pool::PoolError;
    pub use super::proxy::ProxyError;
    pub use super::query::RetryError;
    pub use super::query_creator::NetQueryStats;
    pub use super::query_verifier::VerificationError;
    pub use super::rsa_key_shared::RsaKeyError;
    pub use super::session_multi_proxy::SessionProxyError;
    pub use super::query::TimeoutError;
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
