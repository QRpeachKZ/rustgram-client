// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Response types and JSON serialization.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// A response to be sent to the client.
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Response {
    /// getChats response.
    Chats {
        /// List of chat IDs.
        chat_ids: Vec<i64>,
        /// Total count of chats.
        total_count: i32,
    },

    /// getMe/getUser response.
    User {
        /// User information.
        user: UserInfo,
    },

    /// getUserFull response.
    UserFull {
        /// Full user information.
        user: UserInfo,
        /// User bio.
        bio: Option<String>,
        /// Profile photo info.
        profile_photo: Option<ProfilePhoto>,
    },

    /// getChat response.
    Chat {
        /// Chat ID.
        id: i64,
        /// Chat title.
        title: String,
        /// Chat type.
        type_: ChatType,
    },

    /// sendMessage response.
    Message {
        /// Message ID.
        id: i32,
        /// Chat ID where message was sent.
        chat_id: i64,
        /// Message content.
        content: MessageContent,
        /// Unix timestamp when message was sent.
        date: i32,
    },

    /// getChatHistory response.
    Messages {
        /// List of messages.
        messages: Vec<MessageData>,
    },

    /// Authorization state update.
    AuthorizationState {
        /// Current authorization state.
        state: AuthorizationState,
    },

    /// Generic ok response.
    Ok,

    /// Error response.
    Error {
        /// Error code.
        code: i32,
        /// Error message.
        message: String,
    },
}

impl Response {
    /// Creates a message sent response.
    #[must_use]
    pub fn message_sent(id: i32) -> Self {
        Self::Message {
            id,
            chat_id: 0,
            content: MessageContent::text(""),
            date: 0,
        }
    }

    /// Creates an authorization state waiting response.
    #[must_use]
    pub fn authorization_state_waiting() -> Self {
        Self::AuthorizationState {
            state: AuthorizationState::WaitPhoneNumber,
        }
    }

    /// Creates an ok response.
    #[must_use]
    pub fn ok() -> Self {
        Self::Ok
    }

    /// Creates an error response.
    #[must_use]
    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self::Error {
            code,
            message: message.into(),
        }
    }

    /// Creates a user response.
    #[must_use]
    pub fn user(user: UserInfo) -> Self {
        Self::User { user }
    }

    /// Creates a user full response.
    #[must_use]
    pub fn user_full(user: UserInfo, bio: Option<String>, profile_photo: Option<ProfilePhoto>) -> Self {
        Self::UserFull {
            user,
            bio,
            profile_photo,
        }
    }

    /// Creates a chat response.
    #[must_use]
    pub fn chat(id: i64, title: String, type_: ChatType) -> Self {
        Self::Chat { id, title, type_ }
    }

    /// Creates an authorization state response from an AuthManager state.
    #[must_use]
    pub fn authorization_state(state: rustgram_auth_manager::State) -> Self {
        Self::AuthorizationState {
            state: AuthorizationState::from(state),
        }
    }

    /// Serializes the response to JSON.
    pub fn to_json(&self, extra: Option<serde_json::Value>) -> Result<String> {
        let mut value = match self {
            Self::Chats { chat_ids, total_count } => serde_json::json!({
                "@type": "chats",
                "chat_ids": chat_ids,
                "total_count": total_count,
            }),
            Self::User { user } => serde_json::json!({
                "@type": "user",
                "id": user.id,
                "first_name": user.first_name,
                "last_name": user.last_name,
                "username": user.username,
                "phone_number": user.phone_number,
            }),
            Self::UserFull { user, bio, profile_photo } => {
                let mut json = serde_json::json!({
                    "@type": "userFull",
                    "user": {
                        "@type": "user",
                        "id": user.id,
                        "first_name": user.first_name,
                        "last_name": user.last_name,
                        "username": user.username,
                        "phone_number": user.phone_number,
                    },
                    "bio": bio,
                });
                if let Some(photo) = profile_photo {
                    json["profile_photo"] = serde_json::to_value(photo)?;
                }
                json
            }
            Self::Chat { id, title, type_ } => serde_json::json!({
                "@type": "chat",
                "id": id,
                "title": title,
                "type": type_,
            }),
            Self::Message { id, chat_id, content, date } => serde_json::json!({
                "@type": "message",
                "id": id,
                "chat_id": chat_id,
                "content": content,
                "date": date,
            }),
            Self::Messages { messages } => serde_json::json!({
                "@type": "messages",
                "messages": messages,
                "total_count": messages.len() as i32,
            }),
            Self::AuthorizationState { state } => serde_json::json!({
                "@type": "updateAuthorizationState",
                "authorization_state": state,
            }),
            Self::Ok => serde_json::json!({
                "@type": "ok",
            }),
            Self::Error { code, message } => serde_json::json!({
                "@type": "error",
                "code": code,
                "message": message,
            }),
        };

        if let Some(extra_value) = extra {
            if let Some(obj) = value.as_object_mut() {
                obj.insert("@extra".to_string(), extra_value);
            }
        }

        if let Some(obj) = value.as_object_mut() {
            obj.insert("@client_id".to_string(), serde_json::json!(0));
        }

        serde_json::to_string_pretty(&value).map_err(Into::into)
    }
}

