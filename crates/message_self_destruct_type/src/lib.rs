// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Message Self-Destruct Type
//!
//! Represents the self-destruct timer for messages.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `MessageSelfDestructType` functionality.
//! It defines the time-to-live (TTL) for messages that automatically delete
//! themselves after a specified period.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_self_destruct_type::MessageSelfDestructType;
//!
//! // Create a 30-second self-destruct timer
//! let timer = MessageSelfDestructType::new(30)?;
//! assert_eq!(timer.ttl(), 30);
//! assert!(timer.is_valid());
//!
//! # Ok::<(), rustgram_message_self_destruct_type::Error>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

/// Errors that can occur when working with message self-destruct types.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Invalid TTL value (must be positive and within valid range).
    #[error("Invalid TTL: {0}")]
    InvalidTtl(String),
}

/// Result type for message self-destruct operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Message self-destruct type.
///
/// Represents the time-to-live for self-destructing messages. Messages will
/// automatically delete themselves after the specified duration.
///
/// # TDLib Correspondence
///
/// - TDLib Type: `MessageSelfDestructType`
/// - Valid TTL values: 0, or values in range [1, 2147483647]
/// - Special value 0x7FFFFFFE represents "immediate" deletion (after viewing)
///
/// # Example
///
/// ```rust
/// use rustgram_message_self_destruct_type::MessageSelfDestructType;
/// use std::time::Duration;
///
/// let timer = MessageSelfDestructType::new(30)?;
/// assert_eq!(timer.ttl(), 30);
/// assert_eq!(timer.duration(), Duration::from_secs(30));
///
/// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageSelfDestructType {
    ttl: i32,
}

impl MessageSelfDestructType {
    /// Maximum valid TTL value.
    pub const MAX_TTL: i32 = i32::MAX;

    /// Special TTL value for immediate deletion (after viewing).
    pub const IMMEDIATE_TTL: i32 = 0x7FFFFFFE;

    /// Creates a new message self-destruct type.
    ///
    /// # Arguments
    ///
    /// * `ttl` - Time-to-live in seconds
    ///
    /// # Returns
    ///
    /// Returns `Error::InvalidTtl` if the TTL is negative.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// let timer = MessageSelfDestructType::new(30)?;
    /// assert_eq!(timer.ttl(), 30);
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    pub fn new(ttl: i32) -> Result<Self> {
        if ttl < 0 {
            return Err(Error::InvalidTtl(format!(
                "TTL must be non-negative, got {ttl}"
            )));
        }
        Ok(Self { ttl })
    }

    /// Creates a new message self-destruct type, allowing immediate deletion.
    ///
    /// # Arguments
    ///
    /// * `ttl` - Time-to-live in seconds (can be 0 for immediate)
    /// * `allow_immediate` - If true, allows TTL = 0 for immediate deletion
    ///
    /// # Returns
    ///
    /// Returns `Error::InvalidTtl` if the TTL is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// // Allow immediate deletion
    /// let immediate = MessageSelfDestructType::new_immediate(0, true)?;
    /// assert!(immediate.is_immediate());
    ///
    /// // Normal TTL
    /// let timer = MessageSelfDestructType::new_immediate(30, true)?;
    /// assert_eq!(timer.ttl(), 30);
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    pub fn new_immediate(ttl: i32, allow_immediate: bool) -> Result<Self> {
        if ttl < 0 {
            return Err(Error::InvalidTtl(format!(
                "TTL must be non-negative, got {ttl}"
            )));
        }

        let mut result = Self { ttl };

        // If immediate is not allowed and TTL is 0, use default
        if !allow_immediate && ttl == 0 {
            result.ttl = Self::IMMEDIATE_TTL;
        }

        Ok(result)
    }

    /// Returns the TTL in seconds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// let timer = MessageSelfDestructType::new(30)?;
    /// assert_eq!(timer.ttl(), 30);
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    #[must_use]
    pub const fn ttl(self) -> i32 {
        self.ttl
    }

