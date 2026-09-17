# Absolute paths and hygiene

**Level:** 301 · deep dive

**One line:** A derive's output is compiled in the user's module, among the user's names. So spell every path from a crate root with a leading `::`, as in `::core::fmt::Result` and even `::core::primitive::str`. Give each local variable the macro introduces `Span::mixed_site()`, or a user's field with the same name quietly takes the wrong value. And know what no span protects: a generated method collides with the user's method of the same name, and a user's `const` still captures a mixed-site `let`.

| The generated code says | In a module that | Recorded below |
|---|---|---|
| `fmt::Result`, `FromStr` | never imported `fmt` or `FromStr` | E0433 and E0405 |
| `Result<Self, Self::Err>` | has its own `Result` | E0107 |
| `core::fmt::Result` | has a module named `core` | E0433 |
| `::core::fmt::Result` | has all three | builds |
| `&'static str` | has its own `str` | E0053 and E0308 |
| `&'static ::core::primitive::str` | has its own `str` | builds |
| a local `text`, with `Span::call_site()` | binds a field named `text` | builds, prints the wrong value |
| a local `text`, with `Span::mixed_site()` | binds a field named `text` | builds, prints the right value |
| a method `summary`, with `Span::mixed_site()` | has its own `summary` method | E0592 |
| a local `text`, with `Span::mixed_site()` | has a `const text` | E0530 |

## Paths from the crate root

`#[derive(ByName)]` gives an enum whose variants carry no data a `Display` that prints each variant's name and a `FromStr` that parses the name back. The crate has three versions of it, identical except for how four names are spelled:

<!-- file:demo/by_name/src/lib.rs -->
```rust title="demo/by_name/src/lib.rs"
//! `#[derive(ByName)]` for an enum whose variants carry no data: `Display`
//! prints a variant's name, and `FromStr` parses the name back.
//!
//! The three derives generate the same code and differ only in how they spell
//! four names. Generated code lands in the user's module, so every name in it
//! is looked up there, among the user's own names.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Spelled the way the std docs spell `Display` and `FromStr`, which assumes a
/// module that imported `fmt` and `FromStr` and uses std's `Result`.
#[proc_macro_derive(ByNameAsWritten)]
pub fn by_name_as_written(input: TokenStream) -> TokenStream {
    expand(
        input,
        quote!(fmt),
        quote!(FromStr),
        quote!(Result),
        quote!(str),
    )
}

/// Full paths, but relative ones: `core` is looked up in the user's module first.
/// And `str` as everyone writes it.
#[proc_macro_derive(ByNameCore)]
pub fn by_name_core(input: TokenStream) -> TokenStream {
    expand(
        input,
        quote!(core::fmt),
        quote!(core::str::FromStr),
        quote!(core::result::Result),
        quote!(str),
    )
}

/// A leading `::` starts the path at a crate name, which no module can shadow.
#[proc_macro_derive(ByName)]
pub fn by_name(input: TokenStream) -> TokenStream {
    expand(
        input,
        quote!(::core::fmt),
        quote!(::core::str::FromStr),
        quote!(::core::result::Result),
        quote!(::core::primitive::str),
    )
}

fn expand(
    input: TokenStream,
    fmt: TokenStream2,
    from_str: TokenStream2,
    result: TokenStream2,
    str_type: TokenStream2,
) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(name, "ByName is for enums")
            .into_compile_error()
            .into();
    };
    let variants: Vec<_> = data.variants.iter().map(|v| &v.ident).collect();
    let names: Vec<_> = variants.iter().map(|v| v.to_string()).collect();
    quote! {
        impl #fmt::Display for #name {
            fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result {
                f.write_str(match self {
                    #(Self::#variants => #names,)*
                })
            }
        }

        impl #from_str for #name {
            type Err = &'static #str_type;

            fn from_str(s: &#str_type) -> #result<Self, Self::Err> {
                match s {
                    #(#names => #result::Ok(Self::#variants),)*
                    _ => #result::Err("no variant has that name"),
                }
            }
        }
    }
    .into()
}
```
<!-- /file -->

The last version, in a crate with the first three hazards from the next section at once:

<!-- file:demo/paths_app/src/main.rs -->
```rust title="demo/paths_app/src/main.rs"
//! All three hazards at once, under the fixed derive: no `use std::fmt`, a
//! `Result` of its own, and a module named `core`.

