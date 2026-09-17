# `ref` borrows where a pattern would move

[References](../README.md) › **The `ref` keyword**

**Level:** 201 · working knowledge

**One line:** `ref` changes how a binding binds — a borrow instead of a move — and never which values the pattern matches; `&` in a pattern does the opposite, and since match ergonomics you rarely need to write `ref` at all.

```rust
let maybe_name = Some(String::from("Alice"));
match maybe_name {
    Some(ref n) => println!("Hello, {n}"),   // Hello, Alice   <- n: &String
    _ => println!("Hello, world"),
}
println!("Hello again, {}", maybe_name.unwrap_or("world".into()));   // Hello again, Alice
```

This is the second example on [std's page for the keyword ↗](https://doc.rust-lang.org/std/keyword.ref.html), with its comments swapped for the output. `n` borrows the `String` inside the option, so `maybe_name` is still whole when the match ends.

## Without `ref`: a partial move

std's first example is the same program with `Some(n)`. Its comment says `maybe_name` *"is consumed here"*; rustc says what actually happened:

```text title="Abridged — real rustc output for std_docs_move.rs"
error[E0382]: use of partially moved value: `maybe_name`
 --> std_docs_move.rs:9:33
  |
5 |         Some(n) => println!("Hello, {n}"),
  |              - value partially moved here
...
9 |     println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
  |                                 ^^^^^^^^^^ value used here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
5 |         Some(ref n) => println!("Hello, {n}"),
  |              +++
```

The `String` moved out of the option, so the option can no longer be used as a whole. The header reads *use of* because `unwrap_or` takes `self`; put `println!("{maybe_name:?}")` on that line instead and it reads *borrow of partially moved value*. rustc's `help:` is the keyword itself.

## What `ref` changes, and what it does not

std's docs put it in one line: *"`Foo(ref foo)` matches the same objects as `Foo(foo)`."* `ref` is a **binding mode**. It sits on a name, adds one `&` to that name's type, and plays no part in matching. `&` is a [reference pattern ↗](https://doc.rust-lang.org/reference/patterns.html#reference-patterns): it demands a reference in the value, matches what is behind it, and binds that.

| Pattern | Scrutinee | Matches | Binds |
|---|---|---|---|
| `Some(x)` | `Option<i32>` | every `Some` | `x: i32` — a copy (a move, for a non-`Copy` payload) |
| `Some(ref x)` | `Option<i32>` | every `Some` — the same values | `x: &i32` — points at the payload |
| `Some(x)` | `Option<&i32>` | every `Some` | `x: &i32` — the reference itself |
| `Some(&x)` | `Option<&i32>` | every `Some` | `x: i32` — the reference removed |
| `Some(&x)` | `Option<String>` | — | `E0308` |

A 2018 comment under [the accepted Stack Overflow answer ↗](https://stackoverflow.com/questions/31949579/understanding-and-relationship-between-box-ref-and) calls `Some(&v)` the *"direct opposite of `Some(ref v)`"*, and the table agrees: `ref` adds a reference, `&` takes one away. They are not two spellings of one thing, and the payload decides whether `&` even type-checks:

```text title="Abridged — real rustc output for amp_on_owned.rs"
error[E0308]: mismatched types
 --> amp_on_owned.rs:4:14
  |
3 |     match maybe_name {
  |           ---------- this expression has type `Option<String>`
4 |         Some(&n) => println!("Hello, {n}"),
  |              ^^ expected `String`, found `&_`
  |
  = note: expected struct `String`
          found reference `&_`
help: consider removing `&` from the pattern
  |
4 -         Some(&n) => println!("Hello, {n}"),
4 +         Some(n) => println!("Hello, {n}"),
  |
```

On an `Option<&String>` it type-checks and is still refused: `&n` would move the `String` out from behind a shared reference, `E0507`. `&` in a pattern copies out; it cannot move out. [Where the `&` sits](../../where_the_sigil_sits/README.md) has the same rule for `let &x = r;` and `|&n|`.

## `let ref z = x;` is `let z = &x;`

```rust
let x: u32 = 12;
let ref z = x;   // z: &u32
let z2 = &x;     // z2: &u32 — the same address, per the run below
println!("{z} {z2}");   // 12 12
```

A `let` is a pattern too, so `ref` works there, and the second line is how to write it. clippy's `toplevel_ref_arg` (style group, warn by default) flags the first: *"`ref` on an entire `let` pattern is discouraged, take a reference with `&` instead"*.

`let ref mut zz = x;` is `let zz = &mut x;`, so `x` has to be `mut`. The Stack Overflow answer's 2015 snippet declares `let x: u32 = 12;` and uses both lines:

```text title="Abridged — real rustc output for let_ref_mut.rs, unused-variable warnings trimmed"
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> let_ref_mut.rs:6:9
  |
6 |     let ref mut zz = x;  // zz: &mut u32, points to x
  |         ^^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut x: u32 = 12;
  |         +++
```

## A name binds, and moves, even when you never use it

| Arm, on an `Option<String>` | Binds | `maybe_name` afterwards |
|---|---|---|
| `Some(_)` | nothing | whole |
| `Some(n)`, `n` unused | `n: String` | partially moved — `E0382`, plus an `unused variable` warning |
| `Some(_n)` | `_n: String` | partially moved — `E0382`, no warning |
| `Some(ref _n)` | `_n: &String` | whole |

`_` is not a name, so nothing is bound and nothing moves. `_n` is a name; the underscore silences the lint and changes nothing else:

```text title="Abridged — real rustc output for underscore_name_moves.rs"
error[E0382]: borrow of partially moved value: `maybe_name`
 --> underscore_name_moves.rs:7:16
  |
4 |         Some(_n) => println!("some"),
  |              -- value partially moved here
...
7 |     println!("{maybe_name:?}");
  |                ^^^^^^^^^^ value borrowed here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
4 |         Some(ref _n) => println!("some"),
  |              +++
```

## One pattern, two modes

`ref` goes on one name at a time, so a single pattern can move one field and borrow another:

```rust
struct Order { id: String, note: String }

let order = Order { id: String::from("A-17"), note: String::from("fragile") };
let Order { id, ref note } = order;   // id: String (moved), note: &String
println!("{id} {note}");              // A-17 fragile
println!("{}", order.note);           // fragile — it never left the struct
```

`order.note` is still in place. `order.id` is not — `E0382`, *borrow of moved value: `order.id`* — and neither is `order` as a whole (with `#[derive(Debug)]` added to print it):

```text title="Abridged — real rustc output for partial_move.rs"
error[E0382]: borrow of partially moved value: `order`
  --> partial_move.rs:12:16
   |
 9 |     let Order { id, ref note } = order;
   |                 -- value partially moved here
...
12 |     println!("{order:?}");        // the whole struct: refused
   |                ^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `order.id` has type `String`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
   |
 9 |     let Order { ref id, ref note } = order;
   |                 +++
```

That is [ownership tracked per field](../../ownership_and_moves/README.md#ownership-is-tracked-per-field-not-per-variable); `ref` picks, name by name, which side of it each binding lands on. Matching `&order` instead cannot do this — every binding under it borrows, so none can move. The kata below is that case.

## `ref mut`: write into the place you matched

```rust
let mut tally = Some(42);
if let Some(ref mut n) = tally {   // n: &mut i32, pointing into tally
    *n += 1;
}
println!("{tally:?}");             // Some(43)
```

The place has to be `mut`. Without it:

```text title="Abridged — real rustc output for tally_not_mut.rs"
error[E0596]: cannot borrow `tally.0` as mutable, as `tally` is not declared as mutable
 --> tally_not_mut.rs:3:17
  |
3 |     if let Some(ref mut n) = tally {
  |                 ^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut tally = Some(42);
  |         +++
```

`if let Some(n) = &mut tally` does the same thing without the keyword — [match ergonomics](../../../30_Pattern_Matching/match_ergonomics/README.md) reaches `ref mut` from the scrutinee.

## Edition 2024: `ref` where the borrow is already implied

Match a reference with a non-reference pattern and the default binding mode is already `ref`. Edition 2021 accepts a `ref` written there anyway, as a redundancy. Edition 2024 refuses it:

```rust
let maybe_name = Some(String::from("Alice"));
match &maybe_name {
    Some(n) => println!("Hello, {n}"),   // Hello, Alice   <- n: &String, no `ref` needed
    None => println!("Hello, world"),
}
// Some(ref n) in that arm: compiles under --edition 2021, refused under --edition 2024
```

```text title="Abridged — real rustc output for redundant_ref.rs, --edition 2024"
error: cannot explicitly borrow within an implicitly-borrowing pattern
 --> redundant_ref.rs:4:14
  |
4 |         Some(ref n) => println!("Hello, {n}"),
  |              ^^^ explicit `ref` binding modifier not allowed when implicitly borrowing
  |
  = note: for more information, see <https://doc.rust-lang.org/reference/patterns.html#binding-modes>
note: matching on a reference type with a non-reference pattern implicitly borrows the contents
 --> redundant_ref.rs:4:9
  |
4 |         Some(ref n) => println!("Hello, {n}"),
  |         ^^^^^^^^^^^ this non-reference pattern matches on a reference type `&_`
help: remove the unnecessary binding modifier
  |
4 -         Some(ref n) => println!("Hello, {n}"),
4 +         Some(n) => println!("Hello, {n}"),
  |
```

No error code. The same rule covers `&` and `mut` under a borrowing default mode — the three messages, each compiled under both editions:

| Under `match &value` | `--edition 2021` | `--edition 2024` |
|---|---|---|
| `Some(ref n)` (and `Some(ref mut n)` under `&mut value`) | compiles; `ref` is redundant | *cannot explicitly borrow within an implicitly-borrowing pattern* |
| `Some(&n)`, value an `Option<&i32>` | compiles | *cannot explicitly dereference within an implicitly-borrowing pattern* |
| `(mut a, b)`, value a `(i32, i32)` | compiles; `mut` resets `a` to by-value, so `a: i32` | *cannot mutably bind by value within an implicitly-borrowing pattern* |

Two fixes: delete the `ref`, as rustc's `help:` says, or make the whole pattern explicit — `&Some(ref n)` against `&maybe_name` compiles under 2024, because nothing in it is left to ergonomics. On a 2021 crate, `-W rust-2024-compatibility` reports all three as the lint `rust_2024_incompatible_pat`, with a link to the [edition guide's *Match ergonomics reservations* ↗](https://doc.rust-lang.org/edition-guide/rust-2024/match-ergonomics.html), which says the redundant forms were disallowed *"to leave space for other language possibilities"*.

## Where `ref` is still the natural spelling

Most of the time, borrow the scrutinee instead. What is left:

- **Moving one field and borrowing another in one pattern.** Above, and in the kata. `&` on the scrutinee sets every binding at once; `ref` sets one.
- **`@` on an owned value.** `whole @ Some(inner)` on an `Option<String>` is `E0382`, *use of partially moved value*, and rustc's `help:` writes `ref whole @ Some(ref inner)`. `match &maybe_name { whole @ Some(inner) => … }` compiles too — see [Binding with `@`](../../../30_Pattern_Matching/binding_at/README.md).
- **Keeping a temporary alive.** `let (ref first, _) = pair();` holds the whole tuple open — on [Borrowed state](../../borrowed_state/README.md#ref-keeps-the-whole-value-alive).
- **Taking rustc's suggestion.** Every `E0382` on this page from a pattern move carries a `help:` that adds `ref`. Applying it is correct; borrowing the scrutinee is the other fix.

clippy has `ref_patterns` (restriction group, allow by default) for crates that want the keyword gone.

## Checkpoint

**Predict before you open the answer.** An explanation of `Foo::Bar(x, ref msg)`, where `x` is an `i32`, says *"we can still use the borrowed part, but not `foo`"*. The same shape as a tuple: after `let pair = (5, String::from("hi")); let (count, ref label) = pair;` — is `pair` still usable?

<details markdown="1">
<summary><strong>The answer</strong></summary>

<!-- output:the_ref_keyword -->
*Verified output of [`the_ref_keyword.rs`](examples/the_ref_keyword.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. std's own example: `Some(ref n)` borrows, so `maybe_name` outlives the match
   Hello, Alice
   Hello again, Alice

2. `ref` changes the binding's type, never which values match
   value      Some(x)           Some(ref x)
   Some(3)    x: i32 = 3        x: &i32 = 3
   None       no match          no match

3. `&` is the opposite: part of what matches, and it removes a reference
   value      Some(x)           Some(&x)
   Some(&3)   x: &i32 = 3       x: i32 = 3
   None       no match          no match

4. `let ref z = x;` is `let z = &x;`
   let ref z = x;   z:  &u32 = 12
   let z2 = &x;     z2: &u32 = 12
   same address: true

5. `_` binds nothing, so it moves nothing
   Some(_) matched
   maybe_name afterwards: Some("Alice")

6. One pattern, two modes: move `id`, borrow `note`
   id:   alloc::string::String = A-17
   note: &alloc::string::String = fragile
   order.note is still there: fragile

7. `ref mut` writes through the binding, into the place it matched
   Some(ref mut n) = tally       -> tally = Some(43)
   Some(n) = &mut tally          -> tally = Some(44)

8. `ref` on both sides of `@`: the whole and a part, neither moved
   whole: &core::option::Option<alloc::string::String> = Some("Alice")
   inner: &alloc::string::String = Alice
   maybe_name afterwards: Some("Alice")

Checkpoint. After `let (count, ref label) = pair;` is `pair` still usable?
   count: i32 = 5
   label: &alloc::string::String = hi
   pair afterwards: (5, "hi")
```
<!-- /output -->

Yes. `count` binds an `i32`, which is `Copy`, so binding it by value copies; `label` borrows. Nothing moved out of `pair`. The claim holds only when the by-value field is not `Copy` — the `Order` case above.

</details>

## Practice

**Borrow one field, move the other.** Given `struct Upload { name: String, bytes: Vec<u8> }` and `fn store(bytes: Vec<u8>) -> usize`, write one `let` pattern on `upload` that hands `bytes` to `store` and keeps `name` as a borrow, then print `upload.name` afterwards. Say whether `upload.bytes.len()` and `let whole = upload;` still compile after it, and why `let Upload { name, bytes } = &upload;` cannot replace your pattern.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_ref_keyword_kata -->
*[`the_ref_keyword_kata.rs`](examples/the_ref_keyword_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one `let` pattern that moves one field and borrows the other.
//!
//!   rustc --edition 2024 the_ref_keyword_kata.rs -o /tmp/the_ref_keyword_kata && /tmp/the_ref_keyword_kata

struct Upload {
    name: String,
    bytes: Vec<u8>,
}

/// Takes the buffer by value: whoever calls this gives it up.
fn store(bytes: Vec<u8>) -> usize {
    bytes.len()
}

fn main() {
    let upload = Upload { name: String::from("notes.txt"), bytes: vec![7, 8, 9] };

    // `bytes` binds by value (moves out), `name` binds by reference (borrows).
    let Upload { ref name, bytes } = upload;
    let stored = store(bytes);
    println!("stored {stored} bytes for {name}");

    // `name` never left the struct, so the field is still readable.
    println!("upload.name afterwards: {}", upload.name);

    // Not these two — each is E0382, because `bytes` moved out of `upload`:
    //   println!("{}", upload.bytes.len());   // borrow of moved value: `upload.bytes`
    //   let whole = upload;                   // use of partially moved value: `upload`

    // Why `&upload` cannot do this in one pattern:
    //   let Upload { name, bytes } = &upload;          // bytes: &Vec<u8>, so store(bytes) is E0308
    //   let &Upload { ref name, bytes } = &upload;     // E0507: cannot move out of a shared reference
    println!("one pattern, two binding modes: `ref` switches one name, `&upload` switches them all");
}
```
<!-- /source -->

<!-- output:the_ref_keyword_kata -->
*Verified output of [`the_ref_keyword_kata.rs`](examples/the_ref_keyword_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
stored 3 bytes for notes.txt
upload.name afterwards: notes.txt
one pattern, two binding modes: `ref` switches one name, `&upload` switches them all
```
<!-- /output -->

</details>

## If you are coming from another language

- **C++.** Structured bindings make the choice once, for the whole declaration: `auto& [id, note] = o;` aliases both members, and `auto [id, note] = std::move(o);` moves the entire object into one hidden variable. There is no per-name qualifier — `auto [id, &note] = o;` is a syntax error (clang 21: *expected identifier*). Rust's `ref` is that missing per-name switch. What changes is the aftermath: after a C++ move, `o` is still usable, its moved-from `std::string` members in a valid but unspecified state; after `let Order { id, ref note } = order;` rustc refuses to read `order.id` or use `order` as a whole, and still lets you read `order.note`.
- **Python.** Every capture in a `match` is already an alias, never a move: `case (items, label):` binds names to the very objects inside the tuple — `items is pair[0]` is `True`, and `items.append(...)` changes `pair`. Nothing is ever taken out, so there is no `Some(n)` versus `Some(ref n)` to choose between. Rust's default is the opposite — a binding takes the value — and `ref` is how you ask for an alias. The difference that remains: a Python alias to a mutable object can always write through it, where a `ref` binding is read-only and writing needs `ref mut` on a `mut` place.

## See also

- [Match ergonomics](../../../30_Pattern_Matching/match_ergonomics/README.md) — default binding modes, and [the `ref` spelling they replaced](../../../30_Pattern_Matching/match_ergonomics/README.md#what-it-replaced)
- [Borrowed state](../../borrowed_state/README.md) — what a `ref` binding does to the place it borrows from, including a temporary
- [Where the `&` sits](../../where_the_sigil_sits/README.md) — `&x` in a pattern removes a reference
- [Destructuring enums](../../../30_Pattern_Matching/destructuring_enums/README.md) — the variant patterns `ref` goes inside
- [Binding with `@`](../../../30_Pattern_Matching/binding_at/README.md) — the `ref whole @ …` form
- [Ownership and moves](../../ownership_and_moves/README.md) — the move a plain binding makes
- [Borrowing](../../borrowing/README.md) — the rules a `ref` binding obeys once it exists
- [Reborrowing](../../reborrowing/README.md#the-trap-optionmut-t) — rustc suggesting `Some(ref x)` for an `Option<&mut T>`
- std: [keyword `ref` ↗](https://doc.rust-lang.org/std/keyword.ref.html) · the Reference: [identifier patterns ↗](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns) and [binding modes ↗](https://doc.rust-lang.org/reference/patterns.html#binding-modes) · [RFC 2005, match ergonomics ↗](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html) · [edition guide: Match ergonomics reservations ↗](https://doc.rust-lang.org/edition-guide/rust-2024/match-ergonomics.html)

## Po polsku

`ref` we wzorcu zmienia **sposób wiązania** nazwy, a nie to, co wzorzec dopasowuje: zamiast przeniesienia własności (*move*) dostajesz pożyczenie. `Some(n)` na `Option<String>` wyjmuje `String` z opcji i dalsze użycie `maybe_name` kończy się `E0382` (*partially moved*); `Some(ref n)` dopasowuje dokładnie te same wartości, a `n` ma typ `&String`. `&` we wzorcu działa odwrotnie — jest częścią dopasowania, wymaga referencji w wartości i ją zdejmuje, więc `Some(&x)` pasuje tylko do `Option<&T>`, a na `Option<String>` daje `E0308`.

`let ref z = x;` to `let z = &x;` (clippy podpowiada tę drugą formę). Nazwa zaczynająca się od `_` nadal wiąże i przenosi — nie przenosi tylko samo `_`. Jeden wzorzec może jedno pole przenieść, a drugie pożyczyć: `let Order { id, ref note } = order;` — potem `order.note` działa, a `order.id` i całe `order` już nie. `ref mut` pozwala pisać w dopasowane miejsce, pod warunkiem że zmienna jest `mut`.

Od Rusta 1.26 (*match ergonomics*) zwykle wystarczy pożyczyć samą wartość dopasowywaną (`match &maybe_name`), a w edycji 2024 `ref`, `&` albo `mut` wpisane tam, gdzie domyślny tryb wiązania już pożycza, to błąd kompilacji (w 2021 — ostrzeżenie `rust_2024_incompatible_pat` przy `-W rust-2024-compatibility`). Naturalnym zapisem `ref` pozostaje wzorzec, który jedno pole przenosi, a drugie pożycza, oraz `ref whole @ Some(ref inner)` na wartości posiadanej.

**Szukaj po polsku:** słowo kluczowe `ref` · wiązanie przez referencję · tryb wiązania we wzorcu · częściowe przeniesienie · `rust ref keyword` · `rust ref vs &` · `rust cannot explicitly borrow within an implicitly-borrowing pattern`
