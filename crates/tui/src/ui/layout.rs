//! Layout management for the TUI.
//!
//! This module handles the 3-column layout and responsive sizing
//! for the terminal interface.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};

/// Layout configuration for the TUI.
#[derive(Debug, Clone, Copy)]
pub struct LayoutConfig {
    /// Width percentage for the chat list (left column).
    pub chat_list_width: u16,
    /// Width percentage for the user info (right column).
    pub user_info_width: u16,
    /// Height for the status bar (top).
    pub status_bar_height: u16,
    /// Height for the input area (bottom).
    pub input_area_height: u16,
    /// Minimum width for the chat list.
    pub min_chat_list_width: u16,
    /// Minimum width for the message view.
    pub min_message_view_width: u16,
    /// Minimum width for the user info.
    pub min_user_info_width: u16,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            chat_list_width: 30,
            user_info_width: 25,
            status_bar_height: 3,
            input_area_height: 3,
            min_chat_list_width: 20,
            min_message_view_width: 30,
            min_user_info_width: 20,
        }
    }
}

impl LayoutConfig {
    /// Creates a new layout config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a compact layout for smaller terminals.
    pub fn compact() -> Self {
        Self {
            chat_list_width: 25,
            user_info_width: 0, // Hide user info in compact mode
            status_bar_height: 2,
            input_area_height: 3,
            min_chat_list_width: 15,
            min_message_view_width: 20,
            min_user_info_width: 0,
        }
    }

    /// Checks if the terminal size is sufficient for the 3-column layout.
    pub fn is_sufficient_size(&self, width: u16, height: u16) -> bool {
        // Check if we have enough width for all three columns
        let min_width = if self.user_info_width > 0 {
            self.min_chat_list_width + self.min_message_view_width + self.min_user_info_width
        } else {
            self.min_chat_list_width + self.min_message_view_width
        };

        // Check if we have enough height
        let min_height = self.status_bar_height + self.input_area_height + 10;

        width >= min_width && height >= min_height
    }

    /// Adjusts the layout for the given terminal size.
    pub fn adjust_for_size(&mut self, width: u16, _height: u16) {
        // Hide user info if terminal is narrow
        if width < 100 {
            self.user_info_width = 0;
            self.chat_list_width = 25;
        } else if width < 120 {
            self.user_info_width = 20;
            self.chat_list_width = 25;
        } else {
            self.user_info_width = 25;
            self.chat_list_width = 30;
        }
    }
}

/// Calculated layout rectangles for all UI areas.
#[derive(Debug, Clone, Copy)]
pub struct TuiLayout {
    /// Status bar area (top).
    pub status_bar: Rect,
    /// Main content area (between status bar and input).
    pub main: Rect,
    /// Chat list area (left column).
    pub chat_list: Rect,
    /// Message view area (center column).
    pub message_view: Rect,
    /// User info area (right column, may be hidden).
    pub user_info: Rect,
    /// Input area (bottom).
    pub input_area: Rect,
}

impl TuiLayout {
    /// Calculates the layout for the given terminal area.
    pub fn calculate(area: Rect, config: &LayoutConfig) -> Self {
        // Split vertically: status bar (top), main content, input area (bottom)
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(config.status_bar_height),
                Constraint::Min(0),  // Main content takes remaining space
                Constraint::Length(config.input_area_height),
            ])
            .split(area);

        let status_bar = vertical_chunks[0];
        let main = vertical_chunks[1];
        let input_area = vertical_chunks[2];

        // Split main content horizontally based on config
        let (chat_list, message_view, user_info) = if config.user_info_width > 0 {
            // Three-column layout
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(config.chat_list_width),
                    Constraint::Min(0),
                    Constraint::Percentage(config.user_info_width),
                ])
                .split(main);

            (
                horizontal_chunks[0],
                horizontal_chunks[1],
                horizontal_chunks[2],
            )
        } else {
            // Two-column layout (no user info)
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(config.chat_list_width),
                    Constraint::Min(0),
                ])
                .split(main);

            (
                horizontal_chunks[0],
                horizontal_chunks[1],
                Rect::default(), // Empty user info
            )
        };

        Self {
            status_bar,
            main,
            chat_list,
            message_view,
            user_info,
            input_area,
        }
    }

    /// Checks if the user info panel is visible.
    pub fn is_user_info_visible(&self) -> bool {
        self.user_info.width > 0
    }
}

/// Alignment options for widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetAlignment {
    /// Align to the left.
    Left,
    /// Align to the center.
    Center,
    /// Align to the right.
    Right,
}

impl WidgetAlignment {
    /// Converts to ratatui Alignment.
    pub fn to_ratatui(self) -> Alignment {
        match self {
            Self::Left => Alignment::Left,
            Self::Center => Alignment::Center,
            Self::Right => Alignment::Right,
        }
    }
}

