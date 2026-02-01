//! Chat list widget (left column).
//!
//! Displays the list of dialogs/chats with navigation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::mock::MockDialog;
use crate::ui::style::{TextStyles, Theme};
use crate::ui::widgets::{Interactive, Renderable};

/// Widget for displaying the chat list.
pub struct ChatListWidget {
    /// List of dialogs to display.
    dialogs: Vec<MockDialog>,
    /// Currently selected index.
    selected_index: usize,
    /// List state for ratatui.
    state: ListState,
    /// Theme for styling.
    theme: Theme,
}

impl ChatListWidget {
    /// Creates a new chat list widget.
    pub fn new(theme: Theme) -> Self {
        Self {
            dialogs: Vec::new(),
            selected_index: 0,
            state: ListState::default().with_selected(Some(0)),
            theme,
        }
    }

    /// Sets the dialogs to display.
    pub fn set_dialogs(&mut self, dialogs: Vec<MockDialog>) {
        self.dialogs = dialogs;
        // Clamp selection to valid range
        if self.dialogs.is_empty() {
            self.selected_index = 0;
        } else if self.selected_index >= self.dialogs.len() {
            self.selected_index = self.dialogs.len() - 1;
        }
        self.state.select(Some(self.selected_index));
    }

    /// Gets the currently selected dialog.
    pub fn selected_dialog(&self) -> Option<&MockDialog> {
        self.dialogs.get(self.selected_index)
    }

    /// Gets the selected dialog ID.
    pub fn selected_id(&self) -> Option<i64> {
        self.selected_dialog().map(|d| d.id)
    }

    /// Sets the selected index.
    pub fn set_selected(&mut self, index: usize) {
        if !self.dialogs.is_empty() && index < self.dialogs.len() {
            self.selected_index = index;
            self.state.select(Some(index));
        }
    }

    /// Selects the next dialog.
    pub fn select_next(&mut self) {
        if !self.dialogs.is_empty() {
            self.selected_index = (self.selected_index + 1).min(self.dialogs.len() - 1);
            self.state.select(Some(self.selected_index));
        }
    }

    /// Selects the previous dialog.
    pub fn select_previous(&mut self) {
        if !self.dialogs.is_empty() && self.selected_index > 0 {
            self.selected_index -= 1;
            self.state.select(Some(self.selected_index));
        }
    }

    /// Formats a dialog for display.
    fn format_dialog(&self, dialog: &MockDialog) -> Line<'static> {
        let is_selected = self.selected_index < self.dialogs.len()
            && &self.dialogs[self.selected_index] == dialog;

        let mut spans = Vec::new();

        // Title (truncated if needed)
        let title = if dialog.title.len() > 25 {
            format!("{}...", &dialog.title[..22])
        } else {
            dialog.title.clone()
        };
        let title_width = 25;

        // Pin indicator
        if dialog.is_pinned {
            spans.push(Span::styled(
                "📌 ",
                TextStyles::dialog_title(&self.theme),
            ));
        }

        // Title
        let title_style = if is_selected {
            self.theme.active_style()
        } else {
            TextStyles::dialog_title(&self.theme)
        };
        spans.push(Span::styled(
            format!("{:<width$}", title, width = title_width),
            title_style,
        ));

        // Unread badge
        if dialog.unread_count > 0 {
            let unread_text = if dialog.unread_count > 99 {
                "99+".to_string()
            } else {
                dialog.unread_count.to_string()
            };
            spans.push(Span::styled(
                format!(" [{}]", unread_text),
                TextStyles::unread_count(&self.theme),
            ));
        }

        // Muted indicator
        if dialog.is_muted {
            spans.push(TextStyles::dim(&self.theme, " 🔇"));
        }

        Line::from(spans)
    }

    /// Formats the preview text for a dialog.
    fn format_preview(&self, dialog: &MockDialog) -> Line<'static> {
        let preview = if dialog.last_message.len() > 35 {
            format!("{}...", &dialog.last_message[..32])
        } else {
            dialog.last_message.clone()
        };

        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(preview, TextStyles::dialog_preview(&self.theme)),
        ])
    }
}

