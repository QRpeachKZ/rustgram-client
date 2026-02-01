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

//! Statistics data types.

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Full identifier for a message.
///
/// Combines a dialog ID and message ID into a single compound type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageFullId {
    /// Dialog ID.
    pub dialog_id: DialogId,
    /// Message ID.
    pub message_id: MessageId,
}

impl MessageFullId {
    /// Creates a new message full ID.
    #[must_use]
    pub const fn new(dialog_id: DialogId, message_id: MessageId) -> Self {
        Self {
            dialog_id,
            message_id,
        }
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the message ID.
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }
}

impl fmt::Display for MessageFullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} in {}", self.message_id, self.dialog_id)
    }
}

/// Date range for statistics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange {
    /// Minimum date in the range (Unix timestamp).
    pub min_date: i32,
    /// Maximum date in the range (Unix timestamp).
    pub max_date: i32,
}

impl DateRange {
    /// Creates a new date range.
    ///
    /// # Arguments
    ///
    /// * `min_date` - Minimum date in the range (Unix timestamp)
    /// * `max_date` - Maximum date in the range (Unix timestamp)
    #[must_use]
    pub const fn new(min_date: i32, max_date: i32) -> Self {
        Self { min_date, max_date }
    }

    /// Returns the duration of the date range in days.
    #[must_use]
    pub fn duration_days(&self) -> i32 {
        self.max_date.saturating_sub(self.min_date)
    }

    /// Checks if the date range is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.min_date > 0 && self.max_date >= self.min_date
    }
}

/// Statistical value with current, previous, and percentage change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatisticalValue {
    /// Current value.
    pub current: f64,
    /// Previous value.
    pub previous: f64,
    /// Percentage change from previous to current.
    pub percentage: f64,
}

impl StatisticalValue {
    /// Creates a new statistical value.
    ///
    /// # Arguments
    ///
    /// * `current` - Current value
    /// * `previous` - Previous value
    /// * `percentage` - Percentage change
    #[must_use]
    pub const fn new(current: f64, previous: f64, percentage: f64) -> Self {
        Self {
            current,
            previous,
            percentage,
        }
    }

    /// Returns the absolute difference between current and previous.
    #[must_use]
    pub fn difference(&self) -> f64 {
        self.current - self.previous
    }

    /// Checks if there was growth from previous to current.
    #[must_use]
    pub fn is_growth(&self) -> bool {
        self.current > self.previous
    }

    /// Checks if the value is valid (non-negative).
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.current >= 0.0 && self.previous >= 0.0
    }
}

/// Statistical graph data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatisticalGraph {
    /// Async graph that needs to be loaded with a token.
    Async {
        /// Token for loading the graph.
        token: String,
    },
    /// Error loading graph data.
    Error {
        /// Error message.
        error: String,
    },
    /// Complete graph data.
    Data {
        /// JSON data for the graph.
        json_data: String,
        /// Token for zooming the graph.
        zoom_token: String,
    },
}

impl StatisticalGraph {
    /// Creates a new async graph.
    ///
    /// # Arguments
    ///
    /// * `token` - Token for loading the graph
    #[must_use]
    pub fn async_graph(token: String) -> Self {
        Self::Async { token }
    }

    /// Creates a new error graph.
    ///
    /// # Arguments
    ///
    /// * `error` - Error message
    #[must_use]
    pub fn error(error: String) -> Self {
        Self::Error { error }
    }

    /// Creates a new complete graph data.
    ///
    /// # Arguments
    ///
    /// * `json_data` - JSON data for the graph
    /// * `zoom_token` - Token for zooming
    #[must_use]
    pub fn data(json_data: String, zoom_token: String) -> Self {
        Self::Data {
            json_data,
            zoom_token,
        }
    }

    /// Returns the token if this is an async graph.
    #[must_use]
    pub fn get_token(&self) -> Option<&str> {
        match self {
            Self::Async { token } => Some(token),
            _ => None,
        }
    }

    /// Returns the error message if this is an error graph.
    #[must_use]
    pub fn get_error(&self) -> Option<&str> {
        match self {
            Self::Error { error } => Some(error),
            _ => None,
        }
    }

    /// Returns the graph data if available.
    #[must_use]
    pub fn get_data(&self) -> Option<(&str, &str)> {
        match self {
            Self::Data {
                json_data,
                zoom_token,
            } => Some((json_data, zoom_token)),
            _ => None,
        }
    }

    /// Checks if the graph is loaded (has data).
    #[must_use]
    pub fn is_loaded(&self) -> bool {
        matches!(self, Self::Data { .. })
    }

