// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Changes Processor
//!
//! Process changes after they are finished in order of addition.
//!
//! Based on TDLib's `ChangesProcessor` from `td/utils/ChangesProcessor.h`.
//!
//! # Overview
//!
//! The `ChangesProcessor` manages a sequence of pending changes and ensures
//! they are processed in the order they were added, once they are marked as
//! finished. This is useful for managing state updates that must be applied
//! sequentially even if they complete out of order.
//!
//! # Example
//!
//! ```rust
//! use rustgram_changes_processor::ChangesProcessor;
//!
//! let mut processor = ChangesProcessor::new();
//!
//! // Add pending changes
//! let id1 = processor.add(100);
//! let id2 = processor.add(200);
//! let id3 = processor.add(300);
//!
//! // Finish changes - they'll be processed in order
//! let mut processed = Vec::new();
//! processor.finish(id2, |value| processed.push(value));
//! assert_eq!(processed, &[]); // id1 not finished yet
//!
//! processor.finish(id1, |value| processed.push(value));
//! assert_eq!(processed, &[100, 200]); // Both id1 and id2 processed
//!
//! processor.finish(id3, |value| processed.push(value));
//! assert_eq!(processed, &[100, 200, 300]);
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::num::NonZeroU64;

/// Token type for tracking a change in the processor.
///
/// Each token uniquely identifies a pending change that was added to the processor.
pub type ChangesProcessorId = NonZeroU64;

/// Entry in the changes processor.
///
/// Contains the data value and a boolean indicating whether the change is finished.
#[derive(Debug, Clone)]
struct Entry<T> {
    /// The data value for this change.
    data: Option<T>,
    /// Whether this change has been finished.
    finished: bool,
}

/// Process changes after they are finished in order of addition.
///
/// # Type Parameters
///
/// * `T` - The type of data being processed
///
/// # Example
///
/// ```rust
/// use rustgram_changes_processor::ChangesProcessor;
///
/// let mut processor = ChangesProcessor::new();
///
/// let id1 = processor.add(10);
/// let id2 = processor.add(20);
///
/// // Finish in reverse order
/// let mut results = Vec::new();
/// processor.finish(id2, |v| results.push(v));
/// assert_eq!(results, []); // id1 not finished yet
///
/// processor.finish(id1, |v| results.push(v));
/// assert_eq!(results, [10, 20]); // Both processed in order
/// ```
#[derive(Debug, Clone)]
pub struct ChangesProcessor<T> {
    /// Offset for generating unique IDs.
    offset: u64,
    /// Index of the next entry to process.
    ready_index: usize,
    /// Pending entries.
    entries: Vec<Entry<T>>,
}

