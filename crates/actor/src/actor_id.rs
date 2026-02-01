// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Type-safe actor identifiers.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// A type-safe identifier for an actor.
///
/// `ActorId` provides compile-time type safety by using a phantom type parameter
/// to track which actor type this ID refers to. This prevents sending messages
/// to the wrong actor type at compile time.
///
/// # Type Parameters
///
/// * `T` - The actor type this ID refers to
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::ActorId;
///
/// struct MyActor;
///
/// let id = ActorId::<MyActor>::new(123, 1, 2);
/// assert_eq!(id.as_u64(), 123);
/// assert_eq!(id.scheduler_id(), 1);
/// ```
#[derive(Debug, Copy)]
pub struct ActorId<T> {
    /// The unique numeric ID of the actor.
    id: u64,
    /// The scheduler ID this actor is assigned to.
    scheduler_id: u32,
    /// The generation counter for detecting stale references.
    generation: u32,
    /// Phantom data for the actor type (uses *const T for !Send/Safe).
    _phantom: PhantomData<*const T>,
}

// Manual implementations to avoid requiring T: Trait bounds
impl<T> Clone for ActorId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for ActorId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.scheduler_id == other.scheduler_id
            && self.generation == other.generation
    }
}

impl<T> Eq for ActorId<T> {}

impl<T> Hash for ActorId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.scheduler_id.hash(state);
        self.generation.hash(state);
    }
}

impl<T> PartialOrd for ActorId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for ActorId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.id.cmp(&other.id) {
            std::cmp::Ordering::Equal => match self.scheduler_id.cmp(&other.scheduler_id) {
                std::cmp::Ordering::Equal => self.generation.cmp(&other.generation),
                other => other,
            },
            other => other,
        }
    }
}

impl<T> Default for ActorId<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T> fmt::Display for ActorId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ActorId(id={}, scheduler={}, gen={})",
            self.id, self.scheduler_id, self.generation
        )
    }
}

impl<T> ActorId<T> {
    /// Creates a new `ActorId` with the given components.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique numeric ID
    /// * `scheduler_id` - The scheduler ID this actor is assigned to
    /// * `generation` - The generation counter for detecting stale references
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorId;
    ///
    /// struct MyActor;
    /// let id = ActorId::<MyActor>::new(100, 2, 3);
    /// assert_eq!(id.as_u64(), 100);
    /// ```
    pub fn new(id: u64, scheduler_id: u32, generation: u32) -> Self {
        Self {
            id,
            scheduler_id,
            generation,
            _phantom: PhantomData,
        }
    }

    /// Creates a zero (invalid) `ActorId`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorId;
    ///
    /// struct MyActor;
    /// let id = ActorId::<MyActor>::zero();
    /// assert!(id.is_zero());
    /// ```
    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// Returns the numeric ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorId;
    ///
    /// struct MyActor;
    /// let id = ActorId::<MyActor>::new(42, 0, 0);
    /// assert_eq!(id.as_u64(), 42);
    /// ```
    pub fn as_u64(&self) -> u64 {
        self.id
    }

    /// Returns the scheduler ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorId;
    ///
    /// struct MyActor;
    /// let id = ActorId::<MyActor>::new(0, 5, 0);
    /// assert_eq!(id.scheduler_id(), 5);
    /// ```
    pub fn scheduler_id(&self) -> u32 {
        self.scheduler_id
    }

