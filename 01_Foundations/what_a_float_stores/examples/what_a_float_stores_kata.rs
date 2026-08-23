//! Kata solution: the results table that would not sort.
//!
//! Five candidates, ten ballots, integer scores. Divide by the ballot count to
//! get an average and the column becomes `f64` — at which point `.sort()` and
//! `.sort_by_key()` both stop compiling:
//!
//!   error[E0277]: the trait bound `f64: Ord` is not satisfied
//!        |     rows.sort_by_key(|r| r.avg);
//!        |          ^^^^^^^^^^^ the trait `Ord` is not implemented for `f64`
//!
//! Three ways out, in increasing order of how much I like them.
//!
//!   rustc --edition 2024 what_a_float_stores_kata.rs -o /tmp/wafsk && /tmp/wafsk

use std::cmp::Ordering;
use std::panic;

#[derive(Debug, Clone)]
struct Row {
    name: &'static str,
    total: u32,   // sum of 0-5 scores: exact, and Ord
    ballots: u32, // how many ballots scored this candidate at all
}

impl Row {
    fn avg(&self) -> f64 {
        f64::from(self.total) / f64::from(self.ballots)
    }
}

fn table() -> Vec<Row> {
    vec![
        Row { name: "Alma",  total: 34, ballots: 10 },
        Row { name: "Bruno", total: 41, ballots: 10 },
        Row { name: "Cato",  total: 28, ballots: 10 },
        Row { name: "Delia", total: 41, ballots: 10 },
    ]
}

/// A candidate nobody scored. 0 / 0 is NaN — no panic, no error, just a value
/// that is neither less than nor greater than nor equal to any other.
fn with_an_unscored_candidate() -> Vec<Row> {
    let mut t = table();
    t.push(Row { name: "Emil", total: 0, ballots: 0 });
    t
}

fn names(rows: &[Row]) -> Vec<&'static str> {
    rows.iter().map(|r| r.name).collect()
}

fn main() {
    println!("the table, as integers — nothing here is inexact:");
    for r in table() {
        println!("  {:<6} total {:>3}  over {} ballots  = avg {}", r.name, r.total, r.ballots, r.avg());
    }

    println!("\nFIX A — partial_cmp().unwrap()");
    let mut a = table();
    a.sort_by(|x, y| y.avg().partial_cmp(&x.avg()).unwrap());
    println!("  {:?}", names(&a));
    println!("  works, and hides a panic: unwrap() is a claim that no NaN can reach here.");

    // Prove the claim is false the moment a candidate has no ballots.
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // the panic is expected; don't print rustc's report
    let boom = panic::catch_unwind(|| {
        let mut rows = with_an_unscored_candidate();
        rows.sort_by(|x, y| y.avg().partial_cmp(&x.avg()).unwrap());
        names(&rows)
    });
    panic::set_hook(hook);
    match boom {
        Ok(order) => println!("  with an unscored candidate: {order:?}"),
        Err(_) => println!("  with an unscored candidate: PANIC — 0/0 is NaN, partial_cmp gave None"),
    }

    println!("\nFIX B — total_cmp");
    let mut b = with_an_unscored_candidate();
    b.sort_by(|x, y| y.avg().total_cmp(&x.avg()));
    println!("  {:?}", names(&b));
    println!("  never panics: IEEE-754 totalOrder gives NaN a defined seat (last, here).");
    println!("  but read that last name twice — Emil is RANKED, not excluded.");

    println!("\nFIX C — do not divide at all");
    let mut c = with_an_unscored_candidate();
    // Every candidate is divided by the same ballot count, and dividing by a
    // positive constant cannot reorder anything. So the average was never the
    // thing being compared — the total was. Integers are Ord, Eq and exact.
    c.sort_by(|x, y| y.total.cmp(&x.total).then_with(|| x.name.cmp(y.name)));
    println!("  {:?}", names(&c));
    println!("  no float, no NaN, no unwrap, no tolerance — and a tie stays a tie:");
    let tied = c.windows(2).filter(|w| w[0].total == w[1].total).count();
    println!("  exact ties detected: {tied}  (Bruno and Delia, both 41)");

    println!("\nWHAT THE THREE COST");
    println!("  A  compiles, ranks, panics on data you have not seen yet");
    println!("  B  compiles, ranks, never panics — and silently ranks a NaN");
    println!("  C  compiles, ranks, cannot produce a NaN, and finds the tie");
    let same = names(&b)[..4] == names(&c)[..4];
    println!("  same first four either way? {same}");
    println!("  the float was never carrying information the integer did not have.");

    println!("\nAND THE COMPARISON THAT STARTED IT");
    let alma = table()[0].avg(); // 34 / 10
    let cato = table()[2].avg(); // 28 / 10
    println!("  Alma's average prints as {alma}, Cato's as {cato}");
    println!("  alma == 3.4                 : {}", alma == 3.4);
    println!("  stored value                : {alma:.20}");
    println!("  neither side is 3.4 — 34/10 is 17/5, and 5 is not a power of two.");
    println!("  the division and the literal just rounded to the SAME wrong f64,");
    println!("  which is luck, not correctness. One addition spends it:");
    println!("  alma + cato == 6.2          : {}", alma + cato == 6.2);
    println!("  alma + cato                 : {:.20}", alma + cato);
}

// Keeps `Ordering` in scope for readers who follow the sort_by signature.
#[allow(dead_code)]
fn ordering_reminder(a: f64, b: f64) -> Option<Ordering> {
    a.partial_cmp(&b)
}
