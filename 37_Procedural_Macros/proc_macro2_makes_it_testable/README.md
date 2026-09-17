# `proc-macro2` makes it testable

**Level:** 201 · working knowledge

**One line:** `proc_macro`'s types work only while the compiler is running a macro, and anywhere else, a unit test included, the first call panics. So a macro's logic is written against `proc_macro2`, and the `#[proc_macro_derive]` function only converts its input in and its output back out.

## The shape: a wrapper and an `expand`

This derive adds a constant listing a struct's field names. The function the compiler calls is three lines; the work is in `expand`, which takes and returns `proc_macro2::TokenStream`:

<!-- file:demo/testable_field_names/src/lib.rs -->
```rust title="demo/testable_field_names/src/lib.rs"
//! `#[derive(FieldNames)]`: a constant listing a struct's field names.
//!
//! The macro is the three-line function at the top. Everything it does is in
//! `expand`, which never names a `proc_macro` type, so a unit test can call it.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

/// The only code that touches `proc_macro`: convert in, expand, convert out.
#[proc_macro_derive(FieldNames)]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into()).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// The derive's logic, written against `proc_macro2`.
fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(&input.ident, "FieldNames needs a struct"));
    };
    let names = data.fields.iter().enumerate().map(|(i, field)| match &field.ident {
        Some(ident) => ident.to_string(),
        None => i.to_string(), // a tuple struct's fields are named 0, 1, ...
    });
    let name = &input.ident;
    Ok(quote! {
        impl #name {
            pub const FIELD_NAMES: &[&str] = &[#(#names),*];
        }
    })
}

#[cfg(test)]
mod tests {
    use super::expand;
    use quote::quote;

    #[test]
    fn lists_the_fields() {
        let output = expand(quote! { struct Point { x: i32, y: i32 } }).unwrap();
        let expected = quote! {
            impl Point {
                pub const FIELD_NAMES: &[&str] = &["x", "y"];
            }
        };
        assert_eq!(output.to_string(), expected.to_string());
    }

    #[test]
    fn refuses_an_enum() {
        let error = expand(quote! { enum Direction { Up, Down } }).unwrap_err();
        assert_eq!(error.to_string(), "FieldNames needs a struct");
    }

    /// Fails: the expected side is typed the way rustfmt would lay it out.
    #[test]
    #[ignore = "fails on purpose; run with --ignored"]
    fn expected_as_a_typed_string() {
        let output = expand(quote! { struct Point { x: i32, y: i32 } }).unwrap();
        assert_eq!(
            output.to_string(),
            r#"impl Point { pub const FIELD_NAMES: &[&str] = &["x", "y"]; }"#
        );
    }
}
```
<!-- /file -->

`.into()` works in both directions because `proc_macro2::TokenStream` implements `From<proc_macro::TokenStream>`, and `proc_macro::TokenStream` implements `From<proc_macro2::TokenStream>`. `syn` and `quote` are written against `proc_macro2` too: `syn::parse2` takes its token stream, and `quote!` returns one. `expand` returns `syn::Result`, so the wrapper is also the one place a failure becomes a compiler error; [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) covers that half.

`serde_derive` 1.0.229 is built the same way: [`derive_serialize` ↗](https://docs.rs/crate/serde_derive/1.0.229/source/src/lib.rs#114) parses its input and calls [`ser::expand_derive_serialize` ↗](https://docs.rs/crate/serde_derive/1.0.229/source/src/ser.rs#13), which returns `syn::Result<proc_macro2::TokenStream>`, then ends with the same `.unwrap_or_else(syn::Error::into_compile_error).into()`.

The crate is a proc-macro crate with three ordinary dependencies:

<!-- file:demo/testable_field_names/Cargo.toml -->
```toml title="demo/testable_field_names/Cargo.toml"
[package]
name = "testable_field_names"
version = "0.1.0"
edition = "2024"
publish = false

[lib]
proc-macro = true

