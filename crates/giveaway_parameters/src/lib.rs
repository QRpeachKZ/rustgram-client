//! # Giveaway Parameters
//!
//! Parameters for Telegram giveaways.
//!
//! ## Overview
//!
//! `GiveawayParameters` represents the configuration for a Telegram giveaway
//! in a channel, including which channels are involved and eligibility rules.
//!
//! ## Usage
//!
//! ```
//! use rustgram_giveaway_parameters::GiveawayParameters;
//! use rustgram_types::ChannelId;
//!
//! let boosted_channel = ChannelId::new(123);
//! let params = GiveawayParameters::new(
//!     boosted_channel,
//!     vec![],
//!     false,
//!     true,
//!     1234567890,
//!     vec![],
//!     String::new(),
//! );
//! assert!(params.is_valid());
//! ```

use rustgram_types::ChannelId;
use std::vec::Vec;

/// Parameters for a Telegram giveaway.
///
/// This type contains all the configuration for a giveaway in a Telegram channel,
/// including the boosted channel, additional channels, and eligibility rules.
///
/// # Examples
///
/// ```
/// use rustgram_giveaway_parameters::GiveawayParameters;
/// use rustgram_types::ChannelId;
///
/// let boosted = ChannelId::new(123);
/// let additional = vec![ChannelId::new(456), ChannelId::new(789)];
/// let params = GiveawayParameters::new(
///     boosted,
///     additional.clone(),
///     true,  // only new subscribers
///     false, // winners not visible
///     1234567890,
///     vec!["US".to_string()],
///     "Premium subscription".to_string(),
/// );
/// assert!(params.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GiveawayParameters {
    boosted_channel_id: ChannelId,
    additional_channel_ids: Vec<ChannelId>,
    only_new_subscribers: bool,
    winners_are_visible: bool,
    date: i32,
    country_codes: Vec<String>,
    prize_description: String,
}

