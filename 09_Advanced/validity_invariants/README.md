# Validity invariants

**Level:** 301 · deep dive

**One line:** Some values are undefined behaviour merely to produce — a `bool` holding 2, a `char` past `U+10FFFF`, a null `&T`, an uninitialised `u32` — whether or not anything reads them afterwards, and that list is the compiler's, where a safety invariant belongs to a type's module.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The Reference's list under [invalid values ↗](https://doc.rust-lang.org/reference/behavior-considered-undefined.html): a `bool` is 0 or 1; a `char` is not a surrogate and not above `char::MAX`; no value of `!` exists; integers, floats and raw pointers are initialised; an enum has a valid discriminant; a reference or `Box` is aligned, non-null and not dangling; a `fn` pointer is non-null; a `str` is treated like `[u8]`, so it must only be initialised.
- "Producing" means assigned, read, passed or returned. Is `let b: bool = unsafe { transmute(2u8) };` undefined behaviour before `b` is used? Miri says so at the `transmute`: "constructing invalid value of type bool: encountered 0x02, but expected a boolean".
- Why valid UTF-8 is not on that list: it is the `str` type's *safety* invariant, the kind [What an invariant is](../what_an_invariant_is/README.md) covers, broken only when code relies on it. That page contrasts one `char` and one `str`; this page is the whole list.
- The reason the list exists: niches. `Option<bool>` is 1 byte, `Option<char>` 4 and `Option<&u8>` 8 because the compiler spends the invalid bit patterns on `None` — so an invalid `bool` can read back as a `None`. What does a program that does this print, with and without optimisation?
- [`MaybeUninit<T>` ↗](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html), the type allowed to hold anything: `write`, then `assume_init`, and the rule that `assume_init` is the moment the validity invariant is claimed. `mem::zeroed::<&u8>()` draws rustc's `invalid_value` warning — "the type `&u8` does not permit zero-initialization … references must be non-null" — with a help line pointing at `MaybeUninit`.
- Does a `&T` have to point at a valid `T`? The Reference says that part "remains a subject of some debate" — what does Miri do with it today?
- Padding: the bytes between a `repr(C)` struct's fields are uninitialised, so reading a struct as `[u8; N]` produces uninitialised integers. Which std APIs refuse that, and which crates make it a trait bound?

## The trap it exists for

Believing undefined behaviour needs a use. `let n: u32 = unsafe { MaybeUninit::uninit().assume_init() };` followed by never reading `n` looks harmless and is not: the invalid value was produced at the `assume_init`, and the compiler was entitled to assume it could not be.

## Where this sits

- [What an invariant is](../what_an_invariant_is/README.md) covers safety invariants and the `char`/`str` pair; [What a union is](../what_a_union_is/README.md) covers why the field type decides whether a union read is UB; [`mem::transmute`](../transmute/README.md) covers the tool that produces invalid values most often. This page covers only the list of validity requirements and `MaybeUninit`.

## See also

- [What an invariant is](../what_an_invariant_is/README.md) — the other kind of invariant, owned by a module
- [What a union is](../what_a_union_is/README.md) — "the field type decides, not the union"
- [`mem::transmute`](../transmute/README.md) — the easiest way to produce any value on the list
- [Miri](../miri/README.md) — the tool that reports an invalid value where it is produced
- [Uninitialized reads](../../31_C_and_Cpp/uninitialized_reads/README.md) — the C bug, and `E0381` in safe Rust
- [Meet the `bool`](../../15_First_Programs/meet_the_bool/README.md) · [Why a `char` is 32 bits wide](../../14_Strings/why_char_is_32_bits/README.md) — two types whose spare bit patterns this page is about
- [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) — the niche that makes a null `&T` a `None`
- [GLOSSARY.md](../../GLOSSARY.md) — *niche*, *undefined behaviour* and *invariant*

## If you are coming from another language

- **C.** C's hazard is *reading* an indeterminate value, and [Uninitialized reads](../../31_C_and_Cpp/uninitialized_reads/README.md) shows what `clang` did with one. Rust's rule fires earlier — at *producing* an invalid value — and reaches types C is relaxed about: a C `enum` variable may hold any value of its underlying integer type, where a Rust `enum` holding an undeclared discriminant is undefined behaviour on the spot.
- **C++.** `std::bit_cast` (C++20) checks the size and trivial copyability and leaves the value to you, like `transmute`. C++ has no `MaybeUninit`; raw storage plus placement `new` plays its part, with the same rule that the object does not exist until it is constructed.
- **Python.** No counterpart: every object the interpreter hands you is a valid object of its type, which is exactly the property safe Rust gives you and `unsafe` Rust makes your job. The nearest hazard is `ctypes.cast` onto the wrong structure, and it can crash the interpreter for the same reason.

## Po polsku

**Niezmiennik poprawności** (*validity invariant*) dotyczy wartości, których samo **wytworzenie** jest niezdefiniowanym zachowaniem (*undefined behaviour*): `bool` równy 2, `char` powyżej `U+10FFFF`, pusta referencja `&T`, niezainicjalizowana liczba. Takiej wartości nie trzeba nigdzie czytać — wystarczy przypisać, przekazać albo zwrócić. To lista kompilatora, w odróżnieniu od **niezmiennika bezpieczeństwa** (*safety invariant*), takiego jak poprawne UTF-8 w `str`, którego pilnuje moduł typu. `MaybeUninit<T>` to typ, który może trzymać cokolwiek, a `assume_init` to chwila, w której bierzesz na siebie obietnicę poprawności.

**Szukaj po polsku:** niezdefiniowane zachowanie · niezmiennik poprawności a niezmiennik bezpieczeństwa · `rust validity invariant vs safety invariant` · `rust MaybeUninit assume_init` · `rust invalid value undefined behavior`
