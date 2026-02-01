// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Buffer Types
//!
//! Buffer types for TDLib compatibility.
//!
//! # TODO
//!
//! This is a stub implementation providing minimal functionality for type compatibility.
//! Full buffer implementation with efficient slicing and memory management is needed for production.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;

/// A slice of buffer data.
///
/// This is a stub for TDLib BufferSlice compatibility.
/// A full implementation with efficient slicing is needed for production.
///
/// # TODO
//!
/// Implement efficient buffer slicing with:
/// - Reference counting
/// - Zero-copy operations
/// - Memory pooling
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BufferSlice {
    /// The underlying data
    pub data: Vec<u8>,
}

impl BufferSlice {
    /// Creates a new BufferSlice from a Vec<u8>.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to wrap
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::new(vec![1, 2, 3, 4]);
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Creates an empty BufferSlice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::empty();
    /// assert!(slice.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Creates a BufferSlice from a slice.
    ///
    /// # Arguments
    ///
    /// * `slice` - The slice to copy
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::from_slice(&[1, 2, 3, 4]);
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            data: slice.to_vec(),
        }
    }

    /// Returns the length of the buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::new(vec![1, 2, 3, 4]);
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the buffer is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// assert!(BufferSlice::empty().is_empty());
    /// assert!(!BufferSlice::new(vec![1]).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a reference to the underlying data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::new(vec![1, 2, 3, 4]);
    /// assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    /// ```
    pub fn as_ref(&self) -> &[u8] {
        &self.data
    }

    /// Consumes the BufferSlice and returns the underlying Vec<u8>.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::new(vec![1, 2, 3, 4]);
    /// let vec = slice.into_vec();
    /// assert_eq!(vec, vec![1, 2, 3, 4]);
    /// ```
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    /// Creates a BufferSlice from static bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Static byte slice
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::from_static(b"hello");
    /// assert_eq!(slice.len(), 5);
    /// ```
    pub fn from_static(bytes: &[u8]) -> Self {
        Self {
            data: bytes.to_vec(),
        }
    }

    /// Creates a BufferSlice filled with zeros.
    ///
    /// # Arguments
    ///
    /// * `len` - Length of the buffer
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice = BufferSlice::zero(10);
    /// assert_eq!(slice.len(), 10);
    /// assert!(slice.data.iter().all(|&b| b == 0));
    /// ```
    pub fn zero(len: usize) -> Self {
        Self {
            data: vec![0u8; len],
        }
    }

    /// Concatenates two BufferSlices.
    ///
    /// # Arguments
    ///
    /// * `other` - The other BufferSlice to concatenate
    ///
    /// # Returns
    ///
    /// Returns a new BufferSlice containing the concatenated data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let slice1 = BufferSlice::new(vec![1, 2]);
    /// let slice2 = BufferSlice::new(vec![3, 4]);
    /// let combined = slice1.concat(&slice2);
    /// assert_eq!(combined.as_ref(), &[1, 2, 3, 4]);
    /// ```
    pub fn concat(&self, other: &Self) -> Self {
        let mut data = self.data.clone();
        data.extend_from_slice(&other.data);
        Self { data }
    }
}

impl Default for BufferSlice {
    fn default() -> Self {
        Self::empty()
    }
}

impl Deref for BufferSlice {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl AsRef<[u8]> for BufferSlice {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Debug for BufferSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufferSlice")
            .field("len", &self.data.len())
            .finish()
    }
}

impl From<Vec<u8>> for BufferSlice {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl From<&[u8]> for BufferSlice {
    fn from(slice: &[u8]) -> Self {
        Self {
            data: slice.to_vec(),
        }
    }
}

impl<'a> From<&'a [u8]> for BufferSlice {
    fn from(slice: &'a [u8]) -> Self {
        Self {
            data: slice.to_vec(),
        }
    }
}

/// A slice type for view into buffer data.
///
/// This is a stub for TDLib Slice compatibility.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Slice<'a> {
    /// The slice data
    pub data: &'a [u8],
}

impl<'a> Slice<'a> {
    /// Creates a new Slice from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `data` - The slice data
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::Slice;
    ///
    /// let data = vec![1, 2, 3, 4];
    /// let slice = Slice::new(&data);
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Creates an empty Slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::Slice;
    ///
    /// let slice = Slice::empty();
    /// assert!(slice.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self { data: &[] }
    }

    /// Returns the length of the slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::Slice;
    ///
    /// let data = vec![1, 2, 3, 4];
    /// let slice = Slice::new(&data);
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the slice is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::Slice;
    ///
    /// assert!(Slice::empty().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the underlying slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_buffer::Slice;
    ///
    /// let data = vec![1, 2, 3, 4];
    /// let slice = Slice::new(&data);
    /// assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    /// ```
    pub fn as_ref(&self) -> &'a [u8] {
        self.data
    }
}

impl<'a> Default for Slice<'a> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a> Deref for Slice<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a> AsRef<[u8]> for Slice<'a> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

impl<'a> Debug for Slice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Slice")
            .field("len", &self.data.len())
            .finish()
    }
}

