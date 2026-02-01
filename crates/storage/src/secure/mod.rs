//! Secure encrypted storage module.
//!
//! This module provides encrypted storage for sensitive data using AES-CBC.
//! It is only available when the "secure" feature is enabled.

#[cfg(feature = "secure")]
pub mod types;

#[cfg(feature = "secure")]
pub mod crypto;

#[cfg(feature = "secure")]
pub use types::{Secret, ValueHash};

#[cfg(feature = "secure")]
pub use crypto::{decrypt_value, encrypt_value};

/// Re-exports for the secure storage feature.
pub mod prelude {
    #[cfg(feature = "secure")]
    pub use super::{decrypt_value, encrypt_value, Secret, ValueHash};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_secure_feature_gate() {
        // This test ensures the module is properly feature-gated
        #[cfg(feature = "secure")]
        {
            // Secure storage is available
        }

        #[cfg(not(feature = "secure"))]
        {
            // Secure storage is not available
        }
    }
}
