// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Set With Position
//!
//! Generic container with position tracking for Telegram.
//!
//! Based on TDLib's `SetWithPosition` from `td/telegram/SetWithPosition.h`.
//!
//! # Overview
//!
//! A `SetWithPosition` is a set-like container that tracks position for iteration.
//! It optimizes for the common case of having 0-1 elements, and upgrades to a
//! more complex data structure when needed.
//!
//! # Example
//!
//! ```rust
//! use rustgram_set_with_position::SetWithPosition;
//!
//! let mut set = SetWithPosition::new();
//! set.add(1);
//! set.add(2);
//! assert!(set.has_next());
//! assert_eq!(set.next(), 1);
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

/// Set with position tracking.
///
/// A set-like container that tracks which elements have been visited.
/// Optimized for the common case of 0-1 elements.
///
/// # TDLib Mapping
///
/// - `SetWithPosition::new()` → TDLib: `SetWithPosition()`
/// - `add(value)` → TDLib: `add(T x)`
/// - `next()` → TDLib: `next()` returns checked element and advances position
///
/// # Example
///
/// ```rust
/// use rustgram_set_with_position::SetWithPosition;
///
/// let mut set = SetWithPosition::new();
/// assert!(set.add(1));
/// assert!(!set.add(1)); // Duplicate
/// assert!(set.has_next());
/// assert_eq!(set.next(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetWithPosition<T>
where
    T: Ord + Clone + fmt::Debug,
{
    /// The value (for 0-1 element case)
    value: Option<T>,
    /// Whether the single value has been checked
    is_checked: bool,
    /// Fast set for multiple elements
    fast: FastSetWithPosition<T>,
}

/// Fast set for multiple elements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FastSetWithPosition<T>
where
    T: Ord + Clone + fmt::Debug,
{
    /// Checked elements
    checked: BTreeSet<T>,
    /// Not yet checked elements
    not_checked: BTreeSet<T>,
}

