//! TL (Type Language) types for user-related Telegram API methods.
//!
//! This module defines the TL types used in user-related API calls:
//! - `InputUser` - Input parameter for user references
//! - `UserFull` - Full user profile information
//! - `UserProfilePhoto` - User profile photo information
//! - `GetFullUserRequest` - Request for users.getFullUser
//! - `GetFullUserResponse` - Response from users.getFullUser
//!
//! These types follow the Telegram TL schema and support serialization/deserialization.

use bytes::BytesMut;
use rustgram_dialog_photo::DialogPhoto;
use rustgram_types::tl::Bytes as TlBytes;
use rustgram_types::{AccessHash, TlDeserialize, TlHelper, TlSerialize, UserId};
use rustgram_user_status::UserStatus;

/// Constructor ID for `inputUserEmpty`.
const INPUT_USER_EMPTY: u32 = 0xb98886cf;

/// Constructor ID for `inputUserSelf`.
const INPUT_USER_SELF: u32 = 0xf7c1b13f;

/// Constructor ID for `inputUser`.
const INPUT_USER: u32 = 0xf21158c6;

/// Constructor ID for `inputUserFromMessage`.
const INPUT_USER_FROM_MESSAGE: u32 = 0x1da448e;

/// Input user identifier.
///
/// Used as input parameter for user-related API calls.
/// Based on TL schema:
/// ```tl
/// inputUserEmpty#b98886cf = InputUser;
/// inputUserSelf#f7c1b13f = InputUser;
/// inputUser#f21158c6 user_id:long access_hash:long = InputUser;
/// inputUserFromMessage#1da448e peer:InputPeer msg_id:int user_id:long = InputUser;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputUser {
    /// Empty user (placeholder).
    Empty,
    /// Current user (self).
    InputUserSelf,
    /// User with ID and access hash.
    User {
        /// User ID.
        user_id: UserId,
        /// Access hash for authentication.
        access_hash: AccessHash,
    },
    /// User referenced in a message.
    FromMessage {
        /// Peer containing the message.
        peer: Box<rustgram_types::InputPeer>,
        /// Message ID.
        msg_id: i32,
        /// User ID.
        user_id: UserId,
    },
}

impl InputUser {
    /// Creates an empty input user.
    #[inline]
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Creates an input user referencing the current user (self).
    #[inline]
    pub fn self_() -> Self {
        Self::InputUserSelf
    }

    /// Creates an input user from a user ID (with zero access hash).
    ///
    /// Note: For API calls that require authentication, you should provide
    /// the access hash. Use `with_access_hash` for that purpose.
    #[inline]
    pub fn user(user_id: UserId) -> Self {
        Self::User {
            user_id,
            access_hash: AccessHash::default(),
        }
    }

    /// Creates an input user with ID and access hash.
    #[inline]
    pub fn with_access_hash(user_id: UserId, access_hash: AccessHash) -> Self {
        Self::User {
            user_id,
            access_hash,
        }
    }

    /// Returns the constructor ID for this input user.
    pub fn constructor_id(&self) -> u32 {
        match self {
            Self::Empty => INPUT_USER_EMPTY,
            Self::InputUserSelf => INPUT_USER_SELF,
            Self::User { .. } => INPUT_USER,
            Self::FromMessage { .. } => INPUT_USER_FROM_MESSAGE,
        }
    }

    /// Serializes this input user to TL format.
    pub fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_i32(buf, self.constructor_id() as i32);

        match self {
            Self::Empty | Self::InputUserSelf => {
                // No additional fields
            }
            Self::User {
                user_id,
                access_hash,
            } => {
                TlHelper::write_i64(buf, user_id.get());
                TlHelper::write_i64(buf, access_hash.get());
            }
            Self::FromMessage {
                peer: _,
                msg_id,
                user_id,
            } => {
                // Serialize peer (simplified - in real implementation would use full serialization)
                // For now, just write msg_id and user_id
                TlHelper::write_i32(buf, *msg_id);
                TlHelper::write_i64(buf, user_id.get());
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for InputUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "InputUserEmpty"),
            Self::InputUserSelf => write!(f, "InputUserSelf"),
            Self::User { user_id, .. } => write!(f, "InputUser({})", user_id.get()),
            Self::FromMessage {
                msg_id, user_id, ..
            } => {
                write!(
                    f,
                    "InputUserFromMessage(msg_id={}, user_id={})",
                    msg_id,
                    user_id.get()
                )
            }
        }
    }
}

