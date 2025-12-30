// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Secret chat event types
//!
//! This module defines the log events specific to secret chat operations.

use crate::{
    error::{LogEventError, Result},
    flags::{FlagsParser, FlagsStorer},
    LogEvent, TlParser, TlStorer, ENCRYPTED_INPUT_FILE_MAGIC, LOG_EVENT_VERSION,
};
use std::fmt;

/// Type of secret chat event
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecretChatEventType {
    /// Inbound secret message (received)
    InboundSecretMessage = 1,
    /// Outbound secret message (sending)
    OutboundSecretMessage = 2,
    /// Close secret chat
    CloseSecretChat = 3,
    /// Create secret chat
    CreateSecretChat = 4,
}

impl SecretChatEventType {
    /// Parse from i32
    pub fn from_i32(value: i32) -> Result<Self> {
        match value {
            1 => Ok(Self::InboundSecretMessage),
            2 => Ok(Self::OutboundSecretMessage),
            3 => Ok(Self::CloseSecretChat),
            4 => Ok(Self::CreateSecretChat),
            _ => Err(LogEventError::UnknownEventType(value as u32)),
        }
    }
}

impl fmt::Display for SecretChatEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::InboundSecretMessage => "InboundSecretMessage",
            Self::OutboundSecretMessage => "OutboundSecretMessage",
            Self::CloseSecretChat => "CloseSecretChat",
            Self::CreateSecretChat => "CreateSecretChat",
        };
        write!(f, "{}", name)
    }
}

/// Base trait for secret chat events
pub trait SecretChatEvent: LogEvent {
    /// Returns the event type
    fn event_type(&self) -> SecretChatEventType;

    /// Returns the version of this event type
    const VERSION: i32 = LOG_EVENT_VERSION;
}

/// Enum of all secret chat event types
#[derive(Debug, Clone, PartialEq)]
pub enum SecretChatEventEnum {
    /// Inbound secret message
    InboundSecretMessage(InboundSecretMessage),
    /// Outbound secret message
    OutboundSecretMessage(OutboundSecretMessage),
    /// Close secret chat
    CloseSecretChat(CloseSecretChat),
    /// Create secret chat
    CreateSecretChat(CreateSecretChat),
}

impl SecretChatEventEnum {
    /// Parse a secret chat event from a TL parser
    pub fn parse<P: TlParser>(parser: &mut P) -> Result<Self> {
        let version = parser.fetch_i32()?;
        // Use RangeInclusive::contains instead of manual comparison
        if !(3..=LOG_EVENT_VERSION).contains(&version) {
            return Err(LogEventError::InvalidVersion(version));
        }

        let event_type_raw = parser.fetch_i32()?;
        let event_type = SecretChatEventType::from_i32(event_type_raw)?;

        match event_type {
            SecretChatEventType::InboundSecretMessage => Ok(Self::InboundSecretMessage(
                InboundSecretMessage::parse_versioned(parser, version)?,
            )),
            SecretChatEventType::OutboundSecretMessage => Ok(Self::OutboundSecretMessage(
                OutboundSecretMessage::parse(parser)?,
            )),
            SecretChatEventType::CloseSecretChat => Ok(Self::CloseSecretChat(
                CloseSecretChat::parse_versioned(parser, version)?,
            )),
            SecretChatEventType::CreateSecretChat => Ok(Self::CreateSecretChat(
                CreateSecretChat::parse_versioned(parser, version)?,
            )),
        }
    }

    /// Store this event to a TL storer
    pub fn store<S: TlStorer>(&self, storer: &mut S) {
        storer.store_i32(LOG_EVENT_VERSION);
        storer.store_i32(self.event_type() as i32);

        match self {
            Self::InboundSecretMessage(e) => e.store(storer),
            Self::OutboundSecretMessage(e) => e.store(storer),
            Self::CloseSecretChat(e) => e.store(storer),
            Self::CreateSecretChat(e) => e.store(storer),
        }
    }

    /// Returns the event type
    pub fn event_type(&self) -> SecretChatEventType {
        match self {
            Self::InboundSecretMessage(_) => SecretChatEventType::InboundSecretMessage,
            Self::OutboundSecretMessage(_) => SecretChatEventType::OutboundSecretMessage,
            Self::CloseSecretChat(_) => SecretChatEventType::CloseSecretChat,
            Self::CreateSecretChat(_) => SecretChatEventType::CreateSecretChat,
        }
    }
}