    /// Returns the generation counter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorId;
    ///
    /// struct MyActor;
    /// let id = ActorId::<MyActor>::new(0, 0, 7);
    /// assert_eq!(id.generation(), 7);
    /// ```
    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// Checks if this is a zero (invalid) ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorId;
    ///
    /// struct MyActor;
    /// assert!(ActorId::<MyActor>::zero().is_zero());
    /// assert!(!ActorId::<MyActor>::new(1, 0, 0).is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.id == 0 && self.scheduler_id == 0 && self.generation == 0
    }

    /// Converts this `ActorId<T>` to an `ActorId<()>` for type-erased storage.
    ///
    /// This is useful when storing actor IDs in a generic container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::ActorId;
    ///
    /// struct MyActor;
    /// let typed = ActorId::<MyActor>::new(123, 1, 2);
    /// let erased = typed.erase_type();
    /// assert_eq!(erased.as_u64(), 123);
    /// ```
    pub fn erase_type(self) -> ActorId<()> {
        ActorId::new(self.id, self.scheduler_id, self.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    struct TestActor;

    #[test]
    fn test_actor_id_new() {
        let id = ActorId::<TestActor>::new(100, 2, 3);
        assert_eq!(id.as_u64(), 100);
        assert_eq!(id.scheduler_id(), 2);
        assert_eq!(id.generation(), 3);
    }

    #[test]
    fn test_actor_id_zero() {
        let id = ActorId::<TestActor>::zero();
        assert!(id.is_zero());
        assert_eq!(id.as_u64(), 0);
        assert_eq!(id.scheduler_id(), 0);
        assert_eq!(id.generation(), 0);
    }

    #[test]
    fn test_actor_id_is_zero() {
        assert!(ActorId::<TestActor>::zero().is_zero());
        assert!(!ActorId::<TestActor>::new(1, 0, 0).is_zero());
        assert!(!ActorId::<TestActor>::new(0, 1, 0).is_zero());
        assert!(!ActorId::<TestActor>::new(0, 0, 1).is_zero());
    }

    #[test]
    fn test_actor_id_copy() {
        let id1 = ActorId::<TestActor>::new(42, 1, 2);
        let id2 = id1;
        // ActorId is Copy, so both should be valid
        assert_eq!(id1.as_u64(), 42);
        assert_eq!(id2.as_u64(), 42);
    }

    #[test]
    fn test_actor_id_clone() {
        let id1 = ActorId::<TestActor>::new(42, 1, 2);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_actor_id_equality() {
        let id1 = ActorId::<TestActor>::new(100, 2, 3);
        let id2 = ActorId::<TestActor>::new(100, 2, 3);
        let id3 = ActorId::<TestActor>::new(101, 2, 3);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_actor_id_ordering() {
        let id1 = ActorId::<TestActor>::new(100, 2, 3);
        let id2 = ActorId::<TestActor>::new(100, 3, 3);
        let id3 = ActorId::<TestActor>::new(101, 2, 3);
        assert!(id1 < id2);
        assert!(id2 < id3);
    }

    #[test]
    fn test_actor_id_hash() {
        let mut set = HashSet::new();
        set.insert(ActorId::<TestActor>::new(1, 0, 0));
        set.insert(ActorId::<TestActor>::new(2, 0, 0));
        set.insert(ActorId::<TestActor>::new(1, 0, 0));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_actor_id_display() {
        let id = ActorId::<TestActor>::new(123, 4, 5);
        let display = format!("{}", id);
        assert!(display.contains("123"));
        assert!(display.contains("4"));
        assert!(display.contains("5"));
    }

    #[test]
    fn test_actor_id_debug() {
        let id = ActorId::<TestActor>::new(123, 4, 5);
        let debug = format!("{:?}", id);
        assert!(debug.contains("123"));
    }

    #[test]
    fn test_actor_id_default() {
        let id = ActorId::<TestActor>::default();
        assert!(id.is_zero());
    }

    #[test]
    fn test_actor_id_erase_type() {
        let typed = ActorId::<TestActor>::new(123, 4, 5);
        let erased = typed.erase_type();
        assert_eq!(erased.as_u64(), 123);
        assert_eq!(erased.scheduler_id(), 4);
        assert_eq!(erased.generation(), 5);
    }

    #[test]
    fn test_multiple_actor_types() {
        struct ActorA;
        struct ActorB;

        let id_a = ActorId::<ActorA>::new(100, 1, 0);
        let id_b = ActorId::<ActorB>::new(200, 2, 0);

        assert_eq!(id_a.as_u64(), 100);
        assert_eq!(id_b.as_u64(), 200);
        // Different types can't be compared directly
    }

    // Additional edge case tests
    #[test]
    fn test_actor_id_max_values() {
        let id = ActorId::<TestActor>::new(u64::MAX, u32::MAX, u32::MAX);
        assert_eq!(id.as_u64(), u64::MAX);
        assert_eq!(id.scheduler_id(), u32::MAX);
        assert_eq!(id.generation(), u32::MAX);
    }

    #[test]
    fn test_actor_id_partial_ord() {
        let id1 = ActorId::<TestActor>::new(100, 2, 3);
        let id2 = ActorId::<TestActor>::new(100, 2, 3);
        assert_eq!(id1.partial_cmp(&id2), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn test_actor_id_ord() {
        let id1 = ActorId::<TestActor>::new(100, 2, 3);
        let id2 = ActorId::<TestActor>::new(100, 2, 4);
        assert!(id1 < id2);
    }

    #[test]
    fn test_actor_id_comparison_by_id() {
        let id1 = ActorId::<TestActor>::new(99, 5, 5);
        let id2 = ActorId::<TestActor>::new(100, 0, 0);
        assert!(id1 < id2);
    }

    #[test]
    fn test_actor_id_comparison_by_scheduler() {
        let id1 = ActorId::<TestActor>::new(100, 1, 0);
        let id2 = ActorId::<TestActor>::new(100, 2, 0);
        assert!(id1 < id2);
    }

    #[test]
    fn test_actor_id_comparison_by_generation() {
        let id1 = ActorId::<TestActor>::new(100, 2, 1);
        let id2 = ActorId::<TestActor>::new(100, 2, 2);
        assert!(id1 < id2);
    }

    #[test]
    fn test_actor_id_hash_consistency() {
        let id1 = ActorId::<TestActor>::new(100, 2, 3);
        let id2 = ActorId::<TestActor>::new(100, 2, 3);

        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher1 = DefaultHasher::new();
        id1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        id2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_actor_id_hash_different_for_different_ids() {
        let id1 = ActorId::<TestActor>::new(100, 2, 3);
        let id2 = ActorId::<TestActor>::new(101, 2, 3);

        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher1 = DefaultHasher::new();
        id1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        id2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_actor_id_erase_type_preserves_values() {
        struct TypedActor;
        impl Actor for TypedActor {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let typed = ActorId::<TypedActor>::new(12345, 67, 89);
        let erased = typed.erase_type();

        assert_eq!(erased.as_u64(), 12345);
        assert_eq!(erased.scheduler_id(), 67);
        assert_eq!(erased.generation(), 89);
    }
}
