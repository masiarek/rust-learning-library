//! Chapter 4 of *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025)
//! sorts functions into three groups — taking, giving, and taking-and-returning
//! ownership — and then replaces the third with borrowing. Each numbered block
//! below runs one claim from §4.2 or §4.4 on rustc 1.98.0. The page is
//! 18_Ownership/ownership_in_functions_claims_checked/README.md.
//!
//!   rustc --edition 2024 pg_own_claims.rs -o /tmp/pgoc && /tmp/pgoc

use std::alloc::{GlobalAlloc, Layout, System};
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

/// Counts every heap allocation, so "a new heap allocation" can be a number
/// rather than an adjective. `System` still does the allocating.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// (allocations so far, bytes so far) — read before and after a step.
fn tally() -> (usize, usize) {
    (ALLOCS.load(Relaxed), BYTES.load(Relaxed))
}

fn delta(before: (usize, usize)) -> (usize, usize) {
    let now = tally();
    (now.0 - before.0, now.1 - before.1)
}

// The chapter's functions, as it writes them.

fn takes_ownership(vec: Vec<i32>) {
    println!("   takes_ownership: vec is {vec:?}");
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("   borrows_vec: vec is {vec:?}");
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10);
}

fn stack_function(mut var: i32) {
    println!("   In func, var arrived as {var}");
    var = 56;
    println!("   In func, var is: {var}");
}

/// Listing 4.6's function, reporting the heap pointer it saw on the way through.
fn takes_and_gives_ownership_watched(mut vec: Vec<i32>) -> (Vec<i32>, *const i32) {
    let seen = vec.as_ptr();
    vec.push(10);
    (vec, seen)
}

/// Listing 4.5's function, reporting the heap pointer of the vector it built.
fn gives_ownership_watched() -> (Vec<i32>, *const i32) {
    let vec = vec![4, 5, 6];
    let built_at = vec.as_ptr();
    (vec, built_at)
}

/// A value that says when it is dropped, to date "the variable will be dropped".
struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("   drop({})", self.0);
    }
}

fn takes_loud(l: Loud) {
    println!("   takes_loud holds {}", l.0);
}

/// Stack-only, and Copy by derive.
#[derive(Clone, Copy)]
struct Flat {
    x: i32,
}

/// The same four bytes without the derive: `struct Plain { x: i32 }` moves.
struct Plain {
    x: i32,
}

/// Three elements, one spare slot, so a `push(10)` does not reallocate.
fn one_two_three() -> Vec<i32> {
    let mut vec = Vec::with_capacity(4);
    vec.extend([1, 2, 3]);
    vec
}

fn main() {
    println!("1. The three groups, run (Listings 4.5, 4.6 and 4.17)");
    let vec_2 = gives_ownership();
    println!("   gives_ownership()                    -> {vec_2:?}");
    let vec_1 = vec![1, 2, 3];
    let vec_1 = takes_and_gives_ownership(vec_1);
    println!("   takes_and_gives_ownership(vec_1)     -> {vec_1:?}");
    borrows_vec(&vec_1);
    println!("   after borrows_vec(&vec_1), vec_1 is still {vec_1:?}");
    takes_ownership(vec_1);
    println!("   after takes_ownership(vec_1), any use of vec_1 is E0382 (Listing 4.3)");

    println!();
    println!("2. \"At the end of the function, the variable will be dropped\" — when, exactly");
    takes_loud(Loud("A"));
    println!("   back in main");

    println!();
    println!("3. Listing 4.22: \"moving the entire vector's data\" — measured");
    let words = size_of::<Vec<i32>>() / size_of::<usize>();
    println!("   a Vec<i32> is {words} words on the stack: pointer, length, capacity");
    let vec_1 = one_two_three();
    let heap_before = vec_1.as_ptr();
    let before = tally();
    let (vec_1, heap_inside) = takes_and_gives_ownership_watched(vec_1);
    let moved = delta(before);
    println!(
        "   heap pointer before == inside == after the move: {} {}",
        heap_before == heap_inside,
        heap_inside == vec_1.as_ptr()
    );
    println!("   allocations during the move there and back: {} ({} bytes)", moved.0, moved.1);
    let before = tally();
    let copy = vec_1.clone();
    let cloned = delta(before);
    println!(
        "   allocations during vec_1.clone() (Listing 4.4): {} ({} bytes); clone shares the heap: {}",
        cloned.0,
        cloned.1,
        copy.as_ptr() == vec_1.as_ptr()
    );
    let before = tally();
    let lent = &vec_1;
    let length = lent.len();
    let borrowed = delta(before);
    println!("   allocations during a borrow, &vec_1 (Listing 4.17): {} (len read: {length})", borrowed.0);

    println!();
    println!("4. \"Stack-only data types are copied\": the criterion is Copy, not the stack");
    let a = Flat { x: 1 };
    let b = a;
    println!("   Flat {{ x: 1 }}, {} bytes, derive(Clone, Copy): a used after let b = a: a.x = {}, b.x = {}", size_of::<Flat>(), a.x, b.x);
    let p = Plain { x: 1 };
    println!("   Plain {{ x: 1 }}, {} bytes, no derive: p.x = {} — let q = p; then p.x is E0382 (transcript on the page)", size_of::<Plain>(), p.x);
    let arr = [1u8, 2, 3];
    let arr2 = arr;
    println!("   [u8; 3] is Copy: arr {arr:?} still usable beside arr2 {arr2:?}");
    let text = String::from("hi");
    let r = &text;
    let r2 = r;
    println!("   &String is Copy: r {r:?} and r2 {r2:?} both read");
    println!("   Vec<i32> is {words} words on the stack and moves: the type, not the region, decides");

    println!();
    println!("5. Listing 4.7: the sidebar's stack_function");
    let x = 10;
    stack_function(x);
    println!("   In main, x is: {x}");

    println!();
    println!("6. Listing 4.20: ref1 and ref2 in the same block; a borrow ends at its last use");
    let mut vec_1 = vec![1, 2, 3];
    let ref1 = &vec_1;
    borrows_vec(ref1);
    let ref2 = &mut vec_1;
    ref2.push(4);
    println!("   let ref1 = &vec_1; borrows_vec(ref1); let ref2 = &mut vec_1; ref2.push(4); -> {vec_1:?}");

    println!();
    println!("7. Listings 4.22 and 4.23: shadowing the returned vector, or a &mut — both move nothing on the heap");
    let vec_1 = one_two_three();
    let heap = vec_1.as_ptr();
    let before = tally();
    let vec_1 = takes_and_gives_ownership(vec_1);
    let shadowed = delta(before);
    println!("   let vec_1 = takes_and_gives_ownership(vec_1): {vec_1:?}, allocations {}, heap pointer kept: {}", shadowed.0, heap == vec_1.as_ptr());
    let mut vec_1 = one_two_three();
    let heap = vec_1.as_ptr();
    let before = tally();
    mutably_borrows_vec(&mut vec_1);
    let lent = delta(before);
    println!("   mutably_borrows_vec(&mut vec_1):              {vec_1:?}, allocations {}, heap pointer kept: {}", lent.0, heap == vec_1.as_ptr());

    println!();
    println!("8. Listing 4.25: return the Vec, not a &Vec — the heap bytes built inside come back untouched");
    let (vec_2, built_at) = gives_ownership_watched();
    println!("   gives_ownership() -> {vec_2:?}; heap pointer inside == outside: {}", built_at == vec_2.as_ptr());
}