impl<T> FastSetWithPosition<T>
where
    T: Ord + Clone + fmt::Debug,
{
    fn new() -> Self {
        Self {
            checked: BTreeSet::new(),
            not_checked: BTreeSet::new(),
        }
    }

    fn add(&mut self, x: T) -> bool {
        if self.checked.contains(&x) {
            return false;
        }
        self.not_checked.insert(x)
    }

    fn remove(&mut self, x: &T) -> bool {
        self.checked.remove(x) || self.not_checked.remove(x)
    }

    fn has_next(&self) -> bool {
        !self.not_checked.is_empty()
    }

    fn reset_position(&mut self) {
        if self.not_checked.is_empty() {
            std::mem::swap(&mut self.checked, &mut self.not_checked);
        } else {
            self.not_checked.append(&mut self.checked);
        }
    }

    fn next(&mut self) -> Option<T> {
        if !self.has_next() {
            return None;
        }
        let first = self.not_checked.iter().next().cloned()?;
        self.not_checked.remove(&first);
        self.checked.insert(first.clone());
        Some(first)
    }

    fn merge(&mut self, other: Self) {
        for x in other.checked {
            self.not_checked.remove(&x);
            self.checked.insert(x);
        }
        for x in other.not_checked {
            if self.checked.contains(&x) {
                continue;
            }
            self.not_checked.insert(x);
        }
    }

    fn size(&self) -> usize {
        self.checked.len() + self.not_checked.len()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

impl<T> Default for FastSetWithPosition<T>
where
    T: Ord + Clone + fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SetWithPosition<T>
where
    T: Ord + Clone + fmt::Debug,
{
    /// Creates a new empty set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let set: SetWithPosition<i32> = SetWithPosition::new();
    /// assert!(set.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: None,
            is_checked: false,
            fast: FastSetWithPosition::new(),
        }
    }

    /// Adds a value to the set.
    ///
    /// Returns `true` if the value was added, `false` if it was already present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set = SetWithPosition::new();
    /// assert!(set.add(1));
    /// assert!(!set.add(1)); // Duplicate
    /// assert!(set.add(2));
    /// ```
    pub fn add(&mut self, x: T) -> bool {
        if self.fast.is_empty() {
            if self.value.is_none() {
                self.value = Some(x);
                self.is_checked = false;
                return true;
            }
            if self.value.as_ref() == Some(&x) {
                return false;
            }
            // Upgrade to fast set
            self.make_fast();
            return self.fast.add(x);
        }
        self.fast.add(x)
    }

    /// Removes a value from the set.
    ///
    /// Returns `true` if the value was removed, `false` if it was not present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set = SetWithPosition::new();
    /// set.add(1);
    /// assert!(set.remove(&1));
    /// assert!(!set.remove(&1));
    /// ```
    pub fn remove(&mut self, x: &T) -> bool {
        if self.fast.is_empty() {
            if self.value.as_ref() == Some(x) {
                self.value = None;
                self.is_checked = false;
                return true;
            }
            return false;
        }
        self.fast.remove(x)
    }

    /// Checks if there are more elements to iterate.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set = SetWithPosition::new();
    /// assert!(!set.has_next());
    /// set.add(1);
    /// assert!(set.has_next());
    /// ```
    #[must_use]
    pub fn has_next(&self) -> bool {
        if self.fast.is_empty() {
            self.value.is_some() && !self.is_checked
        } else {
            self.fast.has_next()
        }
    }

    /// Resets the iteration position.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set = SetWithPosition::new();
    /// set.add(1);
    /// set.next(); // Consumes 1
    /// assert!(!set.has_next());
    /// set.reset_position();
    /// assert!(set.has_next());
    /// ```
    pub fn reset_position(&mut self) {
        if self.fast.is_empty() {
            self.is_checked = false;
        } else {
            self.fast.reset_position();
        }
    }

    /// Returns the next unchecked element and marks it as checked.
    ///
    /// Returns `None` if all elements have been checked.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set = SetWithPosition::new();
    /// set.add(1);
    /// set.add(2);
    /// assert_eq!(set.next(), Some(1));
    /// assert_eq!(set.next(), Some(2));
    /// assert_eq!(set.next(), None);
    /// ```
    pub fn next(&mut self) -> Option<T> {
        if !self.has_next() {
            return None;
        }
        if self.fast.is_empty() {
            self.is_checked = true;
            self.value.clone()
        } else {
            self.fast.next()
        }
    }

    /// Merges another set into this one.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set1 = SetWithPosition::new();
    /// set1.add(1);
    ///
    /// let mut set2 = SetWithPosition::new();
    /// set2.add(2);
    ///
    /// set1.merge(set2);
    /// assert_eq!(set1.size(), 2);
    /// ```
    pub fn merge(&mut self, mut other: Self) {
        if self.fast.is_empty() && other.fast.is_empty() {
            if self.value == other.value {
                self.is_checked |= other.is_checked;
            } else if let Some(v) = other.value {
                self.add(v);
            }
            return;
        }
        self.make_fast();
        other.make_fast();
        // Swap the fast sets to merge them
        std::mem::swap(&mut self.fast, &mut other.fast);
        self.fast.merge(other.fast);
    }

    /// Returns the number of elements in the set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set = SetWithPosition::new();
    /// assert_eq!(set.size(), 0);
    /// set.add(1);
    /// assert_eq!(set.size(), 1);
    /// set.add(2);
    /// assert_eq!(set.size(), 2);
    /// ```
    #[must_use]
    pub fn size(&self) -> usize {
        if self.fast.is_empty() {
            usize::from(self.value.is_some())
        } else {
            self.fast.size()
        }
    }

    /// Checks if the set is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_set_with_position::SetWithPosition;
    ///
    /// let mut set = SetWithPosition::new();
    /// assert!(set.is_empty());
    /// set.add(1);
    /// assert!(!set.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Converts to the fast representation.
    fn make_fast(&mut self) {
        if self.fast.is_empty() && self.value.is_some() {
            let v = self.value.take().unwrap();
            if !self.is_checked {
                self.fast.not_checked.insert(v);
            } else {
                self.fast.checked.insert(v);
            }
            self.is_checked = false;
        }
    }
}

