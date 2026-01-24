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

//! # Star Gift Auction User State
//!
//! User state for star gift auctions.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

/// User state for star gift auction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarGiftAuctionUserState {
    /// User ID
    user_id: i64,
}

impl StarGiftAuctionUserState {
    /// Creates a new auction user state.
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }

    /// Returns the user ID.
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let state = StarGiftAuctionUserState::new(123);
        assert_eq!(state.user_id(), 123);
    }
}
