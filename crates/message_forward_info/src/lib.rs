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

//! # Message Forward Info
//!
//! Information about a forwarded message.
//!
//! ## TDLib Alignment
//!
//! This type aligns with TDLib's `MessageForwardInfo` struct.
//! - TDLib header: `td/telegram/MessageForwardInfo.h`
//! - TDLib type: Struct with MessageOrigin, date, LastForwardedMessageInfo, psa_type, is_imported
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_forward_info::{MessageForwardInfo, LastForwardedMessageInfo};
//! use rustgram_message_origin::MessageOrigin;
//! use rustgram_types::{DialogId, MessageId, UserId};
//!
//! let origin = MessageOrigin::new(
//!     UserId::new(123).ok(),
//!     Some(DialogId::default()),
//!     MessageId::from_server_id(100),
//! );
//!
//! let forward_info = MessageForwardInfo::new(origin, 12345);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

use rustgram_message_origin::MessageOrigin;
use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Information about the last forwarded message in a chain.
///
/// Contains details about the most recent forward in a forward chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LastForwardedMessageInfo {
    /// Dialog where the message was forwarded from
    dialog_id: DialogId,

    /// Message ID of the forwarded message
    message_id: MessageId,

    /// Dialog of the sender who forwarded the message
    sender_dialog_id: DialogId,

    /// Name of the sender (if hidden)
    sender_name: String,

    /// Date when the message was forwarded
    date: i32,

    /// Whether this is an outgoing message
    is_outgoing: bool,
}

impl LastForwardedMessageInfo {
    /// Creates a new LastForwardedMessageInfo.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog where the message was forwarded from
    /// * `message_id` - Message ID of the forwarded message
    /// * `sender_dialog_id` - Dialog of the sender who forwarded the message
    /// * `sender_name` - Name of the sender (if hidden)
    /// * `date` - Date when the message was forwarded
    /// * `is_outgoing` - Whether this is an outgoing message
    #[must_use]
    pub fn new(
        dialog_id: DialogId,
        message_id: MessageId,
        sender_dialog_id: DialogId,
        sender_name: String,
        date: i32,
        is_outgoing: bool,
    ) -> Self {
        Self {
            dialog_id,
            message_id,
            sender_dialog_id,
            sender_name,
            date,
            is_outgoing,
        }
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the message ID.
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Returns the sender dialog ID.
    #[must_use]
    pub const fn sender_dialog_id(&self) -> DialogId {
        self.sender_dialog_id
    }

    /// Returns the sender name.
    #[must_use]
    pub fn sender_name(&self) -> &str {
        &self.sender_name
    }

    /// Returns the date.
    #[must_use]
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns `true` if this is an outgoing message.
    #[must_use]
    pub const fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    /// Returns `true` if this info is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.dialog_id.is_valid()
            && !self.message_id.is_valid()
            && !self.sender_dialog_id.is_valid()
            && self.sender_name.is_empty()
    }

    /// Returns `true` if there's a sender name.
    #[must_use]
    pub fn has_sender_name(&self) -> bool {
        !self.sender_name.is_empty()
    }
}

impl fmt::Display for LastForwardedMessageInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "no last forward info");
        }

        write!(f, "from {} msg {}", self.dialog_id, self.message_id.get())?;

        if self.is_outgoing {
            write!(f, " (outgoing)")?;
        }

        if self.has_sender_name() {
            write!(f, " by {}", self.sender_name)?;
        }

        Ok(())
    }
}

/// Information about a forwarded message.
///
/// Contains comprehensive forward information including:
/// - Origin of the message (who originally sent it)
/// - Date when the original message was sent
/// - Last forwarded message info (for forward chains)
/// - PSA type (for public service announcements)
/// - Import status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageForwardInfo {
    /// Origin of the message
    origin: MessageOrigin,

    /// Date when the original message was sent
    date: i32,

    /// Last forwarded message info (for forward chains)
    last_message_info: LastForwardedMessageInfo,

    /// PSA type (for public service announcements)
    psa_type: String,

    /// Whether this message was imported from another app
    is_imported: bool,
}

impl MessageForwardInfo {
    /// Creates a new MessageForwardInfo.
    ///
    /// # Arguments
    ///
    /// * `origin` - Origin of the message
    /// * `date` - Date when the original message was sent
    #[must_use]
    pub fn new(origin: MessageOrigin, date: i32) -> Self {
        Self {
            origin,
            date,
            last_message_info: LastForwardedMessageInfo::default(),
            psa_type: String::new(),
            is_imported: false,
        }
    }

