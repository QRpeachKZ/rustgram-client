// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Option value types.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the value of an option.
///
/// This enum corresponds to TDLib's `OptionValue` types and supports
/// boolean, integer, string, and empty values.
///
/// # TL Correspondence
///
/// ```text
/// optionValueBoolean value:Bool = OptionValue;
/// optionValueEmpty = OptionValue;
/// optionValueInteger value:int64 = OptionValue;
/// optionValueString value:string = OptionValue;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_option_manager::OptionValue;
///
/// let bool_val = OptionValue::Boolean(true);
/// let int_val = OptionValue::Integer(42);
/// let string_val = OptionValue::String("hello".to_string());
/// let empty_val = OptionValue::Empty;
///
/// assert!(bool_val.is_boolean());
/// assert!(int_val.is_integer());
/// assert!(string_val.is_string());
/// assert!(empty_val.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OptionValue {
    /// Boolean value.
    ///
    /// Corresponds to TDLib's `optionValueBoolean`.
    Boolean(bool),

    /// Empty value.
    ///
    /// Corresponds to TDLib's `optionValueEmpty`.
    #[default]
    Empty,

    /// Integer value.
    ///
    /// Corresponds to TDLib's `optionValueInteger`.
    Integer(i64),

    /// String value.
    ///
    /// Corresponds to TDLib's `optionValueString`.
    String(String),
}

impl OptionValue {
    /// Create a boolean option value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::boolean(true);
    /// assert!(value.is_boolean());
    /// ```
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Create an empty option value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::empty();
    /// assert!(value.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Create an integer option value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::integer(42);
    /// assert!(value.is_integer());
    /// ```
    pub fn integer(value: i64) -> Self {
        Self::Integer(value)
    }

    /// Create a string option value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::string("hello");
    /// assert!(value.is_string());
    /// ```
    pub fn string(value: &str) -> Self {
        Self::String(value.to_string())
    }

    /// Check if this value is a boolean.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::Boolean(true);
    /// assert!(value.is_boolean());
    ///
    /// let value = OptionValue::Integer(42);
    /// assert!(!value.is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Check if this value is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::Empty;
    /// assert!(value.is_empty());
    ///
    /// let value = OptionValue::Integer(42);
    /// assert!(!value.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Check if this value is an integer.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::Integer(42);
    /// assert!(value.is_integer());
    ///
    /// let value = OptionValue::Boolean(true);
    /// assert!(!value.is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Check if this value is a string.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::String("hello".to_string());
    /// assert!(value.is_string());
    ///
    /// let value = OptionValue::Integer(42);
    /// assert!(!value.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Get the boolean value if this is a boolean.
    ///
    /// # Returns
    ///
    /// `Some(bool)` if this is a boolean, `None` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::Boolean(true);
    /// assert_eq!(value.as_boolean(), Some(true));
    ///
    /// let value = OptionValue::Integer(42);
    /// assert_eq!(value.as_boolean(), None);
    /// ```
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Get the integer value if this is an integer.
    ///
    /// # Returns
    ///
    /// `Some(i64)` if this is an integer, `None` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::Integer(42);
    /// assert_eq!(value.as_integer(), Some(42));
    ///
    /// let value = OptionValue::Boolean(true);
    /// assert_eq!(value.as_integer(), None);
    /// ```
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Get the string value if this is a string.
    ///
    /// # Returns
    ///
    /// `Some(&str)` if this is a string, `None` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_option_manager::OptionValue;
    ///
    /// let value = OptionValue::String("hello".to_string());
    /// assert_eq!(value.as_string(), Some("hello"));
    ///
    /// let value = OptionValue::Integer(42);
    /// assert_eq!(value.as_string(), None);
    /// ```
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

impl From<bool> for OptionValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i64> for OptionValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<i32> for OptionValue {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<String> for OptionValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for OptionValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        let value = OptionValue::boolean(true);
        assert!(value.is_boolean());
        assert_eq!(value.as_boolean(), Some(true));
        assert!(!value.is_empty());
        assert!(!value.is_integer());
        assert!(!value.is_string());
    }

