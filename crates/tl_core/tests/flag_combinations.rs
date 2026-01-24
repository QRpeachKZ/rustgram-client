// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Flag combination tests for PeerNotifySettings.
//!
//! This module tests various flag combinations to ensure proper
//! handling of optional fields.

use rustgram_tl_core::flags::FlagReader;

// Test all possible single-bit flag values
const ALL_SINGLE_BIT_FLAGS: [u32; 32] = [
    0x00000001, 0x00000002, 0x00000004, 0x00000008, 0x00000010, 0x00000020,
    0x00000040, 0x00000080, 0x00000100, 0x00000200, 0x00000400, 0x00000800,
    0x00001000, 0x00002000, 0x00004000, 0x00008000, 0x00010000, 0x00020000,
    0x00040000, 0x00080000, 0x00100000, 0x00200000, 0x00400000, 0x00800000,
    0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000,
    0x40000000, 0x80000000,
];

#[test]
fn test_all_single_bit_flags() {
    // Test that each single-bit flag is correctly detected
    for (bit_index, &flag_value) in ALL_SINGLE_BIT_FLAGS.iter().enumerate() {
        let reader = FlagReader::new(flag_value);
        assert!(
            reader.has(bit_index as u32),
            "Bit {} should be set for flags 0x{:08x}",
            bit_index,
            flag_value
        );

        // Verify all other bits are not set
        for other_bit in 0..32 {
            if other_bit != bit_index as u32 {
                assert!(
                    !reader.has(other_bit),
                    "Bit {} should NOT be set for flags 0x{:08x}",
                    other_bit,
                    flag_value
                );
            }
        }
    }
}

#[test]
fn test_flag_reader_with_all_bits_set() {
    let reader = FlagReader::new(0xFFFFFFFF);
    assert_eq!(reader.count(), 32);
    for bit in 0..32 {
        assert!(reader.has(bit));
    }
}

#[test]
fn test_flag_reader_with_no_bits_set() {
    let reader = FlagReader::new(0x00000000);
    assert_eq!(reader.count(), 0);
    for bit in 0..32 {
        assert!(!reader.has(bit));
    }
}

#[test]
fn test_flag_reader_with_alternating_bits() {
    // Pattern: 0x55555555 = 01010101... in binary
    let flags = 0x55555555;
    let reader = FlagReader::new(flags);

    assert_eq!(reader.count(), 16);

    for bit in 0..32 {
        if bit % 2 == 0 {
            assert!(
                reader.has(bit),
                "Even bit {} should be set in 0x55555555",
                bit
            );
        } else {
            assert!(
                !reader.has(bit),
                "Odd bit {} should NOT be set in 0x55555555",
                bit
            );
        }
    }
}

#[test]
fn test_flag_reader_with_consecutive_low_bits() {
    // Test patterns with 1, 2, 3, 4, etc. consecutive low bits set
    let test_cases = vec![
        (0b1u32, 1),
        (0b11u32, 2),
        (0b111u32, 3),
        (0b1111u32, 4),
        (0b11111u32, 5),
        (0b111111u32, 6),
        (0b1111111u32, 7),
        (0b11111111u32, 8),
    ];

    for (flags, expected_count) in test_cases {
        let reader = FlagReader::new(flags);
        assert_eq!(
            reader.count(),
            expected_count,
            "Flags 0x{:08x} should have {} bits set",
            flags,
            expected_count
        );
    }
}

#[test]
fn test_flag_reader_with_consecutive_high_bits() {
    // Test patterns with consecutive high bits set
    let test_cases = vec![
        (0x80000000u32, 1, vec![31]),
        (0xC0000000u32, 2, vec![30, 31]),
        (0xE0000000u32, 3, vec![29, 30, 31]),
        (0xF0000000u32, 4, vec![28, 29, 30, 31]),
    ];

    for (flags, expected_count, set_bits) in test_cases {
        let reader = FlagReader::new(flags);
        assert_eq!(reader.count(), expected_count);
        for bit in 0..32 {
            if set_bits.contains(&(bit as i32)) {
                assert!(reader.has(bit));
            } else {
                assert!(!reader.has(bit));
            }
        }
    }
}

#[test]
fn test_has_any_with_various_inputs() {
    let reader = FlagReader::new(0b10101010u32); // bits 1, 3, 5, 7 set

    assert!(reader.has_any(&[1, 3]));
    assert!(reader.has_any(&[1, 2])); // 1 is set
    assert!(reader.has_any(&[0, 1])); // 1 is set
    assert!(reader.has_any(&[5])); // single element

    assert!(!reader.has_any(&[0]));
    assert!(!reader.has_any(&[2, 4, 6]));
    assert!(!reader.has_any(&[8, 9, 10]));

    // Empty slice should return false (no bits to check)
    assert!(!reader.has_any(&[]));
}

