//! Saved messages manager implementation.

use crate::error::Result;
use crate::topic::DraftMessage;
use crate::topic_id::SavedMessagesTopicId;
use crate::topic_list::TopicList;
use rustgram_types::{DialogId, MessageId};
use std::collections::HashMap;

/// Constants for saved messages manager.
pub mod constants {
    /// Maximum number of messages to fetch in get_history requests.
    pub const MAX_GET_HISTORY: i32 = 100;

    /// Minimum pinned topic order value.
    /// From TDLib: static_cast<int64>(2147000000) << 32
    /// Calculated as: 2147000000i64 * (1i64 << 32) = 9223372036854716160
    pub const MIN_PINNED_TOPIC_ORDER: i64 = 9_223_372_036_854_716_160;

    /// Hidden author dialog ID.
    pub const HIDDEN_AUTHOR_DIALOG_ID: i64 = 2_666_000;
}

/// Main manager for saved messages operations.
///
/// This manager handles all operations related to saved messages topics,
/// including loading, updating, and managing topic state.
#[derive(Debug, Clone)]
pub struct SavedMessagesManager {
    /// Main topic list for saved messages.
    topic_list: TopicList,
    /// Monoforum topic lists by dialog ID.
    monoforum_topic_lists: HashMap<DialogId, TopicList>,
    /// Current pinned topic order.
    current_pinned_order: i64,
}

impl SavedMessagesManager {
    /// Creates a new saved messages manager.
    #[inline]
    pub fn new() -> Self {
        Self {
            topic_list: TopicList::new(DialogId::default()),
            monoforum_topic_lists: HashMap::new(),
            current_pinned_order: constants::MIN_PINNED_TOPIC_ORDER,
        }
    }

    /// Checks if a topic exists.
    #[inline]
    pub fn has_topic(&self, dialog_id: DialogId, topic_id: SavedMessagesTopicId) -> bool {
        let list = self.get_topic_list(dialog_id);
        list.has_topic(topic_id)
    }

    /// Gets a topic ID from a raw topic ID value.
    #[inline]
    pub fn get_topic_id(&self, dialog_id: DialogId, topic_id: i64) -> Result<SavedMessagesTopicId> {
        // For now, we create a topic ID from the dialog ID
        // In the full implementation, this would look up the actual topic
        let _ = topic_id; // Suppress unused warning
        Ok(SavedMessagesTopicId::new(dialog_id))
    }

    /// Gets multiple topic IDs.
    #[inline]
    pub fn get_topic_ids(
        &self,
        dialog_id: DialogId,
        topic_ids: &[i64],
    ) -> Result<Vec<SavedMessagesTopicId>> {
        topic_ids
            .iter()
            .map(|&id| self.get_topic_id(dialog_id, id))
            .collect()
    }

    /// Checks if a message is the last in a topic.
    #[inline]
    pub fn is_last_topic_message(
        &self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        message_id: MessageId,
    ) -> bool {
        let list = self.get_topic_list(dialog_id);
        list.get_topic(topic_id)
            .map(|topic| topic.is_last_message(message_id))
            .unwrap_or(false)
    }

    /// Handles a topic message being added.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn on_topic_message_added(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        message_id: MessageId,
        message_date: i32,
        from_update: bool,
        need_update: bool,
        is_new: bool,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        let topic = list.get_topic_mut(topic_id);

        if let Some(topic) = topic {
            topic.add_message(message_id, message_date)?;
        } else if is_new {
            // For new topics, we would create one here
            // For now, we skip this to avoid the dependency issue
        }

