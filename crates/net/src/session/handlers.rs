// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Packet handlers for MTProto sessions.
//!
//! This module implements handlers for processing different packet types.

use crate::packet::PacketInfo;

use super::packets::{PacketDecodeError, ServicePacket};

/// Result of packet handling.
#[derive(Debug, Clone)]
pub enum PacketHandlerResult {
    /// Packet was handled successfully
    Handled,

    /// Packet should be passed to next handler
    Pass,

    /// Packet was handled and a response should be sent
    Respond(Vec<u8>),

    /// Packet handling failed
    Error(String),
}

/// Packet handler trait.
pub trait PacketHandler: Send + Sync {
    /// Handles a packet.
    ///
    /// Returns a PacketHandlerResult indicating what to do next.
    fn handle(&self, packet: &[u8], packet_info: &PacketInfo) -> PacketHandlerResult;

    /// Returns the priority of this handler (lower = earlier).
    fn priority(&self) -> i32 {
        0
    }
}

/// Service packet handler.
///
/// Handles MTProto service packets like ping/pong, ack, etc.
pub struct ServicePacketHandler {
    /// Callback for pong packets
    pong_callback: parking_lot::Mutex<Box<dyn Fn(u64) + Send>>,
}

impl ServicePacketHandler {
    /// Creates a new service packet handler.
    pub fn new() -> Self {
        Self {
            pong_callback: parking_lot::Mutex::new(Box::new(|_| {})),
        }
    }

    /// Sets the pong callback.
    pub fn set_pong_callback<F>(&self, callback: F)
    where
        F: Fn(u64) + Send + 'static,
    {
        *self.pong_callback.lock() = Box::new(callback);
    }
}

impl Default for ServicePacketHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketHandler for ServicePacketHandler {
    fn handle(&self, packet: &[u8], _packet_info: &PacketInfo) -> PacketHandlerResult {
        match ServicePacket::decode(packet) {
            Ok(service_packet) => self.handle_service_packet(service_packet),
            Err(PacketDecodeError::UnknownConstructor(_)) => {
                // Not a service packet, pass to next handler
                PacketHandlerResult::Pass
            }
            Err(e) => PacketHandlerResult::Error(format!("Failed to decode packet: {}", e)),
        }
    }

    fn priority(&self) -> i32 {
        -100 // High priority for service packets
    }
}

impl ServicePacketHandler {
    fn handle_service_packet(&self, packet: ServicePacket) -> PacketHandlerResult {
        match packet {
            ServicePacket::Pong(ping_id) => {
                (self.pong_callback.lock())(ping_id);
                tracing::trace!("Received pong: {}", ping_id);
                PacketHandlerResult::Handled
            }
            ServicePacket::Ack { msg_ids } => {
                tracing::trace!("Received ack for {} messages", msg_ids.len());
                PacketHandlerResult::Handled
            }
            ServicePacket::NewSessionCreated {
                first_msg_id,
                server_salt,
                session_id,
            } => {
                tracing::debug!(
                    "New session created: first_msg_id={}, server_salt={}, session_id={}",
                    first_msg_id,
                    server_salt,
                    session_id
                );
                PacketHandlerResult::Handled
            }
            ServicePacket::BadMsgNotification {
                bad_msg_id,
                error_code,
                ..
            } => {
                tracing::warn!(
                    "Bad message notification: msg_id={}, error_code={}",
                    bad_msg_id,
                    error_code
                );
                PacketHandlerResult::Handled
            }
            ServicePacket::BadServerSalt {
                bad_msg_id,
                new_server_salt,
                ..
            } => {
                tracing::warn!(
                    "Bad server salt: msg_id={}, new_salt={}",
                    bad_msg_id,
                    new_server_salt
                );
                // Should trigger salt update
                PacketHandlerResult::Handled
            }
            ServicePacket::MsgResendReq { msg_ids } => {
                tracing::debug!("Message resend request: {:?} messages", msg_ids.len());
                PacketHandlerResult::Handled
            }
            ServicePacket::MsgsAck { .. } => {
                // We should process these
                PacketHandlerResult::Handled
            }
            ServicePacket::MessageContainer { messages } => {
                tracing::trace!(
                    "Received message container with {} messages",
                    messages.len()
                );
                // Container should be unpacked and each message processed separately
                PacketHandlerResult::Handled
            }
            ServicePacket::Unknown(constructor) => {
                tracing::warn!("Unknown service packet constructor: 0x{:08x}", constructor);
                PacketHandlerResult::Handled
            }
        }
    }
}

