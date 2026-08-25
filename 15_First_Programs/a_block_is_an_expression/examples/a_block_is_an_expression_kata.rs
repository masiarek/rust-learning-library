//! Kata solution: the semicolon that changed the type.
//!
//! Four parts, and the first one is a compile error worth causing on purpose.
//! (1) A tail expression grows a semicolon and the function's return type stops
//! matching. (2) A `mut` builder is sealed behind a block expression, so what
//! escapes is an immutable binding. (3) An assign-in-every-branch `mut` becomes
//! an `if` expression, because `if` is built from blocks and therefore has a
//! value. (4) A borrow is given an end by putting it in a block.
//!
//!   rustc --edition 2024 a_block_is_an_expression_kata.rs -o /tmp/abiek && /tmp/abiek

fn banner(title: &str) {
    println!("\n──── {title}");
}

/// The fixed version. With `;` after `total / n` this is `E0308`.
fn mean(scores: &[u32]) -> u32 {
    let total: u32 = scores.iter().sum();
    let n = scores.len() as u32;
    total / n
}

fn main() {
    banner("Part 1: the semicolon that changed the type");

    println!("  fn mean(scores: &[u32]) -> u32 {{");
    println!("      let total: u32 = scores.iter().sum();");
    println!("      let n = scores.len() as u32;");
    println!("      total / n;          <- one character, and the body is ()");
    println!("  }}");
    println!();
    println!("  error[E0308]: mismatched types");
    println!("     |    ------            ^^^ expected `u32`, found `()`");
    println!("     |    |");
    println!("     |    implicitly returns `()` as its body has no tail");
    println!("     |    or `return` expression");
    println!("     |     total / n;");
    println!("     |              - help: remove this semicolon to return this value");
    println!();
    println!("  Read the help line: rustc is not asking for a `return`. The body");
    println!("  is a block, the block's value is its tail, and a semicolon threw");
    println!("  the tail away. Without it:");
    println!("      mean(&[5, 3, 4]) = {}", mean(&[5, 3, 4]));

    banner("Part 2: the builder that hands out something immutable");

    let raw = [("Cara", 5), ("Ada", 4), ("Ben", 2), ("Dev", 4)];
    let cutoff = 4;

    //  Everything mutable happens inside the braces; an immutable Vec comes out.
    let through = {
        let mut v = Vec::new();
        for (name, score) in &raw {
            if *score >= cutoff {
                v.push(*name);
            }
        }
        v.sort_unstable();
        v
    };

    println!("  cutoff {cutoff} -> {through:?}");
    println!("  `mut` lived for six lines inside the block. The binding that");
    println!("  escaped is plain, so nothing below can push to it.");

    banner("Part 3: the branch that IS the value");

    for turnout in [61, 50, 12] {
        //  Not: let mut verdict = ""; if … { verdict = … } else { … }
        let verdict = if turnout >= 50 {
            "quorate"
        } else if turnout >= 25 {
            "advisory only"
        } else {
            "void"
        };
        println!("  turnout {turnout:>3}% -> {verdict}");
    }
    println!("  No `mut`, no placeholder value, and the compiler checks that");
    println!("  every arm produced one — a missing `else` would not compile.");

    banner("Part 4: the borrow that ends where you say");

    let mut ballots = vec![5, 3, 4];
    let top = {
        let view = &ballots; //     the borrow starts here...
        *view.iter().max().unwrap()
    }; //                           ...and cannot outlive this brace
    ballots.push(9); //             so the Vec is writable again immediately
    println!("  top before the push: {top}");
    println!("  ballots now: {ballots:?}");
    println!();
    println!("  Since 2018 a borrow usually ends at its last USE, so most code");
    println!("  no longer needs this. It is still the tool when the compiler");
    println!("  disagrees with you about where the last use was.");
}
