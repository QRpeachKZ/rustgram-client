//! TL (Type Language) serialization traits.
//!
//! This module defines traits for TL serialization and deserialization,
//! which is the custom binary format used by MTProto.

use crate::error::{TypeError, TypeResult};
use bytes::{Buf, BufMut, BytesMut};

/// Trait for types that can be serialized to TL format.
pub trait TlSerialize {
    /// Serializes this value to the given buffer.
    ///
    /// # Errors
    /// Returns a TypeError if serialization fails.
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()>;
}

/// Trait for types that can be deserialized from TL format.
pub trait TlDeserialize: Sized {
    /// Deserializes a value from the given buffer.
    ///
    /// # Errors
    /// Returns a TypeError if deserialization fails.
    fn deserialize_tl(buf: &mut Bytes) -> TypeResult<Self>;
}

/// Trait for types that have a TL constructor ID.
pub trait TlConstructor {
    /// Returns the constructor ID for this type.
    fn constructor_id(&self) -> u32;
}

/// Helper trait for boxed TL objects (polymorphic types).
pub trait TlBoxed: TlConstructor + TlSerialize + TlDeserialize {
    /// Returns the type name for this object.
    fn type_name(&self) -> &'static str;

    /// Creates a boxed instance from a constructor ID and buffer.
    fn from_constructor_id(id: u32, buf: &mut Bytes) -> TypeResult<Self>
    where
        Self: Sized;
}

/// Helper functions for TL serialization.
pub struct TlHelper;

impl TlHelper {
    /// Writes an i32 to the buffer in little-endian format.
    #[inline]
    pub fn write_i32(buf: &mut BytesMut, value: i32) {
        buf.put_i32_le(value);
    }

    /// Writes a constructor ID (u32) to the buffer as little-endian bytes.
    ///
    /// Constructor IDs in MTProto are conceptually unsigned, but are
    /// written as raw bytes. When read back, they are interpreted as i32.
    /// This function handles the conversion correctly.
    #[inline]
    pub fn write_constructor_id(buf: &mut BytesMut, value: u32) {
        buf.put_u32_le(value);
    }

    /// Writes an i64 to the buffer in little-endian format.
    #[inline]
    pub fn write_i64(buf: &mut BytesMut, value: i64) {
        buf.put_i64_le(value);
    }

    /// Writes a double to the buffer in little-endian format.
    #[inline]
    pub fn write_f64(buf: &mut BytesMut, value: f64) {
        buf.put_f64_le(value);
    }

    /// Writes a bytes value to the buffer with length prefix.
    ///
    /// In MTProto, bytes are serialized as:
    /// - If length < 254: 1 byte length + data + padding
    /// - If length >= 254: 4 bytes length + data + padding
    pub fn write_bytes(buf: &mut BytesMut, data: &[u8]) {
        let len = data.len();
        if len < 254 {
            buf.put_u8(len as u8);
            buf.put_slice(data);
            // Add padding to align to 4 bytes
            let padding = (4 - ((len + 1) % 4)) % 4;
            for _ in 0..padding {
                buf.put_u8(0);
            }
        } else {
            buf.put_u8(254);
            // Write 3 bytes of length (little-endian)
            let len_bytes = (len as u32).to_le_bytes();
            buf.put_slice(&len_bytes[0..3]);
            buf.put_slice(data);
            let padding = (4 - (len % 4)) % 4;
            for _ in 0..padding {
                buf.put_u8(0);
            }
        }
    }

    /// Writes a string to the buffer (same as bytes with UTF-8 encoding).
    #[inline]
    pub fn write_string(buf: &mut BytesMut, s: &str) {
        Self::write_bytes(buf, s.as_bytes());
    }

    /// Reads an i32 from the buffer in little-endian format.
    #[inline]
    pub fn read_i32(buf: &mut Bytes) -> TypeResult<i32> {
        if buf.remaining() < 4 {
            return Err(TypeError::DeserializationError(
                "not enough bytes for i32".to_string(),
            ));
        }
        Ok(buf.get_i32_le())
    }

    /// Reads an i64 from the buffer in little-endian format.
    #[inline]
    pub fn read_i64(buf: &mut Bytes) -> TypeResult<i64> {
        if buf.remaining() < 8 {
            return Err(TypeError::DeserializationError(
                "not enough bytes for i64".to_string(),
            ));
        }
        Ok(buf.get_i64_le())
    }

    /// Reads a double from the buffer in little-endian format.
    #[inline]
    pub fn read_f64(buf: &mut Bytes) -> TypeResult<f64> {
        if buf.remaining() < 8 {
            return Err(TypeError::DeserializationError(
                "not enough bytes for f64".to_string(),
            ));
        }
        Ok(buf.get_f64_le())
    }

    /// Reads a bytes value from the buffer.
    pub fn read_bytes(buf: &mut Bytes) -> TypeResult<Vec<u8>> {
        if buf.is_empty() {
            return Err(TypeError::DeserializationError(
                "not enough bytes for length prefix".to_string(),
            ));
        }

        let first = buf.get_u8();
        let len = if first < 254 {
            first as usize
        } else {
            if buf.remaining() < 3 {
                return Err(TypeError::DeserializationError(
                    "not enough bytes for extended length".to_string(),
                ));
            }
            // Read 3 bytes (little-endian)
            let mut len_bytes = [0u8; 4];
            len_bytes[0..3].copy_from_slice(&buf.chunk()[0..3]);
            buf.advance(3);
            let len_low = u32::from_le_bytes(len_bytes) as usize;
            len_low
        };

        // Calculate padding
        let data_len = len;
        let prefix_len = if first < 254 { 1 } else { 4 };
        let total_with_prefix = data_len + prefix_len;
        let padding = (4 - (total_with_prefix % 4)) % 4;

        if buf.remaining() < len + padding {
            return Err(TypeError::DeserializationError(
                "not enough bytes for data".to_string(),
            ));
        }

        let mut data = vec![0u8; len];
        buf.copy_to_slice(&mut data);

        // Skip padding
        for _ in 0..padding {
            buf.get_u8();
        }

        Ok(data)
    }

