//! Event handling for the TUI application.
//!
//! This module handles terminal events including keyboard input,
//! mouse events, and terminal resizing.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![ deny(clippy::expect_used)]

use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

use crate::error::{Result, TuiError};
use crate::state::{FocusMode, TuiEvent};

/// Default tick rate for the event loop (60 FPS).
pub const DEFAULT_TICK_RATE: Duration = Duration::from_millis(1000 / 60);

/// Event handler for terminal input.
pub struct EventHandler {
    /// Sender for events.
    sender: std::sync::mpsc::Sender<TuiEvent>,
    /// Receiver for events.
    receiver: Option<std::sync::mpsc::Receiver<TuiEvent>>,
    /// Tick rate for generating tick events (reserved for future dynamic adjustments).
    _tick_rate: Duration,
}

impl EventHandler {
    /// Creates a new event handler.
    pub fn new() -> Result<Self> {
        Self::with_tick_rate(DEFAULT_TICK_RATE)
    }

    /// Creates a new event handler with a specific tick rate.
    pub fn with_tick_rate(tick_rate: Duration) -> Result<Self> {
        let (sender, receiver) = std::sync::mpsc::channel();

        // Spawn event polling task
        let sender_clone = sender.clone();
        std::thread::spawn(move || {
            Self::poll_events(sender_clone, tick_rate);
        });

        Ok(Self {
            sender,
            receiver: Some(receiver),
            _tick_rate: tick_rate,
        })
    }

    /// Gets the next event from the event stream.
    pub fn recv(&mut self) -> Result<TuiEvent> {
        match self.receiver.as_mut() {
            Some(rx) => {
                rx.recv()
                    .map_err(|e| TuiError::Receive(format!("Event channel closed: {}", e)))
            }
            None => Err(TuiError::State("Event receiver not available".to_string())),
        }
    }

    /// Sends an event to the handler.
    pub fn send(&self, event: TuiEvent) -> Result<()> {
        self.sender
            .send(event)
            .map_err(|e| TuiError::Send(format!("Failed to send event: {}", e)))
    }

    /// Polls for terminal events in a background thread.
    fn poll_events(sender: std::sync::mpsc::Sender<TuiEvent>, tick_rate: Duration) {
        let mut last_tick = std::time::Instant::now();

        loop {
            // Poll for crossterm event with timeout
            if event::poll(tick_rate.saturating_sub(last_tick.elapsed()))
                .map_err(|e| TuiError::Input(format!("Poll failed: {}", e)))
                .is_err()
            {
                let _ = sender.send(TuiEvent::Tick);
                continue;
            }

            // Match the event
            match event::read()
                .map_err(|e| TuiError::Input(format!("Read failed: {}", e)))
            {
                Ok(Event::Key(key)) => {
                    // Only send key press events, not repeat/release
                    if key.kind == KeyEventKind::Press {
                        let _ = sender.send(TuiEvent::Key(key));
                    }
                }
                Ok(Event::Mouse(mouse)) => {
                    let _ = sender.send(TuiEvent::Mouse(mouse));
                }
                Ok(Event::Resize(width, height)) => {
                    let _ = sender.send(TuiEvent::Resize(width, height));
                }
                Ok(_) => {
                    // Ignore other events
                }
                Err(_) => {
                    // Poll timeout, send tick if needed
                    if last_tick.elapsed() >= tick_rate {
                        let _ = sender.send(TuiEvent::Tick);
                        last_tick = std::time::Instant::now();
                    }
                }
            }
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            let (sender, receiver) = std::sync::mpsc::channel();
            Self {
                sender,
                receiver: Some(receiver),
                _tick_rate: DEFAULT_TICK_RATE,
            }
        })
    }
}

/// Handles keyboard input for the application.
pub struct InputHandler {
    /// Current focus mode.
    focus: FocusMode,
}

impl InputHandler {
    /// Creates a new input handler.
    pub fn new() -> Self {
        Self {
            focus: FocusMode::default(),
        }
    }

    /// Sets the current focus mode.
    pub fn set_focus(&mut self, focus: FocusMode) {
        self.focus = focus;
    }

    /// Gets the current focus mode.
    pub fn focus(&self) -> FocusMode {
        self.focus
    }

    /// Handles a key event and returns the action to take.
    pub fn handle_key(&self, key: &KeyEvent) -> KeyAction {
        match self.focus {
            FocusMode::DialogList => self.handle_dialog_list_key(key),
            FocusMode::MessageView => self.handle_message_view_key(key),
            FocusMode::InputArea => self.handle_input_area_key(key),
        }
    }

