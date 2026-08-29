# `str` methods

**Level:** reference · for working programmers

**One line:** One page per method on the `str` primitive — all 83 that stable Rust 1.98 exposes — each with the signature, what it actually does, the trap it is usually involved in, and a program whose printed output is checked by CI.

These pages are a **reference**, not a course. If you have not met `String` and `&str` yet, read [the strings arc](../README.md) first — it explains the owner-and-view pattern that makes half of this list make sense. Come back here when you know which method you want and need to know how it behaves at the edges.

Every page has the same shape: a one-line summary, the signature, the stability line, the prose, then a **complete runnable program and its verified output**. Nothing on any of these pages is hand-typed output — [`tools/run_examples.py`](../../CONTRIBUTING.md) compiles each example, runs it, and fails the build if what the page shows is not what the program printed.

The signature block is a `text` fence rather than a `rust` one on purpose: a bare `pub fn …` is not something you can paste into a file, and the [house rule](../../CONTRIBUTING.md) is that the first `rust` block on a page must compile. The first `rust` block here is always the real example.

## The pattern argument

Twenty-odd methods below take a `P: Pattern`, and that one trait is why searching, splitting, trimming and replacing all accept the same four shapes:

| shape | example | matches |
|---|---|---|
| `char` | `s.split(',')` | that one character |
| `&str` | `s.split(", ")` | that exact substring |
| `&[char]` | `s.split(&['-', '_'][..])` | **any one** of those characters |
| `FnMut(char) -> bool` | `s.split(char::is_numeric)` | any character the closure approves |

A `&str` pattern is never accepted where the method has to search **backwards from both ends at once** — [`trim_matches`](str_trim_matches/README.md) is the one that refuses it. Chain the two one-sided methods there instead.

## The two panics

Almost every panic in this list is one of two, and both are about byte offsets:

- **out of range** — the offset is past `len()`
- **inside a character** — the offset is not a character boundary, because `len()` counts bytes and a `char` can occupy up to four of them

[`is_char_boundary`](str_is_char_boundary/README.md) is the test, [`get`](str_get/README.md) and the `_checked` methods return `Option` instead of panicking, and [`floor_char_boundary`](str_floor_char_boundary/README.md) repairs an offset rather than refusing it.

---


## How long is it, and what is it made of

| method | what it does |
|---|---|
| [`len`](str_len/README.md) | The **byte** count — not the character count |
| [`is_empty`](str_is_empty/README.md) | `len() == 0`, spelled so it reads |
| [`as_bytes`](str_as_bytes/README.md) | The UTF-8 bytes as `&[u8]`, free |
| [`bytes`](str_bytes/README.md) | The same bytes as an iterator of `u8` |
| [`chars`](str_chars/README.md) | An iterator of `char` — Unicode scalars, not graphemes |
| [`char_indices`](str_char_indices/README.md) | Characters with the **byte offsets** you can slice at |
| [`lines`](str_lines/README.md) | Split on `\n`, dropping a trailing `\r` and the final empty line |
| [`is_ascii`](str_is_ascii/README.md) | Whether every byte is under 128 — the `ascii` family's precondition |


## Is it there, and where

