// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Key Derivation Functions (KDF) for MTProto.
//!
//! Based on TDLib's `td/mtproto/KDF.h` and `td/mtproto/KDF.cpp`.
//!
//! MTProto 2.0 uses SHA-256 based key derivation for computing
//! AES keys and IVs from the auth key and message key.

use crate::crypto::{sha1, sha256};

/// Key derivation output containing AES key and IV.
///
/// MTProto 2.0 derives 256-bit AES keys and 256-bit IVs from
/// the auth key (2048 bits) and message key (128 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfOutput {
    /// Derived AES-256 key (32 bytes).
    pub aes_key: [u8; 32],
    /// Derived AES-256 IV (32 bytes).
    pub aes_iv: [u8; 32],
}

impl Default for KdfOutput {
    fn default() -> Self {
        Self {
            aes_key: [0u8; 32],
            aes_iv: [0u8; 32],
        }
    }
}

impl KdfOutput {
    /// Creates a new `KdfOutput` from the derived key and IV.
    #[must_use]
    pub const fn new(aes_key: [u8; 32], aes_iv: [u8; 32]) -> Self {
        Self { aes_key, aes_iv }
    }
}

/// MTProto 1.0 key derivation function (SHA1-based).
///
/// This function is used for MTProto v1.0 compatibility.
///
/// # Algorithm
///
/// ```text
/// sha256_a = SHA256(msg_key + substr(auth_key, 0, 36))
/// sha256_b = SHA256(substr(auth_key, 40, 36) + msg_key)
///
/// aes_key = substr(sha256_a, 0, 8) + substr(sha256_b, 8, 24)
/// aes_iv  = substr(sha256_b, 0, 8) + substr(sha256_a, 24, 24)
/// ```
///
/// # Arguments
///
/// * `auth_key` - 2048-bit (256 byte) authentication key
/// * `msg_key` - 128-bit (16 byte) message key
/// * `x` - Offset into auth_key (0 for client-to-server, 8 for server-to-client)
///
/// # Returns
///
/// Derived AES key and IV
///
/// # References
///
/// - TDLib: `td/mtproto/KDF.cpp` - `KDF` function
///
/// # Panics
///
/// Panics if `auth_key.len() != 256` or `msg_key.len() != 16`.
#[must_use]
pub fn kdf(auth_key: &[u8], msg_key: &[u8; 16], x: usize) -> KdfOutput {
    assert_eq!(auth_key.len(), 256, "auth_key must be 256 bytes");

    // Based on TDLib KDF::KDF (MTProto v1 with SHA1)
    // sha256_a = SHA256 (msg_key + substr(auth_key, x, 36));
    let mut buf_a = [0u8; 16 + 36];
    buf_a[..16].copy_from_slice(msg_key);
    buf_a[16..].copy_from_slice(&auth_key[x..x + 36]);
    let sha256_a = sha256(&buf_a);

    // sha256_b = SHA256 (substr(auth_key, 40+x, 36) + msg_key);
    let mut buf_b = [0u8; 36 + 16];
    buf_b[..36].copy_from_slice(&auth_key[40 + x..40 + x + 36]);
    buf_b[36..].copy_from_slice(msg_key);
    let sha256_b = sha256(&buf_b);

    // aes_key = substr(sha256_a, 0, 8) + substr(sha256_b, 8, 16) + substr(sha256_a, 24, 8);
    let mut aes_key = [0u8; 32];
    aes_key[0..8].copy_from_slice(&sha256_a[0..8]);
    aes_key[8..24].copy_from_slice(&sha256_b[8..24]);
    aes_key[24..32].copy_from_slice(&sha256_a[24..32]);

    // aes_iv = substr(sha256_b, 0, 8) + substr(sha256_a, 8, 16) + substr(sha256_b, 24, 8);
    let mut aes_iv = [0u8; 32];
    aes_iv[0..8].copy_from_slice(&sha256_b[0..8]);
    aes_iv[8..24].copy_from_slice(&sha256_a[8..24]);
    aes_iv[24..32].copy_from_slice(&sha256_b[24..32]);

    KdfOutput { aes_key, aes_iv }
}

