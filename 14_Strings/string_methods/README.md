# `String` methods

**Level:** reference · for working programmers

**One line:** One page per method on `String` — all 42 that stable Rust 1.98 exposes — each with the signature, the behaviour at the edges, and a runnable program whose output CI checks.

`String` owns its text and can grow it. That is the whole difference from [`str`](../str_methods/README.md), and it is why this list is short: everything about *reading* text is over there, reached automatically because `String` dereferences to `str`. What is here is the part that needs ownership — building, growing, removing, and handing the allocation somewhere else.

So if you are looking for `split`, `trim`, `find`, `replace` or `parse`, they are [on the `str` side](../str_methods/README.md) and they work on a `String` unchanged.

Same page shape as the `str` reference: summary, signature, stability, prose, then a complete program and its verified output.

## The three numbers

Every method below moves one of three numbers, and confusing them is most of the difficulty:

| | what it is | changed by |
|---|---|---|
| `len()` | **bytes** written | `push`, `push_str`, `truncate`, `clear`, `pop`, … |
| `capacity()` | bytes the buffer can hold | `reserve`, `shrink_to_fit`, and growth |
| `chars().count()` | Unicode scalars | nothing directly — it is derived by walking |

`len` is part of the value; `capacity` is not. Two strings with the same text and different capacities are equal, hash the same, and print the same — so never assert on a capacity in a test.

## Byte offsets panic in two ways

`insert`, `remove`, `truncate`, `drain`, `replace_range`, `split_off` and `extend_from_within` all take byte offsets, and all panic on the same two conditions: **out of range**, or **inside a character**. [`is_char_boundary`](../str_methods/str_is_char_boundary/README.md) is the test and [`floor_char_boundary`](../str_methods/str_floor_char_boundary/README.md) is the repair.

---


## Making one

| method | what it does |
|---|---|
| [`new`](string_new/README.md) | Empty, and **allocates nothing** until the first push |
| [`with_capacity`](string_with_capacity/README.md) | Empty, with the buffer bought up front |
| [`from_utf8`](string_from_utf8/README.md) | Validate a `Vec<u8>` — no copy, and the `Vec` comes back on failure |
| [`from_utf8_lossy`](string_from_utf8_lossy/README.md) | Bad bytes become `�`; returns a `Cow`, so clean input is free |
| [`from_utf16`](string_from_utf16/README.md) | Decode UTF-16 code units; fails on an unpaired surrogate |
| [`from_utf16_lossy`](string_from_utf16_lossy/README.md) | The same, replacing instead of failing |
| [`from_utf16le`](string_from_utf16le/README.md) | Decode little-endian UTF-16 **bytes** |
| [`from_utf16le_lossy`](string_from_utf16le_lossy/README.md) | The same, replacing instead of failing |
| [`from_utf16be`](string_from_utf16be/README.md) | Decode big-endian UTF-16 bytes |
| [`from_utf16be_lossy`](string_from_utf16be_lossy/README.md) | The same, replacing instead of failing |


## Growing it

`len` is what you wrote; `capacity` is what you paid for.

| method | what it does |
|---|---|
| [`push`](string_push/README.md) | One `char` — which is **1–4 bytes**, so `len` grows by more than one |
| [`push_str`](string_push_str/README.md) | Text on the end — the workhorse |
| [`insert`](string_insert/README.md) | One `char` at a byte offset, shifting the rest — O(n) |
| [`insert_str`](string_insert_str/README.md) | Text at a byte offset — prepending always costs a copy |
| [`extend_from_within`](string_extend_from_within/README.md) | Append a copy of one of its **own** ranges, no temporary |
| [`capacity`](string_capacity/README.md) | Bytes the buffer holds — **not** part of the value |
| [`reserve`](string_reserve/README.md) | Room for `n` **more** bytes — a headroom, not a total |
| [`reserve_exact`](string_reserve_exact/README.md) | The same without the round-up |
| [`try_reserve`](string_try_reserve/README.md) | `reserve` that returns `Result` instead of aborting |
| [`try_reserve_exact`](string_try_reserve_exact/README.md) | Exact, and reporting |
| [`shrink_to_fit`](string_shrink_to_fit/README.md) | Give the spare capacity back — usually by copying |
| [`shrink_to`](string_shrink_to/README.md) | Shrink toward a floor you name, for a reused buffer |


## Taking things out

| method | what it does |
|---|---|
| [`pop`](string_pop/README.md) | The last `char` — O(1), returns `Option`, cannot panic |
| [`remove`](string_remove/README.md) | The `char` at a byte offset — O(n), and it **panics** |
| [`truncate`](string_truncate/README.md) | Drop everything past an offset — panics off a boundary |
| [`clear`](string_clear/README.md) | Empty it and **keep the buffer** — the reusable-buffer idiom |
| [`drain`](string_drain/README.md) | Remove a range and read what went |
| [`retain`](string_retain/README.md) | Keep what a predicate approves — one linear in-place pass |
| [`replace_range`](string_replace_range/README.md) | Swap a range for different text, in place |
| [`split_off`](string_split_off/README.md) | Cut at an offset and get the tail as an owned `String` |


## Looking at it, and giving it away

The first four borrow; the rest consume.

| method | what it does |
|---|---|
| [`len`](string_len/README.md) | The **byte** count |
| [`is_empty`](string_is_empty/README.md) | `len() == 0` — capacity is irrelevant |
| [`as_str`](string_as_str/README.md) | The whole thing as `&str` — usually supplied by deref coercion |
| [`as_mut_str`](string_as_mut_str/README.md) | As `&mut str` — mutable, but length-preserving |
| [`as_bytes`](string_as_bytes/README.md) | The UTF-8 bytes, borrowed |
| [`into_bytes`](string_into_bytes/README.md) | The `Vec<u8>`, owned — no copy, no UTF-8 promise |
| [`into_boxed_str`](string_into_boxed_str/README.md) | A `Box<str>` — shrinks, and drops the capacity field |
| [`into_raw_parts`](string_into_raw_parts/README.md) | Pointer, length, capacity — and the destructor stops running |
| [`leak`](string_leak/README.md) | A `&'static mut str` that is **never freed** |


## `unsafe` — the checks removed

| method | what it does |
|---|---|
| [`from_utf8_unchecked`](string_from_utf8_unchecked/README.md) | Take a `Vec<u8>` with no validation |
| [`as_mut_vec`](string_as_mut_vec/README.md) | The underlying `Vec<u8>`, resizable — you owe valid UTF-8 back |
| [`from_raw_parts`](string_from_raw_parts/README.md) | Rebuild from pointer, length and capacity — all three exact |


---

## Where to go next

- [`str` methods](../str_methods/README.md) — the borrowed half, 83 more pages, and where all the reading lives
- [The strings arc](../README.md) — the lessons these pages are a reference for
- [`String` in the standard library ↗](https://doc.rust-lang.org/std/string/struct.String.html) — the official documentation these pages explain
