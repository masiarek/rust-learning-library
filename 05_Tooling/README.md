# Tooling

The toolchain rather than the language: `cargo`, `rustc`'s flags, build profiles, and the parts of a working day that are spent waiting rather than writing.

These pages are not about making your program better. They are about the loop you sit inside all day — edit, build, run — and they earn their place because that loop is where Rust asks the most patience of you.

| Lesson | Level | What it teaches |
|---|---|---|
| [Compile times](compile_times/README.md) | 201 | A build is four phases, and each optimization reaches exactly one — reduced debug info, the parallel front end, Cranelift, and why a saving is never portable |
| [Formatting](formatting/README.md) | 101 → 201 | Hand the whitespace argument to `rustfmt` — and learn which of your IDE's *two* Rust formatters just ran, because a selection and a whole file do not go through the same one |

The one tooling page that is a *prerequisite* rather than a refinement lives in Foundations instead: [running a scratch program](../01_Foundations/rustc_without_cargo/README.md), which is how you run anything in this library at all.

## Planned

Rough order, not a promise:

- **`cargo test`, and the three kinds of test** — unit, integration, and doc tests; what each one can see, and which file it belongs in
- **Clippy** — the lints worth arguing with, and `#[allow]` as a comment that the compiler checks
- **Workspaces** — one `target/`, one lockfile, many crates, and the split that actually speeds a build up
- **`cargo add` and semver** — what a caret requirement really permits, and what `cargo update` is allowed to do to you
