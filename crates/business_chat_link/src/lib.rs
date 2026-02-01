// Copyright 2024 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Business Chat Link
//!
//! Business chat links for Telegram business accounts.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_business_chat_link::{BusinessChatLink, BusinessChatLinks};
//! use rustgram_formatted_text::FormattedText;
//!
//! let link = BusinessChatLink::new(
//!     "https://t.me/example",
//!     FormattedText::new("Welcome to our business!"),
//!     "Customer Support",
//!     42
//! );
//! assert!(link.is_valid());
//! ```

use rustgram_formatted_text::FormattedText;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Errors that can occur when working with business chat links.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BusinessChatLinkError {
    /// The link URL is empty
    #[error("Link URL cannot be empty")]
    EmptyLink,

    /// The title is too long
    #[error("Title exceeds maximum length of {max} characters (got {len})")]
    TitleTooLong { max: usize, len: usize },
}

/// A business chat link.
///
/// Represents a link that can be used to start a chat with a business account.
///
/// # Example
///
/// ```rust
/// use rustgram_business_chat_link::BusinessChatLink;
/// use rustgram_formatted_text::FormattedText;
///
/// let link = BusinessChatLink::new(
///     "https://t.me/mybusiness",
///     FormattedText::new("How can we help?"),
///     "Support",
///     100
/// );
/// assert_eq!(link.link(), "https://t.me/mybusiness");
/// assert_eq!(link.title(), Some("Support"));
/// assert_eq!(link.view_count(), 100);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BusinessChatLink {
    /// The link URL
    link: String,
    /// The welcome message text
    text: FormattedText,
    /// The link title
    title: Option<String>,
    /// Number of times the link was viewed
    view_count: i32,
}

impl BusinessChatLink {
    /// Maximum title length
    const MAX_TITLE_LENGTH: usize = 32;

    /// Creates a new business chat link.
    ///
    /// # Arguments
    ///
    /// * `link` - The link URL
    /// * `text` - The welcome message text
    /// * `title` - The link title (optional)
    /// * `view_count` - Number of times the link was viewed
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let link = BusinessChatLink::new(
    ///     "https://t.me/example",
    ///     FormattedText::new("Welcome!"),
    ///     "Support",
    ///     42
    /// );
    /// assert!(link.is_valid());
    /// ```
    pub fn new(link: &str, text: FormattedText, title: &str, view_count: i32) -> Self {
        Self {
            link: link.to_string(),
            text,
            title: if title.is_empty() {
                None
            } else {
                Some(title.to_string())
            },
            view_count,
        }
    }

    /// Creates a business chat link from a mock telegram_api::businessChatLink.
    ///
    /// This is a simplified version for testing.
    ///
    /// # Arguments
    ///
    /// * `link` - The link URL
    /// * `text` - The welcome message text
    /// * `title` - The link title
    /// * `view_count` - Number of views
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    ///
    /// let link = BusinessChatLink::from_telegram_api(
    ///     "https://t.me/example",
    ///     "Welcome!",
    ///     "Support",
    ///     42
    /// );
    /// ```
    pub fn from_telegram_api(link: &str, text: &str, title: &str, view_count: i32) -> Self {
        Self {
            link: link.to_string(),
            text: FormattedText::new(text),
            title: if title.is_empty() {
                None
            } else {
                Some(title.to_string())
            },
            view_count,
        }
    }

    /// Returns the link URL.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let link = BusinessChatLink::new(
    ///     "https://t.me/example",
    ///     FormattedText::new("Welcome!"),
    ///     "",
    ///     0
    /// );
    /// assert_eq!(link.link(), "https://t.me/example");
    /// ```
    pub fn link(&self) -> &str {
        &self.link
    }

    /// Returns the welcome message text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Welcome!");
    /// let link = BusinessChatLink::new("https://t.me/e", text, "", 0);
    /// assert_eq!(link.text().text(), "Welcome!");
    /// ```
    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    /// Returns the link title.
    ///
    /// Returns `None` if no title is set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let link = BusinessChatLink::new(
    ///     "https://t.me/example",
    ///     FormattedText::new("Welcome!"),
    ///     "Support",
    ///     0
    /// );
    /// assert_eq!(link.title(), Some("Support"));
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the view count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let link = BusinessChatLink::new(
    ///     "https://t.me/example",
    ///     FormattedText::new("Welcome!"),
    ///     "",
    ///     42
    /// );
    /// assert_eq!(link.view_count(), 42);
    /// ```
    pub fn view_count(&self) -> i32 {
        self.view_count
    }

    /// Checks if the link is valid.
    ///
    /// A link is valid if it has a non-empty URL.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let link = BusinessChatLink::new(
    ///     "https://t.me/example",
    ///     FormattedText::new("Welcome!"),
    ///     "",
    ///     0
    /// );
    /// assert!(link.is_valid());
    ///
    /// let invalid = BusinessChatLink::new(
    ///     "",
    ///     FormattedText::new("Welcome!"),
    ///     "",
    ///     0
    /// );
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.link.is_empty()
    }

    /// Validates the link title length.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the title is valid, or an error if it's too long.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let link = BusinessChatLink::new(
    ///     "https://t.me/example",
    ///     FormattedText::new("Welcome!"),
    ///     "Support",
    ///     0
    /// );
    /// assert!(link.validate_title().is_ok());
    /// ```
    pub fn validate_title(&self) -> Result<(), BusinessChatLinkError> {
        if let Some(title) = &self.title {
            if title.len() > Self::MAX_TITLE_LENGTH {
                return Err(BusinessChatLinkError::TitleTooLong {
                    max: Self::MAX_TITLE_LENGTH,
                    len: title.len(),
                });
            }
        }
        Ok(())
    }

    /// Returns a mock td_api::businessChatLink object.
    ///
    /// This is a placeholder for the real implementation that would
    /// construct the actual TDLib API object.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLink;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let link = BusinessChatLink::new(
    ///     "https://t.me/example",
    ///     FormattedText::new("Welcome!"),
    ///     "Support",
    ///     42
    /// );
    /// let obj = link.get_business_chat_link_object();
    /// assert_eq!(obj.link, "https://t.me/example");
    /// ```
    pub fn get_business_chat_link_object(&self) -> BusinessChatLinkObject {
        BusinessChatLinkObject {
            link: self.link.clone(),
            text: self.text.text().to_string(),
            title: self.title.clone(),
            view_count: self.view_count,
        }
    }
}

/// A mock TDLib API object for business chat link.
///
/// This is a placeholder for the real td_api::businessChatLink.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessChatLinkObject {
    pub link: String,
    pub text: String,
    pub title: Option<String>,
    pub view_count: i32,
}

/// A collection of business chat links.
///
/// # Example
///
/// ```rust
/// use rustgram_business_chat_link::{BusinessChatLink, BusinessChatLinks};
/// use rustgram_formatted_text::FormattedText;
///
/// let links = vec![
///     BusinessChatLink::new(
///         "https://t.me/support",
///         FormattedText::new("Support"),
///         "Support",
///         100
///     ),
///     BusinessChatLink::new(
///         "https://t.me/sales",
///         FormattedText::new("Sales"),
///         "Sales",
///         50
///     ),
/// ];
/// let chat_links = BusinessChatLinks::new(links);
/// assert_eq!(chat_links.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct BusinessChatLinks {
    /// The business chat links
    links: Vec<BusinessChatLink>,
}

impl BusinessChatLinks {
    /// Creates a new collection of business chat links.
    ///
    /// # Arguments
    ///
    /// * `links` - Vector of business chat links
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLinks;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let links = vec![
    ///     BusinessChatLink::new("https://t.me/e", FormattedText::new(""), "", 0),
    /// ];
    /// let chat_links = BusinessChatLinks::new(links);
    /// ```
    pub fn new(links: Vec<BusinessChatLink>) -> Self {
        Self { links }
    }

    /// Creates business chat links from mock telegram_api objects.
    ///
    /// This is a simplified version for testing.
    ///
    /// # Arguments
    ///
    /// * `link_data` - Vector of tuples (link, text, title, view_count)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLinks;
    ///
    /// let data = vec![
    ///     ("https://t.me/support", "Welcome!", "Support", 100),
    /// ];
    /// let links = BusinessChatLinks::from_telegram_api(data);
    /// assert_eq!(links.len(), 1);
    /// ```
    pub fn from_telegram_api(link_data: Vec<(&str, &str, &str, i32)>) -> Self {
        let links = link_data
            .into_iter()
            .map(|(link, text, title, view_count)| {
                BusinessChatLink::from_telegram_api(link, text, title, view_count)
            })
            .collect();

        Self { links }
    }

    /// Returns the business chat links.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLinks;
    ///
    /// let links = BusinessChatLinks::default();
    /// assert!(links.links().is_empty());
    /// ```
    pub fn links(&self) -> &[BusinessChatLink] {
        &self.links
    }

    /// Returns the number of links.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::{BusinessChatLink, BusinessChatLinks};
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let links = vec![
    ///     BusinessChatLink::new("https://t.me/e", FormattedText::new(""), "", 0),
    ///     BusinessChatLink::new("https://t.me/f", FormattedText::new(""), "", 0),
    /// ];
    /// let chat_links = BusinessChatLinks::new(links);
    /// assert_eq!(chat_links.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// Returns `true` if there are no links.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLinks;
    ///
    /// let links = BusinessChatLinks::default();
    /// assert!(links.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    /// Returns a mock td_api::businessChatLinks object.
    ///
    /// This is a placeholder for the real implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_business_chat_link::BusinessChatLinks;
    ///
    /// let links = BusinessChatLinks::default();
    /// let obj = links.get_business_chat_links_object();
    /// assert!(obj.links.is_empty());
    /// ```
    pub fn get_business_chat_links_object(&self) -> BusinessChatLinksObject {
        BusinessChatLinksObject {
            links: self
                .links
                .iter()
                .map(|link| link.get_business_chat_link_object())
                .collect(),
        }
    }
}

/// A mock TDLib API object for business chat links.
///
/// This is a placeholder for the real td_api::businessChatLinks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessChatLinksObject {
    pub links: Vec<BusinessChatLinkObject>,
}

impl fmt::Display for BusinessChatLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BusinessChatLink[")?;
        write!(f, "link={}, ", self.link)?;
        if let Some(title) = &self.title {
            write!(f, "title={}, ", title)?;
        }
        write!(f, "views={}]", self.view_count)
    }
}

