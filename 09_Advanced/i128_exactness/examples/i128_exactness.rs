//! What `i128` is exact about — and the three places it stops.
//!
//! `i128` is often reached for as "the exact one", usually right after a float
//! has lost a tie. It does deliver something real, but it is a *wider integer*,
//! not an exact-arithmetic library, and the difference shows up in three places
//! this program walks through:
//!
//!   1. It makes `+`, `-` and `*` exact to the last digit — up to a fixed
//!      ceiling you are responsible for proving you stay under.
//!   2. It does nothing whatever for `/`. Integer division truncates in `i128`
//!      exactly as it does in `i64`; the wider type buys range, not closure.
//!   3. It is not a `fractions.Fraction`. Build a rational on top of it and the
//!      `gcd` you pay on every operation turns out to be what buys the range —
//!      and even so, the range runs out at a term you can name.
//!
//! Deliberately no timings: a duration is not something an answer key can hold.
//! The measured costs live on the page.

/// Euclid, on the widest integer the language has.
fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

/// How far `1 + 1/2 + 1/3 + …` gets before the pair no longer fits in `i128`.
///
/// Three reduction strategies, one accumulator each. Every step is `checked_*`,
/// so "it stopped here" is the program reporting a fact rather than wrapping
/// past it and carrying on with a negative denominator.
#[derive(Clone, Copy, PartialEq)]
enum Reduce {
    /// Never call `gcd`. Add the fractions the way the schoolbook does.
    Never,
    /// Reduce the result after each addition — the obvious `Ratio` type.
    AfterTheAdd,
    /// Reduce the cross terms *before* multiplying, so the intermediates stay
    /// as small as the answer. What `num_rational` does.
    BeforeTheMultiply,
}

/// Returns `(last term that fit, numerator, denominator)`.
fn harmonic(strategy: Reduce) -> (i128, i128, i128) {
    let (mut num, mut den) = (0i128, 1i128);
    let mut n = 1i128;
    loop {
        // Add 1/n to num/den, refusing to wrap.
        let next = match strategy {
            Reduce::BeforeTheMultiply => {
                // lcm(den, n) = den / gcd * n, which is the smallest common
                // denominator rather than merely *a* common one.
                let g = gcd(den, n);
                match (den / g).checked_mul(n) {
                    None => None,
                    Some(lcm) => match (num.checked_mul(lcm / den))
                        .and_then(|scaled| scaled.checked_add(lcm / n))
                    {
                        None => None,
                        Some(nn) => {
                            let g2 = gcd(nn, lcm);
                            Some((nn / g2, lcm / g2))
                        }
                    },
                }
            }
            _ => {
                // num/den + 1/n = (num*n + den) / (den*n)
                match (num.checked_mul(n), den.checked_mul(n)) {
                    (Some(a), Some(dd)) => match a.checked_add(den) {
                        Some(nn) if strategy == Reduce::AfterTheAdd => {
                            let g = gcd(nn, dd);
                            Some((nn / g, dd / g))
                        }
                        Some(nn) => Some((nn, dd)),
                        None => None,
                    },
                    _ => None,
                }
            }
        };

        match next {
            Some((nn, dd)) => {
                num = nn;
                den = dd;
                n += 1;
            }
            // n did not fit; the last term that did was n-1.
            None => return (n - 1, num, den),
        }
    }
}

