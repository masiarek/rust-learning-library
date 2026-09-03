# Where the `&` sits decides what it does

**Level:** 101 → 201 · for newcomers

**One line:** `&` and `*` each do a different job in a **type**, in an **expression** and in a **pattern** — so `let r: &S = &a;` holds two unrelated `&`, `let &x = r;` *removes* a reference instead of making one, and the `*` in `*const S` never dereferences anything.

```rust
let a = 1;
let r: &i32 = &a;               // `&i32` is a type; `&a` is an operator
let &back = r;                  // in a pattern, `&` strips the reference off
println!("{a} {} {back}", *r);  // 1 1 1
```

Three `&` in three lines, doing three different things. No error message will point this out, because the compiler assumes you can already read the syntax — every diagnostic you get about references is written on top of it.

---

## The whole rule, in one table

| the character sits in a… | `&` | `*` |
|---|---|---|
| **type** — after a `:`, in a signature, inside another type | `&S`, `&mut S` — the *type* of a reference to an `S` | `*const S`, `*mut S` — the *type* of a raw pointer; two words, one name |
| **expression** — right of an `=`, an argument, a return value | `&a` — take a reference to `a` | `*r` — the place `r` points at |
| **pattern** — left of an `=`, a `match` arm, a closure parameter, a `for` binding | `&x` — match a reference and bind what is *behind* it | not pattern syntax; there is no `*` here |

Read `let r: &S = &a;` through that table and the two characters come apart: the first sits in a type, the second in an expression. Read `let &back = r;` and the third one is doing the opposite job to the second, because the left of a `let` is a pattern.

The pattern row is the one that surprises people, and it is everywhere:

```rust
let scores = [1, 2, 4];
let total: i32 = scores.iter().map(|&n| n).sum();  // 7
```

`scores.iter()` yields `&i32`. The `&` in `|&n|` is not making another reference — it is taking one apart, so `n` is an `i32` and the arithmetic works. `.copied()` is the same thing spelled as a method.

## `*r` is a place, not a value

`*r` does not mean *"the value inside `r`"*. It names **the place `r` points at**, which is why the same three characters behave differently depending on which side of the `=` they land:

```rust
let mut b = S(1);
let w = &mut b;

let before = w.clone();  // RIGHT of `=` — read the place
*w = S(2);               // LEFT  of `=` — write the place; `b` itself changes
```

Everything a `&mut` lets you do is a consequence of that, and so is the one thing it refuses:

| through `let w = &mut b;` | allowed? | |
|---|---|---|
| write the whole place — `*w = S(2);` | yes | the old value is dropped first, on that line |
| mutate part of it — `w.0 = 2;` | yes | `b` still holds a valid `S` |
| read a copy out — `let x = *w;` | yes, **if `S: Copy`** | `b` keeps its value |
| read a clone out — `let x = w.clone();` | yes, if `S: Clone` | the clone is a second value |
| move it out — `let x = *w;` | **no** | `b` would be left empty, and its owner would never find out |

Rows three and five are the same line of code. Which one you get is decided entirely by whether the type is [`Copy`](../../16_Structs/copy_vs_clone/README.md) — the syntax cannot tell you, and this is the single most common reason a `*` that worked yesterday stops working today.

## The read that is refused

```rust
// move_out.rs
let mut b = S(1);
let w = &mut b;
// let taken = *w;   // S is not Copy
```

```text title="Abridged — real rustc output for move_out.rs, help blocks trimmed"
error[E0507]: cannot move out of `*w` which is behind a mutable reference
 --> move_out.rs:7:17
  |
7 |     let taken = *w;
  |                 ^^ move occurs because `*w` has type `S`, which does not implement the `Copy` trait
```

A borrow has to hand the place back **valid** — not unchanged, just valid, and holding the same type. Writing to it is fine because a new value arrives in the same statement. Taking from it is not, because nothing arrives.

Which is exactly how the escape hatch works: put something back in the same breath.

```rust
let old = std::mem::replace(w, S(8));  // hands you the old value, leaves S(8) behind
```

