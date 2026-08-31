# The unit type `()`

**Level:** 101 → 201 · for newcomers

**One line:** `()` is the empty tuple — one value, zero bytes — and it is the type of *"the job is done and there is nothing to hand back"*, which is why it turns up in error messages long before anyone writes it on purpose.

```rust
fn record(score: u8) -> Result<(), String> {
    if score <= 5 { Ok(()) } else { Err(format!("{score} is out of range")) }
}
let mut names = vec!["Cara", "Ada"];
names.sort();                       // returns (), sorts in place
println!("{:?} {:?}", record(5), names);   // Ok(()) ["Ada", "Cara"]
```

## One value, zero bytes

`()` is both a **type** and the single **value** of that type, written the same way. That is not a shortcut — a type with exactly one value carries no information, so there is nothing to store:

```text
size_of::<()>()         = 0
size_of::<[(); 1000]>() = 0    <- a thousand of them still occupy nothing
() == ()                = true <- one value, so equality is always true
```

It is the [empty tuple](../../26_Collections/tuples/README.md), which is why it is spelled with parentheses: `(a, b)` has two fields, `(a,)` has one, `()` has none. Counting values across the primitives puts it in order — `u8` has 256, `bool` has 2, `()` has 1, and `!` — the never type — has 0.

## Where it comes from #1: a function with no `->`

```rust
fn tally() {}            // identical to
fn tally_spelled() -> () {}
```

Every function that does not name a return type returns `()`. So a program is full of unit values already; the type just never had to be written down.

## Where it comes from #2: the semicolon

```rust
let discarded = { 7; };   // ()
let kept      = { 7 };    // 7
println!("{discarded:?} {kept}");   // () 7
```

A `;` throws the value away and leaves `()` behind. That single rule is behind the whole family of beginner errors that read *"expected `i32`, found `()`"* — the block was supposed to be the value, and a semicolon quietly turned it into a statement. [A block is an expression](../a_block_is_an_expression/README.md) is that mechanism in full, and its kata makes you cause the error deliberately.

## Where it turns up #3: `Result<(), E>`

```rust
fn check_score(score: u8) -> Result<(), String> {
    if score <= 5 { Ok(()) } else { Err(format!("score {score} is out of range 0..=5")) }
}
```

`Ok(())` reads oddly the first time and says something precise: **it worked, and there is nothing to hand back**. A validator, a `write!`, a `File::set_len` — each either succeeds with no payload or fails with a reason, and `Result<(), E>` is exactly that shape. The [`?` operator](../../17_Option_and_Result/option_vs_result/README.md) then works as it always does; on success it unwraps a value that happens to carry nothing, so nothing is lost.

## Where it turns up #4: a set is a map whose values are `()`

```rust
use std::collections::HashMap;
let mut seen: HashMap<&str, ()> = HashMap::new();
seen.insert("Ada", ());
println!("{}", seen.contains_key("Ada"));   // true
```

Because `()` costs nothing to store, a `HashMap<T, ()>` *is* a set — the value column is free. `HashSet<T>` in the standard library is [literally that ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html), a wrapper over `HashMap<T, ()>`, and the two have the same size.

The one place the wrapper improves on it is the return type: `HashMap::insert` gives you `Option<()>`, which is a `bool` wearing eight extra characters, so [`HashSet::insert`](../../26_Collections/the_hashset/README.md) hands back a real `bool` instead. Same structure, better name — a small worked example of when a unit value should be translated into something that reads.

## The trap: `let x = v.sort();`

Every in-place method returns `()`, because the answer was written back into the receiver:

```rust
let mut v = vec![3u8, 1, 5];
let x = v.sort();          // x is (), and v is now [1, 3, 5]
println!("{x:?} {v:?}");   // () [1, 3, 5]
```

Nothing complains until you use `x`, and then the message names the type outright:

```text title="Abridged — real rustc output for sortunit.rs"
error[E0599]: no method named `len` found for unit type `()` in the current scope
    --> sortunit.rs:4:27
     |
   4 |     println!("{}", sorted.len());
     |                           ^^^
```

