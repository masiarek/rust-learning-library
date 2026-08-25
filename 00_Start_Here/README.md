# Start here

**Level:** 101 · for newcomers

**One line:** Three free resources cover the whole of beginning Rust between them — [The Book](the_book/README.md) to understand, [Rust by Example](rust_by_example/README.md) to look things up, [rustlings](rustlings/README.md) to type — and they are foundational because they fail in three *different* ways, so each one covers what the other two cannot.

## The plan

If you do nothing else on this page, do this:

1. **Read The Book, chapters 1–10.** Not 11–21 yet. Chapters 1–10 are the language; the rest is application.
2. **Run rustlings alongside it**, from the start. Its own README recommends exactly this pairing, and there is a [chapter-by-chapter mapping](rustlings/README.md#which-exercise-goes-with-which-chapter) so you always know which exercises you are ready for.
3. **Keep Rust by Example open in a tab** for the moments you want *"what does that syntax do"* rather than a chapter.
4. **Then read The Book again** — the second time at [the Brown University edition](the_book/README.md#read-it-twice), which is the same text with quizzes built around the misunderstandings readers actually have.
5. **Then write something.** That is what [the practice tree](../05_Tooling/practice_workspace/README.md) and [the katas](../KATAS.md) are for.

Steps 1–3 happen together, not in sequence. Step 4 is the one nobody does and the one that pays most.

## Why *these* three

They are not three ranked options where you pick the best. They are three different **kinds** of resource, and the reason to use all three is that each one's weakness is another one's whole purpose:

| | Gives you | Fails at | So pair it with |
|---|---|---|---|
| [**The Book**](the_book/README.md) | the *model* — why ownership exists, what the compiler is protecting | making you type; you can read it all and write nothing | rustlings |
| [**Rust by Example**](rust_by_example/README.md) | the *syntax*, runnable and editable in the page | the model — it shows what, rarely why | The Book |
| [**rustlings**](rustlings/README.md) | the *reps* — the compiler as teacher, in small doses | breadth and depth; it is drills, not understanding | The Book |

Reading only The Book produces someone who can explain the borrow checker and freezes at an empty `main`. Doing only rustlings produces someone who can fix other people's programs. Only Rust by Example produces someone who knows the syntax for a thing without knowing when to want it.

**And all three are free, official or semi-official, and maintained.** The Book and Rust by Example ship with your toolchain — `rustup doc --book` works offline — and rustlings lives under `rust-lang/`. That matters more than it sounds: none of them will quietly rot, and none of them is trying to sell you a course.

## Should this folder shadow the rest of the library?

**No — and you were right to be suspicious.** The redundancy is real, not hypothetical: [`10_Resources/books`](../10_Resources/books/README.md) already lists all three of these.

The failure mode is specific. Two copies of the same recommendation drift the moment one is edited, nothing in CI compares them, and a reader who finds the stale one has no way to know. This library has machinery that prevents exactly that for *code* — [`run_examples.py` ↗](https://github.com/masiarek/rust-learning-library/blob/master/tools/run_examples.py) fails the build when a page and its output disagree — and no machinery at all for prose. So prose duplication has to be prevented by structure instead.

The rule this section follows, which is the one the rest of the repo already follows:

> **One owner per fact.** Everything else links to it.

Concretely, the two sections have different jobs and do not overlap:

- **`00_Start_Here/`** — the **path**. One recommended order, for one situation (you are starting), with the three resources explained in depth. Opinionated and short.
- **[`10_Resources/`](../10_Resources/README.md)** — the **shelf**. Everything, sorted by situation, for someone browsing. Comprehensive and neutral.

So `10_Resources/books` now says one line about each of these three and points here, rather than repeating the argument. If you ever find the same claim made twice in this repo, that is a bug — the fix is to delete one and link it.

This is the same reason [`OPTION.md`](../OPTION.md), [`SHADOWING.md`](../SHADOWING.md) and [`TOOLCHAIN.md`](../TOOLCHAIN.md) are *maps* rather than folders: they sequence pages that live somewhere else, and own nothing.

## And where this library fits

Nothing here replaces The Book, and this section exists partly to say so. What this library adds is the thing a general-purpose book cannot: **every claim backed by a program that CI compiles, runs, and diffs against a recorded answer**, plus an *"If you are coming from another language"* section on every page — which, if you already write Python or ABAP, is usually the fastest route in.

Use it as the third voice. When The Book's explanation has not landed and rustlings has told you only that you are wrong, a page here has the same idea from a different angle with the output proved.

## See also

- [The Book](the_book/README.md) · [Rust by Example](rust_by_example/README.md) · [rustlings](rustlings/README.md)
- [Running a scratch program](../01_Foundations/rustc_without_cargo/README.md) — this library's first lesson, and the one to read before any of the above if `rustc` itself is new
- [Resources](../10_Resources/README.md) — the full shelf, for after
