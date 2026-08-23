# Endless iteration

**Level:** 201 · working knowledge

**One line:** [`read_line`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line) reports end of input by returning `Ok(0)` — success, zero bytes — so a loop that only watches for `Err` runs forever on a finished file.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The return value: `Ok(n)` is bytes read *including* the newline, and `Ok(0)` is the only end-of-input signal there is
- Why `while let Ok(_) = reader.read_line(&mut buf)` is an infinite loop, and why it looks correct
- The second bug in the same function: `read_line` **appends**, so a buffer you never `clear()` grows for the length of the file
- The persistent-error version — a reader that fails every call, looped over without checking
- Writing the test that would have caught it: a reader you control, feeding end-of-input or an error on demand, with the loop under a bound

## The trap it exists for

Both bugs pass every test you would think to write, because a test with three lines of input and a correct file ends by luck. They show up on an empty file, on a closed pipe, and in production — the classic shape of a bug that only the *absence* of data can trigger. The fix is not defensive code; it is one test whose input is nothing at all.

## If you are coming from another language

- **Python** — `for line in f` ends because the iterator raises `StopIteration` for you. Rust's `lines()` iterator does the same thing; `read_line` is the lower-level call where ending is *your* comparison to make.
- **ABAP** — this is `READ DATASET ... INTO` inside a `DO` loop: the exit is a condition you write, and forgetting it hangs the work process. Rust does not save you here either — but it does hand you a value that distinguishes "nothing left" from "it broke", which `sy-subrc` alone does not.

## See also

- [Readers are fallible](../readers_are_fallible/README.md) — the `Result` this loop is failing to look at
- [`while let`](../../01_Foundations/while_let/README.md) — the loop whose exit condition is a pattern, and the one bug it can have: a body that never makes progress
