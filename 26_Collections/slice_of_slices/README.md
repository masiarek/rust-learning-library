# Slices of slices — `&[&[T]]` and the grid you cannot pass

**Level:** 201 · working knowledge

**One line:** `&[T]` is the parameter type for a list, so `&[&[T]]` looks like the parameter type for a grid — but a `Vec<Vec<T>>` will not coerce to it, building one that will costs an allocation, and two other signatures take the grid you already have without converting anything.

```rust
fn widest(grid: &[&[i32]]) -> usize {
    grid.iter().map(|row| row.len()).max().unwrap_or(0)
}

fn main() {
    let nested: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5]];
    let rows: Vec<&[i32]> = nested.iter().map(|v| v.as_slice()).collect();
    println!("{}", widest(&rows));   // 3
}
```

That middle line is the whole subject of this page: what it is for, what it costs, and the two signatures that would not have needed it.

## The call that does not compile

Passing the grid straight in looks like it should work, and does not:

```text title="Abridged — real rustc output for grid.rs, without the trailing summary"
error[E0308]: mismatched types
 --> grid.rs:7:27
  |
7 |     println!("{}", widest(&nested));
  |                    ------ ^^^^^^^ expected `&[&[i32]]`, found `&Vec<Vec<i32>>`
  |                    |
  |                    arguments to this function are incorrect
  |
  = note: expected reference `&[&[i32]]`
             found reference `&Vec<Vec<i32>>`
note: function defined here
 --> grid.rs:1:4
  |
1 | fn widest(grid: &[&[i32]]) -> usize {
  |    ^^^^^^ ---------------
```

The surprise is that `&Vec<i32>` → `&[i32]` works everywhere else. It is one of the first coercions you meet, and it is why [`&[T]` is the type that belongs in a signature](../arrays_and_slices/README.md). One nesting level up, it stops.

## Deref coercion rewrites the outer reference, and stops there

`Vec<Vec<i32>>` derefs to `[Vec<i32>]`, so `&nested` coerces to `&[Vec<i32>]` — one step, applied to the reference you passed. It never reaches *inside* the element, because [coercion](../../29_Conversion/coercion/README.md) is not a `map`: there is no rule that turns a `&[A]` into a `&[B]` when `A` coerces to `B`.

And there could not be one, because the two row types are not the same bytes:

| row type | width | what it holds |
|---|---|---|
| [`Vec<i32>`](../the_vec/README.md) | 3 words | pointer, capacity, length |
| `&[i32]` | 2 words | pointer, length |

A `[Vec<i32>]` and a `[&[i32]]` of the same length are different sizes with different contents, so there is nothing to reinterpret. A new array of rows has to be built — and building an array is an allocation.

That also tells you what the conversion does *not* do. It copies pointer-and-length pairs, never the row data, and it borrows rather than moves, so the original grid is untouched and still yours afterwards.

## The three signatures

| signature | accepts | conversion needed |
|---|---|---|
| `&[Vec<i32>]` | `&Vec<Vec<i32>>` | none — one deref |
| `&[&[i32]]` | rows borrowed from anywhere | a `Vec<&[i32]>`, one allocation |
| `&[R] where R: AsRef<[i32]>` | all of the above, and `[[i32; 3]; 2]` | none |

**`&[Vec<i32>]` is the one most code should reach for** and the one most people skip, because `&[&[T]]` looks more general and is not: it is *differently* general. It accepts rows from unrelated owners, and refuses the single most common caller.

**[`AsRef<[T]>`](../../22_Generics/README.md) is the library answer.** One bound, every caller, nothing allocated — paid for with a generic parameter in the signature and a `.as_ref()` on each use. Reach for it when you do not know who calls you.

## When `&[&[T]]` is the right type after all

When no `Vec<Vec<T>>` ever existed. Rows that are views into one buffer arrive as slices already:

```rust
fn main() {
    let flat: Vec<i32> = (1..=9).collect();
    let rows: Vec<&[i32]> = flat.chunks(3).collect();
    println!("{rows:?}");   // [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
}
```

[`chunks`](../slice_methods/slice_chunks/README.md) hands back `&[i32]` because that is what a window into an existing buffer *is* — there is no owner per row to be had. The same goes for a grid assembled from separately-owned pieces, and for `&[&str]`, which is the shape you have already been using every time you took a list of string literals.

So the rule is about direction. **Build a `Vec<&[T]>` when the rows are already borrowed; do not build one to get past a signature.** If the conversion exists only to satisfy a parameter type, the parameter type is the thing to change.

