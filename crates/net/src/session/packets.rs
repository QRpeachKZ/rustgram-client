// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Service packet types for MTProto.
//!
//! This module implements service packets like ping/pong, ack, etc.

use std::fmt;

use bytes::{Buf, Bytes};

/// MTProto service packet.
#[derive(Debug, Clone)]
pub enum ServicePacket {
    /// Require reconnection
    BadMsgNotification {
        bad_msg_id: u64,
        bad_msg_seqno: i32,
        error_code: i32,
        new_server_salt: Option<u64>,
    },

    /// Bad server salt
    BadServerSalt {
        bad_msg_id: u64,
        bad_msg_seqno: i32,
        error_code: i32,
        new_server_salt: u64,
    },

    /// Message acknowledgment
    Ack { msg_ids: Vec<u64> },

    /// Received messages, we should ack them
    MsgsAck { msg_ids: Vec<u64> },

    /// Request for resending messages
    MsgResendReq { msg_ids: Vec<u64> },

    /// Ping response
    Pong(u64),

    /// New session created
    NewSessionCreated {
        first_msg_id: u64,
        server_salt: u64,
        session_id: u64,
    },

    /// Container with messages
    MessageContainer { messages: Vec<ContainerMessage> },

    /// Unknown packet
    Unknown(u32),
}

/// Message in a container.
#[derive(Debug, Clone)]
pub struct ContainerMessage {
    /// Message ID
    pub msg_id: u64,

    /// Sequence number
    pub seqno: i32,

    /// Message bytes
    pub bytes: u32,

    /// Message data
    pub body: Bytes,
}

/// Service packet decoding error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketDecodeError {
    /// Buffer too small
    BufferTooSmall,

    /// Unknown constructor
    UnknownConstructor(u32),

    /// Invalid format
    InvalidFormat,
}

impl fmt::Display for PacketDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferTooSmall => write!(f, "Buffer too small"),
            Self::UnknownConstructor(c) => write!(f, "Unknown constructor: 0x{:08x}", c),
            Self::InvalidFormat => write!(f, "Invalid packet format"),
        }
    }
}

impl std::error::Error for PacketDecodeError {}

// TL constructors
const RPC_ERROR_CONSTRUCTOR: u32 = 0x2144ca19;
const BAD_MSG_NOTIFICATION_CONSTRUCTOR: u32 = 0xa7eff811;
const BAD_SERVER_SALT_CONSTRUCTOR: u32 = 0xedab447b;
const MSGS_ACK_CONSTRUCTOR: u32 = 0x62d6b459;
const MSG_RESEND_REQ_CONSTRUCTOR: u32 = 0x7d861a08;
const PING_CONSTRUCTOR: u32 = 0x7abe77ec;
const PONG_CONSTRUCTOR: u32 = 0x2b0f7de3;
const PING_DELAY_DISCONNECT_CONSTRUCTOR: u32 = 0x34a27b63;
const NEW_SESSION_CREATED_CONSTRUCTOR: u32 = 0x9ec20908;
const MSG_CONTAINER_CONSTRUCTOR: u32 = 0x73f1f8dc;

impl ServicePacket {
    /// Decodes a service packet from bytes.
    pub fn decode(data: &[u8]) -> Result<Self, PacketDecodeError> {
        if data.len() < 4 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let mut cursor = Bytes::copy_from_slice(data);

        let constructor = cursor.get_u32_le();

        match constructor {
            BAD_MSG_NOTIFICATION_CONSTRUCTOR => Self::decode_bad_msg_notification(&mut cursor),
            BAD_SERVER_SALT_CONSTRUCTOR => Self::decode_bad_server_salt(&mut cursor),
            MSGS_ACK_CONSTRUCTOR => Self::decode_msgs_ack(&mut cursor),
            MSG_RESEND_REQ_CONSTRUCTOR => Self::decode_msg_resend_req(&mut cursor),
            PONG_CONSTRUCTOR => Self::decode_pong(&mut cursor),
            NEW_SESSION_CREATED_CONSTRUCTOR => Self::decode_new_session_created(&mut cursor),
            MSG_CONTAINER_CONSTRUCTOR => Self::decode_msg_container(&mut cursor),
            _ => Ok(ServicePacket::Unknown(constructor)),
        }
    }

