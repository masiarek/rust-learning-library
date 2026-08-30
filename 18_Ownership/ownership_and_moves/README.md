# Ownership and moves

**Level:** 101 · for newcomers

**One line:** A move is not a copy and not a free — it is a transfer of *responsibility*. The bytes stay where they are; what changes is who will free them, and therefore when.

This is the idea the rest of Rust is built on. Get it and the borrow checker stops feeling arbitrary; miss it and every error message reads like an obstacle rather than an answer.

---

## The three rules

1. Every value has exactly one **owner**.
2. There is only ever one owner at a time.
3. When the owner goes out of scope, the value is **dropped** — its memory released, its `Drop` code run.

Rule 3 is the one to hold onto, because it is what the other two exist to protect. Freeing memory is safe only if it happens *exactly once*. Rules 1 and 2 are how Rust makes "exactly once" true by construction rather than by discipline.

The example on this page uses a type that announces its own destruction, so this stops being a claim:

```rust
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("· drop({})", self.0);
    }
}
```

Watch where each `drop(…)` lands in the output below. That is ownership, made visible.

## What a move actually is

```rust
let first = Noisy("the-value");
{
    let second = first;    // ← the move
}                          // ← freed HERE
println!("done");          // ← not here
```

Nothing was copied and nothing was freed at the moment of the move. The value's bytes never travelled. What changed is that `second` is now the one who owes a free — so the value dies at the end of the *inner* block, not at the end of the function.

And `first` is not merely stale or empty. It is **unusable by name**; the compiler removes it from the set of things you are allowed to mention:

```text
error[E0382]: borrow of moved value: `s`
2 |     let s = String::from("hi");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     let t = s;
  |             - value moved here
4 |     println!("{s} {t}");
  |                ^ value borrowed here after move
help: consider cloning the value if the performance cost is acceptable
```

Read that error as three facts, because it states all three: *why* it moved (the type is not `Copy`), *where* it moved, and *where* you tried to use it afterwards. Almost every ownership error you will meet has that shape, and the fix is usually to borrow instead — or, when you genuinely need two, to take the `clone` the compiler is offering.

## Why integers feel different

```rust
let a = 5;
let b = a;
println!("{a} {b}");        // fine — both usable
```

`i32` is `Copy`: duplicating it means copying its bytes and nothing else. Integers, `bool`, `char`, `f64`, shared references `&T`, and tuples/arrays of `Copy` things all behave this way.

A `String` cannot. It owns a heap buffer, so a bytewise duplicate would leave **two owners of one allocation** — and rule 3 would then run two frees on it. That is the bug (a *double free*) that moves exist to make unrepresentable. So the rule is not "big things move, small things copy"; it is **"if duplicating the bytes would duplicate an obligation, it moves."**

## Where moves happen

| Moves | Does not move |
|---|---|
| `let b = a;` | `let b = &a;` |
| passing by value: `f(a)` | passing a reference: `f(&a)` |
| returning a value | a method taking `&self` |
| `v.push(a)` | anything `Copy` |
| `for x in v` (`into_iter`) | `for x in &v` |
| a `move` closure capturing `a` | |

Passing to a function is the case that surprises people, and it is worth watching in the output: `consume(a)` frees `a` *inside* `consume`, before the next line of the caller runs. The function did not borrow it — it took it.

## Ownership is tracked per field, not per variable

```rust
let p = Person { name: String::from("Ada"), age: 36 };
let name = p.name;          // moves ONE field out
println!("{}", p.age);      // still fine
// f(p);                    // error[E0382]: use of partially moved value
```

The struct is now half-empty: `p.age` is readable, `p.name` is gone, and `p` as a whole can no longer be passed anywhere. This is a **partial move**, and it is the compiler being more precise than "the variable is used up" — precision that occasionally reads as a puzzle when you have forgotten which field you took.

## Getting a second one, or getting one out

`clone()` is the explicit "I want a real second copy, and I accept the cost". It is not a defeat — it is the honest option when you genuinely need two — but reach for a borrow first, because a `clone` sprinkled to silence an error usually means the design wanted a `&`.

Collections refuse to let you move an element out, for the same reason as everything else on this page: the `Vec` still owes one free per element, so it will not let you walk off with one unless something takes its place.

```rust
let first = v[0];                    // error[E0507]: cannot move out of index
let first = &v[0];                   // borrow it
let first = v[0].clone();            // duplicate it
let first = v.remove(0);             // take it, and shift the rest down
let first = std::mem::take(&mut s);  // swap in Default::default() and walk away
```