/// Full user profile information.
///
/// Contains complete user data including profile photos, bot info, status, etc.
/// This is a simplified version of the TL `users.UserFull` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFull {
    /// Basic user information.
    pub user: Option<super::User>,
    /// Profile photo.
    pub profile_photo: Option<UserProfilePhoto>,
    /// Whether the user can be called.
    pub can_call: bool,
    /// Whether the user has private calls.
    pub has_private_calls: bool,
    /// Whether to block the user.
    pub block: bool,
    /// Whether voice messages are blocked.
    pub voice_messages_blocked: bool,
    /// Phone number.
    pub phone: Option<String>,
    /// Bot information (if this is a bot).
    pub bot_info: Option<String>, // Simplified - real implementation would have proper BotInfo
    /// About/bio text.
    pub about: Option<String>,
    /// Common chat count.
    pub common_chat_count: i32,
    /// User's online status.
    pub status: UserStatus,
}

impl UserFull {
    /// Creates a new empty `UserFull`.
    pub fn new() -> Self {
        Self {
            user: None,
            profile_photo: None,
            can_call: false,
            has_private_calls: false,
            block: false,
            voice_messages_blocked: false,
            phone: None,
            bot_info: None,
            about: None,
            common_chat_count: 0,
            status: UserStatus::Empty,
        }
    }

    /// Returns `true` if this user can be called.
    pub fn can_call(&self) -> bool {
        self.can_call
    }

    /// Returns the user's about/bio text.
    pub fn about(&self) -> Option<&str> {
        self.about.as_deref()
    }

    /// Returns the number of common chats.
    pub fn common_chat_count(&self) -> i32 {
        self.common_chat_count
    }

    /// Returns the user's online status.
    #[must_use]
    pub const fn status(&self) -> UserStatus {
        self.status
    }
}

impl Default for UserFull {
    fn default() -> Self {
        Self::new()
    }
}

/// User profile photo information.
///
/// Contains detailed information about a user's profile photo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfilePhoto {
    /// Photo ID.
    pub photo_id: i64,
    /// Small photo (thumbnail).
    pub small: Option<DialogPhoto>,
    /// Big photo (full size).
    pub big: Option<DialogPhoto>,
    /// Whether this photo has a video.
    pub has_video: bool,
    /// Whether to strip the thumbnail.
    pub stripped_thumb: Option<Vec<u8>>,
}

impl UserProfilePhoto {
    /// Creates a new empty `UserProfilePhoto`.
    pub fn new() -> Self {
        Self {
            photo_id: 0,
            small: None,
            big: None,
            has_video: false,
            stripped_thumb: None,
        }
    }

    /// Returns the photo ID.
    pub fn photo_id(&self) -> i64 {
        self.photo_id
    }

    /// Returns `true` if this photo has an associated video.
    pub fn has_video(&self) -> bool {
        self.has_video
    }
}

impl Default for UserProfilePhoto {
    fn default() -> Self {
        Self::new()
    }
}

/// Request for `users.getFullUser`.
///
/// Fetches full user profile information from Telegram.
/// Based on TL schema:
/// ```tl
/// users.getFullUser#b60f5918 id:InputUser = UserFull;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFullUserRequest {
    /// User to fetch full info for
    pub id: InputUser,
}

impl GetFullUserRequest {
    /// TL constructor ID for `users.getFullUser`.
    pub const CONSTRUCTOR_ID: u32 = 0xb60f5918;

    /// Creates a new get full user request.
    ///
    /// # Arguments
    ///
    /// * `id` - User to fetch full info for
    #[must_use]
    pub fn new(id: InputUser) -> Self {
        Self { id }
    }

    /// Returns the constructor ID for this request.
    #[must_use]
    pub const fn constructor_id(&self) -> u32 {
        Self::CONSTRUCTOR_ID
    }
}

