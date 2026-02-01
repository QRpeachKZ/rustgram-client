// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Hash functions for Telegram MTProto client.
//!
//! This module provides hash computation functions used throughout the client.

/// Computes a truncated MD5 hash of a string as a u64.
///
/// This function computes the MD5 hash of the input string and returns
/// the first 8 bytes as a u64 value.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:301-309`
///
/// # Arguments
///
/// * `str` - The input string to hash
///
/// # Returns
///
/// A u64 hash value computed from the first 8 bytes of the MD5 hash.
///
/// # Examples
///
/// ```
/// use rustgram_misc::get_md5_string_hash;
///
/// let hash1 = get_md5_string_hash("test");
/// let hash2 = get_md5_string_hash("test");
/// assert_eq!(hash1, hash2);
///
/// let hash3 = get_md5_string_hash("different");
/// assert_ne!(hash1, hash3);
/// ```
pub fn get_md5_string_hash(str: &str) -> u64 {
    let digest = md5::compute(str.as_bytes());
    let bytes = digest.0;

    // Convert first 8 bytes to u64 (big-endian)
    let mut result: u64 = 0;
    for (i, &byte) in bytes.iter().take(8).enumerate() {
        result |= (byte as u64) << (56 - 8 * i);
    }
    result
}

/// Computes a hash from a vector of u64 values.
///
/// This function combines all values in the vector using a custom hash algorithm
/// that mixes bits and accumulates the values.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:311-320`
///
/// # Arguments
///
/// * `numbers` - A slice of u64 values to hash
///
/// # Returns
///
/// An i64 hash value computed from the input numbers.
///
/// # Examples
///
/// ```
/// use rustgram_misc::get_vector_hash;
///
/// let vec1 = vec![1u64, 2, 3, 4, 5];
/// let hash1 = get_vector_hash(&vec1);
/// assert_eq!(hash1, get_vector_hash(&vec1));
///
/// let vec2 = vec![1u64, 2, 3, 4, 6];
/// assert_ne!(hash1, get_vector_hash(&vec2));
/// ```
pub fn get_vector_hash(numbers: &[u64]) -> i64 {
    let mut acc: u64 = 0;

    for &number in numbers {
        acc ^= acc >> 21;
        acc ^= acc << 35;
        acc ^= acc >> 4;
        acc = acc.wrapping_add(number);
    }

    acc as i64
}

/// Returns 4 emoji corresponding to a 32-byte buffer.
///
/// This function takes a 32-byte buffer (typically a cryptographic hash or key)
/// and returns 4 emoji that can be used as a visual fingerprint.
///
/// The buffer is divided into 4 segments of 8 bytes each, and each segment
/// is converted to an emoji based on its value.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:380-388`, `misc.cpp:322-378` (emoji list)
///
/// # Arguments
///
/// * `buffer` - A 32-byte buffer to convert to emoji fingerprints
///
/// # Returns
///
/// A vector of 4 emoji strings.
///
/// # Panics
///
/// Panics if the buffer is not exactly 32 bytes.
///
/// # Examples
///
/// ```
/// use rustgram_misc::get_emoji_fingerprints;
///
/// let buffer = [0u8; 32];
/// let emojis = get_emoji_fingerprints(&buffer);
/// assert_eq!(emojis.len(), 4);
/// ```
pub fn get_emoji_fingerprints(buffer: &[u8]) -> Vec<String> {
    assert_eq!(buffer.len(), 32, "Buffer must be exactly 32 bytes");

    let mut result = Vec::with_capacity(4);

    for i in 0..4 {
        let start = i * 8;
        let end = start + 8;

        // Convert 8 bytes to u64 (big-endian)
        let mut num: u64 = 0;
        for (j, &byte) in buffer[start..end].iter().enumerate() {
            num |= (byte as u64) << (56 - 8 * j);
        }

        result.push(get_emoji_fingerprint(num));
    }

    result
}

