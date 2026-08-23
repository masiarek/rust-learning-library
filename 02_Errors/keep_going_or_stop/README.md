# Keep going, or stop

**Level:** 201 · working knowledge

**One line:** One bad row in a thousand is a design decision, not an error-handling one — stop at the first, or process the rest and report at the end — and in Rust the *return type* is where you say which you chose.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `collect::<Result<Vec<T>, E>>()` — an iterator of `Result`s collected into a `Result` of a `Vec`, short-circuiting at the first `Err`; it reads like magic and is an ordinary `FromIterator` impl
- The other direction: keeping both halves, so the caller gets the good rows *and* a list of what was rejected
- An error that names the row it came from — line numbers are the difference between a usable message and a shrug
- `filter_map(Result::ok)` and friends: fine when "skip the junk" is genuinely the specification, silent data loss when it is not
- What exit status a partly-successful run should report

## The trap it exists for

The two policies look identical on a file with no errors, which is every file during development. Choosing by accident means the choice surfaces on the day it matters — a 40,000-line import that stopped on line 3 and told nobody, or one that skipped 200 malformed rows and reported success.

## See also

- [Readers are fallible](../readers_are_fallible/README.md) — where the iterator of `Result`s comes from
- [Returning `None` on error](../../01_Foundations/none_on_error/README.md) — the same loss one level down: four distinct causes arriving as one `None`
- [The long way round](../../ROADMAP.md) — rung 7 is this idea with ballots in it
