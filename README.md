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
| [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md) | How to run a `.rs` file at all — `rustc` alone, `cargo new`, and the edition flag Cargo would have passed for you |
| [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md) | When absence is normal and when it is a failure — and the one question that decides which type you want |

[`01_Foundations/`](01_Foundations/README.md) is the map of everything a first week of Rust needs, now split across six sections by subject rather than kept in one flat run: [first programs](15_First_Programs/README.md), [structs](16_Structs/README.md), [`Option` and `Result`](17_Option_and_Result/README.md), [ownership](18_Ownership/README.md), [strings](14_Strings/README.md) and [numbers and bytes](19_Numbers/README.md). Lessons about the toolchain rather than the language — `cargo`, build profiles, compile times — are in [`05_Tooling/`](05_Tooling/README.md); the ones that assume the foundations — shared state across threads, `unsafe`, FFI — in [`09_Advanced/`](09_Advanced/README.md).

[`13_Enums/`](13_Enums/README.md) and [`12_Traits/`](12_Traits/README.md) sit directly after the foundations, because between them they are what the rest of the language is made of. Enums come first: anyone leaving the foundations has already used several of them — `Option` and `Result` among them — without being told that the feature has a name, or that a `match` which forgets a variant is a build error. Traits are the other half, and the same argument applies twice over: `Copy`, `Display`, `Iterator` and `From` all turn up in the earlier pages long before anything explains what they have in common.

[`22_Generics/`](22_Generics/README.md) follows both, because it needs both: `<T>` is how one definition serves every type, and a trait is what tells the compiler which types are allowed. It is also the section that explains the brackets in `Option<T>`, `Vec<T>` and `Result<T, E>` — read a hundred times by then, and nowhere defined.

One section is not about Rust at all. [`11_Unix/`](11_Unix/README.md) is the shell you run the compiler from — `fzf` for picking a file instead of typing its path, `fd` and `rg` for finding one — and it earns its place because two of those three are themselves Rust programs, measured here against the Unix tools they stand in for.

[`20_Compilers/`](20_Compilers/README.md) is the layer under all of it. Most of what turns a `.rs` file into a binary was not written for Rust — rustc parses and borrow-checks, and then LLVM optimizes, LLVM generates machine code, and a linker nobody on the Rust team maintains finishes the job. The section covers the compile-time/run-time line, what an optimizer is allowed to do to your loop, what LLVM actually is, what the linker does with the blanks rustc leaves, and the same machinery aimed backwards as obfuscation.

Five sections in between follow one long arc: **turning a snippet into a program somebody else runs.** [`02_Errors/`](02_Errors/README.md) is what a failure does on its way out; [`03_Command_Line/`](03_Command_Line/README.md) is what the program was handed on the way in; and [`04_Files/`](04_Files/README.md), [`06_Data/`](06_Data/README.md) and [`07_Clients/`](07_Clients/README.md) are the three things outside it that can say no. **Those pages are stubs today** — one outline and a set of questions per page, with no runnable example behind them yet, each marked as such at the top. They exist so the arc has a shape, and a permanent URL, before the prose does; a page graduates by acquiring an example and losing its notice.

[`21_Observability/`](21_Observability/README.md) picks up where that arc ends: the program is now the service somebody else calls, and the only account of what happened to one request is whatever it chose to write down — a span, a structured line, a counter, and a header carrying the trace to the next process. Stubs as well, and the hardest ones here to finish, because every checked example in this library compiles with `rustc` alone and observability in Rust is a crate story. The section README says how it intends to close that gap rather than skip it.

Five topics are big enough to have a map of their own rather than a single page: [**OPTION.md**](OPTION.md), [**SHADOWING.md**](SHADOWING.md), [**STRUCTS.md**](STRUCTS.md), [**STRINGS.md**](STRINGS.md) and [**TOOLCHAIN.md**](TOOLCHAIN.md) each collect every lesson on their subject in reading order, and say what the idea is before the syllabus starts. The first four cross section boundaries because the lessons do; the fifth sorts one section by the problem you actually have.

Every lesson explains; some of them also ask you to type. Those exercises are collected in [**KATAS.md**](KATAS.md), which is the only place they are ordered — each kata itself sits on the page for the topic it teaches, with a solution CI compiles and runs.

There is also a slow, optional thread running through them: [**the long way round to a STAR count**](ROADMAP.md), which sequences a handful of lessons so that each one is the next thing Rust wants to teach, and the running example happens to be a voting method. Every rung stands alone; the election is the excuse.

## The course, in order

The sidebar is sorted **alphabetically**, because that is how you find a section you can already name. This is the other question — *what should I read next?* — and it is the order these were written to be read in:

| # | section | what it is for |
|---|---|---|
| 1 | [Start here](00_Start_Here/README.md) | The three outside courses this library is a companion to |
| 2 | [Foundations](01_Foundations/README.md) | The map of a first week, pointing into the six sections below |
| 3 | [First programs](15_First_Programs/README.md) | Running a `.rs` file at all, and what the punctuation in it means |
| 4 | [Structs](16_Structs/README.md) | A compound type of your own, before the two the library leans on |
| 5 | [`Option` and `Result`](17_Option_and_Result/README.md) | The two enums everything returns, and the dozen ways to open them |
| 6 | [Ownership](18_Ownership/README.md) | Who owns the value — moves, borrows, and what a shadow does |
| 7 | [Strings](14_Strings/README.md) | Text: the owner and the view, and the bytes underneath |
| 8 | [Numbers and bytes](19_Numbers/README.md) | The unit all of that is counted in, down to the float that cannot hold your value |
| 9 | [Enums](13_Enums/README.md) | The feature `Option` and `Result` were made of all along |
| 10 | [Traits](12_Traits/README.md) | The other half the language is built from — and how a call reaches one |
| 11 | [Generics](22_Generics/README.md) | `<T>`: one definition per idea, instead of one per type it is used with |
| 12 | [Errors](02_Errors/README.md) | What a failure does on its way out of a program |
| 13 | [Command line](03_Command_Line/README.md) | What the program was handed on the way in |
| 14 | [Files](04_Files/README.md) | The filesystem — the first thing outside the program that can say no |
| 15 | [Tooling](05_Tooling/README.md) | `cargo` and the rest of the toolchain, rather than the language |
| 16 | [Data](06_Data/README.md) | Serialization, and the round trip through JSON |
| 17 | [Clients](07_Clients/README.md) | The network — the last thing outside the program that can say no |
| 18 | [Observability](21_Observability/README.md) | What the service says about itself, once somebody else depends on it |
| 19 | [Interfaces](08_Interfaces/README.md) | Putting a face on it |
| 20 | [Advanced](09_Advanced/README.md) | What needs the foundations: threads, `unsafe`, FFI |
| 21 | [Compilers](20_Compilers/README.md) | The layer under all of it, three quarters of which is not Rust |
| 22 | [Resources](10_Resources/README.md) | Books, essays and exercises outside this library |
| 23 | [Unix](11_Unix/README.md) | The shell you run the compiler from — two of its three tools are Rust |

Nothing enforces this order and no page depends on it; skipping around is fine. It is here because a sidebar can be sorted one way only, and A–Z answers the more common question.

## How the library works

```
17_Option_and_Result/
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