impl Default for ChatListWidget {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

impl Interactive for ChatListWidget {
    fn handle_key(&mut self, key: &crossterm::event::KeyEvent) -> bool {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_previous();
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
                true
            }
            KeyCode::Char('g') => {
                // Go to top
                if !self.dialogs.is_empty() {
                    self.set_selected(0);
                }
                true
            }
            KeyCode::Char('G') => {
                // Go to bottom
                if !self.dialogs.is_empty() {
                    self.set_selected(self.dialogs.len() - 1);
                }
                true
            }
            _ => false,
        }
    }
}

impl Renderable for ChatListWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        // Create list items (each dialog is 2 lines: title + preview)
        let mut items = Vec::new();
        for dialog in &self.dialogs {
            items.push(ListItem::new(self.format_dialog(dialog)));
            items.push(ListItem::new(self.format_preview(dialog)));
            // Add spacing between dialogs
            items.push(ListItem::new(" "));
        }

        // Create the list widget
        let list = List::new(items)
            .block(
                Block::default()
                    .title(Span::styled(" Chats ", TextStyles::status_bar(&self.theme)))
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style())
            )
            .style(self.theme.normal_style())
            .highlight_style(self.theme.active_style())
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.state.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockDialog;

    fn create_test_dialogs() -> Vec<MockDialog> {
        vec![
            MockDialog::new(1, "Alice".to_string(), "Hello there!".to_string(), 2),
            MockDialog::new(2, "Bob".to_string(), "See you later".to_string(), 0),
            MockDialog::new(3, "Charlie".to_string(), "How are you?".to_string(), 1),
        ]
    }

    #[test]
    fn test_chat_list_new() {
        let widget = ChatListWidget::new(Theme::default());
        assert_eq!(widget.selected_index, 0);
        assert!(widget.dialogs.is_empty());
    }

    #[test]
    fn test_chat_list_set_dialogs() {
        let mut widget = ChatListWidget::new(Theme::default());
        let dialogs = create_test_dialogs();
        widget.set_dialogs(dialogs);
        assert_eq!(widget.dialogs.len(), 3);
    }

    #[test]
    fn test_chat_list_navigation() {
        let mut widget = ChatListWidget::new(Theme::default());
        widget.set_dialogs(create_test_dialogs());

        assert_eq!(widget.selected_index, 0);

        widget.select_next();
        assert_eq!(widget.selected_index, 1);

        widget.select_next();
        assert_eq!(widget.selected_index, 2);

        widget.select_next();
        assert_eq!(widget.selected_index, 2); // Stays at last

        widget.select_previous();
        assert_eq!(widget.selected_index, 1);
    }

    #[test]
    fn test_chat_list_selected_dialog() {
        let mut widget = ChatListWidget::new(Theme::default());
        widget.set_dialogs(create_test_dialogs());

        assert_eq!(widget.selected_id(), Some(1));
        assert_eq!(widget.selected_dialog().unwrap().title, "Alice");

        widget.set_selected(1);
        assert_eq!(widget.selected_id(), Some(2));
    }

    #[test]
    fn test_chat_list_format_dialog() {
        let widget = ChatListWidget::new(Theme::default());
        let dialog = MockDialog::new(1, "Alice".to_string(), "Hello".to_string(), 0);

        let line = widget.format_dialog(&dialog);
        assert!(line.spans.iter().any(|s| s.content.contains("Alice")));
    }

    #[test]
    fn test_chat_list_handle_key() {
        let mut widget = ChatListWidget::new(Theme::default());
        widget.set_dialogs(create_test_dialogs());

        let down_key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Down,
            crossterm::event::KeyModifiers::empty(),
        );

        assert!(widget.handle_key(&down_key));
        assert_eq!(widget.selected_index, 1);

        let up_key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Up,
            crossterm::event::KeyModifiers::empty(),
        );

        assert!(widget.handle_key(&up_key));
        assert_eq!(widget.selected_index, 0);
    }

    #[test]
    fn test_chat_list_empty() {
        let widget = ChatListWidget::new(Theme::default());
        assert!(widget.selected_dialog().is_none());
        assert!(widget.selected_id().is_none());
    }
}
