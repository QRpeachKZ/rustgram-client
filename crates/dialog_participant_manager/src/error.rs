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

//! Error types for dialog participant manager.

use rustgram_types::{ChatId, UserId};
use std::fmt;

/// Error type for participant operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParticipantError {
    /// Participant already exists in the chat.
    AlreadyExists {
        /// The chat ID where the participant exists.
        chat_id: ChatId,
        /// The user ID of the existing participant.
        user_id: UserId,
    },
    /// Participant not found in the chat.
    NotFound {
        /// The chat ID where the participant was not found.
        chat_id: ChatId,
        /// The user ID of the participant not found.
        user_id: UserId,
    },
    /// Chat not found in the manager.
    ChatNotFound {
        /// The chat ID that was not found.
        chat_id: ChatId,
    },
}

impl fmt::Display for ParticipantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { chat_id, user_id } => write!(
                f,
                "Participant {} already exists in chat {}",
                user_id.get(),
                chat_id.get()
            ),
            Self::NotFound { chat_id, user_id } => write!(
                f,
                "Participant {} not found in chat {}",
                user_id.get(),
                chat_id.get()
            ),
            Self::ChatNotFound { chat_id } => {
                write!(f, "Chat {} not found", chat_id.get())
            }
        }
    }
}

impl std::error::Error for ParticipantError {}

/// Result type for participant operations.
pub type Result<T> = std::result::Result<T, ParticipantError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_already_exists() {
        let chat_id = ChatId::new(123).unwrap();
        let user_id = UserId::new(456).unwrap();
        let error = ParticipantError::AlreadyExists { chat_id, user_id };
        let display = format!("{}", error);
        assert!(display.contains("already exists"));
        assert!(display.contains("123"));
        assert!(display.contains("456"));
    }

    #[test]
    fn test_error_display_not_found() {
        let chat_id = ChatId::new(123).unwrap();
        let user_id = UserId::new(456).unwrap();
        let error = ParticipantError::NotFound { chat_id, user_id };
        let display = format!("{}", error);
        assert!(display.contains("not found"));
        assert!(display.contains("123"));
        assert!(display.contains("456"));
    }

    #[test]
    fn test_error_display_chat_not_found() {
        let chat_id = ChatId::new(123).unwrap();
        let error = ParticipantError::ChatNotFound { chat_id };
        let display = format!("{}", error);
        assert!(display.contains("not found"));
        assert!(display.contains("123"));
    }
}
