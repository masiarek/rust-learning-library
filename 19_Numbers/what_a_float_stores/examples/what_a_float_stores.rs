//! What a float actually stores — and the two traits Rust refuses to give it.
//!
//! Every surprising thing a float does follows from one fact: the number you
//! wrote is not the number that got stored. Rust is unusual in making that
//! visible in the type system rather than leaving it to your discipline.
//!
//!   rustc --edition 2024 what_a_float_stores.rs -o /tmp/wafs && /tmp/wafs

use std::collections::HashMap;

/// The three comparisons that explain the missing traits.
///
/// Writing them is a mistake in real code, and rustc says so: the
/// `invalid_nan_comparisons` lint fires on each one and suggests `.is_nan()`.
/// Here the mistake IS the lesson, so the lint is allowed on this function
/// alone rather than muffled for the whole file.
#[allow(invalid_nan_comparisons)]
fn nan_comparisons() -> (bool, bool, bool) {
    (f64::NAN == f64::NAN, f64::NAN < 1.0, f64::NAN > 1.0)
}

fn main() {
    println!("1. WHAT IS ACTUALLY STORED");
    println!("   you wrote 0.1, and the machine kept:");
    println!("     0.1_f32 = {:.30}", 0.1_f32);
    println!("     0.1_f64 = {:.30}", 0.1_f64);
    println!("   neither is 0.1. In binary, 1/10 repeats forever:");
    println!("     1/10 = 0.0001100110011001100110011...  (0011 forever)");
    println!("   f32 keeps 24 significant bits, f64 keeps 53. The rest is cut.");

    println!("\n2. IT ROUNDS — IT DOES NOT TRUNCATE");
    println!("   the discarded tail votes on the last kept bit first,");
    println!("   so the error goes in BOTH directions:");
    println!("     0.3_f32 = {:.30}   UP", 0.3_f32);
    println!("     0.3_f64 = {:.30}   DOWN", 0.3_f64);
    println!("     0.7_f32 = {:.30}   DOWN", 0.7_f32);
    println!("   same literal, two widths, opposite directions.");

    println!("\n3. THE CUT COMPOUNDS");
    // Ten ballots, every one of them scores this candidate 1 out of 5.
    // The average is 1.0. Two honest ways to compute it, one answer each.
    let ballots: [u8; 10] = [1; 10];

    let mut running = 0.0_f64; // a streaming average: add score/n as you go
    for &s in &ballots {
        running += f64::from(s) / ballots.len() as f64;
    }
    let total: u32 = ballots.iter().map(|&s| u32::from(s)).sum(); // exact: integers
    let once = f64::from(total) / ballots.len() as f64;

    println!("   ten ballots, every one scoring the candidate 1 of 5");
    println!("     sum as integers, divide once : {once}   == 1.0 ? {}", once == 1.0);
    println!("     add 1/10 ten times           : {running}   == 1.0 ? {}", running == 1.0);
    println!("     difference                   : {:e}", once - running);
    println!("   ten roundings instead of one. Same data, same average, same");
    println!("   arithmetic — and one of them cannot be compared to 1.0.");

    println!("\n4. WHY RUST WITHHOLDS Eq AND Ord");
    println!("   because of one value that breaks both laws:");
    let (eq, lt, gt) = nan_comparisons();
    println!("     NAN == NAN                  : {eq}");
    println!("     NAN  < 1.0                  : {lt}");
    println!("     NAN  > 1.0                  : {gt}");
    println!("   not less, not greater, not equal. Ord promises a TOTAL order,");
    println!("   Eq promises reflexivity (a == a). NaN breaks both, so f64 gets");
    println!("   only PartialEq and PartialOrd — and partial_cmp says so:");
    println!("     1.0.partial_cmp(&2.0)       : {:?}", 1.0_f64.partial_cmp(&2.0));
    println!("     1.0.partial_cmp(&NAN)       : {:?}", 1.0_f64.partial_cmp(&f64::NAN));
    println!("   that None is the answer a language without it has to invent.");

    println!("\n5. THE ESCAPE HATCHES, WHEN YOU MEAN IT");
    let mut xs = vec![0.3_f64, f64::NAN, 0.1 + 0.2, 1.0];
    xs.sort_by(f64::total_cmp); // IEEE-754 totalOrder: NaN gets a defined place
    println!("   sort_by(f64::total_cmp)     : {xs:?}");
    println!("   compare within a tolerance you chose for the problem:");
    let (a, b) = (0.1_f64 + 0.2, 0.3_f64);
    println!("     a == b                    : {}", a == b);
    println!("     (a-b).abs() < 1e-9        : {}", (a - b).abs() < 1e-9);
    // Integers keep every promise a float breaks: Eq, Ord, Hash, exactness.
    let mut tally: HashMap<u32, u32> = HashMap::new(); // cents, or tenths
    for &s in &ballots {
        *tally.entry(u32::from(s)).or_insert(0) += 1;
    }
    println!("     HashMap<u32, _> key       : {:?}  (f64 is not Hash)", tally);
}
