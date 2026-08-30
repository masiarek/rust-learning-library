# Grids and nested `Vec`s

**Level:** 201 · working knowledge

**One line:** `vec![vec![0; w]; h]` is a grid that costs one allocation per row and clones every one of them — which is why Python's `[[0] * w] * h` aliasing bug cannot happen here, and why the flat `Vec` with `r * w + c` is what most real code holds.

```rust
fn main() {
    let mut grid = vec![vec![0u8; 4]; 4];
    grid[2][2] = 5;
    for row in &grid {
        println!("{row:?}");   // [0, 0, 0, 0] … [0, 0, 5, 0] … [0, 0, 0, 0]
    }
}
```

## Three shapes, and what each one costs

The same 4×4 grid of `u8`, written three ways. The allocation counts below are measured by a counting [global allocator](../../09_Advanced/the_global_allocator/README.md) in the program at the bottom of this page, not estimated:

| | allocations | rows can differ in length | reading a cell |
|---|---|---|---|
| `[[u8; 4]; 4]` | **0** — every byte inline | no | one offset |
| `vec![vec![0u8; 4]; 4]` | **5** — one per row, plus the outer `Vec` | **yes** | two pointers |
| `vec![0u8; 16]` + a width | **1** | no | one offset |

At 1000 rows the nested form is 1001 allocations against the flat form's 1. The middle column is the whole reason to pay it: rows that need not agree on a length is a thing neither other shape can express.

The outer `Vec` in the nested form is three `usize`s wide — pointer, length, capacity — and holds none of the data. That is [the same three numbers](../the_vec/README.md) a `Vec` always is; there are just five of them stacked here.

## `{:?}` prints one line, not a picture

```rust
fn main() {
    let mut grid = vec![vec![0u8; 4]; 4];
    grid[2][2] = 5;
    println!("{grid:?}");
    // [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 5, 0], [0, 0, 0, 0]]
}
```

That is the whole output — one line. A grid on the screen needs a loop, and `{:#?}` is not the shortcut: it wraps at *every* element, so this grid becomes twenty-six lines rather than four.

Worth checking against your notes, because it is an easy thing to write down wrong: a four-line square of digits under a `println!("{:?}", …)` did not come from that statement.

## The trap: `col = 1` inside `iter_mut`

```rust
fn main() {
    let mut grid = [[0u8; 4]; 2];
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 1;          // `cell = 1` is E0308 — see below
        }
    }
    println!("{grid:?}");       // [[1, 1, 1, 1], [1, 1, 1, 1]]
}
```

`iter_mut()` yields `&mut u8`, so `cell` is a *reference*. Assigning to the binding would repoint the reference; assigning through it, with `*cell`, changes the cell. rustc names the fix in the error:

```text title="Abridged — real rustc output for grid.rs"
error[E0308]: mismatched types
 --> grid.rs:5:19
  |
4 |         for (j, col) in row.iter_mut().enumerate() {
  |                 --- expected due to the type of this binding
5 |             col = 1;
  |                   ^ expected `&mut u8`, found integer
  |
help: consider dereferencing here to assign to the mutably borrowed value
  |
5 |             *col = 1;
  |             +
```

The read-only version of the same loop has no `*` in it, because `print!("{col}")` derefs on your behalf through [`Display`](../../12_Traits/README.md) — which is exactly why the mutable one catches people. The two loops look symmetric and are not.

## The rows are clones, not aliases

`vec![elem; n]` requires `elem: Clone` and produces *n* independent values. So `vec![vec![0; 4]; 4]` is four separate row buffers, and writing to one row is invisible to the others.

This is the single most useful thing to know about it if you arrive from Python, where the innocent-looking `[[0] * 4] * 4` stores **the same list** four times and `grid[0][0] = 9` shows up in every row. Rust has no way to write that by accident. Sharing a row between rows needs `Rc<RefCell<Vec<u8>>>` and announces itself in the type.

## When the flat `Vec` is the right answer

One block, one allocation, and `r * width + c` to index it. `chunks(width)` turns it back into rows on demand — a borrowed slice per row, no copying:

```rust
fn main() {
    let width = 4;
    let cells = vec![0u8; width * 4];
    for row in cells.chunks(width) {
        println!("{row:?}");   // [0, 0, 0, 0], four times
    }
}
```

