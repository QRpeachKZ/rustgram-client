// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Enhanced network query dispatcher.
//!
//! This module implements an enhanced query dispatcher with better routing and error handling.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use tokio::time::Duration;

use crate::dc::{DcId, DcOptionsSet};
use crate::query::NetQuery;
use crate::session::SessionConnection;

/// Dispatch configuration.
#[derive(Debug, Clone, Copy)]
pub struct DispatchConfig {
    /// Maximum queries in flight per DC
    pub max_in_flight: usize,

    /// Query timeout
    pub query_timeout: Duration,

    /// Whether to automatically retry on 503 errors
    pub retry_on_503: bool,

    /// Maximum retries per query
    pub max_retries: u32,
}

impl Default for DispatchConfig {
    fn default() -> Self {
        Self {
            max_in_flight: 1024,
            query_timeout: Duration::from_secs(60),
            retry_on_503: true,
            max_retries: 3,
        }
    }
}

/// Session provider trait.
///
/// Allows the dispatcher to get or create sessions for specific DCs.
pub trait SessionProvider: Send + Sync {
    /// Gets or creates a session for the specified DC.
    fn get_session(&self, dc_id: DcId) -> Option<Arc<SessionConnection>>;
}

/// Enhanced network query dispatcher.
///
/// Routes queries to appropriate sessions with enhanced features.
pub struct EnhancedDispatcher {
    /// Configuration
    config: DispatchConfig,

    /// Main DC ID
    main_dc_id: AtomicI32,

    /// DC options
    dc_options: Arc<Mutex<DcOptionsSet>>,

    /// Session provider
    session_provider: Option<Box<dyn SessionProvider>>,

    /// Queries in flight per DC
    in_flight: Arc<Mutex<HashMap<i32, usize>>>,

    /// Pending queries (dc_id -> Vec<NetQuery>)
    pending: Arc<Mutex<HashMap<i32, Vec<NetQuery>>>>,

    /// Stop flag
    stopped: AtomicBool,
}

impl EnhancedDispatcher {
    /// Creates a new enhanced dispatcher.
    pub fn new(config: DispatchConfig) -> Self {
        Self {
            config,
            main_dc_id: AtomicI32::new(2),
            dc_options: Arc::new(Mutex::new(DcOptionsSet::new())),
            session_provider: None,
            in_flight: Arc::new(Mutex::new(HashMap::new())),
            pending: Arc::new(Mutex::new(HashMap::new())),
            stopped: AtomicBool::new(false),
        }
    }

    /// Returns the main DC ID.
    pub fn main_dc_id(&self) -> DcId {
        DcId::internal(self.main_dc_id.load(Ordering::Relaxed))
    }

    /// Sets the main DC ID.
    pub fn set_main_dc_id(&self, dc_id: DcId) {
        self.main_dc_id.store(dc_id.get_raw_id(), Ordering::Relaxed);
    }

    /// Returns the DC options.
    pub fn dc_options(&self) -> DcOptionsSet {
        self.dc_options.lock().clone()
    }

    /// Sets the DC options.
    pub fn set_dc_options(&self, options: DcOptionsSet) {
        *self.dc_options.lock() = options;
    }

    /// Sets the session provider.
    pub fn set_session_provider(&mut self, provider: Box<dyn SessionProvider>) {
        self.session_provider = Some(provider);
    }

    /// Dispatches a query to the appropriate session.
    pub fn dispatch(&self, query: NetQuery) -> Result<(), crate::query::QueryError> {
        if self.stopped.load(Ordering::Relaxed) {
            return Err(crate::query::QueryError::Generic(
                "Dispatcher stopped".into(),
            ));
        }

        let dc_id = query.dc_id();

        // Check if we have capacity
        let dc_raw = dc_id.get_raw_id();
        let mut in_flight = self.in_flight.lock();

        let current_count = *in_flight.get(&dc_raw).unwrap_or(&0);

        if current_count >= self.config.max_in_flight {
            // Add to pending
            drop(in_flight);
            self.pending
                .lock()
                .entry(dc_raw)
                .or_default()
                .push(query);
            return Ok(());
        }

        // Increment in-flight count
        *in_flight.entry(dc_raw).or_insert(0) += 1;
        drop(in_flight);

        // Get session and send query
        if let Some(provider) = &self.session_provider {
            if let Some(session) = provider.get_session(dc_id) {
                let _ = session.send_query(query);
                return Ok(());
            }
        }

        // No session available, mark as failed
        self.release_slot(dc_raw);
        Err(crate::query::QueryError::Generic(
            "No session available".into(),
        ))
    }