/// MTProto 2.0 key derivation function (SHA256-based).
///
/// This is the current version of MTProto key derivation.
///
/// # Algorithm
///
/// ```text
/// sha256_a = SHA256(msg_key + substr(auth_key, x, 36))
/// sha256_b = SHA256(substr(auth_key, 40+x, 36) + msg_key)
///
/// aes_key = substr(sha256_a, 0, 8) + substr(sha256_b, 8, 16) + substr(sha256_a, 24, 8)
/// aes_iv  = substr(sha256_b, 0, 8) + substr(sha256_a, 8, 16) + substr(sha256_b, 24, 8)
/// ```
///
/// # Arguments
///
/// * `auth_key` - 2048-bit (256 byte) authentication key
/// * `msg_key` - 128-bit (16 byte) message key
/// * `x` - Offset into auth_key (0 for client-to-server, 8 for server-to-client)
///
/// # Returns
///
/// Derived AES key and IV
///
/// # References
///
/// - TDLib: `td/mtproto/KDF.cpp` - `KDF2` function
/// - MTProto 2.0: <https://core.telegram.org/mtproto/description>
///
/// # Panics
///
/// Panics if `auth_key.len() != 256` or `msg_key.len() != 16`.
#[must_use]
pub fn kdf2(auth_key: &[u8], msg_key: &[u8; 16], x: usize) -> KdfOutput {
    assert_eq!(auth_key.len(), 256, "auth_key must be 256 bytes");

    // Based on TDLib KDF::KDF2 (MTProto v2 with SHA256)

    // sha256_a = SHA256 (msg_key + substr(auth_key, x, 36));
    let mut buf_a = [0u8; 16 + 36];
    buf_a[..16].copy_from_slice(msg_key);
    buf_a[16..].copy_from_slice(&auth_key[x..x + 36]);
    let sha256_a = sha256(&buf_a);

    // sha256_b = SHA256 (substr(auth_key, 40+x, 36) + msg_key);
    let mut buf_b = [0u8; 36 + 16];
    buf_b[..36].copy_from_slice(&auth_key[40 + x..40 + x + 36]);
    buf_b[36..].copy_from_slice(msg_key);
    let sha256_b = sha256(&buf_b);

    // aes_key = substr(sha256_a, 0, 8) + substr(sha256_b, 8, 16) + substr(sha256_a, 24, 8);
    let mut aes_key = [0u8; 32];
    aes_key[0..8].copy_from_slice(&sha256_a[0..8]);
    aes_key[8..24].copy_from_slice(&sha256_b[8..24]);
    aes_key[24..32].copy_from_slice(&sha256_a[24..32]);

    // aes_iv = substr(sha256_b, 0, 8) + substr(sha256_a, 8, 16) + substr(sha256_b, 24, 8);
    let mut aes_iv = [0u8; 32];
    aes_iv[0..8].copy_from_slice(&sha256_b[0..8]);
    aes_iv[8..24].copy_from_slice(&sha256_a[8..24]);
    aes_iv[24..32].copy_from_slice(&sha256_b[24..32]);

    KdfOutput { aes_key, aes_iv }
}