    /// Reads a string from the buffer (same as bytes with UTF-8 validation).
    pub fn read_string(buf: &mut Bytes) -> TypeResult<String> {
        let bytes = Self::read_bytes(buf)?;
        String::from_utf8(bytes)
            .map_err(|e| TypeError::DeserializationError(format!("invalid UTF-8 string: {e}")))
    }

    /// Reads a constructor ID from the buffer.
    #[inline]
    pub fn read_constructor_id(buf: &mut Bytes) -> TypeResult<u32> {
        Self::read_i32(buf).map(|v| v as u32)
    }
}

/// Wrapper type for Bytes to use with TL deserialization.
pub struct Bytes {
    inner: bytes::Bytes,
}

impl Bytes {
    /// Creates a new Bytes wrapper.
    #[inline]
    pub fn new(bytes: bytes::Bytes) -> Self {
        Self { inner: bytes }
    }

    /// Creates from a Vec<u8>.
    #[inline]
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            inner: bytes::Bytes::from(vec),
        }
    }

    /// Returns the number of remaining bytes.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    /// Checks if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Advances the buffer by n bytes.
    #[inline]
    pub fn advance(&mut self, n: usize) {
        self.inner.advance(n);
    }

    /// Gets a u8 from the buffer.
    #[inline]
    pub fn get_u8(&mut self) -> u8 {
        self.inner.get_u8()
    }

    /// Gets a u16 in little-endian format.
    #[inline]
    pub fn get_u16_le(&mut self) -> u16 {
        self.inner.get_u16_le()
    }

    /// Gets a u24 in little-endian format.
    #[inline]
    pub fn get_u24_le(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        bytes[0..3].copy_from_slice(&self.inner.chunk()[0..3]);
        self.inner.advance(3);
        u32::from_le_bytes(bytes)
    }

    /// Gets a u32 in little-endian format.
    #[inline]
    pub fn get_u32_le(&mut self) -> u32 {
        self.inner.get_u32_le()
    }

    /// Gets an i32 in little-endian format.
    #[inline]
    pub fn get_i32_le(&mut self) -> i32 {
        self.inner.get_i32_le()
    }

    /// Gets an i64 in little-endian format.
    #[inline]
    pub fn get_i64_le(&mut self) -> i64 {
        self.inner.get_i64_le()
    }

    /// Gets a f64 in little-endian format.
    #[inline]
    pub fn get_f64_le(&mut self) -> f64 {
        self.inner.get_f64_le()
    }

    /// Copies bytes to a slice.
    #[inline]
    pub fn copy_to_slice(&mut self, dst: &mut [u8]) {
        self.inner.copy_to_slice(dst);
    }
}

impl Buf for Bytes {
    fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    fn chunk(&self) -> &[u8] {
        self.inner.chunk()
    }

    fn advance(&mut self, cnt: usize) {
        self.inner.advance(cnt);
    }
}

impl Default for TlHelper {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_read_i32() {
        let mut buf = BytesMut::new();
        TlHelper::write_i32(&mut buf, 0x12345678);
        assert_eq!(buf.len(), 4);

        let mut bytes = Bytes::new(buf.freeze());
        let value = TlHelper::read_i32(&mut bytes).unwrap();
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn test_write_read_i64() {
        let mut buf = BytesMut::new();
        TlHelper::write_i64(&mut buf, 0x123456789abcdef0);
        assert_eq!(buf.len(), 8);

        let mut bytes = Bytes::new(buf.freeze());
        let value = TlHelper::read_i64(&mut bytes).unwrap();
        assert_eq!(value, 0x123456789abcdef0);
    }

    #[test]
    fn test_write_read_bytes() {
        let data = b"hello world";
        let mut buf = BytesMut::new();
        TlHelper::write_bytes(&mut buf, data);

        let mut bytes = Bytes::new(buf.freeze());
        let read_data = TlHelper::read_bytes(&mut bytes).unwrap();
        assert_eq!(&read_data[..], data);
    }

    #[test]
    fn test_write_read_string() {
        let s = "hello world";
        let mut buf = BytesMut::new();
        TlHelper::write_string(&mut buf, s);

        let mut bytes = Bytes::new(buf.freeze());
        let read_s = TlHelper::read_string(&mut bytes).unwrap();
        assert_eq!(read_s, s);
    }

    #[test]
    fn test_bytes_padding() {
        // Test that small bytes are properly padded
        let data = vec![1u8, 2, 3]; // 3 bytes
        let mut buf = BytesMut::new();
        TlHelper::write_bytes(&mut buf, &data);

        // Should have: 1 byte length + 3 bytes data + 0 padding = 4 bytes (aligned)
        assert_eq!(buf.len(), 4);

        let mut bytes = Bytes::new(buf.freeze());
        let read_data = TlHelper::read_bytes(&mut bytes).unwrap();
        assert_eq!(read_data, data);
    }
}
