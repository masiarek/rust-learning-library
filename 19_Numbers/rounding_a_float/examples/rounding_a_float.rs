//! Four ways to make a float whole — and the one thing none of them does.
//!
//! `floor`, `ceil`, `trunc` and `round` each pick a whole number from an
//! `f64`, and they pick four different ones once the value is negative or
//! sits on a .5. Every one of them hands back an `f64`, which is where the
//! traps live: `9.0` prints as `9`, `-0.4` can round to `-0`, NaN stays NaN,
//! and the integer you wanted is still one `as` away.
//!
//!   rustc --edition 2024 rounding_a_float.rs -o /tmp/raf && /tmp/raf

fn main() {
    println!("1. FOUR DIRECTIONS");
    println!("   {:>8} {:>6} {:>6} {:>6} {:>6} {:>10}", "x", "floor", "ceil", "trunc", "round", "ties_even");
    for x in [9.1_f64, 100.7, -1.1, -19.9, 2.5, -2.5, 0.5, -0.5] {
        println!(
            "   {x:>8} {:>6} {:>6} {:>6} {:>6} {:>10}",
            x.floor(),
            x.ceil(),
            x.trunc(),
            x.round(),
            x.round_ties_even()
        );
    }
    println!("   floor goes toward -inf, ceil toward +inf, trunc toward 0.");
    println!("   round goes to the nearest, and a .5 goes AWAY from 0 — so -2.5 rounds");
    println!("   to -3, which is lower. round_ties_even sends a .5 to the even neighbour.");

    println!("\n2. STILL A FLOAT");
    let nine = 9.1_f64.floor();
    println!("   9.1.floor() with {{}}    : {nine}");
    println!("   9.1.floor() with {{:?}}  : {nine:?}      Display drops the .0; Debug keeps it");
    let z = (-0.4_f64).ceil();
    println!(
        "   (-0.4).ceil()           : {z} / {z:?}   == 0.0 ? {}   is_sign_negative ? {}",
        z == 0.0,
        z.is_sign_negative()
    );
    println!("   an integer has no negative zero; this whole number does.");
    println!("   NAN.round()             : {}", f64::NAN.round());
    println!("   INFINITY.floor()        : {}", f64::INFINITY.floor());
    let digits = 1e300_f64.round().to_string().len();
    println!("   1e300.round()           : a whole number {digits} digits long, and no integer type holds it");
    println!("   4503599627370496.5      : {}   2^52 + 0.5 cannot be written down;", 4503599627370496.5_f64);
    println!("                             above 2^52 every f64 is already whole and round() changes nothing.");

    println!("\n3. THREE TIE RULES IN std");
    println!("   {:>6} {:>7} {:>10} {:>6}", "x", "round", "ties_even", "{:.0}");
    for x in [0.5_f64, 1.5, 2.5, -0.5, -1.5, -2.5] {
        println!("   {x:>6} {:>7} {:>10} {:>6}", x.round(), x.round_ties_even(), format!("{x:.0}"));
    }
    println!("   {{:.N}} sends a tie to even, like round_ties_even — judged on the digits actually stored:");
    for x in [0.25_f64, 0.35, 0.45] {
        println!("   {x} with {{:.1}} : {x:.1}    stored: {x:.20}");
    }
    println!("   0.25 is a true tie (1/4 is exact) and goes to even. 0.35 and 0.45 never were ties:");
    println!("   one is stored below its half and one above, and each rounds toward the value it is.");

    println!("\n4. TWO DECIMALS, TWO ANSWERS");
    let p = 2.675_f64;
    println!("   2.675 with {{:.2}}                : {p:.2}");
    println!("   (2.675 * 100.0).round() / 100.0 : {:?}", (p * 100.0).round() / 100.0);
    println!("   because 2.675 * 100.0 is exactly {:?} — the multiply rounded UP onto a tie —", p * 100.0);
    println!("   while 2.675 itself is stored as {p:.20}, below it.");
    println!("   {{:.2}} rounds the value you have; the scale idiom rounds a product that moved.");
    let q = 1.005_f64;
    println!(
        "   1.005: {{:.2}} = {q:.2}, scale idiom = {:?}   (1.005 * 100.0 = {:?}: this one moved DOWN)",
        (q * 100.0).round() / 100.0,
        q * 100.0
    );

    println!("\n5. fract IS THE PART THE OTHERS THREW AWAY");
    for x in [9.1_f64, -9.1, 2.5] {
        println!(
            "   {x:>5}.fract() = {:<22} trunc + fract == x ? {}",
            x.fract(),
            x.trunc() + x.fract() == x
        );
    }
    println!("   9.1 - 9 is not 0.1, because 9.1 was never 9.1 — the cut from the previous page, made visible.");

    println!("\n6. THE INTEGER YOU WANTED IS STILL ONE `as` AWAY");
    for x in [9.7_f64, -9.7] {
        println!(
            "   {x:>5}: x as i64 = {:>3}   x.trunc() as i64 = {:>3}   x.round() as i64 = {:>3}",
            x as i64,
            x.trunc() as i64,
            x.round() as i64
        );
    }
    println!(
        "   a bare `as` is trunc plus saturation: 1e300 as i64 = {}, NAN as i64 = {}",
        1e300_f64 as i64,
        f64::NAN as i64
    );
    println!("   say which rounding first, then cast: `x.round() as i64` reads as what it does.");
}
