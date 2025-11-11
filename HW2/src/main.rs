//! daily homework 2 — array utilities for u32
//!
//! Notes on overflow: the spec doesn't say how to handle arithmetic overflow.
//! The functions below use *wrapping* arithmetic for products/sums so they never
//! panic in debug builds. If you prefer checked arithmetic, say the word and
//! I'll switch it.

use std::collections::HashSet;

/// multiply_array
///
/// Purpose: Multiply all entries of an array of 32-bit unsigned integers.
/// Parameters: `arr: &[u32]` — the input array/slice.
/// Returns: `u32` — the product of all entries (wrapping on overflow).
///
/// Type: `fn multiply_array(arr: &[u32]) -> u32`
pub fn multiply_array(arr: &[u32]) -> u32 {
    arr.iter().copied().fold(1u32, |acc, x| acc.wrapping_mul(x))
}

/// all_different
///
/// Purpose: Determine whether *all* numbers in the array are pairwise distinct.
/// Parameters: `arr: &[u32]`.
/// Returns: `bool` — `true` if all are different, `false` otherwise.
///
/// Type: `fn all_different(arr: &[u32]) -> bool`
pub fn all_different(arr: &[u32]) -> bool {
    let mut seen = HashSet::with_capacity(arr.len());
    for &v in arr {
        if !seen.insert(v) {
            return false;
        }
    }
    true
}

/// all_different_except_zeros
///
/// Purpose: Determine whether all *non-zero* numbers are distinct; zeros may repeat.
/// Parameters: `arr: &[u32]`.
/// Returns: `bool` — `true` if all non-zeros are different, `false` otherwise.
///
/// Type: `fn all_different_except_zeros(arr: &[u32]) -> bool`
pub fn all_different_except_zeros(arr: &[u32]) -> bool {
    let mut seen = HashSet::new();
    for &v in arr {
        if v == 0 {
            continue;
        }
        if !seen.insert(v) {
            return false;
        }
    }
    true
}

/// dot_product
///
/// Purpose: Compute the dot product of two arrays: sum_i (a[i] * b[i]).
/// Parameters: `a: &[u32]`, `b: &[u32]`.
/// Returns: `u32` — the dot product using wrapping arithmetic; returns `0` if lengths differ.
///
/// Type: `fn dot_product(a: &[u32], b: &[u32]) -> u32`
pub fn dot_product(a: &[u32], b: &[u32]) -> u32 {
    if a.len() != b.len() {
        return 0; // per assignment: "panic or produce 0 (your choice) if lengths differ"
    }
    let mut acc: u32 = 0;
    for (&x, &y) in a.iter().zip(b.iter()) {
        acc = acc.wrapping_add(x.wrapping_mul(y));
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        assert_eq!(multiply_array(&[]), 1);
        assert_eq!(multiply_array(&[5]), 5);
        assert_eq!(multiply_array(&[2, 3, 4]), 24);
        // wrapping behavior example (not required but documented)
        let _ = multiply_array(&[u32::MAX, 2]); // should not panic
    }

    #[test]
    fn test_all_different() {
        assert!(all_different(&[]));
        assert!(all_different(&[1, 2, 3, 4]));
        assert!(!all_different(&[1, 2, 1]));
    }

    #[test]
    fn test_all_different_except_zeros() {
        assert!(all_different_except_zeros(&[]));
        assert!(all_different_except_zeros(&[0, 0, 0]));
        assert!(all_different_except_zeros(&[0, 1, 0, 2, 3]));
        assert!(!all_different_except_zeros(&[0, 1, 2, 1]));
    }

    #[test]
    fn test_dot_product_equal_lengths() {
        assert_eq!(dot_product(&[], &[]), 0);
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32);
        assert_eq!(dot_product(&[10], &[20]), 200);
    }

    #[test]
    fn test_dot_product_unequal_lengths_returns_zero() {
        assert_eq!(dot_product(&[1, 2], &[1]), 0);
    }
}

fn main() {
    println!("{}", crate::multiply_array(&[2, 3, 4]));
}

