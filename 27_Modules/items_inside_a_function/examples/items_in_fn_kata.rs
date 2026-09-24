//! Kata solution: from a closure to a nested fn, and a type that never
//! leaves its function.

use std::fmt;

/// `Celsius` and its `impl` live here. The value leaves through
/// `impl Display`; the name does not.
fn celsius_report(c: f64) -> impl fmt::Display {
    struct Celsius(f64);
    impl fmt::Display for Celsius {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:.1} C", self.0)
        }
    }
    Celsius(c)
}

fn main() {
    println!("=== part 1: the captured factor becomes a parameter ===");
    let factor = 3;
    let scale = |x: i32| x * factor;
    fn scale_by(x: i32, factor: i32) -> i32 {
        x * factor
    }
    let by_closure: Vec<i32> = [1, 2, 3].iter().map(|&x| scale(x)).collect();
    let by_fn: Vec<i32> = [1, 2, 3].iter().map(|&x| scale_by(x, factor)).collect();
    println!("  closure:                    {by_closure:?}");
    println!("  nested fn, factor passed in: {by_fn:?}");
    println!("  fn scale(x: i32) -> i32 {{ x * factor }} is E0434: a fn item captures nothing");

    println!("\n=== part 1b: an item is in scope for the whole block, a let from its line down ===");
    println!("  first_word(\"John Archer\") = {:?}, called above its fn", first_word("John Archer"));
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    println!("  a closure is bound by a let, so using it above that line is E0425");

    println!("\n=== part 2: the value leaves, the name does not ===");
    println!("  celsius_report(21.5) = {}", celsius_report(21.5));
    println!("  naming Celsius in main does not compile: the name is scoped to the block");

    println!("\n=== part 3: what a nested fn sees ===");
    fn bump(n: i32) -> i32 {
        (n + BASE).min(CAP)
    }
    const BASE: i32 = 10;
    static CAP: i32 = 25;
    println!("  bump(20) = {} -- reads BASE (declared below it) and CAP", bump(20));
    println!("  a let is the one thing it cannot read; a block or a closure can");
}
