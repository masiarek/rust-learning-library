# CLAUDE.md — Rust learning library

Standing guidance for Claude working in this repo. Adam's main library as of 2026-09-08.

## What this is

A **Rust learning library**: ~500 one-idea-per-page lessons across 33 numbered sections, every one backed by a program that actually ran. Public repo `masiarek/rust-learning-library`, published to <https://masiarek.github.io/rust-learning-library/> by MkDocs Material straight from the repo's own Markdown. Toolchain pinned to **1.98.0** in `rust-toolchain.toml` — read that file's comments before touching the pin; bumping it means re-verifying every answer key.

## The rules live in CONTRIBUTING.md — read it, don't restate it

[`CONTRIBUTING.md`](CONTRIBUTING.md) (236 lines) is the authority on everything about writing a lesson: the folder shape, the `<!-- output:<stem> -->` contract, `--only` with `--update`, stubs, katas and `KATAS.md`, example naming (`foo`/`bar`/`baz` vs a concrete type), prose style, linking, moving a page, sidebar order, the pre-commit gates, and the shared-checkout habits. **Read it at the start of any lesson work.** It fires while you write; this file does not.

This file carries only what CONTRIBUTING doesn't — the operational context around the work.

**The one promise, so it's stated somewhere you'll definitely see it:** a page never claims something a program has not actually printed. `tools/run_examples.py` compiles every `examples/*.rs`, runs it, diffs against the sibling `.out`, and fills the page's output blocks. Inside the markers is generated; outside is yours.

## Where the work is right now

The **encodings library** (`/Volumes/T7/Encodings/encodings-learning-library`, an `additionalDirectories` entry, published to <https://masiarek.github.io/encodings-learning-library/>) is the active sibling, and the two are deliberately entangled:

- Encodings **links into this repo rather than repeating it** — `19_Numbers/meet_the_byte`, `why_hexadecimal`, `14_Strings/meet_the_char`, `anatomy_of_a_string`, `string_slices`. Check here before writing a Rust page there.
- The recent commit run is **return links** pointing back the other way, as pages on the far side get written. When you finish a page that a sibling library stubs or references, add the return link.
- Encodings runs **four languages** (`_py.py`, `_rs.rs`, `_sh.sh`, `_c.c`) and CI on **ubuntu *and* macos**, so its platform-split discipline is much heavier than this repo's. Its own `CONTRIBUTING.md` lists eleven measured BSD/GNU differences; don't re-derive them.

Python, Math and Java libraries are siblings on the same pattern and also cross-link.

## Never quote what the installed toolchain can tell you

Every page here is an answer key, so a signature or lint level recalled from training is exactly the failure this repo exists to prevent. All three of these are one command away:

- **Method lists** — std source, if `rust-src` is on the active toolchain: `$(rustc --print sysroot)/lib/rustlib/src/rust/library/{core,alloc,std}/src`. Walk *backwards* from the `fn` over the attribute run; a forward scan silently drops half the methods, and a multi-line `#[deprecated(` is invisible to a single-line match.
- **Signatures and const-ness** — the *rendered* docs, not the source: `~/.rustup/toolchains/<version>-*/share/doc/rust/html/`. The source marks things `const fn` that a separate `#[rustc_const_unstable]` gate makes uncallable on stable — four `Vec` pages asserted a false signature this way. `rust-src` and `rust-docs` are installed per toolchain and need not both be present on the same one.
- **Clippy lint levels and groups** — `clippy-driver -Whelp` from the pinned sysroot. Lints get regrouped between releases, so a confident wrong claim here is a *stale* fact arriving with authority.

Then **probe-compile before writing a word**: attributes in the source are not proof a method is callable on 1.98.0.

## Cross-platform: only Docker asks the real question

CONTRIBUTING says no local gate catches a machine-dependent example, and that a local gate can't — both true of this repo's *gates*. But the machine can be asked directly, and it's cheap (~15s; `rust:slim` and several pinned tags are already pulled):

```bash
docker run --rm -v "$PWD/<page>/examples:/ex:ro" rust:slim \
  sh -c 'cd /tmp && rustc --edition 2024 /ex/<stem>.rs -o b && ./b' > /tmp/linux.out
diff /tmp/linux.out <page>/examples/<stem>.out
```

Do this for anything touching a backtrace, a panic message, `size_of`, a path, or an environment variable. Not needed for arithmetic. And after pushing a batch of examples, actually look at `gh run list --workflow=examples` — a green docs deploy is **not** evidence the examples gate passed; they are separate workflows.

## Parallel sessions: use a worktree

**Adam's standing rule (2026-09-07): every parallel session gets its own worktree.** Sessions run concurrently on this machine often — check `git worktree list` and `pgrep -f run_examples.py` in the first minute, before assuming a task is unclaimed. A shared checkout gives everyone one working tree, one index and one HEAD, and its failures are silent rather than loud: no gate catches them, only a person re-running a claim.

```bash
git worktree add -b <topic> <scratchpad path> origin/master
```

Then commit, rebase onto `origin/master`, run the gates yourself, fetch late, and push the **literal SHA** — never `HEAD:master`, which re-resolves at push time and carries passengers. Symlink `.venv` in, never `git stash`, and remove the worktree when done. CONTRIBUTING's *When several people share the checkout* section is still worth reading — it's the survival guide for when you're in the main tree anyway.

**Message peers on state, not on intentions.** A message naming a SHA and what's on master is still true when it arrives late; one announcing what you're about to do is stale the moment the other session commits.

## Working with Adam

- **Be self-driven.** Analyze, decide, build, commit, push. Don't stop to ask permission for repo-internal work or offer a menu of options — report what you did and flag anything genuinely uncertain. Adam reviews after the fact, by reading the pages.
- **Commit after every completed piece**, with a real message: imperative summary line, then a body saying what and why. **Never `-m` with backticks in it** — it runs as command substitution and hangs; use `-F`.
- **Markdown prose is not hard-wrapped.** One paragraph per line, soft wrap. Real line breaks only where they're semantic.
- **Show expected output as a trailing comment** on the printing line, so a pasted snippet carries its own answer.
- **Lead a lesson with the working answer**, never with broken code at the top of the page.
- **Counts come from `git grep`**, never bare `grep` — Adam's `grep` is ugrep, which ignores `--include` and searches the whole tree anyway. Label what a count counted; it's pattern-sensitive.
- **Never paste output you piped through a formatter** to read it. `tr -s ' '` and `sed 's/ *$//'` delete exactly the padding evidence a platform-split page is about. Build the fence from the live run.
- `mdview <file.md>` opens Markdown rendered in the browser — hand it to him beside any `.md` deliverable, since he clicks a link to *read* it.

## Memory

Per-project memory lives in `~/.claude/projects/<cwd-slug>/memory/`, keyed by working directory. The store is **shared between concurrent sessions in the same project** — make every edit an assertion (read, check the expected string is present, replace, write), never a blind append, which silently reverts a peer.
