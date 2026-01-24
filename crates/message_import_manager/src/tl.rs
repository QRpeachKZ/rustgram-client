//! TL stub types for message import operations.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{FileUploadId, Result};
use rustgram_types::DialogId;

/// Stub for TDLib InputFile.
///
/// TODO: Replace with full TL implementation when available.
///
/// This stub provides a simplified representation of file input for
/// the message import functionality. In the full TDLib implementation,
/// this would be a complete TL type with all file metadata.
///
/// # Example
///
/// ```rust
/// use rustgram_message_import_manager::InputFile;
///
/// let file = InputFile::new(1, b"file content".to_vec());
/// assert_eq!(file.id, 1);
/// assert_eq!(file.parts.len(), 12);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InputFile {
    /// Unique identifier for the file.
    pub id: i64,

    /// File content or parts.
    pub parts: Vec<u8>,
}

impl InputFile {
    /// Creates a new InputFile.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the file
    /// * `parts` - File content or parts
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::InputFile;
    ///
    /// let file = InputFile::new(123, b"test content".to_vec());
    /// assert_eq!(file.id, 123);
    /// ```
    #[must_use]
    pub const fn new(id: i64, parts: Vec<u8>) -> Self {
        Self { id, parts }
    }

    /// Returns the file size in bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::InputFile;
    ///
    /// let file = InputFile::new(1, b"test".to_vec());
    /// assert_eq!(file.size(), 4);
    /// ```
    #[must_use]
    pub fn size(&self) -> usize {
        self.parts.len()
    }

    /// Returns `true` if the file is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::InputFile;
    ///
    /// let empty = InputFile::new(1, vec![]);
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = InputFile::new(2, b"data".to_vec());
    /// assert!(!non_empty.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Validates the file.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the file is valid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file ID is invalid (zero)
    /// - The file is empty (if required)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::InputFile;
    ///
    /// let file = InputFile::new(1, b"data".to_vec());
    /// assert!(file.validate().is_ok());
    ///
    /// let invalid = InputFile::new(0, b"data".to_vec());
    /// assert!(invalid.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.id == 0 {
            return Err(crate::Error::InvalidFile {
                reason: "File ID cannot be zero".to_string(),
            });
        }

        if self.is_empty() {
            return Err(crate::Error::InvalidFile {
                reason: "File cannot be empty".to_string(),
            });
        }

        Ok(())
    }
}

impl fmt::Display for InputFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InputFile(id={}, size={})", self.id, self.parts.len())
    }
}

/// Internal tracking for uploaded imported messages.
///
/// This structure tracks the state of uploaded message files and their
/// associated attachments during the import process.
///
/// # Example
///
/// ```rust
/// use rustgram_message_import_manager::UploadedImportedMessagesInfo;
/// use rustgram_types::DialogId;
/// use rustgram_types::UserId;
///
/// let user_id = UserId::new(123).unwrap();
/// let dialog_id = DialogId::from_user(user_id);
/// let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);
///
/// assert_eq!(info.dialog_id, dialog_id);
/// assert!(info.attached_file_upload_ids.is_empty());
/// assert!(!info.is_reupload);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadedImportedMessagesInfo {
    /// The dialog where messages will be imported.
    pub dialog_id: DialogId,

    /// Upload IDs for attached files.
    pub attached_file_upload_ids: Vec<FileUploadId>,

    /// Whether this is a reupload attempt.
    pub is_reupload: bool,
}

impl UploadedImportedMessagesInfo {
    /// Creates a new upload info.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The target dialog for import
    /// * `attached_file_upload_ids` - Upload IDs for attached files
    /// * `is_reupload` - Whether this is a reupload attempt
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::UploadedImportedMessagesInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], true);
    ///
    /// assert!(info.is_reupload);
    /// ```
    #[must_use]
    pub const fn new(
        dialog_id: DialogId,
        attached_file_upload_ids: Vec<FileUploadId>,
        is_reupload: bool,
    ) -> Self {
        Self {
            dialog_id,
            attached_file_upload_ids,
            is_reupload,
        }
    }

    /// Returns the number of attached files.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::UploadedImportedMessagesInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);
    ///
    /// assert_eq!(info.attachment_count(), 0);
    /// ```
    #[must_use]
    pub fn attachment_count(&self) -> usize {
        self.attached_file_upload_ids.len()
    }

    /// Returns `true` if there are attached files.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::UploadedImportedMessagesInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);
    ///
    /// assert!(!info.has_attachments());
    /// ```
    #[must_use]
    pub fn has_attachments(&self) -> bool {
        !self.attached_file_upload_ids.is_empty()
    }

    /// Validates the upload info.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the info is valid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The dialog ID is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::UploadedImportedMessagesInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);
    ///
    /// assert!(info.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if !self.dialog_id.is_valid() {
            return Err(crate::Error::InvalidDialog {
                context: "Dialog ID is not valid".to_string(),
            });
        }

        Ok(())
    }
}

