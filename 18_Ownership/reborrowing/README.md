# Reborrowing

**Level:** 201 · working knowledge

**One line:** `&mut T` is not `Copy`, and yet you hand the same one to three calls in a row — because a call site quietly passes `&mut *r` instead of `r`.

```rust
fn bump(n: &mut i32) { *n += 1; }

fn main() {
    let mut total = 0;
    let r = &mut total;
    bump(r);
    bump(r);
    bump(r);
    println!("{}", *r);   // 3
}
```

Three calls, one `&mut`, no `.clone()` and no re-borrowing written down. If `&mut i32` were `Copy` this would be ordinary — but it is not, and the first `bump(r)` would have moved it.

## What actually got passed

Not `r`. Each call site inserts a **reborrow**: a fresh `&mut i32` pointing at the same place, valid only for the call. Written out, the three lines are `bump(&mut *r)` three times. `r` never moved, so it is still there afterwards.

A reborrow is a *nested* borrow, not a copy. While it is alive the original is frozen; when it ends, the original works again. That is the whole mechanism, and it is why the compiler can allow something that looks like duplicating an exclusive reference: at no instant do two usable `&mut` point at the same place.

## The `let` that does not do it

A reborrow is inserted where the compiler already knows what type it wants. A function parameter says so; a bare `let` says nothing, so it takes what it is given — and what it is given is a move:

```rust
let r = &mut total;
let r2 = r;        // MOVE — &mut i32 is not Copy
// bump(r);        // E0382
```

```text title="Abridged — real rustc output for reborrow_move.rs"
error[E0382]: borrow of moved value: `r`
 --> reborrow_move.rs:8:10
  |
5 |     let r = &mut total;
  |         - move occurs because `r` has type `&mut i32`, which does not implement the `Copy` trait
6 |     let r2 = r;
  |              - value moved here
...
8 |     bump(r);
  |          ^ value borrowed here after move
```

Two fixes, and the second one is worth knowing because it looks like it should change nothing:

```rust
let r2 = &mut *r;          // write the reborrow the call site would have written
let r3: &mut i32 = r;      // ...or just state the type, and it is inserted for you
```

Both are reborrows, and after either one's last use `r` works again. The annotation version is the whole rule in miniature: the same right-hand side moves without it and reborrows with it, because the expectation is what triggers the insertion.

## Where it happens for you, and where it does not

| Written | Passed | Original still usable |
|---|---|---|
| `bump(r)` | `&mut *r` | yes |
| `v.push(1)` on `v: &mut Vec<_>` | `&mut *v` | yes |
| `self.n += 1` in a `&mut self` method | `&mut *self.n` | yes |
| `let r2 = r;` | `r` | **no** — moved |
| `let r2: &mut i32 = r;` | `&mut *r` | yes |
| `let r2 = &mut *r;` | `&mut *r` | yes |

The method-receiver row is the load-bearing one. A `for` loop calling `vr.push(x)` on a `&mut Vec<i32>` would end after a single turn if the receiver moved.

## The trap: `Option<&mut T>`

A pattern match takes the reference *out* of the option, which is a move, not a reborrow:

```rust
let mut slot: Option<&mut i32> = Some(&mut n);
// if let Some(x) = slot { *x += 1; }   // compiles
// if let Some(x) = slot { *x += 1; }   // E0382 — the &mut left on the line above
```

```text title="Abridged — real rustc output for option_mut_move.rs"
error[E0382]: use of moved value
 --> option_mut_move.rs:5:17
  |
4 |     if let Some(x) = slot { *x += 1; }
  |                 - value moved here
5 |     if let Some(x) = slot { *x += 1; }
  |                 ^ value used here after move
  |
  = note: move occurs because value has type `&mut i32`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
4 |     if let Some(ref x) = slot { *x += 1; }
  |                 +++
```

`slot.as_deref_mut()` (or `slot.as_mut()`) hands back a fresh `Option<&mut i32>` borrowed from `slot`, which is the reborrow the call site would have inserted for you had this been a function argument. rustc's own `help:` suggests `ref x`, which is the same thing spelled in the pattern — see [Match ergonomics](../../30_Pattern_Matching/match_ergonomics/README.md).

## A `&mut` can hand out shared references too

`&*r` reborrows immutably, and you may have as many at once as you like — for as long as no mutable reborrow is live:

```rust
let a = &*r;
let b = &*r;      // fine: two shared reborrows of one &mut
```

This is the direction [coercion](../../29_Conversion/coercion/README.md) also travels: `&mut T` weakens to `&T`, never the reverse.

## If you are coming from another language