impl TlSerialize for GetFullUserRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, Self::CONSTRUCTOR_ID);

        // Serialize the InputUser field
        self.id.serialize_tl(buf).map_err(|e| {
            rustgram_types::TypeError::SerializationError(format!(
                "failed to serialize InputUser: {e}"
            ))
        })?;

        Ok(())
    }
}

/// Response from `users.getFullUser`.
///
/// Contains full user profile along with related chats and users.
/// Based on TL schema:
/// ```tl
/// users.getFullUser#b60f5918 id:InputUser = UserFull;
/// ```
///
/// Note: This is a simplified version. In production, you would also
/// deserialize the full response including chats and users vectors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFullUserResponse {
    /// Full user profile data
    pub full_user: UserFull,
    /// Related chats (simplified - in production would be Vec<Chat>)
    pub chats: Vec<()>,
    /// Related users (simplified - in production would be Vec<User>)
    pub users: Vec<super::User>,
}

impl GetFullUserResponse {
    /// Creates a new get full user response.
    ///
    /// # Arguments
    ///
    /// * `full_user` - Full user profile data
    #[must_use]
    pub fn new(full_user: UserFull) -> Self {
        Self {
            full_user,
            chats: Vec::new(),
            users: Vec::new(),
        }
    }
}

impl TlDeserialize for GetFullUserResponse {
    fn deserialize_tl(_buf: &mut TlBytes) -> Result<Self, rustgram_types::TypeError> {
        // This is a simplified implementation.
        // In production, you would:
        // 1. Read the UserFull constructor ID
        // 2. Deserialize the full UserFull structure
        // 3. Deserialize the vector of chats
        // 4. Deserialize the vector of users

        // For now, return a placeholder implementation
        // The full implementation would follow the TL schema for UserFull
        let full_user = UserFull::new();

        Ok(Self {
            full_user,
            chats: Vec::new(),
            users: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // InputUser tests
    // =========================================================================

    #[test]
    fn test_input_user_empty() {
        let input = InputUser::empty();
        assert_eq!(input.constructor_id(), INPUT_USER_EMPTY);
        assert!(matches!(input, InputUser::Empty));
    }

    #[test]
    fn test_input_user_self() {
        let input = InputUser::self_();
        assert_eq!(input.constructor_id(), INPUT_USER_SELF);
        assert!(matches!(input, InputUser::InputUserSelf));
    }

    #[test]
    fn test_input_user() {
        let user_id = UserId::from_i32(123);
        let access_hash = AccessHash::new(456);
        let input = InputUser::with_access_hash(user_id, access_hash);
        assert_eq!(input.constructor_id(), INPUT_USER);
    }

    #[test]
    fn test_input_user_with_zero_hash() {
        let user_id = UserId::from_i32(123);
        let input = InputUser::user(user_id);
        assert!(matches!(input, InputUser::User { access_hash, .. } if access_hash.get() == 0));
    }

    #[test]
    fn test_input_user_serialize() {
        let user_id = UserId::from_i32(123);
        let input = InputUser::user(user_id);
        let mut buf = BytesMut::new();

        let result = input.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() >= 4); // At least constructor ID
    }

    #[test]
    fn test_input_user_serialize_empty() {
        let input = InputUser::empty();
        let mut buf = BytesMut::new();

        let result = input.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert_eq!(buf.len(), 4); // Only constructor ID
    }

    #[test]
    fn test_input_user_serialize_self() {
        let input = InputUser::self_();
        let mut buf = BytesMut::new();

        let result = input.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert_eq!(buf.len(), 4); // Only constructor ID
    }

    #[test]
    fn test_input_user_serialize_with_access_hash() {
        let user_id = UserId::from_i32(123);
        let access_hash = AccessHash::new(456);
        let input = InputUser::with_access_hash(user_id, access_hash);
        let mut buf = BytesMut::new();

        let result = input.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() >= 20); // constructor (4) + user_id (8) + access_hash (8)
    }

    #[test]
    fn test_input_user_serialize_from_message() {
        // Note: This is a simplified test since InputPeer is complex
        let user_id = UserId::from_i32(123);
        let input = InputUser::FromMessage {
            peer: Box::new(rustgram_types::InputPeer::Empty),
            msg_id: 456,
            user_id,
        };
        let mut buf = BytesMut::new();

        let result = input.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() >= 4); // At least constructor ID
    }

