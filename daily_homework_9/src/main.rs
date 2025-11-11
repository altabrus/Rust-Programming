//! daily_homework_9 — vectors, iterators, and `filter()`
//!
//! Each required function uses an iterator and `filter()` in a single line
//! (the whole transformation is expressed as one chained expression).

/// smaller_than
///
/// Purpose: Return a new vector with only the values strictly smaller than `limit`.
/// Params:  `limit: u32`, `v: Vec<u32>`
/// Returns: `Vec<u32>`
/// Type:    `fn smaller_than(limit: u32, v: Vec<u32>) -> Vec<u32>`
pub fn smaller_than(limit: u32, v: Vec<u32>) -> Vec<u32> {
    v.into_iter().filter(|&x| x < limit).collect()
}

/// get_values
///
/// Purpose: From `(u32, &str)` pairs, select the `&str` whose key equals `id`.
/// Params:  `id: u32`, `v: Vec<(u32, &'a str)>`
/// Returns: `Vec<&'a str>`
/// Type:    `fn get_values<'a>(id: u32, v: Vec<(u32, &'a str)>) -> Vec<&'a str>`
pub fn get_values<'a>(id: u32, v: Vec<(u32, &'a str)>) -> Vec<&'a str> {
    v.into_iter().filter(|(k, _)| *k == id).map(|(_, s)| s).collect()
}

/// only_some
///
/// Purpose: Drop `None` and keep inner `T` values from a `Vec<Option<T>>`.
/// Params:  `v: Vec<Option<T>>`
/// Returns: `Vec<T>`
/// Type:    `fn only_some<T>(v: Vec<Option<T>>) -> Vec<T>`
pub fn only_some<T>(v: Vec<Option<T>>) -> Vec<T> {
    v.into_iter().filter(|o| o.is_some()).map(|o| o.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_than() {
        assert_eq!(smaller_than(5, vec![1, 5, 3, 7, 2]), vec![1, 3, 2]);
        assert_eq!(smaller_than(0, vec![1, 2, 3]), Vec::<u32>::new());
        assert_eq!(smaller_than(10, vec![]), Vec::<u32>::new());
    }

    #[test]
    fn test_get_values() {
        let data = vec![(1, "a"), (2, "b"), (1, "c"), (3, "d")];
        assert_eq!(get_values(1, data), vec!["a", "c"]);

        let data2 = vec![(9, "x"), (8, "y")];
        assert!(get_values(7, data2).is_empty());
    }

    #[test]
    fn test_only_some() {
        let v = vec![Some(1), None, Some(3), None, Some(5)];
        assert_eq!(only_some(v), vec![1, 3, 5]);

        let v2: Vec<Option<u32>> = vec![];
        assert!(only_some(v2).is_empty());
    }
}

fn main() {
    println!("{:?}", crate::smaller_than(5, vec![1, 5, 3, 7, 2]));
    println!("{:?}", crate::get_values(1, vec![(1, "a"), (2, "b"), (1, "c")]));
    println!("{:?}", crate::only_some(vec![Some("x"), None, Some("y")]));
}