impl GiveawayParameters {
    /// Creates new giveaway parameters.
    ///
    /// # Arguments
    ///
    /// * `boosted_channel_id` - The channel that was boosted for the giveaway
    /// * `additional_channel_ids` - Additional channels participating in the giveaway
    /// * `only_new_subscribers` - Whether only new subscribers can participate
    /// * `winners_are_visible` - Whether the winners will be visible
    /// * `date` - The date when the giveaway ends
    /// * `country_codes` - Country codes that can participate
    /// * `prize_description` - Description of the prize
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let params = GiveawayParameters::new(
    ///     boosted,
    ///     vec![],
    ///     false,
    ///     true,
    ///     1234567890,
    ///     vec![],
    ///     String::new(),
    /// );
    /// ```
    pub fn new(
        boosted_channel_id: ChannelId,
        additional_channel_ids: Vec<ChannelId>,
        only_new_subscribers: bool,
        winners_are_visible: bool,
        date: i32,
        country_codes: Vec<String>,
        prize_description: String,
    ) -> Self {
        Self {
            boosted_channel_id,
            additional_channel_ids,
            only_new_subscribers,
            winners_are_visible,
            date,
            country_codes,
            prize_description,
        }
    }

    /// Returns the boosted channel ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let params = GiveawayParameters::new(boosted, vec![], false, true, 1234567890, vec![], String::new());
    /// assert_eq!(params.boosted_channel_id(), boosted);
    /// ```
    pub fn boosted_channel_id(&self) -> ChannelId {
        self.boosted_channel_id
    }

    /// Returns the additional channel IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let additional = vec![ChannelId::new(456)];
    /// let params = GiveawayParameters::new(boosted, additional.clone(), false, true, 1234567890, vec![], String::new());
    /// assert_eq!(params.additional_channel_ids(), &additional);
    /// ```
    pub fn additional_channel_ids(&self) -> &[ChannelId] {
        &self.additional_channel_ids
    }

    /// Returns `true` if only new subscribers can participate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let params = GiveawayParameters::new(boosted, vec![], true, true, 1234567890, vec![], String::new());
    /// assert!(params.only_new_subscribers());
    /// ```
    pub fn only_new_subscribers(&self) -> bool {
        self.only_new_subscribers
    }

    /// Returns `true` if the winners will be visible.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let params = GiveawayParameters::new(boosted, vec![], false, true, 1234567890, vec![], String::new());
    /// assert!(params.winners_are_visible());
    /// ```
    pub fn winners_are_visible(&self) -> bool {
        self.winners_are_visible
    }

    /// Returns the date when the giveaway ends.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let params = GiveawayParameters::new(boosted, vec![], false, true, 1234567890, vec![], String::new());
    /// assert_eq!(params.date(), 1234567890);
    /// ```
    pub fn date(&self) -> i32 {
        self.date
    }

    /// Returns the country codes that can participate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let countries = vec!["US".to_string(), "UK".to_string()];
    /// let params = GiveawayParameters::new(boosted, vec![], false, true, 1234567890, countries.clone(), String::new());
    /// assert_eq!(params.country_codes(), &countries);
    /// ```
    pub fn country_codes(&self) -> &[String] {
        &self.country_codes
    }

    /// Returns the prize description.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let prize = "Premium subscription".to_string();
    /// let params = GiveawayParameters::new(boosted, vec![], false, true, 1234567890, vec![], prize.clone());
    /// assert_eq!(params.prize_description(), &prize);
    /// ```
    pub fn prize_description(&self) -> &str {
        &self.prize_description
    }

    /// Returns all channel IDs (boosted + additional).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let additional = vec![ChannelId::new(456), ChannelId::new(789)];
    /// let params = GiveawayParameters::new(boosted, additional.clone(), false, true, 1234567890, vec![], String::new());
    /// let all = params.channel_ids();
    /// assert_eq!(all.len(), 3);
    /// assert!(all.contains(&boosted));
    /// ```
    pub fn channel_ids(&self) -> Vec<ChannelId> {
        let mut result = vec![self.boosted_channel_id];
        result.extend(self.additional_channel_ids.iter());
        result
    }

    /// Returns `true` if these parameters are valid.
    ///
    /// Valid parameters must have:
    /// - A valid boosted channel ID
    /// - A positive date
    /// - All additional channel IDs must be valid
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_giveaway_parameters::GiveawayParameters;
    /// use rustgram_types::ChannelId;
    ///
    /// let boosted = ChannelId::new(123);
    /// let params = GiveawayParameters::new(boosted, vec![], false, true, 1234567890, vec![], String::new());
    /// assert!(params.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        if !self.boosted_channel_id.is_valid() || self.date <= 0 {
            return false;
        }
        self.additional_channel_ids.iter().all(|id| id.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn create_test_params() -> GiveawayParameters {
        let boosted = ChannelId::new(123);
        GiveawayParameters::new(
            boosted,
            vec![ChannelId::new(456), ChannelId::new(789)],
            true,
            false,
            1234567890,
            vec!["US".to_string(), "UK".to_string()],
            "Premium".to_string(),
        )
    }

    #[test]
    fn test_new() {
        let boosted = ChannelId::new(123);
        let params = GiveawayParameters::new(
            boosted,
            vec![],
            false,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        assert_eq!(params.boosted_channel_id(), boosted);
    }

    #[test]
    fn test_boosted_channel_id() {
        let boosted = ChannelId::new(123);
        let params = GiveawayParameters::new(
            boosted,
            vec![],
            false,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        assert_eq!(params.boosted_channel_id(), boosted);
    }

    #[test]
    fn test_additional_channel_ids() {
        let boosted = ChannelId::new(123);
        let additional = vec![ChannelId::new(456), ChannelId::new(789)];
        let params = GiveawayParameters::new(
            boosted,
            additional.clone(),
            false,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        assert_eq!(params.additional_channel_ids(), &additional);
    }

    #[test]
    fn test_only_new_subscribers() {
        let boosted = ChannelId::new(123);
        let params = GiveawayParameters::new(
            boosted,
            vec![],
            true,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        assert!(params.only_new_subscribers());
    }

    #[test]
    fn test_winners_are_visible() {
        let boosted = ChannelId::new(123);
        let params = GiveawayParameters::new(
            boosted,
            vec![],
            false,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        assert!(params.winners_are_visible());
    }

    #[test]
    fn test_date() {
        let boosted = ChannelId::new(123);
        let params = GiveawayParameters::new(
            boosted,
            vec![],
            false,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        assert_eq!(params.date(), 1234567890);
    }

    #[test]
    fn test_country_codes() {
        let boosted = ChannelId::new(123);
        let countries = vec!["US".to_string(), "UK".to_string()];
        let params = GiveawayParameters::new(
            boosted,
            vec![],
            false,
            true,
            1234567890,
            countries.clone(),
            String::new(),
        );
        assert_eq!(params.country_codes(), &countries);
    }

    #[test]
    fn test_prize_description() {
        let boosted = ChannelId::new(123);
        let prize = "Premium".to_string();
        let params = GiveawayParameters::new(
            boosted,
            vec![],
            false,
            true,
            1234567890,
            vec![],
            prize.clone(),
        );
        assert_eq!(params.prize_description(), &prize);
    }

    #[test]
    fn test_channel_ids() {
        let params = create_test_params();
        let all = params.channel_ids();
        assert_eq!(all.len(), 3);
    }

    #[rstest]
    #[case(123, 1234567890, vec![], true)]
    #[case(123, 1234567890, vec![456, 789], true)]
    #[case(0, 1234567890, vec![], false)]
    #[case(123, 0, vec![], false)]
    #[case(123, -1, vec![], false)]
    #[case(123, 1234567890, vec![0], false)]
    fn test_is_valid(
        #[case] boosted_id: i64,
        #[case] date: i32,
        #[case] additional: Vec<i64>,
        #[case] expected: bool,
    ) {
        let boosted = ChannelId::new(boosted_id);
        let additional_ids = additional.into_iter().map(ChannelId::new).collect();
        let params = GiveawayParameters::new(
            boosted,
            additional_ids,
            false,
            true,
            date,
            vec![],
            String::new(),
        );
        assert_eq!(params.is_valid(), expected);
    }

    #[test]
    fn test_equality() {
        let boosted = ChannelId::new(123);
        let additional = vec![ChannelId::new(456)];
        let params1 = GiveawayParameters::new(
            boosted,
            additional.clone(),
            false,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        let params2 = GiveawayParameters::new(
            boosted,
            additional,
            false,
            true,
            1234567890,
            vec![],
            String::new(),
        );
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_clone() {
        let params = create_test_params();
        let cloned = params.clone();
        assert_eq!(params, cloned);
    }

    #[test]
    fn test_debug() {
        let params = create_test_params();
        let debug = format!("{:?}", params);
        assert!(debug.contains("GiveawayParameters"));
    }
}
