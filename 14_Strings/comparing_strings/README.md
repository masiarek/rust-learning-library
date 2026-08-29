# Comparing and sorting text

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `Ord` on a string is byte order, and byte order is not human order — `"Zebra" < "apple"` is true, and no amount of `to_lowercase()` makes sorting names correct in the general case.

---

## What this page has to answer

- What `==` and `<` actually do: byte-wise comparison of the UTF-8, which is why uppercase sorts before lowercase and why it is fast.
- `eq_ignore_ascii_case` versus `to_lowercase()` — the first is ASCII-only and allocation-free, the second is Unicode-aware and allocates, and picking the wrong one is a bug in exactly one direction.
- Case folding is not lowercasing: `'ß'`, `'İ'`, and the Turkish dotless i, which is where a `to_lowercase()` comparison quietly gives the wrong answer.
- Normalization — `"é"` written two ways is two different strings to `==`, the same problem [Meet the `char`](../meet_the_char/README.md) demonstrates, and the crate that fixes it.
- Sorting for humans: why collation is locale-dependent, why it is not in `std`, and what to use instead of pretending it is.

## See also

- [Meet the `char`](../meet_the_char/README.md)
- [Walking a `String`](../walking_a_string/README.md)
- [Six kinds of string](../six_kinds_of_string/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime
