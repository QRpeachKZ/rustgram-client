// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Internal types for business connection manager.

use std::fmt;

use rustgram_net::DcId;
use rustgram_types::{MessageId, UserId};

/// Internal business connection state.
///
/// Represents an active business connection to a bot.
///
/// # Examples
///
/// ```rust
/// use rustgram_business_connection_manager::BusinessConnection;
/// use rustgram_types::UserId;
/// use rustgram_net::DcId;
///
/// let conn = BusinessConnection::new(
///     "conn123".to_string(),
///     UserId::new(12345).expect("valid"),
///     DcId::internal(2),
/// );
///
/// assert_eq!(conn.connection_id(), "conn123");
/// assert_eq!(conn.user_id(), UserId::new(12345).expect("valid"));
/// assert!(!conn.can_send_stars());
/// assert!(!conn.is_deleted());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BusinessConnection {
    /// Unique connection identifier
    connection_id: String,
    /// The user who owns the business account
    user_id: UserId,
    /// Data center for the connection
    dc_id: DcId,
    /// Whether stars can be transferred
    can_send_stars: bool,
    /// Whether the connection is deleted
    pub is_deleted: bool,
}

impl BusinessConnection {
    /// Creates a new business connection.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Unique connection identifier
    /// * `user_id` - The user who owns the business account
    /// * `dc_id` - Data center for the connection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnection;
    /// use rustgram_types::UserId;
    /// use rustgram_net::DcId;
    ///
    /// let conn = BusinessConnection::new(
    ///     "conn123".to_string(),
    ///     UserId::new(12345).expect("valid"),
    ///     DcId::internal(2),
    /// );
    /// ```
    pub fn new(connection_id: String, user_id: UserId, dc_id: DcId) -> Self {
        Self {
            connection_id,
            user_id,
            dc_id,
            can_send_stars: false,
            is_deleted: false,
        }
    }

    /// Returns the connection ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnection;
    /// use rustgram_types::UserId;
    /// use rustgram_net::DcId;
    ///
    /// let conn = BusinessConnection::new(
    ///     "conn123".to_string(),
    ///     UserId::new(12345).expect("valid"),
    ///     DcId::internal(2),
    /// );
    ///
    /// assert_eq!(conn.connection_id(), "conn123");
    /// ```
    pub fn connection_id(&self) -> &str {
        &self.connection_id
    }

    /// Returns the user ID of the business account owner.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnection;
    /// use rustgram_types::UserId;
    /// use rustgram_net::DcId;
    ///
    /// let conn = BusinessConnection::new(
    ///     "conn123".to_string(),
    ///     UserId::new(12345).expect("valid"),
    ///     DcId::internal(2),
    /// );
    ///
    /// assert_eq!(conn.user_id(), UserId::new(12345).expect("valid"));
    /// ```
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns the DC ID for the connection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnection;
    /// use rustgram_types::UserId;
    /// use rustgram_net::DcId;
    ///
    /// let conn = BusinessConnection::new(
    ///     "conn123".to_string(),
    ///     UserId::new(12345).expect("valid"),
    ///     DcId::internal(2),
    /// );
    ///
    /// assert_eq!(conn.dc_id().get_raw_id(), 2);
    /// ```
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns true if stars can be transferred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnection;
    /// use rustgram_types::UserId;
    /// use rustgram_net::DcId;
    ///
    /// let conn = BusinessConnection::new(
    ///     "conn123".to_string(),
    ///     UserId::new(12345).expect("valid"),
    ///     DcId::internal(2),
    /// );
    ///
    /// assert!(!conn.can_send_stars());
    /// ```
    pub fn can_send_stars(&self) -> bool {
        self.can_send_stars
    }

    /// Sets whether stars can be transferred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnection;
    /// use rustgram_types::UserId;
    /// use rustgram_net::DcId;
    ///
    /// let mut conn = BusinessConnection::new(
    ///     "conn123".to_string(),
    ///     UserId::new(12345).expect("valid"),
    ///     DcId::internal(2),
    /// );
    ///
    /// conn.set_can_send_stars(true);
    /// assert!(conn.can_send_stars());
    /// ```
    pub fn set_can_send_stars(&mut self, can_send: bool) {
        self.can_send_stars = can_send;
    }
}

impl fmt::Display for BusinessConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessConnection(id={}, user={}, dc={})",
            self.connection_id,
            self.user_id.get(),
            self.dc_id.get_raw_id()
        )
    }
}

