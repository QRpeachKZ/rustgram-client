//! Vector and collection types for TL schema.
//!
//! This module provides types representing collections in the TL schema,
//! including the Vector type and Maybe type for optional fields.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

/// TL Vector type.
///
/// In MTProto TL, vectors have a constructor ID of 0x1cb5c415 and contain
/// elements of a uniform type.
///
/// # Example
/// ```rust
/// use rustgram_types::TlVector;
///
/// let mut vec = TlVector::new();
/// vec.push(1);
/// vec.push(2);
/// vec.push(3);
/// assert_eq!(vec.len(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct TlVector<T> {
    inner: Vec<T>,
}

impl<T> TlVector<T> {
    /// Constructor ID for vector type.
    pub const CONSTRUCTOR_ID: u32 = 0x1cb5c415;

    /// Creates a new empty TL vector.
    #[inline]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Creates a new TL vector with the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// Creates a TL vector from a Vec.
    #[inline]
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { inner: vec }
    }

    /// Pushes a value into the vector.
    #[inline]
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    /// Pops a value from the vector.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    /// Returns an iterator over the values.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.inner.iter()
    }

    /// Returns a mutable iterator over the values.
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.inner.iter_mut()
    }

    /// Returns the inner Vec, consuming self.
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }

    /// Returns a reference to the inner Vec.
    #[inline]
    pub fn as_inner(&self) -> &Vec<T> {
        &self.inner
    }

    /// Clears the vector, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns the number of elements in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the vector is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Reserves capacity for at least `additional` more elements.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }
}

impl<T> Default for TlVector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for TlVector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TlVector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<[T]> for TlVector<T> {
    fn as_ref(&self) -> &[T] {
        &self.inner
    }
}

impl<T> AsMut<[T]> for TlVector<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.inner
    }
}

impl<T: Clone> From<&[T]> for TlVector<T> {
    fn from(slice: &[T]) -> Self {
        Self {
            inner: slice.to_vec(),
        }
    }
}

impl<T> From<Vec<T>> for TlVector<T> {
    fn from(vec: Vec<T>) -> Self {
        Self { inner: vec }
    }
}

impl<T> From<TlVector<T>> for Vec<T> {
    fn from(vec: TlVector<T>) -> Self {
        vec.inner
    }
}

impl<T> IntoIterator for TlVector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a TlVector<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<T> FromIterator<T> for TlVector<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            inner: Vec::from_iter(iter),
        }
    }
}

impl<T: PartialEq> PartialEq for TlVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for TlVector<T> {}

impl<T: Hash> Hash for TlVector<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T: fmt::Display> fmt::Display for TlVector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.inner.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<T: Serialize> Serialize for TlVector<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // In MTProto, vectors serialize with their constructor ID first
        // For JSON/other formats, just serialize as a regular array
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.inner.len()))?;
        for item in &self.inner {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for TlVector<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VectorVisitor<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for VectorVisitor<T> {
            type Value = TlVector<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(item) = seq.next_element()? {
                    vec.push(item);
                }
                Ok(TlVector { inner: vec })
            }
        }

        deserializer.deserialize_seq(VectorVisitor {
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Maybe type for optional fields in TL schema.
///
/// In MTProto, optional fields are represented using the Maybe type:
/// - Maybe some value: 0x3f9c8ef8 (with the value)
/// - Maybe none: 0x27930a7b (empty)
///
/// This enum provides a Rust-friendly representation of that concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maybe<T> {
    /// Some value is present.
    Some(T),
    /// No value (None).
    None,
}

impl<T> Maybe<T> {
    /// Constructor ID for Maybe.Some (true).
    pub const SOME_ID: u32 = 0x3f9c8ef8;

    /// Constructor ID for Maybe.None (false).
    pub const NONE_ID: u32 = 0x27930a7b;

    /// Returns true if this is Some.
    #[inline]
    pub const fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    /// Returns true if this is None.
    #[inline]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns the contained value, if present.
    #[inline]
    pub const fn as_ref(&self) -> Maybe<&T> {
        match self {
            Self::Some(v) => Maybe::Some(v),
            Self::None => Maybe::None,
        }
    }

    /// Converts from Maybe<T> to Option<T>.
    #[inline]
    pub fn to_option(self) -> Option<T> {
        match self {
            Self::Some(v) => Some(v),
            Self::None => None,
        }
    }

    /// Converts from Option<T> to Maybe<T>.
    #[inline]
    pub fn from_option(opt: Option<T>) -> Self {
        match opt {
            Some(v) => Self::Some(v),
            None => Self::None,
        }
    }

    /// Returns the contained value, or a default.
    #[inline]
    pub fn unwrap_or(self, default: T) -> T
    where
        T: Clone,
    {
        match self {
            Self::Some(v) => v,
            Self::None => default,
        }
    }

    /// Returns the contained value, or computes a default.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Self::Some(v) => v,
            Self::None => f(),
        }
    }

    /// Maps the contained value with a function.
    #[inline]
    pub fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(v) => Maybe::Some(f(v)),
            Self::None => Maybe::None,
        }
    }
}

