//! Kata: sum eight tenths so the answer equals the 0.8 you would have typed.
//!
//!   rustc --edition 2024 letting_the_compiler_reorder_kata.rs -o /tmp/lcrk && /tmp/lcrk

/// Add a slice by halving it, so no partial sum ever gets far ahead of the
/// next value it has to absorb. Order is still fixed — the grouping is just
/// a different fixed one.
fn sum_pairwise(xs: &[f64]) -> f64 {
    match xs.len() {
        0 => 0.0,
        1 => xs[0],
        n => {
            let (left, right) = xs.split_at(n / 2);
            sum_pairwise(left) + sum_pairwise(right)
        }
    }
}

fn main() {
    let scores = [0.1_f64; 8];

    let left_to_right = scores.iter().copied().fold(0.0, |a, b| a + b);
    let pairwise = sum_pairwise(&scores);
    let algebraic = scores.iter().copied().fold(0.0_f64, f64::algebraic_add);

    println!("PART 1 — two groupings, one target");
    println!("  0.8 as written    = {:.20}", 0.8_f64);
    println!("  left to right     = {left_to_right:.20}   == 0.8 ? {}", left_to_right == 0.8);
    println!("  pairwise          = {pairwise:.20}   == 0.8 ? {}", pairwise == 0.8);
    println!("  the pairwise grouping is the one the optimizer would reach for,");
    println!("  and on this input it happens to land on the literal exactly.");

    println!("\nPART 2 — does asking for it get it?");
    println!("  algebraic_add     = {algebraic:.20}   == 0.8 ? {}", algebraic == 0.8);
    println!("  no. `algebraic_add` permits the regrouping, it does not perform it,");
    println!("  and an unoptimized build has no reason to bother. If you need a");
    println!("  particular grouping, write the grouping — that is what PART 1 did.");

    println!("\nPART 3 — what you may rely on");
    let bounds = [left_to_right.min(pairwise), left_to_right.max(pairwise)];
    println!("  every legal answer sits in [{:.20}, {:.20}]", bounds[0], bounds[1]);
    println!("  width of that window = {:e}", bounds[1] - bounds[0]);
    println!("  so an assertion on the WINDOW holds under any regrouping:");
    println!("    (algebraic - 0.8).abs() < 1e-12  ->  {}", (algebraic - 0.8).abs() < 1e-12);
    println!("    algebraic == 0.8                 ->  {}   (a bet on the optimizer)", algebraic == 0.8);
}
