# Three kinds of procedural macro

**Level:** 201 · working knowledge

**One line:** A procedural macro is a Rust function the compiler calls while it compiles *your* crate, handing it tokens and taking tokens back. A **derive** gets the item and can only add to it, a **function-like** macro gets whatever sits between its delimiters, and an **attribute** gets its arguments plus the item, and returns the item's replacement.

| Kind | You write | The macro is declared with | It receives | It returns |
|---|---|---|---|---|
| Derive | `#[derive(Serialize)]` on a struct, enum or union | `#[proc_macro_derive(Serialize)]` | the item | items added **after** the item, which stays as written |
| Function-like | `paste!(…)`, `name![…]`, `name!{…}` | `#[proc_macro]` | the tokens between the delimiters | whatever the call becomes |
| Attribute | `#[tokio::main]` on any item | `#[proc_macro_attribute]` | its own arguments, and the item | the item's **replacement** |

All three have the same shape underneath: `TokenStream` in, `TokenStream` out. The attribute is the only one that takes two.

## One of each, printing what it was given

This crate has one macro of each kind, plus two that break on purpose. It uses nothing but the compiler's own `proc_macro` crate: each macro turns the tokens it received into a string, and returns code that carries that string.

<!-- file:demo/show_tokens/src/lib.rs -->
```rust title="demo/show_tokens/src/lib.rs"
//! One procedural macro of each kind, and two that misbehave on purpose.
//!
//! None of them uses `syn` or `quote`. Each turns the tokens it was handed into
//! a string and returns code that carries the string, so the program that uses
//! them can print exactly what a macro receives.

use proc_macro::{TokenStream, TokenTree};

/// A derive receives the item it is attached to, and returns items to add after it.
#[proc_macro_derive(ShowTokens)]
pub fn derive_show_tokens(item: TokenStream) -> TokenStream {
    let name = name_after_keyword(&item);
    let received = item.to_string();
    format!("impl {name} {{ pub const RECEIVED: &str = {received:?}; }}")
        .parse()
        .unwrap()
}

/// A function-like macro receives what sits between its delimiters, and returns
/// what the call becomes.
#[proc_macro]
pub fn show_tokens(input: TokenStream) -> TokenStream {
    format!("{:?}", input.to_string()).parse().unwrap()
}

/// An attribute receives its own arguments and the item, and returns the item's
/// replacement. This one puts the item back and adds two constants beside it.
#[proc_macro_attribute]
pub fn show_attr(args: TokenStream, item: TokenStream) -> TokenStream {
    let name = name_after_keyword(&item).to_uppercase();
    let (args, received) = (args.to_string(), item.to_string());
    format!("{item} pub const {name}_ARGS: &str = {args:?}; pub const {name}_ITEM: &str = {received:?};")
        .parse()
        .unwrap()
}

/// A derive that hands its input straight back. The compiler keeps the original
/// item too, so the program ends up with two.
#[proc_macro_derive(Reemit)]
pub fn derive_reemit(item: TokenStream) -> TokenStream {
    item
}

/// An attribute that returns nothing, so the item is replaced by nothing.
#[proc_macro_attribute]
pub fn swallow(_args: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// The identifier after `struct`, `enum` or `fn`.
fn name_after_keyword(item: &TokenStream) -> String {
    let mut tokens = item.clone().into_iter();
    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(keyword) = token
            && matches!(keyword.to_string().as_str(), "struct" | "enum" | "fn")
            && let Some(TokenTree::Ident(name)) = tokens.next()
        {
            return name.to_string();
        }
    }
    panic!("expected a struct, an enum or a fn")
}
```
<!-- /file -->

Building code by formatting a string and calling `.parse()` works, and it is how this page stays free of dependencies. It is also the thing `quote` exists to replace: a typo in that string is a parse error with no useful location. [Generating with `quote`](../generating_with_quote/README.md) does it properly.

A second crate uses all three:

