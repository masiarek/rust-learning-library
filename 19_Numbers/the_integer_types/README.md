# The integer types

**Level:** 101 → 201 · newcomer

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Rust has twelve integer types — signed and unsigned at 8, 16, 32, 64 and 128 bits, plus `isize` and `usize`, which are as wide as a pointer — and an unannotated integer literal is an `i32` unless something else in the program asks for a different type.

## What it has to cover

- The table: each type's width and its `MIN` and `MAX`, printed by a program rather than typed
- **`usize` is the type of indexes and lengths**, because it is as wide as an address; why `v[i]` refuses an `i32` index and what the fix is
- The default: why `let x = 5;` is an `i32`, and how a later use can change the type inference picks
- Choosing a type: the size of the value's range first, then `usize` where the value indexes something, then `i64` or `u64` for counts that might grow
- Overflow in one line each — a panic in a debug build, wrap-around in a release build — and the four method families (`checked_`, `wrapping_`, `saturating_`, `overflowing_`) that make the choice explicit; the full story is on [Meet the byte](../meet_the_byte/README.md)
- Mixing types: `u8 + u32` does not compile, and the conversions that make it compile belong to [Conversion](../../29_Conversion/README.md)

## The trap it exists for

Choosing `u32` for a length "because it cannot be negative", then subtracting two of them. `a - b` with `b > a` panics in a debug build and wraps to a number near four billion in release, where the test that would have caught it did not run.

## If you are coming from another language

- **C and C++:** the widths are fixed rather than platform-defined — `i32` is 32 bits everywhere — and `usize` is `size_t`. Signed overflow is not undefined behaviour here: it is a panic or a wrap, depending on the build, as [Signed overflow](../../31_C_and_Cpp/signed_overflow/README.md) shows.
- **Python:** `int` has no width and never overflows; every Rust integer does, so the width is a decision you make.
- **Java:** the same signed widths plus unsigned ones Java does not have, and overflow that is checked in debug builds instead of always wrapping.

## See also

- [Meet the byte](../meet_the_byte/README.md) — `u8`, and the overflow that differs by build
- [Writing a number down](../writing_a_number_down/README.md) — literal suffixes such as `5u64`
- [Tables 2.1 and 2.2, run](../../10_Resources/rust_in_action/scalar_number_types/README.md) — ten of the types printed with their ranges, and the targets where `usize` is not the CPU's width
- [Casting with `as`](../../29_Conversion/casting_with_as/README.md) — converting between these types without a check
- [The Reference: numeric types ↗](https://doc.rust-lang.org/reference/types/numeric.html)

## Po polsku

Rust ma dwanaście typów całkowitych: ze znakiem (`i8` … `i128`) i bez znaku (`u8` … `u128`) oraz `isize` i `usize`, których szerokość równa się szerokości wskaźnika — dlatego `usize` jest typem indeksów i długości. Liczba bez adnotacji to domyślnie `i32`. **Przepełnienie** (*overflow*) nie jest tu zachowaniem niezdefiniowanym jak w C: w kompilacji debug kończy się paniką, w release zawinięciem wartości.

**Szukaj po polsku:** typy całkowite w Ruscie · przepełnienie liczb całkowitych · `rust integer types usize` · `rust integer overflow debug release`
