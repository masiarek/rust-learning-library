// A reference borrows; it never owns. The compiler will not let one outlive
// the value it points at, so a freed block has no live pointer into it.

fn main() {
    let mut scores = vec![5, 4, 3];

    let first = scores[0];                      // a copy, not a pointer in
    scores.clear();                             // the buffer is freed here
    println!("copied out, then freed -> {first}");        // 5

    let names = vec!["Ada".to_string()];
    let borrowed = &names[0];
    println!("borrowed while alive   -> {borrowed}");     // Ada

    drop(names);                                // allowed: the borrow ended above
    println!("dropped once the borrow was over");

    // Swap those last two lines and it is E0505 — "cannot move out of `names`
    // because it is borrowed".
}
