// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto authentication TL types.
//!
//! This module provides TL request/response types for MTProto DH key exchange.
//! These types are used during the initial handshake to establish an auth key
//! with Telegram servers.
//!
//! # TL Schema
//!
//! ```text
//! req_pq_multi#be7e8ef1 nonce:int128 = ResPQ;
//! req_DH_params#d712e4be nonce:int128 server_nonce:int128 p:string q:string
//!     public_key_fingerprint:long encrypted_data:string = Server_DH_Params;
//! set_client_DH_params#f5045f1f nonce:int128 server_nonce:int128
//!     encrypted_data:string = Set_client_DH_params_answer;
//!
//! p_q_inner_data_dc#a9f55f95 pq:string p:string q:string nonce:int128
//!     server_nonce:int128 new_nonce:int256 dc:int = P_Q_inner_data;
//! p_q_inner_data_temp_dc#56fddf88 pq:string p:string q:string nonce:int128
//!     server_nonce:int128 new_nonce:int256 dc:int expires_in:int = P_Q_inner_data;
//! server_DH_inner_data#b5890dba nonce:int128 server_nonce:int128 g:int
//!     dh_prime:string g_a:string server_time:int = Server_DH_inner_data;
//! client_DH_inner_data#6643b654 nonce:int128 server_nonce:int128 retry_id:long
//!     g_b:string = Client_DH_Inner_Data;
//! ```
//!
//! # References
//!
//! - TDLib: `td/generate/scheme/mtproto_api.tl`
//! - TDLib: `td/mtproto/Handshake.cpp`

use crate::error::{TypeError, TypeResult};
use crate::primitive::TlInt128;
use crate::tl::{Bytes, TlDeserialize, TlHelper, TlSerialize};
use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// Requests
// ============================================================================

/// Request PQ (Prime Query) from server.
///
/// Corresponds to `req_pq_multi#be7e8ef1`.
///
/// # TL Schema
///
/// ```text
/// req_pq_multi#be7e8ef1 nonce:int128 = ResPQ;
/// ```
///
/// This is the first request sent during MTProto handshake to initiate
/// the Diffie-Hellman key exchange.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReqPqMulti {
    /// Random nonce for this handshake attempt
    pub nonce: TlInt128,
}

impl ReqPqMulti {
    /// Creates a new ReqPqMulti request.
    ///
    /// # Arguments
    ///
    /// * `nonce` - Random 128-bit value for handshake
    pub fn new(nonce: TlInt128) -> Self {
        Self { nonce }
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0xbe7e8ef1
    }
}

impl TlSerialize for ReqPqMulti {
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        buf.put_slice(&self.nonce.get());
        Ok(())
    }
}

/// Request DH parameters from server.
///
/// Corresponds to `req_DH_params#d712e4be`.
///
/// # TL Schema
///
/// ```text
/// req_DH_params#d712e4be nonce:int128 server_nonce:int128 p:string q:string
///     public_key_fingerprint:long encrypted_data:string = Server_DH_Params;
/// ```
///
/// This request is sent after receiving ResPQ, with the factorized PQ values
/// and encrypted inner data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReqDhParams {
    /// Client nonce (must match original request)
    pub nonce: TlInt128,

    /// Server nonce from ResPQ response
    pub server_nonce: TlInt128,

    /// Factorized prime p
    pub p: Vec<u8>,

    /// Factorized prime q
    pub q: Vec<u8>,

    /// Fingerprint of the server's RSA public key
    pub public_key_fingerprint: i64,

    /// Encrypted PQ inner data
    pub encrypted_data: Vec<u8>,
}

impl ReqDhParams {
    /// Creates a new ReqDhParams request.
    ///
    /// # Arguments
    ///
    /// * `nonce` - Client nonce
    /// * `server_nonce` - Server nonce from ResPQ
    /// * `p` - Factorized prime p
    /// * `q` - Factorized prime q
    /// * `public_key_fingerprint` - RSA key fingerprint
    /// * `encrypted_data` - Encrypted inner data
    pub fn new(
        nonce: TlInt128,
        server_nonce: TlInt128,
        p: Vec<u8>,
        q: Vec<u8>,
        public_key_fingerprint: i64,
        encrypted_data: Vec<u8>,
    ) -> Self {
        Self {
            nonce,
            server_nonce,
            p,
            q,
            public_key_fingerprint,
            encrypted_data,
        }
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0xd712e4be
    }
}