impl<T> Default for ChangesProcessor<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ChangesProcessor<T> {
    /// Creates a new `ChangesProcessor`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_changes_processor::ChangesProcessor;
    ///
    /// let processor: ChangesProcessor<i32> = ChangesProcessor::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            offset: 1,
            ready_index: 0,
            entries: Vec::new(),
        }
    }

    /// Clears all pending entries and resets the processor state.
    ///
    /// This preserves the ID generation offset, so new IDs will continue
    /// from where the previous entries left off.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_changes_processor::ChangesProcessor;
    ///
    /// let mut processor = ChangesProcessor::new();
    ///
    /// let id1 = processor.add(1);
    /// processor.finish(id1, |_| {});
    ///
    /// processor.clear();
    /// // Processor is now empty, ready for new entries
    /// ```
    pub fn clear(&mut self) {
        self.offset += self.entries.len() as u64;
        self.ready_index = 0;
        self.entries.clear();
    }

    /// Adds a new pending change.
    ///
    /// # Arguments
    ///
    /// * `data` - The data associated with this change
    ///
    /// # Returns
    ///
    /// A unique ID that can be used to finish this change later
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_changes_processor::ChangesProcessor;
    ///
    /// let mut processor = ChangesProcessor::new();
    ///
    /// let id = processor.add(42);
    /// // Store the ID to finish this change later
    /// ```
    pub fn add(&mut self, data: T) -> ChangesProcessorId {
        let id = self.offset + self.entries.len() as u64;
        self.entries.push(Entry {
            data: Some(data),
            finished: false,
        });

        // Safety: id >= offset >= 1, so it's never zero
        unsafe { NonZeroU64::new_unchecked(id) }
    }

    /// Marks a change as finished and processes any now-ready entries.
    ///
    /// When an entry is marked as finished, this method will process all
    /// consecutive finished entries starting from `ready_index`, calling
    /// the provided function for each one.
    ///
    /// # Arguments
    ///
    /// * `token` - The ID of the change to finish
    /// * `func` - Function to call for each entry that becomes ready
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_changes_processor::ChangesProcessor;
    ///
    /// let mut processor = ChangesProcessor::new();
    ///
    /// let id1 = processor.add(1);
    /// let id2 = processor.add(2);
    /// let id3 = processor.add(3);
    ///
    /// let mut results = Vec::new();
    ///
    /// // Finish id3 first - nothing processed yet
    /// processor.finish(id3, |v| results.push(v));
    /// assert_eq!(results, []);
    ///
    /// // Finish id2 - still nothing (id1 not finished)
    /// processor.finish(id2, |v| results.push(v));
    /// assert_eq!(results, []);
    ///
    /// // Finish id1 - all three processed in order
    /// processor.finish(id1, |v| results.push(v));
    /// assert_eq!(results, [1, 2, 3]);
    /// ```
    pub fn finish(&mut self, token: ChangesProcessorId, mut func: impl FnMut(T))
    where
        T: Clone,
    {
        let pos = token.get() as usize - self.offset as usize;

        if pos >= self.entries.len() {
            return;
        }

        // Mark the entry as finished (data stays in place until processed)
        self.entries[pos].finished = true;

        // Process consecutive finished entries starting from ready_index
        while self.ready_index < self.entries.len() && self.entries[self.ready_index].finished {
            // Take ownership of the data only when processing
            if let Some(data) = self.entries[self.ready_index].data.take() {
                func(data);
            }
            self.ready_index += 1;
        }

        self.try_compactify();
    }

    /// Returns the number of pending entries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_changes_processor::ChangesProcessor;
    ///
    /// let mut processor = ChangesProcessor::new();
    ///
    /// assert_eq!(processor.pending_count(), 0);
    ///
    /// processor.add(1);
    /// assert_eq!(processor.pending_count(), 1);
    /// ```
    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.entries
            .iter()
            .skip(self.ready_index)
            .filter(|e| !e.finished)
            .count()
    }

    /// Returns the total number of entries (including processed ones).
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.entries.len()
    }

    /// Checks if there are no pending entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
            || self.ready_index >= self.entries.len()
            || self
                .entries
                .iter()
                .skip(self.ready_index)
                .all(|e| e.finished)
    }

    /// Compacts the entries vector when possible to free memory.
    ///
    /// Compaction occurs when at least 5 entries are ready and more than
    /// half of all entries are ready. This balances memory usage with
    /// the cost of moving elements.
    fn try_compactify(&mut self) {
        if self.ready_index > 5 && self.ready_index * 2 > self.entries.len() {
            self.entries.drain(0..self.ready_index);
            self.offset += self.ready_index as u64;
            self.ready_index = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let processor: ChangesProcessor<i32> = ChangesProcessor::new();
        assert!(processor.is_empty());
        assert_eq!(processor.pending_count(), 0);
    }

    #[test]
    fn test_default() {
        let processor: ChangesProcessor<i32> = ChangesProcessor::default();
        assert!(processor.is_empty());
    }

    #[test]
    fn test_add() {
        let mut processor = ChangesProcessor::new();
        let id = processor.add(42);
        assert_eq!(id.get(), 1);
        assert_eq!(processor.pending_count(), 1);
        assert!(!processor.is_empty());
    }

    #[test]
    fn test_add_multiple() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add(1);
        let id2 = processor.add(2);
        let id3 = processor.add(3);

        assert_eq!(id1.get(), 1);
        assert_eq!(id2.get(), 2);
        assert_eq!(id3.get(), 3);
        assert_eq!(processor.pending_count(), 3);
    }

    #[test]
    fn test_finish_in_order() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add(1);
        let id2 = processor.add(2);
        let id3 = processor.add(3);

        let mut results = Vec::new();
        processor.finish(id1, |v| results.push(v));
        assert_eq!(results, &[1]);

        processor.finish(id2, |v| results.push(v));
        assert_eq!(results, &[1, 2]);

        processor.finish(id3, |v| results.push(v));
        assert_eq!(results, &[1, 2, 3]);
    }

    #[test]
    fn test_finish_out_of_order() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add(1);
        let id2 = processor.add(2);
        let id3 = processor.add(3);

        let mut results = Vec::new();

        // Finish in reverse order
        processor.finish(id3, |v| results.push(v));
        assert_eq!(results, &[]); // Nothing ready yet

        processor.finish(id2, |v| results.push(v));
        assert_eq!(results, &[]); // id1 still not finished

        processor.finish(id1, |v| results.push(v));
        assert_eq!(results, &[1, 2, 3]); // All processed in order
    }

    #[test]
    fn test_finish_middle_first() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add(1);
        let id2 = processor.add(2);
        let id3 = processor.add(3);

        let mut results = Vec::new();

        processor.finish(id2, |v| results.push(v));
        assert_eq!(results, &[]);

        processor.finish(id3, |v| results.push(v));
        assert_eq!(results, &[]); // id1 still blocking

        processor.finish(id1, |v| results.push(v));
        assert_eq!(results, &[1, 2, 3]);
    }

    #[test]
    fn test_clear() {
        let mut processor = ChangesProcessor::new();
        processor.add(1);
        processor.add(2);

        assert!(!processor.is_empty());

        processor.clear();
        assert!(processor.is_empty());
        assert_eq!(processor.pending_count(), 0);
    }

    #[test]
    fn test_clear_after_finish() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add(1);
        let id2 = processor.add(2);

        let mut results = Vec::new();
        processor.finish(id1, |v| results.push(v));
        processor.finish(id2, |v| results.push(v));

        assert_eq!(results, &[1, 2]);

        processor.clear();
        assert!(processor.is_empty());

        // New IDs should continue from after the cleared entries
        let id3 = processor.add(3);
        assert_eq!(id3.get(), 3); // IDs continue from 3
    }

    #[test]
    fn test_pending_count() {
        let mut processor = ChangesProcessor::new();
        assert_eq!(processor.pending_count(), 0);

        processor.add(1);
        assert_eq!(processor.pending_count(), 1);

        processor.add(2);
        processor.add(3);
        assert_eq!(processor.pending_count(), 3);

        let id1 = processor.add(1);
        let mut results = Vec::new();
        processor.finish(id1, |v| results.push(v));
        assert_eq!(processor.pending_count(), 3);
    }

    #[test]
    fn test_total_count() {
        let mut processor = ChangesProcessor::new();
        assert_eq!(processor.total_count(), 0);

        processor.add(1);
        assert_eq!(processor.total_count(), 1);

        processor.add(2);
        processor.add(3);
        assert_eq!(processor.total_count(), 3);
    }

    #[test]
    fn test_is_empty() {
        let mut processor = ChangesProcessor::new();
        assert!(processor.is_empty());

        let id1 = processor.add(1);
        assert!(!processor.is_empty());

        let mut results = Vec::new();
        processor.finish(id1, |v| results.push(v));
        assert!(processor.is_empty());
    }

    #[test]
    fn test_compactify() {
        let mut processor = ChangesProcessor::new();

        // Add many entries
        let mut ids = Vec::new();
        for i in 0..10 {
            ids.push(processor.add(i));
        }

        // Finish first 7 - should trigger compactification
        let mut results = Vec::new();
        for id in ids {
            processor.finish(id, |v| results.push(v));
        }

        assert_eq!(results, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert!(processor.is_empty());
    }

    #[test]
    fn test_large_values() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add(1_000_000);
        let id2 = processor.add(2_000_000);

        let mut results = Vec::new();
        processor.finish(id1, |v| results.push(v));
        processor.finish(id2, |v| results.push(v));

        assert_eq!(results, &[1_000_000, 2_000_000]);
    }

    #[test]
    fn test_string_data() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add("hello".to_string());
        let id2 = processor.add("world".to_string());

        let mut results = Vec::new();
        processor.finish(id2, |v| results.push(v));
        processor.finish(id1, |v| results.push(v));

        assert_eq!(results, &["hello", "world"]);
    }

    #[test]
    fn test_after_clear_ids_continue() {
        let mut processor = ChangesProcessor::new();
        let _ = processor.add(1);
        let _ = processor.add(2);

        processor.clear();

        let id3 = processor.add(3);
        assert_eq!(id3.get(), 3); // IDs continue from where we left off
    }

    #[test]
    fn test_multiple_clears() {
        let mut processor = ChangesProcessor::new();

        for _ in 0..3 {
            processor.add(1);
            processor.clear();
        }

        let id = processor.add(42);
        assert_eq!(id.get(), 4);
    }

    #[test]
    fn test_finish_invalid_id() {
        let mut processor = ChangesProcessor::new();
        let id1 = processor.add(1);

        // Try to finish an ID that doesn't exist
        let invalid_id = unsafe { NonZeroU64::new_unchecked(999) };
        let mut results = Vec::new();
        processor.finish(invalid_id, |v| results.push(v));

        assert_eq!(results, &[]);

        // Original ID still works
        processor.finish(id1, |v| results.push(v));
        assert_eq!(results, &[1]);
    }

    #[test]
    fn test_clone() {
        let mut processor1 = ChangesProcessor::new();
        let id1 = processor1.add(1);
        let id2 = processor1.add(2);

        let mut processor2 = processor1.clone();

        let mut results1 = Vec::new();
        let mut results2 = Vec::new();

        processor1.finish(id1, |v| results1.push(v));
        processor2.finish(id2, |v| results2.push(v));

        // Each clone maintains its own state
        assert_eq!(results1, &[1]);
        assert_eq!(results2, &[]);
    }

    #[test]
    fn test_empty_processor_finish() {
        let mut processor: ChangesProcessor<i32> = ChangesProcessor::new();
        let mut results = Vec::new();

        // Finishing on an empty processor should do nothing
        let id = processor.add(1);
        processor.finish(id, |v| results.push(v));
        assert_eq!(results, &[1]);
    }
}
