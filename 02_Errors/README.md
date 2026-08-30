# Errors

[Foundations](../01_Foundations/README.md) covers the *types*: what an [`Option` is against a `Result`](../17_Option_and_Result/option_vs_result/README.md), what [`?`](../17_Option_and_Result/the_question_mark_operator/README.md) does, what [`expect`](../17_Option_and_Result/expect/README.md) claims, what [a panic costs](../17_Option_and_Result/what_a_panic_costs/README.md). This section is the other half: what happens to a failure on its way out of your program — from the read that produced it, through the function that could not handle it, to the sentence a person reads and the number a shell script tests.

**Three of these pages are finished; the other seven are stubs** — outlines waiting for a runnable example, written so the arc has a shape and a permanent URL before the prose exists. A page graduates by getting an `examples/` program and losing its stub notice, which is the same promise as everywhere else here: [no page claims something a program has not printed](../CONTRIBUTING.md). The two crate pages at the bottom cannot graduate the usual way, because examples in this library build with no dependencies — see [CONTRIBUTING](../CONTRIBUTING.md).

| Lesson | Level | What it teaches |
|---|---|---|
| [Readers are fallible](readers_are_fallible/README.md) | 201 | Why a line arrives as `io::Result<String>` and not `String` — and what `.flatten()` quietly does to the failure |
| [Endless iteration](endless_iteration/README.md) | 201 | End of input is `Ok(0)`, not an error — so a loop that only watches for `Err` never stops |
| [`main` can return a `Result`](main_returns_result/README.md) | 201 | `?` at the top level, the `Debug` form the runtime prints, and which exit code you get |
| [Standard error, and exit status](stderr_and_exit_status/README.md) | 101 → 201 | Two streams and one number: what belongs on each, and what a caller can actually test |
| [Keep going, or stop](keep_going_or_stop/README.md) | 201 | One bad row is a design decision — `collect::<Result<Vec<_>, _>>()`, and the alternative that keeps both halves |
| [`unwrap` is a TODO you forgot to remove](unwrap_is_a_todo/README.md) | 201 | The three ways Rust panics, and why almost every `unwrap` you have arrived by paste from a crate's own front page — `serde`'s README unwraps twice |
| [What makes a type an error](the_error_trait/README.md) | 201 | `std::error::Error` asks for `Debug`, `Display` and one optional method — and `source()`, the optional one, is where hand-written errors go wrong |
| [Not every error is an `io::Error`](not_every_error_is_io_error/README.md) | 201 | Two error types in one function: erase the difference with `Box<dyn Error>`, or name it with an enum — and which one your caller needs |
| [`anyhow` and context](anyhow_and_context/README.md) | 201 | Turning *"No such file or directory"* into an error that names the file it was reading |
| [`thiserror` vs `anyhow`](thiserror_vs_anyhow/README.md) | 301 | A library names its errors and an application erases them — the same crate on the wrong side of that line is the mistake |

## Where this arc goes

The order is deliberate: the first two pages are about failures you did not notice, the middle three about failures that reach the user, and the last three about the type they travel in. That is roughly the order a real program acquires them — you write the happy path, discover it lies, then discover the lie was structural.
