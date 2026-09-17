# Errors: from `panic!` to `syn::Error`

**Level:** 301 · deep dive

**One line:** A derive that panics gives the user *"proc-macro derive panicked"* underlining the derive's name, with the reason in a `help:` line that a one-line view drops. Returning `compile_error!` makes the reason the error, still at the derive. A `syn::Error` built with `new_spanned` underlines the tokens that are wrong, and `combine` reports every mistake in one build.

| The macro | The error reads | It underlines | Mistakes per build |
|---|---|---|---|
| panics | `proc-macro derive panicked`, and the reason as `help: message:` | the derive's name | the first |
| returns `compile_error!(…)` | the reason | the derive's name | the first, since this version returns early |
| returns a spanned `syn::Error` | the reason | the tokens that are wrong | every one, with `combine` |

## One derive, three ways to refuse

`#[derive(VariantName)]` gives an enum a `name()` method. It only makes sense for variants that carry no data, so a variant like `Custom(String)` is a mistake the macro has to report. The crate reports it three ways:

<!-- file:demo/variant_name/src/lib.rs -->
```rust title="demo/variant_name/src/lib.rs"
//! `#[derive(VariantName)]`: `name()` returns a variant's name, for an enum
//! whose variants carry no data.
//!
//! A variant that does carry data is the user's mistake. The three derives
//! report it three ways, and are otherwise the same.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Variant, parse_macro_input};

/// Way 1: panic.
#[proc_macro_derive(VariantNamePanics)]
pub fn variant_name_panics(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(data) = &input.data else {
        panic!("VariantName is for enums");
    };
    if let Some(variant) = data.variants.iter().find(|v| has_data(v)) {
        panic!("{}", mistake(variant));
    }
    generate(&input.ident, data.variants.iter()).into()
}

/// Way 2: return a `compile_error!` instead of the impl.
#[proc_macro_derive(VariantNameCompileError)]
pub fn variant_name_compile_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(data) = &input.data else {
        return quote!(::core::compile_error!("VariantName is for enums");).into();
    };
    if let Some(variant) = data.variants.iter().find(|v| has_data(v)) {
        let message = mistake(variant);
        return quote!(::core::compile_error!(#message);).into();
    }
    generate(&input.ident, data.variants.iter()).into()
}

/// Way 3: a `syn::Error` for every mistake, each spanned on the wrong tokens.
#[proc_macro_derive(VariantName)]
pub fn variant_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Returning `syn::Result` is what lets every check below use `?`.
fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "VariantName is for enums",
        ));
    };
    check_variants(data.variants.iter())?;
    Ok(generate(&input.ident, data.variants.iter()))
}

/// One error for every variant that carries data, combined into one `Err`.
fn check_variants<'a>(variants: impl Iterator<Item = &'a Variant>) -> syn::Result<()> {
    let combined = variants
        .filter(|v| has_data(v))
        .map(|v| syn::Error::new_spanned(&v.fields, mistake(v)))
        .reduce(|mut all, next| {
            all.combine(next);
            all
        });
    match combined {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn has_data(variant: &Variant) -> bool {
    !matches!(variant.fields, Fields::Unit)
}

fn mistake(variant: &Variant) -> String {
    let name = &variant.ident;
    format!("`{name}` carries data; VariantName needs variants without fields")
}

fn generate<'a>(name: &Ident, variants: impl Iterator<Item = &'a Variant>) -> TokenStream2 {
    let variants: Vec<&Ident> = variants.map(|v| &v.ident).collect();
    quote! {
        impl #name {
            pub fn name(&self) -> &'static str {
                match self {
                    #(Self::#variants => ::core::stringify!(#variants),)*
                }
            }
        }
    }
}
```
<!-- /file -->

Used correctly:

<!-- file:demo/method_app/src/main.rs -->
```rust title="demo/method_app/src/main.rs"
use variant_name_derive::VariantName;

#[derive(VariantName)]
enum Method {
    Get,
    Post,
    Delete,
}

fn main() {
    for method in [Method::Get, Method::Post, Method::Delete] {
        println!("{}", method.name());
    }
}
```
<!-- /file -->

<!-- cargo:variant_name_works -->
*Verified output of `cargo run -q -p variant_name_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Get
Post
Delete
```
<!-- /cargo -->

## The same mistake, reported three ways

This user added a variant that carries data. The three crates that make this mistake differ only in which derive they name; this one uses `VariantNamePanics`:

<!-- file:demo/reported_by_panic/src/main.rs -->
```rust title="demo/reported_by_panic/src/main.rs"
use variant_name_derive::VariantNamePanics;

