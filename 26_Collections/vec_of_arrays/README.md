# A `Vec` of arrays

**Level:** 201 · working knowledge

**One line:** `Vec<[T; N]>` is a growable list of fixed-width rows — one allocation for all of them, rows that copy instead of moving, and a row of the wrong length rejected by the compiler rather than discovered at run time.

```rust
fn main() {
    let mut addrs: Vec<[u8; 4]> = Vec::new();   // no allocation yet
    addrs.push([192, 168, 0, 1]);
    addrs.push([10, 0, 0, 1]);
    addrs.push([127, 0, 0, 1]);

    for addr in &addrs {
        for octet in addr {
            print!(" {octet}");
        }
        println!();
    }
    // 192 168 0 1
    // 10 0 0 1
    // 127 0 0 1
}
```

`Vec::new()` allocates nothing — a `Vec` is [three numbers](../the_vec/README.md) and starts as `(dangling, 0, 0)`. The first `push` takes one block big enough for four rows, and rows two and three are already paid for.

## The fourth shape

[Grids and nested `Vec`s](../vec_of_vecs/README.md) weighs the fixed grid against the nested `Vec`. This is the shape between them:

| | allocations, 1000 rows | rows may differ in length | a row is | wrong length caught |
|---|---|---|---|---|
| `[[u8; 4]; 1000]` | **0** — every byte inline | no | `Copy` | compile time |
| `Vec<[u8; 4]>` | **1** | no | `Copy` | **compile time** |
| `Vec<Vec<u8>>` | **1001** | **yes** | moved | never |
| `Vec<u8>` + a width | **1** | no | a `&[u8]` view | never |

The counts are measured by a counting [global allocator](../../09_Advanced/the_global_allocator/README.md) in the program at the bottom of this page.

Two questions, in order. **Does the row count change while the program runs?** If not, the fixed grid costs no allocation at all. **Do rows have to differ in length?** If not, `Vec<[T; N]>` is the answer, and `Vec<Vec<T>>` is a thousand allocations spent on flexibility nothing uses.

## One allocation, not one per row

A `[u8; 4]` has no indirection: four bytes, nothing else. So a `Vec` of them holds the rows *in* its buffer, end to end, exactly as a `Vec<u8>` holds bytes.

```rust
fn main() {
    let addrs: Vec<[u8; 4]> = vec![[192, 168, 0, 1], [10, 0, 0, 1]];
    let gap = addrs[1].as_ptr() as usize - addrs[0].as_ptr() as usize;
    println!("{gap}");   // 4 — one block, no gaps
}
```

`vec![vec![0u8; 4]; 1000]` is 1001 allocations; `vec![[0u8; 4]; 1000]` is one. The difference is not a micro-optimisation at that scale — it is a thousand round trips to the allocator, a thousand headers, and a pointer chase per row for the rest of the program's life.

## The rows are `Copy`

`[T; N]` is `Copy` when `T` is, which a `Vec<T>` never is. So pushing a row does not consume it:

```rust
fn main() {
    let mut addrs: Vec<[u8; 4]> = Vec::new();
    let loopback = [127, 0, 0, 1];
    addrs.push(loopback);
    println!("{loopback:?}");   // [127, 0, 0, 1] — still yours
}
```

Make the row a `Vec<u8>` and the same four lines stop compiling, with the reason stated in the error rather than left to be deduced:

```text title="Abridged — real rustc output for moved_row.rs, without the help: block"
error[E0382]: borrow of moved value: `loopback`
 --> moved_row.rs:5:16
  |
3 |     let loopback = vec![127, 0, 0, 1];
  |         -------- move occurs because `loopback` has type `Vec<u8>`, which does not implement the `Copy` trait
4 |     rows.push(loopback);
  |               -------- value moved here
5 |     println!("{loopback:?}");
  |                ^^^^^^^^ value borrowed here after move
```

The fix is `.clone()`, which is a second heap allocation. That is the trade the [array-or-`Vec`](../array_or_vec/README.md) page makes for a single row, applied to a list of them.