That last one is the same manoeuvre as [`Option::take`](../../17_Option_and_Result/option_as_collection/README.md) — leave something valid behind, and the obligation stays balanced.

## If you are coming from another language

- **Python.** `b = a` gives you two names for one object, and CPython keeps a reference count so it knows when to free. Rust gives you **one name at a time** instead, and needs no count: the single owner is the answer to "when is this freed?". The everyday consequence is the surprise you no longer get — mutating through `b` cannot change what `a` sees, because after a move there is no `a`.
- **ABAP.** `DATA(lt_copy) = lt_source` deep-copies the internal table: safe, and you pay for the copy every time. Rust's default is neither that deep copy nor a shared alias — it is a *transfer*, which costs nothing and leaves no second name. When you do want ABAP's behaviour you ask for it out loud, with `.clone()`, and the cost is visible at the call site rather than hidden in the assignment.

- **C++.** The defaults are inverted. `auto b = a;` **copies** — the copy constructor runs, silently, however expensive it is — and you opt *out* with `std::move`. Rust moves by default and you opt *in* to the copy, out loud, with `.clone()`. The deeper difference is what is left behind: a C++ moved-from object still exists in a "valid but unspecified" state, still usable and still destroyed at the end of its scope, so reading it is legal and meaningless. Rust's moved-from binding is *gone* — touching it is `E0382` before the program runs, and no destructor fires for it. That is why Rust needs no "valid but unspecified" concept: there is nothing left to specify. (Watch for `std::move` on a type with no move constructor, too — it quietly falls back to the copy you were trying to avoid.)

All three bridges land on the same shift: the thing you used to reason about at runtime — refcounts, defensive copies, valid-but-unspecified states — has become a fact the compiler checks before the program runs.

---

## Practice

**Follow the responsibility.** Give a type a `Drop` that prints, then pass it to a function that takes it by value and one that borrows it. Read the output to see where the free actually happened.

