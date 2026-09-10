# What a clone costs

**Level:** 201 · working knowledge

**One line:** `#[derive(Clone)]` clones every field, so a struct's `.clone()` costs what its fields cost — two allocations for a record holding two `String`s, none for the same record holding `Arc<str>` — and the call site reads the same either way.

```rust
use std::sync::Arc;

#[derive(Clone)]
struct Person { id: u64, name: String, email: String }

#[derive(Clone)]
struct SharedPerson { id: u64, name: Arc<str>, email: Arc<str> }

let a = Person { id: 1, name: "alice".into(), email: "alice@example.com".into() };
let s = SharedPerson { id: 1, name: Arc::from("alice"), email: Arc::from("alice@example.com") };

let b = a.clone();   // alloc 2: a new buffer for each String
let t = s.clone();   // alloc 0: two reference counts go up
```

Every count on this page is taken by [a counting global allocator](../../09_Advanced/the_global_allocator/README.md) in [the run below](#the-verified-output). The timings further down are one machine's, and say which.

## Add up the fields

`Clone` promises an independent value and leaves each type to decide what that takes — [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) has the argument. A derived `Clone` calls every field's `clone` in turn, so a struct costs the sum:

| Field type | Its clone | Allocations |
|---|---|---|
| `u64`, or any `Copy` type | copies the bits | 0 |
| `String` | a new buffer, the bytes copied in | 1 |
| `Box<u64>` | a new box | 1 |
| `Vec<String>` of 3 | the Vec's buffer, then each `String` | 4 |
| [`Rc<str>`](../reference_counting/README.md) | one count goes up | 0 |
| [`Arc<str>`](../sharing_across_threads/README.md) | one count goes up, atomically | 0 |

`Person` is 0 + 1 + 1. `SharedPerson` is 0 + 0 + 0.

## The update that clones a field and throws it away

Changing one field while keeping the original can be written three ways:

| Spelling | Allocations |
|---|---|
| `Person { name, ..a.clone() }` | 2 |
| `Person { id: a.id, name, email: a.email.clone() }` | 1 |
| `Person { name, ..a }`, with `a` used up | 0 |

`..a.clone()` clones all of `a` — `name` too — and drops the cloned `name` unread in the same statement. Naming the fields you keep clones only those. The move clones nothing, so when the old value is finished with, the functional-update style is free; [struct update syntax](../../16_Structs/struct_update/README.md) covers which fields a `..a` leaves readable. With `Arc<str>` fields the first spelling costs 0 as well.

## Making the fields cheap

Swap `String` for `Arc<str>` and `Vec<T>` for `Arc<[T]>`, and every clone is count increments. Three costs come with the swap.

- **Converting copies once.** `Arc::<str>::from(s)` [allocates and copies the text ↗](https://doc.rust-lang.org/std/sync/struct.Arc.html#impl-From%3CString%3E-for-Arc%3Cstr%3E): the counts sit in front of the bytes, so the `String`'s buffer cannot be adopted. `Arc<[T]>::from(v)` allocates once and moves the items in, so a `Vec<String>`'s strings are not copied again — the kata counts it.
- **Shared means read-only.** An `Arc` hands out `&T`. [`Arc::make_mut` ↗](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut) is the write: while another handle exists it clones the whole value first — five allocations for a `Vec` of three `String`s — and once the handle is unique it clones nothing. Clone on write, decided by the count; [`Cow`](../clone_on_write/README.md) makes the same decision by its variant.
- **The count is atomic.** `Rc` keeps the same count with a plain increment when nothing crosses a thread, and the timings below price the difference.

For a collection cloned often and changed a little at a time, persistent collections such as [`imbl` ↗](https://docs.rs/imbl) share structure between versions: a clone is O(1) and a change copies only the path to it. It is a crate, so nothing on this page runs it.

## The call site cannot tell you which one you got

`b = a.clone()` above allocated twice and `t = s.clone()` not at all, and neither line says so.

- **For a pointer, there is a convention.** Write `Arc::clone(&x)`, not `x.clone()`. [The Book recommends it ↗](https://doc.rust-lang.org/book/ch15-04-rc.html) so that a search for expensive clones can skip the refcount ones; the two spellings compile to the same call. Clippy's [`clone_on_ref_ptr` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_ref_ptr) enforces it from the `restriction` group, so it stays off until you ask for it (`clippy-driver -Whelp`, 1.98).
- **For a struct of `Arc`s, std has nothing.** `s.clone()` is cheap and says so nowhere. Crates add a marker trait for exactly this: Meta's [`dupe` ↗](https://docs.rs/dupe) — a `Dupe` trait and a `.dupe()` method — and the newer [`light_clone` ↗](https://docs.rs/light_clone), built to reject a field like `String` at compile time.
- **The language is working on it.** [RFC 3680 ↗](https://github.com/rust-lang/rfcs/pull/3680) proposed `x.use`, which clones only a type that has opted in — a `String` is moved instead, as the run below shows — so it cannot hide a deep copy. It is on nightly as `ergonomic_clones` ([tracking issue ↗](https://github.com/rust-lang/rust/issues/132290)). The 2026 [ergonomic ref-counting goal ↗](https://goals.rust-lang.org/2026/ergonomic-rc.html) has since settled on a `Share` trait — for `Arc`, `Rc` and `&T`, not for `String` or `Vec` — with nightly prototypes targeted for summer 2026. None of it is stable.

The nightly feature, tried on the pinned 1.98.0 compiler with `RUSTC_BOOTSTRAP=1`:

```rust,ignore
#![feature(ergonomic_clones)] // nightly only
use std::rc::Rc;

fn main() {
    let rc = Rc::new(String::from("shared"));
    let also = rc.use; // Rc opted in: a clone, and the count goes up
    println!("{}", Rc::strong_count(&also)); // 2
    let s = String::from("owned");
    let t = s.use; // String did not: this is a move
    println!("{t}"); // owned
    // println!("{s}"); // E0382
}
```

Uncomment the last line and the `String` turns out to have moved:

```text title="Abridged — real rustc 1.98.0 output, without the incomplete-feature warning, the help and the summary"
error[E0382]: borrow of moved value: `s`
  --> use_moves_a_string.rs:11:16
   |
 8 |     let s = String::from("owned");
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
 9 |     let t = s.use; // String did not: this is a move
   |             ----- value moved here
10 |     println!("{t}"); // owned
11 |     println!("{s}"); // E0382
   |                ^ value borrowed here after move
```

## What it costs in time

*Measured 2026-09-10 on one machine — Intel i5-10500, macOS 26 — as the best of seven runs; where the Rust side was run twice, both results are shown. rustc 1.98.0 at `-C opt-level=3` with the system allocator, CPython 3.14.7, Node 20.20.2, C# 9 and F# 5 on .NET 5.0.5. The record has five fields, two of them strings, and an update changes one field and keeps the original. A time belongs to the machine and its allocator; the counts above do not.*

| Operation | Time |
|---|---|
| Rust, `String` fields: `p.clone()` — two allocations | 148–164 ns |
| Rust, `String` fields: update, cloning the new name and the kept email — two | 150–159 ns |
| Rust, `String` fields: the same update written `..p.clone()` — three | 256–284 ns |
| Rust, `String` fields: `Person { name, ..p }`, the original used up — none | 3 ns |
| Rust, `Arc<str>` fields: the update — no allocation, four count changes | 22 ns |
| Rust, `Rc<str>` fields: the update | 4–5 ns |
| C#, record class: `p with { Name = bob }` | 15 ns |
| C#, a struct copied and one field set — what a `record struct`'s `with` does | 5.5 ns |
| F#: `{ p with Name = bob }` | 13 ns |
| JavaScript on Node: `{ ...p, name: bob }` | 20 ns |
| Python: `dataclasses.replace(p, name=bob)` | 1,455 ns |
| Python: `Person(p.id, …, bob, p.email)` written out | 162 ns |

- **The Rust cost is the allocator.** Two malloc/free pairs are most of 150 ns; the move that needs neither is 3.
- **The garbage-collected rows copy references**, so the fair Rust row beside them is `Arc`'s — [the next section](#the-argument-you-will-meet) shows what that sharing buys and what it costs.
- **An atomic count is slower than a plain one, and much slower under contention.** `Rc`'s update is 4–5 ns to `Arc`'s 22. The same `Arc` cloned and dropped from several threads at once:

| Threads | Time per clone and drop |
|---|---|
| 1 | 10–11 ns |
| 2 | 48–55 ns |
| 4 | 134–137 ns |
| 6 | 207–208 ns |

A collection scales the same way. Copying a list of 10,000 integers took 1.5 µs in Rust (`Vec<i64>::clone`), 4.6 µs in C# (`long[].Clone()`), 8.4 µs on Node (`slice()`) and 22 µs in Python (`list()`). Cloning a `Vec<String>` of 10,000 took 1.1 ms — 10,001 allocations — where any of the other three would have copied one reference.

## The argument you will meet

A widely shared 2026 post, [*The Problem with Clones in Rust* ↗](https://hamy.xyz/blog/2026-02_the-problem-with-clones-in-rust) (also [a video ↗](https://www.youtube.com/watch?v=5SntrSo8VMw)), argues that functional-style Rust is slow because Rust deep-copies by default while C#, F#, TypeScript and Python copy cheaply — putting naive Rust at three to five times their cost, and sometimes behind Python. Its remedy is this page's: `Arc<str>`, `Arc<[T]>`, persistent collections, and a trait that makes a cheap clone checkable. Two of its premises do not survive the counts.

**Rust does not deep-copy by default; it moves.** [The Book ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) is flat about it: "Rust will never automatically create “deep” copies of your data." Every clone on this page is written out, and the update that needs none — `..a` — took 3 ns.

**The garbage-collected languages copy references, which is a different operation.** Their copy of a record is shallow: one new object, every field pointing at the same data as before. Run in C#, the post's own `with` example allocates one 32-byte object — the new name is an interned literal, not a new string — and shares the tag list, so a write through the copy lands in the original:

<details markdown="1">
<summary>The C# program, and what it printed</summary>

```csharp
using System;
using System.Collections.Generic;

public record Person(string Name, List<string> Tags);

public static class Program
{
    public static void Main()
    {
        var tags = new List<string> { "admin" };
        var p1 = new Person("alice", tags);
        var warm = p1 with { Name = "warm-up" };

        long before = GC.GetAllocatedBytesForCurrentThread();
        var p2 = p1 with { Name = "bob" };
        long after = GC.GetAllocatedBytesForCurrentThread();

        Console.WriteLine($"heap bytes allocated by `p1 with {{ Name = \"bob\" }}`: {after - before}");
        Console.WriteLine($"p2 is a new object:                  {!ReferenceEquals(p1, p2)}");
        Console.WriteLine($"p2.Name is the interned literal:     {ReferenceEquals(p2.Name, "bob")}");
        Console.WriteLine($"p2.Tags is p1.Tags (shared list):    {ReferenceEquals(p1.Tags, p2.Tags)}");
        p2.Tags.Add("oops");
        Console.WriteLine($"after p2.Tags.Add(\"oops\"), p1.Tags: [{string.Join(", ", p1.Tags)}]");
    }
}
```

```text title="Measured 2026-09-10 — C# 9 on .NET 5.0.5"
heap bytes allocated by `p1 with { Name = "bob" }`: 32
p2 is a new object:                  True
p2.Name is the interned literal:     True
p2.Tags is p1.Tags (shared list):    True
after p2.Tags.Add("oops"), p1.Tags: [admin, oops]
```

</details>

That sharing is `Arc`'s job in Rust, where it comes with the write refused — an `Arc<[String]>` hands out `&[String]`, which has no `push`. Against that row the gap closes: 22 ns beside 13–20. On the other side, Python's `dataclasses.replace` takes ten times as long as Rust's full deep copy. Rust falls behind only when it deep-copies what the other language merely points at, as with the 10,000-string `Vec` above — an O(1) operation turned into an O(n) one by the program, not a constant charged by the language.

## When not to reach for `Arc` fields

- **When a borrow would do.** A `&Person` costs nothing — no allocation, no count. Much cloning-to-compile is this case; [how to learn lifetimes](../how_to_learn_lifetimes/README.md) is the page about that habit.
- **When the old value is finished with.** A move is the free update, and so is a method that takes `self` and returns the changed value.
- **When the value is written more than it is copied.** Every write goes through `make_mut`: a check each time, and a whole copy whenever the value is shared.
- **When many threads clone the same one in a hot loop.** Every clone and drop writes the same counter, and six threads made each one take 207 ns instead of 10.

## If you are coming from another language

**Python.** `copy.copy(p)` and `dataclasses.replace(p, …)` copy references: the new object's fields point at the old object's values. The Rust type that behaves like that is a struct of `Rc` or `Arc` fields. A derived `Clone` over `String` fields is `copy.deepcopy` instead — and it is not the slow one: 0.15 µs against `deepcopy`'s 4.9 µs on the machine above. What moves is where the choice is made. Python makes it per call, `copy` or `deepcopy`; Rust makes it once per field, in the struct's type, and `.clone()` then means whatever the fields say.

**ABAP.** The kernel already does what `Arc::make_mut` does. Assigning a string or an internal table shares the buffer and copies it on the first write to either side, so `lt_b = lt_a` is cheap until somebody changes one of them. Rust's `String` has no such sharing — its `clone` copies now, every time. The combination that behaves like ABAP is `Arc<str>` or `Arc<Vec<T>>` with `make_mut`: the same sharing, written out where ABAP does it for you.

**C#, F#, JavaScript, TypeScript.** `p with { … }`, `{ p with … }` and `{ ...p }` are shallow copies: one new object, every reference copied, the data shared. That is a Rust struct of `Arc` fields, and it costs about the same. What Rust adds is the refusal — data behind an `Arc` is read-only, so the aliasing write in the C# program above does not compile; you ask for `make_mut` and pay for your own copy, or for a `Mutex` and pay for the lock.

## Practice

**Price an order before you run it.** An `Order` has an `id: u64`, a `customer: String` and three items in a `Vec<String>`.

1. Predict the allocations for `order.clone()`, for changing the customer written `Order { customer, ..order.clone() }`, and for the same change written field by field. Then count them.
2. Convert the order to `Arc<str>` and `Arc<[String]>` fields. Predict what the conversion costs, once, and what a clone and the same update cost afterwards. One number in the conversion is lower than it looks — which one, and why?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_a_clone_costs_kata -->
*[`what_a_clone_costs_kata.rs`](examples/what_a_clone_costs_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: price an order before you run it.
//!
//!   rustc --edition 2024 what_a_clone_costs_kata.rs -o /tmp/wacck && /tmp/wacck

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Run `work`, then print the prediction beside what the allocator saw.
fn check<T>(label: &str, predicted: usize, work: impl FnOnce() -> T) -> T {
    let before = ALLOCS.load(Relaxed);
    let out = work();
    let actual = ALLOCS.load(Relaxed) - before;
    let verdict = if actual == predicted { "" } else { "   <- recount" };
    println!("  {label:<44} predicted {predicted:>2}   actual {actual:>2}{verdict}");
    out
}

#[derive(Clone)]
struct Order {
    id: u64,
    customer: String,
    items: Vec<String>,
}

#[derive(Clone)]
struct SharedOrder {
    id: u64,
    customer: Arc<str>,
    items: Arc<[String]>,
}

fn main() {
    println!("An order: an id, a customer, three items.\n");
    let order = Order {
        id: 7,
        customer: "Acme".to_string(),
        items: vec!["bolts".to_string(), "nuts".to_string(), "washers".to_string()],
    };

    println!("Part 1 — String and Vec<String> fields");
    let copy = check("order.clone()", 5, || order.clone());
    let globex = String::from("Globex");
    let lazy = check("Order { customer, ..order.clone() }", 5, || Order { customer: globex, ..order.clone() });
    let globex = String::from("Globex");
    let careful = check("field by field, cloning only items", 4, || Order {
        id: order.id,
        customer: globex,
        items: order.items.clone(),
    });
    println!("  1 + 1 + 3: the customer, the Vec's buffer, one per item. The lazy");
    println!("  update pays all five and drops the cloned \"{}\" unread; the", copy.customer);
    println!("  careful one skips it. Both give {} with {} items.\n", lazy.customer, careful.items.len());

    println!("Part 2 — convert once, then clone for free");
    let shared = check("SharedOrder from the Order (consumes it)", 2, move || SharedOrder {
        id: order.id,
        customer: Arc::from(order.customer),
        items: Arc::from(order.items),
    });
    let again = check("shared.clone()", 0, || shared.clone());
    let globex: Arc<str> = Arc::from("Globex");
    let renamed = check("SharedOrder { customer, ..shared.clone() }", 0, || SharedOrder {
        customer: Arc::clone(&globex),
        ..shared.clone()
    });
    println!("  The conversion is 2, not 5. Arc<str> copies the customer's bytes,");
    println!("  but Arc<[String]> MOVES the three Strings into its buffer, and each");
    println!("  keeps the heap text it already had. After that, counts only:");
    println!(
        "  #{} {} and #{} {} share one item list: {}",
        again.id,
        again.customer,
        renamed.id,
        renamed.customer,
        Arc::ptr_eq(&again.items, &renamed.items)
    );
}
```
<!-- /source -->

<!-- output:what_a_clone_costs_kata -->
*Verified output of [`what_a_clone_costs_kata.rs`](examples/what_a_clone_costs_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
An order: an id, a customer, three items.

Part 1 — String and Vec<String> fields
  order.clone()                                predicted  5   actual  5
  Order { customer, ..order.clone() }          predicted  5   actual  5
  field by field, cloning only items           predicted  4   actual  4
  1 + 1 + 3: the customer, the Vec's buffer, one per item. The lazy
  update pays all five and drops the cloned "Acme" unread; the
  careful one skips it. Both give Globex with 3 items.

Part 2 — convert once, then clone for free
  SharedOrder from the Order (consumes it)     predicted  2   actual  2
  shared.clone()                               predicted  0   actual  0
  SharedOrder { customer, ..shared.clone() }   predicted  0   actual  0
  The conversion is 2, not 5. Arc<str> copies the customer's bytes,
  but Arc<[String]> MOVES the three Strings into its buffer, and each
  keeps the heap text it already had. After that, counts only:
  #7 Acme and #7 Globex share one item list: true
```
<!-- /output -->

</details>

## The verified output

<!-- output:what_a_clone_costs -->
*Verified output of [`what_a_clone_costs.rs`](examples/what_a_clone_costs.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. What a clone costs is decided by the type
   u64           the bits, copied                     alloc 0
   String        a new buffer, the bytes copied in    alloc 1
   Box<u64>      a new box                            alloc 1
   Vec<String>   the buffer, then each of 3 Strings   alloc 4
   Rc<str>       one count goes up                    alloc 0
   Arc<str>      one count goes up, atomically        alloc 0
   One trait, one method name, and anything from nothing to 1 + n.

2. A derived Clone clones every field, so add up the fields
   Person { u64, String, String }.clone()             alloc 2
   equal text: true   same buffer: false
   SharedPerson { u64, Arc<str>, Arc<str> }.clone()   alloc 0
   equal text: true   same buffer: true
   0 + 1 + 1 against 0 + 0 + 0, and both lines read `.clone()`.
   Arc::<str>::from(a String you already have)        alloc 1
   Switching costs one copy, once: an Arc keeps its counts in front
   of the text, so the String's buffer cannot be adopted as it is.

3. Change one field, keep the original: three spellings
   Person { name, ..a.clone() }                       alloc 2
   Person { id: a.id, name, email: a.email.clone() }  alloc 1
   Person { name, ..spare }   (spare is used up)      alloc 0
   All three hold bob <alice@example.com>.
   `..a.clone()` clones a.name as well, and drops it unread.
   Naming the kept fields clones only those. Moving clones nothing.
   SharedPerson { name, ..s.clone() }                 alloc 0
   With Arc fields the careless spelling is free too: #1 bob <alice@example.com>

4. A collection multiplies it
   Vec<String> of 1,000: .clone()                     alloc 1001
   Arc<[String]> of 1,000: Arc::clone                 alloc 0
   1000 tags either way: one buffer per String plus one for the Vec,
   against a single increment.
   Arc::make_mut, another handle alive                alloc 5
   Arc::make_mut, now the only handle                 alloc 0
   Clone on write: the first write while shared pays for all of it,
   1 for the Arc + 1 for the Vec + 1 per String. The copy is unique
   after that, so the next write pays nothing. `other` kept 3 items.
```
<!-- /output -->

## See also

- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — why `Clone` promises ownership rather than depth
- [Struct update syntax](../../16_Structs/struct_update/README.md) — which fields a `..base` moves, and which stay readable
- [`Rc`: the clone that copies a pointer](../reference_counting/README.md) — the count, and the `Rc::clone(&x)` spelling
- [Sharing across threads: `Arc`](../sharing_across_threads/README.md) — what the atomic buys, before this page prices it
- [`Cow`: borrow until somebody writes](../clone_on_write/README.md) — clone on write, decided by the variant rather than the count
- [Stack and heap](../stack_and_heap/README.md) — a move, a `Copy`, a `clone` and an `Arc::clone`, against the heap side
- [How to learn lifetimes](../how_to_learn_lifetimes/README.md) — "clone everything", and when a borrow was the answer
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the counter behind every number above
- [`Clone` ↗](https://doc.rust-lang.org/std/clone/trait.Clone.html) · [`Arc::make_mut` ↗](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut)

## Po polsku

Klonowanie (*cloning*) nie ma w Ruście jednej ceny. `#[derive(Clone)]` klonuje strukturę pole po polu, więc koszt `.clone()` to suma kosztów jej pól: `u64` kopiuje się bit po bicie, `String` (łańcuch znaków) dostaje nowy bufor na stercie (*heap*) i kopię bajtów, a `Arc<str>` tylko zwiększa licznik referencji (*reference count*). To samo `.clone()` kosztuje więc zero alokacji albo tysiąc — i sama linijka kodu tego nie zdradza. Decyduje typ pola.

W polskich materiałach „klonowanie” niemal zawsze znaczy „głęboka kopia” (*deep copy*). W Ruście to prawda tylko dla typów, które same posiadają bufor: `String`, `Vec`, `Box`. Klon `Rc` i `Arc` jest płytki (*shallow*) — kopiuje wskaźnik, nie dane.

Stąd porównania z C#, JavaScriptem czy Pythonem, które krążą w sieci, często mierzą dwie różne operacje. Tamte języki, kopiując rekord, kopiują referencje; rustowym odpowiednikiem nie jest pole `String`, tylko `Arc<str>` — i wtedy różnica znika. Najtańsza aktualizacja to zresztą przeniesienie własności (*move*): `Person { name, ..p }` nie kopiuje niczego, bo `p` nie jest już potrzebne. `Arc::make_mut` to z kolei kopiowanie przy zapisie (*copy-on-write*): płaci się dopiero przy pierwszym zapisie do wartości, którą ktoś jeszcze współdzieli.

**Szukaj po polsku:** koszt klonowania w Ruście · głęboka a płytka kopia · zliczanie referencji · kopiowanie przy zapisie · `rust clone cost` · `rust Arc<str> vs String`
