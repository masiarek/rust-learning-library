# A generic recursive type

**Level:** 201 → 301 · working knowledge

**One line:** A type that contains itself has no finite size until a pointer breaks the cycle — `Option<Box<Self>>` is that pointer, and the `Option` half of it is free.

```rust
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }

    fn push_front(self, data: T) -> Self {
        Self { data, next: Some(Box::new(self)) }
    }
}

let ballot = ListNode::new("Cara").push_front("Ben").push_front("Ada");
println!("{}", ballot.data);  // Ada
```

One definition, and `ListNode<&str>` and `ListNode<u8>` are two unrelated linked lists built from it.

## Why the `Box` is not optional

Write the field as `Option<ListNode<T>>` and the type has no size the compiler can compute — a node contains a node contains a node:

```text
error[E0072]: recursive type `ListNode` has infinite size
 --> e0072.rs:1:1
  |
1 | struct ListNode<T> {
  | ^^^^^^^^^^^^^^^^^^
2 |     data: T,
3 |     next: Option<ListNode<T>>,
  |                  ----------- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
3 |     next: Option<Box<ListNode<T>>>,
  |                  ++++           +
```

A `Box<ListNode<T>>` is one pointer, eight bytes, whatever `T` turns out to be — so the size stops depending on itself and the recursion moves to the heap where it belongs. `Rc` and `&` break the cycle the same way; `Box` is the one to reach for when the node owns the rest of the list.

## The `Option` is free

`None` costs nothing here, because a `Box` is never null and the compiler knows it — so `None` is stored *as* the null pointer rather than as a tag beside one:

| Type | `size_of` |
|---|---|
| `Box<ListNode<i32>>` | 8 |
| `Option<Box<ListNode<i32>>>` | 8 |

That is the [null-pointer optimisation](../../17_Option_and_Result/nullable_pointers/README.md), and it is what makes `Option<Box<T>>` the idiomatic "maybe another node" in Rust rather than a wrapper you pay for.

## Rolling your own `Option` gets you an `Option`

The alternative spelling — an enum with an explicit `End` variant — is a reasonable-looking idea:

```rust
enum NextNode<T> {
    Next(Box<OtherNode<T>>),
    End,
}

struct OtherNode<T> {
    data: T,
    next: NextNode<T>,
}
```

It compiles, it is the same eight bytes (the same niche applies), and it walks the same way. What it is, precisely, is `Option<Box<OtherNode<T>>>` with the variants renamed — and renaming them costs the whole `Option` API: no `as_deref`, `map`, `take`, `is_none`, `unwrap_or`, no `while let Some(…)` that every Rust reader already knows on sight. A bespoke two-variant enum earns its place when the *names* carry meaning the code depends on (`Pending` / `Finalised`); "there is no next node" is not that case.

## Do not box the payload

The `data` field needs no `Box`. `T` is not recursive — only the `next` field is — so boxing it buys nothing and costs an allocation and a dereference per node:

| Node type | `size_of` | Heap allocations per node |
|---|---|---|
| `ListNode<[u8; 64]>` — payload inline | 72 | 1 (for the next pointer) |
| `BoxedData<[u8; 64]>` — `data: Box<T>` | 16 | 2 |

The boxed row is not strictly worse: it is a smaller node, which matters if you move nodes around a lot or if `T` is huge and rarely read. But it is a decision to make about a specific `T`, and the default is inline. A `Box<T>` in a generic struct is right when the type must be a fixed size regardless of `T` — not as a habit.

Likewise a `where T: Clone` on the node definition itself — which is how the books usually print this struct — is a bound in the wrong place: it stops you from ever building a list of something unclonable, and the plainest `impl` block below it becomes six errors. [Where the bound goes](../where_the_bound_goes/README.md) has that transcript.

## Walking one

```rust
let mut current = Some(&ballot);
while let Some(node) = current {
    println!("{}", node.data);
    current = node.next.as_deref();
}
```

[`as_deref()` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref) turns `&Option<Box<ListNode<T>>>` into `Option<&ListNode<T>>` — the loop variable borrows and never owns, so the list is intact afterwards. That one method is why the `Option` spelling is worth keeping.

Recursion works too (`fn len(&self) -> usize { 1 + self.next.as_ref().map_or(0, |n| n.len()) }`), and it has a real limit: nothing here is tail-call optimised, so a long enough list overflows the stack. The iterative walk has no such ceiling. The same asymmetry applies to `Drop`, and it is the one bug this data structure is famous for — dropping a very long list recurses once per node.

## When you actually want one

Rarely. A `Vec<T>` beats a linked list on almost every access pattern on real hardware, and std's own [`LinkedList` ↗](https://doc.rust-lang.org/std/collections/struct.LinkedList.html) says so in its documentation. Build this one to learn what `Box`, `Option` and `<T>` do together — that is what it is for here, and it is why the type turns up in every Rust book. If you want the deep version, [Learn Rust With Entirely Too Many Linked Lists ↗](https://rust-unofficial.github.io/too-many-lists/) is six increasingly honest attempts at exactly this struct.

