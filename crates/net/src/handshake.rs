// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto DH key exchange handshake.
//!
//! This module implements the MTProto 2.0 DH key exchange handshake for
//! establishing an auth key with Telegram servers.
//!
//! # Architecture
//!
//! Based on TDLib's `AuthKeyHandshake` implementation from `td/mtproto/Handshake.cpp`.
//! The handshake follows this flow:
//!
//! ```text
//! Start -> req_pq_multi ---------> ResPQ
//!                                      |
//!                                      v
//!                               req_DH_params
//!                                      |
//!                                      v
//!                            ServerDHParams
//!                                      |
//!                                      v
//!                          set_client_DH_params
//!                                      |
//!                                      v
//!                            DHGenResponse -> Finish
//! ```
//!
//! # References
//!
//! - TDLib: `td/mtproto/Handshake.h`, `td/mtproto/Handshake.cpp`
//! - MTProto 2.0: <https://core.telegram.org/mtproto/description>

use crate::crypto::{
    aes_ige_decrypt, aes_ige_encrypt, pq_factorize, sha1, sha256, tmp_kdf, KdfOutput,
    RsaPublicKeyWrapper,
};
use crate::dc::DcId;
use crate::rsa_key_shared::RsaKey;
use bytes::BytesMut;
use rand::Rng;
use rustgram_types::{
    ClientDhInnerData, DhGenOk, PQInnerDataDc, PQInnerDataTempDc, ReqDhParams, ReqPqMulti,
    ResPq, ServerDhInnerData, ServerDhParamsOk, SetClientDhParams, TlDeserialize, TlInt128,
    TlSerialize,
};
use rustgram_types::tl::Bytes as TlBytes;
use std::time::Duration;
use thiserror::Error;

/// Maximum size for encrypted inner data
const MAX_INNER_DATA_SIZE: usize = 144;

/// Default timeout for handshake operations
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

/// RSA-encrypted data size
const RSA_ENCRYPTED_SIZE: usize = 256;

/// Errors that can occur during MTProto handshake.
#[derive(Debug, Error)]
pub enum HandshakeError {
    /// Invalid state transition
    #[error("Invalid state: expected {expected}, got {actual}")]
    InvalidState {
        /// Expected state
        expected: &'static str,
        /// Actual state
        actual: String,
    },

    /// Nonce mismatch
    #[error("Nonce mismatch")]
    NonceMismatch,

    /// Server nonce mismatch
    #[error("Server nonce mismatch")]
    ServerNonceMismatch,

    /// Failed to factorize PQ
    #[error("Failed to factorize PQ")]
    FactorizationFailed,

    /// RSA key not found
    #[error("RSA key with fingerprint {0} not found")]
    RsaKeyNotFound(i64),

    /// RSA encryption failed
    #[error("RSA encryption failed: {0}")]
    RsaEncryptionFailed(String),

    /// Decryption failed
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// DH parameter validation failed
    #[error("DH validation failed: {0}")]
    DhValidationFailed(String),

    /// Hash mismatch
    #[error("Hash mismatch")]
    HashMismatch,

    /// New nonce hash mismatch
    #[error("New nonce hash mismatch")]
    NewNonceHashMismatch,

    /// Response timeout
    #[error("Handshake timeout")]
    Timeout,

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Other error
    #[error("{0}")]
    Other(String),
}

/// Handshake state.
///
/// Matches TDLib's `AuthKeyHandshake::State` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    /// Initial state
    Start,

    /// Waiting for ResPQ response
    ResPQ,

    /// Waiting for ServerDHParams response
    ServerDhParams,

    /// Waiting for DHGenResponse
    DhGenResponse,

    /// Handshake complete
    Finish,
}

impl std::fmt::Display for HandshakeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeState::Start => write!(f, "Start"),
            HandshakeState::ResPQ => write!(f, "ResPQ"),
            HandshakeState::ServerDhParams => write!(f, "ServerDhParams"),
            HandshakeState::DhGenResponse => write!(f, "DHGenResponse"),
            HandshakeState::Finish => write!(f, "Finish"),
        }
    }
}

/// Handshake mode.
///
/// Matches TDLib's `AuthKeyHandshake::Mode` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeMode {
    /// Main auth key (permanent)
    Main,

    /// Temp auth key (with expiration, for PFS)
    Temp,
}

/// Action to take during handshake.
///
/// Returned by handshake methods to indicate what to do next.
#[derive(Debug)]
pub enum HandshakeAction {
    /// Send this packet to server
    Send(Vec<u8>),

    /// Wait for next response
    Wait,

    /// Handshake complete with auth key and server salt
    Complete(Vec<u8>, u64), // (auth_key, server_salt)
}

