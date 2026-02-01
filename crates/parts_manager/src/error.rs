// Copyright 2025 rustgram-client contributors
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

//! Errors for parts manager operations.

use thiserror::Error;

/// Errors that can occur in parts manager operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// Invalid part size (exceeds maximum)
    #[error("invalid part size: exceeds maximum of {max} bytes", max = 512 * 1024)]
    InvalidPartSize,

    /// File size exceeds maximum allowed
    #[error("file too large: exceeds maximum of {max} bytes", max = 512 * 1024 * 8000i64)]
    FileTooLarge,

    /// Invalid part ID
    #[error("invalid part ID")]
    InvalidPartId,

    /// Invalid size value
    #[error("invalid size")]
    InvalidSize,

    /// Transfer is not ready to finish
    #[error("transfer is not ready: not all parts are complete")]
    NotReady,
}
