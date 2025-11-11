//! daily_homework_3 — array/slice utilities
//!
//! Note: the assignment says “Array”, but for flexibility these functions
//! accept slices (`&[T]`) and mutable slices (`&mut [T]`), which work with
//! arrays and `Vec`s. If you need fixed-size array signatures, I can add
//! `const N: usize` versions too.

// -----------------------------------------------------------
// Function: count_occurrences
// Purpose:  Count how many times a given character occurs in an array.
// Parameters:
//   ch: char         — the character to count
//   arr: &[char]     — the array/slice of characters to scan
// Returns: usize     — number of occurrences of `ch`
// Type:     fn count_occurrences(ch: char, arr: &[char]) -> usize
// -----------------------------------------------------------
pub fn count_occurrences(ch: char, arr: &[char]) -> usize {
    arr.iter().copied().filter(|&c| c == ch).count()
}

// -----------------------------------------------------------
// Function: square_elements
// Purpose:  Square each element of the array in place.
// Parameters:
//   arr: &mut [u32]  — array/slice of 32-bit unsigned integers
// Returns: ()
// Notes:    Uses wrapping arithmetic to avoid debug-overflow panics.
// Type:     fn square_elements(arr: &mut [u32])
// -----------------------------------------------------------
pub fn square_elements(arr: &mut [u32]) {
    for x in arr.iter_mut() {
        *x = x.wrapping_mul(*x);
    }
}

// -----------------------------------------------------------
// Function: map_elements
// Purpose:  Apply a function to each element, modifying the array in place.
// Parameters:
//   f: impl FnMut(u32) -> u32 — mapping function
//   arr: &mut [u32]           — array/slice to modify
// Returns: ()
// Type:     fn map_elements<F: FnMut(u32) -> u32>(f: F, arr: &mut [u32])
// -----------------------------------------------------------
pub fn map_elements<F: FnMut(u32) -> u32>(mut f: F, arr: &mut [u32]) {
    for x in arr.iter_mut() {
        *x = f(*x);
    }
}

// -----------------------------------------------------------
// Function: count_true_and_false
// Purpose:  Count how many `true` and `false` values are in the array.
// Parameters:
//   arr: &[bool]     — array/slice of booleans
// Returns: (usize, usize) — (count_true, count_false)
// Type:     fn count_true_and_false(arr: &[bool]) -> (usize, usize)
// -----------------------------------------------------------
pub fn count_true_and_false(arr: &[bool]) -> (usize, usize) {
    let mut t = 0usize;
    let mut f = 0usize;
    for &b in arr {
        if b { t += 1 } else { f += 1 }
    }
    (t, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurrences() {
        let a = ['a', 'b', 'a', 'c', 'a'];
        assert_eq!(count_occurrences('a', &a), 3);
        assert_eq!(count_occurrences('b', &a), 1);
        assert_eq!(count_occurrences('z', &a), 0);
        let empty: [char; 0] = [];
        assert_eq!(count_occurrences('x', &empty), 0);
    }

    #[test]
    fn test_square_elements() {
        let mut v = [1u32, 3, 5];
        square_elements(&mut v);
        assert_eq!(v, [1, 9, 25]);

        // wrapping behavior (should not panic)
        let mut edge = [u32::MAX];
        square_elements(&mut edge);
        // value is deterministic but not important for the test—no panic is the key
    }

    #[test]
    fn test_map_elements_subtract_one() {
        let mut v = [1u32, 3, 5];
        map_elements(|x| x.saturating_sub(1), &mut v);
        assert_eq!(v, [0, 2, 4]);
    }

    #[test]
    fn test_map_elements_double() {
        fn double(x: u32) -> u32 { x.wrapping_mul(2) }
        let mut v = [2, 4, 6];
        map_elements(double, &mut v);
        assert_eq!(v, [4, 8, 12]);
    }

    #[test]
    fn test_count_true_and_false() {
        let a = [true, false, true, true, false];
        assert_eq!(count_true_and_false(&a), (3, 2));
        let all_t = [true; 5];
        assert_eq!(count_true_and_false(&all_t), (5, 0));
        let all_f = [false; 4];
        assert_eq!(count_true_and_false(&all_f), (0, 4));
        let empty: [bool; 0] = [];
        assert_eq!(count_true_and_false(&empty), (0, 0));
    }
}

fn main() {
    let mut nums = [1u32, 3, 5];
    crate::square_elements(&mut nums);
    println!("squared: {:?}", nums);

    let mut nums2 = [1u32, 3, 5];
    crate::map_elements(|x| x.saturating_sub(1), &mut nums2);
    println!("mapped:  {:?}", nums2);

    let chs = ['a', 'b', 'a', 'c', 'a'];
    println!("count 'a': {}", crate::count_occurrences('a', &chs));

    let bools = [true, false, true, true, false];
    println!("(true,false): {:?}", crate::count_true_and_false(&bools));
}
