//! Kata solution: the rounding Rust does not ship.
//!
//! Schoolbook rounding sends a .5 UP — toward +infinity — so 2.5 is 3 and
//! -2.5 is -2. std has ties-away-from-zero (`round`) and ties-to-even
//! (`round_ties_even`) and nothing for this. The one-liner everyone writes
//! first, `(x + 0.5).floor()`, is wrong on two kinds of input; the kata is
//! to find them and then write the version that is not.
//!
//!   rustc --edition 2024 rounding_a_float_kata.rs -o /tmp/rafk && /tmp/rafk

/// The one-liner. It adds to `x` before it looks, and the addition rounds.
fn half_up_naive(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// Let std find the nearest whole number; decide only the ties yourself.
/// `fract()` is exact — it is `x - x.trunc()`, and subtracting two floats
/// within a factor of two of each other loses nothing — so `== 0.5` is a real
/// test here, not a float comparison to be nervous about: `x` either IS
/// `n + 1/2` or it is not.
fn half_up(x: f64) -> f64 {
    if x.fract().abs() == 0.5 { x.floor() + 1.0 } else { x.round() }
}

fn main() {
    println!("1. ON THE TIES, BOTH VERSIONS AGREE");
    println!("   {:>6} {:>6} {:>8} {:>6} {:>10}", "x", "naive", "by hand", "round", "ties_even");
    for x in [0.5_f64, 1.5, 2.5, -0.5, -1.5, -2.5] {
        println!(
            "   {x:>6} {:>6} {:>8} {:>6} {:>10}",
            half_up_naive(x),
            half_up(x),
            x.round(),
            x.round_ties_even()
        );
    }
    println!("   -2.5 -> -2 is the schoolbook answer. round says -3; ties_even says -2, by a different rule.");

    println!("\n2. THE FIRST INPUT THAT BREAKS THE ONE-LINER");
    let edge = 0.49999999999999994_f64; // the largest f64 below one half
    println!("   x          = {edge:?}   (the largest f64 below one half)");
    println!("   x + 0.5    = {:?}   the exact sum has no f64, and the nearest one is 1.0", edge + 0.5);
    println!("   naive(x)   = {}   wrong: x is below the half and must round down", half_up_naive(edge));
    println!("   by hand(x) = {}   fract is {:?}, not 0.5, so std's round decides", half_up(edge), edge.fract());

    println!("\n3. THE SECOND: A NUMBER THAT WAS ALREADY WHOLE");
    let whole = 4503599627370497.0_f64; // 2^52 + 1: from here up, the spacing between floats is 1
    println!("   x          = {whole}   (2^52 + 1; floats this big are 1 apart)");
    println!("   x + 0.5    = {}   the .5 cannot exist, and the tie went to even — UP", whole + 0.5);
    println!("   naive(x)   = {}   a whole number moved by one", half_up_naive(whole));
    println!("   by hand(x) = {}   fract is 0, round returns x unchanged", half_up(whole));

    println!("\n4. WHY THE FIX HOLDS");
    println!("   none of floor, ceil, trunc, round or fract ever adds to x; each reads it.");
    println!("   the one-liner performs an addition first, and an addition can round.");
}
