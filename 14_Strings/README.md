# Strings

**One line:** Text in Rust is one pattern met over and over — an *owner* and a *view* — and almost every surprise in this section comes from the same place: the owner keeps its bytes on the heap, and those bytes are UTF-8, so an index into them is a byte offset rather than a character.

Two types do nearly all the work. `String` owns its text and can grow it; `&str` is a borrowed window onto text somebody else owns — including the binary itself, which is where a literal lives. A function takes `&str` because it only needs to read; a struct field owns a `String` because it has to outlive the call that built it.

The rest of the section is what follows from the bytes underneath. `len()` counts bytes, not characters. `s[0]` does not compile. `+` insists on an owned value on its left. And a slice whose endpoint lands inside a character panics at run time rather than at compile time — the one place strings ask you to be careful rather than letting the compiler be careful for you.

| Lesson | Level | What it teaches |
|---|---|---|
| [`String` vs `&str`](string_vs_str/README.md) | 101 → 201 | The owner and the view — a literal lives in the *binary*, not the stack; `&String` coerces to `&str` for free; and why parameters take `&str` while fields own `String` |
| [String slices](string_slices/README.md) | 101 → 201 | A view is a pointer and a length — the stale byte index it replaces, the `E0502` that keeps it honest, `&s[..5]`, and the one way a slice panics: an index inside a character |
| [The anatomy of a `String`](anatomy_of_a_string/README.md) | 101 → 201 | Three words on the stack, bytes on the heap — `len` is what you have, `capacity` is what you paid for, growth doubles, and the borrow checker's rule against a view held across a `push_str` |
| [Making a `String`](making_a_string/README.md) | 101 → 201 | Five spellings for one conversion — and why you implement `Display`, never `ToString`, plus the `.to_string()` on a `String` that is a silent clone |
| [Concatenating strings](concatenating_strings/README.md) | 101 → 201 | `format!`, `+` and `join` — the single `Add` impl behind all of it, and the three error codes you get from putting the wrong side on the left |
| [Building a `String`](building_a_string/README.md) | 101 → 201 | `push_str`, `push`, and the `+` that eats its left operand — `format!` vs `write!` in a loop, and why `truncate` panics where a slice does |
| [Meet the `char`](meet_the_char/README.md) | 101 → 201 | One Unicode scalar, four bytes as a value, 1–4 inside a `String` — why `.len()` is not "how many characters", `s[0]` refuses to compile, and `'ß'.to_uppercase()` returns *two* letters |
| [Walking a `String`](walking_a_string/README.md) | 101 → 201 | Three item types and the split family — splits are the gaps between matches, `char_indices()` is not `chars().enumerate()`, and `split_whitespace()` silently shortens a row |
| [`&'static str`](static_str/README.md) | 201 | On a literal it is the same type as `&str` — where the annotation starts refusing things, `const` vs `static`, and the three ways a `String` really can yield one |
| [Six kinds of string](six_kinds_of_string/README.md) | 201 | `OsString`, `CString` and friends are not five more inventions — three promises about the bytes, each owned or borrowed, and narrowing is where a promise gets checked |

## The lessons strings lean on

Strings are the worked example half the ownership pages already use, so the deep explanations live there and this section links them rather than moving them:

- [Ownership and moves](../18_Ownership/ownership_and_moves/README.md) — the `E0382` a moved `String` produces, in full
- [`Copy` vs `Clone`](../16_Structs/copy_vs_clone/README.md) — why `&str` copies freely, and one `String` field makes a whole struct move
- [Borrowing](../18_Ownership/borrowing/README.md) — the rule that refuses a view held across a `push_str`
- [How to learn lifetimes](../18_Ownership/how_to_learn_lifetimes/README.md) — why "own `String`, clone when stuck" is legitimate advice
- [Meet the byte](../19_Numbers/meet_the_byte/README.md) — the unit `len` counts in
- [`Path` and `PathBuf`](../04_Files/path_and_pathbuf/README.md) — the family's honorary pair, a **stub** for now

[STRINGS.md](../STRINGS.md) is the full map: the same lessons with the question each one answers, plus the nine topics that are still outlines rather than lessons.

[Strings: links, books and videos](resources/README.md) is the reading list — the Book, *Programming Rust* ch. 17, Easy Rust, the essays, and the exercise sets outside this library that map onto these pages.
