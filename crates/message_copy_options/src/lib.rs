// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_formatted_text::FormattedText;
use rustgram_message_input_reply_to::MessageInputReplyTo;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReplyMarkup;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageCopyOptions {
    send_copy: bool,
    replace_caption: bool,
    new_invert_media: bool,
    new_caption: FormattedText,
    input_reply_to: MessageInputReplyTo,
    reply_markup: Option<ReplyMarkup>,
}

impl MessageCopyOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_flags(send_copy: bool, replace_caption: bool) -> Self {
        Self {
            send_copy,
            replace_caption,
            new_invert_media: false,
            new_caption: FormattedText::new(""),
            input_reply_to: MessageInputReplyTo::new(),
            reply_markup: None,
        }
    }

    #[must_use]
    pub const fn send_copy(&self) -> bool {
        self.send_copy
    }

    #[must_use]
    pub const fn replace_caption(&self) -> bool {
        self.replace_caption
    }

    #[must_use]
    pub const fn new_invert_media(&self) -> bool {
        self.new_invert_media
    }
}

impl Default for MessageCopyOptions {
    fn default() -> Self {
        Self {
            send_copy: false,
            replace_caption: false,
            new_invert_media: false,
            new_caption: FormattedText::new(""),
            input_reply_to: MessageInputReplyTo::new(),
            reply_markup: None,
        }
    }
}

impl fmt::Display for MessageCopyOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MessageCopyOptions {{ send_copy: {}, replace_caption: {}, invert_media: {} }}",
            self.send_copy, self.replace_caption, self.new_invert_media
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let options = MessageCopyOptions::new();
        assert!(!options.send_copy());
    }

    #[test]
    fn test_with_flags() {
        let options = MessageCopyOptions::with_flags(true, true);
        assert!(options.send_copy());
        assert!(options.replace_caption());
    }
}