    #[test]
    fn test_input_user_display() {
        let input = InputUser::self_();
        assert_eq!(format!("{}", input), "InputUserSelf");

        let user_id = UserId::from_i32(123);
        let input = InputUser::user(user_id);
        assert_eq!(format!("{}", input), "InputUser(123)");

        let input = InputUser::empty();
        assert_eq!(format!("{}", input), "InputUserEmpty");
    }

    #[test]
    fn test_input_user_display_from_message() {
        let user_id = UserId::from_i32(123);
        let input = InputUser::FromMessage {
            peer: Box::new(rustgram_types::InputPeer::Empty),
            msg_id: 456,
            user_id,
        };
        let display = format!("{}", input);
        assert!(display.contains("InputUserFromMessage"));
        assert!(display.contains("456"));
        assert!(display.contains("123"));
    }

    #[test]
    fn test_input_user_equality() {
        let user_id = UserId::from_i32(123);
        let access_hash = AccessHash::new(456);

        let input1 = InputUser::with_access_hash(user_id, access_hash);
        let input2 = InputUser::with_access_hash(user_id, access_hash);

        assert_eq!(input1, input2);
    }

    #[test]
    fn test_input_user_clone() {
        let user_id = UserId::from_i32(123);
        let access_hash = AccessHash::new(456);
        let input = InputUser::with_access_hash(user_id, access_hash);
        let cloned = input.clone();

        assert_eq!(input, cloned);
    }

    // =========================================================================
    // UserFull tests
    // =========================================================================

    #[test]
    fn test_user_full_new() {
        let full = UserFull::new();
        assert!(full.user.is_none());
        assert!(!full.can_call());
        assert_eq!(full.common_chat_count(), 0);
    }

    #[test]
    fn test_user_full_default() {
        let full = UserFull::default();
        assert!(full.user.is_none());
        assert!(!full.can_call());
        assert!(!full.has_private_calls);
        assert!(!full.block);
    }

    #[test]
    fn test_user_full_with_user() {
        let mut user = super::super::User::new();
        user.set_id(UserId::from_i32(123));
        user.set_first_name("Alice".to_string());
        user.set_deleted(false);

        let mut full = UserFull::new();
        full.user = Some(user.clone());

        assert!(full.user.is_some());
        assert_eq!(full.user.as_ref().unwrap().first_name(), "Alice");
    }

    #[test]
    fn test_user_full_can_call() {
        let mut full = UserFull::new();
        assert!(!full.can_call());

        full.can_call = true;
        assert!(full.can_call());
    }

    #[test]
    fn test_user_full_about() {
        let mut full = UserFull::new();
        assert!(full.about().is_none());

        full.about = Some("Bio text".to_string());
        assert_eq!(full.about(), Some("Bio text"));
    }

    #[test]
    fn test_user_full_phone() {
        let mut full = UserFull::new();
        assert!(full.phone.is_none());

        full.phone = Some("+1234567890".to_string());
        assert_eq!(full.phone, Some("+1234567890".to_string()));
    }

    #[test]
    fn test_user_full_common_chat_count() {
        let mut full = UserFull::new();
        assert_eq!(full.common_chat_count(), 0);

        full.common_chat_count = 5;
        assert_eq!(full.common_chat_count(), 5);
    }

    #[test]
    fn test_user_full_with_profile_photo() {
        let mut photo = UserProfilePhoto::new();
        photo.photo_id = 123;

        let mut full = UserFull::new();
        full.profile_photo = Some(photo.clone());

        assert!(full.profile_photo.is_some());
        assert_eq!(full.profile_photo.as_ref().unwrap().photo_id(), 123);
    }

    #[test]
    fn test_user_full_clone() {
        let mut full = UserFull::new();
        full.about = Some("Test".to_string());
        full.status = UserStatus::Online { expires: 1000 };

        let cloned = full.clone();
        assert_eq!(full, cloned);
        assert_eq!(cloned.about(), Some("Test"));
        assert_eq!(cloned.status(), UserStatus::Online { expires: 1000 });
    }