- **C++.** `T&` is copyable and can be passed to as many functions as you like at once, which is exactly the aliasing Rust is removing. The mental swap is that a Rust `&mut` behaves less like `T&` and more like a `unique_ptr` you are lending out: you can lend the loan, but not duplicate it. C++ references also cannot be reseated after initialisation; a Rust `&mut` **can** be reassigned (`r = &mut other;`), which is why it needs a move rule in the first place.
- **Python.** There are no exclusive references at all — every name is a shared, reseatable binding, and two names to one list is the normal case rather than an error. So the thing to port over is not the syntax but the question: in Python you find out about aliasing when a function you called mutates a list you thought was yours; here the compiler asks first. A reborrow is Rust's way of saying *"you may lend this on, but only one lender is live at a time"*, which Python has no way to express and no way to check.
- **ABAP.** The closest constructs are `FIELD-SYMBOLS` and `REF TO`, and both are unchecked in exactly the place this page is about. `ASSIGN wa-total TO <fs>` gives you a writable alias; nothing stops you assigning a second field-symbol to the same field and writing through both, and nothing tells you when the underlying row is freed — `UNASSIGN` and a stale `<fs>` after a `REFRESH` are a familiar class of dump. A reborrow is the compile-time version of the discipline you would otherwise keep in your head: while `<fs2>` is live, do not touch `<fs1>`.

## The verified output

<!-- output:reborrowing -->
*Verified output of [`reborrowing.rs`](examples/reborrowing.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Passing a `&mut` to a function does not move it
   bump(r) three times       -> *r = 3
   each call passed `&mut *r`, inserted for you — `r` itself stayed put

2. A bare `let` moves it — unless the line says what type it wants
   let a: &mut i32 = r       -> *a = 4
   let inner = &mut *r       -> *inner = 5
   ...and r works again      -> *r = 6

3. A shared reborrow: one `&mut` hands out as many `&` as you like
   &*r twice                 -> 6 and 6

4. A method call reborrows its receiver, which is the only reason a loop works
   sr.push() three times     -> [1, 2, 3, 4, 5, 6]

5. `&mut self` is the same insertion, one level in
   c.bump() twice            -> score = 12

6. The trap: `Option<&mut T>` matched by value moves the reference OUT
   slot.as_deref_mut() twice -> Some(7)
   `if let Some(x) = slot` compiles once, then E0382 on the second line
```
<!-- /output -->

## See also

- [Borrowing](../borrowing/README.md) — the rule a reborrow is nested inside: one `&mut`, or any number of `&`
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — why `&T` is `Copy` and `&mut T` is not, which is the fact this page works around
- [Match ergonomics](../../30_Pattern_Matching/match_ergonomics/README.md) — `Some(ref x)` and the default binding mode, the pattern-side spelling of the same idea
- [Coercion](../../29_Conversion/coercion/README.md) — the other thing a call site inserts silently
- [Lifetime annotations](../lifetime_annotations/README.md) — a reborrow is shorter-lived than what it borrows from, which is what `'a` is naming

## Sources

The Reference on [method-call expressions ↗](https://doc.rust-lang.org/reference/expressions/method-call-expr.html) and [borrow expressions ↗](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-operators); [`Option::as_deref_mut` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref_mut). Both `E0382` transcripts are real compiles of the files they name.

## Po polsku

Reguła jest prosta i przez to myląca: `&mut T` **nie** jest `Copy`, a mimo to ten sam `r` można podać do trzech wywołań pod rząd. Dzieje się tak, ponieważ w miejscu wywołania kompilator wstawia **ponowne pożyczenie** (*reborrow*) — zamiast `r` przekazuje `&mut *r`, czyli świeżą referencję wskazującą na to samo miejsce, ważną tylko na czas tego wywołania. Oryginał nigdzie się nie przenosi, więc po powrocie z funkcji nadal działa. To nie jest kopia, tylko **pożyczenie zagnieżdżone**: dopóki żyje ta wewnętrzna referencja, zewnętrzna jest zamrożona, a gdy wewnętrzna kończy się (przy ostatnim użyciu), zewnętrzna wraca do gry. Dlatego w żadnym momencie nie istnieją dwie używalne referencje wyłączne do tej samej pamięci — i dlatego kompilator może na to pozwolić.

Miejsce, w którym to *nie* działa samo, to zwykłe `let`. `let r2 = r;` **przenosi** własność referencji i kolejne `bump(r)` kończy się `E0382`; poprawka to jeden zapis, ten sam, który wstawiłoby wywołanie: `let r2 = &mut *r;`. Ta sama pułapka wraca w `Option<&mut T>` — dopasowanie wzorcem wyjmuje referencję *na zewnątrz*, więc drugie `if let Some(x) = slot` już się nie kompiluje. Rozwiązaniem jest `slot.as_deref_mut()` albo `slot.as_mut()`, czyli poproszenie o nowe pożyczenie zamiast zabrania oryginału. Sam kompilator podpowiada tu `Some(ref x)`, co jest tym samym zapisanym po stronie wzorca.

Dla czytelnika znającego ABAP najbliższe odpowiedniki to `FIELD-SYMBOLS` i `REF TO` — z tą różnicą, że tam nikt niczego nie sprawdza. Nic nie stoi na przeszkodzie, by przypisać dwa symbole pola do tego samego pola i pisać przez oba, i nic nie ostrzeże, gdy wiersz pod spodem zniknie po `REFRESH`. Ponowne pożyczenie to dokładnie ta dyscyplina, tyle że wymuszona przy kompilacji.

**Szukaj po polsku:** ponowne pożyczenie · referencja wyłączna · przeniesienie referencji · `rust reborrow` · `rust E0382 &mut is not Copy`