/// Returns an emoji corresponding to a u64 value.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:322-378`
///
/// The emoji list contains 367 different emoji sequences.
fn get_emoji_fingerprint(num: u64) -> String {
    const EMOJIS: &[&str] = &[
        "\u{1f609}",
        "\u{1f60d}",
        "\u{1f61b}",
        "\u{1f62d}",
        "\u{1f631}",
        "\u{1f621}",
        "\u{1f60e}",
        "\u{1f634}",
        "\u{1f635}",
        "\u{1f608}",
        "\u{1f62c}",
        "\u{1f607}",
        "\u{1f60f}",
        "\u{1f46e}",
        "\u{1f477}",
        "\u{1f482}",
        "\u{1f476}",
        "\u{1f468}",
        "\u{1f469}",
        "\u{1f474}",
        "\u{1f475}",
        "\u{1f63b}",
        "\u{1f63d}",
        "\u{1f640}",
        "\u{1f47a}",
        "\u{1f648}",
        "\u{1f649}",
        "\u{1f64a}",
        "\u{1f480}",
        "\u{1f47d}",
        "\u{1f4a9}",
        "\u{1f525}",
        "\u{1f4a5}",
        "\u{1f4a4}",
        "\u{1f442}",
        "\u{1f440}",
        "\u{1f443}",
        "\u{1f445}",
        "\u{1f444}",
        "\u{1f44d}",
        "\u{1f44e}",
        "\u{1f44c}",
        "\u{1f44a}",
        "\u{270c}",
        "\u{270b}",
        "\u{1f450}",
        "\u{1f446}",
        "\u{1f447}",
        "\u{1f449}",
        "\u{1f448}",
        "\u{1f64f}",
        "\u{1f44f}",
        "\u{1f4aa}",
        "\u{1f6b6}",
        "\u{1f3c3}",
        "\u{1f483}",
        "\u{1f46b}",
        "\u{1f46a}",
        "\u{1f46c}",
        "\u{1f46d}",
        "\u{1f485}",
        "\u{1f3a9}",
        "\u{1f451}",
        "\u{1f452}",
        "\u{1f45f}",
        "\u{1f45e}",
        "\u{1f460}",
        "\u{1f455}",
        "\u{1f457}",
        "\u{1f456}",
        "\u{1f459}",
        "\u{1f45c}",
        "\u{1f453}",
        "\u{1f380}",
        "\u{1f484}",
        "\u{1f49b}",
        "\u{1f499}",
        "\u{1f49c}",
        "\u{1f49a}",
        "\u{1f48d}",
        "\u{1f48e}",
        "\u{1f436}",
        "\u{1f43a}",
        "\u{1f431}",
        "\u{1f42d}",
        "\u{1f439}",
        "\u{1f430}",
        "\u{1f438}",
        "\u{1f42f}",
        "\u{1f428}",
        "\u{1f43b}",
        "\u{1f437}",
        "\u{1f42e}",
        "\u{1f417}",
        "\u{1f434}",
        "\u{1f411}",
        "\u{1f418}",
        "\u{1f43c}",
        "\u{1f427}",
        "\u{1f425}",
        "\u{1f414}",
        "\u{1f40d}",
        "\u{1f422}",
        "\u{1f41b}",
        "\u{1f41d}",
        "\u{1f41c}",
        "\u{1f41e}",
        "\u{1f40c}",
        "\u{1f419}",
        "\u{1f41a}",
        "\u{1f41f}",
        "\u{1f42c}",
        "\u{1f40b}",
        "\u{1f410}",
        "\u{1f40a}",
        "\u{1f42b}",
        "\u{1f340}",
        "\u{1f339}",
        "\u{1f33b}",
        "\u{1f341}",
        "\u{1f33e}",
        "\u{1f344}",
        "\u{1f335}",
        "\u{1f334}",
        "\u{1f333}",
        "\u{1f31e}",
        "\u{1f31a}",
        "\u{1f319}",
        "\u{1f30e}",
        "\u{1f30b}",
        "\u{26a1}",
        "\u{2614}",
        "\u{2744}",
        "\u{26c4}",
        "\u{1f300}",
        "\u{1f308}",
        "\u{1f30a}",
        "\u{1f393}",
        "\u{1f386}",
        "\u{1f383}",
        "\u{1f47b}",
        "\u{1f385}",
        "\u{1f384}",
        "\u{1f381}",
        "\u{1f388}",
        "\u{1f52e}",
        "\u{1f3a5}",
        "\u{1f4f7}",
        "\u{1f4bf}",
        "\u{1f4bb}",
        "\u{260e}",
        "\u{1f4e1}",
        "\u{1f4fa}",
        "\u{1f4fb}",
        "\u{1f509}",
        "\u{1f514}",
        "\u{23f3}",
        "\u{23f0}",
        "\u{231a}",
        "\u{1f512}",
        "\u{1f511}",
        "\u{1f50e}",
        "\u{1f4a1}",
        "\u{1f526}",
        "\u{1f50c}",
        "\u{1f50b}",
        "\u{1f6bf}",
        "\u{1f6bd}",
        "\u{1f527}",
        "\u{1f528}",
        "\u{1f6aa}",
        "\u{1f6ac}",
        "\u{1f4a3}",
        "\u{1f52b}",
        "\u{1f52a}",
        "\u{1f48a}",
        "\u{1f489}",
        "\u{1f4b0}",
        "\u{1f4b5}",
        "\u{1f4b3}",
        "\u{2709}",
        "\u{1f4eb}",
        "\u{1f4e6}",
        "\u{1f4c5}",
        "\u{1f4c1}",
        "\u{2702}",
        "\u{1f4cc}",
        "\u{1f4ce}",
        "\u{2712}",
        "\u{270f}",
        "\u{1f4d0}",
        "\u{1f4da}",
        "\u{1f52c}",
        "\u{1f52d}",
        "\u{1f3a8}",
        "\u{1f3ac}",
        "\u{1f3a4}",
        "\u{1f3a7}",
        "\u{1f3b5}",
        "\u{1f3b9}",
        "\u{1f3bb}",
        "\u{1f3ba}",
        "\u{1f3b8}",
        "\u{1f47e}",
        "\u{1f3ae}",
        "\u{1f0cf}",
        "\u{1f3b2}",
        "\u{1f3af}",
        "\u{1f3c8}",
        "\u{1f3c0}",
        "\u{26bd}",
        "\u{26be}",
        "\u{1f3be}",
        "\u{1f3b1}",
        "\u{1f3c9}",
        "\u{1f3b3}",
        "\u{1f3c1}",
        "\u{1f3c7}",
        "\u{1f3c6}",
        "\u{1f3ca}",
        "\u{1f3c4}",
        "\u{2615}",
        "\u{1f37c}",
        "\u{1f37a}",
        "\u{1f377}",
        "\u{1f374}",
        "\u{1f355}",
        "\u{1f354}",
        "\u{1f35f}",
        "\u{1f357}",
        "\u{1f371}",
        "\u{1f35a}",
        "\u{1f35c}",
        "\u{1f361}",
        "\u{1f373}",
        "\u{1f35e}",
        "\u{1f369}",
        "\u{1f366}",
        "\u{1f382}",
        "\u{1f370}",
        "\u{1f36a}",
        "\u{1f36b}",
        "\u{1f36d}",
        "\u{1f36f}",
        "\u{1f34e}",
        "\u{1f34f}",
        "\u{1f34a}",
        "\u{1f34b}",
        "\u{1f352}",
        "\u{1f347}",
        "\u{1f349}",
        "\u{1f353}",
        "\u{1f351}",
        "\u{1f34c}",
        "\u{1f350}",
        "\u{1f34d}",
        "\u{1f346}",
        "\u{1f345}",
        "\u{1f33d}",
        "\u{1f3e1}",
        "\u{1f3e5}",
        "\u{1f3e6}",
        "\u{26ea}",
        "\u{1f3f0}",
        "\u{26fa}",
        "\u{1f3ed}",
        "\u{1f5fb}",
        "\u{1f5fd}",
        "\u{1f3a0}",
        "\u{1f3a1}",
        "\u{26f2}",
        "\u{1f3a2}",
        "\u{1f6a2}",
        "\u{1f6a4}",
        "\u{2693}",
        "\u{1f680}",
        "\u{2708}",
        "\u{1f681}",
        "\u{1f682}",
        "\u{1f68b}",
        "\u{1f68e}",
        "\u{1f68c}",
        "\u{1f699}",
        "\u{1f697}",
        "\u{1f695}",
        "\u{1f69b}",
        "\u{1f6a8}",
        "\u{1f694}",
        "\u{1f692}",
        "\u{1f691}",
        "\u{1f6b2}",
        "\u{1f6a0}",
        "\u{1f69c}",
        "\u{1f6a6}",
        "\u{26a0}",
        "\u{1f6a7}",
        "\u{26fd}",
        "\u{1f3b0}",
        "\u{1f5ff}",
        "\u{1f3aa}",
        "\u{1f3ad}",
        "\u{1f1ef}\u{1f1f5}",
        "\u{1f1f0}\u{1f1f7}",
        "\u{1f1e9}\u{1f1ea}",
        "\u{1f1e8}\u{1f1f3}",
        "\u{1f1fa}\u{1f1f8}",
        "\u{1f1eb}\u{1f1f7}",
        "\u{1f1ea}\u{1f1f8}",
        "\u{1f1ee}\u{1f1f9}",
        "\u{1f1f7}\u{1f1fa}",
        "\u{1f1ec}\u{1f1e7}",
        "\u{0031}\u{20e3}",
        "\u{0032}\u{20e3}",
        "\u{0033}\u{20e3}",
        "\u{0034}\u{20e3}",
        "\u{0035}\u{20e3}",
        "\u{0036}\u{20e3}",
        "\u{0037}\u{20e3}",
        "\u{0038}\u{20e3}",
        "\u{0039}\u{20e3}",
        "\u{0030}\u{20e3}",
        "\u{1f51f}",
        "\u{2757}",
        "\u{2753}",
        "\u{2665}",
        "\u{2666}",
        "\u{1f4af}",
        "\u{1f517}",
        "\u{1f531}",
        "\u{1f534}",
        "\u{1f535}",
        "\u{1f536}",
        "\u{1f537}",
    ];

    // Use only the lower 63 bits (avoid sign bit issues)
    let index = (num & 0x7FFFFFFFFFFFFFFF) as usize % EMOJIS.len();
    EMOJIS[index].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_md5_string_hash() {
        let hash1 = get_md5_string_hash("test");
        let hash2 = get_md5_string_hash("test");
        assert_eq!(hash1, hash2);

        let hash3 = get_md5_string_hash("different");
        assert_ne!(hash1, hash3);

        // Verify it's deterministic
        assert_eq!(get_md5_string_hash("hello"), get_md5_string_hash("hello"));
    }

    #[test]
    fn test_get_vector_hash() {
        let vec1 = vec![1u64, 2, 3, 4, 5];
        let hash1 = get_vector_hash(&vec1);
        assert_eq!(hash1, get_vector_hash(&vec1));

        let vec2 = vec![1u64, 2, 3, 4, 6];
        assert_ne!(hash1, get_vector_hash(&vec2));

        // Order matters
        let vec3 = vec![5u64, 4, 3, 2, 1];
        assert_ne!(hash1, get_vector_hash(&vec3));
    }

    #[test]
    fn test_get_vector_hash_empty() {
        let hash = get_vector_hash(&[]);
        assert_eq!(hash, 0);
    }

    #[test]
    fn test_get_vector_hash_single() {
        let hash = get_vector_hash(&[42u64]);
        assert_eq!(hash, 42);
    }

    #[test]
    fn test_get_emoji_fingerprints() {
        let buffer = [0u8; 32];
        let emojis = get_emoji_fingerprints(&buffer);
        assert_eq!(emojis.len(), 4);

        // All emojis should be the same for a zero buffer
        assert_eq!(emojis[0], emojis[1]);
        assert_eq!(emojis[1], emojis[2]);
        assert_eq!(emojis[2], emojis[3]);
    }

    #[test]
    #[should_panic(expected = "Buffer must be exactly 32 bytes")]
    fn test_get_emoji_fingerprints_wrong_length() {
        let buffer = [0u8; 16];
        let _ = get_emoji_fingerprints(&buffer);
    }

    #[test]
    fn test_get_emoji_fingerprint_deterministic() {
        let emoji1 = get_emoji_fingerprint(12345);
        let emoji2 = get_emoji_fingerprint(12345);
        assert_eq!(emoji1, emoji2);
    }

    #[test]
    fn test_get_emoji_fingerprint_different_values() {
        let _emoji1 = get_emoji_fingerprint(0);
        let _emoji2 = get_emoji_fingerprint(u64::MAX);
        // With high probability, these should be different
        // (unless the emoji list has length 1 or we're extremely unlucky)
    }
}
