# Standard error, and exit status

**Level:** 101 → 201 · for newcomers

**One line:** A program has two output streams and one number: the streams separate *the answer* from *everything you want to say about it*, and the number is the only part another program reads.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `println!` against `eprintln!`, and the test that shows the difference: `prog > out.txt` still shows you the error
- Why a diagnostic on stdout corrupts a pipeline — `prog file | wc -l` counting your warning as data
- The exit status: zero is success, non-zero is failure, and that is nearly the whole convention
- What a shell sees in `$?`, and what `&&` and `set -e` do with it
- [`process::exit` ↗](https://doc.rust-lang.org/std/process/fn.exit.html) ends the process *immediately* — no destructors, no flush — so a buffered write can be lost
- Presenting a failure to a person: the program's name, the thing it was doing, the underlying cause; no stack trace

## The trap it exists for

Exit status is invisible during development, because you are reading the output rather than testing the number. It is the *only* thing a script, a CI step, or a `Makefile` can see — so a program that prints `error: no such file` and exits 0 reports success to everything except a human.

## If you are coming from another language

- **Python** — `sys.stderr` and `sys.exit(1)`, exactly. What changes is the default: an uncaught exception exits non-zero for you, while in Rust returning an `Err` from a function nobody checks is a *warning*, not an exit.
- **ABAP** — the closest thing is a message type plus `sy-subrc`, which is the same idea split across two mechanisms. The difference worth naming: a Rust exit status is one byte the operating system carries to the caller, so it survives leaving the program.

## See also

- [`main` can return a `Result`](../main_returns_result/README.md) — the shortest path to a non-zero status, and what it prints on the way
- [Testing a command](../../03_Command_Line/testing_a_command/README.md) — asserting the status and the stream from an integration test
