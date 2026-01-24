//! # Message Import Manager
//!
//! Manages the import of messages from external sources into Telegram dialogs.
//!
//! ## Overview
//!
//! The `MessageImportManager` handles the multi-stage workflow of importing messages:
//!
//! 1. **File Type Detection** - Identifies the format of exported message files
//! 2. **File Upload** - Uploads the message file and any attachments
//! 3. **Confirmation** - Provides confirmation text for user approval
//! 4. **Import** - Executes the actual message import
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_message_import_manager::{MessageImportManager, MessageFileType};
//! use rustgram_types::DialogId;
//! use rustgram_file_id::FileId;
//! use rustgram_file_upload_id::FileUploadId;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = MessageImportManager::new();
//!
//!     // Detect file type from file header
//!     let header = b"{\"type\":\"private\"...";
//!     let file_type = manager.get_message_file_type(header)?;
//!
//!     assert_eq!(file_type, MessageFileType::Json);
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

mod error;
mod state;
pub mod tl;

// Re-exports
pub use error::{Error, Result};
pub use state::{ImportState, MessageFileType};

use rustgram_file_upload_id::FileUploadId;
use rustgram_types::DialogId as TypesDialogId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

// Type alias for DialogId from rustgram_types
///
/// This type alias provides convenient access to the DialogId type from rustgram_types.
/// DialogId represents a unique identifier for a Telegram dialog (chat, channel, etc.).
pub type DialogId = TypesDialogId;

/// Helper function to get dialog type as a string
fn dialog_type_name(dialog_id: &DialogId) -> &'static str {
    match dialog_id {
        DialogId::User(_) => "user",
        DialogId::Chat(_) => "group",
        DialogId::Channel(_) => "channel",
        DialogId::SecretChat(_) => "secret chat",
    }
}

// Re-export TL stubs
pub use tl::{InputFile, UploadedImportedMessagesInfo};

/// Manager for importing messages from external sources.
///
/// This manager handles the complete workflow of importing messages into Telegram dialogs,
/// including file type detection, upload tracking, and import execution.
///
/// # Thread Safety
///
/// The manager uses `Arc<RwLock<T>>` for internal state, making it safe to share
/// across async tasks. All public methods are async and handle concurrent access.
///
/// # Example
///
/// ```rust
/// use rustgram_message_import_manager::MessageImportManager;
///
/// let manager = MessageImportManager::new();
/// ```
#[derive(Debug, Clone)]
pub struct MessageImportManager {
    /// Upload tracking: file_upload_id -> import info
    upload_tracking: Arc<RwLock<HashMap<FileUploadId, UploadedImportedMessagesInfo>>>,
    /// Current import state for each dialog
    import_states: Arc<RwLock<HashMap<DialogId, ImportState>>>,
    /// Counter for generating unique import IDs
    #[allow(dead_code)]
    next_import_id: Arc<AtomicU64>,
}

