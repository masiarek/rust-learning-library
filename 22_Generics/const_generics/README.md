# Const generics

**Level:** 301 · deep dive

**One line:** `struct Grid<const N: usize>` takes a *value* as a generic parameter, so `[T; N]` is one generic type rather than a type per length — which is why std's array impls stopped being written once per length.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Writing `fn sum<const N: usize>(xs: [u32; N]) -> u32` and calling it with arrays of three and of five elements
- Which types a const parameter may have on stable (integers, `bool`, `char`) — verify on 1.98.0 — and passing a value: `{ 2 + 1 }`, a `const`, inference from an argument
- What stable Rust still refuses: an expression over a parameter such as `[T; N + 1]` (the unstable `generic_const_exprs`), and the error text it gives
- The array trait history: impls once written by a macro for lengths 0–32, now written once with const generics — and which impls, if any, still stop at 32 (check `Default` on the pinned toolchain)
- `std::array::from_fn` and `<[T; N]>::map` as the const-generic APIs most people meet first
- When a const parameter is the wrong tool: a length that comes from input belongs in a `Vec`, as [array or `Vec`?](../../26_Collections/array_or_vec/README.md) argues

## The trap it exists for

Making a buffer size a const generic to "let the user choose" and then needing the size at run time. `N` must be known at compile time at every use, so a size read from a config file cannot become `Grid<N>`.

## Where this sits

[Arrays and slices](../../26_Collections/arrays_and_slices/README.md) shows why `[T; N]` is a type per length. [What a generic is](../what_a_generic_is/README.md) covers type parameters. This page adds values as parameters.

## See also

- [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) — the length that is part of the type
- [Array or `Vec`?](../../26_Collections/array_or_vec/README.md) — when the length should not be a type
- [A `Vec` of arrays](../../26_Collections/vec_of_arrays/README.md) — a fixed-width row inside a growable list
- [What a generic is](../../22_Generics/what_a_generic_is/README.md) — type parameters, before value parameters
- [Phantom types](../../12_Traits/phantom_types/README.md) — the other way to put information into a type

## If you are coming from another language

- **C++.** `template<std::size_t N>` and `std::array<T, N>` are the model, and C++ allows arithmetic on `N` in the signature that stable Rust does not.
- **C.** A C array parameter decays to a pointer and loses `N`; a variable-length array keeps it at run time. Rust's const generic keeps it at compile time and checks it.
- **Java.** No counterpart: an array's length is a run-time property of the object, never part of its type.

## Po polsku

Generyki stałych (*const generics*) pozwalają przekazać jako parametr generyczny wartość, np. `const N: usize`, więc `[T; N]` to jeden typ generyczny, a nie osobny typ dla każdej długości. `N` musi być znane w czasie kompilacji; rozmiar wczytany z pliku konfiguracyjnego do `Grid<N>` się nie zmieści.

**Szukaj po polsku:** generyki stałych · tablice o stałym rozmiarze · `rust const generics array length` · `rust generic_const_exprs`
