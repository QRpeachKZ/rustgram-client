//! Topic list implementation.

use crate::error::{Result, SavedMessagesError};
use crate::topic::SavedMessagesTopic;
use crate::topic_id::SavedMessagesTopicId;
use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

/// Topic date for ordering.
///
/// Combines an order value with a topic ID for sorting topics by date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicDate {
    /// Order value (higher = more recent).
    order: i64,
    /// Topic ID.
    topic_id: SavedMessagesTopicId,
}

impl TopicDate {
    /// Minimum topic date (for initialization).
    pub const MIN: Self = Self {
        order: i64::MIN,
        topic_id: SavedMessagesTopicId::new(DialogId::User(rustgram_types::UserId(0))),
    };

    /// Maximum topic date (for initialization).
    pub const MAX: Self = Self {
        order: i64::MAX,
        topic_id: SavedMessagesTopicId::new(DialogId::User(rustgram_types::UserId(1))),
    };

    /// Creates a new topic date.
    #[inline]
    pub const fn new(order: i64, topic_id: SavedMessagesTopicId) -> Self {
        Self { order, topic_id }
    }

    /// Returns the order value.
    #[inline]
    pub const fn order(&self) -> i64 {
        self.order
    }

    /// Returns the topic ID.
    #[inline]
    pub const fn topic_id(&self) -> SavedMessagesTopicId {
        self.topic_id
    }
}

impl PartialOrd for TopicDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TopicDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order (higher order = more recent = comes first)
        match other.order.cmp(&self.order) {
            std::cmp::Ordering::Equal => {
                // If orders are equal, compare by unique ID (also reversed)
                other
                    .topic_id
                    .get_unique_id()
                    .cmp(&self.topic_id.get_unique_id())
            }
            other_cmp => other_cmp,
        }
    }
}

/// Container for managing topic collections.
///
/// This structure holds all topics for a specific dialog and provides
/// methods for managing the topic list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicList {
    /// The dialog ID.
    dialog_id: DialogId,
    /// Total server topic count (-1 if unknown).
    server_total_count: i32,
    /// Total sent topic count (-1 if unknown).
    sent_total_count: i32,
    /// Generation counter for tracking updates.
    generation: u32,
    /// Pinned topic IDs.
    pinned_topic_ids: Vec<SavedMessagesTopicId>,
    /// Whether pinned topics are initialized.
    are_pinned_topics_inited: bool,
    /// Ordered topics by date.
    ordered_topics: BTreeSet<TopicDate>,
    /// Last topic date (in memory).
    last_topic_date: TopicDate,
    /// Offset date for pagination.
    offset_date: i32,
    /// Offset dialog ID for pagination.
    offset_dialog_id: DialogId,
    /// Offset message ID for pagination.
    offset_message_id: MessageId,
    /// Topics map.
    topics: HashMap<SavedMessagesTopicId, SavedMessagesTopic>,
}

impl TopicList {
    /// Creates a new topic list.
    #[inline]
    pub fn new(dialog_id: DialogId) -> Self {
        Self {
            dialog_id,
            server_total_count: -1,
            sent_total_count: -1,
            generation: 0,
            pinned_topic_ids: Vec::new(),
            are_pinned_topics_inited: false,
            ordered_topics: BTreeSet::new(),
            last_topic_date: TopicDate::MIN,
            offset_date: i32::MAX,
            offset_dialog_id: DialogId::default(),
            offset_message_id: MessageId::default(),
            topics: HashMap::new(),
        }
    }

    /// Returns the dialog ID.
    #[inline]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the server total count.
    #[inline]
    pub const fn server_total_count(&self) -> i32 {
        self.server_total_count
    }

    /// Returns the sent total count.
    #[inline]
    pub const fn sent_total_count(&self) -> i32 {
        self.sent_total_count
    }

    /// Returns the generation counter.
    #[inline]
    pub const fn generation(&self) -> u32 {
        self.generation
    }

    /// Returns the pinned topic IDs.
    #[inline]
    pub fn pinned_topic_ids(&self) -> &[SavedMessagesTopicId] {
        &self.pinned_topic_ids
    }

    /// Returns whether pinned topics are initialized.
    #[inline]
    pub const fn are_pinned_topics_inited(&self) -> bool {
        self.are_pinned_topics_inited
    }

    /// Returns the ordered topics.
    #[inline]
    pub fn ordered_topics(&self) -> &BTreeSet<TopicDate> {
        &self.ordered_topics
    }

    /// Returns the number of topics.
    #[inline]
    pub fn len(&self) -> usize {
        self.topics.len()
    }

