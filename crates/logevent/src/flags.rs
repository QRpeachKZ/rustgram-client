// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Flag handling for TL serialization
//!
//! Flags are stored as a u32 bitfield in TL format. The FlagsStorer and
//! FlagsParser handle storing and parsing these bitfields.

use crate::{LogEventError, Result};

/// Storer for building a flags bitfield
///
/// # Example
///
/// ```
/// use rustgram_logevent::FlagsStorer;
///
/// let mut storer = FlagsStorer::new();
/// storer.store_flag(true);
/// storer.store_flag(false);
/// storer.store_flag(true);
/// let flags = storer.finish(); // returns 0b101 = 5
/// ```
#[derive(Debug, Clone)]
pub struct FlagsStorer {
    flags: u32,
    bit_offset: u32,
}

impl FlagsStorer {
    /// Creates a new empty flags storer
    #[must_use]
    pub const fn new() -> Self {
        Self {
            flags: 0,
            bit_offset: 0,
        }
    }

    /// Stores a boolean flag at the current bit position
    pub fn store_flag(&mut self, flag: bool) {
        self.flags |= (flag as u32) << self.bit_offset;
        self.bit_offset += 1;
    }

    /// Returns the final flags value and resets the storer
    #[must_use]
    pub const fn finish(self) -> u32 {
        self.flags
    }

    /// Returns the current number of bits stored
    #[must_use]
    pub const fn bit_count(&self) -> u32 {
        self.bit_offset
    }
}

impl Default for FlagsStorer {
    fn default() -> Self {
        Self::new()
    }
}

/// Parser for reading a flags bitfield
///
/// # Example
///
/// ```
/// use rustgram_logevent::FlagsParser;
///
/// let flags = 0b101u32; // bit 0 = true, bit 1 = false, bit 2 = true
/// let mut parser = FlagsParser::new(flags);
/// assert_eq!(parser.parse_flag(), true);
/// assert_eq!(parser.parse_flag(), false);
/// assert_eq!(parser.parse_flag(), true);
/// parser.finish().unwrap(); // verify no extra bits
/// ```
#[derive(Debug, Clone)]
pub struct FlagsParser {
    flags: u32,
    bit_offset: u32,
}

impl FlagsParser {
    /// Creates a new flags parser from a u32 value
    #[must_use]
    pub const fn new(flags: u32) -> Self {
        Self {
            flags,
            bit_offset: 0,
        }
    }

    /// Parses a boolean flag from the current bit position
    pub fn parse_flag(&mut self) -> bool {
        let flag = ((self.flags >> self.bit_offset) & 1) != 0;
        self.bit_offset += 1;
        flag
    }

    /// Parses N flags as a bitmask and advances the bit offset
    pub fn parse_flags_mask(&mut self, count: u32) -> u32 {
        let mask = if self.bit_offset + count <= 32 {
            (self.flags >> self.bit_offset) & ((1 << count) - 1)
        } else {
            self.flags >> self.bit_offset
        };
        self.bit_offset += count;
        mask
    }

    /// Verifies that all bits have been consumed (no unknown flags)
    ///
    /// Returns an error if there are bits set beyond what was parsed.
    pub fn finish(self) -> Result<()> {
        let mask = (1u32 << self.bit_offset).saturating_sub(1);
        if (self.flags & !mask) != 0 {
            return Err(LogEventError::InvalidFlags);
        }
        Ok(())
    }

    /// Returns the current bit position
    #[must_use]
    pub const fn bit_offset(&self) -> u32 {
        self.bit_offset
    }

    /// Checks if a specific bit is set without advancing
    #[must_use]
    pub const fn has_flag(&self, bit: u32) -> bool {
        (self.flags & (1 << bit)) != 0
    }

    /// Peeks at the next flag without consuming it
    #[must_use]
    pub const fn peek_flag(&self) -> bool {
        (self.flags & (1 << self.bit_offset)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_storer_basic() {
        let mut storer = FlagsStorer::new();
        storer.store_flag(true);
        storer.store_flag(false);
        storer.store_flag(true);
        storer.store_flag(false);

        assert_eq!(storer.finish(), 0b0101);
    }

    #[test]
    fn test_flags_storer_all_true() {
        let mut storer = FlagsStorer::new();
        for _ in 0..8 {
            storer.store_flag(true);
        }
        assert_eq!(storer.finish(), 0xFF);
    }

    #[test]
    fn test_flags_storer_all_false() {
        let mut storer = FlagsStorer::new();
        for _ in 0..8 {
            storer.store_flag(false);
        }
        assert_eq!(storer.finish(), 0);
    }

    #[test]
    fn test_flags_parser_basic() {
        let flags = 0b0101u32;
        let mut parser = FlagsParser::new(flags);

        assert!(parser.parse_flag());
        assert!(!parser.parse_flag());
        assert!(parser.parse_flag());
        assert!(!parser.parse_flag());
        parser.finish().unwrap();
    }

    #[test]
    fn test_flags_parser_roundtrip() {
        let input = [true, false, true, true, false, true, false, false];

        let mut storer = FlagsStorer::new();
        for &flag in &input {
            storer.store_flag(flag);
        }
        let flags = storer.finish();

        let mut parser = FlagsParser::new(flags);
        let output: Vec<bool> = (0..input.len()).map(|_| parser.parse_flag()).collect();

        assert_eq!(input, output.as_slice());
        parser.finish().unwrap();
    }

    #[test]
    fn test_flags_parser_extra_bits() {
        let flags = 0b111u32; // 3 bits set
        let mut parser = FlagsParser::new(flags);
        parser.parse_flag(); // only parse 1 bit
        assert!(parser.finish().is_err());
    }

    #[test]
    fn test_flags_parser_mask() {
        let flags = 0b1011_0110u32;
        let mut parser = FlagsParser::new(flags);

        let mask = parser.parse_flags_mask(4);
        assert_eq!(mask, 0b0110); // lower 4 bits

        let mask2 = parser.parse_flags_mask(4);
        assert_eq!(mask2, 0b1011); // next 4 bits
    }

    #[test]
    fn test_flags_parser_has_flag() {
        // Binary: 1 0 1 0 0 0 0 0
        // Bit:    7 6 5 4 3 2 1 0
        let flags = 0b1010_0000u32;
        let parser = FlagsParser::new(flags);

        assert!(parser.has_flag(5)); // bit 5 is set
        assert!(parser.has_flag(7)); // bit 7 is set
        assert!(!parser.has_flag(0)); // bit 0 is not set
        assert!(!parser.has_flag(4)); // bit 4 is not set
    }

    #[test]
    fn test_flags_parser_peek() {
        let flags = 0b1010u32;
        let mut parser = FlagsParser::new(flags);

        assert!(!parser.peek_flag());
        assert!(!parser.peek_flag()); // peek doesn't advance
        assert!(!parser.parse_flag());
        assert!(parser.peek_flag());
    }

    #[test]
    fn test_flags_default() {
        let storer = FlagsStorer::default();
        assert_eq!(storer.finish(), 0);
    }
}
