//! # Premium
//!
//! Telegram Premium subscription functions and types.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/Premium.h`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_premium::PremiumGiftOption;
//!
//! let option = PremiumGiftOption::new(3, "USD", 999);
//! ```

pub use rustgram_premium_gift_option::PremiumGiftOption;

use rustgram_dialog_id::DialogId;
use rustgram_message_full_id::MessageFullId;
use rustgram_user_id::UserId;

/// Check if a payment amount is valid.
///
/// # Arguments
///
/// * `currency` - The currency code (e.g., "USD")
/// * `amount` - The amount in the smallest currency unit
///
/// # Returns
///
/// `Ok` if the amount is valid, `Err` with an error message otherwise.
///
/// # Example
///
/// ```
/// use rustgram_premium::check_payment_amount;
///
/// let mut currency = String::from("USD");
/// assert!(check_payment_amount(&mut currency, 999).is_ok());
/// ```
pub fn check_payment_amount(currency: &mut String, amount: i64) -> Result<(), String> {
    if currency.is_empty() {
        return Err("Currency cannot be empty".to_string());
    }
    if amount <= 0 {
        return Err("Amount must be positive".to_string());
    }
    Ok(())
}

/// Get premium limit keys.
///
/// This returns a list of all available premium limit types.
///
/// # Example
///
/// ```
/// use rustgram_premium::premium_limit_keys;
///
/// let keys = premium_limit_keys();
/// assert!(!keys.is_empty());
/// ```
pub fn premium_limit_keys() -> Vec<String> {
    vec![
        "chat_count".to_string(),
        "chat_folder_count".to_string(),
        "chat_folder_chat_count".to_string(),
        "chat_filter_count".to_string(),
        "chat_filter_chosen_chat_count".to_string(),
        "pinned_chat_count".to_string(),
        "pinned_archived_chat_count".to_string(),
        "saved_gifs_count".to_string(),
        "favorited_sticker_count".to_string(),
        "created_channels_count".to_string(),
        "channels_public_count".to_string(),
    ]
}

/// Premium limit type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PremiumLimitType {
    /// Maximum number of chats
    ChatCount,
    /// Maximum number of chat folders
    ChatFolderCount,
    /// Maximum number of chats per folder
    ChatFolderChatCount,
    /// Maximum number of chat filters
    ChatFilterCount,
    /// Maximum number of pinned chats
    PinnedChatCount,
    /// Maximum number of pinned archived chats
    PinnedArchivedChatCount,
    /// Maximum number of saved GIFs
    SavedGifsCount,
    /// Maximum number of favorited stickers
    FavoritedStickerCount,
    /// Maximum number of created channels
    CreatedChannelsCount,
    /// Maximum number of public channels
    ChannelsPublicCount,
}

/// Premium feature type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PremiumFeature {
    /// Double limits
    DoubleLimits,
    /// More upload speed
    MoreUploadSpeed,
    /// Faster downloads
    FasterDownloads,
    /// Voice to text conversion
    VoiceToText,
    /// No ads
    NoAds,
    /// Exclusive reactions
    ExclusiveReactions,
    /// Premium stickers
    PremiumStickers,
    /// Advanced chat management
    AdvancedChatManagement,
    /// Profile badge
    ProfileBadge,
    /// Animated userpics
    AnimatedUserpics,
}

/// Business feature type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusinessFeature {
    /// Business location
    Location,
    /// Business hours
    OpeningHours,
    /// Quick replies
    QuickReplies,
    /// Greeting messages
    GreetingMessages,
    /// Away messages
    AwayMessages,
    /// Business contacts
    Contacts,
    /// Business links
    BusinessLinks,
    /// Account introduction
    AccountIntroduction,
    /// Chatbot commands
    ChatbotCommands,
    /// Chat links
    ChatLinks,
}

/// Result of getting premium limit.
#[derive(Debug, Clone)]
pub struct PremiumLimit {
    /// The limit type
    pub limit_type: PremiumLimitType,
    /// The default limit for non-premium users
    pub default_limit: i32,
    /// The premium limit
    pub premium_limit: i32,
}

/// Premium features.
#[derive(Debug, Clone)]
pub struct PremiumFeatures {
    /// List of available premium features
    pub features: Vec<PremiumFeature>,
    /// Annual payment available
    pub annual_payment_available: bool,
}

/// Premium state.
#[derive(Debug, Clone)]
pub struct PremiumState {
    /// Whether the user is premium
    pub is_premium: bool,
    /// Payment options
    pub payment_options: Vec<PremiumGiftOption>,
}

/// Premium gift payment options.
#[derive(Debug, Clone)]
pub struct PremiumGiftPaymentOptions {
    /// Payment options
    pub options: Vec<PremiumGiftOption>,
}

/// Premium giveaway options.
#[derive(Debug, Clone)]
pub struct PremiumGiveawayPaymentOptions {
    /// Payment options for giveaways
    pub options: Vec<PremiumGiftOption>,
    /// Boosted dialog ID
    pub boosted_dialog_id: Option<DialogId>,
}

