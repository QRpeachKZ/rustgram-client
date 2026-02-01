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

//! # Updates Manager
//!
//! Manages TDLib updates and state synchronization.
//!
//! ## Overview
//!
//! This module handles processing and managing updates from Telegram servers,
//! including PTS (Permanent Timestamp State) and QTS (Channel Timestamp) management.
//!
//! ## TDLib Correspondence
//!
//! Corresponds to `td/telegram/UpdatesManager.h` in TDLib.
//!
//! ## Example
//!
//! ```rust
//! use rustgram-updates_manager::UpdatesManager;
//!
//! let manager = UpdatesManager::new();
//! let pts = manager.get_pts();
//! let qts = manager.get_qts();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::DialogId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, Ordering};
use std::sync::RwLock;
use thiserror::Error;
use tracing::{debug, info, warn};

/// Errors that can occur in the updates manager.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UpdatesError {
    /// PTS gap detected
    #[error("PTS gap detected: expected={expected}, actual={actual}")]
    PtsGap {
        /// Expected PTS value
        expected: i32,
        /// Actual PTS value received
        actual: i32,
    },

    /// QTS gap detected
    #[error("QTS gap detected: expected={expected}, actual={actual}")]
    QtsGap {
        /// Expected QTS value
        expected: i32,
        /// Actual QTS value received
        actual: i32,
    },

    /// Invalid update sequence
    #[error("Invalid update sequence: {0}")]
    InvalidSequence(String),

    /// Get difference failed
    #[error("Get difference failed: {0}")]
    GetDifferenceFailed(String),

    /// Timeout waiting for updates
    #[error("Timeout waiting for updates")]
    Timeout,

    /// Internal lock was poisoned
    #[error("Internal lock was poisoned")]
    LockPoisoned,
}

/// Result type for updates operations.
pub type UpdatesResult<T> = Result<T, UpdatesError>;

/// Pending PTS update waiting to be applied.
#[derive(Debug, Clone, PartialEq)]
struct PendingPtsUpdate {
    /// PTS value for this update
    pts: i32,
    /// Number of PTS counts this update represents
    pts_count: i32,
    /// When the update was received
    receive_time: f64,
    /// Update sequence number
    seq: i32,
}

impl PartialOrd for PendingPtsUpdate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.pts.cmp(&other.pts) {
            std::cmp::Ordering::Equal => other.pts_count.partial_cmp(&self.pts_count),
            other => Some(other),
        }
    }
}

/// Pending QTS update waiting to be applied.
#[derive(Debug, Clone, PartialEq)]
struct PendingQtsUpdate {
    /// QTS value for this update
    qts: i32,
    /// When the update was received
    receive_time: f64,
}

/// Pending seq update waiting to be applied.
#[derive(Debug, Clone, PartialEq)]
struct PendingSeqUpdate {
    /// Sequence begin
    seq_begin: i32,
    /// Sequence end
    seq_end: i32,
    /// Date of the update
    date: i32,
    /// When the update was received
    receive_time: f64,
}

impl PartialOrd for PendingSeqUpdate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.seq_begin.cmp(&other.seq_begin) {
            std::cmp::Ordering::Equal => other.seq_end.partial_cmp(&self.seq_end),
            other => Some(other),
        }
    }
}

/// Manager for TDLib updates and state synchronization.
///
/// This manager handles:
/// - Processing updates from Telegram servers
/// - Managing PTS (Permanent Timestamp State)
/// - Managing QTS (Channel Timestamp State)
/// - Handling getDifference calls
/// - Processing pending updates
///
/// # Thread Safety
///
/// This manager uses atomic operations and RwLock for thread-safe access.
#[derive(Debug)]
pub struct UpdatesManager {
    /// Current PTS value
    pts: AtomicI32,
    /// Current QTS value
    qts: AtomicI32,
    /// Current date
    date: AtomicI32,
    /// Current sequence number
    seq: AtomicI32,