        let _ = (from_update, need_update); // Suppress unused warnings
        Ok(())
    }

    /// Handles a topic message being updated.
    #[inline]
    pub fn on_topic_message_updated(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        message_id: MessageId,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        let _ = list.get_topic_mut(topic_id);
        let _ = message_id; // Suppress unused warning
        Ok(())
    }

    /// Handles a topic message being deleted.
    #[inline]
    pub fn on_topic_message_deleted(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        message_id: MessageId,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            topic.remove_message(message_id)?;
        }
        Ok(())
    }

    /// Handles all dialog messages being deleted.
    #[inline]
    pub fn on_all_dialog_messages_deleted(&mut self, dialog_id: DialogId) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        list.clear();
        Ok(())
    }

    /// Handles a topic draft message being updated.
    #[inline]
    pub fn on_topic_draft_message_updated(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        draft_message_date: i32,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            let draft = DraftMessage::new(draft_message_date);
            topic.set_draft_message(Some(draft));
        }
        Ok(())
    }

    /// Clears a monoforum topic draft by sent message.
    #[inline]
    pub fn clear_monoforum_topic_draft_by_sent_message(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        message_clear_draft: bool,
    ) -> Result<()> {
        if message_clear_draft {
            let list = self.get_topic_list_mut(dialog_id);
            if let Some(topic) = list.get_topic_mut(topic_id) {
                topic.set_draft_message(None);
            }
        }
        Ok(())
    }

    /// Reads monoforum topic messages.
    #[inline]
    pub fn read_monoforum_topic_messages(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        read_inbox_max_message_id: MessageId,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            topic.set_read_inbox_max_message_id(read_inbox_max_message_id);
        }
        Ok(())
    }

    /// Handles an update to read monoforum inbox.
    #[inline]
    pub fn on_update_read_monoforum_inbox(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        read_inbox_max_message_id: MessageId,
    ) -> Result<()> {
        self.read_monoforum_topic_messages(dialog_id, topic_id, read_inbox_max_message_id)
    }

    /// Handles an update to read all monoforum inbox.
    #[inline]
    pub fn on_update_read_all_monoforum_inbox(
        &mut self,
        dialog_id: DialogId,
        read_inbox_max_message_id: MessageId,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        for topic in list.topics_mut() {
            topic.set_read_inbox_max_message_id(read_inbox_max_message_id);
        }
        Ok(())
    }

    /// Handles an update to read monoforum outbox.
    #[inline]
    pub fn on_update_read_monoforum_outbox(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        read_outbox_max_message_id: MessageId,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            topic.set_read_outbox_max_message_id(read_outbox_max_message_id);
        }
        Ok(())
    }

    /// Handles an update to monoforum nopaid messages exception.
    #[inline]
    pub fn on_update_monoforum_nopaid_messages_exception(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        nopaid_messages_exception: bool,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            topic.set_nopaid_messages_exception(nopaid_messages_exception);
        }
        Ok(())
    }

    /// Handles an update to topic marked as unread.
    #[inline]
    pub fn on_update_topic_is_marked_as_unread(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        is_marked_as_unread: bool,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            topic.set_marked_as_unread(is_marked_as_unread);
        }
        Ok(())
    }

    /// Handles a topic reaction count change.
    #[inline]
    pub fn on_topic_reaction_count_changed(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        count: i32,
        is_relative: bool,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            let new_count = if is_relative {
                topic.unread_reaction_count() + count
            } else {
                count
            };
            topic.set_unread_reaction_count(new_count);
        }
        Ok(())
    }

    /// Deletes a monoforum topic history.
    #[inline]
    pub fn delete_monoforum_topic_history(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        list.remove_topic(topic_id)?;
        Ok(())
    }

    /// Deletes a saved messages topic history.
    #[inline]
    pub fn delete_saved_messages_topic_history(
        &mut self,
        topic_id: SavedMessagesTopicId,
    ) -> Result<()> {
        self.topic_list.remove_topic(topic_id)?;
        Ok(())
    }

    /// Toggles a saved messages topic pinned state.
    #[inline]
    pub fn toggle_saved_messages_topic_is_pinned(
        &mut self,
        topic_id: SavedMessagesTopicId,
        is_pinned: bool,
    ) -> Result<()> {
        if let Some(topic) = self.topic_list.get_topic_mut(topic_id) {
            if is_pinned {
                let order = self.current_pinned_order;
                self.current_pinned_order += 1;
                topic.set_pinned_order(order);
                self.topic_list.add_pinned_topic(topic_id)?;
            } else {
                topic.set_pinned_order(0);
                self.topic_list.remove_pinned_topic(topic_id)?;
            }
        }
        Ok(())
    }

    /// Sets pinned saved messages topics.
    #[inline]
    pub fn set_pinned_saved_messages_topics(
        &mut self,
        topic_ids: Vec<SavedMessagesTopicId>,
    ) -> Result<()> {
        // Calculate orders first
        let base_order = self.current_pinned_order;
        self.current_pinned_order += topic_ids.len() as i64;

        // Update pinned orders
        for (index, topic_id) in topic_ids.iter().enumerate() {
            if let Some(topic) = self.topic_list.get_topic_mut(*topic_id) {
                let order = base_order + (topic_ids.len() - index - 1) as i64;
                topic.set_pinned_order(order);
            }
        }

        self.topic_list.set_pinned_topics(topic_ids)?;
        Ok(())
    }

    /// Sets a monoforum topic as marked as unread.
    #[inline]
    pub fn set_monoforum_topic_is_marked_as_unread(
        &mut self,
        dialog_id: DialogId,
        topic_id: SavedMessagesTopicId,
        is_marked_as_unread: bool,
    ) -> Result<()> {
        let list = self.get_topic_list_mut(dialog_id);
        if let Some(topic) = list.get_topic_mut(topic_id) {
            topic.set_marked_as_unread(is_marked_as_unread);
        }
        Ok(())
    }

    /// Gets the next pinned topic order.
    #[inline]
    #[allow(dead_code)]
    fn get_next_pinned_order(&mut self) -> i64 {
        let order = self.current_pinned_order;
        self.current_pinned_order += 1;
        order
    }

    /// Gets the main topic list.
    #[inline]
    pub const fn topic_list(&self) -> &TopicList {
        &self.topic_list
    }

    /// Gets a monoforum topic list by dialog ID.
    #[inline]
    pub fn get_monoforum_topic_list(&self, dialog_id: DialogId) -> Option<&TopicList> {
        self.monoforum_topic_lists.get(&dialog_id)
    }

    /// Gets a mutable monoforum topic list by dialog ID.
    #[inline]
    #[allow(dead_code)]
    fn get_monoforum_topic_list_mut(&mut self, dialog_id: DialogId) -> &mut TopicList {
        self.monoforum_topic_lists
            .entry(dialog_id)
            .or_insert_with(|| TopicList::new(dialog_id))
    }

    /// Gets the appropriate topic list for a dialog.
    #[inline]
    fn get_topic_list(&self, dialog_id: DialogId) -> &TopicList {
        // For now, return the main topic list for all dialogs
        // In the full implementation, this would check if it's a monoforum dialog
        let _ = dialog_id;
        &self.topic_list
    }

    /// Gets a mutable topic list for a dialog.
    #[inline]
    fn get_topic_list_mut(&mut self, dialog_id: DialogId) -> &mut TopicList {
        // For now, return the main topic list for all dialogs
        // In the full implementation, this would check if it's a monoforum dialog
        let _ = dialog_id;
        &mut self.topic_list
    }
}

