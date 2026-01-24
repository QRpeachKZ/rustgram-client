//! Primitive TL (Type Language) types.
//!
//! This module contains the basic types defined in the MTProto TL schema,
//! corresponding to the core types used in Telegram's protocol.

use crate::error::{TypeError, TypeResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

/// TL int32 type.
///
/// Corresponds to the `int` type in TL schema.
/// Constructor ID: 0xa8509bda (from TDLib tl_core.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TlInt(pub i32);

impl TlInt {
    /// Constructor ID for int type.
    pub const CONSTRUCTOR_ID: u32 = 0xa8509bda;

    /// Creates a new TL int.
    #[inline]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }
}

impl Default for TlInt {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for TlInt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for TlInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for TlInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<TlInt> for i32 {
    fn from(value: TlInt) -> Self {
        value.0
    }
}

impl Serialize for TlInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TlInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i32::deserialize(deserializer).map(TlInt)
    }
}

/// TL int64 type.
///
/// Corresponds to the `long` type in TL schema.
/// Constructor ID: 0x22076cba (from TDLib tl_core.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TlLong(pub i64);

impl TlLong {
    /// Constructor ID for long type.
    pub const CONSTRUCTOR_ID: u32 = 0x22076cba;

    /// Creates a new TL long.
    #[inline]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl Default for TlLong {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for TlLong {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for TlLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for TlLong {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<TlLong> for i64 {
    fn from(value: TlLong) -> Self {
        value.0
    }
}

impl Serialize for TlLong {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TlLong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer).map(TlLong)
    }
}

/// TL double type.
///
/// Corresponds to the `double` type in TL schema.
/// Constructor ID: 0x2210c154 (from TDLib tl_core.h)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TlDouble(pub f64);

impl TlDouble {
    /// Constructor ID for double type.
    pub const CONSTRUCTOR_ID: u32 = 0x2210c154;

    /// Creates a new TL double.
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    #[inline]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl Default for TlDouble {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Hash for TlDouble {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl fmt::Display for TlDouble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f64> for TlDouble {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<TlDouble> for f64 {
    fn from(value: TlDouble) -> Self {
        value.0
    }
}

impl Serialize for TlDouble {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TlDouble {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        f64::deserialize(deserializer).map(TlDouble)
    }
}

/// TL string type.
///
/// Corresponds to the `string` type in TL schema.
/// Constructor ID: 0xb5286e24 (from TDLib tl_core.h)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TlString(pub String);

impl TlString {
    /// Constructor ID for string type.
    pub const CONSTRUCTOR_ID: u32 = 0xb5286e24;

    /// Creates a new TL string.
    #[inline]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    #[inline]
    pub fn get(&self) -> &str {
        &self.0
    }

    /// Returns the inner String, consuming self.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Returns the length of the string.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the string is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for TlString {
    fn default() -> Self {
        Self(String::new())
    }
}

impl AsRef<str> for TlString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TlString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TlString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for TlString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<TlString> for String {
    fn from(value: TlString) -> Self {
        value.0
    }
}

impl FromStr for TlString {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TlString::new(s.to_string()))
    }
}

impl Serialize for TlString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TlString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(TlString)
    }
}

/// TL bytes type.
///
/// Corresponds to the `bytes` type in TL schema.
/// Bytes are serialized differently from strings in MTProto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlBytes(pub Vec<u8>);

impl TlBytes {
    /// Creates a new TL bytes.
    #[inline]
    pub fn new(value: Vec<u8>) -> Self {
        Self(value)
    }

    /// Creates a new TL bytes from a slice.
    #[inline]
    pub fn from_slice(value: &[u8]) -> Self {
        Self(value.to_vec())
    }

    /// Returns the inner value.
    #[inline]
    pub fn get(&self) -> &[u8] {
        &self.0
    }

    /// Returns the inner Vec, consuming self.
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    /// Returns the length of the bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the bytes are empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for TlBytes {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl AsRef<[u8]> for TlBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Hash for TlBytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for TlBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} bytes>", self.0.len())
    }
}

impl From<Vec<u8>> for TlBytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for TlBytes {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}

impl From<TlBytes> for Vec<u8> {
    fn from(value: TlBytes) -> Self {
        value.0
    }
}

impl Serialize for TlBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as bytes if supported, otherwise as base64 string
        if serializer.is_human_readable() {
            use base64::prelude::{Engine, BASE64_STANDARD};
            serializer.serialize_str(&BASE64_STANDARD.encode(&self.0))
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for TlBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor;

        impl serde::de::Visitor<'_> for BytesVisitor {
            type Value = TlBytes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("bytes or base64-encoded string")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TlBytes(v.to_vec()))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use base64::prelude::{Engine, BASE64_STANDARD};
                BASE64_STANDARD
                    .decode(v)
                    .map(TlBytes)
                    .map_err(|e| E::custom(format!("invalid base64: {e}")))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(BytesVisitor)
    }
}

