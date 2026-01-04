// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Network query verifier.
//!
//! This module implements TDLib's NetQueryVerifier from `td/telegram/net/NetQueryVerifier.h`.
//!
//! Handles query verification including human verification (captcha) and recaptcha challenges.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use thiserror::Error;

use crate::query::NetQuery;

/// Verification query type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VerificationType {
    /// Standard verification with nonce
    Verification = 0,
    /// Recaptcha challenge
    Recaptcha = 1,
}

/// Verification query state.
#[derive(Debug, Clone)]
pub struct VerificationQuery {
    /// Query type
    pub query_type: VerificationType,
    /// Nonce for verification or action string for recaptcha
    pub nonce_or_action: String,
    /// Recaptcha key ID (for recaptcha type)
    pub recaptcha_key_id: String,
}

impl VerificationQuery {
    /// Creates a new verification query.
    pub fn verification(nonce: String) -> Self {
        Self {
            query_type: VerificationType::Verification,
            nonce_or_action: nonce,
            recaptcha_key_id: String::new(),
        }
    }

    /// Creates a new recaptcha query.
    pub fn recaptcha(action: String, recaptcha_key_id: String) -> Self {
        Self {
            query_type: VerificationType::Recaptcha,
            nonce_or_action: action,
            recaptcha_key_id,
        }
    }
}

/// Error types for query verification.
#[derive(Debug, Error)]
pub enum VerificationError {
    /// Query not found
    #[error("Query not found: {0}")]
    QueryNotFound(i64),

    /// Invalid verification token
    #[error("Invalid verification token for query {0}")]
    InvalidToken(i64),

    /// Verification failed
    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    /// Query already verified
    #[error("Query {0} already verified")]
    AlreadyVerified(i64),
}

/// Result type for verification operations.
pub type VerificationResult<T> = Result<T, VerificationError>;

/// Promise callback for verification completion.
pub type VerificationCallback = Box<dyn FnOnce(VerificationResult<()>) + Send>;

/// Network query verifier.
///
/// Handles verification challenges for network queries including:
/// - Human verification with nonce
/// - Recaptcha challenges
///
/// Based on TDLib's NetQueryVerifier from `td/telegram/net/NetQueryVerifier.h`.
#[derive(Debug)]
pub struct NetQueryVerifier {
    /// Active queries being verified
    queries: Arc<parking_lot::Mutex<HashMap<i64, (NetQuery, VerificationQuery)>>>,
    /// Next query ID
    next_query_id: Arc<AtomicI64>,
}

impl NetQueryVerifier {
    /// Creates a new query verifier.
    pub fn new() -> Self {
        Self {
            queries: Arc::new(parking_lot::Mutex::new(HashMap::new())),
            next_query_id: Arc::new(AtomicI64::new(1)),
        }
    }

    /// Initiates verification for a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to verify
    /// * `nonce` - Verification nonce
    ///
    /// # Returns
    ///
    /// The verification query ID.
    pub fn verify(&self, query: NetQuery, nonce: String) -> i64 {
        let query_id = self.next_query_id.fetch_add(1, Ordering::SeqCst);
        let verification = VerificationQuery::verification(nonce);

        let mut queries = self.queries.lock();
        queries.insert(query_id, (query, verification));

        query_id
    }

    /// Initiates recaptcha verification for a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to verify
    /// * `action` - Recaptcha action string
    /// * `recaptcha_key_id` - Recaptcha key ID
    ///
    /// # Returns
    ///
    /// The verification query ID.
    pub fn check_recaptcha(
        &self,
        query: NetQuery,
        action: String,
        recaptcha_key_id: String,
    ) -> i64 {
        let query_id = self.next_query_id.fetch_add(1, Ordering::SeqCst);
        let verification = VerificationQuery::recaptcha(action, recaptcha_key_id);

        let mut queries = self.queries.lock();
        queries.insert(query_id, (query, verification));

        query_id
    }

    /// Sets a verification token for a query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The verification query ID
    /// * `_token` - The verification token
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, Err if the query is not found.
    pub fn set_verification_token(&self, query_id: i64, _token: String) -> VerificationResult<()> {
        let mut queries = self.queries.lock();

        if !queries.contains_key(&query_id) {
            return Err(VerificationError::QueryNotFound(query_id));
        }

        // In a real implementation, this would send the token to the server
        // For now, we'll just remove the query from the active list
        queries.remove(&query_id);

        Ok(())
    }

    /// Gets a query by its verification ID.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The verification query ID
    ///
    /// # Returns
    ///
    /// The query and its verification state, or an error if not found.
    pub fn get_query(&self, query_id: i64) -> VerificationResult<(NetQuery, VerificationQuery)> {
        let queries = self.queries.lock();

        queries
            .get(&query_id)
            .cloned()
            .ok_or(VerificationError::QueryNotFound(query_id))
    }

    /// Cancels verification for a query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The verification query ID
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, Err if the query is not found.
    pub fn cancel_verification(&self, query_id: i64) -> VerificationResult<()> {
        let mut queries = self.queries.lock();

        queries
            .remove(&query_id)
            .map(|_| ())
            .ok_or(VerificationError::QueryNotFound(query_id))
    }

