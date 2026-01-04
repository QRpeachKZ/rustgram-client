// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Helper functions for log event operations
//!
//! Provides utilities for adding and deleting log events from the binlog,
//! as well as time-related helpers for storing/parse timestamps.

use crate::{LogEventIdWithGeneration, Result, TlParser, TlStorer};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

/// Add a log event to the binlog
///
/// This is a stub implementation. The full implementation would integrate
/// with the binlog module when it's available.
///
/// # Arguments
///
/// * `id` - The log event ID with generation tracking (will be updated)
/// * `_storer` - The storer containing the event data (currently unused in stub)
/// * `event_type` - The type of log event
/// * `name` - The name of the event (for logging)
pub fn add_log_event<S: TlStorer>(
    id: &mut LogEventIdWithGeneration,
    _storer: &S,
    event_type: u32,
    name: &str,
) -> Result<()> {
    // Stub: binlog integration would go here
    // TODO: Integrate with binlog module when available
    info!(
        "Save {} to binlog (type: 0x{:x}, generation: {})",
        name, event_type, id.generation
    );

    // In full implementation, this would call:
    // if id.log_event_id == 0 {
    //     id.log_event_id = binlog_add(event_type, storer);
    // } else {
    //     id.log_event_id = binlog_rewrite(id.log_event_id, event_type, storer);
    // }
    // id.generation += 1;

    id.generation += 1;
    Ok(())
}

/// Delete a log event from the binlog
///
/// This is a stub implementation. The full implementation would integrate
/// with the binlog module when it's available.
///
/// # Arguments
///
/// * `id` - The log event ID with generation tracking
/// * `generation` - The expected generation (must match current)
/// * `name` - The name of the event (for logging)
pub fn delete_log_event(
    id: &mut LogEventIdWithGeneration,
    generation: u64,
    name: &str,
) -> Result<()> {
    // Stub: binlog integration would go here
    // TODO: Integrate with binlog module when available
    info!(
        "Finish processing {} log event {} with generation {}",
        name, id.log_event_id, generation
    );

    if id.generation == generation {
        // In full implementation, this would call:
        // binlog_erase(id.log_event_id);
        id.log_event_id = 0;
    }

    Ok(())
}

/// Store a timestamp to the storer
///
/// Stores either a relative time or an absolute timestamp, compatible with TDLib's
/// `store_time` function from `LogEventHelper.h`.
///
/// If `time_at` is 0, stores -1.0. Otherwise, stores the remaining time until
/// the event and the current server time.
///
/// # Arguments
///
/// * `time_at` - The absolute timestamp (0 for "not set")
/// * `storer` - The storer to write to
/// * `server_time` - The current server time (seconds since UNIX epoch)
///
/// # Example
///
/// ```rust
/// use rustgram_logevent::{store_time, LogEventStorerVec};
///
/// let mut storer = LogEventStorerVec::new();
/// let server_time = 1704067200.0; // 2024-01-01
///
/// // Store a future timestamp
/// store_time(1704153600.0, &mut storer, server_time);
/// ```
pub fn store_time<S: TlStorer>(time_at: f64, storer: &mut S, server_time: f64) {
    if time_at == 0.0 {
        storer.store_f64(-1.0);
    } else {
        let now = current_time();
        let time_left = (time_at - now).max(0.0);
        storer.store_f64(time_left);
        storer.store_f64(server_time);
    }
}

/// Parse a timestamp from the parser
///
/// Parses a timestamp stored by [`store_time`], compatible with TDLib's
/// `parse_time` function from `LogEventHelper.h`.
///
/// Returns the absolute timestamp calculated from the stored relative time
/// and server time.
///
/// # Arguments
///
/// * `parser` - The parser to read from
/// * `server_time` - The current server time (seconds since UNIX epoch)
///
/// # Errors
///
/// Returns [`LogEventError::UnexpectedEndOfInput`] if there's not enough data.
///
/// # Example
///
/// ```rust
/// use rustgram_logevent::{parse_time, store_time, LogEventParser, LogEventStorerVec};
///
/// // First, store a timestamp
/// let mut storer = LogEventStorerVec::new();
/// let server_time = 1704067200.0;
/// store_time(1704153600.0, &mut storer, server_time);
///
/// // Then parse it back
/// let data = storer.into_inner();
/// let mut parser = LogEventParser::new(&data);
/// let time_at = parse_time(&mut parser, server_time)?;
/// # Ok::<(), rustgram_logevent::LogEventError>(())
/// ```
pub fn parse_time<P: TlParser>(parser: &mut P, server_time: f64) -> Result<f64> {
    let time_left = parser.fetch_f64()?;
    if time_left < -0.1 {
        Ok(0.0)
    } else {
        let old_server_time = parser.fetch_f64()?;
        let passed_server_time = (server_time - old_server_time).max(0.0);
        let adjusted_time_left = (time_left - passed_server_time).max(0.0);
        let now = current_time();
        Ok(now + adjusted_time_left)
    }
}

/// Get the current time in seconds since UNIX epoch
///
/// This is a helper that provides the current time as f64 for use with
/// [`store_time`] and [`parse_time`].
fn current_time() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStorer;

    impl TlStorer for MockStorer {
        fn store_i32(&mut self, _value: i32) {}
        fn store_u32(&mut self, _value: u32) {}
        fn store_i64(&mut self, _value: i64) {}
        fn store_u64(&mut self, _value: u64) {}
        fn store_bytes(&mut self, _data: &[u8]) {}
        fn len(&self) -> usize {
            0
        }
    }

    #[test]
    fn test_add_log_event_increments_generation() {
        let mut id = LogEventIdWithGeneration::default();
        let storer = MockStorer;

        add_log_event(&mut id, &storer, 0x100, "SendMessage").unwrap();

        assert_eq!(id.generation, 1);
    }

    #[test]
    fn test_add_log_event_multiple_times() {
        let mut id = LogEventIdWithGeneration::default();
        let storer = MockStorer;

        add_log_event(&mut id, &storer, 0x100, "SendMessage").unwrap();
        assert_eq!(id.generation, 1);

        add_log_event(&mut id, &storer, 0x100, "SendMessage").unwrap();
        assert_eq!(id.generation, 2);
    }

    #[test]
    fn test_delete_log_event_matching_generation() {
        let mut id = LogEventIdWithGeneration::new(12345, 2);
        let _original_id = id.log_event_id;

        delete_log_event(&mut id, 2, "SendMessage").unwrap();

        // Matching generation should reset the ID
        assert_eq!(id.log_event_id, 0);
        assert_eq!(id.generation, 2);
    }

    #[test]
    fn test_delete_log_event_non_matching_generation() {
        let mut id = LogEventIdWithGeneration::new(12345, 2);
        let original_id = id.log_event_id;

        delete_log_event(&mut id, 1, "SendMessage").unwrap();

        // Non-matching generation should NOT reset the ID
        assert_eq!(id.log_event_id, original_id);
        assert_eq!(id.generation, 2);
    }

    #[test]
    fn test_delete_log_event_after_add() {
        let mut id = LogEventIdWithGeneration::default();
        let storer = MockStorer;

        // Add event
        add_log_event(&mut id, &storer, 0x100, "SendMessage").unwrap();
        let generation = id.generation;

        // Delete with matching generation
        delete_log_event(&mut id, generation, "SendMessage").unwrap();

        // In stub implementation, log_event_id won't be set
        // but in real implementation it would be set then reset to 0
    }
}
