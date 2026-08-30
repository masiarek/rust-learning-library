//! Kata solution: the same grid three ways, and the bill for each.
//!
//!   rustc --edition 2024 vec_of_vecs_kata.rs -o /tmp/vovk && /tmp/vovk

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

/// A grid held as one block. The width has to travel with it — that is the
/// whole difference between this and `Vec<Vec<u8>>`, which carries its own.
struct Flat {
    cells: Vec<u8>,
    width: usize,
}

impl Flat {
    fn new(width: usize, height: usize) -> Self {
        Flat { cells: vec![0; width * height], width }
    }
    fn set(&mut self, r: usize, c: usize, v: u8) {
        assert!(c < self.width, "column {c} is off a {}-wide grid", self.width);
        self.cells[r * self.width + c] = v;
    }
    fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.cells.chunks(self.width)
    }
}

/// Renders any grid-shaped thing as a picture. `{:?}` will not do this.
fn show(label: &str, rows: impl Iterator<Item = Vec<String>>) {
    println!("   {label}");
    for cells in rows {
        println!("     {}", cells.join(" "));
    }
}

fn main() {
    println!("1. The same 3x3 grid, two shapes, one cell set");
    let mut nested: Vec<Vec<u8>> = vec![vec![0u8; 3]; 3];
    nested[1][1] = 5;
    let mut flat = Flat::new(3, 3);
    flat.set(1, 1, 5);
    show("Vec<Vec<u8>>", nested.iter().map(|r| r.iter().map(|c| c.to_string()).collect()));
    show("Flat", flat.rows().map(|r| r.iter().map(|c| c.to_string()).collect()));
    println!("   `chunks(width)` is what turns the flat block back into rows —");
    println!("   a borrowed view per row, with no allocation and no copying.");

    println!();
    println!("2. The bill at scale: a 1000-row grid");
    let before = ALLOCS.load(Relaxed);
    let big_nested: Vec<Vec<u8>> = vec![vec![0u8; 1000]; 1000];
    let mid = ALLOCS.load(Relaxed);
    let big_flat = Flat::new(1000, 1000);
    let after = ALLOCS.load(Relaxed);
    println!("   vec![vec![0; 1000]; 1000]  -> {} allocations", mid - before);
    println!("   Flat::new(1000, 1000)      -> {} allocation", after - mid);
    println!("   {} cells either way ({} rows x {} wide, and {} in one block).",
             big_nested.len() * big_nested[0].len(),
             big_nested.len(),
             big_nested[0].len(),
             big_flat.cells.len());
    println!("   One allocation per row, plus the outer Vec. What those thousand");
    println!("   extra allocations buy is rows that need not agree on a length.");

    println!();
    println!("3. What only Vec<Vec<T>> can do: rows of different lengths");
    let mut ragged: Vec<Vec<u8>> = vec![vec![1], vec![2, 2], vec![3, 3, 3]];
    ragged[2].push(3); // one row grows; the others are untouched
    show("ragged", ragged.iter().map(|r| r.iter().map(|c| c.to_string()).collect()));
    println!("   Row lengths: {:?}", ragged.iter().map(|r| r.len()).collect::<Vec<_>>());
    println!("   [[u8; 3]; 3] cannot express this at all — the length is in the");
    println!("   type. Flat cannot either without a second Vec of row offsets.");
    println!("   If your rows are always the same length, that is a fact the");
    println!("   nested form is paying to keep flexible.");

    println!();
    println!("4. The flat form's real cost: the width can go missing");
    let mut f = Flat::new(3, 3);
    f.set(2, 2, 9);
    println!("   f.set(2, 2, 9) -> index {} in a {}-long Vec", 2 * 3 + 2, f.cells.len());
    println!("   Without the assert, f.set(0, 5, 9) writes index 5 — which is");
    println!("   row 1 column 2, a real cell, silently. The nested form gets an");
    println!("   out-of-bounds panic for free, because each row knows its own len.");
    // Silence the default hook so the transcript stays deterministic — the
    // thread id and backtrace note would differ on every run.
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let ok = std::panic::catch_unwind(|| {
        let mut g = Flat::new(3, 3);
        g.set(0, 5, 9);
    });
    std::panic::set_hook(previous);
    println!("   with the assert in place, f.set(0, 5, 9) panics: {}", ok.is_err());

    println!();
    println!("5. vec![row; 3] clones — it does not alias");
    let row = vec![0u8; 3];
    let mut by_clone = vec![row.clone(), row.clone(), row];
    by_clone[0][0] = 7;
    println!("   {by_clone:?}");
    println!("   Identical to vec![vec![0; 3]; 3], which is that written short.");
    println!("   Python's [[0] * 3] * 3 stores one list three times, so the same");
    println!("   assignment shows up in all three rows. Rust has no way to write");
    println!("   that by accident: sharing needs Rc<RefCell<_>> and says so.");
}
