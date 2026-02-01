// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! String cleaning and manipulation functions.
//!
//! This module provides functions for cleaning and validating strings
//! according to TDLib's implementation in `misc.cpp`.

/// Unicode space character sequences (from misc.cpp:164-167).
///
/// These are various Unicode space characters that get replaced with regular spaces.
const SPACE_CHARS: [&[u8; 3]; 18] = [
    &[0xE1, 0x9A, 0x80], // U+1680 OGHAM SPACE MARK
    &[0xE2, 0x98, 0x8E], // U+180E MONGOLIAN VOWEL SEPARATOR
    &[0xE2, 0x80, 0x80], // U+2000 EN QUAD
    &[0xE2, 0x80, 0x81], // U+2001 EM QUAD
    &[0xE2, 0x80, 0x82], // U+2002 EN SPACE
    &[0xE2, 0x80, 0x83], // U+2003 EM SPACE
    &[0xE2, 0x80, 0x84], // U+2004 THREE-PER-EM SPACE
    &[0xE2, 0x80, 0x85], // U+2005 FOUR-PER-EM SPACE
    &[0xE2, 0x80, 0x86], // U+2006 SIX-PER-EM SPACE
    &[0xE2, 0x80, 0x87], // U+2007 FIGURE SPACE
    &[0xE2, 0x80, 0x88], // U+2008 PUNCTUATION SPACE
    &[0xE2, 0x80, 0x89], // U+2009 THIN SPACE
    &[0xE2, 0x80, 0x8A], // U+200A HAIR SPACE
    &[0xE2, 0x80, 0xAE], // U+202E RIGHT-TO-LEFT OVERRIDE
    &[0xE2, 0x80, 0xAF], // U+202F NARROW NO-BREAK SPACE
    &[0xE2, 0x81, 0x9F], // U+205F MEDIUM MATHEMATICAL SPACE
    &[0xE2, 0xA0, 0x80], // U+2800 BRAILLE PATTERN BLANK
    &[0xE3, 0x80, 0x80], // U+3000 IDEOGRAPHIC SPACE
];

/// Maximum length for cleaned strings.
#[allow(dead_code)]
const MAX_CLEANED_LENGTH: usize = 35000;

/// Cleans a name or dialog title.
///
/// This function:
/// 1. Calls `strip_empty_characters` with the max length
/// 2. Collapses consecutive whitespace into single spaces
/// 3. Replaces non-breaking spaces (U+00A0) with regular spaces
/// 4. Trims the result
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:24-50`
///
/// # Arguments
///
/// * `str` - The input string to clean
/// * `max_length` - Maximum length of the result
///
/// # Returns
///
/// The cleaned string.
///
/// # Examples
///
/// ```
/// use rustgram_misc::clean_name;
///
/// let result = clean_name("Hello\u{A0}  World", 100);
/// assert_eq!(result, "Hello World");
/// ```
pub fn clean_name(str: &str, max_length: usize) -> String {
    let result = strip_empty_characters(str, max_length, false);

    // Collapse consecutive whitespace
    let mut new_result = String::with_capacity(result.len());
    let mut is_previous_space = false;
    let bytes = result.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes[i];

        // Regular space or newline
        if byte == b' ' || byte == b'\n' {
            if !is_previous_space {
                new_result.push(' ');
                is_previous_space = true;
            }
            i += 1;
            continue;
        }

        // Non-breaking space U+00A0 = \xC2\xA0
        if byte == 0xC2 && i + 1 < bytes.len() && bytes[i + 1] == 0xA0 {
            if !is_previous_space {
                new_result.push(' ');
                is_previous_space = true;
            }
            i += 2;
            continue;
        }

        new_result.push(byte as char);
        is_previous_space = false;
        i += 1;
    }

    new_result.trim().to_string()
}

/// Prepares a username or sticker name for search.
///
/// This function:
/// 1. Removes all dots (`.`)
/// 2. Converts to lowercase
/// 3. Trims whitespace
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:52-56`
///
/// # Arguments
///
/// * `str` - The input string to clean
///
/// # Returns
///
/// The cleaned username string.
///
/// # Examples
///
/// ```
/// use rustgram_misc::clean_username;
///
/// assert_eq!(clean_username("Test.Username"), "testusername");
/// assert_eq!(clean_username("  UserName  "), "username");
/// ```
pub fn clean_username(str: &str) -> String {
    str.chars()
        .filter(|&c| c != '.')
        .collect::<String>()
        .to_lowercase()
        .trim()
        .to_string()
}

