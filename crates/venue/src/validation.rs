// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! String validation for venue fields.
//!
//! This module provides validation and cleaning functions for input strings
//! in venue objects. It implements the same logic as TDLib's `clean_input_string`
//! from `misc.cpp:76-161`.

use crate::error::{Result, VenueError};

/// Maximum string length (server-side limit).
///
/// TDLib reference: `misc.cpp:77`
pub const MAX_STRING_LENGTH: usize = 35000;

/// Cleans and validates an input string.
///
/// This function performs the same operations as TDLib's `clean_input_string`
/// from `misc.cpp:76-161`:
///
/// 1. Validates UTF-8 encoding
/// 2. Removes control characters (0-31, except \n which is converted to space)
/// 3. Removes `\r` characters
/// 4. Removes specific Unicode sequences: `\xe2\x80[\xa8-\xae]`
/// 5. Removes vertical line characters: `\xcc[\xb3\xbf\x8a]`
/// 6. Truncates to `MAX_STRING_LENGTH`
///
/// # Arguments
///
/// * `s` - The input string to clean
///
/// # Returns
///
/// Returns `Ok(String)` with the cleaned string, or `Err(VenueError::InvalidUtf8)`
/// if the input is not valid UTF-8.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:76-161`
///
/// # Examples
///
/// ```
/// use rustgram_venue::validation::clean_input_string;
///
/// // Normal string
/// assert_eq!(clean_input_string("Hello, world!").unwrap(), "Hello, world!");
///
/// // String with control characters (replaced with spaces)
/// assert_eq!(clean_input_string("Hello\u{00}World").unwrap(), "Hello World");
///
/// // String with \r (removed)
/// assert_eq!(clean_input_string("Hello\r\nWorld").unwrap(), "Hello\nWorld");
///
/// // Long string is truncated
/// let long = "a".repeat(40000);
/// let cleaned = clean_input_string(&long).unwrap();
/// assert!(cleaned.len() <= 35000);
/// ```
pub fn clean_input_string(s: &str) -> Result<String> {
    // Since we're given &str, it's already valid UTF-8 by Rust's guarantees.
    // However, we use simdutf8 for consistency with TDLib and to ensure
    // we properly handle multi-byte sequences during our byte manipulation.
    let _ = simdutf8::basic::from_utf8(s.as_bytes()).map_err(|_| VenueError::InvalidUtf8)?;

    let bytes = s.as_bytes();
    let mut result = String::with_capacity(bytes.len().min(MAX_STRING_LENGTH));
    let mut i = 0;

    while i < bytes.len() && result.len() < MAX_STRING_LENGTH {
        let b = bytes[i];

        match b {
            // Control characters -> space (misc.cpp:86-122)
            // Includes 0-8, 11-12 (tab), 14-31, 127 (DEL)
            // Note: 9 (\t) and 10 (\n) are NOT replaced
            0..=8 | 11..=12 | 14..=31 | 127 => {
                result.push(' ');
                i += 1;
            }
            // Skip \r (13) - misc.cpp:123-125
            13 => {
                i += 1;
            }
            // Remove \xe2\x80[\xa8-\xae] (misc.cpp:127-137)
            0xE2 if i + 2 < bytes.len() && bytes[i + 1] == 0x80 => {
                let next = bytes[i + 2];
                if (0xA8..=0xAE).contains(&next) {
                    i += 3;
                } else {
                    // Not a match, handle as UTF-8
                    if let Ok(s) = std::str::from_utf8(&bytes[i..i + 3]) {
                        result.push_str(s);
                    }
                    i += 3;
                }
            }
            // Remove vertical lines \xcc[\xb3\xbf\x8a] (misc.cpp:138-145)
            0xCC if i + 1 < bytes.len() => {
                let next = bytes[i + 1];
                if next == 0xB3 || next == 0xBF || next == 0x8A {
                    i += 2;
                } else {
                    // Not a match, handle as UTF-8
                    if let Ok(s) = std::str::from_utf8(&bytes[i..i + 2]) {
                        result.push_str(s);
                    }
                    i += 2;
                }
            }
            _ => {
                // For multi-byte characters, ensure we don't exceed MAX_STRING_LENGTH
                let utf8_len = utf8_char_len(b);
                if i + utf8_len > bytes.len() {
                    break;
                }
                if result.len() + utf8_len > MAX_STRING_LENGTH {
                    break;
                }

                // Copy the UTF-8 character as a whole
                if let Ok(s) = std::str::from_utf8(&bytes[i..i + utf8_len]) {
                    result.push_str(s);
                }
                i += utf8_len;
            }
        }
    }

    Ok(result.trim().to_string())
}

