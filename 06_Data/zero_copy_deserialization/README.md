# Zero-copy deserialization

**Level:** 301 · deep dive

**One line:** A deserialized struct can borrow its strings from the input buffer instead of allocating them — `&'a str` fields and `#[serde(borrow)]` — which costs a lifetime on the type and fails outright on a JSON string that contains an escape.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `struct Row<'a> { name: &'a str }` with `serde_json::from_str`: the allocation count with and without borrowing, measured with a counting allocator (see [the global allocator](../../09_Advanced/the_global_allocator/README.md))
- The failure: a string with `\n` or `\"` in it cannot be a slice of the input — the error text serde_json gives, and `Cow<'a, str>` with `#[serde(borrow)]` as the fix that borrows when it can
- What the lifetime does to the caller: the input buffer must outlive every row — the [lifetimes at the call site](../../18_Ownership/lifetimes_at_the_call_site/README.md) story
- Binary formats built for this: `rkyv` (archived types read in place), `zerocopy` and `bytemuck` (casting byte slices to `#[repr(C)]` structs), and what each checks — alignment, validity, endianness
- When it is not worth it: small inputs, data that outlives the buffer, and the complexity a lifetime parameter adds to every signature

## The trap it exists for

Switching every `String` field to `&'a str` and meeting the escape error in production, on the first input with a quote in it. Borrowing is conditional on the bytes, which is exactly what `Cow` models.

## Where this sits

[Deriving `Serialize` and `Deserialize`](../serde_derive/README.md) is the owned, allocate-everything version. [`Cow`](../../18_Ownership/clone_on_write/README.md) is the type that makes borrowing conditional. [String slices](../../14_Strings/string_slices/README.md) is what *zero-copy* means for text.

## See also

- [Deriving `Serialize` and `Deserialize`](../../06_Data/serde_derive/README.md) — the owned version this page optimises
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — borrow when possible, own when not
- [String slices](../../14_Strings/string_slices/README.md) — a view into a buffer somebody else owns
- [Lifetime annotations](../../18_Ownership/lifetime_annotations/README.md) — `<'a>` on a struct, and what it spreads to
- [Type layout](../../09_Advanced/type_layout/README.md) — what `#[repr(C)]` guarantees a byte cast
- [Glossary: zero-copy](../../GLOSSARY.md) — the one-line definition

## If you are coming from another language

- **C.** Casting a received buffer to a struct pointer is C's default, and the C library's [a record on the wire ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_record_on_the_wire/) page shows what padding and byte order do to it. `zerocopy` and `bytemuck` exist to make that cast checked.
- **Python.** `memoryview` and `struct.unpack_from` read in place without copying; the difference is that nothing stops the view outliving a buffer you meant to reuse.
- **Java.** `ByteBuffer` views and flyweight decoders (SBE, FlatBuffers) are the same idea, with the garbage collector keeping the buffer alive instead of a lifetime proving it.

## Po polsku

Deserializacja bez kopiowania (*zero-copy*) oznacza, że pola tekstowe struktury pożyczają fragmenty bufora wejściowego zamiast alokować własne `String`i. Ceną jest parametr czasu życia na typie, a pułapką — napis JSON zawierający sekwencję ucieczki, którego nie da się wyciąć z wejścia; stąd `Cow<'a, str>` z `#[serde(borrow)]`.

**Szukaj po polsku:** deserializacja bez kopiowania · `serde borrow cow str` · `rkyv zero copy` · `rust zerocopy bytemuck`
