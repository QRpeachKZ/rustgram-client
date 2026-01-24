//! # Premium Gift Option
//!
//! Options for gifting Telegram Premium subscription.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/PremiumGiftOption.h`
//! - TDLib class: `PremiumGiftOption`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_premium_gift_option::PremiumGiftOption;
//!
//! let option = PremiumGiftOption::new(3, "USD", 999);
//! ```

use core::fmt;

/// Options for gifting Telegram Premium subscription.
///
/// TDLib: `class PremiumGiftOption`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PremiumGiftOption {
    months: i32,
    is_current: bool,
    is_upgrade: bool,
    currency: String,
    amount: i64,
    bot_url: String,
    store_product: String,
    transaction: String,
}

impl PremiumGiftOption {
    /// Create a new PremiumGiftOption with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `months` - Number of months of Premium subscription
    /// * `currency` - Currency code (e.g., "USD")
    /// * `amount` - Amount in the smallest currency unit (cents)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_premium_gift_option::PremiumGiftOption;
    ///
    /// let option = PremiumGiftOption::new(3, "USD", 999);
    /// assert_eq!(option.months(), 3);
    /// ```
    pub fn new(months: i32, currency: impl Into<String>, amount: i64) -> Self {
        Self {
            months,
            is_current: false,
            is_upgrade: false,
            currency: currency.into(),
            amount,
            bot_url: String::new(),
            store_product: String::new(),
            transaction: String::new(),
        }
    }

    /// Get the number of months.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_premium_gift_option::PremiumGiftOption;
    ///
    /// let option = PremiumGiftOption::new(3, "USD", 999);
    /// assert_eq!(option.months(), 3);
    /// ```
    pub fn months(&self) -> i32 {
        self.months
    }

    /// Check if this is the current subscription option.
    pub fn is_current(&self) -> bool {
        self.is_current
    }

    /// Check if this is an upgrade option.
    pub fn is_upgrade(&self) -> bool {
        self.is_upgrade
    }

    /// Get the currency code.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_premium_gift_option::PremiumGiftOption;
    ///
    /// let option = PremiumGiftOption::new(3, "USD", 999);
    /// assert_eq!(option.currency(), "USD");
    /// ```
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the amount in the smallest currency unit.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_premium_gift_option::PremiumGiftOption;
    ///
    /// let option = PremiumGiftOption::new(3, "USD", 999);
    /// assert_eq!(option.amount(), 999);
    /// ```
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// Get the bot URL for payment.
    pub fn bot_url(&self) -> &str {
        &self.bot_url
    }

    /// Get the store product identifier.
    pub fn store_product(&self) -> &str {
        &self.store_product
    }

    /// Get the transaction identifier.
    pub fn transaction(&self) -> &str {
        &self.transaction
    }

    /// Calculate the monthly price.
    ///
    /// Returns `None` if months is zero.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_premium_gift_option::PremiumGiftOption;
    ///
    /// let option = PremiumGiftOption::new(3, "USD", 999);
    /// // 999 cents / 3 months = 333 cents per month
    /// assert_eq!(option.monthly_price(), Some(333));
    /// ```
    pub fn monthly_price(&self) -> Option<i64> {
        if self.months > 0 {
            Some(self.amount / self.months as i64)
        } else {
            None
        }
    }

    /// Set whether this is the current option.
    pub fn set_is_current(&mut self, value: bool) {
        self.is_current = value;
    }

    /// Set whether this is an upgrade option.
    pub fn set_is_upgrade(&mut self, value: bool) {
        self.is_upgrade = value;
    }

    /// Set the bot URL.
    pub fn set_bot_url(&mut self, url: impl Into<String>) {
        self.bot_url = url.into();
    }

    /// Set the store product identifier.
    pub fn set_store_product(&mut self, product: impl Into<String>) {
        self.store_product = product.into();
    }

    /// Set the transaction identifier.
    pub fn set_transaction(&mut self, transaction: impl Into<String>) {
        self.transaction = transaction.into();
    }

    /// Check if this option is valid.
    ///
    /// An option is valid if it has at least one month and a positive amount.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_premium_gift_option::PremiumGiftOption;
    ///
    /// let option = PremiumGiftOption::new(3, "USD", 999);
    /// assert!(option.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.months > 0 && self.amount > 0 && !self.currency.is_empty()
    }
}