#[test]
fn test_has_all_with_various_inputs() {
    let reader = FlagReader::new(0b10101010u32); // bits 1, 3, 5, 7 set

    assert!(reader.has_all(&[1, 3]));
    assert!(reader.has_all(&[1, 3, 5, 7]));
    assert!(reader.has_all(&[5])); // single element

    assert!(!reader.has_all(&[1, 2])); // 2 is not set
    assert!(!reader.has_all(&[0, 1])); // 0 is not set
    assert!(!reader.has_all(&[8, 9])); // neither set

    // Empty slice should return true (vacuously true)
    assert!(reader.has_all(&[]));
}

#[test]
fn test_read_bool_all_bits() {
    // Test read_bool for all 32 bit positions
    for bit in 0..32 {
        let flag_value = 1u32 << bit;
        let reader = FlagReader::new(flag_value);
        assert!(reader.read_bool(bit), "read_bool({}) should return true", bit);

        // All other bits should return false
        for other_bit in 0..32 {
            if other_bit != bit {
                assert!(
                    !reader.read_bool(other_bit),
                    "read_bool({}) should return false when only bit {} is set",
                    other_bit,
                    bit
                );
            }
        }
    }
}

#[test]
fn test_read_optional_with_all_bits() {
    // Test read_optional for all 32 bit positions
    for bit in 0..32 {
        let flag_value = 1u32 << bit;
        let reader = FlagReader::new(flag_value);

        // When flag is set, should get Some(value)
        let result = reader.read_optional(bit, || Ok(42i32));
        assert_eq!(result.unwrap(), Some(42), "Bit {} should return Some(42)", bit);

        // When flag is not set, should get None
        let other_bit = (bit + 1) % 32;
        let result = reader.read_optional(other_bit, || Ok(99i32));
        assert_eq!(
            result.unwrap(),
            None,
            "Bit {} should return None",
            other_bit
        );
    }
}

#[test]
fn test_flag_reader_edge_cases() {
    // Test minimum value
    let reader = FlagReader::new(u32::MIN);
    assert_eq!(reader.flags(), 0);
    assert_eq!(reader.count(), 0);

    // Test maximum value
    let reader = FlagReader::new(u32::MAX);
    assert_eq!(reader.flags(), 0xFFFFFFFF);
    assert_eq!(reader.count(), 32);
}

#[test]
fn test_notify_settings_flag_combinations() {
    // Test various realistic flag combinations for PeerNotifySettings
    // which uses flags 0-10

    // No flags set - all optional fields are None
    let reader = FlagReader::new(0u32);
    for bit in 0..=10 {
        assert!(!reader.has(bit));
    }

    // All notification flags set
    let reader = FlagReader::new(0b0000011111111111u32);
    for bit in 0..=10 {
        assert!(reader.has(bit));
    }

    // Only story-related flags (6-10) set
    // Binary: 0000 0111 1100 0000 = 0x07C0
    let reader = FlagReader::new(0x07C0u32);
    for bit in 0..=5 {
        assert!(!reader.has(bit));
    }
    for bit in 6..=10 {
        assert!(reader.has(bit));
    }
}

#[test]
fn test_chat_full_flag_combinations() {
    // Test flag combinations for ChatFull which uses various flag bits
    // Key flags: 2 (photo), 3 (bot_info), 6 (pinned_msg), 7 (can_set_username),
    // 8 (has_scheduled), 11 (folder_id), 12 (call), 13 (exported_invite),
    // 14 (ttl_period), 15 (groupcall), 16 (theme), 17 (requests),
    // 18 (reactions), 19 (translations_disabled), 20 (reactions_limit)

    // Test critical flag combinations
    let test_cases = vec![
        (0x00u32, vec![]), // No flags
        (0x04u32, vec![2]), // Only chat_photo
        (0x08u32, vec![3]), // Only bot_info
        (0x40u32, vec![6]), // Only pinned_msg_id
        (0x80u32, vec![7]), // Only can_set_username
        (0x100u32, vec![8]), // Only has_scheduled
        (0x800u32, vec![11]), // Only folder_id
        (0x1000u32, vec![12]), // Only call
        (0x2000u32, vec![13]), // Only exported_invite
        (0x4000u32, vec![14]), // Only ttl_period
        (0x8000u32, vec![15]), // Only groupcall_default_join_as
        (0x10000u32, vec![16]), // Only theme_emoticon
        (0x20000u32, vec![17]), // Only requests_pending
        (0x40000u32, vec![18]), // Only available_reactions
        (0x80000u32, vec![19]), // Only translations_disabled
        (0x100000u32, vec![20]), // Only reactions_limit
    ];

    for (flags, expected_bits) in test_cases {
        let reader = FlagReader::new(flags);

        for bit in 0..=20 {
            if expected_bits.contains(&(bit as i32)) {
                assert!(
                    reader.has(bit),
                    "Bit {} should be set for flags 0x{:05x}",
                    bit,
                    flags
                );
            } else {
                assert!(
                    !reader.has(bit),
                    "Bit {} should NOT be set for flags 0x{:05x}",
                    bit,
                    flags
                );
            }
        }
    }
}

