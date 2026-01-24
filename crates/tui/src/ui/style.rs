//! Color schemes and styling for the TUI.
//!
//! This module defines the colors, themes, and styles used throughout
//! the terminal interface.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

/// Color theme for the TUI.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// Primary color for accents.
    pub primary: Color,
    /// Secondary color.
    pub secondary: Color,
    /// Background color.
    pub background: Color,
    /// Foreground color.
    pub foreground: Color,
    /// Color for the active item.
    pub active: Color,
    /// Color for borders.
    pub border: Color,
    /// Color for text in the active item.
    pub active_text: Color,
    /// Color for unread indicators.
    pub unread: Color,
    /// Color for error messages.
    pub error: Color,
    /// Color for success/warning messages.
    pub warning: Color,
}

impl Theme {
    /// Creates a new theme with default colors.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the style for the primary action color.
    pub fn primary_style(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Returns the style for the secondary color.
    pub fn secondary_style(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    /// Returns the style for normal text.
    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }

    /// Returns the style for the active item.
    pub fn active_style(&self) -> Style {
        Style::default()
            .fg(self.active_text)
            .bg(self.active)
            .add_modifier(Modifier::BOLD)
    }

    /// Returns the style for borders.
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Returns the style for unread indicators.
    pub fn unread_style(&self) -> Style {
        Style::default()
            .fg(self.unread)
            .add_modifier(Modifier::BOLD)
    }

    /// Returns the style for error messages.
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// Returns the style for warning messages.
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// Returns the style for dimmed text.
    pub fn dim_style(&self) -> Style {
        Style::default()
            .fg(self.foreground)
            .add_modifier(Modifier::DIM)
    }

    /// Returns the style for bold text.
    pub fn bold_style(&self) -> Style {
        Style::default()
            .fg(self.foreground)
            .add_modifier(Modifier::BOLD)
    }

    /// Returns the style for incoming messages.
    pub fn incoming_message_style(&self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }

    /// Returns the style for outgoing messages.
    pub fn outgoing_message_style(&self) -> Style {
        Style::default()
            .fg(self.active_text)
            .bg(self.active)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            background: Color::Black,
            foreground: Color::White,
            active: Color::DarkGray,
            active_text: Color::White,
            border: Color::DarkGray,
            unread: Color::Yellow,
            error: Color::Red,
            warning: Color::LightYellow,
        }
    }
}

/// Predefined color themes.
impl Theme {
    /// Creates a dark theme (default).
    pub fn dark() -> Self {
        Self::default()
    }

