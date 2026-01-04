// Copyright 2024 rustgram-client Authors
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

//! Error types for theme module.

use thiserror::Error;

/// Errors that can occur when working with themes.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ThemeError {
    /// Unknown TL constructor for BaseTheme
    #[error("unknown BaseTheme TL constructor: 0x{0:08x}")]
    UnknownBaseThemeConstructor(u32),

    /// Invalid BaseTheme value
    #[error("invalid BaseTheme value: {0}")]
    InvalidBaseTheme(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_error_display() {
        let err = ThemeError::UnknownBaseThemeConstructor(0x12345678);
        assert_eq!(
            err.to_string(),
            "unknown BaseTheme TL constructor: 0x12345678"
        );

        let err = ThemeError::InvalidBaseTheme(99);
        assert_eq!(err.to_string(), "invalid BaseTheme value: 99");
    }
}
