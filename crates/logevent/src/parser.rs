// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! TL (Type Language) parser for log event data
//!
//! Provides parsing primitives for reading TL-encoded binary data.

use crate::{LogEventError, Result};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// TL parser trait for reading binary data
///
/// This trait provides methods to parse TL-encoded binary data.
/// TL is Telegram's custom binary serialization format.
pub trait TlParser: Sized {
    /// Fetches a 32-bit signed integer
    fn fetch_i32(&mut self) -> Result<i32>;

    /// Fetches a 32-bit unsigned integer
    fn fetch_u32(&mut self) -> Result<u32>;

    /// Fetches a 64-bit signed integer
    fn fetch_i64(&mut self) -> Result<i64>;

    /// Fetches a 64-bit unsigned integer
    fn fetch_u64(&mut self) -> Result<u64>;

    /// Fetches a boolean value (stored as i32 in TL)
    fn fetch_bool(&mut self) -> Result<bool> {
        let value = self.fetch_i32()?;
        Ok(value != 0)
    }

    /// Fetches a byte vector with length prefix
    fn fetch_bytes(&mut self) -> Result<Vec<u8>>;

    /// Fetches a slice reference to remaining data
    fn fetch_slice(&mut self) -> Result<&[u8]>;

    /// Verifies that all data has been consumed
    fn fetch_end(&mut self) -> Result<()>;

    /// Returns the current position in the data
    fn position(&self) -> usize;
}

/// TL parser implementation on byte slices
///
/// # Example
///
/// ```
/// use rustgram_logevent::{LogEventParser, TlParser};
///
/// let data = &[0x00, 0x00, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04];
/// let mut parser = LogEventParser::new(data);
/// let bytes = parser.fetch_bytes().unwrap();
/// assert_eq!(bytes, vec![1, 2, 3, 4]);
/// parser.fetch_end().unwrap();
/// ```
#[derive(Debug)]
pub struct LogEventParser<'a> {
    cursor: Cursor<&'a [u8]>,
    start_len: usize,
}

impl<'a> LogEventParser<'a> {
    /// Creates a new parser from a byte slice
    #[must_use]
    pub fn new(data: &'a [u8]) -> Self {
        let start_len = data.len();
        Self {
            cursor: Cursor::new(data),
            start_len,
        }
    }

    /// Returns the number of bytes remaining
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.cursor
            .get_ref()
            .len()
            .saturating_sub(self.cursor.position() as usize)
    }

    /// Returns the total length of the input data
    #[must_use]
    pub const fn total_len(&self) -> usize {
        self.start_len
    }
}

impl<'a> TlParser for LogEventParser<'a> {
    fn fetch_i32(&mut self) -> Result<i32> {
        self.cursor
            .read_i32::<BigEndian>()
            .map_err(LogEventError::from)
    }

    fn fetch_u32(&mut self) -> Result<u32> {
        self.cursor
            .read_u32::<BigEndian>()
            .map_err(LogEventError::from)
    }

    fn fetch_i64(&mut self) -> Result<i64> {
        self.cursor
            .read_i64::<BigEndian>()
            .map_err(LogEventError::from)
    }

    fn fetch_u64(&mut self) -> Result<u64> {
        self.cursor
            .read_u64::<BigEndian>()
            .map_err(LogEventError::from)
    }

    fn fetch_bytes(&mut self) -> Result<Vec<u8>> {
        // Check for negative length BEFORE casting to usize
        let len_i32 = self.fetch_i32()?;
        if len_i32 < 0 {
            return Err(LogEventError::ParseError(format!(
                "Invalid byte length: {}",
                len_i32
            )));
        }
        let len = len_i32 as usize;
        if len > self.remaining() {
            return Err(LogEventError::UnexpectedEnd);
        }

        let mut bytes = vec![0u8; len];
        self.cursor
            .read_exact(&mut bytes)
            .map_err(LogEventError::from)?;
        Ok(bytes)
    }

