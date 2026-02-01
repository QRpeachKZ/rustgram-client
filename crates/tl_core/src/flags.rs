// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Flag reading utilities for TL deserialization.
//!
//! TL types use a flags field to indicate which optional fields are present.
//! This module provides a utility for reading flag-based optional fields.

use crate::error::TlError;

/// Reader for TL flags bitmask.
///
/// In TL, optional fields are indicated by bits in a flags integer.
/// The `FlagReader` provides convenient methods for checking flags
/// and reading optional values based on flag bits.
///
/// # Example
///
/// ```no_run
/// use rustgram_tl_core::flags::FlagReader;
///
/// let flags = 0b00001011u32; // bits 0, 1, and 3 are set
/// let reader = FlagReader::new(flags);
///
/// assert!(reader.has(0));  // bit 0 is set
/// assert!(reader.has(1));  // bit 1 is set
/// assert!(!reader.has(2)); // bit 2 is not set
/// assert!(reader.has(3));  // bit 3 is set
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlagReader {
    /// The flags bitmask.
    flags: u32,
}

impl FlagReader {
    /// Creates a new flag reader from a flags bitmask.
    ///
    /// # Arguments
    ///
    /// * `flags` - The flags value from the TL binary format
    #[inline]
    pub const fn new(flags: u32) -> Self {
        Self { flags }
    }

    /// Returns the raw flags value.
    #[inline]
    pub const fn flags(&self) -> u32 {
        self.flags
    }

    /// Checks if bit `n` is set (0-indexed).
    ///
    /// # Arguments
    ///
    /// * `n` - The bit index to check (0-31)
    ///
    /// # Returns
    ///
    /// `true` if the bit is set, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_tl_core::flags::FlagReader;
    ///
    /// let reader = FlagReader::new(0b00000101u32); // bits 0 and 2 set
    /// assert!(reader.has(0));
    /// assert!(!reader.has(1));
    /// assert!(reader.has(2));
    /// ```
    #[inline]
    pub fn has(&self, n: u32) -> bool {
        self.flags & (1 << n) != 0
    }