impl TlSerialize for ReqDhParams {
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        buf.put_slice(&self.nonce.get());
        buf.put_slice(&self.server_nonce.get());
        TlHelper::write_bytes(buf, &self.p);
        TlHelper::write_bytes(buf, &self.q);
        TlHelper::write_i64(buf, self.public_key_fingerprint);
        TlHelper::write_bytes(buf, &self.encrypted_data);
        Ok(())
    }
}

/// Set client DH parameters.
///
/// Corresponds to `set_client_DH_params#f5045f1f`.
///
/// # TL Schema
///
/// ```text
/// set_client_DH_params#f5045f1f nonce:int128 server_nonce:int128
///     encrypted_data:string = Set_client_DH_params_answer;
/// ```
///
/// This request completes the DH handshake by sending the client's
/// computed DH public key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetClientDhParams {
    /// Client nonce
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// Encrypted client DH inner data
    pub encrypted_data: Vec<u8>,
}

impl SetClientDhParams {
    /// Creates a new SetClientDhParams request.
    ///
    /// # Arguments
    ///
    /// * `nonce` - Client nonce
    /// * `server_nonce` - Server nonce
    /// * `encrypted_data` - Encrypted client DH inner data
    pub fn new(nonce: TlInt128, server_nonce: TlInt128, encrypted_data: Vec<u8>) -> Self {
        Self {
            nonce,
            server_nonce,
            encrypted_data,
        }
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0xf5045f1f
    }
}

impl TlSerialize for SetClientDhParams {
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        buf.put_slice(&self.nonce.get());
        buf.put_slice(&self.server_nonce.get());
        TlHelper::write_bytes(buf, &self.encrypted_data);
        Ok(())
    }
}

// ============================================================================
// Responses
// ============================================================================

/// Response to req_pq_multi.
///
/// Contains the PQ value to factorize, server nonce, and available
/// server public key fingerprints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResPq {
    /// Client nonce (echoed back)
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// PQ value (product of two primes)
    pub pq: Vec<u8>,

    /// Available server public key fingerprints
    pub server_public_key_fingerprints: Vec<i64>,
}

impl ResPq {
    /// Creates a new ResPq response.
    pub fn new(
        nonce: TlInt128,
        server_nonce: TlInt128,
        pq: Vec<u8>,
        server_public_key_fingerprints: Vec<i64>,
    ) -> Self {
        Self {
            nonce,
            server_nonce,
            pq,
            server_public_key_fingerprints,
        }
    }
}

impl TlDeserialize for ResPq {
    fn deserialize_tl(buf: &mut Bytes) -> TypeResult<Self> {
        // Read constructor ID - should be 0x05162463 for ResPQ
        let constructor_id = TlHelper::read_constructor_id(buf)?;
        if constructor_id != 0x05162463 {
            return Err(TypeError::DeserializationError(format!(
                "Expected ResPQ constructor 0x05162463, got 0x{:08x}",
                constructor_id
            )));
        }

        // Read nonce (int128)
        let mut nonce_bytes = [0u8; 16];
        if buf.remaining() < 16 {
            return Err(TypeError::DeserializationError(
                "not enough bytes for nonce".to_string(),
            ));
        }
        buf.copy_to_slice(&mut nonce_bytes);
        let nonce = TlInt128::new(nonce_bytes);

        // Read server_nonce (int128)
        let mut server_nonce_bytes = [0u8; 16];
        if buf.remaining() < 16 {
            return Err(TypeError::DeserializationError(
                "not enough bytes for server_nonce".to_string(),
            ));
        }
        buf.copy_to_slice(&mut server_nonce_bytes);
        let server_nonce = TlInt128::new(server_nonce_bytes);

        // Read pq (bytes)
        let pq = TlHelper::read_bytes(buf)?;

        // Read server_public_key_fingerprints (vector of long)
        // Vector constructor ID must be 0x1cb5c415.
        let vector_id = TlHelper::read_constructor_id(buf)?;
        if vector_id != 0x1cb5c415 {
            return Err(TypeError::DeserializationError(format!(
                "Expected vector constructor 0x1cb5c415, got 0x{:08x}",
                vector_id
            )));
        }
        let fingerprints_len = TlHelper::read_i32(buf)? as usize;
        let mut server_public_key_fingerprints = Vec::with_capacity(fingerprints_len);
        for _ in 0..fingerprints_len {
            server_public_key_fingerprints.push(TlHelper::read_i64(buf)?);
        }

        Ok(Self::new(nonce, server_nonce, pq, server_public_key_fingerprints))
    }
}

