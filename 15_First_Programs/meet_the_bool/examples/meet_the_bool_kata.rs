//! Kata solution: the guard that stops working when `&&` becomes `&`, and the
//! three Rust spellings of Python's `sum(flags)`.

fn main() {
    println!("=== part 1: the guard, written both ways ===");
    let scores: Vec<u8> = vec![5, 3, 0];

    // The whole job of the left half is to make the right half safe to ask.
    for i in [1usize, 7usize] {
        let ok = (i < scores.len()) && (scores[i] > 0);
        println!("  index {i}: (i < len) && (scores[i] > 0) = {ok}");
    }

    println!("\n  now with a single &, which evaluates BOTH sides whatever the left says:");
    for i in [1usize, 7usize] {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let attempt = std::panic::catch_unwind(|| (i < scores.len()) & (scores[i] > 0));
        std::panic::set_hook(hook);
        match attempt {
            Ok(v) => println!("  index {i}: (i < len) &  (scores[i] > 0) = {v}"),
            Err(_) => println!("  index {i}: (i < len) &  (scores[i] > 0) = PANIC -- index out of bounds"),
        }
    }
    println!("  both forms compile, both are `bool -> bool -> bool`, and one of them");
    println!("  reads the element it was supposed to be protecting you from.");

    println!("\n=== part 2: counting the trues -- Python's sum(flags), three ways ===");
    let approvals = [true, false, true, true, false];
    println!("  approvals = {:?}", approvals);

    let a = approvals.iter().filter(|&&b| b).count();
    let b: u32 = approvals.iter().map(|&f| f as u32).sum();
    let c: u32 = approvals.iter().copied().map(u32::from).sum();
    println!("  filter(|&&b| b).count()        = {a}   <- says 'how many are true'");
    println!("  map(|f| f as u32).sum()        = {b}   <- says 'add them up', via a cast");
    println!("  map(u32::from).sum()           = {c}   <- the same cast as a named conversion");
    println!("  the first one is the one to write: the other two make you re-read a cast");
    println!("  to find out that counting was the point.");

    println!("\n=== part 3: what a bool cannot do, and the escape hatch ===");
    println!("  `true + 1`  -> error[E0369]: cannot add `{{integer}}` to `bool`");
    println!("  `if 1 {{ }}`  -> error[E0308]: mismatched types, expected `bool`, found integer");
    println!("  so the cast is never implicit and never accidental:");
    println!("    true as u8  = {}      false as u8 = {}", true as u8, false as u8);
    println!("    u8::from(true) = {}   u8::from(false) = {}", u8::from(true), u8::from(false));
    println!("  and going back the other way needs a comparison, not a cast:");
    println!("    1u8 as bool -> error[E0054]: cannot cast `u8` as `bool`");
    println!("                   help: compare with zero instead");
    println!("    1u8 != 0    = {}   <- which is the question you actually meant", 1u8 != 0);
}
