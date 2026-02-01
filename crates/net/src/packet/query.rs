// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto Query representation.
//!
//! Based on TDLib's `td/mtproto/MtprotoQuery.h`.
//!
//! Represents an MTProto query that has been sent and is awaiting a response.

use bytes::Bytes;

use crate::packet::MessageId;

/// An MTProto query that has been sent.
///
/// Contains the message ID, container message ID (if part of a container),
/// the query data, and flags for compression and quick ack.
///
/// # References
///
/// - TDLib: `td/mtproto/MtprotoQuery.h`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtprotoQuery {
    /// The message ID assigned to this query.
    pub message_id: MessageId,

    /// Container message ID if this query is part of a container.
    pub container_message_id: MessageId,

    /// The query data (serialized TL object).
    pub data: Bytes,

    /// Whether the data is gzip compressed.
    pub is_gzip: bool,

    /// Quick ack token (for media messages with quick ack enabled).
    pub quick_ack_token: Option<u64>,
}

impl Default for MtprotoQuery {
    fn default() -> Self {
        Self {
            message_id: MessageId::default(),
            container_message_id: MessageId::default(),
            data: Bytes::new(),
            is_gzip: false,
            quick_ack_token: None,
        }
    }
}

impl MtprotoQuery {
    /// Creates a new `MtprotoQuery` with the given message ID and data.
    #[must_use]
    pub fn new(message_id: MessageId, data: Bytes) -> Self {
        Self {
            message_id,
            container_message_id: MessageId::default(),
            data,
            is_gzip: false,
            quick_ack_token: None,
        }
    }

    /// Creates a new `MtprotoQuery` from a vector of bytes.
    #[must_use]
    pub fn from_vec(message_id: MessageId, data: Vec<u8>) -> Self {
        Self::new(message_id, Bytes::from(data))
    }

    /// Creates a new `MtprotoQuery` from a static slice.
    #[must_use]
    pub fn from_static(message_id: MessageId, data: &'static [u8]) -> Self {
        Self::new(message_id, Bytes::from_static(data))
    }

    /// Sets the container message ID.
    #[must_use]
    pub fn with_container(mut self, container_id: MessageId) -> Self {
        self.container_message_id = container_id;
        self
    }

    /// Sets the gzip flag.
    #[must_use]
    pub const fn with_gzip(mut self, is_gzip: bool) -> Self {
        self.is_gzip = is_gzip;
        self
    }

    /// Sets the quick ack token.
    #[must_use]
    pub fn with_quick_ack(mut self, token: u64) -> Self {
        self.quick_ack_token = Some(token);
        self
    }

    /// Returns true if this query is part of a container.
    #[must_use]
    pub fn has_container(&self) -> bool {
        !self.container_message_id.is_empty()
    }

    /// Returns true if the query expects a quick acknowledgment.
    #[must_use]
    pub fn has_quick_ack(&self) -> bool {
        self.quick_ack_token.is_some()
    }

    /// Returns the length of the query data.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the query data is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a slice of the query data.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtproto_query_default() {
        let query = MtprotoQuery::default();
        assert!(query.message_id.is_empty());
        assert!(query.container_message_id.is_empty());
        assert!(query.data.is_empty());
        assert!(!query.is_gzip);
        assert!(query.quick_ack_token.is_none());
    }

    #[test]
    fn test_mtproto_query_new() {
        let msg_id = MessageId::from_u64(0x62000000_00000001);
        let data = Bytes::from(&b"test data"[..]);
        let query = MtprotoQuery::new(msg_id, data.clone());

        assert_eq!(query.message_id, msg_id);
        assert_eq!(query.data, data);
        assert!(!query.is_gzip);
        assert!(query.quick_ack_token.is_none());
    }

    #[test]
    fn test_mtproto_query_from_vec() {
        let msg_id = MessageId::from_u64(1);
        let data = vec![1, 2, 3, 4];
        let query = MtprotoQuery::from_vec(msg_id, data.clone());

        assert_eq!(query.as_slice(), &data[..]);
    }

    #[test]
    fn test_mtproto_query_from_static() {
        let msg_id = MessageId::from_u64(1);
        let query = MtprotoQuery::from_static(msg_id, b"static data");

        assert_eq!(query.as_slice(), b"static data");
    }

    #[test]
    fn test_mtproto_query_builder() {
        let msg_id = MessageId::from_u64(1);
        let container_id = MessageId::from_u64(2);
        let data = Bytes::new();

        let query = MtprotoQuery::new(msg_id, data)
            .with_container(container_id)
            .with_gzip(true)
            .with_quick_ack(42);

        assert!(query.has_container());
        assert_eq!(query.container_message_id, container_id);
        assert!(query.is_gzip);
        assert!(query.has_quick_ack());
        assert_eq!(query.quick_ack_token, Some(42));
    }

    #[test]
    fn test_mtproto_query_has_container() {
        let msg_id = MessageId::from_u64(1);
        let query = MtprotoQuery::new(msg_id, Bytes::new());
        assert!(!query.has_container());

        let with_container = query.with_container(MessageId::from_u64(2));
        assert!(with_container.has_container());
    }

    #[test]
    fn test_mtproto_query_has_quick_ack() {
        let msg_id = MessageId::from_u64(1);
        let query = MtprotoQuery::new(msg_id, Bytes::new());
        assert!(!query.has_quick_ack());

        let with_ack = query.with_quick_ack(123);
        assert!(with_ack.has_quick_ack());
        assert_eq!(with_ack.quick_ack_token, Some(123));
    }

    #[test]
    fn test_mtproto_query_len() {
        let msg_id = MessageId::from_u64(1);
        let data = Bytes::from(&b"hello"[..]);
        let query = MtprotoQuery::new(msg_id, data);

        assert_eq!(query.len(), 5);
        assert!(!query.is_empty());
    }

    #[test]
    fn test_mtproto_query_is_empty() {
        let msg_id = MessageId::from_u64(1);
        let query = MtprotoQuery::new(msg_id, Bytes::new());
        assert!(query.is_empty());
        assert_eq!(query.len(), 0);
    }

    #[test]
    fn test_mtproto_query_as_slice() {
        let msg_id = MessageId::from_u64(1);
        let data = Bytes::from(&b"test"[..]);
        let query = MtprotoQuery::new(msg_id, data.clone());

        assert_eq!(query.as_slice(), &data[..]);
    }

    #[test]
    fn test_mtproto_query_clone() {
        let msg_id = MessageId::from_u64(1);
        let data = Bytes::from(&b"data"[..]);
        let query1 = MtprotoQuery::new(msg_id, data);
        let query2 = query1.clone();

        assert_eq!(query1, query2);
    }

    #[test]
    fn test_mtproto_query_debug() {
        let msg_id = MessageId::from_u64(1);
        let data = Bytes::from(&b"test"[..]);
        let query = MtprotoQuery::new(msg_id, data);
        let debug_str = format!("{query:?}");

        assert!(debug_str.contains("MtprotoQuery"));
    }
}
