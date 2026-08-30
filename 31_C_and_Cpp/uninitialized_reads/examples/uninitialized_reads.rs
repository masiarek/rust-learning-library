// Declaring without a value is fine. The rule is about the *read*: every path
// reaching it must have assigned first, and the compiler checks all of them.

fn total_of(ballots: &[i32]) -> i32 {
    let total;                                  // no value yet

    if ballots.is_empty() {
        total = 0;                              // path 1 assigns
    } else {
        total = ballots.iter().sum();           // path 2 assigns
    }

    total                                       // so this read is allowed
}

fn main() {
    println!("[5, 0, 4] -> {}", total_of(&[5, 0, 4]));   // 9
    println!("[]        -> {}", total_of(&[]));          // 0

    // Delete either branch and the read is E0381 — a build error, not a
    // stale number left on the stack.
}