Every method here takes a [pattern](#the-pattern-argument).

| method | what it does |
|---|---|
| [`contains`](str_contains/README.md) | Whether the pattern occurs anywhere |
| [`starts_with`](str_starts_with/README.md) | Whether it occurs at the front |
| [`ends_with`](str_ends_with/README.md) | Whether it occurs at the back |
| [`find`](str_find/README.md) | The byte offset of the first match, as `Option<usize>` |
| [`rfind`](str_rfind/README.md) | The byte offset of the last match |
| [`matches`](str_matches/README.md) | The matched substrings themselves |
| [`rmatches`](str_rmatches/README.md) | The same, yielded back to front |
| [`match_indices`](str_match_indices/README.md) | Each match with its byte offset |
| [`rmatch_indices`](str_rmatch_indices/README.md) | The same, back to front — the order in-place edits need |
| [`substr_range`](str_substr_range/README.md) | Where a sub-slice sits **in this string** — identity, not search |


## Cutting it up

Splits report the *gaps between* matches, so *n* matches give *n+1* pieces.

| method | what it does |
|---|---|
| [`split`](str_split/README.md) | The pieces between the matches — empties included |
| [`rsplit`](str_rsplit/README.md) | The same pieces, last to first |
| [`splitn`](str_splitn/README.md) | At most `n` pieces; the remainder stays whole |
| [`rsplitn`](str_rsplitn/README.md) | The same limit, applied from the right |
| [`split_once`](str_split_once/README.md) | Exactly two pieces at the first match, or `None` |
| [`rsplit_once`](str_rsplit_once/README.md) | Exactly two pieces at the last match, or `None` |
| [`split_terminator`](str_split_terminator/README.md) | Drops a trailing empty piece |
| [`rsplit_terminator`](str_rsplit_terminator/README.md) | The same rule, iterated from the right |
| [`split_inclusive`](str_split_inclusive/README.md) | Keeps the separator on the piece it ended |
| [`split_whitespace`](str_split_whitespace/README.md) | Collapses runs of whitespace — the editorial split, for prose |
| [`split_ascii_whitespace`](str_split_ascii_whitespace/README.md) | The same, ASCII-only and faster |
| [`split_at`](str_split_at/README.md) | Two halves at a byte offset — **panics** off a boundary |
| [`split_at_checked`](str_split_at_checked/README.md) | The same cut, returning `Option` |
| [`split_at_mut`](str_split_at_mut/README.md) | Two **mutable** halves, which plain borrowing cannot give |
| [`split_at_mut_checked`](str_split_at_mut_checked/README.md) | The mutable cut, returning `Option` |


## Taking things off the ends

| method | what it does |
|---|---|
| [`trim`](str_trim/README.md) | Unicode whitespace off both ends, borrowed |
| [`trim_start`](str_trim_start/README.md) | The front only |
| [`trim_end`](str_trim_end/README.md) | The back only — what to call on a line you just read |
| [`trim_matches`](str_trim_matches/README.md) | A pattern off both ends, **repeatedly** |
| [`trim_start_matches`](str_trim_start_matches/README.md) | Repeatedly, at the front — accepts a `&str` |
| [`trim_end_matches`](str_trim_end_matches/README.md) | Repeatedly, at the back |
| [`trim_ascii`](str_trim_ascii/README.md) | ASCII whitespace only, and `const` |
| [`trim_ascii_start`](str_trim_ascii_start/README.md) | The front only, `const` |
| [`trim_ascii_end`](str_trim_ascii_end/README.md) | The back only, `const` |
| [`strip_prefix`](str_strip_prefix/README.md) | Removes it **once** and tells you whether it was there |
| [`strip_suffix`](str_strip_suffix/README.md) | The same at the back |
| [`strip_circumfix`](str_strip_circumfix/README.md) | Both ends at once — `Some` only if **both** matched |


## Changing the case

The `ascii` half is length-preserving; the Unicode half is not.

| method | what it does |
|---|---|
| [`to_lowercase`](str_to_lowercase/README.md) | Full Unicode — context-sensitive, and can change length |
| [`to_uppercase`](str_to_uppercase/README.md) | Full Unicode — `ß` becomes `SS`, so it does not round-trip |
| [`to_ascii_lowercase`](str_to_ascii_lowercase/README.md) | `A`–`Z` only, everything else untouched |
| [`to_ascii_uppercase`](str_to_ascii_uppercase/README.md) | `a`–`z` only — no `ß` expansion |
| [`make_ascii_lowercase`](str_make_ascii_lowercase/README.md) | The same, **in place**, no allocation |
| [`make_ascii_uppercase`](str_make_ascii_uppercase/README.md) | The same, in place |
| [`eq_ignore_ascii_case`](str_eq_ignore_ascii_case/README.md) | Case-insensitive comparison with no allocation at all |


## Making a new string out of this one

Everything here allocates — a `str` cannot change its own length.

| method | what it does |
|---|---|
| [`replace`](str_replace/README.md) | Every occurrence of a pattern |
| [`replacen`](str_replacen/README.md) | The first `n` occurrences |
| [`repeat`](str_repeat/README.md) | `n` copies, allocated once |
| [`parse`](str_parse/README.md) | Into any `FromStr` type — and it does **not** trim first |
| [`into_string`](str_into_string/README.md) | `Box<str>` → `String`, without copying |
| [`into_boxed_bytes`](str_into_boxed_bytes/README.md) | `Box<str>` → `Box<[u8]>`, without copying |


## Byte offsets, and the panics they cause

The two failure modes are always the same: out of range, or **inside a character**.

| method | what it does |
|---|---|
| [`get`](str_get/README.md) | Slicing that returns `Option` instead of panicking |
| [`get_mut`](str_get_mut/README.md) | The same, mutable |
| [`is_char_boundary`](str_is_char_boundary/README.md) | Whether an offset is a legal slice endpoint |
| [`floor_char_boundary`](str_floor_char_boundary/README.md) | Nudge a bad offset **down** — the safe byte-budget truncation |
| [`ceil_char_boundary`](str_ceil_char_boundary/README.md) | Nudge it **up** — can exceed the budget, and panics past the end |


## Bytes in, bytes out

| method | what it does |
|---|---|
| [`from_utf8`](str_from_utf8/README.md) | Validate `&[u8]` into `&str`, borrowing |
| [`from_utf8_mut`](str_from_utf8_mut/README.md) | The same, mutable |
| [`encode_utf16`](str_encode_utf16/README.md) | Out to UTF-16 code units — surrogate pairs make a third length |
| [`escape_debug`](str_escape_debug/README.md) | As `{:?}` prints it — printable non-ASCII stays readable |
| [`escape_default`](str_escape_default/README.md) | Escaped to **pure ASCII** |
| [`escape_unicode`](str_escape_unicode/README.md) | Every character as `\u{...}`, no exceptions |
| [`as_ptr`](str_as_ptr/README.md) | The raw address — for FFI, and **not** NUL-terminated |
| [`as_mut_ptr`](str_as_mut_ptr/README.md) | The writable raw address |


## `unsafe` — the checks removed

Each one trades a cheap check for an obligation you now carry.

| method | what it does |
|---|---|
| [`as_bytes_mut`](str_as_bytes_mut/README.md) | Writable bytes — must still be valid UTF-8 when the borrow ends |
| [`get_unchecked`](str_get_unchecked/README.md) | `get` with no bounds or boundary check |
| [`get_unchecked_mut`](str_get_unchecked_mut/README.md) | The mutable version |
| [`from_utf8_unchecked`](str_from_utf8_unchecked/README.md) | `&[u8]` → `&str` with no validation |
| [`from_utf8_unchecked_mut`](str_from_utf8_unchecked_mut/README.md) | The mutable version — unvalidated **and** writable |


## Deprecated, and still compiling

Here because you will meet them in older code.

| method | what it does |
|---|---|
| [`trim_left`](str_trim_left/README.md) | → [`trim_start`](str_trim_start/README.md) |
| [`trim_right`](str_trim_right/README.md) | → [`trim_end`](str_trim_end/README.md) |
| [`trim_left_matches`](str_trim_left_matches/README.md) | → [`trim_start_matches`](str_trim_start_matches/README.md) |
| [`trim_right_matches`](str_trim_right_matches/README.md) | → [`trim_end_matches`](str_trim_end_matches/README.md) |
| [`lines_any`](str_lines_any/README.md) | → [`lines`](str_lines/README.md) |
| [`slice_unchecked`](str_slice_unchecked/README.md) | → [`get_unchecked`](str_get_unchecked/README.md) |
| [`slice_mut_unchecked`](str_slice_mut_unchecked/README.md) | → [`get_unchecked_mut`](str_get_unchecked_mut/README.md) |


---

## Where to go next

- [`String` methods](../string_methods/README.md) — the owning half of the pair, 42 more pages
- [The strings arc](../README.md) — the lessons these pages are a reference for
- [`str` in the standard library ↗](https://doc.rust-lang.org/std/primitive.str.html) — the official documentation these pages explain
