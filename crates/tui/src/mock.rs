//! Mock data layer for TUI development.
//!
//! This module provides mock data types and test data for developing
//! the TUI without requiring a fully functional backend.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Mock dialog representing a chat/conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockDialog {
    /// Unique identifier.
    pub id: i64,
    /// Dialog title (name or chat title).
    pub title: String,
    /// Last message preview.
    pub last_message: String,
    /// Timestamp of last message.
    pub last_message_time: i64,
    /// Number of unread messages.
    pub unread_count: usize,
    /// Whether this dialog is pinned.
    pub is_pinned: bool,
    /// Whether this is a muted dialog.
    pub is_muted: bool,
}

impl MockDialog {
    /// Creates a new mock dialog.
    pub fn new(
        id: i64,
        title: String,
        last_message: String,
        unread_count: usize,
    ) -> Self {
        Self {
            id,
            title,
            last_message,
            last_message_time: now(),
            unread_count,
            is_pinned: false,
            is_muted: false,
        }
    }

    /// Creates a pinned dialog.
    pub fn pinned(mut self) -> Self {
        self.is_pinned = true;
        self
    }

    /// Creates a muted dialog.
    pub fn muted(mut self) -> Self {
        self.is_muted = true;
        self
    }
}

/// Mock message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockMessage {
    /// Unique message ID.
    pub id: i64,
    /// Dialog ID this message belongs to.
    pub dialog_id: i64,
    /// Sender name or display name.
    pub sender: String,
    /// Message text content.
    pub text: String,
    /// Timestamp of the message.
    pub timestamp: i64,
    /// Whether this is an outgoing message.
    pub is_outgoing: bool,
    /// Whether this message has been read.
    pub is_read: bool,
}

impl MockMessage {
    /// Creates a new mock message.
    pub fn new(
        id: i64,
        dialog_id: i64,
        sender: String,
        text: String,
        is_outgoing: bool,
    ) -> Self {
        Self {
            id,
            dialog_id,
            sender,
            text,
            timestamp: now(),
            is_outgoing,
            is_read: false,
        }
    }

    /// Marks message as read.
    pub fn mark_read(mut self) -> Self {
        self.is_read = true;
        self
    }

    /// Creates an incoming message.
    pub fn incoming(id: i64, dialog_id: i64, sender: String, text: String) -> Self {
        Self::new(id, dialog_id, sender, text, false)
    }

    /// Creates an outgoing message.
    pub fn outgoing(id: i64, dialog_id: i64, text: String) -> Self {
        Self::new(id, dialog_id, "You".to_string(), text, true)
    }
}

/// User information for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockUserInfo {
    /// User ID.
    pub id: i64,
    /// Display name.
    pub name: String,
    /// Username/handle.
    pub username: Option<String>,
    /// User status.
    pub status: UserStatus,
    /// User bio.
    pub bio: Option<String>,
}

/// User online status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    /// User is online.
    Online,
    /// User was last seen at a specific time.
    LastSeen(i64),
    /// User is offline (no data available).
    Offline,
}

/// Connection status for the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    /// Disconnected from server.
    #[default]
    Disconnected,
    /// Connecting to server.
    Connecting,
    /// Connected to server.
    Connected,
    /// Connection error.
    Error,
}

impl ConnectionStatus {
    /// Returns the display text for the status.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting...",
            Self::Connected => "Connected",
            Self::Error => "Connection Error",
        }
    }
}

/// Generates mock test dialogs.
pub fn generate_mock_dialogs() -> Vec<MockDialog> {
    vec![
        MockDialog::new(1, "Alice Smith".to_string(), "Hey, how are you?".to_string(), 2)
            .pinned(),
        MockDialog::new(2, "Bob Johnson".to_string(), "See you tomorrow!".to_string(), 0),
        MockDialog::new(3, "Work Group".to_string(), "Alice: Meeting at 3 PM".to_string(), 5),
        MockDialog::new(4, "Family Chat".to_string(), "Mom: Call me when you can".to_string(), 1),
        MockDialog::new(5, "Tech News".to_string(), "New Rust release available!".to_string(), 0)
            .muted(),
        MockDialog::new(6, "Charlie Brown".to_string(), "Thanks for the help!".to_string(), 0),
        MockDialog::new(7, "Study Group".to_string(), "Exam next week".to_string(), 3),
        MockDialog::new(8, "David Lee".to_string(), "Photo".to_string(), 0),
        MockDialog::new(9, "Gaming Squad".to_string(), "Anyone up for a game?".to_string(), 10),
        MockDialog::new(10, "Emma Davis".to_string(), "That sounds great!".to_string(), 0),
    ]
}

