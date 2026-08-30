# There is no `Move` trait

**Level:** 201 · working knowledge

**One line:** Moving is what every type does by default, so there is nothing to implement to make a type move — [`Copy`](../../16_Structs/copy_vs_clone/README.md) is the opt-out that *stops* it, which is why the compiler explains a move as an **absence**: *"does not implement the `Copy` trait."*

```rust
#[derive(Debug)]
struct Ballot(u8);      // Debug, and nothing else

let a = Ballot(5);
let b = a;              // moved — no trait was consulted
println!("{b:?}");      // Ballot(5)
```

`Ballot` implements one trait, and it is not the one that made this a move. Moving is the ground state.

---

## Going looking for the trait

```rust
fn needs_move<T: Move>(_x: T) {}
```

```text
error[E0405]: cannot find trait `Move` in this scope
 --> a.rs:1:18
  |
1 | fn needs_move<T: Move>(_x: T) {}
  |                  ^^^^ not found in this scope
```

[`std::marker` ↗](https://doc.rust-lang.org/std/marker/index.html) is where a trait like that would live, and it publishes `Copy`, `Send`, `Sync`, `Sized` and `Unpin`. There is no `Move`, in that module or anywhere else.

## Read the error as an absence

Every ownership error you will meet states the reason in the same shape:

```text
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
```

*Move occurs **because** … does **not** implement.* The move is the fallback, and the message names the trait that is missing rather than one that is present.

Restating that line as *"`String` implements `Move`"* is not a shorthand for the same fact — it inverts it. It makes moving a capability a type provides, and therefore makes `Copy` types the ones lacking something, when the arrangement is the other way round. The misreading is cheap to hold and expensive to use: sooner or later a type "won't move", you go looking for the trait to implement, and there is nothing to find because it already does.

## The opt-out is one derive, and it adds nothing to the value

Two structs, one byte each, differing only in that derive:

```rust
#[derive(Debug)]
struct Moves(u8);

#[derive(Debug, Clone, Copy)]
struct Copies(u8);
```

| | `let b = a;` does | `a` afterwards | `size_of` |
|---|---|---|---|
| `Moves` | a move | unusable — `E0382` | 1 |
| `Copies` | a copy | still usable | 1 |

`Copy` is a marker: no methods, no data, no code. It carries nothing into the type — it only changes what `=` means.

## What you may not opt out *with*

`Copy` is not yours to hand out freely. Two refusals worth meeting before you need them:

```text
error[E0277]: the trait bound `Bare: Clone` is not satisfied
    |
  2 | impl Copy for Bare {}
    |               ^^^^ the trait `Clone` is not implemented for `Bare`
note: required by a bound in `Copy`
    |
454 | pub trait Copy: Clone {
```

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; the type has a destructor
  |
2 | struct Loud;
  |        ^^^^ `Copy` not allowed on types with destructors
```

The second one is the rule underneath this whole page. `Copy` means *duplicating the bytes duplicates the value, completely*. A destructor is an obligation, and duplicating an obligation is the double free that ownership exists to prevent — so the types that own something are exactly the types that cannot opt out of moving. Not by policy: there is no opt-out for them to reach.

## `move` the keyword is a different thing again

```rust
let owned = String::from("captured by value");
let show = move || println!("{owned}");   // captured by value
show();
```

`move` on a closure answers *borrow or take?* for that closure's captures. It is a capture mode, not a bound — it cannot appear after a `:`, and it implements nothing. The word is shared; the mechanism is not.

## If you are coming from another language

- **C++.** This is where the idea comes from, and in C++ it is correct. Moving genuinely *is* a capability a type provides: a move constructor `T(T&&)` and a move assignment operator, requested by `std::move`, which is a cast asking for them. A class with neither silently falls back to the copy constructor — so *"is this type movable?"* is a real question with a per-type answer, and `= default`-ing the move is a real design decision. Rust deletes the question. Every type moves, there is no constructor to write and nothing to call, and the counterpart of `std::move` is plain `=`. What Rust gives in exchange is the guarantee on the far side: a C++ moved-from object still exists in a *valid but unspecified* state — readable, meaningless, and still destroyed at the end of its scope — while Rust's moved-from binding is gone. Reading it is `E0382` before the program runs, and no destructor fires for it, which is why Rust needs no "valid but unspecified" concept at all.
- **Python.** There is no counterpart, and the reason maps onto this page. `b = a` binds a second name to the same object; nothing is transferred, nothing becomes unusable, and there is no `__move__` to define because CPython's reference count already answers *when is this freed?*. The closest thing to a Rust move is `del a` written by hand on the next line — unenforced, and a convention rather than a rule. Coming the other way, the surprise is not that Rust moves; it is that the old name is *gone*, which Python never does to you.
- **ABAP.** Likewise no counterpart, from the opposite direction: `DATA(lt_copy) = lt_source` deep-copies the internal table every time, and the runtime handles the freeing, so a type never has to declare anything about being transferable. The question with no ABAP analogue is not *"how do I move this?"* but *"how do I avoid the copy?"* — in Rust `=` is already the cheap one, and `.clone()` is where you ask for ABAP's default and see the price at the call site.

All three land in the same place: the thing you were reasoning about per type — a move constructor, a refcount, a deep copy — is a property of the language here, and the only per-type decision left is the opt-out.

---

## The verified output

<!-- output:no_move_trait -->
*Verified output of [`no_move_trait.rs`](examples/no_move_trait.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── A plain type moves, and no trait was consulted
  b = Moves(5), and b.0 = 5
  `Moves` implements Debug and nothing else — and it still moved.
  Reading `a` now is error[E0382]: borrow of moved value.

──── `Copy` is the opt-out, and it changes what `=` means
  c = Copies(5), d = Copies(5)   both still usable (c.0 = 5)
  The only difference between the two types is the derive.

──── The opt-out added nothing to the value
  size_of::<Moves>()  = 1
  size_of::<Copies>() = 1
  Same bytes, same layout. `Copy` is a marker: it carries no data
  and no code, it only tells the compiler what `let d = c;` does.

──── `move` the KEYWORD is a capture mode, not a trait either
  closure says: captured by value
  `move` here answers 'borrow or take?' for the closure's captures.
  It is not a bound, cannot appear after a `:`, and implements nothing.

──── So the question 'is this type movable?' has no answer
  Every type is. The real question is whether it is `Copy`,
  and the compiler answers that one in the negative:
  "... which does not implement the `Copy` trait".
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/no_move_trait/examples/no_move_trait.rs -o /tmp/nmt && /tmp/nmt
```

## See also

- [Ownership and moves](../ownership_and_moves/README.md) — what a move *is*, once you have stopped looking for the trait: a transfer of responsibility, with the bytes staying put
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — the opt-out in full, and why a struct is never `Copy` by accident
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — the other half of the same confusion: `let b = a;` duplicates a `&str` because `&T` is `Copy`, not because a literal "lives on the stack"
- [`Copy` ↗](https://doc.rust-lang.org/std/marker/trait.Copy.html) and [E0382 ↗](https://doc.rust-lang.org/error_codes/E0382.html) — the trait, and the error that names its absence

## Po polsku

Częste pytanie kogoś, kto przyszedł z C++: „gdzie jest cecha (*trait*) `Move`?”. Nie ma jej i nie będzie. **Przenoszenie (*move*) jest zachowaniem domyślnym każdego typu w Ruscie** — nie ma czego implementować, żeby typ dało się przenieść.

To, co istnieje, to rezygnacja z przenoszenia: cecha `Copy`. Typ, który implementuje `Copy`, przy przypisaniu jest kopiowany bit po bicie zamiast przenoszony. Dlatego komunikat kompilatora opisuje przeniesienie jako **brak**: *„does not implement the `Copy` trait”*. Warto się z tym oswoić, bo to typowy sposób, w jaki `rustc` mówi o rzeczach domyślnych — nazywa nie to, co się stało, tylko czego zabrakło, żeby stało się coś innego.

Uwaga na pułapkę językową: słowo kluczowe `move` (jak w `move || { … }` przy domknięciach i wątkach) to **coś zupełnie innego** niż przeniesienie własności jako mechanizm. Słowo kluczowe każe domknięciu przejąć przechwycone zmienne na własność zamiast je pożyczyć. Po polsku obie rzeczy nazywa się „przeniesieniem”, więc w zdaniu trzeba dopowiedzieć, o które chodzi.

Nie da się też zaimplementować `Copy` dla dowolnego typu — typ, który posiada zasób (`String`, `Vec`, cokolwiek z `Drop`), tego nie dostanie, i to nie jest ograniczenie do obejścia, tylko cała gwarancja.

**Szukaj po polsku:** przenoszenie własności Rust · cecha Copy · `rust move semantics` · `rust Copy vs Clone` · `E0382`
