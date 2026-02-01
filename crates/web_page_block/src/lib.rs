// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Web Page Block
//!
//! Content blocks for web page instant views.
//!
//! ## Overview
//!
//! Represents different block types for web page content.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_web_page_block::WebPageBlock;
//!
//! let block = WebPageBlock::Title { text: "Hello".to_string() };
//! assert_eq!(block.text(), "Hello");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Web page content block
///
/// Represents a block of content in a web page instant view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebPageBlock {
    /// Title block
    Title {
        /// Text content
        text: String,
    },
    /// Subtitle block
    Subtitle {
        /// Text content
        text: String,
    },
    /// Paragraph block
    Paragraph {
        /// Text content
        text: String,
    },
    /// Preformatted text block
    Preformatted {
        /// Text content
        text: String,
        /// Language
        language: String,
    },
    /// Header block
    Header {
        /// Text content
        text: String,
    },
    /// Divider block
    Divider,
    /// List block
    List {
        /// List items
        items: Vec<String>,
        /// Ordered list
        ordered: bool,
    },
    /// Block quote
    BlockQuote {
        /// Text content
        text: String,
        /// Credit
        credit: Option<String>,
    },
    /// Pull quote
    PullQuote {
        /// Text content
        text: String,
        /// Credit
        credit: Option<String>,
    },
    /// Image block
    Image {
        /// URL
        url: String,
        /// Caption
        caption: Option<String>,
        /// Width
        width: i32,
        /// Height
        height: i32,
    },
    /// Video block
    Video {
        /// URL
        url: String,
        /// Caption
        caption: Option<String>,
        /// Width
        width: i32,
        /// Height
        height: i32,
    },
    /// Cover block
    Cover {
        /// Inner block
        block: Box<WebPageBlock>,
    },
    /// Embedded block
    Embedded {
        /// URL
        url: String,
        /// HTML
        html: String,
        /// Width
        width: i32,
        /// Height
        height: i32,
    },
    /// Related articles block
    RelatedArticles {
        /// Article headers
        headers: Vec<ArticleHeader>,
    },
}

/// Article header for related articles
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleHeader {
    /// Article URL
    pub url: String,
    /// Title
    pub title: String,
    /// Description
    pub description: Option<String>,
    /// Photo URL
    pub photo_url: Option<String>,
    /// Author
    pub author: Option<String>,
    /// Publish date
    pub date: i32,
}

impl WebPageBlock {
    /// Returns the text content of this block
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::Title { text } => text,
            Self::Subtitle { text } => text,
            Self::Paragraph { text } => text,
            Self::Preformatted { text, .. } => text,
            Self::Header { text } => text,
            Self::BlockQuote { text, .. } => text,
            Self::PullQuote { text, .. } => text,
            _ => "",
        }
    }

    /// Returns true if this is a title block
    #[must_use]
    pub const fn is_title(&self) -> bool {
        matches!(self, Self::Title { .. })
    }

    /// Returns true if this is a divider block
    #[must_use]
    pub const fn is_divider(&self) -> bool {
        matches!(self, Self::Divider)
    }
}

impl fmt::Display for WebPageBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title { text } => write!(f, "Title: {}", text),
            Self::Paragraph { text } => write!(f, "Paragraph: {}", text),
            Self::Divider => write!(f, "Divider"),
            _ => write!(f, "WebPageBlock"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        let block = WebPageBlock::Title {
            text: "Hello".to_string(),
        };
        assert_eq!(block.text(), "Hello");
        assert!(block.is_title());
    }

    #[test]
    fn test_paragraph() {
        let block = WebPageBlock::Paragraph {
            text: "Some text".to_string(),
        };
        assert_eq!(block.text(), "Some text");
        assert!(!block.is_title());
    }

    #[test]
    fn test_divider() {
        let block = WebPageBlock::Divider;
        assert!(block.is_divider());
        assert_eq!(block.text(), "");
    }

    #[test]
    fn test_preformatted() {
        let block = WebPageBlock::Preformatted {
            text: "code".to_string(),
            language: "rust".to_string(),
        };
        assert_eq!(block.text(), "code");
    }

    #[test]
    fn test_block_quote() {
        let block = WebPageBlock::BlockQuote {
            text: "Quote".to_string(),
            credit: None,
        };
        assert_eq!(block.text(), "Quote");
    }

    #[test]
    fn test_list() {
        let block = WebPageBlock::List {
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
            ordered: true,
        };
        assert_eq!(block.text(), "");
        assert!(!block.is_title());
    }

    #[test]
    fn test_display() {
        let block = WebPageBlock::Title {
            text: "Test".to_string(),
        };
        assert_eq!(format!("{}", block), "Title: Test");
    }
}
