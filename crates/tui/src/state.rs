//! Application state management.
//!
//! This module defines the state structures for the TUI application,
//! including the active dialog, selected items, and input state.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::sync::{Arc, RwLock};

use crate::mock::{ConnectionStatus, MockDialog, MockMessage, MockUserInfo};
use crate::error::{Result, TuiError};

/// Application state shared across all widgets.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Inner state protected by RwLock.
    inner: Arc<RwLock<AppStateInner>>,
}

impl AppState {
    /// Creates a new application state.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppStateInner::default())),
        }
    }

    /// Sets the current (active) dialog.
    pub fn set_active_dialog(&self, dialog_id: Option<i64>) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.active_dialog_id = dialog_id;
        Ok(())
    }

    /// Gets the current active dialog ID.
    pub fn active_dialog_id(&self) -> Result<Option<i64>> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.active_dialog_id)
    }

    /// Sets the list of dialogs.
    pub fn set_dialogs(&self, dialogs: Vec<MockDialog>) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.dialogs = dialogs;
        Ok(())
    }

    /// Gets a copy of the dialogs list.
    pub fn dialogs(&self) -> Result<Vec<MockDialog>> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.dialogs.clone())
    }

    /// Gets the active dialog if one is selected.
    pub fn active_dialog(&self) -> Result<Option<MockDialog>> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;

        if let Some(dialog_id) = inner.active_dialog_id {
            Ok(inner.dialogs.iter()
                .find(|d| d.id == dialog_id)
                .cloned())
        } else {
            Ok(None)
        }
    }

    /// Sets the messages for the active dialog.
    pub fn set_messages(&self, messages: Vec<MockMessage>) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.messages = messages;
        Ok(())
    }

    /// Gets a copy of the messages list.
    pub fn messages(&self) -> Result<Vec<MockMessage>> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.messages.clone())
    }

    /// Sets the user info for the active dialog.
    pub fn set_user_info(&self, user_info: Option<MockUserInfo>) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.user_info = user_info;
        Ok(())
    }

    /// Gets the user info for the active dialog.
    pub fn user_info(&self) -> Result<Option<MockUserInfo>> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.user_info.clone())
    }

    /// Sets the connection status.
    pub fn set_connection_status(&self, status: ConnectionStatus) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.connection_status = status;
        Ok(())
    }

    /// Gets the current connection status.
    pub fn connection_status(&self) -> Result<ConnectionStatus> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.connection_status)
    }

    /// Sets the current input text.
    pub fn set_input_text(&self, text: String) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.input_text = text;
        Ok(())
    }

    /// Gets the current input text.
    pub fn input_text(&self) -> Result<String> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.input_text.clone())
    }

    /// Appends text to the input.
    pub fn append_input(&self, text: &str) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.input_text.push_str(text);
        Ok(())
    }

    /// Removes the last character from input.
    pub fn backspace_input(&self) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.input_text.pop();
        Ok(())
    }

    /// Clears the input text.
    pub fn clear_input(&self) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.input_text.clear();
        Ok(())
    }

    /// Sets the current dialog list selection.
    pub fn set_selected_dialog_index(&self, index: usize) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.selected_dialog_index = index;
        Ok(())
    }

    /// Gets the current dialog list selection.
    pub fn selected_dialog_index(&self) -> Result<usize> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.selected_dialog_index)
    }

    /// Sets whether we should quit.
    pub fn set_should_quit(&self, should_quit: bool) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.should_quit = should_quit;
        Ok(())
    }

    /// Gets whether we should quit.
    pub fn should_quit(&self) -> Result<bool> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.should_quit)
    }

    /// Gets the terminal size hint (width, height).
    pub fn terminal_size(&self) -> Result<(u16, u16)> {
        let inner = self.inner.read()
            .map_err(|e| TuiError::State(format!("Failed to acquire read lock: {}", e)))?;
        Ok(inner.terminal_size)
    }

    /// Updates the terminal size.
    pub fn update_terminal_size(&self, width: u16, height: u16) -> Result<()> {
        let mut inner = self.inner.write()
            .map_err(|e| TuiError::State(format!("Failed to acquire write lock: {}", e)))?;
        inner.terminal_size = (width, height);
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Inner application state.
#[derive(Debug, Clone, Default)]
struct AppStateInner {
    /// Currently active dialog ID (None = no dialog selected).
    active_dialog_id: Option<i64>,
    /// List of dialogs/chats.
    dialogs: Vec<MockDialog>,
    /// Messages for the active dialog.
    messages: Vec<MockMessage>,
    /// User info for the active dialog.
    user_info: Option<MockUserInfo>,
    /// Current connection status.
    connection_status: ConnectionStatus,
    /// Current input text.
    input_text: String,
    /// Currently selected dialog in the list.
    selected_dialog_index: usize,
    /// Whether the application should quit.
    should_quit: bool,
    /// Terminal size (width, height).
    terminal_size: (u16, u16),
}

/// Focus mode for keyboard navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusMode {
    /// Focus on the dialog list.
    #[default]
    DialogList,
    /// Focus on the message view.
    MessageView,
    /// Focus on the input area.
    InputArea,
}

/// TUI event types.
#[derive(Debug, Clone)]
pub enum TuiEvent {
    /// Key press event.
    Key(crossterm::event::KeyEvent),
    /// Mouse event.
    Mouse(crossterm::event::MouseEvent),
    /// Terminal resize event.
    Resize(u16, u16),
    /// Message to send.
    SendMessage(String),
    /// Dialog selected.
    DialogSelected(i64),
    /// Tick/heartbeat event.
    Tick,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        assert_eq!(state.active_dialog_id().unwrap(), None);
        assert_eq!(state.dialogs().unwrap().len(), 0);
        assert_eq!(state.messages().unwrap().len(), 0);
    }

    #[test]
    fn test_set_active_dialog() {
        let state = AppState::new();
        state.set_active_dialog(Some(123)).unwrap();
        assert_eq!(state.active_dialog_id().unwrap(), Some(123));
    }

    #[test]
    fn test_set_dialogs() {
        let state = AppState::new();
        let dialogs = vec![
            MockDialog::new(1, "Test".to_string(), "Msg".to_string(), 0),
        ];
        state.set_dialogs(dialogs).unwrap();
        assert_eq!(state.dialogs().unwrap().len(), 1);
    }

    #[test]
    fn test_input_operations() {
        let state = AppState::new();

        state.append_input("Hello").unwrap();
        assert_eq!(state.input_text().unwrap(), "Hello");

        state.append_input(" World").unwrap();
        assert_eq!(state.input_text().unwrap(), "Hello World");

        state.backspace_input().unwrap();
        assert_eq!(state.input_text().unwrap(), "Hello Worl");

        state.clear_input().unwrap();
        assert_eq!(state.input_text().unwrap(), "");
    }

    #[test]
    fn test_connection_status() {
        let state = AppState::new();

        state.set_connection_status(ConnectionStatus::Connected).unwrap();
        assert_eq!(state.connection_status().unwrap(), ConnectionStatus::Connected);
    }

    #[test]
    fn test_should_quit() {
        let state = AppState::new();

        assert_eq!(state.should_quit().unwrap(), false);
        state.set_should_quit(true).unwrap();
        assert_eq!(state.should_quit().unwrap(), true);
    }

    #[test]
    fn test_focus_mode_default() {
        let focus = FocusMode::default();
        assert_eq!(focus, FocusMode::DialogList);
    }
}