    /// Returns the TTL as a `Duration`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    /// use std::time::Duration;
    ///
    /// let timer = MessageSelfDestructType::new(30)?;
    /// assert_eq!(timer.duration(), Duration::from_secs(30));
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    #[must_use]
    pub fn duration(self) -> Duration {
        Duration::from_secs(self.ttl as u64)
    }

    /// Checks if this is a valid self-destruct timer.
    ///
    /// A timer is valid if the TTL is non-negative.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// let timer = MessageSelfDestructType::new(30)?;
    /// assert!(timer.is_valid());
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.ttl >= 0
    }

    /// Checks if this is an empty self-destruct timer (no auto-delete).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// let timer = MessageSelfDestructType::new(30)?;
    /// assert!(!timer.is_empty());
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.ttl == 0
    }

    /// Checks if this is an immediate self-destruct timer.
    ///
    /// Immediate timers delete the message right after viewing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// let immediate = MessageSelfDestructType::new_immediate(0, true)?;
    /// assert!(immediate.is_immediate());
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    #[must_use]
    pub const fn is_immediate(self) -> bool {
        self.ttl == 0 || self.ttl == Self::IMMEDIATE_TTL
    }

    /// Ensures the TTL is at least the specified minimum.
    ///
    /// # Arguments
    ///
    /// * `min_ttl` - Minimum TTL in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// let mut timer = MessageSelfDestructType::new(10)?;
    /// timer.ensure_at_least(30);
    /// assert_eq!(timer.ttl(), 30);
    ///
    /// let mut timer = MessageSelfDestructType::new(60)?;
    /// timer.ensure_at_least(30);
    /// assert_eq!(timer.ttl(), 60);
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    pub fn ensure_at_least(&mut self, min_ttl: i32) {
        if self.ttl < min_ttl {
            self.ttl = min_ttl;
        }
    }

    /// Returns the TTL value for input to TDLib.
    ///
    /// This is the same as `ttl()` but provided for API compatibility.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_self_destruct_type::MessageSelfDestructType;
    ///
    /// let timer = MessageSelfDestructType::new(30)?;
    /// assert_eq!(timer.input_ttl(), 30);
    ///
    /// # Ok::<(), rustgram_message_self_destruct_type::Error>(())
    /// ```
    #[must_use]
    pub const fn input_ttl(self) -> i32 {
        self.ttl
    }
}

