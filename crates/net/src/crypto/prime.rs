// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Prime factorization for MTProto key exchange.
//!
//! This module implements prime factorization using Pollard's Rho algorithm,
//! which is required for the MTProto key exchange (request_pq -> pq_factorize).
//!
//! # References
//!
//! - TDLib: `td/mtproto/PacketStorer.h` (uses Pollard's Rho)
//! - TDLib: `td/utils/bigotron.c` (big integer operations)

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use rand::Rng;
use rand::RngCore;
use thiserror::Error;

/// Error types for prime factorization operations.
#[derive(Debug, Error)]
pub enum CryptoError {
    /// Failed to factorize the number
    #[error("Failed to factorize: {0}")]
    FactorizationFailed(String),

    /// Invalid input for factorization
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Number is prime (cannot be factorized)
    #[error("Number is prime: {0}")]
    IsPrime(String),

    /// Number is too large
    #[error("Number is too large: {0} bits")]
    TooLarge(usize),
}

/// Result type for factorization operations.
pub type FactorizeResult<T> = Result<T, CryptoError>;

/// Greatest common divisor using Euclidean algorithm.
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    use num_integer::Integer;
    a.gcd(b)
}

/// Pollard's Rho algorithm for factorization.
///
/// This is a probabilistic algorithm that finds a non-trivial factor
/// of a composite number. It's used by TDLib for pq_factorize.
///
/// # Algorithm
///
/// 1. Pick random x, c
/// 2. Iterate: x' = f(x) = (x*x + c) mod n
/// 3. Look for gcd(|x - x'|, n) > 1
///
/// # Arguments
///
/// * `n` - The number to factorize (must be composite)
/// * `max_iterations` - Maximum iterations before giving up
///
/// # Returns
///
/// A non-trivial factor of `n`
fn pollards_rho(n: &BigUint, max_iterations: usize) -> Option<BigUint> {
    if n.is_one() || n.is_zero() {
        return None;
    }

    // Check for small primes first (optimization)
    if n % 2u32 == BigUint::zero() {
        return Some(BigUint::from(2u32));
    }
    if n % 3u32 == BigUint::zero() {
        return Some(BigUint::from(3u32));
    }

    let mut rng = rand::thread_rng();

    // Floyd's cycle detection: use two pointers
    for _ in 0..10 {
        // Pick random starting values (use simpler approach)
        let x_val: u32 = rng.gen_range(0..u32::MAX);
        let c_val: u32 = rng.gen_range(0..u32::MAX);
        let mut x = BigUint::from(x_val.saturating_add(2));
        let mut y = x.clone();
        let c = BigUint::from(c_val.saturating_add(1));
        let mut d = BigUint::one();

        // f(x) = (x*x + c) mod n
        let f = |x_val: &BigUint| -> BigUint {
            (x_val * x_val + &c) % n
        };

        while d.is_one() && d < *n {
            x = f(&x);
            y = f(&f(&y));
            let diff = if x > y { &x - &y } else { &y - &x };
            d = gcd(&diff, n);
        }

        if d > BigUint::one() && d < *n {
            return Some(d);
        }
    }

    // Try Brent's algorithm as fallback
    brent_rho(n, max_iterations)
}

/// Brent's improvement to Pollard's Rho algorithm.
///
/// Brent's algorithm is generally faster than the basic Floyd cycle detection.
fn brent_rho(n: &BigUint, max_iterations: usize) -> Option<BigUint> {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let x_val: u32 = rng.gen_range(0..u32::MAX);
        let c_val: u32 = rng.gen_range(0..u32::MAX);
        let m_val: u32 = rng.gen_range(0..u32::MAX);

        let mut x = BigUint::from(x_val.saturating_add(2));
        let c = BigUint::from(c_val.saturating_add(1));
        let m = BigUint::from(m_val % 100 + 1);

        let mut y = x.clone();
        let mut r = BigUint::one();
        let mut q = BigUint::one();
        let mut g = BigUint::one();

        let f = |x_val: &BigUint| -> BigUint {
            (x_val * x_val + &c) % n
        };

        let mut iteration = 0;
        while g.is_one() && iteration < max_iterations {
            x = y.clone();
            let r_usize = r.to_usize().unwrap_or(256);
            for _ in 0..r_usize.min(256) {
                y = f(&y);
            }
            let k = BigUint::zero();
            while k < r && g.is_one() {
                let _ys = y.clone();
                let diff = &r - &k;
                let m_usize = std::cmp::min(&m, &diff).to_usize().unwrap_or(256);
                for _ in 0..m_usize.min(256) {
                    y = f(&y);
                    let diff = if x > y { &x - &y } else { &y - &x };
                    q = q * diff % n;
                }
                g = gcd(&q, n);
                iteration += 1;
                if iteration >= max_iterations {
                    break;
                }
            }
            r <<= 1;
        }

        if g > BigUint::one() && g < *n {
            return Some(g);
        }
    }

    None
}

