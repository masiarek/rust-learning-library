# API design

**One line:** An interface is written for a caller who has not read your code — so it should be unsurprising (the names and traits they expect), flexible (it takes what they already hold), obvious (the docs and the types explain it) and constrained (you can change it later without breaking them).

This section is the public surface of a library — names, trait impls, parameter types, destructors, docs and versioning — and not the implementation behind it. [Traits](../12_Traits/README.md) and [Generics](../22_Generics/README.md) teach the mechanisms these pages choose between, [String parameters worth copying](../14_Strings/string_api_design/README.md) is the one design question the library has already answered with a measurement, and the release itself — `cargo publish`, versions, `Cargo.lock` — is [Tooling](../05_Tooling/README.md). The four words in the first line are the four headings of *Rust for Rustaceans* ch. 3 "Designing Interfaces", and each page is cross-checked against the [Rust API Guidelines ↗](https://rust-lang.github.io/api-guidelines/). **Every page here is a stub** — [CONTRIBUTING.md](../CONTRIBUTING.md) says what graduating one takes.

| Lesson | Level | What it covers |
|---|---|---|
| [Naming conventions](naming_conventions/README.md) | 201 | `as_` is free and borrows, `to_` does work, `into_` consumes — plus getters without `get_`, `iter`/`iter_mut`/`into_iter`, `new`/`with_…`/`from_…`, and the casing from RFC 430. Stub |
| [Traits every type should consider](traits_every_type_should_consider/README.md) | 201 | `Debug`, `Clone`, `Default`, `PartialEq`, `Hash` and the rest: which to derive before publishing, since a caller cannot add them later — and the `Send` a private field can take away. Stub |
| [Implementing your trait for references](blanket_impls_for_references/README.md) | 301 | Forwarding impls for `&T`, `&mut T` and `Box<T>`, so a caller holding a reference does not have to move or clone to call a generic function. Stub |
| [Wrapper types](wrapper_types/README.md) | 301 | Newtypes, and the choice between `Deref`, `AsRef`, `Borrow` and a plain method for reaching the inner value — with Deref polymorphism as the anti-pattern. Stub |
| [Generic or concrete parameters](generic_or_concrete_parameters/README.md) | 301 | `impl Trait`, a concrete type or `&dyn Trait` in parameter position, borrowed or owned — what each costs callers, binaries and your future changes. Stub |
| [Dyn compatibility](dyn_compatibility/README.md) | 301 | The rules a trait must meet to be used as `dyn Trait`, `where Self: Sized` as the way to keep a method anyway, and the rename from "object safety". Stub |
| [Destructors that can fail](destructors_that_can_fail/README.md) | 301 | `Drop` returns nothing and cannot be `async`, so a close that can fail needs an explicit `close(self) -> Result` beside a `Drop` that only tidies up. Stub |
| [Documenting an interface](documenting_an_interface/README.md) | 201 | `# Errors`, `# Panics` and `# Safety`, examples that are tests, `#[must_use]`, `#[doc(hidden)]`, and the lints that check each. Stub |
| [SemVer hazards](semver_hazards/README.md) | 301 | Changes that break callers without touching a signature — a public field, an enum variant, a second `From` impl, a lost `Send` — and `#[non_exhaustive]`, sealed traits and `cargo-semver-checks`. Stub |
| [Making misuse a compile error](making_misuse_a_compile_error/README.md) | 301 | Enums instead of bools, newtypes, typestate, `#[must_use]` and builders that cannot be built incomplete: moving a rule from the docs into the types. Stub |

## Where it goes next

Errors are part of every interface, and a library's error type is its own design question — [`thiserror` vs `anyhow`](../02_Errors/thiserror_vs_anyhow/README.md) is where this library takes it up, and *Rust for Rustaceans* follows ch. 3 with ch. 4 "Error Handling". When a function-shaped API cannot express what callers need, the next tool is a macro — [Declarative macros](../38_Declarative_Macros/README.md) — and the Guidelines' own macro chapter applies there.

## Po polsku

Projektowanie API (*API design*) to w tej sekcji projektowanie **publicznej powierzchni biblioteki**: nazw, implementacji cech, typów parametrów, destruktorów, dokumentacji i wersjonowania — nie tego, co jest w środku. Cztery słowa z pierwszego zdania to tytuły czterech części rozdziału 3 *Rust for Rustaceans*: interfejs ma być **nie zaskakujący**, **elastyczny**, **oczywisty** i **ograniczony**, czyli taki, który da się później zmienić bez psucia kodu użytkowników.

Polskie materiały o projektowaniu API w Ruście praktycznie nie istnieją, a angielskie terminy są tu operatywne — *newtype*, *blanket impl*, *dyn compatible*, *semver* pojawiają się w komunikatach kompilatora, w lintach clippy i w Rust API Guidelines, więc szukaj od razu po angielsku. Uwaga na jedno słowo: *object safety* w starszych tekstach to dokładnie to samo, co *dyn compatibility* w dzisiejszym kompilatorze.

**Szukaj po polsku:** projektowanie API w Ruście · konwencje nazewnicze · `rust api guidelines` · `rust for rustaceans designing interfaces` · `rust semver breaking changes`
