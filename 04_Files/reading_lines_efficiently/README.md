# Reading lines efficiently

**Level:** 201 · working knowledge

**One line:** `read_to_string` is one allocation for the whole file, `lines()` is one `String` per line, and `read_line` into a buffer you `clear()` is none per line — three answers, and the file's size decides which is right.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why a bare `File` read per line is slow, and what `BufReader` changes — the system call count, not the code
- The three shapes side by side, with what each costs and what each makes easy
- `lines()` strips the line ending (`\n`, and `\r\n`); `read_line` keeps it — the difference behind a surprising number of off-by-one bugs in counts
- The buffer-reuse loop in full, including the `clear()` whose absence is [an endless-looking bug](../../02_Errors/endless_iteration/README.md)
- When "read the whole file" is simply the right answer, which is more often than performance advice suggests

## The trap it exists for

Reuse is presented as the fast option, and it is — but the borrow it needs (`&mut String` handed to the reader each round) makes it the one shape where you can accidentally *keep* a line past its round. The performance win is real and small; the correctness cost is real and quiet.

## See also

- [Readers are fallible](../../02_Errors/readers_are_fallible/README.md) — every line in all three shapes arrives inside a `Result`
- [Endless iteration](../../02_Errors/endless_iteration/README.md) — the two loops this page's third shape can turn into
- [Borrowing](../../01_Foundations/borrowing/README.md) — why the reused buffer has to be handed over and handed back
