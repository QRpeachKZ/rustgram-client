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

//! # Input Invoice
//!
//! Represents an input reference to an invoice for payments.
//!
//! ## Overview
//!
//! `InputInvoice` represents a reference to an invoice in Telegram.
//! It can be specified by:
//! - A message containing an invoice
//! - A name/slug for the invoice
//! - Telegram-specific payment purposes
//!
//! ## TDLib Correspondence
//!
//! | Rust Type | TDLib Type | TL Schema |
//! |-----------|-----------|-----------|
//! | [`InputInvoice::Message`] | `inputInvoiceMessage` | `td_api.tl:4203` |
//! | [`InputInvoice::Name`] | `inputInvoiceName` | `td_api.tl:4206` |
//! | [`InputInvoice::Telegram`] | `inputInvoiceTelegram` | `td_api.tl:4209` |
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_input_invoice::InputInvoice;
//! use rustgram_types::{ChatId, MessageId};
//!
//! // Via message
//! let chat_id = ChatId(123456);
//! let message_id = MessageId(789);
//! let invoice = InputInvoice::message(chat_id, message_id);
//!
//! // Via name
//! let invoice = InputInvoice::name("my_invoice");
//!
//! // Via Telegram payment purpose
//! let invoice = InputInvoice::telegram(stubs::TelegramPaymentPurpose::premium_gift_code());
//! ```

use rustgram_types::{ChatId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stub module for TelegramPaymentPurpose.
///
/// TODO: Replace with dedicated crate when available.
/// This is a simplified stub containing common payment purpose types.
pub mod stubs {
    use serde::{Deserialize, Serialize};
    use std::fmt;

    /// Telegram payment purpose stub.
    ///
    /// This is a placeholder for the full TelegramPaymentPurpose type.
    /// The full implementation would include variants like:
    /// - PremiumGiftCode
    /// - PremiumGiveaway
    /// - Stars
    /// - etc.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum TelegramPaymentPurpose {
        /// TDLib: `telegramPaymentPurposePremiumGiftCode`
        PremiumGiftCode,

        /// TDLib: `telegramPaymentPurposePremiumGiveaway`
        PremiumGiveaway,

        /// TDLib: `telegramPaymentPurposeStars`
        Stars,

        /// TDLib: `telegramPaymentPurposeStarGift`
        StarGift,

        /// Unknown/other payment purpose
        Other(String),
    }

    impl TelegramPaymentPurpose {
        /// Creates a premium gift code purpose.
        pub fn premium_gift_code() -> Self {
            Self::PremiumGiftCode
        }

        /// Creates a premium giveaway purpose.
        pub fn premium_giveaway() -> Self {
            Self::PremiumGiveaway
        }

        /// Creates a stars purpose.
        pub fn stars() -> Self {
            Self::Stars
        }

        /// Creates a star gift purpose.
        pub fn star_gift() -> Self {
            Self::StarGift
        }

        /// Creates an other payment purpose from a string.
        pub fn other(s: String) -> Self {
            Self::Other(s)
        }

        /// Returns the type name as a string.
        pub fn as_str(&self) -> &str {
            match self {
                Self::PremiumGiftCode => "premium_gift_code",
                Self::PremiumGiveaway => "premium_giveaway",
                Self::Stars => "stars",
                Self::StarGift => "star_gift",
                Self::Other(s) => s,
            }
        }
    }

    impl fmt::Display for TelegramPaymentPurpose {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }
}

/// Input reference to an invoice.
///
/// An invoice can be referenced by:
/// - A message in a chat containing the invoice
/// - A name/slug for the invoice
/// - A Telegram payment purpose
///
/// # Examples
///
/// ```
/// use rustgram_input_invoice::InputInvoice;
/// use rustgram_types::{ChatId, MessageId};
///
/// // Create via message
/// let invoice = InputInvoice::message(ChatId(123), MessageId(456));
/// assert!(matches!(invoice, InputInvoice::Message { .. }));
///
/// // Create via name
/// let invoice = InputInvoice::name("my_invoice");
/// assert!(matches!(invoice, InputInvoice::Name(_)));
///
/// // Create via Telegram payment purpose
/// use rustgram_input_invoice::stubs::TelegramPaymentPurpose;
/// let invoice = InputInvoice::telegram(TelegramPaymentPurpose::stars());
/// assert!(matches!(invoice, InputInvoice::Telegram(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputInvoice {
    /// TDLib: `inputInvoiceMessage`
    ///
    /// An invoice from a message in a chat.
    Message {
        /// The chat containing the message
        chat_id: ChatId,
        /// The message identifier
        message_id: MessageId,
    },

    /// TDLib: `inputInvoiceName`
    ///
    /// An invoice identified by a name/slug.
    Name(String),

    /// TDLib: `inputInvoiceTelegram`
    ///
    /// A Telegram payment (Premium gift codes, Stars, etc.)
    Telegram(stubs::TelegramPaymentPurpose),
}