<!-- file:demo/app/src/main.rs -->
```rust title="demo/app/src/main.rs"
use show_tokens::{ShowTokens, show_attr, show_tokens};

/// A point on a plane.
#[derive(Debug, ShowTokens)]
struct Point {
    x: i32,
    y: i32,
}

#[show_attr(times = 3)]
fn greet(name: &str) -> String {
    format!("hello, {name}")
}

/// Every line of `tokens`, indented under its label.
fn show(label: &str, tokens: &str) {
    println!("   {label}");
    for line in tokens.lines() {
        println!("      {line}");
    }
}

fn main() {
    println!("1. A derive receives the item, and adds to it");
    show("received", Point::RECEIVED);
    let p = Point { x: 1, y: 2 };
    println!("   the struct is still there: {p:?}, x + y = {}", p.x + p.y);
    println!();
    println!("2. A function-like macro receives what is between its delimiters");
    show("received", show_tokens!(GET /users/{id} => list_users));
    println!();
    println!("3. An attribute receives its arguments and the item, and replaces the item");
    show("arguments", GREET_ARGS);
    show("item", GREET_ITEM);
    println!("   the replacement put greet back: {}", greet("Ada"));
}
```
<!-- /file -->

<!-- cargo:three_kinds -->
*Verified output of `cargo run -q -p app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
1. A derive receives the item, and adds to it
   received
      /// A point on a plane.
      struct Point { x: i32, y: i32, }
   the struct is still there: Point { x: 1, y: 2 }, x + y = 3

2. A function-like macro receives what is between its delimiters
   received
      GET /users/{id} => list_users

3. An attribute receives its arguments and the item, and replaces the item
   arguments
      times = 3
   item
      fn greet(name: &str) -> String { format!("hello, {name}") }
   the replacement put greet back: hello, Ada
```
<!-- /cargo -->

Four things in that output are worth reading twice:

- **The derive never sees `#[derive(Debug, ShowTokens)]`.** The compiler removes the `derive` attribute before calling the macro. The doc comment stays: to a macro it is an attribute like any other, and [Tokens and token streams](../tokens_and_token_streams/README.md) takes it apart.
- **The derive's output is added, not substituted.** `Point` is still an ordinary struct afterwards, `Debug` still works on it, and `Point::RECEIVED` exists because the macro returned an `impl` block.
- **`GET /users/{id} => list_users` is not Rust.** A function-like macro's input only has to be *tokens*: identifiers, punctuation, literals, and brackets that balance. What they mean is up to the macro — the subject of [Parsing arbitrary tokens](../parsing_arbitrary_tokens/README.md).
- **The attribute saw `format!("hello, {name}")` unexpanded.** Macros inside an item are expanded after the macro on the item has run, so an attribute macro works on the source as you wrote it.

The text is rebuilt from tokens, not copied from the file: `struct Point` was four lines in `main.rs` and came back as one. The [`Display` docs ↗](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html#impl-Display-for-TokenStream) warn that *"the exact form of the output is subject to change"*, and the key above was recorded with rustc 1.98.0. So never match substrings of it inside a macro; walk the tokens instead, as `name_after_keyword` does.

## The difference that matters: add, or replace

A derive cannot change the item it is attached to. The compiler keeps the original, so a derive that hands the item back produces a second copy:

<!-- file:demo/derive_twice/src/main.rs -->
```rust title="demo/derive_twice/src/main.rs"
use show_tokens::Reemit;

#[derive(Reemit)] // Reemit returns its input unchanged
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p.x + p.y);
}
```
<!-- /file -->

<!-- cargo:a_derive_cannot_replace -->
*Verified output of `cargo build -q -p derive_twice`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0428]: the name `Point` is defined multiple times
 --> derive_twice/src/main.rs:4:1
  |
4 | struct Point {
  | ^^^^^^^^^^^^
  | |
  | `Point` redefined here
  | previous definition of the type `Point` here
  |
  = note: `Point` must be defined only once in the type namespace of this module

For more information about this error, try `rustc --explain E0428`.
error: could not compile `derive_twice` (bin "derive_twice") due to 1 previous error
```
<!-- /cargo -->

The error points at the same line twice. The copied tokens still carry the *spans*, the source locations, of the original, so rustc reports both definitions at `struct Point`.

An attribute has the opposite contract: whatever it returns **is** the item. Return nothing, and the function is gone:

<!-- file:demo/swallowed/src/main.rs -->
```rust title="demo/swallowed/src/main.rs"
use show_tokens::swallow;

#[swallow] // swallow returns an empty TokenStream
fn greet() -> &'static str {
    "hello"
}

fn main() {
    println!("{}", greet());
}
```
<!-- /file -->

<!-- cargo:an_attribute_can_remove -->
*Verified output of `cargo build -q -p swallowed`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0425]: cannot find function `greet` in this scope
 --> swallowed/src/main.rs:9:20
  |
9 |     println!("{}", greet());
  |                    ^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `swallowed` (bin "swallowed") due to 1 previous error
```
<!-- /cargo -->

That is why every attribute macro that means to keep the item has to put it back. The `show_attr` macro above starts its output with `{item}` for exactly that reason, and [Re-emit the item on error](../re_emit_on_error/README.md) is about the case where the macro fails halfway.

## In the wild

Each row is the macro's declaration in the crate's published source:

| Crate | Declaration | You write |
|---|---|---|
| [`serde_derive` 1.0.229 ↗](https://docs.rs/crate/serde_derive/1.0.229/source/src/lib.rs#113) | `#[proc_macro_derive(Serialize, attributes(serde))]` | `#[derive(Serialize)]`, then `#[serde(rename = "id")]` on fields |
| [`thiserror-impl` 2.0.20 ↗](https://docs.rs/crate/thiserror-impl/2.0.20/source/src/lib.rs#39) | `#[proc_macro_derive(Error, attributes(backtrace, error, from, source))]` | `#[derive(Error)]` and `#[error("…")]` |
| [`clap_derive` 4.6.7 ↗](https://docs.rs/crate/clap_derive/4.6.7/source/src/lib.rs#54) | `#[proc_macro_derive(Parser, attributes(clap, structopt, command, arg, group))]` | `#[derive(Parser)]` |
| [`paste` 1.0.15 ↗](https://docs.rs/crate/paste/1.0.15/source/src/lib.rs#164) | `#[proc_macro] pub fn paste(input: TokenStream) -> TokenStream` | `paste! { … }` |
| [`tokio-macros` 2.7.0 ↗](https://docs.rs/crate/tokio-macros/2.7.0/source/src/lib.rs#311) | `#[proc_macro_attribute] pub fn main(args: TokenStream, item: TokenStream) -> TokenStream` | `#[tokio::main]` |

The `attributes(…)` list in a derive's declaration names its **helper attributes**. `#[serde(rename = "id")]` does nothing by itself; it is there for `Serialize` to read, and the compiler accepts it only because the derive declared it. [Helper attributes by hand](../helper_attributes_by_hand/README.md) writes one.

Notice the crate names, too. The macros live in `serde_derive` and `thiserror-impl`, but you depend on `serde` and `thiserror`. A proc-macro crate can export nothing except macros, so the trait and the derive ship in two crates and one re-exports the other — [the re-export pattern](../the_reexport_pattern/README.md).

## The trap: the macros you have used may not be procedural

`#[derive(Debug)]`, `#[test]` and `format_args!` look exactly like a derive, an attribute and a function-like macro, and none of them is a procedural macro. They are built into the compiler. Here is `Debug`'s derive in [`core/src/fmt/mod.rs` at 1.98.0 ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/fmt/mod.rs#L1089-L1095):

```rust,ignore
    /// Derive macro generating an impl of the trait `Debug`.
    #[rustc_builtin_macro]
    #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
    #[allow_internal_unstable(core_intrinsics, fmt_helpers_for_derive)]
    pub macro Debug($item:item) {
        /* compiler built-in */
    }
```

`#[test]` has the same `#[rustc_builtin_macro]` body in [`core/src/macros/mod.rs` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/macros/mod.rs#L1790-L1795), and so does `format_args!`. `println!` and `vec!` are neither: they are ordinary `macro_rules!` macros in std and alloc, and `println!` hands its arguments to the built-in `format_args_nl!`.

So *you have used all three kinds already* is only true if you have used a crate such as `serde`, `thiserror`, `clap` or `tokio`. The syntax is the same either way, which is the point of the design: a crate can add a derive that is used exactly like a built-in one.

## If you are coming from another language

- **C and C++.** `#define` substitutes tokens by fixed rules; it cannot run code or loop over a struct's fields. A procedural macro is a Rust program with the item's tokens as input, so it can do both. The price is that it must be compiled before the crate that uses it, which is why it lives in a crate of its own. The macro also sees tokens, never types: like the preprocessor, it runs before type checking, so it cannot ask what type `i32` is or whether `Point` implements a trait.
- **Python.** An attribute macro is a decorator run by the compiler: a decorator receives a function object and returns whatever should take its name, and so does `#[show_attr]`, except that it receives the function's *tokens*. `@dataclass` is the nearest thing to a derive, generating `__init__` and `__repr__` from the fields. The difference is the one this page is about: `@dataclass` adds methods to the class itself and returns it, while a derive can never touch the struct — it can only add an `impl` beside it.
- **Java.** An annotation processor ([JSR 269 ↗](https://jcp.org/en/jsr/detail?id=269)) runs inside `javac`, sees the annotated declarations, and may generate *new* source files but not modify the class it was given. That is the derive contract exactly. Lombok, which does change the class, reaches into `javac`'s internal syntax tree to do it; in Rust that job is what attribute macros are for, and it is a supported API.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP.)* `DEFINE … END-OF-DEFINITION` is a textual macro with placeholders `&1` … `&9`, closer to C's `#define` than to anything on this page: it substitutes, and runs no code of its own.

## See also

- [Macros](../../25_Control_Flow/macros/README.md) — what the `!` means at a call site, before anyone writes a macro
- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — a `macro_rules!` macro that prints its own source text
- [`Debug` and `Display`](../../15_First_Programs/debug_vs_display/README.md) — the built-in derive this page contrasts with
- [Expanding `thiserror`](../expanding_thiserror/README.md) — the next page: the code a real derive writes
- [A proc-macro crate](../a_proc_macro_crate/README.md) — why the macros above had to live in `show_tokens`, and what else that crate can and cannot hold
- [The Rust Reference: Procedural macros ↗](https://doc.rust-lang.org/reference/procedural-macros.html)

## Po polsku

**Makro proceduralne** (*procedural macro*) to zwykła funkcja w Ruscie, którą kompilator wywołuje w trakcie kompilacji twojego crate'a: dostaje **strumień tokenów** (*token stream*) i zwraca strumień tokenów. Są trzy rodzaje. **Makro derive** (`#[derive(Serialize)]`) dostaje element — strukturę, enum albo unię — i może jedynie *dopisać* coś obok niego, na przykład blok `impl`; samego elementu zmienić nie potrafi, a jeśli zwróci go jeszcze raz, kompilator zgłosi E0428, bo nazwa zostanie zdefiniowana dwukrotnie. **Makro funkcyjne** (*function-like*, `paste!(…)`) dostaje wszystko, co stoi między nawiasami — to nie musi być poprawny Rust, wystarczy, że nawiasy się domykają. **Makro atrybutowe** (`#[tokio::main]`) dostaje swoje argumenty i cały element, a to, co zwróci, *zastępuje* element; zwróci pusty strumień — funkcja znika.

Pułapka: `#[derive(Debug)]`, `#[test]` i `format_args!` wyglądają dokładnie tak samo, ale nie są makrami proceduralnymi, tylko makrami wbudowanymi w kompilator (`#[rustc_builtin_macro]` w źródłach `core`), a `println!` i `vec!` to zwykłe `macro_rules!`. Prawdziwe makra proceduralne spotyka się w crate'ach takich jak `serde`, `thiserror`, `clap` czy `tokio`.

**Szukaj po polsku:** makra proceduralne w Ruscie · `proc_macro_derive` · `proc_macro_attribute` · `rust procedural macro TokenStream` · `rust derive vs attribute macro`
