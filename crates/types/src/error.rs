//! Error types for the rustgram-types crate.

use thiserror::Error;

/// Errors that can occur when working with Telegram types.
#[derive(Error, Debug)]
pub enum TypeError {
    /// Invalid ID value.
    #[error("Invalid ID: {0}")]
    InvalidId(InvalidIdError),

    /// Invalid value for a type.
    #[error("Invalid value: {0}")]
    InvalidValue(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Generic error with a message.
    #[error("{0}")]
    Other(String),
}

/// Error indicating an invalid ID value.
#[derive(Error, Debug)]
#[error("Invalid {id_type}: {detail}")]
pub struct InvalidIdError {
    /// The type of ID that was invalid.
    pub id_type: String,
    /// Details about why the ID is invalid.
    pub detail: String,
}

impl InvalidIdError {
    /// Creates a new invalid ID error.
    pub fn new(id_type: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            id_type: id_type.into(),
            detail: detail.into(),
        }
    }

    /// Creates an invalid UserId error.
    pub fn user_id(detail: impl Into<String>) -> Self {
        Self::new("UserId", detail)
    }

    /// Creates an invalid ChatId error.
    pub fn chat_id(detail: impl Into<String>) -> Self {
        Self::new("ChatId", detail)
    }

    /// Creates an invalid ChannelId error.
    pub fn channel_id(detail: impl Into<String>) -> Self {
        Self::new("ChannelId", detail)
    }

    /// Creates an invalid MessageId error.
    pub fn message_id(detail: impl Into<String>) -> Self {
        Self::new("MessageId", detail)
    }

    /// Creates an invalid SecretChatId error.
    pub fn secret_chat_id(detail: impl Into<String>) -> Self {
        Self::new("SecretChatId", detail)
    }

    /// Creates an invalid DialogId error.
    pub fn dialog_id(detail: impl Into<String>) -> Self {
        Self::new("DialogId", detail)
    }
}

impl From<InvalidIdError> for TypeError {
    fn from(err: InvalidIdError) -> Self {
        TypeError::InvalidId(err)
    }
}

/// Result type for type operations.
pub type TypeResult<T> = Result<T, TypeError>;
