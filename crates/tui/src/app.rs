//! Main application and event loop for the TUI.
//!
//! This module contains the core application logic including the
//! terminal setup, event loop, and rendering.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::error::{Result, TuiError};
use crate::event::{InputHandler, KeyAction};
use crate::mock::{generate_mock_dialogs, generate_mock_messages, generate_mock_user_info, ConnectionStatus};
use crate::state::{AppState, FocusMode};
use crate::ui::{
    ChatListWidget, InputAreaWidget, LayoutConfig, MessageViewWidget,
    StatusBarWidget, TuiLayout, Theme, UserInfoWidget,
};
use crate::ui::widgets::{Interactive, Renderable};

/// Main TUI application.
pub struct RustgramTuiApp {
    /// Application state.
    state: AppState,
    /// Terminal instance.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    /// Input handler.
    input_handler: InputHandler,
    /// Current theme (reserved for future use).
    _theme: Theme,
    /// Layout configuration.
    layout_config: LayoutConfig,
    /// Current focus mode.
    focus: FocusMode,
    /// Widgets.
    status_bar: StatusBarWidget,
    chat_list: ChatListWidget,
    message_view: MessageViewWidget,
    input_area: InputAreaWidget,
    user_info: UserInfoWidget,
}

impl RustgramTuiApp {
    /// Creates a new TUI application.
    pub fn new() -> Result<Self> {
        // Setup terminal
        enable_raw_mode()
            .map_err(|e| TuiError::TerminalInit(format!("Failed to enable raw mode: {}", e)))?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)
            .map_err(|e| TuiError::TerminalInit(format!("Failed to enter alternate screen: {}", e)))?;
        execute!(stdout, EnableMouseCapture)
            .map_err(|e| TuiError::TerminalInit(format!("Failed to enable mouse capture: {}", e)))?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)
            .map_err(|e| TuiError::TerminalInit(format!("Failed to create terminal: {}", e)))?;

        // Create application state
        let state = AppState::new();
        let theme = Theme::default();
        let layout_config = LayoutConfig::default();

        // Initialize with mock data
        let dialogs = generate_mock_dialogs();
        let messages = generate_mock_messages(1, 20);
        let user_info = generate_mock_user_info();

        state.set_dialogs(dialogs.clone())?;
        state.set_messages(messages.clone())?;
        state.set_user_info(Some(user_info.clone()))?;
        state.set_connection_status(ConnectionStatus::Connected)?;

        // Create widgets
        let mut status_bar = StatusBarWidget::new(theme);
        status_bar.set_connection_status(ConnectionStatus::Connected);
        status_bar.set_user_name("You".to_string());

        let mut chat_list = ChatListWidget::new(theme);
        chat_list.set_dialogs(dialogs);

        let mut message_view = MessageViewWidget::new(theme);
        message_view.set_messages(messages);

        let input_area = InputAreaWidget::new(theme);

        let mut user_info_widget = UserInfoWidget::new(theme);
        user_info_widget.set_user_info(Some(user_info));

        Ok(Self {
            state,
            terminal,
            input_handler: InputHandler::new(),
            _theme: theme,
            layout_config,
            focus: FocusMode::DialogList,
            status_bar,
            chat_list,
            message_view,
            input_area,
            user_info: user_info_widget,
        })
    }

    /// Runs the main application loop.
    pub fn run(&mut self) -> Result<()> {
        // Initial render
        self.render()?;

        // Main event loop
        loop {
            // Check if we should quit
            if self.state.should_quit()? {
                break;
            }

            // Poll for events
            if crossterm::event::poll(Duration::from_millis(16))
                .map_err(|e| TuiError::Input(format!("Poll failed: {}", e)))?
            {
                // Handle the event
                if let Ok(Event::Key(key)) = crossterm::event::read() {
                    self.handle_key_event(key)?;
                }
            }

            // Render the UI
            self.render()?;
        }

        Ok(())
    }

    /// Handles a keyboard event.
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        // Get the key action from the input handler
        let action = self.input_handler.handle_key(&key);

        match action {
            KeyAction::Quit => {
                self.state.set_should_quit(true)?;
            }
            KeyAction::FocusDialogList => {
                self.focus = FocusMode::DialogList;
                self.input_handler.set_focus(FocusMode::DialogList);
            }
            KeyAction::FocusMessageView => {
                self.focus = FocusMode::MessageView;
                self.input_handler.set_focus(FocusMode::MessageView);
            }
            KeyAction::FocusInput => {
                self.focus = FocusMode::InputArea;
                self.input_handler.set_focus(FocusMode::InputArea);
            }
            KeyAction::SendMessage => {
                let text = self.state.input_text()?;
                if !text.is_empty() {
                    self.send_message(text)?;
                    self.state.clear_input()?;
                    self.input_area.clear();
                }
            }
            KeyAction::NavigateUp | KeyAction::NavigateDown => {
                if self.focus == FocusMode::DialogList {
                    self.chat_list.handle_key(&key);
                    // Update selected dialog
                    if let Some(dialog) = self.chat_list.selected_dialog() {
                        self.user_info.set_dialog(Some(dialog.clone()));
                    }
                }
            }
            KeyAction::ScrollUp | KeyAction::ScrollDown | KeyAction::PageUp | KeyAction::PageDown => {
                if self.focus == FocusMode::MessageView {
                    self.message_view.handle_key(&key);
                }
            }
            KeyAction::SelectDialog => {
                if let Some(id) = self.chat_list.selected_id() {
                    self.state.set_active_dialog(Some(id))?;
                    self.load_dialog_messages(id)?;
                }
            }
            KeyAction::Backspace => {
                if self.focus == FocusMode::InputArea {
                    self.state.backspace_input()?;
                    self.input_area.backspace();
                }
            }
            KeyAction::NewLine => {
                if self.focus == FocusMode::InputArea {
                    self.state.append_input("\n")?;
                    self.input_area.append("\n");
                }
            }
            _ => {
                // Let the widget handle the key
                match self.focus {
                    FocusMode::DialogList => {
                        self.chat_list.handle_key(&key);
                    }
                    FocusMode::MessageView => {
                        self.message_view.handle_key(&key);
                    }
                    FocusMode::InputArea => {
                        self.input_area.handle_key(&key);
                        // Update state input
                        let text = self.input_area.text().to_string();
                        self.state.set_input_text(text)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Sends a message.
    fn send_message(&mut self, text: String) -> Result<()> {
        use crate::mock::MockMessage;

        // Get the active dialog ID
        let dialog_id = self.state.active_dialog_id()?.unwrap_or(1);

        // Create a new outgoing message
        let msg = MockMessage::outgoing(
            (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0)) % 10000,
            dialog_id,
            text.clone(),
        );

        // Add to message view
        self.message_view.add_message(msg.clone());

        // Update state
        let mut messages = self.state.messages()?;
        messages.push(msg);
        self.state.set_messages(messages)?;

        // Update last message in dialog list
        let mut dialogs = self.state.dialogs()?;
        if let Some(dialog) = dialogs.iter_mut().find(|d| d.id == dialog_id) {
            dialog.last_message = text;
        }
        self.state.set_dialogs(dialogs.clone())?;
        self.chat_list.set_dialogs(dialogs);

        Ok(())
    }

    /// Loads messages for a dialog.
    fn load_dialog_messages(&mut self, dialog_id: i64) -> Result<()> {
        let messages = generate_mock_messages(dialog_id, 20);
        self.state.set_messages(messages.clone())?;
        self.message_view.set_messages(messages);
        Ok(())
    }

    /// Renders the UI.
    fn render(&mut self) -> Result<()> {
        let _terminal_size = self.terminal.size()
            .map_err(|e| TuiError::Render(format!("Failed to get terminal size: {}", e)))?;

        self.terminal.draw(|f| {
            // Calculate layout
            let layout = TuiLayout::calculate(f.area(), &self.layout_config);

            // Render widgets
            self.status_bar.render(f, layout.status_bar);
            self.chat_list.render(f, layout.chat_list);
            self.message_view.render(f, layout.message_view);
            self.input_area.render(f, layout.input_area);

            if layout.is_user_info_visible() {
                self.user_info.render(f, layout.user_info);
            }
        }).map_err(|e| TuiError::Render(format!("Failed to draw: {}", e)))?;

        Ok(())
    }
}

impl Drop for RustgramTuiApp {
    fn drop(&mut self) {
        // Restore terminal state
        disable_raw_mode().ok();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).ok();
        self.terminal.show_cursor().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        // Note: This test may not work in all environments due to terminal requirements
        // It's here for documentation purposes
        // In a real test environment, you would need to mock the terminal
    }
}