impl LogEvent for SecretChatEventEnum {
    fn log_event_id(&self) -> u64 {
        match self {
            Self::InboundSecretMessage(e) => e.log_event_id(),
            Self::OutboundSecretMessage(e) => e.log_event_id(),
            Self::CloseSecretChat(e) => e.log_event_id(),
            Self::CreateSecretChat(e) => e.log_event_id(),
        }
    }

    fn set_log_event_id(&mut self, id: u64) {
        match self {
            Self::InboundSecretMessage(e) => e.set_log_event_id(id),
            Self::OutboundSecretMessage(e) => e.set_log_event_id(id),
            Self::CloseSecretChat(e) => e.set_log_event_id(id),
            Self::CreateSecretChat(e) => e.set_log_event_id(id),
        }
    }
}

/// Type of encrypted input file
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptedInputFileType {
    /// Empty file
    Empty = 0,
    /// Uploaded file
    Uploaded = 1,
    /// Big uploaded file
    BigUploaded = 2,
    /// File location
    Location = 3,
}

impl EncryptedInputFileType {
    /// Parse from i32
    pub fn from_i32(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Uploaded),
            2 => Ok(Self::BigUploaded),
            3 => Ok(Self::Location),
            _ => Err(LogEventError::ParseError(format!(
                "Invalid EncryptedInputFileType: {}",
                value
            ))),
        }
    }
}

/// Encrypted input file for secret chat messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedInputFile {
    /// The magic number (must be ENCRYPTED_INPUT_FILE_MAGIC)
    pub magic: u32,
    /// File type
    pub type_: EncryptedInputFileType,
    /// File ID
    pub id: i64,
    /// Access hash
    pub access_hash: i64,
    /// Number of parts
    pub parts: i32,
    /// Key fingerprint
    pub key_fingerprint: i32,
}

impl Default for EncryptedInputFile {
    fn default() -> Self {
        Self {
            magic: ENCRYPTED_INPUT_FILE_MAGIC,
            type_: EncryptedInputFileType::Empty,
            id: 0,
            access_hash: 0,
            parts: 0,
            key_fingerprint: 0,
        }
    }
}

impl EncryptedInputFile {
    /// Creates a new EncryptedInputFile
    #[must_use]
    pub const fn new(
        type_: EncryptedInputFileType,
        id: i64,
        access_hash: i64,
        parts: i32,
        key_fingerprint: i32,
    ) -> Self {
        Self {
            magic: ENCRYPTED_INPUT_FILE_MAGIC,
            type_,
            id,
            access_hash,
            parts,
            key_fingerprint,
        }
    }

    /// Returns true if this file is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.type_ == EncryptedInputFileType::Empty
    }

    /// Parse from a TL parser
    pub fn parse<P: TlParser>(parser: &mut P) -> Result<Self> {
        let magic = parser.fetch_u32()?;
        let type_ = EncryptedInputFileType::from_i32(parser.fetch_i32()?)?;
        let id = parser.fetch_i64()?;
        let access_hash = parser.fetch_i64()?;
        let parts = parser.fetch_i32()?;
        let key_fingerprint = parser.fetch_i32()?;

        if magic != ENCRYPTED_INPUT_FILE_MAGIC {
            return Err(LogEventError::MagicMismatch {
                expected: ENCRYPTED_INPUT_FILE_MAGIC,
                got: magic,
            });
        }

        Ok(Self {
            magic,
            type_,
            id,
            access_hash,
            parts,
            key_fingerprint,
        })
    }

    /// Store to a TL storer
    pub fn store<S: TlStorer>(&self, storer: &mut S) {
        storer.store_u32(self.magic);
        storer.store_i32(self.type_ as i32);
        storer.store_i64(self.id);
        storer.store_i64(self.access_hash);
        storer.store_i32(self.parts);
        storer.store_i32(self.key_fingerprint);
    }
}

/// Inbound secret message event
#[derive(Debug, Clone, PartialEq)]
pub struct InboundSecretMessage {
    /// Log event ID
    pub log_event_id: u64,
    /// Chat ID
    pub chat_id: i32,
    /// Message date
    pub date: i32,
    /// Auth key ID
    pub auth_key_id: u64,
    /// Message ID
    pub message_id: i32,
    /// My inbound sequence number
    pub my_in_seq_no: i32,
    /// My outbound sequence number
    pub my_out_seq_no: i32,
    /// Their inbound sequence number
    pub his_in_seq_no: i32,
    /// Is message pending
    pub is_pending: bool,
    /// Has encrypted file attachment
    pub has_encrypted_file: bool,
    /// The decrypted message layer data (raw bytes, stub for secret_api types)
    pub decrypted_message_layer: Vec<u8>,
    /// Encrypted file if present
    pub encrypted_file: Option<Vec<u8>>,
}