use by_name_derive::ByName;

/// The application's core logic, in a module named for it.
mod core {
    pub fn is_safe(method: &super::Method) -> bool {
        matches!(method, super::Method::Get)
    }
}

/// This crate's own `Result`, with the error type every function here returns.
type Result<T> = std::result::Result<T, &'static str>;

#[derive(ByName)]
enum Method {
    Get,
    Post,
    Delete,
}

fn parse(text: &str) -> Result<Method> {
    text.parse()
}

fn main() {
    for text in ["Get", "Delete", "PATCH"] {
        match parse(text) {
            Ok(method) => println!("{text:6} -> {method}, safe: {}", core::is_safe(&method)),
            Err(e) => println!("{text:6} -> error: {e}"),
        }
    }
}
```
<!-- /file -->

<!-- cargo:by_name_absolute_paths -->
*Verified output of `cargo run -q -p by_name_paths_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Get    -> Get, safe: true
Delete -> Delete, safe: false
PATCH  -> error: no variant has that name
```
<!-- /cargo -->

A path that starts with `::` is looked up among the crates the program depends on, never in the current module, so `::core` is the `core` crate whatever the module holds. `core` rather than `std`, because `core` is there in a `#![no_std]` crate too, and std's `fmt::Formatter` and `fmt::Result` are [re-exports of core's ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/fmt.rs#L609). [thiserror 2.0.20 ↗](https://docs.rs/crate/thiserror-impl/2.0.20/source/src/expand.rs#160) writes its `Display` impls the same way: `fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result`.

`Ok` and `Err` are spelled out too, as `::core::result::Result::Ok`. They are prelude names, like `Result`, and the second break below shows a user's item winning over a prelude name.

## Four spellings that break

### As the std docs write it

The documentation's examples for [`Display` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/fmt/mod.rs#L1153-L1161) and [`FromStr` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/str/traits.rs#L829-L843) write `fmt::Formatter`, `fmt::Result`, `FromStr` and `Result<Self, Self::Err>`, after a `use` at the top of each example. Generated code brings no `use` of its own, so it gets whatever the user's module imported:

<!-- file:demo/forgot_fmt/src/main.rs -->
```rust title="demo/forgot_fmt/src/main.rs"
use by_name_derive::ByNameAsWritten;

#[derive(ByNameAsWritten)]
enum Method {
    Get,
    Post,
}

fn main() {
    let parsed: Method = "Post".parse().unwrap();
    println!("{} {}", Method::Get, parsed);
}
```
<!-- /file -->

<!-- cargo:by_name_without_fmt_imported -->
*Verified output of `cargo build -q -p by_name_forgot_fmt`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0433]: cannot find module or crate `fmt` in this scope
 --> forgot_fmt/src/main.rs:3:10
  |
3 | #[derive(ByNameAsWritten)]
  |          ^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `fmt`
  |
  = help: if you wanted to use a crate named `fmt`, use `cargo add fmt` to add it to your `Cargo.toml`
  = note: this error originates in the derive macro `ByNameAsWritten` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this module
  |
1 + use std::fmt;
  |

error[E0405]: cannot find trait `FromStr` in this scope
 --> forgot_fmt/src/main.rs:3:10
  |
3 | #[derive(ByNameAsWritten)]
  |          ^^^^^^^^^^^^^^^ not found in this scope
  |
  = note: this error originates in the derive macro `ByNameAsWritten` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this trait
  |
1 + use std::str::FromStr;
  |

Some errors have detailed explanations: E0405, E0433.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `by_name_forgot_fmt` (bin "by_name_forgot_fmt") due to 2 previous errors
```
<!-- /cargo -->

Both errors point at `ByNameAsWritten`, and both suggest the user add a `use` to *their* file. That makes the errors go away, and leaves the bug in the macro for the next user.

### A crate with its own `Result`

This user has done what the compiler suggested, and also defines a `Result` of its own, the way `std::io` has `io::Result`:

<!-- file:demo/own_result/src/main.rs -->
```rust title="demo/own_result/src/main.rs"
use by_name_derive::ByNameAsWritten;
use std::fmt;
use std::str::FromStr;

/// This crate's own `Result`, with the error type every function here returns.
type Result<T> = std::result::Result<T, &'static str>;

#[derive(ByNameAsWritten)]
enum Method {
    Get,
    Post,
}