    #[test]
    fn test_empty() {
        let value = OptionValue::empty();
        assert!(value.is_empty());
        assert_eq!(value.as_boolean(), None);
        assert_eq!(value.as_integer(), None);
        assert_eq!(value.as_string(), None);
    }

    #[test]
    fn test_integer() {
        let value = OptionValue::integer(42);
        assert!(value.is_integer());
        assert_eq!(value.as_integer(), Some(42));
        assert!(!value.is_empty());
        assert!(!value.is_boolean());
        assert!(!value.is_string());
    }

    #[test]
    fn test_string() {
        let value = OptionValue::string("hello");
        assert!(value.is_string());
        assert_eq!(value.as_string(), Some("hello"));
        assert!(!value.is_empty());
        assert!(!value.is_boolean());
        assert!(!value.is_integer());
    }

    #[test]
    fn test_as_boolean() {
        assert_eq!(OptionValue::Boolean(true).as_boolean(), Some(true));
        assert_eq!(OptionValue::Boolean(false).as_boolean(), Some(false));
        assert_eq!(OptionValue::Empty.as_boolean(), None);
        assert_eq!(OptionValue::Integer(1).as_boolean(), None);
        assert_eq!(OptionValue::String("test".to_string()).as_boolean(), None);
    }

    #[test]
    fn test_as_integer() {
        assert_eq!(OptionValue::Integer(42).as_integer(), Some(42));
        assert_eq!(OptionValue::Integer(-1).as_integer(), Some(-1));
        assert_eq!(OptionValue::Empty.as_integer(), None);
        assert_eq!(OptionValue::Boolean(true).as_integer(), None);
        assert_eq!(OptionValue::String("test".to_string()).as_integer(), None);
    }

    #[test]
    fn test_as_string() {
        assert_eq!(
            OptionValue::String("hello".to_string()).as_string(),
            Some("hello")
        );
        assert_eq!(OptionValue::String("".to_string()).as_string(), Some(""));
        assert_eq!(OptionValue::Empty.as_string(), None);
        assert_eq!(OptionValue::Boolean(true).as_string(), None);
        assert_eq!(OptionValue::Integer(42).as_string(), None);
    }

    #[test]
    fn test_default() {
        assert_eq!(OptionValue::default(), OptionValue::Empty);
    }

    #[test]
    fn test_from_bool() {
        let value: OptionValue = true.into();
        assert_eq!(value, OptionValue::Boolean(true));

        let value: OptionValue = false.into();
        assert_eq!(value, OptionValue::Boolean(false));
    }

    #[test]
    fn test_from_i64() {
        let value: OptionValue = 42i64.into();
        assert_eq!(value, OptionValue::Integer(42));
    }

    #[test]
    fn test_from_i32() {
        let value: OptionValue = 42i32.into();
        assert_eq!(value, OptionValue::Integer(42));
    }

    #[test]
    fn test_from_string() {
        let value: OptionValue = String::from("hello").into();
        assert_eq!(value, OptionValue::String("hello".to_string()));
    }

    #[test]
    fn test_from_str() {
        let value: OptionValue = "hello".into();
        assert_eq!(value, OptionValue::String("hello".to_string()));
    }

    #[test]
    fn test_equality() {
        assert_eq!(OptionValue::Boolean(true), OptionValue::Boolean(true));
        assert_ne!(OptionValue::Boolean(true), OptionValue::Boolean(false));
        assert_eq!(OptionValue::Integer(42), OptionValue::Integer(42));
        assert_ne!(OptionValue::Integer(42), OptionValue::Integer(43));
        assert_eq!(
            OptionValue::String("test".to_string()),
            OptionValue::String("test".to_string())
        );
        assert_ne!(
            OptionValue::String("test".to_string()),
            OptionValue::String("other".to_string())
        );
        assert_eq!(OptionValue::Empty, OptionValue::Empty);
    }
}
