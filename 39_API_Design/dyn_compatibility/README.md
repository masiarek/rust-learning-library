# Dyn compatibility

**Level:** 301 · deep dive

**One line:** A trait can be used as `dyn Trait` only if the compiler can build a vtable for it — no generic methods, no `Self` outside the receiver, no associated constants — and `where Self: Sized` is how a trait keeps such a method for concrete types while staying usable as `dyn`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The Reference's rules on 1.98.0, one failing trait each: every supertrait dyn compatible, no `Sized` supertrait, no associated constants, no associated types with generics, and every method either dispatchable — no type parameters, `Self` only in the receiver, a receiver of `&self`, `&mut self`, `Box<Self>`, `Rc<Self>`, `Arc<Self>` or `Pin<P>` of one of those, no `async fn`, no return-position `impl Trait` — or explicitly non-dispatchable with `where Self: Sized`
- The fix the compiler does not suggest: for `fn scaled(&self, factor: f64) -> Self;` the 1.98.0 `E0038` says *…because method `scaled` references the `Self` type in its return type* and helps with *consider moving `scaled` to another trait*, while adding `where Self: Sized` to that one method makes `Vec<Box<dyn Shape>>` compile and keeps `Square(1.0).scaled(3.0)` working
- What `where Self: Sized` costs: the method cannot be called on a `Box<dyn Shape>` — what does that error say, and is it better or worse than splitting the trait?
- Splitting instead: a dyn-compatible core trait plus an extension trait with a blanket impl for the generic methods ([Extension traits](../../12_Traits/extension_traits/README.md)) — when is that worth a second trait name?
- The name: *object safety* became *dyn compatibility* in rust-lang/rust PR #130826, merged September 2024 for 1.83.0. The Reference now notes the concept was formerly known as object safety, while the API Guidelines' C-OBJECT still says "object-safe" — search with both words
- `async fn`: a trait with one is refused as `dyn` — *…because method `fetch` is `async`* on 1.98.0 — unless that method carries `where Self: Sized`, and the `AsyncFn` traits are not dyn compatible either. What do crates that need `dyn` and async methods together do instead?
- The Cargo SemVer reference lists "adding a trait item that makes the trait non-object safe" as a major change — see [SemVer hazards](../semver_hazards/README.md)
- *Rust for Rustaceans* ch. 3 → "Flexible" → "Object Safety" as the source, under its old name

## The trap it exists for

Adding one convenience method — `fn boxed_clone(&self) -> Self`, or `fn map<F: Fn(u32) -> u32>(&self, f: F)` — to a trait your callers hold as `Box<dyn Trait>`. Your crate compiles and its tests pass, because nothing inside it writes `dyn`; the `E0038` appears in theirs, at their `dyn`, after they upgrade.

## Where this sits

[Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) introduces `E0038` with a generic method and explains the rename; [Returning a trait](../../12_Traits/returning_a_trait/README.md) covers `-> impl Trait` against `Box<dyn Trait>`; [Reading the `ToOwned` docs](../../12_Traits/reading_the_to_owned_docs/README.md) reads a *not dyn compatible* badge. This page is the trait author's side: the whole rule list, and how to keep a method without losing `dyn`.

## See also

- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — what a vtable is, and the first `E0038`
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — where `Box<dyn Trait>` usually comes from
- [Reading the `ToOwned` docs](../../12_Traits/reading_the_to_owned_docs/README.md) — the dyn-compatibility line on a std docs page
- [Extension traits](../../12_Traits/extension_traits/README.md) — the second trait that can hold the generic methods
- [Supertraits](../../12_Traits/supertraits/README.md) — what `trait Shape: Display` requires, and so why a supertrait has to be dyn compatible too
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Sized`, the bound `where Self: Sized` names
- [The Reference: dyn compatibility ↗](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) — the rule list, with dispatchable and non-dispatchable examples
- [rust-lang/rust#130826 ↗](https://github.com/rust-lang/rust/pull/130826) — "Compiler: Rename object safe to dyn compatible"

## If you are coming from another language

- **C++.** A member function template cannot be `virtual` — the same rule as "no generic methods in a `dyn` trait", for the same reason: a vtable has one slot per function, and a template is not one function.
- **Java.** Any interface can be used as a type, generic methods included, because generics are erased and every object is already behind a reference. None of these restrictions has a Java counterpart, because none of Rust's per-type code generation does either.

## Po polsku

Cecha nadaje się do użycia jako `dyn Trait` (*dyn compatible*, dawniej *object safe*) tylko wtedy, gdy kompilator może zbudować dla niej tablicę metod wirtualnych: bez metod generycznych, bez `Self` poza odbiorcą, bez stałych skojarzonych. Metodę łamiącą te zasady można zachować, dopisując `where Self: Sized` — wtedy działa dla typów konkretnych i znika z `dyn`. Pułapka autora biblioteki: dodanie jednej wygodnej metody nie psuje jego kompilacji, tylko kod użytkowników, którzy trzymają `Box<dyn Trait>`.

**Szukaj po polsku:** obiekt cechy · zgodność z dyn · `rust dyn compatibility` · `rust object safety where Self: Sized` · `rust E0038`