impl Default for InboundSecretMessage {
    fn default() -> Self {
        Self {
            log_event_id: 0,
            chat_id: 0,
            date: 0,
            auth_key_id: 0,
            message_id: 0,
            my_in_seq_no: -1,
            my_out_seq_no: -1,
            his_in_seq_no: -1,
            is_pending: false,
            has_encrypted_file: false,
            decrypted_message_layer: Vec::new(),
            encrypted_file: None,
        }
    }
}

impl InboundSecretMessage {
    /// Creates a new InboundSecretMessage
    #[must_use]
    pub const fn new(chat_id: i32, date: i32, auth_key_id: u64, message_id: i32) -> Self {
        Self {
            log_event_id: 0,
            chat_id,
            date,
            auth_key_id,
            message_id,
            my_in_seq_no: -1,
            my_out_seq_no: -1,
            his_in_seq_no: -1,
            is_pending: false,
            has_encrypted_file: false,
            decrypted_message_layer: Vec::new(),
            encrypted_file: None,
        }
    }

    fn parse_versioned<P: TlParser>(parser: &mut P, version: i32) -> Result<Self> {
        let has_encrypted_file;
        let is_pending;

        if version >= 2 {
            let mut flags_parser = FlagsParser::new(parser.fetch_u32()?);
            has_encrypted_file = flags_parser.parse_flag();
            is_pending = flags_parser.parse_flag();
            let _no_qts = flags_parser.parse_flag();
            flags_parser.finish()?;
        } else {
            has_encrypted_file = false;
            is_pending = false;
        }

        if version < 2 {
            let _legacy_qts = parser.fetch_i32()?;
        }

        let chat_id = parser.fetch_i32()?;
        let date = parser.fetch_i32()?;

        // Stub: decrypted_message_layer would be parsed by secret_api types
        // For now, read as raw bytes and skip
        let layer_len = parser.fetch_i32()?;
        if layer_len > 0 {
            // In real implementation, this would parse secret_api::decryptedMessageLayer
            // For now, skip the bytes
            let _ = parser.fetch_bytes()?;
        }

        let auth_key_id = parser.fetch_u64()?;
        let message_id = parser.fetch_i32()?;
        let my_in_seq_no = parser.fetch_i32()?;
        let my_out_seq_no = parser.fetch_i32()?;
        let his_in_seq_no = parser.fetch_i32()?;

        let encrypted_file = if has_encrypted_file {
            Some(parser.fetch_bytes()?)
        } else {
            None
        };

        Ok(Self {
            log_event_id: 0,
            chat_id,
            date,
            auth_key_id,
            message_id,
            my_in_seq_no,
            my_out_seq_no,
            his_in_seq_no,
            is_pending,
            has_encrypted_file,
            decrypted_message_layer: Vec::new(),
            encrypted_file,
        })
    }

    /// Parse from a TL parser
    pub fn parse<P: TlParser>(parser: &mut P) -> Result<Self> {
        Self::parse_versioned(parser, LOG_EVENT_VERSION)
    }

    /// Store to a TL storer
    pub fn store<S: TlStorer>(&self, storer: &mut S) {
        let mut flags = FlagsStorer::new();
        flags.store_flag(self.has_encrypted_file);
        flags.store_flag(self.is_pending);
        flags.store_flag(true); // no_qts flag
        let flags_value = flags.finish();

        storer.store_u32(flags_value);
        storer.store_i32(self.chat_id);
        storer.store_i32(self.date);

        // Stub: store decrypted message layer length
        storer.store_i32(self.decrypted_message_layer.len() as i32);
        if !self.decrypted_message_layer.is_empty() {
            storer.store_bytes(&self.decrypted_message_layer);
        }

        storer.store_u64(self.auth_key_id);
        storer.store_i32(self.message_id);
        storer.store_i32(self.my_in_seq_no);
        storer.store_i32(self.my_out_seq_no);
        storer.store_i32(self.his_in_seq_no);

        if let Some(ref file) = self.encrypted_file {
            storer.store_bytes(file);
        }
    }
}

impl LogEvent for InboundSecretMessage {
    fn log_event_id(&self) -> u64 {
        self.log_event_id
    }

