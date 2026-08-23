# Choosing an editor for Rust

**Level:** reference · for newcomers

**One line:** Every editor on this page but one is a window onto the *same* program — `rust-analyzer` — so what you are choosing is not how much Rust intelligence you get, but how much of your attention the window asks for before it gives you any; and while you are still learning the language, the right answer is the one that asks for none.

There is a recommendation at the bottom. The rest of the page is what it rests on.

---

## What you are actually choosing

When your editor underlines a borrow error two seconds after you type it, and before you have saved anything, the editor did not work that out. A separate program did: **`rust-analyzer`**, maintained by the Rust project, which reads your crate, answers questions over the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), and has no opinions about colour schemes. It is very likely already on your machine, because `rustup` ships it as a component:

```bash
rustup which rust-analyzer     # /Users/you/.rustup/toolchains/stable-…/bin/rust-analyzer
```

VS Code, Neovim, Helix, Zed, Emacs, Sublime Text and half a dozen others all ask that same program the same questions and draw its answers differently. So the honest framing of "which editor" is narrower than it looks. You are choosing between front ends, and what separates them is:

- **how much setup stands between downloading it and seeing a type hint,**
- **which of `rust-analyzer`'s non-standard extras are wired up** (expand this macro, show me the MIR, run just this test),
- and how much you like the editor for reasons that have nothing to do with Rust.

**One editor on the list is not in that club.** RustRover runs JetBrains' own analyzer, written in Kotlin, which started about ten years ago as the IntelliJ Rust plugin — before `rust-analyzer` existed. JetBrains' Vitaly Bragilevsky puts the reason plainly: they are "not constrained by the LSP protocol," so they can build things a generic language server cannot be asked for. The practical consequence for you is that RustRover is a genuine **second opinion** rather than a second skin: it occasionally flags what `rust-analyzer` misses, and occasionally the reverse.

And one thing does not change no matter what you pick: **the compiler is the teacher.** `cargo build`, `cargo clippy`, the error messages, [`rustfmt`](../formatting/README.md) — all identical everywhere. No editor on this page makes Rust easier to *compile*. They differ only in how early they tell you what `rustc` was going to say anyway.

---

## What the engine is doing for you, and why it matters most now

The single feature worth choosing an editor for, while you are learning, is the **inlay type hint** — the grey ghost text after a `let` that names the type you did not write.

It is easy to mistake that hint for a restatement of the literal on the right. It is not. It is the compiler's conclusion after reading the *whole function body*, and that includes lines below the one you are looking at. Here is a program that says so, written so the compiler has to agree with it — each `let _: T = x;` is the hint spelled out as an assertion, and a wrong one will not compile:

