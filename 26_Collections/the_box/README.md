# `Box`

**Level:** 201 · working knowledge

**One line:** `Box<T>` puts one value on the heap and leaves an 8-byte pointer behind — which buys two things the stack cannot give you: a type that contains itself, and a size known only at run time.

```rust
fn main() {
    let boxed = Box::new([0u32; 64]);
    println!("{}", boxed.len());          // 64  — no explicit deref needed
    println!("{}", size_of::<Box<[u32; 64]>>());   // 8
}
```

`Box<T>` implements `Deref<Target = T>`, so field access, method calls and `&*b` all reach through it. `*b` on its own **moves** the value out and drops the box — the one operation that is not a borrow.

## What it costs, and what it is not

A `Box` is single ownership: one owner, dropped when that owner goes out of scope, moved rather than copied. Reaching for it just to "put something on the heap" adds an allocation and an indirection for nothing — `Vec`, `String` and `HashMap` are already heap-backed, and their contents were never on the stack to begin with.

Two owners is [`Rc`](../../18_Ownership/reference_counting/README.md); two threads is [`Arc`](../../18_Ownership/sharing_across_threads/README.md). Both are `Box` with a counter, and both are the wrong default.

## Reason one: a type that contains itself

```rust
enum Round {
    Final(&'static str),
    Then(&'static str, Box<Round>),
}
```

Without the `Box`, that does not compile:

```text
error[E0072]: recursive type `Round` has infinite size
 --> round.rs:2:1
  |
2 | enum Round {
  | ^^^^^^^^^^
3 |     Final(&'static str),
4 |     Then(&'static str, Round),
  |                        ----- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
4 |     Then(&'static str, Box<Round>),
  |                        ++++     +
```

Every `Then` would contain a whole `Round`, which contains a whole `Round`, and the compiler has to write down a size before it can lay the type out. A pointer has a size it knows in advance, whatever is on the other end — so `size_of::<Round>()` becomes 24 and the recursion is resolved.

Note what the compiler offers: `Box`, `Rc`, **or `&`**. Any indirection breaks the cycle; `Box` is the one that also owns what it points at.

## Reason two: a size known only at run time

```rust
fn main() {
    let doubler: Box<dyn Fn(u32) -> u32> = Box::new(|n| n * 2);
    let shifter: Box<dyn Fn(u32) -> u32> = Box::new(|n| n + 100);
    for f in [&doubler, &shifter] {
        println!("{}", f(21));   // 42 then 121
    }
}
```

Two closures with different captures have two different anonymous types and two different sizes. `Box<dyn Fn(u32) -> u32>` gives them one type by putting each on the heap and holding a pointer — plus a second pointer to its vtable, which is why `size_of::<Box<dyn Fn(u32) -> u32>>()` is 16 while `size_of::<Box<u32>>()` is 8. **A `dyn` box is fat; a plain `Box<T>` is not.**

## `Option<Box<T>>` is free

```rust
struct Node { score: u32, next: Option<Box<Node>> }
```

`size_of::<Option<Box<Node>>>()` is 8 — the same as the `Box` alone. A `Box` can never be null, so the compiler uses the all-zero bit pattern to mean `None`. That is the **null-pointer optimisation**, and it is what makes the linked list above cost exactly what the C version costs, with the null check moved into the type system.

## The trap: the default `Drop` is recursive

Dropping the head of a long boxed list drops its `next`, which drops *its* next — one stack frame per node. A list of a hundred thousand nodes overflows the stack **in the destructor**, at the end of a scope, with a backtrace that names nothing you wrote. Real linked-list types implement `Drop` by hand, popping in a loop. The same shape appears in any recursive walk of a boxed structure: Rust does not promise tail-call elimination, so a `while let` cursor is the version to write for data of unknown depth.

## If you are coming from another language

