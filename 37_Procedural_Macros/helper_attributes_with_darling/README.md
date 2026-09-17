# Helper attributes with `darling`

**Level:** 301 · deep dive

**One line:** `darling` turns reading attributes into declaring them: derive `FromDeriveInput` for the struct's attribute and `FromField` for a field's, give each key a field whose type says what to accept, and darling writes the parsing, the defaults, and an error for every mistake at once — with a *did you mean*.

## The same five keys, declared

This is the whole of `attrs.rs` for the `CustomDebug` derive from [Helper attributes by hand](../helper_attributes_by_hand/README.md). The by-hand version walked each attribute with `parse_nested_meta`; this one only says what may be there:

<!-- file:demo/darling_derive/src/attrs.rs -->
```rust title="demo/darling_derive/src/attrs.rs"
//! Reading `#[debug(...)]` with darling: declare what the attribute may say
//! as two structs, and derive the code that reads it.

use darling::util::{Flag, Ignored};
use darling::{FromDeriveInput, FromField, ast};
use syn::Ident;

/// What `#[debug(...)]` on the struct asked for.
#[derive(FromDeriveInput)]
#[darling(attributes(debug), supports(struct_named))]
pub struct Container {
    /// `ident` and `data` are filled from the item, not from the attribute.
    pub ident: Ident,
    pub data: ast::Data<Ignored, Field>,
    pub rename: Option<String>,
    #[darling(default = || "***".to_owned())]
    pub mask: String,
}

/// What `#[debug(...)]` on one field asked for.
#[derive(FromField)]
#[darling(attributes(debug))]
pub struct Field {
    pub ident: Option<Ident>,
    pub rename: Option<String>,
    pub skip: Flag,
    pub redact: Flag,
}
```
<!-- /file -->

| Field | Type | Read from | When the key is absent |
|---|---|---|---|
| `ident` | `Ident` | the struct's name; `ident` is a name darling fills from the input | — |
| `data` | [`ast::Data` ↗](https://docs.rs/darling/0.24.1/darling/ast/enum.Data.html)`<Ignored, Field>` | the struct's body, each field through `Field::from_field` | — |
| `rename` | `Option<String>` | `rename = "..."` | `None` |
| `mask` | `String` | `mask = "..."` | the value of the `default` closure |
| `skip`, `redact` | [`Flag` ↗](https://docs.rs/darling/0.24.1/darling/util/struct.Flag.html) | the bare word | not present |

Two container options do the rest. `attributes(debug)` names the attributes to read, and ignores every other attribute on the item, doc comments included. `supports(struct_named)` refuses enums, tuple structs and unit structs with an error, replacing the two `let ... else` checks of the by-hand version.

`lib.rs` differs from the by-hand one only where it meets darling: it calls `from_derive_input`, turns the errors into tokens with `write_errors()`, and reads `data.as_struct()`, an `Option<Ident>` and `Flag::is_present()` where the by-hand structs had a `Vec`, an `Ident` and a `bool`:

<!-- file:demo/darling_derive/src/lib.rs -->
```rust title="demo/darling_derive/src/lib.rs"
//! `#[derive(CustomDebug)]`: a `Debug` impl that the struct can shape with
//! `#[debug(...)]` attributes. This file declares the macro and writes the
//! `impl`; reading the attributes is `attrs.rs`'s job, and darling's.

mod attrs;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// `attributes(debug)` is still the compiler's business: darling reads the
/// attribute, but only this declaration lets the user write it.
#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match attrs::Container::from_derive_input(&input) {
        Ok(container) => expand(&container).into(),
        Err(errors) => errors.write_errors().into(),
    }
}

