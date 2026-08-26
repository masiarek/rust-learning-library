# RustRover: wiring the IDE to the rest of these pages

**Level:** 101 → 201 · working knowledge

**One line:** Everything the other tooling pages set up in files — the pinned compiler, the lint policy, the workspace — RustRover reads on its own; the three things it does *not* do by default are run clippy instead of `cargo check`, know which package in a workspace you meant, and print a path in a warning that names the file.

This page is setup, not selection: [Choosing an editor](../editors/README.md) is where the comparison lives, along with the toolchain-location trap that costs everyone an hour once. What follows assumes you have picked RustRover and want it agreeing with the [practice tree](../practice_workspace/README.md).

## 1. Make clippy the linter, not `cargo check`

**Settings → Rust → External Linters.** Five controls, of which three are decisions:

| Control | What it decides |
|---|---|
| **Run external linter on the fly** | whether it runs in the background, or only when you ask |
| **External tool** | **Cargo Check** or **Clippy** — the default is Cargo Check |
| **Additional arguments** | *what gets linted* — appended to `cargo check` / `cargo clippy` |
| **Channel** | which toolchain runs it; `[default]` is whatever `rustup` resolves |
| **Environment variables** | passed to the linter process only |

Switch the tool to Clippy. `cargo check` answers *does this compile*, which rust-analyzer is already telling you inside the editor; clippy answers *is this right*, which nothing else in the window is saying. If you have adopted a [strict lint policy](../strict_lints/README.md), this is the setting that makes the IDE enforce it — otherwise you write against one standard all afternoon and meet the other at commit time.

To check what is actually active without reopening settings, hover the **linter widget in the status bar** at the bottom of the window; it names the linter and whether on-the-fly is on.

One caveat worth knowing before you turn on-the-fly analysis on: it runs a real `cargo clippy`, so it takes a build lock and can save files to do it. On a large project that is a noticeable background cost, and turning it off does not lose you the feature — it becomes manual instead.

Whichever linter is selected, it also reports rustc's own warnings — so a scratch file that binds five names to show five forms gets five `unused_variable`s in the console before clippy has said anything. That is a project-file decision rather than an IDE one: [the four lints a practice tree turns off](../scaffolding/README.md), and the one in the same group it must not.

### What goes in *Additional arguments*

`--all-targets`, and probably nothing else.

That box is **scope, not policy**. It decides which code the linter compiles, and no manifest key can do that job — while everything you might otherwise be tempted to type there is policy, and policy belongs in a file your colleagues and CI can also read:

| The question | Where it is answered |
|---|---|
| *which* lints, at what level | `[lints.clippy]` in `Cargo.toml` — or `[workspace.lints.clippy]` plus `lints.workspace = true` in each member |
| a lint's threshold or carve-out | `clippy.toml` — `allow-unwrap-in-tests`, `too-many-arguments-threshold`, `msrv` |
| *which code* is looked at | **this box** |

The two are genuinely orthogonal, which is easiest to see by setting a policy and then watching it not reach your tests. With `len_zero = "deny"` in `[lints.clippy]`, and one offending line in the library and another inside `#[cfg(test)] mod tests`:

```text
$ cargo clippy
error: length comparison to zero
 --> src/lib.rs:2:5
error: could not compile `clippy_targets` (lib) due to 1 previous error

$ cargo clippy --all-targets
error: length comparison to zero
 --> src/lib.rs:2:5
error: could not compile `clippy_targets` (lib) due to 1 previous error
error: length comparison to zero
  --> src/lib.rs:12:17
error: could not compile `clippy_targets` (lib test) due to 2 previous errors
```

The policy was in force both times. The second run compiled a target the first one never built — note the `(lib test)` in the last line, which is the test target appearing for the first time. Denying a lint harder cannot reach code the linter is not looking at, so this flag is the only thing that makes the carve-outs in `clippy.toml` mean anything: [an `unwrap` allowed in tests](../strict_lints/README.md) is a rule about code your editor is otherwise not reading.

Two things not to put in there. **`-D warnings`** is the CI form — in the IDE it repaints every warning as an error, and it is a lint level, so it belongs in the manifest anyway. And **individual `-W clippy::…` flags**, for the same reason: they are invisible to everyone who does not use your IDE, including you at the next `git clone`.

**Channel** is best left at `[default]`, so the editor runs the same clippy as [your `rust-toolchain.toml`](../pinning_the_toolchain/README.md) and as CI. Pointing it at [nightly](../nightly/README.md) means the window enforces lints that stable does not, which is the [formatting disagreement](../formatting/README.md) again in a different tool.

*The two transcripts above are real clippy output, captured from a throwaway `cargo new --lib` project on stable. Like the ones on the strict-lints page they are verified once rather than on every commit, since `tools/run_examples.py` compiles single `.rs` files and does not invoke clippy.*

## 2. Or run it in the terminal, which is the same thing

The IDE is calling these; there is no separate "IDE clippy":

```sh
cargo clippy                       # the library/binary
cargo clippy --all-targets         # ...and tests, benches, examples
cargo clippy --fix                 # apply the mechanical suggestions
cargo clippy -- -D warnings        # the CI form: any warning fails
```

`--all-targets` is the one to get in your fingers. Without it, clippy skips your test code — which is exactly where the [test carve-outs](../strict_lints/README.md) are supposed to be proving themselves.

## 3. Teach it which package you meant

This is the one real friction of a [workspace](../practice_workspace/README.md). `cargo run` at the root does not know whether you meant `ex01_hello` or `ex07_lifetimes`, and RustRover inherits the ambiguity.

