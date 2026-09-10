// Rust's side of the C++ Safe Buffers model: the length travels with the
// pointer, so every index can be checked -- and building a slice out of a
// raw pointer and a length is a promise you write down inside `unsafe`.
use std::mem::size_of;

fn sum(xs: &[i32]) -> i32 {
    xs.iter().sum() // no index, so no bound to check
}

fn main() {
    let word = size_of::<usize>();
    println!("Box<i32>   {} word", size_of::<Box<i32>>() / word);    // 1
    println!("Box<[i32]> {} words", size_of::<Box<[i32]>>() / word); // 2: pointer + length
    println!("&[i32]     {} words", size_of::<&[i32]>() / word);     // 2

    let owned: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    println!("owned.get(3)  -> {:?}", owned.get(3)); // None

    let v = vec![1, 2, 3];
    println!("sum(&v)       -> {}", sum(&v)); // 6

    let (ptr, len) = (v.as_ptr(), v.len()); // what a C API hands you
    // SAFETY: ptr and len come from the same live Vec, untouched while `forged` exists.
    let forged: &[i32] = unsafe { std::slice::from_raw_parts(ptr, len) };
    println!("sum(forged)   -> {}", sum(forged)); // 6
}
