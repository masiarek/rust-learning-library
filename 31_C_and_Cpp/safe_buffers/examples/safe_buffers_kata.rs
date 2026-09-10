//! Kata solution: spanify the Safe Buffers docs' own example.
//!
//!   rustc --edition 2024 safe_buffers_kata.rs -o /tmp/k && /tmp/k

/// The literal port of `int get_last_element(int *pointer, size_t size)`.
///
/// # Safety
/// `ptr` must point at `size` initialised `i32`s, and `size` must not be zero.
unsafe fn get_last_raw(ptr: *const i32, size: usize) -> i32 {
    // SAFETY: the caller promised `size` elements starting at `ptr`, size > 0.
    unsafe { *ptr.add(size - 1) }
}

/// The spanified version: the buffer arrives as one value, length included.
fn get_last(xs: &[i32]) -> Option<i32> {
    xs.last().copied()
}

fn main() {
    let v = vec![1, 2, 3];

    println!("THE LITERAL PORT");
    // SAFETY: the pointer and the length come from the same live Vec.
    let last = unsafe { get_last_raw(v.as_ptr(), v.len()) };
    println!("  get_last_raw(ptr, 3) = {last}");
    println!("  At size 0, `size - 1` is the bug. In C++ the size_t wraps to SIZE_MAX");
    println!("  and pointer[SIZE_MAX] is undefined behaviour. This port panics on the");
    println!("  subtraction in a debug build -- and under --release it wraps too, and");
    println!("  ptr.add() is undefined behaviour exactly as in C++. `unsafe` did not");
    println!("  remove the bug; it labelled the function it lives in.");
    println!();

    println!("THE SPANIFIED VERSION");
    println!("  get_last(&v)      = {:?}", get_last(&v));
    println!("  get_last(&[])     = {:?}", get_last(&[]));
    println!("  get_last(&v[..1]) = {:?}", get_last(&v[..1]));
    println!("  The length travels with the buffer, so no caller can pass a wrong one");
    println!("  and there is no subtraction to underflow: last() asks the slice.");

    assert_eq!(last, 3);
    assert_eq!(get_last(&[]), None);
}