    fn decode_bad_msg_notification(cursor: &mut Bytes) -> Result<Self, PacketDecodeError> {
        if cursor.remaining() < 20 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let bad_msg_id = cursor.get_u64_le();
        let bad_msg_seqno = cursor.get_i32_le();
        let error_code = cursor.get_i32_le();

        // new_server_salt is optional
        let new_server_salt = if cursor.remaining() >= 8 {
            Some(cursor.get_u64_le())
        } else {
            None
        };

        Ok(ServicePacket::BadMsgNotification {
            bad_msg_id,
            bad_msg_seqno,
            error_code,
            new_server_salt,
        })
    }

    fn decode_bad_server_salt(cursor: &mut Bytes) -> Result<Self, PacketDecodeError> {
        if cursor.remaining() < 28 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let bad_msg_id = cursor.get_u64_le();
        let bad_msg_seqno = cursor.get_i32_le();
        let error_code = cursor.get_i32_le();
        let new_server_salt = cursor.get_u64_le();

        Ok(ServicePacket::BadServerSalt {
            bad_msg_id,
            bad_msg_seqno,
            error_code,
            new_server_salt,
        })
    }

    fn decode_msgs_ack(cursor: &mut Bytes) -> Result<Self, PacketDecodeError> {
        if cursor.remaining() < 4 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let count = cursor.get_u32_le() as usize;
        let mut msg_ids = Vec::with_capacity(count);

        for _ in 0..count {
            if cursor.remaining() < 8 {
                return Err(PacketDecodeError::BufferTooSmall);
            }
            msg_ids.push(cursor.get_u64_le());
        }

        Ok(ServicePacket::Ack { msg_ids })
    }

    fn decode_msg_resend_req(cursor: &mut Bytes) -> Result<Self, PacketDecodeError> {
        if cursor.remaining() < 4 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let count = cursor.get_u32_le() as usize;
        let mut msg_ids = Vec::with_capacity(count);

        for _ in 0..count {
            if cursor.remaining() < 8 {
                return Err(PacketDecodeError::BufferTooSmall);
            }
            msg_ids.push(cursor.get_u64_le());
        }

        Ok(ServicePacket::MsgResendReq { msg_ids })
    }

    fn decode_pong(cursor: &mut Bytes) -> Result<Self, PacketDecodeError> {
        if cursor.remaining() < 8 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let ping_id = cursor.get_u64_le();
        Ok(ServicePacket::Pong(ping_id))
    }

    fn decode_new_session_created(cursor: &mut Bytes) -> Result<Self, PacketDecodeError> {
        if cursor.remaining() < 24 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let first_msg_id = cursor.get_u64_le();
        let server_salt = cursor.get_u64_le();
        let session_id = cursor.get_u64_le();

        Ok(ServicePacket::NewSessionCreated {
            first_msg_id,
            server_salt,
            session_id,
        })
    }

    fn decode_msg_container(cursor: &mut Bytes) -> Result<Self, PacketDecodeError> {
        if cursor.remaining() < 4 {
            return Err(PacketDecodeError::BufferTooSmall);
        }

        let count = cursor.get_u32_le() as usize;
        let mut messages = Vec::with_capacity(count);

        for _ in 0..count {
            if cursor.remaining() < 20 {
                return Err(PacketDecodeError::BufferTooSmall);
            }

            let msg_id = cursor.get_u64_le();
            let seqno = cursor.get_i32_le();
            let bytes = cursor.get_u32_le();

            if cursor.remaining() < bytes as usize {
                return Err(PacketDecodeError::BufferTooSmall);
            }

            let body = cursor.copy_to_bytes(bytes as usize);

            messages.push(ContainerMessage {
                msg_id,
                seqno,
                bytes,
                body,
            });
        }

        Ok(ServicePacket::MessageContainer { messages })
    }
}

/// Message container decoder.
///
/// Decodes msg_container TL type.
pub struct ContainerDecoder;

impl ContainerDecoder {
    /// Decodes a message container.
    pub fn decode(data: &[u8]) -> Result<Vec<ContainerMessage>, PacketDecodeError> {
        match ServicePacket::decode(data)? {
            ServicePacket::MessageContainer { messages } => Ok(messages),
            _ => Err(PacketDecodeError::InvalidFormat),
        }
    }
}

