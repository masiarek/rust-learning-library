# Tooling

The toolchain rather than the language: `cargo`, `rustc`'s flags, build profiles, and the parts of a working day that are spent waiting rather than writing.

The toolchain pages have a map of their own — [**TOOLCHAIN.md**](../TOOLCHAIN.md) puts them in reading order and sorts them by the problem you actually have.

These pages are not about making your program better. They are about the loop you sit inside all day — edit, build, run — and they earn their place because that loop is where Rust asks the most patience of you.

| Lesson | Level | What it teaches |
|---|---|---|
| [Adding a dependency](cargo_dependencies/README.md) | 101 → 201 | `search`, `info`, `add` — and the fact that `rayon = "1.12.0"` is a *range*, not the version you got |
| [Choosing an editor](editors/README.md) | reference | Every editor but RustRover is a front end for the same `rust-analyzer`, so the choice is what the window costs you before it shows you a type — with the pros and cons of six of them, and one verified way the do-it-yourself path fails silently |
| [Compile times](compile_times/README.md) | 201 | A build is four phases, and each optimization reaches exactly one — reduced debug info, the parallel front end, Cranelift, and why a saving is never portable |
| [devenv](devenv/README.md) | 201 | What a Nix development environment buys — and the ladder of cheaper tools it sits on top of, so you can tell which rung your project is actually standing on |
| [Formatting](formatting/README.md) | 101 → 201 | Hand the whitespace argument to `rustfmt` — and learn which of your IDE's *two* Rust formatters just ran, because a selection and a whole file do not go through the same one |
| [Nightly by default](nightly/README.md) | 201 | `rustup default nightly` changes the compiler for every project on the machine, and is the one toolchain choice recorded nowhere |
| [Pinning the toolchain](pinning_the_toolchain/README.md) | 201 | Which `rustc` verified the answer keys — nothing here says, and `rust-toolchain.toml` is the four-line file that makes the laptop and CI agree on purpose |
| [rustup](rustup/README.md) | 101 → 201 | The `rustc` on your `PATH` is a 154-byte shim, and the five-rung rule it uses to pick the real one |
| [Strict clippy lints](strict_lints/README.md) | 201 | Denying `unwrap`, `panic` and indexing turns runtime aborts into compile errors — and rejects `n + 1` along the way |

The one tooling page that is a *prerequisite* rather than a refinement lives in Foundations instead: [running a scratch program](../01_Foundations/rustc_without_cargo/README.md), which is how you run anything in this library at all.

## Planned

Rough order, not a promise:

- **`cargo test`, and the three kinds of test** — unit, integration, and doc tests; what each one can see, and which file it belongs in
- **Clippy** — the lints worth arguing with, and `#[allow]` as a comment that the compiler checks
- **Workspaces** — one `target/`, one lockfile, many crates, and the split that actually speeds a build up
- **`cargo add` and semver** — what a caret requirement really permits, and what `cargo update` is allowed to do to you
