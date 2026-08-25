//! Kata solution: a favourite number that may not exist.
//!
//! Rust's standard library defines:
//!
//!     enum Option<T> {
//!         Some(T),
//!         None,
//!     }
//!
//! So `favnum` below is *either* an i32 *or* nothing, and the `match` has to say
//! what happens in both cases — that is the exercise.
//!
//!   rustc --edition 2024 some_and_none_kata.rs -o /tmp/sank && /tmp/sank

fn main() {
    // Declared here, assigned below: legal, and the compiler proves it is set
    // before the match reads it.
    let favnum: Option<i32>;

    // Swap these two lines to see the other half.
    favnum = Some(3);
    // favnum = None;

    match favnum {
        Some(n) => println!("Your favourite number is {n}, good choice"),
        None => println!("You don't have a favourite number... what?!"),
    }

    // `unwrap_or` reads it a second way: the value, or 42 if there isn't one.
    // This line compiles only because i32 is Copy — the match above did not
    // consume `favnum`. With an Option<String> it would not.
    println!("Your favourite number, or a stand-in: {}", favnum.unwrap_or(42));

    // The same program with the other assignment, so one run shows both shapes.
    println!();
    describe(None);
}

fn describe(favnum: Option<i32>) {
    match favnum {
        Some(n) => println!("Your favourite number is {n}, good choice"),
        None => println!("You don't have a favourite number... what?!"),
    }
    println!("Your favourite number, or a stand-in: {}", favnum.unwrap_or(42));
}
