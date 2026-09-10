# The string crates, and what `std` makes you do without them

**Level:** reference · the reading list

**One line:** Fifteen crates cover the text jobs `std` leaves out — graphemes, normalization, legacy encodings, locales, bytes that are not UTF-8, short strings off the heap — and each one sits beside the lesson here that says what `std` gives you instead.

**Nothing on this page ran.** Every example in this library is bare `rustc` with no crates, so a crate can be described here and never demonstrated. What a row says a crate is for paraphrases the crate's own published description; the *Latest* and *Released* columns were read from the crates.io API on 2026-09-10 and are a snapshot. The *`std` instead* column links lessons whose claims *are* checked.

---

## Text as a reader sees it

`std` counts and compares scalar values and bytes. It has no notion of what a reader calls one character, of which two spellings are the same word, or of what the reader's language says about either.

| Crate | What it is for | Latest | Released | `std` instead |
|---|---|---|---|---|
| [`unicode-segmentation` ↗](https://docs.rs/unicode-segmentation) | Grapheme-cluster, word and sentence boundaries, by Unicode's UAX #29 | 1.13.3 | 2026-06-01 | `chars()` yields scalar values, not what a reader counts — [Four lengths, and which one the other system means](../four_lengths/README.md) |
| [`unicode-normalization` ↗](https://docs.rs/unicode-normalization) | The four normalization forms — NFC, NFD, NFKC, NFKD — by UAX #15 | 0.1.25 | 2025-10-30 | Nothing: `==` and `sort()` compare the bytes you were given — [Comparing and sorting text](../comparing_strings/README.md) |
| [`icu` ↗](https://docs.rs/icu) | The meta-crate of the ICU4X project: its internationalization components, re-exported, with the CLDR locale data they are driven by | 2.3.1 | 2026-08-20 | Nothing: `str` has no locale, so case mapping and ordering are the same on every machine — see the row above |

## Bytes that are not UTF-8

A `String` is UTF-8 by type. These are for text that arrived some other way, and for the byte order of the numbers around it.

| Crate | What it is for | Latest | Released | `std` instead |
|---|---|---|---|---|
| [`encoding_rs` ↗](https://docs.rs/encoding_rs) | The WHATWG Encoding Standard, as implemented for Firefox's Gecko engine — the legacy encodings a web page or an old file may be in | 0.8.41 | 2026-09-09 | UTF-8 and UTF-16, and nothing else — [`String::from_utf16`](../string_methods/string_from_utf16/README.md) |
| [`bstr` ↗](https://docs.rs/bstr) | A string type for bytes that are mostly text but not promised to be UTF-8 | 1.13.1 | 2026-08-10 | `&[u8]`, and `OsStr` for what the operating system hands you — [Six kinds of string](../six_kinds_of_string/README.md) |
| [`widestring` ↗](https://docs.rs/widestring) | Owned wide strings, `u16` and `u32`, for Windows APIs and other FFI | 1.2.1 | 2025-10-09 | Conversion only — no UTF-16 string type — [`str::encode_utf16`](../str_methods/str_encode_utf16/README.md) |
| [`utf16string` ↗](https://docs.rs/utf16string) | String types that store UTF-16 directly | 0.2.0 | 2020-10-10 | as for `widestring` |
| [`byteorder` ↗](https://docs.rs/byteorder) | Reading and writing numbers big- or little-endian | 1.5.0 | 2023-10-06 | `to_be_bytes` / `from_le_bytes` and friends, on the integer types themselves — [Meet the byte](../../19_Numbers/meet_the_byte/README.md) |

## Short strings, off the heap

A non-empty `String` always owns a heap buffer — three words on the stack, the text behind a pointer. Most of these keep short text *inside* the handle instead, which is the **small-string optimization**; the last makes a clone cheaper than a copy.

| Crate | What it is for | Latest | Released | `std` instead |
|---|---|---|---|---|
| [`compact_str` ↗](https://docs.rs/compact_str) | A string that stays on the stack while it is short enough, and moves to the heap when it is not | 0.10.0 | 2026-07-13 | Always the heap — [The anatomy of a `String`](../anatomy_of_a_string/README.md) |
| [`smartstring` ↗](https://docs.rs/smartstring) | Short strings stored inline in the handle | 1.0.1 | 2022-03-24 | as above |
| [`smallstr` ↗](https://docs.rs/smallstr) | A `String`-like type built on `smallvec`'s inline buffer | 0.3.1 | 2025-08-13 | as above |
| [`tinystr` ↗](https://docs.rs/tinystr) | ASCII only, with a fixed maximum length, never on the heap | 0.8.4 | 2026-08-13 | as above |
| [`arraystring` ↗](https://docs.rs/arraystring) | A fixed-capacity string on the stack | 0.3.0 | 2019-01-21 | as above |
| [`flexstr` ↗](https://docs.rs/flexstr) | An immutable, clone-efficient replacement for `String`, in local and thread-shared forms for `str`, bytes, `CStr`, `OsStr` and `Path` | 0.11.7 | 2026-02-28 | `Rc<str>` / `Arc<str>` — [The third owned form](../boxed_str/README.md) |

## Allocation you did not ask for

| Crate | What it is for | Latest | Released | `std` instead |
|---|---|---|---|---|
| [`cow-utils` ↗](https://docs.rs/cow-utils) | Copy-on-write versions of the `str` methods that return a new `String` even when nothing needed changing — plus its own stand-in for `Pattern`, which `std` still keeps unstable | 0.1.3 | 2023-09-26 | `replace` hands back a new `String` every time; `Cow` is the tool — [Replacing part of a string](../replacing_in_a_string/README.md) · [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) |

## Reading the columns

- **Released** is the newest version's publish date. crates.io also reports an `updated_at`, which moves for reasons that are not releases, so it is not used here. Two of these have not published since 2019 and 2020; a small crate can simply be finished, but that is worth checking before you depend on one.
- **Downloads are left out.** crates.io counts every fetch, including each time a crate arrives as someone else's dependency, so the number measures how widely a crate is *pulled in* — which is a different question from whether you should choose it.
- **`std` instead** names what you have without the crate. Most programs never need a row of this table.

## Where `std` draws the line

`std` does carry some of Unicode's tables — the ones behind a `char`'s own properties and its case mappings, such as `is_alphabetic`, `is_whitespace` and `to_lowercase`. It stops there: normalization, segmentation and locale data are left to crates. [The hard strings ↗](https://masiarek.github.io/encodings-learning-library/14_Resources/hard_strings/index.html), in the sibling encodings library, runs the same sixteen look-alike pairs through Rust's `std` and through Python's standard library, and shows where each one stops.

## See also

- [STRINGS.md](../../STRINGS.md) — every string lesson, in the order the questions come up
- [Strings: links, books and videos](../resources/README.md) — the reading list this page sits beside
- [When `String` is too slow](../when_string_is_too_slow/README.md) — the stub that decides when the third table is worth reaching for
- [Scratch programs with a crate](../../05_Tooling/scratch_with_a_crate/README.md) — how to try one of these without starting a project
