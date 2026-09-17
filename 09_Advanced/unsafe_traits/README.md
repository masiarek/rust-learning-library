# `unsafe trait` and `unsafe impl`

**Level:** 301 · deep dive

**One line:** An `unsafe fn` puts a proof obligation on every caller; an `unsafe trait` puts it on every implementor — so the `unsafe` in `unsafe impl Send for MyType {}` is you signing for a property that generic code elsewhere will rely on without checking.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three declarations worth reading first: [`pub unsafe auto trait Send` ↗](https://doc.rust-lang.org/std/marker/trait.Send.html), `pub unsafe auto trait Sync` and [`pub unsafe trait GlobalAlloc` ↗](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html). Where does the keyword go — on the trait, on each `impl`, and on none of the places the trait is used as a bound?
- The compiler enforces the pairing both ways: implementing an `unsafe trait` without `unsafe impl` is `E0200`, and writing `unsafe impl` for a safe trait such as `Clone` is `E0199`. It enforces nothing about whether the promise is true.
- Who carries the proof, compared: `unsafe fn` — each caller, at each call; `unsafe trait` — each implementor, once; an `unsafe fn` declared inside a safe trait — callers again. Which one fits a contract like "this type may be zeroed" or "this allocator returns memory valid for the layout"?
- Why `Ord` is safe even though `BTreeMap` depends on it: the [Nomicon ↗](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html) says `BTreeMap`'s unsafe code "must be written to be robust against `Ord` implementations which aren't actually total". What decides whether a trait's promise is `unsafe` to make or merely wrong to break?
- Writing one honestly: `unsafe impl Send for Wrapper` around a raw pointer, and the `// SAFETY:` comment that has to argue it. Clippy's allow-by-default `non_send_fields_in_send_ty` flags an `unsafe impl Send` on a struct holding an `Rc` ("some fields in `Holder` are not safe to be sent to another thread") — what does it not catch?
- Documenting one: a public `unsafe trait` without a `# Safety` section trips `clippy::missing_safety_doc`, warn by default. What must that section list for implementors rather than callers?
- `#[unsafe(no_mangle)]` and the other edition-2024 unsafe attributes: the same idea — a promise the compiler cannot check, written where it is made — applied to an attribute instead of a trait.

## The trap it exists for

`unsafe impl Send for Handle {}` added to silence the error at `thread::spawn`, with no argument written. The compiler accepts it without a warning — `unsafe impl Sync` on a struct holding a `Cell` compiles cleanly — and the data race it permits now happens in safe code, in a module that never mentions `unsafe`.

## Where this sits

- [What `unsafe` turns off](../what_unsafe_turns_off/README.md) lists implementing an unsafe trait as one of the five powers; [`Send` and `Sync`](../send_and_sync/README.md) covers what those two traits mean; [The global allocator](../the_global_allocator/README.md) implements one for real. This page covers only where the proof obligation sits and who signs for it.

## See also

- [What `unsafe` turns off](../what_unsafe_turns_off/README.md) — the five powers, and `unsafe fn` against `unsafe {}`
- [`Send` and `Sync`](../send_and_sync/README.md) — the two auto traits almost every `unsafe impl` is for
- [The global allocator](../the_global_allocator/README.md) — `unsafe impl GlobalAlloc`, and the contract every collection trusts
- [Marker traits](../../12_Traits/marker_traits/README.md) — traits with no methods, where the promise is the whole trait
- [What an invariant is](../what_an_invariant_is/README.md) — the property an implementor is vouching for
- [The drop check](../the_drop_check/README.md) — `unsafe impl Drop` with `#[may_dangle]`, another signed promise

## If you are coming from another language

- **C++.** No counterpart in the type system: a concept checks that the expressions compile, not that the type keeps a promise, so "this allocator is thread-safe" lives in documentation. Rust's version is the same documentation plus a keyword at each implementation that `grep` can find.
- **C.** A table of function pointers with a comment — "the `alloc` hook must return memory aligned for any type", "the comparator must be consistent" — is an `unsafe trait` without the keyword. The implementor carries the proof in both languages; only Rust marks the place.
- **Python.** `__hash__` and `__eq__` that disagree corrupt dictionary lookups but can never corrupt memory, because the interpreter checks every access. That is the `Ord`/`BTreeMap` category exactly: a promise that is wrong to break and safe to make, which is why Rust leaves `Hash` and `Ord` as safe traits.
- **Java.** `Serializable` and `Cloneable` are marker interfaces whose implementors promise something the compiler does not check — but breaking the promise throws or misbehaves rather than causing undefined behaviour, so they correspond to Rust's safe marker traits, not to `Send`.

## Po polsku

`unsafe fn` przerzuca obowiązek dowodu na każdego, kto funkcję **wywołuje**; `unsafe trait` — na każdego, kto cechę **implementuje**. Dlatego `unsafe impl Send for MojTyp {}` to podpis pod obietnicą, na której kod generyczny w zupełnie innym miejscu polega bez sprawdzania. Kompilator pilnuje tylko formy (`E0200`, gdy brakuje `unsafe impl`, `E0199`, gdy jest zbędne), a nie prawdziwości obietnicy — `unsafe impl Sync` dla struktury z `Cell` przejdzie bez słowa ostrzeżenia. `Ord` pozostaje cechą bezpieczną, bo zepsuta implementacja daje złe wyniki, ale nie niezdefiniowane zachowanie.

**Szukaj po polsku:** niebezpieczna cecha · obowiązek dowodu · `rust unsafe trait vs unsafe fn` · `rust unsafe impl Send raw pointer` · `nomicon safe unsafe meaning`