impl<T> Default for SetWithPosition<T>
where
    T: Ord + Clone + fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set: SetWithPosition<i32> = SetWithPosition::new();
        assert!(set.is_empty());
        assert_eq!(set.size(), 0);
        assert!(!set.has_next());
    }

    #[test]
    fn test_default() {
        let set: SetWithPosition<i32> = SetWithPosition::default();
        assert!(set.is_empty());
    }

    #[test]
    fn test_add_single() {
        let mut set = SetWithPosition::new();
        assert!(set.add(1));
        assert!(!set.is_empty());
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_add_duplicate() {
        let mut set = SetWithPosition::new();
        assert!(set.add(1));
        assert!(!set.add(1));
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_add_multiple() {
        let mut set = SetWithPosition::new();
        assert!(set.add(1));
        assert!(set.add(2));
        assert!(set.add(3));
        assert_eq!(set.size(), 3);
    }

    #[test]
    fn test_remove() {
        let mut set = SetWithPosition::new();
        set.add(1);
        assert!(set.remove(&1));
        assert!(!set.remove(&1));
        assert!(set.is_empty());
    }

    #[test]
    fn test_has_next() {
        let mut set = SetWithPosition::new();
        assert!(!set.has_next());
        set.add(1);
        assert!(set.has_next());
        set.next();
        assert!(!set.has_next());
    }

    #[test]
    fn test_next() {
        let mut set = SetWithPosition::new();
        set.add(1);
        assert_eq!(set.next(), Some(1));
        assert_eq!(set.next(), None);
    }

    #[test]
    fn test_next_multiple() {
        let mut set = SetWithPosition::new();
        set.add(1);
        set.add(2);
        set.add(3);
        let mut results = Vec::new();
        while let Some(val) = set.next() {
            results.push(val);
        }
        assert_eq!(results.len(), 3);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
        assert!(results.contains(&3));
    }

    #[test]
    fn test_reset_position() {
        let mut set = SetWithPosition::new();
        set.add(1);
        set.next();
        assert!(!set.has_next());
        set.reset_position();
        assert!(set.has_next());
        assert_eq!(set.next(), Some(1));
    }

    #[test]
    fn test_merge_single() {
        let mut set1 = SetWithPosition::new();
        set1.add(1);

        let mut set2 = SetWithPosition::new();
        set2.add(2);

        set1.merge(set2);
        assert_eq!(set1.size(), 2);
    }

    #[test]
    fn test_merge_duplicate() {
        let mut set1 = SetWithPosition::new();
        set1.add(1);

        let mut set2 = SetWithPosition::new();
        set2.add(1);

        set1.merge(set2);
        assert_eq!(set1.size(), 1);
    }

    #[test]
    fn test_size() {
        let mut set = SetWithPosition::new();
        assert_eq!(set.size(), 0);
        set.add(1);
        assert_eq!(set.size(), 1);
        set.add(2);
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut set = SetWithPosition::new();
        assert!(set.is_empty());
        set.add(1);
        assert!(!set.is_empty());
    }

    #[test]
    fn test_iteration_order() {
        let mut set = SetWithPosition::new();
        set.add(3);
        set.add(1);
        set.add(2);

        let mut results = Vec::new();
        while let Some(val) = set.next() {
            results.push(val);
        }
        // BTreeSet iterates in sorted order
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_clone() {
        let mut set1 = SetWithPosition::new();
        set1.add(1);
        set1.add(2);

        let set2 = set1.clone();
        assert_eq!(set1.size(), set2.size());
    }

    #[test]
    fn test_equality() {
        let mut set1 = SetWithPosition::new();
        set1.add(1);
        set1.add(2);

        let mut set2 = SetWithPosition::new();
        set2.add(1);
        set2.add(2);

        assert_eq!(set1, set2);
    }

    #[test]
    fn test_serialization() {
        let mut set = SetWithPosition::new();
        set.add(1);
        set.add(2);

        let json = serde_json::to_string(&set).expect("Failed to serialize");
        let deserialized: SetWithPosition<i32> =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.size(), 2);
    }

    #[test]
    fn test_string_set() {
        let mut set = SetWithPosition::new();
        assert!(set.add("hello"));
        assert!(set.add("world"));
        assert!(!set.add("hello"));
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_checked_state_preserved() {
        let mut set = SetWithPosition::new();
        set.add(1);
        set.next();
        assert!(!set.has_next());

        // Add more elements - should preserve checked state
        set.add(2);
        set.add(3);
        // Now we should have 2 more unchecked elements
        let mut count = 0;
        while set.next().is_some() {
            count += 1;
        }
        assert_eq!(count, 2);
    }
}