    /// Releases a slot for a DC.
    pub fn release_slot(&self, dc_raw: i32) {
        let mut in_flight = self.in_flight.lock();

        if let Some(count) = in_flight.get_mut(&dc_raw) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    /// Processes pending queries for a DC.
    pub fn process_pending(&self, dc_raw: i32) {
        let mut pending = self.pending.lock();

        if let Some(queries) = pending.remove(&dc_raw) {
            drop(pending);

            for query in queries {
                let _ = self.dispatch(query);
            }
        }
    }

    /// Returns the number of in-flight queries for a DC.
    pub fn in_flight_count(&self, dc_id: DcId) -> usize {
        *self.in_flight.lock().get(&dc_id.get_raw_id()).unwrap_or(&0)
    }

    /// Returns the number of pending queries for a DC.
    pub fn pending_count(&self, dc_id: DcId) -> usize {
        self.pending
            .lock()
            .get(&dc_id.get_raw_id())
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Stops the dispatcher.
    pub fn stop(&self) {
        self.stopped.store(true, Ordering::Relaxed);
    }

    /// Returns `true` if the dispatcher is stopped.
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(Ordering::Relaxed)
    }

    /// Processes a query result.
    pub fn on_result(&self, query: NetQuery) {
        let dc_id = query.dc_id();
        let dc_raw = dc_id.get_raw_id();

        self.release_slot(dc_raw);

        // If the query can be resent, dispatch it again
        if query.is_error() {
            let error = query.error();

            if error.is_resend() || (error.code() == 500 && self.config.retry_on_503) {
                let _ = self.dispatch(query);
                return;
            }
        }

        // Try to process pending queries
        self.process_pending(dc_raw);
    }
}

impl Default for EnhancedDispatcher {
    fn default() -> Self {
        Self::new(DispatchConfig::default())
    }
}

/// Simple session provider using a map of sessions.
pub struct MapSessionProvider {
    sessions: Arc<Mutex<HashMap<i32, Arc<SessionConnection>>>>,
}

impl MapSessionProvider {
    /// Creates a new map session provider.
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a session for a DC.
    pub fn add_session(&self, dc_id: DcId, session: Arc<SessionConnection>) {
        self.sessions.lock().insert(dc_id.get_raw_id(), session);
    }

    /// Removes a session for a DC.
    pub fn remove_session(&self, dc_id: DcId) -> Option<Arc<SessionConnection>> {
        self.sessions.lock().remove(&dc_id.get_raw_id())
    }
}

impl Default for MapSessionProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionProvider for MapSessionProvider {
    fn get_session(&self, dc_id: DcId) -> Option<Arc<SessionConnection>> {
        self.sessions.lock().get(&dc_id.get_raw_id()).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_config_default() {
        let config = DispatchConfig::default();
        assert_eq!(config.max_in_flight, 1024);
        assert_eq!(config.query_timeout, Duration::from_secs(60));
        assert!(config.retry_on_503);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_enhanced_dispatcher_new() {
        let dispatcher = EnhancedDispatcher::new(DispatchConfig::default());

        assert_eq!(dispatcher.main_dc_id(), DcId::internal(2));
        assert!(!dispatcher.is_stopped());
    }

    #[test]
    fn test_enhanced_dispatcher_set_main_dc() {
        let dispatcher = EnhancedDispatcher::new(DispatchConfig::default());

        dispatcher.set_main_dc_id(DcId::internal(4));
        assert_eq!(dispatcher.main_dc_id(), DcId::internal(4));
    }

    #[test]
    fn test_enhanced_dispatcher_stop() {
        let dispatcher = EnhancedDispatcher::new(DispatchConfig::default());

        dispatcher.stop();
        assert!(dispatcher.is_stopped());
    }

    #[test]
    fn test_map_session_provider() {
        let provider = MapSessionProvider::new();

        assert!(provider.get_session(DcId::internal(2)).is_none());

        // Note: Can't easily create a SessionConnection without more setup
        // This is just to verify the API exists
    }

    #[test]
    fn test_enhanced_dispatcher_in_flight_count() {
        let dispatcher = EnhancedDispatcher::new(DispatchConfig::default());

        assert_eq!(dispatcher.in_flight_count(DcId::internal(2)), 0);
        assert_eq!(dispatcher.pending_count(DcId::internal(2)), 0);
    }
}