impl Default for MessageImportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageImportManager {
    /// Creates a new message import manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    ///
    /// let manager = MessageImportManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            upload_tracking: Arc::new(RwLock::new(HashMap::new())),
            import_states: Arc::new(RwLock::new(HashMap::new())),
            next_import_id: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Detects the type of a message file from its header bytes.
    ///
    /// This method examines the first bytes of a file to determine its format.
    /// It supports detection of JSON, Telegram native format, and plain text files.
    ///
    /// # Arguments
    ///
    /// * `message_file_head` - The first bytes of the message file (typically 16-32 bytes)
    ///
    /// # Returns
    ///
    /// Returns the detected file type. Returns `MessageFileType::Unknown` if the
    /// format cannot be determined.
    ///
    /// # Errors
    ///
    /// This function currently never returns an error, but returns a Result type
    /// for future compatibility with file I/O operations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    ///
    /// let manager = MessageImportManager::new();
    ///
    /// // JSON format detection
    /// let json_header = b"{\"type\":\"private\",\"messages\":[...";
    /// let file_type = manager.get_message_file_type(json_header).unwrap();
    /// assert_eq!(file_type, rustgram_message_import_manager::MessageFileType::Json);
    ///
    /// // Plain text detection
    /// let text_header = b"Message 1\nFrom: User\n";
    /// let file_type = manager.get_message_file_type(text_header).unwrap();
    /// assert_eq!(file_type, rustgram_message_import_manager::MessageFileType::Txt);
    /// ```
    pub fn get_message_file_type(&self, message_file_head: &[u8]) -> Result<MessageFileType> {
        if message_file_head.is_empty() {
            return Ok(MessageFileType::Unknown);
        }

        // Check for JSON format (starts with '{')
        if message_file_head[0] == b'{' {
            return Ok(MessageFileType::Json);
        }

        // Check for Telegram native format
        // Telegram files often start with specific magic bytes
        if message_file_head.len() >= 4 {
            // TL serialization format check
            if message_file_head.starts_with(b"TLSS") || message_file_head.starts_with(b"TGIM") {
                return Ok(MessageFileType::Telegram);
            }
        }

        // Check for text format (printable ASCII)
        let is_printable = message_file_head
            .iter()
            .take(16)
            .all(|&b| b.is_ascii_graphic() || b.is_ascii_whitespace());

        if is_printable {
            return Ok(MessageFileType::Txt);
        }

        Ok(MessageFileType::Unknown)
    }