/// TL int128 type (16 bytes).
///
/// Used for cryptographic operations and certain internal Telegram types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TlInt128([u8; 16]);

impl TlInt128 {
    /// Creates a new TL int128 from bytes.
    #[inline]
    pub const fn new(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Creates a new zero int128.
    #[inline]
    pub const fn zero() -> Self {
        Self([0; 16])
    }

    /// Returns the inner bytes.
    #[inline]
    pub const fn get(&self) -> [u8; 16] {
        self.0
    }

    /// Converts from u128 (little-endian).
    pub fn from_u128(value: u128) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&value.to_le_bytes());
        Self(bytes)
    }

    /// Converts to u128 (little-endian).
    pub fn to_u128(&self) -> u128 {
        u128::from_le_bytes(self.0)
    }
}

impl Default for TlInt128 {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for TlInt128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl From<[u8; 16]> for TlInt128 {
    fn from(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

impl From<TlInt128> for [u8; 16] {
    fn from(value: TlInt128) -> Self {
        value.0
    }
}

impl From<u128> for TlInt128 {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl Serialize for TlInt128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&hex::encode(self.0))
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for TlInt128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Int128Visitor;

        impl serde::de::Visitor<'_> for Int128Visitor {
            type Value = TlInt128;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("16-byte array or hex-encoded string")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() == 16 {
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(v);
                    Ok(TlInt128(bytes))
                } else {
                    Err(E::custom(format!("expected 16 bytes, got {}", v.len())))
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let bytes = hex::decode(v).map_err(|e| E::custom(format!("invalid hex: {e}")))?;
                if bytes.len() == 16 {
                    let mut arr = [0u8; 16];
                    arr.copy_from_slice(&bytes);
                    Ok(TlInt128(arr))
                } else {
                    Err(E::custom(format!("expected 16 bytes, got {}", bytes.len())))
                }
            }
        }

        deserializer.deserialize_any(Int128Visitor)
    }
}

/// TL int256 type (32 bytes).
///
/// Used for cryptographic operations and MTProto internal types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TlInt256([u8; 32]);

impl TlInt256 {
    /// Creates a new TL int256 from bytes.
    #[inline]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Creates a new zero int256.
    #[inline]
    pub const fn zero() -> Self {
        Self([0; 32])
    }

    /// Returns the inner bytes.
    #[inline]
    pub const fn get(&self) -> [u8; 32] {
        self.0
    }

    /// Converts from two u128 values (little-endian).
    pub fn from_u128_pair(low: u128, high: u128) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..16].copy_from_slice(&low.to_le_bytes());
        bytes[16..32].copy_from_slice(&high.to_le_bytes());
        Self(bytes)
    }

    /// Splits into two u128 values (little-endian).
    pub fn to_u128_pair(&self) -> (u128, u128) {
        // SAFETY: TlInt256 always contains exactly 32 bytes, so both slices
        // [0..16] and [16..32] are guaranteed to have the correct size.
        let low = u128::from_le_bytes(unsafe { *(self.0[0..16].as_ptr() as *const [u8; 16]) });
        let high = u128::from_le_bytes(unsafe { *(self.0[16..32].as_ptr() as *const [u8; 16]) });
        (low, high)
    }
}

impl Default for TlInt256 {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for TlInt256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl From<[u8; 32]> for TlInt256 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl From<TlInt256> for [u8; 32] {
    fn from(value: TlInt256) -> Self {
        value.0
    }
}

impl Serialize for TlInt256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&hex::encode(self.0))
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for TlInt256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Int256Visitor;

        impl serde::de::Visitor<'_> for Int256Visitor {
            type Value = TlInt256;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("32-byte array or hex-encoded string")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() == 32 {
                    let mut bytes = [0u8; 32];
                    bytes.copy_from_slice(v);
                    Ok(TlInt256(bytes))
                } else {
                    Err(E::custom(format!("expected 32 bytes, got {}", v.len())))
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let bytes = hex::decode(v).map_err(|e| E::custom(format!("invalid hex: {e}")))?;
                if bytes.len() == 32 {
                    let mut arr = [0u8; 32];
                    arr.copy_from_slice(&bytes);
                    Ok(TlInt256(arr))
                } else {
                    Err(E::custom(format!("expected 32 bytes, got {}", bytes.len())))
                }
            }
        }

        deserializer.deserialize_any(Int256Visitor)
    }
}