/// Message container type alias.
pub type MessageContainer = Vec<ContainerMessage>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_pong() {
        let ping_id: u64 = 0x123456789ABCDEF0;

        let mut data = Vec::new();
        data.extend_from_slice(&PONG_CONSTRUCTOR.to_le_bytes());
        data.extend_from_slice(&ping_id.to_le_bytes());

        let packet = ServicePacket::decode(&data).unwrap();
        assert!(matches!(packet, ServicePacket::Pong(id) if id == ping_id));
    }

    #[test]
    fn test_decode_ack() {
        let msg_ids: Vec<u64> = vec![1, 2, 3];

        let mut data = Vec::new();
        data.extend_from_slice(&MSGS_ACK_CONSTRUCTOR.to_le_bytes());
        data.extend_from_slice(&(msg_ids.len() as u32).to_le_bytes());
        for msg_id in msg_ids.iter() {
            data.extend_from_slice(&(*msg_id).to_le_bytes());
        }

        let packet = ServicePacket::decode(&data).unwrap();
        assert!(matches!(packet, ServicePacket::Ack { msg_ids: ids } if ids == msg_ids));
    }

    #[test]
    fn test_decode_unknown() {
        let data = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Unknown constructor

        let packet = ServicePacket::decode(&data).unwrap();
        assert!(matches!(packet, ServicePacket::Unknown(0xFFFFFFFF)));
    }

    #[test]
    fn test_decode_buffer_too_small() {
        let data = vec![0x01]; // Too small

        let result = ServicePacket::decode(&data);
        assert!(matches!(result, Err(PacketDecodeError::BufferTooSmall)));
    }

    #[test]
    fn test_decode_bad_msg_notification() {
        let mut data = Vec::new();
        data.extend_from_slice(&BAD_MSG_NOTIFICATION_CONSTRUCTOR.to_le_bytes());
        data.extend_from_slice(&0x123456789ABCDEF0u64.to_le_bytes()); // bad_msg_id
        data.extend_from_slice(&1i32.to_le_bytes()); // bad_msg_seqno
        data.extend_from_slice(&2i32.to_le_bytes()); // error_code

        let packet = ServicePacket::decode(&data).unwrap();

        match packet {
            ServicePacket::BadMsgNotification {
                bad_msg_id,
                bad_msg_seqno,
                error_code,
                ..
            } => {
                assert_eq!(bad_msg_id, 0x123456789ABCDEF0);
                assert_eq!(bad_msg_seqno, 1);
                assert_eq!(error_code, 2);
            }
            _ => panic!("Unexpected packet type"),
        }
    }

    #[test]
    fn test_decode_new_session_created() {
        let mut data = Vec::new();
        data.extend_from_slice(&NEW_SESSION_CREATED_CONSTRUCTOR.to_le_bytes());
        data.extend_from_slice(&1u64.to_le_bytes()); // first_msg_id
        data.extend_from_slice(&2u64.to_le_bytes()); // server_salt
        data.extend_from_slice(&3u64.to_le_bytes()); // session_id

        let packet = ServicePacket::decode(&data).unwrap();

        match packet {
            ServicePacket::NewSessionCreated {
                first_msg_id,
                server_salt,
                session_id,
            } => {
                assert_eq!(first_msg_id, 1);
                assert_eq!(server_salt, 2);
                assert_eq!(session_id, 3);
            }
            _ => panic!("Unexpected packet type"),
        }
    }

    #[test]
    fn test_decode_msg_container() {
        let mut inner_msg = Vec::new();
        inner_msg.extend_from_slice(&0x12345678u32.to_le_bytes()); // Some constructor

        let mut data = Vec::new();
        data.extend_from_slice(&MSG_CONTAINER_CONSTRUCTOR.to_le_bytes());
        data.extend_from_slice(&1u32.to_le_bytes()); // count
        data.extend_from_slice(&1u64.to_le_bytes()); // msg_id
        data.extend_from_slice(&1i32.to_le_bytes()); // seqno
        data.extend_from_slice(&(inner_msg.len() as u32).to_le_bytes()); // bytes
        data.extend_from_slice(&inner_msg);

        let packet = ServicePacket::decode(&data).unwrap();

        match packet {
            ServicePacket::MessageContainer { messages } => {
                assert_eq!(messages.len(), 1);
                assert_eq!(messages[0].msg_id, 1);
                assert_eq!(messages[0].seqno, 1);
            }
            _ => panic!("Unexpected packet type"),
        }
    }
}
