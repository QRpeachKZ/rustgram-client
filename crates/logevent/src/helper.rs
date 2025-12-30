// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Helper functions for log event operations
//!
//! Provides utilities for adding and deleting log events from the binlog.

use crate::{LogEventIdWithGeneration, Result, TlStorer};
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