    /// Creates a new MessageForwardInfo with last message info.
    ///
    /// # Arguments
    ///
    /// * `origin` - Origin of the message
    /// * `date` - Date when the original message was sent
    /// * `last_message_info` - Last forwarded message info
    /// * `psa_type` - PSA type
    /// * `is_imported` - Whether this message was imported
    #[must_use]
    pub fn with_last_message_info(
        origin: MessageOrigin,
        date: i32,
        last_message_info: LastForwardedMessageInfo,
        psa_type: String,
        is_imported: bool,
    ) -> Self {
        Self {
            origin,
            date,
            last_message_info,
            psa_type,
            is_imported,
        }
    }

    /// Returns the origin.
    #[must_use]
    pub const fn origin(&self) -> &MessageOrigin {
        &self.origin
    }

    /// Returns the date.
    #[must_use]
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns the origin date.
    #[must_use]
    pub const fn origin_date(&self) -> i32 {
        self.date
    }

    /// Returns the last message info.
    #[must_use]
    pub const fn last_message_info(&self) -> &LastForwardedMessageInfo {
        &self.last_message_info
    }

    /// Returns the PSA type.
    #[must_use]
    pub fn psa_type(&self) -> &str {
        &self.psa_type
    }

    /// Returns `true` if this message was imported.
    #[must_use]
    pub const fn is_imported(&self) -> bool {
        self.is_imported
    }

    /// Returns the origin dialog ID.
    #[must_use]
    pub fn origin_dialog_id(&self) -> DialogId {
        self.origin.get_sender()
    }

    /// Returns the last dialog ID.
    #[must_use]
    pub const fn last_dialog_id(&self) -> DialogId {
        self.last_message_info.dialog_id
    }

    /// Returns `true` if there's a sender name in last message info.
    #[must_use]
    pub fn has_last_sender_name(&self) -> bool {
        self.last_message_info.has_sender_name()
    }

    /// Sets the origin.
    pub fn set_origin(&mut self, origin: MessageOrigin) {
        self.origin = origin;
    }

    /// Sets the date.
    pub fn set_date(&mut self, date: i32) {
        self.date = date;
    }

    /// Sets the PSA type.
    pub fn set_psa_type(&mut self, psa_type: String) {
        self.psa_type = psa_type;
    }

    /// Sets the imported flag.
    pub fn set_imported(&mut self, imported: bool) {
        self.is_imported = imported;
    }
}

