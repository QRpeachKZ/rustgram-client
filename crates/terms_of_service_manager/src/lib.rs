// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Terms of Service Manager
//!
//! Manages terms of service acceptance for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`TermsOfServiceManager`], which handles
//! terms of service acceptance tracking and periodic fetching.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_terms_of_service_manager::TermsOfServiceManager;
//! use rustgram_terms_of_service::TermsOfService;
//!
//! let manager = TermsOfServiceManager::new();
//! manager.init();
//! ```

use std::sync::Arc;

use tokio::sync::RwLock;

use rustgram_terms_of_service::TermsOfService;

/// Manager for terms of service.
///
/// Tracks the current terms of service and acceptance state.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `TermsOfServiceManager` class in `TermsOfServiceManager.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_terms_of_service_manager::TermsOfServiceManager;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = TermsOfServiceManager::new();
/// manager.init().await;
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TermsOfServiceManager {
    /// Inner state protected by RwLock for concurrent access.
    inner: Arc<RwLock<TermsOfServiceManagerInner>>,
}

impl TermsOfServiceManager {
    /// Creates a new terms of service manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    ///
    /// let manager = TermsOfServiceManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TermsOfServiceManagerInner::new())),
        }
    }

    /// Initializes the manager.
    ///
    /// Marks the manager as initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = TermsOfServiceManager::new();
    /// manager.init().await;
    /// assert!(manager.is_inited());
    /// # }
    /// ```
    pub async fn init(&self) {
        let mut inner = self.inner.write().await;
        inner.is_inited = true;
    }

    /// Accepts the terms of service.
    ///
    /// # Arguments
    ///
    /// * `terms_id` - The ID of the terms to accept
    ///
    /// # Returns
    ///
    /// `Ok(())` if accepted successfully, `Err` if the terms ID doesn't match pending terms
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = TermsOfServiceManager::new();
    /// manager.set_pending_terms(TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true));
    /// manager.accept_terms_of_service("tos_v1").await.unwrap();
    /// # }
    /// ```
    pub async fn accept_terms_of_service(&self, terms_id: &str) -> Result<(), AcceptError> {
        let mut inner = self.inner.write().await;
        if let Some(pending) = &inner.pending_terms_of_service {
            if pending.id() == terms_id {
                inner.pending_terms_of_service = None;
                return Ok(());
            }
        }
        Err(AcceptError::TermsIdMismatch {
            expected: inner
                .pending_terms_of_service
                .as_ref()
                .map(|t| t.id().to_string()),
            got: terms_id.to_string(),
        })
    }

    /// Sets the pending terms of service.
    ///
    /// # Arguments
    ///
    /// * `terms` - The pending terms of service
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = TermsOfServiceManager::new();
    /// manager.set_pending_terms(TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true));
    /// # }
    /// ```
    pub async fn set_pending_terms(&self, terms: TermsOfService) {
        let mut inner = self.inner.write().await;
        inner.pending_terms_of_service = Some(terms);
    }

    /// Returns the pending terms of service.
    ///
    /// # Returns
    ///
    /// `Some(TermsOfService)` if there are pending terms, `None` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = TermsOfServiceManager::new();
    /// let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
    /// manager.set_pending_terms(terms.clone());
    /// let pending = manager.get_pending_terms();
    /// assert_eq!(pending, Some(terms));
    /// # }
    /// ```
    #[must_use]
    pub async fn get_pending_terms(&self) -> Option<TermsOfService> {
        let inner = self.inner.read().await;
        inner.pending_terms_of_service.clone()
    }

    /// Checks if the manager has been initialized.
    ///
    /// # Returns
    ///
    /// `true` if initialized, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = TermsOfServiceManager::new();
    /// assert!(!manager.is_inited());
    /// manager.init().await;
    /// assert!(manager.is_inited());
    /// # }
    /// ```
    #[must_use]
    pub async fn is_inited(&self) -> bool {
        let inner = self.inner.read().await;
        inner.is_inited
    }

    /// Checks if there are pending terms of service.
    ///
    /// # Returns
    ///
    /// `true` if there are pending terms, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = TermsOfServiceManager::new();
    /// assert!(!manager.has_pending_terms());
    /// manager.set_pending_terms(TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true));
    /// assert!(manager.has_pending_terms());
    /// # }
    /// ```
    #[must_use]
    pub async fn has_pending_terms(&self) -> bool {
        let inner = self.inner.read().await;
        inner.pending_terms_of_service.is_some()
    }

    /// Clears all pending terms of service.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service_manager::TermsOfServiceManager;
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = TermsOfServiceManager::new();
    /// manager.set_pending_terms(TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true));
    /// assert!(manager.has_pending_terms());
    /// manager.clear_pending_terms();
    /// assert!(!manager.has_pending_terms());
    /// # }
    /// ```
    pub async fn clear_pending_terms(&self) {
        let mut inner = self.inner.write().await;
        inner.pending_terms_of_service = None;
    }
}

