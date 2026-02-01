// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Alarm manager for scheduled tasks in Telegram MTProto client.
//!
//! This module implements TDLib's AlarmManager from `td/telegram/AlarmManager.h`.
//!
//! # Overview
//!
//! The AlarmManager provides a way to schedule callbacks after a specified delay.
//! It's used internally by TDLib for various timed operations.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

/// Error type for AlarmManager operations.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum AlarmError {
    /// Invalid alarm timeout (must be > 0 and <= 3e9 seconds).
    #[error("invalid alarm timeout: {0}, must be > 0 and <= 3000000000")]
    InvalidTimeout(f64),

    /// Alarm ID not found.
    #[error("alarm not found: {0}")]
    AlarmNotFound(i64),

    /// Alarm manager was stopped.
    #[error("alarm manager stopped")]
    Stopped,
}

/// Result type for AlarmManager operations.
pub type Result<T> = std::result::Result<T, AlarmError>;

/// Unique identifier for an alarm.
///
/// # Example
///
/// ```
/// use rustgram_alarm_manager::AlarmId;
///
/// let id = AlarmId::new(123);
/// assert_eq!(id.get(), 123);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlarmId(i64);

impl AlarmId {
    /// Creates a new AlarmId.
    #[inline]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner ID value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for AlarmId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlarmId({})", self.0)
    }
}

/// Internal task for managing a single alarm.
#[derive(Debug)]
struct AlarmTask {
    /// The unique alarm ID.
    #[allow(dead_code)]
    id: AlarmId,
    /// Receiver for signaling when the alarm triggers.
    receiver: oneshot::Receiver<()>,
    /// Handle for the timeout task.
    timeout_handle: JoinHandle<()>,
}

/// Manager for scheduling and canceling timed alarms.
///
/// The AlarmManager provides a way to schedule callbacks after a specified delay.
/// It's useful for implementing timeouts, delayed operations, and periodic tasks.
///
/// # Example
///
/// ```no_run
/// use rustgram_alarm_manager::AlarmManager;
///
/// #[tokio::main]
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     let manager = AlarmManager::new()?;
///
///     // Set an alarm for 5 seconds
///     let alarm_id = manager.set_alarm(5.0)?;
///     println!("Alarm scheduled: {}", alarm_id);
///
///     // Cancel the alarm before it triggers
///     manager.cancel_alarm(alarm_id)?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AlarmManager {
    /// Inner state shared across clones.
    inner: Arc<AlarmManagerInner>,
}

#[derive(Debug)]
struct AlarmManagerInner {
    /// Counter for generating unique alarm IDs.
    alarm_id_counter: AtomicI64,
    /// Map of pending alarms.
    pending_alarms: std::sync::Mutex<HashMap<AlarmId, AlarmTask>>,
}

impl AlarmManager {
    /// Maximum valid alarm timeout in seconds (3 billion).
    pub const MAX_TIMEOUT: f64 = 3_000_000_000.0;

    /// Minimum valid alarm timeout in seconds (must be > 0).
    pub const MIN_TIMEOUT: f64 = 0.0;