/// Prepares a phone number for search.
///
/// This function removes all non-digit characters from the phone number.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:58-60`
///
/// # Arguments
///
/// * `phone_number` - The phone number to clean
///
/// # Returns
///
/// The cleaned phone number containing only digits.
///
/// # Examples
///
/// ```
/// use rustgram_misc::clean_phone_number;
///
/// assert_eq!(clean_phone_number("+1 (555) 123-4567"), "15551234567");
/// assert_eq!(clean_phone_number("123-456-7890"), "1234567890");
/// ```
pub fn clean_phone_number(phone_number: &str) -> String {
    phone_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

/// Replaces offending characters without changing string length.
///
/// This function replaces invisible RTL (right-to-left) mark sequences
/// with zero-width non-joiners to prevent text spoofing attacks.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:62-74`
///
/// # Arguments
///
/// * `str` - The string to process
///
/// # Returns
///
/// A new string with offending character sequences replaced.
///
/// # Examples
///
/// ```
/// use rustgram_misc::replace_offending_characters;
///
/// let input = "test\u{200F}string"; // Contains RTL mark
/// let result = replace_offending_characters(input);
/// // The offending sequence is replaced with zero-width non-joiner
/// ```
pub fn replace_offending_characters(str: &str) -> String {
    let bytes = str.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let len = bytes.len();

    while i < len {
        // Look for \xe2\x80[\x8e|\x8f] sequences (RTL marks)
        if i + 2 < len
            && bytes[i] == 0xE2
            && bytes[i + 1] == 0x80
            && (bytes[i + 2] == 0x8E || bytes[i + 2] == 0x8F)
        {
            // Replace with zero-width non-joiner \xe2\x80\x8c
            result.extend_from_slice(&[0xE2, 0x80, 0x8C]);

            // Skip consecutive RTL marks and replace them
            let mut j = i + 3;
            while j + 2 < len
                && bytes[j] == 0xE2
                && bytes[j + 1] == 0x80
                && (bytes[j + 2] == 0x8E || bytes[j + 2] == 0x8F)
            {
                result.extend_from_slice(&[0xE2, 0x80, 0x8C]);
                j += 3;
            }

            i = j;
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }

    // SAFETY: We've only replaced byte sequences with other valid UTF-8 sequences
    // of the same length, so the string remains valid UTF-8
    unsafe { String::from_utf8_unchecked(result) }
}

/// Strips empty characters and truncates to max length.
///
/// This function:
/// 1. Replaces various Unicode space characters with regular spaces
/// 2. Truncates to max_length
/// 3. Checks if the result is "empty" (only contains invisible characters)
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:163-254`
///
/// # Arguments
///
/// * `str` - The input string
/// * `max_length` - Maximum length of the result
/// * `strip_rtlo` - Whether to strip right-to-left override character
///
/// # Returns
///
/// The stripped string, or empty string if the result contains only invisible characters.
///
/// # Examples
///
/// ```
/// use rustgram_misc::strip_empty_characters;
///
/// let result = strip_empty_characters("Test\u{1680}String", 100, false);
/// assert_eq!(result, "Test String");
/// ```
pub fn strip_empty_characters(str: &str, max_length: usize, strip_rtlo: bool) -> String {
    let bytes = str.as_bytes();
    let mut result = Vec::with_capacity(bytes.len().min(max_length));
    let mut i = 0;

    // First pass: replace space characters
    while i < bytes.len() && result.len() < max_length {
        let byte = bytes[i];

        // Check if this could be the start of a space character sequence
        let is_space_start = SPACE_CHARS.iter().any(|space| {
            byte == space[0]
                && i + 3 <= bytes.len()
                && bytes.get(i + 1) == Some(&space[1])
                && bytes.get(i + 2) == Some(&space[2])
        });

        if is_space_start {
            // Check for RTL override (U+202E = \xE2\x80\xAE)
            if byte == 0xE2 && i + 2 < bytes.len() && bytes[i + 1] == 0x80 && bytes[i + 2] == 0xAE {
                if strip_rtlo {
                    result.push(b' ');
                } else {
                    result.extend_from_slice(&[0xE2, 0x80, 0xAE]);
                }
            } else {
                result.push(b' ');
            }
            i += 3;
            continue;
        }

        // 4-byte UTF-8 sequences starting with 0xF3
        if byte == 0xF3 && i + 3 < bytes.len() {
            let b1 = bytes[i + 1];
            let b2 = bytes[i + 2];
            if b1 == 0xA0 && (b2 & 0xFE) == 0x80 {
                result.push(b' ');
                i += 4;
                continue;
            }
        }

        result.push(byte);
        i += 1;
    }

    // Truncate and trim
    let result_str = String::from_utf8_lossy(&result);
    let trimmed = result_str.trim();

    // Check if string is empty (only contains invisible characters)
    let empty_bytes = trimmed.as_bytes();
    let mut j = 0;

    while j < empty_bytes.len() {
        let byte = empty_bytes[j];

        if byte == b' ' || byte == b'\n' {
            j += 1;
            continue;
        }

        // Check for zero-width characters and other invisible chars
        if byte == 0xE2 && j + 2 < empty_bytes.len() && empty_bytes[j + 1] == 0x80 {
            let next = empty_bytes[j + 2];
            if (0x8B..=0x8F).contains(&next) || next == 0xAE {
                j += 3;
                continue;
            }
        }

        // Byte order mark U+FEFF = \xEF\xBB\xBF
        if byte == 0xEF
            && j + 2 < empty_bytes.len()
            && empty_bytes[j + 1] == 0xBB
            && empty_bytes[j + 2] == 0xBF
        {
            j += 3;
            continue;
        }

        // Non-breaking space U+00A0 = \xC2\xA0
        if byte == 0xC2 && j + 1 < empty_bytes.len() && empty_bytes[j + 1] == 0xA0 {
            j += 2;
            continue;
        }

        // Found a non-empty character
        break;
    }

    if j >= trimmed.len() {
        // All characters are invisible
        return String::new();
    }

    // Truncate to max_length
    if trimmed.len() > max_length {
        // Find valid UTF-8 boundary
        let mut end = max_length;
        while end > 0 && !utf8_is_first_byte(trimmed.as_bytes()[end]) {
            end -= 1;
        }
        trimmed[..end].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Checks if a byte is the first byte of a UTF-8 character.
fn utf8_is_first_byte(b: u8) -> bool {
    b < 0x80 || (b & 0xC0) != 0x80
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_name() {
        assert_eq!(clean_name("Hello World", 100), "Hello World");
        assert_eq!(clean_name("Hello\u{A0}World", 100), "Hello World");
        assert_eq!(clean_name("Hello  World", 100), "Hello World");
        assert_eq!(clean_name("  Hello  World  ", 100), "Hello World");
    }

    #[test]
    fn test_clean_username() {
        assert_eq!(clean_username("Test.Username"), "testusername");
        assert_eq!(clean_username("Simple"), "simple");
        assert_eq!(clean_username("  UserName  "), "username");
        assert_eq!(clean_username("many.dots.in.name"), "manydotsinname");
    }

    #[test]
    fn test_clean_phone_number() {
        assert_eq!(clean_phone_number("+1 (555) 123-4567"), "15551234567");
        assert_eq!(clean_phone_number("123-456-7890"), "1234567890");
        assert_eq!(clean_phone_number("+44 20 7123 4567"), "442071234567");
        assert_eq!(clean_phone_number(""), "");
    }

    #[test]
    fn test_replace_offending_characters() {
        // Test with RTL mark (U+200F = \xE2\x80\x8F)
        let input = "test\u{200F}string";
        let result = replace_offending_characters(input);
        // Should replace with zero-width non-joiner (U+200C = \xE2\x80\x8C)
        assert_eq!(result, "test\u{200C}string");
    }

    #[test]
    fn test_strip_empty_characters() {
        assert_eq!(
            strip_empty_characters("Hello World", 100, false),
            "Hello World"
        );
        assert_eq!(
            strip_empty_characters("Hello\u{1680}World", 100, false),
            "Hello World"
        );
        assert_eq!(strip_empty_characters("  Test  ", 100, false), "Test");
    }

    #[test]
    fn test_strip_empty_characters_truncation() {
        let long = "a".repeat(100);
        assert_eq!(strip_empty_characters(&long, 10, false).len(), 10);
    }

    #[test]
    fn test_utf8_is_first_byte() {
        assert!(utf8_is_first_byte(0x00)); // ASCII
        assert!(utf8_is_first_byte(0x7F)); // ASCII max
        assert!(utf8_is_first_byte(0xC2)); // 2-byte start
        assert!(utf8_is_first_byte(0xE0)); // 3-byte start
        assert!(utf8_is_first_byte(0xF0)); // 4-byte start
        assert!(!utf8_is_first_byte(0x80)); // Continuation byte
    }
}
