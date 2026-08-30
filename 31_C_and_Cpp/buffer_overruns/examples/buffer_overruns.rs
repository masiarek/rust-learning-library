// Every index is checked. Asking for the fourth of three has two honest
// answers, and neither of them is whatever was next in memory.

use std::panic;

fn main() {
    let scores = vec![5, 4, 3];

    let total: i32 = scores.iter().sum();       // what the C loop was after
    println!("sum of the three -> {total}");    // 12

    println!("scores.get(2)    -> {:?}", scores.get(2));  // Some(3)
    println!("scores.get(3)    -> {:?}", scores.get(3));  // None

    // `scores[3]` ends the program. Caught here only so the run finishes.
    panic::set_hook(Box::new(|_| {}));
    let out_of_range = panic::catch_unwind(|| scores[3]);
    let _ = panic::take_hook();

    let why = out_of_range.unwrap_err();
    if let Some(message) = why.downcast_ref::<String>() {
        println!("scores[3]        -> {message}");
    }
}
