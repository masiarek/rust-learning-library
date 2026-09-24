//! Kata solution: the chapter's three helpers rewritten with references, so
//! the vector is allocated once, cloned never and moved never — and the
//! program proves it with an allocation count and a heap pointer.
//!
//!   rustc --edition 2024 pg_own_kata.rs -o /tmp/pgok && /tmp/pgok

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Gives ownership: the one allocation, with a spare slot for the push.
fn make() -> Vec<i32> {
    let mut vec = Vec::with_capacity(4);
    vec.extend([1, 2, 3]);
    vec
}

/// Reads: a slice, so a `&Vec<i32>`, an array or a slice all fit.
fn show(label: &str, vec: &[i32]) {
    println!("   {label}: {vec:?}");
}

/// Writes: a `&mut Vec<i32>`, because only a `Vec` can grow.
fn append_ten(vec: &mut Vec<i32>) {
    vec.push(10);
}

fn main() {
    println!("Zero moves, one allocation");
    let before = ALLOCS.load(Relaxed);
    let mut vec_1 = make();
    let heap = vec_1.as_ptr();
    show("made", &vec_1);
    append_ten(&mut vec_1);
    show("after append_ten", &vec_1);
    show("as a slice of the middle", &vec_1[1..3]);
    let allocations = ALLOCS.load(Relaxed) - before;
    println!("   allocations in total: {allocations}");
    println!("   heap pointer unchanged: {}", heap == vec_1.as_ptr());
    println!("   vec_1 still owned by main, never shadowed, never cloned: {vec_1:?}");
}
