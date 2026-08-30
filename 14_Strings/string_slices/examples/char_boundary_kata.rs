//! Kata solution: four ways to meet a char boundary — floor to it, walk off it
//! on purpose, count characters instead of bytes, and ask before you split.
//!
//!   rustc --edition 2024 char_boundary_kata.rs -o /tmp/cbk && /tmp/cbk

use std::panic;

/// Slice up to `n` bytes, backing up to the last legal boundary if `n` lands
/// inside a character. Hand-rolled, so the rule is visible: walk down until
/// `is_char_boundary` agrees. 0 is always a boundary, so this always terminates.
fn safe_prefix(s: &str, n: usize) -> &str {
    let mut end = n.min(s.len());
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// The same thing std does for you, stable since 1.91.
fn safe_prefix_std(s: &str, n: usize) -> &str {
    &s[..s.floor_char_boundary(n)]
}

/// The first `n` *characters* — a count, not an offset. `char_indices` gives the
/// byte where character `n` starts; no character means take the whole string.
fn first_chars(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((byte, _)) => &s[..byte],
        None => s,
    }
}

/// Split in two at a byte index, or say it cannot be done there.
fn safe_split(s: &str, at: usize) -> Option<(&str, &str)> {
    if at <= s.len() && s.is_char_boundary(at) {
        Some((&s[..at], &s[at..]))
    } else {
        None
    }
}

/// Run `f` and report the panic message instead of dying. The hook is silenced
/// first so the message arrives on stdout, in order, rather than on stderr.
fn catch(f: impl FnOnce() + panic::UnwindSafe) -> String {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(hook);
    match result {
        Ok(()) => "no panic".to_string(),
        Err(e) => e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| "<non-string panic payload>".to_string()),
    }
}

fn main() {
    let jp = "こんにちは";
    let mixed = "café ☕ time";

    println!("1. The safe slicer — back up to the last legal boundary");
    println!("   {jp:?} is {} bytes, 5 characters of 3 bytes each", jp.len());
    for n in 0..=9 {
        println!("   n={n}  floor {}  ->  {:?}", jp.floor_char_boundary(n), safe_prefix(jp, n));
    }
    println!("   hand-rolled and std agree everywhere: {}",
        (0..=jp.len()).all(|n| safe_prefix(jp, n) == safe_prefix_std(jp, n)));
    println!("   n past the end is clamped, not a panic: {:?}", safe_prefix(jp, 999));

    println!("\n2. The panic trap — slice straight through a character");
    println!("   &jp[0..2] panics: {}", catch(|| {
        let _ = &jp[0..2];
    }));
    println!("   &jp[1..6] panics: {}", catch(|| {
        let _ = &jp[1..6];
    }));
    println!("   This is a runtime panic, not a compile error: the compiler knows");
    println!("   the type is a string, never which bytes are in it. Index arithmetic");
    println!("   on text is the one place strings stop protecting you.");
    println!("   (Caught with catch_unwind here only so the program can carry on.)");

    println!("\n3. The first three CHARACTERS");
    for t in [jp, mixed, "ab", ""] {
        println!("   first_chars({:?}, 3) = {:?}", t, first_chars(t, 3));
    }
    println!("   &s[..3] would have taken three BYTES instead: one character of");
    println!("   {jp:?}, and on {:?} a panic — {}", "ab🦀", catch(|| {
        let _ = &"ab🦀"[..3];
    }));

    println!("\n4. Ask before you split");
    for at in [0, 3, 4, 6, 15, 16] {
        match safe_split(jp, at) {
            Some((a, b)) => println!("   at={at:<2} Some(({a:?}, {b:?}))"),
            None => println!("   at={at:<2} None   <- inside a character, or past the end"),
        }
    }
    println!("   std has this too: split_at_checked(4) = {:?}", jp.split_at_checked(4));
    println!("   and the unchecked split_at(4) would panic, exactly like the slice.");
}
