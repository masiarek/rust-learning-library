//! Two runs of the same function: one during compilation, one after.
//!
//! `triangular` is a `const fn`, so the compiler is allowed to execute it while
//! building the program. The array length below forces it to — an array's size
//! has to be a number before the binary exists — and `black_box` hides the
//! argument in the second call so that one really does happen at run time.

const fn triangular(n: u64) -> u64 {
    let mut sum = 0;
    let mut i = 1;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}

/// Evaluated by the compiler. Nothing here survives into the binary except 55.
const SLOTS: usize = triangular(10) as usize;

fn main() {
    // An array length must be known at compile time, so this line only
    // compiles because the loop above already ran.
    let counters = [0u8; SLOTS];

    println!("compile time: SLOTS = {SLOTS}");
    println!("compile time: counters.len() = {}", counters.len());

    // The same function, called the ordinary way. `black_box` stops the
    // compiler from folding this one too.
    let n = std::hint::black_box(10u64);
    println!("run time:     triangular(n) = {}", triangular(n));

    // A `const` is a value, not a variable: it has no address of its own and
    // is substituted at each use, so this reads as the literal 55.
    println!("both agree:   {}", SLOTS as u64 == triangular(n));
}
