# Nullable pointers

**Level:** 201 · working knowledge

**One line:** Rust has no null, so a pointer that might be absent is a *different type* — `Option<Box<T>>` — which costs the same 8 bytes and cannot be read without saying what happens when it is missing.

Every pointer type in Rust must point at a valid location. `&T`, `&mut T` and `Box<T>` are never null; there is no literal to write one with and no way to produce one in safe code. That sounds like a restriction until you notice what it buys: because the *plain* pointer can never be absent, the compiler is free to use "absent" as the meaning of a bit pattern it could never otherwise hold. Optionality moves out of the value and into the type.

So you write the optional pointer as an optional owned box:

```rust
fn main() {
    let optional = None;
    check_optional(optional);

    let optional = Some(Box::new(9000));
    check_optional(optional);

    fn check_optional(optional: Option<Box<i32>>) {
        match optional {
            Some(p) => println!("has value {p}"),
            None => println!("has no value"),
        }
    }
}
```

`check_optional` cannot get at the `i32` until it has said what happens when there isn't one. **That match is the null check** — not ceremony around it, and not a convention you can forget on the one path nobody tested.

## The same trick, borrowed

`Box` owns its value; when you only want to look at one, the pair is `&T` and `Option<&T>`:

| Type | Means |
|---|---|
| `&i32` | a reference, always valid |
| `Option<&i32>` | a reference that might not be there — the nullable one |
| `Box<i32>` | an owned heap value, always valid |
| `Option<Box<i32>>` | an owned heap value that might not be there |

Because these are genuinely different types, a function's signature states which one it accepts and no caller can supply the other by accident. In a language with null, `describe(p)` takes "a pointer, and good luck" — here it takes one or the other, and the choice is visible at the call site.

## It is free — with one exception

```text
Box<i32>              8 bytes
Option<Box<i32>>      8
&i32                  8
Option<&i32>          8
*const i32            8
Option<*const i32>   16      <- the exception
```

Where the inner type has a bit pattern it can never legally hold — a *niche* — `None` takes that pattern and the tag costs nothing. A `Box` is never null, so null means `None`. That is the [niche optimization](../option_as_collection/README.md), and it is why `Option<Box<T>>` is exactly the machine representation a C programmer would have written by hand.

The exception is worth knowing because it is the one case where the intuition fails: a **raw** pointer (`*const T`, `*mut T`) *is* allowed to be null, so it has no spare pattern left, and `Option<*const T>` grows to 16 bytes to store a real tag. The optimization is not a favour Rust does for `Option`; it is a consequence of the inner type having promised something.

## You rarely write the whole match

The full `match` is the honest first thing to learn, and then almost never what you write:

```rust
if let Some(p) = &boxed { … }                  // one arm, borrow rather than consume
boxed.as_deref().map(|v| v * 2)                // Option<Box<i32>> -> Option<&i32> -> Option<i32>
boxed.as_deref().copied().unwrap_or(-1)        // a default instead of a branch
boxed.as_deref().is_some_and(|v| *v > 100)     // ask a question without unwrapping
let v = boxed?;                                // hand the absence to your caller
```

`as_deref()` is the one to keep: it turns `Option<Box<T>>` into `Option<&T>`, so you can look inside without consuming the box. And note what the binding in the `Some` arm actually is — `p` is a `Box<i32>`, not an `i32`. It prints and dereferences like the number because `Box` derefs to its contents, which is a convenience worth being able to name when the type error arrives.

## Why the type exists at all

The optional box is not really about arguments that might be missing. It is what makes a **recursive** data structure possible:

```rust
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
```

Both halves are load-bearing, and they answer different questions. The `Box` gives `Node` a known size — without it the type contains itself and is infinitely large, which the compiler rejects outright. The `Option` is what lets the chain **stop**. A C linked list writes that ending as a null `next` pointer and trusts every walker to test it; here the end of the list is a value the type system knows about, so `while let Some(n) = cur` cannot walk off the end. It still costs 16 bytes a node, with no separate "is there a next?" flag anywhere in it.

## If you are coming from another language

- **Python** — `None` is a value that any reference may hold, so `p.value` is a legal expression that becomes `AttributeError: 'NoneType' object has no attribute 'value'` at runtime, on whichever input finally reaches that line. `Optional[Node]` says the same thing as `Option<Box<Node>>`, but it is an annotation a type checker may look at, not a fact the language enforces. What changes in Rust is *when* you find out, and that absence is no longer something every type is quietly capable of.
- **ABAP** — this is `IS BOUND` on a data reference, and `IS ASSIGNED` on a field symbol, with the compiler holding you to it. Dereferencing an initial reference raises `CX_SY_REF_IS_INITIAL` at runtime; `Option<Box<T>>` is the same distinction moved to compile time, so the check you were supposed to write in front of every `->*` is the only way to reach the value at all.
- **C, Java, C#** — you already have the representation: `Option<Box<T>>` compiles to the nullable pointer you would have written, one machine word, null meaning absent. The difference is only that the plain `Box<T>` and `&T` also exist, so "cannot be null" is finally something you can *say*. This is the [billion-dollar mistake ↗](https://en.wikipedia.org/wiki/Null_pointer#History) Tony Hoare named after his own 1965 design: not that null exists, but that it was allowed into every type.

---

## Practice

**A list that ends.** Build a small stack of `Node { score, next }` with push, pop, and a walk that collects the scores. The end of the list is a `None`, not a null. Then print `size_of` for `Box<Node>` and `Option<Box<Node>>`.

