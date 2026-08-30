# The example that is a test

**Level:** 201 · working knowledge

**One line:** Every ` ``` ` block in a `///` comment is compiled and run by `cargo test` — which is the answer to the oldest problem in documentation, the example that stopped compiling two releases ago and nobody noticed.

````rust
/// Totals a ballot.
///
/// ```
/// assert_eq!(my_crate::tally(&[5, 3, 0]), 8);
/// ```
pub fn tally(scores: &[u32]) -> u32 {
    scores.iter().sum()
}
````

Rename the function, change the return type, break the arithmetic — the documentation fails the build.

## Each block is a whole program

A doc test is wrapped in `fn main()` for you and compiled as **its own crate**, linked against yours. Two things follow, and both surprise people once:

- It must `use` your crate **by name** (`use my_crate::tally;`), never `use crate::…`.
- It sees the **public API only**. A doc test is an integration test that happens to be printed in the docs — which makes it the cheapest way to discover that something you meant to export is still private.

## `#` hides a line from the reader, not from the compiler

````rust
/// ```
/// # use my_crate::read_cell;
/// # let line = "5,3,0";
/// let cells: Vec<&str> = line.split(',').collect();
/// assert_eq!(read_cell(&cells, 1), Some(3));
/// ```
````

The rendered example is two lines; the compiled one is four. That is the right use — the import is noise a reader can reconstruct.

What `#` must **not** hide is the part that makes the example work. An example whose visible lines do not run when copied is worse than no example, because it looks like one that would.

## The five fences

| Fence | Compiled | Run |
|---|---|---|
| ` ``` ` | yes | yes |
| ` ```no_run ` | yes | no — it opens a socket, or takes an hour |
| ` ```should_panic ` | yes | yes, and must panic |
| ` ```ignore ` | **no** | no |
| ` ```text ` | not Rust at all | — |

`ignore` is the one to distrust: it looks like a test and is a comment. If the block is not Rust, say `text`; if it is Rust that cannot run here, say `no_run`. Reserve `ignore` for the rare block that is real Rust, is worth showing, and genuinely cannot compile — and make it carry a note saying why.

## What they are for, and what they are not

**Good:** the two-line example a reader needs, kept honest. A failing doc test is a documentation bug, which is exactly the right thing to be told about.

**Not:** coverage. They are slower than unit tests — one compilation each — they only reach the public API, and a doc comment full of edge cases is a bad doc comment. Put the first case on the page and the third through twentieth in `#[cfg(test)]`.

Run against the example on this page, `rustdoc --test` prints:

```text title="rustdoc --edition 2024 --test doc_tests.rs -L . --extern doc_tests=libdoc_tests.rlib — one run; the order varies"
running 5 tests
test doc_tests.rs - read_cell (line 42) - should panic ... ok
test doc_tests.rs - read_cell (line 35) ... ok
test doc_tests.rs - tally (line 14) ... ok
test doc_tests.rs - read_cell (line 26) ... ok
test doc_tests.rs - tally (line 8) ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Note what the harness names each test: the **line** the fence starts on. That is the whole naming scheme, and it is why a doc test failure tells you where to look but not what the case was called.

## If you are coming from another language

- **Python.** This is `doctest`, and Rust took the idea and made it the default rather than an opt-in module. Three differences worth carrying over. Python's doctest compares **printed output** against what follows the `>>>`, so whitespace and `repr()` formatting are part of the test; Rust's runs assertions, so nothing is fragile about spacing. Python's runs in the module's own namespace, so it sees privates; Rust's is a separate crate and does not. And `pytest --doctest-modules` is opt-in, so most Python projects' examples are unverified — the exact failure mode Rust's default prevents. If you already write doctests, the habit transfers whole; if you gave up on them because of the whitespace problem, this is the version without it.
- **ABAP.** There is nothing like it: ABAPDoc comments are not executable, and the examples in a class's documentation are prose. The closest thing in spirit is a `FOR TESTING` method named after the scenario, which some teams treat as the executable documentation — and the gap this page describes is why. If you have ever found an SAP note whose sample code no longer compiles, that is the problem doc tests solve.
- **Java / C#.** Javadoc `{@snippet}` (JDK 18+) can be compiled against a real source file, which is the same idea arriving twenty years later; before that, `@code` blocks in Javadoc were prose. C# XML `<example>` blocks are not compiled at all.
- **Go.** `Example` functions in `_test.go` files are the closest match: compiled, run, output-compared, and rendered in the docs. Go compares printed output the way Python does, so it carries Python's whitespace sensitivity rather than Rust's assertions.

---

## The verified output

<!-- output:doc_tests -->
*Verified output of [`doc_tests.rs`](examples/doc_tests.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A doc test is an example that has to keep working
   tally(&[5, 3, 0]) = 8
   read_cell(&["5", "3", "0"], 1) = Some(3)
   Every ``` block in a /// comment is compiled and run by
   `cargo test`. That is the answer to the oldest problem in
   documentation: the example that stopped compiling two releases
   ago and nobody noticed.