/// Server DH parameters response (successful).
///
/// Contains the encrypted server DH inner data with the DH prime,
/// generator, and server's public DH key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerDhParamsOk {
    /// Client nonce
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// Encrypted server DH inner data
    pub encrypted_answer: Vec<u8>,
}

impl ServerDhParamsOk {
    /// Creates a new ServerDhParamsOk response.
    pub fn new(nonce: TlInt128, server_nonce: TlInt128, encrypted_answer: Vec<u8>) -> Self {
        Self {
            nonce,
            server_nonce,
            encrypted_answer,
        }
    }
}

impl TlDeserialize for ServerDhParamsOk {
    fn deserialize_tl(buf: &mut Bytes) -> TypeResult<Self> {
        // Read constructor ID - should be 0xd0e8075c for server_DH_params_ok
        let constructor_id = TlHelper::read_constructor_id(buf)?;
        if constructor_id != 0xd0e8075c {
            return Err(TypeError::DeserializationError(format!(
                "Expected ServerDhParamsOk constructor 0xd0e8075c, got 0x{:08x}",
                constructor_id
            )));
        }

        // Read nonce (int128)
        let mut nonce_bytes = [0u8; 16];
        if buf.remaining() < 16 {
            return Err(TypeError::DeserializationError(
                "not enough bytes for nonce".to_string(),
            ));
        }
        buf.copy_to_slice(&mut nonce_bytes);
        let nonce = TlInt128::new(nonce_bytes);

        // Read server_nonce (int128)
        let mut server_nonce_bytes = [0u8; 16];
        if buf.remaining() < 16 {
            return Err(TypeError::DeserializationError(
                "not enough bytes for server_nonce".to_string(),
            ));
        }
        buf.copy_to_slice(&mut server_nonce_bytes);
        let server_nonce = TlInt128::new(server_nonce_bytes);

        // Read encrypted_answer (bytes)
        let encrypted_answer = TlHelper::read_bytes(buf)?;

        Ok(Self::new(nonce, server_nonce, encrypted_answer))
    }
}

/// DH generation successful response.
///
/// The handshake completed successfully and the auth key is valid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DhGenOk {
    /// Client nonce
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// SHA1 hash of new_nonce + auth_key_id
    pub new_nonce_hash: TlInt128,
}

impl DhGenOk {
    /// Creates a new DhGenOk response.
    pub fn new(nonce: TlInt128, server_nonce: TlInt128, new_nonce_hash: TlInt128) -> Self {
        Self {
            nonce,
            server_nonce,
            new_nonce_hash,
        }
    }
}

// ============================================================================
// Inner Data Types (encrypted)
// ============================================================================

/// PQ inner data for main DC.
///
/// Corresponds to `p_q_inner_data_dc#a9f55f95`.
///
/// # TL Schema
///
/// ```text
/// p_q_inner_data_dc#a9f55f95 pq:string p:string q:string nonce:int128
///     server_nonce:int128 new_nonce:int256 dc:int = P_Q_inner_data;
/// ```
///
/// This data is encrypted with RSA and sent in req_DH_params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PQInnerDataDc {
    /// PQ value
    pub pq: Vec<u8>,

    /// Factorized prime p
    pub p: Vec<u8>,

    /// Factorized prime q
    pub q: Vec<u8>,

    /// Client nonce
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// New client nonce (256-bit)
    pub new_nonce: [u8; 32],

    /// DC ID for this auth key
    pub dc_id: i32,
}

impl PQInnerDataDc {
    /// Creates a new PQInnerDataDc.
    ///
    /// # Arguments
    ///
    /// * `pq` - PQ value
    /// * `p` - Factorized prime p
    /// * `q` - Factorized prime q
    /// * `nonce` - Client nonce
    /// * `server_nonce` - Server nonce
    /// * `new_nonce` - New 256-bit client nonce
    /// * `dc_id` - DC ID
    pub fn new(
        pq: Vec<u8>,
        p: Vec<u8>,
        q: Vec<u8>,
        nonce: TlInt128,
        server_nonce: TlInt128,
        new_nonce: [u8; 32],
        dc_id: i32,
    ) -> Self {
        Self {
            pq,
            p,
            q,
            nonce,
            server_nonce,
            new_nonce,
            dc_id,
        }
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0xa9f55f95
    }
}

