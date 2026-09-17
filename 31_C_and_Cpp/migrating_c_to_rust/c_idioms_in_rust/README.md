# C idioms in Rust

**Level:** 201 → 301 · for C programmers

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Most C idioms have a Rust counterpart that removes a class of mistake — an iterator instead of an index loop, a returned value instead of an out-parameter, a trait instead of a struct of function pointers, a type instead of a bit mask — and the skill is knowing when the boundary still needs the C shape.

## What it has to cover

Each idiom as a C function, its literal translation, and the idiomatic Rust — with the mistake the rewrite removes:

| C idiom | Rust counterpart | Removes |
|---|---|---|
| `for (i = 0; i < n; i++)` over a pointer | an iterator over a slice | the off-by-one and the wrong `n` |
| an out-parameter plus a status return | a returned value, or `Result<T, E>` | reading an output that was never written |
| a struct of function pointers (a vtable) | a trait, generic or `dyn` | a `NULL` slot called at run time |
| an `int` return code | `Result` with an error enum | an unchecked return |
| `#define` flags OR-ed into an `int` | [`bitflags` ↗](https://docs.rs/bitflags/latest/bitflags/), or a struct of `bool`s | a flag from the wrong set |
| a `char *` plus an implicit encoding | `&str` inside, `CStr` at the edge | text that is not what the type says |

- **When to keep the C shape:** at the exported boundary itself, where C callers need the vtable or the out-parameter, while the Rust core behind it uses the idiomatic form
- Callbacks with a `void *user_data` argument, translated into closures and back
- Linked lists and intrusive structures, and why the Rust translation is usually a `Vec`

## The trap it exists for

Translating line by line. The result compiles, is full of `unsafe` and indexes, and keeps every C bug except the ones the borrow checker happens to reject — the migration's cost without its benefit.

## See also

- [Bit flags](../../../19_Numbers/bit_flags/README.md) — flags in one integer, measured
- [When a loop beats a chain](../../../24_Iterators/when_a_loop_beats_a_chain/README.md) — the iterator rewrite is not always the better one
- [Static vs dynamic dispatch](../../../12_Traits/static_vs_dynamic_dispatch/README.md) — the two ways a trait replaces a vtable
- [Errors across FFI](../errors_across_ffi/README.md) — the same return codes, where they must stay
- [Iterator invalidation](../../iterator_invalidation/README.md) — the C loop bug an iterator cannot have

## Po polsku

Większość idiomów C ma w Ruscie odpowiednik, który usuwa całą klasę błędów: iterator zamiast pętli z indeksem, zwracana wartość zamiast **parametru wyjściowego**, trait zamiast struktury wskaźników do funkcji (*vtable*), typ zamiast maski bitowej. Tłumaczenie linijka po linijce daje kod pełen `unsafe` i indeksów, który zachowuje błędy C. Kształt z C warto zostawić tylko na samej granicy eksportowanej do wywołujących w C.

**Szukaj po polsku:** idiomatyczny Rust dla programistów C · `c to rust idioms` · `rust bitflags crate` · `rust vtable vs trait ffi`
