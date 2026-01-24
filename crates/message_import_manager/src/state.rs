//! State types for message import operations.

use serde::{Deserialize, Serialize};

/// State of an active message import operation.
///
/// The import process follows a state machine:
///
/// ```text
///    Idle
///     |
///     v
/// Uploading
///     |
///     v
/// Importing
///     |
///     v
/// Completed
///
/// Any state can transition to Failed on error
/// ```
///
/// # Example
///
/// ```rust
/// use rustgram_message_import_manager::ImportState;
///
/// let state = ImportState::Idle;
/// assert_eq!(state, ImportState::Idle);
///
/// let uploading = ImportState::Uploading;
/// assert!(uploading.is_active());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ImportState {
    /// No import is currently in progress.
    ///
    /// This is the initial state for all dialogs.
    #[default]
    Idle,

    /// Files are being uploaded.
    ///
    /// In this state, the main message file and any attachments
    /// are being uploaded to the server.
    Uploading,

    /// Messages are being imported.
    ///
    /// In this state, the uploaded files are being processed
    /// and messages are being added to the dialog.
    Importing,

    /// The import completed successfully.
    Completed,

    /// The import failed.
    ///
    /// This state is entered when an error occurs at any point
    /// during the import process.
    Failed,
}

impl ImportState {
    /// Returns `true` if the import is currently active.
    ///
    /// Active states are `Uploading` and `Importing`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::ImportState;
    ///
    /// assert!(!ImportState::Idle.is_active());
    /// assert!(ImportState::Uploading.is_active());
    /// assert!(ImportState::Importing.is_active());
    /// assert!(!ImportState::Completed.is_active());
    /// assert!(!ImportState::Failed.is_active());
    /// ```
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Uploading | Self::Importing)
    }

    /// Returns `true` if the import has finished.
    ///
    /// Finished states are `Completed` and `Failed`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::ImportState;
    ///
    /// assert!(!ImportState::Idle.is_finished());
    /// assert!(!ImportState::Uploading.is_finished());
    /// assert!(!ImportState::Importing.is_finished());
    /// assert!(ImportState::Completed.is_finished());
    /// assert!(ImportState::Failed.is_finished());
    /// ```
    #[must_use]
    pub const fn is_finished(self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }

    /// Returns `true` if the import was successful.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::ImportState;
    ///
    /// assert!(!ImportState::Idle.is_successful());
    /// assert!(!ImportState::Uploading.is_successful());
    /// assert!(!ImportState::Importing.is_successful());
    /// assert!(ImportState::Completed.is_successful());
    /// assert!(!ImportState::Failed.is_successful());
    /// ```
    #[must_use]
    pub const fn is_successful(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns `true` if the import failed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::ImportState;
    ///
    /// assert!(!ImportState::Idle.is_failed());
    /// assert!(!ImportState::Uploading.is_failed());
    /// assert!(!ImportState::Importing.is_failed());
    /// assert!(!ImportState::Completed.is_failed());
    /// assert!(ImportState::Failed.is_failed());
    /// ```
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// Type of message file being imported.
///
/// # Example
///
/// ```rust
/// use rustgram_message_import_manager::MessageFileType;
///
/// let file_type = MessageFileType::Json;
/// assert_eq!(file_type, MessageFileType::Json);
///
/// let unknown = MessageFileType::Unknown;
/// assert!(unknown.is_unknown());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MessageFileType {
    /// JSON format export file.
    ///
    /// This is the most common format for message exports,
    /// containing structured JSON data with messages, metadata, and attachments.
    Json,

    /// Telegram native format.
    ///
    /// This is the internal Telegram format used for message backups.
    Telegram,

    /// Plain text format.
    ///
    /// Simple text-based export with limited formatting support.
    Txt,

    /// Unknown or unrecognized format.
    ///
    /// The file type could not be determined from the header.
    #[default]
    Unknown,
}

