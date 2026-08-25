# Readers are fallible

**Level:** 201 · working knowledge

**One line:** A read is a request to something outside your program, so it can fail *after* the file opened fine — which is why [`BufRead::lines()` ↗](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines) hands you an `io::Result<String>` per line rather than a `String`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why the `Result` is per **line**, not per file: opening succeeded, and the failure is somewhere in the middle
- What can actually go wrong that late — a disk or network read error, a pipe whose other end closed, and bytes that are not UTF-8 (`ErrorKind::InvalidData`)
- The three shapes of the loop, and what each one decides: `for line in reader.lines()` with `let line = line?;` inside, `.map_while(Result::ok)`, and `collect::<io::Result<Vec<String>>>()`
- What `lines()` does to the trailing newline — a separate question from failure, and the one that actually bites first
- Reading from standard input rather than a file: `io::stdin().lock()`, and why the same `Result` shows up there

## The trap it exists for

`.flatten()` and `.filter_map(Result::ok)` both make the type error go away, and both convert *"the disk failed halfway through"* into *"the file was shorter than it is."* The program then prints a smaller number with total confidence. A count that silently drops rows is worse than a count that stops, because nothing about the output says anything went wrong.

## See also

- [The `Result` you are reading is probably an alias](../../01_Foundations/result_aliases/README.md) — `io::Result<T>` expanded, and how to find out what `E` really is
- [Endless iteration](../endless_iteration/README.md) — the other half of the same loop: how it ends, and how it fails to
- [Keep going, or stop](../keep_going_or_stop/README.md) — what to do with the bad row once you have stopped throwing it away
