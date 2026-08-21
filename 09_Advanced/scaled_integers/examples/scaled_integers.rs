//! One round of a proportional score count, tallied three ways.
//!
//! The election is real arithmetic, not a contrivance: Reweighted Range Voting
//! gives every ballot a weight of `C / (C + S)`, where `C` is the maximum score
//! and `S` is the total this ballot already spent on winners. So the weights are
//! fractions, and the count has to add them up.
//!
//! `f64` is fast and gets it wrong. Exact rationals get it right and call `gcd`
//! on every operation. Scaling by a denominator fixed before the count starts
//! gets it right in pure integer arithmetic, which is the point of the page.

use std::sync::atomic::{AtomicU64, Ordering};

/// Maximum score on the ballot. STAR ballots are 0-5.
const C: i128 = 5;

/// Seats in this election. Bounds `S`, and so bounds every denominator.
const SEATS: i128 = 5;

// ---------------------------------------------------------------- the election

/// Seat 1 went to Wren. These are the scores the six ballots gave *Wren*, which
/// is what sets each ballot's weight for seat 2: `5 / (5 + s)`.
const SPENT_ON_WREN: [i128; 6] = [5, 4, 3, 2, 1, 0];

/// The two candidates still in for seat 2, as scored by those same six ballots.
const ALMA: [i128; 6] = [0, 0, 1, 1, 1, 5];
const BRUNO: [i128; 6] = [0, 3, 1, 1, 5, 0];

// ------------------------------------------------------------ counting the gcd

/// Every call to `gcd` in this program goes through here, so the page can state
/// how many there were rather than assert that there are "a lot".
static GCD_CALLS: AtomicU64 = AtomicU64::new(0);

fn gcd(mut a: i128, mut b: i128) -> i128 {
    GCD_CALLS.fetch_add(1, Ordering::Relaxed);
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

fn gcd_calls() -> u64 {
    GCD_CALLS.load(Ordering::Relaxed)
}

fn reset_gcd_calls() {
    GCD_CALLS.store(0, Ordering::Relaxed);
}

// --------------------------------------------------------- exact rational type

/// An exact fraction: two integers and an invariant that they share no factor.
///
/// This is what `num_rational::Ratio<i128>` is, and what Python's
/// `fractions.Fraction` is. Keeping it in the file means the `gcd` calls are
/// visible and countable rather than hidden inside a dependency.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Ratio {
    num: i128,
    den: i128,
}

impl Ratio {
    /// Reducing here is what keeps the invariant — and what costs a `gcd`.
    fn new(num: i128, den: i128) -> Ratio {
        let g = gcd(num, den);
        if g == 0 {
            return Ratio { num: 0, den: 1 };
        }
        Ratio { num: num / g, den: den / g }
    }

    fn add(self, other: Ratio) -> Ratio {
        Ratio::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }

    fn mul_int(self, k: i128) -> Ratio {
        Ratio::new(self.num * k, self.den)
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

// ------------------------------------------------------------ the three counts

/// Count with `f64`. One multiply-add per ballot, no allocation, no `gcd`.
fn count_f64(scores: &[i128; 6]) -> f64 {
    let mut total = 0.0;
    for (i, &score) in scores.iter().enumerate() {
        let weight = C as f64 / (C + SPENT_ON_WREN[i]) as f64;
        total += score as f64 * weight;
    }
    total
}

/// Count with exact rationals. Correct, and `gcd` runs on every operation.
fn count_exact(scores: &[i128; 6]) -> Ratio {
    let mut total = Ratio { num: 0, den: 1 };
    for (i, &score) in scores.iter().enumerate() {
        let weight = Ratio::new(C, C + SPENT_ON_WREN[i]);
        total = total.add(weight.mul_int(score));
    }
    total
}

/// The least common multiple of every denominator this election can produce.
///
/// `S` runs from 0 to `SEATS * C`, so the denominator `C + S` runs over a known,
/// finite set — which is the whole precondition for the trick below. Computed
/// once, before any ballot is read.
fn fixed_denominator() -> i128 {
    let mut l: i128 = 1;
    for den in C..=(C + SEATS * C) {
        l = l / gcd(l, den) * den;
    }
    l
}

/// Count with scaled integers: every total is carried as a multiple of `1/l`.
///
/// `l / (C + S)` is an exact integer because `l` is a multiple of every
/// denominator in play — that is what makes this equal to the rational count
/// rather than an approximation of it. The `assert!` states the precondition in
/// code, because a `%` that stops being zero would otherwise silently truncate.
fn count_scaled(scores: &[i128; 6], l: i128) -> i128 {
    let mut total: i128 = 0;
    for (i, &score) in scores.iter().enumerate() {
        let den = C + SPENT_ON_WREN[i];
        assert!(l % den == 0, "denominator {den} does not divide the scale");
        total += score * C * (l / den);
    }
    total
}

// ------------------------------------------------------------------------ main

fn main() {
    println!("Seat 2 of a 5-seat Reweighted Range Voting count, 0-5 ballots.");
    println!("Seat 1 went to Wren, so each ballot now weighs 5/(5+its score for Wren).\n");

    println!("  ballot   gave Wren   weight   Alma   Bruno");
    for i in 0..6 {
        println!(
            "     {}          {}       5/{:<2}      {}      {}",
            i + 1,
            SPENT_ON_WREN[i],
            C + SPENT_ON_WREN[i],
            ALMA[i],
            BRUNO[i],
        );
    }

    // ---- 1. f64
    let (fa, fb) = (count_f64(&ALMA), count_f64(&BRUNO));
    println!("\n1. f64");
    println!("     Alma  {fa:.17}");
    println!("     Bruno {fb:.17}");
    println!("     difference {:e}", fb - fa);
    println!(
        "     -> declares {} the winner of seat 2",
        if fa > fb { "Alma" } else { "Bruno" }
    );

    // ---- 2. exact rationals
    reset_gcd_calls();
    let (ra, rb) = (count_exact(&ALMA), count_exact(&BRUNO));
    let exact_gcds = gcd_calls();
    println!("\n2. exact rationals");
    println!("     Alma  {ra}");
    println!("     Bruno {rb}");
    println!(
        "     -> {}",
        if ra == rb { "TIE — the tiebreak rung has to run" } else { "a winner" }
    );
    println!(
        "     gcd calls: {exact_gcds}  — 3 per ballot-and-candidate, to build the",
    );
    println!("                    weight, scale it by the score, and add it in");

    // ---- 3. scaled integers
    reset_gcd_calls();
    let l = fixed_denominator();
    let setup_gcds = gcd_calls();
    reset_gcd_calls();
    let (sa, sb) = (count_scaled(&ALMA, l), count_scaled(&BRUNO, l));
    let count_gcds = gcd_calls();
    println!("\n3. scaled integers, everything over l = {l}");
    println!("     Alma  {sa}");
    println!("     Bruno {sb}");
    println!(
        "     -> {}",
        if sa == sb { "TIE — same answer as the rationals" } else { "a winner" }
    );
    println!("     gcd calls while counting: {count_gcds}   (setup, once, before any ballot: {setup_gcds})");
    println!("     same value? {}", Ratio::new(sa, l) == ra);

    // ---- the headroom that decides i64 vs i128
    println!("\nWhat the scale costs in headroom. The worst a single ballot can");
    println!("contribute is 5 x 5 x (l/5), so the ballot capacity is that into the type:");
    println!(
        "\n  {:>5}   {:>8}   {:>22}   {:>15}   {:>8}",
        "seats", "denoms", "scale l", "max ballots i64", "in i128"
    );
    for seats in 1..=8i128 {
        let mut l: i128 = 1;
        for den in C..=(C + seats * C) {
            l = l / gcd(l, den) * den;
        }
        let per_ballot = C * C * (l / C);
        println!(
            "  {:>5}   {:>8}   {:>22}   {:>15}   {:>8.1e}",
            seats,
            format!("{}..{}", C, C + seats * C),
            l,
            i64::MAX as i128 / per_ballot,
            (i128::MAX / per_ballot) as f64,
        );
    }

    // ---- the overflow, made explicit
    let l5 = fixed_denominator();
    let per_ballot = (C * C * (l5 / C)) as i64;
    let ballots: i64 = 1_000_000;
    println!("\nOne million ballots at 5 seats, worst case, in i64:");
    match per_ballot.checked_mul(ballots) {
        Some(v) => println!("     checked_mul  -> Some({v})"),
        None => println!("     checked_mul  -> None          (the honest answer)"),
    }
    println!(
        "     wrapping_mul -> {}   (what a release build does if you just write `*`)",
        per_ballot.wrapping_mul(ballots)
    );
    println!(
        "     the same product in i128 -> {}",
        (per_ballot as i128) * (ballots as i128)
    );
}