fn main() {
    // ---------------------------------------------------------------- 1
    println!("1. The type, exactly");
    println!(
        "     i64    {:>2} bytes, align {:>2}   max {}",
        size_of::<i64>(),
        align_of::<i64>(),
        i64::MAX
    );
    println!(
        "     i128   {:>2} bytes, align {:>2}   max {}",
        size_of::<i128>(),
        align_of::<i128>(),
        i128::MAX
    );
    println!(
        "     that is {} digits against {} — {} decimal digits more room.",
        i128::MAX.to_string().len(),
        i64::MAX.to_string().len(),
        i128::MAX.to_string().len() - i64::MAX.to_string().len()
    );
    println!("     It is a primitive: no crate, no heap, no allocation. Just twice");
    println!("     the register and twice the cache line of an i64.");

    // ---------------------------------------------------------------- 2
    println!("\n2. Exact under + - *, right up to the ceiling");
    // The widest thing an i64 can hold, squared. This is the case i128 exists
    // for: two values that each fit in 64 bits, and a product that does not.
    let a: i128 = i64::MAX as i128;
    let exact = a.checked_mul(a).expect("i64::MAX squared fits in i128");
    println!("     i64::MAX x i64::MAX");
    println!("       i128   {exact}");
    let approx = (i64::MAX as f64) * (i64::MAX as f64);
    println!("       f64    {approx:.0}");
    println!(
        "       error  {}  <- larger than i64::MAX itself ({})",
        (exact - approx as i128).abs(),
        i64::MAX
    );
    println!("     Every digit of the i128 answer is right. The f64 keeps about 17");
    println!("     significant digits and fills the rest with whatever the exponent");
    println!("     implies, which is how a tied count stops being tied.");
    println!("\n     The ceiling is real, though, and it is the price of the exactness:");
    match i128::MAX.checked_mul(2) {
        Some(v) => println!("       i128::MAX x 2 = {v}"),
        None => println!("       i128::MAX x 2 = None — there is no wider primitive to escape to"),
    }

    // ---------------------------------------------------------------- 3
    println!("\n3. And exact about nothing at all under /");
    let (ballots, seats): (i128, i128) = (100, 3);
    println!("     100 ballots, 3 candidates, integer division:");
    let each = ballots / seats;
    println!(
        "       each gets {each}, and {each} × {seats} = {} — {} ballot unaccounted for",
        each * seats,
        ballots - each * seats
    );
    println!("     The same statement in i64 loses the same ballot. Widening the type");
    println!("     does not close the integers under division; nothing can. That is");
    println!("     what a rational is for — and what it charges for.");

    // ---------------------------------------------------------------- 4
    println!("\n4. A rational built on i128: how far 1 + 1/2 + 1/3 + ... gets");
    let mut rows = Vec::new();
    for (label, strategy) in [
        ("never call gcd", Reduce::Never),
        ("gcd after each add", Reduce::AfterTheAdd),
        ("gcd before each multiply", Reduce::BeforeTheMultiply),
    ] {
        let (last, num, den) = harmonic(strategy);
        println!("     {label:<26} reaches 1/{last}");
        rows.push((label, last, num, den));
    }
    let (_, best_n, best_num, best_den) = rows[2];
    println!("\n     The furthest an i128 pair gets is the sum of the first {best_n} terms:");
    println!("       {best_num}");
    println!("       {best_den}");
    println!(
        "     Numerator is {} digits, denominator {} — one more term and neither fits.",
        best_num.to_string().len(),
        best_den.to_string().len()
    );

    // ---------------------------------------------------------------- 5
    println!("\n5. So what did the gcd buy?");
    println!(
        "     no reduction        1/{:<4}  ← the denominators just multiply out",
        rows[0].1
    );
    println!(
        "     reduce the result   1/{:<4}  ← +{} terms, and what overflows is now",
        rows[1].1,
        rows[1].1 - rows[0].1
    );
    println!("                                   the INTERMEDIATE, not the answer");
    println!(
        "     reduce first        1/{:<4}  ← +{} more; now the answer itself is",
        rows[2].1,
        rows[2].1 - rows[1].1
    );
    println!("                                   the thing that no longer fits");
    println!("\n     The gcd is not overhead sitting on top of the arithmetic. It IS");
    println!("     the range. Skip it and a 128-bit rational gets no further than 1/33.");
    println!("\n     Python's Fraction has no such term. It reduces with the same gcd");
    println!("     and then simply grows — so it never reports a limit, and never");
    println!("     needs to. That is the whole difference, and it is not free either:");
    println!("     the bill moves from a ceiling you must prove onto a running cost");
    println!("     that rises with the size of the number you are holding.");
}