    /// Creates a light theme.
    pub fn light() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::LightBlue,
            background: Color::White,
            foreground: Color::Black,
            active: Color::LightCyan,
            active_text: Color::Black,
            border: Color::Gray,
            unread: Color::Red,
            error: Color::Red,
            warning: Color::Yellow,
        }
    }

    /// Creates a solarized dark theme.
    pub fn solarized_dark() -> Self {
        Self {
            primary: Color::Rgb(38, 139, 210),   // Solarized Blue
            secondary: Color::Rgb(133, 153, 0),  // Solarized Green
            background: Color::Rgb(0, 43, 54),   // Solarized Base03
            foreground: Color::Rgb(131, 148, 150), // Solarized Base0
            active: Color::Rgb(88, 110, 117),    // Solarized Base01
            active_text: Color::Rgb(253, 246, 227), // Solarized Base3
            border: Color::Rgb(88, 110, 117),    // Solarized Base01
            unread: Color::Rgb(181, 137, 0),     // Solarized Yellow
            error: Color::Rgb(220, 50, 47),      // Solarized Red
            warning: Color::Rgb(203, 75, 22),    // Solarized Orange
        }
    }

    /// Creates a dracula theme.
    pub fn dracula() -> Self {
        Self {
            primary: Color::Rgb(98, 114, 164),   // Dracula Comment
            secondary: Color::Rgb(139, 233, 253), // Dracula Cyan
            background: Color::Rgb(40, 42, 54),  // Dracula Background
            foreground: Color::Rgb(248, 248, 242), // Dracula Foreground
            active: Color::Rgb(68, 71, 90),      // Dracula Current Line
            active_text: Color::Rgb(248, 248, 242), // Dracula Foreground
            border: Color::Rgb(68, 71, 90),      // Dracula Current Line
            unread: Color::Rgb(255, 184, 108),   // Dracula Orange
            error: Color::Rgb(255, 85, 85),      // Dracula Red
            warning: Color::Rgb(241, 250, 140),  // Dracula Yellow
        }
    }

    /// Creates a nord theme.
    pub fn nord() -> Self {
        Self {
            primary: Color::Rgb(136, 192, 208),  // Nord Frost (light)
            secondary: Color::Rgb(94, 129, 172), // Nord Frost (medium)
            background: Color::Rgb(46, 52, 64),  // Nord Polar Night
            foreground: Color::Rgb(216, 222, 233), // Nord Snow Storm
            active: Color::Rgb(59, 66, 82),      // Nord Polar Night (lighter)
            active_text: Color::Rgb(216, 222, 233), // Nord Snow Storm
            border: Color::Rgb(76, 86, 106),     // Nord Polar Night (border)
            unread: Color::Rgb(208, 135, 112),   // Nord Aurora (red/orange)
            error: Color::Rgb(191, 97, 106),     // Nord Aurora (red)
            warning: Color::Rgb(235, 203, 139),  // Nord Aurora (yellow)
        }
    }

    /// Creates a monochrome theme for minimal terminals.
    pub fn monochrome() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::Gray,
            background: Color::Black,
            foreground: Color::White,
            active: Color::Gray,
            active_text: Color::Black,
            border: Color::DarkGray,
            unread: Color::White,
            error: Color::White,
            warning: Color::White,
        }
    }
}

/// Text styles for common UI elements.
pub struct TextStyles;

impl TextStyles {
    /// Style for a dialog title.
    pub fn dialog_title(theme: &Theme) -> Style {
        theme.bold_style()
    }

    /// Style for dialog preview text.
    pub fn dialog_preview(theme: &Theme) -> Style {
        theme.dim_style()
    }

    /// Style for a timestamp.
    pub fn timestamp(theme: &Theme) -> Style {
        theme.dim_style()
    }

    /// Style for unread count badge.
    pub fn unread_count(theme: &Theme) -> Style {
        theme.unread_style()
    }

    /// Style for sender name.
    pub fn sender_name(theme: &Theme) -> Style {
        theme.primary_style().add_modifier(Modifier::BOLD)
    }

    /// Style for message text.
    pub fn message_text(theme: &Theme) -> Style {
        theme.normal_style()
    }

    /// Style for status bar text.
    pub fn status_bar(theme: &Theme) -> Style {
        theme.secondary_style().add_modifier(Modifier::BOLD)
    }

    /// Style for input text.
    pub fn input_text(theme: &Theme) -> Style {
        theme.normal_style()
    }

    /// Style for input placeholder.
    pub fn input_placeholder(theme: &Theme) -> Style {
        theme.dim_style()
    }

    /// Style for user info header.
    pub fn user_info_header(theme: &Theme) -> Style {
        theme.primary_style().add_modifier(Modifier::BOLD)
    }

    /// Style for user info detail.
    pub fn user_info_detail(theme: &Theme) -> Style {
        theme.dim_style()
    }

    /// Creates a span with a specific style.
    pub fn span<S: AsRef<str>>(text: S, style: Style) -> Span<'static> {
        Span::styled(text.as_ref().to_string(), style)
    }

    /// Creates a primary colored span.
    pub fn primary<S: AsRef<str>>(theme: &Theme, text: S) -> Span<'static> {
        Self::span(text, theme.primary_style())
    }

    /// Creates a dim span.
    pub fn dim<S: AsRef<str>>(theme: &Theme, text: S) -> Span<'static> {
        Self::span(text, theme.dim_style())
    }

    /// Creates a bold span.
    pub fn bold<S: AsRef<str>>(theme: &Theme, text: S) -> Span<'static> {
        Self::span(text, theme.bold_style())
    }

    /// Creates an error span.
    pub fn error<S: AsRef<str>>(theme: &Theme, text: S) -> Span<'static> {
        Self::span(text, theme.error_style())
    }

    /// Creates a warning span.
    pub fn warning<S: AsRef<str>>(theme: &Theme, text: S) -> Span<'static> {
        Self::span(text, theme.warning_style())
    }
}