Use the moved value on the next line and read `E0382`. Then move one field out of a tuple and check whether the other is still usable — ownership is tracked per field, which is the detail that makes half of the borrow-checker's messages make sense.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:ownership_and_moves_kata -->
*[`ownership_and_moves_kata.rs`](examples/ownership_and_moves_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: follow the responsibility, not the bytes.
//!
//!   rustc --edition 2024 ownership_and_moves_kata.rs -o /tmp/oamk && /tmp/oamk

/// Announces its own free, so you can see exactly when — and where — it happens.
#[derive(Debug)]
struct BallotBox {
    precinct: &'static str,
}

impl Drop for BallotBox {
    fn drop(&mut self) {
        println!("      [dropped: {} — freed here]", self.precinct);
    }
}

/// Takes ownership. The box is freed when this function ends, not the caller's.
fn seal(b: BallotBox) {
    println!("  sealing {}", b.precinct);
}

/// Borrows. Nothing is freed here; the caller still owns it.
fn inspect(b: &BallotBox) {
    println!("  inspecting {}", b.precinct);
}

/// Takes it and gives it back — the shape you write before you know borrowing.
fn stamp(mut b: BallotBox) -> BallotBox {
    b.precinct = "P7 (stamped)";
    b
}

fn main() {
    println!("Borrowing: responsibility never moves.");
    let kept = BallotBox { precinct: "P12" };
    inspect(&kept);
    println!("  still usable afterwards -> {kept:?}");

    println!("\nMoving into a function: the free happens THERE.");
    let handed_over = BallotBox { precinct: "P3" };
    seal(handed_over);
    println!("  (the drop line above printed before this one — inside `seal`)");
    println!("  `handed_over` is now unusable: E0382, borrow of moved value.");

    println!("\nMove out and back again:");
    let b = BallotBox { precinct: "P7" };
    let b = stamp(b);
    println!("  returned -> {b:?}");

    println!("\nMoves are tracked per field, not per variable:");
    let pair = (BallotBox { precinct: "P1" }, String::from("chain of custody"));
    let note = pair.1; // only the String moves
    println!("  moved out the note -> {note:?}");
    println!("  pair.0 is still owned here -> {:?}", pair.0);

    println!("\nAnd integers only *feel* different because they are Copy:");
    let count = 461;
    let also = count; // a copy, not a move
    println!("  count {count}, also {also} — both usable, nothing was transferred");

    println!("\nEnd of main — everything still owned here is freed now, in reverse:");
}
```
<!-- /source -->

<!-- output:ownership_and_moves_kata -->
*Verified output of [`ownership_and_moves_kata.rs`](examples/ownership_and_moves_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Borrowing: responsibility never moves.
  inspecting P12
  still usable afterwards -> BallotBox { precinct: "P12" }

Moving into a function: the free happens THERE.
  sealing P3
      [dropped: P3 — freed here]
  (the drop line above printed before this one — inside `seal`)
  `handed_over` is now unusable: E0382, borrow of moved value.

Move out and back again:
  returned -> BallotBox { precinct: "P7 (stamped)" }

Moves are tracked per field, not per variable:
  moved out the note -> "chain of custody"
  pair.0 is still owned here -> BallotBox { precinct: "P1" }

And integers only *feel* different because they are Copy:
  count 461, also 461 — both usable, nothing was transferred

End of main — everything still owned here is freed now, in reverse:
      [dropped: P1 — freed here]
      [dropped: P7 (stamped) — freed here]
      [dropped: P12 — freed here]
```
<!-- /output -->

</details>

---

## The verified output

Watch the `· drop(…)` lines: each marks the exact moment a value's owner went out of scope.

<!-- output:ownership_and_moves -->
*Verified output of [`ownership_and_moves.rs`](examples/ownership_and_moves.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Rule 3, made visible: the owner's scope end frees the value
  entering an inner block
  inside — inner is alive
      · drop(inner)
  the block ended, and inner was freed just above
  outer lives until step1 returns
      · drop(outer)

──── Step 2: A move transfers RESPONSIBILITY, not bytes
  `first` owns it
  moved to `second`, inside the block
      · drop(the-value)
  the block ended — and it was freed THERE, not here
      The bytes never travelled. What changed is who frees it, and so
      when. And `first` is not merely stale: it is unusable, by name.

──── Step 3: Copy types are duplicated instead of moved
  i32     a = 5, b = 5        both still usable
  bool    flag = true, same = true
  String  t = "hi"              `s` is gone: reading it is error[E0382]
      A type is Copy when duplicating it means copying its bytes and
      nothing else. A String owns a heap buffer, so a byte copy would
      leave TWO owners of one allocation — and two frees. Moves exist
      to make that unrepresentable, not merely discouraged.

──── Step 4: Passing by value moves; borrowing does not
  inspect() only borrows a
  inspect() only borrows a
  a survived two borrows
  consume() owns a now, and will free it on return
      · drop(a)
  consume() has returned, and a is already gone
  hand_back() received b and returns it
  b was moved out and moved back: still owned here as b
      · drop(b)

──── Step 5: Partial moves: ownership is tracked per FIELD
  moved out   name = "Ada"
  still fine  p.age = 36
      But `p` as a whole is gone — passing it anywhere is
      error[E0382] 'use of partially moved value'. The compiler
      is tracking each field separately, not the variable.

──── Step 6: Asking for a second one, and getting one out of a collection
  clone     original = "ballot", copy = "ballot"   (two allocations)
  &v[0]     "x"   — indexing yields a place, not a value:
            `let first = v[0];` is error[E0507], cannot move out of index
  remove    owned = "x", v is now ["y"]
  take      taken = "here", slot left at Default = ""
      Each of these is the same question answered differently: the
      collection still owes one free per element, so it will not let
      you walk off with an element unless something replaces it.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/ownership_and_moves/examples/ownership_and_moves.rs -o /tmp/oam && /tmp/oam
```

## See also

- [What an address shows](../what_an_address_shows/README.md) — the experiment everyone runs to *see* a move: printing `&x` before and after, and why the number that changes is the header rather than the text
- [Borrowing](../borrowing/README.md) — the half this page hands off: using a value without taking responsibility for it
- [`Option` is a one-item collection](../../17_Option_and_Result/option_as_collection/README.md) — `take()`, the standard way to move out of something you only borrow
- [Shadowing and `unwrap`](../../17_Option_and_Result/shadowing_and_unwrap/README.md) — why a `Copy` type survives what looks like a move
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — the same observable `Drop`, seen from the failure side: destructors still run, in reverse order, while a panic unwinds. Rule 3 holds even when the function does not finish, which is why a lock is released and a file closed — and why the *work* is still half-done
- [The `move` keyword](../../23_Closures/the_move_keyword/README.md) — the same move, performed by a closure capturing the value, and the two errors that demand it
- [The Rust Book, ch. 4 — Understanding Ownership ↗](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