/// Generates mock messages for a dialog.
pub fn generate_mock_messages(dialog_id: i64, count: usize) -> Vec<MockMessage> {
    let names = ["Alice", "Bob", "Charlie", "You"];
    let texts = vec![
        "Hello there!",
        "How's it going?",
        "That's interesting",
        "I'll check it out",
        "Thanks for letting me know",
        "See you later!",
        "Have a great day",
        "What do you think?",
        "I agree with you",
        "That makes sense",
    ];

    let mut messages = Vec::new();
    let time = now() - (count as i64 * 3600); // Spread messages over hours

    for i in 0..count {
        let is_outgoing = i % 3 == 0;
        let name_idx = if is_outgoing { 3 } else { i % 3 };
        let text_idx = i % texts.len();

        messages.push(MockMessage {
            id: (dialog_id * 1000 + i as i64),
            dialog_id,
            sender: names[name_idx].to_string(),
            text: texts[text_idx].to_string(),
            timestamp: time + (i as i64 * 300), // 5 minutes apart
            is_outgoing,
            is_read: is_outgoing || i < count - 2, // All except last 2 are read
        });
    }

    messages
}

/// Generates mock user info.
pub fn generate_mock_user_info() -> MockUserInfo {
    MockUserInfo {
        id: 12345,
        name: "Alice Smith".to_string(),
        username: Some("alice_smith".to_string()),
        status: UserStatus::Online,
        bio: Some("Software developer | Rust enthusiast".to_string()),
    }
}

/// Returns current Unix timestamp.
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Formats a timestamp as a readable time.
pub fn format_timestamp(ts: i64) -> String {
    let datetime = chrono::DateTime::from_timestamp(ts, 0);
    match datetime {
        Some(dt) => dt.format("%H:%M").to_string(),
        None => "??:??".to_string(),
    }
}

/// Simulates an incoming message being added.
pub fn simulate_incoming_message(dialogs: &mut [MockDialog], messages: &mut Vec<MockMessage>) {
    if let Some(dialog) = dialogs.first_mut() {
        dialog.last_message = "New message!".to_string();
        dialog.last_message_time = now();
        dialog.unread_count += 1;

        let new_msg = MockMessage {
            id: messages.iter().map(|m| m.id + 1).max().unwrap_or(1),
            dialog_id: dialog.id,
            sender: dialog.title.clone(),
            text: "New message!".to_string(),
            timestamp: now(),
            is_outgoing: false,
            is_read: false,
        };

        messages.push(new_msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_dialog_creation() {
        let dialog = MockDialog::new(1, "Test".to_string(), "Last msg".to_string(), 5);
        assert_eq!(dialog.id, 1);
        assert_eq!(dialog.title, "Test");
        assert_eq!(dialog.unread_count, 5);
        assert!(!dialog.is_pinned);
    }

    #[test]
    fn test_mock_dialog_builder() {
        let dialog = MockDialog::new(1, "Test".to_string(), "Msg".to_string(), 0)
            .pinned()
            .muted();
        assert!(dialog.is_pinned);
        assert!(dialog.is_muted);
    }

    #[test]
    fn test_mock_message_creation() {
        let msg = MockMessage::new(1, 100, "Alice".to_string(), "Hello".to_string(), false);
        assert_eq!(msg.id, 1);
        assert_eq!(msg.dialog_id, 100);
        assert!(!msg.is_outgoing);
    }

    #[test]
    fn test_mock_message_methods() {
        let msg = MockMessage::outgoing(1, 100, "Test".to_string());
        assert!(msg.is_outgoing);
        assert_eq!(msg.sender, "You");

        let msg = MockMessage::incoming(2, 100, "Alice".to_string(), "Hi".to_string());
        assert!(!msg.is_outgoing);
        assert_eq!(msg.sender, "Alice");
    }

    #[test]
    fn test_connection_status_display() {
        assert_eq!(ConnectionStatus::Disconnected.as_str(), "Disconnected");
        assert_eq!(ConnectionStatus::Connecting.as_str(), "Connecting...");
        assert_eq!(ConnectionStatus::Connected.as_str(), "Connected");
        assert_eq!(ConnectionStatus::Error.as_str(), "Connection Error");
    }

    #[test]
    fn test_generate_mock_dialogs() {
        let dialogs = generate_mock_dialogs();
        assert_eq!(dialogs.len(), 10);
        assert!(dialogs[0].is_pinned);
        assert!(dialogs[4].is_muted);
    }

    #[test]
    fn test_generate_mock_messages() {
        let messages = generate_mock_messages(1, 5);
        assert_eq!(messages.len(), 5);
        assert!(messages.iter().all(|m| m.dialog_id == 1));
    }

    #[test]
    fn test_simulate_incoming_message() {
        let mut dialogs = generate_mock_dialogs();
        let mut messages = generate_mock_messages(1, 5);

        let initial_unread = dialogs[0].unread_count;
        simulate_incoming_message(&mut dialogs, &mut messages);

        assert_eq!(dialogs[0].unread_count, initial_unread + 1);
        assert_eq!(messages.len(), 6);
    }

    #[test]
    fn test_user_status() {
        let user = generate_mock_user_info();
        assert_eq!(user.name, "Alice Smith");
        assert!(user.username.is_some());
        assert!(user.bio.is_some());
    }
}
