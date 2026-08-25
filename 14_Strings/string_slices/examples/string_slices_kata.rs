//! Kata solution: cut a string in half without panicking.
//!
//!   rustc --edition 2024 string_slices_kata.rs -o /tmp/ssk && /tmp/ssk

use std::panic;

/// The obvious version. Correct for ASCII, a panic for everything else.
fn halves_naive(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    (&s[..mid], &s[mid..])
}

/// Walk back from the midpoint until the index is a real char boundary.
fn halves(s: &str) -> (&str, &str) {
    let mut mid = s.len() / 2;
    while mid > 0 && !s.is_char_boundary(mid) {
        mid -= 1;
    }
    (&s[..mid], &s[mid..])
}

/// Same idea, expressed as a search through the boundaries the string reports.
fn halves_via_indices(s: &str) -> (&str, &str) {
    let target = s.len() / 2;
    let mid = s
        .char_indices()
        .map(|(i, _)| i)
        .take_while(|&i| i <= target)
        .last()
        .unwrap_or(0);
    (&s[..mid], &s[mid..])
}

fn try_naive(s: &str) -> String {
    panic::set_hook(Box::new(|_| {}));
    let r = panic::catch_unwind(|| {
        let (a, b) = halves_naive(s);
        format!("{a:?} + {b:?}")
    });
    let _ = panic::take_hook();
    r.unwrap_or_else(|_| "PANIC — byte index is not a char boundary".to_string())
}

fn main() {
    let cases = ["bete noir", "bête noir", "naïve café", "日本語"];

    println!("naive: &s[..len/2]");
    for s in cases {
        println!("   {:?}  ({} bytes)  ->  {}", s, s.len(), try_naive(s));
    }

    println!("\nis_char_boundary, walking back:");
    for s in cases {
        let (a, b) = halves(s);
        println!("   {:?}  ->  {:?} + {:?}", s, a, b);
    }

    println!("\nchar_indices, same answers:");
    for s in cases {
        let (a, b) = halves_via_indices(s);
        println!("   {:?}  ->  {:?} + {:?}", s, a, b);
    }

    println!("\nWhy 'naive' passes and 'naïve' does not:");
    for s in ["naive café", "naïve café"] {
        let mid = s.len() / 2;
        println!("   {:?}  len {}  mid {}  boundary? {}", s, s.len(), mid, s.is_char_boundary(mid));
    }
    println!("   A test suite of ASCII names never finds this. One accent does.");
}