RustRover creates a **run configuration** per binary it finds, so after the first build there is a dropdown in the toolbar with every exercise in it, and switching exercises is choosing from that list rather than typing `-p`. Two things make it pleasant:

- **Pin the one you are working on** and use the run shortcut, rather than picking from the dropdown each time.
- A configuration is just a saved `cargo` command — if you want lint-then-test-then-run on one key, make one whose command is `run -p ex01_hello` and add a **Before launch** step for clippy.

The terminal equivalent, for the same reason:

```sh
cargo run -p ex01_hello
```

## 4. Make a warning name the file, not `src/main.rs`

A warning in the run console points at a span like this:

```text
warning: unused variable: `a`
 --> src/main.rs:5:9
```

`src/main.rs` is not where the file is. It is where the file is *relative to rustc's own working directory*, and Cargo sets that to the workspace root no matter which directory you invoked it from — so every project on the machine reports the same seven characters, and two IDE windows say `src/main.rs` about two different files.

One line in `.cargo/config.toml`, at the root of the tree, spells it out instead:

```toml
[build]
rustflags = ["--remap-path-prefix==/Users/you/RustroverProjects/untitled/"]
```

```text
warning: unused variable: `a`
 --> /Users/you/RustroverProjects/untitled/src/main.rs:5:9
```

The double `==` is not a typo, and it is the whole trick. [`--remap-path-prefix=OLD=NEW` ↗](https://doc.rust-lang.org/rustc/command-line-arguments.html#--remap-path-prefix-remap-source-names-in-output) is being given an **empty** OLD, and the match is by path *component* rather than by text: an empty prefix matches the start of every **relative** path and no absolute one at all. So your files get spelled out and the ones under `~/.cargo/registry` and in `std` — already absolute — pass through untouched. A trailing slash on the target is optional for the same reason.

It applies to clippy as much as to `cargo build`, which matters given section 1: `cargo clippy` reads `build.rustflags` like any other command, and `cargo fix` still applies its suggestions through the rewritten paths.

Three things to know before adopting it:

- **The path is written out because rustflags interpolate nothing** — there is no `$PWD` to reach for. Move or rename the tree and every diagnostic will confidently name the old location, which is worse than a relative path rather than better. That is why [`rust_scaffold.py`](../scaffolding/README.md) gitignores the file and its `doctor` fails when the two disagree.
- **A `RUSTFLAGS` environment variable replaces this wholesale** rather than adding to it, so the remap silently vanishes for any command run with one set. That is the usual explanation for a setting that appears not to apply.
- **`file!()` and panic locations become absolute too.** A backtrace naming a file you can open is an improvement; a recorded answer key containing a panic location is a re-record.

There is one honest caveat about when this is worth doing. Inside a [workspace](../practice_workspace/README.md) the relative form is less useless than it looks — the span reads `exercises/ex01_hello/src/main.rs`, because rustc runs from the workspace root and the member directory is already part of the path. It is the standalone `cargo new` project — the kind RustRover makes when you want to try something out, named `untitled` — where the span collapses to `src/main.rs` and names nothing at all.

What a fresh compile *does* print, and a cached one does not, is worth knowing alongside it:

```text
   Compiling untitled v0.1.0 (/Users/you/RustroverProjects/untitled)
```

That line carries the full path already. It appears only when something was actually compiled, so the run that surprises you — warnings replayed from cache, `Finished in 0.02s`, and no `Compiling` line above them — is exactly the run with nothing but `src/main.rs` on screen. `cargo build -v` prints `Fresh untitled v0.1.0 (…)` in its place, at the cost of the whole rustc command line with it.

## 5. The things you do not have to configure

Worth knowing so you do not go looking:

- **The compiler.** `rust-toolchain.toml` is a rustup mechanism, and RustRover goes through rustup, so [the pin](../pinning_the_toolchain/README.md) applies in the IDE with nothing set. Open the practice tree and it is on 1.97.1 because the file says so.
- **The lint policy.** `[workspace.lints.clippy]` and `clippy.toml` are read by clippy itself, so once the external linter is Clippy the IDE shows exactly what the command line shows.
- **New projects.** There is no template to set up: inside a workspace, `cargo new` already writes the lint opt-in, and the toolchain and lints are inherited from the root. This is the answer to "how do I make these the defaults for every new project" — you do not, the workspace does.

## What about bacon, if the IDE already does this?

A fair question, and the honest answer is that the overlap is largest exactly here — RustRover with clippy on the fly is doing most of what [bacon](../bacon/README.md)'s `c` job does. What is left is the `t` job: a watcher that re-runs your **tests** on every save, which no IDE lint setting gives you. If you do not want a second pane, skip bacon; if the loop you want is test-driven, that is the gap it fills.

## See also

- [Choosing an editor](../editors/README.md) — the comparison, and the `rustup which` trap when a tool wants a real binary rather than a shim
- [Strict clippy lints](../strict_lints/README.md) — the policy this setting enforces
- [A tree of practice projects](../practice_workspace/README.md) — the layout the run configurations come from
- [Scaffolding a practice tree](../scaffolding/README.md) — writing those run configurations into `.idea/` before the IDE has opened the project, and why the External Linters setting above is the one thing no file can carry
- [A tree of practice projects](../practice_workspace/README.md) — why a workspace member's span already names it, and the standalone project where it does not

---

*Settings paths verified against the [RustRover external linters documentation ↗](https://www.jetbrains.com/help/rust/rust-external-linters.html) rather than by driving the IDE; menu wording moves between releases, so treat the names as a route rather than a transcript.*
