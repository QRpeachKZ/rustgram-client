// rustgram_user_status
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # User Status
//!
//! Defines the online/offline status of a user in Telegram.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_user_status::UserStatus;
//!
//! let status = UserStatus::Online { expires: 1704110400 };
//! assert!(status.is_online_now_at(1704100000));
//! ```

use std::fmt;

/// Represents a user's online/offline status in Telegram.
///
/// This enum follows the TDLib API schema and MTProto TL schema for user status.
/// It can represent six different states: empty, online, offline, recently, last week,
/// and last month. Some variants include additional data like timestamps or privacy flags.
///
/// # Variants
///
/// - `Empty` - User has never been online or status is unknown
/// - `Online` - User is currently online (includes expiry timestamp)
/// - `Offline` - User is offline (includes last online timestamp)
/// - `Recently` - User was online recently (privacy-protected)
/// - `LastWeek` - User was online within the last week (privacy-protected)
/// - `LastMonth` - User was online within the last month (privacy-protected)
///
/// # Examples
///
/// ```rust
/// use rustgram_user_status::UserStatus;
///
/// let online = UserStatus::Online { expires: 1704110400 };
/// assert!(online.is_online());
/// assert_eq!(online.expires(), Some(1704110400));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum UserStatus {
    /// User has never been online or status is unknown.
    ///
    /// This is the default status when no information is available.
    #[default]
    Empty,
    /// User is currently online.
    ///
    /// # Fields
    ///
    /// * `expires` - Unix timestamp when the online status will expire
    Online {
        /// Unix timestamp when online status will expire
        expires: i32,
    },
    /// User is offline.
    ///
    /// # Fields
    ///
    /// * `was_online` - Unix timestamp when user was last online
    Offline {
        /// Unix timestamp when user was last online
        was_online: i32,
    },
    /// User was online recently.
    ///
    /// The exact status is hidden due to privacy settings.
    ///
    /// # Fields
    ///
    /// * `by_my_privacy_settings` - True if hidden due to current user's privacy settings
    Recently {
        /// True if hidden due to current user's privacy settings
        by_my_privacy_settings: bool,
    },
    /// User was online within the last week.
    ///
    /// The exact status is hidden due to privacy settings.
    ///
    /// # Fields
    ///
    /// * `by_my_privacy_settings` - True if hidden due to current user's privacy settings
    LastWeek {
        /// True if hidden due to current user's privacy settings
        by_my_privacy_settings: bool,
    },
    /// User was online within the last month.
    ///
    /// The exact status is hidden due to privacy settings.
    ///
    /// # Fields
    ///
    /// * `by_my_privacy_settings` - True if hidden due to current user's privacy settings
    LastMonth {
        /// True if hidden due to current user's privacy settings
        by_my_privacy_settings: bool,
    },
}

impl UserStatus {
    /// Maximum value for validation purposes.
    ///
    /// This represents the highest valid discriminant value for the enum.
    pub const MAX_VALUE: i32 = 5;

    /// TL constructor for `userStatusEmpty` (MTProto).
    pub const TL_CONSTRUCTOR_EMPTY: u32 = 0x9d05049;

    /// TL constructor for `userStatusOnline` (MTProto).
    pub const TL_CONSTRUCTOR_ONLINE: u32 = 0xedb93949;

    /// TL constructor for `userStatusOffline` (MTProto).
    pub const TL_CONSTRUCTOR_OFFLINE: u32 = 0x8c703f;

    /// TL constructor for `userStatusRecently` (MTProto).
    pub const TL_CONSTRUCTOR_RECENTLY: u32 = 0x7b197dc8;

    /// TL constructor for `userStatusLastWeek` (MTProto).
    pub const TL_CONSTRUCTOR_LAST_WEEK: u32 = 0x541a1d1a;

    /// TL constructor for `userStatusLastMonth` (MTProto).
    pub const TL_CONSTRUCTOR_LAST_MONTH: u32 = 0x65899777;

    /// Returns `true` if the status is `Empty`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert!(UserStatus::Empty.is_empty());
    /// assert!(!UserStatus::Online { expires: 0 }.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns `true` if the status is `Online`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert!(UserStatus::Online { expires: 100 }.is_online());
    /// assert!(!UserStatus::Empty.is_online());
    /// ```
    #[must_use]
    pub const fn is_online(self) -> bool {
        matches!(self, Self::Online { .. })
    }