impl<T> Default for Maybe<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(opt: Option<T>) -> Self {
        Self::from_option(opt)
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(maybe: Maybe<T>) -> Self {
        maybe.to_option()
    }
}

impl<T: fmt::Display> fmt::Display for Maybe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Some(v) => write!(f, "Some({v})"),
            Self::None => write!(f, "None"),
        }
    }
}

impl<T: Serialize> Serialize for Maybe<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Some(v) => serializer.serialize_some(v),
            Self::None => serializer.serialize_none(),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Maybe<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Maybe::from_option)
    }
}

/// Dictionary type for TL schema.
///
/// In MTProto, dictionaries are represented as vectors of key-value pairs.
/// This type provides a more idiomatic Rust interface.
#[derive(Debug, Clone)]
pub struct TlDictionary<K, V> {
    inner: Vec<(K, V)>,
}

impl<K, V> TlDictionary<K, V> {
    /// Constructor ID for dictionary type.
    pub const CONSTRUCTOR_ID: u32 = 0x1f4c618f;

    /// Creates a new empty dictionary.
    #[inline]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Creates a new dictionary with the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// Inserts a key-value pair into the dictionary.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        self.inner.push((key, value));
    }

    /// Returns an iterator over the key-value pairs.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<(K, V)> {
        self.inner.iter()
    }

    /// Returns a mutable iterator over the key-value pairs.
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<(K, V)> {
        self.inner.iter_mut()
    }

    /// Returns the number of entries in the dictionary.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the dictionary is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clears the dictionary, removing all entries.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<K, V> Default for TlDictionary<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for TlDictionary<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K: Eq, V: Eq> Eq for TlDictionary<K, V> {}

impl<K, V> IntoIterator for TlDictionary<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K: Serialize, V: Serialize> Serialize for TlDictionary<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, K: Deserialize<'de>, V: Deserialize<'de>> Deserialize<'de> for TlDictionary<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<(K, V)>::deserialize(deserializer).map(|inner| TlDictionary { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tl_vector() {
        let mut vec = TlVector::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec.pop(), Some(3));
    }

    #[test]
    fn test_tl_vector_from_vec() {
        let vec = TlVector::from_vec(vec![1, 2, 3]);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_maybe() {
        let some = Maybe::Some(42);
        assert!(some.is_some());
        assert_eq!(some.to_option(), Some(42));

        let none: Maybe<i32> = Maybe::None;
        assert!(none.is_none());
        assert_eq!(none.to_option(), None);
    }

    #[test]
    fn test_maybe_from_option() {
        let some: Maybe<i32> = Maybe::from_option(Some(42));
        assert!(some.is_some());

        let none: Maybe<i32> = Maybe::from_option(None);
        assert!(none.is_none());
    }

    #[test]
    fn test_maybe_map() {
        let some = Maybe::Some(42);
        let mapped = some.map(|x| x * 2);
        assert_eq!(mapped.to_option(), Some(84));

        let none: Maybe<i32> = Maybe::None;
        let mapped = none.map(|x| x * 2);
        assert!(mapped.is_none());
    }

    #[test]
    fn test_dictionary() {
        let mut dict = TlDictionary::new();
        dict.insert("key1", 1);
        dict.insert("key2", 2);
        assert_eq!(dict.len(), 2);
    }
}
