// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Query dispatching module.
//!
//! This module implements enhanced query dispatching for MTProto.

mod delayer;
mod dispatcher;
mod rate_limit;
mod sequence;

pub use delayer::{DelayConfig, NetQueryDelayer};
pub use dispatcher::{DispatchConfig, EnhancedDispatcher};
pub use rate_limit::{FloodControl, FloodControlConfig, FloodControlResult};
pub use sequence::{SequenceConfig, SequenceDispatcher};