    /// Pending PTS updates
    pending_pts_updates: RwLock<Vec<PendingPtsUpdate>>,
    /// Pending QTS updates
    pending_qts_updates: RwLock<HashMap<i32, PendingQtsUpdate>>,
    /// Pending seq updates (reserved for future use)
    _pending_seq_updates: RwLock<Vec<PendingSeqUpdate>>,

    /// Whether get_difference is running
    running_get_difference: AtomicBool,

    /// PTS gap size
    pts_gap: AtomicI32,
    /// QTS gap size
    qts_gap: AtomicI32,

    /// Last confirmed PTS
    last_confirmed_pts: AtomicI32,
    /// Last confirmed QTS
    last_confirmed_qts: AtomicI32,

    /// Number of updates processed
    updates_processed: AtomicI64,
}

impl Default for UpdatesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdatesManager {
    /// Minimum PTS gap before forcing get_difference
    pub const FORCED_GET_DIFFERENCE_PTS_DIFF: i32 = 100000;

    /// Maximum unfilled gap time
    pub const MAX_UNFILLED_GAP_TIME: f64 = 0.7;

    /// Maximum PTS save delay
    pub const MAX_PTS_SAVE_DELAY: f64 = 0.05;

    /// Creates a new updates manager.
    #[must_use]
    pub fn new() -> Self {
        info!("Creating new UpdatesManager");
        Self {
            pts: AtomicI32::new(0),
            qts: AtomicI32::new(0),
            date: AtomicI32::new(0),
            seq: AtomicI32::new(0),
            pending_pts_updates: RwLock::new(Vec::new()),
            pending_qts_updates: RwLock::new(HashMap::new()),
            _pending_seq_updates: RwLock::new(Vec::new()),
            running_get_difference: AtomicBool::new(false),
            pts_gap: AtomicI32::new(0),
            qts_gap: AtomicI32::new(0),
            last_confirmed_pts: AtomicI32::new(0),
            last_confirmed_qts: AtomicI32::new(0),
            updates_processed: AtomicI64::new(0),
        }
    }

    /// Returns the current PTS value.
    #[must_use]
    pub fn get_pts(&self) -> i32 {
        self.pts.load(Ordering::Acquire)
    }

    /// Returns the current QTS value.
    #[must_use]
    pub fn get_qts(&self) -> i32 {
        self.qts.load(Ordering::Acquire)
    }

    /// Returns the current date.
    #[must_use]
    pub fn get_date(&self) -> i32 {
        self.date.load(Ordering::Acquire)
    }

    /// Returns the current sequence number.
    #[must_use]
    pub fn get_seq(&self) -> i32 {
        self.seq.load(Ordering::Acquire)
    }

    /// Sets the PTS value.
    ///
    /// # Arguments
    ///
    /// * `pts` - The new PTS value
    /// * `source` - Source of the PTS update (for logging)
    pub fn set_pts(&self, pts: i32, source: &str) {
        let old_pts = self.pts.swap(pts, Ordering::Release);
        debug!("PTS changed: {} -> {} (source={})", old_pts, pts, source);
    }

    /// Adds to the current PTS value.
    ///
    /// # Arguments
    ///
    /// * `delta` - The amount to add
    pub fn add_pts(&self, delta: i32) {
        self.pts.fetch_add(delta, Ordering::Release);
        debug!("PTS increased by {}", delta);
    }

    /// Sets the QTS value.
    ///
    /// # Arguments
    ///
    /// * `qts` - The new QTS value
    /// * `source` - Source of the QTS update (for logging)
    pub fn set_qts(&self, qts: i32, source: &str) {
        let old_qts = self.qts.swap(qts, Ordering::Release);
        debug!("QTS changed: {} -> {} (source={})", old_qts, qts, source);
    }

