# `mem::transmute`

**Level:** 301 · deep dive

**One line:** `transmute::<A, B>` reinterprets the bits of an `A` as a `B` and checks one thing at compile time — that the two sizes match — so validity, the alignment of anything a pointer points at, and lifetimes are all yours, which is why nearly every use has a safer replacement.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What rustc checks: size only. `transmute::<u64, u32>` is `E0512`, "cannot transmute between types of different sizes, or dependently-sized types", with both bit widths in the notes. `transmute::<u8, bool>(2)` compiles.
- What [the docs ↗](https://doc.rust-lang.org/std/mem/fn.transmute.html) demand instead: "Both the argument and the result must be valid at their given type"; padding "is not guaranteed to be preserved"; and for pointers, references and boxes "the caller has to ensure proper alignment of the pointed-to values".
- The safer tools, each for one job: `as` for numeric and pointer casts; `f32::from_bits` and `to_bits`; `u32::from_ne_bytes`, `from_le_bytes`, `to_be_bytes`; `char::from_u32`. rustc's `unnecessary_transmutes` lint (warn) now points at them — `transmute::<u32, char>` draws "consider using `char::from_u32(…).unwrap()`".
- [`bytemuck` ↗](https://docs.rs/bytemuck): `cast`, `cast_ref`, `cast_mut`, `cast_slice`, `cast_slice_mut`, gated on the `NoUninit` and `AnyBitPattern` traits (its docs' historical note says the crate started out with a single `Pod` trait). The proof moves from a comment to a trait bound — which conversions from this page can it not express?
- The lints, as `clippy-driver -Whelp` lists them for 1.98. Warn: `transmute_ptr_to_ref`, `transmute_int_to_bool`, `transmute_int_to_non_zero`, `transmute_bytes_to_str`, `useless_transmute`, `crosspointer_transmute`, `transmutes_expressible_as_ptr_casts`, `missing_transmute_annotations`. Deny: `wrong_transmute`, `unsound_collection_transmute`, `transmuting_null`, `transmute_null_to_fn`, `eager_transmute`. Allow: `transmute_ptr_to_ptr`, `transmute_undefined_repr`. And rustc's own `mutable_transmutes` (deny) and `integer_to_ptr_transmutes` (warn). Which of them fires on each example the finished page shows?
- Lifetimes: transmuting `&'a T` into `&'static T` compiles, and hands back the same unbounded lifetime `CStr::from_ptr` does.
- `Vec<A>` into `Vec<B>` with `size_of::<A>() == size_of::<B>()`: why is `unsound_collection_transmute` deny-by-default even then?
- Safe transmutation as a language feature: `std::mem::TransmuteFrom` exists in 1.98 and is nightly-only (`transmutability`).

## The trap it exists for

`transmute` reached for because a C tutorial used a cast, on a conversion `as`, `from_bits` or `from_ne_bytes` already does without `unsafe`. Or two `repr(Rust)` structs with "the same fields" transmuted into each other — the sizes match, the compiler is satisfied, and the field order was never promised.

## Where this sits

- [Casting with `as`](../../29_Conversion/casting_with_as/README.md) covers `as`; [Validity invariants](../validity_invariants/README.md) covers which bit patterns are undefined behaviour to produce; [What a union is](../what_a_union_is/README.md) covers the other way to read bits as another type; [Type layout](../type_layout/README.md) covers why same-fields is not same-layout. This page covers only `transmute` and its replacements.

## See also

- [Casting with `as`](../../29_Conversion/casting_with_as/README.md) — the conversion that is never undefined behaviour, and what it does instead
- [Validity invariants](../validity_invariants/README.md) — the rule "valid at both types" refers to
- [What a union is](../what_a_union_is/README.md) — `f32::to_bits` as the safe answer to the same question
- [Type layout](../type_layout/README.md) — `repr(Rust)`, and what `repr(C)` promises instead
- [What a float actually stores](../../19_Numbers/what_a_float_stores/README.md) — the bits `from_bits` hands back
- [Why a `char` is 32 bits wide](../../14_Strings/why_char_is_32_bits/README.md) — why `transmute` into a `char` is undefined behaviour
- [Clippy: the groups, the settings, the commands](../../34_Templates/clippy/README.md) — how the lints above are enabled
- [Books](../../10_Resources/books/README.md) — the `socket2` 0.3.9 dependency that transmuted a std type into a C struct and stopped compiling

## If you are coming from another language

- **C.** A cast between pointer types, a `union`, or `memcpy` into a variable of another type are C's three spellings, and strict aliasing makes reading through the cast pointer undefined. `memcpy` is the portable one, and it corresponds to `from_ne_bytes`. Rust has no strict-aliasing rule; its equivalent danger is the validity of the result.
- **C++.** `reinterpret_cast` is the unchecked pointer version, and C++20's `std::bit_cast` is the closest match to `transmute`: it requires equal sizes and trivially copyable types, and leaves whether the result is a meaningful value to you.
- **Go.** `unsafe.Pointer` conversions do the same job with the same absence of checks, and `math.Float64bits` / `math.Float64frombits` are the safe path, like `to_bits` and `from_bits`.
- **Python.** Every reinterpretation goes through bytes explicitly — `struct.unpack("<f", data)` or `int.from_bytes` — so the `transmute` shortcut does not exist; `ctypes.cast` is the unsafe exception and can crash the interpreter.

## Po polsku

`mem::transmute` reinterpretuje bity jednej wartości jako inny typ i sprawdza przy kompilacji dokładnie jedno: czy **rozmiary** są równe (inaczej `E0512`). Poprawność wyniku, wyrównanie tego, na co wskazuje wskaźnik, i czasy życia (*lifetimes*) są na twojej głowie. Prawie zawsze istnieje bezpieczniejsze narzędzie — `as`, `f32::from_bits`, `u32::from_ne_bytes`, `char::from_u32` — a rustc sam je podpowiada lintem `unnecessary_transmutes`. Skrzynka `bytemuck` zamienia komentarz z uzasadnieniem na ograniczenie cechy (*trait bound*).

**Szukaj po polsku:** reinterpretacja bitów · rzutowanie typów w Ruscie · `rust transmute alternatives` · `rust bytemuck cast_slice` · `clippy transmute lints`