    fn set_log_event_id(&mut self, id: u64) {
        self.log_event_id = id;
    }
}

impl SecretChatEvent for InboundSecretMessage {
    fn event_type(&self) -> SecretChatEventType {
        SecretChatEventType::InboundSecretMessage
    }
}

/// Outbound secret message event
#[derive(Debug, Clone, PartialEq)]
pub struct OutboundSecretMessage {
    /// Log event ID
    pub log_event_id: u64,
    /// Chat ID
    pub chat_id: i32,
    /// Random message ID
    pub random_id: i64,
    /// Encrypted message data
    pub encrypted_message: Vec<u8>,
    /// Attached file
    pub file: EncryptedInputFile,
    /// Message ID
    pub message_id: i32,
    /// My inbound sequence number
    pub my_in_seq_no: i32,
    /// My outbound sequence number
    pub my_out_seq_no: i32,
    /// Their inbound sequence number
    pub his_in_seq_no: i32,
    /// Is message sent
    pub is_sent: bool,
    /// Need to notify user
    pub need_notify_user: bool,
    /// Is message rewritable
    pub is_rewritable: bool,
    /// Is external message
    pub is_external: bool,
    /// Is silent message
    pub is_silent: bool,
    /// Action data (stub for secret_api types)
    pub action: Option<Vec<u8>>,
}

impl Default for OutboundSecretMessage {
    fn default() -> Self {
        Self {
            log_event_id: 0,
            chat_id: 0,
            random_id: 0,
            encrypted_message: Vec::new(),
            file: EncryptedInputFile::default(),
            message_id: 0,
            my_in_seq_no: -1,
            my_out_seq_no: -1,
            his_in_seq_no: -1,
            is_sent: false,
            need_notify_user: false,
            is_rewritable: false,
            is_external: false,
            is_silent: false,
            action: None,
        }
    }
}

impl OutboundSecretMessage {
    /// Creates a new OutboundSecretMessage
    #[must_use]
    pub fn new(chat_id: i32, random_id: i64) -> Self {
        Self {
            log_event_id: 0,
            chat_id,
            random_id,
            encrypted_message: Vec::new(),
            file: EncryptedInputFile::default(),
            message_id: 0,
            my_in_seq_no: -1,
            my_out_seq_no: -1,
            his_in_seq_no: -1,
            is_sent: false,
            need_notify_user: false,
            is_rewritable: false,
            is_external: false,
            is_silent: false,
            action: None,
        }
    }

    /// Parse from a TL parser
    pub fn parse<P: TlParser>(parser: &mut P) -> Result<Self> {
        let chat_id = parser.fetch_i32()?;
        let random_id = parser.fetch_i64()?;
        let encrypted_message = parser.fetch_bytes()?;
        let file = EncryptedInputFile::parse(parser)?;
        let message_id = parser.fetch_i32()?;
        let my_in_seq_no = parser.fetch_i32()?;
        let my_out_seq_no = parser.fetch_i32()?;
        let his_in_seq_no = parser.fetch_i32()?;

        let mut flags_parser = FlagsParser::new(parser.fetch_u32()?);
        let is_sent = flags_parser.parse_flag();
        let need_notify_user = flags_parser.parse_flag();
        let has_action = flags_parser.parse_flag();
        let is_rewritable = flags_parser.parse_flag();
        let is_external = flags_parser.parse_flag();
        let is_silent = flags_parser.parse_flag();
        flags_parser.finish()?;

        let action = if has_action {
            // Stub: would parse secret_api::DecryptedMessageAction
            Some(parser.fetch_bytes()?)
        } else {
            None
        };

        Ok(Self {
            log_event_id: 0,
            chat_id,
            random_id,
            encrypted_message,
            file,
            message_id,
            my_in_seq_no,
            my_out_seq_no,
            his_in_seq_no,
            is_sent,
            need_notify_user,
            is_rewritable,
            is_external,
            is_silent,
            action,
        })
    }

    /// Store to a TL storer
    pub fn store<S: TlStorer>(&self, storer: &mut S) {
        storer.store_i32(self.chat_id);
        storer.store_i64(self.random_id);
        storer.store_bytes(&self.encrypted_message);
        self.file.store(storer);
        storer.store_i32(self.message_id);
        storer.store_i32(self.my_in_seq_no);
        storer.store_i32(self.my_out_seq_no);
        storer.store_i32(self.his_in_seq_no);

        let mut flags = FlagsStorer::new();
        flags.store_flag(self.is_sent);
        flags.store_flag(self.need_notify_user);
        flags.store_flag(self.action.is_some());
        flags.store_flag(self.is_rewritable);
        flags.store_flag(self.is_external);
        flags.store_flag(self.is_silent);
        storer.store_u32(flags.finish());

        if let Some(ref action) = self.action {
            storer.store_bytes(action);
        }
    }
}