/// Padding for widgets.
#[derive(Debug, Clone, Copy)]
pub struct Padding {
    /// Left padding.
    pub left: u16,
    /// Right padding.
    pub right: u16,
    /// Top padding.
    pub top: u16,
    /// Bottom padding.
    pub bottom: u16,
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            left: 1,
            right: 1,
            top: 1,
            bottom: 1,
        }
    }
}

impl Padding {
    /// Creates a new padding with the specified values.
    pub fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Creates uniform padding on all sides.
    pub fn uniform(value: u16) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    /// Creates horizontal padding only.
    pub fn horizontal(value: u16) -> Self {
        Self {
            left: value,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    /// Creates vertical padding only.
    pub fn vertical(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: value,
            bottom: value,
        }
    }

    /// Returns the total horizontal padding.
    pub fn horizontal_total(&self) -> u16 {
        self.left + self.right
    }

    /// Returns the total vertical padding.
    pub fn vertical_total(&self) -> u16 {
        self.top + self.bottom
    }

    /// Applies the padding to a rectangle, returning the inner rectangle.
    pub fn apply_to(self, rect: Rect) -> Rect {
        let width = rect.width.saturating_sub(self.horizontal_total());
        let height = rect.height.saturating_sub(self.vertical_total());

        Rect {
            x: rect.x.saturating_add(self.left),
            y: rect.y.saturating_add(self.top),
            width,
            height,
        }
    }
}

/// Calculates the maximum number of visible items in a list.
pub fn calculate_visible_items(area_height: u16, item_height: u16, padding: &Padding) -> usize {
    let available_height = area_height.saturating_sub(padding.vertical_total());
    (available_height / item_height.max(1)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_config_default() {
        let config = LayoutConfig::default();
        assert_eq!(config.chat_list_width, 30);
        assert_eq!(config.user_info_width, 25);
        assert_eq!(config.status_bar_height, 3);
        assert_eq!(config.input_area_height, 3);
    }

    #[test]
    fn test_layout_config_compact() {
        let config = LayoutConfig::compact();
        assert_eq!(config.user_info_width, 0);
        assert_eq!(config.chat_list_width, 25);
    }

    #[test]
    fn test_is_sufficient_size() {
        let config = LayoutConfig::default();
        assert!(config.is_sufficient_size(80, 30));
        assert!(!config.is_sufficient_size(40, 10));
    }

    #[test]
    fn test_adjust_for_size() {
        let mut config = LayoutConfig::default();
        config.adjust_for_size(80, 30);
        assert_eq!(config.user_info_width, 0);

        config.adjust_for_size(130, 30);
        assert_eq!(config.user_info_width, 25);
    }

    #[test]
    fn test_tui_layout_calculate() {
        let area = Rect::new(0, 0, 100, 30);
        let config = LayoutConfig::default();
        let layout = TuiLayout::calculate(area, &config);

        assert_eq!(layout.status_bar.height, 3);
        assert_eq!(layout.input_area.height, 3);
        assert!(layout.message_view.width > 0);
        assert!(layout.user_info.width > 0);
    }

    #[test]
    fn test_tui_layout_compact() {
        let area = Rect::new(0, 0, 100, 30);
        let config = LayoutConfig::compact();
        let layout = TuiLayout::calculate(area, &config);

        assert_eq!(layout.user_info.width, 0);
        assert!(!layout.is_user_info_visible());
    }

    #[test]
    fn test_widget_alignment_conversion() {
        assert_eq!(WidgetAlignment::Left.to_ratatui(), Alignment::Left);
        assert_eq!(WidgetAlignment::Center.to_ratatui(), Alignment::Center);
        assert_eq!(WidgetAlignment::Right.to_ratatui(), Alignment::Right);
    }

    #[test]
    fn test_padding_default() {
        let padding = Padding::default();
        assert_eq!(padding.left, 1);
        assert_eq!(padding.right, 1);
        assert_eq!(padding.top, 1);
        assert_eq!(padding.bottom, 1);
    }

    #[test]
    fn test_padding_uniform() {
        let padding = Padding::uniform(2);
        assert_eq!(padding.left, 2);
        assert_eq!(padding.horizontal_total(), 4);
    }

    #[test]
    fn test_padding_horizontal() {
        let padding = Padding::horizontal(3);
        assert_eq!(padding.left, 3);
        assert_eq!(padding.right, 3);
        assert_eq!(padding.top, 0);
        assert_eq!(padding.bottom, 0);
    }

    #[test]
    fn test_padding_apply_to() {
        let rect = Rect::new(0, 0, 100, 50);
        let padding = Padding::uniform(5);
        let inner = padding.apply_to(rect);

        assert_eq!(inner.x, 5);
        assert_eq!(inner.y, 5);
        assert_eq!(inner.width, 90);
        assert_eq!(inner.height, 40);
    }

    #[test]
    fn test_calculate_visible_items() {
        let padding = Padding::default();
        let visible = calculate_visible_items(30, 2, &padding);
        assert_eq!(visible, 14); // (30 - 2) / 2 = 14
    }
}
