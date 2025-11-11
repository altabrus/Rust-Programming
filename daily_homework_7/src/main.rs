//! daily_homework_7 — generic 2D Direction with methods & traits
//!
//! Requirements covered:
//! - Generic struct `Direction<T>`
//! - A `print` method that formats components nicely
//! - An inherent `add` method (component-wise addition)
//! - Implement `Display` to use the same formatting as `print`
//! - Implement `Add` and `Sub` so `+` and `-` work
//!
//! Design notes:
//! - Methods add/subtract by value and return a new `Direction<T>`.
//! - Trait bounds are as lightweight as possible and added only where needed.

use std::fmt;
use std::ops::{Add, Sub};

/// Direction<T>
///
/// Purpose: Represent a 2D vector whose components share the same type.
/// Parameters:
///   - `x: T`, `y: T`
/// Type: `struct Direction<T>`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Direction<T> {
    pub x: T,
    pub y: T,
}

impl<T> Direction<T> {
    /// new
    ///
    /// Purpose: Construct a `Direction<T>` from its two components.
    /// Params: `x: T`, `y: T`
    /// Returns: `Direction<T>`
    /// Type: `fn new(x: T, y: T) -> Self`
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Direction<T>
where
    T: fmt::Display,
{
    /// print
    ///
    /// Purpose: Produce a nice string representation of the direction.
    /// (We return a `String` so `Display` can reuse this formatting.)
    /// Params: `&self`
    /// Returns: `String` like "⟨x, y⟩"
    /// Type: `fn print(&self) -> String`
    pub fn print(&self) -> String {
        format!("⟨{}, {}⟩", self.x, self.y)
    }
}

impl<T> Direction<T>
where
    T: Add<Output = T> + Clone,
{
    /// add
    ///
    /// Purpose: Add two directions component-wise.
    /// Params: `&self`, `other: &Direction<T>`
    /// Returns: `Direction<T>`
    /// Type: `fn add(&self, other: &Direction<T>) -> Direction<T>`
    pub fn add(&self, other: &Direction<T>) -> Direction<T> {
        Direction {
            x: self.x.clone() + other.x.clone(),
            y: self.y.clone() + other.y.clone(),
        }
    }
}

impl<T> fmt::Display for Direction<T>
where
    T: fmt::Display,
{
    /// Use the same formatting as `print()`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.print())
    }
}

impl<T> Add for Direction<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Direction<T>;

    /// Enable `a + b` for directions (component-wise).
    fn add(self, rhs: Direction<T>) -> Self::Output {
        Direction {
            x: self.x.clone() + rhs.x,
            y: self.y.clone() + rhs.y,
        }
    }
}

impl<T> Sub for Direction<T>
where
    T: Sub<Output = T> + Clone,
{
    type Output = Direction<T>;

    /// Enable `a - b` for directions (component-wise).
    fn sub(self, rhs: Direction<T>) -> Self::Output {
        Direction {
            x: self.x.clone() - rhs.x,
            y: self.y.clone() - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn display_and_print_i32() {
        let d = Direction::new(3, -4);
        assert_eq!(d.print(), "⟨3, -4⟩");
        assert_eq!(format!("{}", d), "⟨3, -4⟩");
    }

    #[test]
    fn add_method_i32() {
        let a = Direction::new(1, 2);
        let b = Direction::new(5, 7);
        let c = a.add(&b);
        assert_eq!(c, Direction::new(6, 9));
    }

    #[test]
    fn add_operator_i32() {
        let a = Direction::new(10, 3);
        let b = Direction::new(1, 2);
        let c = a + b;
        assert_eq!(c, Direction::new(11, 5));
    }

    #[test]
    fn sub_operator_i32() {
        let a = Direction::new(8, 5);
        let b = Direction::new(1, 9);
        let c = a - b;
        assert_eq!(c, Direction::new(7, -4));
    }

    #[test]
    fn works_with_f64() {
        let a = Direction::new(1.5_f64, -2.0);
        let b = Direction::new(0.5,  10.0);
        let s = a.clone() + b.clone();
        let d = a - b;
        assert_eq!(format!("{}", s), "⟨2, 8⟩");
        assert_eq!(format!("{}", d), "⟨1, -12⟩");
    }
}


fn main() {
    let a = Direction::new(3, 4);
    let b = Direction::new(1, -2);

    // Call your inherent method (borrowed args):
    println!("a.add(&b) = {}", <Direction<_>>::add(&a, &b));

    // Operator overloads (owned args):
    println!("a + b = {}", a.clone() + b.clone());
    println!("a - b = {}", a.clone() - b.clone());

    // Display uses your print formatting:
    println!("a = {}", a);
    println!("b = {}", b);
}