fn parse(text: &str) -> Result<Method> {
    Method::from_str(text)
}

fn main() {
    println!("{} {}", Method::Get, parse("Post").unwrap());
}
```
<!-- /file -->

<!-- cargo:by_name_with_its_own_result -->
*Verified output of `cargo build -q -p by_name_own_result`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0107]: type alias takes 1 generic argument but 2 generic arguments were supplied
 --> own_result/src/main.rs:8:10
  |
8 | #[derive(ByNameAsWritten)]
  |          ^^^^^^^^^^^^^^^ expected 1 generic argument
  |
note: type alias defined here, with 1 generic parameter: `T`
 --> own_result/src/main.rs:6:6
  |
6 | type Result<T> = std::result::Result<T, &'static str>;
  |      ^^^^^^ -
  = note: this error originates in the derive macro `ByNameAsWritten` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0107`.
error: could not compile `by_name_own_result` (bin "by_name_own_result") due to 1 previous error
```
<!-- /cargo -->

The generated `fn from_str(s: &str) -> Result<Self, Self::Err>` meant std's `Result` and found the user's one-parameter alias. Names from the prelude are the fallback, used only when the module has no item of that name, and that holds for every prelude name generated code leans on, not just `Result`.

### `core::`, without the leading `::`

`ByNameCore` spells every path out in full, and still starts looking in the user's module:

<!-- file:demo/module_named_core/src/main.rs -->
```rust title="demo/module_named_core/src/main.rs"
use by_name_derive::ByNameCore;

/// The application's core logic, in a module named for it.
mod core {
    pub fn is_safe(method: &super::Method) -> bool {
        matches!(method, super::Method::Get)
    }
}

#[derive(ByNameCore)]
enum Method {
    Get,
    Post,
}

fn main() {
    println!("{} {}", Method::Post, core::is_safe(&Method::Post));
}
```
<!-- /file -->

<!-- cargo:by_name_with_a_module_named_core -->
*Verified output of `cargo build -q -p by_name_module_named_core`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0433]: cannot find `fmt` in `core`
  --> module_named_core/src/main.rs:10:10
   |
10 | #[derive(ByNameCore)]
   |          ^^^^^^^^^^ could not find `fmt` in `core`
   |
   = note: this error originates in the derive macro `ByNameCore` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these modules
   |
 1 + use std::fmt;
   |
 1 + use ::core::fmt;
   |

error[E0433]: cannot find `str` in `core`
  --> module_named_core/src/main.rs:10:10
   |
10 | #[derive(ByNameCore)]
   |          ^^^^^^^^^^ could not find `str` in `core`
   |
   = note: this error originates in the derive macro `ByNameCore` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these modules
   |
 1 + use std::str;
   |
 1 + use ::core::str;
   |

error[E0433]: cannot find `result` in `core`
  --> module_named_core/src/main.rs:10:10
   |
10 | #[derive(ByNameCore)]
   |          ^^^^^^^^^^ could not find `result` in `core`
   |
   = note: this error originates in the derive macro `ByNameCore` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these modules
   |
 1 + use std::result;
   |
 1 + use ::core::result;
   |

error[E0433]: cannot find `result` in `core`
  --> module_named_core/src/main.rs:10:10
   |
10 | #[derive(ByNameCore)]
   |          ^^^^^^^^^^ could not find `result` in `core`
   |
   = note: this error originates in the derive macro `ByNameCore` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these items
   |
 1 + use std::fmt::Result;
   |
 1 + use std::io::Result;
   |
 1 + use std::result::Result;
   |
 1 + use std::thread::Result;
   |
   = and 2 other candidates

For more information about this error, try `rustc --explain E0433`.
error: could not compile `by_name_module_named_core` (bin "by_name_module_named_core") due to 4 previous errors
```
<!-- /cargo -->

`core::fmt` looked for `core` among the module's own items first, found `mod core`, and looked inside it for `fmt`. The first three errors each suggest importing from `::core`, which is the spelling the macro needed.

### `str`, which is a name too

`str` is not a keyword. It is a primitive type's name, and an item of the same name in the user's module wins over it, as the user's `Result` did:

<!-- file:demo/own_str_core/src/main.rs -->
```rust title="demo/own_str_core/src/main.rs"
use by_name_derive::ByNameCore;

/// This crate's own `str`. Unusual, and legal: `str` is a name, not a keyword.
#[allow(non_camel_case_types)]
struct str(String);

#[derive(ByNameCore)]
enum Method {
    Get,
    Post,
}

fn main() {
    let custom = str("PURGE".to_string());
    println!("{} {} {}", Method::Get, Method::Post, custom.0);
    println!("{}", "Post".parse::<Method>().is_ok());
}
```
<!-- /file -->

<!-- cargo:by_name_with_its_own_str -->
*Verified output of `cargo build -q --message-format=short -p by_name_own_str_core`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
own_str_core/src/main.rs:7:10: error[E0053]: method `from_str` has an incompatible type for trait: expected `str`, found a different `str`
own_str_core/src/main.rs:7:10: error[E0308]: mismatched types: expected `str`, found a different `str`
own_str_core/src/main.rs:7:10: error[E0308]: mismatched types: expected `str`, found a different `str`, arguments to this enum variant are incorrect
error: could not compile `by_name_own_str_core` (bin "by_name_own_str_core") due to 3 previous errors
```
<!-- /cargo -->

*Expected `str`, found a different `str`*: the trait wants the primitive, and the generated `fn from_str(s: &str)` named the user's struct. This run is recorded with `--message-format=short`, one line per error, because the full transcript carries a note pointing into the standard library's source, and that path differs between machines. The same program under `ByName`, which writes `&'static ::core::primitive::str`:

<!-- file:demo/own_str/src/main.rs -->
```rust title="demo/own_str/src/main.rs"
use by_name_derive::ByName;

/// This crate's own `str`. Unusual, and legal: `str` is a name, not a keyword.
#[allow(non_camel_case_types)]
struct str(String);

#[derive(ByName)]
enum Method {
    Get,
    Post,
}

fn main() {
    let custom = str("PURGE".to_string());
    println!("{} {} {}", Method::Get, Method::Post, custom.0);
    println!("{}", "Post".parse::<Method>().is_ok());
}
```
<!-- /file -->

<!-- cargo:by_name_primitive_str -->
*Verified output of `cargo run -q -p by_name_own_str` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Get Post PURGE
true
```
<!-- /cargo -->

[A `StateMachine` derive](../a_state_machine_derive/README.md) writes `::core::primitive::str` for the same reason.

## Hygiene: a local variable the user can see

Paths are one half of what can clash; the other is the names generated code makes up. `#[derive(Summary)]` binds the struct's fields under their own names, then builds its result in a local variable called `text`. The two versions differ only in the span given to the two identifiers the macro invents, `text` and the method name `summary`:

<!-- file:demo/summary/src/lib.rs -->
```rust title="demo/summary/src/lib.rs"
//! `#[derive(Summary)]`: a `summary()` method that lists every field as
//! `[name = value]`.
//!
//! The generated method binds the user's fields by name, then builds its result
//! in a local variable of its own, `text`. The two derives differ in one thing:
//! the span carried by the identifiers the macro makes up, `text` and `summary`.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, parse_macro_input};

/// Both identifiers resolve as if the user had typed them where `#[derive]` is.
#[proc_macro_derive(SummaryCallSite)]
pub fn summary_call_site(input: TokenStream) -> TokenStream {
    expand(input, Span::call_site())
}

/// `text` becomes a local variable only the macro's own tokens can name.
#[proc_macro_derive(Summary)]
pub fn summary(input: TokenStream) -> TokenStream {
    expand(input, Span::mixed_site())
}

fn expand(input: TokenStream, span: Span) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &input.data
    else {
        return syn::Error::new_spanned(name, "Summary needs named fields")
            .into_compile_error()
            .into();
    };
    // The user's field names, carrying the spans they have in the user's source.
    let fields: Vec<&Ident> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();
    let labels = fields.iter().map(|f| f.to_string());
    let text = Ident::new("text", span);
    let summary = Ident::new("summary", span);
    quote! {
        impl #name {
            pub fn #summary(&self) -> ::std::string::String {
                let Self { #(#fields),* } = self;
                let mut #text = ::std::string::String::new();
                #(#text.push_str(&::std::format!("[{} = {}]", #labels, #fields));)*
                #text
            }
        }
    }
    .into()
}
```
<!-- /file -->

The same struct under each:

<!-- file:demo/comments/src/lib.rs -->
```rust title="demo/comments/src/lib.rs"
//! The same struct twice: once under each version of the derive.

