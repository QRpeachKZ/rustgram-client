// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Actor registry for actor lookup and management.

use crate::actor_info::ActorInfo;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// A registry of all actors in the system.
///
/// The `Registry` provides a centralized location for looking up actors
/// by their ID, and is shared across all schedulers via `Arc`.
///
/// # Examples
///
/// ```rust
/// use rustgram_actor::{Registry, ActorInfo};
///
/// let registry = Registry::new();
/// let info = ActorInfo::new("test_actor".to_string(), 0);
/// registry.insert(123, info);
/// assert!(registry.contains(123));
/// ```
#[derive(Debug, Default)]
pub struct Registry {
    /// Map of actor ID to actor info.
    actors: RwLock<HashMap<u64, ActorInfo>>,
}

impl Registry {
    /// Creates a new empty registry.
    ///
    /// # Examples
    ///
    /// ```rust
/// /// use rustgram_actor::Registry;
    ///
    /// let registry = Registry::new();
    /// assert_eq!(registry.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an actor into the registry.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID
    /// * `info` - The actor info
    ///
    /// # Returns
    ///
    /// * `Some(ActorInfo)` - If an actor with this ID already existed
    /// * `None` - If this is a new actor
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// let info = ActorInfo::new("test".to_string(), 0);
    ///
    /// assert!(registry.insert(123, info).is_none());
    /// assert!(registry.contains(123));
    /// ```
    pub fn insert(&self, id: u64, info: ActorInfo) -> Option<ActorInfo> {
        let mut actors = self.actors.write();
        actors.insert(id, info)
    }

    /// Removes an actor from the registry.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID to remove
    ///
    /// # Returns
    ///
    /// * `Some(ActorInfo)` - If the actor existed
    /// * `None` - If the actor was not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// let info = ActorInfo::new("test".to_string(), 0);
    /// registry.insert(123, info);
    ///
    /// assert!(registry.remove(123).is_some());
    /// assert!(!registry.contains(123));
    /// ```
    pub fn remove(&self, id: u64) -> Option<ActorInfo> {
        let mut actors = self.actors.write();
        actors.remove(&id)
    }

    /// Gets a copy of the actor info for the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID
    ///
    /// # Returns
    ///
    /// * `Some(ActorInfo)` - If the actor exists
    /// * `None` - If the actor was not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// let info = ActorInfo::new("test".to_string(), 1);
    /// registry.insert(42, info.clone());
    ///
    /// let retrieved = registry.get(42);
    /// assert!(retrieved.is_some());
    /// assert_eq!(retrieved.unwrap().scheduler_id(), 1);
    /// ```
    pub fn get(&self, id: u64) -> Option<ActorInfo> {
        let actors = self.actors.read();
        actors.get(&id).cloned()
    }

    /// Checks if an actor exists in the registry.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID
    ///
    /// # Returns
    ///
    /// * `true` - If the actor exists
    /// * `false` - If the actor was not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// assert!(!registry.contains(123));
    ///
    /// let info = ActorInfo::new("test".to_string(), 0);
    /// registry.insert(123, info);
    /// assert!(registry.contains(123));
    /// ```
    pub fn contains(&self, id: u64) -> bool {
        let actors = self.actors.read();
        actors.contains_key(&id)
    }

    /// Returns the number of actors in the registry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// assert_eq!(registry.len(), 0);
    ///
    /// registry.insert(1, ActorInfo::new("a".to_string(), 0));
    /// registry.insert(2, ActorInfo::new("b".to_string(), 0));
    /// assert_eq!(registry.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        let actors = self.actors.read();
        actors.len()
    }

    /// Checks if the registry is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// assert!(registry.is_empty());
    ///
    /// registry.insert(1, ActorInfo::new("a".to_string(), 0));
    /// assert!(!registry.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        let actors = self.actors.read();
        actors.is_empty()
    }

    /// Clears all actors from the registry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// registry.insert(1, ActorInfo::new("a".to_string(), 0));
    /// registry.insert(2, ActorInfo::new("b".to_string(), 0));
    /// assert_eq!(registry.len(), 2);
    ///
    /// registry.clear();
    /// assert!(registry.is_empty());
    /// ```
    pub fn clear(&self) {
        let mut actors = self.actors.write();
        actors.clear();
    }

    /// Updates actor info in the registry.
    ///
    /// # Arguments
    ///
    /// * `id` - The actor ID
    /// * `f` - A function that takes the current info and returns the new info
    ///
    /// # Returns
    ///
    /// * `Some(())` - If the actor existed and was updated
    /// * `None` - If the actor was not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo, ActorState};
    ///
    /// let registry = Registry::new();
    /// let info = ActorInfo::new("test".to_string(), 0);
    /// registry.insert(123, info);
    ///
    /// registry.update(123, |mut info| {
    ///     info.set_state(ActorState::Running);
    ///     info
    /// });
    ///
    /// let updated = registry.get(123).unwrap();
    /// assert_eq!(updated.state(), ActorState::Running);
    /// ```
    pub fn update<F>(&self, id: u64, f: F) -> Option<()>
    where
        F: FnOnce(ActorInfo) -> ActorInfo,
    {
        let mut actors = self.actors.write();
        let info = actors.get(&id)?;
        let new_info = f(info.clone());
        actors.insert(id, new_info);
        Some(())
    }

    /// Gets all actor IDs in the registry.
    ///
    /// # Returns
    ///
    /// A vector of all actor IDs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::{Registry, ActorInfo};
    ///
    /// let registry = Registry::new();
    /// registry.insert(1, ActorInfo::new("a".to_string(), 0));
    /// registry.insert(2, ActorInfo::new("b".to_string(), 0));
    ///
    /// let ids = registry.ids();
    /// assert_eq!(ids.len(), 2);
    /// assert!(ids.contains(&1));
    /// assert!(ids.contains(&2));
    /// ```
    pub fn ids(&self) -> Vec<u64> {
        let actors = self.actors.read();
        actors.keys().copied().collect()
    }

    /// Converts the registry into an Arc for sharing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_actor::Registry;
    /// use std::sync::Arc;
    ///
    /// let registry = Registry::new();
    /// let shared: Arc<Registry> = registry.into_arc();
    /// ```
    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

