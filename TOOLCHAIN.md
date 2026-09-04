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
| 7 | [`Cargo.lock`](05_Tooling/cargo_lock/README.md) | Who writes the lockfile, who reads it, and which command ignores it? |
| 8 | [Two versions of one crate](05_Tooling/two_versions_of_one_crate/README.md) | Why is `rand` in my build twice, and why is a `StdRng` "a different `StdRng`"? |
| 9 | [Vendoring, and the `[patch]` table](05_Tooling/vendoring_and_patch/README.md) | How do I build with no network, and how do I fix a bug in a dependency? |
| 10 | [A tree of practice projects](05_Tooling/practice_workspace/README.md) | Forty small projects want the same four config files — do I need a script? |
| 11 | [Formatting](05_Tooling/formatting/README.md) | Who decides the whitespace, and which formatter just ran? |
| 12 | [Strict clippy lints](05_Tooling/strict_lints/README.md) | Should my project be forbidden from panicking? |
| 13 | [bacon](05_Tooling/bacon/README.md) | How do I stop asking the compiler and let it tell me? |
| 14 | [cargo-nextest](05_Tooling/nextest/README.md) | Is `cargo test` costing me anything? |
| 15 | [Choosing an editor](05_Tooling/editors/README.md) | Which window am I going to read all of this through? |
| 16 | [RustRover setup](05_Tooling/rustrover_setup/README.md) | How do I wire the IDE to everything above? |
| 17 | [RustRover Code Vision](05_Tooling/rustrover_code_vision/README.md) | What is the grey `1 usage` line above every declaration, and which toggle hides it? |
| 18 | [Neovim with LazyVim](05_Tooling/neovim_setup/README.md) | …or the other window, and the two ways it silently does nothing |
| 19 | [Scaffolding a practice tree](05_Tooling/scaffolding/README.md) | I keep writing the same seven config files — what should a script write, and what must it not? |
| 20 | [Compile times](05_Tooling/compile_times/README.md) | Where do the seconds go, and which knob reaches them? |
| 21 | [devenv](05_Tooling/devenv/README.md) | Should I declare the *whole* environment, and what does Nix cost? |

## Or, by the problem you actually have

| If your problem is… | Read |
|---|---|
| "it works on my machine" | [Pinning the toolchain](05_Tooling/pinning_the_toolchain/README.md), then [devenv](05_Tooling/devenv/README.md) if the answer is a system library rather than a compiler |
| "CI disagrees with me about formatting" | [Nightly by default](05_Tooling/nightly/README.md) — unstable `rustfmt` options are the usual cause — and [Formatting](05_Tooling/formatting/README.md) |
| "a tutorial told me to use nightly" | [Nightly by default](05_Tooling/nightly/README.md) |
| "a tutorial's `use rand::…` will not compile" | [A throwaway that needs a crate](05_Tooling/scratch_with_a_crate/README.md) — nothing is missing; the file is being compiled by `rustc` rather than Cargo |
| "my build got slower" | [Compile times](05_Tooling/compile_times/README.md), then [Adding a dependency](05_Tooling/cargo_dependencies/README.md) on what an unused crate costs, and `cargo tree -d` from [Two versions of one crate](05_Tooling/two_versions_of_one_crate/README.md) for a subtree compiled twice |
| "the tool I `cargo install`ed behaves differently from the one I tested" | [`Cargo.lock`](05_Tooling/cargo_lock/README.md) — `cargo install` ignores the lockfile unless you pass `--locked` |
| "expected `StdRng`, found a different `StdRng`" | [Two versions of one crate](05_Tooling/two_versions_of_one_crate/README.md) — two incompatible lines of one crate, and the type from one is a stranger to the other |
| "I need to fix a bug in a dependency, today" | [Vendoring, and the `[patch]` table](05_Tooling/vendoring_and_patch/README.md) — and why editing the copy in `vendor/` or `~/.cargo/registry` is the one thing not to do |
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

## Po polsku

Wszystkie pytania z tego działu są jednym pytaniem zadanym na różnych wysokościach: **której wersji czego używam i kto o tym zdecydował?** Strona układa lekcje od odpowiedzi najtańszej do najdroższej.

Warstwy warto umieć wymienić po polsku, bo w materiałach mieszają się bez ostrzeżenia: `rustup` zarządza **wersjami kompilatora**, `cargo` zarządza **zależnościami i budowaniem**, plik `rust-toolchain.toml` **przypina** wersję do projektu, a Nix czy devenv odtwarzają całe środowisko. Kiedy blog radzi `nightly`, a kolega ma w repozytorium plik, którego nigdy nie widziałeś, to są odpowiedzi z różnych warstw na różne problemy — i żadna z nich zwykle nie mówi, na który.

Dwa słowa zostają po angielsku i nie warto ich tłumaczyć: **toolchain** („łańcuch narzędzi" po polsku brzmi jak coś innego) oraz **stable** i **nightly** jako nazwy kanałów wydawniczych. Reguła praktyczna dla uczącego się: zostań na `stable`, dopóki jakaś konkretna, dająca się nazwać rzecz nie zmusi cię do zmiany — a wtedy zapisz w projekcie, co i dlaczego, bo za pół roku sam siebie o to zapytasz.

**Szukaj po polsku:** `rustup` i kanały wydawnicze · przypinanie wersji kompilatora · `cargo` i zależności · `rust toolchain file` · `rust stable vs nightly`
