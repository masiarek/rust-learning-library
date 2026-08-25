# A tree of practice projects: one workspace, and no script at all

**Level:** 201 · working knowledge

**One line:** Forty exercise folders each want the same four config files, and the reflex is to write a generator — but a Cargo **workspace** already shares all four from the root, and `cargo new` writes the opt-in for you, so the whole ritual for a new exercise is `cargo new`.

## The problem

You are learning, so you make a lot of small projects. Each one wants the same setup: the compiler pinned, the lint policy applied, `unwrap` allowed in tests, formatting settled. Four files, times forty folders.

Three answers present themselves. Copy the files each time (they drift). Write a script (now you maintain a script). Reach for a template engine like [`cargo generate`](#what-about-cargo-generate). All three are worse than the built-in answer, and the reason is worth understanding rather than memorising: **a template copies configuration; a workspace shares it.**

## The layout

```
rust-practice/
  rust-toolchain.toml      the compiler, for everything below
  Cargo.toml               [workspace] + the lint policy
  clippy.toml              the tests carve-out
  exercises/
    ex01_hello/
    ex02_options/          ← cargo new, and nothing else
```

```toml
# Cargo.toml
[workspace]
resolver = "3"
members = ["exercises/*"]

[workspace.lints.clippy]
pedantic = { level = "deny", priority = -1 }
unwrap_used = "deny"
panic = "deny"
```

```toml
# exercises/ex02_options/Cargo.toml — written by `cargo new`, not by you
[lints]
workspace = true
```

## Four mechanisms, none of which you invoke

| What you would otherwise copy | What covers it |
|---|---|
| the compiler version | `rust-toolchain.toml` at the root — [rustup walks **up**](../rustup/README.md) from the working directory, so one file covers the tree |
| membership in the build | `members = ["exercises/*"]` is a **glob**: a new directory is a member the moment it exists |
| the lint policy | `[workspace.lints.clippy]` at the root, inherited by any member saying `workspace = true` |
| that opt-in line itself | **`cargo new` writes it for you** when the workspace defines `[workspace.lints]` |

The fourth is the one that makes a script pointless, and it is easy to miss. Verified by running `cargo new ex01_hello` inside such a workspace and reading back the manifest it produced:

```toml
[package]
name = "ex01_hello"
version = "0.1.0"
edition = "2024"

[dependencies]

[lints]
workspace = true
```

Nothing appended that. Cargo saw the workspace's lint table and opted the new package in.

The root `clippy.toml` reaches members too. In an exercise carrying an `unwrap()` on line 2 of `main` and another inside `#[cfg(test)]` on line 8, clippy flags exactly one of them:

```text
error: used `unwrap()` on a `Result` value
 --> exercises/ex01_hello/src/main.rs:2:18
```

Line 8 is silent. One `clippy.toml` at the root, four `allow-*-in-tests` lines, every member.

## The same trick for dependencies

"Should I have a standard set of crates?" has the same shape as the lint question, and the same answer: declare them once at the root, inherit them per member.

```toml
# Cargo.toml — at the root
[workspace.dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
clap = { version = "4", features = ["derive"] }
```

```toml
# exercises/ex01_hello/Cargo.toml — opt in, per crate, only where used
[dependencies]
anyhow = { workspace = true }
```

Verified: that member resolves `anyhow v1.0.104`, compiles, and runs. The version lives in exactly one place, so a bump moves every exercise at once and no two exercises can silently be on different versions of the same crate.

Note the asymmetry with lints, which is deliberate on Cargo's part. Lints are inherited wholesale with one `workspace = true`; dependencies are inherited **per crate**, because a package should still declare what it actually uses. `[workspace.dependencies]` is a version registry, not an automatic `use`.

**Keep the list short while learning.** Reaching for `itertools` before you have felt why `Iterator` was not enough teaches you the crate instead of the language, and every crate you add is one more thing between you and the error message. Three earn their place early — `anyhow` the day errors stop being toy ones, `serde` the day a file has to be read, `clap` the day a program takes arguments — and the rest can wait for the exercise that needs them.

## And one shared `target/`

Forty standalone projects means forty build caches, each with its own compiled copy of every dependency. A workspace has one, at the root. On a practice tree that is the difference between a directory you occasionally have to go and delete and one you do not think about.

## What it costs

Three things, and they are small but real:

- **`cargo run` needs `-p`.** Inside a workspace, `cargo run -p ex01_hello`. This is the tax you actually notice.
- **One `Cargo.lock`, one dependency resolution.** All members agree on one version of `serde`. Usually a feature; if one exercise genuinely needs a conflicting version, put it in the workspace's `exclude` list and let it stand alone.
- **An exercise is no longer portable on its own.** Copy one folder out of the tree and it has lost its toolchain, its lints, and its opt-in line, which now refers to a workspace that is not there.

## What about `cargo generate`?

[`cargo generate` ↗](https://cargo-generate.github.io/cargo-generate/) fetches a template repository and expands placeholders into a new project. It is genuinely good, and it is aimed at a different problem than this one.

The distinction is **content versus configuration**. `cargo generate leptos-rs/start-trunk` produces nineteen files — components, pages, `index.html`, `Trunk.toml`, a routing skeleton — that you could not type from memory and would not want to. That is content, and a template is exactly right for it.

A throwaway learning folder has no content. What it needs is `fn main() {}`, which `cargo new` already writes. Everything else you wanted automated was *configuration* — and here a template is actively worse than a workspace, for one reason:

> A template **copies** the config into each project. A workspace **shares** it.

With forty generated projects you have forty copies of `rust-toolchain.toml` and forty copies of the lint block. Change your mind about one lint — and you will, that is what learning is — and you edit forty files, or more realistically you edit the template, and the forty existing folders quietly become a fossil of what you believed in March. With the workspace you edit one line at the root and all forty change, including the ones you wrote first.

So: **workspace for the practice tree, `cargo generate` for real project skeletons.** They are not competing; a Leptos app or an embedded project is not an exercise and does not belong in the exercise tree anyway.

## If you are coming from another language

- **Python** — closest to one virtualenv and one `pyproject.toml` covering a folder of scripts, rather than a venv per script. The instinct to isolate every experiment is right in Python, where a project *is* its dependency set; in Rust the dependency set is cheap and the **build cache** is the expensive thing, which flips the answer.
- **ABAP** — a package with a shared transport layer and shared checks, versus a fresh `$TMP` object each time. `$TMP` is the throwaway folder, and it is exactly what you lose when everything is standalone: nothing shared to configure once.

## See also

- [rustup](../rustup/README.md) — why a root `rust-toolchain.toml` covers subdirectories
- [Pinning the toolchain](../pinning_the_toolchain/README.md) — what to write in it
- [Strict clippy lints](../strict_lints/README.md) — the policy this tree inherits, and the three lines worth leaving off while learning
- [bacon](../bacon/README.md) and [nextest](../nextest/README.md) — the two tools that make the loop inside such a tree quick
