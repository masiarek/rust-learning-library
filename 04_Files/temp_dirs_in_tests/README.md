# Temporary directories in tests

**Level:** 201 → 301 · deep dive

**One line:** A test that writes to a fixed path cannot run beside itself — and the usual fix, a self-cleaning temporary directory, deletes itself the instant you stop holding on to the handle.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why `/tmp/test_output.txt` fails: Rust's test harness runs tests in **threads, in parallel**, so two tests sharing a path are a race, and a leftover file from a crashed run is a test that passes for the wrong reason
- [`tempfile::TempDir`](https://docs.rs/tempfile) — a directory whose `Drop` removes it, and the sharp edge that follows: bind it to a variable, because `TempDir::new()?.path().to_owned()` hands you a path to a directory that has already been deleted
- Which is a very concrete instance of a rule this library already has a page for: a value lives until its owner goes out of scope, and a temporary's owner is the end of the statement
- What to assert: the *behaviour* (the file now contains the record) rather than the path, which differs every run
- Keeping an example deterministic when it touches a filesystem at all — the reason this section's pages are harder to finish than the others

## The trap it exists for

The dropped-too-early temp dir produces a `NotFound` from code that just created the directory, which reads as a filesystem or permissions problem and is neither. It is the ownership rules doing exactly what they say, in the one place a newcomer is not thinking about ownership.

## See also

- [Scope is about names, not values](../../01_Foundations/scope_is_about_names/README.md) — the schedule a value actually dies on, and the five things that move it
- [Testing a command](../../03_Command_Line/testing_a_command/README.md) — the tests that need a filesystem of their own
- [Ownership and moves](../../01_Foundations/ownership_and_moves/README.md) — a move transfers responsibility, and the temp directory is the responsibility