/// Message content.
#[derive(Debug, Clone, Serialize)]
pub struct MessageContent {
    #[serde(rename = "@type")]
    content_type: String,

    #[serde(flatten)]
    text: FormattedText,
}

impl MessageContent {
    /// Creates a new text message content.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content_type: "messageText".to_string(),
            text: FormattedText::new(text.into()),
        }
    }
}

/// Message data for getChatHistory response.
#[derive(Debug, Clone, Serialize)]
pub struct MessageData {
    /// Message ID.
    pub id: i32,

    /// Chat ID.
    pub chat_id: i64,

    /// Message content.
    pub content: MessageContent,

    /// Unix timestamp when message was sent.
    pub date: i32,
}

/// Formatted text.
#[derive(Debug, Clone, Serialize)]
struct FormattedText {
    #[serde(rename = "@type")]
    text_type: String,

    text: String,
}

impl FormattedText {
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            text_type: "formattedText".to_string(),
            text,
        }
    }
}

/// Authorization state.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@type")]
pub enum AuthorizationState {
    /// Wait for phone number.
    #[serde(rename = "authorizationStateWaitPhoneNumber")]
    WaitPhoneNumber,

    /// Wait for authentication code.
    #[serde(rename = "authorizationStateWaitCode")]
    WaitCode,

    /// Wait for email code.
    #[serde(rename = "authorizationStateWaitEmailCode")]
    WaitEmailCode,

    /// Wait for password (2FA).
    #[serde(rename = "authorizationStateWaitPassword")]
    WaitPassword,

    /// Authorization complete.
    #[serde(rename = "authorizationStateReady")]
    Ok,

    /// Network error.
    #[serde(rename = "authorizationStateWaitCode")]
    NetworkError(String),
}

impl From<rustgram_auth_manager::State> for AuthorizationState {
    fn from(state: rustgram_auth_manager::State) -> Self {
        match state {
            rustgram_auth_manager::State::None => Self::WaitPhoneNumber,
            rustgram_auth_manager::State::WaitPhoneNumber => Self::WaitPhoneNumber,
            rustgram_auth_manager::State::WaitCode => Self::WaitCode,
            rustgram_auth_manager::State::WaitEmailCode => Self::WaitEmailCode,
            rustgram_auth_manager::State::WaitPassword => Self::WaitPassword,
            rustgram_auth_manager::State::Ok => Self::Ok,
            rustgram_auth_manager::State::LoggingOut => Self::Ok,
            rustgram_auth_manager::State::Closing => Self::Ok,
            rustgram_auth_manager::State::NetworkError(msg) => Self::NetworkError(msg),
            rustgram_auth_manager::State::WaitingRetry { .. } => Self::WaitCode,
        }
    }
}

/// Chat type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum ChatType {
    /// Private chat.
    #[serde(rename = "chatTypePrivate")]
    Private {
        /// User ID
        user_id: i64,
    },

    /// Secret chat.
    #[serde(rename = "chatTypeSecret")]
    Secret {
        /// User ID of the secret chat participant
        user_id: i64,
        /// Secret chat ID
        secret_chat_id: i32,
    },

    /// Basic group.
    #[serde(rename = "chatTypeBasicGroup")]
    BasicGroup {
        /// Basic group ID
        basic_group_id: i64,
    },

    /// Supergroup.
    #[serde(rename = "chatTypeSupergroup")]
    Supergroup {
        /// Supergroup ID
        supergroup_id: i64,
        /// Whether this is a channel
        is_channel: bool,
    },
}

/// Profile photo information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePhoto {
    #[serde(rename = "@type")]
    photo_type: String,

    id: i64,

    small: File,

    big: File,
}

impl ProfilePhoto {
    /// Creates a new profile photo.
    #[must_use]
    pub fn new(id: i64, small: File, big: File) -> Self {
        Self {
            photo_type: "profilePhoto".to_string(),
            id,
            small,
            big,
        }
    }
}

/// File information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "@type")]
    file_type: String,

    id: i64,

    size: i64,

    expected_size: i32,

    local: LocalFile,

    remote: RemoteFile,
}

impl File {
    /// Creates a new file.
    #[must_use]
    pub fn new(id: i64, size: i64) -> Self {
        Self {
            file_type: "file".to_string(),
            id,
            size,
            expected_size: 0,
            local: LocalFile::new(),
            remote: RemoteFile::new(),
        }
    }
}

/// Local file information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalFile {
    #[serde(rename = "@type")]
    local_type: String,

    path: String,

    can_be_downloaded: bool,

    can_be_deleted: bool,

    is_downloading_active: bool,

    is_downloading_completed: bool,

    download_offset: i32,

    downloaded_size: i32,

    downloaded_prefix: String,
}

impl LocalFile {
    /// Creates a new empty local file.
    #[must_use]
    pub fn new() -> Self {
        Self {
            local_type: "localFile".to_string(),
            path: String::new(),
            can_be_downloaded: false,
            can_be_deleted: false,
            is_downloading_active: false,
            is_downloading_completed: false,
            download_offset: 0,
            downloaded_size: 0,
            downloaded_prefix: String::new(),
        }
    }
}

