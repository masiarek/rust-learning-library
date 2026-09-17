# Where an array lives: wherever its owner is

[Arrays: the map](../README.md) › **Lesson 3** · previous: [Arrays in and out of functions](../arrays_in_signatures/README.md) · next: [An array in a `const` or a `static`](../static_arrays/README.md)

**Level:** 201 · working knowledge

**One line:** An array has no header and no pointer of its own — its elements *are* the value — so it lives wherever the value lives: in a stack frame as a local, on the heap inside a `Box` or a `Vec`, in the binary's static data as a `static`. "Arrays are stack-allocated" describes only the first of those four.

```rust
use std::mem::size_of;

fn main() {
    println!("{}", size_of::<[i32; 3]>());      // 12: the three i32s
    println!("{}", size_of::<Box<[i32; 3]>>()); // 8: a pointer; the i32s are on the heap
    println!("{}", size_of::<Box<[i32]>>());    // 16: a pointer and a length
    println!("{}", size_of::<Vec<[i32; 3]>>()); // 24: three words; the rows are on the heap
}
```

## No header, so no home of its own

A `[i32; 3]` is twelve bytes, and a struct with one inside is at least that big: `struct Reading { id: u16, samples: [i32; 3] }` is 16. An array is stored *inline*, as part of whatever contains it. So "where does an array live?" has the same answer as "where does its owner live?":

| Owner | Where the elements are | What sits in the owner |
|---|---|---|
| a local `let a = [1, 2, 3];` | the stack frame | the elements themselves |
| `Box<[i32; 3]>` | the heap | one pointer, 8 bytes |
| `Vec<[i32; 3]>` | the heap, rows end to end in one buffer | pointer, length, capacity |
| `static TABLE: [i32; 3]` | the program's static data | — there is one `TABLE`, with one address |
| a field of a struct | wherever the struct is | the elements |