    /// Checks if the graph has an error.
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }
}

/// Message sender statistics info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageSenderInfo {
    /// User ID of the sender.
    pub user_id: i64,
    /// Number of messages sent.
    pub message_count: i32,
    /// Average characters per message.
    pub avg_chars: f64,
}

impl MessageSenderInfo {
    /// Creates a new message sender info.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `message_count` - Number of messages
    /// * `avg_chars` - Average characters per message
    #[must_use]
    pub const fn new(user_id: i64, message_count: i32, avg_chars: f64) -> Self {
        Self {
            user_id,
            message_count,
            avg_chars,
        }
    }

    /// Checks if the info is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.user_id > 0 && self.message_count >= 0 && self.avg_chars >= 0.0
    }
}

/// Administrator actions statistics info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdministratorActionsInfo {
    /// User ID of the administrator.
    pub user_id: i64,
    /// Number of deleted messages.
    pub deleted_count: i32,
    /// Number of kicked users.
    pub kicked_count: i32,
    /// Number of banned users.
    pub banned_count: i32,
}

impl AdministratorActionsInfo {
    /// Creates a new administrator actions info.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `deleted_count` - Number of deleted messages
    /// * `kicked_count` - Number of kicked users
    /// * `banned_count` - Number of banned users
    #[must_use]
    pub const fn new(
        user_id: i64,
        deleted_count: i32,
        kicked_count: i32,
        banned_count: i32,
    ) -> Self {
        Self {
            user_id,
            deleted_count,
            kicked_count,
            banned_count,
        }
    }

    /// Returns the total number of actions.
    #[must_use]
    pub const fn total_actions(&self) -> i32 {
        self.deleted_count + self.kicked_count + self.banned_count
    }

    /// Checks if the info is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.user_id > 0
            && self.deleted_count >= 0
            && self.kicked_count >= 0
            && self.banned_count >= 0
    }
}

/// Inviter statistics info.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InviterInfo {
    /// User ID of the inviter.
    pub user_id: i64,
    /// Number of users invited.
    pub invitation_count: i32,
}

impl InviterInfo {
    /// Creates a new inviter info.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `invitation_count` - Number of invitations
    #[must_use]
    pub const fn new(user_id: i64, invitation_count: i32) -> Self {
        Self {
            user_id,
            invitation_count,
        }
    }

    /// Checks if the info is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.user_id > 0 && self.invitation_count >= 0
    }
}

/// Chat statistics for a supergroup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStatisticsSupergroup {
    /// Period for the statistics.
    pub period: DateRange,
    /// Member count statistics.
    pub members: StatisticalValue,
    /// Message count statistics.
    pub messages: StatisticalValue,
    /// Viewer count statistics.
    pub viewers: StatisticalValue,
    /// Poster count statistics.
    pub posters: StatisticalValue,
    /// Growth graph.
    pub growth_graph: StatisticalGraph,
    /// Members graph.
    pub members_graph: StatisticalGraph,
    /// New members by source graph.
    pub new_members_by_source_graph: StatisticalGraph,
    /// Languages graph.
    pub languages_graph: StatisticalGraph,
    /// Messages graph.
    pub messages_graph: StatisticalGraph,
    /// Actions graph.
    pub actions_graph: StatisticalGraph,
    /// Top hours graph.
    pub top_hours_graph: StatisticalGraph,
    /// Weekdays graph.
    pub weekdays_graph: StatisticalGraph,
    /// Top senders.
    pub top_senders: Vec<MessageSenderInfo>,
    /// Top administrators.
    pub top_administrators: Vec<AdministratorActionsInfo>,
    /// Top inviters.
    pub top_inviters: Vec<InviterInfo>,
}

impl ChatStatisticsSupergroup {
    /// Creates a new chat statistics for supergroup.
    ///
    /// # Arguments
    ///
    /// * `period` - Statistics period
    /// * `members` - Member statistics
    /// * `messages` - Message statistics
    /// * `viewers` - Viewer statistics
    /// * `posters` - Poster statistics
    #[must_use]
    pub fn new(
        period: DateRange,
        members: StatisticalValue,
        messages: StatisticalValue,
        viewers: StatisticalValue,
        posters: StatisticalValue,
    ) -> Self {
        Self {
            period,
            members,
            messages,
            viewers,
            posters,
            growth_graph: StatisticalGraph::error("Not loaded".to_string()),
            members_graph: StatisticalGraph::error("Not loaded".to_string()),
            new_members_by_source_graph: StatisticalGraph::error("Not loaded".to_string()),
            languages_graph: StatisticalGraph::error("Not loaded".to_string()),
            messages_graph: StatisticalGraph::error("Not loaded".to_string()),
            actions_graph: StatisticalGraph::error("Not loaded".to_string()),
            top_hours_graph: StatisticalGraph::error("Not loaded".to_string()),
            weekdays_graph: StatisticalGraph::error("Not loaded".to_string()),
            top_senders: Vec::new(),
            top_administrators: Vec::new(),
            top_inviters: Vec::new(),
        }
    }