impl TlSerialize for PQInnerDataDc {
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        TlHelper::write_bytes(buf, &self.pq);
        TlHelper::write_bytes(buf, &self.p);
        TlHelper::write_bytes(buf, &self.q);
        buf.put_slice(&self.nonce.get());
        buf.put_slice(&self.server_nonce.get());
        buf.put_slice(&self.new_nonce);
        TlHelper::write_i32(buf, self.dc_id);
        Ok(())
    }
}

/// PQ inner data for temporary DC.
///
/// Corresponds to `p_q_inner_data_temp_dc#56fddf88`.
///
/// # TL Schema
///
/// ```text
/// p_q_inner_data_temp_dc#56fddf88 pq:string p:string q:string nonce:int128
///     server_nonce:int128 new_nonce:int256 dc:int expires_in:int = P_Q_inner_data;
/// ```
///
/// This data is encrypted with RSA and sent in req_DH_params
/// for temporary (PFS) auth keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PQInnerDataTempDc {
    /// PQ value
    pub pq: Vec<u8>,

    /// Factorized prime p
    pub p: Vec<u8>,

    /// Factorized prime q
    pub q: Vec<u8>,

    /// Client nonce
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// New client nonce (256-bit)
    pub new_nonce: [u8; 32],

    /// DC ID for this auth key
    pub dc_id: i32,

    /// Expiration time in seconds
    pub expires_in: i32,
}

/// Options for creating PQInnerDataTempDc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PQInnerDataTempDcOptions {
    /// PQ value
    pub pq: Vec<u8>,
    /// Factorized prime p
    pub p: Vec<u8>,
    /// Factorized prime q
    pub q: Vec<u8>,
    /// Client nonce
    pub nonce: TlInt128,
    /// Server nonce
    pub server_nonce: TlInt128,
    /// New 256-bit client nonce
    pub new_nonce: [u8; 32],
    /// DC ID
    pub dc_id: i32,
    /// Expiration time in seconds
    pub expires_in: i32,
}

impl PQInnerDataTempDc {
    /// Creates a new PQ inner data with temporary DC ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Initialization options
    pub fn new(options: PQInnerDataTempDcOptions) -> Self {
        Self {
            pq: options.pq,
            p: options.p,
            q: options.q,
            nonce: options.nonce,
            server_nonce: options.server_nonce,
            new_nonce: options.new_nonce,
            dc_id: options.dc_id,
            expires_in: options.expires_in,
        }
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0x56fddf88
    }
}

impl TlSerialize for PQInnerDataTempDc {
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        TlHelper::write_bytes(buf, &self.pq);
        TlHelper::write_bytes(buf, &self.p);
        TlHelper::write_bytes(buf, &self.q);
        buf.put_slice(&self.nonce.get());
        buf.put_slice(&self.server_nonce.get());
        buf.put_slice(&self.new_nonce);
        TlHelper::write_i32(buf, self.dc_id);
        TlHelper::write_i32(buf, self.expires_in);
        Ok(())
    }
}

/// Server DH inner data.
///
/// Corresponds to `server_DH_inner_data#b5890dba`.
///
/// # TL Schema
///
/// ```text
/// server_DH_inner_data#b5890dba nonce:int128 server_nonce:int128 g:int
///     dh_prime:string g_a:string server_time:int = Server_DH_inner_data;
/// ```
///
/// This data is decrypted from the encrypted_answer in ServerDhParamsOk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerDhInnerData {
    /// Client nonce
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// DH generator (usually 2 or 5)
    pub g: i32,

    /// DH prime (2048-bit safe prime)
    pub dh_prime: Vec<u8>,

    /// Server's DH public key (g^a mod dh_prime)
    pub ga: Vec<u8>,

    /// Server time
    pub server_time: i32,
}

