//! # Group Call Message
//!
//! Messages sent in group voice/video chats.
//!
//! ## Overview
//!
//! `GroupCallMessage` represents a message sent in a group call, including
//! the sender information, text content, and metadata like star count for paid messages.
//!
//! ## Usage
//!
//! ```
//! use rustgram_group_call_message::GroupCallMessage;
//! use rustgram_dialog_id::DialogId;
//! use rustgram_formatted_text::FormattedText;
//!
//! let sender = DialogId::new(123);
//! let text = FormattedText::plain("Hello!");
//! let message = GroupCallMessage::new(sender, text, 0, false);
//! assert!(message.is_valid());
//! ```

use rustgram_dialog_id::DialogId;
use rustgram_formatted_text::FormattedText;
use core::fmt;

/// A message sent in a group call.
///
/// Group call messages can be text messages or reactions,
/// and may be paid messages (requiring stars to send).
///
/// # Examples
///
/// ```
/// use rustgram_group_call_message::GroupCallMessage;
/// use rustgram_dialog_id::DialogId;
/// use rustgram_formatted_text::FormattedText;
///
/// let sender = DialogId::new(123);
/// let text = FormattedText::plain("Hello world!");
/// let message = GroupCallMessage::new(sender, text, 0, false);
/// assert!(message.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupCallMessage {
    random_id: i64,
    server_id: i32,
    date: i32,
    sender_dialog_id: DialogId,
    text: FormattedText,
    paid_message_star_count: i64,
    from_admin: bool,
    is_local: bool,
}

impl GroupCallMessage {
    /// Creates a new group call message.
    ///
    /// # Arguments
    ///
    /// * `sender_dialog_id` - The sender of the message
    /// * `text` - The formatted text content
    /// * `paid_message_star_count` - Stars paid for this message (0 for free)
    /// * `from_admin` - Whether the sender is an admin
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let sender = DialogId::new(123);
    /// let text = FormattedText::plain("Hi!");
    /// let message = GroupCallMessage::new(sender, text, 100, true);
    /// ```
    pub fn new(
        sender_dialog_id: DialogId,
        text: FormattedText,
        paid_message_star_count: i64,
        from_admin: bool,
    ) -> Self {
        Self {
            random_id: 0,
            server_id: 0,
            date: 0,
            sender_dialog_id,
            text,
            paid_message_star_count,
            from_admin,
            is_local: true,
        }
    }

    /// Returns the random ID of the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
    /// assert_eq!(message.random_id(), 0);
    /// ```
    #[inline]
    pub const fn random_id(&self) -> i64 {
        self.random_id
    }

    /// Returns the server-assigned message ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
    /// assert_eq!(message.server_id(), 0);
    /// ```
    #[inline]
    pub const fn server_id(&self) -> i32 {
        self.server_id
    }

    /// Returns the date when the message was sent.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
    /// assert_eq!(message.date(), 0);
    /// ```
    #[inline]
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns the sender's dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let sender = DialogId::new(456);
    /// let message = GroupCallMessage::new(sender, FormattedText::plain(""), 0, false);
    /// assert_eq!(message.sender_dialog_id(), sender);
    /// ```
    #[inline]
    pub const fn sender_dialog_id(&self) -> DialogId {
        self.sender_dialog_id
    }

    /// Returns the message text content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::plain("Hello");
    /// let message = GroupCallMessage::new(DialogId::new(123), text.clone(), 0, false);
    /// assert_eq!(message.text().text(), "Hello");
    /// ```
    #[inline]
    pub const fn text(&self) -> &FormattedText {
        &self.text
    }

    /// Returns the star count paid for this message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 500, false);
    /// assert_eq!(message.paid_message_star_count(), 500);
    /// ```
    #[inline]
    pub const fn paid_message_star_count(&self) -> i64 {
        self.paid_message_star_count
    }

    /// Returns `true` if the message was sent by an admin.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, true);
    /// assert!(message.is_from_admin());
    /// ```
    #[inline]
    pub const fn is_from_admin(&self) -> bool {
        self.from_admin
    }

    /// Returns `true` if this is a local message (not yet sent to server).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
    /// assert!(message.is_local());
    /// ```
    #[inline]
    pub const fn is_local(&self) -> bool {
        self.is_local
    }

    /// Returns `true` if this message is a reaction (empty text).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
    /// assert!(message.is_reaction());
    /// ```
    #[inline]
    pub fn is_reaction(&self) -> bool {
        self.text.text().is_empty()
    }

    /// Returns `true` if this is a valid message.
    ///
    /// A valid message must have a valid sender dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message::GroupCallMessage;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
    /// assert!(message.is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.sender_dialog_id.is_valid()
    }
}

impl fmt::Display for GroupCallMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GroupCallMessage {{ from: {}, is_admin: {}, is_local: {} }}",
            self.sender_dialog_id, self.from_admin, self.is_local
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let sender = DialogId::new(123);
        let text = FormattedText::plain("Hello");
        let message = GroupCallMessage::new(sender, text, 0, false);
        assert_eq!(message.sender_dialog_id(), sender);
    }

    #[test]
    fn test_random_id() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
        assert_eq!(message.random_id(), 0);
    }

    #[test]
    fn test_server_id() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
        assert_eq!(message.server_id(), 0);
    }

    #[test]
    fn test_date() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
        assert_eq!(message.date(), 0);
    }

    #[test]
    fn test_sender_dialog_id() {
        let sender = DialogId::new(456);
        let message = GroupCallMessage::new(sender, FormattedText::plain(""), 0, false);
        assert_eq!(message.sender_dialog_id(), sender);
    }

    #[test]
    fn test_text() {
        let text = FormattedText::plain("Hello world");
        let message = GroupCallMessage::new(DialogId::new(123), text.clone(), 0, false);
        assert_eq!(message.text().text(), "Hello world");
    }

    #[test]
    fn test_paid_message_star_count() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 500, false);
        assert_eq!(message.paid_message_star_count(), 500);
    }

    #[test]
    fn test_is_from_admin() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, true);
        assert!(message.is_from_admin());
    }

    #[test]
    fn test_is_local() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
        assert!(message.is_local());
    }

    #[test]
    fn test_is_reaction() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
        assert!(message.is_reaction());

        let message2 = GroupCallMessage::new(DialogId::new(123), FormattedText::plain("Hi"), 0, false);
        assert!(!message2.is_reaction());
    }

    #[rstest]
    #[case(1, true)]
    #[case(0, false)]
    fn test_is_valid(#[case] dialog_id: i64, #[case] expected: bool) {
        let message = GroupCallMessage::new(DialogId::new(dialog_id), FormattedText::plain(""), 0, false);
        assert_eq!(message.is_valid(), expected);
    }

    #[test]
    fn test_equality() {
        let sender = DialogId::new(123);
        let text = FormattedText::plain("Test");
        let message1 = GroupCallMessage::new(sender, text.clone(), 0, false);
        let message2 = GroupCallMessage::new(sender, text, 0, false);
        assert_eq!(message1, message2);
    }

    #[test]
    fn test_clone() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain("Hi"), 100, true);
        let cloned = message.clone();
        assert_eq!(message, cloned);
    }

    #[test]
    fn test_display() {
        let message = GroupCallMessage::new(DialogId::new(123), FormattedText::plain(""), 0, false);
        let s = format!("{}", message);
        assert!(s.contains("GroupCallMessage"));
    }
}