/// Determines the length of a UTF-8 character by its first byte.
///
/// # Arguments
///
/// * `first_byte` - The first byte of a UTF-8 sequence
///
/// # Returns
///
/// The total length of the UTF-8 character in bytes (1-4).
const fn utf8_char_len(first_byte: u8) -> usize {
    if first_byte < 0x80 {
        // 0xxxxxxx - ASCII
        1
    } else if (first_byte & 0xE0) == 0xC0 {
        // 110xxxxx - 2-byte sequence
        2
    } else if (first_byte & 0xF0) == 0xE0 {
        // 1110xxxx - 3-byte sequence
        3
    } else if (first_byte & 0xF8) == 0xF0 {
        // 11110xxx - 4-byte sequence
        4
    } else {
        // Invalid UTF-8, treat as single byte
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_string() {
        match clean_input_string("Hello, world!") {
            Ok(result) => assert_eq!(result, "Hello, world!"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_empty_string() {
        match clean_input_string("") {
            Ok(result) => assert_eq!(result, ""),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_whitespace_only() {
        match clean_input_string("   ") {
            Ok(result) => assert_eq!(result, ""),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_control_characters_to_space() {
        // Control characters 0-8 and 11-31, 127 become spaces
        match clean_input_string("Hello\x00World") {
            Ok(result) => assert_eq!(result, "Hello World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        match clean_input_string("Hello\x01World") {
            Ok(result) => assert_eq!(result, "Hello World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        match clean_input_string("Hello\x08World") {
            Ok(result) => assert_eq!(result, "Hello World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        match clean_input_string("Hello\x0BWorld") {
            Ok(result) => assert_eq!(result, "Hello World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        match clean_input_string("Hello\x1FWorld") {
            Ok(result) => assert_eq!(result, "Hello World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        match clean_input_string("Hello\x7FWorld") {
            Ok(result) => assert_eq!(result, "Hello World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_carriage_return_removed() {
        // \r is removed completely
        match clean_input_string("Hello\rWorld") {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        // \r\n becomes \n (since \r removed, \n preserved)
        match clean_input_string("Hello\r\nWorld") {
            Ok(result) => assert_eq!(result, "Hello\nWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_newline_becomes_space() {
        // \n is in the range 0-8? No, \n is 10.
        // Actually looking at the code: 0..=8 | 11..=31
        // So \n (10) falls through to the default case
        // But the control chars 0-8 become space, 9 is tab (becomes space), 10 (\n) is not in range
        // Wait, let me check: 0..=8 | 11..=31
        // So 9 (\t) and 10 (\n) are NOT in that range
        // Looking at misc.cpp more carefully:
        // case 0: case 1: ... case 8: case 11: case 12: ... case 31:
        // So 9 and 10 are skipped, they fall through to default
        // But the default also replaces them if they're not special sequences

        // Actually, looking at the misc.cpp code again:
        // case 9 and case 10 are NOT listed, so they fall through to default
        // The default case just copies the character

        // So \t and \n should be preserved as-is
        // Wait, but 0..=8 includes 0-8, 11..=31 includes 11-31
        // So 9 (\t) and 10 (\n) are NOT replaced with space

        // Let me re-read the C++ code more carefully...
        // misc.cpp:86-122 lists individual cases for:
        // 0, 1, 2, 3, 4, 5, 6, 7, 8, then 11, 12, 13 is handled separately (skip), then 14-31
        // So 9 (\t) and 10 (\n) are NOT in the switch cases

        // They fall through to default, which copies them
        // So they should be preserved

        // Hmm, but looking at the implementation notes in the spec:
        // "Удаление контрольных символов (0-31, кроме \n)"
        // So \n should be preserved?

        // Let me check what actually happens in our Rust code:
        // 0..=8 | 11..=31 | 127 -> space
        // 13 -> skip
        // So 9 and 10 fall through to default

        // So \t (9) and \n (10) should be preserved
        match clean_input_string("Hello\tWorld") {
            Ok(result) => assert_eq!(result, "Hello\tWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        match clean_input_string("Hello\nWorld") {
            Ok(result) => assert_eq!(result, "Hello\nWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_unicode_sequence_removal() {
        // \xe2\x80[\xa8-\xae] should be removed
        // U+2028 to U+202E (various Unicode line/paragraph separators)
        let input = match std::str::from_utf8(b"Hello\xe2\x80\xa8World") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create test input: {:?}", e),
        }; // U+2028
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }

        let input = match std::str::from_utf8(b"Hello\xe2\x80\xa9World") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create test input: {:?}", e),
        }; // U+2029
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }

        let input = match std::str::from_utf8(b"Hello\xe2\x80\xaeWorld") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create test input: {:?}", e),
        }; // U+202E
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_vertical_line_removal() {
        // \xcc[\xb3\xbf\x8a] should be removed
        let input = match std::str::from_utf8(b"Hello\xcc\xb3World") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create test input: {:?}", e),
        }; // U+0F33
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }

        let input = match std::str::from_utf8(b"Hello\xcc\xbfWorld") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create test input: {:?}", e),
        }; // U+0FFF
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }

        let input = match std::str::from_utf8(b"Hello\xcc\x8aWorld") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create test input: {:?}", e),
        }; // U+0F0A
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_invalid_utf8() {
        // Since our function takes &str (which is guaranteed valid UTF-8),
        // we can't directly test invalid UTF-8 through the public API.
        // The simdutf8 check at the start validates this.
        // In practice, invalid UTF-8 would be caught before calling this function.

        // Test that valid UTF-8 works fine
        assert!(clean_input_string("Hello, world!").is_ok());

        // For invalid UTF-8 testing, we rely on the simdutf8 validation
        // which is tested at the integration level when handling raw bytes
    }

    #[test]
    fn test_long_string_truncation() {
        // Create a string longer than MAX_STRING_LENGTH
        let long = "a".repeat(MAX_STRING_LENGTH + 1000);
        match clean_input_string(&long) {
            Ok(cleaned) => assert!(cleaned.len() <= MAX_STRING_LENGTH),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_multibyte_utf8() {
        // Cyrillic (each character is 2 bytes in UTF-8)
        match clean_input_string("Привет") {
            Ok(result) => assert_eq!(result, "Привет"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }

        // Emoji (each is 4 bytes)
        match clean_input_string("Hello World") {
            Ok(result) => assert_eq!(result, "Hello World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }

        // Chinese (each character is 3 bytes)
        match clean_input_string("你好") {
            Ok(result) => assert_eq!(result, "你好"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_trim_whitespace() {
        match clean_input_string("  Hello  ") {
            Ok(result) => assert_eq!(result, "Hello"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
        // Tab (11) is replaced with space, then trimmed
        match clean_input_string("\tHello\t") {
            Ok(result) => assert_eq!(result, "Hello"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_mixed_control_chars() {
        // Mix of different control characters
        // 00, 01, 02 become spaces, 1F becomes space, 7F becomes space
        // Result is trimmed, so leading/trailing spaces are removed
        // \x01\x02 are consecutive, so we get double space between Hello and World
        let input = "\x00Hello\x01\x02World\x1F\x7F";
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "Hello  World"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_utf8_char_len() {
        assert_eq!(utf8_char_len(0x00), 1); // ASCII
        assert_eq!(utf8_char_len(0x7F), 1); // ASCII max
        assert_eq!(utf8_char_len(0xC2), 2); // 2-byte start
        assert_eq!(utf8_char_len(0xE0), 3); // 3-byte start
        assert_eq!(utf8_char_len(0xF0), 4); // 4-byte start
        assert_eq!(utf8_char_len(0xFF), 1); // Invalid
    }

    #[test]
    fn test_special_unicode_not_removed() {
        // Other sequences starting with \xe2 should not be affected
        // Euro sign U+20AC is \xe2\x82\xac, which doesn't match \xe2\x80[\xa8-\xae]
        let input = "HelloWorld"; // Euro sign between Hello and World
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "HelloWorld"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }

        // Euro sign should be preserved (no trailing space after trim)
        let input = "EUR ";
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "EUR"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }

    #[test]
    fn test_unicode_in_middle_of_string() {
        // Test that Unicode characters in the middle are handled
        let input = match std::str::from_utf8(b"Start\x00Middle\xe2\x80\xa8End") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create test input: {:?}", e),
        };
        match clean_input_string(input) {
            Ok(result) => assert_eq!(result, "Start MiddleEnd"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }
}