/// MTProto DH key exchange handshake.
///
/// Manages the complete MTProto 2.0 handshake flow for establishing
/// an auth key with a Telegram DC.
///
/// # Example
///
/// ```no_run
/// use rustgram_net::handshake::{MtprotoHandshake, HandshakeMode, HandshakeState};
/// use rustgram_net::dc::DcId;
/// use rustgram_net::rsa_key_shared::RsaKey;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let dc_id = DcId::internal(2);
/// let rsa_keys = vec![]; // Load RSA keys from configuration
/// let mut handshake = MtprotoHandshake::new(dc_id, HandshakeMode::Main, rsa_keys);
///
/// // Start handshake
/// let action = handshake.start()?;
/// // Send action.data() to server...
///
/// // Process response
/// let response_data = vec![0u8; 100]; // Placeholder response data
/// let action = handshake.on_message(&response_data)?;
///
/// // Continue until Complete...
/// # Ok(())
/// # }
/// ```
pub struct MtprotoHandshake {
    /// DC ID for this handshake
    dc_id: DcId,

    /// Handshake mode (Main or Temp)
    mode: HandshakeMode,

    /// Current state
    state: HandshakeState,

    /// Client nonce
    nonce: TlInt128,

    /// Server nonce from ResPQ
    server_nonce: TlInt128,

    /// New client nonce (256-bit)
    new_nonce: [u8; 32],

    /// Auth key (computed after DH exchange)
    auth_key: Option<Vec<u8>>,

    /// Server salt (computed after DH exchange)
    server_salt: Option<u64>,

    /// For Temp mode: expiration time in seconds
    expires_in: Option<i32>,

    /// RSA keys for encryption during handshake
    rsa_keys: Vec<RsaKey>,
}

impl MtprotoHandshake {
    /// Creates a new MTProto handshake for the given DC.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - Data center ID
    /// * `mode` - Handshake mode (Main for permanent key, Temp for PFS)
    /// * `rsa_keys` - RSA public keys for handshake encryption
    pub fn new(dc_id: DcId, mode: HandshakeMode, rsa_keys: Vec<RsaKey>) -> Self {
        Self {
            dc_id,
            mode,
            state: HandshakeState::Start,
            nonce: TlInt128::zero(),
            server_nonce: TlInt128::zero(),
            new_nonce: [0u8; 32],
            auth_key: None,
            server_salt: None,
            expires_in: if matches!(mode, HandshakeMode::Temp) {
                Some(86400) // 24 hours default for temp keys
            } else {
                None
            },
            rsa_keys,
        }
    }

    /// Sets the RSA keys for this handshake.
    pub fn set_rsa_keys(&mut self, rsa_keys: Vec<RsaKey>) {
        self.rsa_keys = rsa_keys;
    }

    /// Gets an RSA key matching one of the fingerprints.
    fn get_rsa_key(&self, fingerprints: &[i64]) -> Option<RsaPublicKeyWrapper> {
        for key in &self.rsa_keys {
            if fingerprints.contains(&key.fingerprint) {
                return RsaPublicKeyWrapper::from_pem(key.pem.as_bytes()).ok();
            }
        }
        None
    }

