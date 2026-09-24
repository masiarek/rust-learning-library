//! Exercise 2 of chapter 4: `temp = my_vec` moves the vector on the first
//! pass, so the second pass finds it gone. The book clones on every pass. A
//! borrow costs nothing and ends at its last use, before `pop` — and the
//! `temp` can go altogether.
//!
//!   rustc --edition 2024 pg4_02_borrow_in_the_loop.rs -o /tmp/pg402 && /tmp/pg402

fn with_a_borrowed_temp() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;
    while !my_vec.is_empty() {
        temp = &my_vec;
        println!("Elements in temporary vector are: {:?}", temp);
        if let Some(last_element) = my_vec.pop() {
            println!("Popped element: {}", last_element);
        }
    }
}

fn without_a_temp() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    while let Some(last_element) = my_vec.pop() {
        println!("Popped element: {last_element}, leaving {my_vec:?}");
    }
}

fn main() {
    println!("temp = &my_vec, a loan that ends before pop:");
    with_a_borrowed_temp();
    println!();
    println!("no temp at all:");
    without_a_temp();
}
