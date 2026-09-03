# `Rc`: the clone that copies a pointer

**Level:** 201 · working knowledge

**One line:** `Rc<T>` gives one value several owners by counting them, so `Rc::clone` duplicates a pointer and a number and never touches the data — the cheapest `.clone()` in Rust, and the most commonly misread one.

```rust
use std::rc::Rc;

let roster = Rc::new(vec!["Ada".to_string(), "Ben".to_string()]);
let second = Rc::clone(&roster);              // a second owner; nothing copied
println!("{}", Rc::strong_count(&roster));    // 2
```

The [one-owner rule](../ownership_and_moves/README.md) is the default, and `Rc` is its sanctioned exception: several owners, and a count deciding when the value dies. Everything else on this page follows from the count.

## The count is the whole mechanism

One number lives beside the value. `Rc::clone` increments it, dropping an `Rc` decrements it, and the value is freed the moment it reaches zero.

| | count |
|---|---|
| [`Rc::new(v)` ↗](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.new) | 1 |
| [`Rc::clone(&r)` ↗](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.clone) | +1 |
| an `Rc` goes out of scope | −1 |
| the count reaches 0 | `T` is dropped, the allocation is freed |

No owner's scope decides the free on its own. That is the difference from a plain `let`, and it is why `Rc` can hand the same `Vec` to three structs that outlive each other in any order.

## What the clone copied: nothing