impl Default for LocalFile {
    fn default() -> Self {
        Self::new()
    }
}

/// Remote file information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFile {
    #[serde(rename = "@type")]
    remote_type: String,

    id: String,

    is_uploading_active: bool,

    is_uploading_completed: bool,

    uploaded_size: i32,
}

impl RemoteFile {
    /// Creates a new empty remote file.
    #[must_use]
    pub fn new() -> Self {
        Self {
            remote_type: "remoteFile".to_string(),
            id: String::new(),
            is_uploading_active: false,
            is_uploading_completed: false,
            uploaded_size: 0,
        }
    }
}

impl Default for RemoteFile {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal response with context.
#[derive(Debug, Clone)]
pub struct OutgoingResponse {
    /// Request ID this responds to.
    pub request_id: u64,

    /// Serialized JSON response.
    pub response: String,

    /// Client ID.
    pub client_id: i32,
}

impl OutgoingResponse {
    /// Creates a new outgoing response.
    #[must_use]
    pub fn new(request_id: u64, response: String) -> Self {
        Self {
            request_id,
            response,
            client_id: 0,
        }
    }
}

/// User information for responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// User ID.
    pub id: i64,

    /// First name.
    #[serde(default)]
    pub first_name: String,

    /// Last name.
    #[serde(default)]
    pub last_name: String,

    /// Username.
    #[serde(default)]
    pub username: String,

    /// Phone number.
    #[serde(default)]
    pub phone_number: String,
}

impl UserInfo {
    /// Creates a new UserInfo.
    #[must_use]
    pub fn new(id: i64) -> Self {
        Self {
            id,
            first_name: String::new(),
            last_name: String::new(),
            username: String::new(),
            phone_number: String::new(),
        }
    }

    /// Sets the first name.
    #[must_use]
    pub fn with_first_name(mut self, first_name: String) -> Self {
        self.first_name = first_name;
        self
    }

    /// Sets the last name.
    #[must_use]
    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = last_name;
        self
    }

    /// Sets the username.
    #[must_use]
    pub fn with_username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    /// Sets the phone number.
    #[must_use]
    pub fn with_phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = phone_number;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chats_response() {
        let response = Response::Chats {
            chat_ids: vec![1, 2, 3],
            total_count: 3,
        };

        let json = response.to_json(None).unwrap();
        assert!(json.contains("chats"));
    }

    #[test]
    fn test_ok_response() {
        let response = Response::ok();
        let json = response.to_json(None).unwrap();
        assert!(json.contains("ok"));
    }

    #[test]
    fn test_user_response() {
        let user = UserInfo::new(123)
            .with_first_name("John".to_string())
            .with_last_name("Doe".to_string())
            .with_username("johndoe".to_string())
            .with_phone_number("+1234567890".to_string());

        let response = Response::user(user);
        let json = response.to_json(None).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("John"));
        assert!(json.contains("johndoe"));
    }

    #[test]
    fn test_error_response() {
        let response = Response::error(404, "Not found");
        let json = response.to_json(None).unwrap();
        assert!(json.contains("error"));
        assert!(json.contains("404"));
        assert!(json.contains("Not found"));
    }

    #[test]
    fn test_authorization_state_from_state() {
        use rustgram_auth_manager::State;

        // Test WaitPhoneNumber
        let state = State::WaitPhoneNumber;
        let auth_state: AuthorizationState = state.into();
        assert!(matches!(auth_state, AuthorizationState::WaitPhoneNumber));

        // Test WaitCode
        let state = State::WaitCode;
        let auth_state: AuthorizationState = state.into();
        assert!(matches!(auth_state, AuthorizationState::WaitCode));

        // Test WaitPassword
        let state = State::WaitPassword;
        let auth_state: AuthorizationState = state.into();
        assert!(matches!(auth_state, AuthorizationState::WaitPassword));

        // Test Ok
        let state = State::Ok;
        let auth_state: AuthorizationState = state.into();
        assert!(matches!(auth_state, AuthorizationState::Ok));

        // Test NetworkError
        let state = State::NetworkError("test error".to_string());
        let auth_state: AuthorizationState = state.into();
        assert!(matches!(auth_state, AuthorizationState::NetworkError(_)));
    }

    #[test]
    fn test_chat_type_serialization() {
        let chat_type = ChatType::Private { user_id: 123 };
        let json = serde_json::to_string(&chat_type).unwrap();
        assert!(json.contains("chatTypePrivate"));

        let chat_type = ChatType::Supergroup {
            supergroup_id: 456,
            is_channel: true,
        };
        let json = serde_json::to_string(&chat_type).unwrap();
        assert!(json.contains("chatTypeSupergroup"));
    }

    #[test]
    fn test_response_authorization_state() {
        let response = Response::authorization_state_waiting();
        let json = response.to_json(None).unwrap();
        assert!(json.contains("updateAuthorizationState"));
    }
}
