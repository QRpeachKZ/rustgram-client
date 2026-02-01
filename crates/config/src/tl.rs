// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! TL (Type Language) definitions for config-related RPC operations.
//!
//! This module defines the TL constructors and serializers for all
//! config-related requests and responses according to the MTProto protocol.

use bytes::BytesMut;
use rustgram_types::tl::{TlHelper, TlSerialize};
use rustgram_types::TlBool;

use crate::error::{ConfigError, Result};

/// Magic bytes for empty config response.
pub const CONFIG_EMPTY_MAGIC: u32 = 0x77454b61;

/// Magic bytes for config with DC options.
pub const CONFIG_DC_OPTIONS_MAGIC: u32 = 0x88bbb4c0;

/// Magic bytes for getAppConfig request.
pub const GET_APP_CONFIG_MAGIC: u32 = 0x61e3f854;

/// Magic bytes for appConfig response (version 110).
pub const APP_CONFIG_MAGIC: u32 = 0xdd18782e;

/// Magic bytes for vector serialization/deserialization.
pub const VECTOR_MAGIC: u32 = 0x1cb5c415;

/// Magic bytes for getDhConfig request.
pub const GET_DH_CONFIG_MAGIC: u32 = 0x26cf8950;

/// Magic bytes for dhConfig response.
pub const DH_CONFIG_MAGIC: u32 = 0x2c221edd;

/// Magic bytes for dhConfigNotModified response.
pub const DH_CONFIG_NOT_MODIFIED_MAGIC: u32 = 0xc0e24635;

/// Magic bytes for bool true.
pub const BOOL_TRUE_MAGIC: u32 = 0x997275b5;

/// Magic bytes for bool false.
pub const BOOL_FALSE_MAGIC: u32 = 0xbc799737;

/// TL serialization error.
#[derive(Debug, thiserror::Error)]
pub enum TlError {
    /// Invalid constructor ID.
    #[error("Invalid constructor ID: 0x{0:08x}")]
    InvalidConstructor(u32),

    /// Not enough bytes to deserialize.
    #[error("Not enough bytes: need {need}, have {have}")]
    NotEnoughBytes {
        /// Number of bytes needed.
        need: usize,
        /// Number of bytes available.
        have: usize,
    },

    /// Deserialization error.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<TlError> for ConfigError {
    fn from(err: TlError) -> Self {
        ConfigError::SerializationError(err.to_string())
    }
}

/// Request: config.getConfig
///
/// Fetches the basic Telegram configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConfig;

impl GetConfig {
    /// Creates a new getConfig request.
    pub fn new() -> Self {
        Self
    }

    /// Returns the constructor ID.
    pub const fn constructor_id(&self) -> u32 {
        0xc4f9186b
    }
}

impl Default for GetConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TlSerialize for GetConfig {
    fn serialize_tl(&self, buf: &mut BytesMut) -> rustgram_types::TypeResult<()> {
        TlHelper::write_i32(buf, self.constructor_id() as i32);
        Ok(())
    }
}

/// Response: config_dcOptions
///
/// Configuration response containing DC options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigDcOptions {
    /// Available data center options.
    pub dc_options: Vec<DcOption>,
}

impl ConfigDcOptions {
    /// Creates a new ConfigDcOptions.
    pub fn new(dc_options: Vec<DcOption>) -> Self {
        Self { dc_options }
    }
}

