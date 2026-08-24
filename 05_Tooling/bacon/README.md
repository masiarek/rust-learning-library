# bacon: the compiler in the other window

**Level:** 101 → 201 · working knowledge

**One line:** [bacon](https://dystroy.org/bacon/) watches your files and re-runs `cargo check`, `clippy` or the tests on every save into a pane you leave open, so the answer is already on screen by the time you would have finished alt-tabbing to a terminal to ask for it.

**Should you have it? Yes** — it is the cheapest thing in this whole section. One binary, no configuration, and no change to any project:

```sh
cargo install --locked bacon
bacon
```

That is the entire adoption cost. Nothing is added to your repository, so nothing has to be agreed with anyone, and uninstalling it leaves no trace.

## What it does

It runs a **job** and re-runs it whenever a file changes. The default job is `cargo check`, the fastest useful one. Three keys switch:

| Key | Job |
|---|---|
| *(default)* | `cargo check` — does it compile |
| `c` | clippy — does it compile *and* is it idiomatic |
| `t` | tests |
| `d` | `cargo doc`, opened in the browser |

By default it shows you the **first** error and the count, rather than a wall of output — which matters more than it sounds, because the first error is usually the only real one and the rest are its consequences.

No config file is required. `bacon --init` writes a `bacon.toml` when you want custom jobs, and `bacon --prefs` writes a `prefs.toml` for keybindings; both are optional and neither exists until you ask.

## Where it overlaps with what you already have

Worth being straight about, because the overlap is real: **rust-analyzer already runs `cargo check` on save** and underlines the result in your editor. If that is all you want, you have it.

Three things bacon adds on top:

- **It runs clippy and the tests**, which rust-analyzer's check-on-save does not by default. A watcher that re-runs your test suite on every save is a genuinely different working rhythm from one that only tells you it compiles.
- **It is editor-independent.** The same pane, the same keys, whether today's editor is RustRover, Helix, or `vim` over ssh.
- **It is a place to look**, not an annotation to notice. An error count in a dedicated pane is harder to scroll past than a squiggle.

If you live in RustRover with the clippy external linter on, the first point is where the remaining value is.

## Should it be part of automation?

**No** — and the distinction is worth keeping sharp, because it is the same one that separates every tool on this page from every tool in a workflow file.

bacon is an **inner-loop** tool: it exists to shorten the gap between your keystroke and your feedback, and it is interactive, stateful, and never exits. None of those properties are wanted in CI, where the job is to run once, decide, and report. Automation runs the same checks non-interactively:

```sh
cargo clippy --all-targets -- -D warnings
cargo nextest run
```

Same lints, same tests, opposite ergonomics. If you find yourself wanting bacon in a script, what you actually want is the underlying `cargo` command, which is the thing bacon has been calling all along.

## What about watchexec?

[`watchexec`](https://watchexec.github.io/) is the generic version of the same idea: watch files, run *any* command. It appears in the same talk, wired into a devenv script:

```sh
watchexec -c -e rs "cargo clippy && cargo test && cargo run"
```

Note what that buys and bacon's default jobs do not — a **chain** ending in `cargo run`. Lint, then test, then actually execute the program, on every save. For a small project that you are running constantly, that is a genuinely different loop from "does it compile".

You do not need both, and which one you want turns on a single question:

| | bacon | watchexec |
|---|---|---|
| Knows about Rust | yes — parses cargo output, shows the first error, counts the rest | no — it just reruns a command and shows raw output |
| Switch check → clippy → test | a keystroke | edit the command, restart |
| Chain several commands | needs a job defined in `bacon.toml` | it is the default way to use it |
| Works on non-Rust files | not the point | yes, any language, any command |

So: **bacon for the check/clippy/test loop, watchexec when the loop ends in running something** — or when the thing you are watching is not Rust at all.

There is a third answer that removes the choice, which is what to reach for if the chain is what you want and you would rather not run two watchers. `bacon --init` writes a `bacon.toml`, and a job in it can be any command:

```toml
# bacon.toml
[jobs.run-all]
command = ["cargo", "run", "--", "--color", "always"]
need_stdout = true
```

Then `bacon run-all` is the chained loop, inside bacon's pane, with the keys still working. The general rule underneath: reach for the specialised tool first and the generic one when the specialised tool runs out — not the other way round.

## Where it fits with strict lints

bacon and a [strict clippy configuration](../strict_lints/README.md) are designed for each other, and the source of both recommends running them as a pair — `bacon` with the `c` job, against a `[lints.clippy]` block that denies the panic. The pairing is the point: a lint policy that strict is unpleasant if you meet it at commit time in a batch of thirty errors, and unremarkable if you meet each one the second you write it.

## If you are coming from another language

- **Python** — `pytest-watch`, or `watchexec -e py -- pytest`. Same idea, and the same reason it is worth the setup: the cost of a feedback loop is not the seconds it takes, it is the context you lose asking for it.
- **ABAP** — no equivalent, and none needed: the syntax check is in the editor and the activation *is* the compile. Rust's separate build step is what creates the gap bacon fills.

## See also

- [Strict clippy lints](../strict_lints/README.md) — the policy this is the pleasant way to meet
- [cargo-nextest](../nextest/README.md) — the test runner behind the `t` job, if you install it
- [Compile times](../compile_times/README.md) — bacon shortens the loop; that page shortens the build inside it
- [A tree of practice projects](../practice_workspace/README.md) — one `bacon` at the workspace root covers every exercise