/// Chain of packet handlers.
///
/// Processes packets through multiple handlers in priority order.
pub struct PacketHandlerChain {
    /// Handlers in priority order
    handlers: parking_lot::Mutex<Vec<Box<dyn PacketHandler>>>,

    /// Whether to stop after first handled
    stop_on_handled: bool,
}

impl PacketHandlerChain {
    /// Creates a new packet handler chain.
    pub fn new() -> Self {
        Self {
            handlers: parking_lot::Mutex::new(Vec::new()),
            stop_on_handled: true,
        }
    }

    /// Adds a handler to the chain.
    pub fn add_handler(&self, handler: Box<dyn PacketHandler>) {
        let mut handlers = self.handlers.lock();
        handlers.push(handler);
        handlers.sort_by_key(|h| h.priority());
    }

    /// Processes a packet through all handlers.
    pub fn process(&self, packet: &[u8], packet_info: &PacketInfo) -> PacketHandlerResult {
        let handlers = self.handlers.lock();

        for handler in handlers.iter() {
            let result = handler.handle(packet, packet_info);

            match result {
                PacketHandlerResult::Handled => {
                    if self.stop_on_handled {
                        return PacketHandlerResult::Handled;
                    }
                }
                PacketHandlerResult::Pass => continue,
                PacketHandlerResult::Respond(_) | PacketHandlerResult::Error(_) => return result,
            }
        }

        PacketHandlerResult::Handled
    }

    /// Sets whether to stop after first handled.
    pub fn set_stop_on_handled(&mut self, value: bool) {
        self.stop_on_handled = value;
    }
}

impl Default for PacketHandlerChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::PacketType;

    #[test]
    fn test_packet_handler_result_variants() {
        let _ = PacketHandlerResult::Handled;
        let _ = PacketHandlerResult::Pass;
        let _ = PacketHandlerResult::Respond(vec![1, 2, 3]);
        let _ = PacketHandlerResult::Error("test".into());
    }

    #[test]
    fn test_service_packet_handler_new() {
        let handler = ServicePacketHandler::new();
        assert_eq!(handler.priority(), -100);
    }

    #[test]
    fn test_service_packet_handler_pong() {
        let handler = ServicePacketHandler::new();

        let pong_called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let pong_called_clone = pong_called.clone();

        handler.set_pong_callback(move |ping_id| {
            if ping_id == 123 {
                pong_called_clone.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        });

        // Encode a pong packet
        let mut data = Vec::new();
        data.extend_from_slice(&0x2b0f7de3u32.to_le_bytes()); // pong constructor
        data.extend_from_slice(&123u64.to_le_bytes());

        let packet_info = PacketInfo::new();
        let result = handler.handle(&data, &packet_info);

        assert!(matches!(result, PacketHandlerResult::Handled));
        assert!(pong_called.load(std::sync::atomic::Ordering::Relaxed));
    }

    #[test]
    fn test_service_packet_handler_pass_unknown() {
        let handler = ServicePacketHandler::new();

        let data = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Unknown constructor

        let packet_info = PacketInfo::new();
        let result = handler.handle(&data, &packet_info);

        assert!(matches!(result, PacketHandlerResult::Pass));
    }

    #[test]
    fn test_packet_handler_chain_new() {
        let chain = PacketHandlerChain::new();
        assert_eq!(chain.handlers.lock().len(), 0);
    }

    #[test]
    fn test_packet_handler_chain_add() {
        let chain = PacketHandlerChain::new();

        let handler1 = ServicePacketHandler::new();
        let handler2 = ServicePacketHandler::new();

        chain.add_handler(Box::new(handler1));
        chain.add_handler(Box::new(handler2));

        assert_eq!(chain.handlers.lock().len(), 2);
    }

    #[test]
    fn test_packet_handler_chain_process() {
        let chain = PacketHandlerChain::new();
        let handler = ServicePacketHandler::new();

        chain.add_handler(Box::new(handler));

        // Encode a pong packet
        let mut data = Vec::new();
        data.extend_from_slice(&0x2b0f7de3u32.to_le_bytes());
        data.extend_from_slice(&123u64.to_le_bytes());

        let packet_info = PacketInfo::new();
        let result = chain.process(&data, &packet_info);

        assert!(matches!(result, PacketHandlerResult::Handled));
    }
}