    /// Returns the number of top senders.
    #[must_use]
    pub fn top_senders_count(&self) -> usize {
        self.top_senders.len()
    }

    /// Returns the number of top administrators.
    #[must_use]
    pub fn top_administrators_count(&self) -> usize {
        self.top_administrators.len()
    }

    /// Returns the number of top inviters.
    #[must_use]
    pub fn top_inviters_count(&self) -> usize {
        self.top_inviters.len()
    }
}

/// Chat interaction info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatInteractionInfo {
    /// Message ID.
    pub message_id: i32,
    /// View count.
    pub view_count: i32,
    /// Forward count.
    pub forward_count: i32,
    /// Reaction count.
    pub reaction_count: i32,
}

impl ChatInteractionInfo {
    /// Creates a new chat interaction info.
    ///
    /// # Arguments
    ///
    /// * `message_id` - Message ID
    /// * `view_count` - Number of views
    /// * `forward_count` - Number of forwards
    /// * `reaction_count` - Number of reactions
    #[must_use]
    pub const fn new(
        message_id: i32,
        view_count: i32,
        forward_count: i32,
        reaction_count: i32,
    ) -> Self {
        Self {
            message_id,
            view_count,
            forward_count,
            reaction_count,
        }
    }

    /// Returns the total interaction count.
    #[must_use]
    pub const fn total_interactions(&self) -> i32 {
        self.view_count + self.forward_count + self.reaction_count
    }

    /// Checks if the info is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.message_id > 0
            && self.view_count >= 0
            && self.forward_count >= 0
            && self.reaction_count >= 0
    }
}

/// Chat statistics for a channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStatisticsChannel {
    /// Period for the statistics.
    pub period: DateRange,
    /// Follower statistics.
    pub followers: StatisticalValue,
    /// Views per post statistics.
    pub views_per_post: StatisticalValue,
    /// Shares per post statistics.
    pub shares_per_post: StatisticalValue,
    /// Reactions per post statistics.
    pub reactions_per_post: StatisticalValue,
    /// Views per story statistics.
    pub views_per_story: StatisticalValue,
    /// Shares per story statistics.
    pub shares_per_story: StatisticalValue,
    /// Reactions per story statistics.
    pub reactions_per_story: StatisticalValue,
    /// Percentage of users with enabled notifications.
    pub enabled_notifications_percentage: f64,
    /// Growth graph.
    pub growth_graph: StatisticalGraph,
    /// Followers graph.
    pub followers_graph: StatisticalGraph,
    /// Mute graph.
    pub mute_graph: StatisticalGraph,
    /// Top hours graph.
    pub top_hours_graph: StatisticalGraph,
    /// Views by source graph.
    pub views_by_source_graph: StatisticalGraph,
    /// New followers by source graph.
    pub new_followers_by_source_graph: StatisticalGraph,
    /// Languages graph.
    pub languages_graph: StatisticalGraph,
    /// Interactions graph.
    pub interactions_graph: StatisticalGraph,
    /// Reactions by emotion graph.
    pub reactions_by_emotion_graph: StatisticalGraph,
    /// Story interactions graph.
    pub story_interactions_graph: StatisticalGraph,
    /// Story reactions by emotion graph.
    pub story_reactions_by_emotion_graph: StatisticalGraph,
    /// IV interactions graph.
    pub iv_interactions_graph: StatisticalGraph,
    /// Recent message interactions.
    pub recent_interactions: Vec<ChatInteractionInfo>,
}