impl fmt::Display for PremiumGiftOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} months for {}.{:02} {}",
            self.months,
            self.amount / 100,
            self.amount % 100,
            self.currency
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (8 tests)
    #[test]
    fn test_clone() {
        let a = PremiumGiftOption::new(3, "USD", 999);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_partial_eq() {
        let a = PremiumGiftOption::new(3, "USD", 999);
        let b = PremiumGiftOption::new(3, "USD", 999);
        assert_eq!(a, b);

        let c = PremiumGiftOption::new(6, "USD", 1999);
        assert_ne!(a, c);
    }

    #[test]
    fn test_debug() {
        let option = PremiumGiftOption::new(3, "USD", 999);
        let debug_str = format!("{:?}", option);
        assert!(debug_str.contains("PremiumGiftOption"));
        assert!(debug_str.contains("months: 3"));
    }

    // Constructor tests (6 tests)
    #[test]
    fn test_new() {
        let option = PremiumGiftOption::new(3, "USD", 999);
        assert_eq!(option.months(), 3);
        assert_eq!(option.currency(), "USD");
        assert_eq!(option.amount(), 999);
    }

    #[test]
    fn test_new_defaults() {
        let option = PremiumGiftOption::new(3, "USD", 999);
        assert!(!option.is_current());
        assert!(!option.is_upgrade());
        assert_eq!(option.bot_url(), "");
        assert_eq!(option.store_product(), "");
        assert_eq!(option.transaction(), "");
    }

    #[test]
    fn test_new_with_string() {
        let option = PremiumGiftOption::new(3, String::from("USD"), 999);
        assert_eq!(option.currency(), "USD");
    }

    // Getter tests (10 tests)
    #[test]
    fn test_months() {
        assert_eq!(PremiumGiftOption::new(3, "USD", 999).months(), 3);
        assert_eq!(PremiumGiftOption::new(6, "USD", 1999).months(), 6);
        assert_eq!(PremiumGiftOption::new(12, "USD", 3999).months(), 12);
    }

    #[test]
    fn test_is_current() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        assert!(!option.is_current());
        option.set_is_current(true);
        assert!(option.is_current());
    }

    #[test]
    fn test_is_upgrade() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        assert!(!option.is_upgrade());
        option.set_is_upgrade(true);
        assert!(option.is_upgrade());
    }

    #[test]
    fn test_currency() {
        assert_eq!(PremiumGiftOption::new(3, "USD", 999).currency(), "USD");
        assert_eq!(PremiumGiftOption::new(3, "EUR", 899).currency(), "EUR");
        assert_eq!(PremiumGiftOption::new(3, "RUB", 29999).currency(), "RUB");
    }

    #[test]
    fn test_amount() {
        assert_eq!(PremiumGiftOption::new(3, "USD", 999).amount(), 999);
        assert_eq!(PremiumGiftOption::new(3, "USD", 1999).amount(), 1999);
    }

    #[test]
    fn test_bot_url() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        assert_eq!(option.bot_url(), "");
        option.set_bot_url("https://t.me/premiumbot");
        assert_eq!(option.bot_url(), "https://t.me/premiumbot");
    }

    #[test]
    fn test_store_product() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        assert_eq!(option.store_product(), "");
        option.set_store_product("com.telegram.premium.3months");
        assert_eq!(option.store_product(), "com.telegram.premium.3months");
    }

    #[test]
    fn test_transaction() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        assert_eq!(option.transaction(), "");
        option.set_transaction("txn_123");
        assert_eq!(option.transaction(), "txn_123");
    }

    // Method tests (8 tests)
    #[test]
    fn test_monthly_price() {
        assert_eq!(
            PremiumGiftOption::new(3, "USD", 999).monthly_price(),
            Some(333)
        );
        assert_eq!(
            PremiumGiftOption::new(6, "USD", 1999).monthly_price(),
            Some(333)
        );
        assert_eq!(
            PremiumGiftOption::new(12, "USD", 3999).monthly_price(),
            Some(333)
        );
    }

    #[test]
    fn test_monthly_price_zero_months() {
        assert_eq!(PremiumGiftOption::new(0, "USD", 999).monthly_price(), None);
    }

    #[test]
    fn test_monthly_price_rounding() {
        assert_eq!(
            PremiumGiftOption::new(3, "USD", 1000).monthly_price(),
            Some(333)
        );
        assert_eq!(
            PremiumGiftOption::new(7, "USD", 1000).monthly_price(),
            Some(142)
        );
    }

    #[test]
    fn test_is_valid_true() {
        assert!(PremiumGiftOption::new(3, "USD", 999).is_valid());
        assert!(PremiumGiftOption::new(1, "USD", 1).is_valid());
        assert!(PremiumGiftOption::new(12, "EUR", 9999).is_valid());
    }

    #[test]
    fn test_is_valid_zero_months() {
        assert!(!PremiumGiftOption::new(0, "USD", 999).is_valid());
    }

    #[test]
    fn test_is_valid_zero_amount() {
        assert!(!PremiumGiftOption::new(3, "USD", 0).is_valid());
    }

    #[test]
    fn test_is_valid_empty_currency() {
        assert!(!PremiumGiftOption::new(3, "", 999).is_valid());
    }

    // Setter tests (6 tests)
    #[test]
    fn test_set_is_current() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        option.set_is_current(true);
        assert!(option.is_current());
        option.set_is_current(false);
        assert!(!option.is_current());
    }

    #[test]
    fn test_set_is_upgrade() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        option.set_is_upgrade(true);
        assert!(option.is_upgrade());
        option.set_is_upgrade(false);
        assert!(!option.is_upgrade());
    }

    #[test]
    fn test_set_bot_url() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        option.set_bot_url(String::from("https://t.me/bot"));
        assert_eq!(option.bot_url(), "https://t.me/bot");
    }

    #[test]
    fn test_set_store_product() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        option.set_store_product(String::from("product.id"));
        assert_eq!(option.store_product(), "product.id");
    }

    #[test]
    fn test_set_transaction() {
        let mut option = PremiumGiftOption::new(3, "USD", 999);
        option.set_transaction(String::from("txn"));
        assert_eq!(option.transaction(), "txn");
    }

    // Display tests (3 tests)
    #[test]
    fn test_display() {
        let option = PremiumGiftOption::new(3, "USD", 999);
        assert_eq!(format!("{}", option), "3 months for $9.99 USD");
    }

    #[test]
    fn test_display_large() {
        let option = PremiumGiftOption::new(12, "USD", 5999);
        assert_eq!(format!("{}", option), "12 months for $59.99 USD");
    }

    #[test]
    fn test_display_rounding() {
        let option = PremiumGiftOption::new(3, "USD", 1000);
        assert_eq!(format!("{}", option), "3 months for $10.00 USD");
    }
}
