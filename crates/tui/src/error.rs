//! Error types for the TUI application.
//!
//! This module defines all error types that can occur in the TUI,
//! including rendering errors, input errors, and state management errors.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

/// Result type for TUI operations.
pub type Result<T> = std::result::Result<T, TuiError>;

/// Main error type for the TUI application.
#[derive(Debug, thiserror::Error)]
pub enum TuiError {
    /// Terminal initialization error.
    #[error("Failed to initialize terminal: {0}")]
    TerminalInit(String),

    /// Terminal restoration error.
    #[error("Failed to restore terminal: {0}")]
    TerminalRestore(String),

    /// Rendering error.
    #[error("Rendering error: {0}")]
    Render(String),

    /// Input error.
    #[error("Input error: {0}")]
    Input(String),

    /// State error.
    #[error("State error: {0}")]
    State(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Send error for async channels.
    #[error("Send error: {0}")]
    Send(String),

    /// Receive error for async channels.
    #[error("Receive error: {0}")]
    Receive(String),

    /// Integration error (when using real managers).
    #[cfg(feature = "integration")]
    #[error("Integration error: {0}")]
    Integration(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_terminal_init() {
        let err = TuiError::TerminalInit("test error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Failed to initialize terminal"));
        assert!(display.contains("test error"));
    }

    #[test]
    fn test_display_render() {
        let err = TuiError::Render("frame buffer error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Rendering error"));
        assert!(display.contains("frame buffer error"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let tui_err: TuiError = io_err.into();
        assert!(matches!(tui_err, TuiError::Io(_)));
    }

    #[test]
    fn test_from_serialization_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json")
            .unwrap_err();
        let tui_err: TuiError = json_err.into();
        assert!(matches!(tui_err, TuiError::Serialization(_)));
    }
}