2. Each block is a whole program
   A doc test is wrapped in `fn main()` for you and compiled as its
   own crate, which is why it must `use` your crate by name rather
   than by `crate::`. It sees your PUBLIC API only — so a doc test
   is an integration test that happens to be printed in the docs.

3. `#` hides a line from the reader, not from the compiler
   A line starting with `# ` is compiled and not rendered. Use it
   for the `use` line and the fixture, so the example on the page is
   the two lines that matter. Do not use it to hide the part that
   makes the example work — a reader who copies what is rendered
   should get something that runs.

4. The four annotations on the fence
   ```              compile and run          (the default)
   ```no_run        compile, do not run      (network, long jobs)
   ```should_panic  run, and expect a panic
   ```ignore        do not even compile      (almost always wrong:
                    use `text` if it is not Rust)
   `ignore` is the one to be suspicious of — it turns a test into a
   comment while leaving it looking like a test.

5. What they are good for, and what they are not
   Good: the two-line example a reader needs, kept honest. A doc
   test that fails is a documentation bug, which is exactly the
   right thing to be told.
   Not: exhaustive coverage. They are slower than unit tests (one
   compilation each), they only reach the public API, and a doc
   comment full of edge cases is a bad doc comment. Put the third
   through twentieth case in #[cfg(test)] and leave the first one
   on the page.
