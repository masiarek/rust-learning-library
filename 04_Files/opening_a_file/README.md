# Opening a file

**Level:** 201 · working knowledge

**One line:** [`File::open` ↗](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) is read-only, `File::create` **truncates**, and everything else is [`OpenOptions` ↗](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) — the mode is a decision you make, and the wrong one is not a compile error.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three doors: `open` (read, must exist), `create` (write, truncates an existing file to nothing), and `OpenOptions` for append, read-write, and `create_new`
- What a file *handle* is — the operating system's descriptor with a Rust value wrapped around it, and what that value's `Drop` does, which is why there is no `close()` to forget
- `ErrorKind` on the way in: `NotFound`, `PermissionDenied`, `AlreadyExists` — the ones a program should behave differently about
- **`io::Error` does not carry the path.** The failure says *"No such file or directory"* and not which one, which is the single best argument for [adding context](../../02_Errors/anyhow_and_context/README.md)
- Buffering: a bare `File` is a system call per read, and `BufReader` / `BufWriter` are the fix; a `BufWriter` that is not flushed can lose the tail

## The trap it exists for

`File::create` on a path that already has your data in it does exactly what it says and nothing you wanted: opening for writing destroys the contents *at the moment of opening*, before a single byte of yours is written. The append case and the read-modify-write case both look like "create" to someone reading the name.

## See also

- [Missing is not empty](../missing_is_not_empty/README.md) — the `NotFound` that is not a failure
- [`anyhow` and context](../../02_Errors/anyhow_and_context/README.md) — putting the filename back into the error
- [Readers are fallible](../../02_Errors/readers_are_fallible/README.md) — everything that can still go wrong after this succeeded
