//! Widget implementations for the TUI.
//!
//! This module contains all the custom widgets used in the terminal interface.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod chat_list;
pub mod input_area;
pub mod message_view;
pub mod status_bar;
pub mod user_info;

pub use chat_list::ChatListWidget;
pub use input_area::InputAreaWidget;
pub use message_view::MessageViewWidget;
pub use status_bar::StatusBarWidget;
pub use user_info::UserInfoWidget;

use ratatui::Frame;

/// Trait for widgets that can be rendered.
pub trait Renderable {
    /// Renders the widget to the frame.
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect);
}

/// Trait for widgets that handle user input.
pub trait Interactive {
    /// Handles a key event.
    ///
    /// Returns `true` if the event was handled and should not propagate.
    fn handle_key(
        &mut self,
        key: &crossterm::event::KeyEvent,
    ) -> bool {
        let _ = key;
        false
    }

    /// Handles a mouse event.
    ///
    /// Returns `true` if the event was handled and should not propagate.
    fn handle_mouse(
        &mut self,
        mouse: &crossterm::event::MouseEvent,
    ) -> bool {
        let _ = mouse;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_default_key() {
        struct TestWidget;
        impl Interactive for TestWidget {}

        let mut widget = TestWidget;
        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('a'),
            crossterm::event::KeyModifiers::empty(),
        );
        assert!(!widget.handle_key(&key));
    }

    #[test]
    fn test_interactive_default_mouse() {
        struct TestWidget;
        impl Interactive for TestWidget {}

        let mut widget = TestWidget;
        let mouse = crossterm::event::MouseEvent {
            kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: crossterm::event::KeyModifiers::empty(),
        };
        assert!(!widget.handle_mouse(&mouse));
    }
}