    /// Adds to the current QTS value.
    ///
    /// # Arguments
    ///
    /// * `delta` - The amount to add
    pub fn add_qts(&self, delta: i32) {
        self.qts.fetch_add(delta, Ordering::Release);
        debug!("QTS increased by {}", delta);
    }

    /// Sets the date.
    ///
    /// # Arguments
    ///
    /// * `date` - The new date value
    /// * `from_update` - Whether this is from an update
    /// * `source` - Source of the date update
    pub fn set_date(&self, date: i32, from_update: bool, source: &str) {
        let old_date = self.date.swap(date, Ordering::Release);
        debug!(
            "Date changed: {} -> {} (from_update={}, source={})",
            old_date, date, from_update, source
        );
    }

    /// Adds a pending PTS update.
    ///
    /// # Arguments
    ///
    /// * `new_pts` - The new PTS value
    /// * `pts_count` - Number of PTS counts
    /// * `receive_time` - When the update was received
    /// * `seq` - Sequence number
    #[allow(clippy::expect_used)]
    pub fn add_pending_pts_update(
        &self,
        new_pts: i32,
        pts_count: i32,
        receive_time: f64,
        seq: i32,
    ) {
        let update = PendingPtsUpdate {
            pts: new_pts,
            pts_count,
            receive_time,
            seq,
        };
        let mut pending = self
            .pending_pts_updates
            .write()
            .expect("pending_pts_updates lock should not be poisoned");
        pending.push(update);
        // Sort to maintain heap-like ordering (min-heap behavior via reverse sort)
        pending.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        debug!(
            "Added pending PTS update: pts={}, count={}",
            new_pts, pts_count
        );
    }

    /// Returns the number of pending PTS updates.
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn get_pending_pts_update_count(&self) -> usize {
        let pending = self
            .pending_pts_updates
            .read()
            .expect("pending_pts_updates lock should not be poisoned");
        pending.len()
    }

    /// Adds a pending QTS update.
    ///
    /// # Arguments
    ///
    /// * `qts` - The QTS value
    /// * `receive_time` - When the update was received
    #[allow(clippy::expect_used)]
    pub fn add_pending_qts_update(&self, qts: i32, receive_time: f64) {
        let update = PendingQtsUpdate { qts, receive_time };
        let mut pending = self
            .pending_qts_updates
            .write()
            .expect("pending_qts_updates lock should not be poisoned");
        pending.insert(qts, update);
        debug!("Added pending QTS update: qts={}", qts);
    }

    /// Returns the number of pending QTS updates.
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn get_pending_qts_update_count(&self) -> usize {
        let pending = self
            .pending_qts_updates
            .read()
            .expect("pending_qts_updates lock should not be poisoned");
        pending.len()
    }

    /// Returns whether get_difference is currently running.
    #[must_use]
    pub fn running_get_difference(&self) -> bool {
        self.running_get_difference.load(Ordering::Acquire)
    }

    /// Sets whether get_difference is running.
    pub fn set_running_get_difference(&self, running: bool) {
        self.running_get_difference
            .store(running, Ordering::Release);
        debug!("Set running_get_difference = {}", running);
    }

    /// Returns the current PTS gap size.
    #[must_use]
    pub fn get_pts_gap(&self) -> i32 {
        self.pts_gap.load(Ordering::Acquire)
    }

    /// Sets the PTS gap size.
    pub fn set_pts_gap(&self, gap: i32) {
        self.pts_gap.store(gap, Ordering::Release);
    }

    /// Returns the current QTS gap size.
    #[must_use]
    pub fn get_qts_gap(&self) -> i32 {
        self.qts_gap.load(Ordering::Acquire)
    }

    /// Sets the QTS gap size.
    pub fn set_qts_gap(&self, gap: i32) {
        self.qts_gap.store(gap, Ordering::Release);
    }

    /// Returns the last confirmed PTS.
    #[must_use]
    pub fn get_last_confirmed_pts(&self) -> i32 {
        self.last_confirmed_pts.load(Ordering::Acquire)
    }