    /// Handles key events when focused on the dialog list.
    fn handle_dialog_list_key(&self, key: &KeyEvent) -> KeyAction {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => KeyAction::NavigateUp,
            KeyCode::Down | KeyCode::Char('j') => KeyAction::NavigateDown,
            KeyCode::Enter | KeyCode::Char('l') => KeyAction::SelectDialog,
            KeyCode::Char('i') => KeyAction::FocusInput,
            KeyCode::Char('q') | KeyCode::Esc => KeyAction::Quit,
            KeyCode::Char('/') => KeyAction::FocusSearch,
            _ => KeyAction::None,
        }
    }

    /// Handles key events when focused on the message view.
    fn handle_message_view_key(&self, key: &KeyEvent) -> KeyAction {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => KeyAction::ScrollUp,
            KeyCode::Down | KeyCode::Char('j') => KeyAction::ScrollDown,
            KeyCode::PageUp => KeyAction::PageUp,
            KeyCode::PageDown => KeyAction::PageDown,
            KeyCode::Home => KeyAction::ScrollToTop,
            KeyCode::End => KeyAction::ScrollToBottom,
            KeyCode::Char('i') => KeyAction::FocusInput,
            KeyCode::Char('h') => KeyAction::FocusDialogList,
            KeyCode::Char('q') | KeyCode::Esc => KeyAction::Quit,
            _ => KeyAction::None,
        }
    }

    /// Handles key events when focused on the input area.
    fn handle_input_area_key(&self, key: &KeyEvent) -> KeyAction {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key.code {
            KeyCode::Enter => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    KeyAction::NewLine
                } else {
                    KeyAction::SendMessage
                }
            }
            KeyCode::Char('h') => KeyAction::FocusDialogList,
            KeyCode::Esc => KeyAction::CancelInput,
            KeyCode::Backspace => KeyAction::Backspace,
            _ => KeyAction::None,
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Action to take in response to a key event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAction {
    /// No action.
    None,
    /// Navigate up in a list.
    NavigateUp,
    /// Navigate down in a list.
    NavigateDown,
    /// Navigate to the left.
    NavigateLeft,
    /// Navigate to the right.
    NavigateRight,
    /// Select the current item.
    SelectDialog,
    /// Scroll up.
    ScrollUp,
    /// Scroll down.
    ScrollDown,
    /// Page up.
    PageUp,
    /// Page down.
    PageDown,
    /// Scroll to top.
    ScrollToTop,
    /// Scroll to bottom.
    ScrollToBottom,
    /// Focus the dialog list.
    FocusDialogList,
    /// Focus the message view.
    FocusMessageView,
    /// Focus the input area.
    FocusInput,
    /// Focus the search box.
    FocusSearch,
    /// Send the current message.
    SendMessage,
    /// Add a new line to input.
    NewLine,
    /// Delete the last character.
    Backspace,
    /// Cancel current input.
    CancelInput,
    /// Quit the application.
    Quit,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

    #[test]
    fn test_input_handler_default() {
        let handler = InputHandler::new();
        assert_eq!(handler.focus(), FocusMode::DialogList);
    }

    #[test]
    fn test_set_focus() {
        let mut handler = InputHandler::new();
        handler.set_focus(FocusMode::InputArea);
        assert_eq!(handler.focus(), FocusMode::InputArea);
    }

    #[test]
    fn test_dialog_list_navigation() {
        let handler = InputHandler::new();

        let up_key = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&up_key), KeyAction::NavigateUp);

        let down_key = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&down_key), KeyAction::NavigateDown);

        let enter_key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&enter_key), KeyAction::SelectDialog);
    }

    #[test]
    fn test_message_view_navigation() {
        let mut handler = InputHandler::new();
        handler.set_focus(FocusMode::MessageView);

        let page_up = KeyEvent::new(KeyCode::PageUp, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&page_up), KeyAction::PageUp);

        let page_down = KeyEvent::new(KeyCode::PageDown, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&page_down), KeyAction::PageDown);

        let home = KeyEvent::new(KeyCode::Home, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&home), KeyAction::ScrollToTop);
    }

    #[test]
    fn test_input_area_send() {
        let mut handler = InputHandler::new();
        handler.set_focus(FocusMode::InputArea);

        let enter = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&enter), KeyAction::SendMessage);

        let alt_enter = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::ALT);
        assert_eq!(handler.handle_key(&alt_enter), KeyAction::NewLine);
    }

    #[test]
    fn test_quit_key() {
        let handler = InputHandler::new();

        let q_key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&q_key), KeyAction::Quit);

        let esc = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&esc), KeyAction::Quit);
    }

    #[test]
    fn test_focus_switching() {
        let mut handler = InputHandler::new();

        // From dialog list, 'i' switches to input
        let i_key = KeyEvent::new(KeyCode::Char('i'), crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&i_key), KeyAction::FocusInput);

        // From input, 'h' switches to dialog list
        handler.set_focus(FocusMode::InputArea);
        let h_key = KeyEvent::new(KeyCode::Char('h'), crossterm::event::KeyModifiers::empty());
        assert_eq!(handler.handle_key(&h_key), KeyAction::FocusDialogList);
    }
}