impl ConfigDcOptions {
    /// Deserialize from bytes using the TL format.
    pub fn deserialize_tl_bytes(data: bytes::Bytes) -> Result<Self> {
        let mut buf = rustgram_types::tl::Bytes::new(data);

        let magic = TlHelper::read_constructor_id(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        if magic != CONFIG_DC_OPTIONS_MAGIC {
            return Err(TlError::InvalidConstructor(magic).into());
        }

        // Read vector
        let vec_magic = TlHelper::read_constructor_id(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        if vec_magic != VECTOR_MAGIC {
            return Err(TlError::InvalidConstructor(vec_magic).into());
        }

        let count = TlHelper::read_i32(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        if count < 0 {
            return Err(TlError::DeserializationError("Negative vector length".to_string()).into());
        }

        let count = count as usize;
        let mut dc_options = Vec::with_capacity(count);

        for _ in 0..count {
            dc_options.push(DcOption::deserialize_tl_internal(&mut buf)?);
        }

        Ok(Self { dc_options })
    }
}

/// Response: config_dcOptions_empty
///
/// Empty configuration response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigDcOptionsEmpty;

impl ConfigDcOptionsEmpty {
    /// Creates a new empty config response.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConfigDcOptionsEmpty {
    fn default() -> Self {
        Self::new()
    }
}

/// TL representation of a DC option.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DcOption {
    /// DC ID.
    pub dc_id: i32,
    /// IP address.
    pub ip_address: String,
    /// Port number.
    pub port: i32,
    /// Option flags.
    pub flags: i32,
}

impl DcOption {
    /// Creates a new DC option.
    pub fn new(dc_id: i32, ip_address: String, port: i32, flags: i32) -> Self {
        Self {
            dc_id,
            ip_address,
            port,
            flags,
        }
    }

    /// Deserialize from internal buffer.
    fn deserialize_tl_internal(buf: &mut rustgram_types::tl::Bytes) -> Result<Self> {
        let _magic = TlHelper::read_constructor_id(buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let flags =
            TlHelper::read_i32(buf).map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let dc_id =
            TlHelper::read_i32(buf).map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let ip_address =
            TlHelper::read_string(buf).map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let port =
            TlHelper::read_i32(buf).map_err(|e| TlError::DeserializationError(e.to_string()))?;

        Ok(Self {
            dc_id,
            ip_address,
            port,
            flags,
        })
    }
}

impl TlSerialize for DcOption {
    fn serialize_tl(&self, buf: &mut BytesMut) -> rustgram_types::TypeResult<()> {
        TlHelper::write_i32(buf, 0x18da7a8a); // constructor for dcOption
        TlHelper::write_i32(buf, self.flags);
        TlHelper::write_i32(buf, self.dc_id);
        TlHelper::write_string(buf, &self.ip_address);
        TlHelper::write_i32(buf, self.port);
        Ok(())
    }
}

/// Request: help.getAppConfig
///
/// Fetches the application configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GetAppConfig {
    /// Hash for caching (0 for no cache).
    pub hash: i32,
}

impl GetAppConfig {
    /// Creates a new getAppConfig request.
    pub fn new(hash: i32) -> Self {
        Self { hash }
    }
}

impl TlSerialize for GetAppConfig {
    fn serialize_tl(&self, buf: &mut BytesMut) -> rustgram_types::TypeResult<()> {
        TlHelper::write_i32(buf, GET_APP_CONFIG_MAGIC as i32);
        TlHelper::write_i32(buf, self.hash);
        Ok(())
    }
}

/// Response: appConfig (version 110)
///
/// Application configuration response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfigTl {
    /// Config version.
    pub version: i32,
    /// Config date.
    pub date: i32,
    /// Config expiration time.
    pub expires: i32,
    /// Whether this is test mode.
    pub test_mode: TlBool,
    /// Config data as raw JSON bytes.
    pub config: Vec<u8>,
}

impl AppConfigTl {
    /// Creates a new AppConfigTl.
    pub fn new(version: i32, date: i32, expires: i32, test_mode: bool, config: Vec<u8>) -> Self {
        Self {
            version,
            date,
            expires,
            test_mode: if test_mode {
                TlBool::True
            } else {
                TlBool::False
            },
            config,
        }
    }

