//! `_` is a pattern that matches anything and binds nothing.
//!
//! Everything surprising about it follows from the second half. A pattern that
//! binds nothing takes ownership of nothing, so `let _ = s;` does not move `s`
//! — and a temporary it was handed has nowhere to live, so it is dropped at the
//! semicolon instead of at the end of the block.
//!
//!   rustc --edition 2024 the_wildcard.rs -o /tmp/wc && /tmp/wc

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("      drop({})", self.0);
    }
}

fn make(name: &'static str) -> Noisy {
    Noisy(name)
}

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

#[must_use]
fn tally(scores: &[u8]) -> u32 {
    scores.iter().map(|s| u32::from(*s)).sum()
}

fn main() {
    // ───────────────────────────────────────────────────────────── 1
    banner(1, "`_` binds nothing, so it moves nothing");
    let ballot = String::from("5,3,0");
    let _ = ballot; // NOT a move: `_` never took ownership
    println!("  let ballot = String::from(\"5,3,0\");");
    println!("  let _ = ballot;");
    println!("  ballot is still usable afterwards: {ballot:?}");
    println!("      Replace `_` with any name — `let _x = ballot;` — and the");
    println!("      next line stops compiling with E0382, moved value. The");
    println!("      underscore is not a short name. It is the absence of one.");

    // ───────────────────────────────────────────────────────────── 2
    banner(2, "...which is why it changes WHEN a temporary dies");
    println!("  A: let _  = make(\"temp\");     a temporary, matched by `_`");
    {
        let _ = make("temp");
        println!("      ...end of the statement reached");
    }
    println!("  B: let _g = make(\"temp\");     the same temporary, bound");
    {
        let _g = make("temp");
        println!("      ...end of the statement reached");
    }
    println!("      A dropped BEFORE the next line ran; B survived to the end");
    println!("      of its block. Nothing owned A, so it died as a temporary");
    println!("      at the semicolon. That one character is the most expensive");
    println!("      mistake in Rust: `let _ = mutex.lock();` unlocks instantly.");

    // ───────────────────────────────────────────────────────────── 3
    banner(3, "But it does NOT shorten the life of a value that is already owned");
    println!("  C: let s = make(\"named\"); let _ = s;");
    {
        let s = make("named");
        let _ = s;
        println!("      ...after `let _ = s;`");
    }
    println!("      `s` still owns it, so it drops at the end of the block, as");
    println!("      `s` always would. A and C look identical and differ in one");
    println!("      thing: whether the right-hand side was a temporary or a");
    println!("      place. `_` never extends a lifetime and never shortens one;");
    println!("      it declines to participate, and the value keeps its owner.");

    // ───────────────────────────────────────────────────────────── 4
    banner(4, "`_ = expr;` with no `let` is an assignment, not a binding");
    _ = tally(&[5, 3, 0]);
    println!("  #[must_use] fn tally(&[u8]) -> u32");
    println!("  _ = tally(&[5, 3, 0]);      the warning is answered, in one line");
    println!("      Without it, `tally(&[5, 3, 0]);` warns:");
    println!("        warning: unused return value ... that must be used");
    println!("      `let _ = ...` says the same thing and is older; `_ = ...`");
    println!("      is destructuring assignment, and it reads as what it is —");
    println!("      an assignment to nowhere. Same drop timing as `let _ =`.");

    // ───────────────────────────────────────────────────────────── 5
    banner(5, "`_` and `_name` answer two different questions");
    let scores = [5u8, 3, 0];
    let mut counted = 0;
    for _ in &scores {
        counted += 1;
    }
    let _unread = tally(&scores);
    println!("  for _ in &scores       {counted} iterations, no binding made");
    println!("  let _unread = tally(..)  a real binding, silent about being unused");
    println!("      `_`      = 'there is no value here to name'  — no binding");
    println!("      `_name`  = 'a value I am deliberately not reading' — a binding,");
    println!("                 which owns, drops at the end of scope, and can be");
    println!("                 renamed back into use later. Only one of the two");
    println!("                 is safe to put a lock guard in, and it is `_name`.");

    // ───────────────────────────────────────────────────────────── 6
    banner(6, "The other places the same glyph shows up");
    let pair = (7u8, 9u8);
    let (_, second) = pair;
    let list = [1u8, 2, 3, 4];
    let summary = match list {
        [first, .., last] => format!("{first}..{last}"),
    };
    let widths: Vec<_> = scores.iter().map(|_| 1u8).collect();
    println!("  let (_, second) = pair;        second = {second}");
    println!("  [first, .., last]              {summary}   `..` is 'the rest',");
    println!("                                 `_` is 'exactly one, unnamed'");
    println!("  |_| 1u8                        a closure ignoring its argument");
    println!("  Vec<_>                         {:?} — a DIFFERENT `_`: an", widths);
    println!("                                 inference hole in a TYPE, not a");
    println!("                                 pattern. Same character, unrelated");
    println!("                                 feature; it asks the compiler to");
    println!("                                 fill the type in rather than to");
    println!("                                 discard a value.");
}
