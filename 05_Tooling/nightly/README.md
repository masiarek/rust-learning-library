# `rustup default nightly`: what that actually decides

**Level:** 201 · working knowledge

**One line:** `rustup default nightly` does not turn nightly on *for your project* — it changes the compiler for **every** Rust project on the machine, including ones you did not write, and it is the only toolchain choice that is recorded nowhere near the code it affects.

Nightly gets recommended a lot, and the recommendation is usually sound: it is where the interesting features are, and Rust's nightly is far more solid than the word suggests. The part worth slowing down on is not *whether* to use nightly — it is **which of three quite different commands** you reach for, because they are routinely treated as interchangeable and they are not.

## Three scopes, one of which is invisible

| What you want | Command | Who is affected | Where it is written down |
|---|---|---|---|
| Nightly for this one command | `cargo +nightly build` | that invocation | in the command, in front of you |
| Nightly for this project | `channel = "nightly-2026-08-11"` in [`rust-toolchain.toml`](../pinning_the_toolchain/README.md) | everyone who clones it | in the repository, in version control |
| Nightly for everything | `rustup default nightly` | every Rust project on the machine | **nowhere** |

The right-hand column is the whole page. Rows one and two leave evidence: a reader of the command or of the repository can see which compiler was used. Row three leaves none — the fact lives in `~/.rustup/settings.toml`, which is not in anybody's repository, is not in your shell history a month later, and is not visible to the colleague reading your bug report.

That asymmetry is why the middle row is almost always the one you want. It is *also* nightly. It just says so out loud, and it says so to everyone.

## What nightly actually is

A build from Rust's master branch, published most nights, carrying no stability promise: `#![feature(...)]` is accepted, and a feature can change or disappear between two of them. It is genuinely reliable day to day — this is not a warning about crashes. Three specific things go wrong, and none of them announce themselves.

**Your formatting silently stops matching CI's.** `rustfmt`'s unstable options are usable on nightly and ignored on stable — placed in a `rustfmt.toml`, they are [applied on nightly and skipped with a warning on stable ↗](https://github.com/rust-lang/rustfmt/issues/6257). So a nightly machine formats one way, a stable CI runner checks for another, and `cargo fmt -- --check` fails on a diff the author cannot reproduce. See [Formatting](../formatting/README.md) for the check this breaks.

**"Nightly" may not mean last night.** Components like `clippy` and `rust-analyzer` are not built successfully every night. When one is missing, rustup does not fail — it [searches backwards for an older nightly that has it ↗](https://rust-lang.github.io/rustup/concepts/channels.html) and installs that instead. So the compiler you get is *some* recent nightly, chosen by which components happened to build, and it can differ between two machines that ran the same command on the same day. Reproducibility that quietly depends on a build farm's luck is not reproducibility.

**Unstable features become reachable, and reach is enough.** You will not add `#![feature(...)]` by accident. But a `-Z` flag copied into `.cargo/config.toml` is accepted rather than rejected, a dependency's nightly-only path may light up, and none of it fails until somebody on stable tries to build. The library's [compile times](../compile_times/README.md) page already declines nightly's `-Z threads=8` in CI for the neighbouring reason: a nondeterministic compiler is a debugging nightmare wearing a build failure's clothes.

## When nightly is right

- You need a **specific** unstable feature, and you can name it. That is a real reason and it is the common one.
- You are testing your own crate against upcoming Rust, which is what beta and nightly exist for.
- A tool you depend on requires it — `miri`, some proc-macro tooling, the parallel front end.
- The project has decided on nightly as policy, in which case it belongs in the toolchain file where the decision is visible.

In every one of those, put it in [`rust-toolchain.toml`](../pinning_the_toolchain/README.md), not in `rustup default`.

## And pin the date

If you use nightly, use a dated one:

```toml
[toolchain]
channel = "nightly-2026-08-11"
components = ["rustc", "cargo", "clippy", "rustfmt", "rust-analyzer"]
```

The convention is not arbitrary — nightly identifies *itself* by date. `rustup default nightly` prints back `rustc 1.99.0-nightly (3d6c19bb9 2026-08-11)`, because with nightly the version number alone does not identify the compiler and everyone involved knows it. Writing the date down is agreeing with the toolchain about what it just told you.

## If you are coming from another language

- **Python** — the parallel is running `3.15.0a1` as your system Python rather than in one project's virtualenv. Nobody does that, and the reason is the same: the blast radius is every project on the machine, and the choice is invisible from inside any of them.
- **ABAP** — there is no nightly to opt into; the kernel is whatever the landscape runs. What Rust adds is the ability to choose, and therefore the obligation to record the choice.

## See also

- [rustup](../rustup/README.md) — the five-rung precedence this page is about rung 5 of
- [Pinning the toolchain](../pinning_the_toolchain/README.md) — the rung to use instead
- [Strict clippy lints](../strict_lints/README.md) — the other configuration decision from the same talk

---

*No generated output block: nothing on this page is a program's output, and the one transcript quoted — the nightly version string — comes from the slide that prompted the page rather than from a run on this machine, which has only stable installed.*
