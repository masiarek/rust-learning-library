# Associated types or type parameters?

**Level:** 201 · working knowledge

**One line:** `Iterator` has `type Item` and `From` has `<T>` because a type iterates over exactly one kind of item but can be built from many kinds of value — an associated type is an *output* the implementer picks once, a type parameter is an *input* that allows many impls.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The same trait written both ways — `trait Container { type Item; }` and `trait Container<Item>` — and what each allows: one impl per type, or one impl per type *per* `Item`
- Why `Iterator<Item = u32>` needs the `Item =` in a bound, and what `impl Iterator<Item = &str>` is saying
- Inference: with an associated type the compiler finds `Item` from the implementer; with a parameter the caller may have to name it (`E0282`, see [when the compiler cannot infer](../when_the_compiler_cannot_infer/README.md))
- Traits that use both — `Add<Rhs = Self> { type Output; }` — and why `Rhs` is an input and `Output` is not
- Default type parameters, and `Self` as the usual default
- Generic associated types (GATs) as the next step, named only

## The trap it exists for

Making the element type a parameter because generics feel more flexible, then writing `fn sum<C: Container<T>, T>(c: C)` everywhere and meeting "type annotations needed" at every call. If a type can only sensibly have one answer, the associated type puts that answer in the impl and out of every signature.

## Where this sits

[What a generic is](../what_a_generic_is/README.md) covers type parameters on types. [Operators are traits](../../12_Traits/operators_are_traits/README.md) shows `Add<Rhs>` with `type Output` in use. [Implementing `Iterator`](../../24_Iterators/implementing_iterator/README.md) is where most people first write `type Item`.

## See also

- [What a generic is](../../22_Generics/what_a_generic_is/README.md) — type parameters, the input half
- [Operators are traits](../../12_Traits/operators_are_traits/README.md) — `Add<Rhs = Self>` with `type Output`
- [Implementing `Iterator`](../../24_Iterators/implementing_iterator/README.md) — the first `type Item` most people write
- [What a trait is](../../12_Traits/what_a_trait_is/README.md) — associated constants and types in the declaration
- [When the compiler cannot infer](../../22_Generics/when_the_compiler_cannot_infer/README.md) — the error a parameter can cause
- [Blanket impls](../../22_Generics/blanket_impls/README.md) — one impl for every `T`, the other direction

## If you are coming from another language

- **Java.** Java only has type parameters, so `Iterable<T>` is how an element type is expressed; the consequence is the "one impl per type" rule Rust gets from `type Item`, which Java cannot state (a class cannot implement `Iterable<String>` and `Iterable<Integer>` either, but for erasure reasons).
- **C++.** A nested `using value_type = …;` inside a class is an associated type by convention; Rust's version is checked by the trait.
- **Swift.** `associatedtype` is the same feature under almost the same name.

## Po polsku

Typ powiązany (*associated type*, np. `type Item` w `Iterator`) to wyjście wybierane raz przez implementację; parametr typu (np. `<T>` w `From<T>`) to wejście, które pozwala na wiele implementacji dla jednego typu. Jeśli typ może mieć tylko jedną sensowną odpowiedź, typ powiązany usuwa ją ze wszystkich sygnatur.

**Szukaj po polsku:** typ powiązany · parametr typu · `rust associated type vs generic parameter` · `rust iterator item associated type`