impl ChatStatisticsChannel {
    /// Creates a new chat statistics for channel.
    ///
    /// # Arguments
    ///
    /// * `period` - Statistics period
    /// * `followers` - Follower statistics
    /// * `views_per_post` - Views per post statistics
    /// * `shares_per_post` - Shares per post statistics
    /// * `reactions_per_post` - Reactions per post statistics
    #[must_use]
    pub fn new(
        period: DateRange,
        followers: StatisticalValue,
        views_per_post: StatisticalValue,
        shares_per_post: StatisticalValue,
        reactions_per_post: StatisticalValue,
    ) -> Self {
        Self {
            period,
            followers,
            views_per_post,
            shares_per_post,
            reactions_per_post,
            views_per_story: StatisticalValue::new(0.0, 0.0, 0.0),
            shares_per_story: StatisticalValue::new(0.0, 0.0, 0.0),
            reactions_per_story: StatisticalValue::new(0.0, 0.0, 0.0),
            enabled_notifications_percentage: 0.0,
            growth_graph: StatisticalGraph::error("Not loaded".to_string()),
            followers_graph: StatisticalGraph::error("Not loaded".to_string()),
            mute_graph: StatisticalGraph::error("Not loaded".to_string()),
            top_hours_graph: StatisticalGraph::error("Not loaded".to_string()),
            views_by_source_graph: StatisticalGraph::error("Not loaded".to_string()),
            new_followers_by_source_graph: StatisticalGraph::error("Not loaded".to_string()),
            languages_graph: StatisticalGraph::error("Not loaded".to_string()),
            interactions_graph: StatisticalGraph::error("Not loaded".to_string()),
            reactions_by_emotion_graph: StatisticalGraph::error("Not loaded".to_string()),
            story_interactions_graph: StatisticalGraph::error("Not loaded".to_string()),
            story_reactions_by_emotion_graph: StatisticalGraph::error("Not loaded".to_string()),
            iv_interactions_graph: StatisticalGraph::error("Not loaded".to_string()),
            recent_interactions: Vec::new(),
        }
    }

    /// Returns the number of recent interactions.
    #[must_use]
    pub fn recent_interactions_count(&self) -> usize {
        self.recent_interactions.len()
    }
}

/// Chat statistics (either supergroup or channel).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum ChatStatistics {
    /// Supergroup statistics.
    Supergroup(ChatStatisticsSupergroup),
    /// Channel statistics.
    Channel(ChatStatisticsChannel),
}

impl ChatStatistics {
    /// Returns the period for the statistics.
    #[must_use]
    pub fn period(&self) -> &DateRange {
        match self {
            Self::Supergroup(stats) => &stats.period,
            Self::Channel(stats) => &stats.period,
        }
    }

    /// Checks if this is supergroup statistics.
    #[must_use]
    pub const fn is_supergroup(&self) -> bool {
        matches!(self, Self::Supergroup(_))
    }

    /// Checks if this is channel statistics.
    #[must_use]
    pub const fn is_channel(&self) -> bool {
        matches!(self, Self::Channel(_))
    }

    /// Returns the supergroup statistics if available.
    #[must_use]
    pub fn as_supergroup(&self) -> Option<&ChatStatisticsSupergroup> {
        match self {
            Self::Supergroup(stats) => Some(stats),
            _ => None,
        }
    }

    /// Returns the channel statistics if available.
    #[must_use]
    pub fn as_channel(&self) -> Option<&ChatStatisticsChannel> {
        match self {
            Self::Channel(stats) => Some(stats),
            _ => None,
        }
    }
}

/// Message statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageStatistics {
    /// Views graph.
    pub views_graph: StatisticalGraph,
    /// Reactions by emotion graph.
    pub reactions_by_emotion_graph: StatisticalGraph,
}

impl MessageStatistics {
    /// Creates a new message statistics.
    ///
    /// # Arguments
    ///
    /// * `views_graph` - Views graph data
    /// * `reactions_by_emotion_graph` - Reactions graph data
    #[must_use]
    pub fn new(
        views_graph: StatisticalGraph,
        reactions_by_emotion_graph: StatisticalGraph,
    ) -> Self {
        Self {
            views_graph,
            reactions_by_emotion_graph,
        }
    }

    /// Checks if the views graph is loaded.
    #[must_use]
    pub fn is_views_loaded(&self) -> bool {
        self.views_graph.is_loaded()
    }

    /// Checks if the reactions graph is loaded.
    #[must_use]
    pub fn is_reactions_loaded(&self) -> bool {
        self.reactions_by_emotion_graph.is_loaded()
    }
}

/// Story statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryStatistics {
    /// Views graph.
    pub views_graph: StatisticalGraph,
    /// Reactions by emotion graph.
    pub reactions_by_emotion_graph: StatisticalGraph,
}

impl StoryStatistics {
    /// Creates a new story statistics.
    ///
    /// # Arguments
    ///
    /// * `views_graph` - Views graph data
    /// * `reactions_by_emotion_graph` - Reactions graph data
    #[must_use]
    pub fn new(
        views_graph: StatisticalGraph,
        reactions_by_emotion_graph: StatisticalGraph,
    ) -> Self {
        Self {
            views_graph,
            reactions_by_emotion_graph,
        }
    }

