# A struct that points into itself

**Level:** 201 → 301 · working knowledge

**One line:** Safe Rust *will* build a struct whose field borrows another of its own fields, in three different ways. None of them can then be moved, returned from a function, mutated or given a destructor, so the version people actually want, a constructor that returns one, does not compile. Store *where* the part is, a range, instead of a reference to it.

## The working answer: store where, not a reference

```rust
use std::ops::Range;

struct Sentence {
    text: String,
    first_word: Range<usize>, // a position in `text`: borrows nothing
}

impl Sentence {
    fn new(text: String) -> Sentence {
        let end = text.find(' ').unwrap_or(text.len());
        Sentence { text, first_word: 0..end }
    }

    fn first_word(&self) -> &str {
        &self.text[self.first_word.clone()]
    }
}

fn main() {
    let sentences = vec![Sentence::new(String::from("borrow it forever"))]; // returned, then moved
    println!("{}", sentences[0].first_word()); // borrow
}
```

A range has no lifetime, so `Sentence` has none. It can be returned, pushed into a `Vec` and handed to another thread. The reference comes back only when you ask for it, from `&self`, and lasts only as long as that borrow of `self`.

The rest of the page is the version with a reference in it, and quinedot's [*Avoid self-referential structs* ↗](https://quinedot.github.io/rust-learning/pf-meta.html#avoid-self-referential-structs) checked on rustc 1.98.0. The advice is right. One claim is not.

## What people actually want

A constructor:

```rust
struct Snek<'a> {
    owned: String,
    borrowed: &'a str,
}

// fn new<'a>(owned: String) -> Snek<'a> {
//     let mut snek = Snek { owned, borrowed: "" };
//     snek.borrowed = &snek.owned;
//     snek                        // E0505 and E0515
// }
```