    /// Returns the number of active verifications.
    pub fn active_count(&self) -> usize {
        self.queries.lock().len()
    }

    /// Clears all active verifications.
    pub fn clear(&self) {
        self.queries.lock().clear();
    }
}

impl Default for NetQueryVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dc::DcId;
    use crate::query::{AuthFlag, GzipFlag, NetQueryType};

    fn create_test_query() -> NetQuery {
        NetQuery::new(
            1,
            bytes::Bytes::from(vec![1, 2, 3]),
            DcId::main(),
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            0, // tl_constructor
        )
    }

    #[test]
    fn test_verify_creates_query() {
        let verifier = NetQueryVerifier::new();
        let query = create_test_query();
        let nonce = "test_nonce".to_string();

        let query_id = verifier.verify(query.clone(), nonce);

        assert!(query_id >= 1);
        assert_eq!(verifier.active_count(), 1);

        let retrieved = verifier.get_query(query_id);
        assert!(retrieved.is_ok());
        match retrieved {
            Ok((_retrieved_query, verification)) => {
                assert_eq!(verification.query_type, VerificationType::Verification);
                assert_eq!(verification.nonce_or_action, "test_nonce");
            }
            Err(_) => panic!("Expected Ok verification"),
        }
    }

    #[test]
    fn test_check_recaptcha_creates_query() {
        let verifier = NetQueryVerifier::new();
        let query = create_test_query();
        let action = "login".to_string();
        let key_id = "recaptcha_key".to_string();

        let query_id = verifier.check_recaptcha(query.clone(), action, key_id);

        assert!(query_id >= 1);
        assert_eq!(verifier.active_count(), 1);

        let retrieved = verifier.get_query(query_id);
        assert!(retrieved.is_ok());
        match retrieved {
            Ok((_, verification)) => {
                assert_eq!(verification.query_type, VerificationType::Recaptcha);
                assert_eq!(verification.nonce_or_action, "login");
                assert_eq!(verification.recaptcha_key_id, "recaptcha_key");
            }
            Err(_) => panic!("Expected Ok verification"),
        }
    }

    #[test]
    fn test_set_verification_token() {
        let verifier = NetQueryVerifier::new();
        let query = create_test_query();

        let query_id = verifier.verify(query, "nonce".to_string());
        assert_eq!(verifier.active_count(), 1);

        let result = verifier.set_verification_token(query_id, "token".to_string());
        assert!(result.is_ok());
        assert_eq!(verifier.active_count(), 0);
    }

    #[test]
    fn test_set_verification_token_not_found() {
        let verifier = NetQueryVerifier::new();

        let result = verifier.set_verification_token(999, "token".to_string());
        assert!(matches!(result, Err(VerificationError::QueryNotFound(999))));
    }

    #[test]
    fn test_cancel_verification() {
        let verifier = NetQueryVerifier::new();
        let query = create_test_query();

        let query_id = verifier.verify(query, "nonce".to_string());
        assert_eq!(verifier.active_count(), 1);

        let result = verifier.cancel_verification(query_id);
        assert!(result.is_ok());
        assert_eq!(verifier.active_count(), 0);
    }

    #[test]
    fn test_cancel_verification_not_found() {
        let verifier = NetQueryVerifier::new();

        let result = verifier.cancel_verification(999);
        assert!(matches!(result, Err(VerificationError::QueryNotFound(999))));
    }

    #[test]
    fn test_query_ids_are_unique() {
        let verifier = NetQueryVerifier::new();
        let query = create_test_query();

        let id1 = verifier.verify(query.clone(), "nonce1".to_string());
        let id2 = verifier.verify(query, "nonce2".to_string());

        assert_ne!(id1, id2);
    }

    #[test]
    fn test_clear() {
        let verifier = NetQueryVerifier::new();
        let query = create_test_query();

        verifier.verify(query, "nonce".to_string());
        assert_eq!(verifier.active_count(), 1);

        verifier.clear();
        assert_eq!(verifier.active_count(), 0);
    }

    #[test]
    fn test_verification_query_verification_type() {
        let verification = VerificationQuery::verification("test_nonce".to_string());

        assert_eq!(verification.query_type, VerificationType::Verification);
        assert_eq!(verification.nonce_or_action, "test_nonce");
        assert!(verification.recaptcha_key_id.is_empty());
    }

    #[test]
    fn test_verification_query_recaptcha_type() {
        let verification = VerificationQuery::recaptcha("login".to_string(), "key123".to_string());

        assert_eq!(verification.query_type, VerificationType::Recaptcha);
        assert_eq!(verification.nonce_or_action, "login");
        assert_eq!(verification.recaptcha_key_id, "key123");
    }

    #[test]
    fn test_active_count() {
        let verifier = NetQueryVerifier::new();
        let query = create_test_query();

        assert_eq!(verifier.active_count(), 0);

        let _ = verifier.verify(query.clone(), "nonce1".to_string());
        assert_eq!(verifier.active_count(), 1);

        let _ = verifier.verify(query.clone(), "nonce2".to_string());
        assert_eq!(verifier.active_count(), 2);

        let _ = verifier.set_verification_token(1, "token".to_string());
        assert_eq!(verifier.active_count(), 1);
    }
}
