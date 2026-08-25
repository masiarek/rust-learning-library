//! Kata solution: the tally that never tallied.
//!
//! A shadowed accumulator inside a loop. The compiler says nothing, the
//! per-ballot log looks fine, the report prints a real candidate's name — and
//! the number behind that name is zero. Then: which of clippy's three shadow
//! lints finds it, and what that one costs you elsewhere in this same file.
//!
//!   rustc --edition 2024 nothing_checks_a_shadow_kata.rs -o /tmp/ncask && /tmp/ncask

const CANDIDATES: [&str; 3] = ["Ada", "Ben", "Cara"];

/// Three ballots, 0–5 scores. Ben is the honest winner: 3 + 5 + 2 = 10,
/// against Ada's 9 and Cara's 6.
const BALLOTS: [[u32; 3]; 3] = [[5, 3, 0], [4, 5, 1], [0, 2, 5]];

fn banner(title: &str) {
    println!("\n──── {title}");
}

fn winner(totals: [u32; 3]) -> &'static str {
    let mut best = 0;
    for i in 1..totals.len() {
        if totals[i] > totals[best] {
            best = i;
        }
    }
    CANDIDATES[best]
}

// ─────────────────────────────────────────────────────────────── the bug
fn tally_buggy() -> [u32; 3] {
    let totals = [0u32; 3];
    for ballot in BALLOTS {
        // Reads the OUTER `totals` — which is still [0, 0, 0] — adds one
        // ballot, and drops the result at the closing brace.
        let totals = [
            totals[0] + ballot[0],
            totals[1] + ballot[1],
            totals[2] + ballot[2],
        ];
        println!("  counted {ballot:?}  running total {totals:?}");
    }
    totals
}

// ─────────────────────────────────────────────────────────────── the fixes
fn tally_mut() -> [u32; 3] {
    let mut totals = [0u32; 3];
    for ballot in BALLOTS {
        for i in 0..totals.len() {
            totals[i] += ballot[i];
        }
    }
    totals
}

fn tally_fold() -> [u32; 3] {
    BALLOTS.iter().fold([0u32; 3], |mut acc, ballot| {
        for i in 0..acc.len() {
            acc[i] += ballot[i];
        }
        acc
    })
}

fn tally_distinct_names() -> [u32; 3] {
    let mut running = [0u32; 3];
    for ballot in BALLOTS {
        for i in 0..running.len() {
            running[i] += ballot[i];
        }
    }
    running
}

// A CORRECT shadow, in the same file, for the lint to have an opinion about.
fn parse_quorum(raw: &str) -> u32 {
    let raw = raw.trim(); // &str -> &str
    let raw: u32 = raw.parse().unwrap_or(0); // &str -> u32, the Book's own idiom
    raw
}

fn main() {
    // ────────────────────────────────────────────────────────── 1
    banner("As shipped: a log that looks fine and a winner that is not");
    let totals = tally_buggy();
    println!("  Winner: {}", winner(totals));
    println!("      Nothing above looks alarming. Every ballot was counted, each");
    println!("      running total is a real number, and Ada is a real candidate.");
    println!("      Two things give it away, and only in hindsight:");
    println!("  totals = {totals:?}");
    println!("      Every 'running total' was just that ballot echoed back, and");
    println!("      the accumulator never left zero. `winner` broke the all-zero");
    println!("      tie by index, so the report named whoever was first in the");
    println!("      candidate list. The honest winner is Ben, with 10.");

    // ────────────────────────────────────────────────────────── 2
    banner("What the compiler had to say about it: nothing");
    println!("  $ rustc --edition 2024 nothing_checks_a_shadow_kata.rs");
    println!("  $                                    <- no output, exit 0");
    println!("      `unused_variables` cannot fire: the shadow is read on the");
    println!("      next line, by the log. There is no type error: both are");
    println!("      [u32; 3]. And there is no shadowing lint in rustc to fire in");
    println!("      the first place. Three near-misses, all accidents of shape —");
    println!("      drop the log line and the first one would have caught it.");

    // ────────────────────────────────────────────────────────── 3
    banner("Which clippy lint finds it (recorded from a real run on this file)");
    println!("  (clippy needs a cargo crate, so this file is src/main.rs there)");
    println!("  $ cargo clippy -- -W clippy::shadow_same");
    println!("  $                                    <- finds NOTHING");
    println!();
    println!("  $ cargo clippy -- -W clippy::shadow_unrelated");
    println!("  $                                    <- finds NOTHING");
    println!();
    println!("  $ cargo clippy -- -W clippy::shadow_reuse");
    println!("  warning: `totals` is shadowed");
    println!("    --> src/main.rs:36:13        <- the bug");
    println!("  warning: `raw` is shadowed");
    println!("    --> src/main.rs:78:9         <- parse_quorum, line 1");
    println!("  warning: `raw` is shadowed");
    println!("    --> src/main.rs:79:9         <- parse_quorum, line 2");
    println!("      parse_quorum(\"  42 \") = {}", parse_quorum("  42 "));
    println!("      ...which is correct code, flagged twice. It is the idiom");
    println!("      chapter 3.1 of the Book teaches.");

    // ────────────────────────────────────────────────────────── 4
    banner("The trade, stated plainly");
    println!("  shadow_same       `let x = x;`             junk always, the bug never");
    println!("  shadow_unrelated  `let x = something_else;` silent here");
    println!("  shadow_reuse      `let x = f(x);`          catches it — and the idiom");
    println!("      The only lint that would have caught this bug is the one that");
    println!("      also condemns the good use of the feature. All three are");
    println!("      allow-by-default `restriction` lints, which is clippy saying");
    println!("      the same thing in its own vocabulary: a style commitment, not");
    println!("      a bug filter. Turn `shadow_reuse` on and you have banned");
    println!("      `let x = x.trim().parse()?` across the crate. That can be a");
    println!("      trade worth making — a large team, a codebase full of loops —");
    println!("      but make it deliberately, not hoping to catch one accumulator.");

    // ────────────────────────────────────────────────────────── 5
    banner("Three fixes, and the one to ship");
    let (a, b, c) = (tally_mut(), tally_fold(), tally_distinct_names());
    println!("  1. `mut`, no shadow      -> {a:?}  winner {}", winner(a));
    println!("  2. fold, no accumulator  -> {b:?}  winner {}", winner(b));
    println!("  3. no name reused        -> {c:?}  winner {}", winner(c));
    println!("      Fix 2 is the one to ship, and the reason generalises past this");
    println!("      bug: it removes the BINDING, so the mistake has nowhere to");
    println!("      live. Fix 1 works, and is the honest counter-example to");
    println!("      'shadowing lets you keep everything immutable' — an");
    println!("      accumulator is supposed to survive the iteration, so a fresh");
    println!("      binding per pass is precisely the wrong tool. Fix 3 works and");
    println!("      relies on you not reusing a name, which is the discipline that");
    println!("      just failed.");

    println!("\n      The through-line: the compiler polices ownership, types and");
    println!("      exhaustiveness, and a shadow can be wrong in none of those");
    println!("      ways. When a shadow's type matches what it hides, you are the");
    println!("      only check there is.");
}
