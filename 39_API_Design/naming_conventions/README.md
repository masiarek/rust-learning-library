# Naming conventions

**Level:** 201 · working knowledge

**One line:** In Rust a conversion method's prefix is a promise about cost and ownership — `as_` is free and borrows, `to_` does work, `into_` consumes `self` — and the rest of the naming rules (getters without `get_`, `iter`/`iter_mut`/`into_iter`, `new`/`with_…`/`from_…`) let a caller guess a method before looking it up.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The C-CONV table from the Rust API Guidelines: `as_` — free, borrowed to borrowed; `to_` — expensive, borrowed to borrowed or owned, or owned to owned for `Copy` types; `into_` — variable cost, owned to owned — checked against `str::as_bytes`, `str::to_lowercase`, `f64::to_radians` and `String::into_bytes`
- The `into_` that can fail: `BufWriter::into_inner` flushes, so it returns `Result<W, IntoInnerError<BufWriter<W>>>` — cost hidden behind a prefix that usually means "free"
- `clippy::wrong_self_convention` (style, warn by default): which prefix it pairs with which `self` receiver, and what it says about `fn to_x(self)` on a non-`Copy` type
- Getters: `fn first(&self) -> &First`, not `get_first` (C-GETTER); `get` kept for the one obvious thing (`Cell::get`) and for lookups with an argument that may fail (`slice::get` returning `Option`), with `_unchecked` variants for the `unsafe` versions
- Iterators: `iter`, `iter_mut` and `into_iter` returning `Iter`, `IterMut` and `IntoIter` (C-ITER, C-ITER-TY), and `clippy::iter_without_into_iter` (pedantic) for the `IntoIterator for &Collection` impl a `for` loop needs
- Constructors: `new`, `with_capacity`-style `with_…`, `from_…` for conversions (C-CTOR), and `clippy::new_without_default` (style) asking for `Default` beside a `pub fn new() -> Self`
- Casing from RFC 430: `UpperCamelCase` types with acronyms as one word (`Uuid`, not `UUID`), `snake_case` functions and modules, `SCREAMING_SNAKE_CASE` constants — enforced by rustc's `non_camel_case_types`, `non_snake_case` and `non_upper_case_globals`, all warn by default
- Word order (C-WORD-ORDER): the Guidelines list `ParseIntError`, `ParseFloatError` and five more in verb-object-error order and recommend `ParseAddrError` over `AddrParseError` — yet std ships `std::net::AddrParseError`. What does the guideline then ask for?

## The trap it exists for

`to_` on a method that returns a view, or `as_` on one that allocates. The caller trusts the prefix, calls it in a loop, and the cost the name hid becomes the top line of the profile. The prefix is part of the signature a reader sees; the body is not.

## Where this sits

[`ToOwned`](../../12_Traits/to_owned/README.md) is the trait behind `to_owned()`, and [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) covers those three methods from the caller's side. [`str::as_str`](../../14_Strings/str_as_str/README.md) and [RFC 1054](../../14_Strings/rfc_1054_str_words/README.md) are two naming decisions std made and then revisited. This page is the rule a library author follows so callers can predict names.

## See also

- [`ToOwned`: `Clone` for types whose owned twin is a different type](../../12_Traits/to_owned/README.md) — `to_` in its most-used form
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — the three names from the caller's side
- [The `Default` trait](../../03_Command_Line/the_default_trait/README.md) — the impl `new_without_default` asks for
- [`str::as_str`: the method that was stabilized and taken back](../../14_Strings/str_as_str/README.md) — an `as_` that had nothing to do
- [RFC 1054 — the method that renamed itself to promise less](../../14_Strings/rfc_1054_str_words/README.md) — a name changed because it promised too much
- [Rust API Guidelines — Naming ↗](https://rust-lang.github.io/api-guidelines/naming.html) — C-CASE through C-WORD-ORDER
- [RFC 430 — naming conventions ↗](https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html) — the casing rules

## If you are coming from another language

- **Python.** PEP 8 gives the same casing split: `CapWords` classes, `snake_case` functions, `UPPER_CASE` constants. Python names carry no cost convention, so `as_`/`to_`/`into_` is new. A `@property` makes a getter look like a field; Rust has no properties, so the getter is a method named after the field.
- **Java.** JavaBeans' `getName()` and `isActive()` are the convention Rust drops. The words `as` and `to` exist in Java too, and `Arrays.asList` does return a view backed by the array while `toArray` copies — the same distinction, without a style guide promising it.
- **Go.** Effective Go also drops `Get` from getters — `Owner()`, not `GetOwner()` — and names constructors `NewThing`. The cost-and-ownership prefixes have no Go counterpart, because Go has no ownership to describe.

## Po polsku

Przedrostek metody konwertującej to w Ruście obietnica: `as_` jest darmowe i pożycza, `to_` wykonuje pracę (często alokuje), `into_` konsumuje `self`. Gettery nie mają przedrostka `get_`, iteratory nazywają się `iter`, `iter_mut` i `into_iter`, a konstruktory `new`, `with_…` i `from_…`. Pułapka: `as_` na metodzie, która alokuje, albo `to_` na tanim widoku — wywołujący ufa nazwie i płaci koszt, którego się nie spodziewał.

**Szukaj po polsku:** konwencje nazewnicze w Ruście · nazwy metod konwertujących · `rust as_ to_ into_ naming` · `rust api guidelines naming` · `rust getter naming convention`
