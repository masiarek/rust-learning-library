# Command-line arguments

**Level:** 101 · for newcomers

**One line:** [`std::env::args()` ↗](https://doc.rust-lang.org/std/env/fn.args.html) is an iterator over the words the shell already split for you — and its **first** item is the path your program was invoked as, not your first argument.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Argument zero: what it contains, why it is there, and `.skip(1)` as the idiom
- `args()` panics on an argument that is not valid Unicode; [`args_os()` ↗](https://doc.rust-lang.org/std/env/fn.args_os.html) does not — and an `OsString` is not a `String`, which is the whole reason [paths are their own type](../../04_Files/path_and_pathbuf/README.md)
- What the shell did before your program woke up: globbing, quote removal, variable expansion. `prog *.txt` never sees a `*`
- Reaching for `args[1]` indexes a `Vec` and panics when it is missing; `.nth(1)` gives you an `Option`, which is where `ok_or` turns absence into a real error
- Collecting once into a `Vec<String>` versus consuming the iterator in place

## The trap it exists for

The off-by-one is the famous one and it is caught in a minute. The one that survives to production is `args[1]` on a program run with no arguments: the message a user gets is `index out of bounds`, from a program whose *entire job* was to notice a missing filename. Absence is not a bug here — it is the most ordinary input there is.

## If you are coming from another language

- **Python** — `sys.argv`, including the same "element 0 is the program" convention, so the mental model transfers whole. What changes: `argv[1]` on a missing argument raises an `IndexError` you can catch, while Rust's equivalent panic is not something you are meant to catch — the compiler is pushing you toward `.nth(1)` and an `Option` instead.
- **ABAP** — the closest counterpart is a selection screen with `PARAMETERS`, where "required" is a property you declare and the runtime enforces before your code runs. Here nothing enforces it for you; the type does the enforcing, once you ask for an `Option` rather than an index.

## See also

- [Flags by hand](../flags_by_hand/README.md) — what to do with the strings once you have them
- [`Option` vs `Result`](../../01_Foundations/option_vs_result/README.md) — the `ok_or` hop from "not supplied" to "here is why I stopped"
