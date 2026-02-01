// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Internal types for business manager.

use std::fmt;

/// A business chat link.
///
/// Represents a link that can be shared with users to start a chat
/// with a business account.
///
/// # Examples
///
/// ```rust
/// use rustgram_business_manager::BusinessChatLink;
///
/// let link = BusinessChatLink {
///     link: "https://t.me/example".to_string(),
///     title: "Support".to_string(),
///     view_count: 100,
///     click_count: 50,
///     created_date: 1640000000,
/// };
///
/// assert_eq!(link.link, "https://t.me/example");
/// assert_eq!(link.view_count, 100);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BusinessChatLink {
    /// The URL fragment for the link
    pub link: String,
    /// The link title
    pub title: String,
    /// Number of times the link was viewed
    pub view_count: i32,
    /// Number of times the link was clicked
    pub click_count: i32,
    /// When the link was created (Unix timestamp)
    pub created_date: i32,
}

impl fmt::Display for BusinessChatLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessChatLink(link={}, title={}, views={}, clicks={})",
            self.link, self.title, self.view_count, self.click_count
        )
    }
}

/// Information about a business chat link.
///
/// Contains statistics and metadata for a business chat link.
///
/// # Examples
///
/// ```rust
/// use rustgram_business_manager::BusinessChatLinkInfo;
///
/// let info = BusinessChatLinkInfo {
///     link: "https://t.me/example".to_string(),
///     created_date: 1640000000,
///     click_count: 50,
/// };
///
/// assert_eq!(info.link, "https://t.me/example");
/// assert_eq!(info.click_count, 50);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BusinessChatLinkInfo {
    /// The full link URL
    pub link: String,
    /// When the link was created (Unix timestamp)
    pub created_date: i32,
    /// Number of clicks on the link
    pub click_count: i32,
}

impl fmt::Display for BusinessChatLinkInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessChatLinkInfo(link={}, created={}, clicks={})",
            self.link, self.created_date, self.click_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_chat_link_display() {
        let link = BusinessChatLink {
            link: "https://t.me/test".to_string(),
            title: "Test".to_string(),
            view_count: 10,
            click_count: 5,
            created_date: 1640000000,
        };

        let display = format!("{}", link);
        assert!(display.contains("BusinessChatLink"));
        assert!(display.contains("test"));
        assert!(display.contains("10"));
        assert!(display.contains("5"));
    }

    #[test]
    fn test_business_chat_link_info_display() {
        let info = BusinessChatLinkInfo {
            link: "https://t.me/test".to_string(),
            created_date: 1640000000,
            click_count: 42,
        };

        let display = format!("{}", info);
        assert!(display.contains("BusinessChatLinkInfo"));
        assert!(display.contains("test"));
        assert!(display.contains("42"));
    }

    #[test]
    fn test_business_chat_link_clone() {
        let link1 = BusinessChatLink {
            link: "https://t.me/test".to_string(),
            title: "Test".to_string(),
            view_count: 10,
            click_count: 5,
            created_date: 1640000000,
        };

        let link2 = link1.clone();
        assert_eq!(link1, link2);
    }

    #[test]
    fn test_business_chat_link_info_clone() {
        let info1 = BusinessChatLinkInfo {
            link: "https://t.me/test".to_string(),
            created_date: 1640000000,
            click_count: 42,
        };

        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_business_chat_link_equality() {
        let link1 = BusinessChatLink {
            link: "https://t.me/test".to_string(),
            title: "Test".to_string(),
            view_count: 10,
            click_count: 5,
            created_date: 1640000000,
        };

        let link2 = BusinessChatLink {
            link: "https://t.me/test".to_string(),
            title: "Test".to_string(),
            view_count: 10,
            click_count: 5,
            created_date: 1640000000,
        };

        assert_eq!(link1, link2);
    }

    #[test]
    fn test_business_chat_link_inequality() {
        let link1 = BusinessChatLink {
            link: "https://t.me/test1".to_string(),
            title: "Test".to_string(),
            view_count: 10,
            click_count: 5,
            created_date: 1640000000,
        };

        let link2 = BusinessChatLink {
            link: "https://t.me/test2".to_string(),
            title: "Test".to_string(),
            view_count: 10,
            click_count: 5,
            created_date: 1640000000,
        };

        assert_ne!(link1, link2);
    }

    #[test]
    fn test_business_chat_link_info_equality() {
        let info1 = BusinessChatLinkInfo {
            link: "https://t.me/test".to_string(),
            created_date: 1640000000,
            click_count: 42,
        };

        let info2 = BusinessChatLinkInfo {
            link: "https://t.me/test".to_string(),
            created_date: 1640000000,
            click_count: 42,
        };

        assert_eq!(info1, info2);
    }

    #[test]
    fn test_business_chat_link_info_inequality() {
        let info1 = BusinessChatLinkInfo {
            link: "https://t.me/test1".to_string(),
            created_date: 1640000000,
            click_count: 42,
        };

        let info2 = BusinessChatLinkInfo {
            link: "https://t.me/test2".to_string(),
            created_date: 1640000000,
            click_count: 42,
        };

        assert_ne!(info1, info2);
    }
}
