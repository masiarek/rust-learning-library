# Raw pointers: `*const T` and `*mut T`

[Pointers](../README.md) › **Raw pointers**

**Level:** 201 → 301 · working knowledge

**One line:** A raw pointer is an address with a type and no promises — safe code may make, copy, cast and compare one, and only reading, writing or offsetting through it needs `unsafe` — and the borrow checker ignores it without the aliasing rules going away.

```rust
fn main() {
    let x = 5u32;
    let p: *const u32 = &x;          // safe: making a pointer reads nothing
    println!("{}", p.is_null());     // false
    println!("{}", unsafe { *p });   // 5 — reading is the unsafe step
}
```

## What safe code may do with one, and what needs `unsafe`

| Safe | Needs `unsafe` |
|---|---|
| make one: `&x as *const T`, `&raw const x`, `ptr::null()`, `0x1000 as *const T` | read or write: `*p`, `p.read()`, `p.write(v)` |
| copy it — raw pointers are `Copy` | offset within an allocation: `p.add(n)`, `p.offset(n)` |
| cast it: `p.cast::<u8>()`, `p.cast_mut()`, `p as usize` | turn it into a reference: `&*p`, `p.as_ref()` |
| compare it: `==`, `ptr::eq`, `is_null()`, `addr()` | build a slice or `String` from it: `slice::from_raw_parts`, `String::from_raw_parts` |
| offset it with no promise: `p.wrapping_add(n)` | |

The dividing line is whether memory is touched or a promise is made. `add` is `unsafe` and `wrapping_add` is not because `add` promises the result stays inside the same allocation and the optimizer may rely on that; `wrapping_add` promises nothing. [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) lists the five powers the keyword grants — dereferencing a raw pointer is the first — and shows `E0133`, the error you get without the block.

## Casting away `const` is allowed; writing through it may not be

A shared reference cannot be cast straight to `*mut`:

```text title="Real rustc 1.98.0 output — shared_to_mut_cast.rs, --edition 2024"
error[E0606]: casting `&[u8; 10]` as `*mut u8` is invalid
 --> shared_to_mut_cast.rs:4:17
  |
4 |     let first = &NAME as *mut u8;
  |                 ^^^^^^^^^^^^^^^^
```

*Rust in Action*'s listing 6.3 goes through `*const` first, `&B as *const u8 as *mut u8`, and that compiles — a pointer cast only relabels. What the cast cannot do is make the memory writable. Write through that pointer where rustc can see where it came from, and a deny-by-default lint refuses the program:

```rust
static NAME: [u8; 10] = *b"carrytowel";

fn main() {
    let first = &NAME as *const u8 as *mut u8;
    unsafe { *first = b'C' };
    println!("{}", NAME[0]);
}
```

```text title="Real rustc 1.98.0 output — write_through_a_shared_reference.rs, --edition 2024"
error: assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
 --> write_through_a_shared_reference.rs:5:14
  |
4 |     let first = &NAME as *const u8 as *mut u8;
  |                 ----------------------------- casting happened here
5 |     unsafe { *first = b'C' };
  |              ^^^^^^^^^^^^^
  |
  = note: for more information, visit <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html>
  = note: `#[deny(invalid_reference_casting)]` on by default
```

The lint only sees a write in the same function. The listing's `String::from_raw_parts` hides the write inside `String`'s `Drop`, and nothing warns — [the chapter 6 page](../rust_in_action_chapter_6/README.md#listing-63-a-string-that-frees-a-static) runs it.

## No borrow checker, and the rules still apply

Two `&mut` to one place is `E0499`. Two `*mut` compile, and using them one after the other is sound — section 3 of the run writes through both. What changes is who checks. A raw pointer taken from a `&mut` is invalidated when a **new** `&mut` to the same place is made, and writing through the old pointer after that is undefined behaviour:

```rust
fn main() {
    let mut count = 0u32;
    let p = &mut count as *mut u32;
    let r = &mut count;
    *r += 1;
    unsafe { *p += 1 };
    println!("{count}");
}
```

rustc accepts it, and both a plain build and `-O` print `2` on macOS and on Linux. Miri does not accept it:

```text title="Abridged — cargo +nightly miri run, nightly 2026-08-27, raw_after_a_new_mut.rs"
error: Undefined Behavior: attempting a read access using <587> at alloc219[0x0], but that tag does not exist in the borrow stack for this location
 --> src/bin/raw_after_a_new_mut.rs:6:14
  |
6 |     unsafe { *p += 1 };
  |              ^^^^^^^ this error occurs as part of an access at alloc219[0x0..0x4]
  |
  = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
help: <587> was created by a SharedReadWrite retag at offsets [0x0..0x4]
 --> src/bin/raw_after_a_new_mut.rs:3:13
  |
