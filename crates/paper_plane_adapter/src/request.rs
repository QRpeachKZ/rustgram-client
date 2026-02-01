// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Request types for TDLib JSON API.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use crate::error::{AdapterError, Result};
use serde_json::Value;
use std::collections::HashMap;

/// Raw JSON request from TDLib client.
#[derive(Debug, Clone)]
pub struct RawRequest {
    /// The request type (e.g., "getMe", "sendMessage").
    #[allow(dead_code)]
    pub type_name: String,

    /// Extra data that should be echoed in the response.
    pub extra: Option<Value>,

    /// Client ID for multi-client support.
    #[allow(dead_code)]
    pub client_id: Option<i32>,

    /// All other fields from the JSON request.
    pub data: HashMap<String, Value>,
}

impl RawRequest {
    /// Parses a JSON request string.
    pub fn from_json(json: &str) -> Result<Self> {
        let value: Value = serde_json::from_str(json)?;

        let type_name = value
            .get("@type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AdapterError::MissingField("@type".to_string()))?
            .to_string();

        let extra = value.get("@extra").cloned();
        let client_id = value.get("@client_id").and_then(|v| v.as_i64()).map(|v| v as i32);

        let mut data = HashMap::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                if !key.starts_with('@') {
                    data.insert(key.clone(), val.clone());
                }
            }
        }

        Ok(Self {
            type_name,
            extra,
            client_id,
            data,
        })
    }

    /// Converts to a typed Request.
    pub fn to_typed(self) -> Result<Request> {
        Ok(match self.type_name.as_str() {
            "getMe" => Request::GetMe,
            "getChats" => Request::GetChats {
                limit: self.get_i32("limit").unwrap_or(20),
            },
            "getChatHistory" => {
                let chat_id = self.get_i64("chat_id").ok_or_else(|| AdapterError::missing_field("chat_id"))?;
                let limit = self.get_i32("limit").unwrap_or(20);
                let from_message_id = self.get_i64("from_message_id").unwrap_or(0);
                Request::GetChatHistory { chat_id, limit, from_message_id }
            }
            "getChat" => Request::GetChat {
                chat_id: self.get_i64("chat_id").ok_or_else(|| AdapterError::missing_field("chat_id"))?,
            },
            "sendMessage" => {
                let chat_id = self.get_i64("chat_id").ok_or_else(|| AdapterError::missing_field("chat_id"))?;
                let text = self.get_string("text");
                Request::SendMessage { chat_id, text }
            }
            "getUser" => Request::GetUser {
                user_id: self.get_i64("user_id").ok_or_else(|| AdapterError::missing_field("user_id"))?,
            },
            "getUserFull" => Request::GetUserFull {
                user_id: self.get_i64("user_id").ok_or_else(|| AdapterError::missing_field("user_id"))?,
            },
            "setAuthenticationPhoneNumber" => {
                let phone_number = self.get_string("phone_number")
                    .ok_or_else(|| AdapterError::missing_field("phone_number"))?;
                Request::SetAuthenticationPhoneNumber { phone_number }
            }
            "checkAuthenticationCode" => {
                let code = self.get_string("code")
                    .ok_or_else(|| AdapterError::missing_field("code"))?;
                Request::CheckAuthenticationCode { code }
            }
            "checkAuthenticationPassword" => {
                let password = self.get_string("password")
                    .ok_or_else(|| AdapterError::missing_field("password"))?;
                Request::CheckAuthenticationPassword { password }
            }
            "getAuthorizationState" => Request::GetAuthorizationState,
            _ => Request::Unknown(self.type_name.clone()),
        })
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.data.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
    }

    fn get_i32(&self, key: &str) -> Option<i32> {
        self.data.get(key).and_then(|v| v.as_i64()).map(|v| v as i32)
    }

    fn get_i64(&self, key: &str) -> Option<i64> {
        self.data.get(key).and_then(|v| v.as_i64())
    }
}