## If you are coming from another language

- **Python.** `def widest(grid)` takes anything indexable and the question never arises — a list of lists, a list of tuples, and a list of `array.array` all walk the same way, because the row type is checked when you touch it rather than when you pass it. Rust's three signatures are that one Python parameter, split by what you are willing to promise: `&[Vec<i32>]` is *"rows the caller owns"*, `&[&[i32]]` is *"rows borrowed from somewhere"*, and `AsRef<[i32]>` is the one that actually corresponds to duck typing — *"anything that can show me a row"*, resolved at compile time instead of at each `len()`. The habit worth dropping is reaching for the conversion: in Python, rewriting the caller's data to fit a function is a code smell, and it is the same smell here. Note also that Python's `list[list[int]]` type hint is the shape `&[Vec<i32>]` has, not the one `&[&[i32]]` has, so a translated signature usually wants the first.
- **C.** `int **grid` is the closest thing, and the differences are the ones C programmers get bitten by: a `&[&[i32]]` carries a length for the outer array *and* one for every row, so there is no `int rows, int cols` pair to pass alongside and no way to disagree with it. `char *argv[]` is the same shape as `&[&str]`, which is why the argument list is the one place this type feels native. What C cannot express at all is the `&[Vec<i32>]` row — a pointer that also owns and can free what it points at.
- **ABAP.** A nested internal table (`TYPES: ty_row TYPE STANDARD TABLE OF i, ty_grid TYPE STANDARD TABLE OF ty_row`) is `Vec<Vec<i32>>`, and passing it to a `FORM` is always by reference to the table itself — the equivalent of `&[Vec<i32>]`, with no conversion and no choice. Rust's extra option is the one ABAP has no word for: a row that is a *view* into somebody else's table, valid only as long as that table is. The compiler tracks that lifetime, which is why `&[&[i32]]` is a type you can hand around and a field-symbol table is not.

## The verified output