    /// Sets the last confirmed PTS.
    pub fn set_last_confirmed_pts(&self, pts: i32) {
        self.last_confirmed_pts.store(pts, Ordering::Release);
    }

    /// Returns the last confirmed QTS.
    #[must_use]
    pub fn get_last_confirmed_qts(&self) -> i32 {
        self.last_confirmed_qts.load(Ordering::Acquire)
    }

    /// Sets the last confirmed QTS.
    pub fn set_last_confirmed_qts(&self, qts: i32) {
        self.last_confirmed_qts.store(qts, Ordering::Release);
    }

    /// Processes all pending PTS updates.
    ///
    /// # Returns
    ///
    /// The number of updates processed.
    #[allow(clippy::expect_used)]
    pub fn process_all_pending_pts_updates(&self) -> usize {
        let mut pending = self
            .pending_pts_updates
            .write()
            .expect("pending_pts_updates lock should not be poisoned");
        let count = pending.len();

        while let Some(update) = pending.pop() {
            self.set_pts(update.pts, "pending_update");
            self.updates_processed.fetch_add(1, Ordering::Relaxed);
        }

        debug!("Processed {} pending PTS updates", count);
        count
    }

    /// Processes all pending QTS updates.
    ///
    /// # Returns
    ///
    /// The number of updates processed.
    #[allow(clippy::expect_used)]
    pub fn process_all_pending_qts_updates(&self) -> usize {
        let mut pending = self
            .pending_qts_updates
            .write()
            .expect("pending_qts_updates lock should not be poisoned");
        let count = pending.len();

        for (qts, _update) in pending.drain() {
            self.set_qts(qts, "pending_update");
        }

        debug!("Processed {} pending QTS updates", count);
        count
    }

    /// Drops all pending PTS updates.
    #[allow(clippy::expect_used)]
    pub fn drop_all_pending_pts_updates(&self) {
        let mut pending = self
            .pending_pts_updates
            .write()
            .expect("pending_pts_updates lock should not be poisoned");
        let count = pending.len();
        pending.clear();
        debug!("Dropped {} pending PTS updates", count);
    }

    /// Returns the number of updates processed.
    #[must_use]
    pub fn get_updates_processed(&self) -> i64 {
        self.updates_processed.load(Ordering::Relaxed)
    }

    /// Initializes state from saved values.
    ///
    /// # Arguments
    ///
    /// * `pts` - Initial PTS value
    /// * `qts` - Initial QTS value
    /// * `date` - Initial date value
    /// * `seq` - Initial sequence value
    pub fn init_state(&self, pts: i32, qts: i32, date: i32, seq: i32) {
        info!(
            "Initializing state: pts={}, qts={}, date={}, seq={}",
            pts, qts, date, seq
        );
        self.set_pts(pts, "init");
        self.set_qts(qts, "init");
        self.set_date(date, false, "init");
        self.seq.store(seq, Ordering::Release);
    }

    /// Checks if there's a PTS gap.
    ///
    /// # Arguments
    ///
    /// * `new_pts` - The new PTS value
    ///
    /// # Returns
    ///
    /// `true` if there's a gap (new_pts > current_pts + 1).
    #[must_use]
    pub fn check_pts_gap(&self, new_pts: i32) -> bool {
        let current_pts = self.get_pts();
        new_pts > current_pts + 1
    }

    /// Checks if there's a QTS gap.
    ///
    /// # Arguments
    ///
    /// * `new_qts` - The new QTS value
    ///
    /// # Returns
    ///
    /// `true` if there's a gap (new_qts > current_qts + 1).
    #[must_use]
    pub fn check_qts_gap(&self, new_qts: i32) -> bool {
        let current_qts = self.get_qts();
        new_qts > current_qts + 1
    }

