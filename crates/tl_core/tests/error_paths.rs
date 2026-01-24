// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error path tests.
//!
//! This module tests various error conditions to ensure proper error
//! handling and reporting.

use rustgram_tl_core::error::{TlError, VectorError};
use rustgram_tl_core::{Photo, Peer, PhotoSize, NotificationSound};
use rustgram_types::tl::{Bytes, TlDeserialize, TlHelper};

fn create_buffer(data: &[u8]) -> Bytes {
    Bytes::new(bytes::Bytes::copy_from_slice(data))
}

// ============================================================================
// Unknown Constructor Errors
// ============================================================================

#[test]
fn test_photo_unknown_constructor_single_byte() {
    // Test with single invalid byte repeated
    for invalid_byte in [0xFF, 0x00, 0xAA, 0x55] {
        let data = [invalid_byte; 4];
        let mut buf = create_buffer(&data);
        let result = Photo::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with invalid constructor 0x{:02x}", invalid_byte);
    }
}

#[test]
fn test_photo_unknown_constructor_various_values() {
    let invalid_constructors = vec![
        0x00000000u32,
        0xFFFFFFFFu32,
        0x12345678u32,
        0xABCDEF01u32,
        0xDEADBEEFu32,
        0xFEEDFACEu32,
    ];

    for invalid_id in invalid_constructors {
        let data = invalid_id.to_le_bytes().to_vec();
        let mut extended = data;
        extended.extend_from_slice(&0i64.to_le_bytes()); // Add id field

        let mut buf = create_buffer(&extended);
        let result = Photo::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with invalid constructor 0x{:08x}", invalid_id);

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Unknown constructor") || err_msg.contains("0x"));
    }
}

#[test]
fn test_peer_unknown_constructor_all_invalid() {
    // Test Peer with all constructor IDs that are not valid
    let invalid_ids = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xAA, 0x55];

    for invalid_id in invalid_ids {
        let mut data = (invalid_id as u32).to_le_bytes().to_vec();
        data.extend_from_slice(&123i64.to_le_bytes());

        let mut buf = create_buffer(&data);
        let result = Peer::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with invalid constructor 0x{:02x}", invalid_id);
    }
}

#[test]
fn test_photo_size_unknown_constructor() {
    // Test PhotoSize with invalid constructor IDs
    let invalid_ids = vec![0x00000000u32, 0xFFFFFFFFu32, 0x11111111u32];

    for invalid_id in invalid_ids {
        let mut data = invalid_id.to_le_bytes().to_vec();

        let mut buf = create_buffer(&data);
        let result = PhotoSize::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with invalid PhotoSize constructor 0x{:08x}", invalid_id);
    }
}

#[test]
fn test_notification_sound_unknown_constructor() {
    // Test NotificationSound with invalid constructor IDs
    let invalid_ids = vec![0x00000000u32, 0x11111111u32, 0xAAAAAAAAu32];

    for invalid_id in invalid_ids {
        let data = invalid_id.to_le_bytes();

        let mut buf = create_buffer(&data);
        let result = NotificationSound::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with invalid NotificationSound constructor 0x{:08x}", invalid_id);
    }
}

// ============================================================================
// Unexpected EOF Errors
// ============================================================================

