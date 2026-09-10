//! Kata solution: `replace` without `replace` — then the empty pattern that
//! never lets the loop move.
//!
//!   rustc --edition 2024 replace_all_kata.rs -o /tmp/rak && /tmp/rak
//!   rustc --edition 2024 --test replace_all_kata.rs -o /tmp/rakt && /tmp/rakt

/// Every non-overlapping `from`, left to right, copied into one new buffer.
/// `find` hands back a BYTE offset, and slicing there is safe: a match inside
/// a valid `&str` can only start and end on character boundaries.
fn replace_all(s: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        // `find("")` matches at the start of whatever is left, so the loop
        // below would never shorten `rest`. std's `replace` treats the empty
        // pattern as matching between every pair of characters; do the same.
        let mut out = String::with_capacity(s.len() + to.len() * (s.chars().count() + 1));
        out.push_str(to);
        for c in s.chars() {
            out.push(c);
            out.push_str(to);
        }
        return out;
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(at) = rest.find(from) {
        out.push_str(&rest[..at]);
        out.push_str(to);
        rest = &rest[at + from.len()..];
    }
    out.push_str(rest);
    out
}

/// The loop above with the empty-pattern guard removed, stopped after `cap`
/// turns so the demonstration can report what it did instead of hanging.
fn naive_turns(s: &str, from: &str, cap: usize) -> (usize, usize) {
    let mut rest = s;
    let mut turns = 0;
    while let Some(at) = rest.find(from) {
        rest = &rest[at + from.len()..];
        turns += 1;
        if turns == cap {
            break;
        }
    }
    (turns, rest.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_all() {
        assert_eq!(replace_all("hello world", "world", "rust"), "hello rust");
        assert_eq!(replace_all("aaa", "a", "b"), "bbb");
        assert_eq!(replace_all("hello", "x", "y"), "hello");
    }
}

fn main() {
    println!("1. Adam's three cases, checked against std's own replace");
    for (s, from, to) in [("hello world", "world", "rust"), ("aaa", "a", "b"), ("hello", "x", "y")] {
        let mine = replace_all(s, from, to);
        assert_eq!(mine, s.replace(from, to));
        println!("   {:<15} {:<9} {:<8} -> {mine:?}", format!("{s:?}"), format!("{from:?}"), format!("{to:?}"));
    }

    println!();
    println!("2. Matches are taken left to right, and never overlap");
    for (s, from, to) in [("aaaa", "aa", "b"), ("aaa", "aa", "b"), ("banana", "ana", "_")] {
        let mine = replace_all(s, from, to);
        assert_eq!(mine, s.replace(from, to));
        println!("   {:<15} {:<9} {:<8} -> {mine:?}", format!("{s:?}"), format!("{from:?}"), format!("{to:?}"));
    }
    println!("   \"banana\" holds \"ana\" twice, overlapping at the middle \"a\". Once the");
    println!("   first match is consumed the second no longer exists: one replacement.");

    println!();
    println!("3. The empty pattern, and the loop that never moves");
    let (turns, left) = naive_turns("abc", "", 10);
    println!("   without the guard: stopped after {turns} turns, {left} bytes still left");
    println!("   \"abc\".find(\"\") = {:?}", "abc".find(""));
    println!("   std  {:?}", "abc".replace("", "-"));
    println!("   mine {:?}", replace_all("abc", "", "-"));
    assert_eq!(replace_all("abc", "", "-"), "abc".replace("", "-"));
    assert_eq!(replace_all("", "", "-"), "".replace("", "-"));
    println!("   An empty needle is found at offset 0 of every remainder, so cutting");
    println!("   from.len() bytes off the front cuts nothing. Adam's tests never pass");
    println!("   an empty `from`, which is exactly how this loop ships.");

    println!();
    println!("4. Byte offsets, and why slicing at them is safe");
    let s = "żółw i żółw";
    println!("   {s:?}.find(\"ół\") = {:?} — a byte offset, and a char boundary", s.find("ół"));
    println!("   {:?}", replace_all(s, "ół", "OL"));
    assert_eq!(replace_all(s, "ół", "OL"), s.replace("ół", "OL"));
}