Declare `next: Node` first and read `E0072` — a type of infinite size is a better introduction to `Box` than any explanation. The two sizes you print at the end are the payoff: the safety is not costing you a word of memory.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:nullable_pointers_kata -->
*[`nullable_pointers_kata.rs`](examples/nullable_pointers_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a list that ends, built out of a pointer that may be absent.
//!
//!   rustc --edition 2024 nullable_pointers_kata.rs -o /tmp/npk && /tmp/npk

use std::mem::size_of;

/// `next: Node` would be a type of infinite size. `Box` gives it a known size
/// (a pointer), and `Option` is how the last node says "the list stops here" —
/// a job other languages give to a null pointer.
#[derive(Debug)]
struct Node {
    score: u8,
    next: Option<Box<Node>>,
}

struct Stack {
    head: Option<Box<Node>>,
}

impl Stack {
    fn new() -> Self {
        Stack { head: None }
    }

    fn push(&mut self, score: u8) {
        // take() leaves None behind, so the old head can be moved into the new
        // node while `self` is only borrowed.
        let old = self.head.take();
        self.head = Some(Box::new(Node { score, next: old }));
    }

    fn pop(&mut self) -> Option<u8> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.score)
    }

    fn scores(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut cursor = &self.head;
        while let Some(node) = cursor {
            out.push(node.score);
            cursor = &node.next;
        }
        out
    }
}

fn main() {
    let mut stack = Stack::new();
    for s in [5, 3, 0] {
        stack.push(s);
    }

    println!("A list whose end is a None rather than a null:");
    println!("  contents -> {:?}", stack.scores());
    println!("  pop      -> {:?}", stack.pop());
    println!("  contents -> {:?}", stack.scores());
    println!("  pop, pop -> {:?}, {:?}", stack.pop(), stack.pop());
    println!("  pop on empty -> {:?}", stack.pop());

    println!("\nAnd the safety costs nothing:");
    println!("  size_of::<Box<Node>>()         = {}", size_of::<Box<Node>>());
    println!("  size_of::<Option<Box<Node>>>() = {}", size_of::<Option<Box<Node>>>());
    println!("      Same width. None is stored as the one bit pattern a Box can");
    println!("      never hold — the null — so the tag is free. That is why a");
    println!("      linked list in Rust is not paying for its safety.");
}
```
<!-- /source -->

<!-- output:nullable_pointers_kata -->
*Verified output of [`nullable_pointers_kata.rs`](examples/nullable_pointers_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
A list whose end is a None rather than a null:
  contents -> [0, 3, 5]
  pop      -> Some(0)
  contents -> [3, 5]
  pop, pop -> Some(3), Some(5)
  pop on empty -> None

And the safety costs nothing:
  size_of::<Box<Node>>()         = 8
  size_of::<Option<Box<Node>>>() = 8
      Same width. None is stored as the one bit pattern a Box can
      never hold — the null — so the tag is free. That is why a
      linked list in Rust is not paying for its safety.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:nullable_pointers -->
*Verified output of [`nullable_pointers.rs`](examples/nullable_pointers.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: An optional owned box
  has no value
  has value 9000
      To use the i32 you must first say what happens when it is not
      there. The match is not ceremony — it IS the null check, and the
      compiler will not let you skip it.

──── Step 2: The same trick for borrowed pointers
  describe(Some(&n)) -> points at 7
  describe(None)     -> points at nothing
      `&i32` is the non-nullable reference; `Option<&i32>` is the
      nullable one. Two different types, so a function's signature
      says which it accepts and no caller can get it wrong.

──── Step 3: The optional pointer is free — with one exception
  Box<i32>                  8 bytes
  Option<Box<i32>>          8
  &i32                      8
  Option<&i32>              8
  *const i32                8
  Option<*const i32>        16   <- the exception
      A Box or a reference can never be null, so the compiler spends that
      impossible bit pattern on None and the tag costs nothing. A RAW
      pointer is allowed to be null, so it has no spare pattern left and
      Option has to store a real tag beside it. Same idea as C's nullable
      pointer, then, but you cannot forget the check.

──── Step 4: In practice you rarely write the whole match
  if let           got 9000
  map              Some(18000)
  map (on None)    None
  unwrap_or        -1
  is_some_and      true
  deref            *p + 1 = 9001
      Note what `p` is in the Some arm: a Box<i32>, not an i32. It prints
      and dereferences like the number because Box derefs to its contents.
      `as_deref()` is the one to remember: Option<Box<T>> -> Option<&T>,
      so you can look inside without consuming the box.

──── Step 5: Why this type exists at all: a structure that ends
  1 -> 2 -> 3 -> end   (sum 6)
  size_of::<Node>()         16 bytes
      `next: Option<Box<Node>>` carries both halves of the answer. The Box
      gives Node a known size, without which the type is infinitely large
      and does not compile; the Option is what lets the chain STOP. A C
      linked list writes that as a null next-pointer and hopes; here the
      end of the list is a value the type system knows about — and it still
      costs 16 bytes a node, with no separate 'is there a next?' flag.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/nullable_pointers/examples/nullable_pointers.rs -o /tmp/np && /tmp/np
```

## See also

- [`Option` is a one-item collection](../option_as_collection/README.md) — the niche optimization in full, and the sizes at which the free lunch runs out
- [`Option` vs `Result`](../option_vs_result/README.md) — when absence should be a failure with a reason instead
- [`Option` fields](../option_fields/README.md) — `Option` in a type definition rather than a return type
- [`std::boxed::Box` ↗](https://doc.rust-lang.org/std/boxed/struct.Box.html) and [`Option::as_deref` ↗](https://doc.rust-lang.org/core/option/enum.Option.html#method.as_deref)
