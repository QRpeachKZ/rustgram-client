// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Integration tests for TL serialization

#![allow(clippy::unwrap_used, clippy::expect_used)]

use rustgram_logevent::{
    FlagsParser, FlagsStorer, LogEventParser, LogEventStorerVec, TlParser, TlStorer,
};

#[test]
fn test_flags_storer_and_parser_roundtrip() {
    let inputs = vec![
        vec![true, false, true],
        vec![false; 10],
        vec![true; 8],
        vec![true, false, true, false, true, false, true, false],
        vec![false, true, false, true, false, true],
    ];

    for input in inputs {
        let mut storer = FlagsStorer::new();
        for &flag in &input {
            storer.store_flag(flag);
        }
        let flags = storer.finish();

        let mut parser = FlagsParser::new(flags);
        let output: Vec<bool> = (0..input.len()).map(|_| parser.parse_flag()).collect();

        assert_eq!(input, output);
        parser.finish().unwrap();
    }
}

#[test]
fn test_flags_parser_has_flag() {
    let flags = 0b1010_0101u32;
    let parser = FlagsParser::new(flags);

    assert!(parser.has_flag(0));
    assert!(!parser.has_flag(1));
    assert!(parser.has_flag(2));
    assert!(!parser.has_flag(3));
    assert!(parser.has_flag(5));
    assert!(!parser.has_flag(6));
    assert!(parser.has_flag(7));
}

#[test]
fn test_flags_parser_peek() {
    let flags = 0b1010u32;
    let mut parser = FlagsParser::new(flags);

    assert!(!parser.peek_flag());
    assert!(!parser.peek_flag()); // peek doesn't advance

    parser.parse_flag();
    assert!(parser.peek_flag());
}

#[test]
fn test_flags_parser_mask() {
    let flags = 0b1101_0110u32;
    let mut parser = FlagsParser::new(flags);

    let mask1 = parser.parse_flags_mask(3);
    assert_eq!(mask1, 0b110);

    let mask2 = parser.parse_flags_mask(5);
    // flags = 0b1101_0110, after extracting 3 bits:
    // (0b1101_0110 >> 3) & ((1 << 5) - 1) = 0b0001_1010 & 0b11111 = 0b11010 = 26
    assert_eq!(mask2, 0b11010);

    // Should have consumed 8 bits total
    assert_eq!(parser.bit_offset(), 8);
}

#[test]
fn test_tl_parser_i32() {
    let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x42];
    let mut parser = LogEventParser::new(&data);
    assert_eq!(parser.fetch_i32().unwrap(), 0x42);
    parser.fetch_end().unwrap();
}

#[test]
fn test_tl_parser_i64() {
    let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67];
    let mut parser = LogEventParser::new(&data);
    assert_eq!(parser.fetch_i64().unwrap(), 0x0123_4567);
}

#[test]
fn test_tl_parser_bytes() {
    let data: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x03, // length
        0x01, 0x02, 0x03, // data
    ];
    let mut parser = LogEventParser::new(&data);
    assert_eq!(parser.fetch_bytes().unwrap(), vec![0x01, 0x02, 0x03]);
    parser.fetch_end().unwrap();
}

#[test]
fn test_tl_parser_empty_bytes() {
    let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
    let mut parser = LogEventParser::new(&data);
    assert_eq!(parser.fetch_bytes().unwrap(), Vec::<u8>::new());
    parser.fetch_end().unwrap();
}

#[test]
fn test_tl_parser_bool() {
    let data: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x01, // true
        0x00, 0x00, 0x00, 0x00, // false
        0x00, 0x00, 0x00, 0x01, // true
    ];
    let mut parser = LogEventParser::new(&data);
    assert!(parser.fetch_bool().unwrap());
    assert!(!parser.fetch_bool().unwrap());
    assert!(parser.fetch_bool().unwrap());
    parser.fetch_end().unwrap();
}

#[test]
fn test_tl_parser_remaining() {
    let data = vec![0u8; 16];
    let parser = LogEventParser::new(&data);
    assert_eq!(parser.remaining(), 16);
    assert_eq!(parser.total_len(), 16);
}

#[test]
fn test_tl_parser_position() {
    let data = vec![0u8; 16];
    let mut parser = LogEventParser::new(&data);
    assert_eq!(parser.position(), 0);
    let _ = parser.fetch_i32();
    assert_eq!(parser.position(), 4);
    let _ = parser.fetch_i64();
    assert_eq!(parser.position(), 12);
}