<!-- source:inlay_hint_check -->
*[`inlay_hint_check.rs`](examples/inlay_hint_check.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! What your editor's ghost text is claiming — written so the compiler checks it.
//!
//! Every editor compared on the page beside this file draws the inferred type
//! after a `let` that states none. The hint is worth trusting; it is also worth
//! knowing what it is. It is not a rule about the literal on the right. It is a
//! conclusion the compiler reached after reading the whole function, including
//! the lines below the one you are looking at.

fn main() {
    // Nothing here states a type. Your editor draws one after each name.
    let seats = 3;
    let quota = 0.5;
    let winner = "Ada";

    // The same three claims, written as annotations the compiler must check.
    // Change any one of them to another type and this file stops compiling.
    let _: i32 = seats;
    let _: f64 = quota;
    let _: &str = winner;

    println!("alone on the page, an integer literal settles on i32: {seats}");
    println!("a float literal settles on f64: {quota}");
    println!("a bare string literal is a borrowed &str: {winner}");

    // Now the part that reading top-to-bottom will not give you.
    let votes = 3; // written exactly like `seats` above...
    let total: u64 = 10;
    let sum = total + votes; // ...until this line, two lines later

    let _: u64 = votes; // and this is what the hint says now
    let _: u64 = sum;

    println!();
    println!("`votes` is the same literal on the same shape of line as `seats`.");
    println!("Adding it to a u64 two lines later makes it a u64: {votes} + {total} = {sum}");
    println!("Inference runs over the whole body, so the hint on a `let` can be");
    println!("a conclusion drawn from a line you have not read yet. That is the");
    println!("thing an editor tells you and a printout of the source does not.");
}
```
<!-- /source -->

<!-- output:inlay_hint_check -->
*Verified output of [`inlay_hint_check.rs`](examples/inlay_hint_check.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
alone on the page, an integer literal settles on i32: 3
a float literal settles on f64: 0.5
a bare string literal is a borrowed &str: Ada

`votes` is the same literal on the same shape of line as `seats`.
Adding it to a u64 two lines later makes it a u64: 3 + 10 = 13
Inference runs over the whole body, so the hint on a `let` can be
a conclusion drawn from a line you have not read yet. That is the
thing an editor tells you and a printout of the source does not.
```
<!-- /output -->

`votes` and `seats` are the same literal written on the same shape of line, and they are different types — because two lines later one of them is added to a `u64`. Nothing about the `let` says so. The editor says so, and it is right.

Change one assertion to the type you *expected* and the compiler explains the disagreement:

```text title="rustc, with line 30 changed to `let _: i32 = votes;`"
error[E0308]: mismatched types
  --> inlay_hint_check.rs:30:18
   |
30 |     let _: i32 = votes; // WRONG on purpose
   |            ---   ^^^^^ expected `i32`, found `u64`
   |            |
   |            expected due to this
```

That is the whole argument for picking an editor with working hints on day one, and it is why "any text editor is fine, Rust is just text" is bad advice for a beginner and fine advice for someone who has already internalised inference.

---

## The short version

| Editor | Rust engine | What it costs before you write any Rust | Suits |
|---|---|---|---|
| [RustRover](#rustrover) | JetBrains' own | download, sign in, open the folder | learning Rust; anyone who already lives in a JetBrains IDE |
| [VS Code with rust-analyzer](#vs-code-with-rust-analyzer) | `rust-analyzer` | install one extension | the default path, and the one every tutorial assumes |
| [Zed](#zed) | `rust-analyzer` | download; there is nothing to configure | a fast, quiet editor with the hints still on |
| [Neovim with LazyVim](#neovim-with-lazyvim) | `rust-analyzer` via `rustaceanvim` | a config you now own, and a version floor that moves | you already think in vim motions |
| [Helix](#helix) | `rust-analyzer`, built in | nothing at all | modal editing with no config file to maintain |
| [Emacs](#emacs) | `rust-analyzer` via eglot | a config you now own | you already run Emacs |
| [Plain vim, Sublime, anything without LSP](#anything-without-a-language-server) | none | nothing — and it gives nothing back | not while you are learning |

---

## RustRover

JetBrains' standalone Rust IDE. Free for non-commercial use: you sign in, click **Start Non-Commercial Use**, and get a renewable one-year licence on an honour-system declaration that you are not being paid for what you write with it. Commercial work needs a paid subscription.

**Pros**

- **Nothing to assemble.** Open the folder; the analyzer, the debugger, the test runner, the Cargo tool window and the profiler are already there and already wired to each other.
- **A second analyzer.** It is not `rust-analyzer` with a different theme, so on a hard type-inference or macro question you can ask two independent implementations and notice when they disagree. Nothing else on this page offers that.
- **The best debugger story on macOS**, by some distance — breakpoints in tests, expression evaluation, and a memory view, all with no adapter to install.
- **Refactorings that are transactional**, in the JetBrains sense: rename across a workspace, extract a function, change a signature, with a preview and an undo that covers all of it.
- **The learning-relevant one:** its inspections explain themselves in prose, and the error-and-quick-fix loop is aimed at somebody who does not yet know what the fix is called.

**Cons**

- **Heavy.** A JVM IDE indexing a Cargo workspace is the slowest thing on this list to start and the hungriest at rest, which is felt most on an older Intel Mac.
- **A licence question that never fully goes away.** The non-commercial tier is generous and it is also a declaration you have to keep being able to make.
- **Closed engine.** When its analysis is wrong about proc-macro-heavy code, you cannot read the code that got it wrong, and you cannot point another editor at the part that is right.
- **Two formatters, one keystroke.** RustRover runs `rustfmt` on a whole file but its own built-in formatter on a selection — a trap with a page of its own here: [Formatting](../formatting/README.md).
- **The settings tree is large** in the way every JetBrains product's is.

---

## VS Code with rust-analyzer

The path every tutorial, every Stack Overflow answer and every "getting started with Rust" video assumes. The extension is `rust-lang.rust-analyzer`, published by the `rust-analyzer` project itself rather than by a third party.

**Pros**

- **It is the reference client.** The people who write the language server use this extension, so a new server feature is usable here first and any behaviour you read about works the way it is described.
- **One install, then it works** — and it will offer to download the server itself if `rustup` has not already provided one.
- **Everything else you do is also here.** Markdown, YAML, TOML, a Python notebook, a Dockerfile, the repo you actually get paid to work on.
- **The most documented failure modes on the internet**, which matters more than it should when something breaks.

**Cons**

- **Electron.** Two Rust projects open is a lot of memory before `rust-analyzer` has asked for its share, and `rust-analyzer` on a large workspace is itself measured in gigabytes.
- **Debugging needs a second decision.** Microsoft's C++ debugger is not licensed for use outside VS Code and does not cover the macOS Rust case well; you install **CodeLLDB** and point a `launch.json` at your binary. It works, it is documented, and it is a step that RustRover and Zed do not ask for.
- **Settings sprawl.** The extension exposes most of `rust-analyzer`'s surface, so it is possible to configure yourself into a state nobody else has.

---

## Zed

A newer editor written in Rust by Zed Industries, GPU-accelerated, open source, and stable since 1.0 in April 2026. It speaks LSP, so its Rust support is `rust-analyzer` — it will use the one on your `PATH` and download its own if it cannot find one.

**Pros**

- **Fast, and not in a marketing sense** — cold start and large-file scrolling are the things it was built for, and it shows.
- **Nothing to configure to get Rust.** Open a Cargo project and the hints are on.
- **A built-in debugger.** Its DAP support covers Rust binaries and tests out of the box.
- **A single JSON settings file** you can read end to end, and version alongside your dotfiles.

**Cons**

- **A thinner ecosystem.** If your day includes some language or tool with one obscure extension, it is probably a VS Code extension.
- **Young.** Fewer people have hit your bug before you, and the answer to "how do I…" is more often "in the docs" than "on Stack Overflow".
- **Its AI features are the paid part** of an otherwise free editor, which is a fine business model and still a thing to know before you lean on them.
- **Some preferences are JSON-only**, which is a pro and a con depending on the hour.

---

## Neovim with LazyVim

[LazyVim](https://www.lazyvim.org/) is not an editor. It is a **curated configuration** for Neovim, built on the `lazy.nvim` plugin manager, that ships a working IDE-ish setup — completion, fuzzy finding, git signs, a status line, LSP plumbing — so you do not assemble one from thirty plugins yourself. Language support arrives as **extras** you switch on: `:LazyExtras`, tick `lang.rust`, restart.

Ticking that box installs `rustaceanvim` (the Rust front end proper), `crates.nvim` (versions and features inline in `Cargo.toml`), the Rust and RON tree-sitter parsers, and — if `mason.nvim` is present — the `codelldb` debug adapter. `rustaceanvim` is the interesting one: it is a fork of the old `rust-tools.nvim` that goes past ordinary LSP into the extras `rust-analyzer` offers and generic clients never ask for — expand this macro, show me the HIR or MIR, draw the crate graph, run *this* test, explain this error code.

**Pros**

- **Genuinely one line of setup for Rust**, once Neovim itself is set up. The extra is well made and it is maintained by people who use it daily.
- **`rustaceanvim`'s extras are a real capability difference**, not a preference. Macro expansion and MIR views are the tools you want on exactly the day Rust stops making sense.
- **It is fast and it is text.** Over `ssh`, in `tmux`, on a machine with no display server, it is the same editor.
- **You own the config.** Everything the editor does is Lua you can read, in a directory you can put in git. Nothing about the setup is a vendor's decision.
- **The motions.** If they are already in your fingers, no other editor gives them back to you at full strength, and every editor here that offers "vim mode" offers a subset.

**Cons**

- **It is a configuration, so it is a thing you now maintain.** Plugins move; a spec that worked in March throws a Lua error in September; upgrading is a task rather than a notification.
- **A moving version floor, and it fails quietly.** Verified below: on Neovim **0.11.5** — one minor version behind current — the `lang.rust` extra installs perfectly, compiles its parsers, and then never starts a language server, because `rustaceanvim` now requires **0.12**. The only sign is one line in a wall of install output.
- **The learning cost lands on the wrong budget.** If vim motions are not already automatic, the hours go on the editor at exactly the time you wanted to spend them on ownership and lifetimes — and unlike Rust, they buy you nothing you can put in a program.
- **It looks like an IDE and is not one.** Debugging works after you turn on more extras; the test runner is a plugin; a proper project-wide rename is `rust-analyzer`'s, not the editor's.
- **`rustaceanvim` insists on owning `rust-analyzer`.** Set the server up yourself through `nvim-lspconfig` as well and you get two clients fighting; its README says so in bold, which tells you how often it happens.

### What actually happened when I installed it

Installed into a throwaway config so nothing existing was touched — `NVIM_APPNAME` makes this free, and it is the honest way to try a distro:

```bash
git clone https://github.com/LazyVim/starter ~/.config/lazyvim-trial
rm -rf ~/.config/lazyvim-trial/.git
# then enable the Rust extra, and let it install:
NVIM_APPNAME=lazyvim-trial nvim --headless "+Lazy! sync" +qa
```

The numbers from that run (LazyVim 16.0.0, macOS, August 2026):

| | |
|---|---|
| Plugins installed | **34** |
| Disk | **152 MB** |
| Tree-sitter parsers compiled | 29 |
| Time to a working editor | a few minutes, unattended |
| Debug adapter (`codelldb`) | download aborted when the headless session exited — an artifact of installing this way; an interactive first launch finishes it |
| Rust language server attached, on Neovim **0.11.5** | **none** |
| Rust language server attached, on Neovim **0.12.5** | `rust-analyzer`, 3 diagnostics, hints `: i32` and `: f64` |

On the older Neovim the failure is the part worth having: `rustaceanvim requires Neovim 0.12 or above`, printed once, between two hundred lines of parser downloads. Nothing else complains. You still get syntax highlighting — tree-sitter does not care about the version — so the editor looks entirely healthy while giving you no types, no errors, no hints and no clue why. The server was not the problem: started by hand on that same Neovim, `rust-analyzer` came up and reported the deliberate type error in the test file. The plugin had simply declined to start it.

Run the identical config under **0.12.5** and everything advertised is there: the server attaches, `rust-analyzer` reports `expected u8, found &'static str` alongside the two `rustc` diagnostics, and the hints it draws on

```rust
let seats = 3;      //  : i32
let quota = 0.5;    //  : f64
```

are the same two claims the program above this section makes the compiler check. The extra works exactly as advertised. It works on a Neovim newer than the one Homebrew had installed here.

So the fix is `brew upgrade neovim`, and *that* is the finding rather than an aside: **this path means owning a version floor that moves underneath you**, and noticing when it does. Nothing else on this page can fail this way, because nothing else on this page lets you assemble a combination nobody shipped.

---

## Helix

A modal editor written in Rust with the language-server client *built in*. There is no plugin to install for Rust and no config file to write: `hx main.rs` in a Cargo project has hints, diagnostics and code actions.

**Pros**

- **The fastest possible route from nothing to a working Rust editor.** Install, open, done.
- **Modal editing with no maintenance burden** — the thing many people actually wanted from Neovim.
- **Fast, small, and predictable across machines**, because there is no config to drift.

**Cons**

- **The keys are not vim's.** Helix is selection-first (object, then verb) in the Kakoune tradition, so `d`, `c` and `y` do not compose the way your fingers expect. Coming from vim this is a re-learn, not a transfer.
- **No plugin system in a stable release.** The Steel (Scheme) system has been in development in a fork and a long-running pull request for years; if the built-in feature set does not cover your need, there is currently no supported way to add it.
- **Fewer escape hatches** in general — the price of having nothing to configure.

---

## Emacs

`eglot` has been in Emacs since 29, so `rust-analyzer` needs no third-party package; `lsp-mode` plus `rustic` is the heavier, more featureful alternative.

**Pros:** everything else in your life is probably already in it; `magit` has no equal anywhere on this page; the config is a program.
**Cons:** all of the Neovim maintenance costs, plus a smaller Rust-specific plugin community. If you are not already an Emacs user, nothing about Rust is a reason to become one.

---

## Anything without a language server

Plain `vim`, Sublime Text with no LSP package, `nano`, TextEdit. These are fine for editing a `Cargo.toml` and wrong for learning Rust, for one reason: without the analyzer you find out about a type error from `cargo build` at the end of a paragraph of code, instead of from the editor at the end of a line. The lesson is the same lesson; you just get it later and with more of your work to unwind. Come back to them when inference holds no surprises for you.

---

## The AI question, briefly

Every editor here now has an AI story: Cursor and Windsurf are VS Code forks built around one, Zed and the JetBrains IDEs ship their own, and Neovim has plugins for all of them. It matters less to this decision than the marketing implies, because a coding agent that runs in a terminal — Claude Code, or any of its neighbours — sits beside *whichever* editor you picked and does not care which one it is. So: choose the editor for what it tells you about your own code, and let the agent be a separate tool in a separate pane. An editor that has good hints and a bad AI is a much better place to learn Rust than the reverse.

---

## The setup trap that is not your editor's fault

If you installed Rust through **Homebrew's `rustup` formula** rather than the upstream `rustup-init` script, the standard advice — "point your IDE at `~/.cargo/bin`" — fails, because on that install `~/.cargo/bin` holds almost nothing. The shims are elsewhere:

```bash
command -v cargo            # /usr/local/opt/rustup/bin/cargo   ← Homebrew's shim directory
rustup which rust-analyzer  # ~/.rustup/toolchains/stable-…/bin/rust-analyzer   ← the real binary
```

So in RustRover's *Toolchain location*, or any field like it, use the shim directory (`/usr/local/opt/rustup/bin`) — and if a tool refuses a shell-script shim, give it the real toolchain `bin` that `rustup which` printed. Editors that just look for `rust-analyzer` on `PATH` need no help at all, since the shim is on it. This costs people an hour roughly once, and it looks exactly like the editor being broken.

---

## So which one?

**While you are learning Rust, use the editor that asks you no questions — and that is RustRover or VS Code, with Zed a close third.** Any of them has hints on within ten minutes, and the hours you do not spend on the editor go into the language instead. If you already own a JetBrains habit from another language, RustRover is the shortest path of all, and its independent analyzer is a real bonus while your own judgement is still forming.

**LazyVim is an excellent answer to a different question.** It is the best-curated Neovim distribution there is, and `rustaceanvim` gives it Rust features some of the alternatives do not have. But the question it answers is *"how do I get a modern Rust setup inside the editor I already think in?"* — and if you do not already think in vim, the honest cost is a second thing to learn, paid at exactly the moment the first thing is hardest. That is a fine trade when Rust is comfortable and a bad one in month one.

A reasonable order, then:

1. **Now:** whichever full editor you will not fiddle with. Turn on inlay hints. Learn Rust.
2. **Also now, occasionally:** a second editor for a second opinion — one `rust-analyzer` client and one RustRover — costs nothing and is genuinely useful on a confusing error.
3. **Later, deliberately:** if the terminal is where you want to live, take a weekend, upgrade Neovim *first*, install LazyVim into a throwaway `NVIM_APPNAME` config, and switch only when it stops being a project.

---

## If you are coming from another language

- **Python.** The map is nearly exact: RustRover is PyCharm, and VS Code plus `rust-analyzer` is VS Code plus Pylance. What changes is how much the hints are worth. Python's type information is optional and often absent, so an inlay hint is a guess made from what happened to be inferable; Rust's is total, so the hint is the *actual* type and there is no runtime surprise waiting behind it. Hints you could take or leave in Python are the main reason to have an editor at all here.
- **ABAP.** In ABAP the question does not exist: the editor is SE80 or ADT in Eclipse, welded to the system, and the intelligence lives in the server you are logged into. Rust inverts that. The intelligence is a *separate local program* (`rust-analyzer`) that any editor may drive, which is why this page can compare six of them and why "switching editors" does not mean losing your analysis. What transfers is the habit of trusting the tool's navigation — where-used, jump-to-definition — over grep. What is new is that the tool is yours, versioned in your toolchain, and upgraded when *you* upgrade it, not when Basis does.

---

## Sources

- [LazyVim](https://www.lazyvim.org/) and its [`lang.rust` extra](https://www.lazyvim.org/extras/lang/rust) — the plugin list quoted above
- [`rustaceanvim`](https://github.com/mrcjkb/rustaceanvim) — the Neovim version floor and the "do not also configure `lspconfig`" warning
- [rust-analyzer](https://rust-analyzer.github.io/) — the engine behind every editor here but RustRover
- [*Rust in Production: JetBrains*](https://serokell.io/blog/rust-in-production-jetbrains) (Serokell interview with Vitaly Bragilevsky) — the source for RustRover running its own Kotlin analyzer, and the reasoning for keeping it
- [RustRover is released, and includes a free non-commercial option](https://blog.jetbrains.com/rust/2024/05/21/rustrover-is-released-and-includes-a-free-non-commercial-option/) and the [non-commercial licensing FAQ](https://sales.jetbrains.com/hc/en-gb/articles/18950890312210-The-free-non-commercial-licensing-FAQ)
- [Zed's Rust documentation](https://zed.dev/docs/languages/rust)
- [Helix: the Steel plugin-system pull request](https://github.com/helix-editor/helix/pull/8675) — still open, still not in a stable release
- [2025 State of Rust Survey results](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results) — VS Code still dominant, Zed "a remarkable jump upward (with Helix as a good second)", and agentic editors eroding the incumbents
