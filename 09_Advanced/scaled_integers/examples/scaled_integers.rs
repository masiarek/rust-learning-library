//! One round of a proportional allocation, totalled three ways.
//!
//! The arithmetic is real, not a contrivance. A fixed pool is shared out in
//! rounds, and a reviewer who has already been allocated `S` is reweighted to
//! `C / (C + S)` for the next round, where `C` is the per-round cap. So the
//! weights are fractions, and the allocation has to add them up.
//!
//! `f64` is fast and gets it wrong. Exact rationals get it right and call `gcd`
//! on every operation. Scaling by a denominator fixed before the round starts
//! gets it right in pure integer arithmetic, which is the point of the page.

use std::sync::atomic::{AtomicU64, Ordering};

/// The per-round cap. Each reviewer rates a project 0-5.
const C: i128 = 5;

/// Rounds in this allocation. Bounds `S`, and so bounds every denominator.
const ROUNDS: i128 = 5;

// -------------------------------------------------------------- the allocation

/// Round 1 funded WREN. These are the ratings the six reviewers gave *Wren*,
/// which is what sets each reviewer's weight for round 2: `5 / (5 + s)`.
const SPENT_IN_ROUND_1: [i128; 6] = [5, 4, 3, 2, 1, 0];

/// The two projects still in for round 2, as rated by those same six reviewers.
const ALPHA: [i128; 6] = [0, 0, 1, 1, 1, 5];
const BRAVO: [i128; 6] = [0, 3, 1, 1, 5, 0];

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

/// Total with `f64`. One multiply-add per reviewer, no allocation, no `gcd`.
fn total_f64(ratings: &[i128; 6]) -> f64 {
    let mut total = 0.0;
    for (i, &rating) in ratings.iter().enumerate() {
        let weight = C as f64 / (C + SPENT_IN_ROUND_1[i]) as f64;
        total += rating as f64 * weight;
    }
    total
}

/// Total with exact rationals. Correct, and `gcd` runs on every operation.
fn total_exact(ratings: &[i128; 6]) -> Ratio {
    let mut total = Ratio { num: 0, den: 1 };
    for (i, &rating) in ratings.iter().enumerate() {
        let weight = Ratio::new(C, C + SPENT_IN_ROUND_1[i]);
        total = total.add(weight.mul_int(rating));
    }
    total
}

/// The least common multiple of every denominator this allocation can produce.
///
/// `S` runs from 0 to `ROUNDS * C`, so the denominator `C + S` runs over a known,
/// finite set — which is the whole precondition for the trick below. Computed
/// once, before any rating is read.
fn fixed_denominator() -> i128 {
    let mut l: i128 = 1;
    for den in C..=(C + ROUNDS * C) {
        l = l / gcd(l, den) * den;
    }
    l
}

/// Total with scaled integers: every sum is carried as a multiple of `1/l`.
///
/// `l / (C + S)` is an exact integer because `l` is a multiple of every
/// denominator in play — that is what makes this equal to the rational count
/// rather than an approximation of it. The `assert!` states the precondition in
/// code, because a `%` that stops being zero would otherwise silently truncate.
fn total_scaled(ratings: &[i128; 6], l: i128) -> i128 {
    let mut total: i128 = 0;
    for (i, &rating) in ratings.iter().enumerate() {
        let den = C + SPENT_IN_ROUND_1[i];
        assert!(l % den == 0, "denominator {den} does not divide the scale");
        total += rating * C * (l / den);
    }
    total
}

// ------------------------------------------------------------------------ main

fn main() {
    println!("Round 2 of a 5-round proportional allocation, 0-5 ratings.");
    println!("Round 1 funded Wren, so each reviewer now weighs 5/(5+their rating for Wren).\n");

    println!("  reviewer  gave Wren   weight   Alpha   Bravo");
    for i in 0..6 {
        println!(
            "     {}          {}       5/{:<2}      {}      {}",
            i + 1,
            SPENT_IN_ROUND_1[i],
            C + SPENT_IN_ROUND_1[i],
            ALPHA[i],
            BRAVO[i],
        );
    }

    // ---- 1. f64
    let (fa, fb) = (total_f64(&ALPHA), total_f64(&BRAVO));
    println!("\n1. f64");
    println!("     Alpha  {fa:.17}");
    println!("     Bravo {fb:.17}");
    println!("     difference {:e}", fb - fa);
    println!(
        "     -> makes {} the larger share in round 2",
        if fa > fb { "Alpha" } else { "Bravo" }
    );

    // ---- 2. exact rationals
    reset_gcd_calls();
    let (ra, rb) = (total_exact(&ALPHA), total_exact(&BRAVO));
    let exact_gcds = gcd_calls();
    println!("\n2. exact rationals");
    println!("     Alpha  {ra}");
    println!("     Bravo {rb}");
    println!(
        "     -> {}",
        if ra == rb { "a TIE — exactly equal shares" } else { "a clear larger share" }
    );
    println!(
        "     gcd calls: {exact_gcds}  — 3 per reviewer-and-project, to build the",
    );
    println!("                    weight, scale it by the rating, and add it in");

    // ---- 3. scaled integers
    reset_gcd_calls();
    let l = fixed_denominator();
    let setup_gcds = gcd_calls();
    reset_gcd_calls();
    let (sa, sb) = (total_scaled(&ALPHA, l), total_scaled(&BRAVO, l));
    let count_gcds = gcd_calls();
    println!("\n3. scaled integers, everything over l = {l}");
    println!("     Alpha  {sa}");
    println!("     Bravo {sb}");
    println!(
        "     -> {}",
        if sa == sb { "TIE — same answer as the rationals" } else { "a clear larger share" }
    );
    println!("     gcd calls while totalling: {count_gcds}   (setup, once, before any rating: {setup_gcds})");
    println!("     same value? {}", Ratio::new(sa, l) == ra);

    // ---- the headroom that decides i64 vs i128
    println!("\nWhat the scale costs in headroom. The worst a single reviewer can");
    println!("contribute is 5 x 5 x (l/5), so the reviewer capacity is that into the type:");
    println!(
        "\n  {:>5}   {:>8}   {:>22}   {:>15}   {:>8}",
        "rounds", "denoms", "scale l", "max reviewers i64", "in i128"
    );
    for rounds in 1..=8i128 {
        let mut l: i128 = 1;
        for den in C..=(C + rounds * C) {
            l = l / gcd(l, den) * den;
        }
        let per_reviewer = C * C * (l / C);
        println!(
            "  {:>5}   {:>8}   {:>22}   {:>15}   {:>8.1e}",
            rounds,
            format!("{}..{}", C, C + rounds * C),
            l,
            i64::MAX as i128 / per_reviewer,
            (i128::MAX / per_reviewer) as f64,
        );
    }

    // ---- the overflow, made explicit
    let l5 = fixed_denominator();
    let per_reviewer = (C * C * (l5 / C)) as i64;
    let reviewers: i64 = 1_000_000;
    println!("\nOne million reviewers over 5 rounds, worst case, in i64:");
    match per_reviewer.checked_mul(reviewers) {
        Some(v) => println!("     checked_mul  -> Some({v})"),
        None => println!("     checked_mul  -> None          (the honest answer)"),
    }
    println!(
        "     wrapping_mul -> {}   (what a release build does if you just write `*`)",
        per_reviewer.wrapping_mul(reviewers)
    );
    println!(
        "     the same product in i128 -> {}",
        (per_reviewer as i128) * (reviewers as i128)
    );
}
