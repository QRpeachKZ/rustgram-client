//! Message input reply to - Placeholder module

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageInputReplyTo;

impl MessageInputReplyTo {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MessageInputReplyTo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = MessageInputReplyTo::default();
    }
}
