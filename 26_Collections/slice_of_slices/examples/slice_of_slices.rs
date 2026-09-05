//! Writing the parameter type for a grid: `&[Vec<T>]`, `&[&[T]]`, and `AsRef`.
//!
//!   rustc --edition 2024 slice_of_slices.rs -o /tmp/sos && /tmp/sos

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Wraps the default allocator and counts the calls. `System` still does the
/// work, so the only thing this changes is that heap traffic becomes a number.
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

/// Rows are borrowed slices. Accepts rows from anywhere, needs a `Vec<&[T]>`.
fn widest_slices(grid: &[&[i32]]) -> usize {
    grid.iter().map(|row| row.len()).max().unwrap_or(0)
}

/// Rows are owned `Vec`s. Accepts `&Vec<Vec<T>>` directly, and nothing else.
fn widest_vecs(grid: &[Vec<i32>]) -> usize {
    grid.iter().map(|row| row.len()).max().unwrap_or(0)
}

/// Rows are anything that can be seen as a slice. Accepts all of the above.
fn widest<R: AsRef<[i32]>>(grid: &[R]) -> usize {
    grid.iter().map(|row| row.as_ref().len()).max().unwrap_or(0)
}

fn main() {
    let nested: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let rows: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    // ---- 1. One deref reaches the outer Vec, and stops there ---------------
    println!("1. `&Vec<Vec<T>>` is one deref from `&[Vec<T>]`");
    println!("   widest_vecs(&nested)   = {}", widest_vecs(&nested));
    println!("   The coercion rewrites the OUTER reference only: Vec<Vec<i32>>");
    println!("   derefs to [Vec<i32>]. It never reaches inside the element, so");
    println!("   widest_slices(&nested) is error[E0308] — expected &[&[i32]].");

    // ---- 2. Why it cannot: the row types are different shapes --------------
    let words = size_of::<usize>();
    println!("\n2. Because a `Vec<i32>` row and a `&[i32]` row are not the same bytes");
    println!("   size_of::<Vec<i32>>() = {} words (pointer, capacity, length)",
             size_of::<Vec<i32>>() / words);
    println!("   size_of::<&[i32]>()   = {} words (pointer, length)",
             size_of::<&[i32]>() / words);
    println!("   Two different layouts, so there is no reinterpretation to make.");
    println!("   A new row array has to be built, and that is an allocation.");

    // ---- 3. The conversion, counted ---------------------------------------
    // Snapshots are taken with no `println!` between them: printing allocates.
    let a0 = ALLOCS.load(Relaxed);
    let borrowed: Vec<&[i32]> = nested.iter().map(|v| v.as_slice()).collect();
    let a1 = ALLOCS.load(Relaxed);
    let widest_via_slices = widest_slices(&borrowed);
    let a2 = ALLOCS.load(Relaxed);

    println!("\n3. What the conversion costs");
    println!("   nested.iter().map(|v| v.as_slice()).collect::<Vec<&[i32]>>()");
    println!("     {} allocation(s) — {} rows x 2 words of pointer-and-length",
             a1 - a0, borrowed.len());
    println!("   then calling widest_slices on it: {} allocation(s), answer {}",
             a2 - a1, widest_via_slices);
    println!("   The row DATA is not copied — only the pointer-and-length pairs.");
    println!("   `nested` is still here, unchanged: {} rows", nested.len());

    // ---- 4. AsRef takes all three, converting nothing ----------------------
    let b0 = ALLOCS.load(Relaxed);
    let from_vecs = widest(&nested);
    let from_arrays = widest(&rows);
    let from_slices = widest(&borrowed);
    let b1 = ALLOCS.load(Relaxed);

    println!("\n4. `&[R] where R: AsRef<[i32]>` accepts all three, unconverted");
    println!("   Vec<Vec<i32>>  -> {from_vecs}");
    println!("   [[i32; 3]; 2]  -> {from_arrays}");
    println!("   Vec<&[i32]>    -> {from_slices}");
    println!("   total allocations for all three calls: {}", b1 - b0);

    // ---- 5. When `&[&[T]]` is the right type after all ---------------------
    let flat: Vec<i32> = (1..=9).collect();
    let windows: Vec<&[i32]> = flat.chunks(3).collect();
    println!("\n5. When the rows really are borrowed from somewhere else");
    println!("   flat.chunks(3).collect::<Vec<&[i32]>>() -> {windows:?}");
    println!("   Here no `Vec<Vec<i32>>` ever existed: the rows are views into");
    println!("   one buffer, widths {:?}, and `&[&[i32]]` is what they are.",
             windows.iter().map(|w| w.len()).collect::<Vec<_>>());
    println!("   widest_slices(&windows) = {}", widest_slices(&windows));

    // ---- 6. Choosing --------------------------------------------------------
    println!("\n6. Choosing");
    println!("   rows you own, one caller      -> &[Vec<i32>]        no conversion");
    println!("   rows borrowed from a buffer   -> &[&[i32]]          no conversion");
    println!("   a library, or several callers -> &[R: AsRef<[i32]>] no conversion");
    println!("   Build a Vec<&[i32]> only when the CALLER already has one.");
}
