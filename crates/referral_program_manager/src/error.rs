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

//! Error types for referral program manager.

use rustgram_types::UserId;
use std::fmt;

/// Result type for referral program manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for referral program manager.
///
/// # Examples
///
/// ```
/// use rustgram_referral_program_manager::Error;
///
/// let err = Error::InvalidUrl;
/// assert_eq!(format!("{}", err), "invalid URL");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Program not found.
    ProgramNotFound(UserId),
    /// Invalid URL provided.
    InvalidUrl,
    /// Invalid program parameters.
    InvalidParameters,
    /// Program is not connected.
    NotConnected,
    /// Program is already connected.
    AlreadyConnected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProgramNotFound(user_id) => write!(f, "program {} not found", user_id.get()),
            Self::InvalidUrl => write!(f, "invalid URL"),
            Self::InvalidParameters => write!(f, "invalid parameters"),
            Self::NotConnected => write!(f, "program is not connected"),
            Self::AlreadyConnected => write!(f, "program is already connected"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(format!("{}", Error::InvalidUrl), "invalid URL");
        assert_eq!(
            format!("{}", Error::InvalidParameters),
            "invalid parameters"
        );
        assert_eq!(
            format!("{}", Error::NotConnected),
            "program is not connected"
        );
        assert_eq!(
            format!("{}", Error::AlreadyConnected),
            "program is already connected"
        );
    }

    #[test]
    fn test_program_not_found_display() {
        let user_id = UserId::new(123456).unwrap();
        let error = Error::ProgramNotFound(user_id);
        assert_eq!(format!("{}", error), "program 123456 not found");
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(Error::InvalidUrl, Error::InvalidUrl);
        assert_eq!(Error::InvalidParameters, Error::InvalidParameters);
        assert_ne!(Error::InvalidUrl, Error::InvalidParameters);
    }

    #[test]
    fn test_error_clone() {
        let error = Error::InvalidUrl;
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_program_not_found_equality() {
        let user_id1 = UserId::new(123456).unwrap();
        let user_id2 = UserId::new(123456).unwrap();
        let error1 = Error::ProgramNotFound(user_id1);
        let error2 = Error::ProgramNotFound(user_id2);
        assert_eq!(error1, error2);

        let user_id3 = UserId::new(789012).unwrap();
        let error3 = Error::ProgramNotFound(user_id3);
        assert_ne!(error1, error3);
    }
}