## A row of the wrong length does not compile

This is the whole reason to reach for `Vec<[T; N]>` when a width is a fact about the problem rather than about this run. An address is four bytes; three is not a short address, it is a bug:

```rust
fn main() {
    let mut addrs: Vec<[u8; 4]> = Vec::new();
    addrs.push([192, 168, 0, 1]);
    // addrs.push([10, 0, 0]);      // E0308 — see the transcript below
    println!("{addrs:?}");          // [[192, 168, 0, 1]]
}
```

```text title="Abridged — real rustc output for wrong_width.rs, without the note: block"
error[E0308]: mismatched types
 --> wrong_width.rs:4:16
  |
4 |     addrs.push([10, 0, 0]);
  |           ---- ^^^^^^^^^^ expected an array with a size of 4, found one with a size of 3
  |           |
  |           arguments to this method are incorrect
```

`Vec<Vec<u8>>` accepts that push without a word, and the mistake surfaces later as a panic somewhere else, or as a packet nobody can parse. Handing it a `Vec` instead of an array is refused for the same reason — *expected `[u8; 4]`, found `Vec<{integer}>`* — which is worth knowing because it is the first thing a `Vec<Vec<T>>` habit will try.

## `for octet in row` and `for octet in *row`

Both compile, and they hand you different things:

```rust
fn main() {
    let addrs: Vec<[u8; 4]> = vec![[192, 168, 0, 1]];
    for row in &addrs {              // row: &[u8; 4]
        for octet in row {           // octet: &u8
            print!(" {octet}");
        }
        for octet in *row {          // octet: u8 — the row is Copy
            print!(" {octet}");
        }
        println!();                  //  192 168 0 1 192 168 0 1
    }
}
```

`&[T; N]` iterates as a slice and yields references; `[T; N]` by value yields the elements, because [arrays became `IntoIterator` by value ↗](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html) in edition 2021. The `*` is what picks between them, and it costs nothing here since the row is `Copy`.

Nothing on screen tells the two loops apart: `&u8` and `u8` both print as a number, and `octet + 1` compiles for either. The difference shows up somewhere else entirely — a function wanting `u8`, a `collect()` into `Vec<u8>` — with an error naming a line you were not thinking about. When one of those appears, the `&` came from the loop.

## Flat, and back again

The buffer is already flat, so both directions of the conversion are cheap, and only one of them can fail:

| you have | you want | how |
|---|---|---|
| `&Vec<[T; N]>` | `&[T]` | [`as_flattened()` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.as_flattened) — a borrowed view, no copy |
| `Vec<[T; N]>` | `Vec<T>` | [`into_flattened()`](../vec_methods/vec_into_flattened/README.md) — the same buffer, retyped |
| `&[T]` | `Vec<[T; N]>` | [`chunks_exact(N)` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact) plus a `try_into` per row |

The trip back needs the `try_into` because a slice carries its length in the value and an array carries it in the type, so the conversion is a `Result`. Skipping it does not compile:

```text title="Abridged — real rustc output for regroup.rs, first six lines"
error[E0277]: a value of type `Vec<[u8; 4]>` cannot be built from an iterator over elements of type `&[u8]`
 --> regroup.rs:3:53
  |
3 |     let addrs: Vec<[u8; 4]> = bytes.chunks_exact(4).collect();
  |                                                     ^^^^^^^ value of type `Vec<[u8; 4]>` cannot be built from `std::iter::Iterator<Item=&[u8]>`
  |
help: the trait `FromIterator<&[u8]>` is not implemented for `Vec<[u8; 4]>`
      but trait `FromIterator<[u8; 4]>` is implemented for it
```

`chunks_exact` drops a trailing partial chunk rather than yielding a short one, which is what makes the `try_into` unreachable in practice — and `.remainder()` hands that tail back, so nothing is lost silently.

## When it is the wrong type

**Rows genuinely differ in length.** Then `Vec<Vec<T>>` and its thousand allocations are buying something, and this type cannot express the data at all.

