# Generic or concrete parameters

**Level:** 301 · deep dive

**One line:** `fn load(r: impl Read)`, `fn load(r: &mut File)` and `fn load(r: &mut dyn Read)` all accept a file today and cost different things — a copy of the function per type, a vtable call, or callers who must convert first — and whichever one you publish is awkward to change later.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three spellings side by side, with what each costs — monomorphised copies, one indirect call, or no flexibility — using [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) for the machine code rather than repeating it
- `impl Trait` in argument position against a named `<R: Read>`: a caller cannot turbofish an `impl Trait` parameter — `total::<Vec<u32>>(…)` is `E0107` on 1.98.0, with the note *`impl Trait` cannot be explicitly specified as a generic argument* — so changing `<R: Read>` to `impl Read` breaks callers who wrote the type out
- Borrowed or owned, for every type rather than only text: `&[T]`, `&str` or `&Path` when the function only reads; `Vec<T>` or `String` when it keeps; `impl Into<…>` when either — generalising [String parameters worth copying](../../14_Strings/string_api_design/README.md); `clippy::ptr_arg` (style) for `&Vec<T>` and `&String`, `clippy::needless_pass_by_value` (pedantic) for an owned parameter that is only read
- C-CALLER-CONTROL, "Caller decides where to copy and place data": why a function that clones its `&T` argument should take `T` and let the caller decide whether to clone
- C-RW-VALUE, readers and writers taken by value as `R: Read` — and why that still lets a caller keep the file, through `impl Read for &mut R` ([Implementing your trait for references](../blanket_impls_for_references/README.md))
- The bill for generics: compile time and binary size per instantiation ([Compile times](../../05_Tooling/compile_times/README.md)), and the inner non-generic function that keeps only a thin conversion generic — which std functions are written that way?
- A generic method rules the trait out of `dyn` use: `E0038`, *method `set` has generic type parameters* — see [Dyn compatibility](../dyn_compatibility/README.md)
- *Rust for Rustaceans* ch. 3 → "Flexible" → "Generic Arguments" and "Borrowed vs. Owned" as the source

## The trap it exists for

Making every parameter generic "for flexibility". Each new caller type compiles another copy of the function, error messages name bounds instead of types, a trait that gains such a method can no longer be used as `dyn`, and the one caller who wanted `f::<File>` has to learn that `impl Trait` refuses it.

## Where this sits

[Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) explains what the compiler generates for `<P: Trait>` and `&dyn Trait`; [String parameters worth copying](../../14_Strings/string_api_design/README.md) settles this question for text with allocation counts; [Returning a trait](../../12_Traits/returning_a_trait/README.md) covers the return position. This page is the parameter-position decision for any type in a public signature.

## See also

- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — monomorphisation against a vtable
- [String parameters worth copying](../../14_Strings/string_api_design/README.md) — the same question for `&str`, `String`, `AsRef` and `Into`
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — `-> impl Trait` and `Box<dyn Trait>`
- [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) — `impl Trait`, `<T: Trait>` and `where`
- [Compile times](../../05_Tooling/compile_times/README.md) — where the extra instantiations are paid for
- [Implementing your trait for references](../blanket_impls_for_references/README.md) — what makes by-value generic parameters friendly
- [Rust API Guidelines — Flexibility ↗](https://rust-lang.github.io/api-guidelines/flexibility.html) — C-INTERMEDIATE, C-CALLER-CONTROL, C-GENERIC, C-OBJECT

## If you are coming from another language

- **Java.** A parameter typed as an interface (`InputStream`, `List<T>`) is always dynamic dispatch, and generics are erased to one compiled method — the `&dyn Read` column only. Rust's `impl Read` compiles a specialised copy per type, which Java cannot.
- **C++.** A template parameter is `impl Trait`, one instantiation per type; a reference to a base class with virtual functions is `&dyn Trait`. C++20 concepts are the named bound.
- **Go.** An interface parameter is dynamic dispatch. Go's generics are implemented with GC-shape stenciling and dictionaries, so they do not promise a full copy per type the way Rust's monomorphisation does.
- **Python.** Duck typing accepts anything with a `read()` method; `typing.Protocol` writes the bound down for a type checker, and nothing changes at run time.

## Po polsku

Parametr funkcji publicznej można zapisać jako `impl Read`, jako konkretny typ (`&mut File`) albo jako `&mut dyn Read` — dziś wszystkie trzy przyjmą plik, ale kosztują co innego: kopię funkcji dla każdego typu (monomorfizacja), wywołanie przez tablicę metod wirtualnych albo konwersję po stronie wywołującego. Do tego pytanie „pożyczony czy posiadany” (*borrowed vs owned*): `&[T]`, gdy funkcja tylko czyta, `Vec<T>`, gdy zatrzymuje dane. Zmiana tej decyzji po publikacji potrafi zepsuć kod użytkowników, np. `impl Trait` w miejscu nazwanego parametru generycznego blokuje zapis z turbofishem.

**Szukaj po polsku:** parametry generyczne a konkretne · monomorfizacja · `rust impl trait vs generic parameter` · `rust borrowed vs owned parameters` · `rust api guidelines flexibility`
