# Errors

[Foundations](../01_Foundations/README.md) covers the *types*: what an [`Option` is against a `Result`](../01_Foundations/option_vs_result/README.md), what [`?`](../01_Foundations/option_vs_result/README.md) does, what [`expect`](../01_Foundations/expect/README.md) claims, what [a panic costs](../01_Foundations/what_a_panic_costs/README.md). This section is the other half: what happens to a failure on its way out of your program — from the read that produced it, through the function that could not handle it, to the sentence a person reads and the number a shell script tests.

**These pages are stubs.** They are outlines waiting for a runnable example, written so the arc has a shape and a permanent URL before the prose exists. A page graduates by getting an `examples/` program and losing its stub notice — the same promise as everywhere else here: [no page claims something a program has not printed](../CONTRIBUTING.md).

| Lesson | Level | What it will teach |
|---|---|---|
| [Readers are fallible](readers_are_fallible/README.md) | 201 | Why a line arrives as `io::Result<String>` and not `String` — and what `.flatten()` quietly does to the failure |
| [Endless iteration](endless_iteration/README.md) | 201 | End of input is `Ok(0)`, not an error — so a loop that only watches for `Err` never stops |
| [`main` can return a `Result`](main_returns_result/README.md) | 201 | `?` at the top level, the `Debug` form the runtime prints, and which exit code you get |
| [Standard error, and exit status](stderr_and_exit_status/README.md) | 101 → 201 | Two streams and one number: what belongs on each, and what a caller can actually test |
| [Keep going, or stop](keep_going_or_stop/README.md) | 201 | One bad row is a design decision — `collect::<Result<Vec<_>, _>>()`, and the alternative that keeps both halves |
| [Not every error is an `io::Error`](not_every_error_is_io_error/README.md) | 201 | Two error types in one function, and the `From` hop `?` performs to make them one |
| [`anyhow` and context](anyhow_and_context/README.md) | 201 | Turning *"No such file or directory"* into an error that names the file it was reading |
| [`thiserror` vs `anyhow`](thiserror_vs_anyhow/README.md) | 301 | A library names its errors and an application erases them — the same crate on the wrong side of that line is the mistake |

## Where this arc goes

The order is deliberate: the first two pages are about failures you did not notice, the middle three about failures that reach the user, and the last three about the type they travel in. That is roughly the order a real program acquires them — you write the happy path, discover it lies, then discover the lie was structural.
