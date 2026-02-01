// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Utility functions for TL deserialization.

use std::time::{SystemTime, UNIX_EPOCH};

/// Gets the current Unix timestamp.
///
/// Returns the number of seconds since the Unix epoch.
pub fn current_timestamp() -> i32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i32)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp();
        // Timestamp should be reasonably recent (after 2020)
        assert!(ts > 1577836800);
    }
}