3 |     let p = &mut count as *mut u32;
  |             ^^^^^^^^^^
help: <587> was later invalidated at offsets [0x0..0x4] by a Unique retag
 --> src/bin/raw_after_a_new_mut.rs:4:13
  |
4 |     let r = &mut count;
  |             ^^^^^^^^^^
```

Stacked Borrows is Miri's model of those rules, and Miri says the model is still experimental. The practical reading holds either way: once you hold raw pointers, do not make new references to the same place until you are done with them.

## Dangling, null, and address 1

Raw pointers can outlive what they point at, and rustc 1.98.0 now warns in the plainest case — a function returning a pointer to its own local, which *Rust in Action*'s `memscan-3` listing does on purpose:

```rust
fn local_address() -> *const i32 {
    let local = 12345;
    &local as *const i32
}

fn main() {
    let p = local_address();
    println!("{}", p.is_null());
}
```

```text title="Real rustc 1.98.0 output — pointer_to_a_local.rs, --edition 2024"
warning: function returns a dangling pointer to dropped local variable `local`
 --> pointer_to_a_local.rs:3:5
  |
1 | fn local_address() -> *const i32 {
  |                       ---------- return type is `*const i32`
2 |     let local = 12345;
  |         ----- local variable `local` is dropped at the end of the function
3 |     &local as *const i32
  |     ------^^^^^^^^^^^^^^
  |     |
  |     dangling pointer created here
  |
  = note: a dangling pointer is safe, but dereferencing one is undefined behavior
  = note: for more information, see <https://doc.rust-lang.org/reference/destructors.html>
  = note: `#[warn(dangling_pointers_from_locals)]` on by default
```

Change the last line to `println!("{}", unsafe { *p });` and both builds print `12345` on both machines — the stack slot has not been reused yet — while Miri reports `alloc219 has been freed, so this pointer is dangling`. Undefined behaviour that prints the right answer is the dangerous kind.

The book's `memscan-1` and `memscan-2` listings read memory from address 0 and from address 1. The same loop, run on rustc 1.98.0:

```rust
fn main() {
    let mut nonzero = 0;
    for addr in 0..10_000usize {
        let p = addr as *const u8;
        if unsafe { *p } != 0 {
            nonzero += 1;
        }
    }
    println!("non-zero bytes: {nonzero}");
}
```

| Start at | `rustc` (debug assertions on) | `rustc -O` |
|---|---|---|
| 0, macOS | `null pointer dereference occurred`, a non-unwinding panic, exit 134 | segmentation fault, exit 139 |
| 0, Linux | the same panic, exit 134 | segmentation fault, exit 139 |
| 1, macOS and Linux | segmentation fault, exit 139 | segmentation fault, exit 139 |

Debug builds insert a null check before a raw-pointer read, so address 0 panics with a message. Address 1 is not null, is aligned for a `u8`, and is not mapped: the check passes and the operating system stops the process. Miri reports both as undefined behaviour — `got null pointer`, and `0x1[noalloc] which is a dangling pointer (it has no provenance)`.

## `NonNull<T>`: one promise back

[`NonNull<T>` ↗](https://doc.rust-lang.org/std/ptr/struct.NonNull.html) is a `*mut T` that is never null, and that one promise buys the niche: `Option<NonNull<u32>>` is 8 bytes where `Option<*mut u32>` is 16. `Box`, `Rc`, `Vec` and `String` are built on it, which is why `Option<Box<T>>` is free ([Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md)) and why a smart pointer is not a raw pointer ([Smart pointers](../../41_Smart_Pointers/README.md)).

## Back to a reference

`unsafe { &*p }` or `unsafe { &mut *p }` turns a raw pointer back into a reference, and it is the step where you make every promise from [Address, pointer, reference](../address_pointer_reference/README.md#a-reference-adds-the-promises): aligned, not null, a valid value, alive for as long as the reference is used, and no other `&mut` in use. Write them down in a `// SAFETY:` comment — every `unsafe` block in the examples below has one.

## The verified output

<!-- output:raw_pointers -->
*Verified output of [`raw_pointers.rs`](examples/raw_pointers.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Safe code may make, copy, cast and compare raw pointers
   null.is_null()             true
   start.is_null()            false
   end.addr() - start.addr()  12 bytes
   start.cast::<u8>() == start as *const u8  true
   start.cast_mut() compiles too: *const -> *mut is only a relabelling
   None of these reads memory, so none of them needs unsafe.

2. Reading, writing and offsetting need unsafe
   unsafe { *start.add(2) }        30
   unsafe { null.as_ref() }        None
   unsafe { start.as_ref() }       Some(10)
   `add` is unsafe and `wrapping_add` is not: `add` promises the result
   stays inside the allocation, and the compiler may rely on that.

3. No borrow checker: two *mut to one place at once
   write through a, write through b: count = 2
   Two &mut u32 to `count` would be E0499. Two raw pointers compile,
   and this use is sound. The rules have not gone away, only the checks:
   a pointer taken from a &mut is invalid once a NEW &mut to the same
   place is made, and writing through it after that is undefined
   behaviour that no compiler flags. Miri does; see the page.

4. Back to a reference: you make the promises
   unsafe { &mut *a }, += 40: count = 42

5. NonNull<T>: a raw pointer with one promise, never null
   NonNull::new(null_mut)          None
   size_of Option<*mut u32>        16
   size_of Option<NonNull<u32>>    8
   unsafe { *nn.as_ptr() }          5
   The one promise buys the niche: Box, Rc and Vec are built on it.
```
<!-- /output -->

Miri reports no undefined behaviour in this program or in the kata's solution.

## If you are coming from another language

- **C.** This is C's pointer, with two differences worth carrying back. Reading is marked (`unsafe`), so a review can find every place a pointer is trusted; and `*const` versus `*mut` is a relabelling, like casting away `const` in C — legal to write, undefined to use for a write to memory that was never writable. The null check a debug build adds has no C counterpart; UBSan's `-fsanitize=null` is the nearest.
- **C++.** Raw `T*` is the same thing, and the modern-C++ advice to keep it non-owning and prefer `std::unique_ptr` is the same advice Rust gives with `Box`. Rust adds the aliasing rule above, which C++ has only in the weaker form of `restrict` extensions and strict aliasing by type.
- **Python.** `ctypes.POINTER` and `ctypes.cast` are raw pointers; `ctypes.string_at(0)` crashes the interpreter the way the address-0 scan crashes here. Nothing else in Python can dangle.
- **Go.** `unsafe.Pointer` is the escape hatch, with rules about conversion to `uintptr` that are Go's version of provenance: a `uintptr` alone does not keep an object alive or reachable.

---

## Practice

**Walk a slice with a raw pointer.** Write `sum_by_pointer(values: &[u32]) -> u32` that starts a `*const u32` at `values.as_ptr()`, stops at one past the end, and adds each element. Give every `unsafe` block a `// SAFETY:` comment that names why the read is in bounds and why the pointer is still valid. Check it against `values.iter().sum()` on three inputs, one of them empty — and say why the empty slice's start pointer is never read.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:raw_pointers_kata -->
*[`raw_pointers_kata.rs`](examples/raw_pointers_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: walk a slice with a raw pointer, and say why each step is sound.
//!
//!   rustc --edition 2024 raw_pointers_kata.rs -o /tmp/rpk && /tmp/rpk

/// Sums a slice by moving a pointer from its first element to one past its last.
fn sum_by_pointer(values: &[u32]) -> u32 {
    let mut cursor: *const u32 = values.as_ptr();
    // Making the one-past-the-end pointer is allowed, and `wrapping_add` is safe.
    let end: *const u32 = values.as_ptr().wrapping_add(values.len());
    let mut total = 0;
    while cursor != end {
        // SAFETY: cursor starts at element 0 and stops before `end`, so it
        // always points at an element of `values`, which is borrowed for the
        // whole call: aligned, initialized, alive, and not written elsewhere.
        total += unsafe { *cursor };
        // SAFETY: cursor < end, so cursor + 1 is at most one past the end,
        // which `add` allows.
        cursor = unsafe { cursor.add(1) };
    }
    total
}

/// The same walk, but the reads go through `get`, which checks the index.
fn sum_by_index(values: &[u32]) -> u32 {
    (0..values.len()).map(|i| values.get(i).copied().unwrap_or(0)).sum()
}

fn main() {
    let cases: [&[u32]; 3] = [&[10, 20, 30], &[], &[7]];
    println!("Walking a slice with a raw pointer:");
    for values in cases {
        println!(
            "  {:<12} by pointer {:>2}   by index {:>2}   iter().sum() {:>2}",
            format!("{values:?}"),
            sum_by_pointer(values),
            sum_by_index(values),
            values.iter().sum::<u32>()
        );
    }

    println!();
    println!("The empty slice is the case to think about:");
    let empty: &[u32] = &[];
    println!("  start == one-past-the-end  {}", empty.as_ptr() == empty.as_ptr().wrapping_add(0));
    println!("  as_ptr() on an empty slice is never null (rustc's useless_ptr_null_checks");
    println!("  lint says so if you test it), but it points at no element. The loop");
    println!("  body never runs, so that start is never read; reading it would be");
    println!("  undefined behaviour.");

    println!();
    println!("What the unsafe version bought: nothing here. std's slice::Iter holds");
    println!("the same pair, a pointer to the next element and one past the end,");
    println!("and std wrote the SAFETY argument once.");
}
```
<!-- /source -->

<!-- output:raw_pointers_kata -->
*Verified output of [`raw_pointers_kata.rs`](examples/raw_pointers_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Walking a slice with a raw pointer:
  [10, 20, 30] by pointer 60   by index 60   iter().sum() 60
  []           by pointer  0   by index  0   iter().sum()  0
  [7]          by pointer  7   by index  7   iter().sum()  7

The empty slice is the case to think about:
  start == one-past-the-end  true
  as_ptr() on an empty slice is never null (rustc's useless_ptr_null_checks
  lint says so if you test it), but it points at no element. The loop
  body never runs, so that start is never read; reading it would be
  undefined behaviour.

What the unsafe version bought: nothing here. std's slice::Iter holds
the same pair, a pointer to the next element and one past the end,
and std wrote the SAFETY argument once.
```
<!-- /output -->

</details>

## See also

- [Address, pointer, reference](../address_pointer_reference/README.md) — the promises a reference adds, and provenance
- [A reference is aligned to its referent](../aligned_to_the_referent/README.md) — the one raw pointers may break, and `read_unaligned`
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — the keyword, `split_at_mut`, and why the module is the unit of review
- [Null dereference](../../31_C_and_Cpp/null_dereference/README.md) and [Use-after-free](../../31_C_and_Cpp/use_after_free/README.md) — the C bugs these reads are
- [`Vec::from_raw_parts`](../../26_Collections/vec_methods/vec_from_raw_parts/README.md) and [`String::from_raw_parts`](../../14_Strings/string_methods/string_from_raw_parts/README.md) — building owners out of raw pointers, and their contracts
- [Function pointers](../../23_Closures/function_pointers/README.md) — `fn()`, the pointer type that points at code
- [Six pointer types, one table](../../18_Ownership/references/pointer_types_compared/README.md) — the raw pointers beside `&T`, `&mut T`, `Box` and `Rc`, and why `*const T` and `*mut T` differ in variance
- [Lints around references](../../18_Ownership/references/reference_lints/README.md) — `borrow_as_ptr`, `ref_as_ptr`, `ptr_as_ptr`, and three raw-pointer mistakes no lint catches

## Sources

std's [`pointer` ↗](https://doc.rust-lang.org/std/primitive.pointer.html) and [`std::ptr` ↗](https://doc.rust-lang.org/std/ptr/index.html) docs; the Reference's [raw pointers ↗](https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut); [Stacked Borrows ↗](https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md) in the Unsafe Code Guidelines; *Rust in Action* ch. 6, listings 6.3 and `ch6-memscan-1` to `-3` in the book's [code repository ↗](https://github.com/rust-in-action/code/tree/1st-edition/ch6). Compiler output is rustc 1.98.0; runs are x86-64 macOS 26.6 and Docker `rust:1.98-slim`; Miri is nightly 2026-08-27.

## Po polsku

**Surowy wskaźnik** (`*const T`, `*mut T`) to adres z typem i bez obietnic. Bezpieczny kod może go utworzyć (nawet z liczby), skopiować, rzutować i porównać; `unsafe` wymaga dopiero odczyt, zapis, przesunięcie (`add`) i zamiana z powrotem na referencję. `wrapping_add` jest bezpieczne, bo niczego nie obiecuje; `add` obiecuje, że wynik zostanie w tej samej alokacji.

Kontroler pożyczeń surowych wskaźników nie sprawdza, ale **reguły aliasowania nadal obowiązują**. Wskaźnik wzięty z `&mut` przestaje być ważny, gdy powstanie nowe `&mut` do tego samego miejsca — rustc to przepuści, program wypisze `2`, a Miri zgłosi niezdefiniowane zachowanie. Podobnie wskaźnik do zmiennej lokalnej zwrócony z funkcji: rustc 1.98.0 ostrzega (`dangling_pointers_from_locals`), a odczyt i tak drukuje `12345`, bo slot stosu nie został jeszcze nadpisany.

Rzutowanie `*const` na `*mut` tylko zmienia etykietę — pamięci statycznej nie czyni zapisywalną; rustc odrzuca taki zapis lintem `invalid_reference_casting`. Odczyt spod adresu 0 w kompilacji z asercjami kończy się paniką „null pointer dereference”, z `-O` — naruszeniem ochrony pamięci. `NonNull<T>` przywraca jedną obietnicę (nigdy null) i dzięki niej `Option<NonNull<T>>` ma 8 bajtów.

**Szukaj po polsku:** surowe wskaźniki w Ruście · wiszący wskaźnik · aliasowanie wskaźników · `rust raw pointer unsafe` · `rust stacked borrows miri` · `NonNull`