What you give up is that the width now travels separately, and nothing checks it. `set(0, 5, 9)` on a 3-wide grid computes index 5, which is a real cell — row 1, column 2 — and writes there silently. The nested form gets that bounds check for free, because each row knows its own length. Wrap the flat form in a struct that holds the width and asserts on it, or you have traded a `Vec` of `Vec`s for a class of bug that does not panic.

## If you are coming from another language

- **Python.** `vec![vec![0; w]; h]` is `[[0] * w for _ in range(h)]`, **not** `[[0] * w] * h` — the comprehension, always. The star form is Python's most-reported list gotcha, and the reason it happens is that `*` on a list copies references; Rust's `vec![elem; n]` calls `Clone`, so the same shape is safe. The other habit that transfers badly is `grid[r][c]` on a ragged list: Python raises `IndexError` and you catch it, Rust panics and you do not — reach for `grid.get(r).and_then(|row| row.get(c))`, which gives you `Option<&T>` and one `match`. For real numeric grids, Python reaches for NumPy, whose `ndarray` is a flat buffer with a shape — exactly the third shape on this page, and for exactly the same reason. `ndarray` the Rust crate is the same idea again.
- **ABAP.** A nested internal table — `TYPES: ty_row TYPE STANDARD TABLE OF i WITH EMPTY KEY, ty_grid TYPE STANDARD TABLE OF ty_row WITH EMPTY KEY` — is `Vec<Vec<i32>>` closely, including that each row is its own table and rows may differ in length. What transfers is the `LOOP AT ... ASSIGNING <fs>` habit: `<fs>` is a field symbol, a reference, and writing `<fs> = 1` changes the table row rather than the symbol. Rust's `for cell in row.iter_mut()` is that loop, and `*cell = 1` is that assignment — the `*` is doing the work ABAP's angle brackets do. The difference is that ABAP will happily let you `LOOP AT itab` and `DELETE` from it in the same breath; here that is a borrow-check error before the program runs.
- **C.** `uint8_t grid[4][4]` is the first row of the table — one contiguous block, and the dimensions are part of the type. `uint8_t **grid` built with a loop of `malloc` is the second — the pointer-chase-per-row version, plus a matching loop of `free` that Rust's `Drop` writes for you. `malloc(w * h)` with `grid[r * w + c]` is the third, and it is the one numerical C actually uses, for the same cache reasons it wins here.
- **Java / C#.** `new int[4][4]` in Java is genuinely an array of arrays — four separate objects, like the nested form, with the same indirection. C#'s `new int[4, 4]` is a true rectangular array, one block, closer to `[[u8; 4]; 4]`; `new int[4][]` is the jagged one. Rust makes you pick the same way, and the choice is visible in the type rather than in a comma.

---

## The verified output

<!-- output:vec_of_vecs -->
*Verified output of [`vec_of_vecs.rs`](examples/vec_of_vecs.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three ways to hold a 4x4 grid of u8
   [[u8; 4]; 4]          0 allocations — every byte is inline
   vec![vec![0; 4]; 4]   5 allocations — one per row, plus the outer Vec
   vec![0; 16]           1 allocation  — one block, indexed r * W + c
   The nested form is the only one that can have rows of different
   lengths, and the only one that pays for the privilege.

2. Contiguous, or a pointer chase per row
   [[u8; 4]; 4]: row 1 begins 4 bytes after row 0 — one block, no gaps
   size_of_val(&fixed) = 16 bytes, all of it the data
   the outer Vec is 3 usizes wide — ptr, len, cap, and none of the data
   Reading nested[r][c] follows two pointers; flat[r * W + c] follows one.
   flat[2 * W + 2] is the cell nested[2][2] names: 0

3. Printing: `{:?}` is one line, not a picture
   println!("{:?}", grid) gives you this, all on one line:
   [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 5, 0], [0, 0, 0, 0]]
   A grid on the screen needs a loop — the Debug impl never wraps:
     0 0 0 0
     0 0 0 0
     0 0 5 0
     0 0 0 0
   `{:#?}` wraps, but one element per line — sixteen lines for this grid.

