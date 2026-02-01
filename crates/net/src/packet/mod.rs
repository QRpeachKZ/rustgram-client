// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto packet types.
//!
//! This module provides the core packet types used in MTProto 2.0 protocol,
//! including [`PacketInfo`], [`MessageId`], and [`MtprotoQuery`].
//!
//! # Overview
//!
//! MTProto packets are the fundamental unit of communication in the protocol.
//! Each packet contains metadata (salt, session ID, message ID, sequence number)
//! and optionally encrypted payload data.
//!
//! # References
//!
//! - TDLib: `td/mtproto/PacketInfo.h`
//! - TDLib: `td/mtproto/MessageId.h`
//! - TDLib: `td/mtproto/MtprotoQuery.h`

mod info;
mod message_id;
mod query;

pub use info::{PacketInfo, PacketType};
pub use message_id::MessageId;
pub use query::MtprotoQuery;

/// Prelude for packet module imports.
pub mod prelude {
    pub use super::{MessageId, MtprotoQuery, PacketInfo, PacketType};
}
