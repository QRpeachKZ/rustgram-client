// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Request mapper - converts JSON requests to manager calls.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use crate::error::{AdapterError, Result};
use crate::request::Request;
use crate::response::{ChatType, Response, UserInfo};
use rustgram_auth_manager::AuthManager;
use rustgram_dialog_manager::{DialogManager, NetworkClient as DialogNetworkClient};
use rustgram_messages_manager::MessagesManager;
use rustgram_types::{ChatId, DialogId, UserId};
use rustgram_user_manager::UserManager;
use std::sync::Arc;
use tracing::debug;

/// Request mapper that processes requests.
pub struct RequestMapper {
    /// Dialog manager instance.
    #[allow(dead_code)]
    dialog_manager: DialogManager,

    /// Messages manager instance.
    #[allow(dead_code)]
    messages_manager: Option<Arc<MessagesManager>>,

    /// User manager instance.
    #[allow(dead_code)]
    user_manager: Option<Arc<UserManager>>,

    /// Auth manager instance.
    #[allow(dead_code)]
    auth_manager: Option<Arc<AuthManager>>,

    /// Network client for dialog operations.
    #[allow(dead_code)]
    dialog_network_client: Option<Arc<DialogNetworkClient>>,
}

impl RequestMapper {
    /// Creates a new request mapper.
    #[must_use]
    pub fn new(
        dialog_manager: DialogManager,
        messages_manager: Option<Arc<MessagesManager>>,
        user_manager: Option<Arc<UserManager>>,
        auth_manager: Option<Arc<AuthManager>>,
        dialog_network_client: Option<Arc<DialogNetworkClient>>,
    ) -> Self {
        Self {
            dialog_manager,
            messages_manager,
            user_manager,
            auth_manager,
            dialog_network_client,
        }
    }

