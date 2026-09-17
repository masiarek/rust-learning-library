# A first contribution to rustc

**Level:** 201 · working knowledge

**One line:** A first compiler pull request usually starts from an issue labelled `E-easy` or `E-mentor`, carries a test beside the fix, passes `tidy`, and is merged by `@bors` from the merge queue once a reviewer writes `@bors r+` — the rustc dev guide says PRs are never merged by hand.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Finding work: the dev guide's issue search for unassigned `E-easy`, `E-medium`, `E-help-wanted` and `E-mentor` issues; diagnostic issues (`A-diagnostics`), which rarely need deep compiler knowledge; `E-needs-test` issues that need only a regression test; and abandoned PRs labelled `S-inactive` that someone can pick up
- Asking before you start: who to ping, the `#t-compiler` channel on Zulip, and `git shortlog -n 1.68.2.. compiler/rustc_resolve/`-style searches for whoever touched that code recently
- The change itself: a fix plus a UI test in `tests/ui` with a blessed and hand-read `.stderr` ([Using a compiler you built](../using_a_compiler_you_built/README.md)) — and when a reviewer will also want documentation changed
- Committing only your files: the dev guide warns against `git add .`, which can sweep in a submodule change, and has a section on undoing exactly that
- `./x test tidy` before pushing, or `./x test tidy --bless` to run rustfmt first; CI runs tidy anyway and fails the PR if it fails
- Review: a reviewer is assigned, `r? @username` asks for a particular one, and `r=me after fixing …` means approval once the named change is made
- Merging: `@bors r+` puts the PR in the merge queue, where bors runs the tests on every supported platform; `@bors r+ rollup` batches small PRs that are unlikely to conflict; merged code appears in the next nightly
- The Project's policy on LLM-written code and LLM-assisted review, summarised in the dev guide's "Working with LLMs" page with Forge as the canonical text — what it asks of a contributor

## The trap it exists for

Rebasing, then running `git add .` before `./x` has updated the submodules. The commit now records a change to `src/tools/cargo` that nobody meant, and — as the dev guide puts it — the usual first sign is rustbot commenting on the PR that cargo has been modified. Stage the files you edited by name, and check `git status` before every commit.

## Where this sits

[The Rust Project](../the_rust_project/README.md) is the organisation and its release train; [Building the compiler](../building_the_compiler/README.md) and [Using a compiler you built](../using_a_compiler_you_built/README.md) get a change running and tested locally. This page is the social and procedural half: finding the issue, the pull request, review and merge.

## See also

- [The Rust Project](../the_rust_project/README.md) — who reviews, and which train your change boards
- [Using a compiler you built](../using_a_compiler_you_built/README.md) — UI tests and `tidy` before you push
- [Reading a compilation failure](../reading_a_compilation_failure/README.md) — the kind of diagnostic an `A-diagnostics` issue improves
- [What a compiler does before your program runs](../what_a_compiler_does/README.md) — the pipeline your change sits in
- [Printing the HIR](../printing_the_hir/README.md) — one way to see what the compiler did with a test program
- [rustup: the `rustc` you run is not the compiler](../../05_Tooling/rustup/README.md) — where your change will arrive, a nightly later
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — trying the merged change without building it
- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — why a fix you shipped reaches a pinned project only when it bumps
- [rustc dev guide: getting started ↗](https://rustc-dev-guide.rust-lang.org/getting-started.html) — easy and mentored issues, and whom to ask
- [rustc dev guide: the PR lifecycle ↗](https://rustc-dev-guide.rust-lang.org/pr-lifecycle.html) — review, `r+`, rollups and the merge queue
- [rustc dev guide: using Git ↗](https://rustc-dev-guide.rust-lang.org/git.html) — submodules, and "I changed a submodule by accident"
- [The `E-easy` label ↗](https://github.com/rust-lang/rust/labels/E-easy) — the open issues it marks today

## If you are coming from another language

- **Python.** CPython's devguide plays the rustc dev guide's part, "easy" issues are the entry point there too, and a core developer merges an approved PR; Rust hands the merge to a bot that runs the full test matrix first.
- **Go.** Changes are reviewed in Gerrit, where a Code-Review +2 approves a change that still has to pass the TryBots before it is submitted — approval and merge are separate steps there too, as `@bors r+` and the merge queue are here.

## Po polsku

Pierwszy wkład w kompilator zwykle zaczyna się od zgłoszenia oznaczonego `E-easy` albo `E-mentor`, zawiera test UI obok poprawki, przechodzi `./x test tidy`, a scala go bot `@bors` z kolejki (*merge queue*) po tym, jak recenzent napisze `@bors r+` — ręcznie nikt tu niczego nie scala. Typowa wpadka jest gitowa: `git add .` po rebase zapisuje w commicie przypadkową zmianę podmodułu (*submodule*) `src/tools/cargo`, więc dodawaj pliki po nazwie.

**Szukaj po polsku:** pierwszy wkład w kompilator Rusta · recenzja kodu w projekcie Rust · `rustc dev guide getting started` · `bors r+ merge queue` · `rust E-easy issues`
