//! daily_homework_5 — string and &str utilities
//!
//! This module provides simple text and file helpers that will later be useful
//! in the virtual-machine project.

use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// read_program_file
///
/// Purpose: Read a text file and return its lines as a vector of strings.
/// Parameters:  
/// - `filename: &str` — the file to read.
/// Returns:  
/// - `Result<Vec<String>, io::Error>` — Ok(lines) on success, or an error.
///
/// Type: `fn read_program_file(filename: &str) -> Result<Vec<String>, io::Error>`
pub fn read_program_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
}

/// is_keyword
///
/// Purpose: Determine whether a word is one of the reserved keywords.
/// Parameters:  
/// - `word: &str` — candidate word.
/// Returns:  
/// - `bool` — true if the word is a keyword.
///
/// Type: `fn is_keyword(word: &str) -> bool`
pub fn is_keyword(word: &str) -> bool {
    // all language keywords in a static array
    const KEYWORDS: [&str; 16] = [
        "and", "class", "else", "false", "for", "fun", "if",
        "nil", "or", "print", "return", "super", "this",
        "true", "var", "while",
    ];
    KEYWORDS.contains(&word)
}

/// split_string
///
/// Purpose: Split a string slice by any whitespace and return the words.
/// Parameters:  
/// - `input: &str`
/// Returns:  
/// - `Vec<String>` — vector of the whitespace-separated words.
///
/// Type: `fn split_string(input: &str) -> Vec<String>`
pub fn split_string(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_keyword_true_false() {
        assert!(is_keyword("and"));
        assert!(is_keyword("while"));
        assert!(!is_keyword("loop"));
        assert!(!is_keyword("And")); // case sensitive
    }

    #[test]
    fn test_split_string_basic() {
        let text = "hello   world\tfrom\nrust";
        let words = split_string(text);
        assert_eq!(words, vec!["hello", "world", "from", "rust"]);
    }

    #[test]
    fn test_read_program_file_temp() {
        // make a temp file to verify reading works
        let path = "test_input.txt";
        std::fs::write(path, "line1\nline2\nline3").unwrap();
        let lines = read_program_file(path).unwrap();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
        std::fs::remove_file(path).unwrap();
    }
}

fn main() {
    let text = "print return var while fun";
    println!("Split words: {:?}", crate::split_string(text));

    println!("Is 'print' a keyword? {}", crate::is_keyword("print"));
    println!("Is 'dog' a keyword? {}", crate::is_keyword("dog"));

    match crate::read_program_file("Cargo.toml") {
        Ok(lines) => println!("Read {} lines from Cargo.toml", lines.len()),
        Err(e) => eprintln!("Error: {e}"),
    }
}
