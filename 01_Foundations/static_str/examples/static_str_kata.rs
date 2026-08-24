//! Kata solution: three ways to return a label, and what each one costs.
//!
//!   rustc --edition 2024 static_str_kata.rs -o /tmp/stk && /tmp/stk

// The naive version does not compile:
//
//   fn label_naive(n: u32) -> &'static str {
//       let built = format!("row {n}");
//       &built
//   }
//
//   error[E0515]: cannot return reference to local variable `built`
//    --> src/main.rs:3:5
//     |
//   3 |     &built
//     |     ^^^^^^ returns a reference to data owned by the current function

/// Fix 1 — every answer is a literal, so nothing is built at runtime.
/// Works only because the set of labels is closed.
fn label_fixed(n: u32) -> &'static str {
    match n {
        0 => "header",
        1 => "first row",
        2 => "second row",
        _ => "later row",
    }
}

/// Fix 2 — build it, then promise never to free it.
/// Honest &'static str, and a permanent leak of one allocation per call.
fn label_leaked(n: u32) -> &'static str {
    format!("row {n}").leak()
}

/// Fix 3 — hand the caller the buffer and let them drop it.
/// Not &'static str at all, and almost always the right answer.
fn label_owned(n: u32) -> String {
    format!("row {n}")
}

fn main() {
    println!("Fix 1 — match to literals, -> &'static str");
    for n in [0, 1, 2, 7] {
        println!("   label_fixed({n}) = {:?}", label_fixed(n));
    }
    println!("   cost: nothing. limit: cannot name the row number.");

    println!("\nFix 2 — leak, -> &'static str");
    for n in [0, 1, 2, 7] {
        println!("   label_leaked({n}) = {:?}", label_leaked(n));
    }
    println!("   cost: 4 allocations that are never freed. Called in a loop, that is");
    println!("   an unbounded leak — fine for a value computed once at startup, not");
    println!("   for one computed per row.");

    println!("\nFix 3 — return String");
    for n in [0, 1, 2, 7] {
        let owned = label_owned(n);
        println!("   label_owned({n}) = {:?}  ({} bytes, dropped at end of scope)", owned, owned.len());
    }
    println!("   cost: one allocation per call, freed. This is the one to ship.");

    println!("\nWhich to reach for:");
    println!("   closed set of answers        -> &'static str, all arms literals");
    println!("   computed once, lives forever -> .leak(), and say so in a comment");
    println!("   computed per call            -> String");
    println!("   The mistake is reading &'static str as \"a fast string\". It is a");
    println!("   promise about lifetime, and the only free way to keep it is a literal.");
}
