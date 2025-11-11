//! daily_homework_6 — vector and iterator practice
//!
//! Demonstrates mutating vectors, iterating immutably, and using `map()`
//! with tuples.

/// add_one
///
/// Purpose: Add 1 to each element of a mutable vector of u32.
/// Parameters:
///   - `v: &mut Vec<u32>` — mutable reference to the vector.
/// Returns: ()
///
/// Type: `fn add_one(v: &mut Vec<u32>)`
pub fn add_one(v: &mut Vec<u32>) {
    for x in v.iter_mut() {
        *x += 1;
    }
}

/// get_total
///
/// Purpose: Compute the sum of all elements in a vector using an iterator.
/// Parameters:
///   - `v: &Vec<u32>` — reference to the vector.
/// Returns:
///   - `u32` — the total sum.
///
/// Type: `fn get_total(v: &Vec<u32>) -> u32`
pub fn get_total(v: &Vec<u32>) -> u32 {
    v.iter().sum()
}

/// sum_tuple
///
/// Purpose: For each tuple (a, b) in a vector, compute a + b and return a new vector of sums.
/// Parameters:
///   - `v: Vec<(u32, u32)>` — vector of pairs.
/// Returns:
///   - `Vec<u32>` — new vector containing the sums.
///
/// Type: `fn sum_tuple(v: Vec<(u32, u32)>) -> Vec<u32>`
///
/// Implementation uses `map()` in one line.
pub fn sum_tuple(v: Vec<(u32, u32)>) -> Vec<u32> {
    v.into_iter().map(|(a, b)| a + b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let mut v = vec![0u32, 1, 2, 3];
        add_one(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_get_total() {
        let v = vec![1u32, 2, 3, 4];
        assert_eq!(get_total(&v), 10);
        assert_eq!(get_total(&vec![]), 0);
    }

    #[test]
    fn test_sum_tuple() {
        let v = vec![(1, 2), (3, 4), (10, 5)];
        let result = sum_tuple(v);
        assert_eq!(result, vec![3, 7, 15]);
    }
}

fn main() {
    let mut nums = vec![1, 2, 3];
    crate::add_one(&mut nums);
    println!("after add_one: {:?}", nums);

    println!("get_total: {}", crate::get_total(&nums));

    let pairs = vec![(1, 5), (10, 2), (3, 7)];
    let sums = crate::sum_tuple(pairs);
    println!("sum_tuple result: {:?}", sums);
}
