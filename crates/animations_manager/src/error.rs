//! Error types for AnimationsManager.

use std::fmt;

/// Errors that can occur in the AnimationsManager.
///
/// # Examples
///
/// ```
/// use rustgram_animations_manager::AnimationsManagerError;
///
/// let err = AnimationsManagerError::AnimationNotFound(123);
/// assert_eq!(format!("{}", err), "Animation not found: 123");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationsManagerError {
    /// Animation not found for the given FileId.
    AnimationNotFound(i32),

    /// Invalid animation data provided.
    InvalidAnimation(String),

    /// Animation search parameters not initialized.
    SearchParametersNotInitialized,

    /// Saved animations not loaded.
    SavedAnimationsNotLoaded,

    /// Operation failed on server.
    ServerError(String),

    /// Network error occurred.
    NetworkError(String),

    /// File reference error.
    FileReferenceError(String),

    /// Invalid input file.
    InvalidInputFile(String),
}

impl fmt::Display for AnimationsManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AnimationNotFound(id) => write!(f, "Animation not found: {}", id),
            Self::InvalidAnimation(msg) => write!(f, "Invalid animation: {}", msg),
            Self::SearchParametersNotInitialized => {
                write!(f, "Animation search parameters not initialized")
            }
            Self::SavedAnimationsNotLoaded => write!(f, "Saved animations not loaded"),
            Self::ServerError(msg) => write!(f, "Server error: {}", msg),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::FileReferenceError(msg) => write!(f, "File reference error: {}", msg),
            Self::InvalidInputFile(msg) => write!(f, "Invalid input file: {}", msg),
        }
    }
}

impl std::error::Error for AnimationsManagerError {}

/// Result type for AnimationsManager operations.
pub type Result<T> = std::result::Result<T, AnimationsManagerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", AnimationsManagerError::AnimationNotFound(123)),
            "Animation not found: 123"
        );
        assert_eq!(
            format!(
                "{}",
                AnimationsManagerError::InvalidAnimation("test".to_string())
            ),
            "Invalid animation: test"
        );
        assert_eq!(
            format!("{}", AnimationsManagerError::SearchParametersNotInitialized),
            "Animation search parameters not initialized"
        );
        assert_eq!(
            format!("{}", AnimationsManagerError::SavedAnimationsNotLoaded),
            "Saved animations not loaded"
        );
        assert_eq!(
            format!(
                "{}",
                AnimationsManagerError::ServerError("error".to_string())
            ),
            "Server error: error"
        );
        assert_eq!(
            format!(
                "{}",
                AnimationsManagerError::NetworkError("net".to_string())
            ),
            "Network error: net"
        );
        assert_eq!(
            format!(
                "{}",
                AnimationsManagerError::FileReferenceError("ref".to_string())
            ),
            "File reference error: ref"
        );
        assert_eq!(
            format!(
                "{}",
                AnimationsManagerError::InvalidInputFile("file".to_string())
            ),
            "Invalid input file: file"
        );
    }

    #[test]
    fn test_error_equality() {
        let err1 = AnimationsManagerError::AnimationNotFound(123);
        let err2 = AnimationsManagerError::AnimationNotFound(123);
        assert_eq!(err1, err2);

        let err3 = AnimationsManagerError::AnimationNotFound(456);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_result_type() {
        let ok: Result<()> = Ok(());
        assert!(ok.is_ok());

        let err: Result<()> = Err(AnimationsManagerError::AnimationNotFound(0));
        assert!(err.is_err());
    }
}
