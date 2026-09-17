# A reference is aligned to its referent, not to `usize`

[Pointers](../README.md) › **Alignment**

**Level:** 201 → 301 · working knowledge

**One line:** Every `&T` points at an address that is a multiple of `align_of::<T>()` — 1 for a `u8`, 4 for a `u32` — so a `&u8` may sit at an odd address; *Rust in Action*'s "aligned to multiples of `usize`" describes the reference's own slot, not where it points.

```rust
use std::mem::align_of;

fn main() {
    println!("{}", align_of::<u8>());    // 1
    println!("{}", align_of::<u32>());   // 4
    println!("{}", align_of::<usize>()); // 8
    let bytes = [0u8; 2];
    let gap = &bytes[1] as *const u8 as usize - &bytes[0] as *const u8 as usize;
    println!("{gap}");                   // 1: two valid &u8, one byte apart
}
```

## Alignment belongs to the type

`align_of::<T>()` is the number every address of a `T` must be a multiple of. For the integers on this machine it equals the size — `u16` 2, `u32` 4, `u64` 8 — and for an array it is the element's: `[u8; 10]` is 1, `[u32; 3]` is 4. A reference itself is a `usize`-sized value, so `align_of::<&u8>()` is 8; the `u8` it points at needs only 1.

std defines a reference as a pointer "assumed to be aligned" ([`reference` ↗](https://doc.rust-lang.org/std/primitive.reference.html)), and *aligned* there means to `T`'s alignment. Section 3 of the run checks it: every `&u16`, `&u32` and `&u64` made in safe code reports `is_aligned()`.

## The book's own output says otherwise

*Rust in Action* (ch. 6, §6.1) lists as a benefit of references that they are aligned to multiples of `usize`. Its listing 6.1 prints the addresses of two statics, a 10-byte array and an 11-byte array right after it, and the run printed in the book ends in `…480` and `…48a` — the second is not a multiple of 8. Listing 6.2's printed run shows the same arrays at `…c830` and `…c83a`. A reader's run of listing 6.2 on x86-64 macOS put the first array at `0x101b9a6e1`, an odd address. All of those are valid references, because a `[u8; N]` has alignment 1.

## Padding is how a struct keeps its fields aligned

| Layout | size | `tag` at | `count` at | `flag` at |
|---|---|---|---|---|
| `#[repr(C)]` | 12 | 0 | 4 | 8 |
| default (`repr(Rust)`), rustc 1.98.0 | 8 | 4 | 0 | 5 |

Both structs hold a `u8`, a `u32` and a `u8`. With `#[repr(C)]` the fields keep the written order, so 3 bytes of padding go before `count` to put it on a multiple of 4, and 3 more after `flag` so the whole struct's size is a multiple of its alignment. Rust's default layout is free to reorder fields, and rustc 1.98.0 put `count` first and needed 2 bytes of padding. `offset_of!` reads those numbers from the compiler; section 4 of the run prints them.

The book says Rust's types include padding bytes so that references to them do not slow a program down. The padding is real; the reason is stronger than speed. A `&u32` to a field that is not on a multiple of 4 would be an invalid reference, and invalid is undefined behaviour whatever the CPU does with it.

## `repr(packed)` takes the padding away, and rustc refuses the reference

```rust
#[repr(C, packed)]
struct Packed {
    tag: u8,
    count: u32,
}

fn main() {
    let packed = Packed { tag: 1, count: 42 };
    let count: &u32 = &packed.count;
    println!("{} {count}", packed.tag);
}
```

```text title="Real rustc 1.98.0 output — packed_field_reference.rs, --edition 2024"
error[E0793]: reference to field of packed struct is unaligned
 --> packed_field_reference.rs:9:23
  |
9 |     let count: &u32 = &packed.count;
  |                       ^^^^^^^^^^^^^
  |
  = note: this struct is 1-byte aligned, but the type of this field may require higher alignment
  = note: creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
  = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
```

The second note is the rule in one line: the misaligned reference is undefined behaviour **even if it is never dereferenced**. The help names the two ways out, and section 5 of the run uses both — copy the field out with `{ boxed.0.count }`, or take `&raw const` and call [`read_unaligned` ↗](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned). A raw pointer is allowed to be misaligned; a load through `*p` still is not.

## What happens if you build one anyway

Building a misaligned reference needs `unsafe`, and three tools see it differently:

```rust
fn main() {
    let words = [0u32; 2];
    let one_byte_in = unsafe { words.as_ptr().cast::<u8>().add(1) }.cast::<u32>();
    let r: &u32 = unsafe { &*one_byte_in };
    println!("{r}");
}
```

```text title="Real run — x86-64 macOS, rustc 1.98.0, plain rustc (debug assertions on)"
thread 'main' (28941137) panicked at unaligned_reference.rs:4:28:
misaligned pointer dereference: address must be a multiple of 0x4 but is 0x7ff7b132a921
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread caused non-unwinding panic. aborting.
```

```text title="Abridged — cargo +nightly miri run, nightly 2026-08-27"
error: Undefined Behavior: constructing invalid value of type &u32: encountered an unaligned reference (required 4 byte alignment but found 1)
 --> src/bin/unaligned_reference.rs:4:28
  |
4 |     let r: &u32 = unsafe { &*one_byte_in };
  |                            ^^^^^^^^^^^^^ Undefined Behavior occurred here
```

| Build | macOS x86-64 | Linux x86-64 (Docker `rust:1.98-slim`) |
|---|---|---|
| `rustc` (debug assertions on) | the panic above, exit 134 | the same panic, exit 134 |
| `rustc -O` | printed `0`, exit 0 | printed `0`, exit 0 |
| Miri | undefined behaviour, reported | — |

The debug check is a real safety net: since it panics without unwinding, the process aborts rather than read the wrong bytes. `-O` removes it with the other debug assertions, and the same program then prints `0` on both machines: undefined behaviour that looks like success.

The book says CPUs become temperamental with unaligned reads and run more slowly. That is a statement about hardware, and x86-64 is the forgiving kind — this machine read the misaligned `u32` without complaint. Rust's rule does not depend on the CPU: the compiler is allowed to assume every `&T` is aligned, so a program that breaks the assumption has no defined behaviour on any target.

## The verified output

<!-- output:aligned_to_the_referent -->
*Verified output of [`aligned_to_the_referent.rs`](examples/aligned_to_the_referent.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Each type has its own alignment
   align_of::<u8>()         = 1
   align_of::<[u8; 10]>()   = 1
   align_of::<u16>()        = 2
   align_of::<u32>()        = 4
   align_of::<[u32; 3]>()   = 4
   align_of::<u64>()        = 8
   align_of::<usize>()      = 8
   align_of::<&u8>()        = 8
   A &u8 is itself 8-aligned; the u8 it points at needs only 1.

2. So a &u8 may sit at any address
   &bytes[1] - &bytes[0] = 1
   at least one of the two is not a multiple of 8: true
   and both are perfectly good references.

3. Every reference meets ITS type's alignment
   (&small as *const u16).is_aligned() true
   (&mid   as *const u32).is_aligned() true
   (&big   as *const u64).is_aligned() true

4. Padding is how a struct keeps each field aligned
   #[repr(C)]  HeaderC  size 12  tag @0 count @4 flag @8
   repr(Rust)  Header   size  8  tag @4 count @0 flag @5
   repr(C) keeps the written order: 3 bytes of padding before count, 3
   after flag. Rust's default layout may reorder, and on rustc 1.98.0 it
   put count first and needs only 2 bytes of padding.

5. #[repr(packed)] removes the padding, and with it the alignment
   Packed  size 5  align 1  count @1
   &raw const ...count  is_aligned() false
   place.read_unaligned()  = 42
   { boxed.0.count } copied out = 42
   A &u32 there would break the promise every &u32 makes, so rustc
   refuses to create one; the raw pointer and read_unaligned do not
   promise alignment, so they are allowed.
```
<!-- /output -->

Miri reports no undefined behaviour in this program.

## If you are coming from another language

- **C.** The same rules, unchecked. `_Alignof(T)` is `align_of`, `offsetof` is `offset_of!`, and `__attribute__((packed))` is `repr(packed)` — taking `&p.count` of a packed field is a `-Waddress-of-packed-member` warning in GCC 14.4 and Apple clang 21, on by default, where rustc refuses with `E0793`. A C struct is always laid out in written order, which is what `#[repr(C)]` asks Rust for; Rust's default does not promise it.
- **C++.** `alignof`, `alignas` and `std::align_val_t` for over-aligned `new` cover the same ground, and the compilers disagree on the packed case: GCC 14.4 refuses to bind a `uint32_t&` to a packed field ("cannot bind packed field"), while Apple clang 21 compiles the same line under `-Wall` with no diagnostic at all.
- **Python.** The `struct` module shows both layouts: `struct.calcsize("@BIB")` is 9 — native alignment puts 3 bytes before the `I` but adds nothing at the end — and `struct.calcsize("<BIB")` is 6, no padding anywhere (Python 3.14). Which one a file format expects is the question `#[repr(C)]` versus `#[repr(packed)]` answers.
- **Go.** `unsafe.Alignof` and `unsafe.Offsetof`, with the same written-order layout as C; Go has no packed structs.

## See also

- [Address, pointer, reference](../address_pointer_reference/README.md) — the other promises a `&T` makes
- [Raw pointers](../raw_pointers/README.md) — the pointer type that may be misaligned, and what it costs you
- [`Allocator::shrink`](../../09_Advanced/allocator_shrink/README.md) — alignment as part of a `Layout`
- [What a union is](../../09_Advanced/what_a_union_is/README.md) — another layout question `repr(C)` answers
- [Padding is not alignment ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/padding_is_not_alignment/index.html) — the Python learning library's page with the same two words about text columns, not memory, in case a search brought you here for that
- [A record on the wire ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_record_on_the_wire/index.html) — the C learning library's page where padding meets a file format
- [*Rust in Action*, chapter 6, run](../rust_in_action_chapter_6/README.md) — the alignment claim among the others

## Sources

std's [`align_of` ↗](https://doc.rust-lang.org/std/mem/fn.align_of.html), [`offset_of!` ↗](https://doc.rust-lang.org/std/mem/macro.offset_of.html) and [`read_unaligned` ↗](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned); the Reference's [type layout ↗](https://doc.rust-lang.org/reference/type-layout.html) and [behavior considered undefined ↗](https://doc.rust-lang.org/reference/behavior-considered-undefined.html); `rustc --explain E0793`. The printed addresses from *Rust in Action* are from its listings 6.1 and 6.2 (Manning 2021). The Linux column was run in Docker `rust:1.98-slim`.

## Po polsku

**Wyrównanie** (*alignment*) należy do typu: `align_of::<u8>()` to 1, `align_of::<u32>()` to 4. Referencja `&T` obiecuje, że wskazuje na adres będący wielokrotnością wyrównania **typu `T`**, a nie `usize`. Dlatego `&u8` może leżeć pod adresem nieparzystym i jest w pełni poprawna — co widać zresztą w wydrukach samej książki *Rust in Action*, gdzie tablica bajtów zaczyna się pod `…48a`.

**Dopełnienie** (*padding*) istnieje po to, żeby każde pole struktury leżało pod adresem zgodnym ze swoim wyrównaniem. `#[repr(C)]` trzyma kolejność pól z kodu (12 bajtów dla `u8, u32, u8`), domyślny układ Rusta może pola przestawić (8 bajtów na rustc 1.98.0). `#[repr(packed)]` usuwa dopełnienie, a wtedy rustc odmawia utworzenia `&u32` do pola (`E0793`): niewyrównana referencja to niezdefiniowane zachowanie, **nawet jeśli nikt jej nie odczyta**. Wyjścia są dwa: skopiować pole albo użyć surowego wskaźnika i `read_unaligned`.

Książka mówi, że niewyrównany odczyt jest wolniejszy. To opis sprzętu — x86-64 czyta taki `u32` bez problemu. Reguła Rusta od procesora nie zależy: kompilator zakłada wyrównanie, więc jego złamanie jest błędem na każdej platformie. W kompilacji z asercjami (domyślny `rustc`) program kończy się paniką „misaligned pointer dereference”; z `-O` drukuje `0`, jakby wszystko było w porządku.

**Szukaj po polsku:** wyrównanie pamięci · dopełnienie struktury · `rust repr packed E0793` · `rust align_of offset_of` · `misaligned pointer dereference`