    #[test]
    fn test_user_full_equality() {
        let mut full1 = UserFull::new();
        full1.about = Some("Test".to_string());
        full1.status = UserStatus::Offline { was_online: 500 };

        let mut full2 = UserFull::new();
        full2.about = Some("Test".to_string());
        full2.status = UserStatus::Offline { was_online: 500 };

        assert_eq!(full1, full2);
    }

    #[test]
    fn test_user_full_status() {
        let mut full = UserFull::new();
        assert_eq!(full.status(), UserStatus::Empty);

        full.status = UserStatus::Online { expires: 1000 };
        assert_eq!(full.status(), UserStatus::Online { expires: 1000 });
        assert!(full.status().is_online());

        full.status = UserStatus::Offline { was_online: 500 };
        assert_eq!(full.status(), UserStatus::Offline { was_online: 500 });
        assert!(full.status().is_offline());
    }

    #[test]
    fn test_user_full_with_various_statuses() {
        let statuses = vec![
            UserStatus::Empty,
            UserStatus::Online { expires: 1000 },
            UserStatus::Offline { was_online: 500 },
            UserStatus::Recently {
                by_my_privacy_settings: false,
            },
            UserStatus::LastWeek {
                by_my_privacy_settings: true,
            },
            UserStatus::LastMonth {
                by_my_privacy_settings: false,
            },
        ];

        for status in statuses {
            let mut full = UserFull::new();
            full.status = status;
            assert_eq!(full.status(), status);
        }
    }

    // =========================================================================
    // UserProfilePhoto tests
    // =========================================================================

    #[test]
    fn test_user_profile_photo_new() {
        let photo = UserProfilePhoto::new();
        assert_eq!(photo.photo_id(), 0);
        assert!(!photo.has_video());
    }

    #[test]
    fn test_user_profile_photo_default() {
        let photo = UserProfilePhoto::default();
        assert_eq!(photo.photo_id(), 0);
        assert!(photo.small.is_none());
        assert!(photo.big.is_none());
    }

    #[test]
    fn test_user_profile_photo_with_id() {
        let mut photo = UserProfilePhoto::new();
        photo.photo_id = 123;

        assert_eq!(photo.photo_id(), 123);
    }

    #[test]
    fn test_user_profile_photo_has_video() {
        let mut photo = UserProfilePhoto::new();
        assert!(!photo.has_video());

        photo.has_video = true;
        assert!(photo.has_video());
    }

    #[test]
    fn test_user_profile_photo_with_small_photo() {
        let mut photo = UserProfilePhoto::new();
        let small = DialogPhoto::default();
        photo.small = Some(small.clone());

        assert!(photo.small.is_some());
    }

    #[test]
    fn test_user_profile_photo_with_big_photo() {
        let mut photo = UserProfilePhoto::new();
        let big = DialogPhoto::default();
        photo.big = Some(big.clone());

        assert!(photo.big.is_some());
    }

    #[test]
    fn test_user_profile_photo_with_stripped_thumb() {
        let mut photo = UserProfilePhoto::new();
        let thumb = vec![1, 2, 3, 4];
        photo.stripped_thumb = Some(thumb.clone());

        assert!(photo.stripped_thumb.is_some());
        assert_eq!(photo.stripped_thumb.unwrap(), thumb);
    }

    #[test]
    fn test_user_profile_photo_clone() {
        let mut photo = UserProfilePhoto::new();
        photo.photo_id = 123;
        photo.has_video = true;

        let cloned = photo.clone();
        assert_eq!(photo, cloned);
        assert_eq!(cloned.photo_id(), 123);
        assert!(cloned.has_video());
    }

    #[test]
    fn test_user_profile_photo_equality() {
        let mut photo1 = UserProfilePhoto::new();
        photo1.photo_id = 123;

        let mut photo2 = UserProfilePhoto::new();
        photo2.photo_id = 123;

        assert_eq!(photo1, photo2);
    }

    // =========================================================================
    // Edge cases and boundary tests
    // =========================================================================