- **Python.** Every Python object is already boxed — a name holds a reference to a heap object, always — so `Box` looks like nothing at first. The useful reading is inverted: Rust's *default* is what Python has no word for (the value itself, inline, on the stack), and `Box` is how you ask for what Python always does. The place it becomes concrete is the recursive class: `class Node: def __init__(self, score, next=None)` needs no ceremony because `next` is a reference either way, and Rust's `Option<Box<Node>>` is that same field with the reference made explicit and the `None` checked. `Box<dyn Trait>` is duck typing with the duck written down.
- **ABAP.** A `REF TO` data reference is the closest thing, and the correspondence is good: `CREATE DATA` allocates, `->` dereferences (Rust does it implicitly), and a structure containing a `REF TO` itself is exactly the recursive type this page is about — try to embed the structure directly and the ABAP compiler refuses for the same reason. What `Box` adds is ownership: an ABAP reference does not free anything on scope exit, garbage collection does it when the last reference goes, so `Box`'s "dropped when its owner goes out of scope" is closer to a `CLASS` with a destructor you can rely on. And `Box<dyn Trait>` is a `REF TO if_interface` holding an instance of an implementing class — the same dispatch, the same reason you cannot store the object inline.
- **C++.** `std::unique_ptr<T>`, almost exactly: single ownership, move-only, freed on scope exit. `Box<dyn Trait>` is `unique_ptr<Base>` with virtual dispatch, and the vtable pointer sits beside the data pointer rather than inside the object. `make_unique` is `Box::new`.
- **Java / C#.** Everything except a primitive is already a reference, so as with Python the interesting direction is the other one — a Rust struct is a C# `struct`, and `Box<T>` is what makes it behave like a `class`.

---

## The verified output

