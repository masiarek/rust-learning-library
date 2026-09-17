//! Kata solution: a log that owns its text and knows its longest line, built
//! by a constructor, moved into a Vec and mutated. The self-referential
//! version could do none of that; a range per line can do all three.
//!
//!   rustc --edition 2024 self_referential_structs_kata.rs -o /tmp/srsk && /tmp/srsk
//!   rustc --edition 2024 --test self_referential_structs_kata.rs -o /tmp/srskt && /tmp/srskt

use std::ops::Range;

/// Was `struct Log<'a> { text: String, longest: &'a str }`, whose `new` was
/// E0505 (moving `log` out while `log.text` is borrowed) and E0515
/// (returning a value that references the local `log.text`).
struct Log {
    text: String,
    longest: Range<usize>,
}

fn longest_line(text: &str) -> Range<usize> {
    let mut best = 0..0;
    let mut start = 0;
    for line in text.split('\n') {
        if line.len() > best.len() {
            best = start..start + line.len();
        }
        start += line.len() + 1;
    }
    best
}

impl Log {
    fn new(text: String) -> Log {
        let longest = longest_line(&text);
        Log { text, longest }
    }

    fn longest(&self) -> &str {
        &self.text[self.longest.clone()]
    }

    /// Mutating the text invalidates the range, so recompute it here, in the
    /// one method allowed to change `text`.
    fn push_line(&mut self, line: &str) {
        self.text.push('\n');
        self.text.push_str(line);
        self.longest = longest_line(&self.text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_longest_line() {
        assert_eq!(Log::new(String::from("ok\nERROR disk full\nok")).longest(), "ERROR disk full");
        assert_eq!(Log::new(String::from("same\nsize")).longest(), "same");
        assert_eq!(Log::new(String::new()).longest(), "");
    }

    #[test]
    fn survives_a_move_and_a_push() {
        let mut logs = vec![Log::new(String::from("a\nbb"))];
        logs[0].push_line("cccc");
        assert_eq!(logs[0].longest(), "cccc");
    }
}

fn main() {
    println!("1. Built by a constructor and returned");
    let log = Log::new(String::from("ok\nERROR disk full\nok"));
    println!("   longest = {:?}", log.longest());

    println!();
    println!("2. Moved into a Vec");
    let mut logs = vec![log];
    println!("   logs[0].longest = {:?}", logs[0].longest());

    println!();
    println!("3. Mutated, and the range recomputed");
    logs[0].push_line("WARNING disk nearly full again");
    println!("   longest = {:?}", logs[0].longest());
    println!("   range   = {:?}", logs[0].longest);
}
