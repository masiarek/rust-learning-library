//! Kata solution: declare it, and let the compiler check every path.
//!
//!   rustc --edition 2024 uninitialized_reads_kata.rs -o /tmp/urk && /tmp/urk

fn classify(n: i32) -> &'static str {
    // Declared without a value -- legal, and idiomatic. The compiler checks
    // that every path reaching the read assigns exactly once first.
    let label;
    if n < 0 {
        label = "negative";
    } else if n == 0 {
        label = "zero";
    } else {
        label = "positive";
    }
    label
}

fn main() {
    println!("THE C SHAPE");
    println!("  int x;  if (cond) x = 1;  printf(\"%d\", x);");
    println!("  On the path where cond is false, x holds whatever was on the");
    println!("  stack. -Wall may warn; it also may not, once the assignment is");
    println!("  two functions away. And reading an uninitialised value is");
    println!("  UNDEFINED, not merely unpredictable -- so the optimizer may");
    println!("  assume the path never happens.");
    println!();

    println!("RUST LETS YOU DECLARE WITHOUT ASSIGNING TOO");
    for n in [-5, 0, 7] {
        println!("  classify({n:>2}) = {:?}", classify(n));
    }
    println!();
    println!("  `let label;` with no value is fine. What the compiler checks is");
    println!("  that every path to the READ assigns first -- so deleting the");
    println!("  `else` arm above is E0381, 'used binding is possibly-");
    println!("  uninitialized', naming the branch that skipped it.");
    println!();

    println!("AND THAT IS WHY THIS IS AN EXPRESSION LANGUAGE");
    let n = 7;
    let label = if n < 0 { "negative" } else if n == 0 { "zero" } else { "positive" };
    println!("  let label = if ... {{ ... }} else {{ ... }};  -> {label:?}");
    println!("  The if is an expression, so the same code needs no deferred");
    println!("  binding at all -- every branch must produce a value of the same");
    println!("  type, and a missing else is a type error rather than a missing");
    println!("  assignment. Most of the C pattern disappears rather than being");
    println!("  checked.");
    println!();

    println!("THE FLOW ANALYSIS IS NOT ONLY ABOUT INITIALISATION");
    println!("  The same pass tracks MOVES: a value moved out on one branch and");
    println!("  used after the join is 'possibly moved', reported the same way.");
    println!("  Initialisedness and ownership are one analysis, which is why");
    println!("  they produce errors that read alike.");
    println!();

    println!("THE ESCAPE HATCH, AND WHAT IT ADMITS");
    println!("  MaybeUninit<T> exists for the cases that genuinely need an");
    println!("  uninitialised buffer -- reading into it from the OS, say. Every");
    println!("  read from it is `unsafe`, and the word is the point: you are");
    println!("  asserting the initialisation the compiler could not check.");

    assert_eq!(classify(-1), "negative");
    assert_eq!(classify(0), "zero");
    assert_eq!(label, "positive");
}