[The program below](#the-verified-output) checks each row by address: the `Box`'s pointer is its first element, `flat[3]` of the `Vec`'s flattened buffer *is* `in_vec[1][0]`, and `&TABLE` is the same address whichever function asks.

The Book, Rust by Example and Comprehensive Rust all say arrays live on the stack. For a local, which is what their examples use, that is right. As a rule about the type it is not, and [the claims page](../array_claims_checked/README.md) collects the quotes.

## `Box<[i32; 3]>` and `Box<[i32]>`

```rust
fn main() {
    let thin: Box<[i32; 3]> = Box::new([1, 2, 3]);
    let fat: Box<[i32]> = Box::new([1, 2, 3]); // unsized coercion
    println!("{} {}", size_of_val(&thin), size_of_val(&fat)); // 8 16
}
```

Both hold the same three `i32`s on the heap. "A heap-allocated array, coerced to a slice" is an accurate name for the second. The `Box<[i32; 3]>` that `Box::new` built was *coerced* to `Box<[i32]>`, and the length moved out of the type into a second word beside the pointer. That is the same move `&[i32; 3]` → `&[i32]` makes, and [the `Box` lesson](../../the_box/README.md) and [`Box<str>`](../../../14_Strings/boxed_str/README.md) meet the fat box from other directions. [Coercion](../../../29_Conversion/coercion/README.md) lists array-to-slice among the handful of conversions the compiler does for you.

`Box::new([..])` needs a length the compiler knows. For a run-time length, go through a `Vec`: `vec![0; n].into_boxed_slice()`, or `iter.collect::<Box<[i32]>>()`. `try_into()` turns a `Box<[i32]>` of the right length back into a `Box<[i32; 4]>`.

## Moving: the bytes, or a pointer

`let moved = plain;` on a `[u8; 64]` copies 64 bytes. `let moved_box = in_box;` on a `Box<[u8; 64]>` copies 8, and the elements do not move: the program checks their address before and after. Both are moves of ownership; the `Box` only makes the move cheap. That is what an explanation means when it lists "ownership: transfer ownership so the data lives long enough" as a reason to box an array. A plain array can be moved and returned too; boxing it changes what the move costs, not whether it is allowed.

## Big arrays and the stack

A thread's stack is a fixed region chosen when the thread starts, and a local array is part of its frame. So "large fixed-size arrays on the stack could cause a stack overflow" is true. The overflow is an abort, not a panic: [Recursion and the stack](../../../18_Ownership/recursion_and_the_stack/README.md#overflow-is-an-abort-not-a-panic) shows that `catch_unwind` cannot catch it. The program asks for a 256 KiB thread and tries four ways to hold a megabyte:

| Code | Debug build |
|---|---|
| `let buf = [7u8; 1_000_000];` | aborted: *thread 'small' has overflowed its stack* |
| `Box::new([7u8; 1_000_000])` | aborted, the same way |
| `vec![7u8; 1_000_000].into_boxed_slice()` | finished |
| the same, then `.try_into::<Box<[u8; 1_000_000]>>()` | finished |

The second row is the surprise. `Box::new` is an ordinary function, so its argument is built in the caller's frame first and then moved to the heap. The optimizer usually builds it in place:

```text title="Real runs — rustc 1.98.0, x86_64 macOS, the same four calls on a 256 KiB thread"
rustc      local: aborted   Box::new: aborted    into_boxed_slice: finished   try_into: finished
rustc -O   local: aborted   Box::new: finished   into_boxed_slice: finished   try_into: finished
```

"Usually" is the point: a debug build and a test run do not optimize. Build a big buffer with `vec!`, which allocates and zeroes on the heap directly, and turn it into a box if you need one. Clippy's pedantic [`large_stack_arrays`](../array_lints/README.md#large_stack_arrays) flags every local array over 16,384 bytes, including the one inside `Box::new(...)`.

The main thread's stack is bigger than 256 KiB: the same two megabyte lines ran fine in `main` on the same machine. Code that works in `main` can still abort when it moves to a thread with a smaller stack.

## What a boxed array is for

A chat explanation of `Box<[i32]>` gave four reasons to use one. Checked on 1.98.0:

- **"Dynamic size: when the size is not known at compile time."** Not with `Box::new([..])`, whose length is a compile-time constant. A run-time length comes from `Vec::into_boxed_slice` or `collect`, and *then* `Box<[T]>` is the right type: an owned buffer that will not grow, one word smaller than a `Vec`.
- **"Large data."** Only if you never build the array on the stack first. See [the table above](#big-arrays-and-the-stack).
- **"Ownership: transfer ownership so the data lives long enough."** A plain array can be moved and returned as well. The box makes the move a pointer copy. See [moving](#moving-the-bytes-or-a-pointer).
- **"Interoperability."** The vaguest of the four. The concrete case is a function that takes `Box<[T]>`, the type `Vec::into_boxed_slice` and `String::into_boxed_str` produce. Across a C boundary the thin `Box<[T; N]>` matters: one pointer, where `Box<[T]>` needs its length passed separately.

## If you are coming from another language

- **C.** A local `int a[1000000];` is four megabytes of stack frame, with the same problem, and `malloc` is the `vec!` route. C has no type for "a heap array of exactly N", which is what `Box<[T; N]>` is.
- **C++.** `std::array<int, 3>` is Rust's `[i32; 3]`, stored inline wherever its owner is, and `std::vector<int>` is `Vec`. `std::make_unique<std::array<int, 1000000>>()` value-initializes the array directly in the new allocation, so C++ does not share the debug-build trap in the second row.
- **Go.** The compiler decides where a value goes by escape analysis. Return `&a` for a local array and `go build -gcflags=-m` reports *moved to heap: a*, with nothing in the source saying so. Rust has no such step: a local array stays in its frame, and returning a reference to it is an error. It gets to the heap only through a `Box`, a `Vec` or another type that allocates.
- **Java and Python.** Every array and every list is on the heap, always, and the variable holds a reference. There is no inline array to overflow a stack with, and no way to avoid the allocation for a small one.

---

## The verified output

<!-- output:where_an_array_lives -->
*Verified output of [`where_an_array_lives.rs`](examples/where_an_array_lives.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. No header: the value is the elements
   size_of::<[i32; 3]>()        = 12
   size_of::<Reading>()         = 16  (a u16 and the three i32s, inline)
   Reading { id: 1, samples: [4, 5, 6] }

2. Four owners, four places
   local:  the array is the stack slot; &local[0] is the slot's address: true
   Box:    8 bytes on the stack, and the box's pointer is the first element: true
   Vec:    the rows sit end to end in the Vec's one buffer: flat[3] is in_vec[1][0]: true
   static: &TABLE is one address, whichever function asks: true

3. Box<[i32; 3]> and Box<[i32]>
   size_of_val(&thin) = 8  the length is in the type
   size_of_val(&fat)  = 16 the length rides beside the pointer
   size_of_val(&*fat) = 12 the three i32s on the heap
   a run-time length: vec![0; n].into_boxed_slice() = [0, 0, 0, 0]
                      (1..=n).collect::<Box<[i32]>>() = [1, 2, 3, 4]
   and back to a length in the type: Box<[i32; 4]> = [1, 2, 3, 4]

4. Moving an array moves its bytes; moving a Box moves a pointer
   size_of_val(&moved_plain) = 64 bytes copied by `let moved_plain = plain;`
   size_of_val(&moved_box)   = 8  bytes copied by `let moved_box = in_box;`
   the elements did not move: true

5. A megabyte on a 256 KiB thread
   let buf = [7u8; 1_000_000];              aborted: thread 'small' has overflowed its stack
   Box::new([7u8; 1_000_000]), debug build  aborted: thread 'small' has overflowed its stack
   vec![7u8; 1_000_000].into_boxed_slice()  finished, printed 7
   ... .try_into::<Box<[u8; 1_000_000]>>()  finished, printed 7
```
<!-- /output -->

## Practice

**Nine sizes, then a megabyte on a small stack.** Before running anything, write down `size_of` for `[u16; 4]`, `[[u16; 4]; 2]`, `(u8, [u16; 4])`, `&[u16; 4]`, `&[u16]`, `Box<[u16; 4]>`, `Box<[u16]>`, `Option<Box<[u16; 4]>>` and `Vec<[u16; 4]>`. Then print them and explain every one you got wrong.

Then write `fn megabyte() -> Box<[u8; 1 << 20]>` that never holds the megabyte in a stack frame, call it on a thread spawned with a 64 KiB stack, set the last byte to 42, and print the sum of all the bytes.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:where_an_array_lives_kata -->
*[`where_an_array_lives_kata.rs`](examples/where_an_array_lives_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: predict nine sizes, then build a megabyte on a 64 KiB stack.
//!
//!   rustc --edition 2024 where_an_array_lives_kata.rs -o /tmp/walk && /tmp/walk

use std::mem::size_of;
use std::rc::Rc;
use std::thread;

const MIB: usize = 1 << 20;

/// Builds a zeroed megabyte without ever holding it in a stack frame:
/// `vec!` allocates and zeroes on the heap, and the conversions only move the
/// pointer (and the length, until `try_into` puts the length back in the type).
fn megabyte() -> Box<[u8; MIB]> {
    vec![0u8; MIB].into_boxed_slice().try_into().expect("exactly MIB long")
}

fn main() {
    println!("1. Nine sizes, predicted before they were printed");
    let rows: [(&str, usize, &str); 9] = [
        ("[u16; 4]", size_of::<[u16; 4]>(), "the four u16s"),
        ("[[u16; 4]; 2]", size_of::<[[u16; 4]; 2]>(), "eight u16s, still no header"),
        ("(u8, [u16; 4])", size_of::<(u8, [u16; 4])>(), "nine bytes of data, padded to u16's alignment"),
        ("&[u16; 4]", size_of::<&[u16; 4]>(), "a pointer; the 4 is in the type"),
        ("&[u16]", size_of::<&[u16]>(), "a pointer and a length"),
        ("Box<[u16; 4]>", size_of::<Box<[u16; 4]>>(), "thin, like the reference"),
        ("Box<[u16]>", size_of::<Box<[u16]>>(), "fat, like the slice reference"),
        ("Option<Box<[u16; 4]>>", size_of::<Option<Box<[u16; 4]>>>(), "None is the null pointer a Box never is"),
        ("Vec<[u16; 4]>", size_of::<Vec<[u16; 4]>>(), "three words; the rows are on the heap"),
    ];
    for (ty, size, why) in rows {
        println!("   {ty:<22} {size:>2}  {why}");
    }
    println!("   and Rc<[u16]> = {}: fat too; the counts live on the heap beside the elements", size_of::<Rc<[u16]>>());

    println!();
    println!("2. A megabyte on a thread with a 64 KiB stack");
    let (len, sum, box_size) = thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut page = megabyte();
            page[MIB - 1] = 42;
            (page.len(), page.iter().map(|&b| u64::from(b)).sum::<u64>(), size_of::<Box<[u8; MIB]>>())
        })
        .expect("spawn")
        .join()
        .expect("the thread did not overflow");
    println!("   len {len}, sum {sum}, and the Box on that small stack is {box_size} bytes");
    println!("   Box::new([0u8; MIB]) in the same closure aborts a debug build:");
    println!("   the array is built in the closure's frame first, then moved in.");
}
```
<!-- /source -->

<!-- output:where_an_array_lives_kata -->
*Verified output of [`where_an_array_lives_kata.rs`](examples/where_an_array_lives_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Nine sizes, predicted before they were printed
   [u16; 4]                8  the four u16s
   [[u16; 4]; 2]          16  eight u16s, still no header
   (u8, [u16; 4])         10  nine bytes of data, padded to u16's alignment
   &[u16; 4]               8  a pointer; the 4 is in the type
   &[u16]                 16  a pointer and a length
   Box<[u16; 4]>           8  thin, like the reference
   Box<[u16]>             16  fat, like the slice reference
   Option<Box<[u16; 4]>>   8  None is the null pointer a Box never is
   Vec<[u16; 4]>          24  three words; the rows are on the heap
   and Rc<[u16]> = 16: fat too; the counts live on the heap beside the elements

2. A megabyte on a thread with a 64 KiB stack
   len 1048576, sum 42, and the Box on that small stack is 8 bytes
   Box::new([0u8; MIB]) in the same closure aborts a debug build:
   the array is built in the closure's frame first, then moved in.
```
<!-- /output -->

</details>

## See also

- [Arrays: the map](../README.md) — every array page, in reading order
- [Stack and heap](../../../18_Ownership/stack_and_heap/README.md) — the two regions, and why only one of them can grow
- [`Box`](../../the_box/README.md) — one value on the heap, and the fat `Box<[T]>`
- [A `Vec` of arrays](../../vec_of_arrays/README.md) — rows of a fixed width in one heap block
- [Recursion and the stack](../../../18_Ownership/recursion_and_the_stack/README.md) — the other way to run out, and why the abort cannot be caught
- [An array in a `const` or a `static`](../static_arrays/README.md) — the fourth home: the binary itself

## Sources

[The Reference — array types ↗](https://doc.rust-lang.org/reference/types/array.html), whose own example puts one array on the stack and one in a `Box`; [`Vec::into_boxed_slice` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice); [`std::thread::Builder::stack_size` ↗](https://doc.rust-lang.org/std/thread/struct.Builder.html#method.stack_size).

## Po polsku

Tablica nie ma nagłówka ani własnego wskaźnika — jej elementy *są* wartością — więc leży tam, gdzie jej właściciel: w ramce stosu jako zmienna lokalna, na stercie w `Box` albo `Vec`, w danych statycznych programu jako `static`. Zdanie „tablice są alokowane na stosie” opisuje tylko pierwszy przypadek. `Box<[i32; 3]>` zajmuje 8 bajtów (sam wskaźnik), a `Box<[i32]>` — tablica na stercie przekształcona w wycinek — 16, bo obok wskaźnika jest długość.

Duża tablica lokalna może przepełnić stos, a przepełnienie stosu to przerwanie programu, nie panika. Na wątku z 256 KiB stosu `let buf = [7u8; 1_000_000];` kończy program. Tak samo `Box::new([7u8; 1_000_000])` w trybie debug, bo argument powstaje najpierw w ramce wywołującego. Bezpieczna droga to `vec![0u8; N].into_boxed_slice()`, a w razie potrzeby jeszcze `try_into()` do `Box<[u8; N]>`.

**Szukaj po polsku:** tablica na stosie czy na stercie · przepełnienie stosu tablica · `rust Box<[T]> vs Box<[T; N]>` · `rust large array stack overflow` · heap-allocated array
