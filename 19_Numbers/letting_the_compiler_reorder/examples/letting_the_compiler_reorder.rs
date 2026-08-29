//! Rust 1.98's algebraic float methods — and the reordering that `+` forbids.
//!
//! `a + b + c` is pinned to `(a + b) + c`. Float addition is not associative,
//! so regrouping it changes the answer, and the language will not change your
//! answer behind your back. `algebraic_add` is how you hand that choice to the
//! optimizer on purpose, one operation at a time.
//!
//!   rustc --edition 2024 letting_the_compiler_reorder.rs -o /tmp/lcr && /tmp/lcr

fn main() {
    let t = 0.1_f64;

    println!("1. THE SAME TEN NUMBERS, TWO GROUPINGS");
    // Ten tenths added left to right — which is exactly what `+` means.
    let left_to_right = t + t + t + t + t + t + t + t + t + t;
    // The same ten. Every operation is still an ordinary `+`; the parentheses moved.
    let in_pairs = ((t + t) + (t + t)) + ((t + t) + (t + t)) + (t + t);
    println!("   left to right = {left_to_right:.20}   == 1.0 ? {}", left_to_right == 1.0);
    println!("   in pairs      = {in_pairs:.20}   == 1.0 ? {}", in_pairs == 1.0);
    println!("   nothing was rounded twice and nothing is a bug: 0.1 is not 0.1,");
    println!("   so where you put the parentheses decides which errors cancel.");

    println!("\n2. WHY THE COMPILER IS NOT ALLOWED TO CHOOSE FOR YOU");
    let big = 1.0e16_f64;
    println!("   (1e16 + -1e16) + 1 = {}", (big + -big) + 1.0);
    println!("   1e16 + (-1e16 + 1) = {}", big + (-big + 1.0));
    println!("   three values, one operator, two answers — one of which lost the 1.");
    println!("   the same holds for multiplication, where the gap is a range limit:");
    let huge = 1.0e300_f64;
    println!("   (1e300 * 1e300) * 1e-300 = {:e}", (huge * huge) * 1.0e-300);
    println!("   1e300 * (1e300 * 1e-300) = {:e}", huge * (huge * 1.0e-300));
    println!("   so `+` and `*` are evaluated left-associatively, always, and an");
    println!("   optimizer that regrouped a loop for speed would be changing results.");

    println!("\n3. THE 1.98 METHODS SAY 'GO AHEAD'");
    let xs = [t; 10];
    let strict = xs.iter().copied().fold(0.0_f64, |a, b| a + b);
    let algebraic = xs.iter().copied().fold(0.0_f64, f64::algebraic_add);
    println!("   fold with `+`             = {strict:.20}");
    println!("   fold with `algebraic_add` = {algebraic:.20}");
    println!("   identical here — and that is the lesson, not a failure.");
    println!("   this program is built without -O, so nothing reordered anything.");
    println!("   the methods grant permission; the optimizer is what spends it.");

    println!("\n4. THE FIVE OF THEM");
    println!("   algebraic_add  algebraic_sub  algebraic_mul  algebraic_div  algebraic_rem");
    println!("   on f32 and f64 alike:");
    println!("     2.5_f32.algebraic_mul(4.0) = {}", 2.5_f32.algebraic_mul(4.0));
    println!("     7.0_f64.algebraic_div(2.0) = {}", 7.0_f64.algebraic_div(2.0));
    println!("     7.0_f64.algebraic_rem(2.0) = {}", 7.0_f64.algebraic_rem(2.0));
    println!("   each returns a real f32/f64, never a poison value and never UB.");
    println!("   what it does not return is a promise about WHICH valid answer.");
}
