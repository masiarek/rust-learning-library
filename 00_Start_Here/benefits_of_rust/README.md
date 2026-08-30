# Benefits of Rust

**Level:** 101 · for newcomers

**One line:** The twenty selling points on Google's list are three different kinds of claim — a build that fails, a run-time behaviour that is written down, and a convenience — and telling them apart is the difference between trusting the list and taking it on faith.

[Comprehensive Rust ↗](https://google.github.io/comprehensive-rust/hello-world/benefits.html) opens with a list of what Rust buys you: compile-time memory safety, no undefined run-time behaviour, modern language features. The list is accurate and this page does not argue with it. What it adds is the **evidence** behind each bullet, because "no use-after-free" and "built-in dependency manager" are true in two completely different senses — one is a theorem the compiler enforces, the other is that somebody wrote a good tool — and a reader who cannot tell them apart ends up believing all twenty equally or none of them.

Every row links twice. **The claim** goes to [C and C++](../../31_C_and_Cpp/README.md) — the bug it is a reply to, compiled and run, so you can see what it costs when nobody prevents it. **See it in** goes to the page here where you can watch Rust's side happen.

The two groups below that link that way are the ones where there is a bug to show. The third group is features, and the claims there link nowhere but the lesson.

## Compile-time memory safety

This whole group is prevented by the build failing. There is nothing to run and no output to record: the evidence is an error message.

| The claim | What it actually says | See it in |
|---|---|---|
| [No uninitialized variables](../../31_C_and_Cpp/uninitialized_reads/README.md) | `let b: Ballot;` is legal — *reading* `b` before some path assigns it is `E0381`, and the compiler checks every path | [A type is not a constructor](../../16_Structs/a_type_is_not_a_constructor/README.md) |
| [No double-frees](../../31_C_and_Cpp/double_free/README.md) | One value, one owner, so exactly one binding will free it; a move transfers that responsibility rather than duplicating it | [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) |
| [No use-after-free](../../31_C_and_Cpp/use_after_free/README.md) | A reference may not outlive what it points at, and `'a` is how you name which "what" when the compiler cannot work it out alone | [Borrowing](../../18_Ownership/borrowing/README.md) · [Lifetime annotations](../../18_Ownership/lifetime_annotations/README.md) |
| [No `NULL` pointers](../../31_C_and_Cpp/null_dereference/README.md) | Absence is a *different type* — `Option<Box<T>>` — which costs the same eight bytes and cannot be read without saying what happens when it is empty | [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) · [`Some` and `None`](../../17_Option_and_Result/some_and_none/README.md) |
| [No forgotten locked mutexes](../../31_C_and_Cpp/forgotten_unlock/README.md) | There is no `unlock()` to forget: the guard's destructor releases the lock. Binding it to `_`, which drops it immediately, is a deny-by-default lint | [Scope is about names](../../18_Ownership/scope_is_about_names/README.md) · [Mutex poisoning](../../09_Advanced/mutex_poisoning/README.md) |
| [No data races between threads](../../31_C_and_Cpp/data_races/README.md) | `Send` and `Sync` decide what may cross a thread boundary — `Rc` may not and `Arc` may — so the mistake is a build error rather than a wrong answer once a fortnight | [Sharing across threads](../../18_Ownership/sharing_across_threads/README.md) |
| [No iterator invalidation](../../31_C_and_Cpp/iterator_invalidation/README.md) | You cannot hold a reference into a collection and mutate the collection in the same breath. `E0502` — the same borrow rule as every other row, aimed at a loop | [`while let`](../../17_Option_and_Result/while_let/README.md) · [String slices](../../14_Strings/string_slices/README.md) |

## No undefined run-time behaviour

These two are not prevented. They happen, and what happens is written down — which is the actual contrast with C, where the standard declines to say and the answer can differ per run. See [undefined behaviour](../../GLOSSARY.md).

| The claim | What it actually says | See it in |
|---|---|---|
| [Array access is bounds checked](../../31_C_and_Cpp/buffer_overruns/README.md) | `v[9]` on a three-element vector aborts the program; it does not hand you whatever was next in memory. `v.get(9)` is the same question asked without the abort, and returns `None` | [Partial functions](../../17_Option_and_Result/partial_functions/README.md) · [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) |
| [Integer overflow is defined (panic or wrap-around)](../../31_C_and_Cpp/signed_overflow/README.md) | Both — selected by the build profile, which is the trap. Every integer also carries `wrapping_`, `checked_`, `saturating_` and `overflowing_`, so you can put the decision in the method name | [Meet the byte](../../19_Numbers/meet_the_byte/README.md) |

## Modern language features

Nothing here is a safety guarantee, and this is the half of the list that decides whether you enjoy the language.

| The claim | What it actually says | See it in |
|---|---|---|
| Enums and pattern matching | A value is exactly one of a closed set, and a `match` that forgets a variant is a build error rather than a branch nobody wrote | [What an enum is](../../13_Enums/what_an_enum_is/README.md) · [Variants that carry data](../../13_Enums/variants_that_carry_data/README.md) |
| Generics | `<T>` is a type the caller fills in: written once, checked once, then stamped out per type your program actually uses | [What a generic is](../../22_Generics/what_a_generic_is/README.md) |
| No overhead FFI | `extern "C"` is an ordinary C call. No marshalling layer, no JNI, no `ctypes` — and no runtime on your side to start up first | no page yet — nearest are [What a union is](../../09_Advanced/what_a_union_is/README.md) and [The linker](../../20_Compilers/the_linker/README.md) |
| Zero-cost abstractions | Ask for optimization and a loop summing an array comes out as the answer, with no loop and no array left in the machine code | [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) · [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) · [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) |
| Great compiler errors | The message usually contains the fix, and a warning is a question about your intent rather than a complaint about your style | [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) |
| Built-in dependency manager | `cargo add rayon` and it is in the build — though what it writes into the manifest is a version *range*, not the version you got | [Cargo dependencies](../../05_Tooling/cargo_dependencies/README.md) |
| Built-in support for testing | `#[test]` and `cargo test` ship with the toolchain, so there is no framework to choose and no argument to have about it | [nextest](../../05_Tooling/nextest/README.md) · [Commit on green](../../05_Tooling/commit_on_green/README.md) |
| Excellent Language Server Protocol support | Every editor worth using is a window onto the same program, `rust-analyzer`, so the intelligence is the same and only the window differs | [Editors](../../05_Tooling/editors/README.md) |

## The fine print

Six of the twenty are narrower than the one-line version, and each is narrower in a way that eventually costs somebody a day.

- **All of it means *safe* Rust.** `unsafe` reopens every door in the first table — that is what the keyword is for, and `MaybeUninit`, raw pointers and `union` are the named exceptions to its rows. The guarantee is not that the doors are locked; it is that they are labelled, so `grep unsafe` finds every place the compiler stopped checking.
- **"No `NULL`" does not mean absence went away.** It means absence became a value with a type, which you must open before you can use what is inside. `.unwrap()` is still a way to end the program on an empty one — the slide is about the pointer, not about the problem.
- **"No data races" is much narrower than "no concurrency bugs."** A data race is two threads touching one location, at least one of them writing, with no synchronisation; that is what `Send` and `Sync` rule out. Deadlocks, lost updates across two separate locks, and ordering bugs in your own logic remain entirely available. Take two locks at once on one thread and it deadlocks, with nothing objecting at compile time.
- **"Integer overflow is defined" defines it as two different things.** A debug build panics and a release build wraps. So an overflow bug can pass every test you run and still wrap in production, which is the one consequence to internalise from that row.
- **"Zero-cost abstractions" is a claim about most of them.** Generics and iterator chains do compile away. `dyn Trait` costs a pointer indirection and blocks inlining — deliberately, because being able to decide at run time is the feature you are paying for.
- **"No overhead FFI" is about the call, not the data.** Crossing the boundary is free; getting your data into a shape C recognises often is not. `&str` has to become a NUL-terminated `CString`, which allocates and copies, because Rust strings carry a length instead of a terminator.

## If you are coming from another language

Google's speaker notes suggest asking the room which languages they already write, and pitching accordingly. Here are the four pitches, with what actually transfers and what changes.

- **C or C++** — this is the audience the list was written for, and [the whole section written for you](../../31_C_and_Cpp/README.md) runs the nine bugs the first two tables are about. Most of it already has a name in your head: RAII is how the mutex row works, `unique_ptr` is roughly `Box`, `shared_ptr` is `Arc`, and `const`-correctness is the ancestor of `&` versus `&mut`. Three things change. Moves are the *default* rather than an opt-in `std::move`, and a moved-from value is unusable rather than "valid but unspecified" — so there is no state to reason about. The borrow rule is checked rather than documented, which is the one that costs you a fortnight and then pays back forever. And the question *"is this undefined behaviour?"* stops being askable in safe code, which quietly removes a whole genre of code review.
- **Python** — you already never free anything, so ownership is the genuinely new idea rather than a stricter version of an old one. What transfers immediately: `Option` is `None` with the check made compulsory, and `Result` is an exception that travels as a return value, so `try`/`except` becomes something the type system counts. What changes is the width of a number — a Python `int` is unbounded, a Rust one is not — and that `len()` counts bytes here and characters there. And the row that will surprise you most is iterator invalidation: `for x in lst: lst.remove(x)` is a bug Python lets you write, and it silently skips elements — `[1, 2, 3, 4]` comes back as `[2, 4]` — while the same shape in Rust is `E0502` before the program runs. Threads too — the GIL made data races rare rather than impossible, and the free-threaded builds now arriving remove even that accident.
- **ABAP** — three rows map onto things you already do by hand. `TYPE REF TO` can be initial and dereferencing it dumps with `CX_SY_REF_IS_INITIAL` at run time; `Option<Box<T>>` is that same check moved to compile time, so `IS BOUND` becomes the only way to reach the value at all. Arithmetic overflow raises `CX_SY_ARITHMETIC_OVERFLOW` when it happens, where Rust wants the range argued before the run and puts your answer in the method name. And deleting from an internal table inside a `LOOP AT` over that same table is iterator invalidation exactly — ABAP permits it and leaves the consequences to you, where Rust refuses to build it. What does *not* transfer is the runtime: there is no work process, no roll area and no short dump to open afterwards, so the failures you are used to reading in ST22 have to be handled where they occur or the program ends.
- **Java, Go or JavaScript** — you get the memory safety you already have, and keep the high-level feel, plus predictable performance with no collector pause and access to the hardware if you ever need it. The bill is the collector's convenience: a cyclic structure your language builds without being asked needs `Rc<RefCell<T>>` and `Weak` here, and the compiler will argue with you about it. For Go specifically, the channels and lightweight tasks are all present, but the race detector is a run-time tool that finds races on the paths your tests happened to exercise, whereas `Send` and `Sync` are checked on all of them.

## See also

- [Start here](../README.md) — the plan, and the three free resources it is built around
- [Comprehensive Rust in the shelf](../../10_Resources/books/README.md) — what the course is good at, and the one cost of its slide format
- [C and C++](../../31_C_and_Cpp/README.md) — the nine bugs the first two tables prevent, each one compiled and run
- [Undefined behaviour](../../GLOSSARY.md) — the term the second table is really about