impl MessageFileType {
    /// Returns `true` if the file type is known.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageFileType;
    ///
    /// assert!(MessageFileType::Json.is_known());
    /// assert!(MessageFileType::Telegram.is_known());
    /// assert!(MessageFileType::Txt.is_known());
    /// assert!(!MessageFileType::Unknown.is_known());
    /// ```
    #[must_use]
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Returns `true` if the file type is unknown.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageFileType;
    ///
    /// assert!(!MessageFileType::Json.is_unknown());
    /// assert!(!MessageFileType::Telegram.is_unknown());
    /// assert!(!MessageFileType::Txt.is_unknown());
    /// assert!(MessageFileType::Unknown.is_unknown());
    /// ```
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns the file extension for this type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageFileType;
    ///
    /// assert_eq!(MessageFileType::Json.extension(), "json");
    /// assert_eq!(MessageFileType::Telegram.extension(), "tg");
    /// assert_eq!(MessageFileType::Txt.extension(), "txt");
    /// assert_eq!(MessageFileType::Unknown.extension(), "bin");
    /// ```
    #[must_use]
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Telegram => "tg",
            Self::Txt => "txt",
            Self::Unknown => "bin",
        }
    }

    /// Returns the MIME type for this file type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::MessageFileType;
    ///
    /// assert_eq!(MessageFileType::Json.mime_type(), "application/json");
    /// assert_eq!(MessageFileType::Telegram.mime_type(), "application/x-telegram");
    /// assert_eq!(MessageFileType::Txt.mime_type(), "text/plain");
    /// assert_eq!(MessageFileType::Unknown.mime_type(), "application/octet-stream");
    /// ```
    #[must_use]
    pub const fn mime_type(self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::Telegram => "application/x-telegram",
            Self::Txt => "text/plain",
            Self::Unknown => "application/octet-stream",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ImportState tests
    #[test]
    fn test_import_state_is_active() {
        assert!(!ImportState::Idle.is_active());
        assert!(ImportState::Uploading.is_active());
        assert!(ImportState::Importing.is_active());
        assert!(!ImportState::Completed.is_active());
        assert!(!ImportState::Failed.is_active());
    }

    #[test]
    fn test_import_state_is_finished() {
        assert!(!ImportState::Idle.is_finished());
        assert!(!ImportState::Uploading.is_finished());
        assert!(!ImportState::Importing.is_finished());
        assert!(ImportState::Completed.is_finished());
        assert!(ImportState::Failed.is_finished());
    }

    #[test]
    fn test_import_state_is_successful() {
        assert!(!ImportState::Idle.is_successful());
        assert!(!ImportState::Uploading.is_successful());
        assert!(!ImportState::Importing.is_successful());
        assert!(ImportState::Completed.is_successful());
        assert!(!ImportState::Failed.is_successful());
    }

    #[test]
    fn test_import_state_is_failed() {
        assert!(!ImportState::Idle.is_failed());
        assert!(!ImportState::Uploading.is_failed());
        assert!(!ImportState::Importing.is_failed());
        assert!(!ImportState::Completed.is_failed());
        assert!(ImportState::Failed.is_failed());
    }

    #[test]
    fn test_import_state_default() {
        assert_eq!(ImportState::default(), ImportState::Idle);
    }

    #[test]
    fn test_import_state_copy() {
        let state1 = ImportState::Uploading;
        let state2 = state1;
        assert_eq!(state1, ImportState::Uploading);
        assert_eq!(state2, ImportState::Uploading);
    }

    #[test]
    fn test_import_state_equality() {
        assert_eq!(ImportState::Idle, ImportState::Idle);
        assert_eq!(ImportState::Uploading, ImportState::Uploading);
        assert_ne!(ImportState::Uploading, ImportState::Importing);
    }

    #[test]
    fn test_import_state_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ImportState::Idle);
        set.insert(ImportState::Uploading);
        assert_eq!(set.len(), 2);
    }

    // MessageFileType tests
    #[test]
    fn test_message_file_type_is_known() {
        assert!(MessageFileType::Json.is_known());
        assert!(MessageFileType::Telegram.is_known());
        assert!(MessageFileType::Txt.is_known());
        assert!(!MessageFileType::Unknown.is_known());
    }

    #[test]
    fn test_message_file_type_is_unknown() {
        assert!(!MessageFileType::Json.is_unknown());
        assert!(!MessageFileType::Telegram.is_unknown());
        assert!(!MessageFileType::Txt.is_unknown());
        assert!(MessageFileType::Unknown.is_unknown());
    }

    #[test]
    fn test_message_file_type_extension() {
        assert_eq!(MessageFileType::Json.extension(), "json");
        assert_eq!(MessageFileType::Telegram.extension(), "tg");
        assert_eq!(MessageFileType::Txt.extension(), "txt");
        assert_eq!(MessageFileType::Unknown.extension(), "bin");
    }

    #[test]
    fn test_message_file_type_mime_type() {
        assert_eq!(MessageFileType::Json.mime_type(), "application/json");
        assert_eq!(
            MessageFileType::Telegram.mime_type(),
            "application/x-telegram"
        );
        assert_eq!(MessageFileType::Txt.mime_type(), "text/plain");
        assert_eq!(
            MessageFileType::Unknown.mime_type(),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_message_file_type_default() {
        assert_eq!(MessageFileType::default(), MessageFileType::Unknown);
    }

    #[test]
    fn test_message_file_type_copy() {
        let ft1 = MessageFileType::Json;
        let ft2 = ft1;
        assert_eq!(ft1, MessageFileType::Json);
        assert_eq!(ft2, MessageFileType::Json);
    }

    #[test]
    fn test_message_file_type_equality() {
        assert_eq!(MessageFileType::Json, MessageFileType::Json);
        assert_eq!(MessageFileType::Telegram, MessageFileType::Telegram);
        assert_ne!(MessageFileType::Json, MessageFileType::Telegram);
    }

    #[test]
    fn test_message_file_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(MessageFileType::Json);
        set.insert(MessageFileType::Telegram);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_import_state_debug() {
        let state = ImportState::Uploading;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Uploading"));
    }

    #[test]
    fn test_message_file_type_debug() {
        let ft = MessageFileType::Json;
        let debug_str = format!("{:?}", ft);
        assert!(debug_str.contains("Json"));
    }

    #[test]
    fn test_all_import_states_exist() {
        let states = [
            ImportState::Idle,
            ImportState::Uploading,
            ImportState::Importing,
            ImportState::Completed,
            ImportState::Failed,
        ];
        assert_eq!(states.len(), 5);
    }

    #[test]
    fn test_all_message_file_types_exist() {
        let types = [
            MessageFileType::Json,
            MessageFileType::Telegram,
            MessageFileType::Txt,
            MessageFileType::Unknown,
        ];
        assert_eq!(types.len(), 4);
    }

    #[test]
    fn test_import_state_serialization() {
        let state = ImportState::Uploading;
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("Uploading"));

        let deserialized: ImportState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_message_file_type_serialization() {
        let ft = MessageFileType::Json;
        let json = serde_json::to_string(&ft).unwrap();
        assert!(json.contains("Json"));

        let deserialized: MessageFileType = serde_json::from_str(&json).unwrap();
        assert_eq!(ft, deserialized);
    }
}