pub mod call_site {
    #[derive(summary_derive::SummaryCallSite)]
    pub struct Comment {
        pub author: String,
        pub text: String,
    }
}

pub mod mixed_site {
    #[derive(summary_derive::Summary)]
    pub struct Comment {
        pub author: String,
        pub text: String,
    }
}
```
<!-- /file -->

<!-- file:demo/hygiene_app/src/main.rs -->
```rust title="demo/hygiene_app/src/main.rs"
use summary_comments::{call_site, mixed_site};

fn main() {
    let (author, text) = ("Ada".to_string(), "Nice page".to_string());
    let before = call_site::Comment {
        author: author.clone(),
        text: text.clone(),
    };
    let after = mixed_site::Comment { author, text };
    println!("call_site:  {}", before.summary());
    println!("mixed_site: {}", after.summary());
}
```
<!-- /file -->

<!-- cargo:summary_call_site_or_mixed_site -->
*Verified output of `cargo run -q -p summary_hygiene_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
call_site:  [author = Ada][text = [author = Ada]]
mixed_site: [author = Ada][text = Nice page]
```
<!-- /cargo -->

`call_site` printed the summary so far where the comment's text belongs. Yet the expansion is the same text for both structs. `cargo expand` in `demo/comments` would show it; the key was recorded with the command it runs underneath, `-Zunpretty=expanded`:

<!-- cargo:summary_expanded -->
*Verified output of `cargo rustc -q -p summary_comments --profile=check -- -Zunpretty=expanded` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
#![feature(prelude_import)]
//! The same struct twice: once under each version of the derive.
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;

pub mod call_site {
    pub struct Comment {
        pub author: String,
        pub text: String,
    }
    impl Comment {
        pub fn summary(&self) -> ::std::string::String {
            let Self { author, text } = self;
            let mut text = ::std::string::String::new();
            text.push_str(&
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("[{0} = {1}]", "author",
                                    author))
                        }));
            text.push_str(&::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("[{0} = {1}]", "text",
                                    text))
                        }));
            text
        }
    }
}
pub mod mixed_site {
    pub struct Comment {
        pub author: String,
        pub text: String,
    }
    impl Comment {
        pub fn summary(&self) -> ::std::string::String {
            let Self { author, text } = self;
            let mut text = ::std::string::String::new();
            text.push_str(&::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("[{0} = {1}]", "author",
                                    author))
                        }));
            text.push_str(&::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("[{0} = {1}]", "text",
                                    text))
                        }));
            text
        }
    }
}
```
<!-- /cargo -->