impl fmt::Display for UploadedImportedMessagesInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UploadedImportedMessagesInfo(dialog_id={:?}, attachments={}, reupload={})",
            self.dialog_id,
            self.attachment_count(),
            self.is_reupload
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DialogId;
    use rustgram_types::UserId;

    // InputFile tests
    #[test]
    fn test_input_file_new() {
        let file = InputFile::new(1, b"test".to_vec());
        assert_eq!(file.id, 1);
        assert_eq!(file.parts, b"test");
    }

    #[test]
    fn test_input_file_size() {
        let file = InputFile::new(1, b"test content".to_vec());
        assert_eq!(file.size(), 12);
    }

    #[test]
    fn test_input_file_is_empty() {
        let empty = InputFile::new(1, vec![]);
        assert!(empty.is_empty());

        let non_empty = InputFile::new(2, b"data".to_vec());
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_input_file_validate_success() {
        let file = InputFile::new(1, b"data".to_vec());
        assert!(file.validate().is_ok());
    }

    #[test]
    fn test_input_file_validate_zero_id() {
        let file = InputFile::new(0, b"data".to_vec());
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_input_file_validate_empty() {
        let file = InputFile::new(1, vec![]);
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_input_file_clone() {
        let file1 = InputFile::new(1, b"test".to_vec());
        let file2 = file1.clone();
        assert_eq!(file1, file2);
    }

    #[test]
    fn test_input_file_equality() {
        let file1 = InputFile::new(1, b"test".to_vec());
        let file2 = InputFile::new(1, b"test".to_vec());
        assert_eq!(file1, file2);

        let file3 = InputFile::new(2, b"test".to_vec());
        assert_ne!(file1, file3);
    }

    #[test]
    fn test_input_file_display() {
        let file = InputFile::new(1, b"test".to_vec());
        let s = format!("{}", file);
        assert!(s.contains("InputFile"));
        assert!(s.contains("id=1"));
        assert!(s.contains("size=4"));
    }

    #[test]
    fn test_input_file_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(InputFile::new(1, b"test".to_vec()));
        set.insert(InputFile::new(2, b"test".to_vec()));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_input_file_serialization() {
        let file = InputFile::new(1, b"test".to_vec());
        let json = serde_json::to_string(&file).unwrap();

        let deserialized: InputFile = serde_json::from_str(&json).unwrap();
        assert_eq!(file, deserialized);
    }

    // UploadedImportedMessagesInfo tests
    #[test]
    fn test_uploaded_info_new() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);

        assert_eq!(info.dialog_id, dialog_id);
        assert!(info.attached_file_upload_ids.is_empty());
        assert!(!info.is_reupload);
    }

    #[test]
    fn test_uploaded_info_attachment_count() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);

        assert_eq!(info.attachment_count(), 0);
    }

    #[test]
    fn test_uploaded_info_has_attachments() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);

        assert!(!info.has_attachments());
    }

    #[test]
    fn test_uploaded_info_validate_success() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);

        assert!(info.validate().is_ok());
    }

    #[test]
    fn test_uploaded_info_clone() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info1 = UploadedImportedMessagesInfo::new(dialog_id.clone(), vec![], false);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_uploaded_info_equality() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info1 = UploadedImportedMessagesInfo::new(dialog_id.clone(), vec![], false);
        let info2 = UploadedImportedMessagesInfo::new(dialog_id.clone(), vec![], false);
        assert_eq!(info1, info2);

        let info3 = UploadedImportedMessagesInfo::new(dialog_id.clone(), vec![], true);
        assert_ne!(info1, info3);
    }

    #[test]
    fn test_uploaded_info_display() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);

        let s = format!("{}", info);
        assert!(s.contains("UploadedImportedMessagesInfo"));
        assert!(s.contains("attachments=0"));
        assert!(s.contains("reupload=false"));
    }

    #[test]
    fn test_uploaded_info_serialization() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);

        let json = serde_json::to_string(&info).unwrap();

        let deserialized: UploadedImportedMessagesInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }

    #[test]
    fn test_uploaded_info_with_attachments() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let upload_ids = vec![];

        let info = UploadedImportedMessagesInfo::new(dialog_id, upload_ids, false);
        assert_eq!(info.attachment_count(), 0);
    }

    #[test]
    fn test_uploaded_info_is_reupload() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let info1 = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);
        assert!(!info1.is_reupload);

        let info2 = UploadedImportedMessagesInfo::new(dialog_id, vec![], true);
        assert!(info2.is_reupload);
    }

    #[test]
    fn test_uploaded_info_const_constructor() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = UploadedImportedMessagesInfo::new(dialog_id, vec![], false);

        // Verify const constructor works correctly
        assert!(!info.is_reupload);
        assert_eq!(info.attachment_count(), 0);
    }
}
