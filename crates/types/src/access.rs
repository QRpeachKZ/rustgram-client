//! Access hash types for Telegram.
//!
//! Access hashes are used in MTProto for security validation when accessing
//! users, channels, and other entities that require authentication.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Access hash for entities that require authentication.
///
/// Access hashes are required when accessing certain Telegram entities
/// (channels, users with restricted access, etc.) to prevent unauthorized access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AccessHash(pub i64);

impl AccessHash {
    /// Creates a new access hash.
    #[inline]
    pub const fn new(hash: i64) -> Self {
        Self(hash)
    }

    /// Creates an access hash from u64 (for compatibility).
    #[inline]
    pub const fn from_u64(hash: u64) -> Self {
        Self(hash as i64)
    }

    /// Returns the inner hash value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Returns the hash as u64.
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0 as u64
    }

    /// Checks if this is a valid (non-zero) access hash.
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Hash for AccessHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for AccessHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:016x}", self.0 as u64)
    }
}

impl From<i64> for AccessHash {
    fn from(hash: i64) -> Self {
        Self(hash)
    }
}

impl From<u64> for AccessHash {
    fn from(hash: u64) -> Self {
        Self(hash as i64)
    }
}

impl From<AccessHash> for i64 {
    fn from(hash: AccessHash) -> Self {
        hash.0
    }
}

impl From<AccessHash> for u64 {
    fn from(hash: AccessHash) -> Self {
        hash.0 as u64
    }
}

impl Serialize for AccessHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AccessHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer).map(AccessHash)
    }
}

/// File reference hash for accessing cached files.
///
/// File references are used to validate access to cached files
/// and prevent unauthorized file access.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileReference {
    /// The raw reference bytes.
    bytes: Vec<u8>,
}

impl FileReference {
    /// Creates a new file reference from bytes.
    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Creates an empty file reference.
    #[inline]
    pub fn empty() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Returns the inner bytes.
    #[inline]
    pub fn get(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns true if this reference is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl Default for FileReference {
    fn default() -> Self {
        Self::empty()
    }
}

impl Hash for FileReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl fmt::Display for FileReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bytes.is_empty() {
            write!(f, "<empty>")
        } else {
            write!(f, "<{} bytes>", self.bytes.len())
        }
    }
}

impl From<Vec<u8>> for FileReference {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl From<&[u8]> for FileReference {
    fn from(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }
}

impl AsRef<[u8]> for FileReference {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Serialize for FileReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FileReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<u8>::deserialize(deserializer).map(|bytes| FileReference { bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_hash() {
        let hash = AccessHash::new(0x1234567890abcdef);
        assert!(hash.is_valid());
        assert_eq!(hash.get(), 0x1234567890abcdef);
    }

    #[test]
    fn test_access_hash_from_u64() {
        let hash = AccessHash::from_u64(0x1234567890abcdef);
        assert_eq!(hash.as_u64(), 0x1234567890abcdef);
    }

    #[test]
    fn test_access_hash_zero() {
        let hash = AccessHash::default();
        assert!(!hash.is_valid());
    }

    #[test]
    fn test_file_reference() {
        let bytes = vec![1, 2, 3, 4, 5];
        let ref_ = FileReference::new(bytes.clone());
        assert_eq!(ref_.get(), &bytes[..]);
        assert!(!ref_.is_empty());
    }

    #[test]
    fn test_file_reference_empty() {
        let ref_ = FileReference::empty();
        assert!(ref_.is_empty());
        assert_eq!(ref_.get(), &[] as &[u8]);
    }
}
