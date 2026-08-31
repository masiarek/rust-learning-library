# Debugging Rust

**Level:** 101 → 201 · working knowledge

**One line:** Most of what other languages debug at runtime, Rust declines to build — so the debugging left over is a smaller and stranger set of skills, and this page is the map to where each one is taught.

Debugging here is front-loaded. A use-after-free, a null dereference, a data race, an iterator invalidated mid-loop: [nine of them](../31_C_and_Cpp/README.md) are compile errors rather than an afternoon with a debugger. That moves the first skill from *stepping through a running program* to *reading a message about one that never ran* — which is why a section on debugging Rust has to start with the compiler rather than with a breakpoint.

What is genuinely left over splits three ways, and this section is the door to all three: printing a value, surviving a panic, and reading the refusal.

**This is a map, not a place.** Every page below lives in the section that owns it, the same way [`STRINGS.md`](../STRINGS.md) indexes the strings arc. Nothing here is a second copy, and each link goes to the one page that is kept current.

## Printing a value

The everyday tool, and the one with the most surprises in it.

| Page | What it answers |
|---|---|
| [`Debug` and `Display`](../15_First_Programs/debug_vs_display/README.md) | Why `{}` refuses to print your struct and `{:?}` will, why the panic paths all reach for the generated trait rather than the sentence you wrote, and why a width on `{:?}` is accepted and then silently ignored |
| [What `dbg!` does](../15_First_Programs/what_dbg_does/README.md) | The macro that captures the *source text* of the expression, hands the value back so you can wrap anything in place, and writes to stderr — which is where half of the surprise lives |
| [`str::escape_debug`](../14_Strings/str_methods/str_escape_debug/README.md) | Seeing the character that is present and invisible: a tab, a trailing space, a zero-width space |

## When it panics

| Page | What it answers |
|---|---|
| [`unwrap` is a TODO you forgot to remove](../02_Errors/unwrap_is_a_todo/README.md) | Where the `unwrap` you are now debugging came from, and why nothing in the default toolchain ever asked about it |
| [`expect`: writing down the proof](../17_Option_and_Result/expect/README.md) | The sentence is a claim about why this could not fail — and failing to write it is the finding |
| [What a panic costs](../17_Option_and_Result/what_a_panic_costs/README.md) | Unwinding is tidy about memory and careless about work: every destructor runs, and everything unfinished simply never happens |
| [What a test asserts](../28_Testing/what_a_test_asserts/README.md) | A test is a function that panics when it is unhappy — the shortest route from "it broke once" to "it cannot break again" |

## When it will not build

| Page | What it answers |
|---|---|
| [Reading a compilation failure](../20_Compilers/reading_a_compilation_failure/README.md) | Which of four programs produced your message — parser, type checker, borrow checker, linker — because the fix is different in each |
| ["No method named …"](../12_Traits/no_method_named/README.md) | One error code over four unrelated bugs, and the `help:` line that separates them — including the case where the trait to implement is not the trait the method came from |
| [C and C++: the bugs Rust is a reply to](../31_C_and_Cpp/README.md) | Nine runtime bugs, each watched happening in C and then refused by `rustc`. The best argument for reading errors instead of running debuggers |

## Once somebody else depends on it

| Page | What it answers |
|---|---|
| [A log line a machine can read](../21_Observability/structured_logging/README.md) | Printing stops scaling the moment the question is about one request out of a million |

## What is not here yet

Three gaps, named rather than quietly left out:

- **A real debugger.** `rust-gdb` and `rust-lldb` ship with the toolchain — they are in `$(rustc --print sysroot)/bin` alongside `rustc` and `cargo`, so they are already installed — and no page in this library uses one. They are wrappers that load the pretty-printers, without which a `String` shows as its raw parts.
- **Backtraces.** A panic names its own file and line, and then tells you to re-run with `RUST_BACKTRACE=1` to display a backtrace. Doing so is the difference between the line that blew up and the call chain that reached it. No page covers reading one.
- **`tracing` and friends.** [Observability](../21_Observability/README.md) is the section for this and says honestly why its pages are the hardest here to finish: every checked example in this library compiles with `rustc` alone, and that is a crate story.

A page graduates out of that list the way every page here arrives — with a program CI compiles, runs, and diffs against a recorded answer key. [CONTRIBUTING.md](../CONTRIBUTING.md) says how.

## Po polsku

Ta strona jest mapą, nie lekcją, a jej teza jest dla uczącego się wygodna: **większość tego, co w innych językach debuguje się w czasie działania, Rust po prostu odmawia zbudować**. Wartość pusta, wyścig danych, użycie po zwolnieniu, zapomniany wariant w `match` — gdzie indziej bywają sesją przy debuggerze, tutaj są komunikatem kompilatora.

Skutek jest taki, że debugowanie w Ruscie to mniejszy i dziwniejszy zestaw umiejętności niż gdzie indziej, i dzieli się mniej więcej tak: jak **wypisać wartość** (`dbg!`, `{:?}`, `{:#?}`), co robić, **gdy panikuje** (ślad stosu, `RUST_BACKTRACE=1`), co robić, **gdy się nie buduje** — a to przypadek najczęstszy i najmniej podobny do debugowania w klasycznym sensie — i wreszcie co robić, **gdy ktoś już od twojego kodu zależy**.

Jedna uwaga o oczekiwaniach. Krokowanie debuggerem jest w Ruscie znacznie rzadsze niż w C++ czy w Javie, i nie z braku narzędzi — `rust-gdb` i `rust-lldb` są w zestawie — tylko dlatego, że pytania, na które debugger odpowiada, zwykle padają wcześniej, przy kompilacji. Kto szuka polskich materiałów o debugowaniu Rusta i znajduje ich mało, niczego nie przeoczył: po angielsku jest ich niewiele więcej, z tego samego powodu.

**Szukaj po polsku:** debugowanie Rusta · `dbg!` i `RUST_BACKTRACE` · czytanie komunikatów kompilatora · `rust debugging gdb lldb`
