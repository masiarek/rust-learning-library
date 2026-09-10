//! Kata solution: `.` and `*` in a dozen lines, read with slice patterns —
//! and why `.` has to mean one character rather than one byte.
//!
//!   rustc --edition 2024 tiny_regex_kata.rs -o /tmp/trk && /tmp/trk
//!   rustc --edition 2024 --test tiny_regex_kata.rs -o /tmp/trkt && /tmp/trkt

fn is_match(s: &str, pattern: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    here(&s, &p, '.', '*')
}

/// One rule per arm, chosen by the pattern's first two items. An item
/// followed by the star may match zero copies (skip both) or one more copy
/// (consume one input and stay put); anything else must match exactly one.
/// Generic over the unit, so the same matcher can be run on bytes.
fn here<T: PartialEq + Copy>(s: &[T], p: &[T], any: T, star: T) -> bool {
    let one = |c: T| !s.is_empty() && (c == any || c == s[0]);
    match p {
        [] => s.is_empty(),
        [c, st, rest @ ..] if *st == star => here(s, rest, any, star) || (one(*c) && here(&s[1..], p, any, star)),
        [c, rest @ ..] => one(*c) && here(&s[1..], rest, any, star),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert!(is_match("aa", "a*"));
        assert!(is_match("ab", ".*"));
        assert!(is_match("aab", "c*a*b"));
        assert!(!is_match("mississippi", "mis*is*p*."));
        assert!(is_match("", "a*"));
        assert!(!is_match("", "a"));
    }
}

fn main() {
    println!("1. Adam's six cases");
    for (s, p, want) in [
        ("aa", "a*", true),
        ("ab", ".*", true),
        ("aab", "c*a*b", true),
        ("mississippi", "mis*is*p*.", false),
        ("", "a*", true),
        ("", "a", false),
    ] {
        let got = is_match(s, p);
        assert_eq!(got, want);
        println!("   {:<14} {:<13} {got}", format!("{s:?}"), format!("{p:?}"));
    }

    println!();
    println!("2. The unit decides what `.` means");
    println!("   {:<8} {:<9} {:>6} {:>6}", "text", "pattern", "chars", "bytes");
    for (s, p) in [("é", "."), ("é", ".."), ("日本", ".."), ("日本", "......")] {
        let chars = is_match(s, p);
        let bytes = here(s.as_bytes(), p.as_bytes(), b'.', b'*');
        println!("   {:<8} {:<9} {chars:>6} {bytes:>6}", format!("{s:?}"), format!("{p:?}"));
    }
    println!("   Over bytes, `.` is half an é and a third of 日. Collecting a Vec<char>");
    println!("   once, up front, is what makes the pattern mean what a reader reads.");

    println!();
    println!("3. What the slice patterns bought");
    println!("   [c, '*', rest @ ..] names the star case directly. The same logic over");
    println!("   indices has to check i + 1 < p.len() before it may even look at p[i + 1].");
}