**`N` is large and the rows move.** `Vec<[u8; 4096]>` copies four kilobytes per `push` that reallocates and per `remove`; `Vec<Vec<u8>>` moves a pointer. The crossover is not a number worth memorising — measure it — but a `swap_remove` on a `Vec` of big arrays is a memcpy where the nested form is three words.

**`N` is a run-time value.** `[T; n]` needs `n` to be a constant, so a width that arrives from a file or a header leaves you with the flat `Vec` and a width field, checked by hand.

## If you are coming from another language

- **Python.** There is no counterpart in the standard library: a list of lists is `Vec<Vec<T>>`, and a list of 4-tuples is closer in spirit but still one object per row. The type that *does* match is `numpy.ndarray` with `shape=(n, 4)` — one flat buffer, a fixed row width, and rows you cannot make ragged — which is exactly why NumPy exists and why `np.array([[1,2,3],[4,5]])` gives you an array of objects rather than a 2-D array. `array.array('B', …)` is the flat half without the grouping. The habit that transfers badly is unpacking: `for a, b, c, d in addrs` is idiomatic Python and has no direct form here, though `let [a, b, c, d] = *row;` destructures the array and is checked for arity by the compiler.
- **ABAP.** An internal table of a flat structure — `TYPES: BEGIN OF ty_addr, o1 TYPE x, … END OF ty_addr, tt_addr TYPE STANDARD TABLE OF ty_addr WITH EMPTY KEY` — is the same layout: rows of a fixed width, stored end to end, no per-row allocation. The width is fixed by the structure definition, and a row that does not match it is a syntax error, which is precisely what `[u8; 4]` buys. What differs is that ABAP's fixed width comes from *naming every field*, so widening a row means editing the type and every `MOVE-CORRESPONDING` that touches it; `[T; N]` is one number, and the compiler finds all the call sites. Nearer still is a `TYPE x LENGTH 4` field — a fixed-length byte string — which is `[u8; 4]` almost exactly, down to being copied by value on assignment.
- **C.** `uint8_t (*addrs)[4]` — a pointer to arrays of four — is this type, and its declaration syntax is the reason most C programmers reach for `uint8_t **addrs` instead and take the pointer chase. `realloc` on the first is a single block that stays contiguous; the second needs a loop of `malloc` and a matching loop of `free`. Rust writes both of those for you and the type reads left to right.
- **Java / C#.** Java's `byte[][]` is always the nested form — an array of separate row objects — so this shape has no Java spelling short of a flat `byte[]` and arithmetic. C# has both: `byte[][]` is jagged, `byte[,]` is rectangular and one block, but its dimensions are values rather than types, so a 3-wide row assigned into a 4-wide array is a run-time `IndexOutOfRangeException` where Rust's is `E0308`. C#'s `Span<byte>` over a flat buffer is the closest thing to `as_flattened()`.

---

## The verified output

