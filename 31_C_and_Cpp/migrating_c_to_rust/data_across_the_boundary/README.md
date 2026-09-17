# Data across the boundary

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A call crosses the boundary for free, but data does not — every value needs a layout both sides agree on and an owner both sides agree on, and the rule that prevents most FFI memory bugs is that whoever allocated a value is the one who frees it.

## What it has to cover

- **Primitives:** `c_int`, `c_long` and friends from `core::ffi` instead of guessing widths, and `bool`, `char` and `usize` as the three that surprise people
- **Strings:** a Rust `&str` carries a length and no terminator, a C string the reverse. [`CStr` ↗](https://doc.rust-lang.org/std/ffi/struct.CStr.html) for borrowing a C string, `CString` for lending one, the `c"..."` literal, and the encoding question — C's bytes are not promised to be UTF-8
- **Structs:** `#[repr(C)]` layout, padding, and a struct C can see into against an opaque one it only holds a pointer to
- **Collections:** a slice as a pointer and a length, and why a `Vec` cannot be handed over as it stands
- **Ownership:** `Box::into_raw` to give a value to C and `Box::from_raw` in a matching `_free` function to take it back — and why C's `free` on Rust's allocation is undefined behaviour even when it seems to work
- Borrowed data: a pointer C may keep only for the duration of a call, and how to say that in the header

## The trap it exists for

Freeing across the boundary. `free()` on a pointer that came from a Rust `Box`, or dropping in Rust what C's `malloc` produced, mixes two allocators; on some platforms it runs for months before it corrupts the heap.

## See also

- [Calling C](../../../09_Advanced/calling_c/README.md) — "the call is free, the data is not", with `CString` verified
- [FFI-safe types](../ffi_safe_types/README.md) — making the types themselves carry the rules
- [A record on the wire ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_record_on_the_wire/index.html) — struct padding, measured, in the C learning library
- [Six kinds of string](../../../14_Strings/six_kinds_of_string/README.md) — where `CStr` and `CString` sit among Rust's strings
- [The global allocator](../../../09_Advanced/the_global_allocator/README.md) — whose allocation a `Box` is

## Po polsku

Samo wywołanie przez granicę C–Rust nic nie kosztuje, ale dane już tak: każda wartość potrzebuje układu w pamięci, na który zgadzają się obie strony, i właściciela, na którego też się zgadzają. Zasada, która zapobiega większości błędów pamięci w FFI, brzmi: **kto zaalokował, ten zwalnia**. `&str` w Ruscie ma długość i nie ma terminatora, łańcuch w C — odwrotnie, dlatego istnieją `CStr` i `CString`.

**Szukaj po polsku:** przekazywanie danych przez FFI · łańcuchy znaków C w Ruscie · `rust cstr cstring ffi` · `rust box into_raw from_raw ffi`
