# What array explanations get wrong, run

[Arrays: the map](../README.md) › **Beside the lessons** · read after: [An array in a `const` or a `static`](../static_arrays/README.md)

**Level:** 201 · a companion to the lessons

**One line:** Claims about arrays from Rust by Example, The Book, *Rust in Action*, *Programming Rust*, std's own page, a Rustlings hint, a 2015 Stack Overflow answer and chat explanations — each checked on rustc 1.98.0. Most hold only with a condition the source left out, and a few are out of date.

The claims are quoted short, as they were written. Twelve are checked by [the program below](#twelve-claims-run). The rest point to the lesson, error or lint entry where the evidence is.

## The claims

| # | The claim | Where it appears | On 1.98.0 |
|---|---|---|---|
| 1 | "Arrays are stack allocated." | Rust by Example; The Book ("when you want your data allocated on the stack"); Comprehensive Rust; chat answers ("a stack-allocated array") | **Only for a local.** An array lives where its owner does: in a `Box`, a `Vec` or a `static` it is not on the stack. [Output 1](#twelve-claims-run), [Where an array lives](../where_an_array_lives/README.md) |
| 2 | "Out of bound indexing on array causes compile time error." / "Rust does bounds-checking at compile-time and run-time." | Rust by Example; notes | **Only for a constant index**, through the deny-by-default lint `unconditional_panic`, which `#![allow]` switches off. Any other index panics at run time. [Output 2](#twelve-claims-run), [error 10](../array_errors/README.md#10-a-constant-index-past-the-end) |
| 3 | `panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:21:43` | transcripts from older compilers | **Old format.** Since Rust 1.73 the location comes first and the message is on its own line, and 1.98.0 also prints the thread's id. [Output 3](#twelve-claims-run) |
| 4 | "Arrays are limited to 32 elements." | older docs and answers | **Only `Default` still stops at 32.** `Copy`, `Clone`, `Debug`, `PartialEq`, `Hash` and `IntoIterator` work at any length since const generics. Tuple impls stop at 12 elements. [Output 4](#twelve-claims-run), [error 19](../array_errors/README.md#19-default-for-33-elements) |
| 5 | "type signature is superfluous", `size_of_val(&xs)` is 20, `assert_eq!(&empty_array, &[])` | Rust by Example | **True**, all three. [Output 5](#twelve-claims-run) |
| 6 | "Out of bound indexing on slice causes runtime error", shown with `.get(i)` over `0..xs.len() + 1` | Rust by Example | **True of `slice[i]`**, but the example calls `.get`, which returns `None` instead. The `+ 1` is deliberate, and clippy's pedantic [`range_plus_one`](../array_lints/README.md#range_plus_one) would respell it without removing it. [Output 5](#twelve-claims-run) |
| 7 | `assert_eq!([2, 3, 4], nice_slice)` compiles because of coercion | a Rustlings hint | **It is an impl**, `impl PartialEq<&[U]> for [T; N]`, called directly in [output 6](#twelve-claims-run) |
| 8 | in `[expr; N]`, `expr` must be "a value of a type implementing the `Copy` trait" or "a const value" | std's `array` page | **For two or more elements.** `[value; 1]` takes any value, and `[value; 0]` evaluates it and drops it. [Output 7](#twelve-claims-run) |
| 9 | "`[true; 10000]` is an array of 10,000 bool elements", beside code that says `[true; 1000]` | notes | **Each is right about its own number**: 10,000 and 1,000 bytes. Believe the code. [Output 8](#twelve-claims-run) |
| 10 | "`Vec::new()` will not allocate until elements are pushed" | an explanation of `Vec<[i32; 3]>` | **True**: capacity 0, then 4 after one push. [Output 9](#twelve-claims-run), [A `Vec` of arrays](../../vec_of_arrays/README.md) |
| 11 | "iterators are lazy and do nothing unless consumed" | rustc's own note, under [`unused_must_use`](../array_lints/README.md#unused_must_use) | **True**: the filter closure ran 0 times until `collect`. [Output 10](#twelve-claims-run) |
| 12 | filter an array with `\|&&x\|` | a 2015 Stack Overflow answer, last edited before Rust 1.53 | **Right for `.iter()`, wrong for `.into_iter()`** since edition 2021: [`E0308`](../array_errors/README.md#23-x-from-an-answer-written-before-2021). `\|&x\| *x == 2` is [`E0614`](../array_errors/README.md#22-x-in-a-filter-over-into_iter). [Output 11](#twelve-claims-run) |
| 13 | `array.into_iter()` yields references | older books and answers | **Editions 2015 and 2018 only**, and only in method-call syntax. [`array_into_iter`](../array_lints/README.md#array_into_iter) |
| 14 | `(1..6).into_iter().filter(…)` | a filtering example | **Compiles, but `into_iter()` does nothing**: a range is already an iterator. [`useless_conversion`](../array_lints/README.md#useless_conversion) |
| 15 | "The useful methods … are all provided as methods on slices, not arrays" | *Programming Rust*, 2nd ed. | **Mostly true**, but arrays have since gained `map`, `each_ref` and `as_slice`, plus `array::from_fn` and `array::repeat`. [Output 12](#twelve-claims-run) |
| 16 | slices give "fast, read-only access" | *Rust in Action* | **`&mut [T]` writes.** [Arrays and slices](../../arrays_and_slices/README.md#the-mutable-half-and-a-sentence-to-disbelieve) |
| 17 | "the `'static` lifetime means that the data in the `BaseAtom` instance can live for the entire duration of the program" | a chat explanation | **The `static` keyword does that.** `'static` in `BaseAtom<'static>` only says the borrows inside last that long. [An array in a `const` or a `static`](../static_arrays/README.md#what-static-says-in-unitstatic) |
| 18 | `names` is "an array with a single element, the string "meter"" | the same explanation | **A `&'static str`, not a `String`.** [The same page](../static_arrays/README.md#what-static-says-in-unitstatic) |
| 19 | a lifetime cannot be left out of a `static`'s type | a common assumption | **False.** `static X: [BaseAtom; 1]` compiles and means `BaseAtom<'static>`. The struct definition is where it is required. [Where the lifetime has to be written](../static_arrays/README.md#where-the-lifetime-has-to-be-written) |
| 20 | "because the size is known at compile time, a fixed-size array can be stored anywhere, even as a const" | notes | **True when every element can be computed at compile time.** That, not size, is why a `Vec` cannot be a `const`. [The `const` section](../static_arrays/README.md#a-const-array-and-a-local-that-changes-its-contents) |
| 21 | "large fixed-size arrays on the stack could cause a stack overflow" | notes | **True**, and `Box::new([0u8; N])` can too, in a debug build. [Big arrays and the stack](../where_an_array_lives/README.md#big-arrays-and-the-stack) |
| 22 | `Box<[i32]>` from `Box::new([1, 2, 3])` is "a heap-allocated array, coerced to a slice", good for dynamic size, large data, ownership and interoperability | a chat explanation | **The name is right** (8 bytes become 16). **Each of the four reasons needs a condition the explanation left out.** [What a boxed array is for](../where_an_array_lives/README.md#what-a-boxed-array-is-for) |
| 23 | a trailing `//` keeps rustfmt from putting a matrix on one line | a note in a kata solution | **True** with rustfmt 1.9.0. [The transpose kata](../arrays_in_signatures/README.md#practice) |
| 24 | "Rust has two primitive compound types: tuples and arrays" | The Book, ch. 3.2 | **True**, and `Vec` is not one of the two: it is a type from the standard library. [Writing an array down](../writing_an_array_down/README.md#three-spellings) |

## Twelve claims, run

<!-- output:array_claims_checked -->
*Verified output of [`array_claims_checked.rs`](examples/array_claims_checked.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. "Arrays are stack allocated."
   a local [u64; 1000] is 8000 bytes of stack frame
   Box<[u64; 1000]> holds it on the heap; the Box is 8 bytes
   Vec<[u64; 4]> holds its rows on the heap: 2 rows, 32 bytes each
   a static [u64; 1000] is in the binary's data, 8000 bytes
   Verdict: only when the owner is a local. It lives where its owner does.

2. "Out-of-bounds indexing on an array is a compile-time error."
   Only with a constant index: `xs[3]` on [i32; 3] trips the deny-by-default
   lint unconditional_panic. With #![allow(unconditional_panic)] it compiles,
   and any index the compiler cannot see is checked at run time:
   xs.get(i) = None; xs[i] would panic
   Verdict: a compile error only for a constant index; otherwise a panic.

3. The panic text, on 1.98.0
   thread 'main' (<id>) panicked at <dir>/array_claims_checked.rs:33:20:
   index out of bounds: the len is 3 but the index is 99
   Location first, message on its own line: the format since Rust 1.73.
   Older transcripts put the message first, in quotes, then the location.

4. "Arrays are limited to 32 elements."
   [u8; 33]: Copy true, PartialEq true, Hash true, IntoIterator sum 231, Debug [7, 7]..
   Default: [u8; 32] works (32 zeros); [u8; 33] is E0277
   tuples: a 12-tuple is Debug (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12); a 13-tuple is E0277
   From<(T, T, T)> for [T; 3]: [1, 2, 3] (tuples of 1 to 12)
   Verdict: true before const generics; on 1.98.0 only Default stops at 32.

5. Rust by Example's comments
   "type signature is superfluous": unannotated is [i32; 5] - true
   size_of_val(&xs) = 20 - true: five i32s, no header
   assert_eq!(&empty_array, &[]) and &[][..]: both pass
   0..xs.len() + 1 with .get(i): [Some(1), Some(2), Some(3), Some(4), Some(5), None]
   The +1 is on purpose: the last get is the out-of-range one, and it is None.
   "Out of bound indexing on slice causes runtime error": for `slice[i]`
   yes; the example shows `.get`, which is the version that does not.

6. assert_eq!([1, 2], &array[1..])
   passes, and `for x in array` prints: 0 1 2 
   PartialEq::<&[i32]>::eq(&[1, 2], &&array[1..]) = true
   It is an impl, `impl PartialEq<&[U]> for [T; N]`, not a coercion.

7. "The value in [value; N] must be Copy or a constant."
   evaluated loud(one)
   [loud("one"); 1] compiled, len 1: no copy to make
   evaluated loud(zero)
   dropped Loud(zero)
   [loud("zero"); 0] compiled, len 0: evaluated, then dropped at once
   Verdict: true for N of 2 or more.
   dropped Loud(one)

8. "[true; 10000] is an array of 10,000 bools" beside code saying 1000
   size_of::<[bool; 1000]>() = 1000, size_of::<[bool; 10000]>() = 10000
   Verdict: each is right about its own number; read the code, not the prose.

9. "Vec::new() does not allocate until elements are pushed."
   capacity before push 0, after one push 4
   Verdict: true.

10. "Iterators are lazy and do nothing unless consumed."
   after building the filter: closure ran 0 times
   after collect: [2, 2], closure ran 4 times
   Verdict: true.

11. A 2015 answer: filter an array with |&&x|
   numbers.iter().filter(|&&x| x == 2) = [2, 2] - still right for iter()
   numbers.into_iter().filter(|&x| x == 2) = [2, 2] - edition 2021 on
   With into_iter() on 2021, |&&x| is E0308 and |&x| *x == 2 is E0614.

12. "The useful methods are on slices, not arrays."
   array.map(|x| x * 10) = [0, 10, 20], array.each_ref() = [0, 1, 2]
   array.as_slice().len() = 3
   Verdict: mostly still true; arrays have gained a few of their own.
```
<!-- /output -->

The panic in claim 3 is printed by a child process: the program runs itself again with an argument, because a panicking `main` ends the run. The OS thread id and the source file's directory change from run to run, so the program replaces them with `<id>` and `<dir>`. Everything else is rustc's text.

## Where this leaves you

- **Read "arrays are on the stack" as "a local array is in its stack frame".** Every source that says it is using locals in its examples.
- **"Compile-time bounds checking" is one lint on constant indexes.** Plan for the run-time panic, or use `.get`.
- **Date any iteration answer against 2021.** Before the 2021 edition, `array.into_iter()` meant `iter()`, and a pattern written for that, `|&&x|`, no longer compiles.
- **An explanation that mixes up the `static` keyword and the `'static` lifetime** will mislead you about everything after it. The item lives forever; the bound only limits borrows.

## See also

- [Arrays: the map](../README.md) — the lessons these claims point into
- [Every array error, and its fix](../array_errors/README.md) and [Lints around arrays](../array_lints/README.md) — the evidence for the rows without an output number
- [Helpful resources for arrays](../array_resources/README.md) — every source quoted here, with what to read with care
- [What `Cow` explanations get wrong, run](../../../12_Traits/how_to_learn_to_owned/cow_claims_checked/README.md) — the same kind of page for `Cow`

## Po polsku

Twierdzenia o tablicach z Rust by Example, The Book, *Rust in Action*, *Programming Rust*, dokumentacji std, podpowiedzi Rustlings, starej odpowiedzi ze Stack Overflow i wyjaśnień z czatu, sprawdzone na rustc 1.98.0. Najważniejsze poprawki: „tablice są na stosie” dotyczy tylko zmiennych lokalnych. „Sprawdzanie zakresu w czasie kompilacji” to jeden lint dla stałych indeksów. „Tablice są ograniczone do 32 elementów” dotyczy dziś tylko `Default`. `array.into_iter()` zwraca referencje wyłącznie w edycjach 2015 i 2018, więc stare `|&&x|` nie kompiluje się z `into_iter()`.

Osobna grupa to wyjaśnienia `'static`: tabela `static` żyje przez cały program dzięki słowu kluczowemu `static`, a `'static` w typie `BaseAtom<'static>` mówi tylko o referencjach w środku. Pole `names` to `&'static str`, nie `String`. Czas życia w typie `static` można pominąć, ale w definicji struktury — nie.

**Szukaj po polsku:** pytania o tablice w Ruscie · array questions · `rust arrays stack allocated` · `rust array 32 elements limit` · `rust into_iter array edition 2021`
