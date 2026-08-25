# Scaffolding a practice tree: the files that have no owner

**Level:** 201 · working knowledge

**One line:** A script that templates each new project is worse than `cargo new` inside a workspace — but the workspace *root*, everything under `.idea/`, and the gap between what your config files demand and what is actually installed have no owner at all, and those three are what a scaffolding script is for.

The script: [`rust_scaffold.py` ↗](https://github.com/masiarek/rust-learning-library/blob/master/05_Tooling/scaffolding/rust_scaffold.py). Stdlib only, three subcommands, and every transcript below is a real run of it.

```sh
python3 rust_scaffold.py init ~/rust-practice
python3 rust_scaffold.py new ex01_hello --workspace ~/rust-practice
python3 rust_scaffold.py adopt ~/RustroverProjects/untitled
python3 rust_scaffold.py doctor ~/rust-practice
```

## Does it make sense? Six answers, and they are not all yes

| Candidate for automation | Verdict | Why |
|---|---|---|
| A template for each new exercise | **No** | `cargo new` inside a workspace already does it, and does it better — see below |
| The workspace root itself | **Yes, once per tree** | Seven files nobody writes for you, each easy to get subtly wrong and impossible to remember six months later |
| `.idea/` run configurations | **Yes, and it keeps paying** | Outside Cargo entirely; no inheritance mechanism reaches it, so this is the one part a workspace can never replace |
| The IDE's own settings — linter, on-the-fly analysis | **Impossible** | They are IDE-global, not project files. A script can only *tell you* to set them |
| Checking what is declared against what is installed | **Yes — the best one** | Nothing else does it, and the failures are silent by construction |
| Absolute paths in compiler output | **Yes** | One line, and it is the one file here that only an absolute path can express — so it is also the one a `git clone` invalidates |
| The CI workflow | **Yes, marginally** | One file, written once — but it is the file that makes the toolchain pin mean anything |

The first row is the interesting one, because it is the row everybody starts with.

### Why not template the exercises

[A tree of practice projects](../practice_workspace/README.md) settles this and the argument does not need repeating, only pointing at: **a template copies configuration; a workspace shares it.** Forty generated exercise folders are forty copies of a lint policy you will change next week, and the ones you generated in March quietly become a fossil of what you believed in March. A workspace edits one line at the root and all forty change, including the ones you wrote first.

So `rust_scaffold.py new` does not template anything. It shells out to real `cargo new`, then adds only what `cargo new` cannot know about:

```text title="Real output — rust_scaffold.py new ex01_hello"
    Creating binary (application) `ex01_hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  cargo wrote the [lints] workspace opt-in itself
  seeded src/main.rs with a #[cfg(test)] module
  wrote .idea/runConfigurations/Run_ex01_hello.xml
```

Line three is the whole argument in one line of output. The manifest Cargo produced:

```toml
[package]
name = "ex01_hello"
version = "0.1.0"
edition.workspace = true

[dependencies]

[lints]
workspace = true
```

Nothing appended those last four lines and nothing wrote `edition.workspace = true` — Cargo saw `[workspace.lints]` and `[workspace.package]` at the root and opted the new member in by itself. A template would have had to *copy* both, which is the failure mode this whole page is arranged around.

## What `init` writes

```text
rust-practice/
├── rust-toolchain.toml          which compiler every command here gets, the IDE's included
├── Cargo.toml                   [workspace] + the lint policy + the dependency registry
├── clippy.toml                  the tests carve-out
├── rustfmt.toml                 the whitespace answer, for cargo fmt / the IDE / CI alike
├── .editorconfig                the same width, for the files rustfmt never sees
├── .cargo/config.toml           absolute paths in diagnostics — gitignored, it names THIS directory
├── bacon.toml                   the job re-run on every save
├── .gitignore
├── README.md
├── .github/workflows/rust.yml   fmt --check, clippy -D warnings, test
├── .idea/
│   ├── .gitignore               keeps workspace.xml out of git
│   ├── vcs.xml                  registers the git root, so VCS works on first open
│   └── runConfigurations/       Clippy (all targets) · Test (workspace) · Format check
└── exercises/                   empty — cargo new fills it
```

Re-running is safe, which is what makes it usable on a tree that already exists:

```text title="Real output — a second `init` on the same directory"
0 written, 15 kept.
```

Nothing is clobbered without `--force`, so picking up a new run configuration months later is just running `init` again.

## The nightly decision, and the one demo that justifies it

The default channel is a **dated** nightly — `nightly-2026-08-25`, not `nightly`. [Nightly by default](../nightly/README.md) is the page on why the date matters and why this belongs in `rust-toolchain.toml` rather than in `rustup default`. What is worth showing here is why a *learning* tree wants nightly at all, because "the interesting features" is not the reason.

It is `rustfmt`. Most of its useful options are nightly-only, and the failure mode when you mix them up is the nastiest kind: it looks like nothing.

Same `rustfmt.toml`, same three `use` lines, two trees differing only in the channel their `rust-toolchain.toml` names:

```rust
use std::fmt::Debug;
use std::collections::HashMap;
use anyhow::Result;
```

```text title="Real output — `cargo fmt --all` on the nightly tree"
use std::{collections::HashMap, fmt::Debug};
use anyhow::Result;
```

```text title="Real output — `cargo fmt --all` on the same files, with the channel set to stable"
Warning: can't set `wrap_comments = true`, unstable features are only available in nightly channel.
Warning: can't set `format_code_in_doc_comments = true`, unstable features are only available in nightly channel.
Warning: can't set `imports_granularity = Crate`, unstable features are only available in nightly channel.
Warning: can't set `group_imports = StdExternalCrate`, unstable features are only available in nightly channel.

use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Debug;
```

`group_imports = "StdExternalCrate"` put `std` first and the external crate second; `imports_granularity = "Crate"` merged the two `std` lines. On stable both are dropped and the imports are left in one flat alphabetical run. The warnings go to stderr, where a build pipeline usually swallows them — so the observable symptom is `cargo fmt --check` failing in CI on a diff the author cannot reproduce, on a machine that formats "correctly" every time.

Which makes the pairing a rule rather than a preference: **nightly-only options in `rustfmt.toml` require a nightly pin in `rust-toolchain.toml`, for everyone, laptop and runner alike.** `init` writes both together or neither, and `doctor` fails if you later separate them.

The pin installs itself, incidentally — the first `cargo` command in the fresh tree printed this before doing anything else, having been asked for a toolchain that was not on the machine:

```text
info: syncing channel updates for nightly-2026-08-25-x86_64-apple-darwin
info: latest update on 2026-08-25 for version 1.100.0-nightly (e7769602a 2026-08-24)
```

That is also how CI gets it: `rustup show` as the first step, no toolchain action, no version anywhere but the one file.

## The lints a scratch tree turns off

The [strict clippy policy](../strict_lints/README.md) is about lints to turn *on*. A practice tree needs one decision in the other direction, and `init` writes it:

```toml
[workspace.lints.rust]
unused_variables = "allow"
unused_imports = "allow"
unused_mut = "allow"
dead_code = "allow"
```

The reason is what a practice file *is*. It exists to show a form — five ways to make a `String`, each bound to a name nobody ever reads — and `unused_variables` fires on every single one:

```text
warning: unused variable: `a`
 --> src/main.rs:5:9
  |
5 |     let a = literal.to_string();
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
```

Five of those, on a six-line program that is doing exactly what it was written to do. The suggestion is not even wrong — `_a` is the right fix in real code — but taking it renames the thing the lesson is about, and refusing it five times a session teaches the one habit no lint policy survives: that warnings are wallpaper. A tree where a clean run means something is worth more here than four lints that are right about code nobody is going to keep.

**`unused_must_use` is deliberately not in that list**, which is why it is four lines rather than the single `unused = "allow"` the warning's own footer suggests:

```text
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
```

That footer names the group, and reaching for the group is the obvious move. But `unused_must_use` is in it too, and it fires on an ignored `Result` — a dropped error, not an unread name. Those are not the same kind of noise, and only one of them is noise at all:

```text title="Real output — the same tree, on a program that ignores a Result"
warning: unused `std::result::Result` that must be used
```

Pass `--warn-unused` to `init` if you want the four back.

The projects that need this most are the ones `init` never touches. RustRover's *New Project* dialog runs a plain `cargo new`, so every `untitled2` starts out printing `--> src/main.rs` and warning about the bindings the file exists to demonstrate — and by the time it annoys you, the project already exists. `adopt` is `init`'s two scratch defaults applied to a directory that already has a `Cargo.toml`:

```text title="Real output — rust_scaffold.py adopt, on a fresh cargo new"
  wrote  .cargo/config.toml — diagnostics now name the file
  wrote  [lints.rust] in Cargo.toml — unused bindings no longer warn
  wrote  .gitignore entry (the remap names THIS directory)

3 change(s). The next build recompiles: rustflags are part of the fingerprint.
```

It reads the manifest to decide which table to write, because that choice is silent when wrong: `[workspace.lints.rust]` in a package manifest is simply never read, and `[lints.rust]` at a workspace root applies to nothing. Running it twice does nothing the second time. And note the mechanism is the same one the clippy policy uses, so it inherits the same way and has the same silent failure: a member with no `[lints] workspace = true` gets none of it, which is the check `doctor` grew for it.

## The RustRover half

This is the part with no Cargo equivalent, and therefore the part where a script is not merely convenient.

**What a project file can carry.** `.idea/runConfigurations/*.xml` populates the run dropdown before the IDE has ever indexed the project, so a freshly-cloned tree opens with *Clippy (all targets)*, *Test (workspace)*, *Format check* and one *Run …* per exercise already in it:

```xml
<component name="ProjectRunConfigurationManager">
  <configuration default="false" name="Clippy (all targets)" type="CargoCommandRunConfiguration" factoryName="Cargo Command">
    <option name="command" value="clippy --all-targets --all-features" />
    <option name="workingDirectory" value="file://$PROJECT_DIR$" />
    <option name="emulateTerminal" value="true" />
    <method v="2" />
  </configuration>
</component>
```

The two strings that have to be exactly right are `type` and `factoryName`, and getting either wrong fails silently — the configuration simply does not appear, with nothing logged. They are not guessed here: they were read out of the installed RustRover 2026.2.1's own `intellij.rustrover.core.jar`, where `CargoCommandConfigurationType` hands the id `CargoCommandRunConfiguration` to `ConfigurationTypeBase`, and `CargoConfigurationFactory.getId()` returns `Cargo Command`. If a future release renames them, the symptom is an empty dropdown and this paragraph is where to start.

`.idea/vcs.xml` is the other one worth writing: without it the IDE opens a git-backed project with no VCS integration until somebody notices and points it at the root by hand.

**What a project file cannot carry.** The setting the [RustRover setup](../rustrover_setup/README.md) page opens with — *Settings → Rust → External Linters → Clippy*, replacing the default Cargo Check — is IDE-global, not per-project. No file in the repository sets it and no scaffolder can. So the script does the only honest thing and says so on every `doctor` run:

```text
  ! Set by hand, once, in the IDE — no project file can carry it:
      Settings → Rust → External Linters → Clippy (the default is Cargo Check)
```

**And one file that must not be shared:** `.idea/workspace.xml`, which records your window layout, your open tabs and your cursor positions, and changes every time you look at the project. `init` writes the `.idea/.gitignore` that excludes it, which is the same thing JetBrains' own IDEs write — the shared half of `.idea/` (run configurations, code style, VCS mapping) belongs in git, and the per-user half does not.

## `doctor`, the subcommand that keeps earning

Everything above is a one-time job. This is the part that stays useful, because it checks the one thing no single file can: **whether what the tree declares matches what the machine has.**

```text title="Real output — rust_scaffold.py doctor, on a healthy tree"
  ✓ rustup on PATH
  ✓ cargo on PATH
  ✓ rust-toolchain.toml pins nightly-2026-08-25
  ✓ the pin names a version, not a moving channel
  · active toolchain here: nightly-2026-08-25-x86_64-apple-darwin (overridden by '/private/tmp/rust-practice/rust-toolchain.toml')
  ✓ the active toolchain is the pinned one
  ✓ component rustfmt
  ✓ component clippy
  ✓ component rust-analyzer
  ✓ component rust-src
  ✓ rustfmt.toml's nightly-only options are backed by a nightly pin (format_code_in_doc_comments, group_imports, imports_granularity, wrap_comments)
  ✓ rustfmt max_width and .editorconfig max_line_length agree (100)
  ✓ bacon
  ✓ cargo-nextest
  ✓ all 1 member(s) opt in to the workspace lint policy
  ✓ compiler output names files absolutely (the remap points at this tree)
  · .idea/ present, 4 run configuration(s)
  ✓ .idea/workspace.xml is ignored (it is per-user churn)

0 problem(s).
```

The same tree with the channel switched to `stable` and `.editorconfig` widened to 120 — two edits nobody would think of as related to anything:

```text title="Real output — the same tree, two plausible edits later"
  ✗ channel = "stable" moves under you — pin a version or a dated nightly
  ✗ rustfmt.toml sets nightly-only options (format_code_in_doc_comments, group_imports, imports_granularity, wrap_comments) but the channel is "stable" — they are silently ignored, so CI and your editor will disagree
  ✗ rustfmt says 100, .editorconfig says 120 — the IDE's ruler will not match the formatter
```

Every one of those three is a *cross-file* invariant, which is exactly the category no single tool owns. Cargo does not read `rustfmt.toml`. rustfmt does not read `.editorconfig`. The IDE reads `.editorconfig` and not `rustfmt.toml`. Each tool is behaving correctly and the tree is still wrong, and the only thing that can notice is something that reads all of them at once.

Three more in the same family, all cheap and all real:

- **`active toolchain is not <pinned>`** catches a `rustc` that is not a rustup shim — a Homebrew or Nix binary — which ignores `rust-toolchain.toml` completely. That is the standard explanation for a pin that appears not to apply, and nothing else on the machine will tell you.
- **`component rust-analyzer` missing** catches the failure where the rustup shim, asked for a component the pinned toolchain lacks, falls back to itself and loops: `error: infinite recursion detected`. The editor shows no language server and no useful reason.
- **The path remap pointing somewhere else** catches a tree that has been moved, renamed or cloned since `init` wrote `.cargo/config.toml`. It is the only check here whose failure is *louder* than the thing it replaced: the diagnostics keep printing confident absolute paths, at the old location.

```text title="Real output — the same tree, copied to a new name"
  ✗ the path remap points at /private/tmp/rust-practice, but this tree is /private/tmp/rust-practice-moved — every diagnostic names a file that is not here; re-run `init` with --force
```

## What it deliberately does not do

- **No `src/` templates beyond a seven-line seed.** A scaffolder with opinions about your code is a scaffolder you fight.
- **No dependency installation.** `[workspace.dependencies]` is a version registry; a member opts in with `anyhow = { workspace = true }` when it actually uses it. Nothing is compiled until something needs it.
- **No `rustup default`.** Everything it decides about the toolchain is written in a file inside the tree, so it is visible to a reader and to CI. Changing the machine on a user's behalf is exactly the class of action that leaves no evidence.
- **No IDE settings it cannot verify.** See above.

One detail from building it, kept because it is the kind of thing that undermines a tool quietly: the first version's generated seed tripped its own lint policy — `missing_const_for_fn` from `nursery` and `doc_markdown` from `pedantic`, both on seven lines of hello-world. A scaffolder whose own output warns on the first run teaches its reader to ignore warnings, so the seed is now `const fn` with a backticked doc comment, and `clippy --all-targets`, `fmt --check` and `test --workspace` are all clean on what it writes.

## The default lint profile, and why it is `warn`

`--lints learn` is the default and writes `pedantic` and `nursery` at **warn**. [Strict clippy lints](../strict_lints/README.md) calls this the 80% version — the two teaching groups without the panic policy — and the reason to default there rather than at `deny` shows up immediately on the generated seed:

```text
warning: this could be a `const fn`
  = note: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
```

At `deny` that same line is a build failure on code with no defect. `nursery` means *lints still under development*, and meeting a suggestion as a compile error while you are learning is a good way to end up deleting the lint block. `--lints strict` writes the full policy, panic set included, minus `arithmetic_side_effects` — the one that fires on `n + 1` between two integers and changes how everything reads.

## If you are coming from another language

**Python.** The reflex is `cookiecutter`, and the mapping is exact: it is a template engine, it copies, and it has the same fossil problem — a project generated last year does not learn anything the template learned since. What has genuinely replaced it for most people is `uv init`, which is the same move Cargo makes: the tool writes the project, and shared configuration lives in one place the projects *read* rather than in a template they were stamped from. The part with no Python equivalent is the middle column of this page's verdict table — `.idea/`, or `.vscode/`, sitting outside the packaging tool entirely, which is why a Python scaffolder that only wrote `pyproject.toml` would be leaving the same gap. And `doctor` maps onto the thing every Python developer has written by hand at least once: the script that checks whether the interpreter on `PATH` is the one the project expects. Rust needs it for a narrower reason (a non-rustup `rustc` ignores the pin) but it is the same check.

**ABAP.** There is nothing to scaffold, and the reason is worth naming: a package, its transport layer, its ATC variant and its naming conventions are provisioned by Basis and inherited by everything created inside them, so the "workspace root" already exists and was created by someone else, once, for the whole system. Creating a report inherits the checks the way a workspace member inherits `[workspace.lints]` — the mechanism this page recommends is the one ABAP has always had. Two things are genuinely different. The configuration here is **files in your own repository**, so the equivalent of "which ATC variant is blocking" is a diff you can read and argue with rather than a setting in a system you may not administer. And the IDE half has no ABAP counterpart at all: SE80 and ADT read their settings from the system and from your user profile, never from the object, so there is no file to check into a transport that makes a colleague's editor behave like yours.

## See also

- [A tree of practice projects](../practice_workspace/README.md) — the argument this script is built around, and the four mechanisms that make templating unnecessary
- [Pinning the toolchain](../pinning_the_toolchain/README.md) — what goes in `rust-toolchain.toml` and why the version is written out
- [Nightly by default](../nightly/README.md) — three scopes for choosing nightly, one of which is recorded nowhere
- [Formatting](../formatting/README.md) — the stable/nightly rustfmt split demonstrated above, in more depth
- [Strict clippy lints](../strict_lints/README.md) — both profiles the script can write, and the one line to leave out
- [RustRover setup](../rustrover_setup/README.md) — the settings that are not project files, including the one `doctor` reminds you about
- [bacon](../bacon/README.md) — what `bacon.toml` is for
- [A throwaway that needs a crate](../scratch_with_a_crate/README.md) — the opposite end: when one `cargo new` is the whole answer and none of this applies
