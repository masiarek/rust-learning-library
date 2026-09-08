# Match ergonomics

**Level:** 201 · working knowledge

**One line:** Match a reference against a pattern that is not a reference, and every binding inside becomes a reference — which is why `Some(name)` on a `&Option<String>` hands you a `&String` instead of moving the `String` out.

```rust
fn main() {
    let ballot: Option<String> = Some("Ada".to_string());

    if let Some(name) = &ballot {
        println!("{name}");        // Ada        <- name is a &String
    }

    println!("{ballot:?}");        // Some("Ada")  <- still owned, nothing moved
}
```

`Some(name)` is a pattern for an `Option<String>`, and the scrutinee is a `&Option<String>`. The types do not line up, and rather than refuse, the compiler steps through the reference and switches to a **default binding mode** of `ref` — so `name` binds as `&String`.

## The two modes

The mode starts at *move* and changes the moment a non-reference pattern meets a reference:

| Scrutinee | Pattern | Binding mode | `name` is |
|---|---|---|---|
| `Option<String>` | `Some(name)` | move | `String` — moved out |
| `&Option<String>` | `Some(name)` | `ref` | `&String` |
| `&mut Option<String>` | `Some(name)` | `ref mut` | `&mut String` |

Once entered, the mode applies all the way down. `for (name, score) in &rows` over a `Vec<(String, u8)>` gives `&String` and `&u8` from a pattern with no `&` in it anywhere.

## What it replaced

Before [RFC 2005 ↗](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html) landed in Rust 1.26, you wrote the mode into the pattern by hand:

```rust
if let Some(ref name) = ballot { }        // name: &String
if let Some(ref mut n) = tally { }        // n: &mut u32
```

Both still compile and still mean exactly that. You will meet `ref` in older code and in rustc's own `help:` lines, which is the main reason to be able to read it.

## The move it saves you from

Drop the `&` and the pattern moves the `String` out of the option:

```rust
// if let Some(name) = ballot { println!("{name}"); }
// println!("{ballot:?}");   // E0382
```

```text title="Abridged — real rustc output for ergonomics_move.rs"
error[E0382]: borrow of partially moved value: `ballot`
 --> ergonomics_move.rs:6:16
  |
3 |     if let Some(name) = ballot {
  |                 ---- value partially moved here
...
6 |     println!("{ballot:?}");
  |                ^^^^^^ value borrowed here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
3 |     if let Some(ref name) = ballot {
  |                 +++
```

Two fixes, and they are the same fix: put the `&` on the scrutinee (`= &ballot`) and let ergonomics do it, or put `ref` on the binding and do it by hand. The `Option<&mut T>` version of this error is the trap on [Reborrowing](../../18_Ownership/reborrowing/README.md).

## Turning it back off

An `&` in the *pattern* destructures a reference rather than binding one, which is how you copy a value out:

```rust
let total: u32 = scores.iter().map(|&n| u32::from(n)).sum();   // n: u8
```

But that only works where the place really is a reference. Under a default binding mode the sub-places are values, so an `&` there is a type error:

```text title="Abridged — real rustc output for ergonomics_mixed.rs"
error[E0308]: mismatched types
 --> ergonomics_mixed.rs:3:10
  |
3 |     let (&a, b) = pair;
  |          ^^       ---- this expression has type `&(u8, u8)`
  |          |
  |          expected `u8`, found `&_`
  |
help: consider removing `&` from the pattern
```

Read the note carefully: the tuple's field is `u8`, not `&u8`, *because* the mode already handled the outer reference. Pick one mechanism per pattern.

## The trap: the extra `&` you did not ask for

Closure parameters are where this bites, because the adapter has already added one reference and ergonomics adds nothing:

```rust
scores.iter().filter(|n| **n > 3)      // n: &&u8 — iter() gave &u8, filter() gave &that
scores.iter().filter(|&&n| n > 3)      // the same thing, destructured instead
```

`**n` is not a code smell and not a sign you did something wrong. It is two references, both of which are real, and the `&&n` spelling is the one that says so in the signature instead of at the comparison.

## If you are coming from another language

- **Python.** Structural pattern matching (`match`/`case`, 3.10+) looks similar and has no counterpart to any of this: Python binds names to objects and there is no move, no borrow, and no mode to be in. The idea worth carrying is inverted — in Python you *cannot* accidentally move a value out of a container by unpacking it, so the Rust habit to build is noticing that `Some(name)` and `Some(ref name)` differ, where `a, b = pair` never did.
- **C++.** Structured bindings are the near neighbour, and they have the same fork: `auto [a, b] = pair;` copies, `auto& [a, b] = pair;` binds references. Rust moves the decision from the binding site to the *scrutinee*, so `&ballot` in the pattern position is doing what `auto&` does in C++ — and the compiler then propagates it through every nested field rather than making you repeat it.
- **ABAP.** `LOOP AT itab INTO wa` copies each row; `LOOP AT itab ASSIGNING <fs>` gives you a writable alias into the table itself. That is exactly the `Some(name)` versus `Some(ref mut name)` distinction, including the performance reason for preferring the second on wide rows. What Rust adds is that the copy version cannot silently leave the original in a half-moved state — the `INTO` equivalent is refused when the row is not `Copy` and you still need the table afterwards.

