# Pinning the toolchain: which compiler verified the answer keys?

**Level:** 201 · working knowledge

**One line:** Nothing in this library says which `rustc` it needs — the laptop that records the answer keys and the CI job that checks them both happen to run 1.97.1 today, and [`rust-toolchain.toml` ↗](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) is the four-line file that turns a coincidence into a promise.

Every page here rests on one claim: it does not say what a program prints, it says what a program *printed*. [`tools/run_examples.py` ↗](https://github.com/masiarek/rust-learning-library/blob/master/tools/run_examples.py) compiles each example, runs it, and diffs the result against a recorded key, and CI fails when the three drift apart.

Which leaves a question the machinery never answers. **Printed by what?**

## The gap, measured

On the machine that records the keys:

```sh
rustc --version    # rustc 1.97.1 (8bab26f4f 2026-07-14)
```

`examples.yml` opens with a `rustc --version` step for exactly this reason, and on the most recent run it printed the same string — same version, same commit hash. So the recorder and the checker agree.

For most of this library's life, nothing made them agree. `runs-on: ubuntu-latest` supplies whatever Rust the runner image happens to ship this month; the laptop supplies whatever `rustup` last installed. Two unpinned numbers that happened to match, with no file anywhere asserting that they should — which is the state this page was written to describe, and which it no longer describes, because writing it prompted the fix. The repository now carries a [`rust-toolchain.toml` ↗](https://github.com/masiarek/rust-learning-library/blob/master/rust-toolchain.toml); the rest of this page is what is in it and why.

## What that actually exposes — which is less than you would guess

The reflex is to imagine a compiler upgrade silently rewriting outputs. Rust's stability promise makes that mostly wrong, and the runner is narrower still. `run_example()` returns **`run.stdout` and nothing else**: the compiler's stderr is printed as a note for a human and thrown away, and the program's own stderr is discarded too.

That single line of code removes the largest category of drift. A future `rustc` that adds a lint, rewords a warning, or prettifies an error cannot move an answer key, because no diagnostic has ever been inside one. Three real exposures survive it:

| Exposure | Caught by CI? |
|---|---|
| **Prose quoting a diagnostic.** [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) reproduces compiler text in the page body, where nothing checks it | **No** — it rots silently |
| **The edition floor.** The runner passes `--edition 2024`, which needs rustc ≥ 1.85. An older toolchain fails every example at once | Yes, loudly |
| **Unstable-by-design output.** `{:?}` on standard types is explicitly not covered by the stability promise, and neither is float formatting | Yes, as a mystery diff |

The middle row is a non-event: it fails immediately and unmistakably. The bottom row is rare. **The top row is the one worth caring about**, and it is precisely the one the answer-key system cannot see — a page whose subject is the compiler's own words is checked by nobody.

So the honest summary is that this library's reproducibility exposure is small and specific. That matters, because it is the number any proposed fix has to be weighed against — see [devenv](../devenv/README.md), which solves a far larger version of this problem at a far larger price.

## The fix, in five lines

At the repository root:

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.97.1"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

`rustup` reads it on every invocation and installs that exact toolchain if it is missing. Nobody runs a command; nobody reads a README instruction and skips it. The file *is* the instruction.

Two details decide whether it works:

**It is a rustup feature, not a Rust feature.** What intercepts the call is the rustup shim on your `PATH` — the `rustc` in `~/.cargo/bin` is a few kilobytes that reads the toolchain file and re-executes the real compiler. Call a real binary directly, by absolute path, and the file is ignored completely. On this machine the shims live under Homebrew's rustup rather than `~/.cargo/bin`, which is worth knowing before you debug a pin that appears not to apply.

**`channel = "stable"` pins nothing.** It tracks stable, so two machines a month apart get two compilers and you have written a file that looks like a guarantee and is not one. Pin the number.

**`profile` decides what gets downloaded, and it is not free.** A version-pinned toolchain is a *separate install* from `stable`, even when the two resolve to the identical compiler — rustup keys toolchains by name, not by resolved version. Measured here on adding the pin: the default profile installs **1.5 GB**, `minimal` plus those two components installs **657 MB**, and `rustfmt --version` and `cargo clippy --version` both answer from it either way. On CI, which caches nothing between runs, that difference is downloaded on every job.

The cost is a chore, and the chore is the feature: the toolchain now moves only when somebody edits that line, and that edit is a commit, with a diff, in which every answer key is re-verified by the new compiler before anyone else sees it. An unpinned toolchain upgrades itself between your commits, which is the same event with nobody watching.

## What this does not buy

A pinned toolchain gets you one compiler and the components that ship with it. It has nothing to say about the rest of a real environment: `cargo-nextest` and `bacon` are separate installs, a crate binding to `zlib` or `openssl` needs a system library nobody pinned, and a project with a database has a whole service outside the picture.

Each of those is a step further from "a version number in a text file" and a step closer to "a package manager for the entire machine." That is the ladder [devenv](../devenv/README.md) sits at the top of.

## If you are coming from another language

- **Python** — the direct analogue is `.python-version`, and `uv` extends it to the interpreter *and* every dependency through `uv.lock`. This repository already pins its documentation toolchain that way and leaves its compiler unpinned, which is the asymmetry this page is about.
- **ABAP** — there is no equivalent, because there is nothing to pin: the system *is* the version, and the kernel and support package level are properties of a landscape somebody else administers. What changes on Rust is that the environment becomes a file in your repository — and therefore something you can forget to write.

## See also

- [devenv](../devenv/README.md) — the same problem taken to its limit: pin the compiler, the CLI tools, the system libraries, and the running services, at the price of installing Nix
- [Compile times](../compile_times/README.md) — the other half of the build story, and the reason `[profile.dev]` is worth an evening
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — where `--edition 2024` comes from, and why it is passed explicitly

---

*No generated output block on this page, deliberately: everything it describes is a property of the toolchain rather than of a program, and any example that revealed its own compiler version would print something different on every machine — exactly the input an answer key cannot hold.*
