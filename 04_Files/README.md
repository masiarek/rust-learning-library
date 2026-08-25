# Files

The filesystem is the first thing outside your program that you are likely to talk to, and the first place Rust's insistence on types starts paying rent: a path is not a string, a missing file is not an empty one, and a handle that goes out of scope closes itself.

**These pages are stubs** — outlines waiting for a runnable example. See the [Errors](../02_Errors/README.md) section for what that means and how a page graduates. Examples here have a second obstacle to clear, and the finished pages will have to say how: [an example that touches the filesystem is not deterministic](../CONTRIBUTING.md), and every example in this library is checked against a recorded answer key.

| Lesson | Level | What it will teach |
|---|---|---|
| [Opening a file](opening_a_file/README.md) | 201 | `open`, `create` and `OpenOptions` — three doors and one decision the type system will not make for you |
| [`Path` and `PathBuf`](path_and_pathbuf/README.md) | 201 | The same split as `&str` and `String`, plus the `join` that throws your path away |
| [Reading lines efficiently](reading_lines_efficiently/README.md) | 201 | One allocation for the file, one per line, or none per line — and when each is the right answer |
| [Missing is not empty](missing_is_not_empty/README.md) | 201 | *"The file is not there"* and *"the file is there and empty"* are different answers, and only one is an error |
| [Temporary directories in tests](temp_dirs_in_tests/README.md) | 201 → 301 | A test that writes to a fixed path cannot run twice at once — and the fix deletes itself if you drop the handle |

The string half of this story — [`String` against `&str`](../14_Strings/string_vs_str/README.md), and [the slice that panics mid-character](../14_Strings/string_slices/README.md) — is in [Strings](../14_Strings/README.md), where it belongs.
