//! Exercise 6 of chapter 4: `&mut first_num` needs `first_num` declared `mut`,
//! E0596, and the same for `second_num`. With both `mut` the book's program
//! compiles. `ref2 = ref1` then reborrows `ref1` rather than moving it — the
//! assignment target's type is known — so `ref1` works again once `ref2` is
//! done with it.
//!
//!   rustc --edition 2024 pg4_06_two_muts.rs -o /tmp/pg406 && /tmp/pg406

fn main() {
    let mut first_num = 42;
    let mut second_num = 64;
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num;
    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;
    println!("Updated first number: {ref2}"); // Updated first number: 15
    println!("ref1 again, after ref2's last use: {ref1}"); // ref1 again, after ref2's last use: 15
    println!("first_num = {first_num}, second_num = {second_num}"); // first_num = 15, second_num = 10
}
