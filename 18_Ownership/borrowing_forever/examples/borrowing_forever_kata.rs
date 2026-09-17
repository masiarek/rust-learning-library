//! Kata solution: a word cursor whose `next_word` can be called in a loop, and
//! whose words outlive the cursor. The fix is one lifetime moved: the result
//! borrows from the text (`'a`), not from the cursor.
//!
//!   rustc --edition 2024 borrowing_forever_kata.rs -o /tmp/bfk && /tmp/bfk
//!   rustc --edition 2024 --test borrowing_forever_kata.rs -o /tmp/bfkt && /tmp/bfkt

struct Cursor<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(text: &'a str) -> Cursor<'a> {
        Cursor { text, pos: 0 }
    }

    /// Was `fn next_word(&'a mut self) -> Option<&'a str>`: the first call
    /// borrowed the cursor for all of `'a`, so the second was E0499.
    /// Eliding both (`&mut self -> Option<&str>`) still fails in the loop,
    /// because each word then borrows the cursor and the Vec keeps them.
    /// The words come out of `self.text`, which is `&'a str`, so say so.
    fn next_word(&mut self) -> Option<&'a str> {
        let text: &'a str = self.text;
        let rest = &text[self.pos..];
        let start = rest.find(|c: char| !c.is_whitespace())?;
        let len = rest[start..].find(char::is_whitespace).unwrap_or(rest.len() - start);
        self.pos += start + len;
        Some(&rest[start..start + len])
    }
}

fn words(text: &str) -> Vec<&str> {
    let mut cursor = Cursor::new(text);
    let mut found = Vec::new();
    while let Some(word) = cursor.next_word() {
        found.push(word);
    }
    found // the cursor is dropped here; the words are not tied to it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_every_word() {
        assert_eq!(words("borrow it forever"), ["borrow", "it", "forever"]);
        assert_eq!(words("  spaced   out  "), ["spaced", "out"]);
        assert!(words("").is_empty());
    }

    #[test]
    fn cursor_is_usable_between_calls() {
        let mut cursor = Cursor::new("one two");
        assert_eq!(cursor.next_word(), Some("one"));
        assert_eq!(cursor.pos, 3);
        assert_eq!(cursor.next_word(), Some("two"));
        assert_eq!(cursor.next_word(), None);
    }
}

fn main() {
    println!("1. The loop that was E0499");
    let text = String::from("borrow it forever");
    let found = words(&text);
    println!("   words({text:?}) = {found:?}");

    println!();
    println!("2. The cursor is still yours between calls");
    let mut cursor = Cursor::new("  spaced   out  ");
    let first = cursor.next_word();
    println!("   first = {first:?}, cursor.pos = {}", cursor.pos);
    let second = cursor.next_word();
    println!("   second = {second:?}, cursor.pos = {}", cursor.pos);
    drop(cursor);
    println!("   after drop(cursor), both words still print: {first:?} {second:?}");

    println!();
    println!("3. Three signatures, run");
    println!("   fn next_word(&'a mut self) -> Option<&'a str>  E0499 in the loop");
    println!("   fn next_word(&mut self) -> Option<&str>        E0499 in the loop");
    println!("   fn next_word(&mut self) -> Option<&'a str>     compiles, as above");
}
