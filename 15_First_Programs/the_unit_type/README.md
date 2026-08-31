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

`()` is both a **type** and the single **value** of that type, written the same way. The zero is not a special case the compiler grants it; it falls out of what memory is for.

**Memory exists to tell states apart**, so the bits a type needs is `log2(values)`:

| type | values | bits of information | bytes occupied |
|---|---|---|---|
| `u8` | 256 | 8 | 1 |
| `bool` | 2 | 1 | **1** |
| `()` | 1 | **0** | **0** |

One value needs `log2(1) = 0` bits. If a variable has type `()`, its value *must* be `()`, so there is nothing left for memory to record and the compiler stores nothing at all. That makes it a **zero-sized type** (ZST) — a small family that also includes `struct Marker;`, `[(); 1000]`, and [`PhantomData`](../../12_Traits/phantom_types/README.md).

`bool` is the row where the two right-hand columns part company: one bit of information, one whole byte of space, because a byte is the smallest thing a machine can address. `()` is below even that floor — there is no information to round up.

```text
size_of::<()>()         = 0
size_of::<[(); 1000]>() = 0    <- 1000 × 0; the length is compile-time knowledge
() == ()                = true <- one value, so equality cannot be otherwise
```

The array row is worth pausing on: an array's size is its element size times its length, so a thousand units is `1000 × 0`. The length is still tracked — in the *type* — but no stack or heap byte is set aside for the elements.

It is the [empty tuple](../../26_Collections/tuples/README.md), which is why it is spelled with parentheses: `(a, b)` has two fields, `(a,)` has one, `()` has none. Counting values across the primitives puts it in order — `u8` has 256, `bool` has 2, `()` has 1, and `!` — the never type — has 0.

### The equality is settled by the compiler, not at run time

With one value in existence, any two instances are the same one, so `() == ()` is not a comparison — it is a constant. Compile `fn unit_eq(a: (), b: ()) -> bool { a == b }` with `-O` and ask for the assembly:

```text title="Abridged — real `rustc -O --emit asm` output, x86-64"
_unit_eq:
	movb	$1, %al          ; load the constant 1, return
	retq

_bool_eq:
	movl	%edi, %eax       ; the bool version actually compares
	xorl	%esi, %eax
	xorb	$1, %al
```

Neither argument is so much as read. That is the practical shape of "carries no information": there is nothing to look at.

### Zero bytes is not "no address"

A ZST is still a real place, which is what keeps it usable in generic code:

```rust
let here: &() = &();                 // a real reference, at a real, aligned address
println!("{}", align_of::<()>());    // 1
```

And a `Vec` of them never allocates, because there is nothing to allocate for — its capacity is `usize::MAX` from the start:

```rust
let mut many: Vec<()> = Vec::new();
for _ in 0..1_000_000 { many.push(()); }
println!("{} {}", many.len(), many.capacity() == usize::MAX);   // 1000000 true
```

A million elements, no heap traffic: the `Vec` has become a counter with two spare fields. That is the general payoff of a ZST — the ordinary data structures keep working, and cost nothing when what you are storing is *the fact that there is an entry* rather than a value.

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

Because `()` costs nothing to store, a `HashMap<T, ()>` *is* a set — the value column is free. [`HashSet<T>` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html) in the standard library is literally that, two wrappers deep: `std::collections::HashSet` holds a `hashbrown::HashSet`, and in the toolchain's own vendored copy of hashbrown that type is declared as one field — `map: HashMap<T, (), S, A>`. The set is a map with `()` in the value column, and the two have the same size.

The one place the wrapper improves on it is the return type: `HashMap::insert` gives you `Option<()>`, which is a `bool` wearing eight extra characters, so [`HashSet::insert`](../../26_Collections/the_hashset/README.md) hands back a real `bool` instead. Same structure, better name — a small worked example of when a unit value should be translated into something that reads.

## Where it turns up #5: a channel that carries only the fact

```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel::<()>();
tx.send(()).unwrap();
rx.recv().unwrap();          // the message IS the signal
```