#[test]
fn test_user_full_flag_combinations() {
    // Test flag combinations for UserFull
    // Key flags: 0 (blocked), 1 (about), 2 (profile_photo), 3 (bot_info),
    // 4 (phone_calls_available), 5 (phone_calls_private), 6 (pinned_msg),
    // 7 (can_pin_message), 11 (folder_id), 12 (has_scheduled),
    // 13 (video_calls_available), 14 (ttl_period), 15 (theme),
    // 16 (private_forward_name), 20 (voice_messages_forbidden),
    // 21 (personal_photo), 22 (fallback_photo), 23 (translations_disabled)

    // Test common flag combinations
    let test_cases = vec![
        (0x01u32, vec![0]), // blocked
        (0x02u32, vec![1]), // about
        (0x04u32, vec![2]), // profile_photo
        (0x08u32, vec![3]), // bot_info
        (0x10u32, vec![4]), // phone_calls_available
        (0x20u32, vec![5]), // phone_calls_private
        (0x40u32, vec![6]), // pinned_msg_id
        (0x80u32, vec![7]), // can_pin_message
        (0x800u32, vec![11]), // folder_id
        (0x1000u32, vec![12]), // has_scheduled
        (0x2000u32, vec![13]), // video_calls_available
        (0x4000u32, vec![14]), // ttl_period
        (0x8000u32, vec![15]), // theme
        (0x10000u32, vec![16]), // private_forward_name
        (0x100000u32, vec![20]), // voice_messages_forbidden
        (0x200000u32, vec![21]), // personal_photo
        (0x400000u32, vec![22]), // fallback_photo
        (0x800000u32, vec![23]), // translations_disabled
    ];

    for (flags, expected_bits) in test_cases {
        let reader = FlagReader::new(flags);

        for &bit in &expected_bits {
            assert!(
                reader.has(bit as u32),
                "Bit {} should be set for flags 0x{:06x}",
                bit,
                flags
            );
        }
    }
}

#[test]
fn test_flag_patterns() {
    // Test common flag patterns

    // Power of two patterns
    for exp in 0..32 {
        let flags = 1u32 << exp;
        let reader = FlagReader::new(flags);
        assert_eq!(reader.count(), 1);
        assert!(reader.has(exp));
    }

    // All bits in lower half set
    let reader = FlagReader::new(0x0000FFFF);
    assert_eq!(reader.count(), 16);
    for bit in 0..16 {
        assert!(reader.has(bit));
    }
    for bit in 16..32 {
        assert!(!reader.has(bit));
    }

    // All bits in upper half set
    let reader = FlagReader::new(0xFFFF0000);
    assert_eq!(reader.count(), 16);
    for bit in 0..16 {
        assert!(!reader.has(bit));
    }
    for bit in 16..32 {
        assert!(reader.has(bit));
    }

    // Checkerboard pattern: 0xAAAAAAAA = 10101010... in binary
    let reader = FlagReader::new(0xAAAAAAAA);
    assert_eq!(reader.count(), 16);
    for bit in 0..32 {
        if bit % 2 == 1 {
            assert!(reader.has(bit));
        } else {
            assert!(!reader.has(bit));
        }
    }
}

#[test]
fn test_flag_boundary_values() {
    // Test boundary conditions for flag operations

    // Bit 31 (highest bit)
    let reader = FlagReader::new(0x80000000);
    assert!(reader.has(31));
    assert_eq!(reader.count(), 1);

    // Bits 30 and 31
    let reader = FlagReader::new(0xC0000000);
    assert!(reader.has(30));
    assert!(reader.has(31));
    assert_eq!(reader.count(), 2);

    // All bits except 31
    let reader = FlagReader::new(0x7FFFFFFF);
    for bit in 0..31 {
        assert!(reader.has(bit));
    }
    assert!(!reader.has(31));
    assert_eq!(reader.count(), 31);
}

#[test]
fn test_multiple_flags_simultaneously() {
    // Test combinations of multiple flags
    let test_cases = vec![
        (0b00000011u32, 2, vec![0, 1]),
        (0b00000101u32, 2, vec![0, 2]),
        (0b00001001u32, 2, vec![0, 3]),
        (0b00000111u32, 3, vec![0, 1, 2]),
        (0b00001111u32, 4, vec![0, 1, 2, 3]),
        (0b00011111u32, 5, vec![0, 1, 2, 3, 4]),
        (0b00111111u32, 6, vec![0, 1, 2, 3, 4, 5]),
        (0b01111111u32, 7, vec![0, 1, 2, 3, 4, 5, 6]),
        (0b11111111u32, 8, vec![0, 1, 2, 3, 4, 5, 6, 7]),
    ];

    for (flags, expected_count, set_bits) in test_cases {
        let reader = FlagReader::new(flags);
        assert_eq!(reader.count(), expected_count);

        for bit in 0..8 {
            if set_bits.contains(&(bit as i32)) {
                assert!(reader.has(bit));
            } else {
                assert!(!reader.has(bit));
            }
        }
    }
}