#[test]
fn test_tl_storer_vec_i32() {
    let mut storer = LogEventStorerVec::new();
    storer.store_i32(0x12345678);
    assert_eq!(storer.as_slice(), &[0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn test_tl_storer_vec_i64() {
    let mut storer = LogEventStorerVec::new();
    storer.store_i64(0x123456789ABCDEF0);
    assert_eq!(
        storer.as_slice(),
        &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]
    );
}

#[test]
fn test_tl_storer_vec_bytes() {
    let mut storer = LogEventStorerVec::new();
    storer.store_bytes(&[0x01, 0x02, 0x03]);
    assert_eq!(
        storer.as_slice(),
        &[0x00, 0x00, 0x00, 0x03, 0x01, 0x02, 0x03]
    );
}

#[test]
fn test_tl_storer_vec_bool() {
    let mut storer = LogEventStorerVec::new();
    storer.store_bool(true);
    storer.store_bool(false);
    assert_eq!(storer.as_slice(), &[0, 0, 0, 1, 0, 0, 0, 0]);
}

#[test]
fn test_tl_storer_vec_u32() {
    let mut storer = LogEventStorerVec::new();
    storer.store_u32(0xDEADBEEF);
    assert_eq!(storer.as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn test_tl_roundtrip_i32() {
    let values = vec![0, 1, -1, 0x7FFFFFFF, -0x80000000, 42, -42, 0x12345678];

    for value in values {
        let mut storer = LogEventStorerVec::new();
        storer.store_i32(value);
        let data = storer.into_inner();

        let mut parser = LogEventParser::new(&data);
        let parsed = parser.fetch_i32().unwrap();
        assert_eq!(value, parsed);
        parser.fetch_end().unwrap();
    }
}

#[test]
fn test_tl_roundtrip_i64() {
    let values = vec![
        0,
        1,
        -1,
        0x7FFFFFFFFFFFFFFF,
        -0x8000000000000000,
        42,
        -42,
        0x123456789ABCDEF0,
    ];

    for value in values {
        let mut storer = LogEventStorerVec::new();
        storer.store_i64(value);
        let data = storer.into_inner();

        let mut parser = LogEventParser::new(&data);
        let parsed = parser.fetch_i64().unwrap();
        assert_eq!(value, parsed);
        parser.fetch_end().unwrap();
    }
}

#[test]
fn test_tl_roundtrip_bytes() {
    let test_data = vec![
        Vec::<u8>::new(),
        vec![0x01],
        vec![0x01, 0x02, 0x03],
        vec![0xFF; 100],
        vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    ];

    for data in test_data {
        let mut storer = LogEventStorerVec::new();
        storer.store_bytes(&data);
        let serialized = storer.into_inner();

        let mut parser = LogEventParser::new(&serialized);
        let parsed = parser.fetch_bytes().unwrap();
        assert_eq!(data, parsed);
        parser.fetch_end().unwrap();
    }
}

#[test]
fn test_tl_roundtrip_bool() {
    let values = vec![true, false, true, true, false];

    let mut storer = LogEventStorerVec::new();
    for &value in &values {
        storer.store_bool(value);
    }
    let data = storer.into_inner();

    let mut parser = LogEventParser::new(&data);
    for expected in values {
        let parsed = parser.fetch_bool().unwrap();
        assert_eq!(expected, parsed);
    }
    parser.fetch_end().unwrap();
}

#[test]
fn test_tl_roundtrip_mixed() {
    let mut storer = LogEventStorerVec::new();
    storer.store_i32(42);
    storer.store_i64(0x123456789ABCDEF0);
    storer.store_bool(true);
    storer.store_bytes(&[1, 2, 3, 4, 5]);
    storer.store_u32(0xDEADBEEF);
    let data = storer.into_inner();

    let mut parser = LogEventParser::new(&data);
    assert_eq!(parser.fetch_i32().unwrap(), 42);
    assert_eq!(parser.fetch_i64().unwrap(), 0x123456789ABCDEF0);
    assert!(parser.fetch_bool().unwrap());
    assert_eq!(parser.fetch_bytes().unwrap(), vec![1, 2, 3, 4, 5]);
    assert_eq!(parser.fetch_u32().unwrap(), 0xDEADBEEF);
    parser.fetch_end().unwrap();
}

#[test]
fn test_flags_parse_extra_bits_error() {
    let flags = 0b1111u32;
    let mut parser = FlagsParser::new(flags);
    parser.parse_flag(); // only parse 1 bit
    assert!(parser.finish().is_err());
}

#[test]
fn test_tl_parser_unexpected_end() {
    let data = vec![0x00, 0x00, 0x00, 0x05, 0x01, 0x02]; // length 5 but only 2 bytes
    let mut parser = LogEventParser::new(&data);
    assert!(parser.fetch_bytes().is_err());
}

#[test]
fn test_tl_parser_not_fully_consumed() {
    let data = vec![0x00, 0x00, 0x00, 0x01, 0x00];
    let mut parser = LogEventParser::new(&data);
    parser.fetch_i32().unwrap();
    assert!(parser.fetch_end().is_err());
}

#[test]
fn test_tl_storer_vec_clear() {
    let mut storer = LogEventStorerVec::new();
    storer.store_i32(42);
    assert_eq!(storer.len(), 4);

    storer.clear();
    assert_eq!(storer.len(), 0);
    assert!(storer.is_empty());
}

#[test]
fn test_tl_storer_vec_with_capacity() {
    let storer = LogEventStorerVec::with_capacity(100);
    assert!(storer.is_empty());
}

#[test]
fn test_tl_storer_vec_into_inner() {
    let mut storer = LogEventStorerVec::new();
    storer.store_i32(42);
    let data = storer.into_inner();
    // i32(42) in big-endian = [0, 0, 0, 42]
    assert_eq!(data, vec![0, 0, 0, 42]);
}
