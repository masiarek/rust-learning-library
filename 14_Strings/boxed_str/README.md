# The third owned form: `Box<str>`, `Rc<str>`, `Arc<str>`

**Level:** 201 → 301 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** There is an owned string that is not a `String`: drop the capacity word and you get text you cannot grow, 8 bytes smaller, and — in the `Rc`/`Arc` case — shareable without copying.

---

## What this page has to answer

- What `Box<str>` is: the same bytes as a `String` with the capacity field gone, because a value that can never grow has no use for spare room.
- The conversions, both of which are cheap: `String::into_boxed_str` and `Box<str>::into_string`, and what `shrink_to_fit` has to do first.
- When the 8 bytes matter and when caring about them is premature — the honest answer involves counting the values, not the bytes.
- `Rc<str>` and `Arc<str>` as the interned-string move: many owners, one buffer, no `String` per owner. And why `Rc<String>` is the version that does *not* do that.
- The trap [`ToOwned`](../../12_Traits/to_owned/README.md) already names: `.to_owned()` on an `Rc` clones the pointer, not the text.

## See also

- [The anatomy of a `String`](../anatomy_of_a_string/README.md)
- [`ToOwned`](../../12_Traits/to_owned/README.md)
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime
