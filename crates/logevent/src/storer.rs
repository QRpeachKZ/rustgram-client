// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! TL (Type Language) storer for serializing log event data
//!
//! Provides serialization primitives for writing TL-encoded binary data.

use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Cursor, Write};

/// TL storer trait for writing binary data
///
/// This trait provides methods to serialize data in TL format.
/// TL is Telegram's custom binary serialization format.
pub trait TlStorer {
    /// Stores a 32-bit signed integer
    fn store_i32(&mut self, value: i32);

    /// Stores a 32-bit unsigned integer
    fn store_u32(&mut self, value: u32);

    /// Stores a 64-bit signed integer
    fn store_i64(&mut self, value: i64);

    /// Stores a 64-bit unsigned integer
    fn store_u64(&mut self, value: u64);

    /// Stores a 64-bit floating point value
    fn store_f64(&mut self, value: f64) {
        self.store_u64(value.to_bits());
    }

    /// Stores a boolean value (stored as i32 in TL)
    fn store_bool(&mut self, value: bool) {
        self.store_i32(value as i32);
    }

    /// Stores a byte vector with length prefix
    fn store_bytes(&mut self, data: &[u8]);

    /// Returns the current size of stored data
    fn len(&self) -> usize;

    /// Returns true if no data has been stored
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// TL storer that calculates the length without allocating storage
///
/// Useful for pre-calculating buffer sizes before actual serialization.
#[derive(Debug, Default)]
pub struct LogEventStorerCalcLength {
    length: usize,
}

impl LogEventStorerCalcLength {
    /// Creates a new length calculator
    #[must_use]
    pub const fn new() -> Self {
        Self { length: 0 }
    }

    /// Returns the calculated length
    #[must_use]
    pub const fn get_length(&self) -> usize {
        self.length
    }

    /// Resets the length counter
    pub fn reset(&mut self) {
        self.length = 0;
    }
}

impl TlStorer for LogEventStorerCalcLength {
    fn store_i32(&mut self, _value: i32) {
        self.length += 4;
    }

    fn store_u32(&mut self, _value: u32) {
        self.length += 4;
    }

    fn store_i64(&mut self, _value: i64) {
        self.length += 8;
    }

    fn store_u64(&mut self, _value: u64) {
        self.length += 8;
    }

    fn store_bytes(&mut self, data: &[u8]) {
        self.length += 4; // length prefix
        self.length += data.len();
    }

    fn len(&self) -> usize {
        self.length
    }
}

/// TL storer that writes directly to a byte buffer
///
/// # Example
///
/// ```
/// use rustgram_logevent::{LogEventStorerUnsafe, TlStorer};
///
/// let mut buf = vec![0u8; 16];
/// let mut storer = LogEventStorerUnsafe::new(&mut buf);
/// storer.store_i32(42);
/// storer.store_i64(12345);
/// assert_eq!(&buf[..12], &[0x00, 0x00, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39]);
/// ```
#[derive(Debug)]
pub struct LogEventStorerUnsafe<'a> {
    cursor: Cursor<&'a mut [u8]>,
}

impl<'a> LogEventStorerUnsafe<'a> {
    /// Creates a new storer that writes to the provided buffer
    ///
    /// # Panics
    ///
    /// Panics if the buffer is too small for the data being written.
    #[must_use]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            cursor: Cursor::new(buf),
        }
    }

    /// Returns the number of bytes written so far
    #[must_use]
    pub fn bytes_written(&self) -> usize {
        self.cursor.position() as usize
    }

    /// Returns a slice of the data written so far
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        let pos = self.cursor.position() as usize;
        &self.cursor.get_ref()[..pos]
    }

    /// Resets the storer to the beginning of the buffer
    pub fn reset(&mut self) {
        self.cursor.set_position(0);
    }

    /// Returns the remaining capacity in the buffer
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.cursor
            .get_ref()
            .len()
            .saturating_sub(self.cursor.position() as usize)
    }
}

impl<'a> TlStorer for LogEventStorerUnsafe<'a> {
    #[allow(clippy::expect_used)]
    fn store_i32(&mut self, value: i32) {
        // SAFETY: Writing to Cursor<&mut [u8]> only fails if the buffer is too small.
        // This is by design - the user must ensure adequate buffer capacity.
        self.cursor
            .write_i32::<BigEndian>(value)
            .expect("Buffer too small - user must ensure adequate capacity");
    }

    #[allow(clippy::expect_used)]
    fn store_u32(&mut self, value: u32) {
        // SAFETY: Writing to Cursor<&mut [u8]> only fails if the buffer is too small.
        // This is by design - the user must ensure adequate buffer capacity.
        self.cursor
            .write_u32::<BigEndian>(value)
            .expect("Buffer too small - user must ensure adequate capacity");
    }

    #[allow(clippy::expect_used)]
    fn store_i64(&mut self, value: i64) {
        // SAFETY: Writing to Cursor<&mut [u8]> only fails if the buffer is too small.
        // This is by design - the user must ensure adequate buffer capacity.
        self.cursor
            .write_i64::<BigEndian>(value)
            .expect("Buffer too small - user must ensure adequate capacity");
    }

    #[allow(clippy::expect_used)]
    fn store_u64(&mut self, value: u64) {
        // SAFETY: Writing to Cursor<&mut [u8]> only fails if the buffer is too small.
        // This is by design - the user must ensure adequate buffer capacity.
        self.cursor
            .write_u64::<BigEndian>(value)
            .expect("Buffer too small - user must ensure adequate capacity");
    }

    #[allow(clippy::expect_used)]
    fn store_bytes(&mut self, data: &[u8]) {
        // SAFETY: Writing to Cursor<&mut [u8]> only fails if the buffer is too small.
        // This is by design - the user must ensure adequate buffer capacity.
        let len = data.len() as i32;
        self.store_i32(len);
        self.cursor
            .write_all(data)
            .expect("Buffer too small - user must ensure adequate capacity");
    }

    fn len(&self) -> usize {
        self.bytes_written()
    }
}

/// Vec-based storer for convenience
///
/// Automatically grows the buffer as needed.
#[derive(Debug, Default)]
pub struct LogEventStorerVec {
    data: Vec<u8>,
}

impl LogEventStorerVec {
    /// Creates a new vec-based storer
    #[must_use]
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Returns the serialized data
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }

    /// Returns a reference to the serialized data
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Clears the storer
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Creates a new storer with pre-allocated capacity
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

impl TlStorer for LogEventStorerVec {
    #[allow(clippy::expect_used)]
    fn store_i32(&mut self, value: i32) {
        // SAFETY: Writing to Vec<u8> only fails if length exceeds isize::MAX.
        // This is practically impossible as it would require ~8 exabytes of data.
        self.data
            .write_i32::<BigEndian>(value)
            .expect("Vec write should never fail");
    }

    #[allow(clippy::expect_used)]
    fn store_u32(&mut self, value: u32) {
        // SAFETY: Writing to Vec<u8> only fails if length exceeds isize::MAX.
        // This is practically impossible as it would require ~8 exabytes of data.
        self.data
            .write_u32::<BigEndian>(value)
            .expect("Vec write should never fail");
    }

    #[allow(clippy::expect_used)]
    fn store_i64(&mut self, value: i64) {
        // SAFETY: Writing to Vec<u8> only fails if length exceeds isize::MAX.
        // This is practically impossible as it would require ~8 exabytes of data.
        self.data
            .write_i64::<BigEndian>(value)
            .expect("Vec write should never fail");
    }

    #[allow(clippy::expect_used)]
    fn store_u64(&mut self, value: u64) {
        // SAFETY: Writing to Vec<u8> only fails if length exceeds isize::MAX.
        // This is practically impossible as it would require ~8 exabytes of data.
        self.data
            .write_u64::<BigEndian>(value)
            .expect("Vec write should never fail");
    }

