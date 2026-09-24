# *Rust: The Practical Guide*, run

**Level:** reference · a companion to the book

**One line:** Nouman Azam's *Rust: The Practical Guide* (Rheinwerk Computing, 2025) is a course in book form: seventeen chapters, each closed by practice exercises with solutions. These pages run its listings and its solutions on rustc 1.98.0, check the sentences around them, keep the exercises as katas, and point each idea to the lesson here that teaches it.

The book is [Rheinwerk Computing's Rust title ↗](https://www.rheinwerk-verlag.de/rust-the-practical-guide/) (first edition 2025, 627 pages, ISBN 978-1-4932-2687-0; Amazon lists it as *Rust Programming: A Practical Guide to Fast, Efficient, and Safe Code with Ownership, Concurrency, and Web Programming*). Its author teaches online Rust courses, and the book reads like one: short listings, a sentence of explanation each, then exercises. The [full table of contents ↗](https://www.rheinwerk-verlag.de/rust_6056/toc) is public.

How these pages treat it: the book's listings are a few lines each and are reproduced only where a claim is about that exact code; every exercise is restated in this library's words, its code kept because the code is what gets fixed; the book's solutions are compiled and judged, and where a solution warns, draws a clippy lint or hides a better answer, the page says so. Every kata is indexed in [KATAS.md](../../KATAS.md) under the lesson it exercises.

## The chapters, and what is checked here

| Chapter | Read here | Pages |
|---|---|---|
| 1 Introduction | — | not read here yet |
| 2 Variables, Data Types, and Functions | §2.3 Functions · §2.4 Code Blocks · §2.5–2.6 exercises | [What the book says about functions and code blocks, run](../../25_Control_Flow/functions_claims_checked/README.md) · [Chapter 2 exercises, run](ch2_katas/README.md) |
| 3 Conditionals and Control Flow | §3.4–3.5 exercises | [Chapter 3 exercises, run](ch3_katas/README.md) |
| 4 Ownership | §4.2 Ownership in Functions · §4.4 Borrowing in Functions · §4.6 Mutable and Immutable Binding of References · §4.7–4.8 exercises | [What the book says about ownership in functions, run](../../18_Ownership/ownership_in_functions_claims_checked/README.md) · [Mutable binding, mutable reference](../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) · [Chapter 4 exercises, run](ch4_katas/README.md) |
| 5 Custom and Library-Provided Useful Types | §5.5 HashMaps · §5.6 HashSets · §5.7–5.8 exercises | [A first `HashMap`](../../26_Collections/a_first_hashmap/README.md) · [A first `HashSet`](../../26_Collections/a_first_hashset/README.md) · [Chapter 5 exercises, run](ch5_katas/README.md) |
| 6 Organizing Your Code | §6.1 Code Organization · §6.2 Module Basics · §6.3 Visualizing and Organizing Modules · §6.4 Re-Exporting and Privacy · §6.7–6.8 exercises | [What the book says about packages, crates and modules, run](../../27_Modules/modules_claims_checked/README.md) · [Packages and crates](../../27_Modules/packages_and_crates/README.md) · [Chapter 6 exercises, run](ch6_katas/README.md) |
| 7 Testing Code | — | not read here yet; the section here is [Testing](../../28_Testing/README.md) |
| 8 Flexibility and Abstraction with Generics and Traits | — | [Generics](../../22_Generics/README.md) · [Traits](../../12_Traits/README.md) |
| 9 Functional Programming Aspects | — | [Closures](../../23_Closures/README.md) · [Iterators](../../24_Iterators/README.md) |
| 10 Memory Management Features | — | [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) · [Smart pointers](../../41_Smart_Pointers/README.md) |
| 11 Implementing Typical Data Structures | — | not read here yet |
| 12 Useful Patterns for Handling Structs | — | [Structs](../../16_Structs/README.md) |
| 13 Understanding Size in Rust | — | [Pointers](../../36_Pointers/README.md) |
| 14 Concurrency | — | [Async](../../35_Async/README.md) and the [Concurrency library ↗](https://masiarek.github.io/concurrency-learning-library/) |
| 15 Macros | — | [Declarative macros](../../38_Declarative_Macros/README.md) |
| 16 Web Programming | — | not read here yet |
| 17 Text Processing, File Handling, and Directory Management | — | [Files](../../04_Files/README.md) · [Strings](../../14_Strings/README.md) |

## What the pages found

Every listing in the sections read so far compiles where the book says it compiles and fails where it says it fails, with one exception (Listing 6.41). The disagreements are in the prose around the code. Each row links the page that runs it.

| Where | The book, paraphrased | rustc 1.98.0 | Page |
|---|---|---|---|
| §2.3, Listing 2.7 | `return 45;` on the first line returns early | it does, with three warnings the book does not mention: *unreachable statement*, and `num1` and `num2` reported unused because their only use is unreachable | [§2.3–2.4](../../25_Control_Flow/functions_claims_checked/README.md) |
| §2.3, sidebar | expressions are code lines with a value; `println!` is a statement | an expression is not a line, and `println!` expands to a block of type `()`: a `-> ()` function may end in one with no semicolon | [§2.3–2.4](../../25_Control_Flow/functions_claims_checked/README.md) |
| §2.3 | one and only one returning expression can exist | an `if`/`else` or `match` tail has an arm per case, and any of them is the value; what the grammar refuses is two tails in a row | [§2.3–2.4](../../25_Control_Flow/functions_claims_checked/README.md) |
| §2.3 | the argument's type and the parameter's type must match | `&String` coerces to `&str`, and an unsuffixed literal takes the parameter's type | [§2.3–2.4](../../25_Control_Flow/functions_claims_checked/README.md) |
| §2.4, Listing 2.9 | a semicolon "can be added" after a block assigned with `let` | it is required: *expected `;`, found `println`* | [§2.3–2.4](../../25_Control_Flow/functions_claims_checked/README.md) |
| §2.4, sidebar | a function sees only its parameters and locals | it also sees every `const`, `static` and item around it; a nested `fn` reading a local is `E0434` | [Items inside a function](../../27_Modules/items_inside_a_function/README.md) |
| §2.6, exercise 2 | add `#![allow(unused)]` "if you do want to see the warning" | the attribute silences it; `let x2;` avoids the warning with no `mut` at all | [Chapter 2 exercises](ch2_katas/README.md) |
| §2.6, exercise 8 | the solution prints the year 2020 | the exercise printed 2010 | [Chapter 2 exercises](ch2_katas/README.md) |
| §3.5, exercise 3 | `as i32` is used "to round the values" | it truncates: 16.575 becomes 16 where `round()` gives 17; `hours` cancels out of the per-minute rate | [Chapter 3 exercises](ch3_katas/README.md) |
| §3.5, exercise 4 | the byte-by-byte check decides whether a string is a palindrome | it answers `false` for the book's own example "Able was I ere I saw Elba" and for "été" | [Chapter 3 exercises](ch3_katas/README.md) |
| §3.5, exercise 5 | three nested loops and a flag find the triple | they look at 80,778,750 candidates; computing `c` and a labelled `break` looks at 69,676 | [Chapter 3 exercises](ch3_katas/README.md) |
| §3.5, exercise 6 | the printed solution | has lost its spaces (`fncan_see_movie`) and does not parse; `main` says John is 18 and passes 17 | [Chapter 3 exercises](ch3_katas/README.md) |
| §4.2, Listing 4.3 | the error is "borrowed of a moved value" | `E0382` *borrow of moved value* | [§4.2 and §4.4](../../18_Ownership/ownership_in_functions_claims_checked/README.md) |
| §4.2, sidebar | stack-only types are copied, heap types are moved | the `Copy` trait decides: a plain 4-byte struct moves, `[u8; 3]` and every `&T` copy | [§4.2 and §4.4](../../18_Ownership/ownership_in_functions_claims_checked/README.md) |
| §4.4, Listing 4.20 | it compiles because mutable and immutable references "do not coexist within the same scope" | they share one block in the listing; it compiles because the first borrow ended at its last use | [§4.2 and §4.4](../../18_Ownership/ownership_in_functions_claims_checked/README.md) |
| §4.4, Listing 4.22 | moving a `Vec` into a function "moves the entire vector's data" | the heap pointer is identical before, inside and after, and nothing is allocated: a move copies the three-word header | [§4.2 and §4.4](../../18_Ownership/ownership_in_functions_claims_checked/README.md) |
| §4.4, Listing 4.25 | `-> &Vec<i32>` fails for violating borrowing rule 2 | `E0106` is a signature error raised before the borrow checker runs; with `'static` written in, `E0515` is the dangling-reference error | [§4.2 and §4.4](../../18_Ownership/ownership_in_functions_claims_checked/README.md) |
| §4.6, Table 4.1 | six types of references | two independent choices, `let`/`let mut` and `&`/`&mut`, applied again at each nesting level; `& &mut T` and `&mut &mut T` are missing | [Mutable binding, mutable reference](../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) |
| §4.6, Listing 4.33 | mutating through an immutable binding "is not possible with other types of references" | `let r = &mut v; *r = vec![9];` does it through a plain `&mut`; the listing's `&mut &vec_2` is a deref coercion | [Mutable binding, mutable reference](../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) |
| §4.8, exercise 6 | `ref2 = ref1` moves the reference | it reborrows: `ref1` is usable again after `ref2`'s last use | [Chapter 4 exercises](ch4_katas/README.md) |
| §5.5 | `contains_key` returns an `Option` | it returns a `bool`; `get` is the method that returns `Option<&V>` | [A first `HashMap`](../../26_Collections/a_first_hashmap/README.md) |
| §5.5 | inserting `"Programming"` again shows 15, once | it does, and `insert` hands the old value back as `Some(5)`, which the book leaves out | [A first `HashMap`](../../26_Collections/a_first_hashmap/README.md) |
| §5.5 | `HashMap<&str, u8>` holds the counts | a `u8` count panics at 256 in a debug build; `u32` or `usize` is the usual count type | [A first `HashMap`](../../26_Collections/a_first_hashmap/README.md) |
| §5.5 | `println!("{:?}", word_counts)` shows the map | in an order that differs from run to run; sort first | [A first `HashMap`](../../26_Collections/a_first_hashmap/README.md) |
| §5.6 | `take` removes an item and returns it | it does, as `Option<T>`; the `take` that linked-list code reaches for is `Option::take`, a different method with the same name | [A first `HashSet`](../../26_Collections/a_first_hashset/README.md) |
| §5.8, exercise 1 | `Float(f32)` | the task asked for `f64`; the `f32` payload loses digits past the seventh | [Chapter 5 exercises](ch5_katas/README.md) |
| §5.8, exercise 3 | `Some =>` needs its argument | the given arm is `E0530` *match bindings cannot shadow tuple variants*, not the `E0532` a reader might expect | [Chapter 5 exercises](ch5_katas/README.md) |
| §5.8, exercise 7 | `contains_key` then `insert` | two lookups where `entry` does one; clippy names it `map_entry` | [Chapter 5 exercises](ch5_katas/README.md) |
| §6.1, Listing 6.1 | `cargo new` writes `edition = "2021"` | 1.98.0 writes `edition = "2024"` | [Packages and crates](../../27_Modules/packages_and_crates/README.md) |
| §6.1, Listing 6.2 | extra binaries go in a `bin/` folder beside `src/` | Cargo discovers only `src/bin/`; a root `bin/` is not a target, and `cargo run` runs `main.rs` without complaint | [Packages and crates](../../27_Modules/packages_and_crates/README.md) |
| §6.1, Listing 6.3 | `cargo run` cannot choose between two binaries; use `--bin` | true; `default-run` in `Cargo.toml` is the third way | [Packages and crates](../../27_Modules/packages_and_crates/README.md) |
| §6.2, Listing 6.7 | an item in another module needs its absolute path | `super::product::Product` is a relative path and works | [Chapter 6](../../27_Modules/modules_claims_checked/README.md) |
| §6.2.4 | parent modules cannot access the items within their child modules | only the private ones; a child's `pub` items are reachable | [Chapter 6](../../27_Modules/modules_claims_checked/README.md) |
| §6.3, Listing 6.28 | the error is "unresolved module" | that is rust-analyzer's wording; rustc says `E0583` *file not found for module* and names both accepted files | [Chapter 6](../../27_Modules/modules_claims_checked/README.md) |
| §6.3.1 | `pub(self)` means the current module only | the module and its descendants; `pub(super)` and `pub(in path)` exist too | [Chapter 6](../../27_Modules/modules_claims_checked/README.md) |
| §6.4, Listing 6.34 | the re-export "might throw an error in 1.80 or later"; the fix is `crate::` | it fails on 1.79.0 too, with `E0603`; the fix is the path through the re-export, `pub use product::Category`, with or without `crate::` | [Chapter 6](../../27_Modules/modules_claims_checked/README.md) |
| §6.4, Listing 6.40–6.41 | `Customer::new(...)` builds a customer in `main` | `new` is not `pub` in Listing 6.40, so Listing 6.41 is `E0624` *associated function `new` is private*; it also names the crate `my_package_book` | [Chapter 6](../../27_Modules/modules_claims_checked/README.md) |
| §6.8, exercise 4 | make the three fields `pub` | compiles with three warnings (`non_snake_case` on `mod University`, an unneeded `mut`, `marks` never read); a constructor with getters removes all three | [Chapter 6 exercises](ch6_katas/README.md) |

## The exercises, as katas

Every exercise is on its chapter page under one `## Practice` heading, restated, compiled as given (with rustc's transcript when it refuses) and solved by a recorded program. The rows are in [KATAS.md](../../KATAS.md), one per exercise, filed under the lesson each one exercises.

| Chapter | Page | Katas | What they exercise |
|---|---|---|---|
| 2 | [Chapter 2 exercises, run](ch2_katas/README.md) | 12 | `let`, `mut`, assigning once, shadowing, `u8`, floats, choosing integer types, a tuple alias, four function exercises |
| 3 | [Chapter 3 exercises, run](ch3_katas/README.md) | 6 | two sums over `1..n`, an assembly line and its rounding, a palindrome by bytes and by chars, a Pythagorean triple without a flag, a cinema rule |
| 4 | [Chapter 4 exercises, run](ch4_katas/README.md) | 6 | a `String` lent as `&str`, a move inside a loop, a block that closed, `E0502` across a `push`, a `&Vec` handed a `Vec`, two `&mut`s |
| 5 | [Chapter 5 exercises, run](ch5_katas/README.md) | 7 | an enum with data in one `Vec`, a record with an enum field, `Some(_)` and `None`, `Ok`/`Err` arms, a `Result` return type, a `HashMap` register |
| 6 | [Chapter 6 exercises, run](ch6_katas/README.md) | 5 | `pub` on an enum one module down, a path through a private module, one `use` line, private fields against a constructor, two `pub use` lines |

The sections checked claim by claim carry a kata each as well: [§2.3–2.4](../../25_Control_Flow/functions_claims_checked/README.md#practice), [§4.2 and §4.4](../../18_Ownership/ownership_in_functions_claims_checked/README.md#practice), [Mutable binding, mutable reference](../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md#practice), [A first `HashMap`](../../26_Collections/a_first_hashmap/README.md#practice), [A first `HashSet`](../../26_Collections/a_first_hashset/README.md#practice), [chapter 6](../../27_Modules/modules_claims_checked/README.md#practice), and the two lessons the book's pages graduated, [Items inside a function](../../27_Modules/items_inside_a_function/README.md#practice) and [Packages and crates](../../27_Modules/packages_and_crates/README.md#practice).

## Reading it beside this library

The book's order is close to the [course order](../../index.md#the-course-in-order) here, so a chapter can be read with the matching section open: chapter 2 with [First programs](../../15_First_Programs/README.md) and [Numbers](../../19_Numbers/README.md), chapter 3 with [Control flow](../../25_Control_Flow/README.md), chapter 4 with [Ownership](../../18_Ownership/README.md), chapter 5 with [Structs](../../16_Structs/README.md), [Enums](../../13_Enums/README.md), [`Option` and `Result`](../../17_Option_and_Result/README.md) and [Collections](../../26_Collections/README.md), chapter 6 with [Modules](../../27_Modules/README.md). Where the book states a rule in one sentence, the lesson here usually has the transcript that shows it; where the book's sentence does not hold, the table above says which page shows that.

For what this library thinks of the book beside the others, see [Books](../books/README.md#the-friendly-paid-on-ramps). The same treatment of another book is [*Rust in Action*, run](../rust_in_action/README.md).

## See also

- [Books](../books/README.md) — the shelf, and where this one sits on it
- [*Rust in Action*, run](../rust_in_action/README.md) — the other book checked listing by listing
- [`and`, `or` and a first program's explanation, run](../../15_First_Programs/and_or_claims_checked/README.md) — a third book's opening chapter, checked the same way
- [Katas](../../KATAS.md) — the practice track, where every exercise above has a row
- [Exercises](../exercises/README.md) — the other practice tracks

## Po polsku

*Rust: The Practical Guide* Noumana Azama (Rheinwerk Computing, 2025) to kurs w formie książki: siedemnaście rozdziałów, każdy zakończony ćwiczeniami z rozwiązaniami. Te strony uruchamiają listingi i rozwiązania z rozdziałów 2–6 na rustc 1.98.0, sprawdzają zdania wokół nich i zachowują ćwiczenia jako katy (*katas*) w tej bibliotece. Kod z książki kompiluje się tam, gdzie książka to obiecuje; poprawki dotyczą prozy — na przykład przeniesienie (*move*) wektora nie kopiuje jego danych, `contains_key` zwraca `bool`, a nie `Option`, a folder `bin/` musi leżeć w `src/`.

**Szukaj po polsku:** Rust: The Practical Guide po polsku · ćwiczenia z Rusta z rozwiązaniami · książka do nauki Rusta · `rust practical guide exercises solutions`