impl InputInvoice {
    /// Creates an `InputInvoice` from a chat and message ID.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat containing the invoice message
    /// * `message_id` - The message identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::InputInvoice;
    /// use rustgram_types::{ChatId, MessageId};
    ///
    /// let invoice = InputInvoice::message(ChatId(123), MessageId(456));
    /// let (chat, msg) = invoice.as_message().unwrap();
    /// assert_eq!(chat.get(), 123);
    /// assert_eq!(msg.get(), 456);
    /// ```
    pub fn message(chat_id: ChatId, message_id: MessageId) -> Self {
        Self::Message {
            chat_id,
            message_id,
        }
    }

    /// Creates an `InputInvoice` from a name/slug.
    ///
    /// # Arguments
    ///
    /// * `name` - The invoice name/slug
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::InputInvoice;
    ///
    /// let invoice = InputInvoice::name("my_invoice");
    /// assert_eq!(invoice.as_name(), Some("my_invoice"));
    /// ```
    pub fn name(name: &str) -> Self {
        Self::Name(name.to_string())
    }

    /// Creates an `InputInvoice` from a Telegram payment purpose.
    ///
    /// # Arguments
    ///
    /// * `purpose` - The Telegram payment purpose
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::{InputInvoice, stubs::TelegramPaymentPurpose};
    ///
    /// let invoice = InputInvoice::telegram(TelegramPaymentPurpose::stars());
    /// assert!(invoice.as_telegram().is_some());
    /// ```
    pub fn telegram(purpose: stubs::TelegramPaymentPurpose) -> Self {
        Self::Telegram(purpose)
    }

    /// Returns `true` if this is a message variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::InputInvoice;
    /// use rustgram_types::{ChatId, MessageId};
    ///
    /// let invoice = InputInvoice::message(ChatId(123), MessageId(456));
    /// assert!(invoice.is_message());
    /// ```
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message { .. })
    }

    /// Returns `true` if this is a name variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::InputInvoice;
    ///
    /// let invoice = InputInvoice::name("test");
    /// assert!(invoice.is_name());
    /// ```
    pub fn is_name(&self) -> bool {
        matches!(self, Self::Name(_))
    }

    /// Returns `true` if this is a Telegram variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::{InputInvoice, stubs::TelegramPaymentPurpose};
    ///
    /// let invoice = InputInvoice::telegram(TelegramPaymentPurpose::stars());
    /// assert!(invoice.is_telegram());
    /// ```
    pub fn is_telegram(&self) -> bool {
        matches!(self, Self::Telegram(_))
    }

    /// Returns the chat and message ID if this is a message variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::InputInvoice;
    /// use rustgram_types::{ChatId, MessageId};
    ///
    /// let invoice = InputInvoice::message(ChatId(123), MessageId(456));
    /// let (chat, msg) = invoice.as_message().unwrap();
    /// assert_eq!(chat.get(), 123);
    /// assert_eq!(msg.get(), 456);
    /// ```
    pub fn as_message(&self) -> Option<(ChatId, MessageId)> {
        match self {
            Self::Message {
                chat_id,
                message_id,
            } => Some((*chat_id, *message_id)),
            _ => None,
        }
    }

    /// Returns the name if this is a name variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::InputInvoice;
    ///
    /// let invoice = InputInvoice::name("my_invoice");
    /// assert_eq!(invoice.as_name(), Some("my_invoice"));
    /// ```
    pub fn as_name(&self) -> Option<&str> {
        match self {
            Self::Name(name) => Some(name),
            _ => None,
        }
    }

    /// Returns the payment purpose if this is a Telegram variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_input_invoice::{InputInvoice, stubs::TelegramPaymentPurpose};
    ///
    /// let invoice = InputInvoice::telegram(TelegramPaymentPurpose::stars());
    /// assert!(invoice.as_telegram().is_some());
    /// ```
    pub fn as_telegram(&self) -> Option<&stubs::TelegramPaymentPurpose> {
        match self {
            Self::Telegram(purpose) => Some(purpose),
            _ => None,
        }
    }
}

