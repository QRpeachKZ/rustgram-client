// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Message Query Manager for Telegram MTProto client.
//!
//! This module implements the MessageQueryManager from TDLib's
//! `td/telegram/MessageQueryManager.h` and `MessageQueryManager.cpp`.
//!
//! # Overview
//!
//! The MessageQueryManager handles various message-related queries and operations
//! including:
//!
//! - **Search**: Search messages, public posts, and hashtag posts
//! - **Delete**: Delete messages and dialog history
//! - **Read**: Mark messages as read, track mentions and reactions
//! - **Upload**: Upload message covers and photos
//! - **View**: Track message views and viewers
//! - **Reload**: Refresh extended media, fact checks, reactions
//!
//! # Architecture
//!
//! The manager uses `Arc<RwLock<T>>` for thread-safe concurrent state management:
//!
//! - `being_uploaded_covers`: Tracks ongoing cover uploads
//! - `being_reloaded_extended_media`: Tracks extended media reloads
//! - `being_reloaded_fact_checks`: Tracks fact check reloads
//! - `being_reloaded_views`: Tracks view count reloads
//! - `need_view_counter_increment`: Messages pending view increment
//! - `being_reloaded_reactions`: Per-dialog reaction reload state
//! - `pending_read_reactions`: Messages pending reaction read status
//!
//! # TDLib Correspondence
//!
//! - Header: `td/telegram/MessageQueryManager.h` (324 lines, 53 methods)
//! - Implementation: `td/telegram/MessageQueryManager.cpp` (3528 lines)
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use rustgram_message_query_manager::MessageQueryManager;
//!
//! let manager = MessageQueryManager::new();
//! assert!(manager.is_empty());
//! assert!(!manager.has_pending_operations());
//! ```
//!
//! ## Searching Messages
//!
//! ```rust
//! use rustgram_message_query_manager::MessageQueryManager;
//! use rustgram_dialog_list_id::DialogListId;
//!
//! # #[tokio::main]
//! # async fn example() {
//! let manager = MessageQueryManager::new();
//! let results = manager.search_messages(
//!     DialogListId::main(),
//!     "search query".to_string(),
//!     String::new(),
//!     10,
//! ).await;
//! # }
//! ```
//!
//! ## Deleting Messages
//!
//! ```rust
//! use rustgram_message_query_manager::MessageQueryManager;
//! use rustgram_types::{ChatId, DialogId, MessageId};
//!
//! # #[tokio::main]
//! # async fn example() {
//! let manager = MessageQueryManager::new();
//! let result = manager.delete_messages_on_server(
//!     DialogId::from_chat(ChatId::new(123).unwrap()),
//!     vec![MessageId::from_server_id(456)],
//!     true,
//! ).await;
//! # }
//! ```
//!
//! ## Uploading Message Covers
//!
//! ```rust
//! use rustgram_message_query_manager::MessageQueryManager;
//! use rustgram_business_connection_id::BusinessConnectionId;
//! use rustgram_file_id::FileId;
//! use rustgram_file_upload_id::FileUploadId;
//! use rustgram_message_extended_media::Photo;
//! use rustgram_types::{ChatId, DialogId};
//!
//! # #[tokio::main]
//! # async fn example() {
//! let manager = MessageQueryManager::new();
//! let conn_id = BusinessConnectionId::default();
//! let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
//! let photo = Photo::new();
//! let file_id = FileId::new(1, 0);
//! let upload_id = FileUploadId::new(file_id, 1);
//!
//! manager.upload_message_cover(conn_id, dialog_id, photo, upload_id).await;
//! # }
//! ```
//!
//! # Thread Safety
//!
//! The MessageQueryManager is designed for concurrent access:
//!
//! ```rust
//! use rustgram_message_query_manager::MessageQueryManager;
//! use std::sync::Arc;
//!
//! # #[tokio::main]
//! # async fn example() {
//! let manager = Arc::new(MessageQueryManager::new());
//! // Can be safely shared across tasks
//! # }
//! ```
//!
//! # Stub Implementation Notice
//!
//! This is a stub implementation that matches the TDLib API structure.
//! The actual TDLib client integration will be implemented in a future update.
//! Current methods return default/empty values for testing purposes.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod error;
pub mod manager;
pub mod state;
pub mod tl;

// Re-exports
pub use error::{Error, Result};
pub use manager::{MessageQueryManager, MAX_SEARCH_MESSAGES};
pub use state::{BeingUploadedCover, MessageReloadState, ReactionsToReload, ReloadType};
pub use tl::{
    DiscussionMessage, FactCheck, FoundMessages, InputFile, MessageMedia, SearchPostsFlood,
};

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-message-query-manager";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-message-query-manager");
    }

    #[test]
    fn test_constants_exist() {
        assert_eq!(MAX_SEARCH_MESSAGES, 100);
    }

    // Test all re-exports are accessible
    #[test]
    fn test_error_export() {
        let _err = Error::InvalidDialog;
    }

    #[test]
    fn test_manager_import() {
        let _manager = MessageQueryManager::new();
    }

    #[test]
    fn test_state_exports() {
        let _reload = ReloadType::ExtendedMedia;
    }

    #[test]
    fn test_tl_exports() {
        let _media = MessageMedia::photo();
    }

    // Module-level documentation examples as doctests
    #[test]
    fn test_basic_usage_example() {
        let manager = MessageQueryManager::new();
        assert!(manager.is_empty());
        assert!(!manager.has_pending_operations());
    }

    // Error variant coverage tests
    #[test]
    fn test_all_error_variants() {
        let errors = vec![
            Error::InvalidDialog,
            Error::InvalidMessage,
            Error::UploadFailed,
            Error::DeleteFailed,
            Error::SearchFailed,
            Error::InvalidState,
            Error::IoError("test".to_string()),
            Error::Other("test".to_string()),
        ];

        for err in errors {
            let _display = format!("{err}");
        }
    }

    // ReloadType coverage tests
    #[test]
    fn test_all_reload_types() {
        let types = vec![
            ReloadType::ExtendedMedia,
            ReloadType::FactChecks,
            ReloadType::Views,
            ReloadType::Reactions,
        ];

        for reload_type in types {
            let _display = format!("{reload_type}");
        }
    }

    // MessageMedia coverage tests
    #[test]
    fn test_all_message_media_types() {
        let photo = MessageMedia::photo();
        let video = MessageMedia::video();
        let document = MessageMedia::Document { id: String::new() };
        let unsupported = MessageMedia::Unsupported;

        assert!(!photo.is_empty());
        assert!(!video.is_empty());
        assert!(!document.is_empty());
        assert!(unsupported.is_empty());
    }
}