/// Border styles for widgets.
#[derive(Debug, Clone, Copy)]
pub struct BorderStyle {
    /// Border type.
    pub border_type: ratatui::widgets::BorderType,
    /// Border style.
    pub style: Style,
}

impl BorderStyle {
    /// Creates a new border style.
    pub fn new(theme: &Theme) -> Self {
        Self {
            border_type: ratatui::widgets::BorderType::Plain,
            style: theme.border_style(),
        }
    }

    /// Creates a border with rounded corners.
    pub fn rounded(theme: &Theme) -> Self {
        Self {
            border_type: ratatui::widgets::BorderType::Rounded,
            style: theme.border_style(),
        }
    }

    /// Creates a double-line border.
    pub fn double(theme: &Theme) -> Self {
        Self {
            border_type: ratatui::widgets::BorderType::Double,
            style: theme.border_style(),
        }
    }

    /// Creates a thick border.
    pub fn thick(theme: &Theme) -> Self {
        Self {
            border_type: ratatui::widgets::BorderType::Thick,
            style: theme.border_style(),
        }
    }

    /// Creates a bordered widget style.
    pub fn widget_style(&self) -> ratatui::widgets::Borders {
        ratatui::widgets::Borders::ALL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme.primary, Color::Cyan);
        assert_eq!(theme.background, Color::Black);
    }

    #[test]
    fn test_theme_primary_style() {
        let theme = Theme::new();
        let style = theme.primary_style();
        assert_eq!(style.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_theme_active_style() {
        let theme = Theme::new();
        let style = theme.active_style();
        assert_eq!(style.fg, Some(Color::White));
        assert_eq!(style.bg, Some(Color::DarkGray));
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.background, Color::White);
        assert_eq!(theme.foreground, Color::Black);
    }

    #[test]
    fn test_theme_dracula() {
        let theme = Theme::dracula();
        assert_eq!(theme.background, Color::Rgb(40, 42, 54));
        assert_eq!(theme.primary, Color::Rgb(98, 114, 164));
    }

    #[test]
    fn test_text_styles_span() {
        let theme = Theme::new();
        let span = TextStyles::span("test", theme.primary_style());
        assert_eq!(span.content, "test");
    }

    #[test]
    fn test_text_styles_primary() {
        let theme = Theme::new();
        let span = TextStyles::primary(&theme, "test");
        assert_eq!(span.content, "test");
    }

    #[test]
    fn test_border_style_new() {
        let theme = Theme::new();
        let border = BorderStyle::new(&theme);
        assert_eq!(border.style.fg, Some(Color::DarkGray));
    }

    #[test]
    fn test_border_style_rounded() {
        let theme = Theme::new();
        let border = BorderStyle::rounded(&theme);
        assert_eq!(border.border_type, ratatui::widgets::BorderType::Rounded);
    }

    #[test]
    fn test_border_style_double() {
        let theme = Theme::new();
        let border = BorderStyle::double(&theme);
        assert_eq!(border.border_type, ratatui::widgets::BorderType::Double);
    }

    #[test]
    fn test_outgoing_message_style() {
        let theme = Theme::new();
        let style = theme.outgoing_message_style();
        assert_eq!(style.bg, Some(Color::DarkGray));
    }

    #[test]
    fn test_incoming_message_style() {
        let theme = Theme::new();
        let style = theme.incoming_message_style();
        assert_eq!(style.bg, Some(Color::Black));
    }
}