#[derive(VariantNamePanics)]
enum Method {
    Get,
    Post,
    Custom(String),
}

fn main() {
    let method = Method::Custom("PURGE".to_string());
    println!("{}", method.name());
}
```
<!-- /file -->

### 1. `panic!`

<!-- cargo:variant_name_reported_by_panic -->
*Verified output of `cargo build -q -p variant_name_by_panic`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: proc-macro derive panicked
 --> reported_by_panic/src/main.rs:3:10
  |
3 | #[derive(VariantNamePanics)]
  |          ^^^^^^^^^^^^^^^^^
  |
  = help: message: `Custom` carries data; VariantName needs variants without fields

error[E0599]: no method named `name` found for enum `Method` in the current scope
  --> reported_by_panic/src/main.rs:12:27
   |
 4 | enum Method {
   | ----------- method `name` not found for this enum
...
12 |     println!("{}", method.name());
   |                           ^^^^ method not found in `Method`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `variant_name_by_panic` (bin "variant_name_by_panic") due to 2 previous errors
```
<!-- /cargo -->

The compiler catches the panic and reports it at the only place it knows about, the derive's name. The reason is there, but as a `help:` line under a headline that says nothing about `Custom`. No location inside the macro crate is printed, and no backtrace hint: the key above was recorded on macOS and checked unchanged on Linux.

The second error follows from the first. The derive returned no `impl`, so `name()` does not exist, and every version on this page produces the same `E0599`. [Re-emit the item on error](../re_emit_on_error/README.md) is about the attribute-macro version of that follow-on error.

### 2. `compile_error!`

<!-- cargo:variant_name_reported_by_compile_error -->
*Verified output of `cargo build -q -p variant_name_by_compile_error`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: `Custom` carries data; VariantName needs variants without fields
 --> reported_by_compile_error/src/main.rs:3:10
  |