    /// Checks if the topic list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.topics.is_empty()
    }

    /// Gets a topic by ID.
    #[inline]
    pub fn get_topic(&self, topic_id: SavedMessagesTopicId) -> Option<&SavedMessagesTopic> {
        self.topics.get(&topic_id)
    }

    /// Gets a mutable topic by ID.
    #[inline]
    pub fn get_topic_mut(
        &mut self,
        topic_id: SavedMessagesTopicId,
    ) -> Option<&mut SavedMessagesTopic> {
        self.topics.get_mut(&topic_id)
    }

    /// Checks if a topic exists.
    #[inline]
    pub fn has_topic(&self, topic_id: SavedMessagesTopicId) -> bool {
        self.topics.contains_key(&topic_id)
    }

    /// Adds a topic to the list.
    #[inline]
    pub fn add_topic(&mut self, topic: SavedMessagesTopic) -> Result<()> {
        let topic_id = topic.topic_id;
        self.topics.insert(topic_id, topic);
        self.increment_generation();
        Ok(())
    }

    /// Removes a topic from the list.
    #[inline]
    pub fn remove_topic(&mut self, topic_id: SavedMessagesTopicId) -> Result<()> {
        self.topics
            .remove(&topic_id)
            .ok_or_else(|| SavedMessagesError::topic_not_found(topic_id.get()))?;
        self.ordered_topics.retain(|td| td.topic_id != topic_id);
        self.pinned_topic_ids.retain(|id| id != &topic_id);
        self.increment_generation();
        Ok(())
    }

    /// Adds a pinned topic ID.
    #[inline]
    pub fn add_pinned_topic(&mut self, topic_id: SavedMessagesTopicId) -> Result<()> {
        if !self.pinned_topic_ids.contains(&topic_id) {
            self.pinned_topic_ids.push(topic_id);
            self.are_pinned_topics_inited = true;
            self.increment_generation();
        }
        Ok(())
    }

    /// Removes a pinned topic ID.
    #[inline]
    pub fn remove_pinned_topic(&mut self, topic_id: SavedMessagesTopicId) -> Result<()> {
        let original_len = self.pinned_topic_ids.len();
        self.pinned_topic_ids.retain(|id| id != &topic_id);
        if self.pinned_topic_ids.len() != original_len {
            self.increment_generation();
        }
        Ok(())
    }

    /// Sets the pinned topic IDs.
    #[inline]
    pub fn set_pinned_topics(&mut self, topic_ids: Vec<SavedMessagesTopicId>) -> Result<()> {
        self.pinned_topic_ids = topic_ids;
        self.are_pinned_topics_inited = true;
        self.increment_generation();
        Ok(())
    }

    /// Sets the server total count.
    #[inline]
    pub fn set_server_total_count(&mut self, count: i32) {
        self.server_total_count = count;
    }

    /// Sets the sent total count.
    #[inline]
    pub fn set_sent_total_count(&mut self, count: i32) {
        self.sent_total_count = count;
    }

    /// Adds a topic to the ordered list.
    #[inline]
    pub fn add_ordered_topic(&mut self, topic_date: TopicDate) -> Result<()> {
        self.ordered_topics.insert(topic_date);
        self.last_topic_date = topic_date;
        Ok(())
    }

    /// Sets the last topic date.
    #[inline]
    pub fn set_last_topic_date(&mut self, topic_date: TopicDate) {
        self.last_topic_date = topic_date;
    }

    /// Returns the last topic date.
    #[inline]
    pub const fn last_topic_date(&self) -> TopicDate {
        self.last_topic_date
    }

    /// Sets the offset for pagination.
    #[inline]
    pub fn set_offset(&mut self, date: i32, dialog_id: DialogId, message_id: MessageId) {
        self.offset_date = date;
        self.offset_dialog_id = dialog_id;
        self.offset_message_id = message_id;
    }

    /// Returns the offset date.
    #[inline]
    pub const fn offset_date(&self) -> i32 {
        self.offset_date
    }

    /// Returns the offset dialog ID.
    #[inline]
    pub const fn offset_dialog_id(&self) -> DialogId {
        self.offset_dialog_id
    }

    /// Returns the offset message ID.
    #[inline]
    pub const fn offset_message_id(&self) -> MessageId {
        self.offset_message_id
    }

    /// Increments the generation counter.
    #[inline]
    fn increment_generation(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    /// Clears all topics.
    #[inline]
    pub fn clear(&mut self) {
        self.topics.clear();
        self.ordered_topics.clear();
        self.pinned_topic_ids.clear();
        self.last_topic_date = TopicDate::MIN;
        self.increment_generation();
    }

    /// Returns an iterator over all topics.
    #[inline]
    pub fn topics(&self) -> impl Iterator<Item = &SavedMessagesTopic> {
        self.topics.values()
    }

    /// Returns a mutable iterator over all topics.
    #[inline]
    pub fn topics_mut(&mut self) -> impl Iterator<Item = &mut SavedMessagesTopic> {
        self.topics.values_mut()
    }
}

