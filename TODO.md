# What to cover next — the strings backlog

**Level:** reference · the backlog

**One line:** Three backlogs for the strings chapter, all filed by Adam — **198 terms** that should each be answerable somewhere, a **progression** that puts the existing lessons in teaching order, and **20 katas** that supply the practice neither of the other two has.

| # | Backlog | What it is | The work in it |
|---|---|---|---|
| 1 | [The vocabulary](#the-list) | 198 terms in 15 groups | Mostly a glossary sweep |
| 2 | [The progression](#second-backlog-the-progression) | 8 steps, 8 exercises, a mental model | One ordering decision, then exercises |
| 3 | [The twenty katas](#third-backlog-the-twenty-katas) | Graded `hello()` → Levenshtein, as test cases | ~14–18 new programs — the real build |

The first is a dictionary, the second a route, the third the practice. They overlap on purpose; where two of them ask for the same page, the later section says so and names the merge.

## The job, for Thursday 2026-09-10 — backlog 1

**Result — done 2026-09-10.** All 198 are answerable. By where the first link goes: **120** to a lesson, **63** to a page of the method reference, **12** to a stub (the term is on a page, but no example backs that page yet), and **3** to a page of the sibling encodings library, which owns the Unicode theory this one leans on. **65** terms gained a glossary entry. The fifteen crates share one new page, [The string crates](14_Strings/string_crates/README.md), since none of them can run here. Three things the sweep found are worth more than the ticks: the stub for [String parameters worth copying](14_Strings/string_api_design/README.md) claimed `PathBuf` satisfies `impl AsRef<str>`, and it does not (`E0277`; corrected in the same sweep); `std::string::ParseError` is a type alias for `Infallible` that no page mentioned; and `Pin<Box<str>>` compiles and pins nothing, because `str` is `Unpin` — both measured on 1.98.0.

Adam's ask, in his words: *"make sure we have a page for all these terms (even if a stub)."* The list is below, unedited apart from grouping and code formatting.

**Audit before writing anything.** [`14_Strings/`](14_Strings/README.md) already holds 33 lessons and [STRINGS.md](STRINGS.md) already maps them, so the honest first pass is a sweep that says, per term, which of three states it is in:

1. **A lesson teaches it** — nothing to do but check [GLOSSARY.md](GLOSSARY.md) has the entry and it points at that lesson.
2. **The glossary defines it and links somewhere real** — that is coverage. A term does not need a folder to be answerable.
3. **Neither** — only then is there a page to write, and the next question is whether it is a page of its own or a paragraph on one that exists.

That order matters because of what a new folder costs. A folder name here is a permanent URL, so minting 198 stub folders would mint 198 permanent URLs before anyone knows what any of those pages says — and most of these terms are not lessons, they are *vocabulary*, which is what a glossary is for. Expect the sweep's answer to be mostly "glossary entry pointing at an existing lesson", with a short list of genuine gaps behind it.

## Three things already known, so they do not get rediscovered

**The crates group cannot have runnable examples.** Every example in this library is bare `rustc`, no Cargo and no crates, so the fifteen crate names below can be *described* and cannot be *demonstrated*. That is a single survey page — what each crate is for, and what std makes you do without it — not fifteen stubs. The same page is where `unicode-segmentation` and `unicode-normalization` belong, and the sibling encodings library's corpus page already states the std gap they fill.

**Some of these are one lesson, not several.** `Chars` / `CharIndices` / `Bytes` are three types answering one question, and `to_lowercase` / `to_uppercase` / `make_ascii_lowercase` / `eq_ignore_ascii_case` are four names for one decision. Group the terms into the lesson that answers them and let the glossary carry the individual names — a page per method would be a reference manual, and [docs.rs ↗](https://docs.rs) is already better at that than this library will ever be.

**A few sit outside strings entirely.** `MaybeUninit`, `UnsafeCell`, `NonNull`, data races and pointer alignment are ownership and unsafety topics that a string page would borrow rather than own. Check [18_Ownership](18_Ownership/README.md) and [09_Advanced](09_Advanced/README.md) before assuming they belong in [14_Strings](14_Strings/README.md).

## The list

198 terms. A box is ticked when the term is *answerable* — a lesson, or a glossary entry that links to one — not when it has a folder.

**Swept 2026-09-10.** Every box is ticked, and the link beside each term is where its answer is — the first link is the one to read. `glossary` marks a term that gained an entry in [GLOSSARY.md](GLOSSARY.md) in the same sweep.

### Core types (15)

- [x] `str` — [`str` is unsized](14_Strings/str_is_unsized/README.md) · [glossary](GLOSSARY.md)
- [x] `String` — [`String` vs `&str`](14_Strings/string_vs_str/README.md) · [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md)
- [x] `&str` — [`String` vs `&str`](14_Strings/string_vs_str/README.md) · [String slices](14_Strings/string_slices/README.md)
- [x] `&String` — [`String` vs `&str`](14_Strings/string_vs_str/README.md) · [Coercion: the conversion you never write](29_Conversion/coercion/README.md)
- [x] `Box<str>` — [The third owned form: `Box<str>`, `Rc<str>`, `Arc<str>`](14_Strings/boxed_str/README.md) · [glossary](GLOSSARY.md)
- [x] `Cow<str>` — [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md)
- [x] `OsStr` — [Six kinds of string](14_Strings/six_kinds_of_string/README.md)
- [x] `OsString` — [Six kinds of string](14_Strings/six_kinds_of_string/README.md)
- [x] `CStr` — [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [Calling C — the call is free, the data is not](09_Advanced/calling_c/README.md)
- [x] `CString` — [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [Calling C — the call is free, the data is not](09_Advanced/calling_c/README.md)
- [x] `Path` — [`Path` and `PathBuf`](04_Files/path_and_pathbuf/README.md) *(stub)* · [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [glossary](GLOSSARY.md)
- [x] `PathBuf` — [`Path` and `PathBuf`](04_Files/path_and_pathbuf/README.md) *(stub)* · [glossary](GLOSSARY.md)
- [x] `char` — [Meet the `char`](14_Strings/meet_the_char/README.md) · [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md)
- [x] `u8` — [Meet the byte](19_Numbers/meet_the_byte/README.md)
- [x] bytes — [Meet the byte](19_Numbers/meet_the_byte/README.md)


### Encodings (13)

- [x] UTF-8 — [Meet the `char`](14_Strings/meet_the_char/README.md)
- [x] UTF-16 — [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md) · [glossary](GLOSSARY.md)
- [x] UTF-32 — [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md) · [glossary](GLOSSARY.md)
- [x] ASCII — [Meet the byte](19_Numbers/meet_the_byte/README.md) · [`str::is_ascii`](14_Strings/str_methods/str_is_ascii/README.md) · [glossary](GLOSSARY.md)
- [x] WTF-8 — [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [`OsStr`, `Path`, and WTF-8 ↗](https://masiarek.github.io/encodings-learning-library/05_Rust/osstr_path_and_wtf8/index.html) · [glossary](GLOSSARY.md)
- [x] Unicode scalar value — [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md) · [glossary](GLOSSARY.md)
- [x] Unicode code point — [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md) · [glossary](GLOSSARY.md)
- [x] surrogate pairs — [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md) · [glossary](GLOSSARY.md)
- [x] byte order mark (BOM) — [`String::from_utf16le`](14_Strings/string_methods/string_from_utf16le/README.md) · [Byte order and the BOM ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/byte_order_and_bom/index.html) · [glossary](GLOSSARY.md)
- [x] little-endian — [Meet the byte](19_Numbers/meet_the_byte/README.md)
- [x] big-endian — [Meet the byte](19_Numbers/meet_the_byte/README.md)
- [x] locale encoding — [Comparing and sorting text](14_Strings/comparing_strings/README.md) · [Locale and `LC_CTYPE` ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/locale_and_lc_ctype/index.html) · [glossary](GLOSSARY.md)
- [x] platform encoding — [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [glossary](GLOSSARY.md)


### Conversions (14)

- [x] `from_utf8` — [`str::from_utf8`](14_Strings/str_methods/str_from_utf8/README.md) · [`String::from_utf8`](14_Strings/string_methods/string_from_utf8/README.md)
- [x] `from_utf8_lossy` — [`String::from_utf8_lossy`](14_Strings/string_methods/string_from_utf8_lossy/README.md)
- [x] `to_string_lossy` — [Six kinds of string](14_Strings/six_kinds_of_string/README.md)
- [x] `as_bytes` — [`str::as_bytes`](14_Strings/str_methods/str_as_bytes/README.md)
- [x] `as_os_str` — [`Path` and `PathBuf`](04_Files/path_and_pathbuf/README.md) *(stub)*
- [x] `to_str` — [Six kinds of string](14_Strings/six_kinds_of_string/README.md)
- [x] `to_owned` — [`ToOwned`: `Clone` for types whose owned twin is a different type](12_Traits/to_owned/README.md)
- [x] `into_owned` — [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md)
- [x] `to_string` — [Making a `String`](14_Strings/making_a_string/README.md)
- [x] `into_string` — [`str::into_string`](14_Strings/str_methods/str_into_string/README.md)
- [x] `from_utf8_unchecked` — [`str::from_utf8_unchecked`](14_Strings/str_methods/str_from_utf8_unchecked/README.md)
- [x] `as_ptr` — [`str::as_ptr`](14_Strings/str_methods/str_as_ptr/README.md)
- [x] `as_mut_ptr` — [`str::as_mut_ptr`](14_Strings/str_methods/str_as_mut_ptr/README.md)
- [x] `into_bytes` — [`String::into_bytes`](14_Strings/string_methods/string_into_bytes/README.md)


### Operations (13)

- [x] slicing — [String slices](14_Strings/string_slices/README.md)
- [x] indexing — [Meet the `char`](14_Strings/meet_the_char/README.md)
- [x] concatenation — [Concatenating strings](14_Strings/concatenating_strings/README.md)
- [x] trimming — [`str::trim`](14_Strings/str_methods/str_trim/README.md)
- [x] splitting — [Walking a `String`](14_Strings/walking_a_string/README.md) · [Inside a `Split`](14_Strings/inside_a_split/README.md)
- [x] pattern matching — [Searching without splitting](14_Strings/searching_a_string/README.md)
- [x] substring search — [Searching without splitting](14_Strings/searching_a_string/README.md)
- [x] char iteration — [Walking a `String`](14_Strings/walking_a_string/README.md)
- [x] byte iteration — [Walking a `String`](14_Strings/walking_a_string/README.md)
- [x] grapheme clusters — [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md) · [Meet the `char`](14_Strings/meet_the_char/README.md)
- [x] replacement — [Replacing part of a string](14_Strings/replacing_in_a_string/README.md)
- [x] case conversion — [Comparing and sorting text](14_Strings/comparing_strings/README.md)
- [x] normalization — [Comparing and sorting text](14_Strings/comparing_strings/README.md) · [Normalization ↗](https://masiarek.github.io/encodings-learning-library/04_Python/normalization/index.html) · [glossary](GLOSSARY.md)


### Traits a string type implements (15)

- [x] `Display` — [Debug and Display](15_First_Programs/debug_vs_display/README.md)
- [x] `Debug` — [Debug and Display](15_First_Programs/debug_vs_display/README.md)
- [x] `ToString` — [Making a `String`](14_Strings/making_a_string/README.md)
- [x] `FromStr` — [Parsing out of a string](14_Strings/parsing_a_string/README.md)
- [x] `Deref<Target=str>` — [`String` vs `&str`](14_Strings/string_vs_str/README.md)
- [x] `AsRef<str>` — [String parameters worth copying](14_Strings/string_api_design/README.md) *(stub)* · [glossary](GLOSSARY.md)
- [x] `Borrow<str>` — [When the UTF-8 invariant broke](14_Strings/when_the_invariant_broke/README.md) · [glossary](GLOSSARY.md)
- [x] `Into<String>` — [String parameters worth copying](14_Strings/string_api_design/README.md) *(stub)* · [`From` and `Into`](29_Conversion/from_and_into/README.md)
- [x] `From<String>` — [`From` and `Into`](29_Conversion/from_and_into/README.md) · [glossary](GLOSSARY.md)
- [x] `PartialEq` — [The comparison traits](12_Traits/comparison_traits/README.md) *(stub)* · [Comparing and sorting text](14_Strings/comparing_strings/README.md)
- [x] `Eq` — [The comparison traits](12_Traits/comparison_traits/README.md) *(stub)*
- [x] `Ord` — [The comparison traits](12_Traits/comparison_traits/README.md) *(stub)* · [Comparing and sorting text](14_Strings/comparing_strings/README.md)
- [x] `Hash` — [`HashSet`](26_Collections/the_hashset/README.md) · [glossary](GLOSSARY.md)
- [x] `Clone` — [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md)
- [x] `Copy` (for `&str`) — [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md)


### Memory and layout (11)

- [x] heap allocation — [Stack and heap](18_Ownership/stack_and_heap/README.md) · [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md)
- [x] stack allocation — [Stack and heap](18_Ownership/stack_and_heap/README.md)
- [x] capacity — [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md)
- [x] length — [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) · [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md)
- [x] reallocation — [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md)
- [x] fat pointer — [`str` is unsized](14_Strings/str_is_unsized/README.md)
- [x] thin pointer — [`str` is unsized](14_Strings/str_is_unsized/README.md) · [glossary](GLOSSARY.md)
- [x] string interning — [The third owned form: `Box<str>`, `Rc<str>`, `Arc<str>`](14_Strings/boxed_str/README.md) · [glossary](GLOSSARY.md)
- [x] small-string optimization — [The string crates](14_Strings/string_crates/README.md) · [When `String` is too slow](14_Strings/when_string_is_too_slow/README.md) *(stub)* · [glossary](GLOSSARY.md)
- [x] null-terminated — [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [glossary](GLOSSARY.md)
- [x] non-null-terminated — [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [glossary](GLOSSARY.md)


### Raw and FFI (10)

- [x] `std::ffi` — [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [Calling C — the call is free, the data is not](09_Advanced/calling_c/README.md)
- [x] `std::os::raw` — [Calling C — the call is free, the data is not](09_Advanced/calling_c/README.md) · [glossary](GLOSSARY.md)
- [x] `c_char` — [Calling C — the call is free, the data is not](09_Advanced/calling_c/README.md)
- [x] NUL byte — [Six kinds of string](14_Strings/six_kinds_of_string/README.md)
- [x] pointer casting — [What `unsafe` turns off](09_Advanced/what_unsafe_turns_off/README.md) · [glossary](GLOSSARY.md)
- [x] `transmute` — [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md) · [glossary](GLOSSARY.md)
- [x] manual UTF-8 validation — [`str::from_utf8`](14_Strings/str_methods/str_from_utf8/README.md) · [UTF-8 by hand ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/utf8_by_hand/index.html) · [glossary](GLOSSARY.md)
- [x] `unsafe` — [What `unsafe` turns off](09_Advanced/what_unsafe_turns_off/README.md)
- [x] `extern "C"` — [Calling C — the call is free, the data is not](09_Advanced/calling_c/README.md)
- [x] zero-copy — [String slices](14_Strings/string_slices/README.md) · [Inside a `Split`](14_Strings/inside_a_split/README.md) · [glossary](GLOSSARY.md)


### Iterators and views (14)

- [x] `Chars` — [`str::chars`](14_Strings/str_methods/str_chars/README.md)
- [x] `CharIndices` — [`str::char_indices`](14_Strings/str_methods/str_char_indices/README.md)
- [x] `Bytes` — [`str::bytes`](14_Strings/str_methods/str_bytes/README.md)
- [x] `Lines` — [`str::lines`](14_Strings/str_methods/str_lines/README.md)
- [x] `Split` — [`str::split`](14_Strings/str_methods/str_split/README.md) · [Inside a `Split`](14_Strings/inside_a_split/README.md)
- [x] `SplitWhitespace` — [`str::split_whitespace`](14_Strings/str_methods/str_split_whitespace/README.md)
- [x] `RSplit` — [`str::rsplit`](14_Strings/str_methods/str_rsplit/README.md)
- [x] `MatchIndices` — [`str::match_indices`](14_Strings/str_methods/str_match_indices/README.md)
- [x] `Matches` — [`str::matches`](14_Strings/str_methods/str_matches/README.md)
- [x] `RMatchIndices` — [`str::rmatch_indices`](14_Strings/str_methods/str_rmatch_indices/README.md)
- [x] `EncodeUtf16` — [`str::encode_utf16`](14_Strings/str_methods/str_encode_utf16/README.md)
- [x] `EscapeDebug` — [`str::escape_debug`](14_Strings/str_methods/str_escape_debug/README.md)
- [x] `EscapeDefault` — [`str::escape_default`](14_Strings/str_methods/str_escape_default/README.md)
- [x] `EscapeUnicode` — [`str::escape_unicode`](14_Strings/str_methods/str_escape_unicode/README.md)


### Crates and ecosystem (15)

- [x] `unicode-segmentation` — [The string crates](14_Strings/string_crates/README.md) · [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md) · [glossary](GLOSSARY.md)
- [x] `unicode-normalization` — [The string crates](14_Strings/string_crates/README.md) · [Comparing and sorting text](14_Strings/comparing_strings/README.md) · [glossary](GLOSSARY.md)
- [x] `encoding_rs` — [The string crates](14_Strings/string_crates/README.md) · [glossary](GLOSSARY.md)
- [x] `icu` — [The string crates](14_Strings/string_crates/README.md) · [Comparing and sorting text](14_Strings/comparing_strings/README.md) · [glossary](GLOSSARY.md)
- [x] `bstr` — [The string crates](14_Strings/string_crates/README.md) · [Six kinds of string](14_Strings/six_kinds_of_string/README.md) · [glossary](GLOSSARY.md)
- [x] `byteorder` — [The string crates](14_Strings/string_crates/README.md) · [Meet the byte](19_Numbers/meet_the_byte/README.md) · [glossary](GLOSSARY.md)
- [x] `widestring` — [The string crates](14_Strings/string_crates/README.md) · [`str::encode_utf16`](14_Strings/str_methods/str_encode_utf16/README.md) · [glossary](GLOSSARY.md)
- [x] `utf16string` — [The string crates](14_Strings/string_crates/README.md) · [glossary](GLOSSARY.md)
- [x] `compact_str` — [The string crates](14_Strings/string_crates/README.md) · [glossary](GLOSSARY.md)
- [x] `smartstring` — [The string crates](14_Strings/string_crates/README.md) · [glossary](GLOSSARY.md)
- [x] `smallstr` — [The string crates](14_Strings/string_crates/README.md) · [glossary](GLOSSARY.md)
- [x] `tinystr` — [The string crates](14_Strings/string_crates/README.md) · [glossary](GLOSSARY.md)
- [x] `arraystring` — [The string crates](14_Strings/string_crates/README.md) · [glossary](GLOSSARY.md)
- [x] `flexstr` — [The string crates](14_Strings/string_crates/README.md)
- [x] `cow-utils` — [The string crates](14_Strings/string_crates/README.md) · [Replacing part of a string](14_Strings/replacing_in_a_string/README.md) · [glossary](GLOSSARY.md)


### Unicode concepts (12)

- [x] grapheme — [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md) · [Meet the `char`](14_Strings/meet_the_char/README.md)
- [x] code unit — [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md) · [glossary](GLOSSARY.md)
- [x] code point — [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md) · [glossary](GLOSSARY.md)
- [x] scalar value — [Why a `char` is 32 bits wide](14_Strings/why_char_is_32_bits/README.md) · [glossary](GLOSSARY.md)
- [x] combining character — [Meet the `char`](14_Strings/meet_the_char/README.md) · [glossary](GLOSSARY.md)
- [x] normalization forms (NFC, NFD, NFKC, NFKD) — [Comparing and sorting text](14_Strings/comparing_strings/README.md) · [Normalization ↗](https://masiarek.github.io/encodings-learning-library/04_Python/normalization/index.html) · [glossary](GLOSSARY.md)
- [x] case folding — [Comparing and sorting text](14_Strings/comparing_strings/README.md)
- [x] canonical equivalence — [The hard strings ↗](https://masiarek.github.io/encodings-learning-library/14_Resources/hard_strings/index.html) · [Normalization ↗](https://masiarek.github.io/encodings-learning-library/04_Python/normalization/index.html) · [glossary](GLOSSARY.md)
- [x] compatibility equivalence — [The hard strings ↗](https://masiarek.github.io/encodings-learning-library/14_Resources/hard_strings/index.html) · [Normalization ↗](https://masiarek.github.io/encodings-learning-library/04_Python/normalization/index.html) · [glossary](GLOSSARY.md)
- [x] invisible characters — [The hard strings ↗](https://masiarek.github.io/encodings-learning-library/14_Resources/hard_strings/index.html) · [glossary](GLOSSARY.md)
- [x] zero-width joiner — [Meet the `char`](14_Strings/meet_the_char/README.md) · [glossary](GLOSSARY.md)
- [x] variation selector — [Meet the `char`](14_Strings/meet_the_char/README.md) · [glossary](GLOSSARY.md)


### Error types (6)

- [x] `Utf8Error` — [`str::from_utf8`](14_Strings/str_methods/str_from_utf8/README.md) · [glossary](GLOSSARY.md)
- [x] `FromUtf8Error` — [`String::from_utf8`](14_Strings/string_methods/string_from_utf8/README.md) · [glossary](GLOSSARY.md)
- [x] `FromUtf16Error` — [`String::from_utf16`](14_Strings/string_methods/string_from_utf16/README.md) · [glossary](GLOSSARY.md)
- [x] `ParseError` — [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md) · [glossary](GLOSSARY.md)
- [x] `TryFromIntError` — [`TryFrom` and `TryInto`](29_Conversion/tryfrom_and_tryinto/README.md)
- [x] `Infallible` — [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md) · [The never type `!`](15_First_Programs/the_never_type/README.md)


### Common methods (33)

- [x] `len()` — [`str::len`](14_Strings/str_methods/str_len/README.md) · [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md)
- [x] `is_empty()` — [`str::is_empty`](14_Strings/str_methods/str_is_empty/README.md)
- [x] `contains()` — [`str::contains`](14_Strings/str_methods/str_contains/README.md)
- [x] `starts_with()` — [`str::starts_with`](14_Strings/str_methods/str_starts_with/README.md)
- [x] `ends_with()` — [`str::ends_with`](14_Strings/str_methods/str_ends_with/README.md)
- [x] `find()` — [`str::find`](14_Strings/str_methods/str_find/README.md) · [Searching without splitting](14_Strings/searching_a_string/README.md)
- [x] `rfind()` — [`str::rfind`](14_Strings/str_methods/str_rfind/README.md)
- [x] `replace()` — [`str::replace`](14_Strings/str_methods/str_replace/README.md) · [Replacing part of a string](14_Strings/replacing_in_a_string/README.md)
- [x] `replacen()` — [`str::replacen`](14_Strings/str_methods/str_replacen/README.md)
- [x] `to_lowercase()` — [`str::to_lowercase`](14_Strings/str_methods/str_to_lowercase/README.md) · [Comparing and sorting text](14_Strings/comparing_strings/README.md)
- [x] `to_uppercase()` — [`str::to_uppercase`](14_Strings/str_methods/str_to_uppercase/README.md)
- [x] `repeat()` — [`str::repeat`](14_Strings/str_methods/str_repeat/README.md)
- [x] `push()` — [`String::push`](14_Strings/string_methods/string_push/README.md)
- [x] `push_str()` — [`String::push_str`](14_Strings/string_methods/string_push_str/README.md)
- [x] `pop()` — [`String::pop`](14_Strings/string_methods/string_pop/README.md)
- [x] `insert()` — [`String::insert`](14_Strings/string_methods/string_insert/README.md)
- [x] `insert_str()` — [`String::insert_str`](14_Strings/string_methods/string_insert_str/README.md)
- [x] `remove()` — [`String::remove`](14_Strings/string_methods/string_remove/README.md)
- [x] `truncate()` — [`String::truncate`](14_Strings/string_methods/string_truncate/README.md)
- [x] `clear()` — [`String::clear`](14_Strings/string_methods/string_clear/README.md)
- [x] `drain()` — [`String::drain`](14_Strings/string_methods/string_drain/README.md)
- [x] `split_off()` — [`String::split_off`](14_Strings/string_methods/string_split_off/README.md)
- [x] `retain()` — [`String::retain`](14_Strings/string_methods/string_retain/README.md)
- [x] `reserve()` — [`String::reserve`](14_Strings/string_methods/string_reserve/README.md)
- [x] `shrink_to_fit()` — [`String::shrink_to_fit`](14_Strings/string_methods/string_shrink_to_fit/README.md)
- [x] `with_capacity()` — [`String::with_capacity`](14_Strings/string_methods/string_with_capacity/README.md)
- [x] `from_raw_parts()` — [`String::from_raw_parts`](14_Strings/string_methods/string_from_raw_parts/README.md)
- [x] `make_ascii_lowercase()` — [`str::make_ascii_lowercase`](14_Strings/str_methods/str_make_ascii_lowercase/README.md)
- [x] `make_ascii_uppercase()` — [`str::make_ascii_uppercase`](14_Strings/str_methods/str_make_ascii_uppercase/README.md)
- [x] `is_ascii()` — [`str::is_ascii`](14_Strings/str_methods/str_is_ascii/README.md)
- [x] `eq_ignore_ascii_case()` — [`str::eq_ignore_ascii_case`](14_Strings/str_methods/str_eq_ignore_ascii_case/README.md)
- [x] `escape_default()` — [`str::escape_default`](14_Strings/str_methods/str_escape_default/README.md)
- [x] `parse::<T>()` — [`str::parse`](14_Strings/str_methods/str_parse/README.md) · [Parsing out of a string](14_Strings/parsing_a_string/README.md)


### Pointer and safety (11)

- [x] dangling pointer — [Use-after-free](31_C_and_Cpp/use_after_free/README.md)
- [x] null pointer — [Null dereference](31_C_and_Cpp/null_dereference/README.md) · [Nullable pointers](17_Option_and_Result/nullable_pointers/README.md)
- [x] pointer alignment — [`Allocator::shrink`](09_Advanced/allocator_shrink/README.md) · [glossary](GLOSSARY.md)
- [x] memory leak — [`String::leak`](14_Strings/string_methods/string_leak/README.md)
- [x] buffer overflow — [Buffer overruns](31_C_and_Cpp/buffer_overruns/README.md)
- [x] out-of-bounds — [Buffer overruns](31_C_and_Cpp/buffer_overruns/README.md) · [String slices](14_Strings/string_slices/README.md)
- [x] invalid UTF-8 — [`str::from_utf8`](14_Strings/str_methods/str_from_utf8/README.md)
- [x] data race (via `unsafe`) — [Data races](31_C_and_Cpp/data_races/README.md)
- [x] `MaybeUninit` — [Uninitialized reads](31_C_and_Cpp/uninitialized_reads/README.md) · [`Vec::spare_capacity_mut`](26_Collections/vec_methods/vec_spare_capacity_mut/README.md) · [glossary](GLOSSARY.md)
- [x] `NonNull` — [Nullable pointers](17_Option_and_Result/nullable_pointers/README.md) · [glossary](GLOSSARY.md)
- [x] `UnsafeCell` — [Interior mutability](09_Advanced/interior_mutability/README.md) *(stub)* · [glossary](GLOSSARY.md)


### Patterns and matching (9)

- [x] string literal — [`&'static str`](14_Strings/static_str/README.md) · [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md)
- [x] raw string (`r"..."`, `r#"..."#`) — [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md)
- [x] byte string (`b"..."`) — [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md) · [RFC 69 — how Rust got `b'A'`](14_Strings/rfc_69_byte_literals/README.md)
- [x] raw byte string (`br"..."`) — [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md)
- [x] multiline string — [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md)
- [x] escape sequences (`\n`, `\t`, `\u{...}`, `\x..`) — [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md)
- [x] char literal — [Meet the `char`](14_Strings/meet_the_char/README.md)
- [x] the `Pattern` trait — [Searching without splitting](14_Strings/searching_a_string/README.md)
- [x] `Sealed` — [Searching without splitting](14_Strings/searching_a_string/README.md) · [Sealed traits, C-SEALED ↗](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed) · [glossary](GLOSSARY.md)


### Smart pointers and wrappers (7)

- [x] `Rc<str>` — [The third owned form: `Box<str>`, `Rc<str>`, `Arc<str>`](14_Strings/boxed_str/README.md) · [glossary](GLOSSARY.md)
- [x] `Arc<str>` — [The third owned form: `Box<str>`, `Rc<str>`, `Arc<str>`](14_Strings/boxed_str/README.md) · [glossary](GLOSSARY.md)
- [x] `Mutex<String>` — [Lock poisoning](09_Advanced/mutex_poisoning/README.md)
- [x] `RwLock<String>` — [`RwLock` and atomics](09_Advanced/rwlock_and_atomics/README.md) *(stub)*
- [x] `RefCell<String>` — [Interior mutability](09_Advanced/interior_mutability/README.md) *(stub)*
- [x] `Cell<&str>` — [Interior mutability](09_Advanced/interior_mutability/README.md) *(stub)*
- [x] `Pin<Box<str>>` — [There is no `Move` trait](18_Ownership/no_move_trait/README.md) · [glossary](GLOSSARY.md)

---

## Second backlog — the progression

**Level:** reference · the backlog

**One line:** Adam's step-by-step route from ownership to fluent string code, filed 2026-09-08 — a *route* rather than a dictionary, and the half of it this library has least of is the practice.

Where [the vocabulary list](#the-list) asks *"is this term answerable?"*, this asks a different question: **can a person walk from nothing to fluent in a fixed order, and write a program at each stop?** The library already has the lessons — [`14_Strings/`](14_Strings/README.md) holds 33 of them and [STRINGS.md](STRINGS.md) maps them by the question each answers. What it has never had is the *order*, stated as a path, with something to write at every step.

**The same audit rule applies as above.** Every "where it should land" below is a **routing hypothesis, not a coverage claim** — it names the page that ought to answer the item so the sweep can confirm or contradict it one page at a time. None of it has been read against the page's actual text yet. Tick a box when the page has been opened and it really does answer.

### The open question this backlog has to settle first

[STRINGS.md](STRINGS.md) already calls itself *"the door to every lesson about it, **in the order the questions come up**"* — so the library has a route already, and it is ordered by *the question a reader arrives with*. Adam's plan is ordered by *prerequisite*: ownership and memory before any string type, conversions before programs, programs before edge cases. **These are two different orderings of the same 33 lessons, and both are defensible.** So the first decision, before any page gets written:

- **(a)** a new page — a study path that links the existing lessons in Adam's order and owns the exercises, leaving STRINGS.md as the by-question map; or
- **(b)** a second table *inside* STRINGS.md — "if you are starting from nothing, read them in this order" — and no new URL at all.

**(b) is cheaper and probably right for the route itself**, since a folder name is a permanent URL. But the exercises below are the part that has no home under either option, and they are the part with real work in them. Decide this before minting anything.

### Foundation first — the four prerequisites

Adam puts these *before* any string page, which the library agrees with: STRINGS.md already files them under "the lessons strings lean on".

| Prerequisite | Where it should land | Note |
|---|---|---|
| Ownership & borrowing rules | [Ownership and moves](18_Ownership/ownership_and_moves/README.md) · [Borrowing](18_Ownership/borrowing/README.md) | Already the worked example half these pages use |
| Stack vs heap memory | [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) | Three words on the stack, bytes on the heap |
| References vs values | [Borrowing](18_Ownership/borrowing/README.md) | |
| `Copy` vs `Clone` vs `Move` | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | Why `&str` copies and one `String` field moves a whole struct |
| Slices (`&[T]`), and `&str` as `&[u8]` + a guarantee | [String slices](14_Strings/string_slices/README.md) · [Arrays and slices](26_Collections/arrays_and_slices/README.md) | The "UTF-8 guarantee" framing is the bridge between the two pages — check it is actually said on one of them |

### The seven claims a reader must be able to state

Adam's "core concepts to master", as claims a reader should be able to make unprompted.

| # | The claim | Where it should land |
|---|---|---|
| 1 | `str` is unsized — you can never own a bare `str` | [`str` is unsized](14_Strings/str_is_unsized/README.md) |
| 2 | `&str` is a fat pointer: (data pointer, length) | [`str` is unsized](14_Strings/str_is_unsized/README.md) — the fat pointer's second word |
| 3 | `String` is a growable heap buffer: (pointer, length, capacity) | [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) |
| 4 | `String` derefs to `&str` via `Deref<Target = str>` | [`String` vs `&str`](14_Strings/string_vs_str/README.md) |
| 5 | `&String` auto-coerces to `&str` (deref coercion) | [`String` vs `&str`](14_Strings/string_vs_str/README.md) |
| 6 | UTF-8 is variable width, 1–4 bytes | [Meet the `char`](14_Strings/meet_the_char/README.md) · [Four lengths](14_Strings/four_lengths/README.md) |
| 7 | Slicing by byte index panics off a char boundary | [String slices](14_Strings/string_slices/README.md) |

### The eight steps

| Step | Adam's ask | Where it should land | Likely state |
|---|---|---|---|
| 1 | Read the official docs — `std::string::String`, `std::str`, `std::primitive::str`, Book ch. 4 and ch. 8 | [Strings: links, books and videos](14_Strings/resources/README.md) | Covered — confirm the Book chapters are named by number |
| 2 | Draw the memory: `String` on the heap vs `&str` into read-only memory; compare `String::from("hello")` / `"hello"` / `&String::from("hello")` | [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) · [`&'static str`](14_Strings/static_str/README.md) | The picture exists; the **three-way comparison as one exercise** probably does not |
| 3 | Practise the conversions — `&String`→`&str`, `String`→`&str`, `&str`→`String` four ways, `String`→`&[u8]`, `Vec<u8>`→`String` | [Making a `String`](14_Strings/making_a_string/README.md) · [`String` vs `&str`](14_Strings/string_vs_str/README.md) | Mostly covered; `from_utf8` / `as_bytes` may only live in the method reference |
| 4 | Write small programs — take `&str` return `String`; take `String` return `&str` (the lifetime trap); `&str` in a struct; build with `push_str` vs `format!` vs `+` | [Building a `String`](14_Strings/building_a_string/README.md) · [Concatenating strings](14_Strings/concatenating_strings/README.md) · [How to learn lifetimes](18_Ownership/how_to_learn_lifetimes/README.md) | **The programs are the gap** — the explanations exist, the write-it-yourself does not |
| 5 | Edge cases — `"héllo"`, `"日本語"`, `"🦀"`, `.len()` vs `.chars().count()`, the boundary panic, `.get(0..3)`, byte vs char vs grapheme | [Meet the `char`](14_Strings/meet_the_char/README.md) · [Four lengths](14_Strings/four_lengths/README.md) · [Walking a `String`](14_Strings/walking_a_string/README.md) | Best-covered step in the plan |
| 6 | `Cow<str>` — functions that sometimes borrow and sometimes own; `String::from_utf8_lossy()` | [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md) | Covered; check `from_utf8_lossy` is the named example |
| 7 | Study the APIs — every method on `str`, every method on `String`, the pattern-based ones | [`str` methods](14_Strings/str_methods/README.md) (83) · [`String` methods](14_Strings/string_methods/README.md) (42) | Covered, and more thoroughly than the plan asks |
| 8 | Common patterns — builder, `&str` parameters, `String` returns, `impl Into<String>` / `impl AsRef<str>` | [String parameters worth copying](14_Strings/string_api_design/README.md) | **Still a stub** — this step is the one the plan most directly asks to finish |

### The eight exercises

This is where the plan actually adds work. [KATAS.md](KATAS.md) has 170 katas; a grep for these found **one**.

| # | Exercise | State | Note |
|---|---|---|---|
| 1 | Memory — without running, where does each of `a`…`e` live, and what is its size? | **Gap** | The five-binding table (`&str`, `String`, `&String`, `.as_str()`, `&&str`) is a good kata for [anatomy](14_Strings/anatomy_of_a_string/README.md); no crate, no I/O, prints sizes — cheap to write |
| 2 | Lifetimes — why does `fn longest(a: &str, b: &str) -> &str` not compile, and how is it fixed? | **Gap** | `E0106`. Appears in [GLOSSARY.md](GLOSSARY.md) and [KATAS.md](KATAS.md) but not as this canonical exercise; belongs on [How to learn lifetimes](18_Ownership/how_to_learn_lifetimes/README.md) |
| 3 | UTF-8 — what does `"🦀"` print for `.len()` and `.chars().count()`, and what happens at `&s[0..1]`? | **Likely covered** | Confirm on [Meet the `char`](14_Strings/meet_the_char/README.md); if it is there, this is a tick, not a task |
| 4 | Ownership transfer — which of `take_string(s)` / `take_str(&s)` / `take_str(s.as_str())` / `take_string(&s)` compile? | **Gap** | The fourth line is the lesson: `E0308`, and the fix is not `&`. Fits [`String` vs `&str`](14_Strings/string_vs_str/README.md) |
| 5 | Build a CSV parser — lines from a `&str`, split on commas, **handle quoted fields**, trim, return `Vec<String>` | **Gap — the biggest one** | The quoted-field rule is what makes it a real exercise rather than a `split(',')` demo. Bare `rustc`, no crate needed |
| 6 | Implement `to_camel_case` — `&str` in, `String` out, handling spaces, underscores and hyphens | **Gap** | Small, self-contained, exercises `char` boundaries and `push` |
| 7 | Zero-copy logger — store log lines as `&str` into a shared buffer | **Gap, and partly out of scope** | Adam names `bstr` / `memchr`; per the crates rule above, **the crate half cannot be demonstrated here** — the std-only version (lifetimes tying views to one owned buffer) is the runnable part |
| 8 | Tiny string interner — dedupe into a `Vec<String>`, hand back `&str` handles | **Partly covered** | [K138](KATAS.md) already interns a repeated column via `Rc<str>` on [The third owned form](14_Strings/boxed_str/README.md). Decide: extend K138, or write the `Vec<String>` + handle variant as its own |

### Common mistakes — Adam's list

Worth keeping as a checklist even where a lesson covers it, because this is the set a reviewer scans for.

- [ ] Returning `&str` that points into a local `String`
- [ ] Slicing at a non-char boundary
- [ ] Comparing `String` with `&str` incorrectly
- [ ] Forgetting `.as_str()` where it is needed
- [ ] `+` with two `&str` — the left operand must be an owned `String` ([Concatenating strings](14_Strings/concatenating_strings/README.md) is the whole of this one)
- [ ] Assuming `.len()` counts characters — it counts bytes
- [ ] Storing `&str` in a struct without a lifetime parameter
- [ ] Reaching for `String` where `&str` would do — an allocation nobody asked for

### The mental model, as Adam states it

```text
String  =  owned, mutable, heap, like Vec<u8> + UTF-8 valid
&str    =  borrowed, immutable, points to String/static/other
str     =  unsized type, never directly held, only behind reference
String literal = &'static str
```

This is the same "owner and view" pattern [STRINGS.md](STRINGS.md) opens with, said in four lines. If the route page in option (a) gets written, this block is its opening.

### Rule of thumb

- **Function parameter:** `&str`, unless you need to consume it
- **Function return:** `String` if it is new data, `&str` if it borrows the input (with lifetimes)
- **Struct field:** `String` — rarely `&str`, and only with a lifetime parameter
- **Temporary:** `&str` for read-only access

### Resources Adam names

Check these against [Strings: links, books and videos](14_Strings/resources/README.md) and add whatever is missing:

- The Rust Book — ch. 4 (ownership), ch. 8 (strings), ch. 10 (lifetimes)
- Rust by Example — the Strings section
- *Programming Rust* (O'Reilly) — the text chapter
- The std source: `library/alloc/src/string.rs`, `library/core/src/str/` (the plan's `src/liballoc/…` paths are pre-2020 names — use the current ones)
- `rustc --explain` for `E0106`, `E0506`, `E0716` — [ERRORS.md](ERRORS.md) already carries the last two
- Practice sets: Advent of Code string-heavy days, the Exercism Rust track

---

## Third backlog — the twenty katas

**Level:** reference · the backlog

**One line:** Adam's graded set of 20 string katas, filed 2026-09-08, from `hello()` to Levenshtein — the practice half the [progression](#second-backlog-the-progression) above asks for, already written as test cases.

These arrived as `#[cfg(test)] mod tests` blocks with the body left as `// Your code here`, which is the right shape for a kata: the tests *are* the specification. [KATAS.md](KATAS.md) currently holds 170; these would be K171–K190 if all twenty are taken.

### The shape question is already settled — do not re-argue it

Adam's katas are written in `cargo test` form, and this library has no Cargo. **That is not a problem**, and the precedent is already here:

- **`rustc --edition 2024 --test <file>.rs` builds the harness on a loose file** — no Cargo, no crates. It is defined in [GLOSSARY.md](GLOSSARY.md), taught on [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md), and used by [Where a test goes](28_Testing/where_a_test_goes/README.md).
- **But the diffed `.out` artefact cannot come from the harness.** The test harness runs in parallel, so its output order varies — [Where a test goes](28_Testing/where_a_test_goes/README.md) says so in the title of its own fence, and its recorded `.out` is a **`main()` run**, not a harness run.

So the house pattern for all twenty is the one `where_a_test_goes.rs` already uses: **keep Adam's `#[test]` functions, and add a `main()`** that walks the same cases in fixed order and prints them. The tests give the reader `cargo test`-shaped practice; the `main()` gives the library its byte-stable recorded output. A harness run may still appear on the page as a labelled fence, never as the diffed artefact.

### Four things found reading the twenty

Recorded so they are not rediscovered one at a time while writing.

1. **Kata 1 has nothing to fix.** The body already returns `"Hello, World!"` and the test already passes; the `// Make it work` comment has no bug behind it. Either give it a real defect (`"Hello World!"`, a missing comma — the assertion then earns its keep) or drop it and start the set at Kata 2.

2. **Kata 7 is `str::replace` with a different name.** As written the answer is one line, so it teaches nothing. It needs the constraint Kata 4 already has — Kata 4 says *"without using `.rev()`"*, and Kata 7 should say *"without using `replace()`"*. Then it becomes a real exercise in `find`, byte offsets and `push_str`, and it pairs with [Replacing part of a string](14_Strings/replacing_in_a_string/README.md), whose lesson is that a chain of replaces is not a substitution table.

3. **Kata 18 asks for graphemes and tests for `char`s.** It says *"count grapheme clusters (user-perceived characters) … without external crates"*, but every assertion — `"hello"` 5, `"héllo"` 5, `"日本語"` 3 — is satisfied by `.chars().count()`, because that `é` is one precomposed scalar. The test cannot tell the two answers apart, so the stated goal and the specification disagree. Worse, the two are *genuinely* different work: real grapheme clustering is UAX #29, which is what `unicode-segmentation` exists for and which the no-crates rule forbids. Fix it one of two ways — rename it to `count_chars` and keep it easy, or keep the name and add a decomposed case (`"he\u{301}llo"`, or a flag, or a ZWJ family emoji) and scope it honestly to a subset of UAX #29. [Four lengths](14_Strings/four_lengths/README.md) is the page that already draws this distinction, and it should be the kata's home either way.

4. **Four of them are algorithm katas wearing string clothes.** Kata 13 (regex), 14 (word ladder, a BFS), 15 (interleaving, a DP) and 20 (edit distance, a DP) exercise dynamic programming and graph search; the string is the input, not the subject. That is fine — but this library's katas are about *Rust's behaviour*, and these would be the first that are not. Decide whether they join the set, go to a separate "algorithms on text" group, or stay out. Kata 16 is the counter-example and should definitely be in: `fn longest_palindrome(s: &str) -> &str` returns a slice **borrowed from its input**, which is lifetime elision doing exactly the thing [Step 4](#the-eight-steps) of the progression is about.

### The twenty

Adam's levels, kept as he graded them. "Lands on" is a routing hypothesis, same rule as everywhere above.

#### Beginner

| # | Kata | What it exercises | Lands on |
|---|---|---|---|
| 1 | `hello()` returns `"Hello, World!"` | `&str` → `String` | [Making a `String`](14_Strings/making_a_string/README.md) — **needs a real defect first**, see above |
| 2 | `full_name(first, last)` with a space between | `format!` vs `+`; the `("", "")` case proves the space is unconditional | [Concatenating strings](14_Strings/concatenating_strings/README.md) |
| 3 | `is_blank(s)` — empty or all whitespace | `trim().is_empty()`, and that `"\t\n"` is whitespace | [`str` methods](14_Strings/str_methods/README.md) |

#### Intermediate

| # | Kata | What it exercises | Lands on |
|---|---|---|---|
| 4 | `reverse_string(s)` without `.rev()` | Manual iteration over `chars()`. Worth adding a note that reversing by `char` **breaks combining marks** — the ASCII tests hide it | [Walking a `String`](14_Strings/walking_a_string/README.md) |
| 5 | `word_count(s)` — `"  multiple   spaces  "` is 2 | The exact `split(' ')` vs `split_whitespace()` split that page is about | [Walking a `String`](14_Strings/walking_a_string/README.md) |
| 6 | `is_palindrome(s)` ignoring case and punctuation | `filter`, `to_lowercase`, `is_alphanumeric`; the ASCII-vs-Unicode case decision | [Comparing and sorting text](14_Strings/comparing_strings/README.md) |
| 7 | `replace_all(s, from, to)` | `find` + byte offsets + `push_str` — **once the "without `replace()`" constraint is added** | [Replacing part of a string](14_Strings/replacing_in_a_string/README.md) |

#### Upper intermediate

| # | Kata | What it exercises | Lands on |
|---|---|---|---|
| 8 | `to_camel_case` — snake, kebab, already-camel | Same exercise as [Exercise 6](#the-eight-exercises) of the progression — **merge them, do not write both** | [Building a `String`](14_Strings/building_a_string/README.md) |
| 9 | `compress_string` — run-length, return original if longer | Building with `push`/`push_str`, and a length comparison that is in **bytes** | [Building a `String`](14_Strings/building_a_string/README.md) |
| 10 | `are_anagrams(s1, s2)` | Counting chars into a map; `"debit card"` / `"bad credit"` counts the space too | [Walking a `String`](14_Strings/walking_a_string/README.md) |

#### Advanced

| # | Kata | What it exercises | Lands on |
|---|---|---|---|
| 11 | `longest_common_prefix(&[&str])` | Slices of slices, and the `&[]` empty case | [Searching without splitting](14_Strings/searching_a_string/README.md) |
| 12 | `permutations(s)` — unique, `""` yields one | Recursion producing `Vec<String>`; dedup | [Building a `String`](14_Strings/building_a_string/README.md) |
| 13 | `is_match(s, pattern)` — `.` and `*` | DP / recursion — **see finding 4** | undecided |
| 14 | `word_ladder_length(...)` | BFS — **see finding 4** | undecided |

#### Expert

| # | Kata | What it exercises | Lands on |
|---|---|---|---|
| 15 | `is_interleave(s1, s2, s3)` | DP — **see finding 4** | undecided |
| 16 | `longest_palindrome(s) -> &str` | **Returning a slice borrowed from the input** — lifetime elision, and the best kata in the set for this library | [String slices](14_Strings/string_slices/README.md) · [How to learn lifetimes](18_Ownership/how_to_learn_lifetimes/README.md) |
| 17 | `tokenize(input, &[char])` with quoted tokens | Near-duplicate of [Exercise 5](#the-eight-exercises), the CSV parser — **merge, or make one the multi-delimiter variant of the other** | [Walking a `String`](14_Strings/walking_a_string/README.md) |
| 18 | `count_characters(s)` | See finding 3 — the goal and the tests disagree | [Four lengths](14_Strings/four_lengths/README.md) |
| 19 | `justify_text(words, width)` | Padding and width arithmetic; the natural door to the format mini-language's `{:<}` / `{:^}` / `{:width$}` | [The format mini-language](14_Strings/the_format_language/README.md) |
| 20 | `edit_distance(s1, s2)` | DP — **see finding 4**; note the distance is over `char`s, not bytes | undecided |

### What the three backlogs add up to

- The **vocabulary** list is mostly a [GLOSSARY.md](GLOSSARY.md) sweep — few new pages.
- The **progression** is mostly an ordering decision — one page, or one table inside [STRINGS.md](STRINGS.md).
- The **katas** are the real build: after merging the duplicates (8 with Exercise 6, 17 with Exercise 5) and settling the four algorithm katas, roughly **fourteen to eighteen new programs**, each with `#[test]` functions, a `main()`, and a recorded `.out`.

Take them in Adam's order. Katas 2–10 are cheap and each one lands on a lesson that already exists, so the early ones cost a program and no prose.
