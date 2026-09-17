# Ranges

**Level:** 201 · working knowledge

**One line:** `0..5` is not syntax for a loop — it is a value of type `Range<i32>`, a struct with `start` and `end` that is also an iterator — which is why it can be stored, passed, sliced with, matched against, and why iterating it consumes it.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The six range types behind `a..b`, `a..=b`, `a..`, `..b`, `..=b` and `..`, and which of them can be iterated (a range with no start cannot)
- Half-open by default: `0..len` never needs a `- 1`, and `..=` for the case that includes its end
- Ranges beyond `for`: slicing (`&v[1..3]`), `contains`, `rev`, `step_by`, `len` on an exact-size range, `(1..=n).sum()`
- Ranges in patterns: `0..=9 =>` in a `match` (see [one arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md))
- `Range` is an `Iterator` and therefore not `Copy`: storing one in a struct and iterating it twice — and the E0382 "use of moved value" that follows
- An empty or reversed range (`5..0`) iterates nothing and does not panic; slicing with one does panic
- RFC 3550's new range types: which parts, if any, are stable on 1.98.0 — check before writing a word about it

## The trap it exists for

Writing `for i in 0..=v.len()` to "include the last element". The last valid index is `len - 1`, so the inclusive range reads one past the end and panics. A plain `0..v.len()` — or iterating `&v` directly — was right.

## Where this sits

[`for` loops](../../25_Control_Flow/for_loops/README.md) uses ranges as the thing a loop walks. This page is the type itself. [String slices](../../14_Strings/string_slices/README.md) and [arrays and slices](../../26_Collections/arrays_and_slices/README.md) use ranges as indices.

## See also

- [`for` loops](../../25_Control_Flow/for_loops/README.md) — the loop that walks a range
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — `0..=7` as a pattern
- [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) — `&five[1..3]`, half-open
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — a range adapter computes nothing until consumed
- [`DoubleEndedIterator` and `ExactSizeIterator`](../../24_Iterators/double_ended_and_exact_size/README.md) — why a range has `rev` and `len`
- [`zip` and `enumerate`](../../24_Iterators/zip_and_enumerate/README.md) — the usual replacement for `0..len` indexing

## If you are coming from another language

- **Python.** `range(0, 5)` is the same half-open sequence and is also a lazy object; `range(0, 6)` is how Python spells `0..=5`. Python's range is reusable, while Rust's is consumed by iteration.
- **Go.** `for i := range 5` (Go 1.22) and `for i := 0; i < n; i++` cover the loop use; Go has no range value you can store or pass.
- **Java.** `IntStream.range(0, 5)` and `rangeClosed` map directly, including being single-use like Rust's.
- **C.** A `for` loop's three clauses; there is no range value, so slicing needs a pointer and a length instead of `&v[a..b]`.

## Po polsku

Zakres `0..5` to nie składnia pętli, tylko wartość typu `Range<i32>` — struktura z polami `start` i `end`, która jest też iteratorem. Dlatego zakres można przechować, przekazać, użyć do wycinania i w dopasowaniu wzorca; i dlatego iteracja go zużywa — `Range` nie jest `Copy`.

**Szukaj po polsku:** zakres w Ruście · przedział półotwarty · `rust range inclusive` · `rust range not copy`