    /// Returns `true` if the status is `Offline`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert!(UserStatus::Offline { was_online: 100 }.is_offline());
    /// assert!(!UserStatus::Empty.is_offline());
    /// ```
    #[must_use]
    pub const fn is_offline(self) -> bool {
        matches!(self, Self::Offline { .. })
    }

    /// Returns `true` if the status is `Recently`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert!(UserStatus::Recently { by_my_privacy_settings: false }.is_recently());
    /// assert!(!UserStatus::Empty.is_recently());
    /// ```
    #[must_use]
    pub const fn is_recently(self) -> bool {
        matches!(self, Self::Recently { .. })
    }

    /// Returns `true` if the status is `LastWeek`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert!(UserStatus::LastWeek { by_my_privacy_settings: false }.is_last_week());
    /// assert!(!UserStatus::Empty.is_last_week());
    /// ```
    #[must_use]
    pub const fn is_last_week(self) -> bool {
        matches!(self, Self::LastWeek { .. })
    }

    /// Returns `true` if the status is `LastMonth`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert!(UserStatus::LastMonth { by_my_privacy_settings: false }.is_last_month());
    /// assert!(!UserStatus::Empty.is_last_month());
    /// ```
    #[must_use]
    pub const fn is_last_month(self) -> bool {
        matches!(self, Self::LastMonth { .. })
    }

    /// Returns the expires timestamp if status is `Online`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// let status = UserStatus::Online { expires: 1704110400 };
    /// assert_eq!(status.expires(), Some(1704110400));
    /// assert_eq!(UserStatus::Empty.expires(), None);
    /// ```
    #[must_use]
    pub const fn expires(self) -> Option<i32> {
        match self {
            Self::Online { expires } => Some(expires),
            _ => None,
        }
    }

    /// Returns the was_online timestamp if status is `Offline`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// let status = UserStatus::Offline { was_online: 1704100400 };
    /// assert_eq!(status.was_online(), Some(1704100400));
    /// assert_eq!(UserStatus::Empty.was_online(), None);
    /// ```
    #[must_use]
    pub const fn was_online(self) -> Option<i32> {
        match self {
            Self::Offline { was_online } => Some(was_online),
            _ => None,
        }
    }

    /// Returns the privacy settings flag for privacy-protected statuses.
    ///
    /// Returns `None` for statuses that don't have this flag.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// let status = UserStatus::Recently { by_my_privacy_settings: true };
    /// assert_eq!(status.by_my_privacy_settings(), Some(true));
    /// assert_eq!(UserStatus::Empty.by_my_privacy_settings(), None);
    /// ```
    #[must_use]
    pub const fn by_my_privacy_settings(self) -> Option<bool> {
        match self {
            Self::Recently {
                by_my_privacy_settings,
            }
            | Self::LastWeek {
                by_my_privacy_settings,
            }
            | Self::LastMonth {
                by_my_privacy_settings,
            } => Some(by_my_privacy_settings),
            _ => None,
        }
    }

