# Parse, tweak, re-emit

**Level:** 301 · deep dive

**One line:** An attribute macro on a function parses it into a `syn::ItemFn`, changes the one part it means to change, and returns the whole function, because whatever it returns replaces what was there.

`#[trace]` prints a line when a function is entered and another when it is left. Parse, tweak and re-emit are the last three steps of `trace`:

<!-- file:demo/trace/src/lib.rs -->
```rust title="demo/trace/src/lib.rs"
//! `#[trace]` prints a line when a function is entered and another when it is left.
//!
//! An attribute's output replaces the item, so the macro parses the function,
//! changes one field of it, and returns all of it: the attributes, visibility
//! and signature go back exactly as they were parsed.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn trace(args: TokenStream, item: TokenStream) -> TokenStream {
    // The first stream is whatever sat between the parentheses of `#[trace(…)]`.
    // This macro takes nothing there, so any token at all is an error.
    if !args.is_empty() {
        let args = proc_macro2::TokenStream::from(args);
        let item = proc_macro2::TokenStream::from(item);
        let error = syn::Error::new_spanned(args, "#[trace] takes no arguments");
        let error = error.into_compile_error();
        // The function goes back too, or every call to it is a second error.
        return quote!(#error #item).into();
    }

    let mut function = parse_macro_input!(item as ItemFn);
    let name = function.sig.ident.to_string();
    let body = &function.block;
    // `leave` is printed by a value dropped on the way out, so it runs however
    // the body ends: at its last expression, at a `return`, or at a `?`.
    function.block = parse_quote! {{
        struct Leave;
        impl ::core::ops::Drop for Leave {
            fn drop(&mut self) {
                ::std::println!("leave {}", #name);
            }
        }
        ::std::println!("enter {}", #name);
        let _leave = Leave;
        #body
    }};
    quote!(#function).into()
}

/// The version that looks right: print `leave` after the body. A `return` or a
/// `?` inside the body leaves the function before that line is reached.
#[proc_macro_attribute]
pub fn trace_after_body(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let name = function.sig.ident.to_string();
    let body = &function.block;
    function.block = parse_quote! {{
        ::std::println!("enter {}", #name);
        let result = #body;
        ::std::println!("leave {}", #name);
        result
    }};
    quote!(#function).into()
}
```
<!-- /file -->

- **Parse.** `parse_macro_input!(item as ItemFn)` turns the item's tokens into a syntax tree, or returns syn's error from the macro if they are not a function.
- **Tweak.** Only `function.block` is assigned. The new block sets up the printing and then holds the original body, `#body`, as a nested block whose value is the function's value.
- **Re-emit.** `quote!(#function)` prints the whole `ItemFn` back to tokens: attributes, visibility, signature and the new body. Leave out a part and the compiler never sees it.

The second macro in the file, `trace_after_body`, is the obvious way to write the same thing, and it is wrong. [The trap](#the-trap-code-after-the-body) below shows where.

## Three functions that must survive it

A function with two early exits, a generic one with a `where` clause, and an `async fn`:

<!-- file:demo/app/src/lib.rs -->
```rust title="demo/app/src/lib.rs"
//! Three functions a `#[trace]` must hand back unchanged apart from the body:
//! one with an early `return` and a `?`, one generic with a `where` clause, and
//! an `async fn`.

use trace::trace;

/// The port in `host:port`.
#[trace]
pub fn port(address: &str) -> Result<u16, String> {
    let Some((_, port)) = address.split_once(':') else {
        return Err(format!("no port in {address:?}"));
    };
    let port = port.parse::<u16>().map_err(|e| e.to_string())?;
    Ok(port)
}

#[trace]
pub fn largest<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    let mut best = items.first()?;
    for item in items {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

#[trace]
pub async fn double(n: u32) -> u32 {
    n * 2
}
```
<!-- /file -->

<!-- file:demo/app/src/main.rs -->
```rust title="demo/app/src/main.rs"
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use trace_app::{double, largest, port};

fn main() {
    let ok = port("localhost:8080");
    println!("= {ok:?}\n");
    let early = port("localhost");
    println!("= {early:?}\n");
    let question_mark = port("localhost:http");
    println!("= {question_mark:?}\n");
    let max = largest(&[3, 7, 5]);
    println!("= {max:?}\n");

    let future = double(21);
    println!("double(21) called, nothing printed yet");
    // `double` never waits, so one poll with a waker that does nothing finishes it.
    let mut context = Context::from_waker(Waker::noop());
    if let Poll::Ready(n) = pin!(future).poll(&mut context) {
        println!("= {n}");
    }
}
```
<!-- /file -->

<!-- cargo:trace_enter_and_leave -->
*Verified output of `cargo run -q -p trace_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
enter port
leave port
= Ok(8080)

enter port
leave port
= Err("no port in \"localhost\"")

enter port
leave port
= Err("invalid digit found in string")

enter largest
leave largest
= Some(7)

double(21) called, nothing printed yet
enter double
leave double
= 42
```
<!-- /cargo -->

- **Every call printed `leave`.** `port("localhost")` left through the `return` inside [`let … else`](../../30_Pattern_Matching/let_else/README.md), and `port("localhost:http")` through [`?`](../../17_Option_and_Result/the_question_mark_operator/README.md). Each way out drops `_leave`, and [`Drop`](../../12_Traits/drop_and_raii/README.md) prints the line.
- **`largest` compiled with its generics and `where` clause untouched.** The macro never read them; it put `sig` back as parsed.
- **`double(21)` printed nothing until it was polled.** The new statements went inside the body, and the body of an `async fn` runs only when its future is polled ([What a future is](../../35_Async/what_a_future_is/README.md)). So `enter double` means *the body started*, not *the function was called*.

## What `syn` hands you: `ItemFn`

This program parses a function from a string and prints each field of the `ItemFn` back as tokens:

<!-- file:demo/item_fn_fields/src/main.rs -->
```rust title="demo/item_fn_fields/src/main.rs"
//! What `syn` makes of a function: the fields of `syn::ItemFn`, each printed
//! back as tokens.

use quote::ToTokens;
use syn::ItemFn;

const SOURCE: &str = r#"
/// The port in `host:port`.
#[inline]
pub async fn port<T: Copy>(address: &str) -> Result<u16, String> where T: Default {
    todo!()
}
"#;

fn show(field: &str, value: &dyn ToTokens) {
    println!("{field:<15} {}", value.to_token_stream());
}

fn main() {
    let function: ItemFn = syn::parse_str(SOURCE).unwrap();
    println!("{} attrs", function.attrs.len());
    for attr in &function.attrs {
        show("  attr", attr);
    }
    show("vis", &function.vis);
    println!("{:<15} {:?}", "modifiers", function.modifiers);
    show("sig", &function.sig);
    show("  asyncness", &function.sig.asyncness);
    println!("{:<15} {:?}", "  safety", function.sig.safety);
    show("  ident", &function.sig.ident);
    show("  generics", &function.sig.generics);
    show("  where_clause", &function.sig.generics.where_clause);
    show("  inputs", &function.sig.inputs);
    show("  output", &function.sig.output);
    show("block", &function.block);
}
```
<!-- /file -->

<!-- cargo:trace_item_fn_fields -->
*Verified output of `cargo run -q -p item_fn_fields` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
2 attrs
  attr          # [doc = " The port in `host:port`."]
  attr          # [inline]
vis             pub
modifiers       FnModifiers { defaultness: None }
sig             async fn port < T : Copy > (address : & str) -> Result < u16 , String > where T : Default
  asyncness     async
  safety        Safety::Default
  ident         port
  generics      < T : Copy >
  where_clause  where T : Default
  inputs        address : & str
  output        -> Result < u16 , String >
block           { todo ! () }
```
<!-- /cargo -->

| Field | What it held above | What `#[trace]` does with it |
|---|---|---|
| [`attrs` ↗](https://docs.rs/syn/3.0.6/syn/struct.ItemFn.html#structfield.attrs) | the doc comment, as `#[doc = "…"]`, and `#[inline]` | puts them back |
| [`vis` ↗](https://docs.rs/syn/3.0.6/syn/struct.ItemFn.html#structfield.vis) | `pub` | puts it back |
| [`modifiers` ↗](https://docs.rs/syn/3.0.6/syn/struct.FnModifiers.html) | nothing: its one field, `defaultness`, is `None` | nothing to put back |
| [`sig` ↗](https://docs.rs/syn/3.0.6/syn/struct.Signature.html) | everything from `async` to the `where` clause | reads `sig.ident` for the name |
| [`block` ↗](https://docs.rs/syn/3.0.6/syn/struct.Block.html) | the braces and the statements in them | replaces it |

The generics live inside the signature: `sig.generics` holds `<T: Copy>`, and the `where` clause is `sig.generics.where_clause`, so a macro that keeps `sig` keeps both. The attribute being expanded is not among `attrs`; the compiler removes it before calling the macro, as [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) shows.

**Two fields differ from syn 2**, which most tutorials were written against. `modifiers` is new in syn 3, and it stays empty when the tokens parse as a free function: [`parse_rest_of_fn` ↗](https://docs.rs/crate/syn/3.0.6/source/src/item.rs#1889) builds it as `FnModifiers { defaultness: None }`. And the signature's `unsafety: Option<Token![unsafe]>` ([syn 2.0.119 ↗](https://docs.rs/crate/syn/2.0.119/source/src/item.rs#799)) became `safety: Safety` ([syn 3.0.6 ↗](https://docs.rs/crate/syn/3.0.6/source/src/item.rs#975)), an enum of `Safe`, `Unsafe` and `Default`, printed above as `Safety::Default`. Code that reads `sig.unsafety` stops compiling on syn 3.

The spacing (`address : & str`) is `proc-macro2`'s own printer, used when no compiler is running the code; [`proc-macro2` makes it testable](../proc_macro2_makes_it_testable/README.md) is about that difference.

## What the compiler received

`cargo expand` is the command you would run to see this, with [cargo-expand ↗](https://github.com/dtolnay/cargo-expand) installed. The key below was recorded without it, by asking the compiler directly for the expanded source of `trace_app`'s library:

<!-- cargo:trace_expanded -->
*Verified output of `cargo rustc -q -p trace_app --lib --profile=check -- -Zunpretty=expanded` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
#![feature(prelude_import)]
//! Three functions a `#[trace]` must hand back unchanged apart from the body:
//! one with an early `return` and a `?`, one generic with a `where` clause, and
//! an `async fn`.
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;

use trace::trace;

#[doc = " The port in `host:port`."]
pub fn port(address: &str) -> Result<u16, String> {
    struct Leave;
    impl ::core::ops::Drop for Leave {
        fn drop(&mut self) {


            { ::std::io::_print(format_args!("leave {0}\n", "port")); };
        }
    }
    { ::std::io::_print(format_args!("enter {0}\n", "port")); };
    let _leave = Leave;
    {
        let Some((_, port)) =
            address.split_once(':') else {
                return Err(::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("no port in {0:?}",
                                        address))
                            }));
            };
        let port = port.parse::<u16>().map_err(|e| e.to_string())?;
        Ok(port)
    }
}
pub fn largest<T>(items: &[T]) -> Option<&T> where T: PartialOrd {
    struct Leave;
    impl ::core::ops::Drop for Leave {
        fn drop(&mut self) {
            { ::std::io::_print(format_args!("leave {0}\n", "largest")); };
        }
    }
    { ::std::io::_print(format_args!("enter {0}\n", "largest")); };
    let _leave = Leave;
    {
        let mut best = items.first()?;
        for item in items { if item > best { best = item; } }
        Some(best)
    }
}
pub async fn double(n: u32) -> u32 {
    struct Leave;
    impl ::core::ops::Drop for Leave {
        fn drop(&mut self) {
            { ::std::io::_print(format_args!("leave {0}\n", "double")); };
        }
    }
    { ::std::io::_print(format_args!("enter {0}\n", "double")); };
    let _leave = Leave;
    { n * 2 }
}
```
<!-- /cargo -->

- `#[trace]` is gone from all three functions, and the doc comment is back as the `#[doc]` attribute it always was.
- Each signature is exactly what `lib.rs` says, including `where T: PartialOrd` and `async`.
- Each original body is intact, as the last block of the new one.
- The `println!` calls the macro generated were expanded too: a macro's output goes through expansion again.

`-Zunpretty=expanded` is an unstable flag, and `RUSTC_BOOTSTRAP=1` in [`cargo_runs.toml`](demo/cargo_runs.toml) is what lets the stable 1.98.0 compiler accept it.

## Arguments: the first `TokenStream`

An attribute macro gets two streams. The first holds the tokens between the parentheses of `#[trace(…)]`, and is empty for a bare `#[trace]`. A macro that takes no arguments should say so rather than ignore them:

<!-- file:demo/trace_args/src/main.rs -->
```rust title="demo/trace_args/src/main.rs"
use trace::trace;

#[trace(verbose)] // #[trace] takes no arguments
fn greet(name: &str) -> String {
    format!("hello, {name}")
}

fn main() {
    println!("{}", greet("Ada"));
    println!("{}", greet("Grace"));
}
```
<!-- /file -->

<!-- cargo:trace_takes_no_arguments -->
*Verified output of `cargo build -q -p trace_args`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: #[trace] takes no arguments
 --> trace_args/src/main.rs:3:9
  |
3 | #[trace(verbose)] // #[trace] takes no arguments
  |         ^^^^^^^

error: could not compile `trace_args` (bin "trace_args") due to 1 previous error
```
<!-- /cargo -->

`syn::Error::new_spanned(args, …)` underlines exactly the argument tokens. One error, although `main` calls `greet` twice: the error path returns the function along with the error. Return the error alone and each call becomes an error of its own, which is [Re-emit the item on error](../re_emit_on_error/README.md). For a macro that does take arguments, [A `#[retry]` attribute](../a_retry_attribute/README.md) parses `times = 3, delay_ms = 100` with `darling`.

## The trap: code after the body

`trace_after_body`, in the same crate, puts the `leave` line after the body instead of in a `Drop`. Here it is on the same `port` function:

<!-- file:demo/after_body/src/main.rs -->
```rust title="demo/after_body/src/main.rs"
use trace::trace_after_body;

/// The same `port` as in `trace_app`, under the macro that prints `leave` after the body.
#[trace_after_body]
fn port(address: &str) -> Result<u16, String> {
    let Some((_, port)) = address.split_once(':') else {
        return Err(format!("no port in {address:?}"));
    };
    let port = port.parse::<u16>().map_err(|e| e.to_string())?;
    Ok(port)
}

fn main() {
    println!("= {:?}\n", port("localhost:8080"));
    println!("= {:?}\n", port("localhost"));
    println!("= {:?}", port("localhost:http"));
}
```
<!-- /file -->

<!-- cargo:trace_after_body_misses_leave -->
*Verified output of `cargo run -q -p after_body` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
enter port
leave port
= Ok(8080)

enter port
= Err("no port in \"localhost\"")

enter port
= Err("invalid digit found in string")
```
<!-- /cargo -->

Only the call that reached the end of the body printed `leave`. The body is spliced in as a block, but a `return` or a `?` inside it leaves the whole *function*, not the block, so `println!("leave …")` is never reached on those two paths. It compiles without a warning, and it is correct for every function that has no early exit, which is why it survives testing. The value dropped on the way out is the version that holds for every exit.

## If you are coming from another language

- **Python.** An attribute macro on a function is a decorator that runs in the compiler. A decorator has to copy `__name__` and `__doc__` onto its wrapper with `functools.wraps`, because it returns a *different* function; the macro returns the same function with a new body, so the name, the doc comment and the signature are simply the originals. The trap transfers exactly: a decorator that logs after `result = func(*args)` misses every exception, which is why the leave line goes in `try: … finally:`, the job `Drop` does here. So does the `async` surprise: calling an `async def` returns a coroutine without running its body, as `double(21)` did.
- **Java.** An annotation processor cannot modify the method it sees; it can only generate new source files. Wrapping a method is done at run time instead, by an aspect or a proxy: Spring's `@Around` advice wraps `joinPoint.proceed()`, in `try … finally` when the exit has to be logged every time. A proxy intercepts only calls that go through it, so a bean calling its own method is not traced. The macro rewrote the function itself, so every call to it is traced, from anywhere.
- **C++.** No attribute can rewrite a function. The `Drop` guard is RAII, and the same scope guard works by hand: a local object whose destructor prints, declared at the top of each function, often through a `#define` that passes `__func__`. What the attribute adds is that you cannot forget it in the function, nor mistype the name.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP.)* The nearest thing is an implicit enhancement option at the start and end of a method or function module: code inserted around the original without editing it. It is attached to the object in the system, not written as an annotation on the source, and it changes the program for every caller at once.

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — an attribute's output is the item's replacement
- [Re-emit the item on error](../re_emit_on_error/README.md) — what to return when parsing or checking fails
- [A `#[retry]` attribute](../a_retry_attribute/README.md) — a body wrapped in a loop, with arguments parsed by `darling`
- [Parsing with `syn`](../parsing_with_syn/README.md) — the syntax tree a derive gets instead
- [Generating with `quote`](../generating_with_quote/README.md) — `#body` and `#function` interpolation
- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — why `_leave` prints on every path
- [`syn::ItemFn` ↗](https://docs.rs/syn/3.0.6/syn/struct.ItemFn.html)

## Po polsku

**Makro atrybutowe** na funkcji dostaje jej tokeny, **parsuje** je do `syn::ItemFn`, **zmienia** tę jedną część, którą ma zmienić — tu ciało funkcji (`block`) — i **zwraca całą funkcję**, bo to, co zwróci, zastępuje oryginał. Atrybuty (także komentarz dokumentacyjny jako `#[doc]`), widoczność i sygnatura razem z parametrami generycznymi, klauzulą `where` i `async` wracają tak, jak je sparsowano. W syn 3 `ItemFn` ma nowe pole `modifiers`, a `Signature::unsafety` zamieniło się w `safety: Safety`; kod pisany pod syn 2, który czyta `sig.unsafety`, przestaje się kompilować.

Pułapka: linijka „wyjście z funkcji” dopisana *za* ciałem nie wykona się, gdy ciało kończy się przez `return` albo `?` — one opuszczają całą funkcję, nie tylko blok. Wartość, której `Drop` wypisuje tę linijkę, działa na każdej ścieżce wyjścia. Pierwszy `TokenStream` makra to jego argumenty; makro, które argumentów nie przyjmuje, powinno je odrzucić błędem wskazującym dokładnie te tokeny i mimo to oddać funkcję.

**Szukaj po polsku:** makro atrybutowe na funkcji · `syn ItemFn` · `proc_macro_attribute` · `rust attribute macro wrap function body` · `cargo expand`
