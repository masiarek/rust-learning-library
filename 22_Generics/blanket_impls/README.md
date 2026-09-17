# Blanket impls

**Level:** 301 · deep dive

**One line:** `impl<T: Display> ToString for T` implements a trait for every type that meets a bound — one impl, infinitely many types — and the coherence rules exist so that two such impls can never both apply to the same type.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Reading std's most-used blanket impls: `ToString` for `Display`, `Into` for every `From`, `ToOwned` for every `Clone`, `impl<R: Read + ?Sized> Read for &mut R`
- Writing one: `impl<T: Iterator> MyExt for T` — the [extension trait](../../12_Traits/extension_traits/README.md) pattern
- `E0119` conflicting implementations: why adding a blanket impl blocks every specific impl it overlaps with
- Why a downstream crate may implement `From<Mine> for Theirs` but not `impl<T> Theirs for T` — the orphan rule applied to generics (see [`From` and `Into`](../../29_Conversion/from_and_into/README.md))
- Adding a blanket impl to a published crate as a breaking change — [semver hazards](../../39_API_Design/semver_hazards/README.md)
- Specialization: the unstable feature that would allow overlap, and why it is still unstable

## The trap it exists for

Writing `impl<T: Display> MyTrait for T` and then `impl MyTrait for MyType` with a custom body. If `MyType: Display` — now or in any future version of your code — the two overlap and the second is `E0119`. The blanket impl claimed every displayable type, including yours.

## Where this sits

[Step 6: one blanket impl covers every `Clone` type](../../12_Traits/how_to_learn_to_owned/the_blanket_to_owned/README.md) walks through one blanket impl in detail. [Extension traits](../../12_Traits/extension_traits/README.md) is the most common reason to write one. This page is the rule for all of them.

## See also

- [Step 6: one blanket impl covers every `Clone` type](../../12_Traits/how_to_learn_to_owned/the_blanket_to_owned/README.md) — one blanket impl, read closely
- [Extension traits](../../12_Traits/extension_traits/README.md) — the pattern that needs a blanket impl
- [`From` and `Into`](../../29_Conversion/from_and_into/README.md) — the orphan rule, and the `Into` you get for free
- ["No method named …"](../../12_Traits/no_method_named/README.md) — `E0599` when a blanket impl's bound is not met
- [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) — the bound that decides which types an impl covers
- [Associated types or type parameters?](../../22_Generics/associated_types_or_type_parameters/README.md) — how the trait's shape limits its impls

## If you are coming from another language

- **Go.** Interfaces are satisfied implicitly by method sets, so every type with `String() string` is a `Stringer` without any impl — a blanket impl everywhere, with no way to add behaviour the type does not already have.
- **C++.** A template specialization or a constrained function template reaches every type that meets a concept; overlapping ones are resolved by partial ordering, which Rust refuses rather than ranks.
- **Java.** A default method on an interface adds behaviour to every implementer, but only to classes that opt in by implementing it; Rust's blanket impl reaches types that never mentioned the trait.

## Po polsku

Implementacja hurtowa (*blanket impl*), np. `impl<T: Display> ToString for T`, implementuje cechę dla każdego typu spełniającego ograniczenie. Reguły spójności (*coherence*) pilnują, żeby dwie takie implementacje nigdy nie pasowały do tego samego typu — stąd `E0119`, gdy po implementacji hurtowej dopisujesz konkretną dla typu, który też spełnia jej warunek.

**Szukaj po polsku:** implementacja hurtowa · spójność cech · `rust blanket implementation` · `rust E0119 conflicting implementations`