/// `f.debug_struct(name).field(..)...finish()`, one `.field` per shown field.
/// Generics are ignored here; `generics_lifetimes_and_where` adds them.
fn expand(container: &attrs::Container) -> proc_macro2::TokenStream {
    let ident = &container.ident;
    let name = container
        .rename
        .clone()
        .unwrap_or_else(|| ident.to_string());
    let mask = &container.mask;
    let fields = container.data.as_struct().expect("supports(struct_named)");
    let shown = fields.iter().filter(|field| !field.skip.is_present());
    let calls = shown.map(|field| {
        let member = field.ident.as_ref().expect("named fields have names");
        let label = field.rename.clone().unwrap_or_else(|| member.to_string());
        if field.redact.is_present() {
            quote!(.field(#label, &::core::format_args!("{}", #mask)))
        } else {
            quote!(.field(#label, &self.#member))
        }
    });
    // A skipped field still exists, so say so: `finish_non_exhaustive` prints `..`.
    let finish = if fields.iter().any(|field| field.skip.is_present()) {
        quote!(finish_non_exhaustive)
    } else {
        quote!(finish)
    };
    quote! {
        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(#name) #(#calls)* .#finish()
            }
        }
    }
}
```
<!-- /file -->

`attributes(debug)` appears twice, and both are needed. The one in `#[proc_macro_derive]` is the declaration that makes `#[debug]` legal for the user to write; the one in `#[darling]` tells the generated parser which attributes are its business.

The program that uses the derive is `by_hand_app` copied unchanged into `darling_app` — the counting program at the bottom of this page checks that — and it prints the same four lines. Its `use custom_debug::CustomDebug;` works in both demos because each program's `Cargo.toml` depends on its lesson's derive crate under that one name:

<!-- file:demo/darling_app/Cargo.toml -->
```toml title="demo/darling_app/Cargo.toml"
[package]
name = "darling_app"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
custom_debug = { path = "../darling_derive", package = "darling_derive" }
```
<!-- /file -->

<!-- cargo:custom_debug_with_darling -->
*Verified output of `cargo run -q -p darling_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Account { id: 7, email: "ada@example.com", password: <hidden>, .. }
Account {
    id: 7,
    email: "ada@example.com",
    password: <hidden>,
    ..
}
hidden, not gone: password has 7 bytes, failed_logins = 3
Point { x: 1, y: 2 }
```
<!-- /cargo -->

## Every mistake at once

The same three mistakes as on the by-hand page:

<!-- file:demo/darling_mistakes/src/main.rs -->
```rust title="demo/darling_mistakes/src/main.rs"
use custom_debug::CustomDebug;

#[derive(CustomDebug)]
#[debug(rename = "Account", rename = "User")] // the same key twice
pub struct User {
    #[debug(skipp)] // a typo
    pub id: u64,
    #[debug(rename = 5)] // a number where a string belongs
    pub password: String,
}

fn main() {}
```
<!-- /file -->

<!-- cargo:darling_reports_every_mistake -->
*Verified output of `cargo build -q -p darling_mistakes`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: Duplicate field `rename`
 --> darling_mistakes/src/main.rs:4:29
  |
4 | #[debug(rename = "Account", rename = "User")] // the same key twice
  |                             ^^^^^^

error: Unknown field: `skipp`. Did you mean `skip`?
 --> darling_mistakes/src/main.rs:6:13
  |
6 |     #[debug(skipp)] // a typo
  |             ^^^^^

error: Unexpected type `int`
 --> darling_mistakes/src/main.rs:8:22
  |
8 |     #[debug(rename = 5)] // a number where a string belongs
  |                      ^

error: could not compile `darling_mistakes` (bin "darling_mistakes") due to 3 previous errors
```
<!-- /cargo -->

By hand, the same file produced one error, and the duplicate `rename` got none at all. Here:

- **All three are reported.** The code darling generates collects errors in an [`Accumulator` ↗](https://docs.rs/darling/0.24.1/darling/error/struct.Accumulator.html) and keeps going ([`codegen/error.rs` ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/codegen/error.rs#13)), and `write_errors()` emits every one, each at its own span.
- **A duplicate key is an error**, not a silent overwrite.
- **The typo gets a suggestion.** It comes from `strsim`, which darling's default `suggestions` feature pulls in.
- **The wrong type is named**, `int`, and underlined at the literal.

## The trap: a key darling cannot leave out

`mask` has `#[darling(default = ...)]`. Here is the same key without it, in a derive of its own:

<!-- file:demo/mask_required/src/lib.rs -->
```rust title="demo/mask_required/src/lib.rs"
//! `mask` from `darling_derive`, declared without `#[darling(default)]`.

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(attributes(debug))]
struct Container {
    ident: syn::Ident,
    mask: String, // no #[darling(default)]
}

/// Adds `const MASK`, so there is something to generate from the attribute.
#[proc_macro_derive(MaskRequired, attributes(debug))]
pub fn derive_mask_required(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match Container::from_derive_input(&input) {
        Ok(Container { ident, mask }) => {
            quote!(impl #ident { pub const MASK: &str = #mask; }).into()
        }
        Err(errors) => errors.write_errors().into(),
    }
}
```
<!-- /file -->

<!-- file:demo/forgot_mask/src/main.rs -->
```rust title="demo/forgot_mask/src/main.rs"
use mask_required::MaskRequired;

#[derive(MaskRequired)]
pub struct User {
    pub id: u64,
}

fn main() {}
```
<!-- /file -->

<!-- cargo:darling_missing_field -->
*Verified output of `cargo build -q -p forgot_mask`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: Missing field `mask`
 --> forgot_mask/src/main.rs:3:10
  |
3 | #[derive(MaskRequired)]
  |          ^^^^^^^^^^^^
  |
  = note: this error originates in the derive macro `MaskRequired` (in Nightly builds, run with -Z macro-backtrace for more info)

error: could not compile `forgot_mask` (bin "forgot_mask") due to 1 previous error
```
<!-- /cargo -->

The mistake is in the macro, and the error lands on its user, who did nothing wrong: leaving a key out is what a default is for. It points at the derive because there is no attribute to underline.

The rule, from darling's source: a missing key calls [`FromMeta::from_none` ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/from_meta.rs#94) on the field's type, and the default implementation returns `None`, which means *required*. `Option<T>` overrides it to return `Some(None)` ([line 769 ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/from_meta.rs#769)), and `Flag` to return an absent flag ([`flag.rs` ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/util/flag.rs#70)). `String` does not, and neither does `bool` ([line 276 ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/from_meta.rs#276)): a `skip: bool` would accept the bare word `skip`, and then demand it on every field. So:

| For a key that is | Declare it as |
|---|---|
| a flag | `Flag`, which also keeps the word's span for a later error |
| optional, with no sensible default | `Option<T>` |
| optional, with a default | `T` plus `#[darling(default)]` (for `Default::default()`) or `#[darling(default = ...)]` |
| required | `T` alone, deliberately |

## What darling does not do

- **Declare the attribute.** `attributes(debug)` in `#[proc_macro_derive]` is still yours to write; darling cannot add to that line.
- **Check the meaning.** Nothing in `attrs.rs` relates `skip` to `redact`, here or by hand. A rule that involves two keys is still code you write, typically a function named by `#[darling(and_then = ...)]`, and a `Flag` keeps its word's span so that the error can point at the second one — the example in [`Flag`'s own documentation ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/util/flag.rs#18) does exactly that.
- **Come for free.** It is three more crates of darling's own, plus `ident_case` and `strsim`, for every project that builds your macro:

<!-- cargo:darling_dependency_tree -->
*Verified output of `cargo tree -p darling_derive -e normal` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
darling_derive v0.1.0 (proc-macro)
├── darling v0.24.1
│   ├── darling_core v0.24.1
│   │   ├── ident_case v1.0.1
│   │   ├── proc-macro2 v1.0.107
│   │   │   └── unicode-ident v1.0.25
│   │   ├── quote v1.0.47
│   │   │   └── proc-macro2 v1.0.107 (*)
│   │   ├── strsim v0.11.1
│   │   └── syn v3.0.6
│   │       ├── proc-macro2 v1.0.107 (*)
│   │       ├── quote v1.0.47 (*)
│   │       └── unicode-ident v1.0.25
│   └── darling_macro v0.24.1 (proc-macro)
│       ├── darling_core v0.24.1 (*)
│       ├── quote v1.0.47 (*)
│       └── syn v3.0.6 (*)
├── proc-macro2 v1.0.107 (*)
├── quote v1.0.47 (*)
└── syn v3.0.6 (*)
```
<!-- /cargo -->

The tree's first line gave the absolute path of the `darling_derive` folder, which differs between machines, so the recorded run filters it out. `darling_macro` is itself a proc-macro crate, built on `darling_core`. [Compile times](../../05_Tooling/compile_times/README.md) shows where a build's seconds go.

## What it costs, and what it saves

`darling_line_counts` reads both lessons' files while it compiles, and counts the lines of each, and the lines that are neither blank nor comments:

<!-- file:demo/darling_line_counts/src/main.rs -->
```rust title="demo/darling_line_counts/src/main.rs"
//! Compares this lesson's crates with helper_attributes_by_hand's, so the
//! numbers on both pages are a program's output rather than a hand count.
//!
//! `include_str!` reads each file while compiling, and Cargo rebuilds this
//! program whenever one of them changes.

/// A source file of the by-hand derive, in the neighbouring lesson's demo.
macro_rules! by_hand {
    ($file:literal) => {
        include_str!(concat!(
            "../../../../helper_attributes_by_hand/demo/by_hand_derive/src/",
            $file
        ))
    };
}

/// A source file of this lesson's derive.
macro_rules! darling {
    ($file:literal) => {
        include_str!(concat!("../../darling_derive/src/", $file))
    };
}

const MACRO_FILES: [(&str, &str, &str); 4] = [
    ("attrs.rs", "by hand", by_hand!("attrs.rs")),
    ("attrs.rs", "darling", darling!("attrs.rs")),
    ("lib.rs", "by hand", by_hand!("lib.rs")),
    ("lib.rs", "darling", darling!("lib.rs")),
];

fn main() {
    println!(
        "{:<9} {:<8} {:>5} {:>5}",
        "file", "version", "lines", "code"
    );
    for (file, version, text) in MACRO_FILES {
        let lines = text.lines().count();
        // Code: not blank and not a comment. Doc comments count as comments.
        let code = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .count();
        println!("{file:<9} {version:<8} {lines:>5} {code:>5}");
    }

    // The programs that use the derive are copies; check that they still are.
    let app = include_str!("../../../../helper_attributes_by_hand/demo/by_hand_app/src/main.rs")
        == include_str!("../../darling_app/src/main.rs");
    let mistakes =
        include_str!("../../../../helper_attributes_by_hand/demo/by_hand_mistakes/src/main.rs")
            == include_str!("../../darling_mistakes/src/main.rs");
    println!();
    println!("darling_app is by_hand_app, unchanged: {app}");
    println!("darling_mistakes is by_hand_mistakes, unchanged: {mistakes}");
}
```
<!-- /file -->

<!-- cargo:darling_line_counts -->
*Verified output of `cargo run -q -p darling_line_counts` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
file      version  lines  code
attrs.rs  by hand     96    84
attrs.rs  darling     28    20
lib.rs    by hand     54    42
lib.rs    darling     56    44

darling_app is by_hand_app, unchanged: true
darling_mistakes is by_hand_mistakes, unchanged: true
```
<!-- /cargo -->

`attrs.rs` shrank; `lib.rs` stayed about the same size, because it generates the same `impl`. The count also leaves out what the by-hand version never had: duplicate detection, every error at once, and suggestions.

## `syn` 3

darling's changelog gives 0.20.0 as the release that moved to syn 2 and 0.24.0 as the one that moved to syn 3 (*"Update `syn` to v3"*). The version has to match the `syn` your macro uses: `from_derive_input` takes darling's `syn::DeriveInput`, and syn 2's and syn 3's are [two versions of one crate](../../05_Tooling/two_versions_of_one_crate/README.md), so two different types. A tutorial written before 0.24 targets syn 2. The changelog lists one addition in 0.24.0 that touches this page's shape: `data` may now be any type implementing `TryFrom<&syn::Data>` with `darling::Error` as its error, without `#[darling(with = ...)]`.

## If you are coming from another language

- **Java.** darling turns a Rust helper attribute into what a Java annotation already is: a declared type. `@interface Debug { String rename() default ""; String mask() default "***"; }` is this page's `Container`, element for field and `default` for `#[darling(default = ...)]`. An element with no default must be written, and `javac` says so, as darling said *Missing field `mask`*; and like darling, `javac` reports each broken annotation rather than stopping at the first. The difference is where the declaration lives. Java's is a type the compiler itself checks against; darling's is a private struct inside the macro crate, and the checking is code it generated there, running while the user's crate compiles.
- **Python.** It is the step from reading `sys.argv` in a loop to `argparse`: declare what is accepted, and the library writes the parsing and the error messages. A dataclass shows the defaulting rule: a field with no default is a required argument, and one with `field(default=...)` is not — `mask: String` without `#[darling(default)]` is the first kind.

## See also

- [Helper attributes by hand](../helper_attributes_by_hand/README.md) — the same derive with `parse_nested_meta`, and the refusals that belong to the compiler rather than the macro
- [A `#[retry]` attribute](../a_retry_attribute/README.md) — darling's `FromMeta` for an attribute macro's arguments
- [A `StateMachine` derive](../a_state_machine_derive/README.md) — helper attributes on enum variants
- [Testing with `trybuild`](../testing_with_trybuild/README.md) — putting error messages like these under test
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — what `write_errors()` produces underneath
- [`darling` 0.24.1 on docs.rs ↗](https://docs.rs/darling/0.24.1/darling/)

## Po polsku

**`darling`** zamienia czytanie atrybutów w ich **deklarację**: zamiast przechodzić po `#[debug(...)]` metodą `parse_nested_meta`, piszesz strukturę z `#[derive(FromDeriveInput)]` dla atrybutów kontenera i `#[derive(FromField)]` dla atrybutów pola, a typ każdego pola mówi, co wolno napisać: `Option<String>` to klucz opcjonalny, `Flag` to **flaga** (samo słowo, np. `skip`), a `String` z `#[darling(default = ...)]` to klucz z **wartością domyślną** (*default*). `#[darling(attributes(debug), supports(struct_named))]` wybiera atrybuty do przeczytania i odrzuca enumy oraz struktury krotkowe. Wygenerowany kod zbiera wszystkie błędy naraz — powtórzony klucz, literówkę z podpowiedzią *Did you mean `skip`?*, zły typ wartości — a `write_errors()` zwraca je jako błędy kompilacji.

Pułapka: pole typu `String` albo `bool` bez `#[darling(default)]` jest **wymagane**, więc użytkownik makra dostaje *Missing field `mask`* wskazujące na `#[derive]`, choć błąd popełnił autor makra. `darling` nie deklaruje też atrybutu za ciebie (`attributes(debug)` w `#[proc_macro_derive]` nadal trzeba napisać), nie sprawdza sensu kombinacji kluczy i dokłada pięć crate'ów do kompilacji. Od wersji 0.24 `darling` używa syn 3.

**Szukaj po polsku:** `darling` w makrach derive · `FromDeriveInput` i `FromField` · `rust darling tutorial` · `darling default Flag Option` · `darling Missing field`