impl fmt::Display for MessageSelfDestructType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_immediate() {
            write!(f, "immediate")
        } else if self.is_empty() {
            write!(f, "disabled")
        } else {
            write!(f, "{}s", self.ttl)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let timer = MessageSelfDestructType::new(30).unwrap();
        assert_eq!(timer.ttl(), 30);
        assert!(timer.is_valid());
        assert!(!timer.is_empty());
        assert!(!timer.is_immediate());
    }

    #[test]
    fn test_new_zero() {
        let timer = MessageSelfDestructType::new(0).unwrap();
        assert_eq!(timer.ttl(), 0);
        assert!(timer.is_valid());
        assert!(timer.is_empty());
        assert!(timer.is_immediate());
    }

    #[test]
    fn test_new_negative() {
        let result = MessageSelfDestructType::new(-1);
        assert!(result.is_err());
        match result {
            Err(Error::InvalidTtl(msg)) => {
                assert!(msg.contains("non-negative"));
            }
            _ => panic!("Expected InvalidTtl error"),
        }
    }

    #[test]
    fn test_new_immediate_valid() {
        let timer = MessageSelfDestructType::new_immediate(30, true).unwrap();
        assert_eq!(timer.ttl(), 30);
    }

    #[test]
    fn test_new_immediate_zero_allowed() {
        let timer = MessageSelfDestructType::new_immediate(0, true).unwrap();
        assert_eq!(timer.ttl(), 0);
        assert!(timer.is_immediate());
    }

    #[test]
    fn test_new_immediate_zero_not_allowed() {
        let timer = MessageSelfDestructType::new_immediate(0, false).unwrap();
        assert_eq!(timer.ttl(), MessageSelfDestructType::IMMEDIATE_TTL);
        assert!(timer.is_immediate());
    }

    #[test]
    fn test_is_valid() {
        let valid = MessageSelfDestructType::new(30).unwrap();
        assert!(valid.is_valid());

        let empty = MessageSelfDestructType::new(0).unwrap();
        assert!(empty.is_valid());
    }

    #[test]
    fn test_is_empty() {
        let empty = MessageSelfDestructType::new(0).unwrap();
        assert!(empty.is_empty());

        let timer = MessageSelfDestructType::new(30).unwrap();
        assert!(!timer.is_empty());
    }

    #[test]
    fn test_is_immediate() {
        let zero = MessageSelfDestructType::new(0).unwrap();
        assert!(zero.is_immediate());

        let immediate = MessageSelfDestructType::new_immediate(0, true).unwrap();
        assert!(immediate.is_immediate());

        let timer = MessageSelfDestructType::new(30).unwrap();
        assert!(!timer.is_immediate());
    }

    #[test]
    fn test_duration() {
        let timer = MessageSelfDestructType::new(30).unwrap();
        assert_eq!(timer.duration(), Duration::from_secs(30));
    }

    #[test]
    fn test_ensure_at_least() {
        let mut timer = MessageSelfDestructType::new(10).unwrap();
        timer.ensure_at_least(30);
        assert_eq!(timer.ttl(), 30);

        let mut timer = MessageSelfDestructType::new(60).unwrap();
        timer.ensure_at_least(30);
        assert_eq!(timer.ttl(), 60);
    }

    #[test]
    fn test_input_ttl() {
        let timer = MessageSelfDestructType::new(30).unwrap();
        assert_eq!(timer.input_ttl(), 30);
    }

    #[test]
    fn test_display() {
        let timer = MessageSelfDestructType::new(30).unwrap();
        assert_eq!(format!("{}", timer), "30s");

        let empty = MessageSelfDestructType::new(0).unwrap();
        assert_eq!(format!("{}", empty), "immediate");

        let immediate = MessageSelfDestructType::new_immediate(0, true).unwrap();
        assert_eq!(format!("{}", immediate), "immediate");
    }

    #[test]
    fn test_default() {
        let timer = MessageSelfDestructType::default();
        assert_eq!(timer.ttl(), 0);
        assert!(timer.is_empty());
    }

    #[test]
    fn test_equality() {
        let timer1 = MessageSelfDestructType::new(30).unwrap();
        let timer2 = MessageSelfDestructType::new(30).unwrap();
        assert_eq!(timer1, timer2);

        let timer3 = MessageSelfDestructType::new(60).unwrap();
        assert_ne!(timer1, timer3);
    }

    #[test]
    fn test_clone() {
        let timer1 = MessageSelfDestructType::new(30).unwrap();
        let timer2 = timer1;
        assert_eq!(timer1, timer2);
    }

    #[test]
    fn test_serialization() {
        let timer = MessageSelfDestructType::new(30).unwrap();
        let json = serde_json::to_string(&timer).unwrap();
        let parsed: MessageSelfDestructType = serde_json::from_str(&json).unwrap();
        assert_eq!(timer, parsed);
    }

    #[test]
    fn test_max_ttl() {
        let timer = MessageSelfDestructType::new(MessageSelfDestructType::MAX_TTL).unwrap();
        assert_eq!(timer.ttl(), MessageSelfDestructType::MAX_TTL);
    }

    #[test]
    fn test_special_immediate_ttl() {
        let timer = MessageSelfDestructType::new(MessageSelfDestructType::IMMEDIATE_TTL).unwrap();
        assert_eq!(timer.ttl(), MessageSelfDestructType::IMMEDIATE_TTL);
        assert!(timer.is_immediate());
    }
}
