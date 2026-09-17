//! Raw pointers: what safe code may do with one, what needs `unsafe`, and the
//! rules that still apply when the borrow checker is not looking.
//!
//!   rustc --edition 2024 raw_pointers.rs -o /tmp/rp && /tmp/rp

use std::mem::size_of;
use std::ptr::{self, NonNull};

fn main() {
    println!("1. Safe code may make, copy, cast and compare raw pointers");
    let scores = [10u32, 20, 30];
    let start: *const u32 = scores.as_ptr();
    let null: *const u32 = ptr::null();
    let end = start.wrapping_add(scores.len()); // one past the end: fine to make
    println!("   null.is_null()             {}", null.is_null());
    println!("   start.is_null()            {}", start.is_null());
    println!("   end.addr() - start.addr()  {} bytes", end.addr() - start.addr());
    println!("   start.cast::<u8>() == start as *const u8  {}", start.cast::<u8>() == start as *const u8);
    println!("   start.cast_mut() compiles too: *const -> *mut is only a relabelling");
    println!("   None of these reads memory, so none of them needs unsafe.");

    println!();
    println!("2. Reading, writing and offsetting need unsafe");
    // SAFETY: start points at scores[0]; index 2 is inside the 3-element array.
    let third = unsafe { *start.add(2) };
    println!("   unsafe {{ *start.add(2) }}        {third}");
    // SAFETY: as_ref checks for null itself; a non-null start is a valid &u32.
    println!("   unsafe {{ null.as_ref() }}        {:?}", unsafe { null.as_ref() });
    println!("   unsafe {{ start.as_ref() }}       {:?}", unsafe { start.as_ref() });
    println!("   `add` is unsafe and `wrapping_add` is not: `add` promises the result");
    println!("   stays inside the allocation, and the compiler may rely on that.");

    println!();
    println!("3. No borrow checker: two *mut to one place at once");
    let mut count = 0u32;
    let a = &raw mut count;
    let b = &raw mut count;
    // SAFETY: a and b both come straight from `count`, nothing else borrows it
    // while they are used, and each write finishes before the next begins.
    unsafe {
        *a += 1;
        *b += 1;
    }
    println!("   write through a, write through b: count = {count}");
    println!("   Two &mut u32 to `count` would be E0499. Two raw pointers compile,");
    println!("   and this use is sound. The rules have not gone away, only the checks:");
    println!("   a pointer taken from a &mut is invalid once a NEW &mut to the same");
    println!("   place is made, and writing through it after that is undefined");
    println!("   behaviour that no compiler flags. Miri does; see the page.");

    println!();
    println!("4. Back to a reference: you make the promises");
    // SAFETY: `a` points at `count`, which is alive, aligned and initialized,
    // and no other reference to `count` is used while `r` is.
    let r: &mut u32 = unsafe { &mut *a };
    *r += 40;
    println!("   unsafe {{ &mut *a }}, += 40: count = {count}");

    println!();
    println!("5. NonNull<T>: a raw pointer with one promise, never null");
    let mut n = 5u32;
    let nn = NonNull::from(&mut n);
    println!("   NonNull::new(null_mut)          {:?}", NonNull::new(ptr::null_mut::<u32>()));
    println!("   size_of Option<*mut u32>        {}", size_of::<Option<*mut u32>>());
    println!("   size_of Option<NonNull<u32>>    {}", size_of::<Option<NonNull<u32>>>());
    // SAFETY: nn came from &mut n, and n is alive and unborrowed.
    println!("   unsafe {{ *nn.as_ptr() }}          {}", unsafe { *nn.as_ptr() });
    println!("   The one promise buys the niche: Box, Rc and Vec are built on it.");
}
