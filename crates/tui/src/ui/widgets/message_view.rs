//! Message view widget (center column).
//!
//! Displays messages in a chat with bubbles.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::mock::{format_timestamp, MockMessage};
use crate::ui::style::{TextStyles, Theme};
use crate::ui::widgets::{Interactive, Renderable};

/// Widget for displaying messages in a conversation.
pub struct MessageViewWidget {
    /// Messages to display.
    messages: Vec<MockMessage>,
    /// Vertical scroll offset.
    scroll_offset: usize,
    /// Auto-scroll to bottom on new messages.
    auto_scroll: bool,
    /// Theme for styling.
    theme: Theme,
}

impl MessageViewWidget {
    /// Creates a new message view widget.
    pub fn new(theme: Theme) -> Self {
        Self {
            messages: Vec::new(),
            scroll_offset: 0,
            auto_scroll: true,
            theme,
        }
    }

    /// Sets the messages to display.
    pub fn set_messages(&mut self, messages: Vec<MockMessage>) {
        let was_empty = self.messages.is_empty();
        self.messages = messages;

        // If auto-scroll is enabled and this wasn't the first load, scroll to bottom
        if self.auto_scroll && !was_empty {
            self.scroll_to_bottom();
        } else if was_empty {
            // Initial load, start at bottom
            self.scroll_to_bottom();
        }
    }

    /// Adds a new message to the view.
    pub fn add_message(&mut self, message: MockMessage) {
        self.messages.push(message);
        if self.auto_scroll {
            self.scroll_to_bottom();
        }
    }

    /// Scrolls up one line.
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Scrolls down one line.
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Scrolls to the top.
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scrolls to the bottom.
    pub fn scroll_to_bottom(&mut self) {
        // Scroll offset will be calculated during rendering based on content height
        self.scroll_offset = usize::MAX;
    }

    /// Scrolls up one page.
    pub fn page_up(&mut self, page_height: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(page_height);
    }

    /// Scrolls down one page.
    pub fn page_down(&mut self, page_height: usize) {
        self.scroll_offset = self.scroll_offset.saturating_add(page_height);
    }

    /// Toggles auto-scroll mode.
    pub fn toggle_auto_scroll(&mut self) {
        self.auto_scroll = !self.auto_scroll;
        if self.auto_scroll {
            self.scroll_to_bottom();
        }
    }

    /// Formats a single message for display.
    fn format_message(&self, msg: &MockMessage) -> Line<'static> {
        let time_str = format_timestamp(msg.timestamp);

        let prefix = if msg.is_outgoing {
            format!("{} ✓ ", time_str)
        } else {
            format!("{} ", time_str)
        };

        let sender_style = if msg.is_outgoing {
            TextStyles::sender_name(&self.theme).bg(Color::DarkGray)
        } else {
            TextStyles::sender_name(&self.theme)
        };

        let text_style = if msg.is_outgoing {
            self.theme.outgoing_message_style()
        } else {
            self.theme.incoming_message_style()
        };

        // Combine sender and message text
        let _full_text = format!("{}: {}", msg.sender, msg.text);

        Line::from(vec![
            Span::styled(prefix, Style::default().fg(Color::DarkGray)),
            Span::styled(msg.sender.clone(), sender_style),
            Span::styled(": ", Style::default()),
            Span::styled(msg.text.clone(), text_style),
        ])
    }

    /// Renders the message list as a Text widget.
    fn render_messages(&self, _area: Rect) -> Text<'static> {
        let mut lines = Vec::new();

        // Add empty state if no messages
        if self.messages.is_empty() {
            lines.push(Line::from(vec![
                TextStyles::dim(&self.theme, "No messages yet."),
            ]));
            lines.push(Line::from(vec![
                TextStyles::dim(&self.theme, "Start a conversation!"),
            ]));
            return Text::from(lines);
        }

        // Format each message
        for msg in &self.messages {
            lines.push(self.format_message(msg));
            lines.push(Line::from("")); // Empty line between messages
        }

        Text::from(lines)
    }
}

