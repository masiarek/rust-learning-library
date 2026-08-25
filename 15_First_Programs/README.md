# First programs

**One line:** Before any of the language itself, four things have to work — getting a file to run at all, reading what the compiler says back about it, the two pieces of punctuation every later page uses without explaining, and getting the program to tell you something.

Nothing here is a Rust *feature*. It is the loop you will be inside for every other page in the library: write it, run it, read the message, print something to find out what happened. The pages are short and the traps are the interesting half — a `///` that is not a comment, a `_name` and a `_` that mean different things, an extra semicolon that is `E0308`, and `{n + 1}` in a format string, which does not do what a Python f-string does.

| Lesson | Level | What it teaches |
|---|---|---|
| [Running a scratch program](rustc_without_cargo/README.md) | 101 | `rustc` alone, `cargo new`, and `src/bin/` — rustc's edition-2015 default, and what Cargo was quietly doing for you |
| [Comments that compile](comments_that_compile/README.md) | 101 → 201 | Four forms, and two of them are not comments — `///` is `#[doc = "..."]`, a misplaced one is a *warning* rather than an error, and the examples inside are compiled and run as tests |
| [What a warning is asking](what_a_warning_is_asking/README.md) | 101 → 201 | A warning is the compiler asking whether you meant it — `rustc` raises it itself, `_name` and `_` are different answers, and only one of them leaves your value alive to the end of the scope |
| [A block is an expression](a_block_is_an_expression/README.md) | 101 → 201 | `{ }` does two jobs, and the second is the surprise: it *has a value* — its last line without a semicolon. Why a function body needs no `return`, and why one extra character is `E0308` |
| [The braces take a name](braces_take_a_name/README.md) | 101 → 201 | `{n}` in a format string is an **identifier**, not a Python f-string — `{n + 1}`, `{v.len()}` and `{self.voter}` are three different compile errors, and only one of them is diagnosed as what it is |
| [Debug and Display](debug_vs_display/README.md) | 101 → 201 | Two printing traits, two audiences: `{:?}` can be derived because it is structural and `{}` never will be — plus the compiler note that spreads the habit, and the four default paths that print your error's `Debug` form instead of the sentence you wrote |
| [What `dbg!` does](what_dbg_does/README.md) | 101 → 201 | Why it is not a shorter `println!("{:?}")` — it returns your value, captures the expression source, writes to stderr, always pretty-prints, moves a non-`Copy` argument, and survives `--release` |
| [Randomness](randomness/README.md) | 101 → 201 | `std` has no generator, so the Rust Book's guessing game no longer compiles — `thread_rng`/`gen_range` are gone, the trait to import is `RngExt`, and `% n` is biased for a reason no better generator can fix |

## Where it goes next

The compiler's *errors* get their own treatment once you have a type of your own to produce them: [when a struct refuses](../16_Structs/when_a_struct_refuses/README.md) reads eight of them the same way this section reads a warning. And the section that follows is [Structs](../16_Structs/README.md) — the first type you write yourself.
