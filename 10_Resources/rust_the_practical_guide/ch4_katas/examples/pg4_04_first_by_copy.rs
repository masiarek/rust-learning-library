//! Exercise 4 of chapter 4: `first` borrows `some_vec` until the `println!`,
//! so the `push` between them is E0502. The book moves the push below the
//! print. Returning the number — an `i32` is Copy — ends the borrow at the
//! call, and `&[i32]` takes any slice, not only a `Vec`.
//!
//!   rustc --edition 2024 pg4_04_first_by_copy.rs -o /tmp/pg404 && /tmp/pg404

fn main() {
    let mut some_vec = vec![1, 2, 3];
    let first = get_first_element(&some_vec);
    some_vec.push(4);
    if let Some(first) = first {
        println!("The first number is: {}", first); // The first number is: 1
    }
    println!("some_vec is now {:?}", some_vec); // some_vec is now [1, 2, 3, 4]
    println!("of an empty slice: {:?}", get_first_element(&[])); // of an empty slice: None
}

fn get_first_element(num_vec: &[i32]) -> Option<i32> {
    num_vec.first().copied()
}
