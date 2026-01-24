// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0

//! Message reply header for thread replies.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Reply header for thread replies.
///
/// This is an empty stub placeholder for TDLib's MessageReplyHeader.
/// Thread reply functionality is deferred to a future phase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageReplyHeader {
    _empty: (),
}

impl MessageReplyHeader {
    /// Creates a new empty reply header.
    pub const fn new() -> Self {
        Self { _empty: () }
    }
}

impl fmt::Display for MessageReplyHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reply header")
    }
}
