# When `String` is too slow

**Level:** 301 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Almost never — and this page is mostly about how to find out, because the three real fixes (`with_capacity`, not allocating at all, and a different type) are only worth applying once a measurement says which one.

---

## What this page has to answer

- Measure first: what to actually count — allocations, not lines — and why a `cargo bench` at `-O0` has told you nothing (a mistake the [`ToOwned`](../../12_Traits/to_owned/README.md) page documents in the wild). [The global allocator](../../09_Advanced/the_global_allocator/README.md) is the tool: it already counts, so this page's job is deciding what to point it at.
- `with_capacity` in anger: the reallocation ladder from [the anatomy page](../anatomy_of_a_string/README.md), and how to know the size up front.
- Not allocating at all — `write!` into an existing buffer, returning `&str` from an owned input, and `Cow` for the branch that does not change anything.
- `format!("{x}")` where a `.to_owned()` would do, and clippy's `useless_format`; the cost is real but small, and the readability cost is the bigger one.
- The crates, and when they are justified: `smallstr` / `smartstring` for short strings inline on the stack, `compact_str`, and the honest note that most programs never need them.

## See also

- [The anatomy of a `String`](../anatomy_of_a_string/README.md)
- [Building a `String`](../building_a_string/README.md)
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md)
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the counter that makes every claim on this page checkable
- [The third owned form](../boxed_str/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime
