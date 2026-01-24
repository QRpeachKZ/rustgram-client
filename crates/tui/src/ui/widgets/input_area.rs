//! Input area widget (bottom area).
//!
//! Text input field for composing messages.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use ratatui::{
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::ui::style::{TextStyles, Theme};
use crate::ui::widgets::{Interactive, Renderable};

/// Widget for the message input area.
pub struct InputAreaWidget {
    /// Current input text.
    text: String,
    /// Cursor position.
    cursor_position: usize,
    /// Placeholder text when input is empty.
    placeholder: String,
    /// Whether to show the placeholder.
    show_placeholder: bool,
    /// Theme for styling.
    theme: Theme,
}

impl InputAreaWidget {
    /// Creates a new input area widget.
    pub fn new(theme: Theme) -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
            placeholder: "Type a message... (Alt+Enter for new line, Enter to send)".to_string(),
            show_placeholder: true,
            theme,
        }
    }

    /// Sets the placeholder text.
    pub fn set_placeholder(&mut self, placeholder: String) {
        self.placeholder = placeholder;
    }

    /// Gets the current input text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the input text.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.cursor_position = self.text.len();
        self.show_placeholder = self.text.is_empty();
    }

    /// Appends text to the input.
    pub fn append(&mut self, text: &str) {
        self.text.push_str(text);
        self.cursor_position = self.text.len();
        self.show_placeholder = false;
    }

    /// Removes the character before the cursor.
    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
            self.show_placeholder = self.text.is_empty();
        }
    }

    /// Removes the character at the cursor.
    pub fn delete(&mut self) {
        if self.cursor_position < self.text.len() {
            self.text.remove(self.cursor_position);
        }
    }

    /// Moves the cursor left.
    pub fn move_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Moves the cursor right.
    pub fn move_right(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }

    /// Moves the cursor to the start.
    pub fn move_to_start(&mut self) {
        self.cursor_position = 0;
    }

    /// Moves the cursor to the end.
    pub fn move_to_end(&mut self) {
        self.cursor_position = self.text.len();
    }

    /// Clears the input text.
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_position = 0;
        self.show_placeholder = true;
    }

    /// Returns whether the input is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Gets the cursor position.
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Checks if the text should wrap to multiple lines.
    pub fn should_wrap(&self, width: u16) -> bool {
        if self.text.is_empty() {
            return false;
        }
        // Account for borders and padding
        let available_width = width.saturating_sub(4);
        self.text.len() > available_width as usize
    }

    /// Renders the input text with placeholder.
    fn render_text(&self) -> Text<'static> {
        if self.text.is_empty() && self.show_placeholder {
            Text::from(vec![
                ratatui::text::Line::from(vec![
                    ratatui::text::Span::styled(
                        self.placeholder.clone(),
                        TextStyles::input_placeholder(&self.theme),
                    ),
                ]),
            ])
        } else {
            Text::from(self.text.clone())
        }
    }
}

impl Default for InputAreaWidget {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

impl Interactive for InputAreaWidget {
    fn handle_key(&mut self, key: &crossterm::event::KeyEvent) -> bool {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key.code {
            KeyCode::Char(c) => {
                // Handle character input
                self.text.insert(self.cursor_position, c);
                self.cursor_position += 1;
                self.show_placeholder = false;
                true
            }
            KeyCode::Backspace => {
                self.backspace();
                true
            }
            KeyCode::Delete => {
                self.delete();
                true
            }
            KeyCode::Left => {
                self.move_left();
                true
            }
            KeyCode::Right => {
                self.move_right();
                true
            }
            KeyCode::Home => {
                self.move_to_start();
                true
            }
            KeyCode::End => {
                self.move_to_end();
                true
            }
            KeyCode::Enter if key.modifiers.contains(KeyModifiers::ALT) => {
                // Alt+Enter adds a newline
                self.text.insert(self.cursor_position, '\n');
                self.cursor_position += 1;
                self.show_placeholder = false;
                true
            }
            _ => false,
        }
    }
}

impl Renderable for InputAreaWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.render_text())
            .block(
                Block::default()
                    .title(Span::styled(
                        " Input ",
                        TextStyles::status_bar(&self.theme),
                    ))
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style())
            )
            .style(self.theme.normal_style())
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, area);

        // Set cursor position if the widget is focused
        // Note: In a real application, you would track focus state
        // and only show the cursor when this widget has focus
    }
}

use ratatui::text::Span;

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_input_area_new() {
        let widget = InputAreaWidget::new(Theme::default());
        assert!(widget.is_empty());
        assert_eq!(widget.cursor_position(), 0);
        assert!(widget.show_placeholder);
    }

    #[test]
    fn test_input_area_set_text() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());
        assert_eq!(widget.text(), "Hello");
        assert_eq!(widget.cursor_position(), 5);
        assert!(!widget.show_placeholder);
    }

    #[test]
    fn test_input_area_append() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.append("Hello");
        widget.append(" World");
        assert_eq!(widget.text(), "Hello World");
        assert!(!widget.show_placeholder);
    }

    #[test]
    fn test_input_area_backspace() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());
        widget.backspace();
        assert_eq!(widget.text(), "Hell");
        assert_eq!(widget.cursor_position(), 4);
    }

    #[test]
    fn test_input_area_delete() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());
        widget.move_left(); // cursor at 4 (after 'o')
        widget.delete(); // removes character at cursor, 'o'
        assert_eq!(widget.text(), "Hell"); // Result: "Hell"
    }

    #[test]
    fn test_input_area_cursor_movement() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());

        widget.move_to_start();
        assert_eq!(widget.cursor_position(), 0);

        widget.move_right();
        assert_eq!(widget.cursor_position(), 1);

        widget.move_to_end();
        assert_eq!(widget.cursor_position(), 5);
    }

    #[test]
    fn test_input_area_clear() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());
        widget.clear();
        assert!(widget.is_empty());
        assert!(widget.show_placeholder);
    }

    #[test]
    fn test_input_area_handle_char() {
        let mut widget = InputAreaWidget::new(Theme::default());
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        widget.handle_key(&key);

        assert_eq!(widget.text(), "a");
        assert_eq!(widget.cursor_position(), 1);
    }

    #[test]
    fn test_input_area_handle_backspace() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());

        let key = KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty());
        widget.handle_key(&key);

        assert_eq!(widget.text(), "Hell");
    }

    #[test]
    fn test_input_area_handle_arrow_keys() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());

        let left_key = KeyEvent::new(KeyCode::Left, KeyModifiers::empty());
        widget.handle_key(&left_key);
        assert_eq!(widget.cursor_position(), 4);

        let right_key = KeyEvent::new(KeyCode::Right, KeyModifiers::empty());
        widget.handle_key(&right_key);
        assert_eq!(widget.cursor_position(), 5);

        let home_key = KeyEvent::new(KeyCode::Home, KeyModifiers::empty());
        widget.handle_key(&home_key);
        assert_eq!(widget.cursor_position(), 0);

        let end_key = KeyEvent::new(KeyCode::End, KeyModifiers::empty());
        widget.handle_key(&end_key);
        assert_eq!(widget.cursor_position(), 5);
    }

    #[test]
    fn test_input_area_alt_enter() {
        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("Hello".to_string());

        let alt_enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT);
        widget.handle_key(&alt_enter);

        assert_eq!(widget.text(), "Hello\n");
    }

    #[test]
    fn test_input_area_should_wrap() {
        let widget = InputAreaWidget::new(Theme::default());
        assert!(!widget.should_wrap(100));

        let mut widget = InputAreaWidget::new(Theme::default());
        widget.set_text("a".repeat(100));
        assert!(widget.should_wrap(100));
    }
}