    /// Deserialize from bytes using the TL format.
    pub fn deserialize_tl_bytes(data: bytes::Bytes) -> Result<Self> {
        let mut buf = rustgram_types::tl::Bytes::new(data);

        let magic = TlHelper::read_constructor_id(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        if magic != APP_CONFIG_MAGIC {
            return Err(TlError::InvalidConstructor(magic).into());
        }

        let version = TlHelper::read_i32(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let date = TlHelper::read_i32(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let expires = TlHelper::read_i32(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let test_mode_raw = TlHelper::read_i32(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let test_mode = match test_mode_raw as u32 {
            BOOL_TRUE_MAGIC => TlBool::True,
            BOOL_FALSE_MAGIC => TlBool::False,
            _ => {
                return Err(TlError::DeserializationError(format!(
                    "Invalid bool value: 0x{:08x}",
                    test_mode_raw
                ))
                .into());
            }
        };

        let config = TlHelper::read_bytes(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        Ok(Self {
            version,
            date,
            expires,
            test_mode,
            config,
        })
    }
}

/// Request: messages.getDhConfig
///
/// Fetches Diffie-Hellman configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDhConfig {
    /// Current DH version.
    pub version: i32,
    /// Random data length.
    pub random_length: i32,
}

impl GetDhConfig {
    /// Creates a new getDhConfig request.
    pub fn new(version: i32, random_length: i32) -> Self {
        Self {
            version,
            random_length,
        }
    }
}

impl TlSerialize for GetDhConfig {
    fn serialize_tl(&self, buf: &mut BytesMut) -> rustgram_types::TypeResult<()> {
        TlHelper::write_i32(buf, GET_DH_CONFIG_MAGIC as i32);
        TlHelper::write_i32(buf, self.version);
        TlHelper::write_i32(buf, self.random_length);
        Ok(())
    }
}

/// Response: messages.dhConfig
///
/// DH configuration response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhConfigTl {
    /// Generator value.
    pub g: i32,
    /// Prime modulus.
    pub prime: Vec<u8>,
    /// DH config version.
    pub version: i32,
}

impl DhConfigTl {
    /// Creates a new DhConfigTl.
    pub fn new(g: i32, prime: Vec<u8>, version: i32) -> Self {
        Self { g, prime, version }
    }

    /// Deserialize from bytes using the TL format.
    pub fn deserialize_tl_bytes(data: bytes::Bytes) -> Result<Self> {
        let mut buf = rustgram_types::tl::Bytes::new(data);

        let magic = TlHelper::read_constructor_id(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        if magic != DH_CONFIG_MAGIC {
            return Err(TlError::InvalidConstructor(magic).into());
        }

        let g = TlHelper::read_i32(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let prime = TlHelper::read_bytes(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        let version = TlHelper::read_i32(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        Ok(Self { g, prime, version })
    }
}

/// Response: messages.dhConfigNotModified
///
/// DH config hasn't changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhConfigNotModified {
    /// Random data.
    pub random: Vec<u8>,
}

impl DhConfigNotModified {
    /// Creates a new DhConfigNotModified.
    pub fn new(random: Vec<u8>) -> Self {
        Self { random }
    }

    /// Deserialize from bytes using the TL format.
    pub fn deserialize_tl_bytes(data: bytes::Bytes) -> Result<Self> {
        let mut buf = rustgram_types::tl::Bytes::new(data);

        let magic = TlHelper::read_constructor_id(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        if magic != DH_CONFIG_NOT_MODIFIED_MAGIC {
            return Err(TlError::InvalidConstructor(magic).into());
        }

        let random = TlHelper::read_bytes(&mut buf)
            .map_err(|e| TlError::DeserializationError(e.to_string()))?;

        Ok(Self { random })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_serialize() {
        let req = GetConfig::new();
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .expect("GetConfig serialization should succeed");

        assert_eq!(buf.len(), 4);
        // First byte of 0xc4f9186b (little-endian)
        assert_eq!(buf[0], 0x6b);
    }

    #[test]
    fn test_get_app_config_serialize() {
        let req = GetAppConfig::new(123);
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .expect("GetAppConfig serialization should succeed");

        assert_eq!(buf.len(), 8);
    }

    #[test]
    fn test_get_dh_config_serialize() {
        let req = GetDhConfig::new(1, 256);
        let mut buf = BytesMut::new();
        req.serialize_tl(&mut buf)
            .expect("GetDhConfig serialization should succeed");

        assert_eq!(buf.len(), 12);
    }

    #[test]
    fn test_dc_option_serialize() {
        let opt = DcOption::new(2, "149.154.167.51".to_string(), 443, 0);
        let mut buf = BytesMut::new();
        opt.serialize_tl(&mut buf)
            .expect("DcOption serialization should succeed");

        assert!(!buf.is_empty());
    }

    #[test]
    fn test_magic_values() {
        assert_eq!(GET_APP_CONFIG_MAGIC, 0x61e3f854);
        assert_eq!(APP_CONFIG_MAGIC, 0xdd18782e);
        assert_eq!(GET_DH_CONFIG_MAGIC, 0x26cf8950);
        assert_eq!(DH_CONFIG_MAGIC, 0x2c221edd);
        assert_eq!(DH_CONFIG_NOT_MODIFIED_MAGIC, 0xc0e24635);
        assert_eq!(CONFIG_DC_OPTIONS_MAGIC, 0x88bbb4c0);
        assert_eq!(CONFIG_EMPTY_MAGIC, 0x77454b61);
        assert_eq!(VECTOR_MAGIC, 0x1cb5c415);
    }
}