    /// Reads an optional field based on a flag bit.
    ///
    /// If the flag bit is set, calls the reader function and returns `Some(value)`.
    /// If the flag bit is not set, returns `None` without calling the reader.
    ///
    /// # Arguments
    ///
    /// * `bit` - The flag bit to check
    /// * `reader` - Function to read the value if the flag is set
    ///
    /// # Returns
    ///
    /// `Some(value)` if the flag is set and reading succeeds, `None` if the flag is not set.
    ///
    /// # Errors
    ///
    /// Returns an error if the flag is set and the reader function fails.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_tl_core::flags::FlagReader;
    ///
    /// let flags = 0b00000001u32; // bit 0 is set
    /// let reader = FlagReader::new(flags);
    ///
    /// // When bit 0 is set, the closure runs and returns Some(value)
    /// let result = reader.read_optional(0, || Ok::<i32, rustgram_types::TypeError>(42));
    /// assert_eq!(result.unwrap(), Some(42));
    ///
    /// // When bit 1 is NOT set, returns None without calling the closure
    /// let result = reader.read_optional(1, || Ok::<i32, rustgram_types::TypeError>(99));
    /// assert_eq!(result.unwrap(), None);
    /// ```
    pub fn read_optional<T, F>(&self, bit: u32, reader: F) -> rustgram_types::TypeResult<Option<T>>
    where
        F: FnOnce() -> rustgram_types::TypeResult<T>,
    {
        if self.has(bit) {
            match reader() {
                Ok(value) => Ok(Some(value)),
                Err(err) => {
                    let tl_err = TlError::FlagFieldError {
                        flag_index: bit,
                        field_name: "unknown".to_string(),
                        cause: Box::new(TlError::from(err)),
                    };
                    Err(rustgram_types::TypeError::from(tl_err))
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Reads a boolean flag value directly.
    ///
    /// This is a convenience method for flags that represent boolean values.
    ///
    /// # Arguments
    ///
    /// * `bit` - The flag bit to read
    ///
    /// # Returns
    ///
    /// `true` if the bit is set, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_tl_core::flags::FlagReader;
    ///
    /// let flags = 0b00001000u32; // bit 3 set
    /// let reader = FlagReader::new(flags);
    ///
    /// assert_eq!(reader.read_bool(3), true);
    /// assert_eq!(reader.read_bool(0), false);
    /// ```
    #[inline]
    pub fn read_bool(&self, bit: u32) -> bool {
        self.has(bit)
    }

    /// Reads an optional field with a named error context.
    ///
    /// Similar to `read_optional`, but includes the field name in error messages.
    ///
    /// # Arguments
    ///
    /// * `bit` - The flag bit to check
    /// * `field_name` - Name of the field for error reporting
    /// * `reader` - Function to read the value if the flag is set
    ///
    /// # Returns
    ///
    /// `Some(value)` if the flag is set and reading succeeds, `None` if the flag is not set.
    ///
    /// # Errors
    ///
    /// Returns an error if the flag is set and the reader function fails.
    pub fn read_optional_named<T, F>(
        &self,
        bit: u32,
        field_name: impl Into<String>,
        reader: F,
    ) -> rustgram_types::TypeResult<Option<T>>
    where
        F: FnOnce() -> rustgram_types::TypeResult<T>,
    {
        if self.has(bit) {
            match reader() {
                Ok(value) => Ok(Some(value)),
                Err(err) => {
                    let tl_err = TlError::FlagFieldError {
                        flag_index: bit,
                        field_name: field_name.into(),
                        cause: Box::new(TlError::from(err)),
                    };
                    Err(rustgram_types::TypeError::from(tl_err))
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Returns the number of bits set in the flags.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_tl_core::flags::FlagReader;
    ///
    /// let reader = FlagReader::new(0b00010101u32); // 3 bits set
    /// assert_eq!(reader.count(), 3);
    /// ```
    #[inline]
    pub fn count(&self) -> u32 {
        self.flags.count_ones()
    }

    /// Checks if any of the specified bits are set.
    ///
    /// # Arguments
    ///
    /// * `bits` - Slice of bit indices to check
    ///
    /// # Returns
    ///
    /// `true` if any of the specified bits are set, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_tl_core::flags::FlagReader;
    ///
    /// let reader = FlagReader::new(0b00001010u32); // bits 1 and 3 set
    /// assert!(reader.has_any(&[1, 2, 4])); // bit 1 is set
    /// assert!(!reader.has_any(&[0, 5, 6])); // none of these bits are set
    /// ```
    pub fn has_any(&self, bits: &[u32]) -> bool {
        bits.iter().any(|&bit| self.has(bit))
    }

    /// Checks if all of the specified bits are set.
    ///
    /// # Arguments
    ///
    /// * `bits` - Slice of bit indices to check
    ///
    /// # Returns
    ///
    /// `true` if all of the specified bits are set, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_tl_core::flags::FlagReader;
    ///
    /// let reader = FlagReader::new(0b00001010u32); // bits 1 and 3 set
    /// assert!(reader.has_all(&[1, 3])); // both bits are set
    /// assert!(!reader.has_all(&[1, 2])); // bit 2 is not set
    /// ```
    pub fn has_all(&self, bits: &[u32]) -> bool {
        bits.iter().all(|&bit| self.has(bit))
    }
}

impl From<u32> for FlagReader {
    fn from(flags: u32) -> Self {
        Self::new(flags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_reader_creation() {
        let reader = FlagReader::new(0x12345678);
        assert_eq!(reader.flags(), 0x12345678);
    }

    #[test]
    fn test_has_bit() {
        let reader = FlagReader::new(0b10101010u32);
        assert!(reader.has(1));
        assert!(reader.has(3));
        assert!(reader.has(5));
        assert!(reader.has(7));
        assert!(!reader.has(0));
        assert!(!reader.has(2));
        assert!(!reader.has(4));
        assert!(!reader.has(6));
    }

    #[test]
    fn test_read_bool() {
        let reader = FlagReader::new(0b00001000u32);
        assert_eq!(reader.read_bool(3), true);
        assert_eq!(reader.read_bool(0), false);
        assert_eq!(reader.read_bool(1), false);
        assert_eq!(reader.read_bool(2), false);
    }

    #[test]
    fn test_read_optional_some() {
        let reader = FlagReader::new(0b00000001u32);
        let result = reader.read_optional(0, || Ok(42));
        assert_eq!(result.unwrap(), Some(42));
    }

    #[test]
    fn test_read_optional_none() {
        let reader = FlagReader::new(0b00000000u32);
        let result = reader.read_optional(0, || Ok(42));
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_read_optional_error() {
        let reader = FlagReader::new(0b00000001u32);
        let result = reader.read_optional(0, || {
            Err::<i32, _>(rustgram_types::TypeError::DeserializationError(
                "test error".to_string(),
            ))
        });
        assert!(result.is_err());
        // Just check that we got an error, the wrapped TlError is internal
        assert!(result.unwrap_err().to_string().contains("test error"));
    }

    #[test]
    fn test_read_optional_named() {
        let reader = FlagReader::new(0b00000010u32);
        let result = reader.read_optional_named(1, "test_field", || Ok("value"));
        assert_eq!(result.unwrap(), Some("value"));

        let result = reader.read_optional_named(0, "missing_field", || Ok("value"));
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_count_bits() {
        let reader = FlagReader::new(0b00010101u32);
        assert_eq!(reader.count(), 3);

        let reader = FlagReader::new(0xFFFFFFFFu32);
        assert_eq!(reader.count(), 32);

        let reader = FlagReader::new(0u32);
        assert_eq!(reader.count(), 0);
    }

    #[test]
    fn test_has_any() {
        let reader = FlagReader::new(0b00001010u32);
        assert!(reader.has_any(&[1, 2, 4]));
        assert!(!reader.has_any(&[0, 5, 6]));
        assert!(!reader.has_any(&[])); // empty slice returns false (no bits to check)
    }

    #[test]
    fn test_has_all() {
        let reader = FlagReader::new(0b00001010u32);
        assert!(reader.has_all(&[1, 3]));
        assert!(!reader.has_all(&[1, 2]));
        assert!(reader.has_all(&[])); // empty slice returns true (vacuously true)
    }

    #[test]
    fn test_from_u32() {
        let reader: FlagReader = 0xABCD1234u32.into();
        assert_eq!(reader.flags(), 0xABCD1234);
    }

    #[test]
    fn test_high_bits() {
        let reader = FlagReader::new(0x80000000u32); // bit 31 set
        assert!(reader.has(31));
        assert!(!reader.has(30));
        assert_eq!(reader.count(), 1);
    }

    #[test]
    fn test_all_bits_zero() {
        let reader = FlagReader::new(0u32);
        for i in 0..32 {
            assert!(!reader.has(i));
        }
        assert_eq!(reader.count(), 0);
    }

    #[test]
    fn test_all_bits_one() {
        let reader = FlagReader::new(0xFFFFFFFFu32);
        for i in 0..32 {
            assert!(reader.has(i));
        }
        assert_eq!(reader.count(), 32);
    }

    #[test]
    fn test_copy_and_clone() {
        let reader1 = FlagReader::new(0x12345678);
        let reader2 = reader1;
        assert_eq!(reader1.flags(), reader2.flags());

        let reader3 = reader1.clone();
        assert_eq!(reader1.flags(), reader3.flags());
    }
}