impl Default for TermsOfServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Inner state of the terms of service manager.
#[derive(Debug, Default)]
struct TermsOfServiceManagerInner {
    /// Pending terms of service.
    pending_terms_of_service: Option<TermsOfService>,

    /// Whether the manager has been initialized.
    is_inited: bool,
}

impl TermsOfServiceManagerInner {
    /// Creates a new inner state.
    #[must_use]
    fn new() -> Self {
        Self::default()
    }
}

/// Error accepting terms of service.
///
/// # Example
///
/// ```rust
/// use rustgram_terms_of_service_manager::AcceptError;
///
/// let error = AcceptError::TermsIdMismatch {
///     expected: Some("tos_v1".to_string()),
///     got: "tos_v2".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AcceptError {
    /// The terms ID doesn't match the pending terms.
    #[error("Terms ID mismatch: expected {expected:?}, got {got}")]
    TermsIdMismatch {
        /// The expected terms ID (if any pending terms exist).
        expected: Option<String>,

        /// The terms ID that was provided.
        got: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== new Tests ==========

    #[tokio::test]
    async fn test_new() {
        let manager = TermsOfServiceManager::new();
        assert!(!manager.is_inited().await);
        assert!(!manager.has_pending_terms().await);
    }

    #[tokio::test]
    async fn test_default() {
        let manager = TermsOfServiceManager::default();
        assert!(!manager.is_inited().await);
    }

    // ========== init Tests ==========

    #[tokio::test]
    async fn test_init() {
        let manager = TermsOfServiceManager::new();
        manager.init().await;
        assert!(manager.is_inited().await);
    }

    #[tokio::test]
    async fn test_init_multiple() {
        let manager = TermsOfServiceManager::new();
        manager.init().await;
        manager.init().await;
        assert!(manager.is_inited().await);
    }

    // ========== accept_terms_of_service Tests ==========

    #[tokio::test]
    async fn test_accept_terms_of_service_success() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms).await;
        let result = manager.accept_terms_of_service("tos_v1").await;
        assert!(result.is_ok());
        assert!(!manager.has_pending_terms().await);
    }

    #[tokio::test]
    async fn test_accept_terms_of_service_no_pending() {
        let manager = TermsOfServiceManager::new();
        let result = manager.accept_terms_of_service("tos_v1").await;
        assert!(result.is_err());
        match result {
            Err(AcceptError::TermsIdMismatch { expected, got }) => {
                assert_eq!(expected, None);
                assert_eq!(got, "tos_v1");
            }
            _ => panic!("Expected TermsIdMismatch error"),
        }
    }

    #[tokio::test]
    async fn test_accept_terms_of_service_mismatch() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms).await;
        let result = manager.accept_terms_of_service("tos_v2").await;
        assert!(result.is_err());
        match result {
            Err(AcceptError::TermsIdMismatch { expected, got }) => {
                assert_eq!(expected, Some("tos_v1".to_string()));
                assert_eq!(got, "tos_v2");
            }
            _ => panic!("Expected TermsIdMismatch error"),
        }
    }

    #[tokio::test]
    async fn test_accept_terms_of_service_empty_id() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms).await;
        let result = manager.accept_terms_of_service("").await;
        assert!(result.is_ok());
    }

    // ========== set_pending_terms Tests ==========

    #[tokio::test]
    async fn test_set_pending_terms() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms.clone()).await;
        let pending = manager.get_pending_terms().await;
        assert_eq!(pending, Some(terms));
    }

    #[tokio::test]
    async fn test_set_pending_terms_replace() {
        let manager = TermsOfServiceManager::new();
        let terms1 = TermsOfService::new("tos_v1".to_string(), "Terms1...".to_string(), 18, true);
        let terms2 = TermsOfService::new("tos_v2".to_string(), "Terms2...".to_string(), 21, false);
        manager.set_pending_terms(terms1).await;
        manager.set_pending_terms(terms2.clone()).await;
        let pending = manager.get_pending_terms().await;
        assert_eq!(pending, Some(terms2));
    }

    // ========== get_pending_terms Tests ==========

    #[tokio::test]
    async fn test_get_pending_terms_none() {
        let manager = TermsOfServiceManager::new();
        let pending = manager.get_pending_terms().await;
        assert!(pending.is_none());
    }

    #[tokio::test]
    async fn test_get_pending_terms_some() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms.clone()).await;
        let pending = manager.get_pending_terms().await;
        assert_eq!(pending, Some(terms));
    }

    #[tokio::test]
    async fn test_get_pending_terms_returns_clone() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms.clone());
        let pending1 = manager.get_pending_terms().await;
        let pending2 = manager.get_pending_terms().await;
        assert_eq!(pending1, pending2);
    }

    // ========== is_inited Tests ==========

    #[tokio::test]
    async fn test_is_inited_false_initially() {
        let manager = TermsOfServiceManager::new();
        assert!(!manager.is_inited().await);
    }

    #[tokio::test]
    async fn test_is_inited_true_after_init() {
        let manager = TermsOfServiceManager::new();
        manager.init().await;
        assert!(manager.is_inited().await);
    }

    // ========== has_pending_terms Tests ==========

    #[tokio::test]
    async fn test_has_pending_terms_false_initially() {
        let manager = TermsOfServiceManager::new();
        assert!(!manager.has_pending_terms().await);
    }

    #[tokio::test]
    async fn test_has_pending_terms_true_after_set() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms).await;
        assert!(manager.has_pending_terms().await);
    }

    #[tokio::test]
    async fn test_has_pending_terms_false_after_accept() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms).await;
        manager.accept_terms_of_service("tos_v1").await.unwrap();
        assert!(!manager.has_pending_terms().await);
    }

    // ========== clear_pending_terms Tests ==========

    #[tokio::test]
    async fn test_clear_pending_terms() {
        let manager = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager.set_pending_terms(terms).await;
        assert!(manager.has_pending_terms().await);
        manager.clear_pending_terms().await;
        assert!(!manager.has_pending_terms().await);
    }

    #[tokio::test]
    async fn test_clear_pending_terms_when_empty() {
        let manager = TermsOfServiceManager::new();
        manager.clear_pending_terms().await;
        assert!(!manager.has_pending_terms().await);
    }

    // ========== clone Tests ==========

    #[tokio::test]
    async fn test_clone_shares_state() {
        let manager1 = TermsOfServiceManager::new();
        manager1.init().await;
        let manager2 = manager1.clone();
        assert!(manager2.is_inited().await);
    }

    #[tokio::test]
    async fn test_clone_pending_terms() {
        let manager1 = TermsOfServiceManager::new();
        let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
        manager1.set_pending_terms(terms).await;
        let manager2 = manager1.clone();
        assert!(manager2.has_pending_terms().await);
    }

    // ========== concurrent access Tests ==========

    #[tokio::test]
    async fn test_concurrent_init_and_check() {
        let manager = Arc::new(TermsOfServiceManager::new());
        let manager_clone = Arc::clone(&manager);

        tokio::spawn(async move {
            manager_clone.init().await;
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        let _ = manager.is_inited().await;
        // Test passes if no deadlock or panic occurs
    }

    #[tokio::test]
    async fn test_concurrent_set_and_get() {
        let manager = Arc::new(TermsOfServiceManager::new());
        let manager_clone = Arc::clone(&manager);

        tokio::spawn(async move {
            let terms = TermsOfService::new("tos_v1".to_string(), "Terms...".to_string(), 18, true);
            manager_clone.set_pending_terms(terms).await;
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        let _ = manager.get_pending_terms().await;
        // Test passes if no deadlock or panic occurs
    }

    // ========== Error Tests ==========

    #[tokio::test]
    async fn test_error_display() {
        let error = AcceptError::TermsIdMismatch {
            expected: Some("tos_v1".to_string()),
            got: "tos_v2".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("mismatch"));
    }

    #[tokio::test]
    async fn test_error_equality() {
        let error1 = AcceptError::TermsIdMismatch {
            expected: Some("tos_v1".to_string()),
            got: "tos_v2".to_string(),
        };
        let error2 = AcceptError::TermsIdMismatch {
            expected: Some("tos_v1".to_string()),
            got: "tos_v2".to_string(),
        };
        assert_eq!(error1, error2);
    }
}