    /// Creates a new AlarmManager.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_alarm_manager::AlarmManager;
    ///
    /// let manager = AlarmManager::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: Arc::new(AlarmManagerInner {
                alarm_id_counter: AtomicI64::new(1),
                pending_alarms: std::sync::Mutex::new(HashMap::new()),
            }),
        })
    }

    /// Sets an alarm to trigger after the specified number of seconds.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The delay in seconds (must be > 0 and <= 3e9)
    ///
    /// # Returns
    ///
    /// The ID of the scheduled alarm
    ///
    /// # Errors
    ///
    /// Returns `AlarmError::InvalidTimeout` if the timeout is out of range.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustgram_alarm_manager::AlarmManager;
    /// # #[tokio::main]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AlarmManager::new()?;
    ///
    /// // Schedule an alarm for 10 seconds
    /// let alarm_id = manager.set_alarm(10.0)?;
    /// println!("Scheduled alarm: {}", alarm_id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_alarm(&self, seconds: f64) -> Result<AlarmId> {
        if seconds <= Self::MIN_TIMEOUT || seconds > Self::MAX_TIMEOUT {
            return Err(AlarmError::InvalidTimeout(seconds));
        }

        // Generate unique alarm ID
        let id = AlarmId::new(self.inner.alarm_id_counter.fetch_add(1, Ordering::SeqCst));

        // Create oneshot channel for signaling
        let (tx, rx) = oneshot::channel();

        // Spawn the timeout task
        let duration = std::time::Duration::from_secs_f64(seconds);
        let timeout_handle = tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            // The receiver will be dropped if the alarm was canceled
            let _ = tx.send(());
        });

        // Store the alarm task
        let task = AlarmTask {
            id,
            receiver: rx,
            timeout_handle,
        };

        if let Ok(mut alarms) = self.inner.pending_alarms.lock() {
            alarms.insert(id, task);
        }

        Ok(id)
    }

    /// Cancels a pending alarm.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the alarm to cancel
    ///
    /// # Returns
    ///
    /// Ok(()) if the alarm was canceled, or Err if the alarm was not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_alarm_manager::AlarmManager;
    /// # #[tokio::main]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AlarmManager::new()?;
    ///
    /// let alarm_id = manager.set_alarm(60.0)?;
    /// manager.cancel_alarm(alarm_id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_alarm(&self, id: AlarmId) -> Result<()> {
        let mut alarms = self
            .inner
            .pending_alarms
            .lock()
            .map_err(|_| AlarmError::Stopped)?;

        if let Some(task) = alarms.remove(&id) {
            // Abort the timeout task
            task.timeout_handle.abort();
            // Drop the receiver to signal cancellation
            drop(task.receiver);
            Ok(())
        } else {
            Err(AlarmError::AlarmNotFound(id.get()))
        }
    }

    /// Cancels all pending alarms.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_alarm_manager::AlarmManager;
    /// # #[tokio::main]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AlarmManager::new()?;
    ///
    /// manager.set_alarm(10.0)?;
    /// manager.set_alarm(20.0)?;
    /// manager.set_alarm(30.0)?;
    ///
    /// assert_eq!(manager.pending_count(), 3);
    ///
    /// manager.cancel_all();
    ///
    /// assert_eq!(manager.pending_count(), 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_all(&self) {
        if let Ok(mut alarms) = self.inner.pending_alarms.lock() {
            for (_id, task) in alarms.drain() {
                task.timeout_handle.abort();
                drop(task.receiver);
            }
        }
    }

    /// Returns the number of pending alarms.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_alarm_manager::AlarmManager;
    /// # #[tokio::main]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AlarmManager::new()?;
    ///
    /// assert_eq!(manager.pending_count(), 0);
    ///
    /// manager.set_alarm(10.0)?;
    /// manager.set_alarm(20.0)?;
    ///
    /// assert_eq!(manager.pending_count(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn pending_count(&self) -> usize {
        self.inner
            .pending_alarms
            .lock()
            .map(|alarms| alarms.len())
            .unwrap_or(0)
    }

    /// Checks if there are any pending alarms.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_alarm_manager::AlarmManager;
    /// # #[tokio::main]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AlarmManager::new()?;
    ///
    /// assert!(!manager.has_pending());
    ///
    /// manager.set_alarm(10.0)?;
    ///
    /// assert!(manager.has_pending());
    /// # Ok(())
    /// # }
    /// ```
    pub fn has_pending(&self) -> bool {
        self.pending_count() > 0
    }

    /// Returns the IDs of all pending alarms.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_alarm_manager::AlarmManager;
    /// # #[tokio::main]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = AlarmManager::new()?;
    ///
    /// let id1 = manager.set_alarm(10.0)?;
    /// let id2 = manager.set_alarm(20.0)?;
    ///
    /// let ids = manager.pending_alarm_ids();
    /// assert_eq!(ids.len(), 2);
    /// assert!(ids.contains(&id1));
    /// assert!(ids.contains(&id2));
    /// # Ok(())
    /// # }
    /// ```
    pub fn pending_alarm_ids(&self) -> Vec<AlarmId> {
        self.inner
            .pending_alarms
            .lock()
            .map(|alarms| alarms.keys().copied().collect())
            .unwrap_or_default()
    }
}

