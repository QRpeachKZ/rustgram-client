//! # Git Commit Hash
//!
//! Provides access to the git commit hash of the build.
//!
//! ## Overview
//!
//! This module provides a function to retrieve the git commit hash that was used
//! to build the application. This is useful for debugging and version tracking.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_git_commit_hash::get_git_commit_hash;
//!
//! let hash = get_git_commit_hash();
//! println!("Built from: {}", hash);
//! ```

/// Returns the git commit hash of the current build.
///
/// This function returns a string representation of the git commit hash
/// that was used to build the application.
///
/// # Returns
///
/// A string slice containing the git commit hash.
///
/// # Examples
///
/// ```
/// use rustgram_git_commit_hash::get_git_commit_hash;
///
/// let hash = get_git_commit_hash();
/// assert!(!hash.is_empty());
/// ```
#[inline]
pub fn get_git_commit_hash() -> &'static str {
    // Build script would typically set this via cargo:rustc-env
    // For now, return a placeholder or use the env var if set
    option_env!("RUSTGRAM_GIT_COMMIT_HASH").unwrap_or("unknown")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_get_git_commit_hash() {
        let hash = get_git_commit_hash();
        // The hash should be a non-empty string
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_length() {
        let hash = get_git_commit_hash();
        // Git commit hashes are typically 40 characters (SHA-1) or
        // can be abbreviated to 7+ characters
        assert!(hash.len() >= 7);
    }

    #[test]
    fn test_hash_is_ascii() {
        let hash = get_git_commit_hash();
        // Git hashes should be hexadecimal (ASCII), or "unknown" if not set
        if hash != "unknown" {
            assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[rstest]
    #[case("dev")]
    #[case("test")]
    fn test_multiple_calls_consistent(#[case] _env: &str) {
        let hash1 = get_git_commit_hash();
        let hash2 = get_git_commit_hash();
        assert_eq!(hash1, hash2);
    }
}