<!-- output:the_box -->
*Verified output of [`the_box.rs`](examples/the_box.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A Box is a pointer, whatever it points at
   size_of::<Ballot>()      = 256
   size_of::<Box<Ballot>>() = 8
   size_of::<Box<u8>>()     = 8
   The 256 bytes moved to the heap; 8 bytes stayed on the stack.
   Moving a Box copies those 8 bytes and nothing else.

2. It behaves like the value it holds
   boxed.scores.len() = 64 — no explicit deref needed
   *boxed moves the value back out: 64 scores
   `Box<T>` implements `Deref<Target = T>`, so field access, method
   calls and `&*b` all reach through. `*b` on its own MOVES the value
   out and drops the box — the one operation that is not a borrow.

3. The reason Box exists: a type that contains itself
   enum Round { Final(&str), Then(&str, Round) }      <- E0072
   "recursive type `Round` has infinite size". Each `Then` would
   contain a whole `Round`, which contains a whole `Round`…
   Box breaks the chain, because a pointer has a size the compiler
   can write down before it knows what is on the other end.
   size_of::<Round>() = 24 — one tag plus the largest variant
   eliminated(&rounds) = ["Ada", "Ben"]
   winner(&rounds) = Cara

4. And the other reason: a size known only at run time
   two closures with different captures, one type: Box<dyn Fn>
   named(21) = 42, shifted(21) = 121
   size_of::<Box<dyn Fn(u32) -> u32>>() = 16 — pointer to the value
   AND pointer to its vtable. A `dyn` box is fat; a `Box<T>` is not.

5. What it is not
   Box is single ownership. One owner, dropped when that owner goes
   out of scope, moved rather than copied. For two owners you want
   Rc; for two threads, Arc. Reaching for Box to "put it on the heap"
   when nothing needs the heap just adds an allocation and an
   indirection: a Vec, a String and a HashMap are already heap-backed.
```
<!-- /output -->

## Practice

**A list that ends, and the three sizes that explain it.** Build a singly linked list of scores with `Option<Box<Node>>`, then total it two ways: a recursive function, and a `while let` that walks a cursor. Print `size_of` for `Box<Node>`, `Option<Box<Node>>` and `Node`, and explain why the first two are equal.

Then make two things happen that the sizes do not show. Add a type whose `Drop` prints, put two of them in a scope, and predict the order before running it. And take the `Box` out of the `next` field — write down the error code and the fix rustc offers, in its own words.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_box_kata -->
*[`the_box_kata.rs`](examples/the_box_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a list that ends, and the three sizes that explain it.
//!
//!   rustc --edition 2024 the_box_kata.rs -o /tmp/bk && /tmp/bk

/// A singly linked list of scores. `None` is the end.
#[derive(Debug)]
struct Node {
    score: u32,
    next: Option<Box<Node>>,
}

fn from_slice(scores: &[u32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &score in scores.iter().rev() {
        head = Some(Box::new(Node { score, next: head }));
    }
    head
}

fn total(node: &Option<Box<Node>>) -> u32 {
    match node {
        None => 0,
        Some(n) => n.score + total(&n.next),
    }
}

fn to_vec(node: &Option<Box<Node>>) -> Vec<u32> {
    let mut out = Vec::new();
    let mut cursor = node;
    while let Some(n) = cursor {
        out.push(n.score);
        cursor = &n.next;
    }
    out
}

/// Prints when it is dropped, so drop order is observable.
struct Loud(&'static str);
impl Drop for Loud {
    fn drop(&mut self) {
        println!("   dropping {}", self.0);
    }
}

fn main() {
    println!("1. The list");
    let list = from_slice(&[5, 3, 0, 4]);
    println!("   scores : {:?}", to_vec(&list));
    println!("   total  : {}", total(&list));

    println!();
    println!("2. Why it costs nothing to say \"or nothing\"");
    println!("   size_of::<Box<Node>>()         = {}", size_of::<Box<Node>>());
    println!("   size_of::<Option<Box<Node>>>() = {}", size_of::<Option<Box<Node>>>());
    println!("   size_of::<Node>()              = {}", size_of::<Node>());
    println!("   The Option is the same size as the Box. A Box can never be null,");
    println!("   so the compiler uses the all-zero bit pattern to mean `None` —");
    println!("   the null-pointer optimisation. `Option<Box<T>>` is a nullable");
    println!("   pointer with the null checked by the type system.");

    println!();
    println!("3. What the recursion costs");
    println!("   `total` calls itself once per node, so a list of 100_000 nodes");
    println!("   would need 100_000 stack frames and overflow the stack. The");
    println!("   `while let` in `to_vec` walks the same list in constant stack:");
    println!("   to_vec = {:?}", to_vec(&list));
    println!("   Rust does not promise tail-call elimination, so the iterative");
    println!("   form is the one to write for a list of unknown length.");

    println!();
    println!("4. Drop order, which the list makes visible");
    {
        let _outer = Loud("outer");
        let _inner = Loud("inner");
        println!("   two values, declared outer then inner:");
    }
    println!("   Last declared, first dropped. The default recursive drop of a");
    println!("   long list has the same stack problem as `total` — which is why");
    println!("   real list types implement Drop by hand, popping in a loop.");

    println!();
    println!("5. The whole point, in one line");
    println!("   Take the Box out of `next: Option<Box<Node>>` and rustc says");
    println!("   \"recursive type `Node` has infinite size\" [E0072], and offers the");
    println!("   fix itself: \"insert some indirection (e.g., a `Box`, `Rc`, or");
    println!("   `&`) to break the cycle\". The type needs a size; a pointer has one.");
}
```
<!-- /source -->

<!-- output:the_box_kata -->
*Verified output of [`the_box_kata.rs`](examples/the_box_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The list
   scores : [5, 3, 0, 4]
   total  : 12

2. Why it costs nothing to say "or nothing"
   size_of::<Box<Node>>()         = 8
   size_of::<Option<Box<Node>>>() = 8
   size_of::<Node>()              = 16
   The Option is the same size as the Box. A Box can never be null,
   so the compiler uses the all-zero bit pattern to mean `None` —
   the null-pointer optimisation. `Option<Box<T>>` is a nullable
   pointer with the null checked by the type system.

3. What the recursion costs
   `total` calls itself once per node, so a list of 100_000 nodes
   would need 100_000 stack frames and overflow the stack. The
   `while let` in `to_vec` walks the same list in constant stack:
   to_vec = [5, 3, 0, 4]
   Rust does not promise tail-call elimination, so the iterative
   form is the one to write for a list of unknown length.

4. Drop order, which the list makes visible
   two values, declared outer then inner:
   dropping inner
   dropping outer
   Last declared, first dropped. The default recursive drop of a
   long list has the same stack problem as `total` — which is why
   real list types implement Drop by hand, popping in a loop.

5. The whole point, in one line
   Take the Box out of `next: Option<Box<Node>>` and rustc says
   "recursive type `Node` has infinite size" [E0072], and offers the
   fix itself: "insert some indirection (e.g., a `Box`, `Rc`, or
   `&`) to break the cycle". The type needs a size; a pointer has one.
```
<!-- /output -->

</details>

---

## See also

- [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) — the null-pointer optimisation, with the `size_of` proof
- [A generic recursive type](../../22_Generics/a_generic_recursive_type/README.md) — the same list with the element type left open
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — what "on the heap" costs to reach
- [`Rc`: the clone that copies a pointer](../../18_Ownership/reference_counting/README.md) — `Box` when one owner is not enough
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — `impl Trait` versus `Box<dyn Trait>`, and when the box is unavoidable
- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — what the second pointer in a fat box is for

## Sources

[Std library types: Box, stack and heap ↗](https://doc.rust-lang.org/rust-by-example/std/box.html) in Rust by Example, and [`std::boxed::Box` ↗](https://doc.rust-lang.org/std/boxed/struct.Box.html). The two rustc transcripts above were produced by compiling the two-line broken versions and are quoted in full.

## Po polsku

`Box<T>` to najprostszy inteligentny wskaźnik (*smart pointer*) w Ruscie: jedna wartość ląduje na stercie, a na stosie zostaje 8 bajtów adresu. Nazwy typu się nie tłumaczy — mówi się „`Box`”, nigdy „pudełko” — i warto wiedzieć, dlaczego po polsku brzmi to obco: polski przekład Tour of Rust urywa się na rozdziale 5, a inteligentne wskaźniki są w rozdziale 8, więc na tym terenie ustalonego polskiego słownictwa po prostu nie ma. Dzięki `Deref<Target = T>` `Box` zachowuje się jak wartość, którą trzyma — pola, metody i `&*b` sięgają przez niego same. Jedynym wyjątkiem jest samo `*b`: to nie pożyczenie, tylko **przeniesienie własności** wartości na zewnątrz i wypuszczenie (*drop*) `Box`a.

Powody, dla których w ogóle się po niego sięga, są dwa i oba dotyczą rozmiaru. Pierwszy: typ, który zawiera sam siebie. Bez `Box`a każdy wariant `Then` zawierałby całe `Round`, które zawiera całe `Round`, i kompilator nie umiałby zapisać rozmiaru — stąd `error[E0072]` i komunikat *„recursive type `Round` has infinite size”*. Zwróć uwagę, co rustc podpowiada: `Box`, `Rc` **albo `&`** — cykl przerywa każde pośrednictwo, a `Box` jest tym, które przy okazji **posiada** to, na co wskazuje; po wstawieniu go `size_of::<Round>()` to 24. Drugi powód: rozmiar znany dopiero w czasie działania. Dwa domknięcia (*closures*) o różnych przechwyceniach mają dwa różne, anonimowe typy, a `Box<dyn Fn(u32) -> u32>` daje im jeden wspólny — kosztem drugiego wskaźnika, na tablicę metod (*vtable*), przez co zajmuje 16 bajtów zamiast 8. Pudełko z `dyn` jest grube, zwykłe `Box<T>` nie.

Na koniec trzy rzeczy praktyczne. `Option<Box<T>>` nie kosztuje ani bajtu więcej niż sam `Box` — `Box` nigdy nie bywa pusty, więc kompilator używa wzorca samych zer na oznaczenie `None` (*null-pointer optimisation*). To dokładnie ten „wskaźnik, który może być pusty” znany z C, tyle że sprawdzenie przeniosło się do systemu typów. Dalej pułapka, o którą łatwo się potknąć: domyślny `Drop` jest **rekurencyjny**, więc długa lista złożona z `Box`ów przepełnia stos w destruktorze, na końcu zasięgu, ze śladem stosu, który nie wskazuje żadnej linii napisanej przez ciebie — dlatego prawdziwe listy implementują `Drop` ręcznie, a strukturę o nieznanej głębokości obchodzi się pętlą `while let`, nie rekurencją. I wreszcie to, czym `Box` nie jest: to pojedyncza własność. Dwóch właścicieli to `Rc`, dwa wątki to `Arc`, a sięganie po `Box` tylko po to, żeby „przenieść coś na stertę”, dokłada alokację i skok w pamięci, nic nie dając w zamian — `Vec`, `String` i `HashMap` i tak już trzymają swoje dane na stercie.

**Szukaj po polsku:** inteligentne wskaźniki w Ruscie · sterta a stos · `rust E0072 recursive type has infinite size` · `rust Box vs Rc vs Arc` · `rust Box<dyn Trait> vtable`
