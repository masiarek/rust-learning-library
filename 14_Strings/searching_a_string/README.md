# Searching without splitting

**Level:** 101 → 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `contains`, `find`, `starts_with` and the split family are one API wearing four names: they all take a `Pattern`, which is why a `char`, a `&str` and a closure are interchangeable arguments.

---

## What this page has to answer

- The `Pattern` trait as the thing that unifies them — `s.find('x')`, `s.find("xy")` and `s.find(char::is_numeric)` are the same call three ways.
- `find` returns a **byte** offset, not a character index — so its output is exactly what `&s[..i]` wants and exactly not what a Python habit expects.
- `find` vs `rfind` vs `match_indices`, and the case where you wanted `matches().count()` all along.
- `starts_with` / `strip_prefix`: why the second is almost always the one you meant, because it returns the remainder instead of making you slice it off.
- What is *not* here: regex is a crate, not `std`, and the honest guidance on when to reach for it rather than a chain of `find`.

## See also

- [String slices](../string_slices/README.md)
- [Walking a `String`](../walking_a_string/README.md)
- [Meet the `char`](../meet_the_char/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime
