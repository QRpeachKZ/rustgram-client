// Copyright (c) 2024 rustgram-client contributions
//
// Licensed under MIT OR Apache-2.0

//! # Secure Value
//!
//! Secure value types for Telegram Passport.
//!
//! Based on TDLib's `SecureValue` from `td/telegram/SecureValue.h`.
//!
//! # Overview
//!
//! Secure values represent different types of documents and personal information
//! that can be stored in Telegram Passport.
//!
//! # Example
//!
//! ```rust
//! use rustgram_secure_value::{SecureValueType, SecureValue};
//!
//! let value = SecureValue::new(SecureValueType::Passport);
//! assert_eq!(value.type_(), SecureValueType::Passport);
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Secure value type enumeration.
///
/// Represents the different types of secure values in Telegram Passport.
///
/// # TDLib Mapping
///
/// - Each variant maps to TDLib's `SecureValueType` enum
///
/// # Example
///
/// ```rust
/// use rustgram_secure_value::SecureValueType;
///
/// let passport_type = SecureValueType::Passport;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[repr(i32)]
pub enum SecureValueType {
    /// No type
    #[default]
    None = 0,
    /// Personal details
    PersonalDetails = 1,
    /// Passport
    Passport = 2,
    /// Driver license
    DriverLicense = 3,
    /// Identity card
    IdentityCard = 4,
    /// Internal passport
    InternalPassport = 5,
    /// Address
    Address = 6,
    /// Utility bill
    UtilityBill = 7,
    /// Bank statement
    BankStatement = 8,
    /// Rental agreement
    RentalAgreement = 9,
    /// Passport registration
    PassportRegistration = 10,
    /// Temporary registration
    TemporaryRegistration = 11,
    /// Phone number
    PhoneNumber = 12,
    /// Email address
    EmailAddress = 13,
}

impl SecureValueType {
    /// Returns the integer representation of this type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_value::SecureValueType;
    ///
    /// assert_eq!(SecureValueType::Passport.as_i32(), 2);
    /// ```
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Creates a SecureValueType from an integer.
    ///
    /// Returns `None` if the integer doesn't match a valid type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_value::SecureValueType;
    ///
    /// assert_eq!(SecureValueType::from_i32(2), Some(SecureValueType::Passport));
    /// assert_eq!(SecureValueType::from_i32(99), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::PersonalDetails),
            2 => Some(Self::Passport),
            3 => Some(Self::DriverLicense),
            4 => Some(Self::IdentityCard),
            5 => Some(Self::InternalPassport),
            6 => Some(Self::Address),
            7 => Some(Self::UtilityBill),
            8 => Some(Self::BankStatement),
            9 => Some(Self::RentalAgreement),
            10 => Some(Self::PassportRegistration),
            11 => Some(Self::TemporaryRegistration),
            12 => Some(Self::PhoneNumber),
            13 => Some(Self::EmailAddress),
            _ => None,
        }
    }
}

impl fmt::Display for SecureValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::None => "None",
            Self::PersonalDetails => "PersonalDetails",
            Self::Passport => "Passport",
            Self::DriverLicense => "DriverLicense",
            Self::IdentityCard => "IdentityCard",
            Self::InternalPassport => "InternalPassport",
            Self::Address => "Address",
            Self::UtilityBill => "UtilityBill",
            Self::BankStatement => "BankStatement",
            Self::RentalAgreement => "RentalAgreement",
            Self::PassportRegistration => "PassportRegistration",
            Self::TemporaryRegistration => "TemporaryRegistration",
            Self::PhoneNumber => "PhoneNumber",
            Self::EmailAddress => "EmailAddress",
        };
        write!(f, "{}", name)
    }
}

/// Dated file reference.
///
/// Represents a file with an associated date.
///
/// # TDLib Mapping
///
/// - `DatedFile::new()` → TDLib: `DatedFile(FileId, int32)`
///
/// # Example
///
/// ```rust
/// use rustgram_secure_value::DatedFile;
///
/// let file = DatedFile::new(12345, 1234567890);
/// assert_eq!(file.file_id(), 12345);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatedFile {
    /// File ID
    file_id: i64,
    /// Date
    date: i32,
}

impl DatedFile {
    /// Creates a new dated file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - File ID
    /// * `date` - Unix timestamp
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_value::DatedFile;
    ///
    /// let file = DatedFile::new(12345, 1234567890);
    /// ```
    #[must_use]
    pub fn new(file_id: i64, date: i32) -> Self {
        Self { file_id, date }
    }

    /// Returns the file ID.
    #[must_use]
    pub fn file_id(&self) -> i64 {
        self.file_id
    }

    /// Returns the date.
    #[must_use]
    pub fn date(&self) -> i32 {
        self.date
    }
}

impl Default for DatedFile {
    fn default() -> Self {
        Self {
            file_id: 0,
            date: 0,
        }
    }
}

/// Secure value for Telegram Passport.

/// Secure value for Telegram Passport.
///
/// Represents a secure value with its type and associated data/files.
///
/// # TDLib Mapping
///
/// - `SecureValue::new()` → TDLib: `SecureValue(SecureValueType, ...)`
///
/// # Example
///
/// ```rust
/// use rustgram_secure_value::{SecureValueType, SecureValue};
///
/// let value = SecureValue::new(SecureValueType::Passport);
/// assert_eq!(value.type_(), SecureValueType::Passport);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecureValue {
    /// Type of secure value
    type_: SecureValueType,
    /// Data
    data: String,
    /// Files
    files: Vec<DatedFile>,
    /// Front side
    front_side: Option<DatedFile>,
    /// Reverse side
    reverse_side: Option<DatedFile>,
    /// Selfie
    selfie: Option<DatedFile>,
    /// Translations
    translations: Vec<DatedFile>,
}

impl SecureValue {
    /// Creates a new secure value.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of secure value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_value::{SecureValueType, SecureValue};
    ///
    /// let value = SecureValue::new(SecureValueType::Passport);
    /// ```
    #[must_use]
    pub fn new(type_: SecureValueType) -> Self {
        Self {
            type_,
            data: String::new(),
            files: Vec::new(),
            front_side: None,
            reverse_side: None,
            selfie: None,
            translations: Vec::new(),
        }
    }

    /// Sets the data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data string
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }

    /// Adds a file.
    ///
    /// # Arguments
    ///
    /// * `file` - Dated file to add
    pub fn add_file(&mut self, file: DatedFile) {
        self.files.push(file);
    }

    /// Sets the front side.
    ///
    /// # Arguments
    ///
    /// * `file` - Dated file for front side
    pub fn set_front_side(&mut self, file: DatedFile) {
        self.front_side = Some(file);
    }

    /// Sets the reverse side.
    ///
    /// # Arguments
    ///
    /// * `file` - Dated file for reverse side
    pub fn set_reverse_side(&mut self, file: DatedFile) {
        self.reverse_side = Some(file);
    }

    /// Sets the selfie.
    ///
    /// # Arguments
    ///
    /// * `file` - Dated file for selfie
    pub fn set_selfie(&mut self, file: DatedFile) {
        self.selfie = Some(file);
    }

    /// Adds a translation.
    ///
    /// # Arguments
    ///
    /// * `file` - Dated file to add as translation
    pub fn add_translation(&mut self, file: DatedFile) {
        self.translations.push(file);
    }

    /// Returns the type.
    #[must_use]
    pub fn type_(&self) -> SecureValueType {
        self.type_
    }

    /// Returns the data.
    #[must_use]
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Returns the files.
    #[must_use]
    pub fn files(&self) -> &[DatedFile] {
        &self.files
    }

    /// Returns the front side.
    #[must_use]
    pub fn front_side(&self) -> Option<&DatedFile> {
        self.front_side.as_ref()
    }

    /// Returns the reverse side.
    #[must_use]
    pub fn reverse_side(&self) -> Option<&DatedFile> {
        self.reverse_side.as_ref()
    }

    /// Returns the selfie.
    #[must_use]
    pub fn selfie(&self) -> Option<&DatedFile> {
        self.selfie.as_ref()
    }

    /// Returns the translations.
    #[must_use]
    pub fn translations(&self) -> &[DatedFile] {
        &self.translations
    }
}

impl Default for SecureValue {
    fn default() -> Self {
        Self::new(SecureValueType::None)
    }
}

impl fmt::Display for SecureValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecureValue({})", self.type_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SecureValueType tests
    #[test]
    fn test_secure_value_type_as_i32() {
        assert_eq!(SecureValueType::None.as_i32(), 0);
        assert_eq!(SecureValueType::Passport.as_i32(), 2);
        assert_eq!(SecureValueType::EmailAddress.as_i32(), 13);
    }

    #[test]
    fn test_secure_value_type_from_i32() {
        assert_eq!(SecureValueType::from_i32(0), Some(SecureValueType::None));
        assert_eq!(
            SecureValueType::from_i32(2),
            Some(SecureValueType::Passport)
        );
        assert_eq!(
            SecureValueType::from_i32(13),
            Some(SecureValueType::EmailAddress)
        );
        assert_eq!(SecureValueType::from_i32(99), None);
    }

    #[test]
    fn test_secure_value_type_roundtrip() {
        let types = vec![
            SecureValueType::None,
            SecureValueType::PersonalDetails,
            SecureValueType::Passport,
            SecureValueType::DriverLicense,
            SecureValueType::IdentityCard,
            SecureValueType::InternalPassport,
            SecureValueType::Address,
            SecureValueType::UtilityBill,
            SecureValueType::BankStatement,
            SecureValueType::RentalAgreement,
            SecureValueType::PassportRegistration,
            SecureValueType::TemporaryRegistration,
            SecureValueType::PhoneNumber,
            SecureValueType::EmailAddress,
        ];

        for t in types {
            assert_eq!(SecureValueType::from_i32(t.as_i32()), Some(t));
        }
    }

    #[test]
    fn test_secure_value_type_display() {
        assert_eq!(format!("{}", SecureValueType::Passport), "Passport");
        assert_eq!(format!("{}", SecureValueType::EmailAddress), "EmailAddress");
    }

    // DatedFile tests
    #[test]
    fn test_dated_file_new() {
        let file = DatedFile::new(12345, 1234567890);
        assert_eq!(file.file_id(), 12345);
        assert_eq!(file.date(), 1234567890);
    }

    #[test]
    fn test_dated_file_default() {
        let file = DatedFile::default();
        assert_eq!(file.file_id(), 0);
        assert_eq!(file.date(), 0);
    }

    #[test]
    fn test_dated_file_equality() {
        let file1 = DatedFile::new(12345, 1234567890);
        let file2 = DatedFile::new(12345, 1234567890);
        let file3 = DatedFile::new(54321, 987654320);

        assert_eq!(file1, file2);
        assert_ne!(file1, file3);
    }

    // SecureValue tests
    #[test]
    fn test_secure_value_new() {
        let value = SecureValue::new(SecureValueType::Passport);
        assert_eq!(value.type_(), SecureValueType::Passport);
        assert_eq!(value.data(), "");
        assert!(value.files().is_empty());
    }

    #[test]
    fn test_secure_value_default() {
        let value = SecureValue::default();
        assert_eq!(value.type_(), SecureValueType::None);
    }

    #[test]
    fn test_secure_value_set_data() {
        let mut value = SecureValue::new(SecureValueType::Passport);
        value.set_data("some data");
        assert_eq!(value.data(), "some data");
    }

    #[test]
    fn test_secure_value_add_file() {
        let mut value = SecureValue::new(SecureValueType::Passport);
        value.add_file(DatedFile::new(1, 100));
        value.add_file(DatedFile::new(2, 200));
        assert_eq!(value.files().len(), 2);
    }

    #[test]
    fn test_secure_value_set_front_side() {
        let mut value = SecureValue::new(SecureValueType::Passport);
        value.set_front_side(DatedFile::new(1, 100));
        assert!(value.front_side().is_some());
        assert_eq!(value.front_side().unwrap().file_id(), 1);
    }

    #[test]
    fn test_secure_value_set_reverse_side() {
        let mut value = SecureValue::new(SecureValueType::IdentityCard);
        value.set_reverse_side(DatedFile::new(2, 200));
        assert!(value.reverse_side().is_some());
        assert_eq!(value.reverse_side().unwrap().file_id(), 2);
    }

    #[test]
    fn test_secure_value_set_selfie() {
        let mut value = SecureValue::new(SecureValueType::Passport);
        value.set_selfie(DatedFile::new(3, 300));
        assert!(value.selfie().is_some());
        assert_eq!(value.selfie().unwrap().file_id(), 3);
    }

    #[test]
    fn test_secure_value_add_translation() {
        let mut value = SecureValue::new(SecureValueType::Passport);
        value.add_translation(DatedFile::new(1, 100));
        value.add_translation(DatedFile::new(2, 200));
        assert_eq!(value.translations().len(), 2);
    }

    #[test]
    fn test_secure_value_equality() {
        let value1 = SecureValue::new(SecureValueType::Passport);
        let value2 = SecureValue::new(SecureValueType::Passport);
        let value3 = SecureValue::new(SecureValueType::DriverLicense);

        assert_eq!(value1, value2);
        assert_ne!(value1, value3);
    }

    #[test]
    fn test_secure_value_clone() {
        let mut value1 = SecureValue::new(SecureValueType::Passport);
        value1.set_data("test");
        value1.add_file(DatedFile::new(1, 100));

        let value2 = value1.clone();
        assert_eq!(value2.data(), "test");
        assert_eq!(value2.files().len(), 1);
    }

    #[test]
    fn test_secure_value_display() {
        let value = SecureValue::new(SecureValueType::Passport);
        let display = format!("{value}");
        assert!(display.contains("Passport"));
    }

    #[test]
    fn test_serialization_secure_value_type() {
        let t = SecureValueType::Passport;
        let json = serde_json::to_string(&t).expect("Failed to serialize");
        let deserialized: SecureValueType =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, t);
    }

    #[test]
    fn test_serialization_dated_file() {
        let file = DatedFile::new(12345, 1234567890);
        let json = serde_json::to_string(&file).expect("Failed to serialize");
        let deserialized: DatedFile = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, file);
    }

    #[test]
    fn test_serialization_secure_value() {
        let mut value = SecureValue::new(SecureValueType::Passport);
        value.set_data("test");
        value.add_file(DatedFile::new(1, 100));

        let json = serde_json::to_string(&value).expect("Failed to serialize");
        let deserialized: SecureValue = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, value);
    }
}
