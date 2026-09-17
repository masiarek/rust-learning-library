# Lints around tests: bad and good, run

[Testing](../README.md) › **Lints**

**Level:** 201 · a reference, by lint

**One line:** 18 warnings from rustc and clippy that fire on test code, or on code written for tests, each with a program that triggers it, what the tool prints, a silent version, and when the lint is right or noise. One of them is a deny-by-default lint that clippy switches off inside tests.

Every transcript is the output of the command in its title, run on rustc 1.98.0 and clippy 0.1.98 against the program above it saved as `bad.rs`, and every silent version printed nothing under the same command. A title with `--test` means the lint only sees the code in a test build, because every `#[test]` function and `#[cfg(test)]` module is configured out of an ordinary one. The lines dropped from each transcript are the documentation link, the note naming the lint's level, and the closing count. Both programs of every pair are compiled on every build by [`check_fences.py`](../../tools/check_fences.py), as edition 2024, and a fence marked `test` is compiled as a test build too. The build does not re-run clippy, so a later clippy can reword a warning.

## Turning them on

rustc's lints and clippy's default groups need nothing: `cargo clippy --all-targets` prints them, and `--all-targets` is what makes clippy look at test code at all. Plain `cargo clippy` builds without `--test` and skips every test module. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy --all-targets -- -W clippy::should_panic_without_expect -W clippy::ignore_without_reason
```

```toml
[lints.clippy]
should_panic_without_expect = "warn"
ignore_without_reason = "warn"
manual_assert_eq = "warn"
```

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`eq_op`](#eq_op) | clippy · correctness | yes, deny, **but not inside a test** | `assert_eq!(total, total)` |
| [`assertions_on_constants`](#assertions_on_constants) | clippy · style | yes | `assert!(MAX_ITEMS > 0)` on a `const` |
| [`bool_assert_comparison`](#bool_assert_comparison) | clippy · style | yes | `assert_eq!(is_free(0), true)` |
| [`items_after_test_module`](#items_after_test_module) | clippy · style | yes | a `pub fn` after `mod tests` |
| [`test_attr_in_doctest`](#test_attr_in_doctest) | clippy · suspicious | yes | `#[test]` inside a doc-comment example |
| [`dead_code`](#dead_code) | rustc | yes | a helper used only by tests, outside `#[cfg(test)]` |
| [`unused_must_use`](#unused_must_use) | rustc | yes | `std::fs::write(..);` in a test, with the `Result` dropped |
| [`should_panic_without_expect`](#should_panic_without_expect) | clippy · pedantic | no | `#[should_panic]` with no `expected` |
| [`ignore_without_reason`](#ignore_without_reason) | clippy · pedantic | no | `#[ignore]` with no reason |
| [`float_cmp`](#float_cmp) | clippy · pedantic | no | `assert_eq!` on two `f64`s |
| [`manual_assert`](#manual_assert) | clippy · pedantic | no | `if count == 0 { panic!(..) }` |
| [`manual_assert_eq`](#manual_assert_eq) | clippy · pedantic | no | `assert!(total(..) == 649)` |
| [`tests_outside_test_module`](#tests_outside_test_module) | clippy · restriction | no | a `#[test]` function at the top level of a file |
| [`redundant_test_prefix`](#redundant_test_prefix) | clippy · restriction | no | `fn test_totals_prices()` |
| [`assertions_on_result_states`](#assertions_on_result_states) | clippy · restriction | no | `assert!(parse_cents(..).is_ok())` |
| [`missing_assert_message`](#missing_assert_message) | clippy · restriction | no | `assert!(count > 0)` in library code |
| [`cfg_not_test`](#cfg_not_test) | clippy · restriction | no | `#[cfg(not(test))]` |
| [`unwrap_used`](#unwrap_used) | clippy · restriction | no | `.unwrap()` anywhere, tests included |

## The lint that stays quiet in tests

### `eq_op`

**clippy · correctness** · deny by default · fires on `x == x` and `assert_eq!(x, x)`, **except inside a `#[test]` function**

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        let total = total(&[250, 399]);
        assert_eq!(total, total);
    }
}
```

`clippy-driver --edition 2024 --crate-type lib --test bad.rs` printed nothing for that program. The same comparison in an ordinary function:

```rust
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

pub fn check() {
    let t = total(&[250, 399]);
    assert_eq!(t, t);
    let _same = t == t;
}
```

```text title="clippy-driver --edition 2024 --crate-type lib outside.rs — clippy 0.1.98"
error: identical args used in this `assert_eq!` macro call
 --> bad.rs:7:16
  |
7 |     assert_eq!(t, t);
  |                ^^^^
  |

error: equal expressions as operands to `==`
 --> bad.rs:8:17
  |
8 |     let _same = t == t;
  |                 ^^^^^^
  |

```

rustc compiles it; clippy does not. That is an error, and a build that denies it stops. Inside the test above it is silent, so the assertion that cannot fail is the one clippy lets through where it matters most. The good version compares against a value worked out by hand:

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        assert_eq!(total(&[250, 399]), 649);
    }
}
```

**Right, where it runs.** In a test the check is yours: [What a test asserts](../what_a_test_asserts/README.md#the-trap-the-assertion-that-cannot-fail).

## On by default

### `assertions_on_constants`

**clippy · style** · warn by default · fires on `assert!(MAX_ITEMS > 0)` on a `const`

```rust,test
pub const MAX_ITEMS: usize = 50;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_limit_is_positive() {
        assert!(MAX_ITEMS > 0);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test bad.rs — clippy 0.1.98"
warning: this assertion has a constant value
 --> bad.rs:9:9
  |
9 |         assert!(MAX_ITEMS > 0);
  |         ^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: consider moving this into a const block: `const { assert!(..) }`
```

Silent:

```rust,test
pub const MAX_ITEMS: usize = 50;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_limit_is_positive() {
        const { assert!(MAX_ITEMS > 0) };
    }
}
```

**Right.** A test whose assertion is decided at compile time runs, passes, and tests nothing at run time. A `const` block turns it into a build failure instead, which is the stronger claim, and needs no test at all.

### `bool_assert_comparison`

**clippy · style** · warn by default · fires on `assert_eq!(is_free(0), true)`

```rust,test
pub fn is_free(cents: u32) -> bool {
    cents == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_free() {
        assert_eq!(is_free(0), true);
        assert_eq!(is_free(250), false);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test bad.rs — clippy 0.1.98"
warning: used `assert_eq!` with a literal bool
  --> bad.rs:11:9
   |
11 |         assert_eq!(is_free(0), true);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: replace it with `assert!(..)`
   |
11 -         assert_eq!(is_free(0), true);
11 +         assert!(is_free(0));
   |

warning: used `assert_eq!` with a literal bool
  --> bad.rs:12:9
   |
12 |         assert_eq!(is_free(250), false);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: replace it with `assert!(..)`
   |
12 -         assert_eq!(is_free(250), false);
12 +         assert!(!is_free(250));
   |
```

Silent:

```rust,test
pub fn is_free(cents: u32) -> bool {
    cents == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_free() {
        assert!(is_free(0));
        assert!(!is_free(250));
    }
}
```

**Right, and cosmetic.** Both versions fail on the same inputs. The `assert!` form reads as the sentence it means, and on failure prints the condition rather than `left: false, right: true`.

### `items_after_test_module`

**clippy · style** · warn by default · fires on a `pub fn` after `mod tests`

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        assert_eq!(total(&[250, 399]), 649);
    }
}

pub fn average(prices: &[u32]) -> Option<u32> {
    u32::try_from(prices.len()).ok().filter(|&n| n > 0).map(|n| total(prices) / n)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test bad.rs — clippy 0.1.98"
warning: items after a test module
  --> bad.rs:6:1
   |
 6 | mod tests {
   | ^^^^^^^^^
...
15 | pub fn average(prices: &[u32]) -> Option<u32> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: move the items to before the test module was defined
```

Silent:

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

pub fn average(prices: &[u32]) -> Option<u32> {
    u32::try_from(prices.len()).ok().filter(|&n| n > 0).map(|n| total(prices) / n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        assert_eq!(total(&[250, 399]), 649);
    }
}
```

**Right.** Nothing is wrong at compile time, but a reader who reaches `mod tests` stops reading for code. `average` was hidden below the tests.

### `test_attr_in_doctest`

**clippy · suspicious** · warn by default · fires on `#[test]` inside a doc-comment example

```rust
/// Adds two prices.
///
/// ```
/// #[test]
/// fn adds() {
///     assert_eq!(my_crate::add(250, 399), 649);
/// }
/// ```
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — clippy 0.1.98"
warning: unit tests in doctest are not executed
 --> bad.rs:4:5
  |
4 | /// #[test]
  |     ^^^^^^^
  |
```

Silent:

```rust
/// Adds two prices.
///
/// ```
/// assert_eq!(my_crate::add(250, 399), 649);
/// ```
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

**Right.** A doc test is already a test. A `#[test]` function inside one is defined and never called, so the example looks checked and checks nothing. Write the assertion at the top level of the example.

### `dead_code`

**rustc** · warn by default · fires on a helper used only by tests, outside `#[cfg(test)]`

```rust
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

fn sample_prices() -> Vec<u32> {
    vec![250, 399]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_the_sample() {
        assert_eq!(total(&sample_prices()), 649);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — clippy 0.1.98"
warning: function `sample_prices` is never used
 --> bad.rs:5:4
  |
5 | fn sample_prices() -> Vec<u32> {
  |    ^^^^^^^^^^^^^
  |
```

Silent:

```rust
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_prices() -> Vec<u32> {
        vec![250, 399]
    }

    #[test]
    fn totals_the_sample() {
        assert_eq!(total(&sample_prices()), 649);
    }
}
```

**Right.** In a normal build nothing calls the helper, so it is dead there, and ships in the binary. Moving it into the test module, or marking it `#[cfg(test)]`, removes the warning and the code.

### `unused_must_use`

**rustc** · warn by default · fires on `std::fs::write(..);` in a test, with the `Result` dropped

```rust,test
#[cfg(test)]
mod tests {
    #[test]
    fn writes_a_fixture() {
        let path = std::env::temp_dir().join("unused_must_use_fixture.txt");
        std::fs::write(&path, "apple=3\n");
        assert!(path.exists());
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test bad.rs — clippy 0.1.98"
warning: unused `std::result::Result` that must be used
 --> bad.rs:6:9
  |
6 |         std::fs::write(&path, "apple=3\n");
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
help: use `let _ = ...` to ignore the resulting value
  |
6 |         let _ = std::fs::write(&path, "apple=3\n");
  |         +++++++
```

Silent:

```rust,test
#[cfg(test)]
mod tests {
    #[test]
    fn writes_a_fixture() {
        let path = std::env::temp_dir().join("unused_must_use_fixture.txt");
        std::fs::write(&path, "apple=3\n").unwrap();
        assert!(path.exists());
    }
}
```

**Right, always, in tests.** A fixture write that failed is ignored, and the test goes on to fail somewhere else, or pass for the wrong reason. In a test, `.unwrap()` is the handling: it turns the setup failure into the test's failure, with the error in the message.

## Pedantic: worth turning on for test code

### `should_panic_without_expect`

**clippy · pedantic** · allow by default · fires on `#[should_panic]` with no `expected`

```rust,test
pub fn price_of(prices: &[u32], index: usize) -> u32 {
    prices[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn indexing_past_the_end_panics() {
        price_of(&[250], 9);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::should_panic_without_expect bad.rs — clippy 0.1.98"
warning: #[should_panic] attribute without a reason
  --> bad.rs:10:5
   |
10 |     #[should_panic]
   |     ^^^^^^^^^^^^^^^ help: consider specifying the expected panic: `#[should_panic(expected = /* panic message */)]`
   |
```

Silent:

```rust,test
pub fn price_of(prices: &[u32], index: usize) -> u32 {
    prices[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn indexing_past_the_end_panics() {
        price_of(&[250], 9);
    }
}
```

**Right, and worth turning on for test code.** A bare `#[should_panic]` passes on any panic, including one the test was never about. [Where a test goes](../where_a_test_goes/README.md) has a practice built on exactly this.

### `ignore_without_reason`

**clippy · pedantic** · allow by default · fires on `#[ignore]` with no reason

```rust,test
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn sums_a_million_prices() {
        let total: u64 = (0..1_000_000u64).sum();
        assert_eq!(total, 499_999_500_000);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::ignore_without_reason bad.rs — clippy 0.1.98"
warning: `#[ignore]` without reason
 --> bad.rs:4:5
  |
4 |     #[ignore]
  |     ^^^^^^^^^
  |
  = help: add a reason with `= ".."`
```

Silent:

```rust,test
#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "slow; run with cargo test -- --ignored"]
    fn sums_a_million_prices() {
        let total: u64 = (0..1_000_000u64).sum();
        assert_eq!(total, 499_999_500_000);
    }
}
```

**Right.** The reason string is printed on every run beside `ignored`, and it is the only place anyone learns whether the test is slow, flaky or waiting for a bug fix.

### `float_cmp`

**clippy · pedantic** · allow by default · fires on `assert_eq!` on two `f64`s

```rust,test
pub fn average(prices: &[f64]) -> f64 {
    prices.iter().sum::<f64>() / prices.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages_three_prices() {
        assert_eq!(average(&[0.1, 0.2, 0.3]), 0.2);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::float_cmp bad.rs — clippy 0.1.98"
warning: strict comparison of `f32` or `f64`
  --> bad.rs:11:9
   |
11 |         assert_eq!(average(&[0.1, 0.2, 0.3]), 0.2);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
```

Silent:

```rust,test
pub fn average(prices: &[f64]) -> f64 {
    prices.iter().sum::<f64>() / prices.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages_three_prices() {
        assert!((average(&[0.1, 0.2, 0.3]) - 0.2).abs() < 1e-9);
    }
}
```

**Right for computed values, noise for exact ones.** An average of `0.1, 0.2, 0.3` is not bit-for-bit `0.2`; this test fails. Comparing a float that was copied rather than computed, or a sentinel like `0.0`, is exact and fine. [What a test asserts](../what_a_test_asserts/README.md#floats-is-the-wrong-assertion) runs two computed comparisons, one equal by luck and one not.

### `manual_assert`

**clippy · pedantic** · allow by default · fires on `if count == 0 { panic!(..) }`

```rust
pub fn check_stock(count: u32) {
    if count == 0 {
        panic!("out of stock");
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::manual_assert bad.rs — clippy 0.1.98"
warning: only a `panic!` in `if`-then statement
 --> bad.rs:2:5
  |
2 | /     if count == 0 {
3 | |         panic!("out of stock");
4 | |     }
  | |_____^
  |
help: replace `if`-then-`panic!` with `assert!`
  |
2 -     if count == 0 {
3 -         panic!("out of stock");
4 -     }
2 +     assert!(count != 0, "out of stock");
  |
```

Silent:

```rust
pub fn check_stock(count: u32) {
    assert!(count != 0, "out of stock");
}
```

**Mostly right.** `assert!` states the invariant rather than its negation, and the lint fires only when the `if` holds nothing but the `panic!`, so it rarely points at code that reads better the long way.

### `manual_assert_eq`

**clippy · pedantic** · allow by default · fires on `assert!(total(..) == 649)`

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        assert!(total(&[250, 399]) == 649);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::manual_assert_eq bad.rs — clippy 0.1.98"
warning: used `assert!` with an equality comparison
  --> bad.rs:11:9
   |
11 |         assert!(total(&[250, 399]) == 649);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: replace it with `assert_eq!(..)`
   |
11 -         assert!(total(&[250, 399]) == 649);
11 +         assert_eq!(total(&[250, 399]), 649);
   |
```

Silent:

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        assert_eq!(total(&[250, 399]), 649);
    }
}
```

**Right.** The same test with worse failure output: `assert!` prints the expression, `assert_eq!` prints both values. [What a test asserts](../what_a_test_asserts/README.md#assert_eq-over-assert-whenever-both-sides-are-values) shows the two messages side by side.

## Restriction: a policy, not a defect

### `tests_outside_test_module`

**clippy · restriction** · allow by default · fires on a `#[test]` function at the top level of a file

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[test]
fn totals_prices() {
    assert_eq!(total(&[250, 399]), 649);
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::tests_outside_test_module bad.rs — clippy 0.1.98"
warning: this function marked with #[test] is outside a #[cfg(test)] module
 --> bad.rs:6:1
  |
6 | / fn totals_prices() {
7 | |     assert_eq!(total(&[250, 399]), 649);
8 | | }
  | |_^
  |
  = note: move it to a testing module marked with #[cfg(test)]
```

Silent:

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        assert_eq!(total(&[250, 399]), 649);
    }
}
```

**A convention, not a defect.** It compiles and runs exactly the same. The module is what gives test-only helpers and imports a place that disappears from the normal build. Fine to leave off in `tests/` files, which are test-only anyway.

### `redundant_test_prefix`

**clippy · restriction** · allow by default · fires on `fn test_totals_prices()`

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totals_prices() {
        assert_eq!(total(&[250, 399]), 649);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::redundant_test_prefix bad.rs — clippy 0.1.98"
warning: redundant `test_` prefix in test function name
  --> bad.rs:10:8
   |
10 |     fn test_totals_prices() {
   |        ^^^^^^^^^^^^^^^^^^ help: consider removing the `test_` prefix: `totals_prices`
   |
```

Silent:

```rust,test
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_prices() {
        assert_eq!(total(&[250, 399]), 649);
    }
}
```

**Taste.** The harness prints `tests::test_totals_prices`, and the prefix says nothing the module and the attribute have not already said. A habit from pytest and JUnit 3, where the prefix was how tests were found.

### `assertions_on_result_states`

**clippy · restriction** · allow by default · fires on `assert!(parse_cents(..).is_ok())`

```rust,test
pub fn parse_cents(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cents() {
        assert!(parse_cents("250").is_ok());
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::assertions_on_result_states bad.rs — clippy 0.1.98"
warning: called `assert!` with `Result::is_ok`
  --> bad.rs:11:9
   |
11 |         assert!(parse_cents("250").is_ok());
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `parse_cents("250").unwrap()`
   |
```

Silent:

```rust,test
pub fn parse_cents(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cents() {
        assert_eq!(parse_cents("250"), Ok(250));
    }
}
```

**Right about the message, and there is a better fix than its suggestion.** `assert!(r.is_ok())` fails with `assertion failed: parse_cents("250").is_ok()` and throws the error away. clippy suggests `.unwrap()`, which prints the error. `assert_eq!(r, Ok(250))` also checks the value.

### `missing_assert_message`

**clippy · restriction** · allow by default · fires on `assert!(count > 0)` in library code

```rust
pub fn check_stock(count: u32) {
    assert!(count > 0);
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::missing_assert_message bad.rs — clippy 0.1.98"
warning: assert without any message
 --> bad.rs:2:5
  |
2 |     assert!(count > 0);
  |     ^^^^^^^^^^^^^^^^^^
  |
  = help: consider describing why the failing assert is problematic
```

Silent:

```rust
pub fn check_stock(count: u32) {
    assert!(count > 0, "out of stock");
}
```

**Right in library code, noise in tests.** A test's name is already its message. A precondition in a library panics in someone else's program, where the message is all they get.

### `cfg_not_test`

**clippy · restriction** · allow by default · fires on `#[cfg(not(test))]`

```rust
#[cfg(not(test))]
pub fn config_path() -> &'static str {
    "/etc/shop/config.txt"
}

#[cfg(test)]
pub fn config_path() -> &'static str {
    "tests/fixtures/config.txt"
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::cfg_not_test bad.rs — clippy 0.1.98"
warning: code is excluded from test builds
 --> bad.rs:1:1
  |
1 | #[cfg(not(test))]
  | ^^^^^^^^^^^^^^^^^
  |
  = help: consider not excluding any code from test builds
  = note: this could increase code coverage despite not actually being tested
```

Silent:

```rust
pub fn load_config(path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

**Right.** Code under `#[cfg(not(test))]` is exactly the code tests never see, and a test-only twin of it tests the twin. Pass the path in instead, as the good version does: [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md#the-function-has-to-take-the-path).

### `unwrap_used`

**clippy · restriction** · allow by default · fires on `.unwrap()` anywhere, tests included

```rust,test
pub fn parse_cents(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cents() {
        assert_eq!(parse_cents("250").unwrap(), 250);
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib --test -W clippy::unwrap_used bad.rs — clippy 0.1.98"
warning: used `unwrap()` on a `Result` value
  --> bad.rs:11:20
   |
11 |         assert_eq!(parse_cents("250").unwrap(), 250);
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: if this value is an `Err`, it will panic
   = help: consider using `expect()` to provide a better panic message
```

The silent version is the same file, beside this `clippy.toml`:

```toml
allow-unwrap-in-tests = true
```

**Right outside tests, noise inside them.** In a test, `unwrap` is the assertion that setup worked. The good version is the same code: `allow-unwrap-in-tests = true` in `clippy.toml` keeps the lint on for the library and off for `#[test]` functions and `#[cfg(test)]` modules. [Strict lints](../../05_Tooling/strict_lints/README.md) sets this up for a whole project.

## What no lint catches

- **A test whose expected value every implementation produces.** `assert_eq!(average(&[1, 1, 1]), Some(1.0))` is not `eq_op`: the two sides are different expressions. [What a test asserts](../what_a_test_asserts/README.md#the-trap-the-assertion-that-cannot-fail)
- **`#[should_panic(expected = "index")]`** that matches a different panic whose message happens to contain the substring. The lint only checks that `expected` exists.
- **Tests that share process state.** No lint follows `set_current_dir`, a `static` or a fixed temp path across two test functions. [How `cargo test` runs your tests](../how_cargo_test_runs/README.md#tests-in-one-binary-share-a-process)
- **A `TempDir` dropped too early** through `.to_owned()` or `let _ =`: the `PathBuf` is a valid value, and a stand-in without `#[must_use]` gives `let _ =` nothing to warn about. [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md#the-directory-lives-exactly-as-long-as-its-owner)
- **A mock expectation that pins the implementation.** `times(3)` is valid code that fails on a harmless refactor. [A test double is a second `impl`](../a_test_double_by_hand/README.md#outcome-tests-and-interaction-tests)
- **A `#[test]` in a `harness = false` target**, which neither rustc nor clippy mentions. [A harness of your own](../a_harness_of_your_own/README.md#switching-it-off)

## See also

- [Every testing error, and its fix](../testing_errors/README.md): the errors, where these are the warnings
- [Strict lints](../../05_Tooling/strict_lints/README.md): a whole-project lint set, and where `unwrap` is allowed again
- [Testing: courses and links](../resources/README.md): the reading list for this section

## Po polsku

Lint to po polsku najczęściej po prostu „lint” albo „ostrzeżenie Clippy”, i to wystarczy do szukania. Najważniejsza rzecz na tej stronie jest zaskakująca: `clippy::eq_op`, lint z grupy *correctness*, który poza testami zatrzymuje build na `assert_eq!(x, x)`, wewnątrz funkcji `#[test]` milczy. Asercja, która nie może się nie udać, przechodzi więc bez słowa dokładnie tam, gdzie szkodzi najbardziej. Druga rzecz praktyczna: samo `cargo clippy` w ogóle nie zagląda do testów, bo buduje bez `--test`, a moduł `#[cfg(test)]` jest wtedy wycinany. Potrzebne jest `cargo clippy --all-targets`.

Z lintów domyślnie wyłączonych dla kodu testów najwięcej dają dwa z grupy *pedantic*: `should_panic_without_expect` (gołe `#[should_panic]` przechodzi przy każdej panice) i `ignore_without_reason` (powód przy `#[ignore]` jest jedyną informacją, dlaczego test nie biegnie). `unwrap_used` z grupy *restriction* ma w `clippy.toml` ustawienie `allow-unwrap-in-tests`, dzięki któremu zostaje włączony dla biblioteki i wyłączony w testach, gdzie `unwrap` jest właśnie asercją.

**Szukaj po polsku:** lints Clippy w testach · `cargo clippy --all-targets` · `clippy should_panic_without_expect` · `clippy allow-unwrap-in-tests` · `clippy eq_op test`
