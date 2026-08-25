# Rust — Learning Library

<!-- --8<-- [start:hero] -->

A learning library for Rust, built the same way as its sibling [star-voting-library ↗](https://github.com/masiarek/star-voting-library): **one idea per page, and every claim backed by a program that actually runs.**

No page here hand-types what a program prints. Each lesson links a real `.rs` file; a tool compiles it, runs it, checks the output against a recorded answer key, and pastes that verified output into the page. CI fails if any of the three drift apart. So when a page says *"this prints `Some(12)`"*, that is not a promise — it is a test result.

📖 **Read it as a site:** <https://masiarek.github.io/rust-learning-library/>

<!-- --8<-- [end:hero] -->

<!-- --8<-- [start:below-hero] -->

## Start here

**New to Rust entirely?** [**00_Start_Here/**](00_Start_Here/README.md) is the plan: which three free resources to use, in what order, and why each covers what the other two cannot.

Already writing Rust? Two lessons to begin with:

| Lesson | What it teaches |
|---|---|
| [Running a scratch program](01_Foundations/rustc_without_cargo/README.md) | How to run a `.rs` file at all — `rustc` alone, `cargo new`, and the edition flag Cargo would have passed for you |
| [`Option` vs `Result`](01_Foundations/option_vs_result/README.md) | When absence is normal and when it is a failure — and the one question that decides which type you want |

More lessons land in [`01_Foundations/`](01_Foundations/README.md) as they are written; the ones about the toolchain rather than the language — `cargo`, build profiles, compile times — in [`05_Tooling/`](05_Tooling/README.md); and the ones that assume the foundations — shared state across threads, `unsafe`, FFI — in [`09_Advanced/`](09_Advanced/README.md).

[`14_Strings/`](14_Strings/README.md) sits directly after the foundations too, and used to be part of them: ten lessons on the one pattern text follows in Rust, an owner and a view. They lean on the ownership pages hard enough that they were written there, and they outgrew it.

[`13_Enums/`](13_Enums/README.md) and [`12_Traits/`](12_Traits/README.md) sit directly after the foundations, because between them they are what the rest of the language is made of. Enums come first: anyone leaving the foundations has already used several of them — `Option` and `Result` among them — without being told that the feature has a name, or that a `match` which forgets a variant is a build error. Traits are the other half, and the same argument applies twice over: `Copy`, `Display`, `Iterator` and `From` all turn up in the earlier pages long before anything explains what they have in common.

One section is not about Rust at all. [`11_Unix/`](11_Unix/README.md) is the shell you run the compiler from — `fzf` for picking a file instead of typing its path, `fd` and `rg` for finding one — and it earns its place because two of those three are themselves Rust programs, measured here against the Unix tools they stand in for.

Five sections in between follow one long arc: **turning a snippet into a program somebody else runs.** [`02_Errors/`](02_Errors/README.md) is what a failure does on its way out; [`03_Command_Line/`](03_Command_Line/README.md) is what the program was handed on the way in; and [`04_Files/`](04_Files/README.md), [`06_Data/`](06_Data/README.md) and [`07_Clients/`](07_Clients/README.md) are the three things outside it that can say no. **Those pages are stubs today** — one outline and a set of questions per page, with no runnable example behind them yet, each marked as such at the top. They exist so the arc has a shape, and a permanent URL, before the prose does; a page graduates by acquiring an example and losing its notice.

Four topics are big enough to have a map of their own rather than a single page: [**OPTION.md**](OPTION.md), [**SHADOWING.md**](SHADOWING.md), [**STRUCTS.md**](STRUCTS.md) and [**STRINGS.md**](STRINGS.md) each collect every lesson on their subject in reading order, and say what the idea is before the syllabus starts.

Every lesson explains; some of them also ask you to type. Those exercises are collected in [**KATAS.md**](KATAS.md), which is the only place they are ordered — each kata itself sits on the page for the topic it teaches, with a solution CI compiles and runs.

There is also a slow, optional thread running through them: [**the long way round to a STAR count**](ROADMAP.md), which sequences a handful of lessons so that each one is the next thing Rust wants to teach, and the running example happens to be a voting method. Every rung stands alone; the election is the excuse.

## How the library works

```
01_Foundations/
  option_vs_result/
    README.md                       the lesson  (prose + code + a generated output block)
    examples/
      option_vs_result.rs           the program the lesson is about
      option_vs_result.out          its recorded output — the answer key
tools/run_examples.py               compiles, runs, compares, and refills the pages
```

A lesson marks the spot where output belongs and lets the tool fill it:

```markdown
<!-- output:option_vs_result -->
<!-- /output -->
```

Inside the markers is generated; outside is yours. Run the tool after any change:

```bash
python3 tools/run_examples.py
```

## Running things

| Task | Command |
|---|---|
| Verify every example and refresh the pages | `python3 tools/run_examples.py` |
| Accept new output as the answer key | `python3 tools/run_examples.py --update` |
| Check without writing (what CI runs) | `python3 tools/run_examples.py --check` |
| Preview the site locally | `uv run --group docs mkdocs serve` |
| Run one example by hand | `rustc --edition 2024 path/to/example.rs -o /tmp/x && /tmp/x` |

Only `rustc` and Python 3.11+ are needed for the examples; `uv` is needed only to preview the site.

## Adding a lesson

See [CONTRIBUTING.md](CONTRIBUTING.md) — it is short, and it is mostly about the one rule above.

<!-- --8<-- [end:below-hero] -->