/// Typed request enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Get current user.
    GetMe,

    /// Get list of chats.
    GetChats {
        /// Maximum number of chats to return.
        limit: i32,
    },

    /// Get chat history (messages).
    GetChatHistory {
        /// Chat ID.
        chat_id: i64,
        /// Maximum number of messages to return.
        limit: i32,
        /// Get messages starting from this message ID (0 for latest).
        from_message_id: i64,
    },

    /// Get a specific chat.
    GetChat {
        /// Chat ID.
        chat_id: i64,
    },

    /// Send a message.
    SendMessage {
        /// Target chat ID.
        chat_id: i64,
        /// Message text.
        text: Option<String>,
    },

    /// Get a user.
    GetUser {
        /// User ID.
        user_id: i64,
    },

    /// Get full user info.
    GetUserFull {
        /// User ID.
        user_id: i64,
    },

    /// Set authentication phone number.
    SetAuthenticationPhoneNumber {
        /// Phone number in international format.
        phone_number: String,
    },

    /// Check authentication code.
    CheckAuthenticationCode {
        /// Authentication code from SMS.
        code: String,
    },

    /// Check authentication password (2FA).
    CheckAuthenticationPassword {
        /// Two-factor authentication password.
        password: String,
    },

    /// Get current authorization state.
    GetAuthorizationState,

    /// Unknown request type.
    Unknown(String),
}

impl Request {
    /// Returns the type name of this request.
    pub fn type_name(&self) -> &str {
        match self {
            Self::GetMe => "getMe",
            Self::GetChats { .. } => "getChats",
            Self::GetChatHistory { .. } => "getChatHistory",
            Self::GetChat { .. } => "getChat",
            Self::SendMessage { .. } => "sendMessage",
            Self::GetUser { .. } => "getUser",
            Self::GetUserFull { .. } => "getUserFull",
            Self::SetAuthenticationPhoneNumber { .. } => "setAuthenticationPhoneNumber",
            Self::CheckAuthenticationCode { .. } => "checkAuthenticationCode",
            Self::CheckAuthenticationPassword { .. } => "checkAuthenticationPassword",
            Self::GetAuthorizationState => "getAuthorizationState",
            Self::Unknown(name) => name,
        }
    }

    /// Checks if this request can be executed synchronously.
    pub fn is_synchronous(&self) -> bool {
        matches!(
            self,
            Self::GetAuthorizationState | Self::GetMe | Self::GetUser { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_raw_request_from_json() {
        let json = r#"{"@type":"getMe","@extra":"test"}"#;
        let req = RawRequest::from_json(json).unwrap();
        assert_eq!(req.type_name, "getMe");
        assert_eq!(req.extra, Some(json!("test")));
    }

    #[test]
    fn test_raw_request_missing_type() {
        let json = r#"{"foo":"bar"}"#;
        let result = RawRequest::from_json(json);
        assert!(matches!(result, Err(AdapterError::MissingField(_))));
    }

    #[test]
    fn test_raw_request_to_typed_get_me() {
        let json = r#"{"@type":"getMe"}"#;
        let raw = RawRequest::from_json(json).unwrap();
        let typed = raw.to_typed().unwrap();
        assert_eq!(typed, Request::GetMe);
    }

    #[test]
    fn test_raw_request_to_typed_get_chats() {
        let json = r#"{"@type":"getChats","limit":50}"#;
        let raw = RawRequest::from_json(json).unwrap();
        let typed = raw.to_typed().unwrap();
        assert_eq!(typed, Request::GetChats { limit: 50 });
    }

    #[test]
    fn test_raw_request_to_typed_send_message() {
        let json = r#"{"@type":"sendMessage","chat_id":123456,"text":"Hello"}"#;
        let raw = RawRequest::from_json(json).unwrap();
        let typed = raw.to_typed().unwrap();
        assert_eq!(
            typed,
            Request::SendMessage {
                chat_id: 123456,
                text: Some("Hello".to_string())
            }
        );
    }

    #[test]
    fn test_raw_request_to_typed_unknown() {
        let json = r#"{"@type":"unknownMethod"}"#;
        let raw = RawRequest::from_json(json).unwrap();
        let typed = raw.to_typed().unwrap();
        assert_eq!(typed, Request::Unknown("unknownMethod".to_string()));
    }

    #[test]
    fn test_request_type_name() {
        assert_eq!(Request::GetMe.type_name(), "getMe");
        assert_eq!(Request::GetChats { limit: 20 }.type_name(), "getChats");
        assert_eq!(
            Request::SendMessage {
                chat_id: 123,
                text: None
            }
            .type_name(),
            "sendMessage"
        );
    }

    #[test]
    fn test_request_is_synchronous() {
        assert!(Request::GetMe.is_synchronous());
        assert!(Request::GetAuthorizationState.is_synchronous());
        assert!(!Request::GetChats { limit: 20 }.is_synchronous());
        assert!(!Request::SendMessage {
            chat_id: 123,
            text: None
        }
        .is_synchronous());
    }
}