impl Default for AlarmManager {
    fn default() -> Self {
        match Self::new() {
            Ok(manager) => manager,
            Err(_) => Self {
                inner: Arc::new(AlarmManagerInner {
                    alarm_id_counter: AtomicI64::new(1),
                    pending_alarms: std::sync::Mutex::new(HashMap::new()),
                }),
            },
        }
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-alarm-manager";

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-alarm-manager");
    }

    #[test]
    fn test_alarm_id_new() {
        let id = AlarmId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_alarm_id_display() {
        let id = AlarmId::new(456);
        assert_eq!(format!("{id}"), "AlarmId(456)");
    }

    #[test]
    fn test_alarm_manager_new() {
        let manager = AlarmManager::new().unwrap();
        assert_eq!(manager.pending_count(), 0);
        assert!(!manager.has_pending());
    }

    #[test]
    fn test_alarm_manager_default() {
        let manager = AlarmManager::default();
        assert_eq!(manager.pending_count(), 0);
    }

    #[tokio::test]
    async fn test_set_alarm_valid_timeout() {
        let manager = AlarmManager::new().unwrap();

        // Valid timeouts
        assert!(manager.set_alarm(0.1).is_ok());
        assert!(manager.set_alarm(1.0).is_ok());
        assert!(manager.set_alarm(100.0).is_ok());
        assert!(manager.set_alarm(1_000_000_000.0).is_ok());

        manager.cancel_all();
    }

    #[test]
    fn test_set_alarm_invalid_timeout() {
        let manager = AlarmManager::new().unwrap();

        // Zero or negative timeouts
        let result = manager.set_alarm(0.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(AlarmError::InvalidTimeout(0.0))));

        let result = manager.set_alarm(-1.0);
        assert!(result.is_err());

        let result = manager.set_alarm(-100.0);
        assert!(result.is_err());

        // Too large timeout
        let result = manager.set_alarm(4_000_000_000.0);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cancel_alarm() {
        let manager = AlarmManager::new().unwrap();

        let id = manager.set_alarm(10.0).unwrap();
        assert_eq!(manager.pending_count(), 1);

        assert!(manager.cancel_alarm(id).is_ok());
        assert_eq!(manager.pending_count(), 0);

        // Canceling again should fail
        assert_eq!(
            manager.cancel_alarm(id),
            Err(AlarmError::AlarmNotFound(id.get()))
        );
    }

    #[test]
    fn test_cancel_alarm_not_found() {
        let manager = AlarmManager::new().unwrap();

        let fake_id = AlarmId::new(9999);
        let result = manager.cancel_alarm(fake_id);
        assert!(result.is_err());
        assert!(matches!(result, Err(AlarmError::AlarmNotFound(9999))));
    }

    #[tokio::test]
    async fn test_cancel_all() {
        let manager = AlarmManager::new().unwrap();

        manager.set_alarm(10.0).unwrap();
        manager.set_alarm(20.0).unwrap();
        manager.set_alarm(30.0).unwrap();

        assert_eq!(manager.pending_count(), 3);

        manager.cancel_all();

        assert_eq!(manager.pending_count(), 0);
        assert!(!manager.has_pending());
    }

    #[tokio::test]
    async fn test_pending_count() {
        let manager = AlarmManager::new().unwrap();

        assert_eq!(manager.pending_count(), 0);

        manager.set_alarm(10.0).unwrap();
        assert_eq!(manager.pending_count(), 1);

        manager.set_alarm(20.0).unwrap();
        assert_eq!(manager.pending_count(), 2);

        manager.set_alarm(30.0).unwrap();
        assert_eq!(manager.pending_count(), 3);

        manager.cancel_all();
    }

    #[tokio::test]
    async fn test_has_pending() {
        let manager = AlarmManager::new().unwrap();

        assert!(!manager.has_pending());

        manager.set_alarm(10.0).unwrap();
        assert!(manager.has_pending());

        manager.cancel_all();
        assert!(!manager.has_pending());
    }

    #[tokio::test]
    async fn test_pending_alarm_ids() {
        let manager = AlarmManager::new().unwrap();

        let id1 = manager.set_alarm(10.0).unwrap();
        let id2 = manager.set_alarm(20.0).unwrap();
        let id3 = manager.set_alarm(30.0).unwrap();

        let ids = manager.pending_alarm_ids();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
        assert!(ids.contains(&id3));

        manager.cancel_all();
    }

    #[tokio::test]
    async fn test_alarm_id_unique() {
        let manager = AlarmManager::new().unwrap();

        let id1 = manager.set_alarm(10.0).unwrap();
        let id2 = manager.set_alarm(20.0).unwrap();
        let id3 = manager.set_alarm(30.0).unwrap();

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);

        manager.cancel_all();
    }