3 | #[derive(VariantNameCompileError)]
  |          ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the derive macro `VariantNameCompileError` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `name` found for enum `Method` in the current scope
  --> reported_by_compile_error/src/main.rs:12:27
   |
 4 | enum Method {
   | ----------- method `name` not found for this enum
...
12 |     println!("{}", method.name());
   |                           ^^^^ method not found in `Method`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `variant_name_by_compile_error` (bin "variant_name_by_compile_error") due to 2 previous errors
```
<!-- /cargo -->

The reason is now the error message itself, and the error is a normal compile error rather than a crashed macro. It still underlines the derive's name: `quote!` gives the tokens it writes [`Span::call_site()` ↗](https://docs.rs/quote/1.0.47/quote/macro.quote.html#hygiene), and for a derive the call site is its name inside `#[derive(…)]`. The `note:` says as much: *this error originates in the derive macro*. The macro also stops at the first mistake, as the panic did.

### 3. A spanned `syn::Error`

<!-- cargo:variant_name_reported_by_syn_error -->
*Verified output of `cargo build -q -p variant_name_by_syn_error`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: `Custom` carries data; VariantName needs variants without fields
 --> reported_by_syn_error/src/main.rs:7:11
  |
7 |     Custom(String),
  |           ^^^^^^^^

error[E0599]: no method named `name` found for enum `Method` in the current scope
  --> reported_by_syn_error/src/main.rs:12:27
   |
 4 | enum Method {
   | ----------- method `name` not found for this enum
...
12 |     println!("{}", method.name());
   |                           ^^^^ method not found in `Method`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `variant_name_by_syn_error` (bin "variant_name_by_syn_error") due to 2 previous errors
```
<!-- /cargo -->

The underline is on `(String)`, the fields that should not be there, and the `originates in the derive macro` note is gone because the span is the user's own code. `Error::new_spanned(&variant.fields, …)` [keeps the span of the first and last token ↗](https://docs.rs/crate/syn/3.0.6/source/src/error.rs#199) it was given, here `(` and `)`. [`into_compile_error` ↗](https://docs.rs/crate/syn/3.0.6/source/src/error.rs#361) then writes `::core::compile_error!{ "…" }` with the path spanned at the first and the braces at the last, and rustc underlines everything between. So a `syn::Error` *is* a `compile_error!`, with spans taken from the input instead of the call site.

An editor that shows `cargo check`'s diagnostics draws its underline from the same span as the carets: under `VariantNamePanics` and `VariantNameCompileError` inside `#[derive(…)]` for the first two, and under `(String)` for the third.

## Every mistake in one build

A user with two mistakes, under the `syn::Error` version:

<!-- file:demo/two_mistakes/src/main.rs -->
```rust title="demo/two_mistakes/src/main.rs"
use variant_name_derive::VariantName;

#[derive(VariantName)]
enum Method {
    Get,
    Custom(String),
    Post,
    Numbered { code: u16 },
}

fn main() {
    let method = Method::Custom("PURGE".to_string());
    println!("{}", method.name());
}
```
<!-- /file -->

<!-- cargo:variant_name_two_mistakes_combined -->
*Verified output of `cargo build -q -p variant_name_two_mistakes`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: `Custom` carries data; VariantName needs variants without fields
 --> two_mistakes/src/main.rs:6:11
  |
6 |     Custom(String),
  |           ^^^^^^^^

error: `Numbered` carries data; VariantName needs variants without fields
 --> two_mistakes/src/main.rs:8:14
  |
8 |     Numbered { code: u16 },
  |              ^^^^^^^^^^^^^

error[E0599]: no method named `name` found for enum `Method` in the current scope
  --> two_mistakes/src/main.rs:13:27
   |
 4 | enum Method {
   | ----------- method `name` not found for this enum
...
13 |     println!("{}", method.name());
   |                           ^^^^ method not found in `Method`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `variant_name_two_mistakes` (bin "variant_name_two_mistakes") due to 3 previous errors
```
<!-- /cargo -->

`check_variants` makes one `syn::Error` per bad variant and folds them together with `reduce`, calling [`Error::combine` ↗](https://docs.rs/crate/syn/3.0.6/source/src/error.rs#355), which appends the other error's messages. `into_compile_error` then writes one `compile_error!` per message. syn also implements [`Extend<Error>` for `Error` ↗](https://docs.rs/crate/syn/3.0.6/source/src/error.rs#546) by calling `combine`, so `first.extend(rest)` does the same. The panic and `compile_error!` versions would have needed a second build to find `Numbered`.

## The `Result` shape that makes `?` work

The `syn::Error` version is written as two layers:

- **`variant_name`**, the entry point, only converts: `expand(&input).unwrap_or_else(syn::Error::into_compile_error).into()`. `into_compile_error` has the signature `fn(Error) -> proc_macro2::TokenStream`, so it can be passed straight to `unwrap_or_else`.
- **`expand`** returns `syn::Result<proc_macro2::TokenStream>`, so each check ends in `?` and the first `Err` returns from the whole macro. `syn::Result<T>` is [`Result<T, syn::Error>` ↗](https://docs.rs/crate/syn/3.0.6/source/src/error.rs#21).

[`parse_macro_input!` ↗](https://docs.rs/crate/syn/3.0.6/source/src/parse_macro_input.rs#108) is the same idea for parsing: on a parse error it `return`s `err.to_compile_error()`, converted to a `proc_macro::TokenStream`, from the function it is written in. That is why it goes in the entry point, and why `expand` takes an already-parsed `DeriveInput`.

## In one line each

`cargo build --message-format=short` prints one line per diagnostic, location first:

<!-- cargo:variant_name_panic_short -->
*Verified output of `cargo build -q --message-format=short -p variant_name_by_panic`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
reported_by_panic/src/main.rs:3:10: error: proc-macro derive panicked
reported_by_panic/src/main.rs:12:27: error[E0599]: no method named `name` found for enum `Method` in the current scope: method not found in `Method`
error: could not compile `variant_name_by_panic` (bin "variant_name_by_panic") due to 2 previous errors
```
<!-- /cargo -->

<!-- cargo:variant_name_compile_error_short -->
*Verified output of `cargo build -q --message-format=short -p variant_name_by_compile_error`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
reported_by_compile_error/src/main.rs:3:10: error: `Custom` carries data; VariantName needs variants without fields
reported_by_compile_error/src/main.rs:12:27: error[E0599]: no method named `name` found for enum `Method` in the current scope: method not found in `Method`
error: could not compile `variant_name_by_compile_error` (bin "variant_name_by_compile_error") due to 2 previous errors
```
<!-- /cargo -->

<!-- cargo:variant_name_syn_error_short -->
*Verified output of `cargo build -q --message-format=short -p variant_name_by_syn_error`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
reported_by_syn_error/src/main.rs:7:11: error: `Custom` carries data; VariantName needs variants without fields
reported_by_syn_error/src/main.rs:12:27: error[E0599]: no method named `name` found for enum `Method` in the current scope: method not found in `Method`
error: could not compile `variant_name_by_syn_error` (bin "variant_name_by_syn_error") due to 2 previous errors
```
<!-- /cargo -->

The panic's reason is gone: its line says *proc-macro derive panicked* and nothing else, because the reason was only ever a `help:` note. Only the `syn::Error` line's location, `7:11`, is where the mistake is.

## syn 2 and syn 3

Everything this page calls exists in syn 2 with the same behaviour. syn 3 changed the signatures of `Error::new` and `Error::new_spanned` from type parameters to `impl Display` and `impl ToTokens` arguments, which only breaks a call that names the types with a turbofish, and added `Error::new_range`, which spans a range of parse cursors. `combine`, `to_compile_error` and `into_compile_error` are unchanged.

## If you are coming from another language

- **C and C++.** `#error "message"` stops compilation with your message at the directive's own line, which is the `compile_error!` version: the right words, the wrong place. C++'s `static_assert(cond, "message")` inside a template has the same problem, softened by the *required from here* chain GCC prints back to the user's line.
- **Python.** A decorator or metaclass that raises while the class is being built gives a traceback whose last frame is inside the library, the panic version. `warnings.warn(message, stacklevel=2)` is the closest thing to `new_spanned`: it attributes the warning to the caller's line instead of the library's. Validators that collect every problem before raising, as `pydantic` does with a `ValidationError` listing each field, are `combine`.
- **Java.** An annotation processor that throws an exception crashes the compilation with a stack trace, the panic. The supported way is `Messager.printMessage(Diagnostic.Kind.ERROR, message, element)`: the `element` argument places the error on the user's declaration, as `new_spanned` places it on tokens, and a processor can report several before the round ends, which is `combine`.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP.)* `MESSAGE … TYPE 'E'` stops at the first problem, like the panic and `compile_error!` versions. The application log (`BAL_LOG_CREATE`, `BAL_LOG_MSG_ADD`, then one display) collects every message and shows them together, which is `combine`.

## See also

- [Every struct and enum shape](../every_struct_and_enum_shape/README.md) — refusing a union with a spanned error
- [Re-emit the item on error](../re_emit_on_error/README.md) — the attribute-macro version, and the follow-on errors a missing item causes
- [Testing with `trybuild`](../testing_with_trybuild/README.md) — holding these messages to a recorded answer
- [Parsing with `syn`](../parsing_with_syn/README.md) — where `parse_macro_input!` comes from
- [`unwrap` is a TODO](../../02_Errors/unwrap_is_a_todo/README.md) — the same argument against panicking, for ordinary code
- [Procedural macros](../README.md) — the chapter, in reading order

## Po polsku

Ten sam błąd użytkownika — wariant enuma z danymi, `Custom(String)`, tam gdzie makro obsługuje tylko warianty bez pól — zgłoszony na trzy sposoby, każdy z zapisanym komunikatem kompilatora. **`panic!`** daje nagłówek *proc-macro derive panicked* wskazujący nazwę makra, a przyczynę tylko w linijce `help:`, której w formacie `--message-format=short` w ogóle nie ma. **`compile_error!`** robi z przyczyny właściwy komunikat, ale nadal podkreśla nazwę makra, bo `quote!` nadaje tokenom `Span::call_site()`. **`syn::Error::new_spanned`** podkreśla dokładnie te tokeny, które są złe — tu `(String)` — a `Error::combine` pozwala zgłosić wszystkie pomyłki w jednej kompilacji.

Wygodny układ kodu: punkt wejścia tylko zamienia wynik, `expand(&input).unwrap_or_else(syn::Error::into_compile_error)`, a funkcja `expand` zwraca `syn::Result`, więc każde sprawdzenie może kończyć się operatorem `?`.

**Szukaj po polsku:** błędy w makrach proceduralnych · `syn::Error::new_spanned` · `compile_error!` · `rust proc macro error handling` · `syn Error combine`