/// A shared reference to a registry.
///
/// This is a type alias for convenience when using Arc<Registry>.
pub type SharedRegistry = Arc<Registry>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor_info::ActorState;

    #[test]
    fn test_registry_new() {
        let registry = Registry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_insert() {
        let registry = Registry::new();
        let info = ActorInfo::new("test".to_string(), 0);

        assert!(registry.insert(123, info).is_none());
        assert!(registry.contains(123));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_insert_overwrite() {
        let registry = Registry::new();
        let info1 = ActorInfo::new("test1".to_string(), 0);
        let info2 = ActorInfo::new("test2".to_string(), 1);

        assert!(registry.insert(123, info1).is_none());
        let prev = registry.insert(123, info2);
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().name(), "test1");
    }

    #[test]
    fn test_registry_remove() {
        let registry = Registry::new();
        let info = ActorInfo::new("test".to_string(), 0);
        registry.insert(123, info);

        assert!(registry.remove(123).is_some());
        assert!(!registry.contains(123));
        assert!(registry.remove(123).is_none());
    }

    #[test]
    fn test_registry_get() {
        let registry = Registry::new();
        let info = ActorInfo::new("test".to_string(), 1);
        registry.insert(42, info.clone());

        let retrieved = registry.get(42);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().scheduler_id(), 1);
    }

    #[test]
    fn test_registry_get_missing() {
        let registry = Registry::new();
        assert!(registry.get(999).is_none());
    }

    #[test]
    fn test_registry_contains() {
        let registry = Registry::new();
        let info = ActorInfo::new("test".to_string(), 0);
        assert!(!registry.contains(123));

        registry.insert(123, info);
        assert!(registry.contains(123));
    }

    #[test]
    fn test_registry_len() {
        let registry = Registry::new();
        assert_eq!(registry.len(), 0);

        for i in 0..10 {
            let info = ActorInfo::new(format!("actor_{}", i), 0);
            registry.insert(i, info);
        }
        assert_eq!(registry.len(), 10);
    }

    #[test]
    fn test_registry_is_empty() {
        let registry = Registry::new();
        assert!(registry.is_empty());

        let info = ActorInfo::new("test".to_string(), 0);
        registry.insert(1, info);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_registry_clear() {
        let registry = Registry::new();
        for i in 0..5 {
            let info = ActorInfo::new(format!("actor_{}", i), 0);
            registry.insert(i, info);
        }
        assert_eq!(registry.len(), 5);

        registry.clear();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_update() {
        let registry = Registry::new();
        let info = ActorInfo::new("test".to_string(), 0);
        registry.insert(123, info);

        let result = registry.update(123, |mut info| {
            info.set_state(ActorState::Running);
            info
        });
        assert!(result.is_some());

        let updated = registry.get(123).unwrap();
        assert_eq!(updated.state(), ActorState::Running);
    }

    #[test]
    fn test_registry_update_missing() {
        let registry = Registry::new();
        let result = registry.update(999, |info| info);
        assert!(result.is_none());
    }

    #[test]
    fn test_registry_ids() {
        let registry = Registry::new();
        registry.insert(1, ActorInfo::new("a".to_string(), 0));
        registry.insert(2, ActorInfo::new("b".to_string(), 0));
        registry.insert(3, ActorInfo::new("c".to_string(), 0));

        let ids = registry.ids();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
    }

    #[test]
    fn test_registry_into_arc() {
        let registry = Registry::new();
        let shared: SharedRegistry = registry.into_arc();
        assert!(shared.is_empty());
    }

    #[test]
    fn test_registry_concurrent_access() {
        let registry = Arc::new(Registry::new());
        let mut handles = vec![];

        // Spawn multiple threads that insert actors
        for i in 0..10 {
            let registry_clone = Arc::clone(&registry);
            let handle = std::thread::spawn(move || {
                let info = ActorInfo::new(format!("actor_{}", i), i as u32);
                registry_clone.insert(i as u64, info);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(registry.len(), 10);
    }
}
