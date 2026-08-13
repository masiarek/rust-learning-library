# Rust — Learning Library

<!-- --8<-- [start:hero] -->

A learning library for Rust, built the same way as its sibling [star-voting-library](https://github.com/masiarek/star-voting-library): **one idea per page, and every claim backed by a program that actually runs.**

No page here hand-types what a program prints. Each lesson links a real `.rs` file; a tool compiles it, runs it, checks the output against a recorded answer key, and pastes that verified output into the page. CI fails if any of the three drift apart. So when a page says *"this prints `Some(12)`"*, that is not a promise — it is a test result.

📖 **Read it as a site:** <https://masiarek.github.io/rust-learning-library/>

<!-- --8<-- [end:hero] -->

<!-- --8<-- [start:below-hero] -->

## Start here

| Lesson | What it teaches |
|---|---|
| [`Option` vs `Result`](01_Foundations/option_vs_result/README.md) | When absence is normal and when it is a failure — and the one question that decides which type you want |

More lessons land in [`01_Foundations/`](01_Foundations/README.md) as they are written.

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