    /// Checks if the views graph is loaded.
    #[must_use]
    pub fn is_views_loaded(&self) -> bool {
        self.views_graph.is_loaded()
    }

    /// Checks if the reactions graph is loaded.
    #[must_use]
    pub fn is_reactions_loaded(&self) -> bool {
        self.reactions_by_emotion_graph.is_loaded()
    }
}

/// Revenue withdrawal state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevenueWithdrawalState {
    /// Withdrawal is pending.
    Pending,
    /// Withdrawal succeeded.
    Succeeded {
        /// Withdrawal date.
        date: i32,
        /// Transaction URL.
        url: String,
    },
    /// Withdrawal failed.
    Failed,
}

impl RevenueWithdrawalState {
    /// Creates a pending withdrawal state.
    #[must_use]
    pub fn pending() -> Self {
        Self::Pending
    }

    /// Creates a succeeded withdrawal state.
    ///
    /// # Arguments
    ///
    /// * `date` - Withdrawal date
    /// * `url` - Transaction URL
    #[must_use]
    pub fn succeeded(date: i32, url: String) -> Self {
        Self::Succeeded { date, url }
    }

    /// Creates a failed withdrawal state.
    #[must_use]
    pub fn failed() -> Self {
        Self::Failed
    }

    /// Checks if the withdrawal is pending.
    #[must_use]
    pub const fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Checks if the withdrawal succeeded.
    #[must_use]
    pub const fn is_succeeded(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Checks if the withdrawal failed.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// Revenue transaction type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevenueTransactionType {
    /// Unsupported transaction type.
    Unsupported,
    /// Fragment refund.
    FragmentRefund {
        /// Refund date.
        date: i32,
    },
    /// Fragment withdrawal.
    FragmentWithdrawal {
        /// Withdrawal date.
        date: i32,
        /// Withdrawal state.
        state: RevenueWithdrawalState,
    },
    /// Sponsored message earnings.
    SponsoredMessageEarnings {
        /// Start date.
        from_date: i32,
        /// End date.
        to_date: i32,
    },
    /// Suggested post earnings.
    SuggestedPostEarnings {
        /// User ID.
        user_id: i64,
    },
}

impl RevenueTransactionType {
    /// Creates an unsupported transaction type.
    #[must_use]
    pub fn unsupported() -> Self {
        Self::Unsupported
    }

    /// Checks if this transaction is earnings.
    #[must_use]
    pub const fn is_earnings(&self) -> bool {
        matches!(
            self,
            Self::SponsoredMessageEarnings { .. } | Self::SuggestedPostEarnings { .. }
        )
    }

    /// Creates a fragment refund.
    ///
    /// # Arguments
    ///
    /// * `date` - Refund date
    #[must_use]
    pub fn fragment_refund(date: i32) -> Self {
        Self::FragmentRefund { date }
    }

    /// Creates a fragment withdrawal.
    ///
    /// # Arguments
    ///
    /// * `date` - Withdrawal date
    /// * `state` - Withdrawal state
    #[must_use]
    pub fn fragment_withdrawal(date: i32, state: RevenueWithdrawalState) -> Self {
        Self::FragmentWithdrawal { date, state }
    }

    /// Creates sponsored message earnings.
    ///
    /// # Arguments
    ///
    /// * `from_date` - Start date
    /// * `to_date` - End date
    #[must_use]
    pub fn sponsored_message_earnings(from_date: i32, to_date: i32) -> Self {
        Self::SponsoredMessageEarnings { from_date, to_date }
    }

    /// Creates suggested post earnings.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    #[must_use]
    pub fn suggested_post_earnings(user_id: i64) -> Self {
        Self::SuggestedPostEarnings { user_id }
    }
}

/// Revenue transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevenueTransaction {
    /// Transaction type.
    pub transaction_type: RevenueTransactionType,
    /// Amount in TON.
    pub amount: f64,
}

impl RevenueTransaction {
    /// Creates a new revenue transaction.
    ///
    /// # Arguments
    ///
    /// * `transaction_type` - Type of transaction
    /// * `amount` - Amount in TON
    #[must_use]
    pub fn new(transaction_type: RevenueTransactionType, amount: f64) -> Self {
        Self {
            transaction_type,
            amount,
        }
    }

    /// Checks if the transaction is a withdrawal.
    #[must_use]
    pub fn is_withdrawal(&self) -> bool {
        matches!(
            self.transaction_type,
            RevenueTransactionType::FragmentWithdrawal { .. }
        )
    }

