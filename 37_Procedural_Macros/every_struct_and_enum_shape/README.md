# Every struct and enum shape

**Level:** 301 · deep dive

**One line:** A derive written with `struct Point { x: i32, y: i32 }` in mind breaks on the first `struct Meters(f64)`. Handle all three of `Fields::Named`, `Fields::Unnamed` and `Fields::Unit`, twice: a struct's fields are `self.x` and `self.0`, where the `0` must be a `syn::Index`, and an enum's are reachable only through a `match` pattern of the variant's own shape. A union gets a spanned error.

| You declare | `syn` hands the derive | The generated code reaches a field as |
|---|---|---|
| `struct Point { x: i32, y: i32 }` | `Data::Struct`, `Fields::Named` | `self.x` |
| `struct Meters(f64)` | `Data::Struct`, `Fields::Unnamed` | `self.0` |
| `struct Origin;` | `Data::Struct`, `Fields::Unit` | there are none |
| `enum Shape { Circle { radius: f64 }, Rect(u32, u32), Empty }` | `Data::Enum`: a `Variant` each, with its own `Fields` | a binding in a `match` arm |
| `union IntOrFloat { i: u32, f: f32 }` | `Data::Union`, always named fields | refuse it |

## One derive for every shape

`#[derive(Describe)]` adds a `describe()` method that returns the value's shape, written as Rust writes it, and every field's name and value:

<!-- file:demo/describe/src/lib.rs -->
```rust title="demo/describe/src/lib.rs"
//! `#[derive(Describe)]`: a `describe()` method for every shape a struct or an
//! enum can take.
//!
//! The method returns the shape, written the way Rust writes it, and each
//! field's name and value. Getting there means handling all three kinds of
//! `Fields` twice: reached through `self` in a struct, bound by a pattern in an
//! enum.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Index, Variant, parse_macro_input};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let body = match &input.data {
        Data::Struct(data) => describe_struct(name, &data.fields),
        Data::Enum(data) => {
            let arms = data.variants.iter().map(|v| describe_variant(name, v));
            quote!(match self { #(#arms)* })
        }
        // A union does not record which field holds a value, so there is
        // nothing safe to read. Refuse it, pointing at the `union` keyword.
        Data::Union(data) => {
            let message = "Describe cannot read a union: nothing records which field is set";
            return syn::Error::new_spanned(data.union_token, message)
                .into_compile_error()
                .into();
        }
    };
    quote! {
        impl #name {
            pub fn describe(&self) -> (&'static str, ::std::vec::Vec<(&'static str, &dyn ::core::fmt::Debug)>) {
                #body
            }
        }
    }
    .into()
}

/// A struct's fields are reached through `self`: `self.x` when a field has a
/// name, `self.0` when it only has a position.
fn describe_struct(name: &Ident, fields: &Fields) -> TokenStream2 {
    let label = label(name.to_string(), fields);
    let names = field_names(fields);
    let members = fields
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(ident) => quote!(#ident),
            None => {
                let index = Index::from(i); // prints as `0`; a bare usize prints as `0usize`
                quote!(#index)
            }
        });
    quote! {
        (#label, ::std::vec::Vec::from([
            #((#names, &self.#members as &dyn ::core::fmt::Debug)),*
        ]))
    }
}

/// An enum's fields are only reachable through a pattern, one `match` arm per
/// variant, and the pattern's shape has to match the variant's.
fn describe_variant(enum_name: &Ident, variant: &Variant) -> TokenStream2 {
    let variant_name = &variant.ident;
    let label = label(format!("{enum_name}::{variant_name}"), &variant.fields);
    let names = field_names(&variant.fields);
    let bindings: Vec<Ident> = (0..variant.fields.len())
        .map(|i| format_ident!("field_{i}"))
        .collect();
    let pattern = match &variant.fields {
        Fields::Named(fields) => {
            let idents = fields.named.iter().map(|f| &f.ident);
            quote!(Self::#variant_name { #(#idents: #bindings),* })
        }
        Fields::Unnamed(_) => quote!(Self::#variant_name(#(#bindings),*)),
        Fields::Unit => quote!(Self::#variant_name),
    };
    quote! {
        #pattern => (#label, ::std::vec::Vec::from([
            #((#names, #bindings as &dyn ::core::fmt::Debug)),*
        ])),
    }
}

