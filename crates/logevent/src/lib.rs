// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]

//! # LogEvent Module
//!
//! This module provides TDLib-compatible event logging for Telegram client operations.
//! It handles serialization and deserialization of log events used for binlog persistence.
//!
//! ## Overview
//!
//! Log events are used to persist operations that need to survive application restarts.
//! Each event has a type ([`HandlerType`]) and can be serialized/deserialized using
//! TL (Type Language) format.
//!
//! ## Core Types
//!
//! - [`LogEvent`]: Base trait for all log events
//! - [`HandlerType`]: Enum defining all event types
//! - [`LogEventIdWithGeneration`]: Tracks event IDs across generations
//!
//! ## Secret Chat Events
//!
//! Secret chat events are a special category of log events:
//! - [`InboundSecretMessage`]: Received secret messages
//! - [`OutboundSecretMessage`]: Sent secret messages
//! - [`CloseSecretChat`]: Secret chat closure
//! - [`CreateSecretChat`]: Secret chat creation
//!
//! ## Example
//!
//! ```rust
//! use rustgram_logevent::{LogEvent, CreateSecretChat};
//!
//! let event = CreateSecretChat {
//!     log_event_id: 0,
//!     random_id: 12345,
//!     user_id: 12345678,
//!     user_access_hash: 98765432,
//! };
//!
//! assert_eq!(event.log_event_id(), 0);
//! ```

mod error;
mod flags;
mod helper;
mod parser;
mod secret;
mod storer;
mod types;

pub use error::{LogEventError, Result};
pub use flags::{FlagsParser, FlagsStorer};
pub use helper::{add_log_event, delete_log_event};
pub use parser::{LogEventParser, TlParser};
pub use secret::{
    CloseSecretChat, CreateSecretChat, EncryptedInputFile, EncryptedInputFileType,
    InboundSecretMessage, OutboundSecretMessage, SecretChatEvent, SecretChatEventEnum,
    SecretChatEventType,
};
pub use storer::{LogEventStorerCalcLength, LogEventStorerUnsafe, LogEventStorerVec, TlStorer};
pub use types::{HandlerType, LogEventIdWithGeneration, LogEventIdWithGenerationExt};

/// Current version of the log event format
pub const LOG_EVENT_VERSION: i32 = 4;

/// Magic number for EncryptedInputFile
pub const ENCRYPTED_INPUT_FILE_MAGIC: u32 = 0x4328_d38a;

/// Base trait for all log events.
///
/// All log events must implement this trait to support serialization
/// and tracking of their unique IDs.
pub trait LogEvent: std::fmt::Debug + Send + Sync {
    /// Returns the unique log event ID
    fn log_event_id(&self) -> u64;

    /// Sets the unique log event ID
    fn set_log_event_id(&mut self, id: u64);
}
