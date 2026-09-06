//! Kata solution: the offset that is not an index, `strip_prefix` as the honest
//! form of `starts_with`, and the three helpers you write once you know you
//! cannot name `Pattern`.
//!
//!   rustc --edition 2024 searching_a_string_kata.rs -o /tmp/sask && /tmp/sask

/// The *character* index of the first match, for a message a person reads.
/// `find` reports bytes; converting costs a walk of everything before the
/// match, which is why std hands you the cheap number and lets you choose.
fn char_index_of(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle).map(|at| haystack[..at].chars().count())
}

/// Three shapes that stand in for a `P: Pattern` you are not allowed to write.
mod helpers {
    /// 1. Take the concrete shape the caller actually uses.
    pub fn after_str<'a>(text: &'a str, marker: &str) -> Option<&'a str> {
        text.find(marker).map(|at| &text[at + marker.len()..])
    }

    /// 2. Take a closure -- that covers the char-predicate shape, which is the
    ///    one worth being generic over.
    pub fn from_first(text: &str, pred: impl FnMut(char) -> bool) -> Option<&str> {
        text.find(pred).map(|at| &text[at..])
    }

    /// 3. Do not take a pattern at all. Take the answer, and let the caller
    ///    pick the search.
    pub fn from_offset(text: &str, at: usize) -> &str {
        &text[at..]
    }
}

fn main() {
    println!("1. The two numbers, and why the bug hides");
    for (text, needle) in [("disk sda1 full", "full"), ("dysk żółw pełny", "pełny")] {
        let bytes = text.find(needle);
        let chars = char_index_of(text, needle);
        let same = if bytes == chars { "agree" } else { "DIFFER" };
        println!("   {:<20} find {:?}  chars {:?}  {}", format!("{text:?}"), bytes, chars, same);
    }
    println!("   Every ASCII test agrees, so a wrong choice ships. The first row of");
    println!("   real data with a two-byte letter in it is where you find out.");
    println!("   Rule of thumb: bytes for slicing, chars for telling a person.");

    println!("\n2. starts_with, then a constant you have to keep right");
    let path = "/dev/sda1";
    let by_hand = if path.starts_with("/dev/") { &path[4..] } else { path };
    let by_strip = path.strip_prefix("/dev/").unwrap_or(path);
    println!("   &path[4..]             {by_hand:?}   <- \"/dev/\" is 5 bytes, not 4");
    println!("   strip_prefix(\"/dev/\")  {by_strip:?}");
    println!("   The two lines carry the same string; only one of them carries it");
    println!("   twice. strip_prefix has no length to disagree with.");

    println!("\n3. The trait you can use but cannot name");
    println!("   fn occurs<P: Pattern>(..) is E0658 on stable, so:");
    println!("   after_str(\"a=1;b=2\", \"b=\")            {:?}",
        helpers::after_str("a=1;b=2", "b="));
    println!("   from_first(\"timeout = 30s\", numeric)  {:?}",
        helpers::from_first("timeout = 30s", char::is_numeric));
    let text = "timeout = 30s";
    println!("   from_offset(text, rfind(' ') + 1)     {:?}",
        text.rfind(' ').map(|at| helpers::from_offset(text, at + 1)));
    println!("   The third is the one to reach for when the caller already knows");
    println!("   where to look: it is the only one of the three that cannot search");
    println!("   for the wrong thing.");
}