impl LogEvent for OutboundSecretMessage {
    fn log_event_id(&self) -> u64 {
        self.log_event_id
    }

    fn set_log_event_id(&mut self, id: u64) {
        self.log_event_id = id;
    }
}

impl SecretChatEvent for OutboundSecretMessage {
    fn event_type(&self) -> SecretChatEventType {
        SecretChatEventType::OutboundSecretMessage
    }
}

/// Close secret chat event
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CloseSecretChat {
    /// Log event ID
    pub log_event_id: u64,
    /// Chat ID
    pub chat_id: i32,
    /// Delete chat history
    pub delete_history: bool,
    /// Is already discarded
    pub is_already_discarded: bool,
}

impl CloseSecretChat {
    /// Creates a new CloseSecretChat
    #[must_use]
    pub const fn new(chat_id: i32) -> Self {
        Self {
            log_event_id: 0,
            chat_id,
            delete_history: false,
            is_already_discarded: false,
        }
    }

    fn parse_versioned<P: TlParser>(parser: &mut P, version: i32) -> Result<Self> {
        let (delete_history, is_already_discarded) = if version >= 3 {
            let mut flags_parser = FlagsParser::new(parser.fetch_u32()?);
            let delete_history = flags_parser.parse_flag();
            let is_already_discarded = flags_parser.parse_flag();
            flags_parser.finish()?;
            (delete_history, is_already_discarded)
        } else {
            (false, false)
        };

        let chat_id = parser.fetch_i32()?;

        Ok(Self {
            log_event_id: 0,
            chat_id,
            delete_history,
            is_already_discarded,
        })
    }

    /// Parse from a TL parser
    pub fn parse<P: TlParser>(parser: &mut P) -> Result<Self> {
        Self::parse_versioned(parser, LOG_EVENT_VERSION)
    }

    /// Store to a TL storer
    pub fn store<S: TlStorer>(&self, storer: &mut S) {
        let mut flags = FlagsStorer::new();
        flags.store_flag(self.delete_history);
        flags.store_flag(self.is_already_discarded);
        storer.store_u32(flags.finish());
        storer.store_i32(self.chat_id);
    }
}

impl LogEvent for CloseSecretChat {
    fn log_event_id(&self) -> u64 {
        self.log_event_id
    }

    fn set_log_event_id(&mut self, id: u64) {
        self.log_event_id = id;
    }
}

impl SecretChatEvent for CloseSecretChat {
    fn event_type(&self) -> SecretChatEventType {
        SecretChatEventType::CloseSecretChat
    }
}

/// Create secret chat event
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CreateSecretChat {
    /// Log event ID
    pub log_event_id: u64,
    /// Random ID
    pub random_id: i32,
    /// User ID (stored as i64 for compatibility)
    pub user_id: i64,
    /// User access hash
    pub user_access_hash: i64,
}

impl CreateSecretChat {
    /// Creates a new CreateSecretChat
    #[must_use]
    pub const fn new(random_id: i32, user_id: i64, user_access_hash: i64) -> Self {
        Self {
            log_event_id: 0,
            random_id,
            user_id,
            user_access_hash,
        }
    }

    fn parse_versioned<P: TlParser>(parser: &mut P, version: i32) -> Result<Self> {
        let random_id = parser.fetch_i32()?;
        let user_id = if version >= 4 {
            parser.fetch_i64()?
        } else {
            parser.fetch_i32()? as i64
        };
        let user_access_hash = parser.fetch_i64()?;

        Ok(Self {
            log_event_id: 0,
            random_id,
            user_id,
            user_access_hash,
        })
    }

    /// Parse from a TL parser
    pub fn parse<P: TlParser>(parser: &mut P) -> Result<Self> {
        Self::parse_versioned(parser, LOG_EVENT_VERSION)
    }

    /// Store to a TL storer
    pub fn store<S: TlStorer>(&self, storer: &mut S) {
        storer.store_i32(self.random_id);
        storer.store_i64(self.user_id);
        storer.store_i64(self.user_access_hash);
    }
}