Measured, not asserted — section 2 of [the run below](#the-verified-output) counts allocations through a custom global allocator:

```text
(*roster).clone()                  alloc 4     one Vec buffer + one per String
Rc::clone(&roster)                 alloc 0
```

`Rc::ptr_eq` confirms the second names the same allocation. This is the axis [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) settles: `Clone` promises a value you may keep, at whatever depth the type chooses. `Rc` chooses the shallowest one there is.

## `Rc::clone(&x)` and `x.clone()` are the same call

They compile to the same thing. `Rc<T>` implements `Clone` itself, so method resolution finds it before ever reaching the inner type — even when the inner type has its own `clone`:

```rust
let name: Rc<String> = Rc::new("Ada".to_string());
let same = name.clone();                    // clones the Rc, NOT the String
println!("{}", Rc::strong_count(&name));    // 2
let inner: String = (*name).clone();        // this is how you ask for the String
```

The compiler is never confused here. The *reader* is, because both jobs are spelled `.clone()` and the line says nothing about which happened. Write `Rc::clone(&name)` and it does — same code, same cost, one fewer thing to work out.

That is not a hypothetical. *Idiomatic Rust*'s teaching linked list is `Rc<RefCell<ListItem<T>>>`, and a widely circulated summary of the book reports its `.clone()` calls as a deep-copy example; they are refcount bumps. [The four-book comparison](../../16_Structs/copy_vs_clone/README.md#sources) has the details.

## Shared means read-only

An `Rc<T>` derefs to `&T` and stops there:

```text title="Abridged — real rustc output, without the file-and-line header"
error[E0596]: cannot borrow data in an `Rc` as mutable
  |
5 |     roster.push("Cara".to_string());
  |     ^^^^^^ cannot borrow as mutable
  |
  = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Vec<String>>`
```

Two ways forward, and they answer different questions.

**`Rc::get_mut`** hands you a `&mut T` when the count is 1 and `None` otherwise — uniqueness re-earning the write. Useful for building a value before sharing it; useless once it is shared.

**`Rc<RefCell<T>>`** is the pairing to reach for when several owners must write. `Rc` grants the ownership, `RefCell` grants the write, and neither substitutes for the other. The cost is real: the borrow rule moves from compile time to run time, so two live `borrow_mut()`s panic instead of failing to build.

## The one leak safe Rust still permits

Two `Rc`s pointing at each other never reach zero. Nothing is unsafe and nothing warns — the memory is simply unreachable and never freed.

```rust
// a.next -> b and b.back -> a, both strong: counts stop at 1, no Drop runs.
// Make the back edge a Weak and the loop never closes.
```

`Rc::downgrade(&a)` makes a `Weak<T>`, which does not own and does not count. Getting at the value means `upgrade()`, which returns an `Option` — the API stating that the target may already be gone. Child-to-parent is almost always the edge to demote.

## When not to reach for it

- **When one owner would do.** `Rc` is for a value with genuinely several owners whose lifetimes cross. Reaching for it to quiet the borrow checker is the same reflex as reaching for `.clone()`, one indirection further along.
- **When a `&` would do.** A borrow costs nothing and needs no count; `Rc` earns its keep only where the borrow cannot be made to live long enough.
- **For a `Copy` scalar.** `Rc::new(5)` heap-allocates a control block — two `usize` counters sitting beside a four-byte value — so that several owners can share what a register duplicates for free. Tutorials demo `Rc` on an `i32` because [std's own doc example ↗](https://doc.rust-lang.org/std/rc/struct.Rc.html#impl-Clone-for-Rc%3CT,+A%3E) does; that example is being brief, not giving advice. The types worth counting are the ones that own a heap buffer.
- **Across threads.** `Rc` is deliberately not [`Send`](../../12_Traits/marker_traits/README.md) — `assert_send::<Rc<i32>>()` is `E0277` while the `Arc` spelling compiles, on two values of the same size. [`Arc`](../sharing_across_threads/README.md) is the atomic version, and the compiler enforces the split.

## If you are coming from another language

**Python.** This is the model you already have. CPython refcounts every object, `sys.getrefcount(x)` is `Rc::strong_count`, and `b = a` is `Rc::clone` — a new name for one object, no copy. Three things change. The count is **per type, not per language**: Rust refcounts only what you wrap in an `Rc`, and everything else is decided at compile time with no runtime number at all. The clone is **explicit**: Python's assignment silently shares and `copy.deepcopy` is the unusual spelling, while Rust makes you write `Rc::clone` and moves by default. And Python's cycle collector eventually reclaims a reference cycle, where Rust's does not — `Rc` is CPython's refcounting *without* the `gc` module behind it, which is why `Weak` is your job rather than the runtime's.

**ABAP.** A data reference (`REF TO`) is the closest thing, and the resemblance is in the sharing rather than the counting: `lr_b = lr_a` gives two references to one object, exactly like `Rc::clone`, while a structure or internal table assignment copies deeply. ABAP's garbage collector then frees the object when the last reference goes away, cycles included, and you never see a number. Rust replaces that with an explicit count you can print — which is more work and buys you the timing: an `Rc` value drops at a knowable point, not whenever a collector next runs. Note that the deep-copy default runs the other way round from Rust's: `ls_b = ls_a` copies, `lr_b = lr_a` shares, and in Rust the sharing form is the one that needs a wrapper type.

**C++.** `Rc<T>` is `std::shared_ptr<T>` with the atomics removed, and `Arc<T>` is `shared_ptr` proper; `Weak<T>` is `std::weak_ptr<T>`, `upgrade()` is `lock()`. The API maps almost one to one, and two things do not. `shared_ptr` gives you `T&` and `T*` freely, so shared-and-mutable is available and unpoliced — the whole `RefCell` question above simply does not arise in C++, and neither does the protection. And `shared_ptr`'s control block is *always* atomic even in single-threaded code, which is the cost Rust splits into two types and lets you decline. Coming from C++ the trap is reaching for `Arc` reflexively because `shared_ptr` was thread-safe by default; `Rc` is the right one until a value actually crosses a thread.

## Practice

**Predict the count four times, then find the edge that leaks.** One roster, three scoreboards, and a team whose members point back at it.

1. Write `Rc<Vec<String>>` shared by three `Scoreboard` structs. Predict `strong_count` after the `Rc::new`, after two scoreboards exist, inside a block holding a third, and after that block ends. Then print it at all four points.
2. Replace one `Rc::clone` with `(*roster).clone()`. Say which of the four numbers changes and why, before running it — then name the bug that version makes writable.
3. Give a `Team` a `Vec<Rc<Member>>` and each `Member` a back-reference to its team. Predict whether `Drop` runs when the team goes out of scope, first with `Rc<Team>` on the back edge and then with `Weak<Team>`. Check both.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:reference_counting_kata -->
*[`reference_counting_kata.rs`](examples/reference_counting_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: predict the count four times, then find the edge that leaks.
//!
//!   rustc --edition 2024 reference_counting_kata.rs -o /tmp/rck && /tmp/rck

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Part 1 and 2: one roster, several readers.
struct Scoreboard {
    roster: Rc<Vec<String>>,
    points: Vec<u32>,
}

impl Scoreboard {
    fn new(roster: &Rc<Vec<String>>) -> Self {
        Scoreboard { roster: Rc::clone(roster), points: vec![0; roster.len()] }
    }
    fn score(&mut self, i: usize) {
        self.points[i] += 1;
    }
    fn leader(&self) -> &str {
        let best = self
            .points
            .iter()
            .enumerate()
            .max_by_key(|(i, v)| (**v, std::cmp::Reverse(*i)))
            .map(|(i, _)| i)
            .unwrap();
        &self.roster[best]
    }
}

/// Part 3: a team owns its members, and each member refers back to it.
struct Team {
    name: &'static str,
    members: RefCell<Vec<Rc<Member>>>,
}

struct Member {
    name: &'static str,
    /// The back edge. `Weak` is the answer; `Rc<Team>` here is the leak.
    team: RefCell<Weak<Team>>,
}

impl Drop for Team {
    fn drop(&mut self) {
        println!("     drop ran for team {}", self.name);
    }
}

impl Drop for Member {
    fn drop(&mut self) {
        println!("     drop ran for member {}", self.name);
    }
}

fn enroll(team: &Rc<Team>, name: &'static str) {
    let member = Rc::new(Member { name, team: RefCell::new(Weak::new()) });
    *member.team.borrow_mut() = Rc::downgrade(team);
    team.members.borrow_mut().push(member);
}

fn main() {
    println!("Part 1 — predict the count at four points.\n");

    let roster = Rc::new(vec!["Ada".to_string(), "Ben".to_string(), "Cara".to_string()]);
    println!("  (a) let roster = Rc::new(..)        predicted 1  actual {}", Rc::strong_count(&roster));

    let mut morning = Scoreboard::new(&roster);
    let mut evening = Scoreboard::new(&roster);
    println!("  (b) two Scoreboard::new(&roster)    predicted 3  actual {}", Rc::strong_count(&roster));

    {
        let _spare = Scoreboard::new(&roster);
        println!("  (c) a third, inside a block         predicted 4  actual {}", Rc::strong_count(&roster));
    }
    println!("  (d) the block ended                 predicted 3  actual {}", Rc::strong_count(&roster));

    println!("\n  The count tracks OWNERS, not uses. `morning` and `evening` are");
    println!("  still alive at (d), so the roster is; `_spare` is not, so its");
    println!("  share went back. The Vec is freed when the last one leaves, and");
    println!("  no single owner's scope decides that.");

    morning.score(0);
    morning.score(0);
    morning.score(1);
    evening.score(2);
    evening.score(2);
    evening.score(2);
    println!("\n  Three Scoreboard values, one roster, three names stored once:");
    println!("    morning leader {}   evening leader {}", morning.leader(), evening.leader());

    println!("\nPart 2 — swap one Rc::clone for a deep clone. Which numbers move?\n");
    let independent: Vec<String> = (*roster).clone();
    println!("  Rc::strong_count(&roster)   {}   <- unchanged: a deep clone", Rc::strong_count(&roster));
    println!("                                   makes a value with NO owner in");
    println!("                                   common with this one.");
    println!("  same buffer as the roster?  {}", roster.as_ptr() == independent.as_ptr());
    println!("  Three fresh Strings and a fresh Vec, so a name edited here would");
    println!("  not be seen by any Scoreboard. That divergence is the bug the count");
    println!("  was preventing, and it compiles either way.");

    println!("\nPart 3 — the back edge, and whether Drop runs.\n");
    println!("  With `team: RefCell<Rc<Team>>` the prediction is NOTHING");
    println!("  prints: the team owns each member and each member owns the");
    println!("  team, so both counts stop at 1 and neither reaches zero.");
    println!("  `Weak` breaks it — a member can SEE its team without owning it:\n");
    {
        let riverside = Rc::new(Team { name: "Riverside", members: RefCell::new(Vec::new()) });
        enroll(&riverside, "Ada");
        enroll(&riverside, "Ben");
        let ada = Rc::clone(&riverside.members.borrow()[0]);
        let seen = ada.team.borrow().upgrade().map(|t| t.name);
        println!("    Ada can still reach her team: {seen:?}");
        println!("    strong team {}   weak team {}   strong Ada {}",
                 Rc::strong_count(&riverside),
                 Rc::weak_count(&riverside),
                 Rc::strong_count(&ada));
        println!("    leaving the block:");
    }
    println!("\n  All three ran, in owner order: the team's count hit zero");
    println!("  first, which dropped its Vec, which dropped the members.");
    println!("  The rule to carry away: a cycle needs every edge to be an owner,");
    println!("  so make exactly one of them an observer. Child-to-parent is");
    println!("  almost always the one to demote.");
}
```
<!-- /source -->

<!-- output:reference_counting_kata -->
*Verified output of [`reference_counting_kata.rs`](examples/reference_counting_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Part 1 — predict the count at four points.

  (a) let roster = Rc::new(..)        predicted 1  actual 1
  (b) two Scoreboard::new(&roster)    predicted 3  actual 3
  (c) a third, inside a block         predicted 4  actual 4
  (d) the block ended                 predicted 3  actual 3

  The count tracks OWNERS, not uses. `morning` and `evening` are
  still alive at (d), so the roster is; `_spare` is not, so its
  share went back. The Vec is freed when the last one leaves, and
  no single owner's scope decides that.

  Three Scoreboard values, one roster, three names stored once:
    morning leader Ada   evening leader Cara

Part 2 — swap one Rc::clone for a deep clone. Which numbers move?

  Rc::strong_count(&roster)   3   <- unchanged: a deep clone
                                   makes a value with NO owner in
                                   common with this one.
  same buffer as the roster?  false
  Three fresh Strings and a fresh Vec, so a name edited here would
  not be seen by any Scoreboard. That divergence is the bug the count
  was preventing, and it compiles either way.

Part 3 — the back edge, and whether Drop runs.

  With `team: RefCell<Rc<Team>>` the prediction is NOTHING
  prints: the team owns each member and each member owns the
  team, so both counts stop at 1 and neither reaches zero.
  `Weak` breaks it — a member can SEE its team without owning it:

    Ada can still reach her team: Some("Riverside")
    strong team 1   weak team 2   strong Ada 2
    leaving the block:
     drop ran for team Riverside
     drop ran for member Ada
     drop ran for member Ben

  All three ran, in owner order: the team's count hit zero
  first, which dropped its Vec, which dropped the members.
  The rule to carry away: a cycle needs every edge to be an owner,
  so make exactly one of them an observer. Child-to-parent is
  almost always the one to demote.
```
<!-- /output -->

</details>

## The verified output

<!-- output:reference_counting -->
*Verified output of [`reference_counting.rs`](examples/reference_counting.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The count is the whole mechanism
   after Rc::new                      1
   after Rc::clone                    2
   a third owner, inside a block      3
   the block ended, so its owner left 2
   The value is freed at zero, not at the end of any one owner's scope.

2. What the clone copied: nothing
   (*roster).clone()                  alloc 4
   Rc::clone(&roster)                 alloc 0
   same allocation as the original?
     Rc::clone   true
     deep clone  false   <- a separate Vec, and separate Strings inside it
   One Vec buffer plus one buffer per String, against nothing at all.

3. `Rc::clone(&x)` and `x.clone()` are the same call
   Rc<String>, cloned with .clone():
     strong_count 2   same String? true
   The String was NOT cloned. `Rc<T>` implements `Clone` itself, so
   method resolution finds it first and never reaches `String::clone`.
   Both spellings compile to that. Only one of them says so on the line.
   To reach the String you have to ask: (*name).clone() = "Ada"

4. An `Rc` hands out `&T`, so uniqueness is what buys a write
   strong_count 1 -> get_mut gave a &mut, pushed: [1, 2, 3, 4]
   strong_count 2 -> get_mut is None
   Shared and mutable is the pair Rust never hands out for free.

5. `Rc<RefCell<T>>` is how a shared value gets written to
   two owners wrote through the same cell: 8
   `Rc` grants the ownership; `RefCell` grants the write. Neither
   substitutes for the other, and the borrow rule moves to run time.

6. The one leak safe Rust still permits
   a <-> b, back edge STRONG:
     b's back edge: strong -> a, an owner
     while alive: strong a 2  b 2   weak a 0
     leaving the scope, so both locals are about to drop:
     ...nothing printed. Each still holds the other at 1, so neither
     reaches zero. Both nodes are unreachable and never freed.
   a <-> b, back edge WEAK:
     b's back edge: weak -> a, upgrade() gave Some
     while alive: strong a 1  b 2   weak a 1
     leaving the scope, so both locals are about to drop:
     drop ran for a
     drop ran for b
     Both ran. A `Weak` does not own, so the loop never closes.
   `Rc::downgrade` makes one; `upgrade()` returns an Option, because
   the value it points at may already be gone.
```
<!-- /output -->

## See also

- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — where the trait's promise is settled: an owned value, at a depth the *type* chooses
- [Ownership and moves](../ownership_and_moves/README.md) — the one-owner rule this type is the sanctioned exception to
- [`Cow`: borrow until somebody writes](../clone_on_write/README.md) — the other way out of a copy, decided by the data rather than by the owner count
- [Sharing across threads: `Arc`](../sharing_across_threads/README.md) — the same counter made atomic, and the `Send` refusal that separates them
- [`ToOwned`](../../12_Traits/to_owned/README.md) — where the same misreading bites hardest
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the counting allocator section 2 measures with
- [`Rc` ↗](https://doc.rust-lang.org/std/rc/struct.Rc.html) · [`Weak` ↗](https://doc.rust-lang.org/std/rc/struct.Weak.html) · [`RefCell` ↗](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

## Po polsku

`Rc<T>` (od *reference counted* — ze zliczaniem referencji) daje jednej wartości **wielu właścicieli**, licząc ich. `Rc::clone` kopiuje wskaźnik i zwiększa licznik o jeden — i **nie dotyka danych**. To najtańszy `.clone()` w Ruscie i zarazem najczęściej źle odczytywany, bo słowo „klonowanie” w polskich materiałach niemal zawsze znaczy „głęboka kopia”.

Stąd konwencja, którą warto stosować: pisz `Rc::clone(&x)`, a nie `x.clone()`. To dokładnie to samo wywołanie, ale forma z nazwą typu **mówi czytelnikowi, że to tanie** — że kopiowany jest wskaźnik i licznik, a nie zawartość.

Dane pod `Rc` są **tylko do odczytu**. Współdzielenie oznacza brak zapisu — żeby pisać, trzeba dołożyć mutowalność wewnętrzną (`Cell`, `RefCell`), a przy wielu wątkach sięgnąć po `Arc` z `Mutex`em.

`Rc` to również jedyne miejsce, gdzie bezpieczny Rust wciąż pozwala na **wyciek pamięci**: dwa obiekty wskazujące na siebie nawzajem nigdy nie zejdą do zera i nigdy nie zostaną zwolnione. Lekarstwem jest `Weak` — referencja, która nie liczy się do licznika. Warto wiedzieć, że wyciek pamięci nie jest w Ruscie uznawany za naruszenie bezpieczeństwa; niebezpieczne jest użycie zwolnionej pamięci, nie jej niezwolnienie.

**Szukaj po polsku:** zliczanie referencji · `Rc` Rust · cykle referencji · mutowalność wewnętrzna · `rust Rc Weak reference cycle`