    /// Schedules get_difference to be called.
    pub fn schedule_get_difference(&self) {
        if !self.running_get_difference() {
            info!("Scheduling get_difference");
            self.set_running_get_difference(true);
        }
    }

    /// Confirms PTS and QTS values.
    ///
    /// # Arguments
    ///
    /// * `qts` - The QTS value to confirm
    pub fn confirm_pts_qts(&self, qts: i32) {
        let pts = self.get_pts();
        self.set_last_confirmed_pts(pts);
        self.set_last_confirmed_qts(qts);
        debug!("Confirmed PTS={}, QTS={}", pts, qts);
    }

    /// Returns dialog IDs from updates.
    ///
    /// This is a stub for TDLib compatibility.
    /// In the full implementation, this would extract dialog IDs from updates.
    #[must_use]
    pub fn get_update_dialog_ids(&self) -> Vec<DialogId> {
        Vec::new()
    }

    /// Checks if updates are empty.
    ///
    /// This is a stub for TDLib compatibility.
    /// In the full implementation, this would check if an updates object is empty.
    #[must_use]
    pub fn are_updates_empty(&self) -> bool {
        true
    }

    /// Processes an update from Telegram.
    ///
    /// This method handles the routing of updates to the appropriate managers
    /// based on the update type. It also updates PTS/QTS state.
    ///
    /// # Arguments
    ///
    /// * `update` - The update to process
    ///
    /// # Errors
    ///
    /// Returns an error if the update cannot be processed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rustgram_updates_manager::UpdatesManager;
    /// use rustgram_types::{Update, UpdateType, NewMessageUpdate, MessageId};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = UpdatesManager::new();
    /// let update = Update::new(UpdateType::NewMessage(NewMessageUpdate::new(MessageId::new(1))));
    ///
    /// // Process the update - will be routed to MessagesManager
    /// manager.process_update(update)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn process_update(&self, update: rustgram_types::Update) -> UpdatesResult<()> {
        // Update PTS/QTS if present
        if let Some(pts) = update.pts {
            let pts_count = update.pts_count.unwrap_or(1);

            // Check for PTS gap
            if self.check_pts_gap(pts) {
                warn!(
                    "PTS gap detected: current={}, new={}",
                    self.get_pts(),
                    pts
                );
                return Err(UpdatesError::PtsGap {
                    expected: self.get_pts() + 1,
                    actual: pts,
                });
            }

            self.set_pts(pts, "process_update");
            if pts_count > 0 {
                self.add_pts(pts_count);
            }
        }

        // Route update to appropriate handler
        self.route_update(&update.update_type)?;

        // Increment processed counter
        self.updates_processed.fetch_add(1, Ordering::Release);