Read as source, both are wrong: `let mut text` shadows the `text` bound on the line above, so `format!` receives the half-built summary. That is exactly what the `call_site` version does. [`quote!` gives every token it writes `Span::call_site()` ↗](https://docs.rs/quote/1.0.47/quote/macro.quote.html#hygiene), and a call-site identifier resolves as if the user had typed it at the `#[derive]`. Nothing fails; the build's only complaint is about the field the user did use:

<!-- cargo:summary_call_site_warning -->
*Verified output of `cargo build -q -p summary_comments` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
warning: unused variable: `text`
 --> comments/src/lib.rs:7:13
  |
7 |         pub text: String,
  |             ^^^^ help: try ignoring the field: `text: _`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
```
<!-- /cargo -->

With `Span::mixed_site()`, the two `text`s are different variables with the same spelling. The `text` handed to `format!` is the identifier from the struct definition, carrying the user's span, so it names the user's binding; the macro's `text` is invisible to it. Hygiene lives in the spans, not in the text, which is why the expansion cannot show it. And `after.summary()` in `main` still works, although that `summary` has a mixed-site span too: user code can call a method the macro named.

## What `mixed_site` does not protect

The [`Span::mixed_site` docs ↗](https://doc.rust-lang.org/proc_macro/struct.Span.html#method.mixed_site) describe it as `macro_rules` hygiene: local variables, labels and `$crate` resolve at the macro's definition site, and everything else at the call site. Two runs show what *everything else* includes.

A method is an item, not a local variable. This user already had a `summary` of their own:

<!-- file:demo/own_summary_method/src/main.rs -->
```rust title="demo/own_summary_method/src/main.rs"
use summary_derive::Summary;

#[derive(Summary)]
struct Comment {
    author: String,
    text: String,
}

impl Comment {
    /// The user's own `summary`, written before anyone derived one.
    fn summary(&self) -> String {
        format!("{}: {}", self.author, self.text)
    }
}

fn main() {
    let comment = Comment {
        author: "Ada".to_string(),
        text: "Nice page".to_string(),
    };
    println!("{}", comment.summary());
}
```
<!-- /file -->

<!-- cargo:summary_method_name_collides -->
*Verified output of `cargo build -q -p summary_own_method`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0592]: duplicate definitions with name `summary`
  --> own_summary_method/src/main.rs:3:10
   |
 3 | #[derive(Summary)]
   |          ^^^^^^^ duplicate definitions for `summary`
...
11 |     fn summary(&self) -> String {
   |     --------------------------- other definition for `summary`
   |
   = note: this error originates in the derive macro `Summary` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0034]: multiple applicable items in scope
  --> own_summary_method/src/main.rs:21:28
   |
21 |     println!("{}", comment.summary());
   |                            ^^^^^^^ multiple `summary` found
   |
note: candidate #1 is defined in an impl for the type `Comment`
  --> own_summary_method/src/main.rs:3:10
   |
 3 | #[derive(Summary)]
   |          ^^^^^^^
note: candidate #2 is defined in an impl for the type `Comment`
  --> own_summary_method/src/main.rs:11:5
   |
11 |     fn summary(&self) -> String {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the derive macro `Summary` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0034, E0592.
For more information about an error, try `rustc --explain E0034`.
error: could not compile `summary_own_method` (bin "summary_own_method") due to 2 previous errors
```
<!-- /cargo -->

The derived `summary` has a mixed-site span and collides anyway, and the call in `main` cannot choose between the two. By the docs' rule the same holds for every item a derive adds, since items are *everything else*: its name has to be one the user will not also choose, and belongs in the derive's documentation.

A local variable is protected from the user's local variables. But a `let` pattern is checked against constants in scope, and a constant is an item:

<!-- file:demo/const_named_text/src/main.rs -->
```rust title="demo/const_named_text/src/main.rs"
use summary_derive::Summary;

/// Unconventional, since constants are usually upper case, and legal.
#[allow(non_upper_case_globals)]
const text: &str = "untitled";

#[derive(Summary)]
struct Comment {
    author: String,
}

fn main() {
    let comment = Comment {
        author: "Ada".to_string(),
    };
    println!("{} {}", text, comment.summary());
}
```
<!-- /file -->

<!-- cargo:summary_local_meets_a_const -->
*Verified output of `cargo build -q -p summary_const_named_text`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0530]: let bindings cannot shadow constants
 --> const_named_text/src/main.rs:7:10
  |
5 | const text: &str = "untitled";
  | ------------------------------ the constant `text` is defined here
6 |
7 | #[derive(Summary)]
  |          ^^^^^^^ cannot be named the same as a constant
  |
  = note: this error originates in the derive macro `Summary` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0530`.
error: could not compile `summary_const_named_text` (bin "summary_const_named_text") due to 1 previous error
```
<!-- /cargo -->

This `Comment` has no field named `text`, so the only `text` binding is the macro's mixed-site local. It still found the user's `const text`, and a `let` binding cannot shadow a constant. What keeps this rare is the convention that constants are upper case, not the span.

So for the names a macro makes up, from strongest:

- **No invented name at all.** `self.author` names nothing new, so there is nothing to collide. An enum's fields have no such path and have to be bound: [Every struct and enum shape](../every_struct_and_enum_shape/README.md) binds them as `field_0`, `field_1`, and the next two points apply to those.
- **`Span::mixed_site()` on every local variable,** as `Summary` does: `Ident::new("text", Span::mixed_site())`, or `quote_spanned!(Span::mixed_site()=> …)` around the code that declares and uses them. It keeps the local apart from the user's variables and fields, not from a lower-case `const`, and it does nothing for an item.
- **A name nobody types,** for locals and items alike. thiserror calls its formatter `__formatter`, in the line quoted earlier. That makes a clash unlikely, not impossible.

The `f` and `s` parameters in `ByName` are call-site identifiers too. Nothing the user wrote is placed inside those two function bodies, so no user variable can meet them; a parameter is a pattern like a `let`, though, so the constant rule still reaches them.

## If you are coming from another language

- **C and C++.** A function-like `#define` has both problems, with neither fix. A macro that calls `abs` gets whatever `abs` means where it is expanded, and the C++ defence is the same leading `::`, as in `::std::abs`. The classic `SWAP(a, b)` that declares `int tmp` inside `do { … } while (0)` compiles and breaks when called as `SWAP(tmp, x)`, because the macro's `tmp` captures the caller's: the preprocessor has no hygiene at all, and the only defence is a name like `_swap_tmp`, the `__formatter` approach. `Span::mixed_site()` is what the preprocessor never had.
- **Python.** Code generation is usually source text run through `exec`, and name capture is decided by the namespaces you hand it. `dataclasses` is the case in the standard library, and its source has both of this page's worries. It builds `__init__` as text, executed against the globals of the user's module, so it passes `object` in under the name `__dataclass_builtins_object__` rather than trusting what `object` means there. And it does not always call the first parameter `self`: a comment in `_field_assign` says not to hard-code it, *"since that might be a field name"*. Python has no warning when capture does happen; the program prints the wrong thing.
- **Java.** An annotation processor writes a whole `.java` file, so it has its own imports and no user variables in scope. The path problem remains: the generated file joins the user's package, and a class in that package named `Result` or `String` wins over an unqualified name. Generators such as JavaPoet deal with it by writing a fully qualified name wherever a simple one would clash, the Java spelling of `::core::`. A generated class's own name is another matter, as a derive's generated method is: if the user has a class by that name, the two collide.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP.)* A `DEFINE` macro is text substituted into the program that uses it, so every name in it resolves there, and a variable the macro declares is declared in the caller's scope. The defence is a prefix nobody else uses; nothing like `Span::mixed_site()` exists.

## See also

- [Every struct and enum shape](../every_struct_and_enum_shape/README.md) — the derive that binds enum fields under generated names
- [Expanding `thiserror`](../expanding_thiserror/README.md) — a real derive's output, with `::core::` throughout
- [A `StateMachine` derive](../a_state_machine_derive/README.md) — a derive that invents no names, and writes `::core::primitive::str`
- [The re-export pattern](../the_reexport_pattern/README.md) — naming your own crate from generated code, which `::core` cannot do
- [Generating with `quote`](../generating_with_quote/README.md) — `quote_spanned!` and `format_ident!`
- [Hygiene](../../38_Declarative_Macros/hygiene/README.md) — the fixed rule of `macro_rules!`, which `Span::mixed_site()` copies
- [Bringing names in with `use`](../../27_Modules/the_use_declaration/README.md) — what the generated code was missing when it said `fmt::`
- [Procedural macros](../README.md) — the chapter, in reading order

## Po polsku

Kod wygenerowany przez makro derive trafia do **modułu użytkownika** i tam jest rozwiązywany — wśród nazw, które zadeklarował użytkownik. Stąd cztery pułapki, każda sprawdzona kompilacją: `fmt::Result` nie działa, jeśli użytkownik nie zaimportował `fmt` (a kompilator radzi *użytkownikowi* dopisać `use`); `Result<Self, Self::Err>` trafia na własny alias `Result<T>` użytkownika (E0107); `core::fmt::Result` zawodzi, gdy w module istnieje `mod core`; a nawet `str` to tylko nazwa, którą przesłania `struct str`. Rozwiązaniem jest **ścieżka bezwzględna** z wiodącym `::` — `::core::fmt::Result`, `::core::primitive::str` — która zaczyna się od nazwy crate'a, więc żaden moduł jej nie przesłoni.

Druga połowa to **higiena** (*hygiene*). `quote!` nadaje każdemu tokenowi `Span::call_site()`, więc zmienna lokalna makra zachowuje się tak, jakby użytkownik sam ją napisał: jeśli struktura ma pole `text`, a makro deklaruje własne `text`, program kompiluje się i wypisuje złą wartość, a jedyną wskazówką jest ostrzeżenie o nieużywanej zmiennej. Z `Span::mixed_site()` to dwie różne zmienne, choć `cargo expand` pokazuje dla obu wersji identyczny tekst. Ale `mixed_site` chroni tylko zmienne lokalne, etykiety i `$crate`: wygenerowana metoda `summary` nadal koliduje z metodą użytkownika (E0592), a lokalne `let text` nadal trafia na stałą `const text` (E0530).

**Szukaj po polsku:** higiena makr w Ruscie · ścieżki bezwzględne w makrach · `Span::mixed_site` · `rust proc macro hygiene` · `rust derive ::core::primitive::str`
