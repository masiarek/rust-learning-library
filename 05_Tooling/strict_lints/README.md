# Strict clippy: denying the panic, and the arithmetic that comes with it

**Level:** 201 · working knowledge

**One line:** A `[lints.clippy]` block that denies `unwrap`, `expect`, indexing and `panic` converts a whole class of runtime abort into a compile error — and the same block rejects `n + 1`, which is the half of the trade nobody puts on the slide.

The configuration below comes from [Tris Oaten's Rust notes ↗](https://namtao.com/rust), where it is offered with a two-line rationale worth quoting for its honesty about what it is for: the lints *teach you Rust* and *stop panics at runtime*. Both halves are true. This page is about what each one costs, because the answer is not the same for the two groups and the panic set.

Both files are in [`config/`](config/clippy.toml) beside this page, ready to copy into a project.

## The configuration

```toml
# Cargo.toml
[lints.clippy]
# UM, ACTUALLY
pedantic = { level = "deny", priority = -1 }
# DEVELOPING LINTS
nursery = { level = "deny", priority = -1 }
# DENY PANICS
unwrap_used = "deny"
expect_used = "deny"
indexing_slicing = "deny"
arithmetic_side_effects = "deny"
unreachable = "deny"
unimplemented = "deny"
unchecked_time_subtraction = "deny"
todo = "deny"
string_slice = "deny"
panic_in_result_fn = "deny"
panic = "deny"
exit = "deny"
as_conversions = "deny"
```

```toml
# clippy.toml
allow-unwrap-in-tests = true
allow-expect-in-tests = true
allow-panic-in-tests = true
allow-indexing-slicing-in-tests = true
```

**`priority = -1` is load-bearing, not decoration.** Cargo applies lint entries in priority order, and a group and one of its members are in conflict by construction. The negative priority makes `pedantic` and `nursery` land *first*, so an individual line beneath them can still override a lint they contain. Drop it and Cargo rejects the manifest rather than guessing.

## Three groups doing three different jobs

**`pedantic`** — opinionated but finished lints, allow-by-default because they are style rather than bugs. Denying them is a real commitment and mostly a pleasant one: it is where you learn the idiom the standard library expects.

**`nursery`** — lints still under development. Useful, and noisier than the name suggests. Verified here on a two-line function that does nothing wrong:

```text
error: this could be a `const fn`
 --> src/main.rs:5:1
  |
5 | / fn double(n: i32) -> i32 {
6 | |     n.saturating_mul(2)
7 | | }
  | |_^
  = note: `-D clippy::missing-const-for-fn` implied by `-D clippy::nursery`
```

That is a fair suggestion, but it is a *suggestion*, and at `deny` it is a build failure on code with no defect. Expect to add `#[allow]`s, and expect the set to shift between releases, which is what "nursery" means.

**The panic set** — these are `restriction` lints, and the [glossary entry](../../GLOSSARY.md) for that group is the right frame: they forbid something legal and idiomatic, for a codebase that has decided against it. Clippy is not calling `unwrap` a bug. You are declaring that this program may not abort.

## What it does, verified

Run against a project carrying both files. An `unwrap()` in `main`:

```text
error: used `unwrap()` on a `Result` value
 --> src/main.rs:2:18
  |
2 |     let n: i32 = "21".parse().unwrap();
  |                  ^^^^^^^^^^^^^^^^^^^^^
  = note: if this value is an `Err`, it will panic
  = note: requested on the command line with `-D clippy::unwrap-used`
```

The *same* `unwrap()` inside `#[cfg(test)] mod tests`: **nothing**. The `clippy.toml` carve-out works exactly as advertised, which is the practical tip that makes the whole configuration liveable — prototype with `unwrap` inside a unit test, and clippy stays quiet.

And the one to know about before you adopt this:

```text
error: arithmetic operation that can potentially result in unexpected side-effects
 --> src/main.rs:3:20
  |
3 |     println!("{}", n + 1);
  |                    ^^^^^
  = note: requested on the command line with `-D clippy::arithmetic-side-effects`
```

`n + 1` on two integers. `arithmetic_side_effects` is not a corner-case lint; it fires on ordinary addition, and adopting it means writing `checked_add`, `saturating_add` or `wrapping_add` throughout. That is a defensible position — it is the same argument [scaling the denominator away](../../09_Advanced/scaled_integers/README.md) makes about arithmetic whose range you have not proved — and it is a much larger change to how code reads than the rest of the block put together. Adopt it deliberately, or leave that one line out.

## Where this library disagrees, and why that is fine

[`expect`](../../01_Foundations/expect/README.md) here argues that `expect` should be preferred over `unwrap` everywhere, because the message records *why* you believed this could not fail. This configuration denies both.

The two are not in conflict once you see they answer different questions. That lesson asks *"given that this code may panic, how should it panic?"* — and the answer is: with a sentence explaining the assumption. This block asks *"may this code panic at all?"* — and answers no, so the question the lesson resolves never arises. A `restriction` lint is a policy, not a correction.

Which is right depends on what the program is. A CLI that aborts with a clear message is behaving reasonably; a long-running service is not. Both pages are describing good practice for their own case.

## The 80% version

If the full block is more than you want to argue with today:

```sh
cargo clippy -- -D clippy::pedantic -D clippy::nursery
```

That is the two groups without the panic policy — the "teach you Rust" half on its own, and much the cheaper half to adopt.

## If you are coming from another language

- **Python** — closest to `ruff` with most rule families switched on, including the opinionated ones. The difference is that the panic set has no Python equivalent: there is no configuration that forbids raising.
- **ABAP** — the Code Inspector / ATC variant is the near analogue, and the same politics apply: the interesting question was never which checks exist, it is which ones the team agreed to make blocking.

## See also

- [Formatting](../formatting/README.md) — the other half of "the tool decides, not the reviewer"
- [`expect`](../../01_Foundations/expect/README.md) — the position this configuration overrules, and why
- [What a panic costs](../../01_Foundations/what_a_panic_costs/README.md) — what the panic set is actually buying you
- [Scale the denominator away](../../09_Advanced/scaled_integers/README.md) — `checked_*` and `saturating_*`, which `arithmetic_side_effects` makes mandatory
- Erik Schwartz, [*Your clippy config should be stricter* ↗](https://emschwartz.me/your-clippy-config-should-be-stricter/) — the argument at length, cited from the source above

---

*The three transcripts above are real clippy output, captured by running `cargo clippy --all-targets` against a throwaway project carrying both configuration files on stable 1.97.1. They are not regenerated by `tools/run_examples.py`, which compiles and runs single `.rs` files and does not invoke clippy — so unlike a generated block, these are verified once rather than on every commit.*
