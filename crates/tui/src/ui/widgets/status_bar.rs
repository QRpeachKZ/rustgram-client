//! Status bar widget (top area).
//!
//! Displays connection status, user info, and unread counts.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::mock::ConnectionStatus;
use crate::ui::style::{TextStyles, Theme};
use crate::ui::widgets::Renderable;

/// Widget for the status bar.
pub struct StatusBarWidget {
    /// Connection status to display.
    connection_status: ConnectionStatus,
    /// Current user display name.
    user_name: String,
    /// Total unread message count.
    total_unread: usize,
    /// Theme for styling.
    theme: Theme,
}

impl StatusBarWidget {
    /// Creates a new status bar widget.
    pub fn new(theme: Theme) -> Self {
        Self {
            connection_status: ConnectionStatus::Disconnected,
            user_name: "Not logged in".to_string(),
            total_unread: 0,
            theme,
        }
    }

    /// Sets the connection status.
    pub fn set_connection_status(&mut self, status: ConnectionStatus) {
        self.connection_status = status;
    }

    /// Sets the user name.
    pub fn set_user_name(&mut self, name: String) {
        self.user_name = name;
    }

    /// Sets the total unread count.
    pub fn set_total_unread(&mut self, count: usize) {
        self.total_unread = count;
    }

    /// Renders the status bar content.
    fn render_content(&self) -> Line<'static> {
        use ratatui::text::Span;

        // Connection status with color
        let status_color = match self.connection_status {
            ConnectionStatus::Connected => Color::Green,
            ConnectionStatus::Connecting => Color::Yellow,
            ConnectionStatus::Disconnected => Color::DarkGray,
            ConnectionStatus::Error => Color::Red,
        };

        let status_text = match self.connection_status {
            ConnectionStatus::Connected => "●",
            ConnectionStatus::Connecting => "○",
            ConnectionStatus::Disconnected => "○",
            ConnectionStatus::Error => "✖",
        };

        let mut spans = vec![
            Span::styled(status_text, Style::default().fg(status_color)),
            Span::styled(" ", Style::default()),
            TextStyles::dim(&self.theme, self.connection_status.as_str()),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                self.user_name.clone(),
                TextStyles::status_bar(&self.theme),
            ),
        ];

        // Add unread count if there are unread messages
        if self.total_unread > 0 {
            spans.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
            let unread_text = if self.total_unread > 99 {
                "99+ unread".to_string()
            } else {
                format!("{} unread", self.total_unread)
            };
            spans.push(Span::styled(
                unread_text,
                TextStyles::unread_count(&self.theme),
            ));
        }

        Line::from(spans)
    }
}

impl Default for StatusBarWidget {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

use ratatui::style::{Color, Style};

impl Renderable for StatusBarWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.render_content())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style())
            )
            .style(self.theme.normal_style());

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_new() {
        let widget = StatusBarWidget::new(Theme::default());
        assert_eq!(widget.connection_status, ConnectionStatus::Disconnected);
        assert_eq!(widget.user_name, "Not logged in");
        assert_eq!(widget.total_unread, 0);
    }

    #[test]
    fn test_status_bar_set_connection_status() {
        let mut widget = StatusBarWidget::new(Theme::default());
        widget.set_connection_status(ConnectionStatus::Connected);
        assert_eq!(widget.connection_status, ConnectionStatus::Connected);
    }

    #[test]
    fn test_status_bar_set_user_name() {
        let mut widget = StatusBarWidget::new(Theme::default());
        widget.set_user_name("Alice".to_string());
        assert_eq!(widget.user_name, "Alice");
    }

    #[test]
    fn test_status_bar_set_total_unread() {
        let mut widget = StatusBarWidget::new(Theme::default());
        widget.set_total_unread(5);
        assert_eq!(widget.total_unread, 5);
    }

    #[test]
    fn test_status_bar_render_content() {
        let mut widget = StatusBarWidget::new(Theme::default());
        widget.set_connection_status(ConnectionStatus::Connected);
        widget.set_user_name("Alice".to_string());
        widget.set_total_unread(3);

        let line = widget.render_content();
        // The line should contain status indicator and user name
        assert!(line.spans.iter().any(|s| s.content.contains("Connected")));
    }
}
