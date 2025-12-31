// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Cryptography primitives for MTProto 2.0.
//!
//! This module provides the cryptographic operations needed for MTProto,
//! including AES-IGE encryption/decryption, key derivation functions (KDF),
//! and hash functions.
//!
//! # Overview
//!
//! MTProto 2.0 uses:
//! - AES-256-IGE for encryption/decryption
//! - SHA-256 for message key derivation and integrity checks
//! - SHA-1 for legacy (v1.0) compatibility
//!
//! # References
//!
//! - TDLib: `td/mtproto/KDF.h`, `td/mtproto/KDF.cpp`
//! - TDLib: `td/utils/crypto.h`
//! - MTProto 2.0: <https://core.telegram.org/mtproto/description>

mod aes_ige;
mod crypto_auth_key;
mod hash;
mod kdf;

pub use aes_ige::{aes_ige_decrypt, aes_ige_encrypt, AesIge, CryptoError};
pub use crypto_auth_key::{
    compute_auth_key_id, AuthKeyError, AuthKeyHelper, ComputeAuthKeyId, CryptoAuthKey,
    CryptoAuthKeyHelper, DefaultAuthKeyHelper,
};
pub use hash::{sha1, sha256};
pub use kdf::{kdf, kdf2, tmp_kdf, KdfOutput};

/// Prelude for crypto module imports.
pub mod prelude {
    pub use super::{aes_ige_decrypt, aes_ige_encrypt, AesIge, CryptoError};
    pub use super::{kdf, kdf2, tmp_kdf, KdfOutput};
    pub use super::{sha1, sha256};
    pub use super::{
        AuthKeyError, AuthKeyHelper, CryptoAuthKey, CryptoAuthKeyHelper, DefaultAuthKeyHelper,
    };
}
