//! UI module for the TUI application.
//!
//! Contains layout, styling, and widget implementations.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod layout;
pub mod style;
pub mod widgets;

pub use layout::{LayoutConfig, Padding, TuiLayout, WidgetAlignment};
pub use style::{BorderStyle, TextStyles, Theme};
pub use widgets::{ChatListWidget, InputAreaWidget, MessageViewWidget, Renderable, StatusBarWidget, UserInfoWidget};