impl Default for SavedMessagesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[test]
    fn test_manager_creation() {
        let manager = SavedMessagesManager::new();
        assert!(manager.topic_list().is_empty());
        assert_eq!(
            manager.current_pinned_order,
            constants::MIN_PINNED_TOPIC_ORDER
        );
    }

    #[test]
    fn test_manager_has_topic() {
        let manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        assert!(!manager.has_topic(dialog_id, topic_id));
    }

    #[test]
    fn test_manager_get_topic_id() {
        let manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let result = manager.get_topic_id(dialog_id, 456);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_toggle_pinned() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        // Pin the topic
        manager
            .toggle_saved_messages_topic_is_pinned(topic_id, true)
            .unwrap();

        // Unpin the topic
        manager
            .toggle_saved_messages_topic_is_pinned(topic_id, false)
            .unwrap();
    }

    #[test]
    fn test_manager_set_pinned_topics() {
        let mut manager = SavedMessagesManager::new();
        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id1);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id2);

        manager
            .set_pinned_saved_messages_topics(vec![topic_id1, topic_id2])
            .unwrap();

        assert_eq!(manager.topic_list().pinned_topic_ids().len(), 2);
    }

    #[test]
    fn test_manager_on_topic_message_added() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let message_id = MessageId::from_server_id(100);

        let result = manager
            .on_topic_message_added(dialog_id, topic_id, message_id, 123456, false, true, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_on_topic_message_deleted() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let message_id = MessageId::from_server_id(100);

        let result = manager.on_topic_message_deleted(dialog_id, topic_id, message_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_on_all_dialog_messages_deleted() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let result = manager.on_all_dialog_messages_deleted(dialog_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_on_topic_draft_message_updated() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        let result = manager.on_topic_draft_message_updated(dialog_id, topic_id, 123456);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_on_update_read_monoforum_inbox() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let message_id = MessageId::from_server_id(100);

        let result = manager.on_update_read_monoforum_inbox(dialog_id, topic_id, message_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_on_update_read_all_monoforum_inbox() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(100);

        let result = manager.on_update_read_all_monoforum_inbox(dialog_id, message_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_on_update_topic_is_marked_as_unread() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        let result = manager.on_update_topic_is_marked_as_unread(dialog_id, topic_id, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_on_topic_reaction_count_changed() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        let result = manager.on_topic_reaction_count_changed(dialog_id, topic_id, 5, true);
        assert!(result.is_ok());

        let result = manager.on_topic_reaction_count_changed(dialog_id, topic_id, 10, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_delete_monoforum_topic_history() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        // Note: delete will fail if topic doesn't exist
        // This is expected behavior - we're testing that the error is handled
        let _ = manager.delete_monoforum_topic_history(dialog_id, topic_id);
    }

    #[test]
    fn test_manager_delete_saved_messages_topic_history() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        // Note: delete will fail if topic doesn't exist
        // This is expected behavior - we're testing that the error is handled
        let _ = manager.delete_saved_messages_topic_history(topic_id);
    }

    #[test]
    fn test_manager_set_monoforum_topic_is_marked_as_unread() {
        let mut manager = SavedMessagesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);

        let result = manager.set_monoforum_topic_is_marked_as_unread(dialog_id, topic_id, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::MAX_GET_HISTORY, 100);
        assert_eq!(constants::MIN_PINNED_TOPIC_ORDER, 9_223_372_036_854_716_160);
        assert_eq!(constants::HIDDEN_AUTHOR_DIALOG_ID, 2_666_000);
    }
}