impl Default for MessageViewWidget {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

impl Interactive for MessageViewWidget {
    fn handle_key(&mut self, key: &crossterm::event::KeyEvent) -> bool {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll_up();
                self.auto_scroll = false;
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll_down();
                true
            }
            KeyCode::PageUp => {
                self.page_up(20);
                self.auto_scroll = false;
                true
            }
            KeyCode::PageDown => {
                self.page_down(20);
                true
            }
            KeyCode::Home => {
                self.scroll_to_top();
                self.auto_scroll = false;
                true
            }
            KeyCode::End => {
                self.scroll_to_bottom();
                self.auto_scroll = true;
                true
            }
            KeyCode::Char('a') => {
                self.toggle_auto_scroll();
                true
            }
            _ => false,
        }
    }
}

impl Renderable for MessageViewWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let messages = self.render_messages(area);

        let paragraph = Paragraph::new(messages)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style())
            )
            .style(self.theme.normal_style())
            .wrap(Wrap { trim: true })
            .scroll((self.scroll_offset as u16, 0));

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockMessage;

    fn create_test_messages() -> Vec<MockMessage> {
        vec![
            MockMessage::incoming(1, 100, "Alice".to_string(), "Hello!".to_string()),
            MockMessage::outgoing(2, 100, "Hi there!".to_string()),
            MockMessage::incoming(3, 100, "Bob".to_string(), "How are you?".to_string()),
        ]
    }

    #[test]
    fn test_message_view_new() {
        let widget = MessageViewWidget::new(Theme::default());
        assert_eq!(widget.scroll_offset, 0);
        assert!(widget.auto_scroll);
    }

    #[test]
    fn test_message_view_set_messages() {
        let mut widget = MessageViewWidget::new(Theme::default());
        let messages = create_test_messages();
        widget.set_messages(messages);
        assert_eq!(widget.messages.len(), 3);
    }

    #[test]
    fn test_message_view_add_message() {
        let mut widget = MessageViewWidget::new(Theme::default());
        widget.set_messages(create_test_messages());

        let new_msg = MockMessage::incoming(4, 100, "Charlie".to_string(), "Test".to_string());
        widget.add_message(new_msg);

        assert_eq!(widget.messages.len(), 4);
    }

    #[test]
    fn test_message_view_scroll() {
        let mut widget = MessageViewWidget::new(Theme::default());
        // Don't use set_messages as it calls scroll_to_bottom which sets offset to MAX
        widget.messages = create_test_messages();
        widget.scroll_to_top(); // Reset to 0

        widget.scroll_down();
        assert_eq!(widget.scroll_offset, 1);

        widget.scroll_up();
        assert_eq!(widget.scroll_offset, 0);
    }

    #[test]
    fn test_message_view_page_scroll() {
        let mut widget = MessageViewWidget::new(Theme::default());
        // Don't use set_messages as it calls scroll_to_bottom which sets offset to MAX
        widget.messages = create_test_messages();
        widget.scroll_to_top(); // Reset to 0

        widget.page_up(10);
        assert_eq!(widget.scroll_offset, 0);

        widget.page_down(10);
        assert_eq!(widget.scroll_offset, 10);
    }

    #[test]
    fn test_message_view_scroll_to_bottom() {
        let mut widget = MessageViewWidget::new(Theme::default());
        widget.set_messages(create_test_messages());

        widget.scroll_to_bottom();
        assert_eq!(widget.scroll_offset, usize::MAX);
    }

    #[test]
    fn test_message_view_toggle_auto_scroll() {
        let mut widget = MessageViewWidget::new(Theme::default());
        assert!(widget.auto_scroll);

        widget.toggle_auto_scroll();
        assert!(!widget.auto_scroll);

        widget.toggle_auto_scroll();
        assert!(widget.auto_scroll);
    }

    #[test]
    fn test_message_view_handle_key() {
        let mut widget = MessageViewWidget::new(Theme::default());
        // Don't use set_messages as it calls scroll_to_bottom which sets offset to MAX
        widget.messages = create_test_messages();
        widget.scroll_to_top(); // Reset to 0

        let down_key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Down,
            crossterm::event::KeyModifiers::empty(),
        );

        assert!(widget.handle_key(&down_key));
        assert_eq!(widget.scroll_offset, 1);

        let page_down_key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::PageDown,
            crossterm::event::KeyModifiers::empty(),
        );

        assert!(widget.handle_key(&page_down_key));
        assert_eq!(widget.scroll_offset, 21);
    }

    #[test]
    fn test_message_view_empty() {
        let widget = MessageViewWidget::new(Theme::default());
        assert_eq!(widget.messages.len(), 0);
    }
}
