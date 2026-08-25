//! Kata solution: what `if let` stops checking.
//!
//!   rustc --edition 2024 if_let_kata.rs -o /tmp/ilk && /tmp/ilk

#[derive(Debug)]
enum Status {
    Counting,
    Certified { winner: &'static str },
    // Added later — the whole point of the exercise. A `match` on Status stops
    // compiling the moment this line appears. Every `if let` keeps compiling.
    Contested { by: &'static str },
}

/// Exhaustive: the compiler made me come back and handle Contested.
fn announce(s: &Status) -> String {
    match s {
        Status::Counting => "still counting".to_string(),
        Status::Certified { winner } => format!("{winner} has won"),
        Status::Contested { by } => format!("result challenged by {by}"),
    }
}

/// One arm, and silence for everything else — which is right here, because
/// "print a banner only when we have a winner" genuinely has one case.
fn banner(s: &Status) {
    if let Status::Certified { winner } = s {
        println!("  ★ {winner} ★");
    }
}

/// The same shape, used wrongly: this one *looks* like it covers the states,
/// and quietly says nothing at all for Contested.
fn misleading(s: &Status) -> String {
    if let Status::Certified { winner } = s {
        format!("{winner} has won")
    } else {
        "still counting".to_string() // a lie for Contested
    }
}

fn main() {
    let states = [
        Status::Counting,
        Status::Certified { winner: "Ada" },
        Status::Contested { by: "Ben" },
    ];

    println!("match — the compiler made me handle the new variant:");
    for s in &states {
        println!("  {:<28} -> {}", format!("{s:?}"), announce(s));
    }

    println!("\nif let, used for what it is good at (one case, silence otherwise):");
    for s in &states {
        banner(s);
    }
    println!("  (only one banner printed, and that was the intent)");

    println!("\nif let/else, used as a substitute for match:");
    for s in &states {
        println!("  {:<28} -> {}", format!("{s:?}"), misleading(s));
    }
    println!("      The last line is wrong, and nothing warned. That is the");
    println!("      exhaustiveness you traded away for the deleted arm.");

    println!("\nlet-else keeps the happy path at the left margin:");
    for s in &states {
        println!("  {}", certified_or_bail(s));
    }
}

fn certified_or_bail(s: &Status) -> String {
    let Status::Certified { winner } = s else {
        return "  (not certified — nothing to print)".to_string();
    };
    format!("  certified: {winner}")
}
