# A `#[retry]` attribute

**Level:** 301 · deep dive

**One line:** `#[retry(times = 3, delay_ms = 100)]` on a function that returns `Result` runs its body again after an `Err`, up to three attempts in all with 100 ms between them, and `darling`'s `FromMeta` turns those arguments into a struct, with the defaults, type checks and unknown-key errors generated for you.

<!-- file:demo/retry/src/lib.rs -->
```rust title="demo/retry/src/lib.rs"
//! `#[retry(times = 3, delay_ms = 100)]` runs a function that returns `Result`
//! again whenever it returns `Err`, up to `times` attempts in all, waiting
//! `delay_ms` milliseconds between one attempt and the next.

use std::num::NonZeroU32;

use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemFn, parse_quote};

/// The arguments. `darling` generates the parsing, the defaults and the errors.
#[derive(FromMeta)]
struct RetryArgs {
    /// Attempts in all, the first one included. Zero is not a `NonZeroU32`.
    times: NonZeroU32,
    /// Milliseconds between attempts, or none when left out.
    #[darling(default)]
    delay_ms: u64,
    /// The function that waits. A program can name one that only records the delay.
    #[darling(default = default_sleep)]
    sleep: syn::Path,
}

fn default_sleep() -> syn::Path {
    parse_quote!(::std::thread::sleep)
}

#[proc_macro_attribute]
pub fn retry(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    match expand(args.into(), item.clone()) {
        Ok(function) => function.into(),
        Err(errors) => {
            // Every error darling collected, and the function as it was written.
            let errors = errors.write_errors();
            quote!(#errors #item).into()
        }
    }
}

fn expand(args: TokenStream2, item: TokenStream2) -> darling::Result<TokenStream2> {
    let args = RetryArgs::from_list(&NestedMeta::parse_meta_list(args)?)?;
    let mut function: ItemFn = syn::parse2(item)?;
    if let Some(asyncness) = &function.sig.asyncness {
        let message = "#[retry] cannot wait inside an `async fn`";
        return Err(darling::Error::custom(message).with_span(asyncness));
    }
    let syn::ReturnType::Type(_, output) = &function.sig.output else {
        let message = "#[retry] needs a function that returns a `Result`";
        return Err(darling::Error::custom(message).with_span(&function.sig.ident));
    };

    let RetryArgs { times, delay_ms, sleep } = args;
    let times = times.get();
    let body = &function.block;
    // The body becomes a closure called once per attempt, so a `return` or a
    // `?` inside it ends that attempt, not the whole function.
    function.block = parse_quote! {{
        let mut attempts_left: u32 = #times;
        loop {
            attempts_left -= 1;
            match (|| -> #output #body)() {
                ::core::result::Result::Err(_) if attempts_left > 0 => {
                    #sleep(::core::time::Duration::from_millis(#delay_ms));
                }
                result => return result,
            }
        }
    }};
    Ok(quote!(#function))
}
```
<!-- /file -->

- **The arguments are a struct.** [`NestedMeta::parse_meta_list` ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/ast/data/nested_meta.rs#225) splits the tokens between the parentheses at the commas, and [`from_list` ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/from_meta.rs#106), which `#[derive(FromMeta)]` writes, matches each `key = value` to the field of that name. A field with `#[darling(default)]` may be left out.
- **The body becomes a closure, called once per attempt.** A `return` or a `?` inside the body returns from the closure, so it ends one attempt and the `match` decides whether there is another. Written straight into the loop, the same `?` would leave the function after the first failure, the trap [Parse, tweak, re-emit](../parse_tweak_reemit/README.md#the-trap-code-after-the-body) shows.
- **The wait is a path.** `sleep` names the function that waits, and defaults to `::std::thread::sleep`. That one extra key, which the course outline does not have, is what lets the program below show every delay without waiting for any of them.
- **On any error the function goes back too**, behind darling's messages, as [Re-emit the item on error](../re_emit_on_error/README.md) explains.

## Three attempts, and no waiting

<!-- file:demo/app/src/main.rs -->
```rust title="demo/app/src/main.rs"
use std::cell::Cell;
use std::num::ParseIntError;
use std::time::Duration;

use retry::retry;

/// Stands in for `std::thread::sleep`: prints the delay it was asked for and
/// returns at once, so this program never waits.
fn record_sleep(delay: Duration) {
    println!("  asked to sleep {delay:?}");
}

/// What a sensor returns on each read: two bad readings, then a number.
const READINGS: [&str; 3] = ["", "n/a", "21"];

#[retry(times = 3, delay_ms = 100, sleep = record_sleep)]
fn read_sensor(reads: &Cell<usize>) -> Result<u32, ParseIntError> {
    let text = READINGS[reads.get()];
    reads.set(reads.get() + 1);
    println!("attempt {}: {text:?}", reads.get());
    let value = text.parse::<u32>()?;
    Ok(value)
}

#[retry(times = 2, delay_ms = 250, sleep = record_sleep)]
fn always_refused(tries: &Cell<u32>) -> Result<(), String> {
    tries.set(tries.get() + 1);
    println!("attempt {}", tries.get());
    Err(format!("refused on attempt {}", tries.get()))
}

fn main() {
    let reads = Cell::new(0);
    println!("= {:?}\n", read_sensor(&reads));

    let tries = Cell::new(0);
    println!("= {:?}", always_refused(&tries));
}
```
<!-- /file -->

<!-- cargo:retry_attempts_and_delays -->
*Verified output of `cargo run -q -p retry_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
attempt 1: ""
  asked to sleep 100ms
attempt 2: "n/a"
  asked to sleep 100ms
attempt 3: "21"
= Ok(21)

attempt 1
  asked to sleep 250ms
attempt 2
= Err("refused on attempt 2")
```
<!-- /cargo -->

- **`read_sensor` failed twice through `?` and was run again each time.** The third attempt parsed `"21"`, and its `Ok` came back from the function.
- **Two waits for three attempts, one for two.** There is no wait after the last attempt: `always_refused` returned its second `Err` as it was.
- **Nothing slept.** `sleep = record_sleep` makes the generated code call `record_sleep` where it would call `std::thread::sleep`. Both take a `Duration`, so the macro needs no second code path, and the delay is recorded as the `Duration` asked for, `100ms`, instead of being measured. The answer key holds no clock reading, and the program finishes at once.

## As the outline writes it

With no `sleep` key, the default path is the one generated. This crate is expanded, never run:

<!-- file:demo/outline/src/lib.rs -->
```rust title="demo/outline/src/lib.rs"
//! The attribute exactly as the course outline writes it, with no `sleep` key.
//! Nothing calls `fetch`: this crate is here to be expanded, not run.

use retry::retry;

#[retry(times = 3, delay_ms = 100)]
pub fn fetch(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```
<!-- /file -->

`cargo expand` is the command you would run to see this, with [cargo-expand ↗](https://github.com/dtolnay/cargo-expand) installed. The key below was recorded without it, by asking the compiler directly, as on [Parse, tweak, re-emit](../parse_tweak_reemit/README.md#what-the-compiler-received):

<!-- cargo:retry_expanded -->
*Verified output of `cargo rustc -q -p outline --profile=check -- -Zunpretty=expanded` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
#![feature(prelude_import)]
//! The attribute exactly as the course outline writes it, with no `sleep` key.
//! Nothing calls `fetch`: this crate is here to be expanded, not run.
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;

use retry::retry;

pub fn fetch(path: &str) -> Result<String, std::io::Error> {
    let mut attempts_left: u32 = 3u32;
    loop {
        attempts_left -= 1;
        match (|| -> Result<String, std::io::Error>
                        { std::fs::read_to_string(path) })() {
            ::core::result::Result::Err(_) if attempts_left > 0 => {
                ::std::thread::sleep(::core::time::Duration::from_millis(100u64));
            }
            result => return result,
        }
    }
}
```
<!-- /cargo -->

The signature is unchanged, the original body sits inside the closure, the numbers arrived as the literals `3u32` and `100u64`, and the wait is `::std::thread::sleep`, an absolute path for the reasons in [Absolute paths and hygiene](../absolute_paths_and_hygiene/README.md).

## Bad arguments, reported by darling

Every mistake below is caught by `darling`: by the `from_list` it generated from `RetryArgs`, or by its `FromMeta` for a field's type. The macro itself has no check for any of them:

<!-- file:demo/bad_args/src/main.rs -->
```rust title="demo/bad_args/src/main.rs"
use retry::retry;

#[retry(times = 3, delay = 100)]
fn unknown_key() -> Result<(), String> {
    Ok(())
}

#[retry(times = 3, delay_ms = 0.5)]
fn wrong_type() -> Result<(), String> {
    Ok(())
}

#[retry(times = 0)]
fn zero_times() -> Result<(), String> {
    Ok(())
}

#[retry(delay_ms = 100)]
fn missing_times() -> Result<(), String> {
    Ok(())
}

#[retry(times = "3")]
fn quoted_number() -> Result<(), String> {
    Ok(())
}

fn main() {}
```
<!-- /file -->

<!-- cargo:retry_darling_errors -->
*Verified output of `cargo build -q -p bad_args`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: Unknown field: `delay`. Did you mean `delay_ms`?
 --> bad_args/src/main.rs:3:20
  |
3 | #[retry(times = 3, delay = 100)]
  |                    ^^^^^

error: Unexpected type `float`
 --> bad_args/src/main.rs:8:31
  |
8 | #[retry(times = 3, delay_ms = 0.5)]
  |                               ^^^

error: number would be zero for non-zero type
  --> bad_args/src/main.rs:13:17
   |
13 | #[retry(times = 0)]
   |                 ^

error: Missing field `times`
  --> bad_args/src/main.rs:18:1
   |
18 | #[retry(delay_ms = 100)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in the attribute macro `retry` (in Nightly builds, run with -Z macro-backtrace for more info)

error: could not compile `bad_args` (bin "bad_args") due to 4 previous errors
```
<!-- /cargo -->

| You wrote | darling said | Why |
|---|---|---|
| `delay = 100` | ``Unknown field: `delay`. Did you mean `delay_ms`?`` | the key matches no field; the suggestion comes from the field names |
| `delay_ms = 0.5` | ``Unexpected type `float` `` | a float literal is not a `u64` |
| `times = 0` | `number would be zero for non-zero type` | `times` is a `NonZeroU32`; the text is std's own [`ParseIntError` message ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/num/error.rs#L149) |
| no `times` | ``Missing field `times` ``, under the whole attribute | no default; the error has no token to point at, so it falls back to [the call site ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/error/mod.rs#356), the attribute |

**`quoted_number` is not in the output.** `times = "3"` is accepted as 3: darling's `FromMeta` for the integer types parses a string literal as well as an integer one ([from_meta.rs, line 341 ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/from_meta.rs#341)).

`times = 0` is refused by the field's type, not by an `if`. The rule *at least one attempt* lives in `NonZeroU32`, the error is underlined at the `0` that broke it, and code reading `args.times` never has to consider zero.

## Functions it refuses

darling checks the arguments; the item is the macro's own business. Two checks in `expand` return `darling::Error::custom(…).with_span(…)`:

<!-- file:demo/bad_functions/src/main.rs -->
```rust title="demo/bad_functions/src/main.rs"
use retry::retry;

#[retry(times = 3)]
async fn in_async() -> Result<(), String> {
    Ok(())
}

#[retry(times = 3)]
fn no_result() {}

fn main() {}
```
<!-- /file -->

<!-- cargo:retry_refused_functions -->
*Verified output of `cargo build -q -p bad_functions`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error: #[retry] cannot wait inside an `async fn`
 --> bad_functions/src/main.rs:4:1
  |
4 | async fn in_async() -> Result<(), String> {
  | ^^^^^

error: #[retry] needs a function that returns a `Result`
 --> bad_functions/src/main.rs:9:4
  |
9 | fn no_result() {}
  |    ^^^^^^^^^

error: could not compile `bad_functions` (bin "bad_functions") due to 2 previous errors
```
<!-- /cargo -->

**An `async fn` is refused rather than retried.** The default wait is `std::thread::sleep`, and its documentation says it *"is blocking, and should not be used in `async` functions"* ([std/src/thread/functions.rs ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/std/src/thread/functions.rs#L259)). Retrying an `async fn` needs a different expansion: a new future for every attempt, because [a finished future must not be polled again ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/future/future.rs#L54), and a wait that is itself `.await`ed. This macro has only the blocking expansion, so it points at the `async` keyword instead of generating it.

## The trap: an attempt that uses up its arguments

Every attempt runs the same body with the same arguments. A body that moves an argument can run only once:

<!-- file:demo/consumes_argument/src/main.rs -->
```rust title="demo/consumes_argument/src/main.rs"
use retry::retry;

fn deliver(message: String) -> Result<(), String> {
    Err(message)
}

#[retry(times = 3)]
fn send(message: String) -> Result<(), String> {
    deliver(message)
}

fn main() {
    let _ = send(String::from("hello"));
}
```
<!-- /file -->

<!-- cargo:retry_consumed_argument -->
*Verified output of `cargo build -q -p consumes_argument`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0382]: use of moved value: `message`
 --> consumes_argument/src/main.rs:7:1
  |
7 | #[retry(times = 3)]
  | ^^^^^^^^^^^^^^^^^^^
  | |
  | inside of this loop
  | value moved into closure here, in previous iteration of loop
8 | fn send(message: String) -> Result<(), String> {
  |         ------- move occurs because `message` has type `String`, which does not implement the `Copy` trait
9 |     deliver(message)
  |             ------- use occurs due to use in closure
  |
note: consider changing this parameter type in function `deliver` to borrow instead if owning the value isn't necessary
 --> consumes_argument/src/main.rs:3:21
  |
3 | fn deliver(message: String) -> Result<(), String> {
  |    -------          ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
  = note: this error originates in the attribute macro `retry` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
9 |     deliver(message.clone())
  |                    ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `consumes_argument` (bin "consumes_argument") due to 1 previous error
```
<!-- /cargo -->

The first attempt moved `message` into `deliver`, and there is nothing left for the second. The labels land on the attribute line because the closure and the loop are tokens the macro wrote, and [`quote!` gives those tokens ↗](https://docs.rs/crate/quote/1.0.47/source/src/lib.rs#169) the call site's span, which for an attribute is the attribute. So `retry` works on functions whose attempts can be repeated: arguments taken by reference, as both functions in `retry_app` take `&Cell`, or cloned inside the body, as the `help` suggests.

## `darling` follows `syn`'s major version

A darling release is built against one `syn`, and its `NestedMeta` and `FromMeta` hand back that version's types. Its [CHANGELOG ↗](https://docs.rs/crate/darling/0.24.1/source/CHANGELOG.md) records both moves: 0.20.0 went to syn 2 and replaced `parse_macro_input!(args as AttributeArgs)` with `NestedMeta::parse_meta_list(args)`, and 0.24.0 went to syn 3. Tutorials written for syn 1 still show `AttributeArgs`, which was syn 1's `Vec<NestedMeta>` ([syn 1.0.109 attr.rs ↗](https://docs.rs/crate/syn/1.0.109/source/src/attr.rs#464)); no later syn has it. This demo's `Cargo.lock` pins darling 0.24.1 with syn 3.0.6.

## If you are coming from another language

- **Python.** The [`tenacity` ↗](https://tenacity.readthedocs.io/) decorator is the same idea at run time: `@retry(stop=stop_after_attempt(3), wait=wait_fixed(0.1))`. Its arguments are checked when Python runs that code, not before; `#[retry]` reports all four mistakes above before the program exists. And a retrying decorator calls the function again with the same argument objects: a function that drains an iterator it was given gets an empty one on attempt two, silently. Rust refuses that function, the E0382 above.
- **Java.** Spring Retry's `@Retryable(maxAttempts = 3, backoff = @Backoff(delay = 100))` reads the same, and `javac` checks the element types of an annotation, so `maxAttempts = "3"` does not compile, where darling accepts `times = "3"`. The retrying itself happens in a proxy at run time, so a bean calling its own `@Retryable` method directly is not retried. `#[retry]` rewrites the function, so every call retries.
- **C++.** No attribute generates code, so a retry is a function template taking a lambda: `retry(3, 100ms, [&] { return read_sensor(reads); })`. That lambda can move from a captured `std::string` on its first run, and the second run gets a moved-from string, valid but with unspecified contents. The borrow checker's E0382 is the same bug refused at compile time.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP.)* No annotation does this: the retry is a `DO n TIMES` loop around the call, leaving it with `EXIT` on success, with `WAIT UP TO … SECONDS` between attempts. The loop is written by hand at every call site that needs it, which is the repetition the attribute removes.

## See also

- [Parse, tweak, re-emit](../parse_tweak_reemit/README.md) — replacing a function's body, and the trap that a closure avoids here
- [Re-emit the item on error](../re_emit_on_error/README.md) — why `retry` returns the function behind its errors
- [Helper attributes with `darling`](../helper_attributes_with_darling/README.md) — darling on a derive's helper attributes, with `FromDeriveInput` and `FromField`
- [Testing with `trybuild`](../testing_with_trybuild/README.md) — the four darling errors as a test instead of a recorded build
- [Common async pitfalls](../../35_Async/common_async_pitfalls/README.md) — a blocking `std::thread::sleep` inside a task
- [`darling::FromMeta` ↗](https://docs.rs/darling/0.24.1/darling/trait.FromMeta.html)
- [`std::num::NonZeroU32` ↗](https://doc.rust-lang.org/std/num/type.NonZeroU32.html)

## Po polsku

`#[retry(times = 3, delay_ms = 100)]` na funkcji zwracającej `Result` uruchamia jej ciało ponownie po każdym `Err` — najwyżej trzy **próby** (*attempts*) w sumie, ze 100 ms przerwy między nimi. Argumenty makra to struktura z `#[derive(FromMeta)]` z crate'a **`darling`**: `NestedMeta::parse_meta_list` dzieli tokeny po przecinkach, a wygenerowane `from_list` dopasowuje klucze do pól, uzupełnia wartości domyślne i zgłasza błędy — nieznany klucz (z podpowiedzią `delay_ms`), zły typ (`0.5` zamiast liczby całkowitej), brak `times`. `times = 0` odrzuca sam typ pola, `NonZeroU32`. Uwaga: `times = "3"` w cudzysłowie zostaje przyjęte jako 3.

Ciało funkcji trafia do domknięcia wywoływanego raz na próbę, więc `return` i `?` kończą próbę, a nie całą funkcję. Oczekiwanie to ścieżka do funkcji (`sleep`, domyślnie `::std::thread::sleep`), więc program demonstracyjny podstawia funkcję, która tylko wypisuje żądany `Duration` — nic nie śpi, a klucz odpowiedzi nie zależy od zegara. Makro odrzuca `async fn`, bo `std::thread::sleep` blokuje wątek. Pułapka: ciało, które **przenosi** argument (*move*), może wykonać się tylko raz — kompilator zgłasza E0382.

**Szukaj po polsku:** makro atrybutowe `retry` · `darling FromMeta` · ponawianie wywołań w Ruscie · `rust retry attribute macro` · `NestedMeta parse_meta_list`