<!-- output:vec_of_arrays -->
*Verified output of [`vec_of_arrays.rs`](examples/vec_of_arrays.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A list that grows, of rows that do not
   192.168.0.1
   10.0.0.1
   127.0.0.1
   The Vec's length is data; each row's is part of its type.

2. Vec::new() allocates nothing until you push
   Vec::new()             -> 0 allocations, capacity 0
   three pushes           -> 1 allocation, capacity 4
   The first push takes capacity 4 in one block. Rows two and
   three cost nothing at all: they were already paid for.

3. One allocation for a thousand rows, not a thousand and one
   vec![[0u8; 4]; 1000]      -> 1 allocation
   vec![vec![0u8; 4]; 1000]  -> 1001 allocations
   1000 rows of four bytes either way; 1000 in one block, 1000 in a
   thousand separate ones the outer Vec only points at.
   Row 1 begins 4 bytes after row 0 — one block, no gaps.

4. The rows are Copy, so pushing one does not consume it
   after addrs.push(loopback), loopback is still [127, 0, 0, 1]
   the Vec<u8> row moves instead: push consumes it, and reading
   it afterwards is error[E0382]: borrow of moved value
   addrs holds 4 rows, rows holds 1

5. What each loop hands you
   for row in &addrs      -> &[u8; 4]
   for octet in row       -> &u8
   for octet in *row      -> u8
   The `*` copies the row out, because [u8; 4] is Copy. Without
   it you iterate a &[u8; 4] and get references — which print
   identically, so `{}` will never tell you which one you have.

6. Flat, and back again
   as_flattened(): 4 rows -> 16 bytes, borrowed, no copy
   into_flattened(): the same buffer, retyped — 16 bytes
   18 bytes regroup into 4 addresses, with [8, 8] left over
   The trip back is fallible and the type says so: a slice has no
   length in its type, so try_into is the step that supplies one.
```
<!-- /output -->

## Practice

**Read the wire, and make the short row impossible.** Four-byte addresses arrive as one flat `Vec<u8>`, and the last read stopped two bytes into an address. Write `regroup(bytes: &[u8]) -> (Vec<[u8; 4]>, &[u8])` that returns the complete rows and the tail that did not divide, then sort and dedup the result.

Then send both shapes the bill: build a thousand addresses as `Vec<[u8; 4]>` and as `Vec<Vec<u8>>` from the same flat buffer, and count the allocations with a counting global allocator.

Three questions the code raises. Why does `bytes.chunks_exact(4).collect::<Vec<[u8; 4]>>()` not compile, when every chunk it yields is four bytes long? What does `.remainder()` return, and what would have happened to those bytes without it? And `Vec<Vec<u8>>` can hold a two-byte address — is that a feature here, or the bug you were trying to make unrepresentable?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:vec_of_arrays_kata -->
*[`vec_of_arrays_kata.rs`](examples/vec_of_arrays_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: regroup a flat byte buffer into fixed-width rows, and
//! send both shapes the allocation bill.
//!
//!   rustc --edition 2024 vec_of_arrays_kata.rs -o /tmp/voak && /tmp/voak

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

/// Split a flat buffer into four-byte rows plus whatever did not divide.
///
/// `chunks_exact` guarantees every chunk is four bytes long and the type
/// system cannot see it, so `try_into` is where the guarantee is cashed.
/// `expect` is honest here in a way `unwrap` on user input never is: the
/// iterator's own contract is what makes it unreachable.
fn regroup(bytes: &[u8]) -> (Vec<[u8; 4]>, &[u8]) {
    let rows = bytes
        .chunks_exact(4)
        .map(|c| c.try_into().expect("chunks_exact(4) yields four bytes"))
        .collect();
    (rows, bytes.chunks_exact(4).remainder())
}

fn dotted(addr: &[u8; 4]) -> String {
    addr.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(".")
}

fn main() {
    println!("1. A short row at the end of the buffer");
    let mut wire: Vec<u8> = vec![192, 168, 0, 1, 10, 0, 0, 1, 127, 0, 0, 1];
    wire.extend_from_slice(&[8, 8]); // the read stopped mid-address
    let (rows, leftover) = regroup(&wire);
    println!("   {} bytes in", wire.len());
    for row in &rows {
        println!("     {}", dotted(row));
    }
    println!("   leftover: {leftover:?} — {} bytes, not enough for a row",
             leftover.len());
    println!("   Nothing was dropped silently: remainder() is the part the");
    println!("   iterator refused, and it is a slice you can keep and prepend");
    println!("   to the next read.");

    println!();
    println!("2. Why the obvious one-liner does not compile");
    println!("   bytes.chunks_exact(4).collect::<Vec<[u8; 4]>>()");
    println!("   -> error[E0277]: a value of type `Vec<[u8; 4]>` cannot be");
    println!("      built from an iterator over elements of type `&[u8]`");
    println!("   A &[u8] has its length in the value; a [u8; 4] has it in the");
    println!("   type. Converting between them is exactly what try_into does,");
    println!("   and it returns a Result because in general it can fail.");

    println!();
    println!("3. The bill, for a thousand addresses");
    let flat: Vec<u8> = (0..4000u32).map(|b| b as u8).collect();
    let before = ALLOCS.load(Relaxed);
    let packed: Vec<[u8; 4]> = flat.chunks_exact(4).map(|c| c.try_into().unwrap()).collect();
    let mid = ALLOCS.load(Relaxed);
    let nested: Vec<Vec<u8>> = flat.chunks_exact(4).map(|c| c.to_vec()).collect();
    let after = ALLOCS.load(Relaxed);
    println!("   shape          rows   allocations");
    println!("   Vec<[u8; 4]>   {:>4}   {:>4}", packed.len(), mid - before);
    println!("   Vec<Vec<u8>>   {:>4}   {:>4}", nested.len(), after - mid);
    println!("   One, not two: chunks_exact knows its own length, so collect");
    println!("   reserves the whole buffer up front instead of doubling into");
    println!("   it. The nested form pays that once too — the extra thousand");
    println!("   are the rows themselves, each its own block.");

    println!();
    println!("4. Fixed-width rows sort and dedup for free");
    let mut seen: Vec<[u8; 4]> = vec![
        [10, 0, 0, 1],
        [192, 168, 0, 1],
        [10, 0, 0, 1],
        [127, 0, 0, 1],
    ];
    seen.sort();
    seen.dedup();
    let listed: Vec<String> = seen.iter().map(dotted).collect();
    println!("   {}", listed.join("  "));
    println!("   [u8; 4] is Ord and Eq because u8 is, so the derive ladder");
    println!("   reaches the row without anyone writing an impl.");

    println!();
    println!("5. What only Vec<Vec<u8>> could have held");
    let ragged: Vec<Vec<u8>> = vec![vec![10, 0, 0, 1], vec![255, 255]];
    println!("   {:?} — row lengths {:?}", ragged,
             ragged.iter().map(|r| r.len()).collect::<Vec<_>>());
    println!("   Here that is not a feature. An address that is not four bytes");
    println!("   is a bug, and Vec<[u8; 4]> makes it one the compiler catches:");
    println!("   push([10, 0, 0]) is error[E0308], expected an array with a");
    println!("   size of 4, found one with a size of 3.");
}
```
<!-- /source -->

<!-- output:vec_of_arrays_kata -->
*Verified output of [`vec_of_arrays_kata.rs`](examples/vec_of_arrays_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A short row at the end of the buffer
   14 bytes in
     192.168.0.1
     10.0.0.1
     127.0.0.1
   leftover: [8, 8] — 2 bytes, not enough for a row
   Nothing was dropped silently: remainder() is the part the
   iterator refused, and it is a slice you can keep and prepend
   to the next read.

2. Why the obvious one-liner does not compile
   bytes.chunks_exact(4).collect::<Vec<[u8; 4]>>()
   -> error[E0277]: a value of type `Vec<[u8; 4]>` cannot be
      built from an iterator over elements of type `&[u8]`
   A &[u8] has its length in the value; a [u8; 4] has it in the
   type. Converting between them is exactly what try_into does,
   and it returns a Result because in general it can fail.

3. The bill, for a thousand addresses
   shape          rows   allocations
   Vec<[u8; 4]>   1000      1
   Vec<Vec<u8>>   1000   1001
   One, not two: chunks_exact knows its own length, so collect
   reserves the whole buffer up front instead of doubling into
   it. The nested form pays that once too — the extra thousand
   are the rows themselves, each its own block.

4. Fixed-width rows sort and dedup for free
   10.0.0.1  127.0.0.1  192.168.0.1
   [u8; 4] is Ord and Eq because u8 is, so the derive ladder
   reaches the row without anyone writing an impl.

5. What only Vec<Vec<u8>> could have held
   [[10, 0, 0, 1], [255, 255]] — row lengths [4, 2]
   Here that is not a feature. An address that is not four bytes
   is a bug, and Vec<[u8; 4]> makes it one the compiler catches:
   push([10, 0, 0]) is error[E0308], expected an array with a
   size of 4, found one with a size of 3.
```
<!-- /output -->

</details>

---

## See also

- [Grids and nested `Vec`s](../vec_of_vecs/README.md) — the shape to reach for when rows *do* differ in length: what the extra allocations buy, and the `*` that `iter_mut` needs
- [Array or `Vec`?](../array_or_vec/README.md) — the same decision for a single row, and the four things an array buys when the length is a fact about the problem
- [Arrays and slices](../arrays_and_slices/README.md) — why `[u8; 4]` and `[u8; 3]` are different types, which is what makes the push above an error
- [`Vec::into_flattened`](../vec_methods/vec_into_flattened/README.md) — the free conversion out, and why `Vec<Vec<T>>` has no equivalent
- [`Vec`](../the_vec/README.md) — the three numbers the outer `Vec` is, and how the buffer doubles

## Sources

[`Vec::into_flattened` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_flattened) and [`slice::as_flattened` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.as_flattened), both stable since 1.80.0; [`slice::chunks_exact` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact) for the regrouping and its `remainder`; the by-value array loop is [IntoIterator for arrays ↗](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html) in the 2021 edition guide.

## Po polsku

`Vec<[T; N]>` to rosnąca lista wierszy o **stałej** szerokości — i po polsku warto od razu rozdzielić dwa słowa, bo oba tłumaczą się jako „tablica". Wiersz to `[u8; 4]`, czyli tablica o rozmiarze zapisanym **w typie**; pojemnik to `Vec`, czyli wektor, którego długość jest daną. Tysiąc adresów w postaci `vec![[0u8; 4]; 1000]` kosztuje **jedną** alokację, bo wiersze leżą jeden po drugim w tym samym bloku; `vec![vec![0u8; 4]; 1000]` kosztuje **1001**, bo każdy wiersz to osobny blok, a wektor zewnętrzny trzyma same wskaźniki. Te tysiąc dodatkowych alokacji kupuje dokładnie jedną rzecz — wiersze o różnej długości — i jeśli twoje dane takich nie mają, płacisz za elastyczność, z której nic nie korzysta.

Najważniejsza zaleta jest jednak nie wydajnościowa, tylko typowa: **wiersz o złej szerokości nie kompiluje się**. `addrs.push([10, 0, 0])` kończy się błędem `E0308` z komunikatem *expected an array with a size of 4, found one with a size of 3*. `Vec<Vec<u8>>` przyjmie taki wiersz bez słowa, a błąd wyjdzie dużo później — jako panika w innym miejscu albo jako pakiet, którego nikt nie potrafi rozłożyć. To ta sama zasada, którą zna każdy, kto pisał `TYPE x LENGTH 4` w ABAP-ie: szerokość jest częścią typu, więc pilnuje jej kompilator, a nie autor.

Drobiazg, który łatwo przeoczyć przy pętli. `for octet in row`, gdzie `row` ma typ `&[u8; 4]`, daje **referencje** (`&u8`); `for octet in *row` daje **wartości** (`u8`), bo tablica jest `Copy`. Obie wersje się kompilują i obie wypisują to samo, więc `println!` nigdy nie zdradzi, którą masz — różnica wyjdzie gdzie indziej, przy `collect()` albo przy wywołaniu funkcji, w błędzie wskazującym linijkę, o której wcale się nie myślało.

Droga powrotna z płaskiego bufora jest **zawodna i typ o tym mówi**: wycinek `&[u8]` trzyma długość w wartości, a tablica w typie, więc `chunks_exact(4)` trzeba domknąć przez `try_into`. Samo `collect()` nie przejdzie — `E0277`, *cannot be built from an iterator over elements of type `&[u8]`*. Końcówka, która nie podzieliła się przez cztery, nie znika po cichu: oddaje ją `remainder()`.

**Szukaj po polsku:** wektor tablic · tablica o stałym rozmiarze · rozmiar w typie · `rust Vec<[T; N]> allocations` · `rust chunks_exact try_into array` · `rust E0308 expected an array with a size of`