impl ServerDhInnerData {
    /// Creates a new ServerDhInnerData.
    ///
    /// # Arguments
    ///
    /// * `nonce` - Client nonce
    /// * `server_nonce` - Server nonce
    /// * `g` - DH generator
    /// * `dh_prime` - DH prime
    /// * `ga` - Server's DH public key
    /// * `server_time` - Server time
    pub fn new(
        nonce: TlInt128,
        server_nonce: TlInt128,
        g: i32,
        dh_prime: Vec<u8>,
        ga: Vec<u8>,
        server_time: i32,
    ) -> Self {
        Self {
            nonce,
            server_nonce,
            g,
            dh_prime,
            ga,
            server_time,
        }
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0xb5890dba
    }
}

impl TlSerialize for ServerDhInnerData {
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        buf.put_slice(&self.nonce.get());
        buf.put_slice(&self.server_nonce.get());
        TlHelper::write_i32(buf, self.g);
        TlHelper::write_bytes(buf, &self.dh_prime);
        TlHelper::write_bytes(buf, &self.ga);
        TlHelper::write_i32(buf, self.server_time);
        Ok(())
    }
}

/// Client DH inner data.
///
/// Corresponds to `client_DH_inner_data#6643b654`.
///
/// # TL Schema
///
/// ```text
/// client_DH_inner_data#6643b654 nonce:int128 server_nonce:int128
///     retry_id:long g_b:string = Client_DH_Inner_Data;
/// ```
///
/// This data is encrypted and sent in set_client_DH_params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientDhInnerData {
    /// Client nonce
    pub nonce: TlInt128,

    /// Server nonce
    pub server_nonce: TlInt128,

    /// Retry ID (0 for first attempt)
    pub retry_id: i64,

    /// Client's DH public key (g^b mod dh_prime)
    pub gb: Vec<u8>,
}

impl ClientDhInnerData {
    /// Creates a new ClientDhInnerData.
    ///
    /// # Arguments
    ///
    /// * `nonce` - Client nonce
    /// * `server_nonce` - Server nonce
    /// * `retry_id` - Retry ID (0 for first attempt)
    /// * `gb` - Client's DH public key
    pub fn new(nonce: TlInt128, server_nonce: TlInt128, retry_id: i64, gb: Vec<u8>) -> Self {
        Self {
            nonce,
            server_nonce,
            retry_id,
            gb,
        }
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0x6643b654
    }
}

impl TlSerialize for ClientDhInnerData {
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        buf.put_slice(&self.nonce.get());
        buf.put_slice(&self.server_nonce.get());
        TlHelper::write_i64(buf, self.retry_id);
        TlHelper::write_bytes(buf, &self.gb);
        Ok(())
    }
}

// ============================================================================
// DH Response Types
// ============================================================================

/// Set_client_DH_params_answer response types.
///
/// The server can respond with one of these after set_client_DH_params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DhGenResponse {
    /// DH generation successful
    Ok(DhGenOk),

    /// DH generation failed - retry with new nonce
    Retry,

    /// DH generation failed - abort
    Fail,

    /// Unknown response
    Unknown,
}

impl DhGenResponse {
    /// Creates a DhGenResponse from a constructor ID.
    ///
    /// Note: For DhGenResponse::Ok, you'll need to parse the actual DhGenOk data
    /// separately. This only identifies the response type.
    pub fn from_constructor_id(id: u32) -> Self {
        match id {
            0x3bcbf734 => DhGenResponse::Unknown,  // dh_gen_ok - needs full parsing
            0x46dc1fb9 => DhGenResponse::Retry,     // dh_gen_retry
            0xa69dae02 => DhGenResponse::Fail,      // dh_gen_fail
            _ => DhGenResponse::Unknown,
        }
    }

    /// Returns true if this is a successful DH generation response.
    pub fn is_ok(&self) -> bool {
        matches!(self, DhGenResponse::Ok(_))
    }
}

impl fmt::Display for DhGenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DhGenResponse::Ok(_) => write!(f, "DhGenOk"),
            DhGenResponse::Retry => write!(f, "DhGenRetry"),
            DhGenResponse::Fail => write!(f, "DhGenFail"),
            DhGenResponse::Unknown => write!(f, "Unknown"),
        }
    }
}

// ============================================================================
// Helper Types
// ============================================================================

/// 256-bit integer type for DH operations.
///
/// Used for new_nonce and DH keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TlInt256([u8; 32]);

