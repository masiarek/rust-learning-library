# `Rc`: the clone that copies a pointer

**Level:** 201 · working knowledge

**One line:** `Rc<T>` gives one value several owners by counting them, so `Rc::clone` duplicates a pointer and a number and never touches the data — which makes it the cheapest `.clone()` in Rust and the most commonly misread one.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What the count *is* — a strong count stored beside the value, incremented on clone and decremented on drop, with the value freed when it reaches zero; `Rc::strong_count` is how you watch it happen
- Why `Rc::clone(&item)` is the house spelling rather than `item.clone()`: both compile to the same call, but the inner type may have a `clone` of its own, and only one of the two spellings says which one ran
- What it costs — a pointer copy and an increment, against a `String`'s allocate-and-copy; and what that buys nothing for, since an `Rc` is immutable without `RefCell`
- `Rc<RefCell<T>>` as the standard pairing, and why the moment you want to *write* through a shared owner you have left `Rc` alone behind
- `Weak` and the cycle `Rc` cannot free on its own — the one leak safe Rust still permits
- Where it does not go: `Rc` is not thread-safe by construction, which is a separate page

## The trap it exists for

`.clone()` is spelled the same whichever it does, so a reader meeting `self.head.clone()` in a list built from `Rc<RefCell<Node>>` has nothing on the line to tell them whether the list was duplicated or a counter went up. It is the counter, every time — but the code does not say so, and the fix is a spelling, not a type.

## See also

- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — where the trait's promise is settled: an owned value, at a depth the *type* chooses
- [Ownership and moves](../ownership_and_moves/README.md) — the one-owner rule this type is the sanctioned exception to
- [`Cow`: borrow until somebody writes](../clone_on_write/README.md) — the other way to avoid a copy, decided by the data rather than by the owner count
- [Sharing across threads](../sharing_across_threads/README.md) — `Arc`, and the compile error you get for sending an `Rc` instead
- [`ToOwned`](../../12_Traits/to_owned/README.md) — where the same trap bites hardest