impl<'a> From<&'a [u8]> for Slice<'a> {
    fn from(data: &'a [u8]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BufferSlice tests
    #[test]
    fn test_buffer_slice_new() {
        let slice = BufferSlice::new(vec![1, 2, 3, 4]);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_buffer_slice_empty() {
        let slice = BufferSlice::empty();
        assert!(slice.is_empty());
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn test_buffer_slice_from_slice() {
        let slice = BufferSlice::from_slice(&[1, 2, 3, 4]);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_buffer_slice_len() {
        let empty = BufferSlice::empty();
        assert_eq!(empty.len(), 0);

        let single = BufferSlice::new(vec![1]);
        assert_eq!(single.len(), 1);

        let multiple = BufferSlice::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(multiple.len(), 5);
    }

    #[test]
    fn test_buffer_slice_is_empty() {
        assert!(BufferSlice::empty().is_empty());
        assert!(!BufferSlice::new(vec![1]).is_empty());
        assert!(!BufferSlice::new(vec![1, 2, 3]).is_empty());
    }

    #[test]
    fn test_buffer_slice_into_vec() {
        let slice = BufferSlice::new(vec![1, 2, 3, 4]);
        let vec = slice.into_vec();
        assert_eq!(vec, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_buffer_slice_from_static() {
        let slice = BufferSlice::from_static(b"hello");
        assert_eq!(slice.len(), 5);
        assert_eq!(slice.as_ref(), b"hello");
    }

    #[test]
    fn test_buffer_slice_zero() {
        let slice = BufferSlice::zero(10);
        assert_eq!(slice.len(), 10);
        assert!(slice.data.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_buffer_slice_concat() {
        let slice1 = BufferSlice::new(vec![1, 2]);
        let slice2 = BufferSlice::new(vec![3, 4]);
        let combined = slice1.concat(&slice2);
        assert_eq!(combined.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_buffer_slice_clone() {
        let slice1 = BufferSlice::new(vec![1, 2, 3]);
        let slice2 = slice1.clone();
        assert_eq!(slice1, slice2);
    }

    #[test]
    fn test_buffer_slice_equality() {
        let slice1 = BufferSlice::new(vec![1, 2, 3]);
        let slice2 = BufferSlice::new(vec![1, 2, 3]);
        let slice3 = BufferSlice::new(vec![4, 5, 6]);
        assert_eq!(slice1, slice2);
        assert_ne!(slice1, slice3);
    }

    #[test]
    fn test_buffer_slice_default() {
        let slice = BufferSlice::default();
        assert!(slice.is_empty());
    }

    #[test]
    fn test_buffer_slice_debug() {
        let slice = BufferSlice::new(vec![1, 2, 3]);
        let debug_str = format!("{:?}", slice);
        assert!(debug_str.contains("BufferSlice"));
        assert!(debug_str.contains("len"));
        assert!(debug_str.contains("3"));
    }

    #[test]
    fn test_buffer_slice_from_vec() {
        let vec = vec![1, 2, 3, 4];
        let slice = BufferSlice::from(vec);
        assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_buffer_slice_from_byte_slice() {
        let data: &[u8] = &[1, 2, 3, 4];
        let slice = BufferSlice::from(data);
        assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_buffer_slice_deref() {
        let slice = BufferSlice::new(vec![1, 2, 3, 4]);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice[0], 1);
        assert_eq!(slice[3], 4);
    }

    #[test]
    fn test_buffer_slice_as_ref_trait() {
        let slice = BufferSlice::new(vec![1, 2, 3, 4]);
        let bytes: &[u8] = slice.as_ref();
        assert_eq!(bytes, &[1, 2, 3, 4]);
    }

    // Slice tests
    #[test]
    fn test_slice_new() {
        let data = vec![1, 2, 3, 4];
        let slice = Slice::new(&data);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_slice_empty() {
        let slice = Slice::empty();
        assert!(slice.is_empty());
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn test_slice_len() {
        let data = vec![1, 2, 3, 4, 5];
        let slice = Slice::new(&data);
        assert_eq!(slice.len(), 5);
    }

    #[test]
    fn test_slice_is_empty() {
        assert!(Slice::empty().is_empty());

        let data = vec![1];
        let slice = Slice::new(&data);
        assert!(!slice.is_empty());
    }

    #[test]
    fn test_slice_as_ref() {
        let data = vec![1, 2, 3];
        let slice = Slice::new(&data);
        assert_eq!(slice.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn test_slice_default() {
        let slice = Slice::default();
        assert!(slice.is_empty());
    }

    #[test]
    fn test_slice_debug() {
        let data = vec![1, 2, 3];
        let slice = Slice::new(&data);
        let debug_str = format!("{:?}", slice);
        assert!(debug_str.contains("Slice"));
        assert!(debug_str.contains("len"));
        assert!(debug_str.contains("3"));
    }

    #[test]
    fn test_slice_from_byte_slice() {
        let data: &[u8] = &[1, 2, 3, 4];
        let slice = Slice::from(data);
        assert_eq!(slice.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_slice_deref() {
        let data = vec![1, 2, 3, 4];
        let slice = Slice::new(&data);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice[0], 1);
        assert_eq!(slice[3], 4);
    }

    #[test]
    fn test_slice_clone() {
        let data = vec![1, 2, 3];
        let slice1 = Slice::new(&data);
        let slice2 = slice1;
        assert_eq!(slice1, slice2);
    }

    #[test]
    fn test_slice_equality() {
        let data = vec![1, 2, 3];
        let slice1 = Slice::new(&data);
        let slice2 = Slice::new(&data);
        assert_eq!(slice1, slice2);
    }

    // Combined tests
    #[test]
    fn test_buffer_slice_and_slice_compatibility() {
        let data = vec![1, 2, 3, 4];
        let buffer_slice = BufferSlice::from_slice(&data);
        let slice = Slice::new(&data);

        assert_eq!(buffer_slice.as_ref(), slice.as_ref());
    }

    #[test]
    fn test_buffer_slice_empty_and_default() {
        let empty = BufferSlice::empty();
        let default = BufferSlice::default();
        assert_eq!(empty, default);
    }

    #[test]
    fn test_slice_empty_and_default() {
        let empty = Slice::empty();
        let default = Slice::default();
        assert_eq!(empty.as_ref(), default.as_ref());
    }
}
