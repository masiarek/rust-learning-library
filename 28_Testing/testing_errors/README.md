# Every testing error, and its fix

[Testing](../README.md) › **Errors, by symptom**

**Level:** 201 · a reference, by symptom

**One line:** 16 compiler errors you meet while writing tests: the test function itself, the assertion, where the test lives, test doubles, and the process state and temporary files tests share. For each, the code, what rustc 1.98.0 prints, the mistake, and a fix. The broken code must fail and the fix must compile: [`check_fences.py`](../../tools/check_fences.py) holds both on every build.

Each transcript is the fence above it, saved under the file name in its title and compiled with `rustc --edition 2024 --crate-type lib`, adding `--test` where the title says so. The closing `error: aborting…` and `For more information…` lines are dropped. A fence marked `test` is checked as a test build too, which is the only build in which an error inside a `#[test]` function, or about one, exists: in an ordinary build every `#[test]` item is configured out. Transcripts are not regenerated on each commit the way an example's output is, so an upgrade can reword one; the fences, which are checked, cannot silently stop failing.

## Find the error

| # | Code | What rustc says | The mistake, briefly |
|---|---|---|---|
| 1 | no code | [functions used as tests can not have any arguments](#1-a-test-function-with-a-parameter) | a `#[test]` function with a parameter |
| 2 | no code | [async functions cannot be used for tests](#2-an-async-fn-test-with-no-runtime) | an `async fn` test, with no runtime |
| 3 | no code | [functions using `#[should_panic]` must return `()`](#3-should_panic-on-a-test-that-returns-result) | `#[should_panic]` on a test that returns `Result` |
| 4 | `E0277` | [the trait bound `Option<()>: Termination` is not satisfied](#4-a-test-that-returns-option) | a test that returns `Option` |
| 5 | `E0369` | [binary operation `==` cannot be applied to type `Price`](#5-assert_eq-on-a-struct-with-no-partialeq) | `assert_eq!` on a struct with no `PartialEq` |
| 6 | `E0277` | [`Price` doesn't implement `Debug`](#6-assert_eq-on-a-struct-with-no-debug) | `assert_eq!` on a struct with no `Debug` |
| 7 | no code | [format argument must be a string literal](#7-a-string-variable-as-the-assertion-message) | a `String` variable as the assertion message |
| 8 | `E0432` | [unresolved import `crate::tally`](#8-use-crate-in-a-file-under-tests) | `use crate::` in a file under `tests/` |
| 9 | `E0603` | [function `is_spoiled` is private](#9-a-test-calling-a-private-function-from-outside-its-module) | a test calling a private function from outside its module |
| 10 | `E0425` | [cannot find function `sample_prices` in this scope](#10-a-cfgtest-helper-called-by-ordinary-code) | a `#[cfg(test)]` helper called by ordinary code |
| 11 | `E0308` | [mismatched types](#11-a-fake-passed-where-a-concrete-type-is-expected) | a fake passed where a concrete type is expected |
| 12 | `E0596` | [cannot borrow `self.sent` as mutable, as it is behind a `&` reference](#12-a-recording-fake-that-pushes-through-self) | a recording fake that pushes through `&self` |
| 13 | `E0277` | [the size for values of type `dyn Mailer` cannot be known at compilation time](#13-a-dyn-trait-passed-to-an-impl-trait-parameter) | a `&dyn Trait` passed to an `&impl Trait` parameter |
| 14 | `E0133` | [call to unsafe function `set_var` is unsafe and requires unsafe block](#14-stdenvset_var-in-edition-2024) | `std::env::set_var` in edition 2024 |
| 15 | `E0515` | [cannot return value referencing local variable `dir`](#15-returning-a-borrowed-path-out-of-a-helper-that-owns-the-tempdir) | returning a borrowed path out of a helper that owns the `TempDir` |
| 16 | `E0716` | [temporary value dropped while borrowed](#16-borrowing-a-path-from-tempdirnew-in-one-statement) | borrowing a path from `TempDir::new()` in one statement |

## Writing the test function

### 1. A `#[test]` function with a parameter

```rust,compile_fail,test
#[test]
fn parses_every_price(input: &str) {
    assert!(input.parse::<u32>().is_ok());
}
```

```text title="rustc 1.98.0 --test on test_takes_an_argument.rs"
error: functions used as tests can not have any arguments
 --> test_takes_an_argument.rs:2:1
  |
2 | / fn parses_every_price(input: &str) {
3 | |     assert!(input.parse::<u32>().is_ok());
4 | | }
  | |_^
```

**The mistake.** A `#[test]` function is called by the harness with nothing, so it cannot ask for anything. There is no fixture injection or parameterisation in the built-in harness.

**The fix.**

```rust,test
#[test]
fn parses_every_price() {
    for input in ["250", "399", "1000"] {
        assert!(input.parse::<u32>().is_ok(), "{input} did not parse");
    }
}
```

**Read:** The loop stops at the first bad input. One named test per input is what `rstest` cases or a [harness of your own](../a_harness_of_your_own/README.md) give you.

### 2. An `async fn` test, with no runtime

```rust,compile_fail,test
pub async fn fetch_price() -> u32 {
    250
}

#[test]
async fn fetches_a_price() {
    assert_eq!(fetch_price().await, 250);
}
```

```text title="rustc 1.98.0 --test on async_test_without_a_runtime.rs"
error: async functions cannot be used for tests
 --> async_test_without_a_runtime.rs:6:1
  |
6 |   async fn fetches_a_price() {
  |   ^----
  |   |
  |  _`async` because of this
  | |
7 | |     assert_eq!(fetch_price().await, 250);
8 | | }
  | |_^
```

**The mistake.** std has no async runtime, so the harness has nothing to run an `async fn` on. `#[tokio::test]` exists to supply one: it rewrites the test into an ordinary `#[test]` that builds a runtime and calls `block_on`.

**The fix.**

```rust,test
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

pub async fn fetch_price() -> u32 {
    250
}

/// Enough for a future that never waits on anything. Real async code wants `#[tokio::test]`.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(out) = future.as_mut().poll(&mut cx) {
            return out;
        }
    }
}

#[test]
fn fetches_a_price() {
    assert_eq!(block_on(fetch_price()), 250);
}
```

**Read:** [What a future is](../../35_Async/what_a_future_is/README.md)

### 3. `#[should_panic]` on a test that returns `Result`

```rust,compile_fail,test
#[test]
#[should_panic(expected = "divide by zero")]
fn dividing_by_zero_panics() -> Result<(), String> {
    let zero = std::hint::black_box(0);
    let _ = 1 / zero;
    Ok(())
}
```

```text title="rustc 1.98.0 --test on should_panic_returning_result.rs"
error: functions using `#[should_panic]` must return `()`
 --> should_panic_returning_result.rs:3:1
  |
3 | / fn dividing_by_zero_panics() -> Result<(), String> {
4 | |     let zero = std::hint::black_box(0);
5 | |     let _ = 1 / zero;
6 | |     Ok(())
7 | | }
  | |_^
```

**The mistake.** A test can fail two ways, by panicking or by returning `Err`, and `#[should_panic]` only knows about the first. The combination is refused rather than guessed at.

**The fix.**

```rust,test
#[test]
#[should_panic(expected = "divide by zero")]
fn dividing_by_zero_panics() {
    let zero = std::hint::black_box(0);
    let _ = 1 / zero;
}
```

**Read:** [Where a test goes](../where_a_test_goes/README.md)

### 4. A test that returns `Option`

```rust,compile_fail,test
#[test]
fn finds_the_first_word() -> Option<()> {
    let first = "hello world".split(' ').next()?;
    assert_eq!(first, "hello");
    Some(())
}
```

```text title="rustc 1.98.0 --test on test_returning_option.rs, the note pointing into libtest's source dropped"
error[E0277]: the trait bound `Option<()>: Termination` is not satisfied
   --> test_returning_option.rs:2:30
    |
  1 | #[test]
    | ------- in this attribute macro expansion
  2 | fn finds_the_first_word() -> Option<()> {
    |                              ^^^^^^^^^^ the trait `Termination` is not implemented for `Option<()>`
    |
```

**The mistake.** A test's return type must implement `Termination`, the trait `main`'s return type needs too. std implements it for `()`, `ExitCode`, `!`, `Infallible`, and `Result<T, E>` when `E: Debug`. `Option` is not on the list: a `None` carries no error to print.

**The fix.**

```rust,test
#[test]
fn finds_the_first_word() -> Result<(), String> {
    let first = "hello world".split(' ').next().ok_or("no words")?;
    assert_eq!(first, "hello");
    Ok(())
}
```

**Read:** [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md)

## Asserting

### 5. `assert_eq!` on a struct with no `PartialEq`

```rust,compile_fail
#[derive(Debug)]
pub struct Price {
    pub cents: u32,
}

pub fn check_price() {
    assert_eq!(Price { cents: 250 }, Price { cents: 250 });
}
```

```text title="rustc 1.98.0 on assert_eq_without_partialeq.rs"
error[E0369]: binary operation `==` cannot be applied to type `Price`
 --> assert_eq_without_partialeq.rs:7:5
  |
7 |     assert_eq!(Price { cents: 250 }, Price { cents: 250 });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     Price
  |     Price
  |
note: an implementation of `PartialEq` might be missing for `Price`
 --> assert_eq_without_partialeq.rs:2:1
  |
2 | pub struct Price {
  | ^^^^^^^^^^^^^^^^ must implement `PartialEq`
help: consider annotating `Price` with `#[derive(PartialEq)]`
  |
2 + #[derive(PartialEq)]
3 | pub struct Price {
  |
```

**The mistake.** `assert_eq!` compares with `==`, and a struct has no `==` until it derives or implements `PartialEq`.

**The fix.**

```rust,test
#[derive(Debug, PartialEq)]
pub struct Price {
    pub cents: u32,
}

pub fn check_price() {
    assert_eq!(Price { cents: 250 }, Price { cents: 250 });
}
```

**Read:** [What a test asserts](../what_a_test_asserts/README.md)

### 6. `assert_eq!` on a struct with no `Debug`

```rust,compile_fail
#[derive(PartialEq)]
pub struct Price {
    pub cents: u32,
}

pub fn check_price() {
    assert_eq!(Price { cents: 250 }, Price { cents: 250 });
}
```

```text title="rustc 1.98.0 on assert_eq_without_debug.rs"
error[E0277]: `Price` doesn't implement `Debug`
 --> assert_eq_without_debug.rs:7:5
  |
7 |     assert_eq!(Price { cents: 250 }, Price { cents: 250 });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Debug` is not implemented for `Price`
  |
help: consider annotating `Price` with `#[derive(Debug)]`
  |
2 + #[derive(Debug)]
3 | pub struct Price {
  |

error[E0277]: `Price` doesn't implement `Debug`
 --> assert_eq_without_debug.rs:7:5
  |
7 |     assert_eq!(Price { cents: 250 }, Price { cents: 250 });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Debug` is not implemented for `Price`
  |
help: consider annotating `Price` with `#[derive(Debug)]`
  |
2 + #[derive(Debug)]
3 | pub struct Price {
  |
```

**The mistake.** `assert_eq!` prints both sides when they differ, so both have to be `Debug`. The error appears twice because each side is checked separately.

**The fix.**

```rust,test
#[derive(Debug, PartialEq)]
pub struct Price {
    pub cents: u32,
}

pub fn check_price() {
    assert_eq!(Price { cents: 250 }, Price { cents: 250 });
}
```

**Read:** [What a test asserts](../what_a_test_asserts/README.md)

### 7. A `String` variable as the assertion message

```rust,compile_fail
pub fn check_total(total: u32) {
    let why = String::from("the total should be positive");
    assert!(total > 0, why);
}
```

```text title="rustc 1.98.0 on assert_message_not_a_literal.rs"
error: format argument must be a string literal
 --> assert_message_not_a_literal.rs:3:24
  |
3 |     assert!(total > 0, why);
  |                        ^^^
  |
help: you might be missing a string literal to format with
  |
3 |     assert!(total > 0, "{}", why);
  |                        +++++
```

**The mistake.** Everything after the condition is a `format!` call, and a format call's first argument must be a string literal, even when there is nothing to fill in.

**The fix.**

```rust,test
pub fn check_total(total: u32) {
    let why = String::from("the total should be positive");
    assert!(total > 0, "{why}");
}
```

**Read:** [What a test asserts](../what_a_test_asserts/README.md#the-message)

## Where the test lives

### 8. `use crate::` in a file under `tests/`

```rust,compile_fail
use crate::tally;

pub fn totals_three_scores() {
    assert_eq!(tally(&[5, 3, 0]), 8);
}
```

```text title="rustc 1.98.0 on use_crate_from_integration_test.rs"
error[E0432]: unresolved import `crate::tally`
 --> use_crate_from_integration_test.rs:1:5
  |
1 | use crate::tally;
  |     ^^^^^^^-----
  |            |
  |            no `tally` in the root
```

**The mistake.** In `tests/api.rs`, `crate` is the test crate itself, not your library. The same transcript came from `cargo test --test api` on a package whose `src/lib.rs` does define `tally`.

**The fix.** `ignore`, because it compiles only inside a package named `demo`; there it passed with `cargo test --test api`.

```rust,ignore
use demo::tally; // the package name, with `-` written as `_`

#[test]
fn totals_three_scores() {
    assert_eq!(tally(&[5, 3, 0]), 8);
}
```

**Read:** [Where a test goes](../where_a_test_goes/README.md)

### 9. A test calling a private function from outside its module

```rust,compile_fail
pub mod scoring {
    fn is_spoiled(score: u32) -> bool {
        score > 5
    }
}

pub fn spots_a_spoiled_score() {
    assert!(scoring::is_spoiled(9));
}
```

```text title="rustc 1.98.0 on calling_a_private_function.rs"
error[E0603]: function `is_spoiled` is private
 --> calling_a_private_function.rs:8:22
  |
8 |     assert!(scoring::is_spoiled(9));
  |                      ^^^^^^^^^^ private function
  |
note: the function `is_spoiled` is defined here
 --> calling_a_private_function.rs:2:5
  |
2 |     fn is_spoiled(score: u32) -> bool {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**The mistake.** A test outside the module sees what any other caller sees. An integration test in `tests/` gets this error for every private item, because it is a separate crate.

**The fix.**

```rust,test
pub mod scoring {
    fn is_spoiled(score: u32) -> bool {
        score > 5
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn spots_a_spoiled_score() {
            assert!(is_spoiled(9));
        }
    }
}
```

**Read:** [Where a test goes](../where_a_test_goes/README.md)

### 10. A `#[cfg(test)]` helper called by ordinary code

```rust,compile_fail
#[cfg(test)]
fn sample_prices() -> Vec<u32> {
    vec![250, 399, 1000]
}

pub fn cheapest() -> Option<u32> {
    sample_prices().into_iter().min()
}
```

```text title="rustc 1.98.0 on test_helper_used_outside_tests.rs"
error[E0425]: cannot find function `sample_prices` in this scope
 --> test_helper_used_outside_tests.rs:7:5
  |
7 |     sample_prices().into_iter().min()
  |     ^^^^^^^^^^^^^ not found in this scope
  |
note: found an item that was configured out
 --> test_helper_used_outside_tests.rs:2:4
  |
1 | #[cfg(test)]
  |       ---- the item is gated here
2 | fn sample_prices() -> Vec<u32> {
  |    ^^^^^^^^^^^^^
```

**The mistake.** `#[cfg(test)]` removes the item from every build that is not a test build. `cargo build` fails, and so does plain `cargo test`, which also builds the library the ordinary way for its doc tests. Only `cargo test --lib` passed, in a package checked both ways, which is how the mistake survives an editor that runs just the unit tests.

**The fix.**

```rust,test
pub fn cheapest(prices: &[u32]) -> Option<u32> {
    prices.iter().copied().min()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_prices() -> Vec<u32> {
        vec![250, 399, 1000]
    }

    #[test]
    fn finds_the_cheapest() {
        assert_eq!(cheapest(&sample_prices()), Some(250));
    }
}
```

**Read:** [Where a test goes](../where_a_test_goes/README.md)

## Test doubles

### 11. A fake passed where a concrete type is expected

```rust,compile_fail
pub struct SmtpClient;

impl SmtpClient {
    pub fn send(&self, _to: &str) -> bool {
        true
    }
}

pub fn remind(customers: &[&str], mailer: &SmtpClient) -> usize {
    customers.iter().filter(|c| mailer.send(c)).count()
}

pub struct RecordingMailer;

pub fn reminds_every_customer() {
    assert_eq!(remind(&["acme"], &RecordingMailer), 1);
}
```

```text title="rustc 1.98.0 on fake_where_a_type_is_expected.rs"
error[E0308]: mismatched types
  --> fake_where_a_type_is_expected.rs:16:34
   |
16 |     assert_eq!(remind(&["acme"], &RecordingMailer), 1);
   |                ------            ^^^^^^^^^^^^^^^^ expected `&SmtpClient`, found `&RecordingMailer`
   |                |
   |                arguments to this function are incorrect
   |
   = note: expected reference `&SmtpClient`
              found reference `&RecordingMailer`
note: function defined here
  --> fake_where_a_type_is_expected.rs:9:8
   |
 9 | pub fn remind(customers: &[&str], mailer: &SmtpClient) -> usize {
   |        ^^^^^^                     -------------------
```

**The mistake.** A function that asks for a concrete type accepts that type and nothing else. There is no run-time patching to swap it; the signature has to ask for a trait.

**The fix.**

```rust,test
pub trait Mailer {
    fn send(&self, to: &str) -> bool;
}

pub struct SmtpClient;

impl Mailer for SmtpClient {
    fn send(&self, _to: &str) -> bool {
        true
    }
}

pub fn remind(customers: &[&str], mailer: &impl Mailer) -> usize {
    customers.iter().filter(|c| mailer.send(c)).count()
}

pub struct RecordingMailer;

impl Mailer for RecordingMailer {
    fn send(&self, _to: &str) -> bool {
        true
    }
}

pub fn reminds_every_customer() {
    assert_eq!(remind(&["acme"], &RecordingMailer), 1);
}
```

**Read:** [A test double is a second `impl`](../a_test_double_by_hand/README.md#first-refactor-to-an-interface)

### 12. A recording fake that pushes through `&self`

```rust,compile_fail
pub trait Mailer {
    fn send(&self, to: &str) -> bool;
}

pub struct RecordingMailer {
    pub sent: Vec<String>,
}

impl Mailer for RecordingMailer {
    fn send(&self, to: &str) -> bool {
        self.sent.push(to.to_string());
        true
    }
}
```

```text title="rustc 1.98.0 on recording_fake_without_refcell.rs"
error[E0596]: cannot borrow `self.sent` as mutable, as it is behind a `&` reference
  --> recording_fake_without_refcell.rs:11:9
   |
11 |         self.sent.push(to.to_string());
   |         ^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference in the `impl` method and the `trait` definition
   |
 2 ~     fn send(&mut self, to: &str) -> bool;
 3 | }
...
 9 | impl Mailer for RecordingMailer {
10 ~     fn send(&mut self, to: &str) -> bool {
   |
```

**The mistake.** The real client sends through `&self`, so the trait says `&self`, and a fake that wants to remember the call has nowhere to write. rustc's suggestion, `&mut self` in the trait, changes production code to suit a test.

**The fix.**

```rust,test
use std::cell::RefCell;

pub trait Mailer {
    fn send(&self, to: &str) -> bool;
}

pub struct RecordingMailer {
    pub sent: RefCell<Vec<String>>,
}

impl Mailer for RecordingMailer {
    fn send(&self, to: &str) -> bool {
        self.sent.borrow_mut().push(to.to_string());
        true
    }
}
```

**Read:** [A test double is a second `impl`](../a_test_double_by_hand/README.md#three-fakes-three-jobs)

### 13. A `&dyn Trait` passed to an `&impl Trait` parameter

```rust,compile_fail
pub trait Mailer {
    fn send(&self, to: &str) -> bool;
}

pub fn remind(customers: &[&str], mailer: &impl Mailer) -> usize {
    customers.iter().filter(|c| mailer.send(c)).count()
}

pub fn remind_everyone(mailer: &dyn Mailer) -> usize {
    remind(&["acme", "initech"], mailer)
}
```

```text title="rustc 1.98.0 on dyn_passed_to_impl.rs"
error[E0277]: the size for values of type `dyn Mailer` cannot be known at compilation time
  --> dyn_passed_to_impl.rs:10:34
   |
10 |     remind(&["acme", "initech"], mailer)
   |     ------                       ^^^^^^ doesn't have a size known at compile-time
   |     |
   |     required by a bound introduced by this call
   |
   = help: the trait `Sized` is not implemented for `dyn Mailer`
note: required by an implicit `Sized` bound in `remind`
  --> dyn_passed_to_impl.rs:5:44
   |
 5 | pub fn remind(customers: &[&str], mailer: &impl Mailer) -> usize {
   |                                            ^^^^^^^^^^^ required by the implicit `Sized` requirement on this type parameter in `remind`
help: consider relaxing the implicit `Sized` restriction
   |
 5 | pub fn remind(customers: &[&str], mailer: &impl Mailer + ?Sized) -> usize {
   |                                                        ++++++++
```

**The mistake.** `impl Mailer` in argument position is a generic parameter, and generic parameters are `Sized` unless they say otherwise. `dyn Mailer` is not.

**The fix.**

```rust,test
pub trait Mailer {
    fn send(&self, to: &str) -> bool;
}

pub fn remind(customers: &[&str], mailer: &(impl Mailer + ?Sized)) -> usize {
    customers.iter().filter(|c| mailer.send(c)).count()
}

pub fn remind_everyone(mailer: &dyn Mailer) -> usize {
    remind(&["acme", "initech"], mailer)
}
```

**Read:** [Static and dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md)

## Process state and temporary files

### 14. `std::env::set_var` in edition 2024

```rust,compile_fail
pub fn run_in_test_mode() {
    std::env::set_var("APP_MODE", "test");
}
```

```text title="rustc 1.98.0 on set_var_without_unsafe.rs"
error[E0133]: call to unsafe function `set_var` is unsafe and requires unsafe block
 --> set_var_without_unsafe.rs:2:5
  |
2 |     std::env::set_var("APP_MODE", "test");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior
```

**The mistake.** Edition 2024 made `set_var` `unsafe`. Its docs say the only sound choice in a multi-threaded program outside Windows is not to call it, and a test binary always runs tests on threads.

**The fix.**

```rust,test
pub fn run(mode: &str) -> String {
    format!("running in {mode} mode")
}

pub fn run_in_test_mode() -> String {
    run("test")
}
```

**Read:** [How `cargo test` runs your tests](../how_cargo_test_runs/README.md#tests-in-one-binary-share-a-process)

### 15. Returning a borrowed path out of a helper that owns the `TempDir`

```rust,compile_fail
use std::path::{Path, PathBuf};

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> TempDir {
        TempDir { path: std::env::temp_dir().join("fixture") }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub fn fixture_dir() -> &'static Path {
    let dir = TempDir::new();
    dir.path()
}
```

```text title="rustc 1.98.0 on returning_a_temp_dirs_path.rs"
error[E0515]: cannot return value referencing local variable `dir`
  --> returning_a_temp_dirs_path.rs:19:5
   |
19 |     dir.path()
   |     ---^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `dir` is borrowed here
```

**The mistake.** `dir` is dropped when the helper returns, and a borrowed path would outlive it. This is the version the compiler catches. `dir.path().to_owned()` compiles and hands back the name of a directory that is already gone.

**The fix.**

```rust,test
use std::path::{Path, PathBuf};

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> TempDir {
        TempDir { path: std::env::temp_dir().join("fixture") }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub fn fixture_dir() -> TempDir {
    TempDir::new() // the caller keeps the owner, and the directory with it
}
```

**Read:** [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md#the-directory-lives-exactly-as-long-as-its-owner)

### 16. Borrowing a path from `TempDir::new()` in one statement

```rust,compile_fail
use std::path::{Path, PathBuf};

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> TempDir {
        TempDir { path: std::env::temp_dir().join("fixture") }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {}
}

pub fn config_path() -> PathBuf {
    let dir = TempDir::new().path();
    dir.join("config.txt")
}
```

```text title="rustc 1.98.0 on temp_dir_temporary_borrowed.rs"
error[E0716]: temporary value dropped while borrowed
  --> temp_dir_temporary_borrowed.rs:22:15
   |
22 |     let dir = TempDir::new().path();
   |               ^^^^^^^^^^^^^^       - temporary value is freed at the end of this statement
   |               |
   |               creates a temporary value which is freed while still in use
23 |     dir.join("config.txt")
   |     --- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
22 ~     let binding = TempDir::new();
23 ~     let dir = binding.path();
   |
```

**The mistake.** `TempDir::new()` is a temporary that dies at the `;`, and `dir` borrows from it. rustc's suggested `let binding` compiles, but `config_path` would then return a path into a directory dropped at the function's end.

**The fix.**

```rust,test
use std::path::{Path, PathBuf};

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> TempDir {
        TempDir { path: std::env::temp_dir().join("fixture") }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {}
}

pub fn config_path(dir: &TempDir) -> PathBuf {
    dir.path().join("config.txt")
}
```

**Read:** [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md)

## What no error catches

The costly testing mistakes compile cleanly. Each has a page:

- **A test that cannot fail.** `assert_eq!(average(&[1, 1, 1]), Some(1.0))` passes for every plausible implementation. [What a test asserts](../what_a_test_asserts/README.md#the-trap-the-assertion-that-cannot-fail)
- **A bare `#[should_panic]` that passes on the wrong panic.** [Where a test goes](../where_a_test_goes/README.md)
- **Tests that share the working directory, the environment or a static**, and fail only when run together. [How `cargo test` runs your tests](../how_cargo_test_runs/README.md#tests-in-one-binary-share-a-process)
- **A path to a `TempDir` that is already gone**, via `.to_owned()`, `let _ =`, or a helper that returns only the path. [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md#the-directory-lives-exactly-as-long-as-its-owner)
- **A `#[test]` inside a `harness = false` target**, which is dropped without a warning. [A harness of your own](../a_harness_of_your_own/README.md#switching-it-off)
- **A mock that pins the calls instead of the outcome**, and fails on a refactor that broke nothing. [A test double is a second `impl`](../a_test_double_by_hand/README.md#outcome-tests-and-interaction-tests)
- **A doc test marked `ignore`**, which looks like a test and is a comment. [The example that is a test](../doc_tests/README.md)

## See also

- [Lints around tests](../testing_lints/README.md): the warnings rustc and clippy give on test code, bad and good
- [Testing: courses and links](../resources/README.md): the reading list for this section
- [ERRORS.md](../../ERRORS.md): every error code this library explains, across all sections

## Po polsku

Błędy kompilacji przy pisaniu testów dzielą się na dwie grupy i warto wiedzieć, do której należy ten, który właśnie widzisz. Pierwsza istnieje **tylko w buildzie testowym**: funkcja `#[test]` z parametrem, `async fn` bez środowiska uruchomieniowego, `#[should_panic]` na teście zwracającym `Result`, test zwracający `Option`. W zwykłym `cargo build` tych błędów nie ma, bo wszystko, co oznaczone `#[test]` albo `#[cfg(test)]`, jest wtedy wycinane. Druga grupa to zwykłe błędy Rusta, które pojawiają się akurat przy testach: `assert_eq!` wymaga `PartialEq` i `Debug`, test integracyjny w `tests/` jest osobnym crate'em, więc `use crate::` wskazuje na niego samego, a atrapa (*mock*) przekazana tam, gdzie funkcja żąda konkretnego typu, to po prostu niezgodność typów. Rozwiązaniem nie jest sztuczka, tylko cecha (*trait*) w sygnaturze.

Najdroższe pomyłki w testach kompilują się jednak bez słowa: test, który nie może się nie udać, `#[should_panic]` bez `expected`, testy dzielące katalog roboczy procesu, ścieżka do skasowanego już katalogu tymczasowego. Lista na końcu strony prowadzi do lekcji, które je pokazują.

**Szukaj po polsku:** błędy kompilacji w testach Rusta · `rust test function can not have arguments` · `rust should_panic Result` · `rust assert_eq PartialEq Debug` · `rust integration test use crate` · `rust set_var unsafe 2024`
