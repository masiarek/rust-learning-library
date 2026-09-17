# Validating at the boundary

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Every function you export to C is a trust boundary — the caller can pass a null pointer, an out-of-range value or bytes that are not UTF-8, and the compiler cannot stop it — so each one validates and narrows its inputs first, and only checked, typed values reach safe Rust.

## What it has to cover

- **The shape of an exported function:** a thin `extern "C"` layer that checks and converts, then calls an ordinary safe function that never sees a raw type
- Null pointers: checked once, then turned into references or `NonNull`
- Ranges and enums: an integer narrowed with `TryFrom` into the type that only holds valid values
- **Text:** `CStr::from_ptr` needs a NUL terminator the caller promised, and `to_str` fails on bytes that are not UTF-8 — decide per parameter whether that is an error or a lossy conversion
- Slices from a pointer and a length: what `slice::from_raw_parts` requires, and the length that is really a byte count
- **Panics:** a panic inside `extern "C"` aborts the process — rustc 1.98 prints *"panic in a function that cannot unwind"* and the process exits with `SIGABRT`. `catch_unwind` at the boundary, or `extern "C-unwind"` when unwinding into C is intended
- Where not to validate: inside the safe core, which should be able to assume its types mean what they say

## The trap it exists for

Validating deep inside the logic instead of at the edge. The checks end up scattered, one path misses one, and the business logic gets written defensively against inputs its own types should have ruled out.

## See also

- [FFI-safe types](../ffi_safe_types/README.md) — the types validation produces
- [Errors across FFI](../errors_across_ffi/README.md) — what to return when validation fails
- [What a panic costs](../../../17_Option_and_Result/what_a_panic_costs/README.md) — unwinding, and why it cannot cross into C
- [A length you did not check ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_length_you_did_not_check/index.html) — the same trust question for bytes off a socket, in the C learning library
- [FFI and unwinding ↗](https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding) — the Nomicon

## Po polsku

Każda funkcja eksportowana do C to **granica zaufania**: wywołujący może przekazać wskaźnik zerowy, wartość spoza zakresu albo bajty, które nie są poprawnym UTF-8, a kompilator Rusta tego nie zatrzyma. Dlatego cienka warstwa `extern "C"` sprawdza i zawęża dane wejściowe, a dopiero sprawdzone, typowane wartości trafiają do bezpiecznego Rusta. Panika wewnątrz funkcji `extern "C"` kończy proces przez `abort`.

**Szukaj po polsku:** walidacja danych na granicy FFI · granica zaufania · `rust ffi input validation` · `rust panic extern c abort catch_unwind`
