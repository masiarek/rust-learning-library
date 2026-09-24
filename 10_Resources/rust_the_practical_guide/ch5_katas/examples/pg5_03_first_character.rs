//! Exercise 3 (§5.7): the first character of a `Vec`, or nothing.
//!
//! The one-word fix is `Some(character)`, a pattern that names the payload.
//! The function itself is the book's with its two clippy warnings fixed, and
//! below it the same function the way std would write it.
//!
//!   rustc --edition 2024 pg5_03_first_character.rs -o /tmp/pg503 && /tmp/pg503

/// A slice parameter instead of `&Vec<char>`, and `is_empty` instead of `len() > 0`.
fn first_character(chars: &[char]) -> Option<char> {
    if chars.is_empty() {
        None
    } else {
        Some(chars[0])
    }
}

/// The same thing in one call: `first` gives `Option<&char>`, `copied` makes it `Option<char>`.
fn first_character_std(chars: &[char]) -> Option<char> {
    chars.first().copied()
}

fn report(chars: &[char]) {
    match first_character(chars) {
        Some(character) => println!("First character: {character}"),
        None => println!("Empty array"),
    }
}

fn main() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    let no_chars: Vec<char> = Vec::new();

    report(&my_chars);
    report(&no_chars);

    println!("first().copied(): {:?} and {:?}", first_character_std(&my_chars), first_character_std(&no_chars));
}
