// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! TL (Type Language) serialization for TempPasswordState.
//!
//! Implements TDLib-compatible binlog serialization format.

use crate::{Result, TempPasswordError, TempPasswordState};
use rustgram_logevent::{LogEventParser, LogEventStorerVec, TlParser, TlStorer};

impl TempPasswordState {
    /// Serializes the temp password state to bytes (TDLib binlog format).
    ///
    /// # Format
    ///
    /// - temp_password: String (TL string format)
    /// - valid_until: i32 (big endian)
    ///
    /// # Errors
    ///
    /// Returns error if no temp password is set.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    ///
    /// let state = TempPasswordState::new("secret123", 1735795200);
    /// let bytes = state.store().unwrap();
    /// assert!(!bytes.is_empty());
    /// ```
    pub fn store(&self) -> Result<Vec<u8>> {
        if !self.has_temp_password() {
            return Err(TempPasswordError::NotSet);
        }

        let mut storer = LogEventStorerVec::new();
        storer.store_bytes(self.temp_password().as_bytes());
        storer.store_i32(self.valid_until());

        Ok(storer.into_inner())
    }

    /// Deserializes the temp password state from bytes (TDLib binlog format).
    ///
    /// # Errors
    ///
    /// Returns error if data is malformed.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_temp_password_state::TempPasswordState;
    ///
    /// let state = TempPasswordState::new("secret123", 1735795200);
    /// let bytes = state.store().unwrap();
    ///
    /// let parsed = TempPasswordState::parse(&bytes).unwrap();
    /// assert_eq!(parsed.temp_password(), "secret123");
    /// assert_eq!(parsed.valid_until(), 1735795200);
    /// ```
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut parser = LogEventParser::new(data);

        let temp_password_bytes = parser
            .fetch_bytes()
            .map_err(|e| TempPasswordError::SerializationError(e.to_string()))?;

        let temp_password = String::from_utf8(temp_password_bytes)
            .map_err(|e| TempPasswordError::SerializationError(format!("Invalid UTF-8: {}", e)))?;

        let valid_until = parser
            .fetch_i32()
            .map_err(|e| TempPasswordError::SerializationError(e.to_string()))?;

        parser
            .fetch_end()
            .map_err(|e| TempPasswordError::SerializationError(e.to_string()))?;

        Ok(Self::new(temp_password, valid_until))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_parse() {
        let original = TempPasswordState::new("test_password", 1735795200);
        let bytes = original.store().unwrap();
        let parsed = TempPasswordState::parse(&bytes).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_store_parse_empty_password() {
        let original = TempPasswordState::new("", 1735795200);
        let bytes = original.store().unwrap();
        let parsed = TempPasswordState::parse(&bytes).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(parsed.temp_password(), "");
    }

    #[test]
    fn test_store_parse_unicode_password() {
        let original = TempPasswordState::new("secret-пароль-123", 1735795200);
        let bytes = original.store().unwrap();
        let parsed = TempPasswordState::parse(&bytes).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_store_without_password() {
        let state = TempPasswordState::default();
        assert!(matches!(state.store(), Err(TempPasswordError::NotSet)));
    }

    #[test]
    fn test_parse_empty() {
        let result = TempPasswordState::parse(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_truncated() {
        let state = TempPasswordState::new("secret", 1735795200);
        let mut bytes = state.store().unwrap();

        // Truncate the data
        bytes.truncate(bytes.len() - 2);

        let result = TempPasswordState::parse(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_extra_data() {
        let state = TempPasswordState::new("secret", 1735795200);
        let mut bytes = state.store().unwrap();

        // Add extra data
        bytes.push(0xFF);

        let result = TempPasswordState::parse(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_store_format() {
        let state = TempPasswordState::new("secret", 1735795200);
        let bytes = state.store().unwrap();

        // Format: [length: i32][password bytes][valid_until: i32]
        // "secret" = 6 bytes, so length should be 6
        assert!(bytes.len() >= 10); // At least 4 (length) + 6 (data)
    }

    #[test]
    fn test_parse_invalid_utf8() {
        // Create invalid UTF-8 sequence
        let mut storer = LogEventStorerVec::new();
        storer.store_i32(3); // length
        storer.store_bytes(&[0xFF, 0xFE, 0xFD]); // invalid UTF-8
        storer.store_i32(1735795200);

        let bytes = storer.into_inner();

        let result = TempPasswordState::parse(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip_multiple_times() {
        let original = TempPasswordState::new("test123", 1735795200);

        let mut current = original.clone();
        for _ in 0..10 {
            let bytes = current.store().unwrap();
            current = TempPasswordState::parse(&bytes).unwrap();
        }

        assert_eq!(original, current);
    }

    #[test]
    fn test_store_negative_timestamp() {
        let state = TempPasswordState::new("secret", -1);
        let bytes = state.store().unwrap();
        let parsed = TempPasswordState::parse(&bytes).unwrap();

        assert_eq!(parsed.valid_until(), -1);
    }
}
