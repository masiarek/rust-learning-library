# Type layout

**Level:** 301 · deep dive

**One line:** The default `repr(Rust)` promises almost nothing about where fields sit — the compiler may reorder them, so `struct { a: u8, b: u32, c: u8 }` is 8 bytes where `#[repr(C)]` makes it 12 — and `repr(C)`, `repr(packed)`, `repr(align)` and the niche optimisation are how you trade that freedom for a layout you can rely on.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `repr(Rust)`: the [Reference ↗](https://doc.rust-lang.org/reference/type-layout.html) says "the only data layout guarantees made by this representation are those required for soundness" — each field's offset is divisible by its alignment, the type's alignment is at least its largest field's, and fields do not overlap — and "the ordering does not have to be the same as the order in which the fields are specified". The finished page prints `size_of` and each field's offset with `offset_of!` (stable since 1.77.0) for the three-field struct under each representation: 8 bytes, 12 with `repr(C)`, 6 with `repr(C, packed)`.
- May two instantiations of one generic struct, `Pair<u8, u32>` and `Pair<u32, u8>`, get different field orders? What else is `repr(Rust)` free to change between compiler versions?
- `repr(C)`: declaration order and C's padding rules, and why that makes a layout a contract — for FFI, for writing bytes to a file, for a `union`.
- `repr(packed)`: no padding, alignment 1, and the cost — `&packed.b` is `E0793`, "reference to field of packed struct is unaligned … creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)". Clippy's `repr_packed_without_abi` (warn) asks for `repr(C, packed)` or `repr(Rust, packed)` so the intent is explicit.
- `repr(align(N))`: raising alignment — `#[repr(align(64))]` on a `u64` wrapper gives size 64 and array elements 64 bytes apart, the tool [False sharing](../false_sharing/README.md) needs.
- Enums: `repr(u8)` and friends fix the discriminant's type; `repr(C)` on a fieldful enum gives the tag-plus-union layout [What a union is](../what_a_union_is/README.md) describes; `repr(transparent)` makes a one-field wrapper ABI-identical to the field. rustc's deny-by-default `conflicting_repr_hints` refuses incompatible combinations — which ones?
- Niches, measured on this machine: `Option<&u8>` 8 bytes, `Option<*const u8>` 16, `Option<bool>` 1, `Option<char>` 4, `Option<NonZeroU32>` 4, `Option<u32>` 8, `Option<extern "C" fn()>` 8. Where does `None` go in each, and which of these does std *guarantee* rather than merely do?
- Padding bytes are uninitialised — so writing a struct's memory to a file sends bytes nobody set, which is the C library's [A record on the wire ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_record_on_the_wire/) in Rust.

## The trap it exists for

Assuming Rust keeps fields in declaration order, as C does, and then writing a `repr(Rust)` struct's bytes to disk, passing it to C, or transmuting it into a "matching" struct. Every size matches, nothing warns, and the layout the code depends on was never promised by any version of the compiler.

## Where this sits

- [What a union is](../what_a_union_is/README.md) covers overlapping fields; [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) covers the niche for `Option<Box<T>>` and the raw-pointer exception; [What is a record, in memory?](../../16_Structs/representing_a_record/README.md) covers choosing a shape for data; [Calling C](../calling_c/README.md) introduces `#[repr(C)]` in one section. This page covers only the layout rules. [STRUCTS.md](../../STRUCTS.md) lists layout, padding and `repr` as missing; this is that page's outline.

## See also

- [What a union is](../what_a_union_is/README.md) — `repr(C)` unions, and the enum as a tagged union
- [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) — the niche, and why `Option<*const T>` is 16 bytes
- [What is a record, in memory?](../../16_Structs/representing_a_record/README.md) — six shapes for one row, sized
- [Calling C](../calling_c/README.md) — "a struct needs `#[repr(C)]`, or it is not a layout"
- [FFI-safe types](../../31_C_and_Cpp/migrating_c_to_rust/ffi_safe_types/README.md) — layout as a contract with another compiler
- [False sharing](../false_sharing/README.md) — `repr(align)` used for speed
- [`mem::transmute`](../transmute/README.md) — the operation that most often assumes a layout
- [STRUCTS.md](../../STRUCTS.md) · [GLOSSARY.md](../../GLOSSARY.md) — the structs map, and *niche*

## If you are coming from another language

- **C.** Declaration order is guaranteed, padding is inserted to satisfy alignment, `offsetof` reports it, and `__attribute__((packed))` or `#pragma pack` removes it — `repr(C)` is exactly that model, and `repr(Rust)` is what you get without asking. [A record on the wire ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_record_on_the_wire/) prints the offsets and the padding a struct sends when its memory is written straight to a file.
- **Python.** `struct.pack` with the default `@` prefix uses native sizes and alignment, padding included, while `<`, `>` and `=` use standard sizes with no padding; `ctypes.Structure` follows the C compiler, with `_pack_` to change it. The encodings library's [Packing a record ↗](https://masiarek.github.io/encodings-learning-library/07_Real_Data/packing_a_record/) lays the four languages' answers side by side.
- **Go.** The gc compiler keeps declaration order, and `unsafe.Sizeof`, `Alignof` and `Offsetof` are `size_of`, `align_of` and `offset_of!`. Reordering fields to save padding is left to you — the `fieldalignment` analyzer suggests it — where `repr(Rust)` does it for you and takes away the order in exchange.
- **ABAP.** A structure in a Unicode system carries alignment gaps before fields that need them, and `ASSIGN … CASTING` onto a structure is checked against the fragment views of source and target, so character and numeric regions have to line up. That check is the ABAP runtime refusing the mistake this page's trap describes; Rust's `transmute` checks the total size and nothing else.

## Po polsku

Domyślna reprezentacja `repr(Rust)` nie obiecuje kolejności pól: kompilator może je przestawić, żeby zmniejszyć **wypełnienie** (*padding*), więc `struct { a: u8, b: u32, c: u8 }` zajmuje 8 bajtów, a z `#[repr(C)]` — 12. `repr(C)` daje kolejność deklaracji i reguły wyrównania (*alignment*) z C, `repr(packed)` usuwa wypełnienie za cenę zakazu referencji do pól (`E0793`), a `repr(align(N))` podnosi wyrównanie. **Nisza** (*niche*) to niedozwolony wzorzec bitów, który kompilator wydaje na `None` — dlatego `Option<&u8>` ma 8 bajtów, a `Option<*const u8>` już 16.

**Szukaj po polsku:** układ struktury w pamięci · wyrównanie i wypełnienie · optymalizacja niszy · `rust repr C vs repr Rust field order` · `rust repr packed E0793` · `rust niche optimization Option size`
