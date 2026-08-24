# rustlings

**Level:** 101 · for newcomers

**One line:** [rustlings](https://github.com/rust-lang/rustlings) is ~100 small files that **do not compile**, and your job is the smallest change that makes each one work — which makes the compiler the teacher, in doses small enough that a red error is a puzzle rather than a wall.

## Why it is foundational

Because reading about ownership and writing code that owns something are different skills, and only one of them is what you actually wanted.

The specific thing rustlings does that neither [The Book](../the_book/README.md) nor [Rust by Example](../rust_by_example/README.md) can: it puts you in front of a **compiler error you have to resolve**, roughly a hundred times, early. That is the loop you will be in for your whole Rust career, and the sooner `error[E0382]: borrow of moved value` stops reading as a rejection and starts reading as information, the sooner Rust becomes pleasant.

It is also under `rust-lang/` — official, maintained, 35k stars — and its own README recommends running it **in parallel with The Book**, which is the pairing this section is built around.

## Installing it

```sh
cargo install rustlings
rustlings init
cd rustlings
rustlings
```

Bare `rustlings` starts the interactive watcher: it opens the next unsolved exercise, recompiles on save, and moves you on when it passes.

**The `curl … install.sh | bash` line you may have seen is dead.** That installer was removed; the URL now 404s. `cargo install rustlings` is the current method — verified here against rustlings **6.5.0**, whose subcommands are:

| Command | Does |
|---|---|
| `rustlings` | the watcher — next pending exercise, recompiling as you save |
| `rustlings run <name>` | run one exercise |
| `rustlings hint` | a hint for the current one |
| `rustlings check-all` | mark everything done or pending |
| `rustlings reset <name>` | put an exercise back how it started |

`hint` is the one to know about. Use it — a stuck exercise teaches nothing, and the hints are written to nudge rather than solve.

## Which exercise goes with which chapter

The 24 exercise sets, in rustlings' own order, against the Book chapter that prepares you for each — and, in the third column, the page here that goes deeper when one of them will not click.

Chapter **names** are given as well as numbers on purpose: the numbering changed with the 2024 edition, so any older mapping you find (including ones that cite "§18.3" for patterns) points at the wrong chapter now.

| rustlings | The Book | Deeper, here |
|---|---|---|
| `00_intro` | ch1 Getting Started | [Running a scratch program](../../01_Foundations/rustc_without_cargo/README.md) |
| `01_variables` | ch3.1 Variables and Mutability | [Initial values](../../01_Foundations/initial_values/README.md) · [A name is not a place](../../01_Foundations/a_name_is_not_a_place/README.md) |
| `02_functions` | ch3.3 Functions | [A block is an expression](../../01_Foundations/a_block_is_an_expression/README.md) |
| `03_if` | ch3.5 Control Flow | [A block is an expression](../../01_Foundations/a_block_is_an_expression/README.md) |
| `04_primitive_types` | ch3.2 Data Types · ch4.3 The Slice Type | [Six kinds of zero](../../01_Foundations/six_kinds_of_zero/README.md) · [What a float stores](../../01_Foundations/what_a_float_stores/README.md) |
| `05_vecs` | ch8.1 Vectors | [`Option` is a one-item collection](../../01_Foundations/option_as_collection/README.md) |
| `06_move_semantics` | ch4.1 What Is Ownership? · ch4.2 References and Borrowing | [Ownership and moves](../../01_Foundations/ownership_and_moves/README.md) · [Borrowing](../../01_Foundations/borrowing/README.md) |
| `07_structs` | ch5.1 Defining Structs · ch5.3 Methods | [Representing a ballot](../../01_Foundations/representing_a_ballot/README.md) · [The newtype](../../01_Foundations/newtype_score/README.md) |
| `08_enums` | ch6 Enums and Pattern Matching · ch19 Patterns and Matching | [`if let`](../../01_Foundations/if_let/README.md) · [`while let`](../../01_Foundations/while_let/README.md) |
| `09_strings` | ch8.2 Strings | [Meet the byte](../../01_Foundations/meet_the_byte/README.md) |
| `10_modules` | ch7 Packages, Crates, and Modules | [Scope is about names](../../01_Foundations/scope_is_about_names/README.md) |
| `11_hashmaps` | ch8.3 Hash Maps | — |
| `12_options` | ch6.1 Defining an Enum | [**OPTION.md**](../../OPTION.md) — the whole map |
| `13_error_handling` | ch9 Error Handling | [`Option` vs `Result`](../../01_Foundations/option_vs_result/README.md) · [`unwrap` is a TODO](../../02_Errors/unwrap_is_a_todo/README.md) |
| `14_generics` | ch10.1 Generic Data Types | [`Result` aliases](../../01_Foundations/result_aliases/README.md) |
| `15_traits` | ch10.2 Traits | [`Debug` and `Display`](../../01_Foundations/debug_vs_display/README.md) |
| `16_lifetimes` | ch10.3 Lifetimes | [Borrowing](../../01_Foundations/borrowing/README.md) |
| `17_tests` | ch11 Writing Automated Tests | [cargo-nextest](../../05_Tooling/nextest/README.md) |
| `18_iterators` | ch13.1 Closures · ch13.2 Iterators | [`Option` is a one-item collection](../../01_Foundations/option_as_collection/README.md) |
| `19_smart_pointers` | ch15 Smart Pointers | [Nullable pointers](../../01_Foundations/nullable_pointers/README.md) |
| `20_threads` | ch16 Fearless Concurrency | [Mutex poisoning](../../09_Advanced/mutex_poisoning/README.md) |
| `21_macros` | ch20 Advanced Features | [The braces take a name](../../01_Foundations/braces_take_a_name/README.md) |
| `22_clippy` | *(no chapter — tooling)* | [Strict clippy lints](../../05_Tooling/strict_lints/README.md) |
| `23_conversions` | ch10.2 Traits (`From` / `Into`) | [`Option` vs `Result`](../../01_Foundations/option_vs_result/README.md) |

There is also a `quizzes` set, interleaved between the others, which combines several topics at once — those are the ones worth slowing down on, because a quiz you cannot do names the chapter you skimmed.

## Where it stops

By around exercise 60 you are fixing other people's nearly-correct programs, which is a genuine skill and **not the one you eventually need**. Nothing in rustlings asks you to design a type, choose between two valid approaches, or decide what your program should do when input is bad — because every exercise has exactly one intended answer.

So finish it, or do not; either is fine. But when it starts feeling like filling in blanks, that is the signal to move to something with an open answer: [100 Exercises to Learn Rust](../../10_Resources/exercises/README.md), [the katas here](../../KATAS.md), or a program of your own in [the practice tree](../../05_Tooling/practice_workspace/README.md).

## If you are coming from another language

- **Python** — closest to a test-driven kata site, except the failing thing is the *compiler* rather than an assertion, which is the adjustment. In Python a program that is wrong still runs; here it does not build, and learning to read that as help rather than obstruction is most of what rustlings is for.
- **ABAP** — the nearest feeling is a syntax check that refuses activation, which you already know how to work with. The difference in degree is large: Rust's checks cover ownership and lifetimes, so the compiler objects to things that would have been runtime bugs elsewhere.

## See also

- [The Book](../the_book/README.md) — the chapters this table maps onto
- [Exercises](../../10_Resources/exercises/README.md) — what to do after, and the other tracks
- [Katas](../../KATAS.md) — this library's own, one per lesson
