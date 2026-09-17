# Traits every type should consider

**Level:** 201 · working knowledge

**One line:** A caller cannot implement std's traits for your type — the orphan rule forbids it — so `Debug`, `Clone`, `Default`, `PartialEq`, `Hash` and the rest are yours to derive or to leave out on purpose, and `Send` and `Sync` are yours to lose without noticing.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why it is your job: a downstream `impl fmt::Display for PathBuf` is `E0117`, *only traits defined in the current crate can be implemented for types defined outside of the crate*, and a caller's `#[derive(Debug)]` on a struct holding your type fails with *`Price` doesn't implement `Debug`*
- The Guidelines' C-COMMON-TRAITS list — `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default` — and the one on it `#[derive]` cannot write
- `Debug` on every public type (C-DEBUG), a `Debug` that is never empty (C-DEBUG-NONEMPTY), and `clippy::missing_fields_in_debug` (pedantic) for a hand-written one that skips a field — or leaves out a secret on purpose
- `Copy` as a promise: removing it in a later version breaks every caller that used a value after assigning it. When should a small public struct *not* be `Copy`, even though it could be?
- `PartialEq` without `Eq` and `PartialOrd` without `Ord` because of a float field; `clippy::derive_partial_eq_without_eq` (nursery) for the `Eq` you forgot; and derived `Ord` comparing fields in declaration order — [The comparison traits](../../12_Traits/comparison_traits/README.md)
- `Hash` has to agree with `Eq`: `clippy::derived_hash_with_manual_eq` is deny-by-default (correctness), and what a `HashMap` does with a type that breaks the rule
- `Default` beside `pub fn new() -> Self` (`clippy::new_without_default`), and `Serialize`/`Deserialize` behind an optional `serde` feature (C-SERDE) so callers who do not use serde never compile it — [Deriving `Serialize` and `Deserialize`](../../06_Data/serde_derive/README.md)
- `Send` and `Sync` are auto traits: nobody writes them, and a private `Rc<Vec<u8>>` field removes `Send` from a public `Cache`, so the caller's `thread::spawn` fails with *`Rc<Vec<u8>>` cannot be sent between threads safely* — naming a type they have never seen (C-SEND-SYNC)

## The trap it exists for

Publishing a struct without `Debug`. The caller who needs to print a value containing it cannot derive `Debug` on their own type and cannot implement it for yours; they write a wrapper, fork, or file an issue and wait for your next release.

## Where this sits

[The comparison traits](../../12_Traits/comparison_traits/README.md), [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) and [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) teach the traits themselves; [Marker traits](../../12_Traits/marker_traits/README.md) and [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) explain auto traits. This page is the checklist a library author runs over a type before publishing it.

## See also

- [The comparison traits](../../12_Traits/comparison_traits/README.md) — `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and the derived field order
- [The `Default` trait](../../03_Command_Line/the_default_trait/README.md) — what `Default` promises and who calls it
- [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) — the two auto traits and the error three layers deep
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — what the implicit copy commits you to
- [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) — why only one of the two can be derived
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — `#[derive]` as one of the five attribute families
- [SemVer hazards](../semver_hazards/README.md) — removing a trait, or losing `Send`, in a later version
- [Rust API Guidelines — Interoperability ↗](https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits) — C-COMMON-TRAITS, C-SERDE, C-SEND-SYNC

## If you are coming from another language

- **Java.** `equals`, `hashCode` and `toString` come from `Object` with identity-based defaults, so every class has them, usually wrong. A Rust type has none until you derive them, and `==` or `{:?}` on it does not compile. The `equals`/`hashCode` contract is the same rule as `Eq`/`Hash`.
- **Python.** `@dataclass` writes `__repr__` and `__eq__`, and with `eq=True` and no `frozen=True` it sets `__hash__` to `None` — the "hash must agree with equality" rule, enforced by making the type unhashable. [`repr` is not `str` ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/repr_is_not_str/) is the `Debug`/`Display` split in Python.
- **C++.** Since C++20, `= default` on `operator==` and `operator<=>` asks the compiler for memberwise comparisons, like deriving `PartialEq` and `PartialOrd`. There is no standard counterpart of `Debug`.
- **Go.** `==` works on any struct whose fields are all comparable, and `%v` prints any value, with no opt-in. Rust asks for both explicitly — and so lets a type refuse them.

## Po polsku

Wywołujący nie może zaimplementować cech biblioteki standardowej dla twojego typu — zabrania tego reguła sieroty (*orphan rule*, błąd `E0117`) — więc `Debug`, `Clone`, `Default`, `PartialEq`, `Hash` i reszta muszą pochodzić od ciebie, zanim opublikujesz crate. `Send` i `Sync` są cechami automatycznymi: nikt ich nie pisze, a jedno prywatne pole z `Rc` po cichu odbiera typowi `Send` i psuje kod użytkownika, który przekazuje go do innego wątku.

**Szukaj po polsku:** cechy standardowe dla typu · reguła sieroty · `rust api guidelines common traits` · `rust derive debug public types` · `rust auto traits send sync leak`