    /// Processes a request and returns the response.
    pub async fn process_request(&self, request: &Request, request_id: u64) -> Result<Response> {
        debug!("Processing request {} ({})", request_id, request.type_name());

        match request {
            Request::GetMe => {
                debug!("getMe request");
                match &self.user_manager {
                    Some(manager) => {
                        match manager.get_me().await {
                            Some(user) => {
                                let user_info = self.convert_user_to_info(user);
                                Ok(Response::user(user_info))
                            }
                            None => Ok(Response::error(404, "User not found")),
                        }
                    }
                    None => Ok(Response::error(500, "UserManager not available")),
                }
            }
            Request::GetUser { user_id } => {
                debug!("getUser request for {}", user_id);
                match &self.user_manager {
                    Some(manager) => {
                        match UserId::new(*user_id) {
                            Ok(uid) => {
                                match manager.get_user(uid).await {
                                    Some(user) => {
                                        let user_info = self.convert_user_to_info(user);
                                        Ok(Response::user(user_info))
                                    }
                                    None => Ok(Response::error(404, "User not found")),
                                }
                            }
                            Err(_) => Ok(Response::error(400, "Invalid user_id")),
                        }
                    }
                    None => Ok(Response::error(500, "UserManager not available")),
                }
            }
            Request::GetUserFull { user_id } => {
                debug!("getUserFull request for {}", user_id);
                match &self.user_manager {
                    Some(manager) => {
                        match UserId::new(*user_id) {
                            Ok(uid) => {
                                match manager.fetch_full_user(uid).await {
                                    Ok(Some(full_user)) => {
                                        let user_info = full_user.user.clone().map_or_else(
                                            || UserInfo::new(uid.get()),
                                            |u| self.convert_user_to_info(u),
                                        );
                                        // TODO: Extract bio and profile_photo from full_user
                                        Ok(Response::user_full(user_info, full_user.about.clone(), None))
                                    }
                                    Ok(None) => Ok(Response::error(404, "User not found")),
                                    Err(e) => Ok(Response::error(500, format!("Network error: {}", e))),
                                }
                            }
                            Err(_) => Ok(Response::error(400, "Invalid user_id")),
                        }
                    }
                    None => Ok(Response::error(500, "UserManager not available")),
                }
            }
            Request::GetChats { limit } => {
                debug!("getChats request (limit: {})", limit);
                // Note: DialogManager.load_dialogs is async
                // For now, return cached dialogs or use network client
                match &self.dialog_network_client {
                    Some(_client) => {
                        // TODO: Use network client to load dialogs
                        // This would require implementing load_dialogs
                        Ok(Response::Chats {
                            chat_ids: vec![],
                            total_count: 0,
                        })
                    }
                    None => Ok(Response::error(500, "Network client not available")),
                }
            }
            Request::GetChatHistory { chat_id, limit, from_message_id } => {
                debug!("getChatHistory request for {} (limit: {}, from: {})", chat_id, limit, from_message_id);
                match ChatId::new(*chat_id) {
                    Ok(chat) => {
                        // TODO: MessagesManager doesn't have get_history method yet
                        // For now, return empty list
                        let _chat_id_value = chat.get();
                        Ok(Response::Messages {
                            messages: vec![],
                        })
                    }
                    Err(_) => Ok(Response::error(400, "Invalid chat_id")),
                }
            }
            Request::GetChat { chat_id } => {
                debug!("getChat request for {}", chat_id);
                match ChatId::new(*chat_id) {
                    Ok(chat) => {
                        let dialog_id = DialogId::from_chat(chat);
                        let chat_id_value = chat.get();
                        if self.dialog_manager.has_dialog(dialog_id) {
                            match self.dialog_manager.get_dialog_title(dialog_id) {
                                Some(title) => {
                                    let chat_type = ChatType::Private { user_id: *chat_id };
                                    Ok(Response::chat(chat_id_value, title, chat_type))
                                }
                                None => Ok(Response::error(404, "Chat title not found")),
                            }
                        } else {
                            Ok(Response::error(404, "Chat not found"))
                        }
                    }
                    Err(_) => Ok(Response::error(400, "Invalid chat_id")),
                }
            }
            Request::SendMessage { chat_id, text } => {
                debug!("sendMessage request to {} (text: {:?})", chat_id, text);
                match (&self.messages_manager, &text) {
                    (Some(manager), Some(msg_text)) => {
                        match ChatId::new(*chat_id) {
                            Ok(chat) => {
                                let dialog_id = DialogId::from_chat(chat);
                                let chat_id_value = chat.get();
                                match manager.send_text(dialog_id, msg_text.clone(), None).await {
                                    Ok(msg_id) => {
                                        let id = msg_id.get() as i32;
                                        Ok(Response::Message {
                                            id,
                                            chat_id: chat_id_value,
                                            content: crate::response::MessageContent::text(msg_text),
                                            date: 0,
                                        })
                                    }
                                    Err(e) => Ok(Response::error(500, format!("Send error: {}", e))),
                                }
                            }
                            Err(_) => Ok(Response::error(400, "Invalid chat_id")),
                        }
                    }
                    (None, _) => Ok(Response::error(500, "MessagesManager not available")),
                    (_, None) => Ok(Response::error(400, "Message text is required")),
                }
            }
            Request::SetAuthenticationPhoneNumber { phone_number } => {
                debug!("setAuthenticationPhoneNumber for {}", phone_number);
                match &self.auth_manager {
                    Some(manager) => {
                        match manager.set_phone_number(phone_number.clone()).await {
                            Ok(()) => {
                                let state = manager.get_state();
                                Ok(Response::authorization_state(state))
                            }
                            Err(e) => Ok(Response::error(400, format!("Auth error: {}", e))),
                        }
                    }
                    None => Ok(Response::error(500, "AuthManager not available")),
                }
            }
            Request::CheckAuthenticationCode { code } => {
                debug!("checkAuthenticationCode");
                match &self.auth_manager {
                    Some(manager) => {
                        match manager.check_code(code.clone(), None).await {
                            Ok(()) => {
                                let state = manager.get_state();
                                Ok(Response::authorization_state(state))
                            }
                            Err(e) => Ok(Response::error(400, format!("Invalid code: {}", e))),
                        }
                    }
                    None => Ok(Response::error(500, "AuthManager not available")),
                }
            }
            Request::CheckAuthenticationPassword { password } => {
                debug!("checkAuthenticationPassword");
                match &self.auth_manager {
                    Some(manager) => {
                        match manager.check_password(password.clone()).await {
                            Ok(()) => {
                                let state = manager.get_state();
                                Ok(Response::authorization_state(state))
                            }
                            Err(e) => Ok(Response::error(400, format!("Invalid password: {}", e))),
                        }
                    }
                    None => Ok(Response::error(500, "AuthManager not available")),
                }
            }
            Request::GetAuthorizationState => {
                debug!("getAuthorizationState");
                match &self.auth_manager {
                    Some(manager) => {
                        let state = manager.get_state();
                        Ok(Response::authorization_state(state))
                    }
                    None => Ok(Response::authorization_state_waiting()),
                }
            }
            Request::Unknown(name) => {
                debug!("Unknown request type: {}", name);
                Err(AdapterError::UnknownType(name.clone()))
            }
        }
    }

