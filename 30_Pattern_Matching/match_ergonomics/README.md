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