    fn store_bytes(&mut self, data: &[u8]) {
        let len = data.len() as i32;
        self.store_i32(len);
        self.data.extend_from_slice(data);
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

/// Helper trait for storing TL objects
///
/// This trait is reserved for future use when implementing automatic TL
/// serialization for complex types. Currently unused but part of the public API.
#[allow(dead_code)]
pub trait StoreTl {
    /// Store self to a TL storer
    fn store<S: TlStorer>(&self, storer: &mut S);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_length_i32() {
        let mut storer = LogEventStorerCalcLength::new();
        storer.store_i32(42);
        assert_eq!(storer.get_length(), 4);
    }

    #[test]
    fn test_calc_length_i64() {
        let mut storer = LogEventStorerCalcLength::new();
        storer.store_i64(42);
        assert_eq!(storer.get_length(), 8);
    }

    #[test]
    fn test_calc_length_bytes() {
        let mut storer = LogEventStorerCalcLength::new();
        storer.store_bytes(&[1, 2, 3, 4, 5]);
        assert_eq!(storer.get_length(), 9); // 4 for length + 5 for data
    }

    #[test]
    fn test_calc_length_combined() {
        let mut storer = LogEventStorerCalcLength::new();
        storer.store_i32(42);
        storer.store_i64(123);
        storer.store_bytes(&[1, 2]);
        assert_eq!(storer.get_length(), 18); // 4 + 8 + 6
    }

    #[test]
    fn test_unsafe_storer_i32() {
        let mut buf = vec![0u8; 4];
        let mut storer = LogEventStorerUnsafe::new(&mut buf);
        storer.store_i32(0x12345678);
        assert_eq!(storer.bytes_written(), 4);
        assert_eq!(buf, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_unsafe_storer_i64() {
        let mut buf = vec![0u8; 8];
        let mut storer = LogEventStorerUnsafe::new(&mut buf);
        storer.store_i64(0x123456789ABCDEF0);
        assert_eq!(storer.bytes_written(), 8);
        assert_eq!(buf, [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
    }

    #[test]
    fn test_unsafe_storer_bytes() {
        let mut buf = vec![0u8; 10];
        let mut storer = LogEventStorerUnsafe::new(&mut buf);
        storer.store_bytes(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(storer.bytes_written(), 10);
        assert_eq!(
            buf,
            [0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        );
    }

    #[test]
    fn test_unsafe_storer_as_slice() {
        let mut buf = vec![0u8; 16];
        let mut storer = LogEventStorerUnsafe::new(&mut buf);
        storer.store_i32(42);
        storer.store_i64(123);
        assert_eq!(storer.as_slice().len(), 12);
    }

    #[test]
    fn test_unsafe_storer_remaining() {
        let mut buf = vec![0u8; 16];
        let mut storer = LogEventStorerUnsafe::new(&mut buf);
        assert_eq!(storer.remaining(), 16);
        storer.store_i32(42);
        assert_eq!(storer.remaining(), 12);
    }

    #[test]
    fn test_vec_storer() {
        let mut storer = LogEventStorerVec::new();
        storer.store_i32(42);
        storer.store_i64(123);
        // i32(42) in big-endian = [0, 0, 0, 42], i64(123) = [0, 0, 0, 0, 0, 0, 0, 123]
        assert_eq!(storer.as_slice(), [0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 123]);
    }

    #[test]
    fn test_vec_storer_with_capacity() {
        let storer = LogEventStorerVec::with_capacity(100);
        assert_eq!(storer.as_slice().len(), 0);
    }

    #[test]
    fn test_vec_storer_into_inner() {
        let mut storer = LogEventStorerVec::new();
        storer.store_i32(42);
        let data = storer.into_inner();
        // i32(42) in big-endian = [0, 0, 0, 42]
        assert_eq!(data, [0, 0, 0, 42]);
    }

    #[test]
    fn test_vec_storer_clear() {
        let mut storer = LogEventStorerVec::new();
        storer.store_i32(42);
        storer.clear();
        assert_eq!(storer.as_slice().len(), 0);
    }

    #[test]
    fn test_storer_bool() {
        let mut storer = LogEventStorerVec::new();
        storer.store_bool(true);
        storer.store_bool(false);
        assert_eq!(storer.as_slice(), [0, 0, 0, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_storer_is_empty() {
        let mut storer = LogEventStorerVec::new();
        assert!(storer.is_empty());
        storer.store_i32(42);
        assert!(!storer.is_empty());
    }
}