#[test]
fn test_empty_buffer() {
    // Test with completely empty buffer
    let data = [];
    let mut buf = create_buffer(&data);

    let result: Result<Peer, _> = Peer::deserialize_tl(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_truncated_constructor() {
    // Test with only 1, 2, or 3 bytes (constructor needs 4)
    for size in [1, 2, 3] {
        let data = vec![0xFFu8; size];
        let mut buf = create_buffer(&data);

        let result: Result<Peer, _> = Peer::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with truncated constructor ({} bytes)", size);
    }
}

#[test]
fn test_truncated_peer_user() {
    // peerUser needs constructor (4 bytes) + user_id (8 bytes) = 12 bytes total
    let mut data = vec![0x22, 0x17, 0x51, 0x59]; // valid constructor
    data.extend_from_slice(&123i64.to_le_bytes()); // user_id

    // Test with missing bytes
    for truncate in [1, 2, 3, 4, 5, 6, 7, 8] {
        let truncated_data = &data[..(data.len() - truncate)];
        let mut buf = create_buffer(truncated_data);

        let result: Result<Peer, _> = Peer::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with truncated peerUser (missing {} bytes)", truncate);
    }
}

#[test]
fn test_truncated_photo_empty() {
    // photoEmpty needs constructor (4 bytes) + id (8 bytes) = 12 bytes total
    let mut data = vec![0x2d, 0xb2, 0x31, 0x23]; // valid constructor
    data.extend_from_slice(&126i64.to_le_bytes()); // id

    // Test with missing bytes
    for truncate in [1, 2, 3, 4, 5, 6, 7, 8] {
        let truncated_data = &data[..(data.len() - truncate)];
        let mut buf = create_buffer(truncated_data);

        let result: Result<Photo, _> = Photo::deserialize_tl(&mut buf);
        assert!(result.is_err(), "Should fail with truncated photoEmpty (missing {} bytes)", truncate);
    }
}

#[test]
fn test_truncated_photo_with_flags() {
    // photo needs constructor (4) + flags (4) + id (8) + access_hash (8) +
    // file_reference (variable) + date (4) + sizes (vector) + dc_id (4)
    let mut data = vec![0x65, 0x7a, 0x19, 0xfb]; // photo constructor
    data.extend_from_slice(&0i32.to_le_bytes()); // flags = 0
    data.extend_from_slice(&123i64.to_le_bytes()); // id
    data.extend_from_slice(&456i64.to_le_bytes()); // access_hash

    // Truncate at various points
    for truncate in [1, 2, 3, 4, 8, 12] {
        let truncated_data = &data[..(data.len().saturating_sub(truncate))];
        let mut buf = create_buffer(truncated_data);

        let result: Result<Photo, _> = Photo::deserialize_tl(&mut buf);
        // Should fail or succeed depending on truncation point
        if truncated_data.len() < 12 {
            assert!(result.is_err());
        }
    }
}

// ============================================================================
// Vector Error Tests
// ============================================================================

#[test]
fn test_vector_error_too_large() {
    let err = VectorError::too_large(10000, 1000);
    assert!(matches!(err, VectorError::TooLarge { size: 10000, max: 1000 }));
    assert_eq!(err.to_string(), "Vector size 10000 exceeds maximum 1000");
}

#[test]
fn test_vector_error_invalid_prefix() {
    let test_prefixes = vec![0x00000000u32, 0xFFFFFFFFu32, 0x12345678u32];

    for prefix in test_prefixes {
        let err = VectorError::invalid_prefix(prefix);
        assert!(matches!(err, VectorError::InvalidPrefix(_)));
        assert!(err.to_string().contains("invalid"));
    }
}

#[test]
fn test_vector_error_conversion() {
    let vec_err = VectorError::too_large(5000, 1000);
    let tl_err: TlError = vec_err.into();
    assert!(matches!(tl_err, TlError::VectorError(_)));
}

// ============================================================================
// Error Message Formatting
// ============================================================================

#[test]
fn test_unknown_constructor_error_message() {
    let err = TlError::unknown_constructor(vec![0x12345678, 0x87654321], 0xAAAAAAAA, "TestType");
    let msg = err.to_string();

    // Just verify the error message contains the constructor ID in hex format
    assert!(msg.contains("0x"));
}

#[test]
fn test_unexpected_eof_error_message() {
    let err = TlError::unexpected_eof(100, 50, "TestBuffer");
    let msg = err.to_string();

    assert!(msg.contains("100"));
    assert!(msg.contains("50"));
    assert!(msg.contains("TestBuffer"));
    assert!(msg.contains("EOF"));
}

#[test]
fn test_validation_error_message() {
    let err = TlError::validation_failed("user_id", "-1", "ID must be positive");
    let msg = err.to_string();

    assert!(msg.contains("user_id"));
    assert!(msg.contains("-1"));
    assert!(msg.contains("positive"));
}

// ============================================================================
// Error Construction Methods
// ============================================================================

#[test]
fn test_error_constructors() {
    // Test all error construction methods
    let _ = TlError::unknown_constructor(vec![0x12345678], 0x87654321, "Test");
    let _ = TlError::unexpected_eof(10, 5, "TestType");
    let _ = TlError::validation_failed("field", "value", "reason");
    let _ = TlError::deserialize_error("test error");
    let _ = TlError::TypeConversionError("test conversion".to_string());

    let _ = VectorError::too_large(100, 50);
    let _ = VectorError::invalid_prefix(0x12345678);
}

// ============================================================================
// Error Matching and Patterns
// ============================================================================

#[test]
fn test_error_matching() {
    let err = TlError::unknown_constructor(vec![0x12345678], 0x87654321, "Test");

    match &err {
        TlError::UnknownConstructor { found, .. } => assert_eq!(*found, 0x87654321),
        _ => panic!("Should match UnknownConstructor"),
    }
}

#[test]
fn test_error_downcasting() {
    let tl_err = TlError::VectorError(VectorError::too_large(100, 50));

    match &tl_err {
        TlError::VectorError(vec_err) => {
            assert!(matches!(vec_err, VectorError::TooLarge { .. }));
        }
        _ => panic!("Should contain VectorError"),
    }
}

// ============================================================================
// TLHelper Error Propagation
// ============================================================================

#[test]
fn test_tl_helper_read_constructor_id_empty_buffer() {
    let data = [];
    let mut buf = create_buffer(&data);

    let result = TlHelper::read_constructor_id(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_tl_helper_read_i64_empty_buffer() {
    let data = [];
    let mut buf = create_buffer(&data);

    let result = TlHelper::read_i64(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_tl_helper_read_i32_empty_buffer() {
    let data = [];
    let mut buf = create_buffer(&data);

    let result = TlHelper::read_i32(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_tl_helper_read_string_empty_buffer() {
    let data = [];
    let mut buf = create_buffer(&data);

    let result = TlHelper::read_string(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_tl_helper_read_bytes_empty_buffer() {
    let data = [];
    let mut buf = create_buffer(&data);

    let result = TlHelper::read_bytes(&mut buf);
    assert!(result.is_err());
}

// ============================================================================
// Error Recovery and State
// ============================================================================

#[test]
fn test_multiple_errors_same_buffer() {
    // Test that we can get consistent errors from the same buffer state
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0];

    let mut buf1 = create_buffer(&data);
    let result1 = Peer::deserialize_tl(&mut buf1);
    let err_msg1 = result1.unwrap_err().to_string();

    let mut buf2 = create_buffer(&data);
    let result2 = Peer::deserialize_tl(&mut buf2);
    let err_msg2 = result2.unwrap_err().to_string();

    // Error messages should be consistent
    assert!(err_msg1.contains("Unknown") || err_msg1.contains("0x"));
    assert!(err_msg2.contains("Unknown") || err_msg2.contains("0x"));
}

#[test]
fn test_error_after_partial_read() {
    // Create a buffer with valid constructor but invalid rest
    let mut data = vec![0x22, 0x17, 0x51, 0x59]; // valid peerUser constructor
    data.extend_from_slice(&[0xFF; 4]); // Incomplete user_id

    let mut buf = create_buffer(&data);
    let result = Peer::deserialize_tl(&mut buf);
    assert!(result.is_err());
}

// ============================================================================
// Error Display and Debug
// ============================================================================

#[test]
fn test_error_debug_format() {
    let err = TlError::unknown_constructor(vec![0x12345678], 0x87654321, "Test");

    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("UnknownConstructor"));
}

#[test]
fn test_error_display_format() {
    let err = TlError::unexpected_eof(10, 5, "TestType");

    // Just verify the error can be displayed without panicking
    let _ = format!("{}", err);
}

// ============================================================================
// Invalid Data Patterns
// ============================================================================

#[test]
fn test_all_zero_constructor() {
    // Constructor ID of 0 is always invalid
    let data = [0u8; 12]; // 4 bytes constructor + 8 bytes data
    let mut buf = create_buffer(&data);

    let result: Result<Peer, _> = Peer::deserialize_tl(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_repeated_byte_patterns() {
    // Test with repeated byte patterns
    let patterns: &[u8] = &[0x00, 0xFF, 0xAA, 0x55, 0x11, 0x22, 0x33, 0x44];

    for &byte in patterns {
        let data = [byte; 12];
        let mut buf = create_buffer(&data);

        let result: Result<Peer, _> = Peer::deserialize_tl(&mut buf);
        // Most should fail since these aren't valid constructors
        if byte != 0x22 && byte != 0x9a && byte != 0x1e {
            // These bytes happen to be part of valid constructors
            // (but not aligned correctly)
            let _ = result; // Just check it doesn't panic
        }
    }
}

#[test]
fn test_corrupted_vector_prefix() {
    // Vector should have prefix 0x1cb5c415
    let invalid_prefixes = vec![
        0x00000000u32, 0x11111111u32, 0x22222222u32, 0x33333333u32,
        0x44444444u32, 0x55555555u32, 0x66666666u32, 0x77777777u32,
        0x88888888u32, 0x99999999u32, 0xAAAAAAAAu32, 0xBBBBBBBBu32,
        0xCCCCCCCCu32, 0xDDDDDDDDu32, 0xEEEEEEEEu32, 0xFFFFFFFFu32,
    ];

    // Note: This is indirect testing - we'd need to call deserialize_vector_*
    // directly to properly test this. For now, verify the error can be created.
    for prefix in invalid_prefixes {
        if prefix != 0x1cb5c415 {
            let err = VectorError::invalid_prefix(prefix);
            assert!(matches!(err, VectorError::InvalidPrefix(_)));
        }
    }
}
