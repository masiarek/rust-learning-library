# Deriving a parser with `clap`

**Level:** 201 · working knowledge

**One line:** With [`clap` ↗](https://docs.rs/clap)'s `derive` feature, a struct **is** the command-line interface: `#[derive(Parser)]` generates the parsing, the `--help` text, the `--version`, and the error message for a flag nobody defined.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Cargo features: `clap` does nothing until `features = ["derive"]` is on, and what a feature actually is
- The mapping — a `bool` field is a switch, an `Option<T>` is optional, a `Vec<T>` collects repeats, a plain `T` is required
- `#[command(...)]` metadata pulled from `Cargo.toml`, so the version in `--version` cannot drift from the version you shipped
- **Doc comments become help text** — the same `///` that [is really an attribute](../../01_Foundations/comments_that_compile/README.md), doing a third job
- `Parser::parse()` in `main` against `try_parse_from(["prog", "--words"])` in a test, which is what makes the interface unit-testable at all
- The counterweight: one flag and no dependencies does not need a dependency

## The trap it exists for

A derived parser is so cheap that it becomes the *specification* of the program without anyone deciding it should be. Rename a field and you have renamed a flag; make a field required and you have broken every caller's script. The struct is a public interface — the page should say so where a reader will meet it.

## See also

- [Flags by hand](../flags_by_hand/README.md) — the work this is replacing, and the conventions it gets right for free
- [Testing a command](../testing_a_command/README.md) — testing the parser and testing the program are two different tests
- [Comments that compile](../../01_Foundations/comments_that_compile/README.md) — why a `///` above a field can turn into help text at all
