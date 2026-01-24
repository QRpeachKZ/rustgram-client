// Copyright 2024 rustgram-client contributors
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

//! # JSON Value
//!
//! Represents JSON data types for Telegram API.
//!
//! ## Overview
//!
//! `JsonValue` represents JSON data with support for all common JSON types:
//! - Null
//! - Boolean
//! - Number
//! - String
//! - Array (recursive)
//! - Object (key-value pairs)
//!
//! ## TDLib Correspondence
//!
//! | Rust Type | TDLib Type | TL Schema |
//! |-----------|-----------|-----------|
//! | [`JsonValue::Null`] | `jsonValueNull` | `td_api.tl:7917` |
//! | [`JsonValue::Boolean`] | `jsonValueBoolean` | `td_api.tl:7920` |
//! | [`JsonValue::Number`] | `jsonValueNumber` | `td_api.tl:7923` |
//! | [`JsonValue::String`] | `jsonValueString` | `td_api.tl:7926` |
//! | [`JsonValue::Array`] | `jsonValueArray` | `td_api.tl:7929` |
//! | [`JsonValue::Object`] | `jsonValueObject` | `td_api.tl:7932` |
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_json_value::JsonValue;
//!
//! // Create various JSON values
//! let null = JsonValue::null();
//! let bool_val = JsonValue::boolean(true);
//! let num = JsonValue::number(42.0);
//! let str_val = JsonValue::string("hello");
//! let arr = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
//! let obj = JsonValue::object(vec![("key".to_string(), JsonValue::string("value"))]);
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value as SerdeValue;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