    fn fetch_slice(&mut self) -> Result<&[u8]> {
        // Check for negative length BEFORE casting to usize
        let len_i32 = self.fetch_i32()?;
        if len_i32 < 0 {
            return Err(LogEventError::ParseError(format!(
                "Invalid slice length: {}",
                len_i32
            )));
        }
        let len = len_i32 as usize;
        if len > self.remaining() {
            return Err(LogEventError::UnexpectedEnd);
        }

        let pos = self.cursor.position() as usize;
        let data = self.cursor.get_ref();
        let slice = &data[pos..pos + len];
        self.cursor.set_position((pos + len) as u64);
        Ok(slice)
    }

    fn fetch_end(&mut self) -> Result<()> {
        if self.remaining() != 0 {
            return Err(LogEventError::ParseError(format!(
                "{} bytes remaining",
                self.remaining()
            )));
        }
        Ok(())
    }

    fn position(&self) -> usize {
        self.cursor.position() as usize
    }
}

/// Helper trait for parsing TL objects
///
/// This trait is reserved for future use when implementing automatic TL
/// parsing for complex types. Currently unused but part of the public API.
#[allow(dead_code)]
pub trait ParseTl: Sized {
    /// Parse self from a TL parser
    fn parse<P: TlParser>(parser: &mut P) -> Result<Self>;
}

/// Parse helper macro for implementing ParseTl
#[macro_export]
macro_rules! impl_parse_tl {
    ($ty:ty) => {
        impl ParseTl for $ty {
            fn parse<P: TlParser>(parser: &mut P) -> Result<Self> {
                Self::parse(parser)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_i32() {
        let data = [0x00, 0x00, 0x00, 0x42];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_i32().unwrap(), 0x42);
        parser.fetch_end().unwrap();
    }

    #[test]
    fn test_parser_i32_negative() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_i32().unwrap(), -1);
    }

    #[test]
    fn test_parser_u32() {
        let data = [0x00, 0x00, 0x01, 0x00];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_u32().unwrap(), 0x100);
    }

    #[test]
    fn test_parser_i64() {
        let data = [0x00, 0x00, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_i64().unwrap(), 0x0123_4567);
    }

    #[test]
    fn test_parser_bool() {
        let data = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_bool().unwrap(), true);
        assert_eq!(parser.fetch_bool().unwrap(), false);
    }

    #[test]
    fn test_parser_bytes() {
        let data = [
            0x00, 0x00, 0x00, 0x03, // length
            0x01, 0x02, 0x03, // data
        ];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_bytes().unwrap(), vec![0x01, 0x02, 0x03]);
        parser.fetch_end().unwrap();
    }

    #[test]
    fn test_parser_bytes_empty() {
        let data = [0x00, 0x00, 0x00, 0x00];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_bytes().unwrap(), Vec::<u8>::new());
        parser.fetch_end().unwrap();
    }

    #[test]
    fn test_parser_slice() {
        let data = [
            0x00, 0x00, 0x00, 0x03, // length
            0x01, 0x02, 0x03, // data
        ];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.fetch_slice().unwrap(), &[0x01, 0x02, 0x03]);
        parser.fetch_end().unwrap();
    }

    #[test]
    fn test_parser_unexpected_end() {
        let data = [0x00, 0x00, 0x00, 0x05, 0x01, 0x02]; // length 5 but only 2 bytes
        let mut parser = LogEventParser::new(&data);
        assert!(matches!(
            parser.fetch_bytes(),
            Err(LogEventError::UnexpectedEnd)
        ));
    }

    #[test]
    fn test_parser_not_consumed() {
        let data = [0x00, 0x00, 0x00, 0x01, 0x00];
        let mut parser = LogEventParser::new(&data);
        parser.fetch_i32().unwrap();
        assert!(matches!(
            parser.fetch_end(),
            Err(LogEventError::ParseError(_))
        ));
    }

    #[test]
    fn test_parser_position() {
        let data = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.position(), 0);
        parser.fetch_i32().unwrap();
        assert_eq!(parser.position(), 4);
        parser.fetch_i32().unwrap();
        assert_eq!(parser.position(), 8);
    }

    #[test]
    fn test_parser_remaining() {
        let data = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let mut parser = LogEventParser::new(&data);
        assert_eq!(parser.remaining(), 8);
        parser.fetch_i32().unwrap();
        assert_eq!(parser.remaining(), 4);
    }
}