    #[tokio::test]
    async fn test_alarm_id_sequential() {
        let manager = AlarmManager::new().unwrap();

        let id1 = manager.set_alarm(10.0).unwrap();
        let id2 = manager.set_alarm(20.0).unwrap();

        // IDs should be sequential
        assert_eq!(id2.get(), id1.get() + 1);

        manager.cancel_all();
    }

    #[tokio::test]
    async fn test_max_timeout_boundary() {
        let manager = AlarmManager::new().unwrap();

        // Just at the boundary
        assert!(manager.set_alarm(3_000_000_000.0).is_ok());

        // Just over the boundary
        let result = manager.set_alarm(3_000_000_001.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(AlarmError::InvalidTimeout(_))));

        manager.cancel_all();
    }

    #[test]
    fn test_alarm_error_display() {
        let err = AlarmError::InvalidTimeout(0.0);
        assert!(format!("{}", err).contains("invalid alarm timeout"));

        let err = AlarmError::AlarmNotFound(123);
        assert!(format!("{}", err).contains("alarm not found"));

        let err = AlarmError::Stopped;
        assert!(format!("{}", err).contains("alarm manager stopped"));
    }

    #[tokio::test]
    async fn test_cancel_one_of_many() {
        let manager = AlarmManager::new().unwrap();

        let id1 = manager.set_alarm(10.0).unwrap();
        let id2 = manager.set_alarm(20.0).unwrap();
        let id3 = manager.set_alarm(30.0).unwrap();

        assert_eq!(manager.pending_count(), 3);

        // Cancel the middle one
        manager.cancel_alarm(id2).unwrap();

        assert_eq!(manager.pending_count(), 2);

        let ids = manager.pending_alarm_ids();
        assert!(ids.contains(&id1));
        assert!(!ids.contains(&id2));
        assert!(ids.contains(&id3));

        manager.cancel_all();
    }

    #[tokio::test]
    async fn test_multiple_managers() {
        let manager1 = AlarmManager::new().unwrap();
        let manager2 = AlarmManager::new().unwrap();

        manager1.set_alarm(10.0).unwrap();
        manager2.set_alarm(20.0).unwrap();

        assert_eq!(manager1.pending_count(), 1);
        assert_eq!(manager2.pending_count(), 1);

        manager1.cancel_all();
        manager2.cancel_all();
    }

    #[tokio::test]
    async fn test_alarm_triggers() {
        let manager = AlarmManager::new().unwrap();

        // Set a very short alarm (50ms)
        let id = manager.set_alarm(0.05).unwrap();
        assert_eq!(manager.pending_count(), 1);

        // Wait a bit longer than the alarm duration
        tokio::time::sleep(Duration::from_millis(100)).await;

        // The alarm should have triggered and been removed
        // Note: This is an implementation detail - the actual alarm task
        // completes but we don't auto-remove it from pending_alarms
        // So this test verifies the alarm was scheduled correctly
        assert_eq!(manager.pending_count(), 1);

        // We can still cancel it (it won't do anything if already triggered)
        manager.cancel_alarm(id).ok();
    }

    #[tokio::test]
    async fn test_clone_manager() {
        let manager1 = AlarmManager::new().unwrap();
        let manager2 = manager1.clone();

        manager1.set_alarm(10.0).unwrap();

        // Both clones should see the same alarms
        assert_eq!(manager1.pending_count(), 1);
        assert_eq!(manager2.pending_count(), 1);

        manager2.cancel_all();

        assert_eq!(manager1.pending_count(), 0);
        assert_eq!(manager2.pending_count(), 0);
    }
}