/// Checks if a number is likely prime using Miller-Rabin test.
///
/// # Arguments
///
/// * `n` - The number to test
/// * `rounds` - Number of test rounds (default: 5 for 64-bit, more for larger)
fn is_prime_miller_rabin(n: &BigUint, rounds: usize) -> bool {
    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    if n <= &BigUint::from(1u32) {
        return false;
    }
    if n <= &BigUint::from(3u32) {
        return true;
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    // Write n-1 as 2^r * d
    let mut d = n - 1u32;
    let mut r = 0usize;
    while &d % 2u32 == BigUint::zero() {
        d /= 2u32;
        r += 1;
    }

    let mut rng = rand::thread_rng();
    let n_minus_1 = n - 1u32;

    for _ in 0..rounds {
        let a_val: u32 = rng.gen_range(0..u32::MAX);
        let a = BigUint::from(a_val % (u32::MAX - 3) + 2) % (n - 3u32) + 2u32;
        let mut x = a.modpow(&d, n);

        if x == BigUint::one() || x == n_minus_1 {
            continue;
        }

        let mut composite = true;
        for _ in 0..r - 1 {
            x = &x * &x % n;
            if x == n_minus_1 {
                composite = false;
                break;
            }
        }

        if composite {
            return false;
        }
    }

    true
}

/// Factorizes a 64-bit number into two primes.
///
/// This is the main function used in MTProto key exchange for
/// `request_pq` response processing. The Telegram server sends a
/// composite number `pq` which must be factorized to continue.
///
/// # Arguments
///
/// * `pq` - The composite number to factorize (typically 64-bit)
///
/// # Returns
///
/// `Some((p, q))` where p * q = pq, or `None` if factorization fails
///
/// # Example
///
/// ```ignore
/// use rustgram_net::crypto::pq_factorize;
///
/// // Small example
/// let pq = 15u64; // 3 * 5
/// let result = pq_factorize(pq);
/// assert_eq!(result, Some((3, 5)));
///
/// // Larger example (Telegram uses ~64-bit numbers)
/// let pq = 1000000007u64 * 1000000009u64;
/// let result = pq_factorize(pq);
/// assert!(result.is_some());
/// ```
pub fn pq_factorize(pq: u64) -> Option<(u64, u64)> {
    if pq < 4 {
        return None;
    }

    // Check for small primes first
    if pq % 2 == 0 {
        let p = 2u64;
        let q = pq / 2;
        return Some((p, q));
    }

    let n = BigUint::from(pq);

    // Check if it's prime
    if is_prime_miller_rabin(&n, 5) {
        return None;
    }

    // Try Pollard's Rho
    if let Some(p_factor) = pollards_rho(&n, 10000) {
        let q_factor = &n / &p_factor;

        // Convert to u64 if possible
        if let Some(p_u64) = p_factor.to_u64() {
            if let Some(q_u64) = q_factor.to_u64() {
                // Return in sorted order (smaller first)
                if p_u64 <= q_u64 {
                    return Some((p_u64, q_u64));
                } else {
                    return Some((q_u64, p_u64));
                }
            }
        }
    }

    // Fallback: trial division for small numbers
    if pq < 1_000_000 {
        return trial_division(pq);
    }

    None
}

/// Trial division fallback for small numbers.
fn trial_division(n: u64) -> Option<(u64, u64)> {
    let mut p = 3u64;
    while p * p <= n {
        if n % p == 0 {
            return Some((p, n / p));
        }
        p += 2;
    }
    None
}

/// Factorizes a big integer represented as bytes.
///
/// Used for larger numbers that don't fit in u64.
///
/// # Arguments
///
/// * `pq_bytes` - The big-endian bytes representing the number
///
/// # Returns
///
/// A tuple of byte vectors (p, q) or an error
pub fn pq_factorize_big(pq_bytes: &[u8]) -> FactorizeResult<(Vec<u8>, Vec<u8>)> {
    if pq_bytes.is_empty() {
        return Err(CryptoError::InvalidInput("Empty input".into()));
    }

    let n = BigUint::from_bytes_be(pq_bytes);

    // Check size limit (prevent DOS)
    let bits = n.bits();
    if bits > 256 {
        return Err(CryptoError::TooLarge(bits.try_into().unwrap_or(usize::MAX)));
    }

    // Check if it's prime
    let rounds = if bits > 128 { 10 } else { 5 };
    if is_prime_miller_rabin(&n, rounds) {
        return Err(CryptoError::IsPrime(format!("{} bits", bits)));
    }

    // Try Pollard's Rho
    let max_iterations = if bits > 128 { 50000 } else { 10000 };
    if let Some(p_factor) = pollards_rho(&n, max_iterations) {
        let q_factor = &n / &p_factor;

        // Convert to bytes
        let p_bytes = p_factor.to_bytes_be();
        let q_bytes = q_factor.to_bytes_be();

        return Ok((p_bytes, q_bytes));
    }

    // Try to convert to u64 for simpler factorization
    if let Some(n_u64) = n.to_u64() {
        if let Some((p, q)) = pq_factorize(n_u64) {
            let p_bytes = BigUint::from(p).to_bytes_be();
            let q_bytes = BigUint::from(q).to_bytes_be();
            return Ok((p_bytes, q_bytes));
        }
    }

    Err(CryptoError::FactorizationFailed(
        "Pollard's Rho failed to find a factor".into(),
    ))
}

/// Utility function to generate a random prime number.
///
/// # Arguments
///
/// * `bits` - Number of bits for the prime
///
/// # Returns
///
/// A random prime number of approximately `bits` bits
pub fn generate_prime(bits: usize) -> BigUint {
    let mut rng = rand::thread_rng();

    // Generate random odd number with specified bit length
    loop {
        // Generate random bytes
        let num_bytes = bits.div_ceil(8);
        let mut bytes = vec![0u8; num_bytes];
        rng.fill_bytes(&mut bytes);

        // Set the highest bit to ensure we have exactly 'bits' bits
        bytes[0] |= 0x80;
        // Ensure the number is odd
        bytes[num_bytes - 1] |= 0x01;

        let candidate = BigUint::from_bytes_be(&bytes);

        // Check if it's prime using Miller-Rabin
        let rounds = if bits > 128 { 10 } else { 5 };
        if is_prime_miller_rabin(&candidate, rounds) {
            return candidate;
        }
    }
}

/// Utility function to multiply two primes (inverse of factorize).
///
/// # Arguments
///
/// * `p` - First prime
/// * `q` - Second prime
///
/// # Returns
///
/// The product p * q
pub fn multiply_primes(p: u64, q: u64) -> u64 {
    p.saturating_mul(q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pq_factorize_small() {
        // Test small known factorizations
        assert_eq!(pq_factorize(15), Some((3, 5)));
        assert_eq!(pq_factorize(21), Some((3, 7)));
        assert_eq!(pq_factorize(35), Some((5, 7)));
        assert_eq!(pq_factorize(77), Some((7, 11)));
    }

    #[test]
    fn test_pq_factorize_even() {
        assert_eq!(pq_factorize(14), Some((2, 7)));
        assert_eq!(pq_factorize(100), Some((2, 50)));
    }

    #[test]
    fn test_pq_factorize_prime() {
        // Primes cannot be factorized
        assert_eq!(pq_factorize(13), None);
        assert_eq!(pq_factorize(17), None);
        assert_eq!(pq_factorize(997), None);
    }

    #[test]
    fn test_pq_factorize_large() {
        // Test with larger primes
        let p = 1000003u64;
        let q = 1000033u64;
        let pq = p * q;

        let result = pq_factorize(pq);
        assert!(result.is_some());

        let (found_p, found_q) = result.unwrap();
        assert_eq!(found_p * found_q, pq);
    }

    #[test]
    fn test_pq_factorize_order() {
        // Result should be sorted (smaller first)
        let p = 1000003u64;
        let q = 1000033u64;
        let pq = p * q;

        let result = pq_factorize(pq);
        assert!(result.is_some());

        let (found_p, found_q) = result.unwrap();
        assert!(found_p <= found_q);
    }

    #[test]
    fn test_pq_factorize_big() {
        // Test big integer factorization
        let p = 10007u64;
        let q = 10009u64;
        let pq = p * q;

        let pq_bytes = BigUint::from(pq).to_bytes_be();
        let result = pq_factorize_big(&pq_bytes);

        assert!(result.is_ok());

        let (p_bytes, q_bytes) = result.unwrap();
        let found_p = BigUint::from_bytes_be(&p_bytes).to_u64().unwrap();
        let found_q = BigUint::from_bytes_be(&q_bytes).to_u64().unwrap();

        assert_eq!(found_p * found_q, pq);
    }

    #[test]
    fn test_pq_factorize_big_empty_input() {
        let result = pq_factorize_big(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_pq_factorize_big_prime() {
        // Prime number should return error
        let prime = 99991u64;
        let prime_bytes = BigUint::from(prime).to_bytes_be();

        let result = pq_factorize_big(&prime_bytes);
        assert!(result.is_err());

        if let Err(CryptoError::IsPrime(_)) = result {
            // Expected error type
        } else {
            panic!("Expected IsPrime error");
        }
    }

    #[test]
    fn test_multiply_primes() {
        assert_eq!(multiply_primes(3, 5), 15);
        assert_eq!(multiply_primes(7, 11), 77);
        assert_eq!(multiply_primes(10007, 10009), 100160063);
    }

    #[test]
    fn test_generate_prime() {
        let prime = generate_prime(64);
        assert!(prime.bits() <= 64);
        assert!(prime > BigUint::from(2u32));

        // Verify it's likely prime
        assert!(is_prime_miller_rabin(&prime, 10));
    }

    #[test]
    fn test_is_prime_miller_rabin() {
        // Known primes
        assert!(is_prime_miller_rabin(&BigUint::from(2u32), 5));
        assert!(is_prime_miller_rabin(&BigUint::from(3u32), 5));
        assert!(is_prime_miller_rabin(&BigUint::from(5u32), 5));
        assert!(is_prime_miller_rabin(&BigUint::from(13u32), 5));
        assert!(is_prime_miller_rabin(&BigUint::from(997u32), 5));

        // Known composites
        assert!(!is_prime_miller_rabin(&BigUint::from(4u32), 5));
        assert!(!is_prime_miller_rabin(&BigUint::from(15u32), 5));
        assert!(!is_prime_miller_rabin(&BigUint::from(100u32), 5));
        assert!(!is_prime_miller_rabin(&BigUint::from(561u32), 5)); // Carmichael number
    }

    #[test]
    fn test_trial_division() {
        assert_eq!(trial_division(15), Some((3, 5)));
        assert_eq!(trial_division(21), Some((3, 7)));
        assert_eq!(trial_division(77), Some((7, 11)));

        // Primes return None
        assert_eq!(trial_division(13), None);
    }

    #[test]
    fn test_gcd() {
        use num_bigint::BigUint;

        assert_eq!(gcd(&BigUint::from(12u32), &BigUint::from(8u32)), BigUint::from(4u32));
        assert_eq!(gcd(&BigUint::from(17u32), &BigUint::from(23u32)), BigUint::from(1u32));
        assert_eq!(gcd(&BigUint::from(100u32), &BigUint::from(10u32)), BigUint::from(10u32));
    }

    // Property-based test: factorizing a product of two random primes
    #[test]
    fn test_factorize_product_of_primes() {
        for _ in 0..10 {
            let p = generate_prime(16);
            let q = generate_prime(16);

            // Skip if p == q
            if p == q {
                continue;
            }

            let p_u64 = p.to_u64().unwrap();
            let q_u64 = q.to_u64().unwrap();
            let pq = p_u64 * q_u64;

            let result = pq_factorize(pq);
            if let Some((found_p, found_q)) = result {
                assert_eq!(found_p * found_q, pq);
            } else {
                panic!("Failed to factorize {}", pq);
            }
        }
    }

    #[test]
    fn test_telegram_like_pq() {
        // Test with numbers similar to what Telegram uses
        // These are small examples for testing
        let test_cases = vec![
            (17, 19),      // Small primes
            (997, 991),    // Medium primes
            (10007, 10009), // Larger primes
        ];

        for (p, q) in test_cases {
            let pq = p * q;
            let result = pq_factorize(pq);
            assert!(result.is_some(), "Failed to factorize {} * {} = {}", p, q, pq);

            let (found_p, found_q) = result.unwrap();
            assert_eq!(found_p * found_q, pq);
        }
    }
}