impl fmt::Display for BusinessChatLinks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BusinessChatLinks[{} links]", self.links.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_chat_link_new() {
        let text = FormattedText::new("Welcome!");
        let link = BusinessChatLink::new("https://t.me/example", text, "Support", 42);

        assert_eq!(link.link(), "https://t.me/example");
        assert_eq!(link.text().text(), "Welcome!");
        assert_eq!(link.title(), Some("Support"));
        assert_eq!(link.view_count(), 42);
    }

    #[test]
    fn test_business_chat_link_new_empty_title() {
        let text = FormattedText::new("Welcome!");
        let link = BusinessChatLink::new("https://t.me/example", text, "", 0);

        assert_eq!(link.title(), None);
    }

    #[test]
    fn test_business_chat_link_from_telegram_api() {
        let link =
            BusinessChatLink::from_telegram_api("https://t.me/example", "Welcome!", "Support", 42);

        assert_eq!(link.link(), "https://t.me/example");
        assert_eq!(link.text().text(), "Welcome!");
        assert_eq!(link.title(), Some("Support"));
        assert_eq!(link.view_count(), 42);
    }

    #[test]
    fn test_business_chat_link_is_valid() {
        let text = FormattedText::new("Welcome!");
        let link = BusinessChatLink::new("https://t.me/example", text, "", 0);
        assert!(link.is_valid());

        let empty_link = BusinessChatLink::new("", FormattedText::new(""), "", 0);
        assert!(!empty_link.is_valid());
    }

    #[test]
    fn test_business_chat_link_validate_title() {
        let text = FormattedText::new("Welcome!");
        let link = BusinessChatLink::new("https://t.me/example", text, "Support", 0);
        assert!(link.validate_title().is_ok());

        let long_title = "a".repeat(100);
        let link2 = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new(""),
            &long_title,
            0,
        );
        assert!(link2.validate_title().is_err());
    }

    #[test]
    fn test_business_chat_link_validate_title_max_length() {
        let max_title = "a".repeat(32);
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new(""),
            &max_title,
            0,
        );
        assert!(link.validate_title().is_ok());

        let too_long = "a".repeat(33);
        let link2 =
            BusinessChatLink::new("https://t.me/example", FormattedText::new(""), &too_long, 0);
        assert!(link2.validate_title().is_err());
    }

    #[test]
    fn test_business_chat_link_no_title() {
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "",
            0,
        );
        assert!(link.validate_title().is_ok());
        assert_eq!(link.title(), None);
    }

    #[test]
    fn test_business_chat_link_get_object() {
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        );
        let obj = link.get_business_chat_link_object();

        assert_eq!(obj.link, "https://t.me/example");
        assert_eq!(obj.text, "Welcome!");
        assert_eq!(obj.title, Some("Support".to_string()));
        assert_eq!(obj.view_count, 42);
    }

    #[test]
    fn test_business_chat_links_new() {
        let links = vec![
            BusinessChatLink::new("https://t.me/a", FormattedText::new(""), "", 0),
            BusinessChatLink::new("https://t.me/b", FormattedText::new(""), "", 0),
        ];
        let chat_links = BusinessChatLinks::new(links);

        assert_eq!(chat_links.len(), 2);
    }

    #[test]
    fn test_business_chat_links_default() {
        let links = BusinessChatLinks::default();
        assert!(links.is_empty());
        assert_eq!(links.len(), 0);
    }

    #[test]
    fn test_business_chat_links_from_telegram_api() {
        let data = vec![
            ("https://t.me/support", "Welcome!", "Support", 100),
            ("https://t.me/sales", "Hello!", "Sales", 50),
        ];
        let links = BusinessChatLinks::from_telegram_api(data);

        assert_eq!(links.len(), 2);
        assert_eq!(links.links()[0].link(), "https://t.me/support");
        assert_eq!(links.links()[1].link(), "https://t.me/sales");
    }

    #[test]
    fn test_business_chat_links_get_object() {
        let links = vec![BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        )];
        let chat_links = BusinessChatLinks::new(links);
        let obj = chat_links.get_business_chat_links_object();

        assert_eq!(obj.links.len(), 1);
        assert_eq!(obj.links[0].link, "https://t.me/example");
    }

    #[test]
    fn test_business_chat_link_display() {
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        );
        let display = format!("{}", link);
        assert!(display.contains("https://t.me/example"));
        assert!(display.contains("Support"));
        assert!(display.contains("42"));
    }

    #[test]
    fn test_business_chat_link_display_no_title() {
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "",
            42,
        );
        let display = format!("{}", link);
        assert!(display.contains("https://t.me/example"));
        assert!(!display.contains("title="));
    }

    #[test]
    fn test_business_chat_links_display() {
        let links = vec![
            BusinessChatLink::new("https://t.me/a", FormattedText::new(""), "", 0),
            BusinessChatLink::new("https://t.me/b", FormattedText::new(""), "", 0),
        ];
        let chat_links = BusinessChatLinks::new(links);
        let display = format!("{}", chat_links);
        assert!(display.contains("2 links"));
    }

    #[test]
    fn test_equality() {
        let text = FormattedText::new("Welcome!");
        let link1 = BusinessChatLink::new("https://t.me/example", text.clone(), "Support", 42);
        let link2 = BusinessChatLink::new("https://t.me/example", text, "Support", 42);
        assert_eq!(link1, link2);
    }

    #[test]
    fn test_inequality() {
        let link1 = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        );
        let link2 = BusinessChatLink::new(
            "https://t.me/other",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        );
        assert_ne!(link1, link2);
    }

    #[test]
    fn test_clone() {
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        );
        let cloned = link.clone();
        assert_eq!(link, cloned);
    }

    #[test]
    fn test_serialization() {
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        );
        let json = serde_json::to_string(&link).unwrap();
        let parsed: BusinessChatLink = serde_json::from_str(&json).unwrap();
        assert_eq!(link, parsed);
    }

    #[test]
    fn test_serialization_links() {
        let links = vec![BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new("Welcome!"),
            "Support",
            42,
        )];
        let chat_links = BusinessChatLinks::new(links);

        let json = serde_json::to_string(&chat_links).unwrap();
        let parsed: BusinessChatLinks = serde_json::from_str(&json).unwrap();
        assert_eq!(chat_links, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let text = FormattedText::new("Welcome!");
        let link1 = BusinessChatLink::new("https://t.me/example", text.clone(), "Support", 42);
        let link2 = BusinessChatLink::new("https://t.me/example", text, "Support", 42);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        link1.hash(&mut hasher1);
        link2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_title_with_unicode() {
        let unicode_title = "Support 支持"; // Short enough
        let link = BusinessChatLink::new(
            "https://t.me/example",
            FormattedText::new(""),
            unicode_title,
            0,
        );
        assert_eq!(link.title(), Some(unicode_title));
        assert!(link.validate_title().is_ok());
    }

    #[test]
    fn test_view_count_negative() {
        let link = BusinessChatLink::new("https://t.me/example", FormattedText::new(""), "", -1);
        assert_eq!(link.view_count(), -1);
    }

    #[test]
    fn test_view_count_zero() {
        let link = BusinessChatLink::new("https://t.me/example", FormattedText::new(""), "", 0);
        assert_eq!(link.view_count(), 0);
    }

    #[test]
    fn test_empty_links() {
        let links = BusinessChatLinks::new(vec![]);
        assert!(links.is_empty());
        assert_eq!(links.len(), 0);
    }
}
