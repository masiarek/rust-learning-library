# `Path` and `PathBuf`

**Level:** 201 · working knowledge

**One line:** [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) is to [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html) what `&str` is to `String` — borrowed against owned — and a function that takes `impl AsRef<Path>` accepts all four spellings without the caller converting anything.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why a path is not a `String`: on Unix a filename is bytes, on Windows it is UTF-16, and neither is guaranteed to be valid UTF-8 — hence `OsStr` underneath
- The consequence you meet first: `println!("{}", path)` does not compile; `path.display()` is the escape hatch and it is lossy on purpose
- `join` and `push`, and the sharp edge — joining an **absolute** path discards everything to its left
- The parts: `file_name`, `file_stem`, `extension`, `parent`, and the fact that each returns an `Option`
- `AsRef<Path>` as the shape of a good API, and what generic argument actually does at the call site

## The trap it exists for

`base.join(user_input)` looks like string concatenation and is not. If the input is `/etc/passwd`, the result is `/etc/passwd` — the base is gone. That is a path-traversal bug written in one method call, in a language people reach for *because* it is careful.

## See also

- [Opening a file](../opening_a_file/README.md) — the first thing you do with a path
- [Command-line arguments](../../03_Command_Line/command_line_arguments/README.md) — where the non-UTF-8 filename problem starts
- [A score is not a number](../../01_Foundations/newtype_score/README.md) — the same idea one level down: a type that refuses to be confused with the thing it wraps
