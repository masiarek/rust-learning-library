# String parameters worth copying

**Level:** 201 → 301 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** The signature you write decides what your callers have to allocate — and the four good answers (`&str`, `impl AsRef<str>`, `impl Into<String>`, `Cow<str>`) each buy something different, which is why there is no single right one.

---

## What this page has to answer

- The default and why it is the default: take `&str`, because deref coercion means a `&String` caller pays nothing.
- `impl AsRef<str>` — when you want `&str`, `String`, `&String` and `PathBuf` all to work, and the monomorphization cost of getting it.
- `impl Into<String>` — when the function is going to own the text anyway, so the caller's existing `String` can be moved in rather than copied.
- `Cow<'_, str>` in a **return** position, which is the one place it earns its keep: borrow when nothing changed, allocate only on the write.
- The anti-pattern: taking `String` by value "to keep it simple", and what it costs every caller that has a `&str`.
- Returning text: `String` versus `&str` versus writing into a `&mut String`, and the `E0515` that ends the second option.

## See also

- [`String` vs `&str`](../string_vs_str/README.md)
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md)
- [`ToOwned`](../../12_Traits/to_owned/README.md)
- [`&'static str`](../static_str/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime
