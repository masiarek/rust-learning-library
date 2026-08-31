# Where a test goes

**Level:** 201 · working knowledge

**One line:** Beside the code in a `#[cfg(test)] mod tests`, or outside it in `tests/` — and the one difference that decides it is whether the test needs to see private items.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_a_ballot() {
        assert_eq!(tally(&[5, 3, 0]), 8);
    }
}
```

| | Lives in | Sees |
|---|---|---|
| **unit test** | `#[cfg(test)] mod tests` inside `src/` | everything, private items included |
| **integration test** | a file directly in `tests/` | the public API only — it is a separate crate |
| **doc test** | a ` ``` ` block in a `///` comment | the public API only, and it is printed in the docs |

`cargo test` runs all three in one command.

## `#[cfg(test)]` is not a run-time skip

The module is compiled **only** under `cargo test`. In a normal build it does not exist — not "exists and is skipped" — so test-only helpers, fixtures and `[dev-dependencies]` cost the shipped binary nothing at all.

That is also why `use super::*;` at the top of a test module is the one glob nobody argues about: the test module is a *child* of the module under test, so it can see the private items, and there is exactly one place those names can have come from.

## The two attributes worth knowing on day one

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn indexing_past_the_end_panics() { /* … */ }

#[test]
#[ignore = "slow; run with --ignored"]
fn the_expensive_one() { /* … */ }
```

`#[ignore]` compiles the test and does not run it; `cargo test -- --ignored` runs exactly those, and the reason string is printed in the summary.

`#[should_panic]` **without** `expected` is the trap. It passes on *any* panic — including one introduced by the very refactor the test was written to catch. The practice below has a function that panics two completely different ways, and a bare `#[should_panic]` is green for both.

Run the example on this page with `rustc --test` and the harness prints:

```text title="One run of rustc --edition 2024 --test where_a_test_goes.rs — the harness runs tests in parallel, so the order varies"
running 4 tests
test tests::the_expensive_one ... ignored, slow; run with --ignored
test tests::spots_a_spoiled_score ... ok
test tests::totals_a_ballot ... ok
test tests::indexing_past_the_end_panics - should panic ... ok

test result: ok. 3 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## The `tests/` directory has one rule people trip on

Every `.rs` file **directly in** `tests/` is compiled as its own crate. A subdirectory is not — which is exactly why shared helpers go in `tests/common/mod.rs` rather than `tests/common.rs`, since the second would be compiled and run as a test crate of its own and report zero tests.

## Which to write

Ask one question: **can the behaviour be observed through the public API?**

- **Yes** → `tests/`. An integration test survives a refactor of the internals, which is what makes it worth keeping for years.
- **No** → `#[cfg(test)]` beside the code. And then ask whether the thing you are testing should be public, because sometimes the honest answer is that the API is missing a method.

## If you are coming from another language

- **Python.** `pytest` discovers `test_*.py` files anywhere and imports them, so the unit/integration split is a directory convention rather than a language rule, and a test can reach `_private` names freely. Rust makes the boundary real: an integration test genuinely cannot see private items, which turns "should this be public?" from a style question into a compile error you have to answer. The nearest Python equivalent of `#[cfg(test)]` is that test files are simply not shipped in the wheel — same outcome, enforced by packaging rather than by the compiler. `pytest.mark.skip` is `#[ignore]`, and `pytest.raises(ValueError, match="…")` is `#[should_panic(expected = "…")]`, including the reason the `match` argument exists.
- **ABAP.** A local test class in the `CCIMP`/test include of a class pool is the unit test — it is compiled only for ABAP Unit, exactly like `#[cfg(test)]`, and it can be a `FRIEND` of the class under test to reach private members, which is `use super::*` written out. A separate global test class calling only public methods is the integration test. `FOR TESTING` is `#[test]`, and the `RISK LEVEL`/`DURATION` annotations are roughly `#[ignore]`'s job of keeping the slow ones out of the fast run. The habit that transfers best: ABAP developers already know that making a class a test friend is a smell if the public API could have shown the behaviour, and that is this page's closing question.
- **Java.** `src/test/java` mirroring `src/main/java` gives tests package-private access, which sits between Rust's two options — closer to the unit test. There is no equivalent of a test file that genuinely cannot see internals unless you deliberately test from another package.

---

## The verified output

<!-- output:where_a_test_goes -->
*Verified output of [`where_a_test_goes.rs`](examples/where_a_test_goes.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The two places, and the one difference that decides it
   src/lib.rs        #[cfg(test)] mod tests   -> UNIT tests
   tests/api.rs      no attribute needed      -> INTEGRATION tests
   A unit test is inside the module, so it can call private items.
   An integration test is a separate crate that `use`s yours, so it
   sees exactly what a real user sees — which is the point of it.
   Test the behaviour from outside; test the awkward internals from
   inside, and only when the outside cannot reach them.
   tally(&[5, 3, 0]) = 8, is_spoiled(9) = true

2. `#[cfg(test)]` is not a run-time skip
   The `tests` module above is compiled ONLY under `cargo test`.
   In this binary it does not exist: cfg!(test) = false
   So test-only helpers, fixtures and dev-dependencies cost the
   shipped artefact nothing at all.

3. `use super::*` is the one glob nobody argues about
   The test module is a CHILD of the module under test, so it can
   see its parent's private items — but it still has to name them.
   `use super::*;` is the first line of almost every test module in
   Rust, and it is safe for the reason globs usually are not: there
   is exactly one place those names can have come from.

4. The two attributes worth knowing on day one
   #[should_panic(expected = "…")]  the test PASSES if it panics,
     and the message must contain that substring. Without `expected`
     it passes on ANY panic, including one from a typo — which is
     how a should_panic test quietly stops testing anything.
   #[ignore = "reason"]  compiled, not run; `cargo test -- --ignored`
     runs exactly these. The reason string is printed in the summary.

5. What `cargo test` runs, in one list
   a. unit tests      #[test] fns anywhere in src/, usually in a
                      #[cfg(test)] module
   b. integration     every .rs file directly in tests/, each its own
                      crate; tests/common/mod.rs for shared helpers,
                      because a subdirectory is not compiled as a test
   c. doc tests       the ``` blocks in your /// comments
   All three in one command, and the doc tests are the ones people
   forget they are shipping.
```
<!-- /output -->

## Practice

**The test that could not see it, and the one that passed for the wrong panic.** Write a `tally` that is public and a `parse_cell` that is private, then write a unit test for each. Say what an integration test in `tests/` would have to write to reach `parse_cell`, and what happens when it tries.

Then write a function that panics two different ways depending on its argument, and put a bare `#[should_panic]` test on the wrong one. It passes. Explain in one sentence what that test is now guarding, and fix it.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:where_a_test_goes_kata -->
*[`where_a_test_goes_kata.rs`](examples/where_a_test_goes_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the test that could not see it, and the one that passed
//! for the wrong panic.
//!
//!   rustc --edition 2024 where_a_test_goes_kata.rs -o /tmp/wtgk && /tmp/wtgk
//!   rustc --edition 2024 --test where_a_test_goes_kata.rs -o /tmp/wtgt && /tmp/wtgt

/// Public: an integration test in tests/ can call this.
pub fn tally(line: &str) -> Option<u32> {
    let cells: Vec<&str> = line.split(',').collect();
    let mut total = 0;
    for c in &cells {
        total += parse_cell(c)?;
    }
    Some(total)
}

/// Private: only a test inside this module can call it.
fn parse_cell(cell: &str) -> Option<u32> {
    match cell.trim().parse::<u32>() {
        Ok(n) if n <= 5 => Some(n),
        _ => None,
    }
}

fn panics_two_ways(mode: u32) -> u32 {
    let scores = [5u32, 3];
    if mode == 0 {
        scores[std::hint::black_box(9)]          // index out of bounds
    } else {
        panic!("the ballot was never counted")   // a completely different bug
    }
}

fn caught(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(f);
    std::panic::set_hook(hook);
    match r {
        Ok(()) => "(no panic)".into(),
        Err(e) => e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "(non-string panic)".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_a_good_line() {
        assert_eq!(tally("5,3,0"), Some(8));
    }

    /// Only reachable from in here. An integration test cannot name it.
    #[test]
    fn rejects_a_score_above_five() {
        assert_eq!(parse_cell("9"), None);
        assert_eq!(parse_cell(" 3 "), Some(3));
    }

    /// The careless version: passes on ANY panic.
    #[test]
    #[should_panic]
    fn out_of_bounds_careless() {
        panics_two_ways(1);   // not the panic this test is named after
    }

    /// The version that says which panic it means.
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn out_of_bounds_careful() {
        panics_two_ways(0);
    }
}

fn main() {
    println!("1. What each kind of test can reach");
    println!("   tally(\"5,3,0\")   = {:?}   pub — an integration test can call it",
             tally("5,3,0"));
    println!("   parse_cell(\"9\")  = {:?}        private — only a unit test can",
             parse_cell("9"));
    println!("   An integration test in tests/api.rs is a SEPARATE CRATE. It writes");
    println!("   `use my_crate::tally;` and there is no spelling of parse_cell that");
    println!("   works: E0603, private function. That is the boundary doing its");
    println!("   job, and the reason to put the parse_cell tests inline.");

    println!();
    println!("2. The should_panic that tests nothing");
    println!("   panics_two_ways(0) -> {}", caught(|| { panics_two_ways(0); }));
    println!("   panics_two_ways(1) -> {}", caught(|| { panics_two_ways(1); }));
    println!("   A bare #[should_panic] passes on BOTH. So a test named");
    println!("   `out_of_bounds` goes green when the function panics for an");
    println!("   entirely unrelated reason — including a panic introduced by the");
    println!("   very refactor the test was there to catch.");
    println!("   #[should_panic(expected = \"index out of bounds\")] passes on the");
    println!("   first only. The `expected` string is a SUBSTRING match, so it need");
    println!("   not be the whole message, and it should be the part that is about");
    println!("   the failure rather than the part about the data.");

    println!();
    println!("3. Running this file as a test binary");
    println!("   rustc --edition 2024 --test where_a_test_goes_kata.rs");
    println!("   ...gives the harness that `cargo test` would run for the unit");
    println!("   half. The integration half needs a package, because a separate");
    println!("   crate needs something to link against.");

    println!();
    println!("4. Where to put a test, decided in one question");
    println!("   Can the behaviour be observed through the public API?");
    println!("     yes -> tests/, as an integration test. It survives a refactor of");
    println!("            the internals, which is what makes it worth keeping.");
    println!("     no  -> #[cfg(test)] beside the code. And then ask whether the");
    println!("            thing being tested should be public, because sometimes");
    println!("            the honest answer is that the API is missing a method.");
}
```
<!-- /source -->

<!-- output:where_a_test_goes_kata -->
*Verified output of [`where_a_test_goes_kata.rs`](examples/where_a_test_goes_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. What each kind of test can reach
   tally("5,3,0")   = Some(8)   pub — an integration test can call it
   parse_cell("9")  = None        private — only a unit test can
   An integration test in tests/api.rs is a SEPARATE CRATE. It writes
   `use my_crate::tally;` and there is no spelling of parse_cell that
   works: E0603, private function. That is the boundary doing its
   job, and the reason to put the parse_cell tests inline.

2. The should_panic that tests nothing
   panics_two_ways(0) -> index out of bounds: the len is 2 but the index is 9
   panics_two_ways(1) -> the ballot was never counted
   A bare #[should_panic] passes on BOTH. So a test named
   `out_of_bounds` goes green when the function panics for an
   entirely unrelated reason — including a panic introduced by the
   very refactor the test was there to catch.
   #[should_panic(expected = "index out of bounds")] passes on the
   first only. The `expected` string is a SUBSTRING match, so it need
   not be the whole message, and it should be the part that is about
   the failure rather than the part about the data.

3. Running this file as a test binary
   rustc --edition 2024 --test where_a_test_goes_kata.rs
   ...gives the harness that `cargo test` would run for the unit
   half. The integration half needs a package, because a separate
   crate needs something to link against.

4. Where to put a test, decided in one question
   Can the behaviour be observed through the public API?
     yes -> tests/, as an integration test. It survives a refactor of
            the internals, which is what makes it worth keeping.
     no  -> #[cfg(test)] beside the code. And then ask whether the
            thing being tested should be public, because sometimes
            the honest answer is that the API is missing a method.
```
<!-- /output -->

</details>

---

## See also

- [What a test asserts](../what_a_test_asserts/README.md) — the assertions the tests above are made of
- [The example that is a test](../doc_tests/README.md) — the third kind, which runs in the same command
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — `#[cfg]`, `#[test]` and the lint levels
- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — the privacy boundary this page's split is built on
- [cargo-nextest](../../05_Tooling/nextest/README.md) — a different harness, one process per test
- [Testing a command](../../03_Command_Line/testing_a_command/README.md) — integration testing a binary rather than a library

## Sources

[Unit testing ↗](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html), [Integration testing ↗](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html) and [Development dependencies ↗](https://doc.rust-lang.org/rust-by-example/testing/dev_dependencies.html) in Rust by Example; [Test Organization ↗](https://doc.rust-lang.org/book/ch11-03-test-organization.html) in The Book for the `tests/common/mod.rs` rule.

## Po polsku

W polskich materiałach „test jednostkowy” i „test integracyjny” opisują przede wszystkim **zakres** — czy sprawdzamy jedną funkcję, czy współpracę kilku kawałków systemu. W Ruscie te same słowa znaczą coś węższego i to jest pierwsza rzecz do przestawienia w głowie: rozstrzyga **miejsce pliku i to, co ten plik widzi**. Test jednostkowy leży obok kodu, w `#[cfg(test)] mod tests` wewnątrz `src/`, i widzi wszystko — również elementy prywatne. Test integracyjny to plik bezpośrednio w katalogu `tests/`, kompilowany jako **osobny crate**, więc widzi dokładnie tyle, co zwykły użytkownik biblioteki: publiczne API. Pytanie, które wybiera za nas, jest jedno: czy to zachowanie da się zaobserwować przez publiczne API? Jeśli tak — `tests/`, bo taki test przeżyje przebudowę wnętrza modułu. Jeśli nie — `#[cfg(test)]` obok kodu, a przy okazji warto zapytać, czy testowana rzecz nie powinna po prostu być publiczna.

`#[cfg(test)]` **nie jest pominięciem w czasie działania**. To nie jest odpowiednik `if not TESTING: return` — moduł testowy w zwykłej kompilacji nie jest „obecny i pomijany”, tylko w ogóle nie powstaje (w wydanym binarium `cfg!(test)` to `false`), więc pomocnicze funkcje testowe, dane testowe i `[dev-dependencies]` nie kosztują wysyłanego artefaktu ani bajta. Stąd bierze się też jedyny glob, o który nikt się nie kłóci: `use super::*;`. Moduł testowy jest **dzieckiem** modułu testowanego, więc sięga po jego prywatne nazwy, a przy tym istnieje tylko jedno miejsce, z którego te nazwy mogą pochodzić.

Pułapka tej strony to `#[should_panic]` bez `expected`. Taki test przechodzi przy **dowolnej** panice — także tej, którą wprowadził refaktor, przed którym ów test miał chronić. W ćwiczeniu funkcja `panics_two_ways` panikuje na dwa zupełnie różne sposoby (`index out of bounds: the len is 2 but the index is 9` oraz `the ballot was never counted`), a goły `#[should_panic]` świeci się na zielono dla obu. Właściwa forma to `#[should_panic(expected = "index out of bounds")]`, przy czym `expected` dopasowuje **podłańcuch**, więc wystarczy fragment komunikatu — i niech to będzie fragment mówiący o awarii, a nie o danych. Kto zna `pytest`, ma gotową analogię: `pytest.raises(ValueError, match="…")` istnieje z dokładnie tego samego powodu. Druga pułapka siedzi w katalogu `tests/`: osobnym crate'em jest każdy plik `.rs` leżący **bezpośrednio** w nim, a podkatalog już nie — dlatego wspólne funkcje pomocnicze trafiają do `tests/common/mod.rs`, a nie do `tests/common.rs`, który zostałby skompilowany jako własny crate testowy i zgłosił zero testów.

**Szukaj po polsku:** testy jednostkowe w Ruscie · testy integracyjne · moduł testowy · `rust cfg test module` · `rust should_panic expected` · `rust tests common mod.rs`