impl LogEvent for CreateSecretChat {
    fn log_event_id(&self) -> u64 {
        self.log_event_id
    }

    fn set_log_event_id(&mut self, id: u64) {
        self.log_event_id = id;
    }
}

impl SecretChatEvent for CreateSecretChat {
    fn event_type(&self) -> SecretChatEventType {
        SecretChatEventType::CreateSecretChat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_from_i32() {
        assert_eq!(
            SecretChatEventType::from_i32(1).unwrap(),
            SecretChatEventType::InboundSecretMessage
        );
        assert_eq!(
            SecretChatEventType::from_i32(2).unwrap(),
            SecretChatEventType::OutboundSecretMessage
        );
        assert_eq!(
            SecretChatEventType::from_i32(3).unwrap(),
            SecretChatEventType::CloseSecretChat
        );
        assert_eq!(
            SecretChatEventType::from_i32(4).unwrap(),
            SecretChatEventType::CreateSecretChat
        );
    }

    #[test]
    fn test_event_type_invalid() {
        assert!(SecretChatEventType::from_i32(99).is_err());
    }

    #[test]
    fn test_encrypted_input_file_default() {
        let file = EncryptedInputFile::default();
        assert_eq!(file.magic, ENCRYPTED_INPUT_FILE_MAGIC);
        assert!(file.is_empty());
    }

    #[test]
    fn test_encrypted_input_file_new() {
        let file = EncryptedInputFile::new(EncryptedInputFileType::Uploaded, 12345, 67890, 3, 42);
        assert_eq!(file.id, 12345);
        assert_eq!(file.access_hash, 67890);
        assert!(!file.is_empty());
    }

    #[test]
    fn test_inbound_secret_message_default() {
        let msg = InboundSecretMessage::default();
        assert_eq!(msg.log_event_id, 0);
        assert_eq!(msg.my_in_seq_no, -1);
    }

    #[test]
    fn test_inbound_secret_message_new() {
        let msg = InboundSecretMessage::new(123, 456, 789, 101);
        assert_eq!(msg.chat_id, 123);
        assert_eq!(msg.date, 456);
        assert_eq!(msg.auth_key_id, 789);
        assert_eq!(msg.message_id, 101);
    }

    #[test]
    fn test_outbound_secret_message_default() {
        let msg = OutboundSecretMessage::default();
        assert_eq!(msg.log_event_id, 0);
        assert!(msg.file.is_empty());
    }

    #[test]
    fn test_outbound_secret_message_new() {
        let msg = OutboundSecretMessage::new(123, 456);
        assert_eq!(msg.chat_id, 123);
        assert_eq!(msg.random_id, 456);
    }

    #[test]
    fn test_close_secret_chat_default() {
        let event = CloseSecretChat::default();
        assert_eq!(event.log_event_id, 0);
        assert!(!event.delete_history);
    }

    #[test]
    fn test_close_secret_chat_new() {
        let event = CloseSecretChat::new(123);
        assert_eq!(event.chat_id, 123);
    }

    #[test]
    fn test_create_secret_chat_default() {
        let event = CreateSecretChat::default();
        assert_eq!(event.log_event_id, 0);
    }

    #[test]
    fn test_create_secret_chat_new() {
        let event = CreateSecretChat::new(123, 456, 789);
        assert_eq!(event.random_id, 123);
        assert_eq!(event.user_id, 456);
        assert_eq!(event.user_access_hash, 789);
    }

    #[test]
    fn test_log_event_trait() {
        let mut msg = InboundSecretMessage::new(123, 456, 789, 101);
        assert_eq!(msg.log_event_id(), 0);
        msg.set_log_event_id(999);
        assert_eq!(msg.log_event_id(), 999);
    }

    #[test]
    fn test_secret_chat_event_trait() {
        let msg = InboundSecretMessage::new(123, 456, 789, 101);
        assert_eq!(msg.event_type(), SecretChatEventType::InboundSecretMessage);
    }

    #[test]
    fn test_event_enum_roundtrip() {
        let event = SecretChatEventEnum::CreateSecretChat(CreateSecretChat::new(123, 456, 789));
        assert_eq!(event.event_type(), SecretChatEventType::CreateSecretChat);
    }

    #[test]
    fn test_event_display() {
        assert_eq!(
            format!("{}", SecretChatEventType::InboundSecretMessage),
            "InboundSecretMessage"
        );
        assert_eq!(
            format!("{}", SecretChatEventType::CloseSecretChat),
            "CloseSecretChat"
        );
    }
}