4. Updating through iter_mut: the `*` is the whole lesson
   after *cell = r * W + c:
      0  1  2  3
      4  5  6  7
      8  9 10 11
     12 13 14 15
   `cell` is a &mut u8. Assigning to it repoints the binding, which is
   both a type error and, if it compiled, not what you meant.

5. The rows are clones, not aliases
   rows[0] and rows[1] share a buffer? false
   after rows[0][0] = 9 -> [[9, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
   vec![elem; n] CLONES elem n times. Python's [[0]*4]*4 stores the
   same list four times, so the same assignment changes every row.
```
<!-- /output -->

## Practice

**Build the same grid twice and send it the bill.** Write a `Flat` struct holding `cells: Vec<u8>` and a `width`, with `set(r, c, v)` and a `rows()` that hands back one borrowed slice per row. Then build a 3×3 grid both as `Vec<Vec<u8>>` and as your `Flat`, set the middle cell in each, and print both as pictures rather than as `{:?}`.

Then measure instead of assuming: count the allocations for a 1000×1000 grid in both shapes, using a counting global allocator, and take the snapshots with no `println!` between them.

Three questions the numbers raise. What can `Vec<Vec<u8>>` express that neither other shape can — and does your program actually need it? What does `Flat::set(0, 5, 9)` do on a 3-wide grid if you leave the assertion out, and why is that worse than a panic? And `vec![row.clone(), row.clone(), row]` against `vec![row; 3]`: same thing, or not?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:vec_of_vecs_kata -->
*[`vec_of_vecs_kata.rs`](examples/vec_of_vecs_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
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
```
<!-- /source -->

<!-- output:vec_of_vecs_kata -->
*Verified output of [`vec_of_vecs_kata.rs`](examples/vec_of_vecs_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The same 3x3 grid, two shapes, one cell set
   Vec<Vec<u8>>
     0 0 0
     0 5 0
     0 0 0
   Flat
     0 0 0
     0 5 0
     0 0 0
   `chunks(width)` is what turns the flat block back into rows —
   a borrowed view per row, with no allocation and no copying.

2. The bill at scale: a 1000-row grid
   vec![vec![0; 1000]; 1000]  -> 1001 allocations
   Flat::new(1000, 1000)      -> 1 allocation
   1000000 cells either way (1000 rows x 1000 wide, and 1000000 in one block).
   One allocation per row, plus the outer Vec. What those thousand
   extra allocations buy is rows that need not agree on a length.

3. What only Vec<Vec<T>> can do: rows of different lengths
   ragged
     1
     2 2
     3 3 3 3
   Row lengths: [1, 2, 4]
   [[u8; 3]; 3] cannot express this at all — the length is in the
   type. Flat cannot either without a second Vec of row offsets.
   If your rows are always the same length, that is a fact the
   nested form is paying to keep flexible.

4. The flat form's real cost: the width can go missing
   f.set(2, 2, 9) -> index 8 in a 9-long Vec
   Without the assert, f.set(0, 5, 9) writes index 5 — which is
   row 1 column 2, a real cell, silently. The nested form gets an
   out-of-bounds panic for free, because each row knows its own len.
   with the assert in place, f.set(0, 5, 9) panics: true

5. vec![row; 3] clones — it does not alias
   [[7, 0, 0], [0, 0, 0], [0, 0, 0]]
   Identical to vec![vec![0; 3]; 3], which is that written short.
   Python's [[0] * 3] * 3 stores one list three times, so the same
   assignment shows up in all three rows. Rust has no way to write
   that by accident: sharing needs Rc<RefCell<_>> and says so.
```
<!-- /output -->

</details>

---

## See also

- [`Vec`](../the_vec/README.md) — the three numbers each of these rows is, and how one grows
- [Arrays and slices](../arrays_and_slices/README.md) — why `[[u8; 4]; 4]` is a different type from `[[u8; 5]; 4]`, and what `chunks` hands back
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the counting allocator this page's numbers come from
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — which of the three yields the `&mut` that needs the `*`
- [Interior mutability](../../09_Advanced/interior_mutability/README.md) — `Rc<RefCell<T>>`, the thing you would need to get Python's aliasing on purpose

## Sources

[`std::vec::Vec` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html) and [`slice::chunks` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks); the `vec![elem; n]` clone semantics are stated under [the `vec!` macro ↗](https://doc.rust-lang.org/std/macro.vec.html).