/// `Point { .. }`, `Meters(..)` or `Origin`: the shape, in Rust's own syntax.
fn label(name: String, fields: &Fields) -> String {
    match fields {
        Fields::Named(_) => format!("{name} {{ .. }}"),
        Fields::Unnamed(_) => format!("{name}(..)"),
        Fields::Unit => name,
    }
}

/// `x` for a named field, `0` for the first unnamed one.
fn field_names(fields: &Fields) -> impl Iterator<Item = String> + '_ {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(ident) => ident.to_string(),
            None => i.to_string(),
        })
}
```
<!-- /file -->

Four structs and an enum, one of each shape:

<!-- file:demo/shapes/src/lib.rs -->
```rust title="demo/shapes/src/lib.rs"
//! Every shape a struct can have, and an enum with every shape of variant.

use describe_derive::Describe;

#[derive(Describe)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Describe)]
pub struct Meters(pub f64);

#[derive(Describe)]
pub struct Origin;

#[derive(Describe)]
pub enum Shape {
    Circle { radius: f64 },
    Rect(u32, u32),
    Empty,
}
```
<!-- /file -->

<!-- file:demo/shapes_app/src/main.rs -->
```rust title="demo/shapes_app/src/main.rs"
use describe_shapes::{Meters, Origin, Point, Shape};
use std::fmt::Debug;

/// The shape on one line, then each field on a line of its own.
fn show((shape, fields): (&str, Vec<(&str, &dyn Debug)>)) {
    println!("{shape}");
    for (name, value) in fields {
        println!("   {name} = {value:?}");
    }
}

fn main() {
    show(Point { x: 1, y: 2 }.describe());
    show(Meters(5.5).describe());
    show(Origin.describe());
    show(Shape::Circle { radius: 1.5 }.describe());
    show(Shape::Rect(3, 4).describe());
    show(Shape::Empty.describe());
}
```
<!-- /file -->

<!-- cargo:describe_every_shape -->
*Verified output of `cargo run -q -p describe_shapes_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Point { .. }
   x = 1
   y = 2
Meters(..)
   0 = 5.5
Origin
Shape::Circle { .. }
   radius = 1.5
Shape::Rect(..)
   0 = 3
   1 = 4
Shape::Empty
```
<!-- /cargo -->

The field `0` is printed under its position, the same name `self.0` uses. `Origin` and `Shape::Empty` have no fields, and neither needed a special case in the body: `Fields::iter()` on `Fields::Unit` yields nothing, so the repetition runs zero times.

## What it generated

To see this yourself, run `cargo expand` in `demo/shapes`. It is a separate install, so the key below was recorded with the command it runs underneath, `-Zunpretty=expanded`, which `RUSTC_BOOTSTRAP=1` lets the stable compiler accept:

<!-- cargo:describe_every_shape_expanded -->
*Verified output of `cargo rustc -q -p describe_shapes --profile=check -- -Zunpretty=expanded` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
#![feature(prelude_import)]
//! Every shape a struct can have, and an enum with every shape of variant.
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;

use describe_derive::Describe;

pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn describe(&self)
        ->
            (&'static str,
            ::std::vec::Vec<(&'static str, &dyn ::core::fmt::Debug)>) {
        ("Point { .. }",
            ::std::vec::Vec::from([("x", &self.x as &dyn ::core::fmt::Debug),
                        ("y", &self.y as &dyn ::core::fmt::Debug)]))
    }
}

pub struct Meters(pub f64);
impl Meters {
    pub fn describe(&self)
        ->
            (&'static str,
            ::std::vec::Vec<(&'static str, &dyn ::core::fmt::Debug)>) {
        ("Meters(..)",
            ::std::vec::Vec::from([("0",
                            &self.0 as &dyn ::core::fmt::Debug)]))
    }
}

pub struct Origin;
impl Origin {
    pub fn describe(&self)
        ->
            (&'static str,
            ::std::vec::Vec<(&'static str, &dyn ::core::fmt::Debug)>) {
        ("Origin", ::std::vec::Vec::from([]))
    }
}

pub enum Shape {
    Circle {
        radius: f64,
    },
    Rect(u32, u32),
    Empty,
}
impl Shape {
    pub fn describe(&self)
        ->
            (&'static str,
            ::std::vec::Vec<(&'static str, &dyn ::core::fmt::Debug)>) {
        match self {
            Self::Circle { radius: field_0 } =>
                ("Shape::Circle { .. }",
                    ::std::vec::Vec::from([("radius",
                                    field_0 as &dyn ::core::fmt::Debug)])),
            Self::Rect(field_0, field_1) =>
                ("Shape::Rect(..)",
                    ::std::vec::Vec::from([("0",
                                    field_0 as &dyn ::core::fmt::Debug),
                                ("1", field_1 as &dyn ::core::fmt::Debug)])),
            Self::Empty => ("Shape::Empty", ::std::vec::Vec::from([])),
        }
    }
}
```
<!-- /cargo -->

