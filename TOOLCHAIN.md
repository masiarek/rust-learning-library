# The toolchain map

**Level:** reference · the map

**One line:** Every question in this cluster is the same question at a different altitude — *which version of what, and who decided?* — and this page is the door to the lessons that answer it, cheapest answer first.

Rust's tooling has a good reputation and deserves it, but it is also four layers deep, and the layers are usually met in a jumble: a video recommends nightly, a blog recommends Nix, a colleague's project has a file you have never seen, and none of them say which problem they were solving. What follows sorts them. **If you want the idea rather than the syllabus, read the next section** — the tables after it are a reading order.

---

## The stack, before the reading order

Four tools, each wrapping the one above it, each answering "which version?" about something bigger:

| Layer | Decides | Written in |
|---|---|---|
| **rustup** | which *compiler* | `rust-toolchain.toml`, or a machine-wide default |
| **cargo** | which *dependencies* | `Cargo.toml` (a range) and `Cargo.lock` (the answer) |
| **clippy / rustfmt** | which *code* is acceptable | `[lints.clippy]`, `rustfmt.toml`, `clippy.toml` |
| **devenv / Nix** | which *everything else* — system libraries, CLI tools, databases | `devenv.nix` and `devenv.lock` |

Two patterns repeat all the way down, and noticing them is most of the benefit of reading these pages in order.

**A manifest states a range; a lockfile states what you got.** `Cargo.toml` says *"any 1.x"* and `Cargo.lock` says *"1.12.0"*. `devenv.nix` names inputs and `devenv.lock` pins their revisions. A `rust-toolchain.toml` saying `channel = "stable"` is a manifest with no lockfile — it looks like a pin and is not one, which is why the advice is always to write the number.

**Every one of these has a machine-wide version and a project-wide version, and the project-wide one is nearly always right.** `rustup default nightly` versus a toolchain file; `cargo install` versus a declared dev-tool; a globally-installed formatter versus one the toolchain carries. The machine-wide form is faster to type and invisible to everyone else, which is the same sentence twice.

---

## Reading order

Start wherever your question is; each page stands alone.

| # | Page | The question it answers |
|---|---|---|
| 1 | [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md) | How do I run a `.rs` file at all? |
| 2 | [rustup](05_Tooling/rustup/README.md) | Why is the `rustc` on my `PATH` 154 bytes, and how does it choose? |
| 3 | [Pinning the toolchain](05_Tooling/pinning_the_toolchain/README.md) | Which compiler verified this — and how do I stop that being luck? |
| 4 | [Nightly by default](05_Tooling/nightly/README.md) | Should I run `rustup default nightly`? |
| 5 | [A throwaway that needs a crate](05_Tooling/scratch_with_a_crate/README.md) | My scratch file says `use rand::…` and will not compile — what is missing? |
| 6 | [Adding a dependency](05_Tooling/cargo_dependencies/README.md) | What did `cargo add` actually write in my manifest? |
| 7 | [A tree of practice projects](05_Tooling/practice_workspace/README.md) | Forty small projects want the same four config files — do I need a script? |
| 8 | [Formatting](05_Tooling/formatting/README.md) | Who decides the whitespace, and which formatter just ran? |
| 9 | [Strict clippy lints](05_Tooling/strict_lints/README.md) | Should my project be forbidden from panicking? |
| 10 | [bacon](05_Tooling/bacon/README.md) | How do I stop asking the compiler and let it tell me? |
| 11 | [cargo-nextest](05_Tooling/nextest/README.md) | Is `cargo test` costing me anything? |
| 12 | [Choosing an editor](05_Tooling/editors/README.md) | Which window am I going to read all of this through? |
| 13 | [RustRover setup](05_Tooling/rustrover_setup/README.md) | How do I wire the IDE to everything above? |
| 14 | [RustRover Code Vision](05_Tooling/rustrover_code_vision/README.md) | What is the grey `1 usage` line above every declaration, and which toggle hides it? |
| 15 | [Neovim with LazyVim](05_Tooling/neovim_setup/README.md) | …or the other window, and the two ways it silently does nothing |
| 16 | [Scaffolding a practice tree](05_Tooling/scaffolding/README.md) | I keep writing the same seven config files — what should a script write, and what must it not? |
| 17 | [Compile times](05_Tooling/compile_times/README.md) | Where do the seconds go, and which knob reaches them? |
| 18 | [devenv](05_Tooling/devenv/README.md) | Should I declare the *whole* environment, and what does Nix cost? |

## Or, by the problem you actually have

| If your problem is… | Read |
|---|---|
| "it works on my machine" | [Pinning the toolchain](05_Tooling/pinning_the_toolchain/README.md), then [devenv](05_Tooling/devenv/README.md) if the answer is a system library rather than a compiler |
| "CI disagrees with me about formatting" | [Nightly by default](05_Tooling/nightly/README.md) — unstable `rustfmt` options are the usual cause — and [Formatting](05_Tooling/formatting/README.md) |
| "a tutorial told me to use nightly" | [Nightly by default](05_Tooling/nightly/README.md) |
| "a tutorial's `use rand::…` will not compile" | [A throwaway that needs a crate](05_Tooling/scratch_with_a_crate/README.md) — nothing is missing; the file is being compiled by `rustc` rather than Cargo |
| "my build got slower" | [Compile times](05_Tooling/compile_times/README.md), then [Adding a dependency](05_Tooling/cargo_dependencies/README.md) on what an unused crate costs |
| "my editor, `cargo fmt` and CI disagree with each other" | [Scaffolding a practice tree](05_Tooling/scaffolding/README.md) — its `doctor` checks the cross-file invariants (nightly-only rustfmt options against the pinned channel, `max_width` against `.editorconfig`) that no single tool owns |
| "I want fewer runtime panics" | [Strict clippy lints](05_Tooling/strict_lints/README.md), and [`expect`](17_Option_and_Result/expect/README.md) for the position it overrules |
| "onboarding a machine takes a day" | [devenv](05_Tooling/devenv/README.md) |
| "I make a lot of tiny projects and copy the same config" | [A tree of practice projects](05_Tooling/practice_workspace/README.md) |
| "I want the compiler to just tell me, without me asking" | [bacon](05_Tooling/bacon/README.md) |
| "I broke it an hour ago and cannot get back" | [Commit on green](05_Tooling/commit_on_green/README.md) |
| "my editor shows no types and I cannot see why" | [Neovim with LazyVim](05_Tooling/neovim_setup/README.md) — two independent silent failures — or [Choosing an editor](05_Tooling/editors/README.md) for the shim trap |
| "my tests are slow, or one of them takes the run down" | [cargo-nextest](05_Tooling/nextest/README.md) |
| "which pin actually applies?" | [rustup](05_Tooling/rustup/README.md) — the five-rung precedence table |
| "my editor is asking whether to allow MCP" | [What MCP is](05_Tooling/what_mcp_is/README.md) — what the two halves of `rustrover:read_file` are, and the difference between the two *Always allow* buttons |

---

Several of these pages exist because of a [No Boilerplate ↗](https://www.youtube.com/@NoBoilerplate) walkthrough of the Rust tooling stack, whose recommendations — nightly by default, a strict clippy configuration, devenv for the environment — are each defensible and each carry a cost the format of a talk has no room for. Where a page disagrees with that source it says so and says why; where it agrees, it says that too.