/// Premium gift code info.
#[derive(Debug, Clone)]
pub struct PremiumGiftCodeInfo {
    /// The code
    pub code: String,
    /// Creator user ID
    pub creator_user_id: UserId,
    /// Creation date
    pub creation_date: i32,
    /// Expiration date
    pub expiration_date: i32,
    /// Use count
    pub use_count: i32,
    /// Max use count
    pub max_use_count: i32,
}

/// Giveaway info.
#[derive(Debug, Clone)]
pub struct GiveawayInfo {
    /// Message full ID
    pub message_full_id: MessageFullId,
    /// Giveaway parameters
    pub parameters: GiveawayParameters,
}

/// Giveaway parameters.
#[derive(Debug, Clone)]
pub struct GiveawayParameters {
    /// Quantity of winners
    pub quantity: i32,
    /// Month count
    pub month_count: i32,
    /// Only new subscribers
    pub only_new_subscribers: bool,
}

/// Store payment purpose.
#[derive(Debug, Clone)]
pub enum StorePaymentPurpose {
    /// Premium subscription
    PremiumSubscription,
    /// Gift to user
    GiftToUser(UserId),
    /// Giveaway
    Giveaway(DialogId),
}

/// Store transaction.
#[derive(Debug, Clone)]
pub struct StoreTransaction {
    /// Transaction ID
    pub transaction_id: String,
    /// Receipt
    pub receipt: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic tests (6 tests)
    #[test]
    fn test_check_payment_amount_valid() {
        let mut currency = String::from("USD");
        assert!(check_payment_amount(&mut currency, 999).is_ok());
    }

    #[test]
    fn test_check_payment_amount_empty_currency() {
        let mut currency = String::from("");
        assert!(check_payment_amount(&mut currency, 999).is_err());
    }

    #[test]
    fn test_check_payment_amount_zero_amount() {
        let mut currency = String::from("USD");
        assert!(check_payment_amount(&mut currency, 0).is_err());
    }

    #[test]
    fn test_check_payment_amount_negative_amount() {
        let mut currency = String::from("USD");
        assert!(check_payment_amount(&mut currency, -100).is_err());
    }

    #[test]
    fn test_premium_limit_keys() {
        let keys = premium_limit_keys();
        assert!(!keys.is_empty());
        assert!(keys.contains(&"chat_count".to_string()));
    }

    #[test]
    fn test_premium_limit_type() {
        let limit_type = PremiumLimitType::ChatCount;
        assert_eq!(format!("{:?}", limit_type), "ChatCount");
    }

    // Feature tests (4 tests)
    #[test]
    fn test_premium_feature() {
        let feature = PremiumFeature::DoubleLimits;
        assert_eq!(format!("{:?}", feature), "DoubleLimits");
    }

    #[test]
    fn test_business_feature() {
        let feature = BusinessFeature::Location;
        assert_eq!(format!("{:?}", feature), "Location");
    }

    #[test]
    fn test_premium_limit() {
        let limit = PremiumLimit {
            limit_type: PremiumLimitType::ChatCount,
            default_limit: 500,
            premium_limit: 1000,
        };
        assert_eq!(limit.default_limit, 500);
        assert_eq!(limit.premium_limit, 1000);
    }

    #[test]
    fn test_premium_features() {
        let features = PremiumFeatures {
            features: vec![PremiumFeature::DoubleLimits, PremiumFeature::NoAds],
            annual_payment_available: true,
        };
        assert_eq!(features.features.len(), 2);
    }

    // State tests (4 tests)
    #[test]
    fn test_premium_state() {
        let state = PremiumState {
            is_premium: true,
            payment_options: vec![],
        };
        assert!(state.is_premium);
    }

    #[test]
    fn test_premium_gift_payment_options() {
        let options = PremiumGiftPaymentOptions { options: vec![] };
        assert!(options.options.is_empty());
    }

    #[test]
    fn test_premium_giveaway_payment_options() {
        let options = PremiumGiveawayPaymentOptions {
            options: vec![],
            boosted_dialog_id: None,
        };
        assert!(options.boosted_dialog_id.is_none());
    }

    #[test]
    fn test_premium_gift_code_info() {
        let info = PremiumGiftCodeInfo {
            code: "CODE123".to_string(),
            creator_user_id: UserId::new(123),
            creation_date: 1234567890,
            expiration_date: 1234567890,
            use_count: 0,
            max_use_count: 1,
        };
        assert_eq!(info.code, "CODE123");
    }

    // Giveaway tests (2 tests)
    #[test]
    fn test_giveaway_parameters() {
        let params = GiveawayParameters {
            quantity: 10,
            month_count: 3,
            only_new_subscribers: true,
        };
        assert_eq!(params.quantity, 10);
    }

    #[test]
    fn test_store_payment_purpose() {
        let purpose = StorePaymentPurpose::GiftToUser(UserId::new(123));
        assert!(matches!(purpose, StorePaymentPurpose::GiftToUser(_)));
    }
}
