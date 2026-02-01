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

//! Error types for reaction manager.

use rustgram_message_effect_id::MessageEffectId;
use std::fmt;

/// Result type for reaction manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for reaction manager.
///
/// # Examples
///
/// ```
/// use rustgram_reaction_manager::Error;
///
/// let err = Error::InvalidTagTitle;
/// assert_eq!(format!("{}", err), "invalid tag title");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Message effect not found.
    EffectNotFound(MessageEffectId),
    /// Tag title is too long.
    InvalidTagTitle,
    /// Tag not found.
    TagNotFound,
    /// Reaction is not active.
    ReactionNotActive,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EffectNotFound(id) => write!(f, "effect {} not found", id.get()),
            Self::InvalidTagTitle => write!(f, "invalid tag title"),
            Self::TagNotFound => write!(f, "tag not found"),
            Self::ReactionNotActive => write!(f, "reaction is not active"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(format!("{}", Error::InvalidTagTitle), "invalid tag title");
        assert_eq!(format!("{}", Error::TagNotFound), "tag not found");
        assert_eq!(
            format!("{}", Error::ReactionNotActive),
            "reaction is not active"
        );
    }

    #[test]
    fn test_effect_not_found_display() {
        let error = Error::EffectNotFound(MessageEffectId::new(123));
        assert_eq!(format!("{}", error), "effect 123 not found");
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(Error::InvalidTagTitle, Error::InvalidTagTitle);
        assert_eq!(Error::TagNotFound, Error::TagNotFound);
        assert_ne!(Error::InvalidTagTitle, Error::TagNotFound);
    }

    #[test]
    fn test_error_clone() {
        let error = Error::InvalidTagTitle;
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_effect_not_found_equality() {
        let error1 = Error::EffectNotFound(MessageEffectId::new(123));
        let error2 = Error::EffectNotFound(MessageEffectId::new(123));
        assert_eq!(error1, error2);

        let error3 = Error::EffectNotFound(MessageEffectId::new(456));
        assert_ne!(error1, error3);
    }
}