impl fmt::Display for MessageForwardInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "forwarded from {}", self.origin)?;

        if self.date > 0 {
            write!(f, " at {}", self.date)?;
        }

        if self.is_imported {
            write!(f, " (imported)")?;
        }

        if !self.psa_type.is_empty() {
            write!(f, " [PSA: {}]", self.psa_type)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    // LastForwardedMessageInfo tests (8)
    #[test]
    fn test_last_forwarded_new() {
        let info = LastForwardedMessageInfo::new(
            DialogId::default(),
            MessageId::from_server_id(100),
            DialogId::default(),
            String::new(),
            12345,
            false,
        );

        assert_eq!(info.message_id(), MessageId::from_server_id(100));
        assert_eq!(info.date(), 12345);
    }

    #[test]
    fn test_last_forwarded_default() {
        let info = LastForwardedMessageInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_last_forwarded_is_empty() {
        let info = LastForwardedMessageInfo::new(
            DialogId::default(),
            MessageId::default(),
            DialogId::default(),
            String::new(),
            0,
            false,
        );

        assert!(info.is_empty());
    }

    #[test]
    fn test_last_forwarded_is_not_empty() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = LastForwardedMessageInfo::new(
            dialog_id,
            MessageId::from_server_id(100),
            DialogId::default(),
            String::new(),
            12345,
            false,
        );

        assert!(!info.is_empty());
    }

    #[test]
    fn test_last_forwarded_has_sender_name() {
        let info = LastForwardedMessageInfo::new(
            DialogId::default(),
            MessageId::default(),
            DialogId::default(),
            "sender".to_string(),
            0,
            false,
        );

        assert!(info.has_sender_name());
    }

    #[test]
    fn test_last_forwarded_is_outgoing() {
        let info = LastForwardedMessageInfo::new(
            DialogId::default(),
            MessageId::default(),
            DialogId::default(),
            String::new(),
            0,
            true,
        );

        assert!(info.is_outgoing());
    }

    #[test]
    fn test_last_forwarded_clone() {
        let info1 = LastForwardedMessageInfo::new(
            DialogId::default(),
            MessageId::from_server_id(100),
            DialogId::default(),
            String::new(),
            12345,
            false,
        );

        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_last_forwarded_display() {
        let info = LastForwardedMessageInfo::default();
        let display = format!("{}", info);
        assert!(display.contains("no last forward info"));
    }

    // MessageForwardInfo constructor tests (2)
    #[test]
    fn test_forward_new() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin.clone(), 12345);
        assert_eq!(info.date(), 12345);
        assert!(!info.is_imported());
    }

    #[test]
    fn test_forward_default() {
        let info = MessageForwardInfo::default();
        assert_eq!(info.date(), 0);
    }

    // Property tests (10)
    #[test]
    fn test_origin() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin.clone(), 12345);
        assert_eq!(info.origin().sender_user_id(), UserId::new(123).ok());
    }

    #[test]
    fn test_date() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert_eq!(info.date(), 12345);
    }

    #[test]
    fn test_origin_date() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert_eq!(info.origin_date(), 12345);
    }

    #[test]
    fn test_last_message_info() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert!(info.last_message_info().is_empty());
    }

    #[test]
    fn test_psa_type() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert_eq!(info.psa_type(), "");
    }

    #[test]
    fn test_is_imported() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert!(!info.is_imported());
    }

    #[test]
    fn test_origin_dialog_id() {
        let user_id = UserId::new(123).unwrap();
        let origin = MessageOrigin::new(
            Some(user_id),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert_eq!(info.origin_dialog_id(), DialogId::from(user_id));
    }

    #[test]
    fn test_last_dialog_id() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert!(!info.last_dialog_id().is_valid());
    }

    #[test]
    fn test_has_last_sender_name() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        assert!(!info.has_last_sender_name());
    }

    #[test]
    fn test_with_last_message_info() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let last_info = LastForwardedMessageInfo::new(
            DialogId::default(),
            MessageId::from_server_id(200),
            DialogId::default(),
            "sender".to_string(),
            12345,
            false,
        );

        let info = MessageForwardInfo::with_last_message_info(
            origin,
            12345,
            last_info,
            "psa".to_string(),
            true,
        );

        assert!(info.is_imported());
        assert!(info.has_last_sender_name());
        assert_eq!(info.psa_type(), "psa");
    }

    // Method tests (4)
    #[test]
    fn test_set_origin() {
        let mut info = MessageForwardInfo::default();

        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        info.set_origin(origin.clone());
        assert_eq!(info.origin().sender_user_id(), UserId::new(123).ok());
    }

    #[test]
    fn test_set_date() {
        let mut info = MessageForwardInfo::default();
        info.set_date(12345);
        assert_eq!(info.date(), 12345);
    }

    #[test]
    fn test_set_psa_type() {
        let mut info = MessageForwardInfo::default();
        info.set_psa_type("test_psa".to_string());
        assert_eq!(info.psa_type(), "test_psa");
    }

    #[test]
    fn test_set_imported() {
        let mut info = MessageForwardInfo::default();
        info.set_imported(true);
        assert!(info.is_imported());
    }

    // Clone tests (2)
    #[test]
    fn test_clone() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info1 = MessageForwardInfo::new(origin, 12345);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_clone_independence() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let mut info1 = MessageForwardInfo::new(origin, 12345);
        let info2 = info1.clone();
        info1.set_date(99999);

        assert_eq!(info2.date(), 12345);
    }

    // Display tests (2)
    #[test]
    fn test_display() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        let display = format!("{}", info);
        assert!(display.contains("forwarded"));
    }

    #[test]
    fn test_display_imported() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        let mut imported_info = info;
        imported_info.set_imported(true);

        let display = format!("{}", imported_info);
        assert!(display.contains("imported"));
    }

    // Serialization tests (3)
    #[test]
    fn test_serialize_forward() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::new(origin, 12345);
        let json = serde_json::to_string(&info).unwrap();
        let parsed: MessageForwardInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, parsed);
    }

    #[test]
    fn test_serialize_last_info() {
        let info = LastForwardedMessageInfo::new(
            DialogId::default(),
            MessageId::from_server_id(100),
            DialogId::default(),
            "sender".to_string(),
            12345,
            true,
        );

        let json = serde_json::to_string(&info).unwrap();
        let parsed: LastForwardedMessageInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, parsed);
    }

    #[test]
    fn test_serialize_with_psa() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            Some(DialogId::default()),
            MessageId::from_server_id(100),
        );

        let info = MessageForwardInfo::with_last_message_info(
            origin,
            12345,
            LastForwardedMessageInfo::default(),
            "psa_type".to_string(),
            true,
        );

        let json = serde_json::to_string(&info).unwrap();
        let parsed: MessageForwardInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, parsed);
    }
}
