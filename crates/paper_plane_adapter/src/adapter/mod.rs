// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Adapter modules for paper_plane integration.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod update_bridge;

pub use update_bridge::{BridgeError, UpdateBridge, UpdateConverter};

// Re-export update types
pub use crate::update::{Update, UpdateBridge as UpdateBridgeType};