impl Default for TopicList {
    fn default() -> Self {
        Self::new(DialogId::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[test]
    fn test_topic_date_ordering() {
        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id1);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id2);

        let date1 = TopicDate::new(100, topic_id1);
        let date2 = TopicDate::new(200, topic_id2);

        // Higher order comes first (Greater in reverse order)
        assert_eq!(date1.cmp(&date2), std::cmp::Ordering::Greater);
        assert_eq!(date2.cmp(&date1), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_topic_date_equality() {
        let user_id = UserId::new(123).unwrap();
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id);

        let date1 = TopicDate::new(100, topic_id1);
        let date2 = TopicDate::new(100, topic_id2);

        assert_eq!(date1, date2);
    }

    #[test]
    fn test_topic_list_creation() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let list = TopicList::new(dialog_id);

        assert_eq!(list.dialog_id(), dialog_id);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.server_total_count(), -1);
        assert_eq!(list.sent_total_count(), -1);
        assert!(!list.are_pinned_topics_inited());
    }

    #[test]
    fn test_topic_list_add_topic() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut list = TopicList::new(dialog_id);

        let topic = SavedMessagesTopic::new(dialog_id, topic_id);
        list.add_topic(topic).unwrap();

        assert_eq!(list.len(), 1);
        assert!(list.has_topic(topic_id));
        assert!(list.get_topic(topic_id).is_some());
    }

    #[test]
    fn test_topic_list_remove_topic() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut list = TopicList::new(dialog_id);

        let topic = SavedMessagesTopic::new(dialog_id, topic_id);
        list.add_topic(topic.clone()).unwrap();
        list.remove_topic(topic_id).unwrap();

        assert_eq!(list.len(), 0);
        assert!(!list.has_topic(topic_id));
    }

    #[test]
    fn test_topic_list_pinned() {
        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let dialog_id = DialogId::from_user(user_id1);
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id1);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id2);
        let mut list = TopicList::new(dialog_id);

        list.add_pinned_topic(topic_id1).unwrap();
        list.add_pinned_topic(topic_id2).unwrap();

        assert_eq!(list.pinned_topic_ids().len(), 2);
        assert!(list.are_pinned_topics_inited());

        list.remove_pinned_topic(topic_id1).unwrap();
        assert_eq!(list.pinned_topic_ids().len(), 1);
    }

    #[test]
    fn test_topic_list_set_pinned() {
        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let dialog_id = DialogId::from_user(user_id1);
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id1);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id2);
        let mut list = TopicList::new(dialog_id);

        list.set_pinned_topics(vec![topic_id1, topic_id2]).unwrap();

        assert_eq!(list.pinned_topic_ids().len(), 2);
        assert!(list.are_pinned_topics_inited());
    }

    #[test]
    fn test_topic_list_ordered_topics() {
        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let dialog_id = DialogId::from_user(user_id1);
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id1);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id2);
        let mut list = TopicList::new(dialog_id);

        let date1 = TopicDate::new(100, topic_id1);
        let date2 = TopicDate::new(200, topic_id2);

        list.add_ordered_topic(date1).unwrap();
        list.add_ordered_topic(date2).unwrap();

        assert_eq!(list.ordered_topics().len(), 2);
    }

    #[test]
    fn test_topic_list_offsets() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let msg_id = MessageId::from_server_id(100);
        let mut list = TopicList::new(dialog_id);

        list.set_offset(123456, dialog_id, msg_id);

        assert_eq!(list.offset_date(), 123456);
        assert_eq!(list.offset_dialog_id(), dialog_id);
        assert_eq!(list.offset_message_id(), msg_id);
    }

    #[test]
    fn test_topic_list_clear() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut list = TopicList::new(dialog_id);

        let topic = SavedMessagesTopic::new(dialog_id, topic_id);
        list.add_topic(topic).unwrap();

        list.clear();

        assert!(list.is_empty());
        assert!(list.ordered_topics().is_empty());
        assert!(list.pinned_topic_ids().is_empty());
    }

    #[test]
    fn test_topic_list_generation() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let mut list = TopicList::new(dialog_id);

        let gen1 = list.generation();
        let topic = SavedMessagesTopic::new(dialog_id, topic_id);
        list.add_topic(topic).unwrap();
        let gen2 = list.generation();

        assert_ne!(gen1, gen2);
    }

    #[test]
    fn test_topic_list_total_counts() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let mut list = TopicList::new(dialog_id);

        list.set_server_total_count(10);
        list.set_sent_total_count(5);

        assert_eq!(list.server_total_count(), 10);
        assert_eq!(list.sent_total_count(), 5);
    }
}
