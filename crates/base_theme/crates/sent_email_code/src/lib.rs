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

//! # Sent Email Code
//!
//! Email authentication code types for Telegram client.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Sent email code for authentication.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SentEmailCode {
    /// Email address pattern
    email_pattern: String,
}

impl SentEmailCode {
    /// Creates a new sent email code.
    pub fn new(email_pattern: String) -> Self {
        Self { email_pattern }
    }

    /// Returns the email pattern.
    pub fn email_pattern(&self) -> &str {
        &self.email_pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let code = SentEmailCode::new("test@example.com".to_string());
        assert_eq!(code.email_pattern(), "test@example.com");
    }
}
