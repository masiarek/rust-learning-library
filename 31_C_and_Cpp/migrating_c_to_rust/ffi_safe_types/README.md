# FFI-safe types

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Raw pointers and bare integers are where FFI bugs live, and Rust has types with the same layout that carry more meaning — [`NonNull` ↗](https://doc.rust-lang.org/std/ptr/struct.NonNull.html), `Option<&T>`, `#[repr(transparent)]` newtypes — so the signature a C caller sees stays the same while the Rust side can no longer mix up a count with an id, or forget that a pointer may be null.

## What it has to cover

- **The null-pointer niche:** `Option<&T>`, `Option<NonNull<T>>` and `Option<extern "C" fn(...)>` are guaranteed to be pointer-sized with `None` as null — so "may be null" goes in the type
- `NonNull<T>` for a pointer that is never null but still raw, and `&T` / `&mut T` where the lifetime can honestly be stated
- **Newtypes:** `#[repr(transparent)]` around an integer or a handle, so a `UserId` and a `u32` count cannot be swapped at a call site while C still sees a `uint32_t`
- **Enums:** a `#[repr(C)]` or `#[repr(u32)]` enum with explicit discriminants — and why turning an integer from C straight into a Rust enum is undefined behaviour when the value has no variant; take the integer and convert with `TryFrom`
- `bool` across FFI, and the values other than 0 and 1 a C caller can send
- Designing for invalid states: types that cannot hold a value the C side has no business sending

## The trap it exists for

Declaring a C `enum` parameter as a Rust enum. C will happily pass `7` where only 0–2 exist, and in Rust that value is not merely wrong, it is undefined behaviour the optimizer may act on.

## See also

- [Data across the boundary](../data_across_the_boundary/README.md) — the layouts these types must match
- [Validating at the boundary](../validating_at_the_boundary/README.md) — where the integer becomes the enum, checked
- [Nullable pointers](../../../17_Option_and_Result/nullable_pointers/README.md) — the niche that makes `Option<&T>` free
- [Newtype score](../../../16_Structs/newtype_score/README.md) — a newtype that enforces a range
- [`TryFrom` and `TryInto`](../../../29_Conversion/tryfrom_and_tryinto/README.md) — the checked conversion
- [Other reprs ↗](https://doc.rust-lang.org/nomicon/other-reprs.html) — the Nomicon on `repr(C)`, `repr(transparent)` and integer reprs

## Po polsku

Surowe wskaźniki i gołe liczby całkowite to miejsce, w którym żyją błędy FFI. Rust ma typy o tym samym układzie w pamięci, ale z większym znaczeniem: `NonNull`, `Option<&T>` (gdzie `None` to gwarantowany wskaźnik zerowy) i nowe typy z `#[repr(transparent)]`. Strona C widzi tę samą sygnaturę, a strona Rusta nie pomyli już licznika z identyfikatorem. Liczby z C nie wolno zamieniać wprost na enum Rusta — wartość spoza wariantów to zachowanie niezdefiniowane.

**Szukaj po polsku:** bezpieczne typy w FFI · `rust nonnull option ffi` · `rust repr transparent newtype ffi` · `rust enum from c int undefined behavior`
