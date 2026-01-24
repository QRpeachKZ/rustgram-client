// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Per-actor message queue.

use crate::event::Event;
use crossbeam::queue::SegQueue;
use std::fmt;

/// A per-actor mailbox for message queuing.
///
/// `Mailbox` provides a thread-safe queue for events destined for a specific actor.
/// It uses a lock-free segmented queue for efficient concurrent access.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::{Mailbox, Event};
///
/// let mailbox = Mailbox::new();
/// mailbox.push(Event::Start);
/// assert_eq!(mailbox.len(), 1);
/// ```
#[derive(Debug)]
pub struct Mailbox {
    /// The underlying queue for events.
    queue: SegQueue<Event>,
    /// The maximum capacity of the mailbox (None for unlimited).
    capacity: Option<usize>,
}

impl Mailbox {
    /// Creates a new mailbox with unlimited capacity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Mailbox;
    ///
    /// let mailbox = Mailbox::new();
    /// assert!(mailbox.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            queue: SegQueue::new(),
            capacity: None,
        }
    }

    /// Creates a new mailbox with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The maximum number of events in the mailbox
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Mailbox;
    ///
    /// let mailbox = Mailbox::with_capacity(100);
    /// assert_eq!(mailbox.capacity(), Some(100));
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: SegQueue::new(),
            capacity: Some(capacity),
        }
    }

    /// Pushes an event into the mailbox.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to push
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the event was pushed successfully
    /// * `Err(())` - If the mailbox is full
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Mailbox, Event};
    ///
    /// let mailbox = Mailbox::with_capacity(1);
    /// assert!(mailbox.push(Event::Start).is_ok());
    /// assert!(mailbox.push(Event::Stop).is_err());
    /// ```
    pub fn push(&self, event: Event) -> Result<(), ()> {
        if let Some(cap) = self.capacity {
            if self.len() >= cap {
                return Err(());
            }
        }
        self.queue.push(event);
        Ok(())
    }

    /// Attempts to pop an event from the mailbox.
    ///
    /// # Returns
    ///
    /// * `Some(Event)` - If an event was available
    /// * `None` - If the mailbox is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Mailbox, Event};
    ///
    /// let mailbox = Mailbox::new();
    /// assert!(mailbox.pop().is_none());
    ///
    /// mailbox.push(Event::Start);
    /// assert!(mailbox.pop().is_some());
    /// ```
    pub fn pop(&self) -> Option<Event> {
        self.queue.pop()
    }

    /// Checks if the mailbox is empty.
    ///
    /// # Returns
    ///
    /// * `true` - If the mailbox has no events
    /// * `false` - If the mailbox has events
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Mailbox, Event};
    ///
    /// let mailbox = Mailbox::new();
    /// assert!(mailbox.is_empty());
    ///
    /// mailbox.push(Event::Start);
    /// assert!(!mailbox.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the number of events in the mailbox.
    ///
    /// # Note
    ///
    /// This operation is O(n) where n is the number of elements in the queue.
    /// For frequent checks, consider tracking the length externally.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Mailbox, Event};
    ///
    /// let mailbox = Mailbox::new();
    /// assert_eq!(mailbox.len(), 0);
    ///
    /// mailbox.push(Event::Start);
    /// assert_eq!(mailbox.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut iter = self.queue.iter();
        while iter.next().is_some() {
            count += 1;
        }
        count
    }

    /// Returns the capacity of the mailbox.
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - The maximum capacity
    /// * `None` - Unlimited capacity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Mailbox;
    ///
    /// let unlimited = Mailbox::new();
    /// assert_eq!(unlimited.capacity(), None);
    ///
    /// let limited = Mailbox::with_capacity(100);
    /// assert_eq!(limited.capacity(), Some(100));
    /// ```
    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }

    /// Clears all events from the mailbox.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Mailbox, Event};
    ///
    /// let mailbox = Mailbox::new();
    /// mailbox.push(Event::Start);
    /// mailbox.push(Event::Stop);
    /// assert!(!mailbox.is_empty());
    ///
    /// mailbox.clear();
    /// assert!(mailbox.is_empty());
    /// ```
    pub fn clear(&self) {
        while self.pop().is_some() {
            // Keep popping until empty
        }
    }
}

impl Default for Mailbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Mailbox {
    fn clone(&self) -> Self {
        // Create a new mailbox with the same capacity
        let new = Self {
            queue: SegQueue::new(),
            capacity: self.capacity,
        };
        // Note: We don't clone the contents as events can't be cloned
        // This is primarily for copying the mailbox configuration
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailbox_new() {
        let mailbox = Mailbox::new();
        assert!(mailbox.is_empty());
        assert_eq!(mailbox.capacity(), None);
    }

    #[test]
    fn test_mailbox_with_capacity() {
        let mailbox = Mailbox::with_capacity(100);
        assert!(mailbox.is_empty());
        assert_eq!(mailbox.capacity(), Some(100));
    }

    #[test]
    fn test_mailbox_push_pop() {
        let mailbox = Mailbox::new();
        assert!(mailbox.pop().is_none());

        assert!(mailbox.push(Event::Start).is_ok());
        assert!(!mailbox.is_empty());

        let event = mailbox.pop();
        assert!(event.is_some());
        assert!(event.unwrap().is_start());
        assert!(mailbox.is_empty());
    }

    #[test]
    fn test_mailbox_push_full() {
        let mailbox = Mailbox::with_capacity(1);
        assert!(mailbox.push(Event::Start).is_ok());
        assert!(mailbox.push(Event::Stop).is_err());
    }

    #[test]
    fn test_mailbox_is_empty() {
        let mailbox = Mailbox::new();
        assert!(mailbox.is_empty());

        mailbox.push(Event::Start);
        assert!(!mailbox.is_empty());

        mailbox.pop();
        assert!(mailbox.is_empty());
    }

    #[test]
    fn test_mailbox_len() {
        let mailbox = Mailbox::new();
        assert_eq!(mailbox.len(), 0);

        mailbox.push(Event::Start);
        assert_eq!(mailbox.len(), 1);

        mailbox.push(Event::Stop);
        assert_eq!(mailbox.len(), 2);

        mailbox.pop();
        assert_eq!(mailbox.len(), 1);

        mailbox.pop();
        assert_eq!(mailbox.len(), 0);
    }

    #[test]
    fn test_mailbox_clear() {
        let mailbox = Mailbox::new();
        mailbox.push(Event::Start);
        mailbox.push(Event::Stop);
        mailbox.push(Event::Yield);
        assert_eq!(mailbox.len(), 3);

        mailbox.clear();
        assert!(mailbox.is_empty());
        assert_eq!(mailbox.len(), 0);
    }

    #[test]
    fn test_mailbox_default() {
        let mailbox = Mailbox::default();
        assert!(mailbox.is_empty());
        assert_eq!(mailbox.capacity(), None);
    }

    #[test]
    fn test_mailbox_clone() {
        let mailbox1 = Mailbox::with_capacity(50);
        let mailbox2 = mailbox1.clone();
        assert_eq!(mailbox2.capacity(), Some(50));
        // Note: contents are not cloned
    }

    #[test]
    fn test_mailbox_fifo_order() {
        let mailbox = Mailbox::new();
        mailbox.push(Event::Start);
        mailbox.push(Event::Stop);
        mailbox.push(Event::Yield);

        assert!(mailbox.pop().unwrap().is_start());
        assert!(mailbox.pop().unwrap().is_stop());
        assert!(mailbox.pop().unwrap().is_yield());
        assert!(mailbox.pop().is_none());
    }
}