Two errors ([entry 14](../borrowing_forever_errors/README.md#14-a-constructor-that-returns-one) has the transcript). Returning `snek` moves it while `snek.owned` is borrowed (`E0505`), and the value returned would refer to `snek.owned`, a local of `new` (`E0515`). No lifetime written on `new` can help, because the text does not exist until `new` runs. This is the case people usually mean by "you can't do that in safe Rust".

## Built in place: no `&'a mut` needed

Inside one function, with no constructor, it compiles:

```rust
struct Snek<'a> {
    owned: String,
    borrowed: &'a str,
}

fn main() {
    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    snek.borrowed = &snek.owned; // a shared borrow of one field, stored in another
    println!("{} {}", snek.borrowed, snek.owned); // hiss hiss
    let by_ref = &snek;
    println!("{}", by_ref.borrowed); // hiss
}
```

`&snek.owned` borrows the *place* `snek.owned`, not `snek`, and assigning to a different field of `snek` is allowed while that borrow is live. Reading afterwards is fine, through `snek` or through `&snek`, because shared borrows overlap. Everything else is refused:

| Then you write | rustc 1.98.0 | Entry |
|---|---|---|
| `let sneks = vec![snek];` | `E0505` cannot move out of `snek` because it is borrowed | [10](../borrowing_forever_errors/README.md#10-moving-a-struct-built-in-place) |
| `snek.borrowed = "elsewhere";` and then the move | `E0505` again | [11](../borrowing_forever_errors/README.md#11-moving-it-after-pointing-the-field-elsewhere) |
| `snek.owned.push_str("!");` | `E0502` cannot borrow `snek.owned` as mutable because it is also borrowed as immutable | [12](../borrowing_forever_errors/README.md#12-changing-the-owned-field) |
| `let m = &mut snek;` | `E0502` cannot borrow `snek` as mutable because it is also borrowed as immutable | |
| `impl Drop for Snek<'_>` | `E0713` borrow may still be in use when destructor runs | [13](../borrowing_forever_errors/README.md#13-a-drop-impl-on-a-self-referential-struct) |

The second row is the surprising one. Pointing `borrowed` somewhere else changes the value but not its type. `snek: Snek<'a>` and the borrow `&snek.owned` share one `'a`, and a later use of the moved value keeps that `'a`, and so the borrow, alive at the move.

## Through a method: `bite(&'a mut self)`

quinedot's version builds the same thing from inside a method:

```rust
struct Snek<'a> {
    owned: String,
    borrowed: &'a str,
}

impl<'a> Snek<'a> {
    fn bite(&'a mut self) -> &'a Snek<'a> { // the handle has to be shared
        self.borrowed = &self.owned;
        self
    }
}

fn main() {
    let mut snek = Snek { owned: String::from("rattle"), borrowed: "" };
    let handle = snek.bite();
    println!("{}", handle.borrowed); // rattle
    // println!("{}", snek.borrowed); // E0502: `snek` is borrowed for the rest of its life
}
```

`&'a mut self` on a `Snek<'a>` is [borrowing something forever](../borrowing_forever/README.md), so `snek` can never be named again ([entry 8](../borrowing_forever_errors/README.md#8-reading-a-field-after-bitea-mut-self)). What `bite` hands back is the only way left to reach the struct, and it must be shared. `-> &'a mut Self` fails inside the method (`E0502`, [entry 9](../borrowing_forever_errors/README.md#9-bite-handing-back-a-mut-self)), because `self.owned` is already shared-borrowed for `'a`.

## Through `&'a self` and a `Cell`

A third route, with no `&mut` at all: put the reference in a [`Cell`](../../09_Advanced/interior_mutability/README.md), whose `set` takes `&self`.

```rust
use std::cell::Cell;

struct CellSnek<'a> {
    owned: String,
    borrowed: Cell<&'a str>,
}

impl<'a> CellSnek<'a> {
    fn bite(&'a self) {
        self.borrowed.set(&self.owned);
    }
}

fn main() {
    let snek = CellSnek { owned: String::from("coil"), borrowed: Cell::new("") };
    snek.bite();
    snek.bite(); // shared borrows overlap, so a second call is fine
    println!("{} {}", snek.owned, snek.borrowed.get()); // coil coil
}
```

It is still the same lock, only shared: moving `snek` is `E0505` and `&mut snek` is `E0502`.

## Why a move that would be harmless is refused

Section 1 of the run below prints `true` for "the text's heap bytes stayed put through the move". Moving a `String` copies its three-word header ([What an address shows](../what_an_address_shows/README.md)), and the bytes a `&str` into it points at do not move ([Drawing the owner and the view](../../14_Strings/drawing_the_owner_and_the_view/README.md) draws both). So for this particular `Snek`, the move would leave `borrowed` valid.

The borrow checker does not reason about heaps. `&snek.owned` is a borrow of the place `snek.owned`, and a move ends that place. Crates such as [`ouroboros` ↗](https://docs.rs/ouroboros/latest/ouroboros/), [`self_cell` ↗](https://docs.rs/self_cell/latest/self_cell/) and [`yoke` ↗](https://docs.rs/yoke/latest/yoke/) rely on exactly that stable heap address. They wrap the `unsafe` code that asserts it behind an API the borrow checker can accept. (They are listed, not run, on [the reading list](../borrowing_forever_resources/README.md#crates-that-build-one-for-you).) When the borrowed data is inline and really does move with the struct, as in the state machine an `async fn` compiles to, [`Pin` ↗](https://doc.rust-lang.org/std/pin/index.html#a-self-referential-struct) is the promise that it will not move.

## The whole verified run

<!-- output:self_referential_structs -->
*Verified output of [`self_referential_structs.rs`](examples/self_referential_structs.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A range instead of a reference: returned, moved, mutated
   first word after a return and a move into a Vec: "borrow"
   the text's heap bytes stayed put through the move: true
   replaced, and the range was recomputed: "pin"

2. Built in place, with no &'a mut anywhere
   snek.borrowed = "hiss", snek.owned = "hiss"
   through & and a &self method, points into itself: true
   refused from here on: moving snek, &mut snek, snek.owned.push_str,
   returning it from a function, and a Drop impl on Snek.

3. Built through bite(&'a mut self)
   through the returned handle: "rattle", points into itself: true
   `bitten` itself may not be named again, not even to read a field.

4. Built through &'a self and a Cell
   bitten twice, still readable: owned "coil", borrowed "coil"
   shared access survives; moving it or taking &mut does not.
```
<!-- /output -->

## quinedot's claims, run

| The guide says | Run on 1.98.0 |
|---|---|
| The only safe way to construct one is to take a `&'a mut Snek<'a>`, borrow `owned` and assign it to `borrowed` | **False.** Assigning `snek.borrowed = &snek.owned` in place (section 2) and `&'a self` with a `Cell` (section 4) both build one with no `&'a mut` anywhere. |
| `fn bite(&'a mut self) { self.borrowed = &self.owned; }` compiles | **True.** |
| After `bite`, you cannot use the struct directly ever again | **True** for `bite` (`E0502`). Not for the in-place build, which stays readable, though it can never move, mutate or drop. |
| The only way to use it is through a reborrowed return value from that method | **True, with a condition the guide leaves out:** the reborrow must be shared. `-> &'a mut Self` is `E0502` inside `bite`. |
| Technically possible, but so restrictive it is pretty much always useless | **Fair.** All three builds are stuck where they were made: no move, no return, no `&mut`, no `Drop`. |
| The answer you will hear: "you can't do that in safe Rust" | **An approximation, as the guide says.** You can build one. You cannot return one from a constructor, which is what the question usually wants. |

## Ways out

- **A range or an index** into the owned data: [the working answer](#the-working-answer-store-where-not-a-reference) and [the kata](#practice). The one cost is keeping it up to date when the data changes.
- **Keep the owner outside** and make a second struct of views that borrows from it: [entry 10's fix](../borrowing_forever_errors/README.md#10-moving-a-struct-built-in-place), and section 2 of the [zero-copy log kata](../how_to_learn_lifetimes/README.md#practice).
- **Borrow on demand:** a method `fn borrowed(&self) -> &str` computes the part each time it is asked ([entry 8's fix](../borrowing_forever_errors/README.md#8-reading-a-field-after-bitea-mut-self)).
- **A crate** that generates the `unsafe` code for a heap-backed owner (above), when recomputing is too expensive and a range cannot describe the borrowed part, such as a parsed structure full of `&str`.

## If you are coming from another language

- **C.** Legal and silent. `struct snek { char owned[16]; const char *borrowed; }` with `a.borrowed = a.owned` compiles, and `struct snek b = a;` copies the bytes, so `b.borrowed` still points into `a`. Run with `cc -std=c17 -Wall -Wextra`: no warning, and after `strcpy(a.owned, "gone")` printing `b.borrowed` prints `gone`. That copy is the move Rust refuses. Nothing in C knows the pointer was meant to follow the struct.
- **C++.** Same default, plus a repair hook: a copy or move constructor can re-point the pointer at the new object. GCC's own `std::string` does this. Run under `g++` 14.4 (libstdc++), a short string's first word is a pointer into the string object, and after `std::string t = std::move(s)` the first word of `t` points into `t`. The same program under macOS's libc++ prints `0` for both, because that library lays out short strings differently. Rust has no move constructors, since a move is always a byte copy ([There is no `Move` trait](../no_move_trait/README.md)), so there is nowhere to repair the pointer. That is why the compiler refuses, and why `Pin` exists.
- **Go.** `b := a` copies the struct, so a `*byte` that pointed into `a.owned` still points into `a`. Run on Go 1.25: `b.borrowed == &a.owned[0]` is `true`, and `go vet` says nothing. The garbage collector keeps `a` alive, so the result is a stale alias rather than a dangling pointer. The one case Go tooling does check is a lock: the [Go library's mutex page ↗](https://masiarek.github.io/go-learning-library/04_Sync/a_mutex_guards_a_counter/index.html) shows `go vet`'s `copylocks` catching a copied `sync.Mutex`, for the same reason: some values must not move once in use.
- **Python and Java.** An object whose attribute refers to itself or to one of its own members is ordinary. Names hold references to heap objects, so nothing is copied out from under a reference, and a compacting collector updates the references when it moves an object. The garbage collector does the job this page does by hand.
- **Async in any language.** An `async fn`'s state machine can hold a reference to one of its own locals across an `.await`, which makes it self-referential. That is where `Pin` comes from ([Pinning, a stub in the Concurrency library ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/pinning/index.html)).

## Practice

**Give the log a constructor.** This does not compile:

```rust
struct Log<'a> {
    text: String,
    longest: &'a str, // the longest line of `text`
}

// fn new<'a>(text: String) -> Log<'a> { … } // E0505 and E0515, whatever the body
```

Rewrite `Log` without a lifetime so that `Log::new(text)` returns one, `longest()` hands back the longest line as `&str`, the log can be pushed into a `Vec`, and `push_line(&mut self, line)` appends a line and keeps `longest()` correct. For `"ok\nERROR disk full\nok"` the longest line is `"ERROR disk full"`. On a tie keep the first, and an empty log's longest line is `""`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:self_referential_structs_kata -->
*[`self_referential_structs_kata.rs`](examples/self_referential_structs_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a log that owns its text and knows its longest line, built
//! by a constructor, moved into a Vec and mutated. The self-referential
//! version could do none of that; a range per line can do all three.
//!
//!   rustc --edition 2024 self_referential_structs_kata.rs -o /tmp/srsk && /tmp/srsk
//!   rustc --edition 2024 --test self_referential_structs_kata.rs -o /tmp/srskt && /tmp/srskt

use std::ops::Range;

/// Was `struct Log<'a> { text: String, longest: &'a str }`, whose `new` was
/// E0505 (moving `log` out while `log.text` is borrowed) and E0515
/// (returning a value that references the local `log.text`).
struct Log {
    text: String,
    longest: Range<usize>,
}

fn longest_line(text: &str) -> Range<usize> {
    let mut best = 0..0;
    let mut start = 0;
    for line in text.split('\n') {
        if line.len() > best.len() {
            best = start..start + line.len();
        }
        start += line.len() + 1;
    }
    best
}

impl Log {
    fn new(text: String) -> Log {
        let longest = longest_line(&text);
        Log { text, longest }
    }

    fn longest(&self) -> &str {
        &self.text[self.longest.clone()]
    }

    /// Mutating the text invalidates the range, so recompute it here, in the
    /// one method allowed to change `text`.
    fn push_line(&mut self, line: &str) {
        self.text.push('\n');
        self.text.push_str(line);
        self.longest = longest_line(&self.text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_longest_line() {
        assert_eq!(Log::new(String::from("ok\nERROR disk full\nok")).longest(), "ERROR disk full");
        assert_eq!(Log::new(String::from("same\nsize")).longest(), "same");
        assert_eq!(Log::new(String::new()).longest(), "");
    }

    #[test]
    fn survives_a_move_and_a_push() {
        let mut logs = vec![Log::new(String::from("a\nbb"))];
        logs[0].push_line("cccc");
        assert_eq!(logs[0].longest(), "cccc");
    }
}

fn main() {
    println!("1. Built by a constructor and returned");
    let log = Log::new(String::from("ok\nERROR disk full\nok"));
    println!("   longest = {:?}", log.longest());

    println!();
    println!("2. Moved into a Vec");
    let mut logs = vec![log];
    println!("   logs[0].longest = {:?}", logs[0].longest());

    println!();
    println!("3. Mutated, and the range recomputed");
    logs[0].push_line("WARNING disk nearly full again");
    println!("   longest = {:?}", logs[0].longest());
    println!("   range   = {:?}", logs[0].longest);
}
```
<!-- /source -->

<!-- output:self_referential_structs_kata -->
*Verified output of [`self_referential_structs_kata.rs`](examples/self_referential_structs_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Built by a constructor and returned
   longest = "ERROR disk full"

2. Moved into a Vec
   logs[0].longest = "ERROR disk full"

3. Mutated, and the range recomputed
   longest = "WARNING disk nearly full again"
   range   = 22..52
```
<!-- /output -->

</details>

## See also

- [Borrowing something forever](../borrowing_forever/README.md) — `&'a mut Thing<'a>`, the lock `bite` takes
- [Every borrowed-forever error, and its fix](../borrowing_forever_errors/README.md) — entries 8 to 14 are this page's refusals, each with its transcript and a fix that compiles
- [Lints around borrowing forever](../borrowing_forever_lints/README.md#what-no-lint-catches) — no lint names any of the three builds
- [Reading on borrowing forever](../borrowing_forever_resources/README.md) — quinedot, fasterthanlime's video, the Stack Overflow answer, the `Pin` docs, crates
- [How to learn lifetimes](../how_to_learn_lifetimes/README.md#practice) — the zero-copy log kata, which meets this wall as `E0515` and `E0505` and gets out with ranges
- [There is no `Move` trait](../no_move_trait/README.md) — moves are byte copies with no hook to run, the reason a self-pointer cannot be repaired
- [What an address shows](../what_an_address_shows/README.md) — a move copies the header and leaves the heap bytes where they are
- [Drawing the owner and the view](../../14_Strings/drawing_the_owner_and_the_view/README.md) — the `String` and the `&str` into its buffer, drawn: this page puts both in one struct
- [Borrowed state](../borrowed_state/README.md) — `E0505`, the owner refused its own binding
- [A borrow is a loan](../references/a_borrow_is_a_loan/README.md) — `&snek.owned` is a loan on one place, which is why the other field can still be assigned
- [References: the map](../references/README.md) — the path through `&`, `&mut` and the pointer types beside them
- [Interior mutability](../../09_Advanced/interior_mutability/README.md) — the `Cell` that makes the `&'a self` build possible
- [Implementing `Iterator`](../../24_Iterators/implementing_iterator/README.md) — a borrowing iterator is a struct holding a reference to data *outside* it, the shape that works
- [Structs resources](../../10_Resources/structs/README.md) — fasterthanlime's *Self-referential structs*, the video to watch after this page

## Po polsku

Struktura samoreferencyjna (*self-referential struct*) to taka, której jedno pole pożycza z innego pola **tej samej** struktury. Bezpieczny Rust potrafi ją zbudować na trzy sposoby: przypisaniem `snek.borrowed = &snek.owned` w miejscu, metodą `bite(&'a mut self)` i przez `Cell` z `&'a self`. Twierdzenie z przewodnika quinedota, że jedyną drogą jest `&'a mut Snek<'a>`, jest więc **fałszywe**, co sprawdziliśmy kompilatorem.

Każda z tych wersji zostaje jednak **przybita do miejsca**: nie da się jej przenieść, zwrócić z funkcji, zmienić przez `&mut` ani dodać jej `Drop`. A konstruktor `fn new(...) -> Snek`, o który zwykle chodzi, daje `E0505` i `E0515`. Rozwiązanie w praktyce: zamiast referencji przechowuj **zakres** (`Range<usize>`) albo indeks, trzymaj właściciela danych poza strukturą, albo sięgnij po gotowy crate (`ouroboros`, `self_cell`, `yoke`).

**Szukaj po polsku:** struktura samoreferencyjna · przypinanie (`Pin`) · referencja do własnego pola · `rust self-referential struct` · `rust store value and reference same struct`