/// Temporary key derivation function for DH exchange.
///
/// Used during the initial key exchange handshake.
///
/// # Algorithm
///
/// ```text
/// tmp_aes_key := SHA1(new_nonce + server_nonce) + substr(SHA1(server_nonce + new_nonce), 0, 12)
/// tmp_aes_iv := substr(SHA1(server_nonce + new_nonce), 12, 8) + SHA1(new_nonce + new_nonce) + substr(new_nonce, 0, 4)
/// ```
///
/// # Arguments
///
/// * `server_nonce` - 128-bit (16 byte) server nonce
/// * `new_nonce` - 256-bit (32 byte) client nonce
///
/// # Returns
///
/// Derived temporary AES key and IV
///
/// # References
///
/// - TDLib: `td/mtproto/KDF.cpp` - `tmp_KDF` function
#[must_use]
pub fn tmp_kdf(server_nonce: &[u8; 16], new_nonce: &[u8; 32]) -> KdfOutput {
    // tmp_aes_key := SHA1(new_nonce + server_nonce) + substr(SHA1(server_nonce + new_nonce), 0, 12);
    let mut buf_new_server = [0u8; 32 + 16];
    buf_new_server[..32].copy_from_slice(new_nonce);
    buf_new_server[32..].copy_from_slice(server_nonce);
    let sha_new_server = sha1(&buf_new_server);

    let mut buf_server_new = [0u8; 16 + 32];
    buf_server_new[..16].copy_from_slice(server_nonce);
    buf_server_new[16..].copy_from_slice(new_nonce);
    let sha_server_new = sha1(&buf_server_new);

    let mut tmp_aes_key = [0u8; 32];
    tmp_aes_key[..20].copy_from_slice(&sha_new_server);
    tmp_aes_key[20..].copy_from_slice(&sha_server_new[..12]);

    // tmp_aes_iv := substr(SHA1(server_nonce + new_nonce), 12, 8) + SHA1(new_nonce + new_nonce) + substr(new_nonce, 0, 4)
    let mut buf_new_new = [0u8; 32 + 32];
    buf_new_new[..32].copy_from_slice(new_nonce);
    buf_new_new[32..].copy_from_slice(new_nonce);
    let sha_new_new = sha1(&buf_new_new);

    let mut tmp_aes_iv = [0u8; 32];
    tmp_aes_iv[..8].copy_from_slice(&sha_server_new[12..20]);
    tmp_aes_iv[8..28].copy_from_slice(&sha_new_new);
    tmp_aes_iv[28..].copy_from_slice(&new_nonce[..4]);

    KdfOutput {
        aes_key: tmp_aes_key,
        aes_iv: tmp_aes_iv,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_output_default() {
        let output = KdfOutput::default();
        assert_eq!(output.aes_key, [0u8; 32]);
        assert_eq!(output.aes_iv, [0u8; 32]);
    }

    #[test]
    fn test_kdf_output_new() {
        let key = [1u8; 32];
        let iv = [2u8; 32];
        let output = KdfOutput::new(key, iv);
        assert_eq!(output.aes_key, key);
        assert_eq!(output.aes_iv, iv);
    }

    #[test]
    #[should_panic(expected = "auth_key must be 256 bytes")]
    fn test_kdf_invalid_auth_key_length() {
        let auth_key = [0u8; 128]; // Wrong length
        let msg_key = [0u8; 16];
        kdf(&auth_key, &msg_key, 0);
    }

    #[test]
    #[should_panic(expected = "auth_key must be 256 bytes")]
    fn test_kdf2_invalid_auth_key_length() {
        let auth_key = [0u8; 128]; // Wrong length
        let msg_key = [0u8; 16];
        kdf2(&auth_key, &msg_key, 0);
    }

    #[test]
    fn test_kdf_deterministic() {
        let auth_key = [42u8; 256];
        let msg_key = [1u8; 16];

        let output1 = kdf(&auth_key, &msg_key, 0);
        let output2 = kdf(&auth_key, &msg_key, 0);

        assert_eq!(output1, output2);
    }

    #[test]
    fn test_kdf2_deterministic() {
        let auth_key = [42u8; 256];
        let msg_key = [1u8; 16];

        let output1 = kdf2(&auth_key, &msg_key, 0);
        let output2 = kdf2(&auth_key, &msg_key, 0);

        assert_eq!(output1, output2);
    }

    #[test]
    fn test_tmp_kdf_deterministic() {
        let server_nonce = [1u8; 16];
        let new_nonce = [2u8; 32];

        let output1 = tmp_kdf(&server_nonce, &new_nonce);
        let output2 = tmp_kdf(&server_nonce, &new_nonce);

        assert_eq!(output1, output2);
    }

    #[test]
    fn test_kdf_different_x_different_output() {
        let auth_key = [42u8; 256];
        let msg_key = [1u8; 16];

        let output0 = kdf(&auth_key, &msg_key, 0);
        let output8 = kdf(&auth_key, &msg_key, 8);

        assert_ne!(output0, output8);
    }

    #[test]
    fn test_kdf2_different_x_different_output() {
        let auth_key = [42u8; 256];
        let msg_key = [1u8; 16];

        let output0 = kdf2(&auth_key, &msg_key, 0);
        let output8 = kdf2(&auth_key, &msg_key, 8);

        assert_ne!(output0, output8);
    }

    #[test]
    fn test_kdf_different_msg_key_different_output() {
        let auth_key = [42u8; 256];
        let msg_key1 = [1u8; 16];
        let msg_key2 = [2u8; 16];

        let output1 = kdf(&auth_key, &msg_key1, 0);
        let output2 = kdf(&auth_key, &msg_key2, 0);

        assert_ne!(output1, output2);
    }

    #[test]
    fn test_kdf2_different_msg_key_different_output() {
        let auth_key = [42u8; 256];
        let msg_key1 = [1u8; 16];
        let msg_key2 = [2u8; 16];

        let output1 = kdf2(&auth_key, &msg_key1, 0);
        let output2 = kdf2(&auth_key, &msg_key2, 0);

        assert_ne!(output1, output2);
    }

    #[test]
    fn test_tmp_kdf_different_nonces_different_output() {
        let server_nonce1 = [1u8; 16];
        let new_nonce1 = [2u8; 32];
        let server_nonce2 = [3u8; 16];
        let new_nonce2 = [4u8; 32];

        let output1 = tmp_kdf(&server_nonce1, &new_nonce1);
        let output2 = tmp_kdf(&server_nonce2, &new_nonce2);

        assert_ne!(output1, output2);
    }

    #[test]
    fn test_kdf_output_clone() {
        let auth_key = [42u8; 256];
        let msg_key = [1u8; 16];

        let output1 = kdf(&auth_key, &msg_key, 0);
        let output2 = output1;

        assert_eq!(output1, output2);
    }

    #[test]
    fn test_kdf_key_iv_are_32_bytes() {
        let auth_key = [42u8; 256];
        let msg_key = [1u8; 16];

        let output = kdf(&auth_key, &msg_key, 0);
        assert_eq!(output.aes_key.len(), 32);
        assert_eq!(output.aes_iv.len(), 32);

        let output = kdf2(&auth_key, &msg_key, 0);
        assert_eq!(output.aes_key.len(), 32);
        assert_eq!(output.aes_iv.len(), 32);

        let server_nonce = [1u8; 16];
        let new_nonce = [2u8; 32];
        let output = tmp_kdf(&server_nonce, &new_nonce);
        assert_eq!(output.aes_key.len(), 32);
        assert_eq!(output.aes_iv.len(), 32);
    }

    #[test]
    fn test_kdf_avalanche_effect() {
        let auth_key = [42u8; 256];
        let msg_key1 = [0u8; 16];
        let mut msg_key2 = [0u8; 16];
        msg_key2[0] = 1; // One bit difference

        let output1 = kdf2(&auth_key, &msg_key1, 0);
        let output2 = kdf2(&auth_key, &msg_key2, 0);

        // Most bytes should differ (avalanche effect)
        let key_diff = output1
            .aes_key
            .iter()
            .zip(output2.aes_key.iter())
            .filter(|(a, b)| a != b)
            .count();
        let iv_diff = output1
            .aes_iv
            .iter()
            .zip(output2.aes_iv.iter())
            .filter(|(a, b)| a != b)
            .count();

        assert!(key_diff > 10, "Key should show avalanche effect");
        assert!(iv_diff > 10, "IV should show avalanche effect");
    }

    #[test]
    fn test_kdf_debug() {
        let auth_key = [42u8; 256];
        let msg_key = [1u8; 16];

        let output = kdf(&auth_key, &msg_key, 0);
        let debug_str = format!("{output:?}");

        assert!(debug_str.contains("KdfOutput"));
    }
}
