//! Kata: a signature you cannot change, and the two you would have written.
//!
//!   rustc --edition 2024 slice_of_slices_kata.rs -o /tmp/sosk && /tmp/sosk

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

/// The signature you were handed. Pretend it lives in a crate you do not own.
fn column_sums(grid: &[&[i32]]) -> Vec<i32> {
    let width = grid.iter().map(|r| r.len()).max().unwrap_or(0);
    (0..width)
        .map(|c| grid.iter().filter_map(|r| r.get(c)).sum())
        .collect()
}

/// Task 3: the signature that would have needed no conversion from anybody.
fn column_sums_asref<R: AsRef<[i32]>>(grid: &[R]) -> Vec<i32> {
    let width = grid.iter().map(|r| r.as_ref().len()).max().unwrap_or(0);
    (0..width)
        .map(|c| grid.iter().filter_map(|r| r.as_ref().get(c)).sum())
        .collect()
}

fn main() {
    let owned: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![10, 20], vec![100]];
    let fixed: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    // ---- Task 1: call it with a Vec<Vec<i32>> ------------------------------
    let a0 = ALLOCS.load(Relaxed);
    let as_slices: Vec<&[i32]> = owned.iter().map(|v| v.as_slice()).collect();
    let a1 = ALLOCS.load(Relaxed);
    let sums = column_sums(&as_slices);
    let a2 = ALLOCS.load(Relaxed);

    println!("1. Vec<Vec<i32>> through a &[&[i32]] signature");
    println!("   column_sums = {sums:?}");
    println!("   the conversion cost {} allocation(s); the call itself {}",
             a1 - a0, a2 - a1);
    println!("   (the call allocates too: column_sums builds its own Vec)");

    // ---- Task 2: the same for an array of arrays ---------------------------
    // `[i32; 3]` has as_slice() as well, so the line is the same shape.
    let fixed_slices: Vec<&[i32]> = fixed.iter().map(|r| r.as_slice()).collect();
    println!("\n2. [[i32; 3]; 2] through the same signature");
    println!("   column_sums = {:?}", column_sums(&fixed_slices));
    println!("   `fixed.iter()` yields &[i32; 3], and as_slice() forgets the 3.");

    // ---- Task 3: the signature that needs neither line ---------------------
    let b0 = ALLOCS.load(Relaxed);
    let from_owned = column_sums_asref(&owned);
    let from_fixed = column_sums_asref(&fixed);
    let b1 = ALLOCS.load(Relaxed);

    println!("\n3. The signature that takes both, unconverted");
    println!("   from Vec<Vec<i32>> : {from_owned:?}");
    println!("   from [[i32; 3]; 2] : {from_fixed:?}");
    println!("   allocations for both calls: {} — two result Vecs, no row arrays",
             b1 - b0);

    // ---- And the one-caller answer -----------------------------------------
    println!("\n4. If only the Vec<Vec<i32>> ever called it: &[Vec<i32>]");
    println!("   `&owned` coerces to that directly — no generics, no conversion.");
}