[`replace` ↗](https://doc.rust-lang.org/std/mem/fn.replace.html), [`take` ↗](https://doc.rust-lang.org/std/mem/fn.take.html) (which needs `Default`) and [`swap` ↗](https://doc.rust-lang.org/std/mem/fn.swap.html) are safe for one reason: the hole is filled before anybody can look into it.

## The `*` that never dereferences

```rust
let p: *const S = &a;
```

There is no dereference on that line. `*const S` and `*mut S` are the *names* of the raw-pointer types, spelled with two words — the `*` belongs to the type the way `&` does in `&S`. The variable `p` holds the same number `r` held; what changed is everything the compiler is prepared to promise about it:

| | `&S` | `*const S` |
|---|---|---|
| what it stores | an address | the same address |
| target is alive | guaranteed, at compile time | no guarantee |
| target is a valid `S` | guaranteed | no guarantee |
| can be null | no | yes |
| aliasing rules enforced | yes | no |
| reading it | `*p` is safe | `*p` needs `unsafe` |

A raw pointer is a reference with the promises subtracted. `unsafe` does not switch checks off — there were never any checks on this address to switch off — it marks the line where **you** make the promise instead. [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) has the full list of five powers, and the reason making a raw pointer is safe while using one is not.

## The sigils you do not write

Rust inserts some for you. `r.0` reads a field through a reference with no `*` anywhere, and `w.clone()` finds `S::clone` by dereferencing `w` first. Both are conveniences, and one of them has a sharp edge:

```rust
// clone_a_reference.rs
fn snapshot<T>(r: &T) -> T {
    r.clone()          // no `T: Clone` bound in scope
}
```

```text title="Abridged — real rustc output for clone_a_reference.rs, help block trimmed"
error[E0308]: mismatched types
 --> clone_a_reference.rs:2:5
  |
2 |     r.clone()
  |     ^^^^^^^^^ expected type parameter `T`, found `&T`
  |
note: `T` does not implement `Clone`, so `&T` was cloned instead
```

When the compiler can see `S: Clone`, `w.clone()` derefs and clones the target — which is what you wanted. When it cannot, `&T` is itself `Clone`, so the call still compiles and quietly clones **the reference**. Modern `rustc` names it in a `note:`; for years it did not. The same fact bites on `&str`, for a different reason — `str` is not `Sized`, so it has no `Clone` impl, which is [why `ToOwned` exists](../../12_Traits/marker_traits/README.md).

## If you are coming from another language

- **Python.** There is no `&` because every name is already a reference: `r = a` gives you two names for one object, and that is the *only* thing binding does. The gap is on the other side. `*r = S(2)` has no Python spelling at all — `r = S(2)` re-points the name `r` and leaves `a` untouched, so writing *through* an alias needs an in-place mutation instead: `r.x = 2`, `r[:] = [...]`, `r.__dict__.update(...)`. Python cannot distinguish *point somewhere else* from *write to where you point*, because it only has the first; Rust makes you spell which one you mean, `r = &b;` against `*r = b;`, and the borrow checker's whole job is the second. Moving out has no counterpart either — CPython refcounts, so an object is never left as a hole, which is why the refusal above reads as arbitrary until you remember that a Rust `S` is a value somebody owns rather than a thing on the heap everyone points at.
- **ABAP.** This is the closest of the three, and the vocabulary lines up almost item for item. `DATA r TYPE REF TO ty` is the type position; `GET REFERENCE OF a INTO r` is `&mut a`; and `r->*` is `*r` exactly — `d = r->*` reads the place, `r->* = v` writes it, same left/right split as the table above. Field symbols are closer still: after `FIELD-SYMBOLS <r> TYPE ty. ASSIGN a TO <r>.` the symbol *is* `a`'s storage, so `<r> = v.` changes `a` with no dereference operator anywhere — which is what Rust's auto-deref is doing when `w.0 = 2` works. Two real differences to carry across. `ASSIGN` will point a field symbol at almost anything and re-point it later, where a Rust `&mut` locks the owner out for as long as it lives — that lock is the part with no ABAP equivalent. And the dangling case is caught at a different time: a field symbol whose target has gone out of scope dumps **at run time**, on the statement that touches it, while Rust proves at compile time that a `&` cannot dangle and, for `*const S`, declines to check at all and makes you write `unsafe`. Same hazard, three different places to pay for it.

---

## The verified output

<!-- output:where_the_sigil_sits -->
*Verified output of [`where_the_sigil_sits.rs`](examples/where_the_sigil_sits.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── One line, two different `&`
  let r: &S = &a;
  the first `&` named a TYPE, the second took an ADDRESS
  a = S(1), *r = S(1)
  and field access needs no `*` at all: r.0 = 1

──── In a pattern, `&` REMOVES a reference
  let &copied = rc;       copied = C(5)   (a C, not a &C)
  .map(|&C(n)| n).sum()   total  = 7

──── `*r` on the right READS the place; on the left it WRITES it
  before the write:  S(1)
  b afterwards:      S(2)   (b itself changed)

──── Reading a place OUT is a copy, or a move
  C is Copy:      let taken = *rd;  -> C(9), d still C(9)
  S is not Copy:  mem::replace(..)  -> S(7), e now  S(8)

──── The `*` in `*const S` is not a dereference
  `*const S` is the NAME of a type; nothing was dereferenced.
  the one `*` that does dereference: unsafe { &*p } = S(1)
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/where_the_sigil_sits/examples/where_the_sigil_sits.rs -o /tmp/wtss && /tmp/wtss
```

## See also

- [Borrowing: `&T`, `&mut T`, and where a borrow ends](../borrowing/README.md) — the rules these characters obey, once you can read them
- [What an address shows](../what_an_address_shows/README.md) — what the number in `r` actually points at, which for a `String` is not the text
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — the trait that decides whether `let x = *w;` is a copy or `E0507`
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — raw pointers in full: five powers, and `E0133`
- [Interior mutability](../../09_Advanced/interior_mutability/README.md) — the one way to write through a `&T`, once the table above has told you that you cannot
- [E0507 ↗](https://doc.rust-lang.org/error_codes/E0507.html) and [`std::mem::replace` ↗](https://doc.rust-lang.org/std/mem/fn.replace.html) — the refusal, and the way round it

## Po polsku

Ta strona jest o **czytaniu składni**, a nie o pożyczaniu. Znaki `&` i `*` robią w Ruscie co innego w zależności od tego, gdzie stoją — a to jest jedyna rzecz, której kompilator nigdy nie wytłumaczy, bo zakłada, że składnię już umiesz czytać.

Trzy pozycje, trzy znaczenia:

| znak stoi w… | `&` | `*` |
|---|---|---|
| **typie** (po `:`, w sygnaturze) | `&S`, `&mut S` — *typ* referencji do `S` | `*const S`, `*mut S` — *nazwa* typu surowego wskaźnika |
| **wyrażeniu** (po prawej stronie `=`) | `&a` — weź referencję do `a` | `*r` — **miejsce**, na które wskazuje `r` |
| **wzorcu** (po lewej stronie `=`, w `match`, w `for`) | `&x` — dopasuj referencję i zwiąż to, co jest *pod* nią | nie występuje |

Stąd `let r: &S = &a;` zawiera dwa różne `&`: pierwsze to typ, drugie to operator. A `let &back = r;` **zdejmuje** referencję, zamiast ją tworzyć — bo lewa strona `let` to wzorzec. To samo dzieje się w `.map(|&n| n)`.

Najważniejsze zdanie o `*r`: to nie jest „wartość w `r`”, tylko **miejsce**, na które `r` wskazuje. Po lewej stronie `=` zapisujesz to miejsce (`*w = S(2)` zmienia samo `b`), po prawej je czytasz. I dlatego jedna linijka `let x = *w;` raz się kompiluje, a raz nie: dla typu z `Copy` to skopiowanie, dla pozostałych to **przeniesienie własności**, po którym `b` zostałoby puste — `error[E0507]: cannot move out of ... which is behind a mutable reference`. Wyjściem jest `std::mem::replace`, które w tej samej instrukcji wstawia coś w to miejsce.

Uwaga terminologiczna: `*const S` i `*mut S` to **dwuwyrazowe nazwy typów**, a nie wyłuskanie. Surowy wskaźnik to referencja z odjętymi gwarancjami — może być pusty, może wskazywać na nieżyjącą wartość, nie podlega regułom pożyczania — i dlatego jego odczyt wymaga `unsafe`. Samo utworzenie wskaźnika jest bezpieczne; dopiero `*p` jest obietnicą.

Dla osób z ABAP-a: `TYPE REF TO` to pozycja typu, `GET REFERENCE OF a INTO r` to `&mut a`, a `r->*` to dokładnie `*r`. Symbole pola (`FIELD-SYMBOLS <r>`, `ASSIGN a TO <r>`) są jeszcze bliżej, bo `<r>` *jest* pamięcią zmiennej `a`. Dwie różnice: `&mut` blokuje właściciela na czas swojego życia (`ASSIGN` niczego nie blokuje), a wiszący symbol pola wywala program **w czasie wykonania**, podczas gdy Rust dowodzi tego samego w czasie kompilacji.

**Szukaj po polsku:** referencje w Ruscie · wyłuskanie referencji · wzorce z `&` · surowe wskaźniki · `rust reference vs pointer` · `rust E0507` · `rust deref pattern`