        Ok(())
    }

    /// Routes an update type to the appropriate handler.
    ///
    /// This method determines which manager should handle the update
    /// and dispatches it accordingly.
    ///
    /// # Arguments
    ///
    /// * `update_type` - The update type to route
    fn route_update(&self, update_type: &rustgram_types::UpdateType) -> UpdatesResult<()> {
        match update_type {
            // Message updates -> MessagesManager
            rustgram_types::UpdateType::NewMessage(_) => {
                debug!("Routing NewMessage update to MessagesManager");
                // TODO: Call MessagesManager::on_new_message()
                Ok(())
            }
            rustgram_types::UpdateType::NewChannelMessage(_) => {
                debug!("Routing NewChannelMessage update to MessagesManager");
                // TODO: Call MessagesManager::on_new_channel_message()
                Ok(())
            }
            rustgram_types::UpdateType::DeleteMessages(_) => {
                debug!("Routing DeleteMessages update to MessagesManager");
                // TODO: Call MessagesManager::on_delete_messages()
                Ok(())
            }
            rustgram_types::UpdateType::DeleteChannelMessages(_) => {
                debug!("Routing DeleteChannelMessages update to MessagesManager");
                // TODO: Call MessagesManager::on_delete_channel_messages()
                Ok(())
            }
            rustgram_types::UpdateType::MessageIdAssigned(_) => {
                debug!("Routing MessageIdAssigned update to MessagesManager");
                // TODO: Call MessagesManager::on_message_id_assigned()
                Ok(())
            }

            // User updates -> UserManager
            rustgram_types::UpdateType::UserStatus(_) => {
                debug!("Routing UserStatus update to UserManager");
                // TODO: Call UserManager::on_user_status_update()
                Ok(())
            }
            rustgram_types::UpdateType::UserName(_) => {
                debug!("Routing UserName update to UserManager");
                // TODO: Call UserManager::on_user_name_update()
                Ok(())
            }
            rustgram_types::UpdateType::UserTyping(_) => {
                debug!("Routing UserTyping update to UserManager");
                // TODO: Call UserManager::on_user_typing()
                Ok(())
            }

            // Chat updates -> ChatManager
            rustgram_types::UpdateType::ChatUserTyping(_) => {
                debug!("Routing ChatUserTyping update to ChatManager");
                // TODO: Call ChatManager::on_chat_user_typing()
                Ok(())
            }

            // Dialog updates -> DialogManager
            rustgram_types::UpdateType::DialogPinned => {
                debug!("Routing DialogPinned update to DialogManager");
                // TODO: Call DialogManager::on_dialog_pinned()
                Ok(())
            }
            rustgram_types::UpdateType::PinnedDialogs => {
                debug!("Routing PinnedDialogs update to DialogManager");
                // TODO: Call DialogManager::on_pinned_dialogs_update()
                Ok(())
            }

            // Authorization updates -> AuthManager
            rustgram_types::UpdateType::NewAuthorization(_) => {
                debug!("Routing NewAuthorization update to AuthManager");
                // TODO: Call AuthManager::on_new_authorization()
                Ok(())
            }

            // Configuration updates -> ConfigManager
            rustgram_types::UpdateType::DcOptions => {
                debug!("Routing DcOptions update to ConfigManager");
                // TODO: Call ConfigManager::on_dc_options_update()
                Ok(())
            }
            rustgram_types::UpdateType::Config => {
                debug!("Routing Config update to ConfigManager");
                // TODO: Call ConfigManager::on_config_update()
                Ok(())
            }
            rustgram_types::UpdateType::NotifySettings => {
                debug!("Routing NotifySettings update to NotificationManager");
                // TODO: Call NotificationManager::on_notify_settings_update()
                Ok(())
            }

            // Unknown updates
            rustgram_types::UpdateType::Unknown { constructor_id } => {
                warn!("Received unknown update with constructor_id: 0x{:08x}", constructor_id);
                Ok(())
            }
        }
    }

    /// Processes multiple updates in sequence.
    ///
    /// # Arguments
    ///
    /// * `updates` - Iterator of updates to process
    ///
    /// # Errors
    ///
    /// Returns an error if any update cannot be processed.
    /// Processing stops at the first error.
    pub fn process_updates(
        &self,
        updates: impl IntoIterator<Item = rustgram_types::Update>,
    ) -> UpdatesResult<()> {
        for update in updates {
            self.process_update(update)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UpdatesManager::new();
        assert_eq!(manager.get_pts(), 0);
        assert_eq!(manager.get_qts(), 0);
        assert_eq!(manager.get_date(), 0);
        assert_eq!(manager.get_seq(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = UpdatesManager::default();
        assert_eq!(manager.get_pts(), 0);
        assert!(!manager.running_get_difference());
    }

    #[test]
    fn test_set_pts() {
        let manager = UpdatesManager::new();
        manager.set_pts(100, "test");
        assert_eq!(manager.get_pts(), 100);
    }

    #[test]
    fn test_add_pts() {
        let manager = UpdatesManager::new();
        manager.set_pts(100, "test");
        manager.add_pts(5);
        assert_eq!(manager.get_pts(), 105);
    }

    #[test]
    fn test_set_qts() {
        let manager = UpdatesManager::new();
        manager.set_qts(50, "test");
        assert_eq!(manager.get_qts(), 50);
    }

    #[test]
    fn test_add_qts() {
        let manager = UpdatesManager::new();
        manager.set_qts(50, "test");
        manager.add_qts(3);
        assert_eq!(manager.get_qts(), 53);
    }

    #[test]
    fn test_set_date() {
        let manager = UpdatesManager::new();
        manager.set_date(12345, true, "test");
        assert_eq!(manager.get_date(), 12345);
    }

    #[test]
    fn test_running_get_difference() {
        let manager = UpdatesManager::new();
        assert!(!manager.running_get_difference());

        manager.set_running_get_difference(true);
        assert!(manager.running_get_difference());

        manager.set_running_get_difference(false);
        assert!(!manager.running_get_difference());
    }

    #[test]
    fn test_pts_gap() {
        let manager = UpdatesManager::new();
        manager.set_pts(100, "test");

        assert!(!manager.check_pts_gap(101)); // No gap
        assert!(manager.check_pts_gap(103)); // Gap of 2
        assert!(manager.check_pts_gap(200)); // Large gap
    }

    #[test]
    fn test_qts_gap() {
        let manager = UpdatesManager::new();
        manager.set_qts(50, "test");

        assert!(!manager.check_qts_gap(51)); // No gap
        assert!(manager.check_qts_gap(53)); // Gap of 2
    }

    #[test]
    fn test_add_pending_pts_update() {
        let manager = UpdatesManager::new();
        manager.add_pending_pts_update(100, 1, 0.0, 0);

        assert_eq!(manager.get_pending_pts_update_count(), 1);
    }

    #[test]
    fn test_add_pending_qts_update() {
        let manager = UpdatesManager::new();
        manager.add_pending_qts_update(50, 0.0);

        assert_eq!(manager.get_pending_qts_update_count(), 1);
    }

    #[test]
    fn test_process_all_pending_pts_updates() {
        let manager = UpdatesManager::new();
        manager.add_pending_pts_update(100, 1, 0.0, 0);
        manager.add_pending_pts_update(101, 1, 0.0, 0);
        manager.add_pending_pts_update(102, 1, 0.0, 0);

        let count = manager.process_all_pending_pts_updates();
        assert_eq!(count, 3);
        assert_eq!(manager.get_pending_pts_update_count(), 0);
    }

    #[test]
    fn test_process_all_pending_qts_updates() {
        let manager = UpdatesManager::new();
        manager.add_pending_qts_update(50, 0.0);
        manager.add_pending_qts_update(51, 0.0);

        let count = manager.process_all_pending_qts_updates();
        assert_eq!(count, 2);
        assert_eq!(manager.get_pending_qts_update_count(), 0);
    }

    #[test]
    fn test_drop_all_pending_pts_updates() {
        let manager = UpdatesManager::new();
        manager.add_pending_pts_update(100, 1, 0.0, 0);
        manager.add_pending_pts_update(101, 1, 0.0, 0);

        manager.drop_all_pending_pts_updates();
        assert_eq!(manager.get_pending_pts_update_count(), 0);
    }

    #[test]
    fn test_last_confirmed_pts() {
        let manager = UpdatesManager::new();
        assert_eq!(manager.get_last_confirmed_pts(), 0);

        manager.set_last_confirmed_pts(100);
        assert_eq!(manager.get_last_confirmed_pts(), 100);
    }

    #[test]
    fn test_last_confirmed_qts() {
        let manager = UpdatesManager::new();
        assert_eq!(manager.get_last_confirmed_qts(), 0);

        manager.set_last_confirmed_qts(50);
        assert_eq!(manager.get_last_confirmed_qts(), 50);
    }

    #[test]
    fn test_confirm_pts_qts() {
        let manager = UpdatesManager::new();
        manager.set_pts(100, "test");
        manager.set_qts(50, "test");

        manager.confirm_pts_qts(50);

        assert_eq!(manager.get_last_confirmed_pts(), 100);
        assert_eq!(manager.get_last_confirmed_qts(), 50);
    }

    #[test]
    fn test_updates_processed() {
        let manager = UpdatesManager::new();
        assert_eq!(manager.get_updates_processed(), 0);

        manager.process_all_pending_pts_updates(); // No updates
        assert_eq!(manager.get_updates_processed(), 0);

        manager.add_pending_pts_update(100, 1, 0.0, 0);
        manager.process_all_pending_pts_updates();
        assert_eq!(manager.get_updates_processed(), 1);
    }

    #[test]
    fn test_init_state() {
        let manager = UpdatesManager::new();
        manager.init_state(100, 50, 12345, 10);

        assert_eq!(manager.get_pts(), 100);
        assert_eq!(manager.get_qts(), 50);
        assert_eq!(manager.get_date(), 12345);
        assert_eq!(manager.get_seq(), 10);
    }

    #[test]
    fn test_schedule_get_difference() {
        let manager = UpdatesManager::new();
        assert!(!manager.running_get_difference());

        manager.schedule_get_difference();
        assert!(manager.running_get_difference());

        // Should not schedule again if already running
        manager.schedule_get_difference();
        assert!(manager.running_get_difference());
    }

    #[test]
    fn test_pts_gap_size() {
        let manager = UpdatesManager::new();
        assert_eq!(manager.get_pts_gap(), 0);

        manager.set_pts_gap(10);
        assert_eq!(manager.get_pts_gap(), 10);
    }

    #[test]
    fn test_qts_gap_size() {
        let manager = UpdatesManager::new();
        assert_eq!(manager.get_qts_gap(), 0);

        manager.set_qts_gap(5);
        assert_eq!(manager.get_qts_gap(), 5);
    }

    #[test]
    fn test_get_update_dialog_ids() {
        let manager = UpdatesManager::new();
        let ids = manager.get_update_dialog_ids();
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_are_updates_empty() {
        let manager = UpdatesManager::new();
        assert!(manager.are_updates_empty());
    }

    #[test]
    fn test_forced_get_difference_pts_diff_const() {
        assert_eq!(UpdatesManager::FORCED_GET_DIFFERENCE_PTS_DIFF, 100000);
    }

    #[test]
    fn test_max_unfilled_gap_time_const() {
        assert!((UpdatesManager::MAX_UNFILLED_GAP_TIME - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn test_max_pts_save_delay_const() {
        assert!((UpdatesManager::MAX_PTS_SAVE_DELAY - 0.05).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multiple_pts_updates() {
        let manager = UpdatesManager::new();

        for i in 1..=10 {
            manager.add_pending_pts_update(i * 10, 1, 0.0, i);
        }

        assert_eq!(manager.get_pending_pts_update_count(), 10);

        let count = manager.process_all_pending_pts_updates();
        assert_eq!(count, 10);
        assert_eq!(manager.get_updates_processed(), 10);
    }

    #[test]
    fn test_multiple_qts_updates() {
        let manager = UpdatesManager::new();

        for i in 1..=5 {
            manager.add_pending_qts_update(i, 0.0);
        }

        assert_eq!(manager.get_pending_qts_update_count(), 5);

        let count = manager.process_all_pending_qts_updates();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_pending_update_ordering() {
        let update1 = PendingPtsUpdate {
            pts: 100,
            pts_count: 1,
            receive_time: 0.0,
            seq: 1,
        };
        let update2 = PendingPtsUpdate {
            pts: 101,
            pts_count: 1,
            receive_time: 0.0,
            seq: 2,
        };

        assert!(update2 < update1); // BinaryHeap is a max-heap
    }
}