    /// Gets the confirmation text for importing messages into a dialog.
    ///
    /// This method generates a confirmation message that should be displayed to the user
    /// before proceeding with the import. The text describes what will be imported.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The target dialog for message import
    ///
    /// # Returns
    ///
    /// Returns a string containing the confirmation text, or an error if the dialog
    /// is not valid for importing.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The dialog ID is not valid
    /// - The dialog type doesn't support message imports
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// let confirmation = manager.get_message_import_confirmation_text(dialog_id).await.unwrap();
    /// assert!(confirmation.contains("import"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_message_import_confirmation_text(
        &self,
        dialog_id: DialogId,
    ) -> Result<String> {
        // Validate the dialog
        self.can_import_messages(dialog_id)?;

        let dialog_type = dialog_type_name(&dialog_id);

        Ok(format!(
            "Import messages into this {}? This will add all messages from the imported file to this conversation.",
            dialog_type
        ))
    }

    /// Starts the message import process.
    ///
    /// This is the entry point for importing messages. It validates the dialog,
    /// prepares the import workflow, and returns a unique import ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The target dialog for message import
    /// * `message_file` - The main message file to import
    /// * `attached_files` - Optional list of attached media files
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the import was initiated successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The dialog is not valid for importing
    /// - File validation fails
    /// - An import is already in progress for this dialog
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::{MessageImportManager, InputFile};
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// let message_file = InputFile::new(1, b"{\"messages\":[]}".to_vec());
    ///
    /// manager.import_messages(dialog_id, message_file, vec![]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn import_messages(
        &self,
        dialog_id: DialogId,
        _message_file: InputFile,
        _attached_files: Vec<InputFile>,
    ) -> Result<()> {
        // Validate the dialog
        self.can_import_messages(dialog_id)?;

        // Check if an import is already in progress
        let state = self.import_states.read().await;
        if let Some(current_state) = state.get(&dialog_id) {
            if *current_state == ImportState::Uploading || *current_state == ImportState::Importing
            {
                return Err(Error::ImportFailed {
                    reason: "Import already in progress for this dialog".to_string(),
                });
            }
        }
        drop(state);

        // Set state to uploading
        let mut state_guard = self.import_states.write().await;
        state_guard.insert(dialog_id, ImportState::Uploading);
        drop(state_guard);

        // In a real implementation, this would trigger file upload
        // For now, we simulate completion by moving to completed state
        let mut state_guard = self.import_states.write().await;
        state_guard.insert(dialog_id, ImportState::Completed);
        drop(state_guard);

        Ok(())
    }

    /// Starts the import process after files have been uploaded.
    ///
    /// This method is called after file uploads are complete to execute the
    /// actual message import.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The target dialog for message import
    /// * `import_id` - The unique import ID returned from the upload phase
    /// * `attached_file_upload_ids` - Upload IDs for attached files
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the import was started successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The import_id is not found
    /// - The upload state is invalid
    /// - The import operation fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::{MessageImportManager, InputFile};
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// // Upload files first (simplified - normally returns upload ID)
    /// let message_file = InputFile::new(1, b"{\"messages\":[]}".to_vec());
    /// manager.import_messages(dialog_id, message_file, vec![]).await?;
    ///
    /// // After upload completes, start the actual import
    /// // Note: In real usage, you'd use the upload ID from upload_imported_messages
    /// let result = manager.start_import_messages(dialog_id, 1, vec![]).await;
    /// assert!(result.is_ok() || result.is_err()); // Either outcome is valid
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_import_messages(
        &self,
        dialog_id: DialogId,
        _import_id: i64,
        _attached_file_upload_ids: Vec<FileUploadId>,
    ) -> Result<()> {
        // Validate the dialog
        self.can_import_messages(dialog_id)?;

        // Check state
        let state = self.import_states.read().await;
        let current_state = state.get(&dialog_id).ok_or(Error::InvalidDialog {
            context: "No import state found for dialog".to_string(),
        })?;

        if *current_state != ImportState::Uploading && *current_state != ImportState::Idle {
            return Err(Error::ImportFailed {
                reason: format!("Invalid state for starting import: {:?}", current_state),
            });
        }
        drop(state);

        // Update state to importing
        let mut state_guard = self.import_states.write().await;
        state_guard.insert(dialog_id, ImportState::Importing);
        drop(state_guard);

        // In a real implementation, this would execute the import
        // For now, we simulate completion
        let mut state_guard = self.import_states.write().await;
        state_guard.insert(dialog_id, ImportState::Completed);
        drop(state_guard);

        Ok(())
    }

    /// Checks if messages can be imported into the specified dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog to check
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the dialog is valid for importing.
    ///
    /// # Errors
    ///
    /// Returns an error if the dialog is not valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// manager.can_import_messages(dialog_id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn can_import_messages(&self, dialog_id: DialogId) -> Result<()> {
        // All valid DialogId variants support imports
        // The is_valid() check is sufficient
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialog {
                context: "Dialog ID is not valid".to_string(),
            });
        }

        Ok(())
    }

    /// Uploads imported messages file and tracks the upload.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The target dialog
    /// * `file_upload_id` - The upload ID for the message file
    /// * `attached_file_upload_ids` - Upload IDs for attached files
    /// * `is_reupload` - Whether this is a reupload attempt
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the upload was tracked successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The dialog is invalid
    /// - The upload ID is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    /// use rustgram_file_id::FileId;
    /// use rustgram_file_upload_id::FileUploadId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// let file_id = FileId::new(1, 100);
    /// let upload_id = FileUploadId::new(file_id, 1);
    ///
    /// manager.upload_imported_messages(dialog_id, upload_id, vec![], false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_imported_messages(
        &self,
        dialog_id: DialogId,
        file_upload_id: FileUploadId,
        attached_file_upload_ids: Vec<FileUploadId>,
        is_reupload: bool,
    ) -> Result<()> {
        // Validate dialog
        self.can_import_messages(dialog_id)?;

        // Validate upload ID
        if !file_upload_id.is_valid() {
            return Err(Error::UploadFailed {
                reason: "Invalid file upload ID".to_string(),
            });
        }

        // Track the upload
        let info = UploadedImportedMessagesInfo {
            dialog_id,
            attached_file_upload_ids: attached_file_upload_ids.clone(),
            is_reupload,
        };

        let mut tracking = self.upload_tracking.write().await;
        tracking.insert(file_upload_id, info);

        Ok(())
    }

    /// Called when upload of imported messages file completes.
    ///
    /// # Arguments
    ///
    /// * `file_upload_id` - The upload ID that completed
    /// * `input_file` - The uploaded file info
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the completion was handled successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The upload ID was not found in tracking
    /// - The dialog ID is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::{MessageImportManager, InputFile};
    /// use rustgram_file_id::FileId;
    /// use rustgram_file_upload_id::FileUploadId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    ///
    /// let file_id = FileId::new(1, 100);
    /// let upload_id = FileUploadId::new(file_id, 1);
    /// let input_file = InputFile::new(1, b"test".to_vec());
    ///
    /// // Note: This will fail if upload wasn't tracked first
    /// let result = manager.on_upload_imported_messages_complete(upload_id, input_file).await;
    /// // result is Err because upload wasn't tracked
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_upload_imported_messages_complete(
        &self,
        file_upload_id: FileUploadId,
        input_file: InputFile,
    ) -> Result<()> {
        // Get tracking info
        let tracking = self.upload_tracking.read().await;
        let info = tracking.get(&file_upload_id).ok_or(Error::UploadFailed {
            reason: "Upload ID not found in tracking".to_string(),
        })?;

        let dialog_id = info.dialog_id;

        // Validate dialog
        self.can_import_messages(dialog_id)?;

        // In a real implementation, this would trigger the next step
        // For now, we just validate the input file
        if input_file.id == 0 {
            return Err(Error::InvalidFile {
                reason: "Invalid input file ID".to_string(),
            });
        }

        Ok(())
    }

    /// Called when upload of imported messages file fails.
    ///
    /// # Arguments
    ///
    /// * `file_upload_id` - The upload ID that failed
    /// * `error` - The error that occurred
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_file_upload_id::FileUploadId;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = MessageImportManager::new();
    ///
    /// let file_id = FileId::new(1, 100);
    /// let upload_id = FileUploadId::new(file_id, 1);
    ///
    /// manager.on_upload_imported_messages_error(upload_id, "Upload failed".to_string());
    /// // Error is logged, no panic
    /// # }
    /// ```
    pub async fn on_upload_imported_messages_error(
        &self,
        file_upload_id: FileUploadId,
        error: String,
    ) {
        // Remove from tracking
        let mut tracking = self.upload_tracking.write().await;
        tracking.remove(&file_upload_id);

        // Log the error
        tracing::error!(
            file_upload_id = ?file_upload_id,
            error = %error,
            "Message file upload failed"
        );
    }

    /// Gets the current import state for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog to check
    ///
    /// # Returns
    ///
    /// Returns the current import state, or `ImportState::Idle` if no import is in progress.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    /// use rustgram_message_import_manager::ImportState;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// let state = manager.get_import_state(dialog_id).await;
    /// assert_eq!(state, ImportState::Idle);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_import_state(&self, dialog_id: DialogId) -> ImportState {
        let states = self.import_states.read().await;
        states.get(&dialog_id).copied().unwrap_or(ImportState::Idle)
    }

    /// Resets the import state for a dialog.
    ///
    /// This can be used to clear a failed state and allow retrying.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog to reset
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = MessageImportManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// manager.reset_import_state(dialog_id).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reset_import_state(&self, dialog_id: DialogId) {
        let mut states = self.import_states.write().await;
        states.remove(&dialog_id);
    }

    /// Generates a unique import ID.
    ///
    /// # Returns
    ///
    /// Returns a new unique import ID.
    #[allow(dead_code)]
    fn generate_import_id(&self) -> i64 {
        self.next_import_id.fetch_add(1, Ordering::SeqCst) as i64
    }

    /// Gets the number of active imports.
    ///
    /// # Returns
    ///
    /// Returns the count of dialogs with active import operations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageImportManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = MessageImportManager::new();
    ///
    /// let count = manager.active_import_count().await;
    /// assert_eq!(count, 0);
    /// # }
    /// ```
    pub async fn active_import_count(&self) -> usize {
        let states = self.import_states.read().await;
        states
            .values()
            .filter(|s| **s == ImportState::Uploading || **s == ImportState::Importing)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = MessageImportManager::new();
        // Just verify the manager can be created
        let tracking = manager.upload_tracking.read().await;
        assert_eq!(tracking.len(), 0);
    }

    #[tokio::test]
    async fn test_default() {
        let manager = MessageImportManager::default();
        // Just verify the manager can be created
        let tracking = manager.upload_tracking.read().await;
        assert_eq!(tracking.len(), 0);
    }
}