    /// Checks if the transaction is earnings.
    #[must_use]
    pub fn is_earnings(&self) -> bool {
        matches!(
            self.transaction_type,
            RevenueTransactionType::SponsoredMessageEarnings { .. }
                | RevenueTransactionType::SuggestedPostEarnings { .. }
        )
    }
}

/// Revenue transactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevenueTransactions {
    /// Total balance in TON.
    pub balance: f64,
    /// List of transactions.
    pub transactions: Vec<RevenueTransaction>,
    /// Offset for pagination.
    pub next_offset: String,
}

impl RevenueTransactions {
    /// Creates a new revenue transactions.
    ///
    /// # Arguments
    ///
    /// * `balance` - Total balance
    /// * `transactions` - Transaction list
    /// * `next_offset` - Next offset for pagination
    #[must_use]
    pub fn new(balance: f64, transactions: Vec<RevenueTransaction>, next_offset: String) -> Self {
        Self {
            balance,
            transactions,
            next_offset,
        }
    }

    /// Returns the number of transactions.
    #[must_use]
    pub fn count(&self) -> usize {
        self.transactions.len()
    }

    /// Checks if there are more transactions to fetch.
    #[must_use]
    pub fn has_more(&self) -> bool {
        !self.next_offset.is_empty()
    }
}

/// Revenue statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevenueStatistics {
    /// Top hours graph.
    pub top_hours_graph: StatisticalGraph,
    /// Revenue graph.
    pub revenue_graph: StatisticalGraph,
    /// Overall revenue amount.
    pub overall_revenue: f64,
    /// Current balance amount.
    pub current_balance: f64,
    /// Available balance amount.
    pub available_balance: f64,
    /// Whether withdrawal is enabled.
    pub withdrawal_enabled: bool,
    /// USD exchange rate.
    pub usd_rate: f64,
}

impl RevenueStatistics {
    /// Creates a new revenue statistics.
    ///
    /// # Arguments
    ///
    /// * `overall_revenue` - Overall revenue
    /// * `current_balance` - Current balance
    /// * `available_balance` - Available balance
    /// * `withdrawal_enabled` - Whether withdrawal is enabled
    /// * `usd_rate` - USD exchange rate
    #[must_use]
    pub fn new(
        overall_revenue: f64,
        current_balance: f64,
        available_balance: f64,
        withdrawal_enabled: bool,
        usd_rate: f64,
    ) -> Self {
        Self {
            top_hours_graph: StatisticalGraph::error("Not loaded".to_string()),
            revenue_graph: StatisticalGraph::error("Not loaded".to_string()),
            overall_revenue,
            current_balance,
            available_balance,
            withdrawal_enabled,
            usd_rate,
        }
    }

    /// Returns the pending balance (current - available).
    #[must_use]
    pub fn pending_balance(&self) -> f64 {
        self.current_balance - self.available_balance
    }

    /// Checks if withdrawal is available.
    #[must_use]
    pub fn can_withdraw(&self) -> bool {
        self.withdrawal_enabled && self.available_balance > 0.0
    }
}

/// Public forward.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum PublicForward {
    /// Message forward.
    Message {
        /// Dialog ID.
        dialog_id: i64,
        /// Message ID.
        message_id: i32,
        /// Message date.
        date: i32,
    },
    /// Story forward.
    Story {
        /// Dialog ID.
        dialog_id: i64,
        /// Story ID.
        story_id: i32,
        /// Story date.
        date: i32,
    },
}

impl PublicForward {
    /// Creates a new message forward.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `message_id` - Message ID
    /// * `date` - Message date
    #[must_use]
    pub const fn message(dialog_id: i64, message_id: i32, date: i32) -> Self {
        Self::Message {
            dialog_id,
            message_id,
            date,
        }
    }

    /// Creates a new story forward.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `story_id` - Story ID
    /// * `date` - Story date
    #[must_use]
    pub const fn story(dialog_id: i64, story_id: i32, date: i32) -> Self {
        Self::Story {
            dialog_id,
            story_id,
            date,
        }
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> i64 {
        match self {
            Self::Message { dialog_id, .. } => *dialog_id,
            Self::Story { dialog_id, .. } => *dialog_id,
        }
    }

    /// Returns the date.
    #[must_use]
    pub const fn date(&self) -> i32 {
        match self {
            Self::Message { date, .. } => *date,
            Self::Story { date, .. } => *date,
        }
    }

    /// Checks if this is a message forward.
    #[must_use]
    pub const fn is_message(&self) -> bool {
        matches!(self, Self::Message { .. })
    }

    /// Checks if this is a story forward.
    #[must_use]
    pub const fn is_story(&self) -> bool {
        matches!(self, Self::Story { .. })
    }
}

/// Public forwards.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicForwards {
    /// Total count of forwards.
    pub total_count: i32,
    /// List of forwards.
    pub forwards: Vec<PublicForward>,
    /// Offset for pagination.
    pub next_offset: String,
}