*"for unit type `()`"* is the whole diagnosis: you took the receipt instead of the result. `push`, `dedup`, `retain`, `clear`, `insert` on a `Vec` and `sort_unstable` all behave the same way, which is also why none of them chains — `v.push(9).dedup()` does not compile. Mutate, then use; or clone first and sort the copy.

## `()` versus the two things it is confused with

| | values | means |
|---|---|---|
| `()` | exactly **1** | "nothing to say, and we got here" |
| [`Option::None`](../../17_Option_and_Result/some_and_none/README.md) | one variant of a type | "there might have been something, and there wasn't" |
| [`!`](../../25_Control_Flow/the_loop_keyword/README.md) (never) | **0** | "this never returns at all" |

The first two get muddled most. `()` is a value you *have*; `None` is a value that records an absence — and `Option<()>` is both at once, which is why it reads as a clumsy `bool`. `!` is the odd one: `panic!()` and `loop {}` have type `!`, and because no value of it can exist, it coerces into any type at all — which is what lets `Err(_) => panic!("…")` sit in a `match` arm beside `Ok(n) => n`.

## If you are coming from another language

**Python.** The nearest thing is `None`, and the resemblance is misleading in a way worth being explicit about. A Python function with no `return` gives back `None`, exactly as a Rust function with no `->` gives back `()` — so far so similar. But `None` in Python is the *only* value of `NoneType` **and** the value that means "missing", so one object does both jobs. Rust splits them: `()` is "done, nothing to report", `Option::None` is "there is no value here", and mixing them up produces a type error rather than a puzzling `AttributeError` three functions later. Two Python habits that translate:

| Python | Rust |
|---|---|
| `def f(): pass` → returns `None` | `fn f() {}` → returns `()` |
| `x = lst.sort()` → `x` is `None` | `let x = v.sort();` → `x` is `()` |
| `if x is None:` for "no value" | `match opt { None => … }` |
| `None` as a sentinel *and* as "no return" | `Option::None` and `()`, two different types |

The `sort` row is the same bug in both languages, and Rust catches it earlier: Python raises `AttributeError: 'NoneType' object has no attribute 'append'` at run time, Rust says `no method named … for unit type ()` at compile time. Note the direction of the shared design lesson — both languages made `sort` return nothing *deliberately*, so that a mutation cannot be mistaken for a copy.

**ABAP.** ABAP has no unit type and does not need one, because the two things `()` covers are separate constructs there. A procedure that returns nothing is a `METHOD` with no `RETURNING` parameter (or a `FORM`/`PERFORM`), and the language simply has no expression for it — you cannot write `DATA(lv_x) = lo_obj->do_something( )` when `do_something` returns nothing, so ABAP's version of the `let x = v.sort()` trap is a syntax error at the call site rather than a confusing type later.

Two mappings that do carry over. The `Result<(), E>` shape is `sy-subrc` without the discipline: an ABAP statement that either works or sets a return code is the same *idea* — success carries no payload, failure carries a reason — but `sy-subrc` can be ignored by simply not looking at it, whereas `Result<(), E>` is `#[must_use]` and the compiler warns when you drop it. That is the single biggest practical difference between the two error styles, and it is much easier to see on `Result<(), E>` than on a `Result` that carries a value you obviously wanted. Second, ABAP's sorted internal-table operations are the `()` pattern in disguise: `SORT lt_tab BY score DESCENDING.` is a *statement* that mutates in place and yields nothing, exactly as `v.sort()` is. Both languages decided the same way, so the instinct transfers even though the syntax does not.

---

## Practice

**Three places `()` shows up, and what each one is telling you.**

Take a `Vec<u8>` of scores and write `let x = scores.sort();`. Say what `x` is before you print it, then try to ask `x` for its length and read the error message carefully — it names the type. Show the two ways to end up holding a sorted value, and confirm that `push`, `dedup` and `retain` all behave the same way (so none of them chains).