    /// Converts a UserManager User to Response UserInfo.
    fn convert_user_to_info(&self, user: rustgram_user_manager::User) -> UserInfo {
        UserInfo {
            id: user.id().get(),
            first_name: user.first_name().to_string(),
            last_name: user.last_name().to_string(),
            username: user.usernames().first_username().to_string(),
            phone_number: user.phone_number().to_string(),
        }
    }
}

impl Default for RequestMapper {
    fn default() -> Self {
        Self {
            dialog_manager: DialogManager::new(),
            messages_manager: None,
            user_manager: None,
            auth_manager: None,
            dialog_network_client: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapper_creation() {
        let dialog_manager = DialogManager::new();
        let mapper = RequestMapper::new(
            dialog_manager,
            None,
            None,
            None,
            None,
        );
        drop(mapper);
    }

    #[test]
    fn test_mapper_default() {
        let mapper = RequestMapper::default();
        drop(mapper);
    }

    #[tokio::test]
    async fn test_process_get_chats() {
        let mapper = RequestMapper::default();
        let request = Request::GetChats { limit: 20 };
        let response = mapper.process_request(&request, 1).await;

        // Returns error because network client is not available
        assert!(response.is_ok());
        match response.unwrap() {
            Response::Error { code, .. } => {
                assert_eq!(code, 500);
            }
            _ => panic!("Expected Error response for missing network client"),
        }
    }

    #[tokio::test]
    async fn test_process_get_me() {
        let mapper = RequestMapper::default();
        let request = Request::GetMe;
        let response = mapper.process_request(&request, 1).await;

        // Returns error because user_manager is not available
        assert!(response.is_ok());
        match response.unwrap() {
            Response::Error { code, .. } => {
                assert_eq!(code, 500);
            }
            _ => panic!("Expected Error response for missing UserManager"),
        }
    }

    #[tokio::test]
    async fn test_process_send_message() {
        let mapper = RequestMapper::default();
        let request = Request::SendMessage {
            chat_id: 123,
            text: Some("Hello".to_string()),
        };
        let response = mapper.process_request(&request, 1).await;

        // Returns error because messages_manager is not available
        assert!(response.is_ok());
        match response.unwrap() {
            Response::Error { code, .. } => {
                assert_eq!(code, 500);
            }
            _ => panic!("Expected Error response for missing MessagesManager"),
        }
    }

    #[tokio::test]
    async fn test_process_unknown_request() {
        let mapper = RequestMapper::default();
        let request = Request::Unknown("unknownMethod".to_string());
        let response = mapper.process_request(&request, 1).await;

        assert!(response.is_err());
        assert!(matches!(response, Err(AdapterError::UnknownType(_))));
    }

    #[tokio::test]
    async fn test_process_get_user_invalid_id() {
        let mapper = RequestMapper::default();
        let request = Request::GetUser { user_id: -1 };
        let response = mapper.process_request(&request, 1).await;

        // Returns error because user_manager is not available (before validation)
        assert!(response.is_ok());
        match response.unwrap() {
            Response::Error { code, .. } => {
                assert_eq!(code, 500);
            }
            _ => panic!("Expected Error response"),
        }
    }

    #[tokio::test]
    async fn test_process_send_message_no_text() {
        let mapper = RequestMapper::default();
        let request = Request::SendMessage {
            chat_id: 123,
            text: None,
        };
        let response = mapper.process_request(&request, 1).await;

        // Returns error because messages_manager is not available (before validation)
        assert!(response.is_ok());
        match response.unwrap() {
            Response::Error { code, .. } => {
                assert_eq!(code, 500);
            }
            _ => panic!("Expected Error response for missing MessagesManager"),
        }
    }
}