The line breaks are rustc's pretty-printer, not `quote`'s. Read it for these:

- **`self.x` and `self.0` are the same kind of expression.** A tuple field is a field whose name is a number, written as a bare integer literal, and `syn::Index::from(0)` prints that literal. If you would rather not match on `field.ident` yourself, [`Fields::members()` ↗](https://docs.rs/crate/syn/3.0.6/source/src/data.rs#144) yields a `Member` per field, which prints as the name or as the `Index`.
- **`Vec::from([])` for `Origin`.** The unit struct went through the same code as the other two structs.
- **Three patterns for three variant shapes:** `Self::Circle { radius: field_0 }`, `Self::Rect(field_0, field_1)` and `Self::Empty`. A variant's fields have no `self.` path at all; the only way to reach them is to bind them. Binding the named field under a made-up name as well means one list of bindings serves every shape; binding it as `radius` would put a name the user chose into generated code, which is what [Absolute paths and hygiene](../absolute_paths_and_hygiene/README.md) is careful about.
- **`&self.x`, but `field_0` without a `&`.** `match self` matches a `&Shape`, so [match ergonomics](../../30_Pattern_Matching/match_ergonomics/README.md) bind `field_0` as a `&f64` already.

The derive never reads `input.generics`, so it is for types without type parameters; [Generics, lifetimes and `where`](../generics_lifetimes_and_where/README.md) is the page that handles the rest.

Nothing this page uses differs between syn 2 and syn 3: `Data`, `Fields`, `Variant`, `Index` and `Fields::members()` are unchanged.

## Three ways to get it wrong

### Named fields only

`DescribeNamedOnly` was written with `Point` in mind. It interpolates `field.ident`, which is an `Option<Ident>`:

<!-- file:demo/describe_mistakes/src/lib.rs -->
```rust title="demo/describe_mistakes/src/lib.rs"
//! Two derives that work on `struct Point { x: i32, y: i32 }` and break on
//! `struct Meters(f64)`. Both return each field's value, and nothing else.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Written with only named fields in mind. `field.ident` is an `Option<Ident>`,
/// and `quote` prints `None` as no tokens at all, so on a tuple struct this
/// generates `&self. as &dyn Debug`.
#[proc_macro_derive(DescribeNamedOnly)]
pub fn derive_named_only(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        panic!("structs only");
    };
    let idents = data.fields.iter().map(|field| &field.ident);
    quote! {
        impl #name {
            pub fn describe(&self) -> ::std::vec::Vec<&dyn ::core::fmt::Debug> {
                ::std::vec::Vec::from([#(&self.#idents as &dyn ::core::fmt::Debug),*])
            }
        }
    }
    .into()
}

/// Knows that tuple fields have positions, but interpolates the position as a
/// `usize`, and `quote` prints a `usize` with its type suffix: `0usize`.
#[proc_macro_derive(DescribeUsizeIndex)]
pub fn derive_usize_index(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        panic!("structs only");
    };
    let positions = 0..data.fields.len();
    quote! {
        impl #name {
            pub fn describe(&self) -> ::std::vec::Vec<&dyn ::core::fmt::Debug> {
                ::std::vec::Vec::from([#(&self.#positions as &dyn ::core::fmt::Debug),*])
            }
        }
    }
    .into()
}
```
<!-- /file -->

<!-- file:demo/named_only_on_a_tuple/src/main.rs -->
```rust title="demo/named_only_on_a_tuple/src/main.rs"
use describe_mistakes::DescribeNamedOnly;

#[derive(DescribeNamedOnly)]
struct Meters(f64);

fn main() {
    println!("{:?}", Meters(5.5).describe());
}
```
<!-- /file -->

<!-- cargo:describe_named_only_on_a_tuple -->
*Verified output of `cargo build -q -p describe_named_only_on_a_tuple`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: expected identifier, found keyword `as`
 --> named_only_on_a_tuple/src/main.rs:3:10
  |
3 | #[derive(DescribeNamedOnly)]
  |          ^^^^^^^^^^^^^^^^^ expected identifier, found keyword
  |
  = note: this error originates in the derive macro `DescribeNamedOnly` (in Nightly builds, run with -Z macro-backtrace for more info)

error: expected expression, found keyword `dyn`
 --> named_only_on_a_tuple/src/main.rs:3:10
  |
3 | #[derive(DescribeNamedOnly)]
  |          ^^^^^^^^^^^^^^^^^ expected expression
  |
  = note: this error originates in the derive macro `DescribeNamedOnly` (in Nightly builds, run with -Z macro-backtrace for more info)

error: proc-macro derive produced unparsable tokens
 --> named_only_on_a_tuple/src/main.rs:3:10
  |
3 | #[derive(DescribeNamedOnly)]
  |          ^^^^^^^^^^^^^^^^^

error: could not compile `describe_named_only_on_a_tuple` (bin "describe_named_only_on_a_tuple") due to 3 previous errors
```
<!-- /cargo -->

`quote` prints an `Option` that is `None` as [no tokens at all ↗](https://docs.rs/crate/quote/1.0.47/source/src/to_tokens.rs#114), so the macro wrote `&self. as &dyn ::core::fmt::Debug`. The user sees two errors about the keywords `as` and `dyn`, which they never wrote, and a third saying the derive produced unparsable tokens, all underlining the derive rather than the tuple struct. Nothing in the message says *tuple struct*; that is what a missing `Fields` arm costs.

### A `usize` as the field index

`DescribeUsizeIndex` in the same file knows tuple fields have positions, and interpolates the position straight from the `0..len` range:

<!-- file:demo/usize_as_an_index/src/main.rs -->
```rust title="demo/usize_as_an_index/src/main.rs"
use describe_mistakes::DescribeUsizeIndex;

#[derive(DescribeUsizeIndex)]
struct Meters(f64);

fn main() {
    println!("{:?}", Meters(5.5).describe());
}
```
<!-- /file -->

<!-- cargo:describe_usize_as_a_tuple_index -->
*Verified output of `cargo build -q -p describe_usize_as_an_index`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: suffixes on a tuple index are invalid
 --> usize_as_an_index/src/main.rs:3:10
  |
3 | #[derive(DescribeUsizeIndex)]
  |          ^^^^^^^^^^^^^^^^^^ invalid suffix `usize`
  |
  = note: this error originates in the derive macro `DescribeUsizeIndex` (in Nightly builds, run with -Z macro-backtrace for more info)

error: proc-macro derive produced unparsable tokens
 --> usize_as_an_index/src/main.rs:3:10
  |
3 | #[derive(DescribeUsizeIndex)]
  |          ^^^^^^^^^^^^^^^^^^

error: could not compile `describe_usize_as_an_index` (bin "describe_usize_as_an_index") due to 2 previous errors
```
<!-- /cargo -->

The label names the suffix. quote's `ToTokens` for `usize` [appends `Literal::usize_suffixed` ↗](https://docs.rs/crate/quote/1.0.47/source/src/to_tokens.rs#200), so position `0` became the token `0usize`: an ordinary literal in an expression, and not a field name after a `.`. `syn::Index` [appends `Literal::i64_unsuffixed` ↗](https://docs.rs/crate/syn/3.0.6/source/src/expr.rs#4208) instead, and prints `0`.

### A union

A union's fields share their storage, and nothing records which one was written last, so there is no correct `describe()` to generate. `Describe` refuses it with a `syn::Error` spanned on the `union` keyword:

<!-- file:demo/describe_a_union/src/main.rs -->
```rust title="demo/describe_a_union/src/main.rs"
use describe_derive::Describe;

#[derive(Describe)]
union IntOrFloat {
    i: u32,
    f: f32,
}

fn main() {
    let value = IntOrFloat { f: 1.0 };
    println!("{}", unsafe { value.i });
}
```
<!-- /file -->

<!-- cargo:describe_refuses_a_union -->
*Verified output of `cargo build -q -p describe_a_union`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: Describe cannot read a union: nothing records which field is set
 --> describe_a_union/src/main.rs:4:1
  |
4 | union IntOrFloat {
  | ^^^^^

error: could not compile `describe_a_union` (bin "describe_a_union") due to 1 previous error
```
<!-- /cargo -->

The error underlines `union`, the token that is wrong, where the other two underlined the derive's name. [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) compares that with the alternatives.

## If you are coming from another language

- **C and C++.** C has no way to walk a struct's members, so code that needs to is written from a second list of the fields — the X-macro pattern, where a `#define` lists every field once and is expanded several times. A derive reads the one list the compiler has. C's own union is the reason for the refusal above: a C union carries no record of which member was last written, and the idiom is a struct with a tag beside it, which is what a Rust `enum` is. C++ aggregates and `std::tuple` split the same way Rust's structs do, into members reached by name and elements reached by position (`std::get<0>`).
- **Python.** Every attribute has a name, so the tuple-struct problem does not arise: `dataclasses.fields()` lists them at run time, and a `namedtuple` answers to both `p.x` and `p[0]`. The nearest thing to an enum's variant patterns is `match` with class patterns, where `Point(1, 2)` matches positionally through `__match_args__` and `Point(x=1)` by keyword. What Rust changes is when this happens: the derive sees the shapes while compiling, so a shape it forgot is a compile error, not an `AttributeError` at run time.
- **Java.** A record's components always have names, and reflection's `getDeclaredFields()` finds them at run time. A Java `enum`'s constants are instances of one class, with one set of fields; the counterpart of `enum Shape` is a `sealed interface` with a `record` per variant, taken apart by record patterns in a `switch`. An annotation processor reading those records sees element kinds, as a derive sees `Fields`, and must handle every kind it can be applied to.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP.)* A structure's components are always named, but ABAP also reaches them by position: `ASSIGN COMPONENT sy-index OF STRUCTURE ls_row TO <lv_field>` walks them in order at run time, and `cl_abap_structdescr` lists their names and types. That is the run-time version of what `Describe` does while compiling.

## See also

- [Parsing with `syn`](../parsing_with_syn/README.md) — `DeriveInput`, `Data` and `Fields` from the start
- [Generating with `quote`](../generating_with_quote/README.md) — interpolation, repetition and `format_ident!`
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — the union refusal, and what the other ways of failing look like
- [Generics, lifetimes and `where`](../generics_lifetimes_and_where/README.md) — the part of `DeriveInput` this derive ignored
- [Variants that carry data](../../13_Enums/variants_that_carry_data/README.md) — the three variant shapes, from the user's side
- [What a union is](../../09_Advanced/what_a_union_is/README.md) — why there is nothing safe to read
- [Procedural macros](../README.md) — the chapter, in reading order

## Po polsku

Makro **derive** musi obsłużyć każdy kształt typu, a `syn` opisuje go przez `Data` i `Fields`. Struktura z **polami nazwanymi** (`Fields::Named`, `struct Point { x: i32, y: i32 }`) ma pola `self.x`; **struktura krotkowa** (*tuple struct*, `Fields::Unnamed`, `struct Meters(f64)`) ma pola `self.0`; **struktura jednostkowa** (*unit struct*, `Fields::Unit`) nie ma ich wcale. W enumie każdy wariant ma własne `Fields`, a do jego pól można dojść tylko przez wzorzec w `match` o tym samym kształcie: `Self::Circle { radius: field_0 }`, `Self::Rect(field_0, field_1)`, `Self::Empty`.

Dwie pułapki, obie sprawdzone kompilacją. `field.ident` to `Option<Ident>`, a `quote` wypisuje `None` jako nic — makro pisane tylko dla pól nazwanych generuje `&self. as …`, a kompilator skarży się na słowo kluczowe `as`, nie na strukturę krotkową. Numer pola podany jako `usize` zamienia się w token `0usize`, więc powstaje `self.0usize` i błąd *suffixes on a tuple index are invalid*; właściwy typ to `syn::Index`, który wypisuje samo `0`. Unii nie da się bezpiecznie odczytać, więc makro odrzuca ją błędem `syn::Error` wskazującym słowo `union`.

**Szukaj po polsku:** makro derive dla struktury krotkowej · `syn::Fields` · `syn::Index` · `rust derive tuple struct self.0` · `rust proc macro enum variants match`