[dependencies]
syn = { version = "3", features = ["full", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
```
<!-- /file -->

A program uses it like any derive:

<!-- file:demo/testable_app/src/main.rs -->
```rust title="demo/testable_app/src/main.rs"
use testable_field_names::FieldNames;

#[derive(FieldNames)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(FieldNames)]
struct Meters(f64);

fn main() {
    println!("{:?}", Point::FIELD_NAMES); // ["x", "y"]
    println!("{:?}", Meters::FIELD_NAMES); // ["0"]
    let (p, m) = (Point { x: 1, y: 2 }, Meters(0.5));
    println!("{} {} {}", p.x, p.y, m.0); // 1 2 0.5
}
```
<!-- /file -->

<!-- cargo:testable_field_names_app -->
*Verified output of `cargo run -q -p testable_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
["x", "y"]
["0"]
1 2 0.5
```
<!-- /cargo -->

## Testing `expand`

The `tests` module at the bottom of `lib.rs` calls `expand` directly, with input built by `quote!`:

<!-- cargo:field_names_unit_tests -->
*Verified output of `cargo test -q -p testable_field_names` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
running 3 tests
i..
test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in <elapsed>


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in <elapsed>
```
<!-- /cargo -->

`i..` is one ignored test and two passes; the ignored one is the trap further down. The second `running 0 tests` block is the crate's doctests, which `cargo test` runs for a proc-macro crate as for any library. `finished in` is followed by a time, which changes on every run: the declared run replaces it with `<elapsed>` before the key is compared.

`lists_the_fields` builds the expected output with `quote!` too, and compares the two as strings. `proc_macro2::TokenStream` has no `PartialEq`, so `to_string()` is the usual way to compare two streams, and it is safe only when both sides were printed by the same printer.

## Without `proc-macro2`: the panic

This ordinary program parses the same text twice, once into each `TokenStream`:

<!-- file:demo/outside_a_macro/src/main.rs -->
```rust title="demo/outside_a_macro/src/main.rs"
//! The same parse, done twice in an ordinary program: once with `proc_macro2`,
//! then with the compiler's own `proc_macro`.

// Cargo links `proc_macro` into proc-macro crates only; any other crate names it.
extern crate proc_macro;

fn main() {
    let tokens: proc_macro2::TokenStream = "struct Point { x: i32 }".parse().unwrap();
    println!("proc_macro2: {tokens}");

    let tokens: proc_macro::TokenStream = "struct Point { x: i32 }".parse().unwrap();
    println!("proc_macro: {tokens}"); // never printed: the parse above panics
}
```
<!-- /file -->

<!-- cargo:proc_macro_outside_a_macro -->
*Verified output of `cargo run -q -p outside_a_macro`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
proc_macro2: struct Point { x : i32 }

thread 'main' (<id>) panicked at /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/proc_macro/src/bridge/client.rs:200:32:
procedural macro API is used outside of a procedural macro
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
<!-- /cargo -->

`proc_macro2` parsed and printed the struct. `proc_macro` panicked inside the parse, so its `println!` never ran.

- **The message comes from the bridge.** Every `proc_macro` call goes through a connection to the compiler that exists only while a macro is running, and [`Bridge::with` in `proc_macro/src/bridge/client.rs` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/proc_macro/src/bridge/client.rs#L200) panics when there is none. A unit test is no different: it is an ordinary program too.
- **The location is inside `proc_macro`, not in `main.rs`.** `/rustc/88d9e12ae…/library/…` is where the 1.98.0 release build says the standard library's source was, so the path is the same on every machine running that toolchain; the page's key was checked on macOS and in a Linux container.
- **One thing was filtered.** rustc 1.98.0 prints a numeric thread id after the thread's name, `thread 'main' (…)`, and it changes on every run. The declared run replaces the number with `<id>`.

`proc_macro2` avoids the panic by asking first. The first time it needs to know, it calls [`proc_macro::is_available()` ↗](https://docs.rs/crate/proc-macro2/1.0.107/source/src/detection.rs#26), and from then on [every new stream ↗](https://docs.rs/crate/proc-macro2/1.0.107/source/src/wrapper.rs#90) is either the compiler's `TokenStream`, wrapped, or `proc_macro2`'s own implementation of one. Inside `testable_app`'s build the derive used the compiler's; inside `cargo test` the same code used the fallback.

## The trap: `to_string()` is not source code

The third test types the expected output as a string, laid out the way `rustfmt` would lay it out. It is marked `#[ignore]`, so it runs only when asked for:

<!-- cargo:field_names_typed_string_test -->
*Verified output of `cargo test -q -p testable_field_names -- --ignored`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
running 1 test
tests::expected_as_a_typed_string --- FAILED

failures:

---- tests::expected_as_a_typed_string stdout ----

thread 'tests::expected_as_a_typed_string' (<id>) panicked at testable_field_names/src/lib.rs:61:9:
assertion `left == right` failed
  left: "impl Point { pub const FIELD_NAMES : & [& str] = & [\"x\" , \"y\"] ; }"
 right: "impl Point { pub const FIELD_NAMES: &[&str] = &[\"x\", \"y\"]; }"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::expected_as_a_typed_string

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out; finished in <elapsed>

error: test failed, to rerun pass `-p testable_field_names --lib`
```
<!-- /cargo -->

The tokens are right; the spaces are not. `proc_macro2`'s printer [puts a space between every two tokens ↗](https://docs.rs/crate/proc-macro2/1.0.107/source/src/fallback.rs#225) unless the first is a punctuation mark marked as joined to the next, which is why it wrote `& [& str]`. The compiler's printer is a different one: in [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md), a struct printed from inside a derive came out as `struct Point { x: i32, y: i32, }`. A string typed by hand agrees with neither printer except by luck, and the [`Display` docs ↗](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html#impl-Display-for-TokenStream) say the exact form is subject to change. Build the expected side with `quote!`, as `lists_the_fields` does, and both sides go through the same printer.

The same thread-id and time filters apply to this key as to the ones above.

## If you are coming from another language

- **Python.** A decorator is a function, and a test can call it with any function object; the `ast` module parses source in any program. `proc_macro` is not like that: its `TokenStream` is a handle into the running compiler, valid only during a macro call. `proc_macro2` gives Rust what Python has by default, a token type that also exists in an ordinary program.
- **Java.** An annotation processor receives `javax.lang.model` elements that `javac` creates, and they have no existence outside a compilation, so testing one means running `javac` from the test. Rust has the same split: code written against `proc_macro` can only be tested by compiling a crate, which is what [`trybuild`](../testing_with_trybuild/README.md) does. The `proc_macro2` shape is what Rust adds: the logic between the conversions can be tested as a plain function.
- **C and C++.** A `#define` cannot be tested apart from preprocessing a file that uses it. The nearest habit is the same one this page recommends: keep the macro a thin layer and put the logic in a function a test can call.

## See also

- [Tokens and token streams](../tokens_and_token_streams/README.md) — the types being wrapped
- [Parsing with `syn`](../parsing_with_syn/README.md) — the `syn::parse2` that `expand` starts with
- [Generating with `quote`](../generating_with_quote/README.md) — the `quote!` that builds the output and the tests' inputs
- [Testing with `trybuild`](../testing_with_trybuild/README.md) — the other half: testing the errors a user sees
- [Where a test goes](../../28_Testing/where_a_test_goes/README.md) — why `mod tests` can call the private `expand`
- [What a test asserts](../../28_Testing/what_a_test_asserts/README.md) — `assert_eq!` and the `left`/`right` report above
- [The `proc-macro2` crate ↗](https://docs.rs/proc-macro2/1.0.107/proc_macro2/)

## Po polsku

Typy z crate'a **`proc_macro`** działają tylko wtedy, gdy kompilator właśnie wykonuje makro: każde wywołanie idzie przez „most” (*bridge*) do kompilatora, a poza makrem — w zwykłym programie albo w **teście jednostkowym** — mostu nie ma i pierwsze wywołanie kończy się paniką *procedural macro API is used outside of a procedural macro*. Dlatego logikę makra pisze się na typach z **`proc-macro2`**: sama funkcja oznaczona `#[proc_macro_derive]` ma trzy linijki i tylko konwertuje strumień tokenów przez `.into()` w obie strony, a cała praca dzieje się w funkcji `expand`, którą test może wywołać bezpośrednio. `proc-macro2` sprawdza raz, czy kompilator jest dostępny (`proc_macro::is_available()`), i w zależności od tego opakowuje prawdziwy `TokenStream` albo używa własnej implementacji.

Pułapka: porównywanie wyniku z ręcznie wpisanym napisem. `to_string()` to nie kod źródłowy — drukarka `proc-macro2` wstawia spację między prawie każdą parą tokenów (`& [& str]`), a kompilator drukuje inaczej. Stronę oczekiwaną też buduje się przez `quote!`, żeby obie przeszły przez tę samą drukarkę.

**Szukaj po polsku:** testowanie makr proceduralnych w Ruscie · `proc-macro2` · `procedural macro API is used outside of a procedural macro` · `rust proc macro unit test` · `proc_macro::is_available`