When one thread needs to tell another *something happened* and there is no data to hand over — a shutdown request, a tick, "the file is written" — the payload type is `()`. Zero bytes cross the channel; what crosses is the fact that a send occurred. `Sender<()>` says that in the type, where a `Sender<bool>` carrying a permanent `true` would only imply it.

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

=== why zero: memory exists to tell states apart ===
  type      values   bits   bytes
  u8           256      8       1
  bool           2      1       1
  ()             1      0       0
  bits = log2(values). One value needs log2(1) = 0 bits, so there is nothing
  to store: if a variable has type (), its value must be (). bool is the row
  where the two columns part -- 1 bit of information, 1 whole byte of space,
  because a byte is the smallest thing a machine can address.

=== the equality is decided at compile time, not run time ===
  () == ()   = true   <- one value, so it cannot be otherwise
  compiled with -O, `fn unit_eq(a: (), b: ()) -> bool { a == b }` is:
      movb  $1, %al        <- load the constant 1, and return
  neither argument is read. The bool version really compares:
      movl  %edi, %eax ; xorl %esi, %eax ; xorb $1, %al

=== zero bytes is not 'no address' ===
  align_of::<()>()      = 1   <- still aligned, still a real place
  &() is a real reference at a nonzero address: true
  a Vec<()> after 1,000,000 pushes: len 1000000
  ...and it never allocated: capacity == usize::MAX is true
  there is no data to store, so the Vec is just a counter with a spare field

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

=== where it turns up #5: a channel that carries only the fact ===
  mpsc::channel::<()>() -- the message IS the signal, with no payload
  received one; size of what crossed the channel = 0 bytes

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

`()` to pusta krotka — **jedna** wartość zajmująca **zero** bajtów — i jest zarazem typem, i jedyną wartością tego typu, zapisywaną tak samo. Po polsku mówi się o **typie jednostkowym** (*unit type*). Zero nie jest tu ulgą przyznaną przez kompilator, tylko wynikiem tego, po co w ogóle jest pamięć: **pamięć służy do odróżniania stanów**, więc liczba potrzebnych bitów to `log2(liczba wartości)`. `u8` ma 256 wartości, czyli 8 bitów; `bool` ma 2, czyli 1 bit; `()` ma jedną, czyli `log2(1) = 0` bitów. Jeśli zmienna jest typu `()`, jej wartością **musi** być `()` — nie ma czego zapisywać, więc kompilator nie zapisuje niczego. To czyni z niej **typ o zerowym rozmiarze** (*zero-sized type*, ZST) — do tej rodziny należą też `struct Marker;`, `[(); 1000]` i `PhantomData`.

