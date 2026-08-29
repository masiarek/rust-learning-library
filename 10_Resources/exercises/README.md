# Exercises

**Level:** reference

**One line:** Four practice tracks, and they are for four different stages — the common mistake is starting the hardest one first and concluding that Rust is not for you.

All links checked 2026-08-23.

| Track | Stage | Shape |
|---|---|---|
| [rustlings ↗](https://github.com/rust-lang/rustlings) | first week | ~100 tiny broken files; fix the compile error |
| [100 Exercises to Learn Rust ↗](https://github.com/mainmatter/100-exercises-to-learn-rust) | first month | a course that builds one program, test-driven |
| [Rust By Practice ↗](https://practice.course.rs/why-exercise.html) | any, as a drill | topic-by-topic problems with solutions |
| [this library's katas](../../KATAS.md) | alongside a lesson | one exercise per idea, in reading order |

## rustlings

Covered in full in [**Start here**](../../00_Start_Here/rustlings/README.md) — why it is foundational, the current install (the widely-copied `curl … install.sh` line is dead), the 6.5 subcommands, and the exercise-to-Book-chapter mapping for all 24 sets.

The short version: do it first, do it alongside The Book, and stop when it starts feeling like filling in blanks.

## 100 Exercises to Learn Rust — the best of them

[Mainmatter's course ↗](https://github.com/mainmatter/100-exercises-to-learn-rust) is a *book and a test suite in one repository*: a hundred exercises that accumulate into one real program, each with a chapter of prose and a failing test. There is a [PDF ↗](https://github.com/mainmatter/100-exercises-to-learn-rust) and a [solutions branch ↗](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions). The prose half is also readable online at [rust-exercises.com/100-exercises ↗](https://rust-exercises.com/100-exercises/) without cloning anything, which is the form to link when you want one chapter — [String slices ↗](https://rust-exercises.com/100-exercises/04_traits/06_str_slice) is the one this library sends people to most.

This is the one to do properly if you are going to do one properly. It is test-driven throughout, which means it also teaches the habit this library's [strict lints](../../05_Tooling/strict_lints/README.md) page depends on — that `unwrap` in a test is fine and everywhere else is a decision.

## Rust By Practice — the drill book

[practice.course.rs ↗](https://practice.course.rs/why-exercise.html) is organised by topic rather than as a sequence, which makes it the one to use when you know *what* you do not understand. Solutions included, so it is self-checking.

## Workshops with a syllabus

- **[Advanced testing ↗](https://rust-exercises.com/advanced-testing/)** ([repo ↗](https://github.com/mainmatter/rust-advanced-testing-workshop)) — the workshop for after `#[test]` stops being enough.
- **[proc-macro-workshop ↗](https://github.com/dtolnay/proc-macro-workshop)** — David Tolnay's procedural macro exercises, and the standard way people learn macros. Genuinely hard; save it.
- **[teach-rs ↗](https://github.com/trifectatechfoundation/teach-rs)** — a full modular course, if you are the one teaching.
- **[Kobzol on Rust exercises ↗](https://kobzol.github.io/teaching/2024/12/18/rust-exercises.html)** — a write-up of designing exercises for a university Rust course, and a useful survey of what exists.

## Building something instead

At some point the answer stops being an exercise track.

**[Command-Line Rust ↗](https://github.com/kyclark/command-line-rust)** (Ken Youens-Clark) rebuilds the classic Unix tools — `head`, `cat`, `wc`, `grep` — one per chapter, each with a full test suite. It is the natural next step after an exercise track because the programs are real, small, and have an obvious definition of correct.

And [the long way round to a STAR count](../../ROADMAP.md) is this library's own version of that: a sequence of lessons where each is the next thing Rust wants to teach, and the running example happens to be a voting method.

## See also

- [Katas](../../KATAS.md) — this library's own practice track, in order
- [Books](../books/README.md) — when you would rather read than type
- [A tree of practice projects](../../05_Tooling/practice_workspace/README.md) — where to put all of this on disk
