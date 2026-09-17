# SemVer hazards

**Level:** 301 · deep dive

**One line:** Some changes break callers without altering a single signature they call — a new public field, a new enum variant, a second `From` impl, a private field that is not `Send` — and `#[non_exhaustive]`, sealed traits and `cargo-semver-checks` exist so that you find them before your callers do.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Struct fields: adding a public field to a struct with no private fields is major (it breaks every struct literal and exhaustive destructuring), and so is adding a private field to a struct whose fields were all public — the Cargo SemVer reference's `struct-add-public-field-when-no-private` and `struct-add-private-field-when-public`
- Enum variants: a new variant breaks every exhaustive `match` (`enum-variant-new`) unless the enum was `#[non_exhaustive]` from the start — and adding `#[non_exhaustive]` later is itself major (`attr-adding-non-exhaustive`)
- `#[non_exhaustive]` does nothing inside the defining crate: a same-crate exhaustive `match` and a struct literal both still compile on 1.98.0, so your own tests can never show the error your callers get
- A new trait impl that breaks inference: with only `impl From<u32> for Meters`, a caller's `Meters::from(5)` compiles; add `impl From<u16> for Meters` and the literal falls back to `i32`, giving *the trait bound `Meters: From<i32>` is not satisfied*. RFC 1105 still classes implementing a non-fundamental trait as a minor change — why?
- Auto traits leak: a private `Rc<Vec<u8>>` field removes `Send` from the public type, and the caller's `thread::spawn` fails with *`Rc<Vec<u8>>` cannot be sent between threads safely*. No signature changed, and nothing in your crate spawns a thread
- Traits: a new method without a default is major (`trait-new-item-no-default`), with a default only "possibly-breaking" (`trait-new-default-item`), and one that makes the trait not dyn compatible is major — [Dyn compatibility](../dyn_compatibility/README.md)
- Sealed traits (C-SEALED): a public trait with a supertrait from a private module, so callers can use it but not implement it, which leaves you free to add methods
- [`cargo-semver-checks` ↗](https://github.com/obi1kenobi/cargo-semver-checks): `cargo semver-checks` lints a release against the published version and cites the Cargo SemVer reference in each failure — which of the hazards above does it report, and which (inference, auto traits) can a tool see at all?

## The trap it exists for

Treating the crate's own test suite as the compatibility check. The tests live inside the crate, where `#[non_exhaustive]` has no effect, private fields are visible, and nothing sends your type to another thread — most of the breaks on this page show up only from outside it.

## Where this sits

[Two versions of one crate in one binary](../../05_Tooling/two_versions_of_one_crate/README.md) covers what Cargo does with the version number you choose; [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) covers what the caret lets a caller receive; [Publishing a crate](../../05_Tooling/publishing_a_crate/README.md) covers the release itself. This page is which source changes break callers, and which of those SemVer counts as major.

## See also

- [Two versions of one crate in one binary](../../05_Tooling/two_versions_of_one_crate/README.md) — what "SemVer compatible" means to Cargo
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — the caret requirement that delivers your minor release to everyone
- [Publishing a crate](../../05_Tooling/publishing_a_crate/README.md) — the version number you spend on each upload
- [When the compiler cannot infer](../../22_Generics/when_the_compiler_cannot_infer/README.md) — the inference a new impl can take away
- [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) — the auto traits a private field decides
- [Traits every type should consider](../traits_every_type_should_consider/README.md) — what to commit to before the first release
- [Cargo SemVer compatibility ↗](https://doc.rust-lang.org/cargo/reference/semver.html) — every change, classed major, minor or possibly-breaking, with an example
- [RFC 1105 — API evolution ↗](https://rust-lang.github.io/rfcs/1105-api-evolution.html#minor-change-implementing-any-non-fundamental-trait) — why a breaking trait impl is still called minor

## If you are coming from another language

- **Java.** Adding a method to an interface broke every implementing class until Java 8's `default` methods; Rust's defaulted trait item is the same fix, still "possibly-breaking" when its name collides with another trait's method. Java 17's `sealed` interfaces are C-SEALED as a keyword.
- **Go.** Adding a method to an exported interface breaks every type outside the package that implemented it, and a major version goes into the import path (`/v2`), so both versions can coexist — the same outcome [Two versions of one crate](../../05_Tooling/two_versions_of_one_crate/README.md) shows Cargo reaching.
- **C.** A field added to a public struct changes its size, so a program compiled against the old header misbehaves at run time with no error — an ABI break. Cargo rebuilds dependencies from source, so in Rust the same change is either harmless to a caller or a compile error, never a silent layout mismatch.

## Po polsku

Niektóre zmiany psują kod użytkowników, choć nie zmieniają żadnej sygnatury, którą ci wywołują: nowe pole publiczne, nowy wariant enuma, druga implementacja `From` psująca inferencję typów, prywatne pole z `Rc`, które odbiera typowi `Send`. Część z nich formalnie wymaga podbicia wersji głównej (*major*), a nowa implementacja cechy uchodzi za zmianę *minor*, choć też potrafi zepsuć kompilację — i większości nie wykryją testy samego crate'a, bo na przykład `#[non_exhaustive]` nie działa wewnątrz crate'a definiującego. Pomagają `#[non_exhaustive]` od pierwszego wydania, cechy zapieczętowane (*sealed traits*) i `cargo-semver-checks`.

**Szukaj po polsku:** wersjonowanie semantyczne · zmiany łamiące zgodność · `rust semver breaking changes` · `cargo semver-checks` · `rust non_exhaustive sealed trait`
