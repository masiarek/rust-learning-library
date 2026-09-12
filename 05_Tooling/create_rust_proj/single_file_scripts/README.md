# One file that names its own crates: cargo-eval, rust-script, and cargo's own `-Zscript`

**Level:** 201 · working knowledge

**One line:** A `.rs` file with a manifest at the top runs as one command and never needs a folder — Cargo has it built in behind `-Zscript` on nightly (3.1 s the first time here, 0.6 s after), `rust-script` does the same on stable (4.2 s, then 0.02 s from cache), and `cargo-eval`, the 2019 fork of the 2017 original, no longer compiles on rustc 1.98 — so of the four names you will meet, two are live and they do not read each other's syntax.

## The file

[`hello.rs`](hello.rs):

```rust
#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"

[dependencies]
anyhow = "1"
---

fn double(n: i32) -> i32 {
    n * 2
}

fn main() -> anyhow::Result<()> {
    println!("hello from a single-file script, {}", double(21)); // 42
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn doubles() {
        assert_eq!(super::double(2), 4);
    }
}
```

```text title="Real output — cargo 1.100.0-nightly (2026-08-22), a script that pulls anyhow"
$ time cargo +nightly -Zscript hello.rs
hello from a single-file script, 42
cargo +nightly -Zscript hello.rs  3.113 total

$ time cargo +nightly -Zscript hello.rs
hello from a single-file script, 42
cargo +nightly -Zscript hello.rs  0.597 total

$ chmod +x hello.rs && ./hello.rs          # the shebang line does the same
hello from a single-file script, 42

$ cargo +nightly -Zscript test --manifest-path hello.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The manifest between the `---` lines is ordinary `Cargo.toml` content. Leave `package.edition` out and every run prints *"warning: `package.edition` is unspecified, defaulting to the latest edition"* — put it in. The build goes to `~/.cargo/build/<hash>/` (8.9 MB for this file), not beside the script, which is what makes the script a one-file thing.

Where it stands: the feature is [tracked as Cargo issue #12207 ↗](https://github.com/rust-lang/cargo/issues/12207) and documented under [`script` in the unstable reference ↗](https://doc.rust-lang.org/cargo/reference/unstable.html#script). On the pinned stable toolchain it declines rather than degrades — *"the `-Z` flag is only accepted on the nightly channel of Cargo"* — so `+nightly` in the shebang is load-bearing. This machine has a nightly installed, which is why the transcript above exists; [Nightly by default](../../nightly/README.md) is the page on why that `+nightly` should stay in the command and not become `rustup default`.

## The same file for rust-script

Stable-only, and it predates Cargo's syntax, so the manifest goes in a fenced `cargo` block inside the crate doc comment. [`hello_rust_script.rs`](hello_rust_script.rs):

```rust
#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! ```

fn main() -> anyhow::Result<()> {
    println!("hello from rust-script, {}", 6 * 7); // 42
    Ok(())
}
```

```text title="Real output — rust-script 0.36.0 on stable 1.98.0"
$ cargo install --locked rust-script                 # 50.9 s
$ time rust-script hello_rust_script.rs
hello from rust-script, 42
rust-script hello_rust_script.rs  4.199 total

$ time rust-script hello_rust_script.rs
hello from rust-script, 42
rust-script hello_rust_script.rs  0.019 total

$ rust-script --test hello_rust_script.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ rust-script -e '(1..=10).sum::<u32>()'
55
$ rust-script -d itertools -e 'use itertools::Itertools; (1..=5).map(|n| n * n).join("+")'
"1+4+9+16+25"
```

The second run is twenty milliseconds because `rust-script` runs the cached binary without asking Cargo anything; the cache is `~/Library/Caches/rust-script/` on macOS (16 MB after these runs), and `rust-script --clear-cache` empties it. The `-e` form is the part with no Cargo equivalent: an expression on the command line, printed with `{:?}` — the `"…"` around the joined string is the `Debug` quoting — and `-d` adds a crate to it.

## They do not read each other's files

```text title="Real output — each tool on the other's file"
$ rust-script hello.rs
error[E0658]: frontmatters are experimental