Then write a ballot validator as `fn check_ballot(&[u8]) -> Result<(), String>` that uses `?` on a per-score check, and run it over one valid and one invalid ballot. Say what `?` discards on success.

Finish by building a set out of `HashMap<T, ()>` — insert five names with two repeats, and recover the first-appearance order. What does `insert` return, and why does `HashSet::insert` return something else?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_unit_type_kata -->
*[`the_unit_type_kata.rs`](examples/the_unit_type_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three places `()` shows up, and what each one is telling you.

use std::collections::HashMap;

fn main() {
    println!("=== part 1: the in-place methods hand back nothing ===");
    let mut scores = vec![3u8, 5, 1, 5, 0];
    println!("  before                 = {:?}", scores);

    let returned: () = scores.sort();
    println!("  let x = scores.sort()  -> x is {:?}, NOT the sorted vector", returned);
    println!("  scores (mutated)       = {:?}", scores);
    println!("  asking x for its length is where you find out:");
    println!("    error[E0599]: no method named `len` found for unit type `()` in the current scope");
    println!("  the message names the type: `()` is what an in-place method returns,");
    println!("  because the answer was written back into the receiver.");

    println!("\n  the two ways to get a sorted value out of it:");
    let mut in_place = vec![3u8, 5, 1];
    in_place.sort();
    println!("    mutate, then use     = {:?}", in_place);
    let original = vec![3u8, 5, 1];
    let mut copy = original.clone();
    copy.sort();
    println!("    clone, sort the copy = {:?}  (original still {:?})", copy, original);

    println!("\n  every in-place method on Vec does this:");
    let mut v = vec![3u8, 1, 1, 5];
    let a: () = v.push(9);
    let b: () = v.dedup();
    let c: () = v.retain(|&s| s <= 5);
    println!("    push / dedup / retain all return {:?} {:?} {:?} -> v = {:?}", a, b, c, v);
    println!("    so none of them chains: v.push(9).dedup() does not compile");

    println!("\n=== part 2: Ok(()) is 'it worked, and there is nothing to hand back' ===");
    fn check_score(score: u8) -> Result<(), String> {
        if score <= 5 { Ok(()) } else { Err(format!("score {score} is out of range 0..=5")) }
    }
    fn check_ballot(ballot: &[u8]) -> Result<(), String> {
        for &s in ballot {
            check_score(s)?;
        }
        Ok(())
    }
    for ballot in [&[5u8, 3, 0][..], &[5u8, 9, 0][..]] {
        println!("  check_ballot({:?}) = {:?}", ballot, check_ballot(ballot));
    }
    println!("  `check_score(s)?` discards nothing on success -- there was nothing to discard.");
    println!("  Result<(), E> is the return type of a job that either works or explains itself.");

    println!("\n=== part 3: a set is a map whose values are () ===");
    let mut seen: HashMap<&str, ()> = HashMap::new();
    let mut order: Vec<&str> = Vec::new();
    for name in ["Ada", "Ben", "Ada", "Cara", "Ben"] {
        if seen.insert(name, ()).is_none() {
            order.push(name);
        }
    }
    println!("  first appearance order = {:?}", order);
    println!("  seen.contains_key(\"Ada\")  = {}", seen.contains_key("Ada"));
    println!("  seen.len()                = {}", seen.len());
    println!("  `insert` returns Option<()> -- Some(()) means it was already there.");
    println!("  That Option<()> is a bool with extra syntax, which is exactly why");
    println!("  HashSet::insert returns a real bool instead. Same structure, better name.");
}
```
<!-- /source -->

<!-- output:the_unit_type_kata -->
*Verified output of [`the_unit_type_kata.rs`](examples/the_unit_type_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== part 1: the in-place methods hand back nothing ===
  before                 = [3, 5, 1, 5, 0]
  let x = scores.sort()  -> x is (), NOT the sorted vector
  scores (mutated)       = [0, 1, 3, 5, 5]
  asking x for its length is where you find out:
    error[E0599]: no method named `len` found for unit type `()` in the current scope
  the message names the type: `()` is what an in-place method returns,
  because the answer was written back into the receiver.

  the two ways to get a sorted value out of it:
    mutate, then use     = [1, 3, 5]
    clone, sort the copy = [1, 3, 5]  (original still [3, 5, 1])

  every in-place method on Vec does this:
    push / dedup / retain all return () () () -> v = [3, 1, 5]
    so none of them chains: v.push(9).dedup() does not compile

=== part 2: Ok(()) is 'it worked, and there is nothing to hand back' ===
  check_ballot([5, 3, 0]) = Ok(())
  check_ballot([5, 9, 0]) = Err("score 9 is out of range 0..=5")
  `check_score(s)?` discards nothing on success -- there was nothing to discard.
  Result<(), E> is the return type of a job that either works or explains itself.

=== part 3: a set is a map whose values are () ===
  first appearance order = ["Ada", "Ben", "Cara"]
  seen.contains_key("Ada")  = true
  seen.len()                = 3
  `insert` returns Option<()> -- Some(()) means it was already there.
  That Option<()> is a bool with extra syntax, which is exactly why
  HashSet::insert returns a real bool instead. Same structure, better name.
```
<!-- /output -->

</details>

## The verified output

<!-- output:the_unit_type -->
*Verified output of [`the_unit_type.rs`](examples/the_unit_type.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== one value, zero bytes ===
  ()                     = ()
  size_of::<()>()        = 0   <- a zero-sized type: it occupies nothing
  size_of::<[(); 1000]>() = 0   <- a thousand of them also occupy nothing
  () == ()               = true   <- one value, so equality is always true
  it is the only type with exactly one value; bool has 2, u8 has 256, () has 1

=== where it comes from #1: a function with no -> ===
  fn no_return_type() {}    returns ()
  fn explicit_unit() -> () {} returns ()   <- the same signature, spelled out
  fn returns_a_number() -> i32 returns 7

=== where it comes from #2: the semicolon ===
  { 7; }  = ()   <- the ; discards the value and leaves ()
  { 7 }   = 7    <- no ;, so the block IS the value
  that is the whole mechanism behind `expected i32, found ()`

=== where it turns up #3: a Result that carries no success value ===
  record_vote(5) = Ok(())
  record_vote(9) = Err("score 9 is out of range 0..=5")
  Ok(()) says 'it worked, and there is nothing to hand back'

=== where it turns up #4: a set is a map whose values are () ===
  size_of::<HashSet<&str>>() == size_of::<HashMap<&str, ()>>() : true
  ...because () costs nothing to store, so the map's value column is free
  set.contains("Ada")       = true
  map.contains_key("Ada")   = true

=== the operations that hand you one back ===
  names.sort()              -> ()   <- sorts in place, returns nothing
  names                     =  ["Ada", "Ben", "Cara"]
  names.push("Dev")         -> ()
  println! itself           -> the line you are reading
  ...and its value          =  ()
  that is why `let x = v.sort();` compiles and then confuses you:
  x is (), not the sorted vector

=== () versus the two things it is confused with ===
  ()          one value,  zero bytes   'nothing to say'
  Option::None one variant of a type   'there might have been something'
  !           NO values                'this never returns at all'
  Option<()>  = None or Some(()) -- a bool wearing two extra characters
  size_of::<Option<()>>() = 1   <- one byte, because None needs a tag
```
<!-- /output -->

## See also

- [A block is an expression](../a_block_is_an_expression/README.md) — the semicolon that produces `()`, and the `E0308` it causes
- [Tuples](../../26_Collections/tuples/README.md) — `()` is the zero-field member of that family
- [Values](../values/README.md) — the census of everything you can write a literal for; `()` is the one with a single value
- [`Some` and `None`](../../17_Option_and_Result/some_and_none/README.md) — the absence `()` is confused with
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — where `Result<(), E>` sits
- [The `HashSet`](../../26_Collections/the_hashset/README.md) — the standard library's `HashMap<T, ()>`, with a better `insert`

## Po polsku

`()` to pusta krotka — **jedna** wartość zajmująca **zero** bajtów — i jest zarazem typem, i jedyną wartością tego typu, zapisywaną tak samo. To nie skrót notacyjny: typ o dokładnie jednej wartości nie niesie żadnej informacji, więc nie ma czego przechowywać, i dlatego `size_of::<[(); 1000]>()` też wynosi 0. Po polsku mówi się o **typie jednostkowym** (*unit type*), a policzenie wartości układa prymitywy w ciąg: `u8` ma ich 256, `bool` 2, `()` jedną, a `!` (typ „nigdy") — zero.

Wartości `()` w programie jest pełno, zanim ktokolwiek napisze ją celowo, bo bierze się z dwóch miejsc. Po pierwsze, **każda funkcja bez `->` zwraca `()`** — `fn tally() {}` i `fn tally() -> () {}` to ta sama sygnatura. Po drugie, **średnik**: `{ 7; }` ma wartość `()`, a `{ 7 }` ma wartość 7, i właśnie z tej jednej reguły bierze się cała rodzina komunikatów w rodzaju *„expected `i32`, found `()`”* — blok miał być wartością, a średnik po cichu zamienił go w instrukcję.

Dwa miejsca, w których `()` pojawia się już świadomie, warto znać z nazwy. `Result<(), E>` to typ zadania, które albo się udaje bez żadnego wyniku, albo zawodzi z powodem — `Ok(())` czyta się jako „zadziałało i nie ma czego oddać”, a `?` nie gubi tu niczego, bo nie było czego zgubić. Oraz: **zbiór to mapa, której wartościami są `()`** — skoro `()` nic nie kosztuje, `HashMap<T, ()>` *jest* zbiorem, i dokładnie tym jest `HashSet<T>` w bibliotece standardowej. Jedyne, co opakowanie poprawia, to typ zwracany: `HashMap::insert` oddaje `Option<()>`, czyli `bool` przebrany za coś innego, więc `HashSet::insert` zwraca prawdziwy `bool`.

Pułapka do zapamiętania to `let x = v.sort();`. Każda metoda działająca „w miejscu” zwraca `()`, bo odpowiedź została wpisana z powrotem do odbiorcy — nic nie protestuje, dopóki nie użyjesz `x`, a wtedy komunikat wprost nazywa typ: `no method named 'len' found for unit type '()'`. Wzięło się pokwitowanie zamiast wyniku. Tak samo zachowują się `push`, `dedup`, `retain`, `clear` i `sort_unstable`, i dlatego żadna z nich się nie łańcuchuje. Ta sama pomyłka istnieje w Pythonie (`x = lst.sort()` daje `None`), z tą różnicą, że Python zgłasza ją dopiero w czasie działania. W ABAP-ie odpowiednikiem jest `SORT lt_tab BY …` — instrukcja, która zmienia tabelę w miejscu i niczego nie zwraca; ABAP nie ma typu jednostkowego, więc próba przypisania wyniku jest po prostu błędem składni w miejscu wywołania, a nie mylącym typem kilka linijek dalej.

Na koniec rozróżnienie, które najczęściej się zaciera: `()` to wartość, **którą masz** („nie ma nic do powiedzenia, ale doszliśmy tutaj”); `Option::None` to wartość zapisująca **brak** („mogło coś być, a nie ma”); a `!` to typ **bez żadnej wartości** („to nigdy nie wraca”) — mają go `panic!()` i `loop {}`, i właśnie dlatego, że nie może istnieć żadna jego wartość, dopasowuje się do dowolnego typu, co pozwala postawić `Err(_) => panic!("…")` obok `Ok(n) => n` w jednym `match`.

**Szukaj po polsku:** typ jednostkowy · pusta krotka · `rust expected i32 found ()` · `rust let x = v.sort()` · `rust Result<(), E>`