Wiersz `bool` jest tym, w którym rozjeżdżają się dwie kolumny: jeden bit informacji, ale cały bajt miejsca, bo bajt jest najmniejszą jednostką mającą własny adres. `()` leży poniżej nawet tej podłogi — nie ma informacji, którą trzeba by zaokrąglić w górę. Stąd `size_of::<[(); 1000]>()` również wynosi 0: rozmiar tablicy to rozmiar elementu razy długość, czyli `1000 × 0`, a sama długość jest wiedzą **z czasu kompilacji**, nie bajtem w pamięci. Policzenie wartości układa zresztą prymitywy w ciąg: `u8` ma ich 256, `bool` 2, `()` jedną, a `!` (typ „nigdy") — zero.

Dwie rzeczy, które z tego wynikają, warto zobaczyć na własne oczy. Po pierwsze, **porównanie rozstrzyga kompilator, a nie procesor**: skoro istnieje tylko jedna wartość, `() == ()` nie jest porównaniem, tylko stałą, i funkcja `fn unit_eq(a: (), b: ()) -> bool { a == b }` skompilowana z `-O` to dosłownie `movb $1, %al` — wpisz jedynkę i wróć, nie zaglądając do żadnego z argumentów (wersja dla `bool` wykonuje prawdziwe `xor`). Po drugie, **zero bajtów to nie „brak adresu"**: `align_of::<()>()` wynosi 1, a `&()` jest prawdziwą referencją pod prawdziwym, wyrównanym adresem — i właśnie dlatego typ o zerowym rozmiarze da się używać w kodzie generycznym. Konsekwencja, która najbardziej zaskakuje: `Vec<()>` **nigdy nie alokuje**, bo nie ma czego alokować — jego pojemność od początku wynosi `usize::MAX`, więc milion elementów nie powoduje ani jednego dotknięcia sterty, a sam wektor staje się licznikiem z dwoma niepotrzebnymi polami.

Wartości `()` w programie jest pełno, zanim ktokolwiek napisze ją celowo, bo bierze się z dwóch miejsc. Po pierwsze, **każda funkcja bez `->` zwraca `()`** — `fn tally() {}` i `fn tally() -> () {}` to ta sama sygnatura. Po drugie, **średnik**: `{ 7; }` ma wartość `()`, a `{ 7 }` ma wartość 7, i właśnie z tej jednej reguły bierze się cała rodzina komunikatów w rodzaju *„expected `i32`, found `()`”* — blok miał być wartością, a średnik po cichu zamienił go w instrukcję.

Trzy miejsca, w których `()` pojawia się już świadomie, warto znać z nazwy. `Result<(), E>` to typ zadania, które albo się udaje bez żadnego wyniku, albo zawodzi z powodem — `Ok(())` czyta się jako „zadziałało i nie ma czego oddać”, a `?` nie gubi tu niczego, bo nie było czego zgubić. Oraz: **zbiór to mapa, której wartościami są `()`** — skoro `()` nic nie kosztuje, `HashMap<T, ()>` *jest* zbiorem, i dokładnie tym jest `HashSet<T>` w bibliotece standardowej. Jedyne, co opakowanie poprawia, to typ zwracany: `HashMap::insert` oddaje `Option<()>`, czyli `bool` przebrany za coś innego, więc `HashSet::insert` zwraca prawdziwy `bool`. I trzecie: **kanał, który niesie wyłącznie sam fakt** — gdy jeden wątek ma powiedzieć drugiemu „coś się stało", a nie ma czego przekazać (żądanie zamknięcia, takt zegara, „plik zapisany"), typem ładunku jest `()`. Przez kanał przechodzi zero bajtów; przechodzi sama informacja, że nadano. `Sender<()>` mówi to **w typie**, podczas gdy `Sender<bool>` wiecznie wysyłający `true` mógłby to najwyżej sugerować.

Pułapka do zapamiętania to `let x = v.sort();`. Każda metoda działająca „w miejscu” zwraca `()`, bo odpowiedź została wpisana z powrotem do odbiorcy — nic nie protestuje, dopóki nie użyjesz `x`, a wtedy komunikat wprost nazywa typ: `no method named 'len' found for unit type '()'`. Wzięło się pokwitowanie zamiast wyniku. Tak samo zachowują się `push`, `dedup`, `retain`, `clear` i `sort_unstable`, i dlatego żadna z nich się nie łańcuchuje. Ta sama pomyłka istnieje w Pythonie (`x = lst.sort()` daje `None`), z tą różnicą, że Python zgłasza ją dopiero w czasie działania. W ABAP-ie odpowiednikiem jest `SORT lt_tab BY …` — instrukcja, która zmienia tabelę w miejscu i niczego nie zwraca; ABAP nie ma typu jednostkowego, więc próba przypisania wyniku jest po prostu błędem składni w miejscu wywołania, a nie mylącym typem kilka linijek dalej.

Na koniec rozróżnienie, które najczęściej się zaciera: `()` to wartość, **którą masz** („nie ma nic do powiedzenia, ale doszliśmy tutaj”); `Option::None` to wartość zapisująca **brak** („mogło coś być, a nie ma”); a `!` to typ **bez żadnej wartości** („to nigdy nie wraca”) — mają go `panic!()` i `loop {}`, i właśnie dlatego, że nie może istnieć żadna jego wartość, dopasowuje się do dowolnego typu, co pozwala postawić `Err(_) => panic!("…")` obok `Ok(n) => n` w jednym `match`.

**Szukaj po polsku:** typ jednostkowy · typ o zerowym rozmiarze · pusta krotka · `rust expected i32 found ()` · `rust let x = v.sort()` · `rust Result<(), E>` · `rust zero sized type`
