# Not every error is an `io::Error`

**Level:** 201 · working knowledge

**One line:** A function that opens a file *and* parses a number has two error types and one return type — and `?` only bridges them if something can convert one into the other.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The error you actually get: `?` returns `Err(From::from(e))`, so a missing `From` impl is a **type** error at the `?`, not at the call
- `Box<dyn Error>` — the any-error type, what it costs (an allocation, and a value you can no longer `match` on), and `downcast_ref` for when you must
- The enum you write instead, with one variant per thing a caller might handle differently
- Writing the `From` impls by hand, and what that boilerplate looks like before a derive removes it
- Why the answer differs for a binary and for a library — the question the next page is entirely about

## The trap it exists for

`Box<dyn Error>` makes the compile error disappear in one keystroke, and it disappears the information with it. Once every failure is the same type, the only thing a caller can do is print it — which is exactly right at the top of a program and a dead end anywhere a caller wanted to *recover*.

## See also

- [`Option` vs `Result`](../../01_Foundations/option_vs_result/README.md) — Step 8 designs the `E`, Step 9 is when you would rather not
- [`thiserror` vs `anyhow`](../thiserror_vs_anyhow/README.md) — the same decision, with the two crates that own each side of it
- [The `Result` you are reading is probably an alias](../../01_Foundations/result_aliases/README.md) — how to expand `io::Result` and see the `E` you are trying to convert
