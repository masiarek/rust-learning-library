# Copy or move? Swap the value, keep the program

**Level:** 101 · for newcomers

**One line:** `let b = a;` never fails on its own. Whether `a` is still usable afterwards depends on one question about its type — is it [`Copy` ↗](https://doc.rust-lang.org/std/marker/trait.Copy.html)? — and a string literal is a `&str`, which is.

```rust
fn main() {
    let a = "hi";
    let b = a;
    println!("{a:?} {b:?}");  // "hi" "hi"
}
```

No error, because `"hi"` is not a `String`. It is a `&str`: a pointer and a length, aimed at text compiled into the program. Copying those two numbers gives a second view of the same text, and neither view owns anything that needs freeing.

---

## Swap the value, keep the program

Put each value into `let a = …;`, leave the other lines alone, and compile. The right-hand column is printed by [the example at the bottom of this page](#the-verified-output), which asks the compiler rather than a list.

| `let a = …;` | Type | After `let b = a;` |
|---|---|---|
| `"hi"` | `&str` | copy — `a` still usable |
| `5` | `i32` | copy |
| `3.5` | `f64` | copy |
| `'x'` | `char` | copy |
| `true` | `bool` | copy |
| `(1, "hi")` | `(i32, &str)` | copy |
| `[1, 2, 3]` | `[i32; 3]` | copy |
| `&owned`, an existing `String` | `&String` | copy |
| `String::from("hi")` | [`String` ↗](https://doc.rust-lang.org/std/string/struct.String.html) | **move** — `a` is dead |
| `vec![1, 2]` | [`Vec<i32>` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html) | **move** |
| `Box::new(5)` | [`Box<i32>` ↗](https://doc.rust-lang.org/std/boxed/struct.Box.html) | **move** |
| `&mut n` | `&mut i32` | **move** |
| `(1, String::from("hi"))` | `(i32, String)` | **move** |

## The rule the column follows

A type is `Copy` when duplicating its bytes duplicates the whole value and leaves nothing owned twice.

- **Copy:** numbers, `bool`, `char`, every shared reference `&T` whatever `T` is, and tuples and arrays whose elements are all `Copy`.
- **Move:** anything that owns something it must free — `String`, `Vec`, `Box` — and any tuple or array holding one. One `String` inside moves the whole tuple.
- **`&mut T` moves although it owns nothing.** It promises to be the only way to reach its target, and a second copy would break that promise. [Reborrowing](../reborrowing/README.md) is why passing one to a function still leaves it usable.

Every row copies `a`'s own bytes into `b`, the moving rows included — for a `String` that is the pointer, length and capacity, while the heap buffer is not touched. `Copy` decides only whether `a` may be used afterwards.

## The trap: "a string" is two types

[The Book's move example ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) is `let s1 = String::from("hello"); let s2 = s1;`, and earlier in the same section it uses a plain literal, `let s = "hello";`, to talk about scope. Retype the move example from memory with the literal and the error never comes: the value was a `&str` all along.

The names do not help. Both are "strings", both print the same, and both compare equal to `"hi"`. The difference is ownership, which is what [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) is about.

## Where the error lands

The `let b = a;` line compiles in every row. For a move, the refusal comes at the first *use* of `a` afterwards — here the `println!`, which borrows `a`, hence *borrow of* moved value:

```rust
fn main() {
    let a = String::from("hi");
    let b = a;
    // println!("{a:?} {b:?}");  // error[E0382] — transcript below
    println!("{b:?}");           // "hi"
}
```

With that line uncommented:

```text
error[E0382]: borrow of moved value: `a`
 --> main.rs:4:16
  |
2 |     let a = String::from("hi");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{a:?} {b:?}");  // error[E0382] — transcript below
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let b = a.clone();
  |              ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

The `help` names one way to keep both names. The other is a borrow:

```rust
let b = &a;          // one owner, a second name that only reads
let c = a.clone();   // two owners, each with its own buffer
```

Both are in the verified output below. [Borrowing](../borrowing/README.md) is the first to reach for, and [what a clone costs](../what_a_clone_costs/README.md) is the price of the second.

## How the example asks the compiler

The column is not typed in. The example binds each value at the type written beside it, so a wrong type is a compile error, and then asks this:

```rust
use std::marker::PhantomData;

struct Probe<T>(PhantomData<T>);

trait Moves {
    fn verdict(&self) -> &'static str { "move" }
}
impl<T> Moves for Probe<T> {}

impl<T: Copy> Probe<T> {
    fn verdict(&self) -> &'static str { "copy" }
}
```

Method lookup tries the inherent `verdict` before the trait's. Where `T: Copy` does not hold, the inherent one does not apply, and lookup falls through to the trait's default. It only works where the type is concrete: inside `fn f<T>(x: T)` nothing says `T: Copy`, so the probe answers `move` even for an `i32`, and the last section of the output shows it. It makes this table a measurement; it is not a technique for real code.

## If you are coming from another language

- **Python.** `b = a` never fails and never copies: it binds a second name to the same object, whatever the object is. Rust's `&str` row behaves the same way in practice — two views of one immutable text. The rows that differ are the owning ones. A Python list assigned to `b` is still one list, and appending through `b` changes what `a` sees; the Rust `Vec` row refuses to leave two names at all, unless you borrow (`&a`, shared and read-only) or clone (two separate vectors). Python never tells you whether an assignment shared something. Here the type decides, and the compiler reports it at the first use.
- **ABAP.** `lv_b = lv_a.` leaves both variables usable for every data type, a `STRING` or an internal table included, so ABAP has no moved-from state and no row of this table would fail there. Rust agrees with ABAP silently for the `Copy` rows. For `String` and `Vec` it transfers instead, and ABAP's default becomes `.clone()` — the same two usable copies, written at the call site where the cost can be seen.

## Practice

**Predict, then ask the compiler.** Write *copy* or *move* beside each of these before compiling anything:

```rust
let a: Option<i32> = Some(5);
let a: Option<String> = Some(String::from("hi"));
let a: &[i32] = &[1, 2, 3][..];
let a: std::ops::Range<i32> = 1..3;
let a: std::rc::Rc<i32> = std::rc::Rc::new(5);
let a: std::cmp::Ordering = std::cmp::Ordering::Less;
```

Then copy the probe above and run each one through it. One of the six holds nothing but two `i32`s and still moves — work out why before opening the solution.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:copy_or_move_kata -->
*[`copy_or_move_kata.rs`](examples/copy_or_move_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: predict copy or move, then ask the compiler. Each value is
//! bound at the type written beside it, so the type column is checked too.
//!
//!   rustc --edition 2024 copy_or_move_kata.rs -o /tmp/comk && /tmp/comk

use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Range;
use std::rc::Rc;

/// The probe from the page: the inherent `verdict` applies only where `T: Copy`.
struct Probe<T>(PhantomData<T>);

trait Moves {
    fn verdict(&self) -> &'static str {
        "move"
    }
}

impl<T> Moves for Probe<T> {}

impl<T: Copy> Probe<T> {
    fn verdict(&self) -> &'static str {
        "copy"
    }
}

fn probe<T>(_: &T) -> Probe<T> {
    Probe(PhantomData)
}

macro_rules! row {
    ($value:expr => $ty:ty) => {{
        let a: $ty = $value;
        println!("  {:<26} {:<16} {}", stringify!($value), stringify!($ty), probe(&a).verdict());
    }};
}

fn main() {
    println!("  {:<26} {:<16} {}", "let a = …;", "type", "let b = a;");
    row!(Some(5) => Option<i32>);
    row!(Some(String::from("hi")) => Option<String>);
    row!(&[1, 2, 3][..] => &[i32]);
    row!(1..3 => Range<i32>);
    row!(Rc::new(5) => Rc<i32>);
    row!(Ordering::Less => Ordering);

    println!();
    println!("Option<T> copies exactly when T does: the enum adds a tag, not an owner.");
    println!("&[i32] is a shared reference, so it copies like every &T.");
    println!("Rc<i32> moves: it has a Drop that lowers the count, and a type with a");
    println!("destructor can never be Copy. A second handle comes from Rc::clone.");

    println!();
    println!("Range<i32> is the surprise: two i32s, and it still moves, because it is");
    println!("an Iterator itself. If it were Copy, it.take(2) would take a silent copy,");
    println!("and `it` would not advance. Written out with clone(), that looks like this:");

    let mut it = 1..6;
    let first: Vec<i32> = it.by_ref().take(2).collect();
    let rest: Vec<i32> = it.collect();
    println!("  it.by_ref().take(2): first = {first:?}, rest = {rest:?}");

    let it = 1..6;
    let first: Vec<i32> = it.clone().take(2).collect();
    let rest: Vec<i32> = it.collect();
    println!("  it.clone().take(2):  first = {first:?}, rest = {rest:?}");

    println!();
    println!("RFC 3550 records that Copy was removed from every iterator in 2015. Its");
    println!("core::range::Range is Copy because it is only IntoIterator, but on 1.98");
    println!("it is nightly-only, and 1..3 still builds std::ops::Range.");
}
```
<!-- /source -->

<!-- output:copy_or_move_kata -->
*Verified output of [`copy_or_move_kata.rs`](examples/copy_or_move_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
  let a = …;                 type             let b = a;
  Some(5)                    Option<i32>      copy
  Some(String::from("hi"))   Option<String>   move
  &[1, 2, 3][..]             &[i32]           copy
  1..3                       Range<i32>       move
  Rc::new(5)                 Rc<i32>          move
  Ordering::Less             Ordering         copy

Option<T> copies exactly when T does: the enum adds a tag, not an owner.
&[i32] is a shared reference, so it copies like every &T.
Rc<i32> moves: it has a Drop that lowers the count, and a type with a
destructor can never be Copy. A second handle comes from Rc::clone.

Range<i32> is the surprise: two i32s, and it still moves, because it is
an Iterator itself. If it were Copy, it.take(2) would take a silent copy,
and `it` would not advance. Written out with clone(), that looks like this:
  it.by_ref().take(2): first = [1, 2], rest = [3, 4, 5]
  it.clone().take(2):  first = [1, 2], rest = [1, 2, 3, 4, 5]

RFC 3550 records that Copy was removed from every iterator in 2015. Its
core::range::Range is Copy because it is only IntoIterator, but on 1.98
it is nightly-only, and 1..3 still builds std::ops::Range.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:copy_or_move -->
*Verified output of [`copy_or_move.rs`](examples/copy_or_move.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── The program that was expected to fail
  a = "hi", b = "hi"   both still usable
  "hi" is a &str: a string, but not a String.

──── Swap the value, keep the program
  let a = …;                 type             let b = a;
  "hi"                       &str             copy
  5                          i32              copy
  3.5                        f64              copy
  'x'                        char             copy
  true                       bool             copy
  (1, "hi")                  (i32, &str)      copy
  [1, 2, 3]                  [i32; 3]         copy
  &owned                     &String          copy
  String::from("hi")         String           move
  vec![1, 2]                 Vec<i32>         move
  Box::new(5)                Box<i32>         move
  &mut n                     &mut i32         move
  (1, String::from("hi"))    (i32, String)    move

──── A move is refused at the next use, not at the let
  b = "hi"   only b is left

──── Two ways to keep both names when the type moves
  let b = &a;          a = "hi", b = "hi"
  let c = a.clone();   a = "hi", c = "hi"

──── The probe cannot see through a generic
  i32 at the call site:        copy
  i32 inside fn f<T>(x: T):    move
  Inside f nothing says T: Copy, so the fallback answers.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/copy_or_move/examples/copy_or_move.rs -o /tmp/com && /tmp/com
```

## See also

- [Ownership and moves](../ownership_and_moves/README.md) — what a move is: a transfer of responsibility for the free
- [There is no `Move` trait](../no_move_trait/README.md) — why the compiler explains a move as an absence, *"does not implement the `Copy` trait"*
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — opting your own struct in, and the three refusals that stop you
- [`Rc`: the clone that copies a pointer](../reference_counting/README.md) — the kata's `Rc` row: the cheapest clone there is, and still not `Copy`
- [E0382 ↗](https://doc.rust-lang.org/error_codes/E0382.html) — the error the moving rows produce

## Po polsku

Pytanie, od którego zaczyna się ta strona: dlaczego `let a = "hi"; let b = a;` **nie** daje błędu o przeniesionej wartości? Bo `"hi"` to nie `String`, tylko `&str` — referencja do tekstu zapisanego w samym programie. Referencje współdzielone implementują cechę `Copy`, więc przypisanie tworzy kopię i obie zmienne pozostają ważne. Błąd `E0382` pojawia się dopiero dla `String::from("hi")`, bo `String` jest **właścicielem** bufora na stercie, a dwie kopie oznaczałyby dwa zwolnienia tej samej pamięci.

Reguła do zapamiętania: `let b = a;` zawsze kopiuje bajty samej wartości, a typ decyduje tylko o tym, czy `a` wolno użyć później. Kopiują się liczby, `bool`, `char`, referencje `&T` oraz krotki i tablice złożone wyłącznie z takich typów. Przenoszą się `String`, `Vec`, `Box`, `&mut T` i wszystko, co któryś z nich zawiera.

Błąd nie pojawia się w linii przypisania, tylko przy pierwszym **użyciu** `a` po przeniesieniu — stąd „borrow of moved value”, bo `println!` zmienną pożycza.

**Szukaj po polsku:** cecha Copy w Ruscie · przeniesienie własności · `&str` a `String` · `rust copy vs move` · `E0382 borrow of moved value`