$ cargo +nightly -Zscript hello_rust_script.rs
warning: `package.edition` is unspecified, defaulting to the latest edition (currently `2024`)
error[E0433]: cannot find module or crate `anyhow` in this scope
```

The first is rustc itself refusing the `---` block on stable ([the `frontmatter` feature, issue #136889 ↗](https://github.com/rust-lang/rust/issues/136889)), so it is not something `rust-script` could fix. The second is the mirror image: Cargo does not look inside doc comments, sees no manifest, links no crates. A file is written for one of them.

## The lineage, and where cargo-eval sits

| Tool | Version, date | Manifest syntax | On this machine |
|---|---|---|---|
| [cargo-script ↗](https://crates.io/crates/cargo-script) | 0.2.8, 2017-10 | `//! ```cargo` block, or a `// cargo-deps:` line | the original; its own README calls it unmaintained |
| [cargo-eval ↗](https://crates.io/crates/cargo-eval) | 0.1.0, 2019-08 | the same two, plus `-e` expressions and `--loop` stream filters | **does not install** — see below |
| [rust-script ↗](https://crates.io/crates/rust-script) | 0.36.0, 2025-08 | the `//! ```cargo` block | installed, 50.9 s, MSRV 1.74 |
| `cargo -Zscript` | in Cargo, nightly | `---` frontmatter | works with `+nightly` |

`cargo-eval` is the one you asked about. It is a fork of `cargo-script`, made when the original stopped, and `rust-script`'s README still lists it as *"maintained fork of cargo-script"*; the repository had a push in March 2024. But the crate on crates.io is the 2019 release, and that release pins `rustc-serialize 0.3.24`, which the current compiler rejects:

```text title="Real output — cargo install --locked cargo-eval, rustc 1.98.0"
error[E0310]: the parameter type `T` may not live long enough
    --> /Users/amasa/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rustc-serialize-0.3.24/src/serialize.rs:1155:5
     |
1155 |     fn decode<D: Decoder>(d: &mut D) -> Result<Cow<'static, T>, D::Error> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |     the parameter type `T` must be valid for the static lifetime...
error: could not compile `rustc-serialize` (lib) due to 1 previous error
error: failed to compile `cargo-eval v0.1.0`
```

Its README also still says to invoke it as `cargo script` in one paragraph and `cargo eval` in the next, and its examples use `extern crate time;` and `time = "0.1.25"`, both 2017. Everything it added over `cargo-script` — the `-e` expression, `-d` for a dependency on the command line, the `--loop` filter over stdin — `rust-script` carries today with the same flag names, which is where the transcript above comes from. So: read `cargo-eval` as a name you will meet in older posts, and reach for `rust-script` or `-Zscript`.

## What a one-file script gives up

- **No IDE.** There is no `Cargo.toml` on disk, so RustRover opens the file under *"Project not associated with a Cargo.toml file"* — no inferred types, no go-to-definition, no Run button. That is the same banner the library's own examples get, and [the fix written for them](../../../CONTRIBUTING.md) is a manifest, which is the thing a script was avoiding.
- **One file.** A second module means a second file means a folder, at which point `cargo init` in that folder is the same work.
- **A warm cache is the only fast case.** Change one character and it is a rebuild of your crate — cheap, since the dependencies stay cached — but the 0.02 s second run above is the *unchanged* file.

Which leaves the scripts for what they are best at: a program you will run more than once *as a program* — a small tool, a check you re-run each morning, a snippet you keep in a gist — rather than a place to type while thinking. For that, the folder routes on [the hub page](../README.md) are faster to be inside.

## See also

- [The hub — a disposable Rust workspace, six ways](../README.md) — the comparison table this page is one row of
- [A throwaway that needs a crate](../../scratch_with_a_crate/README.md) — the three-command folder route, and its own note on `-Zscript`
- [Running a scratch program](../../../15_First_Programs/rustc_without_cargo/README.md) — the even smaller case: `rustc` on one file, no crates at all
- [rust-script.org ↗](https://rust-script.org) — every flag, including templates for `-e`