impl fmt::Display for InputInvoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message {
                chat_id,
                message_id,
            } => write!(f, "invoice:message:{}/{}", chat_id.get(), message_id.get()),
            Self::Name(name) => write!(f, "invoice:name:{}", name),
            Self::Telegram(purpose) => write!(f, "invoice:telegram:{}", purpose),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_message_invoice() -> InputInvoice {
        InputInvoice::message(ChatId(123456), MessageId(789012))
    }

    #[test]
    fn test_message_variant() {
        let invoice = create_test_message_invoice();
        assert!(invoice.is_message());
        assert!(!invoice.is_name());
        assert!(!invoice.is_telegram());
        assert_eq!(
            invoice.as_message(),
            Some((ChatId(123456), MessageId(789012)))
        );
        assert_eq!(invoice.as_name(), None);
        assert_eq!(invoice.as_telegram(), None);
    }

    #[test]
    fn test_name_variant() {
        let invoice = InputInvoice::name("test_invoice");
        assert!(!invoice.is_message());
        assert!(invoice.is_name());
        assert!(!invoice.is_telegram());
        assert_eq!(invoice.as_message(), None);
        assert_eq!(invoice.as_name(), Some("test_invoice"));
        assert_eq!(invoice.as_telegram(), None);
    }

    #[test]
    fn test_telegram_variant() {
        let invoice = InputInvoice::telegram(stubs::TelegramPaymentPurpose::stars());
        assert!(!invoice.is_message());
        assert!(!invoice.is_name());
        assert!(invoice.is_telegram());
        assert_eq!(invoice.as_message(), None);
        assert_eq!(invoice.as_name(), None);
        assert!(invoice.as_telegram().is_some());
    }

    #[test]
    fn test_message_equality() {
        let invoice1 = InputInvoice::message(ChatId(123), MessageId(456));
        let invoice2 = InputInvoice::message(ChatId(123), MessageId(456));
        assert_eq!(invoice1, invoice2);

        let invoice3 = InputInvoice::message(ChatId(123), MessageId(789));
        assert_ne!(invoice1, invoice3);
    }

    #[test]
    fn test_name_equality() {
        let invoice1 = InputInvoice::name("test");
        let invoice2 = InputInvoice::name("test");
        assert_eq!(invoice1, invoice2);

        let invoice3 = InputInvoice::name("other");
        assert_ne!(invoice1, invoice3);
    }

    #[test]
    fn test_telegram_equality() {
        let invoice1 = InputInvoice::telegram(stubs::TelegramPaymentPurpose::stars());
        let invoice2 = InputInvoice::telegram(stubs::TelegramPaymentPurpose::stars());
        assert_eq!(invoice1, invoice2);

        let invoice3 = InputInvoice::telegram(stubs::TelegramPaymentPurpose::star_gift());
        assert_ne!(invoice1, invoice3);
    }

    #[test]
    fn test_cross_variant_inequality() {
        let message = InputInvoice::message(ChatId(123), MessageId(456));
        let name = InputInvoice::name("test");
        let telegram = InputInvoice::telegram(stubs::TelegramPaymentPurpose::stars());

        assert_ne!(message, name);
        assert_ne!(message, telegram);
        assert_ne!(name, telegram);
    }

    #[test]
    fn test_clone() {
        let invoice = create_test_message_invoice();
        let cloned = invoice.clone();
        assert_eq!(invoice, cloned);
    }

    #[test]
    fn test_debug() {
        let invoice = create_test_message_invoice();
        let debug = format!("{:?}", invoice);
        assert!(debug.contains("Message"));
    }

    #[test]
    fn test_display_message() {
        let invoice = InputInvoice::message(ChatId(123), MessageId(456));
        assert_eq!(format!("{}", invoice), "invoice:message:123/456");
    }

    #[test]
    fn test_display_name() {
        let invoice = InputInvoice::name("my_invoice");
        assert_eq!(format!("{}", invoice), "invoice:name:my_invoice");
    }

    #[test]
    fn test_display_telegram() {
        let invoice = InputInvoice::telegram(stubs::TelegramPaymentPurpose::stars());
        assert_eq!(format!("{}", invoice), "invoice:telegram:stars");
    }

    #[test]
    fn test_serialization_message() {
        let invoice = InputInvoice::message(ChatId(123), MessageId(456));
        let json = serde_json::to_string(&invoice).unwrap();
        let parsed: InputInvoice = serde_json::from_str(&json).unwrap();
        assert_eq!(invoice, parsed);
    }

    #[test]
    fn test_serialization_name() {
        let invoice = InputInvoice::name("test");
        let json = serde_json::to_string(&invoice).unwrap();
        let parsed: InputInvoice = serde_json::from_str(&json).unwrap();
        assert_eq!(invoice, parsed);
    }

    #[test]
    fn test_serialization_telegram() {
        let invoice = InputInvoice::telegram(stubs::TelegramPaymentPurpose::stars());
        let json = serde_json::to_string(&invoice).unwrap();
        let parsed: InputInvoice = serde_json::from_str(&json).unwrap();
        assert_eq!(invoice, parsed);
    }

    #[test]
    fn test_empty_name() {
        let invoice = InputInvoice::name("");
        assert!(invoice.is_name());
        assert_eq!(invoice.as_name(), Some(""));
    }

    #[test]
    fn test_telegram_premium_gift_code() {
        let invoice = InputInvoice::telegram(stubs::TelegramPaymentPurpose::premium_gift_code());
        assert!(matches!(
            invoice.as_telegram(),
            Some(stubs::TelegramPaymentPurpose::PremiumGiftCode)
        ));
    }

    #[test]
    fn test_telegram_premium_giveaway() {
        let invoice = InputInvoice::telegram(stubs::TelegramPaymentPurpose::premium_giveaway());
        assert!(matches!(
            invoice.as_telegram(),
            Some(stubs::TelegramPaymentPurpose::PremiumGiveaway)
        ));
    }

    #[test]
    fn test_telegram_star_gift() {
        let invoice = InputInvoice::telegram(stubs::TelegramPaymentPurpose::star_gift());
        assert!(matches!(
            invoice.as_telegram(),
            Some(stubs::TelegramPaymentPurpose::StarGift)
        ));
    }

    #[test]
    fn test_telegram_other() {
        let invoice =
            InputInvoice::telegram(stubs::TelegramPaymentPurpose::other("custom".to_string()));
        assert!(matches!(
            invoice.as_telegram(),
            Some(stubs::TelegramPaymentPurpose::Other(_))
        ));
        if let Some(stubs::TelegramPaymentPurpose::Other(s)) = invoice.as_telegram() {
            assert_eq!(s, "custom");
        }
    }

    #[test]
    fn test_telegram_purpose_as_str() {
        assert_eq!(stubs::TelegramPaymentPurpose::stars().as_str(), "stars");
        assert_eq!(
            stubs::TelegramPaymentPurpose::premium_gift_code().as_str(),
            "premium_gift_code"
        );
        assert_eq!(
            stubs::TelegramPaymentPurpose::other("custom".to_string()).as_str(),
            "custom"
        );
    }

    #[test]
    fn test_telegram_purpose_display() {
        assert_eq!(
            format!("{}", stubs::TelegramPaymentPurpose::stars()),
            "stars"
        );
    }

    #[test]
    fn test_zero_ids_message() {
        let invoice = InputInvoice::message(ChatId(0), MessageId(0));
        assert!(invoice.is_message());
        let (chat, msg) = invoice.as_message().unwrap();
        assert_eq!(chat.get(), 0);
        assert_eq!(msg.get(), 0);
    }

    #[test]
    fn test_name_with_special_chars() {
        let name = "invoice-123_test";
        let invoice = InputInvoice::name(name);
        assert_eq!(invoice.as_name(), Some(name));
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let invoice1 = InputInvoice::name("test");
        let invoice2 = InputInvoice::name("test");

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        invoice1.hash(&mut h1);
        invoice2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_message_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let invoice1 = InputInvoice::message(ChatId(123), MessageId(456));
        let invoice2 = InputInvoice::message(ChatId(123), MessageId(456));

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        invoice1.hash(&mut h1);
        invoice2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_different_names_different_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let invoice1 = InputInvoice::name("test1");
        let invoice2 = InputInvoice::name("test2");

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        invoice1.hash(&mut h1);
        invoice2.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }
}
