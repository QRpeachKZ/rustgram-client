//! Terminal User Interface for rustgram-client.
//!
//! This crate provides a TUI (Terminal User Interface) for the rustgram-client
//! Telegram client using the ratatui framework.
//!
//! # Features
//!
//! - **3-column layout**: Chat list, message view, and user info
//! - **Interactive widgets**: Keyboard navigation, message input
//! - **Multiple themes**: Dark, light, solarized, dracula, nord
//! - **Mock data support**: For development without full backend
//!
//! # Example
//!
//! ```no_run
//! use rustgram_tui::RustgramTuiApp;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut app = RustgramTuiApp::new()?;
//!     app.run()?;
//!     Ok(())
//! }
//! ```
//!
//! # Keyboard Shortcuts
//!
//! ## Dialog List (default focus)
//! - `j` / `↓` - Move down
//! - `k` / `↑` - Move up
//! - `Enter` / `l` - Select dialog
//! - `i` - Switch to input mode
//! - `q` / `Esc` - Quit
//!
//! ## Message View
//! - `j` / `↓` - Scroll down
//! - `k` / `↑` - Scroll up
//! - `PageUp` / `PageDown` - Page scroll
//! - `Home` / `End` - Jump to top/bottom
//! - `h` - Switch to dialog list
//! - `i` - Switch to input mode
//!
//! ## Input Area
//! - Type to enter text
//! - `Alt+Enter` - Add new line
//! - `Enter` - Send message
//! - `Backspace` - Delete character
//! - `h` - Switch to dialog list

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod app;
pub mod error;
pub mod event;
pub mod mock;
pub mod state;
pub mod ui;

pub use app::RustgramTuiApp;
pub use error::{Result, TuiError};
pub use event::{EventHandler, InputHandler, KeyAction};
pub use mock::{
    ConnectionStatus, MockDialog, MockMessage, MockUserInfo, UserStatus,
    format_timestamp, generate_mock_dialogs, generate_mock_messages, generate_mock_user_info,
};
pub use state::{AppState, FocusMode, TuiEvent};
pub use ui::{LayoutConfig, Padding, TuiLayout, Theme, WidgetAlignment};
pub use ui::widgets::{
    ChatListWidget, InputAreaWidget, Interactive, MessageViewWidget,
    Renderable, StatusBarWidget, UserInfoWidget,
};

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-tui";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-tui");
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        // Just verify the theme can be created
        let _ = theme.primary_style();
        let _ = theme.active_style();
    }

    #[test]
    fn test_focus_mode_default() {
        let focus = FocusMode::default();
        assert_eq!(focus, FocusMode::DialogList);
    }
}