/// JSON value type.
///
/// Represents any valid JSON value, including nested structures.
///
/// # Examples
///
/// ```
/// use rustgram_json_value::JsonValue;
///
/// // Create various types
/// let null = JsonValue::null();
/// let boolean = JsonValue::boolean(true);
/// let number = JsonValue::number(42.5);
/// let string = JsonValue::string("hello");
/// let array = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
/// let object = JsonValue::object(vec![("key".to_string(), JsonValue::null())]);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum JsonValue {
    /// TDLib: `jsonValueNull`
    ///
    /// JSON null value.
    #[default]
    Null,

    /// TDLib: `jsonValueBoolean`
    ///
    /// JSON boolean value.
    Boolean(bool),

    /// TDLib: `jsonValueNumber`
    ///
    /// JSON number value (stored as f64).
    Number(f64),

    /// TDLib: `jsonValueString`
    ///
    /// JSON string value.
    String(String),

    /// TDLib: `jsonValueArray`
    ///
    /// JSON array (recursively contains JsonValue).
    Array(Vec<JsonValue>),

    /// TDLib: `jsonValueObject`
    ///
    /// JSON object with string keys and JsonValue values.
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Creates a null JSON value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let val = JsonValue::null();
    /// assert!(val.is_null());
    /// ```
    pub fn null() -> Self {
        Self::Null
    }

    /// Creates a boolean JSON value.
    ///
    /// # Arguments
    ///
    /// * `value` - The boolean value
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let val = JsonValue::boolean(true);
    /// assert!(val.is_boolean());
    /// ```
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Creates a number JSON value.
    ///
    /// # Arguments
    ///
    /// * `value` - The number value (f64)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let val = JsonValue::number(42.0);
    /// assert!(val.is_number());
    /// ```
    pub fn number(value: f64) -> Self {
        Self::Number(value)
    }

    /// Creates a string JSON value.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let val = JsonValue::string("hello");
    /// assert!(val.is_string());
    /// assert_eq!(val.as_str(), Some("hello"));
    /// ```
    pub fn string(value: &str) -> Self {
        Self::String(value.to_string())
    }

    /// Creates an array JSON value.
    ///
    /// # Arguments
    ///
    /// * `values` - Vector of JSON values
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let arr = JsonValue::array(vec![
    ///     JsonValue::number(1.0),
    ///     JsonValue::number(2.0),
    /// ]);
    /// assert!(arr.is_array());
    /// assert_eq!(arr.as_array().unwrap().len(), 2);
    /// ```
    pub fn array(values: Vec<JsonValue>) -> Self {
        Self::Array(values)
    }

    /// Creates an object JSON value.
    ///
    /// # Arguments
    ///
    /// * `members` - Vector of key-value pairs
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let obj = JsonValue::object(vec![
    ///     ("key".to_string(), JsonValue::string("value")),
    /// ]);
    /// assert!(obj.is_object());
    /// assert_eq!(obj.as_object().unwrap().len(), 1);
    /// ```
    pub fn object(members: Vec<(String, JsonValue)>) -> Self {
        let map = members.into_iter().collect();
        Self::Object(map)
    }

    /// Returns `true` if this is a null value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert!(JsonValue::null().is_null());
    /// assert!(!JsonValue::boolean(true).is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if this is a boolean value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert!(JsonValue::boolean(true).is_boolean());
    /// assert!(!JsonValue::null().is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Returns `true` if this is a number value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert!(JsonValue::number(42.0).is_number());
    /// assert!(!JsonValue::null().is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns `true` if this is a string value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert!(JsonValue::string("test").is_string());
    /// assert!(!JsonValue::null().is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns `true` if this is an array value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert!(JsonValue::array(vec![]).is_array());
    /// assert!(!JsonValue::null().is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns `true` if this is an object value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert!(JsonValue::object(vec![]).is_object());
    /// assert!(!JsonValue::null().is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns the boolean value if this is a boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert_eq!(JsonValue::boolean(true).as_bool(), Some(true));
    /// assert_eq!(JsonValue::null().as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the number value if this is a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert_eq!(JsonValue::number(42.5).as_f64(), Some(42.5));
    /// assert_eq!(JsonValue::null().as_f64(), None);
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the string value if this is a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// assert_eq!(JsonValue::string("hello").as_str(), Some("hello"));
    /// assert_eq!(JsonValue::null().as_str(), None);
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the array if this is an array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let arr = JsonValue::array(vec![JsonValue::null()]);
    /// assert!(arr.as_array().is_some());
    /// assert_eq!(arr.as_array().unwrap().len(), 1);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns the object if this is an object.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let obj = JsonValue::object(vec![("key".to_string(), JsonValue::null())]);
    /// assert!(obj.as_object().is_some());
    /// assert_eq!(obj.as_object().unwrap().len(), 1);
    /// ```
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            Self::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Gets a value from an object by key.
    ///
    /// Returns `None` if this is not an object or the key doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let obj = JsonValue::object(vec![
    ///     ("key".to_string(), JsonValue::string("value")),
    /// ]);
    /// assert_eq!(obj.get("key").and_then(|v| v.as_str()), Some("value"));
    /// assert_eq!(obj.get("missing"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(map) => map.get(key),
            _ => None,
        }
    }

    /// Gets a value from an array by index.
    ///
    /// Returns `None` if this is not an array or the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let arr = JsonValue::array(vec![
    ///     JsonValue::number(1.0),
    ///     JsonValue::number(2.0),
    /// ]);
    /// assert_eq!(arr.get_at(0).and_then(|v| v.as_f64()), Some(1.0));
    /// assert_eq!(arr.get_at(10), None);
    /// ```
    pub fn get_at(&self, index: usize) -> Option<&JsonValue> {
        match self {
            Self::Array(arr) => arr.get(index),
            _ => None,
        }
    }

    /// Converts from serde_json::Value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    /// use serde_json::json;
    ///
    /// let serde_val = json!(42);
    /// let val = JsonValue::from_serde(serde_val);
    /// assert_eq!(val.as_f64(), Some(42.0));
    /// ```
    pub fn from_serde(value: SerdeValue) -> Self {
        match value {
            SerdeValue::Null => Self::Null,
            SerdeValue::Bool(b) => Self::Boolean(b),
            SerdeValue::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Self::Number(f)
                } else if let Some(u) = n.as_u64() {
                    Self::Number(u as f64)
                } else if let Some(i) = n.as_i64() {
                    Self::Number(i as f64)
                } else {
                    Self::Null
                }
            }
            SerdeValue::String(s) => Self::String(s),
            SerdeValue::Array(arr) => Self::Array(arr.into_iter().map(Self::from_serde).collect()),
            SerdeValue::Object(obj) => {
                let map = obj
                    .into_iter()
                    .map(|(k, v)| (k, Self::from_serde(v)))
                    .collect();
                Self::Object(map)
            }
        }
    }

    /// Converts to serde_json::Value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_json_value::JsonValue;
    ///
    /// let val = JsonValue::string("hello");
    /// let serde_val = val.to_serde();
    /// assert_eq!(serde_val.as_str(), Some("hello"));
    /// ```
    pub fn to_serde(&self) -> SerdeValue {
        match self {
            Self::Null => SerdeValue::Null,
            Self::Boolean(b) => SerdeValue::Bool(*b),
            Self::Number(n) => serde_json::Number::from_f64(*n)
                .map(SerdeValue::Number)
                .unwrap_or(SerdeValue::Null),
            Self::String(s) => SerdeValue::String(s.clone()),
            Self::Array(arr) => SerdeValue::Array(arr.iter().map(|v| v.to_serde()).collect()),
            Self::Object(map) => {
                let obj = map.iter().map(|(k, v)| (k.clone(), v.to_serde())).collect();
                SerdeValue::Object(obj)
            }
        }
    }
}

