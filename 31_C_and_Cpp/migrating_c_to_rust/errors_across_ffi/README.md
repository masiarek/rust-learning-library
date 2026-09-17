# Errors across FFI

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `Result` and `?` stop at `extern "C"`, so an error has to become something C can read — an error-code enum as the return value and the real result through an out-parameter — without leaking Rust's internal error types, and with an API coarse enough that there are few ways to fail.

## What it has to cover

- **Error codes:** a `#[repr(C)]` enum of failures with `0` for success, generated into the header, and a `From` conversion from the internal error type so `?` still works up to the boundary
- **Out-parameters:** the result written through a pointer only on success, and what the caller may assume about it on failure
- Richer failure information without leaking internals: a "last error message" call, or an error object with its own `_free`
- Return values against out-parameters: when a C API should return the value and signal failure with a sentinel, and why a sentinel is usually the worse choice
- **API design:** coarse operations (`config_load(path)`) rather than per-field setters, so the C caller cannot build a half-valid object — and a small surface, because every exported function is maintained for as long as a C caller exists
- The mapping in reverse: C's `errno`, negative return codes and `NULL` returns turned into `Result` on the Rust side of a binding

## The trap it exists for

Exposing one setter per field because it mirrors the Rust struct. Every combination of calls a C program might make becomes a state the Rust side has to handle, including the ones its constructor was written to prevent.

## See also

- [Validating at the boundary](../validating_at_the_boundary/README.md) — where most of these errors are produced
- [C idioms in Rust](../c_idioms_in_rust/README.md) — return codes and out-parameters from the other direction
- [The question mark operator](../../../17_Option_and_Result/the_question_mark_operator/README.md) and [`From` and `Into`](../../../29_Conversion/from_and_into/README.md) — how `?` converts up to the boundary
- [The `Error` trait](../../../02_Errors/the_error_trait/README.md) — the internal type the codes are mapped from

## Po polsku

`Result` i operator `?` kończą się na `extern "C"`, więc błąd trzeba zamienić na coś, co C potrafi odczytać: **kod błędu** w postaci enuma zwracanego z funkcji, a właściwy wynik przez **parametr wyjściowy** (*out-parameter*). Wewnętrzne typy błędów Rusta nie powinny wyciekać do API. Druga połowa tematu to projekt samego API: grube operacje zamiast osobnego settera dla każdego pola i jak najmniej funkcji, bo każdą trzeba utrzymywać tak długo, jak długo istnieje wywołujący ją kod C.

**Szukaj po polsku:** obsługa błędów w FFI · kody błędów C · `rust ffi error handling error codes` · `rust ffi out parameter`