<!-- output:slice_of_slices -->
*Verified output of [`slice_of_slices.rs`](examples/slice_of_slices.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `&Vec<Vec<T>>` is one deref from `&[Vec<T>]`
   widest_vecs(&nested)   = 4
   The coercion rewrites the OUTER reference only: Vec<Vec<i32>>
   derefs to [Vec<i32>]. It never reaches inside the element, so
   widest_slices(&nested) is error[E0308] — expected &[&[i32]].

2. Because a `Vec<i32>` row and a `&[i32]` row are not the same bytes
   size_of::<Vec<i32>>() = 3 words (pointer, capacity, length)
   size_of::<&[i32]>()   = 2 words (pointer, length)
   Two different layouts, so there is no reinterpretation to make.
   A new row array has to be built, and that is an allocation.

3. What the conversion costs
   nested.iter().map(|v| v.as_slice()).collect::<Vec<&[i32]>>()
     1 allocation(s) — 3 rows x 2 words of pointer-and-length
   then calling widest_slices on it: 0 allocation(s), answer 4
   The row DATA is not copied — only the pointer-and-length pairs.
   `nested` is still here, unchanged: 3 rows

4. `&[R] where R: AsRef<[i32]>` accepts all three, unconverted
   Vec<Vec<i32>>  -> 4
   [[i32; 3]; 2]  -> 3
   Vec<&[i32]>    -> 4
   total allocations for all three calls: 0

5. When the rows really are borrowed from somewhere else
   flat.chunks(3).collect::<Vec<&[i32]>>() -> [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
   Here no `Vec<Vec<i32>>` ever existed: the rows are views into
   one buffer, widths [3, 3, 3], and `&[&[i32]]` is what they are.
   widest_slices(&windows) = 3

6. Choosing
   rows you own, one caller      -> &[Vec<i32>]        no conversion
   rows borrowed from a buffer   -> &[&[i32]]          no conversion
   a library, or several callers -> &[R: AsRef<[i32]>] no conversion
   Build a Vec<&[i32]> only when the CALLER already has one.
```
<!-- /output -->

## Practice

**A signature you cannot change, and the two you would have written.** A crate you do not own exports `fn column_sums(grid: &[&[i32]]) -> Vec<i32>`. You have a `Vec<Vec<i32>>` whose rows are ragged, and a `[[i32; 3]; 2]`.

1. Call `column_sums` with the `Vec<Vec<i32>>`. Count the allocations your conversion costs, and separately the ones the call itself makes — they are not the same number, and only one of them is yours to avoid.
2. Call it with the array of arrays. The line has the same shape, but work out what `fixed.iter()` yields before you write it, and what `as_slice()` throws away.
3. Now write the signature that would have taken both with no conversion at all, and check the allocation count against your answer to 1.

Then say which signature you would have used if the `Vec<Vec<i32>>` were the only caller that ever existed.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:slice_of_slices_kata -->
*[`slice_of_slices_kata.rs`](examples/slice_of_slices_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
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
```
<!-- /source -->

<!-- output:slice_of_slices_kata -->
*Verified output of [`slice_of_slices_kata.rs`](examples/slice_of_slices_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Vec<Vec<i32>> through a &[&[i32]] signature
   column_sums = [111, 22, 3]
   the conversion cost 1 allocation(s); the call itself 1
   (the call allocates too: column_sums builds its own Vec)

2. [[i32; 3]; 2] through the same signature
   column_sums = [5, 7, 9]
   `fixed.iter()` yields &[i32; 3], and as_slice() forgets the 3.

3. The signature that takes both, unconverted
   from Vec<Vec<i32>> : [111, 22, 3]
   from [[i32; 3]; 2] : [5, 7, 9]
   allocations for both calls: 2 — two result Vecs, no row arrays

4. If only the Vec<Vec<i32>> ever called it: &[Vec<i32>]
   `&owned` coerces to that directly — no generics, no conversion.
```
<!-- /output -->

</details>

## See also

- [Grids and nested `Vec`s](../vec_of_vecs/README.md) — where the grid comes from, and what each shape costs to build
- [Arrays and slices](../arrays_and_slices/README.md) — why `&[T]` and not `&Vec<T>` in a signature, one nesting level down
- [`Vec::as_slice`](../vec_methods/vec_as_slice/README.md) — the method the conversion calls on every row
- [`slice::chunks`](../slice_methods/slice_chunks/README.md) — where a `Vec<&[T]>` legitimately comes from
- [Deref coercion](../../29_Conversion/coercion/README.md) — the rule that gets you one level and no further
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — why the conversion uses `iter()` and leaves the grid intact

## Sources

[`AsRef` ↗](https://doc.rust-lang.org/std/convert/trait.AsRef.html) and [`Vec::as_slice` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice) in std; the coercion rules are in the reference's [*Type coercions* ↗](https://doc.rust-lang.org/reference/type-coercions.html) chapter, which is where to confirm that no coercion reaches inside a generic parameter.

## Po polsku

`&[T]` to typ, który wpisuje się w sygnaturę funkcji zamiast `&Vec<T>` — i to jest jedna z pierwszych rzeczy, których uczy się w Ruscie. Naturalny wniosek, że siatkę przekazuje się jako `&[&[T]]`, jest **błędny**, i to w sposób, który zaskakuje: `&Vec<Vec<i32>>` nie skonwertuje się na `&[&[i32]]`, bo koercja przez `Deref` przepisuje tylko **zewnętrzną** referencję i nigdy nie wchodzi do środka elementu.

Powód jest fizyczny, nie składniowy. `Vec<i32>` to trzy słowa maszynowe (wskaźnik, pojemność, długość), a `&[i32]` to dwa (wskaźnik, długość) — to po prostu inne układy bajtów, więc nie ma czego „przeinterpretować". Trzeba zbudować nową tablicę wierszy, a to znaczy jedną alokację. Kopiowane są wyłącznie pary wskaźnik-długość, nigdy dane wierszy, i oryginalna siatka zostaje nietknięta.

Praktyczny wniosek jest taki, że w większości przypadków **nie chcesz `&[&[T]]`**. Jeśli wołający ma `Vec<Vec<T>>`, napisz `&[Vec<T>]` — przejdzie bez żadnej konwersji. Jeśli piszesz bibliotekę i nie wiesz, kto zawoła, użyj `&[R] where R: AsRef<[i32]>`, które przyjmie wszystkie trzy warianty za darmo. `&[&[T]]` jest właściwe wtedy, gdy wiersze **naprawdę są pożyczone** skądinąd — na przykład z `chunks()` na jednym płaskim buforze — czyli gdy żaden `Vec<Vec<T>>` nigdy nie istniał.

**Szukaj po polsku:** wycinek w Ruscie · `slice` a `Vec` · koercja `Deref` · tablica dwuwymiarowa w Ruscie · `rust &[&[T]] expected found` · `E0308`