impl TlInt256 {
    /// Creates a new TlInt256 from bytes.
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Creates a new zero int256.
    pub fn zero() -> Self {
        Self([0; 32])
    }

    /// Returns the inner bytes.
    pub fn get(&self) -> [u8; 32] {
        self.0
    }

    /// Returns as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Default for TlInt256 {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<[u8; 32]> for TlInt256 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_req_pq_multi_constructor_id() {
        let nonce = TlInt128::new([0u8; 16]);
        let req = ReqPqMulti::new(nonce);
        assert_eq!(req.constructor_id(), 0xbe7e8ef1);
    }

    #[test]
    fn test_req_dh_params_constructor_id() {
        let nonce = TlInt128::new([0u8; 16]);
        let server_nonce = TlInt128::new([0u8; 16]);
        let req = ReqDhParams::new(
            nonce,
            server_nonce,
            vec![],
            vec![],
            0,
            vec![],
        );
        assert_eq!(req.constructor_id(), 0xd712e4be);
    }

    #[test]
    fn test_set_client_dh_params_constructor_id() {
        let nonce = TlInt128::new([0u8; 16]);
        let server_nonce = TlInt128::new([0u8; 16]);
        let req = SetClientDhParams::new(nonce, server_nonce, vec![]);
        assert_eq!(req.constructor_id(), 0xf5045f1f);
    }

    #[test]
    fn test_pq_inner_data_dc_constructor_id() {
        let inner = PQInnerDataDc::new(
            vec![],
            vec![],
            vec![],
            TlInt128::new([0u8; 16]),
            TlInt128::new([0u8; 16]),
            [0u8; 32],
            2,
        );
        assert_eq!(inner.constructor_id(), 0xa9f55f95);
    }

    #[test]
    fn test_pq_inner_data_temp_dc_constructor_id() {
        let inner = PQInnerDataTempDc::new(PQInnerDataTempDcOptions {
            pq: vec![],
            p: vec![],
            q: vec![],
            nonce: TlInt128::new([0u8; 16]),
            server_nonce: TlInt128::new([0u8; 16]),
            new_nonce: [0u8; 32],
            dc_id: 2,
            expires_in: 86400,
        });
        assert_eq!(inner.constructor_id(), 0x56fddf88);
    }

    #[test]
    fn test_server_dh_inner_data_constructor_id() {
        let inner = ServerDhInnerData::new(
            TlInt128::new([0u8; 16]),
            TlInt128::new([0u8; 16]),
            2,
            vec![],
            vec![],
            0,
        );
        assert_eq!(inner.constructor_id(), 0xb5890dba);
    }

    #[test]
    fn test_client_dh_inner_data_constructor_id() {
        let inner = ClientDhInnerData::new(
            TlInt128::new([0u8; 16]),
            TlInt128::new([0u8; 16]),
            0,
            vec![],
        );
        assert_eq!(inner.constructor_id(), 0x6643b654);
    }

    #[test]
    fn test_req_pq_multi_serialize() {
        let nonce = TlInt128::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let req = ReqPqMulti::new(nonce);

        let mut buf = BytesMut::new();
        let result = req.serialize_tl(&mut buf);
        assert!(result.is_ok());

        // Should have constructor ID (4 bytes) + nonce (16 bytes)
        assert_eq!(buf.len(), 20);

        // Verify constructor ID
        assert_eq!(buf[0..4], [0xf1, 0x8e, 0x7e, 0xbe]);
    }

    #[test]
    fn test_tl_int256() {
        let bytes = [1u8; 32];
        let int256 = TlInt256::new(bytes);
        assert_eq!(int256.get(), bytes);
        assert_eq!(int256.as_slice(), &bytes[..]);
    }

    #[test]
    fn test_dh_gen_response_from_id() {
        // dh_gen_ok returns Unknown because it needs full parsing
        assert_eq!(
            DhGenResponse::from_constructor_id(0x3bcbf734),
            DhGenResponse::Unknown
        );
        assert_eq!(
            DhGenResponse::from_constructor_id(0x46dc1fb9),
            DhGenResponse::Retry
        );
        assert_eq!(
            DhGenResponse::from_constructor_id(0xa69dae02),
            DhGenResponse::Fail
        );
        assert_eq!(
            DhGenResponse::from_constructor_id(0xffffffff),
            DhGenResponse::Unknown
        );
    }
}