```
<!-- /output -->

## Practice

**The example that documents half the sentence.** Write a `winner(&[u32]) -> Option<usize>` whose doc comment promises two things: the winner's index, and `None` on a tie. Give it one doc test showing a clear winner — then notice that the tie half of the promise is now a sentence nobody checks, and add the example that checks it.

Then a third case the sentence does not mention at all, and decide what to do about the mismatch: is the fix a new example, or a better sentence? Finally, hide the `use` line with `#` and say, in one rule, what you are allowed to hide that way.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:doc_tests_kata -->
*[`doc_tests_kata.rs`](examples/doc_tests_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: an example that compiles and documents the wrong thing.
//!
//!   rustc --edition 2024 doc_tests_kata.rs -o /tmp/dtk && /tmp/dtk
//!   rustc --edition 2024 --crate-type lib --crate-name doc_tests_kata doc_tests_kata.rs
//!   rustdoc --edition 2024 --test doc_tests_kata.rs -L . \
//!       --extern doc_tests_kata=libdoc_tests_kata.rlib

/// Returns the winning candidate's index, or `None` on a tie.
///
/// The example below passes, and documents only half of that sentence:
///
/// ```
/// # use doc_tests_kata::winner;
/// assert_eq!(winner(&[3, 9, 5]), Some(1));
/// ```
///
/// The tie is the case the doc comment promises and the example above never
/// reaches, so here it is:
///
/// ```
/// # use doc_tests_kata::winner;
/// assert_eq!(winner(&[9, 9, 5]), None);
/// ```
///
/// And the empty ballot, which the sentence above says nothing about at all:
///
/// ```
/// # use doc_tests_kata::winner;
/// assert_eq!(winner(&[]), None);
/// ```
pub fn winner(scores: &[u32]) -> Option<usize> {
    let best = *scores.iter().max()?;
    let mut leaders = scores.iter().enumerate().filter(|(_, s)| **s == best);
    let first = leaders.next()?;
    if leaders.next().is_some() { None } else { Some(first.0) }
}

fn main() {
    println!("1. The three examples, run here as ordinary code");
    println!("   winner(&[3, 9, 5]) = {:?}", winner(&[3, 9, 5]));
    println!("   winner(&[9, 9, 5]) = {:?}", winner(&[9, 9, 5]));
    println!("   winner(&[])        = {:?}", winner(&[]));

    println!();
    println!("2. Why the first example was not enough");
    println!("   The doc comment makes two promises: a winner's INDEX, and None on");
    println!("   a TIE. One clear-winner example tests the first half and leaves");
    println!("   the second as a sentence nobody checks. A doc test earns its");
    println!("   place when it demonstrates the part of the contract a reader");
    println!("   would otherwise have to take on trust — which is usually the");
    println!("   None, the Err, or the edge.");
    println!("   And the third case was not promised at ALL: the sentence says");
    println!("   nothing about an empty ballot. Writing the example is what");
    println!("   surfaced the gap; the fix is a better sentence, not just a test.");

    println!();
    println!("3. What `#` hid, and what it must not hide");
    println!("   Each block above starts with a hidden `# use doc_tests_kata::winner;`");
    println!("   so the rendered example is one line. That is the right use: the");
    println!("   import is noise a reader can reconstruct.");
    println!("   Hiding the SETUP would not be — an example whose visible lines do");
    println!("   not run when copied is worse than no example, because it looks");
    println!("   like it should.");

    println!();
    println!("4. The fence annotations, and the one to distrust");
    println!("   ```              compiled and run");
    println!("   ```no_run        compiled, not run  (it opens a socket)");
    println!("   ```should_panic  run, must panic");
    println!("   ```ignore        not even compiled");
    println!("   ```text          not Rust at all, so nothing is attempted");
    println!("   `ignore` looks like a test and is a comment. If the block is not");
    println!("   Rust, say `text`; if it is Rust that cannot run here, say");
    println!("   `no_run`; `ignore` is for the rare block that is real Rust, is");
    println!("   worth showing, and genuinely cannot compile — and it should carry");
    println!("   a note saying why.");

    println!();
    println!("5. Why a doc test is an integration test");
    println!("   Each block is wrapped in its own `fn main()` and compiled as its");
    println!("   own crate, linked against yours. So it sees the PUBLIC API only,");
    println!("   exactly as a user does — which is why the hidden line says");
    println!("   `use doc_tests_kata::winner` and not `use crate::winner`, and why");
    println!("   a doc test is the cheapest way to find out that something you");
    println!("   meant to export is still private.");
}
```
<!-- /source -->

<!-- output:doc_tests_kata -->
*Verified output of [`doc_tests_kata.rs`](examples/doc_tests_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The three examples, run here as ordinary code
   winner(&[3, 9, 5]) = Some(1)
   winner(&[9, 9, 5]) = None
   winner(&[])        = None

2. Why the first example was not enough
   The doc comment makes two promises: a winner's INDEX, and None on
   a TIE. One clear-winner example tests the first half and leaves
   the second as a sentence nobody checks. A doc test earns its
   place when it demonstrates the part of the contract a reader
   would otherwise have to take on trust — which is usually the
   None, the Err, or the edge.
   And the third case was not promised at ALL: the sentence says
   nothing about an empty ballot. Writing the example is what
   surfaced the gap; the fix is a better sentence, not just a test.

3. What `#` hid, and what it must not hide
   Each block above starts with a hidden `# use doc_tests_kata::winner;`
   so the rendered example is one line. That is the right use: the
   import is noise a reader can reconstruct.
   Hiding the SETUP would not be — an example whose visible lines do
   not run when copied is worse than no example, because it looks
   like it should.

4. The fence annotations, and the one to distrust
   ```              compiled and run
   ```no_run        compiled, not run  (it opens a socket)
   ```should_panic  run, must panic
   ```ignore        not even compiled
   ```text          not Rust at all, so nothing is attempted
   `ignore` looks like a test and is a comment. If the block is not
   Rust, say `text`; if it is Rust that cannot run here, say
   `no_run`; `ignore` is for the rare block that is real Rust, is
   worth showing, and genuinely cannot compile — and it should carry
   a note saying why.

5. Why a doc test is an integration test
   Each block is wrapped in its own `fn main()` and compiled as its
   own crate, linked against yours. So it sees the PUBLIC API only,
   exactly as a user does — which is why the hidden line says
   `use doc_tests_kata::winner` and not `use crate::winner`, and why
   a doc test is the cheapest way to find out that something you
   meant to export is still private.
```
<!-- /output -->

</details>

---

## See also

- [Comments that compile](../../15_First_Programs/comments_that_compile/README.md) — `///`, `//!`, and the two error codes a misplaced one gives
- [Where a test goes](../where_a_test_goes/README.md) — the other two kinds, and why this one behaves like an integration test
- [What a test asserts](../what_a_test_asserts/README.md) — the assertions inside the fences
- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — why a doc test can only see `pub` items

## Sources

[Documentation testing ↗](https://doc.rust-lang.org/rust-by-example/testing/doc_testing.html) in Rust by Example, and the [rustdoc book on documentation tests ↗](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html), which is where the fence annotations are listed. The transcript above was produced by building this page's example as a library and running `rustdoc --test` against it.
