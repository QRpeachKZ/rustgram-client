// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! Bot command type for Telegram MTProto client.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::hash::{Hash, Hasher};

/// Bot command type. Commands must start with '/'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotCommand { command: String, description: String }

impl BotCommand {
    /// Create a new empty bot command.
    #[must_use] pub fn new() -> Self { Self { command: String::new(), description: String::new() } }
    /// Create with command and description.
    #[must_use] pub fn with_params(command: String, description: String) -> Self { Self { command, description } }
    /// Get the command string.
    #[must_use] pub fn command(&self) -> &str { &self.command }
    /// Get the description.
    #[must_use] pub fn description(&self) -> &str { &self.description }
    /// Check if empty.
    #[must_use] pub fn is_empty(&self) -> bool { self.command.is_empty() && self.description.is_empty() }
    /// Check if valid (starts with '/').
    #[must_use] pub fn is_valid(&self) -> bool { !self.command.is_empty() && self.command.starts_with('/') }
}

impl Hash for BotCommand {
    fn hash<H: Hasher>(&self, state: &mut H) { self.command.hash(state); self.description.hash(state); }
}

impl Default for BotCommand {
    fn default() -> Self { Self::new() }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-bot-command";

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_version() { assert_eq!(CRATE_NAME, "rustgram-bot-command"); }
    #[test] fn test_new() { assert!(BotCommand::new().is_empty()); }
    #[test] fn test_with_params() { let c = BotCommand::with_params("/s".into(), "d".into()); assert!(c.is_valid()); }
    #[test] fn test_is_valid() { assert!(BotCommand::with_params("/h".into(), "d".into()).is_valid()); }
    #[test] fn test_invalid() { assert!(!BotCommand::with_params("h".into(), "d".into()).is_valid()); }
    #[test] fn test_equality() { assert_eq!(BotCommand::with_params("/x".into(), "y".into()), BotCommand::with_params("/x".into(), "y".into())); }
}