## Practice

**One character apart.** Match `&opt` against `Some(name)` where `opt: Option<String>`, and say what type `name` is and whether `opt` survives. Then match `opt` itself with the same pattern and answer both again.

Then the surprise: destructure a `&Option<(String, u32)>` and say what type the `u32` binding has. Finish by naming the rule that predicts all three answers — it is about the scrutinee, not the pattern.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:match_ergonomics_kata -->
*[`match_ergonomics_kata.rs`](examples/match_ergonomics_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: match a reference against a non-reference pattern.
//!
//!   rustc --edition 2024 match_ergonomics_kata.rs -o /tmp/mek && /tmp/mek

fn main() {
    let opt: Option<String> = Some(String::from("hello"));

    println!("1. MATCHING A REFERENCE WITH AN ORDINARY PATTERN");
    match &opt {
        Some(name) => println!("  name is a &String: {name:?}, len {}", name.len()),
        None => println!("  nothing"),
    }
    println!("  `opt` is still ours afterwards: {opt:?}");
    println!();
    println!("  The scrutinee was `&Option<String>` and the pattern was");
    println!("  `Some(name)`, which is not a reference pattern. Rather than");
    println!("  refusing, the compiler DEREFERENCED the scrutinee and made every");
    println!("  binding inside a reference -- so `name: &String`, and nothing");
    println!("  moved out of `opt`. That is match ergonomics, RFC 2005.");
    println!();

    println!("2. WHAT IT SAVED YOU WRITING");
    match &opt {
        &Some(ref name) => println!("  the pre-2018 spelling: {name:?}"),
        &None => println!("  nothing"),
    }
    println!("  Both forms still compile and mean the same thing. The second is");
    println!("  what everyone had to write before, and it is why old code is");
    println!("  full of `ref`.");
    println!();

    println!("3. THE CASE THAT SURPRISES PEOPLE");
    let pair: Option<(String, u32)> = Some((String::from("k"), 1));
    if let Some((key, n)) = &pair {
        println!("  key: {key:?} is a &String, n: {n} is a &u32");
        println!("  n + 1 = {}", *n + 1);
    }
    println!("  Every binding became a reference, including the u32 -- which is");
    println!("  Copy, so people expect a value and get `&u32`. It usually still");
    println!("  works because of auto-deref in arithmetic and method calls, and");
    println!("  then fails the one time you need it by value.");
    println!();

    println!("4. WHEN YOU DO WANT TO MOVE");
    let taken = match opt {
        Some(name) => name,
        None => String::new(),
    };
    println!("  matching the VALUE (no &) moves it out: {taken:?}");
    println!("  ...and `opt` is no longer usable. Which one you get is decided");
    println!("  entirely by whether the scrutinee is a reference -- so `match");
    println!("  opt` and `match &opt` are two different programs, one character");
    println!("  apart.");
    println!();

    println!("THE RULE TO CARRY");
    println!("  Ask what the SCRUTINEE is, not what the pattern looks like. A");
    println!("  reference scrutinee gives reference bindings, all the way down,");
    println!("  and `as_ref()` is the explicit spelling when you want that from");
    println!("  an owned value.");

    let o2: Option<String> = Some(String::from("x"));
    let len = match &o2 { Some(s) => s.len(), None => 0 };
    assert_eq!(len, 1);
    assert!(o2.is_some());
}
```
<!-- /source -->

<!-- output:match_ergonomics_kata -->
*Verified output of [`match_ergonomics_kata.rs`](examples/match_ergonomics_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. MATCHING A REFERENCE WITH AN ORDINARY PATTERN
  name is a &String: "hello", len 5
  `opt` is still ours afterwards: Some("hello")

  The scrutinee was `&Option<String>` and the pattern was
  `Some(name)`, which is not a reference pattern. Rather than
  refusing, the compiler DEREFERENCED the scrutinee and made every
  binding inside a reference -- so `name: &String`, and nothing
  moved out of `opt`. That is match ergonomics, RFC 2005.

2. WHAT IT SAVED YOU WRITING
  the pre-2018 spelling: "hello"
  Both forms still compile and mean the same thing. The second is
  what everyone had to write before, and it is why old code is
  full of `ref`.

3. THE CASE THAT SURPRISES PEOPLE
  key: "k" is a &String, n: 1 is a &u32
  n + 1 = 2
  Every binding became a reference, including the u32 -- which is
  Copy, so people expect a value and get `&u32`. It usually still
  works because of auto-deref in arithmetic and method calls, and
  then fails the one time you need it by value.

4. WHEN YOU DO WANT TO MOVE
  matching the VALUE (no &) moves it out: "hello"
  ...and `opt` is no longer usable. Which one you get is decided
  entirely by whether the scrutinee is a reference -- so `match
  opt` and `match &opt` are two different programs, one character
  apart.

THE RULE TO CARRY
  Ask what the SCRUTINEE is, not what the pattern looks like. A
  reference scrutinee gives reference bindings, all the way down,
  and `as_ref()` is the explicit spelling when you want that from
  an owned value.
```
<!-- /output -->

</details>

## The verified output

<!-- output:match_ergonomics -->
*Verified output of [`match_ergonomics.rs`](examples/match_ergonomics.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A `&` on the scrutinee makes every binding a `&`
   name: &String         -> Ada, 3 bytes
   ballot survived       -> Some("Ada")

2. The spelling it replaced, which still works and still means this
   Some(ref name)        -> Ada

3. `&mut` on the scrutinee gives `&mut` bindings, so you can write through them
   after `*n += 1`       -> Some(8)

4. It goes all the way down a nested pattern
   for (name, score)     -> Ada leads with 5
   rows survived         -> 2 still owned

5. Writing `&` in the PATTERN turns it back off — that destructures
   |&n| copies n out     -> total 16

6. The trap: a binding you think is `u8` is `&&u8` inside a closure
   filter(|n| **n > 3)   -> [9, 4]
   filter(|&&n| n > 3)   -> [9, 4]   (the same list, spelled the other way)
```
<!-- /output -->

## See also

- [Destructuring enums](../destructuring_enums/README.md) — the patterns this changes the binding mode of
- [Irrefutable patterns](../irrefutable_patterns/README.md) — where a `let` pattern is allowed to be one
- [Reborrowing](../../18_Ownership/reborrowing/README.md) — the `Option<&mut T>` move, the same trap one type up
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — why the move only bites on non-`Copy` payloads
- [Coercion](../../29_Conversion/coercion/README.md) — the other silent insertion, on the expression side

## Sources

[RFC 2005 — match ergonomics ↗](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html), and the Reference on [binding modes ↗](https://doc.rust-lang.org/reference/patterns.html#binding-modes). Both transcripts are real compiles of the files they name, under `--edition 2024`; the `E0308` one is identical under 2021.

## Po polsku

Reguła w jednym zdaniu: gdy wzorzec, który **nie** jest referencją, spotyka wartość, która nią jest, kompilator przechodzi przez referencję i przełącza *domyślny tryb wiązania* (*default binding mode*) na `ref`. Dlatego `if let Some(name) = &ballot` daje `name` typu `&String`, a `ballot` pozostaje nietknięty. Ten sam zapis bez `&` przenosi `String` na zewnątrz i kolejne użycie `ballot` kończy się `E0382` — z podpowiedzią `Some(ref name)`, czyli starszym zapisem tego samego. Tryb raz włączony obowiązuje w głąb: `for (name, score) in &rows` po `Vec<(String, u8)>` daje `&String` i `&u8`, choć we wzorcu nie ma ani jednego `&`.

Odwrotny ruch to `&` **we wzorcu**, które nie wiąże referencji, tylko ją rozbiera — stąd `map(|&n| ...)`, gdzie `n` jest już wartością. Działa to jednak wyłącznie tam, gdzie w danym miejscu naprawdę stoi referencja. Jeśli tryb domyślny już zadziałał, pola w środku są zwykłymi wartościami i `&` daje `E0308` z komunikatem *expected `u8`, found `&_`*. Zasada praktyczna brzmi więc: w jednym wzorcu jeden mechanizm.

Czytelnikowi znającemu ABAP najszybciej wyjaśnia to `LOOP AT`: `INTO wa` kopiuje wiersz, `ASSIGNING <fs>` daje zapisywalny alias do tabeli. To dokładnie różnica między `Some(name)` a `Some(ref mut name)`, łącznie z powodem wydajnościowym. Różnica polega na tym, że w Ruscie wariant „kopiujący” zostanie po prostu odrzucony, jeśli typ nie jest `Copy`, a oryginał ma być używany dalej — zamiast zostawić strukturę w stanie częściowo przeniesionym.

**Szukaj po polsku:** domyślny tryb wiązania · dopasowanie wzorca do referencji · `ref` we wzorcu · `rust match ergonomics` · `rust E0382 partially moved value`