    /// Returns a human-readable display name for this status.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert_eq!(UserStatus::Empty.display_name(), "Empty");
    /// assert_eq!(UserStatus::Online { expires: 100 }.display_name(), "Online");
    /// assert_eq!(UserStatus::Offline { was_online: 100 }.display_name(), "Offline");
    /// ```
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Online { .. } => "Online",
            Self::Offline { .. } => "Offline",
            Self::Recently { .. } => "Recently",
            Self::LastWeek { .. } => "LastWeek",
            Self::LastMonth { .. } => "LastMonth",
        }
    }

    /// Checks if the user is currently online at a given timestamp.
    ///
    /// This is a deterministic version that takes the current time as a parameter,
    /// making it suitable for testing.
    ///
    /// # Arguments
    ///
    /// * `current_time` - Unix timestamp to check against
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// let status = UserStatus::Online { expires: 1704110400 };
    /// assert!(status.is_online_now_at(1704100000));
    /// assert!(!status.is_online_now_at(1704120000));
    /// ```
    #[must_use]
    pub const fn is_online_now_at(self, current_time: i32) -> bool {
        match self {
            Self::Online { expires } => current_time < expires,
            _ => false,
        }
    }

    /// Checks if an `Online` status has expired at a given timestamp.
    ///
    /// # Arguments
    ///
    /// * `current_time` - Unix timestamp to check against
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// let status = UserStatus::Online { expires: 1704110400 };
    /// assert!(!status.is_expired_at(1704100000));
    /// assert!(status.is_expired_at(1704120000));
    /// ```
    #[must_use]
    pub const fn is_expired_at(self, current_time: i32) -> bool {
        match self {
            Self::Online { expires } => current_time >= expires,
            _ => false,
        }
    }

    /// Returns the time elapsed since the user was last online.
    ///
    /// - For `Online`: returns `Some(0)` (user is currently online)
    /// - For `Offline`: returns `Some(current_time - was_online)`
    /// - For privacy-protected statuses: returns `None`
    /// - For `Empty`: returns `None`
    ///
    /// # Arguments
    ///
    /// * `current_time` - Unix timestamp to calculate elapsed time from
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// let online = UserStatus::Online { expires: 1704110400 };
    /// assert_eq!(online.time_since_last_online_at(1704100000), Some(0));
    ///
    /// let offline = UserStatus::Offline { was_online: 1704100000 };
    /// assert_eq!(offline.time_since_last_online_at(1704100100), Some(100));
    /// ```
    #[must_use]
    pub const fn time_since_last_online_at(self, current_time: i32) -> Option<i32> {
        match self {
            Self::Online { .. } => Some(0),
            Self::Offline { was_online } => {
                if current_time > was_online {
                    Some(current_time - was_online)
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }

    /// Creates a `UserStatus` from a TL constructor number.
    ///
    /// # Arguments
    ///
    /// * `constructor` - TL constructor u32 value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_status::UserStatus;
    ///
    /// assert_eq!(
    ///     UserStatus::from_tl_constructor(UserStatus::TL_CONSTRUCTOR_EMPTY),
    ///     Some(UserStatus::Empty)
    /// );
    /// ```
    #[must_use]
    pub const fn from_tl_constructor(constructor: u32) -> Option<Self> {
        match constructor {
            Self::TL_CONSTRUCTOR_EMPTY => Some(Self::Empty),
            _ => None, // Other constructors require additional data
        }
    }
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Online { expires } => write!(f, "Online(expires={expires})"),
            Self::Offline { was_online } => write!(f, "Offline(was_online={was_online})"),
            Self::Recently {
                by_my_privacy_settings,
            } => write!(f, "Recently(by_me={by_my_privacy_settings})"),
            Self::LastWeek {
                by_my_privacy_settings,
            } => write!(f, "LastWeek(by_me={by_my_privacy_settings})"),
            Self::LastMonth {
                by_my_privacy_settings,
            } => write!(f, "LastMonth(by_me={by_my_privacy_settings})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[test]
    fn test_default() {
        assert_eq!(UserStatus::default(), UserStatus::Empty);
    }

    #[test]
    fn test_copy() {
        let status = UserStatus::Online { expires: 100 };
        let copied = status;
        assert_eq!(status, copied);
    }

    #[test]
    fn test_clone() {
        let status = UserStatus::Online { expires: 100 };
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(UserStatus::Empty, UserStatus::Empty);
        assert_eq!(
            UserStatus::Online { expires: 100 },
            UserStatus::Online { expires: 100 }
        );
        assert_ne!(
            UserStatus::Online { expires: 100 },
            UserStatus::Online { expires: 200 }
        );
        assert_ne!(UserStatus::Empty, UserStatus::Online { expires: 0 });
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(UserStatus::Empty);
        set.insert(UserStatus::Online { expires: 100 });
        set.insert(UserStatus::Offline { was_online: 100 });
        set.insert(UserStatus::Recently {
            by_my_privacy_settings: true,
        });
        set.insert(UserStatus::LastWeek {
            by_my_privacy_settings: false,
        });
        set.insert(UserStatus::LastMonth {
            by_my_privacy_settings: false,
        });
        assert_eq!(set.len(), 6);
    }

    #[test]
    fn test_send_sync() {
        // This test compiles if UserStatus is Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<UserStatus>();
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", UserStatus::Empty), "Empty");
        assert_eq!(
            format!("{}", UserStatus::Online { expires: 123 }),
            "Online(expires=123)"
        );
        assert_eq!(
            format!("{}", UserStatus::Offline { was_online: 456 }),
            "Offline(was_online=456)"
        );
        assert_eq!(
            format!(
                "{}",
                UserStatus::Recently {
                    by_my_privacy_settings: true
                }
            ),
            "Recently(by_me=true)"
        );
    }

    // ============================================================================
    // Variant Creation Tests
    // ============================================================================

    #[test]
    fn test_empty() {
        let status = UserStatus::Empty;
        assert!(status.is_empty());
        assert!(!status.is_online());
        assert!(!status.is_offline());
    }

    #[test]
    fn test_online() {
        let status = UserStatus::Online {
            expires: 1704110400,
        };
        assert!(status.is_online());
        assert!(!status.is_empty());
        assert_eq!(status.expires(), Some(1704110400));
    }

    #[test]
    fn test_offline() {
        let status = UserStatus::Offline {
            was_online: 1704100400,
        };
        assert!(status.is_offline());
        assert!(!status.is_empty());
        assert_eq!(status.was_online(), Some(1704100400));
    }

    #[test]
    fn test_recently() {
        let status = UserStatus::Recently {
            by_my_privacy_settings: true,
        };
        assert!(status.is_recently());
        assert!(!status.is_empty());
        assert_eq!(status.by_my_privacy_settings(), Some(true));
    }

    #[test]
    fn test_last_week() {
        let status = UserStatus::LastWeek {
            by_my_privacy_settings: false,
        };
        assert!(status.is_last_week());
        assert!(!status.is_empty());
        assert_eq!(status.by_my_privacy_settings(), Some(false));
    }

    #[test]
    fn test_last_month() {
        let status = UserStatus::LastMonth {
            by_my_privacy_settings: false,
        };
        assert!(status.is_last_month());
        assert!(!status.is_empty());
        assert_eq!(status.by_my_privacy_settings(), Some(false));
    }

    // ============================================================================
    // Variant Check Tests
    // ============================================================================

    #[test]
    fn test_is_empty() {
        assert!(UserStatus::Empty.is_empty());
        assert!(!UserStatus::Online { expires: 0 }.is_empty());
        assert!(!UserStatus::Offline { was_online: 0 }.is_empty());
        assert!(!UserStatus::Recently {
            by_my_privacy_settings: false
        }
        .is_empty());
        assert!(!UserStatus::LastWeek {
            by_my_privacy_settings: false
        }
        .is_empty());
        assert!(!UserStatus::LastMonth {
            by_my_privacy_settings: false
        }
        .is_empty());
    }

    #[test]
    fn test_is_online() {
        assert!(UserStatus::Online { expires: 100 }.is_online());
        assert!(!UserStatus::Empty.is_online());
        assert!(!UserStatus::Offline { was_online: 0 }.is_online());
        assert!(!UserStatus::Recently {
            by_my_privacy_settings: false
        }
        .is_online());
    }

    #[test]
    fn test_is_offline() {
        assert!(UserStatus::Offline { was_online: 100 }.is_offline());
        assert!(!UserStatus::Empty.is_offline());
        assert!(!UserStatus::Online { expires: 0 }.is_offline());
        assert!(!UserStatus::Recently {
            by_my_privacy_settings: false
        }
        .is_offline());
    }

    #[test]
    fn test_is_recently() {
        assert!(UserStatus::Recently {
            by_my_privacy_settings: true
        }
        .is_recently());
        assert!(UserStatus::Recently {
            by_my_privacy_settings: false
        }
        .is_recently());
        assert!(!UserStatus::Empty.is_recently());
        assert!(!UserStatus::LastWeek {
            by_my_privacy_settings: false
        }
        .is_recently());
    }

    #[test]
    fn test_is_last_week() {
        assert!(UserStatus::LastWeek {
            by_my_privacy_settings: true
        }
        .is_last_week());
        assert!(UserStatus::LastWeek {
            by_my_privacy_settings: false
        }
        .is_last_week());
        assert!(!UserStatus::Empty.is_last_week());
        assert!(!UserStatus::Recently {
            by_my_privacy_settings: false
        }
        .is_last_week());
    }

    #[test]
    fn test_is_last_month() {
        assert!(UserStatus::LastMonth {
            by_my_privacy_settings: true
        }
        .is_last_month());
        assert!(UserStatus::LastMonth {
            by_my_privacy_settings: false
        }
        .is_last_month());
        assert!(!UserStatus::Empty.is_last_month());
        assert!(!UserStatus::LastWeek {
            by_my_privacy_settings: false
        }
        .is_last_month());
    }

    // ============================================================================
    // Field Accessor Tests
    // ============================================================================

    #[test]
    fn test_online_expires() {
        let status = UserStatus::Online { expires: 12345 };
        assert_eq!(status.expires(), Some(12345));

        let status = UserStatus::Online { expires: -1 };
        assert_eq!(status.expires(), Some(-1));

        let status = UserStatus::Online { expires: i32::MAX };
        assert_eq!(status.expires(), Some(i32::MAX));
    }

    #[test]
    fn test_offline_was_online() {
        let status = UserStatus::Offline { was_online: 12345 };
        assert_eq!(status.was_online(), Some(12345));

        let status = UserStatus::Offline { was_online: 0 };
        assert_eq!(status.was_online(), Some(0));

        let status = UserStatus::Offline {
            was_online: i32::MIN,
        };
        assert_eq!(status.was_online(), Some(i32::MIN));
    }

    #[test]
    fn test_recently_by_my_privacy_settings() {
        let status = UserStatus::Recently {
            by_my_privacy_settings: true,
        };
        assert_eq!(status.by_my_privacy_settings(), Some(true));

        let status = UserStatus::Recently {
            by_my_privacy_settings: false,
        };
        assert_eq!(status.by_my_privacy_settings(), Some(false));
    }

    #[test]
    fn test_last_week_by_my_privacy_settings() {
        let status = UserStatus::LastWeek {
            by_my_privacy_settings: true,
        };
        assert_eq!(status.by_my_privacy_settings(), Some(true));

        let status = UserStatus::LastWeek {
            by_my_privacy_settings: false,
        };
        assert_eq!(status.by_my_privacy_settings(), Some(false));
    }

    #[test]
    fn test_last_month_by_my_privacy_settings() {
        let status = UserStatus::LastMonth {
            by_my_privacy_settings: true,
        };
        assert_eq!(status.by_my_privacy_settings(), Some(true));

        let status = UserStatus::LastMonth {
            by_my_privacy_settings: false,
        };
        assert_eq!(status.by_my_privacy_settings(), Some(false));
    }

    #[test]
    fn test_field_accessors_for_non_applicable_statuses() {
        assert_eq!(UserStatus::Empty.expires(), None);
        assert_eq!(UserStatus::Empty.was_online(), None);
        assert_eq!(UserStatus::Empty.by_my_privacy_settings(), None);

        assert_eq!(UserStatus::Online { expires: 100 }.was_online(), None);
        assert_eq!(
            UserStatus::Online { expires: 100 }.by_my_privacy_settings(),
            None
        );

        assert_eq!(UserStatus::Offline { was_online: 100 }.expires(), None);
        assert_eq!(
            UserStatus::Offline { was_online: 100 }.by_my_privacy_settings(),
            None
        );
    }

    // ============================================================================
    // Helper Method Tests
    // ============================================================================

    #[test]
    fn test_display_name() {
        assert_eq!(UserStatus::Empty.display_name(), "Empty");
        assert_eq!(UserStatus::Online { expires: 100 }.display_name(), "Online");
        assert_eq!(
            UserStatus::Offline { was_online: 100 }.display_name(),
            "Offline"
        );
        assert_eq!(
            UserStatus::Recently {
                by_my_privacy_settings: true
            }
            .display_name(),
            "Recently"
        );
        assert_eq!(
            UserStatus::LastWeek {
                by_my_privacy_settings: false
            }
            .display_name(),
            "LastWeek"
        );
        assert_eq!(
            UserStatus::LastMonth {
                by_my_privacy_settings: false
            }
            .display_name(),
            "LastMonth"
        );
    }

    #[test]
    fn test_is_online_now_at() {
        let status = UserStatus::Online { expires: 1000 };

        // Before expiry
        assert!(status.is_online_now_at(999));
        assert!(status.is_online_now_at(0));

        // At expiry
        assert!(!status.is_online_now_at(1000));

        // After expiry
        assert!(!status.is_online_now_at(1001));

        // Non-online statuses always return false
        assert!(!UserStatus::Empty.is_online_now_at(0));
        assert!(!UserStatus::Offline { was_online: 0 }.is_online_now_at(0));
        assert!(!UserStatus::Recently {
            by_my_privacy_settings: false
        }
        .is_online_now_at(0));
    }

    #[test]
    fn test_is_expired_at() {
        let status = UserStatus::Online { expires: 1000 };

        // Before expiry - not expired
        assert!(!status.is_expired_at(999));

        // At expiry - expired
        assert!(status.is_expired_at(1000));

        // After expiry - expired
        assert!(status.is_expired_at(1001));

        // Non-online statuses always return false
        assert!(!UserStatus::Empty.is_expired_at(0));
        assert!(!UserStatus::Offline { was_online: 0 }.is_expired_at(0));
    }

    #[test]
    fn test_time_since_last_online_at() {
        // Online status returns 0
        let online = UserStatus::Online { expires: 1000 };
        assert_eq!(online.time_since_last_online_at(500), Some(0));
        assert_eq!(online.time_since_last_online_at(1500), Some(0));

        // Offline status returns elapsed time
        let offline = UserStatus::Offline { was_online: 1000 };
        assert_eq!(offline.time_since_last_online_at(1500), Some(500));
        assert_eq!(offline.time_since_last_online_at(2000), Some(1000));

        // Future was_online (edge case)
        let offline_future = UserStatus::Offline { was_online: 2000 };
        assert_eq!(offline_future.time_since_last_online_at(1000), Some(0));

        // Privacy-protected statuses return None
        assert_eq!(
            UserStatus::Recently {
                by_my_privacy_settings: false
            }
            .time_since_last_online_at(1000),
            None
        );
        assert_eq!(
            UserStatus::LastWeek {
                by_my_privacy_settings: false
            }
            .time_since_last_online_at(1000),
            None
        );
        assert_eq!(
            UserStatus::LastMonth {
                by_my_privacy_settings: false
            }
            .time_since_last_online_at(1000),
            None
        );

        // Empty returns None
        assert_eq!(UserStatus::Empty.time_since_last_online_at(1000), None);
    }

    // ============================================================================
    // TL Constructor Tests
    // ============================================================================

    #[test]
    fn test_from_tl_constructor() {
        assert_eq!(
            UserStatus::from_tl_constructor(UserStatus::TL_CONSTRUCTOR_EMPTY),
            Some(UserStatus::Empty)
        );
        assert_eq!(UserStatus::from_tl_constructor(0xFFFFFFFF), None);
    }

    #[test]
    fn test_tl_constructor_constants() {
        assert_eq!(UserStatus::TL_CONSTRUCTOR_EMPTY, 0x9d05049);
        assert_eq!(UserStatus::TL_CONSTRUCTOR_ONLINE, 0xedb93949);
        assert_eq!(UserStatus::TL_CONSTRUCTOR_OFFLINE, 0x8c703f);
        assert_eq!(UserStatus::TL_CONSTRUCTOR_RECENTLY, 0x7b197dc8);
        assert_eq!(UserStatus::TL_CONSTRUCTOR_LAST_WEEK, 0x541a1d1a);
        assert_eq!(UserStatus::TL_CONSTRUCTOR_LAST_MONTH, 0x65899777);
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn test_online_with_past_expiry() {
        let status = UserStatus::Online { expires: 1000 };
        assert!(!status.is_online_now_at(2000));
        assert!(status.is_expired_at(2000));
    }

    #[test]
    fn test_online_with_future_expiry() {
        let status = UserStatus::Online { expires: 3000 };
        assert!(status.is_online_now_at(2000));
        assert!(!status.is_expired_at(2000));
    }

    #[test]
    fn test_online_with_negative_expiry() {
        let status = UserStatus::Online { expires: -1000 };
        // Negative timestamps represent dates before Unix epoch
        assert!(!status.is_online_now_at(0));
        assert!(status.is_expired_at(0));
    }

    #[test]
    fn test_offline_with_zero_timestamp() {
        let status = UserStatus::Offline { was_online: 0 };
        assert_eq!(status.was_online(), Some(0));
        assert_eq!(status.time_since_last_online_at(1000), Some(1000));
    }

    #[test]
    fn test_offline_with_future_timestamp() {
        let status = UserStatus::Offline { was_online: 2000 };
        assert_eq!(status.was_online(), Some(2000));
        // Future was_online returns 0 elapsed time
        assert_eq!(status.time_since_last_online_at(1000), Some(0));
    }

    #[test]
    fn test_offline_with_negative_timestamp() {
        let status = UserStatus::Offline { was_online: -1000 };
        assert_eq!(status.was_online(), Some(-1000));
        assert_eq!(status.time_since_last_online_at(1000), Some(2000));
    }

    #[test]
    fn test_privacy_flag_combinations() {
        // Test all combinations for Recently
        let recently_true = UserStatus::Recently {
            by_my_privacy_settings: true,
        };
        let recently_false = UserStatus::Recently {
            by_my_privacy_settings: false,
        };
        assert_ne!(recently_true, recently_false);
        assert_eq!(recently_true.by_my_privacy_settings(), Some(true));
        assert_eq!(recently_false.by_my_privacy_settings(), Some(false));

        // Test all combinations for LastWeek
        let last_week_true = UserStatus::LastWeek {
            by_my_privacy_settings: true,
        };
        let last_week_false = UserStatus::LastWeek {
            by_my_privacy_settings: false,
        };
        assert_ne!(last_week_true, last_week_false);

        // Test all combinations for LastMonth
        let last_month_true = UserStatus::LastMonth {
            by_my_privacy_settings: true,
        };
        let last_month_false = UserStatus::LastMonth {
            by_my_privacy_settings: false,
        };
        assert_ne!(last_month_true, last_month_false);
    }

    #[test]
    fn test_boundary_timestamps() {
        // Test with i32::MAX
        let online_max = UserStatus::Online { expires: i32::MAX };
        assert!(online_max.is_online_now_at(i32::MAX - 1));
        assert_eq!(online_max.expires(), Some(i32::MAX));

        let offline_max = UserStatus::Offline {
            was_online: i32::MAX,
        };
        assert_eq!(offline_max.was_online(), Some(i32::MAX));

        // Test with i32::MIN
        let online_min = UserStatus::Online { expires: i32::MIN };
        assert!(!online_min.is_online_now_at(0));

        let offline_min = UserStatus::Offline {
            was_online: i32::MIN,
        };
        assert_eq!(offline_min.was_online(), Some(i32::MIN));
    }

    #[test]
    fn test_max_value_constant() {
        assert_eq!(UserStatus::MAX_VALUE, 5);
    }

    // ============================================================================
    // Serde Tests (feature-gated)
    // ============================================================================

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_empty() {
        let status = UserStatus::Empty;
        let json = serde_json::to_string(&status).expect("serialize failed");
        assert_eq!(json, r#""Empty""#);

        let deserialized: UserStatus = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(deserialized, status);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_online() {
        let status = UserStatus::Online {
            expires: 1704110400,
        };
        let json = serde_json::to_string(&status).expect("serialize failed");
        assert_eq!(json, r#"{"Online":{"expires":1704110400}}"#);

        let deserialized: UserStatus = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(deserialized, status);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_offline() {
        let status = UserStatus::Offline {
            was_online: 1704100400,
        };
        let json = serde_json::to_string(&status).expect("serialize failed");
        assert_eq!(json, r#"{"Offline":{"was_online":1704100400}}"#);

        let deserialized: UserStatus = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(deserialized, status);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_recently() {
        let status = UserStatus::Recently {
            by_my_privacy_settings: true,
        };
        let json = serde_json::to_string(&status).expect("serialize failed");
        assert_eq!(json, r#"{"Recently":{"by_my_privacy_settings":true}}"#);

        let deserialized: UserStatus = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(deserialized, status);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_last_week() {
        let status = UserStatus::LastWeek {
            by_my_privacy_settings: false,
        };
        let json = serde_json::to_string(&status).expect("serialize failed");
        assert_eq!(json, r#"{"LastWeek":{"by_my_privacy_settings":false}}"#);

        let deserialized: UserStatus = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(deserialized, status);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_last_month() {
        let status = UserStatus::LastMonth {
            by_my_privacy_settings: true,
        };
        let json = serde_json::to_string(&status).expect("serialize failed");
        assert_eq!(json, r#"{"LastMonth":{"by_my_privacy_settings":true}}"#);

        let deserialized: UserStatus = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(deserialized, status);
    }
}