## If you are coming from another language

**Python.** `class Node: def __init__(self, data, next=None)` needs no equivalent of the `Box`, and it is worth knowing why: every Python value is already a reference, so `self.next = other` stores a pointer whether you think about it or not, and a class instance has no fixed inline size to compute. Rust makes the same choice visible — `Box` is where you say *this part lives on the heap* — and hands you the counterpart in exchange: the node owns its successor, so the whole list is freed when the head is dropped, with no reference counting and no collector. `None` maps to `None`, pleasingly exactly.

**ABAP.** The same wall, and the same fix. A structure cannot contain itself by value — there is no size for it — so a linked node is built from a **reference** component: `TYPE REF TO ty_node` for a data reference, or the more common shape, a class whose attribute is `TYPE REF TO lcl_node`. That is `Box` under a different name, minus the ownership: ABAP references are garbage collected, so two nodes may point at the same successor and nobody has to decide who frees it. Rust's `Box` says exactly one owner, which is what lets the memory be released at a known moment; when you genuinely need the ABAP arrangement — several owners, freed when the last one goes — the type is `Rc<T>`.

**C++.** `struct Node { T data; std::unique_ptr<Node> next; };` is the same design with the same reasoning, and `Box` is `unique_ptr`. Rust's version needs no rule of five, no explicit destructor, and cannot be double-freed or read after the move, because the move is the compiler's business rather than yours. The recursive-`Drop` stack overflow, however, is shared exactly — both languages destroy the list one frame per node unless you write the loop by hand.

**Java.** `class Node<T> { T data; Node<T> next; }` compiles as written, since every object reference is already a pointer, and the collector handles the rest. Rust asks for the `Box` because it lays a struct out inline by default, and hands back deterministic freeing for the trouble.

## The verified output

[`examples/a_generic_recursive_type.rs`](examples/a_generic_recursive_type.rs) compiled and run:

<!-- output:a_generic_recursive_type -->
*Verified output of [`a_generic_recursive_type.rs`](examples/a_generic_recursive_type.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
  1. Ada
  2. Ben
  3. Cara
length 3

size_of::<Box<ListNode<i32>>>()          8
size_of::<Option<Box<ListNode<i32>>>>()  8
size_of::<NextNode<i32>>()               8

size_of::<ListNode<i32>>()               16
size_of::<ListNode<[u8; 64]>>()          72
size_of::<BoxedData<[u8; 64]>>()         16

ListNode<&str> of 3, ListNode<u8> of 2
the hand-rolled spelling walks the same way: 1 then 0
BoxedData holds 64 bytes behind one more pointer
```
<!-- /output -->

## Practice

**Walk it without recursion.** Give `ListNode<T>` a method `values(&self) -> Vec<&T>` that returns every payload in order, head first, using a loop rather than recursion — and without moving or cloning anything, so the list is still usable afterwards.

Write the obvious recursive version first if you like; then convert it, and note what the loop needs that the recursion did not. The method that makes the loop possible is on `Option`, not on your type.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_generic_recursive_type_kata -->
*[`a_generic_recursive_type_kata.rs`](examples/a_generic_recursive_type_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Kata solution: walk a recursive generic list without recursion.

struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }

    fn push_front(self, data: T) -> Self {
        Self { data, next: Some(Box::new(self)) }
    }

    // `as_deref()` turns &Option<Box<ListNode<T>>> into Option<&ListNode<T>>,
    // which is the whole trick: the loop variable never owns anything.
    fn values(&self) -> Vec<&T> {
        let mut out = Vec::new();
        let mut current = Some(self);
        while let Some(node) = current {
            out.push(&node.data);
            current = node.next.as_deref();
        }
        out
    }
}

fn main() {
    let ballot = ListNode::new("Cara").push_front("Ben").push_front("Ada");
    println!("names   {:?}", ballot.values());
    println!("length  {}", ballot.values().len());

    // The same walk, over a different T. One definition, two lists.
    let scores = ListNode::new(0u8).push_front(3).push_front(5);
    println!("scores  {:?}", scores.values());

    // The list is still there afterwards: values() borrowed, it did not consume.
    println!("first   {}", ballot.values()[0]);
}
```
<!-- /source -->

<!-- output:a_generic_recursive_type_kata -->
*Verified output of [`a_generic_recursive_type_kata.rs`](examples/a_generic_recursive_type_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
names   ["Ada", "Ben", "Cara"]
length  3
scores  [5, 3, 0]
first   Ada
```
<!-- /output -->

</details>

## See also

- [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) — why `Option<Box<T>>` is free, measured
- [Variants that carry data](../../13_Enums/variants_that_carry_data/README.md) — what a payload costs, and the niche this page is spending
- [Where the bound goes](../where_the_bound_goes/README.md) — why `struct ListNode<T> where T: Clone` is the wrong shape
- [Generic enums](../generic_enums/README.md) — `NextNode<T>` in its own right, and the two-parameter case
- [What a generic is](../what_a_generic_is/README.md) — the `<T>` being made recursive here
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — what `push_front(self, …)` is doing to the old head
