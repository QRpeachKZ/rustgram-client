//! Error types for forum topic manager.

use std::fmt;

/// Errors that can occur when working with forum topics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The topic title exceeds maximum length.
    TitleTooLong {
        /// The actual title length.
        length: usize,
        /// The maximum allowed length.
        max: usize,
    },

    /// The dialog was not found.
    DialogNotFound,

    /// The topic was not found.
    TopicNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TitleTooLong { length, max } => {
                write!(
                    f,
                    "Title length {} exceeds maximum allowed length {}",
                    length, max
                )
            }
            Error::DialogNotFound => write!(f, "Dialog not found"),
            Error::TopicNotFound => write!(f, "Topic not found"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_title_too_long_display() {
        let error = Error::TitleTooLong {
            length: 200,
            max: 128,
        };
        let display = format!("{}", error);
        assert!(display.contains("200"));
        assert!(display.contains("128"));
        assert!(display.contains("Title"));
    }

    #[test]
    fn test_error_dialog_not_found_display() {
        let error = Error::DialogNotFound;
        let display = format!("{}", error);
        assert!(display.contains("Dialog"));
        assert!(display.contains("not found"));
    }

    #[test]
    fn test_error_topic_not_found_display() {
        let error = Error::TopicNotFound;
        let display = format!("{}", error);
        assert!(display.contains("Topic"));
        assert!(display.contains("not found"));
    }

    #[test]
    fn test_error_title_too_long_equality() {
        let error1 = Error::TitleTooLong {
            length: 200,
            max: 128,
        };
        let error2 = Error::TitleTooLong {
            length: 200,
            max: 128,
        };
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_title_too_long_inequality() {
        let error1 = Error::TitleTooLong {
            length: 200,
            max: 128,
        };
        let error2 = Error::TitleTooLong {
            length: 150,
            max: 128,
        };
        assert_ne!(error1, error2);
    }

    #[test]
    fn test_error_dialog_not_found_equality() {
        let error1 = Error::DialogNotFound;
        let error2 = Error::DialogNotFound;
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_topic_not_found_equality() {
        let error1 = Error::TopicNotFound;
        let error2 = Error::TopicNotFound;
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_variants_not_equal() {
        assert_ne!(Error::DialogNotFound, Error::TopicNotFound);
    }
}
