//! User info widget (right column).
//!
//! Displays information about the current chat/dialog.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::mock::{MockDialog, MockUserInfo, UserStatus};
use crate::ui::style::{TextStyles, Theme};
use crate::ui::widgets::Renderable;

/// Widget for displaying user info.
pub struct UserInfoWidget {
    /// Current dialog info.
    dialog: Option<MockDialog>,
    /// User information.
    user_info: Option<MockUserInfo>,
    /// Theme for styling.
    theme: Theme,
}

impl UserInfoWidget {
    /// Creates a new user info widget.
    pub fn new(theme: Theme) -> Self {
        Self {
            dialog: None,
            user_info: None,
            theme,
        }
    }

    /// Sets the current dialog.
    pub fn set_dialog(&mut self, dialog: Option<MockDialog>) {
        self.dialog = dialog;
    }

    /// Sets the user info.
    pub fn set_user_info(&mut self, user_info: Option<MockUserInfo>) {
        self.user_info = user_info;
    }

    /// Clears all displayed information.
    pub fn clear(&mut self) {
        self.dialog = None;
        self.user_info = None;
    }

    /// Renders the user info content.
    fn render_content(&self) -> Text<'static> {
        let mut lines = Vec::new();

        // Show dialog info
        if let Some(dialog) = &self.dialog {
            // Title
            lines.push(Line::from(vec![
                Span::styled(
                    dialog.title.clone(),
                    TextStyles::user_info_header(&self.theme),
                ),
            ]));
            lines.push(Line::from(""));

            // Unread count
            if dialog.unread_count > 0 {
                lines.push(Line::from(vec![
                    Span::styled(
                        "Unread: ",
                        TextStyles::user_info_detail(&self.theme),
                    ),
                    Span::styled(
                        format!("{}", dialog.unread_count),
                        TextStyles::unread_count(&self.theme),
                    ),
                ]));
                lines.push(Line::from(""));
            }

            // Status indicators
            let mut status = Vec::new();
            if dialog.is_pinned {
                status.push("📌 Pinned");
            }
            if dialog.is_muted {
                status.push("🔇 Muted");
            }
            if !status.is_empty() {
                lines.push(Line::from(status.join(" | ")));
                lines.push(Line::from(""));
            }
        }

        // Show user info if available
        if let Some(user) = &self.user_info {
            // Username
            if let Some(username) = &user.username {
                lines.push(Line::from(vec![
                    Span::styled(
                        "@",
                        TextStyles::user_info_detail(&self.theme),
                    ),
                    TextStyles::primary(&self.theme, username.clone()),
                ]));
                lines.push(Line::from(""));
            }

            // Status
            let status_text: String = match &user.status {
                UserStatus::Online => "Online".to_string(),
                UserStatus::LastSeen(ts) => {
                    // Simple timestamp formatting
                    format!("Last seen: {}", ts)
                }
                UserStatus::Offline => "Offline".to_string(),
            };
            lines.push(Line::from(vec![
                Span::styled(
                    "Status: ",
                    TextStyles::user_info_detail(&self.theme),
                ),
                TextStyles::dim(&self.theme, status_text),
            ]));
            lines.push(Line::from(""));

            // Bio
            if let Some(bio) = &user.bio {
                lines.push(Line::from(vec![
                    Span::styled(
                        "Bio:",
                        TextStyles::user_info_detail(&self.theme),
                    ),
                ]));
                for line in bio.split('\n') {
                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default()),
                        TextStyles::dim(&self.theme, line),
                    ]));
                }
            }
        }

        // Empty state
        if lines.is_empty() {
            lines.push(Line::from(vec![
                TextStyles::dim(&self.theme, "No chat selected"),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                TextStyles::dim(&self.theme, "Select a chat to view details"),
            ]));
        }

        Text::from(lines)
    }
}

impl Default for UserInfoWidget {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

impl Renderable for UserInfoWidget {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let paragraph = Paragraph::new(self.render_content())
            .block(
                Block::default()
                    .title(Span::styled(
                        " Info ",
                        TextStyles::status_bar(&self.theme),
                    ))
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style())
            )
            .style(self.theme.normal_style())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{MockDialog, MockUserInfo};

    #[test]
    fn test_user_info_new() {
        let widget = UserInfoWidget::new(Theme::default());
        assert!(widget.dialog.is_none());
        assert!(widget.user_info.is_none());
    }

    #[test]
    fn test_user_info_set_dialog() {
        let mut widget = UserInfoWidget::new(Theme::default());
        let dialog = MockDialog::new(1, "Alice".to_string(), "Hello".to_string(), 0);
        widget.set_dialog(Some(dialog));
        assert!(widget.dialog.is_some());
    }

    #[test]
    fn test_user_info_set_user_info() {
        let mut widget = UserInfoWidget::new(Theme::default());
        let user_info = MockUserInfo {
            id: 1,
            name: "Alice".to_string(),
            username: Some("alice".to_string()),
            status: UserStatus::Online,
            bio: Some("Test bio".to_string()),
        };
        widget.set_user_info(Some(user_info));
        assert!(widget.user_info.is_some());
    }

    #[test]
    fn test_user_info_clear() {
        let mut widget = UserInfoWidget::new(Theme::default());
        let dialog = MockDialog::new(1, "Alice".to_string(), "Hello".to_string(), 0);
        widget.set_dialog(Some(dialog));
        widget.clear();
        assert!(widget.dialog.is_none());
        assert!(widget.user_info.is_none());
    }

    #[test]
    fn test_user_info_render_content_with_dialog() {
        let mut widget = UserInfoWidget::new(Theme::default());
        let dialog = MockDialog::new(1, "Alice".to_string(), "Hello".to_string(), 5)
            .pinned()
            .muted();
        widget.set_dialog(Some(dialog));

        let content = widget.render_content();
        // Should contain the title
        assert!(content.lines.iter().any(|l| l.to_string().contains("Alice")));
    }

    #[test]
    fn test_user_info_render_content_with_user() {
        let mut widget = UserInfoWidget::new(Theme::default());
        let user_info = MockUserInfo {
            id: 1,
            name: "Alice".to_string(),
            username: Some("alice".to_string()),
            status: UserStatus::Online,
            bio: Some("Software developer".to_string()),
        };
        widget.set_user_info(Some(user_info));

        let content = widget.render_content();
        // Should contain the username
        assert!(content.lines.iter().any(|l| l.to_string().contains("@alice")));
    }

    #[test]
    fn test_user_info_render_content_empty() {
        let widget = UserInfoWidget::new(Theme::default());
        let content = widget.render_content();
        // Should show empty state message
        assert!(content.lines.iter().any(|l| l.to_string().contains("No chat selected")));
    }
}