/// UInt256 type (32 bytes).
///
/// Alias for TlInt256 for TDLib compatibility.
pub type UInt256 = TlInt256;

/// TL Bool type.
///
/// In MTProto, bool values have specific constructor IDs:
/// - boolFalse: 0xbc799737
/// - boolTrue: 0x997275b5
///
/// This enum represents those values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TlBool {
    /// False value (constructor ID: 0xbc799737).
    False,
    /// True value (constructor ID: 0x997275b5).
    True,
}

impl TlBool {
    /// Constructor ID for boolFalse.
    pub const FALSE_ID: u32 = 0xbc799737;

    /// Constructor ID for boolTrue.
    pub const TRUE_ID: u32 = 0x997275b5;

    /// Creates a new TL bool from a constructor ID.
    pub fn from_constructor_id(id: u32) -> TypeResult<Self> {
        match id {
            Self::FALSE_ID => Ok(Self::False),
            Self::TRUE_ID => Ok(Self::True),
            _ => Err(TypeError::InvalidValue(format!(
                "Invalid bool constructor ID: 0x{:08x}",
                id
            ))),
        }
    }

    /// Returns the constructor ID for this bool value.
    #[inline]
    pub const fn constructor_id(self) -> u32 {
        match self {
            Self::False => Self::FALSE_ID,
            Self::True => Self::TRUE_ID,
        }
    }

    /// Returns the Rust bool value.
    #[inline]
    pub const fn as_bool(self) -> bool {
        matches!(self, Self::True)
    }

    /// Creates a TL bool from a Rust bool.
    #[inline]
    pub const fn from_bool(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl Default for TlBool {
    fn default() -> Self {
        Self::False
    }
}

impl fmt::Display for TlBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_bool())
    }
}

impl From<bool> for TlBool {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl From<TlBool> for bool {
    fn from(value: TlBool) -> Self {
        value.as_bool()
    }
}

impl Serialize for TlBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.as_bool())
    }
}

impl<'de> Deserialize<'de> for TlBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        bool::deserialize(deserializer).map(TlBool::from_bool)
    }
}

/// TL True type.
///
/// This is a special type in TL schema that only has one value: true.
/// Constructor ID: 0x3fedd339
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TlTrue;

impl TlTrue {
    /// Constructor ID for True type.
    pub const CONSTRUCTOR_ID: u32 = 0x3fedd339;

    /// Returns the singleton True value.
    #[inline]
    pub const fn get() -> Self {
        TlTrue
    }
}

impl Default for TlTrue {
    fn default() -> Self {
        TlTrue
    }
}

impl fmt::Display for TlTrue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "true")
    }
}

impl Serialize for TlTrue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

impl<'de> Deserialize<'de> for TlTrue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TrueVisitor;

        impl serde::de::Visitor<'_> for TrueVisitor {
            type Value = TlTrue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("true value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v {
                    Ok(TlTrue)
                } else {
                    Err(E::custom("expected true, got false"))
                }
            }
        }

        deserializer.deserialize_any(TrueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tl_int() {
        let val = TlInt::new(42);
        assert_eq!(val.get(), 42);
        assert_eq!(val, TlInt::from(42));
    }

    #[test]
    fn test_tl_long() {
        let val = TlLong::new(42i64);
        assert_eq!(val.get(), 42);
    }

    #[test]
    fn test_tl_string() {
        let val: TlString = "hello".parse().expect("valid string");
        assert_eq!(val.get(), "hello");
        assert_eq!(val.len(), 5);
        assert!(!val.is_empty());
    }

    #[test]
    fn test_tl_bool() {
        assert!(TlBool::True.as_bool());
        assert!(!TlBool::False.as_bool());
        assert_eq!(TlBool::from_bool(true), TlBool::True);
        assert_eq!(TlBool::from_bool(false), TlBool::False);
    }

    #[test]
    fn test_tl_bool_constructor_id() {
        assert_eq!(TlBool::True.constructor_id(), 0x997275b5);
        assert_eq!(TlBool::False.constructor_id(), 0xbc799737);
    }

    #[test]
    fn test_tl_int128() {
        let val = TlInt128::from_u128(0x123456789abcdef0);
        assert_eq!(val.to_u128(), 0x123456789abcdef0);
    }

    #[test]
    fn test_tl_int256() {
        let val = TlInt256::from_u128_pair(0x1111, 0x2222);
        let (low, high) = val.to_u128_pair();
        assert_eq!(low, 0x1111);
        assert_eq!(high, 0x2222);
    }
}