impl Eq for JsonValue {}

impl Hash for JsonValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Null => 0u8.hash(state),
            Self::Boolean(b) => {
                1u8.hash(state);
                b.hash(state);
            }
            Self::Number(n) => {
                2u8.hash(state);
                // Use float-to-integer conversion for hashing
                let bits: u64 = if n.is_nan() {
                    0x7FF80000u64 << 32 // Canonical NaN (as u64)
                } else {
                    n.to_bits()
                };
                bits.hash(state);
            }
            Self::String(s) => {
                3u8.hash(state);
                s.hash(state);
            }
            Self::Array(arr) => {
                4u8.hash(state);
                arr.len().hash(state);
                for item in arr {
                    item.hash(state);
                }
            }
            Self::Object(map) => {
                5u8.hash(state);
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                keys.hash(state);
                for key in keys {
                    key.hash(state);
                    map.get(key).hash(state);
                }
            }
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Number(n) => {
                if n.is_nan() {
                    write!(f, "NaN")
                } else if n.is_infinite() {
                    if *n < 0.0 {
                        write!(f, "-Infinity")
                    } else {
                        write!(f, "Infinity")
                    }
                } else {
                    write!(f, "{}", n)
                }
            }
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Self::Object(map) => {
                write!(f, "{{")?;
                let mut entries: Vec<(&String, &JsonValue)> = map.iter().collect();
                entries.sort_by_key(|(k, _)| *k);
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl From<JsonValue> for SerdeValue {
    fn from(val: JsonValue) -> Self {
        val.to_serde()
    }
}

impl From<SerdeValue> for JsonValue {
    fn from(val: SerdeValue) -> Self {
        Self::from_serde(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_object() -> JsonValue {
        JsonValue::object(vec![
            ("null".to_string(), JsonValue::null()),
            ("bool".to_string(), JsonValue::boolean(true)),
            ("num".to_string(), JsonValue::number(42.5)),
            ("str".to_string(), JsonValue::string("hello")),
            (
                "arr".to_string(),
                JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]),
            ),
        ])
    }

    #[test]
    fn test_null() {
        let val = JsonValue::null();
        assert!(val.is_null());
        assert!(!val.is_boolean());
        assert!(!val.is_number());
    }

    #[test]
    fn test_boolean() {
        let val = JsonValue::boolean(true);
        assert!(val.is_boolean());
        assert_eq!(val.as_bool(), Some(true));
        assert!(!val.is_null());
    }

    #[test]
    fn test_number() {
        let val = JsonValue::number(42.5);
        assert!(val.is_number());
        assert_eq!(val.as_f64(), Some(42.5));
    }

    #[test]
    fn test_string() {
        let val = JsonValue::string("hello");
        assert!(val.is_string());
        assert_eq!(val.as_str(), Some("hello"));
    }

    #[test]
    fn test_array() {
        let val = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
        assert!(val.is_array());
        assert_eq!(val.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_object() {
        let val = JsonValue::object(vec![("key".to_string(), JsonValue::null())]);
        assert!(val.is_object());
        assert_eq!(val.as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_default() {
        let val = JsonValue::default();
        assert!(val.is_null());
    }

    #[test]
    fn test_as_bool_none() {
        assert_eq!(JsonValue::null().as_bool(), None);
        assert_eq!(JsonValue::number(42.0).as_bool(), None);
    }

    #[test]
    fn test_as_f64_none() {
        assert_eq!(JsonValue::null().as_f64(), None);
        assert_eq!(JsonValue::boolean(true).as_f64(), None);
    }

    #[test]
    fn test_as_str_none() {
        assert_eq!(JsonValue::null().as_str(), None);
        assert_eq!(JsonValue::number(42.0).as_str(), None);
    }

    #[test]
    fn test_as_array_none() {
        assert_eq!(JsonValue::null().as_array(), None);
        assert_eq!(JsonValue::string("test").as_array(), None);
    }

    #[test]
    fn test_as_object_none() {
        assert_eq!(JsonValue::null().as_object(), None);
        assert_eq!(JsonValue::string("test").as_object(), None);
    }

    #[test]
    fn test_get_from_object() {
        let obj = create_test_object();
        assert_eq!(obj.get("null").map(|v| v.is_null()), Some(true));
        assert_eq!(obj.get("bool").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(obj.get("num").and_then(|v| v.as_f64()), Some(42.5));
        assert_eq!(obj.get("str").and_then(|v| v.as_str()), Some("hello"));
        assert!(obj.get("arr").map(|v| v.is_array()).unwrap_or(false));
        assert_eq!(obj.get("missing"), None);
    }

    #[test]
    fn test_get_at_from_array() {
        let arr = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
        assert_eq!(arr.get_at(0).and_then(|v| v.as_f64()), Some(1.0));
        assert_eq!(arr.get_at(1).and_then(|v| v.as_f64()), Some(2.0));
        assert_eq!(arr.get_at(10), None);
    }

    #[test]
    fn test_get_at_from_non_array() {
        assert_eq!(JsonValue::null().get_at(0), None);
        assert_eq!(JsonValue::string("test").get_at(0), None);
    }

    #[test]
    fn test_get_from_non_object() {
        assert_eq!(JsonValue::null().get("key"), None);
        assert_eq!(JsonValue::string("test").get("key"), None);
    }

    #[test]
    fn test_equality() {
        assert_eq!(JsonValue::null(), JsonValue::null());
        assert_eq!(JsonValue::boolean(true), JsonValue::boolean(true));
        assert_eq!(JsonValue::number(42.0), JsonValue::number(42.0));
        assert_eq!(JsonValue::string("test"), JsonValue::string("test"));
        assert_ne!(JsonValue::boolean(true), JsonValue::boolean(false));
        assert_ne!(JsonValue::number(42.0), JsonValue::number(43.0));
    }

    #[test]
    fn test_array_equality() {
        let arr1 = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
        let arr2 = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
        assert_eq!(arr1, arr2);

        let arr3 = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(3.0)]);
        assert_ne!(arr1, arr3);
    }

    #[test]
    fn test_object_equality() {
        let obj1 = JsonValue::object(vec![
            ("a".to_string(), JsonValue::number(1.0)),
            ("b".to_string(), JsonValue::number(2.0)),
        ]);
        let obj2 = JsonValue::object(vec![
            ("b".to_string(), JsonValue::number(2.0)),
            ("a".to_string(), JsonValue::number(1.0)),
        ]);
        assert_eq!(obj1, obj2);

        let obj3 = JsonValue::object(vec![("a".to_string(), JsonValue::number(1.0))]);
        assert_ne!(obj1, obj3);
    }

    #[test]
    fn test_clone() {
        let val = create_test_object();
        let cloned = val.clone();
        assert_eq!(val, cloned);
    }

    #[test]
    fn test_debug() {
        let val = JsonValue::number(42.0);
        let debug = format!("{:?}", val);
        assert!(debug.contains("Number"));
    }

    #[test]
    fn test_display_null() {
        assert_eq!(format!("{}", JsonValue::null()), "null");
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(format!("{}", JsonValue::boolean(true)), "true");
        assert_eq!(format!("{}", JsonValue::boolean(false)), "false");
    }

    #[test]
    fn test_display_number() {
        assert_eq!(format!("{}", JsonValue::number(42.5)), "42.5");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(format!("{}", JsonValue::string("hello")), "\"hello\"");
    }

    #[test]
    fn test_display_array() {
        let arr = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
        assert_eq!(format!("{}", arr), "[1, 2]");
    }

    #[test]
    fn test_display_object() {
        let obj = JsonValue::object(vec![
            ("a".to_string(), JsonValue::number(1.0)),
            ("b".to_string(), JsonValue::number(2.0)),
        ]);
        let display = format!("{}", obj);
        assert!(display.contains("a"));
        assert!(display.contains("b"));
        assert!(display.contains("1"));
        assert!(display.contains("2"));
    }

    #[test]
    fn test_display_nan() {
        assert_eq!(format!("{}", JsonValue::number(f64::NAN)), "NaN");
    }

    #[test]
    fn test_display_infinity() {
        assert_eq!(format!("{}", JsonValue::number(f64::INFINITY)), "Infinity");
        assert_eq!(
            format!("{}", JsonValue::number(f64::NEG_INFINITY)),
            "-Infinity"
        );
    }

    #[test]
    fn test_from_serde_null() {
        let val = JsonValue::from_serde(serde_json::json!(null));
        assert!(val.is_null());
    }

    #[test]
    fn test_from_serde_bool() {
        let val = JsonValue::from_serde(serde_json::json!(true));
        assert_eq!(val.as_bool(), Some(true));
    }

    #[test]
    fn test_from_serde_number() {
        let val = JsonValue::from_serde(serde_json::json!(42));
        assert_eq!(val.as_f64(), Some(42.0));
    }

    #[test]
    fn test_from_serde_string() {
        let val = JsonValue::from_serde(serde_json::json!("hello"));
        assert_eq!(val.as_str(), Some("hello"));
    }

    #[test]
    fn test_from_serde_array() {
        let val = JsonValue::from_serde(serde_json::json!([1, 2, 3]));
        assert!(val.is_array());
        assert_eq!(val.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_from_serde_object() {
        let val = JsonValue::from_serde(serde_json::json!({"key": "value"}));
        assert!(val.is_object());
        assert_eq!(val.as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_to_serde() {
        let val = JsonValue::string("hello");
        let serde_val = val.to_serde();
        assert_eq!(serde_val.as_str(), Some("hello"));
    }

    #[test]
    fn test_serialization() {
        let val = create_test_object();
        let json = serde_json::to_string(&val).unwrap();
        let parsed: JsonValue = serde_json::from_str(&json).unwrap();
        assert_eq!(val, parsed);
    }

    #[test]
    fn test_hash_null() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let val1 = JsonValue::null();
        let val2 = JsonValue::null();

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        val1.hash(&mut h1);
        val2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_hash_boolean() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let val1 = JsonValue::boolean(true);
        let val2 = JsonValue::boolean(true);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        val1.hash(&mut h1);
        val2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_hash_number() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let val1 = JsonValue::number(42.0);
        let val2 = JsonValue::number(42.0);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        val1.hash(&mut h1);
        val2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_hash_string() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let val1 = JsonValue::string("test");
        let val2 = JsonValue::string("test");

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        val1.hash(&mut h1);
        val2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_hash_array() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let val1 = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);
        let val2 = JsonValue::array(vec![JsonValue::number(1.0), JsonValue::number(2.0)]);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        val1.hash(&mut h1);
        val2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_hash_object() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let obj1 = JsonValue::object(vec![
            ("a".to_string(), JsonValue::number(1.0)),
            ("b".to_string(), JsonValue::number(2.0)),
        ]);
        let obj2 = JsonValue::object(vec![
            ("b".to_string(), JsonValue::number(2.0)),
            ("a".to_string(), JsonValue::number(1.0)),
        ]);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        obj1.hash(&mut h1);
        obj2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_empty_array() {
        let arr = JsonValue::array(vec![]);
        assert!(arr.is_array());
        assert_eq!(arr.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_empty_object() {
        let obj = JsonValue::object(vec![]);
        assert!(obj.is_object());
        assert_eq!(obj.as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_nested_array() {
        let nested = JsonValue::array(vec![
            JsonValue::array(vec![JsonValue::number(1.0)]),
            JsonValue::array(vec![JsonValue::number(2.0)]),
        ]);
        assert!(nested.is_array());
        assert_eq!(nested.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_nested_object() {
        let nested = JsonValue::object(vec![(
            "outer".to_string(),
            JsonValue::object(vec![("inner".to_string(), JsonValue::number(1.0))]),
        )]);
        assert!(nested.is_object());
        assert_eq!(nested.as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_special_float_values() {
        let nan = JsonValue::number(f64::NAN);
        assert!(nan.is_number());
        assert!(nan.as_f64().unwrap().is_nan());

        let inf = JsonValue::number(f64::INFINITY);
        assert!(inf.is_number());
        assert!(inf.as_f64().unwrap().is_infinite());
    }

    #[test]
    fn test_from_serde_conversion() {
        let serde_val = serde_json::json!({
            "key": "value",
            "num": 42,
            "arr": [1, 2, 3]
        });
        let val: JsonValue = serde_val.into();
        assert!(val.is_object());
        assert_eq!(val.get("key").and_then(|v| v.as_str()), Some("value"));
    }

    #[test]
    fn test_to_serde_conversion() {
        let val = JsonValue::object(vec![
            ("key".to_string(), JsonValue::string("value")),
            ("num".to_string(), JsonValue::number(42.0)),
        ]);
        let serde_val: SerdeValue = val.into();
        assert_eq!(serde_val.get("key").and_then(|v| v.as_str()), Some("value"));
    }
}