/// Result of sending a single business message.
///
/// Contains information about a sent message.
///
/// # Examples
///
/// ```rust
/// use rustgram_business_connection_manager::BusinessMessage;
/// use rustgram_types::MessageId;
///
/// let msg = BusinessMessage {
///     message_id: MessageId::new(1, 0),
///     date: 1640000000,
/// };
///
/// assert_eq!(msg.message_id, MessageId::new(1, 0));
/// assert_eq!(msg.date, 1640000000);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BusinessMessage {
    /// ID of the sent message
    pub message_id: MessageId,
    /// When the message was sent (Unix timestamp)
    pub date: i32,
}

/// Result of sending multiple business messages (album).
///
/// Contains information about sent messages in an album.
///
/// # Examples
///
/// ```rust
/// use rustgram_business_connection_manager::BusinessMessages;
///
/// let msgs = BusinessMessages {
///     messages: vec![],
///     total_count: 5,
/// };
///
/// assert_eq!(msgs.total_count, 5);
/// assert!(msgs.messages.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BusinessMessages {
    /// List of sent messages
    pub messages: Vec<BusinessMessage>,
    /// Total number of messages
    pub total_count: i32,
}

impl BusinessMessages {
    /// Returns true if there are no messages.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessMessages;
    ///
    /// let msgs = BusinessMessages {
    ///     messages: vec![],
    ///     total_count: 0,
    /// };
    ///
    /// assert!(msgs.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Returns the number of messages.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessMessages;
    /// use rustgram_types::MessageId;
    /// use rustgram_business_connection_manager::BusinessMessage;
    ///
    /// let msgs = BusinessMessages {
    ///     messages: vec![
    ///         BusinessMessage {
    ///             message_id: MessageId::new(1, 0),
    ///             date: 0,
    ///         },
    ///     ],
    ///     total_count: 1,
    /// };
    ///
    /// assert_eq!(msgs.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_connection_new() {
        let conn = BusinessConnection::new(
            "conn123".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        assert_eq!(conn.connection_id(), "conn123");
        assert_eq!(conn.user_id(), UserId::new(12345).expect("valid"));
        assert_eq!(conn.dc_id().get_raw_id(), 2);
        assert!(!conn.can_send_stars());
        assert!(!conn.is_deleted);
    }

    #[test]
    fn test_business_connection_set_can_send_stars() {
        let mut conn = BusinessConnection::new(
            "conn123".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        conn.set_can_send_stars(true);
        assert!(conn.can_send_stars());

        conn.set_can_send_stars(false);
        assert!(!conn.can_send_stars());
    }

    #[test]
    fn test_business_connection_display() {
        let conn = BusinessConnection::new(
            "conn123".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        let display = format!("{}", conn);
        assert!(display.contains("BusinessConnection"));
        assert!(display.contains("conn123"));
        assert!(display.contains("12345"));
        assert!(display.contains("2"));
    }

    #[test]
    fn test_business_connection_clone() {
        let conn1 = BusinessConnection::new(
            "conn123".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        let conn2 = conn1.clone();
        assert_eq!(conn1, conn2);
    }

    #[test]
    fn test_business_connection_equality() {
        let conn1 = BusinessConnection::new(
            "conn123".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        let conn2 = BusinessConnection::new(
            "conn123".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        assert_eq!(conn1, conn2);
    }

    #[test]
    fn test_business_connection_inequality() {
        let conn1 = BusinessConnection::new(
            "conn123".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        let conn2 = BusinessConnection::new(
            "conn456".to_string(),
            UserId::new(12345).expect("valid"),
            DcId::internal(2),
        );

        assert_ne!(conn1, conn2);
    }

    #[test]
    fn test_business_message_new() {
        let msg = BusinessMessage {
            message_id: MessageId::new(1, 0),
            date: 1640000000,
        };

        assert_eq!(msg.message_id, MessageId::new(1, 0));
        assert_eq!(msg.date, 1640000000);
    }

    #[test]
    fn test_business_message_clone() {
        let msg1 = BusinessMessage {
            message_id: MessageId::new(1, 0),
            date: 1640000000,
        };

        let msg2 = msg1.clone();
        assert_eq!(msg1, msg2);
    }

    #[test]
    fn test_business_messages_empty() {
        let msgs = BusinessMessages {
            messages: vec![],
            total_count: 0,
        };

        assert!(msgs.is_empty());
        assert_eq!(msgs.len(), 0);
    }

    #[test]
    fn test_business_messages_with_items() {
        let msgs = BusinessMessages {
            messages: vec![
                BusinessMessage {
                    message_id: MessageId::new(1, 0),
                    date: 0,
                },
                BusinessMessage {
                    message_id: MessageId::new(2, 0),
                    date: 0,
                },
            ],
            total_count: 2,
        };

        assert!(!msgs.is_empty());
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs.total_count, 2);
    }
}
