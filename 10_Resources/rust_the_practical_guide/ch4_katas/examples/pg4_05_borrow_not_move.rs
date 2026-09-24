//! Exercise 5 of chapter 4: `vec_ptr` is declared `&Vec<i32>`, and
//! `vec_ptr = vec_1` hands it the vector itself, E0308. Borrow with `&`. The
//! book keeps a `mut` on `vec_1` that nothing writes to, and never reads
//! `vec_ptr`, so its solution compiles with four warnings; reading the
//! pointer settles them.
//!
//!   rustc --edition 2024 pg4_05_borrow_not_move.rs -o /tmp/pg405 && /tmp/pg405

fn main() {
    let vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let mut vec_ptr: &Vec<i32> = &vec_1;
    println!("vec ptr is pointing to vec_1: {vec_ptr:?}"); // vec ptr is pointing to vec_1: [1, 2, 3]
    vec_ptr = &vec_2;
    println!("vec ptr is updated and now pointing to vec_2: {vec_ptr:?}"); // vec ptr is updated and now pointing to vec_2: [4, 5, 6]
}
