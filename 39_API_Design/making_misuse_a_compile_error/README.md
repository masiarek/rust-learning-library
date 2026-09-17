# Making misuse a compile error

**Level:** 301 · deep dive

**One line:** Every rule a caller has to remember is a bug waiting for the caller who did not — so put the rule in a type: an enum instead of a `bool`, a newtype instead of a bare `u64`, a state in a type parameter, a builder with no `build()` until its required fields are set.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- C-CUSTOM-TYPE, "Arguments convey meaning through types, not bool or Option": `connect(true, false)` against `connect(Tls::On, Retry::Never)` — [An enum instead of a bool](../../13_Enums/an_enum_instead_of_a_bool/README.md) — and `clippy::fn_params_excessive_bools` (pedantic) as the lint that notices
- Newtypes for ids and units (C-NEWTYPE): a `UserId(u64)` and an `OrderId(u64)` that cannot be passed in each other's place — [A score is not a number](../../16_Structs/newtype_score/README.md)
- Typestate: `Connection<Closed>` whose `open(self)` returns `Connection<Open>`, with `send` defined only on `Connection<Open>` — the zero-sized marker types from [Phantom types](../../12_Traits/phantom_types/README.md), and what the caller's error looks like when they call `send` too early
- A permission as a value: the token you can only hold after the check succeeded — [The right to post is a value, not a flag](../../09_Advanced/one_account_one_review/README.md)
- Builders (C-BUILDER): a plain builder whose `build()` returns `Result` or panics when a field is missing, against a typestate builder where `build()` does not exist until it can succeed — and what the second costs in type parameters, docs and error messages
- `#[must_use]` on types and on methods that return `Self`, so `builder.retries(3);` without reassigning the result warns — `clippy::return_self_not_must_use` (pedantic) for finding them
- Private fields plus a validating constructor (C-STRUCT-PRIVATE, C-VALIDATE), so an invalid value cannot be built at all — and the trap that privacy is per module, not per type
- *Rust for Rustaceans* ch. 3 → "Obvious" → "Type System Guidance" as the source, and where encoding a rule in types stops paying: which rules are cheaper as a runtime check and a `# Panics` line?

## The trap it exists for

Stopping at a runtime check. `fn send(&self, msg: &[u8]) -> Result<(), NotConnected>` is correct, and now every caller handles — or `?`s past, or `unwrap`s — an error that a `Connection<Open>` receiver would have made impossible to write. The check runs on every call, and the mistake is still found in production rather than by the compiler.

## Where this sits

[An enum instead of a bool](../../13_Enums/an_enum_instead_of_a_bool/README.md), [A score is not a number](../../16_Structs/newtype_score/README.md), [Phantom types](../../12_Traits/phantom_types/README.md) and [The right to post is a value](../../09_Advanced/one_account_one_review/README.md) each demonstrate one technique with a program. This page is the design question across them: which rule in a public API is worth a type, and what each encoding costs its callers.

## See also

- [An enum instead of a bool](../../13_Enums/an_enum_instead_of_a_bool/README.md) — the argument that cannot be passed backwards
- [The right to post is a value, not a flag](../../09_Advanced/one_account_one_review/README.md) — a capability you have to hold
- [Phantom types](../../12_Traits/phantom_types/README.md) — typestate with zero-sized tags
- [A score is not a number: the newtype](../../16_Structs/newtype_score/README.md) — one checked door into a type
- [An enum as a state machine](../../13_Enums/an_enum_as_a_state_machine/README.md) — the runtime alternative to typestate
- [What an invariant is](../../09_Advanced/what_an_invariant_is/README.md) — the rule a type is protecting
- [Documenting an interface](../documenting_an_interface/README.md) — for the rules that stay in the docs
- [Rust API Guidelines — Type safety ↗](https://rust-lang.github.io/api-guidelines/type-safety.html) — C-NEWTYPE, C-CUSTOM-TYPE, C-BITFLAG, C-BUILDER

## If you are coming from another language

- **Java.** Builders are everywhere, and a missing required field is found at run time: `HttpRequest.Builder.build()` throws `IllegalStateException` when no URI was set. A typestate builder turns that exception into a method that does not exist yet.
- **Python.** `Enum` and `typing.NewType` exist, but `NewType` is erased at run time and enforced only by a type checker such as mypy — the distinction is real in the editor and absent in the program.
- **Go.** A defined type (`type UserID int64`) gives the newtype cheaply, but every type has a usable zero value, so a `Connection{}` that was never opened can always be written.
- **C++.** Wrapper structs and tag types give newtypes and typestate, and `[[nodiscard]]` is `#[must_use]`. A moved-from object is still usable, though, so a typestate transition that consumes the old state cannot be enforced the way Rust's move does.

## Po polsku

Każda reguła, o której wywołujący musi pamiętać, to błąd czekający na tego, kto zapomni — więc lepiej zapisać ją w typie: enum zamiast `bool`, nowy typ (*newtype*) zamiast gołego `u64`, stan w parametrze typu (*typestate*), budowniczy (*builder*) bez metody `build()`, dopóki nie ustawiono wymaganych pól. Pułapka: poprzestanie na sprawdzeniu w czasie działania — funkcja zwracająca `Result<(), NotConnected>` jest poprawna, ale każdy wywołujący musi obsłużyć błąd, którego odbiorca `Connection<Open>` w ogóle nie pozwoliłby napisać.

**Szukaj po polsku:** bezpieczeństwo typów w API · wzorzec typestate · `rust make invalid states unrepresentable` · `rust typestate builder` · `rust api guidelines type safety`