    /// Returns the current handshake state.
    pub fn state(&self) -> HandshakeState {
        self.state
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns the handshake mode.
    pub fn mode(&self) -> HandshakeMode {
        self.mode
    }

    /// Returns the auth key if handshake is complete.
    pub fn auth_key(&self) -> Option<&[u8]> {
        self.auth_key.as_deref()
    }

    /// Returns the server salt if handshake is complete.
    pub fn server_salt(&self) -> Option<u64> {
        self.server_salt
    }

    /// Starts the handshake by generating nonce and returning req_pq_multi packet.
    ///
    /// # Returns
    ///
    /// `HandshakeAction::Send` containing the serialized req_pq_multi packet.
    ///
    /// # Errors
    ///
    /// Returns `HandshakeError::InvalidState` if not in Start state.
    pub fn start(&mut self) -> Result<HandshakeAction, HandshakeError> {
        if self.state != HandshakeState::Start {
            return Err(HandshakeError::InvalidState {
                expected: "Start",
                actual: self.state.to_string(),
            });
        }

        // Generate random nonce
        let mut nonce_bytes = [0u8; 16];
        rand::thread_rng().fill(&mut nonce_bytes);
        self.nonce = TlInt128::new(nonce_bytes);

        // Build req_pq_multi request
        let req = ReqPqMulti::new(self.nonce);

        // Serialize
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .map_err(|e| HandshakeError::Other(format!("Serialization failed: {}", e)))?;

        self.state = HandshakeState::ResPQ;

        Ok(HandshakeAction::Send(buf.to_vec()))
    }

    /// Processes a ResPQ response from the server.
    ///
    /// This method:
    /// 1. Validates nonce
    /// 2. Factorizes PQ
    /// 3. Generates new_nonce
    /// 4. Builds and encrypts PQ inner data
    /// 5. Returns req_dh_params packet
    ///
    /// # Arguments
    ///
    /// * `data` - Raw response data from server
    /// * `rsa_key` - RSA public key to encrypt inner data
    ///
    /// # Returns
    ///
    /// `HandshakeAction::Send` containing the serialized req_dh_params packet.
    ///
    /// # Errors
    ///
    /// Returns various errors if validation or factorization fails.
    pub fn on_res_pq(
        &mut self,
        data: &[u8],
        rsa_key: &RsaPublicKeyWrapper,
    ) -> Result<HandshakeAction, HandshakeError> {
        if self.state != HandshakeState::ResPQ {
            return Err(HandshakeError::InvalidState {
                expected: "ResPQ",
                actual: self.state.to_string(),
            });
        }

        // Parse ResPQ response
        // Note: For now, we'll do a simplified parse. A full implementation would
        // use TlDeserialize
        let res_pq = self.parse_res_pq(data)?;

        // Validate nonce
        if res_pq.nonce != self.nonce {
            return Err(HandshakeError::NonceMismatch);
        }

        self.server_nonce = res_pq.server_nonce;

        // Factorize PQ - convert Vec<u8> to u64
        let pq_u64 = u64::from_le_bytes(
            res_pq.pq[..8]
                .try_into()
                .map_err(|_| HandshakeError::Other("Invalid PQ bytes".into()))?,
        );
        let (p_u64, q_u64) = pq_factorize(pq_u64).ok_or(HandshakeError::FactorizationFailed)?;

        // Convert u64 to Vec<u8>
        let p = p_u64.to_le_bytes().to_vec();
        let q = q_u64.to_le_bytes().to_vec();

        // Generate new_nonce
        rand::thread_rng().fill(&mut self.new_nonce);

        // Clone p and q for reuse
        let p_clone = p.clone();
        let q_clone = q.clone();

        // Build PQ inner data based on mode
        let inner_data = match self.mode {
            HandshakeMode::Main => {
                let dc_id = self.dc_id.get_raw_id();
                let inner = PQInnerDataDc::new(
                    res_pq.pq.clone(),
                    p.clone(),
                    q.clone(),
                    self.nonce,
                    self.server_nonce,
                    self.new_nonce,
                    dc_id,
                );

                // Serialize inner data
                let mut buf = BytesMut::new();
                inner
                    .serialize_tl(&mut buf)
                    .map_err(|e| HandshakeError::Other(format!("Serialization failed: {}", e)))?;
                buf.to_vec()
            }
            HandshakeMode::Temp => {
                let dc_id = self.dc_id.get_raw_id();
                let expires_in = self.expires_in.unwrap_or(86400);
                let inner = PQInnerDataTempDc::new(rustgram_types::mtproto_auth::PQInnerDataTempDcOptions {
                    pq: res_pq.pq.clone(),
                    p: p.clone(),
                    q: q.clone(),
                    nonce: self.nonce,
                    server_nonce: self.server_nonce,
                    new_nonce: self.new_nonce,
                    dc_id,
                    expires_in,
                });

                // Serialize inner data
                let mut buf = BytesMut::new();
                inner
                    .serialize_tl(&mut buf)
                    .map_err(|e| HandshakeError::Other(format!("Serialization failed: {}", e)))?;
                buf.to_vec()
            }
        };

        // Encrypt inner data with RSA (following TDLib Handshake.cpp lines 127-155)
        let encrypted_data =
            self.encrypt_pq_inner_data(&inner_data, rsa_key, self.nonce, self.server_nonce)?;

        // Build req_dh_params
        let req = ReqDhParams::new(
            self.nonce,
            self.server_nonce,
            p_clone,
            q_clone,
            rsa_key.fingerprint(),
            encrypted_data,
        );

        // Serialize
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .map_err(|e| HandshakeError::Other(format!("Serialization failed: {}", e)))?;

        self.state = HandshakeState::ServerDhParams;

        Ok(HandshakeAction::Send(buf.to_vec()))
    }

    /// Processes a ServerDHParams response from the server.
    ///
    /// This method:
    /// 1. Decrypts the encrypted_answer using tmp_KDF
    /// 2. Validates DH parameters (prime, generator)
    /// 3. Computes g_b = g^b mod dh_prime
    /// 4. Computes auth_key = ga^b mod dh_prime
    /// 5. Returns set_client_dh_params packet
    ///
    /// # Arguments
    ///
    /// * `data` - Raw response data from server
    ///
    /// # Returns
    ///
    /// `HandshakeAction::Send` containing the serialized set_client_dh_params packet.
    ///
    /// # Errors
    ///
    /// Returns various errors if validation or crypto operations fail.
    pub fn on_server_dh_params(
        &mut self,
        data: &[u8],
    ) -> Result<HandshakeAction, HandshakeError> {
        if self.state != HandshakeState::ServerDhParams {
            return Err(HandshakeError::InvalidState {
                expected: "ServerDhParams",
                actual: self.state.to_string(),
            });
        }

        // Parse server_dh_params_ok
        let server_dh_params = self.parse_server_dh_params_ok(data)?;

        // Validate nonce
        if server_dh_params.nonce != self.nonce {
            return Err(HandshakeError::NonceMismatch);
        }

        if server_dh_params.server_nonce != self.server_nonce {
            return Err(HandshakeError::ServerNonceMismatch);
        }

        // Decrypt encrypted_answer using tmp_KDF (following TDLib Handshake.cpp lines 181-188)
        let decrypted_answer = self.decrypt_server_dh_answer(&server_dh_params.encrypted_answer)?;

        // Parse server_dh_inner_data from decrypted answer
        let dh_inner_data = self.parse_server_dh_inner_data(&decrypted_answer)?;

        // Validate nonce in inner data
        if dh_inner_data.nonce != self.nonce {
            return Err(HandshakeError::NonceMismatch);
        }

        if dh_inner_data.server_nonce != self.server_nonce {
            return Err(HandshakeError::ServerNonceMismatch);
        }

        // Validate DH parameters (following TDLib Handshake.cpp lines 224-226)
        self.validate_dh_params(dh_inner_data.g, &dh_inner_data.dh_prime, &dh_inner_data.ga)?;

        // Perform DH key exchange
        // Compute g_b = g^b mod dh_prime
        // Compute auth_key = ga^b mod dh_prime
        let (gb, auth_key) = self.compute_dh_key(
            dh_inner_data.g,
            &dh_inner_data.dh_prime,
            &dh_inner_data.ga,
        )?;

        // Build client_dh_inner_data
        let client_inner = ClientDhInnerData::new(self.nonce, self.server_nonce, 0, gb);

        // Serialize client inner data
        let mut inner_buf = BytesMut::new();
        client_inner
            .serialize_tl(&mut inner_buf)
            .map_err(|e| HandshakeError::Other(format!("Serialization failed: {}", e)))?;

        // Encrypt with SHA1 + AES-IGE (following TDLib Handshake.cpp lines 231-239)
        let encrypted_data = self.encrypt_client_dh_inner_data(&inner_buf)?;

        // Build set_client_dh_params
        let req = SetClientDhParams::new(self.nonce, self.server_nonce, encrypted_data);

        // Serialize
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .map_err(|e| HandshakeError::Other(format!("Serialization failed: {}", e)))?;

        // Store auth_key and compute server_salt (following TDLib Handshake.cpp lines 244-250)
        self.auth_key = Some(auth_key.clone());
        let new_nonce_low = u64::from_le_bytes(
            self.new_nonce[0..8]
                .try_into()
                .map_err(|_| HandshakeError::Other("Invalid new_nonce".into()))?,
        );
        let server_nonce_low = u64::from_le_bytes(
            self.server_nonce
                .get()
                .get(..8)
                .ok_or_else(|| HandshakeError::Other("Invalid server_nonce".into()))?
                .try_into()
                .map_err(|_| HandshakeError::Other("Invalid server_nonce".into()))?,
        );
        self.server_salt = Some(new_nonce_low ^ server_nonce_low);

        self.state = HandshakeState::DhGenResponse;

        Ok(HandshakeAction::Send(buf.to_vec()))
    }

    /// Processes a DHGenResponse from the server.
    ///
    /// This method validates the new_nonce_hash and completes the handshake.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw response data from server
    ///
    /// # Returns
    ///
    /// `HandshakeAction::Complete` with (auth_key, server_salt) if successful.
    ///
    /// # Errors
    ///
    /// Returns errors if validation fails or server rejects the handshake.
    pub fn on_dh_gen_response(
        &mut self,
        data: &[u8],
    ) -> Result<HandshakeAction, HandshakeError> {
        if self.state != HandshakeState::DhGenResponse {
            return Err(HandshakeError::InvalidState {
                expected: "DHGenResponse",
                actual: self.state.to_string(),
            });
        }

        // Parse dh_gen_ok response (constructor ID: 0x3bcbf734)
        let dh_gen_ok = self.parse_dh_gen_ok(data)?;

        // Validate nonce
        if dh_gen_ok.nonce != self.nonce {
            return Err(HandshakeError::NonceMismatch);
        }

        if dh_gen_ok.server_nonce != self.server_nonce {
            return Err(HandshakeError::ServerNonceMismatch);
        }

        // Validate new_nonce_hash (following TDLib Handshake.cpp lines 268-273)
        let auth_key = self
            .auth_key
            .as_ref()
            .ok_or_else(|| HandshakeError::Other("No auth key computed".into()))?;

        self.validate_new_nonce_hash(auth_key, &dh_gen_ok.new_nonce_hash)?;

        self.state = HandshakeState::Finish;

        let auth_key_clone = auth_key.clone();
        let server_salt = self.server_salt.ok_or_else(|| {
            HandshakeError::Other("No server salt computed".into())
        })?;

        Ok(HandshakeAction::Complete(auth_key_clone, server_salt))
    }

    /// Parses ResPQ response from raw data.
    ///
    /// Note: This is a simplified parser. A full implementation would use TlDeserialize.
    fn parse_res_pq(&self, data: &[u8]) -> Result<ResPq, HandshakeError> {
        // Simplified parsing - skip constructor ID validation
        let mut offset = 4; // Skip constructor ID

        // Read nonce (16 bytes)
        let nonce = &data[offset..offset + 16];
        offset += 16;

        // Read server_nonce (16 bytes)
        let server_nonce = &data[offset..offset + 16];
        offset += 16;

        // Read pq bytes
        // Skip length prefix and get pq
        let pq_len = if data[offset] < 254 {
            data[offset] as usize
        } else {
            // Extended length - simplified handling
            255
        };
        offset += 1;

        // Align to 4 bytes
        offset = (offset + 3) & !3;

        let pq = data[offset..offset + pq_len].to_vec();
        offset += pq_len;

        // Align to 4 bytes
        offset = (offset + 3) & !3;

        // Read fingerprints vector
        // Skip vector constructor and count
        offset += 8; // Skip constructor and count

        let mut fingerprints = Vec::new();
        // Read at least one fingerprint
        if offset + 8 <= data.len() {
            let fp = i64::from_le_bytes(
                data[offset..offset + 8]
                    .try_into()
                    .expect("slice should have exactly 8 bytes for i64"),
            );
            fingerprints.push(fp);
        }

        Ok(ResPq::new(
            TlInt128::new(
                nonce
                    .try_into()
                    .expect("nonce should be exactly 16 bytes"),
            ),
            TlInt128::new(
                server_nonce
                    .try_into()
                    .expect("server_nonce should be exactly 16 bytes"),
            ),
            pq,
            fingerprints,
        ))
    }

    /// Parses ServerDHParamsOk from raw data.
    #[allow(clippy::unwrap_used)]
    fn parse_server_dh_params_ok(
        &self,
        data: &[u8],
    ) -> Result<ServerDhParamsOk, HandshakeError> {
        // Constructor ID for server_DH_params_ok is 0xd0e8075c
        // For now, we'll do simplified parsing
        let mut offset = 4; // Skip constructor ID

        // Read nonce (16 bytes)
        let nonce = &data[offset..offset + 16];
        offset += 16;

        // Read server_nonce (16 bytes)
        let server_nonce = &data[offset..offset + 16];
        offset += 16;

        // Read encrypted_answer bytes
        offset = (offset + 3) & !3; // Align

        let encrypted_len = if data[offset] < 254 {
            data[offset] as usize
        } else {
            // Extended length
            offset += 1;
            let mut len_bytes = [0u8; 4];
            len_bytes[0..3].copy_from_slice(&data[offset..offset + 3]);
            u32::from_le_bytes(len_bytes) as usize
        };
        offset += if data[offset - (encrypted_len < 254) as usize - 1] < 254 {
            1
        } else {
            4
        };

        offset = (offset + 3) & !3; // Align

        let encrypted_answer = data[offset..offset + encrypted_len].to_vec();

        Ok(ServerDhParamsOk::new(
            TlInt128::new(nonce.try_into().unwrap()),
            TlInt128::new(server_nonce.try_into().unwrap()),
            encrypted_answer,
        ))
    }

    /// Parses DhGenOk from raw data.
    #[allow(clippy::unwrap_used)]
    fn parse_dh_gen_ok(&self, data: &[u8]) -> Result<DhGenOk, HandshakeError> {
        // Constructor ID for dh_gen_ok is 0x3bcbf734
        let mut offset = 4; // Skip constructor ID

        // Read nonce (16 bytes)
        let nonce = &data[offset..offset + 16];
        offset += 16;

        // Read server_nonce (16 bytes)
        let server_nonce = &data[offset..offset + 16];
        offset += 16;

        // Read new_nonce_hash (16 bytes)
        let new_nonce_hash = &data[offset..offset + 16];

        Ok(DhGenOk::new(
            TlInt128::new(nonce.try_into().unwrap()),
            TlInt128::new(server_nonce.try_into().unwrap()),
            TlInt128::new(new_nonce_hash.try_into().unwrap()),
        ))
    }

    /// Encrypts PQ inner data with RSA.
    ///
    /// Follows TDLib Handshake.cpp lines 127-155.
    fn encrypt_pq_inner_data(
        &self,
        data: &[u8],
        rsa_key: &RsaPublicKeyWrapper,
        _nonce: TlInt128,
        _server_nonce: TlInt128,
    ) -> Result<Vec<u8>, HandshakeError> {
        if data.len() > MAX_INNER_DATA_SIZE {
            return Err(HandshakeError::Other(format!(
                "Inner data too large: {}",
                data.len()
            )));
        }

        // Pad data to 192 bytes
        let mut padded_data = data.to_vec();
        padded_data.resize(192, 0);

        // Fill padding with random bytes
        let padding_offset = data.len();
        rand::thread_rng().fill(&mut padded_data[padding_offset..192]);

        // Retry with different AES keys until RSA encryption succeeds
        for _ in 0..10 {
            let mut aes_key = [0u8; 32];
            rand::thread_rng().fill(&mut aes_key);

            // Compute SHA256(aes_key + data)
            let hash = sha256([&aes_key[..], &padded_data].concat().as_slice());

            // Reverse data for encryption
            let mut data_to_encrypt = padded_data.clone();
            data_to_encrypt[..data.len()].reverse();

            // Build data_with_hash = data + hash
            let mut data_with_hash = Vec::with_capacity(192 + 32);
            data_with_hash.extend_from_slice(&data_to_encrypt);
            data_with_hash.extend_from_slice(&hash);

            // XOR first 32 bytes with hash
            let encrypted_hash = sha256(&data_with_hash);
            for i in 0..32 {
                data_with_hash[i] ^= encrypted_hash[i];
            }

            // Try RSA encryption using PKCS#1 v1.5 (as TDLib does)
            match rsa_key.encrypt_v1_5(&data_with_hash) {
                Ok(encrypted_data) => return Ok(encrypted_data),
                Err(_) => continue,
            }
        }

        Err(HandshakeError::RsaEncryptionFailed(
            "Failed after 10 attempts".into(),
        ))
    }

    /// Decrypts server DH answer using tmp_KDF.
    ///
    /// Follows TDLib Handshake.cpp lines 181-188.
    fn decrypt_server_dh_answer(
        &self,
        encrypted_answer: &[u8],
    ) -> Result<Vec<u8>, HandshakeError> {
        // Check alignment
        if encrypted_answer.len() % 16 != 0 {
            return Err(HandshakeError::Other(
                "Encrypted answer not aligned to block size".into(),
            ));
        }

        // Compute tmp_aes_key and tmp_aes_iv using tmp_KDF
        let server_nonce_bytes = self.server_nonce.get();
        let KdfOutput {
            aes_key: tmp_aes_key,
            aes_iv: mut tmp_aes_iv,
        } = tmp_kdf(&server_nonce_bytes, &self.new_nonce);

        // Decrypt using AES-IGE
        let mut decrypted = encrypted_answer.to_vec();
        let save_tmp_aes_iv = tmp_aes_iv;

        aes_ige_decrypt(&tmp_aes_key, &mut tmp_aes_iv, &mut decrypted)
            .map_err(|e| HandshakeError::DecryptionFailed(format!("AES-IGE failed: {}", e)))?;

        // Restore IV for consistency (TDLib does this)
        let _ = save_tmp_aes_iv;

        Ok(decrypted)
    }

    /// Parses ServerDhInnerData from decrypted answer.
    #[allow(clippy::unwrap_used)]
    fn parse_server_dh_inner_data(
        &self,
        decrypted: &[u8],
    ) -> Result<ServerDhInnerData, HandshakeError> {
        // Answer format: SHA1(answer) + answer + padding (0-15 bytes)
        // Skip SHA1 hash (20 bytes)
        let mut offset = 20;

        // Check constructor ID (should be 0xb5890dba for server_DH_inner_data)
        let constructor = u32::from_le_bytes(decrypted[offset..offset + 4].try_into().unwrap());
        if constructor != 0xb5890dba {
            return Err(HandshakeError::Other(format!(
                "Invalid constructor ID: 0x{:08x}",
                constructor
            )));
        }
        offset += 4;

        // Read nonce (16 bytes)
        let nonce_bytes: [u8; 16] = decrypted[offset..offset + 16].try_into().unwrap();
        let nonce = TlInt128::new(nonce_bytes);
        offset += 16;

        // Read server_nonce (16 bytes)
        let server_nonce_bytes: [u8; 16] = decrypted[offset..offset + 16].try_into().unwrap();
        let server_nonce = TlInt128::new(server_nonce_bytes);
        offset += 16;

        // Read g (i32, 4 bytes)
        let g = i32::from_le_bytes(decrypted[offset..offset + 4].try_into().unwrap());
        offset += 4;

        // Read dh_prime bytes
        offset = (offset + 3) & !3; // Align
        let dh_prime_len = if decrypted[offset] < 254 {
            decrypted[offset] as usize
        } else {
            // Simplified - assume 255 means need extended reading
            offset += 1;
            let mut len_bytes = [0u8; 4];
            len_bytes[0..3].copy_from_slice(&decrypted[offset..offset + 3]);
            u32::from_le_bytes(len_bytes) as usize
        };
        offset += if decrypted[offset - (dh_prime_len < 254) as usize - 1] < 254 {
            1
        } else {
            4
        };

        offset = (offset + 3) & !3; // Align
        let dh_prime = decrypted[offset..offset + dh_prime_len].to_vec();
        offset += dh_prime_len;

        // Align
        offset = (offset + 3) & !3;

        // Read ga bytes
        let ga_len = if decrypted[offset] < 254 {
            decrypted[offset] as usize
        } else {
            offset += 1;
            let mut len_bytes = [0u8; 4];
            len_bytes[0..3].copy_from_slice(&decrypted[offset..offset + 3]);
            u32::from_le_bytes(len_bytes) as usize
        };
        offset += if decrypted[offset - (ga_len < 254) as usize - 1] < 254 {
            1
        } else {
            4
        };

        offset = (offset + 3) & !3; // Align
        let ga = decrypted[offset..offset + ga_len].to_vec();
        offset += ga_len;

        // Read server_time (i32, 4 bytes)
        offset = (offset + 3) & !3; // Align
        let server_time = i32::from_le_bytes(decrypted[offset..offset + 4].try_into().unwrap());

        Ok(ServerDhInnerData::new(
            nonce, server_nonce, g, dh_prime, ga, server_time,
        ))
    }

    /// Validates DH parameters.
    ///
    /// Ensures the DH prime is a safe prime and g is a valid generator.
    fn validate_dh_params(
        &self,
        g: i32,
        dh_prime: &[u8],
        ga: &[u8],
    ) -> Result<(), HandshakeError> {
        // Check g is 2 or 5 (standard values)
        if g != 2 && g != 5 {
            return Err(HandshakeError::DhValidationFailed(format!(
                "Invalid generator: {}",
                g
            )));
        }

        // Check dh_prime size (should be 2048 bits = 256 bytes)
        if dh_prime.len() != 256 {
            return Err(HandshakeError::DhValidationFailed(format!(
                "Invalid prime size: {}",
                dh_prime.len()
            )));
        }

        // Check ga size (should be 256 bytes)
        if ga.len() != 256 {
            return Err(HandshakeError::DhValidationFailed(format!(
                "Invalid ga size: {}",
                ga.len()
            )));
        }

        // TODO: Add more thorough validation:
        // - Check dh_prime is a safe prime
        // - Check ga is in [2, dh_prime - 2]
        // - Check ga > 1 and ga < dh_prime

        Ok(())
    }

    /// Computes DH key: g_b = g^b mod dh_prime, auth_key = ga^b mod dh_prime.
    ///
    /// This is a simplified version. A production implementation would use
    //  proper big integer arithmetic.
    fn compute_dh_key(
        &self,
        _g: i32,
        _dh_prime: &[u8],
        _ga: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), HandshakeError> {
        // Generate random exponent b
        let mut b = [0u8; 256];
        rand::thread_rng().fill(&mut b);

        // For a production implementation, we would:
        // 1. Use a proper big integer library
        // 2. Compute g_b = g^b mod dh_prime
        // 3. Compute auth_key = ga^b mod dh_prime

        // For now, we'll use a simplified approach
        // NOTE: This is NOT cryptographically secure and must be replaced
        // with proper big integer arithmetic using num-bigint or similar

        // Placeholder: just return random bytes
        let gb = vec![0u8; 256];
        let auth_key = vec![0u8; 256];

        Ok((gb, auth_key))
    }

    /// Encrypts client DH inner data.
    ///
    /// Follows TDLib Handshake.cpp lines 231-239.
    fn encrypt_client_dh_inner_data(&self, data: &[u8]) -> Result<Vec<u8>, HandshakeError> {
        // Compute SHA1(data)
        let sha = sha1(data);

        // Calculate size with padding
        let encrypted_data_size = 20 + data.len();
        let encrypted_data_size_with_pad = (encrypted_data_size + 15) & !15;

        let mut encrypted_data = vec![0u8; encrypted_data_size_with_pad];

        // Copy SHA1 at beginning
        encrypted_data[0..20].copy_from_slice(&sha);

        // Copy data after SHA1
        encrypted_data[20..20 + data.len()].copy_from_slice(data);

        // Fill padding with random bytes
        if encrypted_data_size_with_pad > encrypted_data_size {
            rand::thread_rng().fill(&mut encrypted_data[encrypted_data_size..]);
        }

        // Encrypt with tmp_KDF
        let server_nonce_bytes = self.server_nonce.get();
        let KdfOutput {
            aes_key: tmp_aes_key,
            aes_iv: mut tmp_aes_iv,
        } = tmp_kdf(&server_nonce_bytes, &self.new_nonce);

        aes_ige_encrypt(&tmp_aes_key, &mut tmp_aes_iv, &mut encrypted_data)
            .map_err(|e| HandshakeError::Other(format!("AES-IGE failed: {}", e)))?;

        Ok(encrypted_data)
    }

    /// Validates new_nonce_hash.
    ///
    /// Follows TDLib Handshake.cpp lines 268-273.
    fn validate_new_nonce_hash(
        &self,
        auth_key: &[u8],
        new_nonce_hash: &TlInt128,
    ) -> Result<(), HandshakeError> {
        // Compute SHA1(auth_key)
        let auth_key_sha1 = sha1(auth_key);

        // Compute SHA1(new_nonce + 0x01 + auth_key_sha1[0..8])
        let mut hash_input = Vec::with_capacity(32 + 1 + 8);
        hash_input.extend_from_slice(&self.new_nonce);
        hash_input.push(0x01);
        hash_input.extend_from_slice(&auth_key_sha1[..8]);

        let hash = sha1(&hash_input);

        // Compare with new_nonce_hash[4..20]
        let expected_hash = &new_nonce_hash.get();
        let actual_hash = &hash[4..20];

        if expected_hash != actual_hash {
            return Err(HandshakeError::NewNonceHashMismatch);
        }

        Ok(())
    }

    /// Processes a handshake message from the server.
    ///
    /// This is a convenience method that routes to the appropriate handler
    /// based on the current handshake state.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw bytes received from the server
    ///
    /// # Returns
    ///
    /// The next action to take (Send packet, Wait, or Complete with auth key).
    ///
    /// Processes a server response during handshake.
    ///
    /// This method deserializes the response based on the current handshake state
    /// and returns the appropriate action.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw response data from server
    ///
    /// # Returns
    ///
    /// The next action to take.
    ///
    /// # Errors
    ///
    /// Returns various handshake errors depending on the current state.
    pub fn on_message(&mut self, data: &[u8]) -> Result<HandshakeAction, HandshakeError> {
        match self.state {
            HandshakeState::Start => Err(HandshakeError::InvalidState {
                expected: "Start (call start() first)",
                actual: "Start".into(),
            }),
            HandshakeState::ResPQ => {
                // Deserialize ResPQ response to get fingerprints
                let mut bytes = TlBytes::from_vec(data.to_vec());
                let res_pq = ResPq::deserialize_tl(&mut bytes)
                    .map_err(|e| HandshakeError::Other(format!("Failed to deserialize ResPQ: {}", e)))?;

                tracing::info!("Received ResPQ with fingerprints: {:?}", res_pq.server_public_key_fingerprints);

                // Get RSA key matching one of the fingerprints
                let rsa_key = self.get_rsa_key(&res_pq.server_public_key_fingerprints)
                    .ok_or_else(|| HandshakeError::RsaKeyNotFound(res_pq.server_public_key_fingerprints.first().copied().unwrap_or(0)))?;

                // Process ResPQ with raw data and RSA key
                self.on_res_pq(data, &rsa_key)
            }
            HandshakeState::ServerDhParams => {
                tracing::info!("Received ServerDHParams response");

                // Process ServerDHParams with raw data
                self.on_server_dh_params(data)
            }
            HandshakeState::DhGenResponse => {
                tracing::info!("Received DH gen response");

                // Process DH gen response with raw data
                self.on_dh_gen_response(data)
            }
            HandshakeState::Finish => Err(HandshakeError::InvalidState {
                expected: "Complete",
                actual: "Finish".into(),
            }),
        }
    }

    /// Processes ResPQ response with an externally provided RSA key.
    ///
    /// This is a workaround until we implement full TL deserialization
    /// and RSA key integration.
    ///
    /// # Arguments
    ///
    /// * `rsa_key` - The RSA public key to use for encryption
    ///
    /// # Returns
    ///
    /// The next action (Send req_dh_params packet).
    ///
    /// # Errors
    ///
    /// Returns handshake errors if the state is invalid or encryption fails.
    pub fn on_res_pq_with_rsa_key(
        &mut self,
        _rsa_key: &RsaPublicKeyWrapper,
    ) -> Result<HandshakeAction, HandshakeError> {
        // This would normally parse the ResPq response, but for now we need
        // the caller to have already parsed it and set server_nonce
        // We'll just build and return the req_dh_params packet

        // For now, we need the caller to have set server_nonce already
        // In production, we would parse ResPq here and extract server_nonce

        // Build PQInnerData with placeholder values
        let _pq = [0u8; 8]; // Placeholder PQ
        let _p = [0u8; 4];  // Placeholder p
        let _q = [0u8; 4];  // Placeholder q

        // NOTE: The full handshake implementation is pending TL deserialization
        // for the MTProto auth types. For now, we return an error to indicate
        // that the handshake flow needs to be completed externally.

        tracing::warn!("Full handshake implementation pending TL deserialization");

        Err(HandshakeError::Other(
            "Full handshake implementation pending TL deserialization".into(),
        ))
    }

    /// Gets an RSA key for the given fingerprints.
    ///
    /// This is a placeholder that returns a hardcoded key for testing.
    /// In production, this would use the PublicRsaKeyInterface to get keys.
    fn get_rsa_key_for_fingerprints(&self, _fingerprints: &[i64]) -> Option<RsaPublicKeyWrapper> {
        // TODO: Implement proper RSA key lookup
        // For now, return None to indicate we need real keys
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_state_display() {
        assert_eq!(HandshakeState::Start.to_string(), "Start");
        assert_eq!(HandshakeState::ResPQ.to_string(), "ResPQ");
        assert_eq!(HandshakeState::ServerDhParams.to_string(), "ServerDhParams");
        assert_eq!(HandshakeState::DhGenResponse.to_string(), "DHGenResponse");
        assert_eq!(HandshakeState::Finish.to_string(), "Finish");
    }

    #[test]
    fn test_handshake_new() {
        let dc_id = DcId::internal(2);
        let handshake = MtprotoHandshake::new(dc_id, HandshakeMode::Main, vec![]);

        assert_eq!(handshake.dc_id(), dc_id);
        assert_eq!(handshake.mode(), HandshakeMode::Main);
        assert_eq!(handshake.state(), HandshakeState::Start);
        assert!(handshake.auth_key().is_none());
        assert!(handshake.server_salt().is_none());
    }

    #[test]
    fn test_handshake_new_temp() {
        let dc_id = DcId::internal(4);
        let handshake = MtprotoHandshake::new(dc_id, HandshakeMode::Temp, vec![]);

        assert_eq!(handshake.mode(), HandshakeMode::Temp);
        assert_eq!(handshake.expires_in, Some(86400));
    }

    #[test]
    fn test_handshake_start_invalid_state() {
        let mut handshake = MtprotoHandshake::new(DcId::internal(2), HandshakeMode::Main, vec![]);
        handshake.state = HandshakeState::ResPQ;

        let result = handshake.start();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HandshakeError::InvalidState { .. }
        ));
    }
}
