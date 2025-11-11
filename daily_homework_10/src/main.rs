//! daily_homework_10 — vectors, iterators, and `fold()`
//!
//! Each function below uses an iterator and `fold()` in a single expression,
//! matching the assignment’s “one line” requirement inside the body.
//!
//! Notes:
//! - `smallest_word` assumes a non-empty vector (typical for “find the minimum”).
//!   If you want it to handle empty inputs, we can return `Option<&str>` instead.

/// smallest_word
///
/// Purpose: Return the *first* shortest `&str` in the vector.
/// Params:  `v: Vec<&'a str>`
/// Returns: `&'a str` (panics if `v` is empty)
///
/// Type:    `fn smallest_word<'a>(v: Vec<&'a str>) -> &'a str`
pub fn smallest_word<'a>(v: Vec<&'a str>) -> &'a str {
    v.into_iter()
        .fold(None::<&'a str>, |acc, s| Some(match acc { None => s, Some(a) => if s.len() < a.len() { s } else { a } }))
        .unwrap()
}

/// any_smaller
///
/// Purpose: Return `true` if *any* tuple `(x, y)` has both components `< limit`.
/// Params:  `limit: u32`, `v: Vec<(u32, u32)>`
/// Returns: `bool`
///
/// Type:    `fn any_smaller(limit: u32, v: Vec<(u32, u32)>) -> bool`
pub fn any_smaller(limit: u32, v: Vec<(u32, u32)>) -> bool {
    v.into_iter()
        .fold(false, |acc, (x, y)| acc || (x < limit && y < limit))
}

/// first_some
///
/// Purpose: Return the first `Some(T)` in the vector; `None` if all are `None`.
/// Params:  `v: Vec<Option<T>>`
/// Returns: `Option<T>`
///
/// Type:    `fn first_some<T>(v: Vec<Option<T>>) -> Option<T>`
pub fn first_some<T>(v: Vec<Option<T>>) -> Option<T> {
    v.into_iter()
        .fold(None, |acc, x| acc.or(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_word() {
        assert_eq!(smallest_word(vec!["see", "two", "ant"]), "see"); // all len=3 → first wins
        assert_eq!(smallest_word(vec!["alpha", "to", "be", "zen", "a"]), "a");
    }

    #[test]
    fn test_any_smaller() {
        assert!(any_smaller(5, vec![(1,4), (9,9)]));
        assert!(!any_smaller(3, vec![(1,5), (0,3)]));
        assert!(!any_smaller(10, vec![]));
    }

    #[test]
    fn test_first_some() {
        assert_eq!(first_some::<i32>(vec![]), None);
        assert_eq!(first_some(vec![None, None, Some(7), Some(9)]), Some(7));
        assert_eq!(first_some(vec![Some("a"), Some("b")]), Some("a"));
    }
}


fn main() {
    println!("{}", crate::smallest_word(vec!["rust", "go", "c"]));
    println!("{}", crate::any_smaller(5, vec![(1, 1), (10, 2)]));
    println!("{:?}", crate::first_some::<i32>(vec![None, Some(42), Some(7)]));
}
