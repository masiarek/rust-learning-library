# Indices instead of references

**Level:** 301 · deep dive

**One line:** When a graph, a tree with parent links or a game world will not fit the borrow checker, store the nodes in a `Vec` and refer to them by index — the aliasing moves from references the compiler checks to numbers it does not, which is both the escape and the price.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The shape that fails: a node holding `&Node` to its neighbours, and the lifetime errors that follow
- The same graph as `Vec<Node>` plus `usize` ids — which borrows now compile, and why
- What the compiler no longer checks: a stale index after a removal points at the wrong node, silently — the use-after-free bug, returned as a logic error
- Generational indices (`slotmap`) as the fix for stale ids, and typed ids (`struct NodeId(u32)`) so a node id cannot index the edge list
- The other two escapes and their costs: `Rc<RefCell<Node>>` with `Weak` parents, and an arena allocator
- Entity–component systems as this pattern at scale — the games section's [entities and components](../../43_Games/entities_and_components/README.md)

## The trap it exists for

Deleting a node with `Vec::remove` or `swap_remove`. Every index after it — or the one moved into its slot — now names a different node, and nothing panics. That is why a real design keeps tombstones or uses generational ids.

## Where this sits

[`Rc`](../reference_counting/README.md) and [interior mutability](../../09_Advanced/interior_mutability/README.md) are the other two escapes. [A generic recursive type](../../22_Generics/a_generic_recursive_type/README.md) is the case a `Box` still handles, a tree with no back edges.

## See also

- [`Rc`: the clone that copies a pointer](../../18_Ownership/reference_counting/README.md) — shared ownership instead of indices
- [Interior mutability](../../09_Advanced/interior_mutability/README.md) — `RefCell` for the parts that change
- [A generic recursive type](../../22_Generics/a_generic_recursive_type/README.md) — the tree that needs no escape
- [Use-after-free](../../31_C_and_Cpp/use_after_free/README.md) — the bug a stale index reintroduces as a logic error
- [Borrowing](../../18_Ownership/borrowing/README.md) — the rule the index design steps around
- [Entities and components](../../43_Games/entities_and_components/README.md) — the same design as a game engine

## If you are coming from another language

- **C.** Handles into a pool are the same pattern, and a C programmer reaches for it to avoid dangling pointers; the difference is that in Rust the references were never allowed to dangle to begin with.
- **Python.** References are free in Python, so graphs use objects directly; an index design shows up only for performance (NumPy arrays of ids), which is also a reason to use it in Rust.
- **Java.** The garbage collector makes object graphs cheap, so the pattern appears mostly in data-oriented code — entity systems, column stores.

## Po polsku

Gdy graf, drzewo z odnośnikami do rodzica albo świat gry nie mieszczą się w regułach pożyczania, węzły trzyma się w `Vec`, a odwołuje się do nich przez indeksy. Kompilator przestaje wtedy pilnować aliasów: nieaktualny indeks po usunięciu węzła wskazuje po cichu inny węzeł — to ten sam błąd co użycie po zwolnieniu, tylko jako błąd logiki.

**Szukaj po polsku:** indeksy zamiast referencji · graf w Ruście · `rust graph vector indices` · `rust slotmap generational index`
