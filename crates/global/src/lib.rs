//! # Global State
//!
//! Global application state for the Telegram client.
//!
//! ## Overview
//!
//! This module provides a stub implementation for the global state manager.
//! In the full TDLib implementation, this is a massive class that manages
//! all the managers and state for the Telegram client.
//!
//! ## TODO
//!
//! This is a simplified stub. The full implementation would include:
//! - Manager accessors (AuthManager, ChatManager, MessagesManager, etc.)
//! - Server time tracking
//! - Option management
//! - Database access
//! - Network query dispatching
//!
//! ## Usage
//!
//! ```
//! use rustgram_global::Global;
//!
//! // This is a placeholder stub
//! // Full implementation would provide access to all managers
//! ```

/// Stub for Global state manager.
///
/// TODO: Full implementation with all manager accessors and state management.
/// This is a minimal placeholder for compilation purposes.
///
/// The TDLib `Global` class is a massive ~800 line class that manages:
/// - All manager ActorIds
/// - Server time synchronization
/// - Option/setting management
/// - Database access
/// - Network query creator/dispatcher
/// - File storage paths
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Global {
    _private: (),
}

impl Global {
    /// Creates a new Global instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_global::Global;
    ///
    /// let global = Global::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for Global {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let global = Global::new();
        // Just verify it compiles
        let _ = global;
    }

    #[test]
    fn test_default() {
        let global = Global::default();
        // Just verify it compiles
        let _ = global;
    }

    #[test]
    fn test_clone() {
        let global1 = Global::new();
        let global2 = global1;
        assert_eq!(global1, global2);
    }
}