    #[test]
    fn test_input_user_max_id() {
        let user_id = UserId::from_i32(i32::MAX);
        let access_hash = AccessHash::new(i64::MAX);
        let input = InputUser::with_access_hash(user_id, access_hash);

        let mut buf = BytesMut::new();
        let result = input.serialize_tl(&mut buf);
        assert!(result.is_ok());
    }

    #[test]
    fn test_input_user_min_id() {
        let user_id = UserId::from_i32(i32::MIN);
        let access_hash = AccessHash::new(i64::MIN);
        let input = InputUser::with_access_hash(user_id, access_hash);

        let mut buf = BytesMut::new();
        let result = input.serialize_tl(&mut buf);
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_full_empty_strings() {
        let mut full = UserFull::new();
        full.phone = Some(String::new());
        full.about = Some(String::new());

        assert_eq!(full.phone, Some(String::new()));
        assert_eq!(full.about(), Some(""));
    }

    #[test]
    fn test_user_profile_photo_zero_id() {
        let photo = UserProfilePhoto::new();
        assert_eq!(photo.photo_id(), 0);

        // Zero is a valid photo_id (means no photo)
        let mut photo = UserProfilePhoto::new();
        photo.photo_id = 0;
        assert_eq!(photo.photo_id(), 0);
    }

    #[test]
    fn test_user_full_negative_chat_count() {
        let mut full = UserFull::new();
        full.common_chat_count = -1;

        assert_eq!(full.common_chat_count(), -1);
    }

    // =========================================================================
    // GetFullUserRequest tests
    // =========================================================================

    #[test]
    fn test_get_full_user_request_new() {
        let input = InputUser::self_();
        let request = GetFullUserRequest::new(input.clone());

        assert_eq!(request.id, input);
        assert_eq!(request.constructor_id(), 0xb60f5918);
    }

    #[test]
    fn test_get_full_user_request_constructor_id() {
        assert_eq!(GetFullUserRequest::CONSTRUCTOR_ID, 0xb60f5918);

        let input = InputUser::user(UserId::from_i32(123));
        let request = GetFullUserRequest::new(input);
        assert_eq!(request.constructor_id(), 0xb60f5918);
    }

    #[test]
    fn test_get_full_user_request_clone() {
        let input = InputUser::user(UserId::from_i32(123));
        let request1 = GetFullUserRequest::new(input.clone());
        let request2 = request1.clone();

        assert_eq!(request1, request2);
    }

    #[test]
    fn test_get_full_user_request_equality() {
        let input = InputUser::self_();
        let request1 = GetFullUserRequest::new(input.clone());
        let request2 = GetFullUserRequest::new(input);

        assert_eq!(request1, request2);
    }

    // =========================================================================
    // GetFullUserResponse tests
    // =========================================================================

    #[test]
    fn test_get_full_user_response_new() {
        let full_user = UserFull::new();
        let response = GetFullUserResponse::new(full_user.clone());

        assert_eq!(response.full_user, full_user);
        assert!(response.chats.is_empty());
        assert!(response.users.is_empty());
    }

    #[test]
    fn test_get_full_user_response_deserialize() {
        // Test the simplified deserialization
        let data = vec![0x00, 0x01, 0x02, 0x03];
        let mut buf = TlBytes::from_vec(data);
        let result = GetFullUserResponse::deserialize_tl(&mut buf);

        // Should succeed with placeholder implementation
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.chats.is_empty());
        assert!(response.users.is_empty());
    }

    #[test]
    fn test_get_full_user_response_clone() {
        let mut full_user = UserFull::new();
        full_user.about = Some("Test".to_string());

        let response1 = GetFullUserResponse::new(full_user.clone());
        let response2 = response1.clone();

        assert_eq!(response1, response2);
        assert_eq!(response2.full_user.about(), Some("Test"));
    }

    #[test]
    fn test_get_full_user_response_equality() {
        let full_user = UserFull::new();
        let response1 = GetFullUserResponse::new(full_user.clone());
        let response2 = GetFullUserResponse::new(full_user);

        assert_eq!(response1, response2);
    }

    #[test]
    fn test_get_full_user_constants() {
        assert_eq!(GetFullUserRequest::CONSTRUCTOR_ID, 0xb60f5918);
        // Verified against telegram_api.tl
    }
}
