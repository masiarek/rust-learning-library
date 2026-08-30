//! Three ways to hold a grid, and what each one costs.
//!
//!   rustc --edition 2024 vec_of_vecs.rs -o /tmp/vov && /tmp/vov

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Wraps the default allocator and counts the calls. `System` still does the
/// work, so the only thing this changes about the program is that the heap
/// traffic becomes a number the page can print.
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

const W: usize = 4;
const H: usize = 4;

fn main() {
    // ---- 1. build each one, counting allocations, printing nothing yet -----
    // `println!` allocates too, so every snapshot is taken with no output in
    // between and the numbers are reported afterwards.
    let a0 = ALLOCS.load(Relaxed);
    let fixed: [[u8; W]; H] = [[0; W]; H];
    let a1 = ALLOCS.load(Relaxed);
    let nested: Vec<Vec<u8>> = vec![vec![0u8; W]; H];
    let a2 = ALLOCS.load(Relaxed);
    let flat: Vec<u8> = vec![0u8; W * H];
    let a3 = ALLOCS.load(Relaxed);

    println!("1. Three ways to hold a {W}x{H} grid of u8");
    println!("   [[u8; 4]; 4]          {} allocations — every byte is inline", a1 - a0);
    println!("   vec![vec![0; 4]; 4]   {} allocations — one per row, plus the outer Vec", a2 - a1);
    println!("   vec![0; 16]           {} allocation  — one block, indexed r * W + c", a3 - a2);
    println!("   The nested form is the only one that can have rows of different");
    println!("   lengths, and the only one that pays for the privilege.");

    println!();
    println!("2. Contiguous, or a pointer chase per row");
    let row_gap = fixed[1].as_ptr() as usize - fixed[0].as_ptr() as usize;
    println!("   [[u8; 4]; 4]: row 1 begins {row_gap} bytes after row 0 — one block, no gaps");
    println!("   size_of_val(&fixed) = {} bytes, all of it the data", size_of_val(&fixed));
    println!(
        "   the outer Vec is {} usizes wide — ptr, len, cap, and none of the data",
        size_of_val(&nested) / size_of::<usize>()
    );
    println!("   Reading nested[r][c] follows two pointers; flat[r * W + c] follows one.");
    println!("   flat[2 * W + 2] is the cell nested[2][2] names: {}", flat[2 * W + 2]);

    println!();
    println!("3. Printing: `{{:?}}` is one line, not a picture");
    let mut small = vec![vec![0u8; W]; H];
    small[2][2] = 5;
    println!("   println!(\"{{:?}}\", grid) gives you this, all on one line:");
    println!("   {small:?}");
    println!("   A grid on the screen needs a loop — the Debug impl never wraps:");
    for row in &small {
        let cells: Vec<String> = row.iter().map(|c| c.to_string()).collect();
        println!("     {}", cells.join(" "));
    }
    println!("   `{{:#?}}` wraps, but one element per line — sixteen lines for this grid.");

    println!();
    println!("4. Updating through iter_mut: the `*` is the whole lesson");
    let mut counted: Vec<Vec<u8>> = vec![vec![0u8; W]; H];
    for (r, row) in counted.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            *cell = (r * W + c) as u8; // `cell` is &mut u8; `cell = …` is E0308
        }
    }
    println!("   after *cell = r * W + c:");
    for row in &counted {
        let cells: Vec<String> = row.iter().map(|c| format!("{c:>2}")).collect();
        println!("     {}", cells.join(" "));
    }
    println!("   `cell` is a &mut u8. Assigning to it repoints the binding, which is");
    println!("   both a type error and, if it compiled, not what you meant.");

    println!();
    println!("5. The rows are clones, not aliases");
    let mut rows = vec![vec![0u8; W]; H];
    let shared = rows[0].as_ptr() == rows[1].as_ptr();
    rows[0][0] = 9;
    println!("   rows[0] and rows[1] share a buffer? {shared}");
    println!("   after rows[0][0] = 9 -> {rows:?}");
    println!("   vec![elem; n] CLONES elem n times. Python's [[0]*4]*4 stores the");
    println!("   same list four times, so the same assignment changes every row.");
}