impl PublicForwards {
    /// Creates a new public forwards.
    ///
    /// # Arguments
    ///
    /// * `total_count` - Total count
    /// * `forwards` - List of forwards
    /// * `next_offset` - Next offset for pagination
    #[must_use]
    pub fn new(total_count: i32, forwards: Vec<PublicForward>, next_offset: String) -> Self {
        Self {
            total_count,
            forwards,
            next_offset,
        }
    }

    /// Returns the number of forwards.
    #[must_use]
    pub fn count(&self) -> usize {
        self.forwards.len()
    }

    /// Checks if there are more forwards to fetch.
    #[must_use]
    pub fn has_more(&self) -> bool {
        !self.next_offset.is_empty()
    }
}

/// Story full ID for statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StoryFullId {
    /// Dialog ID where the story was posted.
    pub dialog_id: i64,
    /// Story ID.
    pub story_id: i32,
}

impl StoryFullId {
    /// Creates a new story full ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `story_id` - Story ID
    #[must_use]
    pub const fn new(dialog_id: i64, story_id: i32) -> Self {
        Self {
            dialog_id,
            story_id,
        }
    }

    /// Checks if the story full ID is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.dialog_id != 0 && self.story_id > 0
    }
}

impl fmt::Display for StoryFullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "story {} in dialog {}", self.story_id, self.dialog_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range() {
        let range = DateRange::new(1609459200, 1609545600); // Jan 1, 2021 to Jan 2, 2021
        assert_eq!(range.duration_days(), 86400);
        assert!(range.is_valid());
    }

    #[test]
    fn test_statistical_value() {
        let value = StatisticalValue::new(100.0, 80.0, 25.0);
        assert_eq!(value.difference(), 20.0);
        assert!(value.is_growth());
        assert!(value.is_valid());
    }

    #[test]
    fn test_statistical_graph_async() {
        let graph = StatisticalGraph::async_graph("token123".to_string());
        assert_eq!(graph.get_token(), Some("token123"));
        assert!(!graph.is_loaded());
        assert!(!graph.is_error());
    }

    #[test]
    fn test_statistical_graph_error() {
        let graph = StatisticalGraph::error("Failed to load".to_string());
        assert_eq!(graph.get_error(), Some("Failed to load"));
        assert!(!graph.is_loaded());
        assert!(graph.is_error());
    }

    #[test]
    fn test_statistical_graph_data() {
        let graph = StatisticalGraph::data("json data".to_string(), "zoom token".to_string());
        assert_eq!(graph.get_data(), Some(("json data", "zoom token")));
        assert!(graph.is_loaded());
        assert!(!graph.is_error());
    }

    #[test]
    fn test_message_sender_info() {
        let info = MessageSenderInfo::new(123, 100, 50.0);
        assert!(info.is_valid());
        assert_eq!(info.user_id, 123);
        assert_eq!(info.message_count, 100);
    }

    #[test]
    fn test_administrator_actions_info() {
        let info = AdministratorActionsInfo::new(123, 10, 5, 2);
        assert!(info.is_valid());
        assert_eq!(info.total_actions(), 17);
    }

    #[test]
    fn test_inviter_info() {
        let info = InviterInfo::new(123, 50);
        assert!(info.is_valid());
        assert_eq!(info.invitation_count, 50);
    }

    #[test]
    fn test_chat_interaction_info() {
        let info = ChatInteractionInfo::new(1, 1000, 50, 25);
        assert!(info.is_valid());
        assert_eq!(info.total_interactions(), 1075);
    }

    #[test]
    fn test_chat_statistics_supergroup() {
        let period = DateRange::new(1609459200, 1609545600);
        let members = StatisticalValue::new(1000.0, 900.0, 11.11);
        let stats = ChatStatisticsSupergroup::new(
            period.clone(),
            members.clone(),
            StatisticalValue::new(500.0, 400.0, 25.0),
            StatisticalValue::new(200.0, 150.0, 33.33),
            StatisticalValue::new(50.0, 40.0, 25.0),
        );
        assert_eq!(stats.period, period);
        assert_eq!(stats.members, members);
    }

    #[test]
    fn test_chat_statistics_channel() {
        let period = DateRange::new(1609459200, 1609545600);
        let followers = StatisticalValue::new(10000.0, 9500.0, 5.26);
        let stats = ChatStatisticsChannel::new(
            period.clone(),
            followers.clone(),
            StatisticalValue::new(5000.0, 4500.0, 11.11),
            StatisticalValue::new(200.0, 180.0, 11.11),
            StatisticalValue::new(100.0, 90.0, 11.11),
        );
        assert_eq!(stats.period, period);
        assert_eq!(stats.followers, followers);
    }

    #[test]
    fn test_chat_statistics() {
        let period = DateRange::new(1609459200, 1609545600);
        let supergroup_stats = ChatStatisticsSupergroup::new(
            period.clone(),
            StatisticalValue::new(1000.0, 900.0, 11.11),
            StatisticalValue::new(500.0, 400.0, 25.0),
            StatisticalValue::new(200.0, 150.0, 33.33),
            StatisticalValue::new(50.0, 40.0, 25.0),
        );
        let stats = ChatStatistics::Supergroup(supergroup_stats);
        assert!(stats.is_supergroup());
        assert!(!stats.is_channel());
        assert!(stats.as_supergroup().is_some());
        assert!(stats.as_channel().is_none());
    }

    #[test]
    fn test_message_statistics() {
        let views_graph = StatisticalGraph::async_graph("token".to_string());
        let reactions_graph = StatisticalGraph::async_graph("token2".to_string());
        let stats = MessageStatistics::new(views_graph, reactions_graph);
        assert!(!stats.is_views_loaded());
        assert!(!stats.is_reactions_loaded());
    }

    #[test]
    fn test_story_statistics() {
        let views_graph = StatisticalGraph::async_graph("token".to_string());
        let reactions_graph = StatisticalGraph::async_graph("token2".to_string());
        let stats = StoryStatistics::new(views_graph, reactions_graph);
        assert!(!stats.is_views_loaded());
        assert!(!stats.is_reactions_loaded());
    }

    #[test]
    fn test_revenue_withdrawal_state() {
        assert!(RevenueWithdrawalState::pending().is_pending());
        assert!(RevenueWithdrawalState::failed().is_failed());
        let succeeded =
            RevenueWithdrawalState::succeeded(1609459200, "https://example.com".to_string());
        assert!(succeeded.is_succeeded());
    }

    #[test]
    fn test_revenue_transaction_type() {
        let refund = RevenueTransactionType::fragment_refund(1609459200);
        assert!(matches!(
            refund,
            RevenueTransactionType::FragmentRefund { .. }
        ));

        let earnings = RevenueTransactionType::sponsored_message_earnings(1609459200, 1609545600);
        assert!(earnings.is_earnings());
    }

    #[test]
    fn test_revenue_transaction() {
        let txn =
            RevenueTransaction::new(RevenueTransactionType::fragment_refund(1609459200), 100.0);
        assert!(!txn.is_withdrawal());
        assert!(!txn.is_earnings());
    }

    #[test]
    fn test_revenue_transactions() {
        let txns = RevenueTransactions::new(1000.0, Vec::new(), String::new());
        assert_eq!(txns.count(), 0);
        assert!(!txns.has_more());
        assert_eq!(txns.balance, 1000.0);
    }

    #[test]
    fn test_revenue_statistics() {
        let stats = RevenueStatistics::new(10000.0, 5000.0, 3000.0, true, 1.0);
        assert_eq!(stats.pending_balance(), 2000.0);
        assert!(stats.can_withdraw());
    }

    #[test]
    fn test_public_forward() {
        let msg_forward = PublicForward::message(123, 456, 1609459200);
        assert!(msg_forward.is_message());
        assert!(!msg_forward.is_story());
        assert_eq!(msg_forward.dialog_id(), 123);
        assert_eq!(msg_forward.date(), 1609459200);

        let story_forward = PublicForward::story(123, 789, 1609459200);
        assert!(!story_forward.is_message());
        assert!(story_forward.is_story());
    }

    #[test]
    fn test_public_forwards() {
        let forwards = PublicForwards::new(100, Vec::new(), String::new());
        assert_eq!(forwards.count(), 0);
        assert!(!forwards.has_more());
        assert_eq!(forwards.total_count, 100);
    }

    #[test]
    fn test_story_full_id() {
        let id = StoryFullId::new(123, 456);
        assert!(id.is_valid());
        assert_eq!(id.dialog_id, 123);
        assert_eq!(id.story_id, 456);
    }
}
