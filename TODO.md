# What to cover next — the strings vocabulary

**Level:** reference · the backlog

**One line:** 198 terms Adam wants covered somewhere in this library, in 15 groups — the vocabulary a person needs to read Rust string code without stopping, listed here so that "do we have a page for X?" has an answer that is checked rather than remembered.

## The job, for Thursday 2026-09-10

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

### Core types (15)

- [ ] `str`
- [ ] `String`
- [ ] `&str`
- [ ] `&String`
- [ ] `Box<str>`
- [ ] `Cow<str>`
- [ ] `OsStr`
- [ ] `OsString`
- [ ] `CStr`
- [ ] `CString`
- [ ] `Path`
- [ ] `PathBuf`
- [ ] `char`
- [ ] `u8`
- [ ] bytes


### Encodings (13)

- [ ] UTF-8
- [ ] UTF-16
- [ ] UTF-32
- [ ] ASCII
- [ ] WTF-8
- [ ] Unicode scalar value
- [ ] Unicode code point
- [ ] surrogate pairs
- [ ] byte order mark (BOM)
- [ ] little-endian
- [ ] big-endian
- [ ] locale encoding
- [ ] platform encoding


### Conversions (14)

- [ ] `from_utf8`
- [ ] `from_utf8_lossy`
- [ ] `to_string_lossy`
- [ ] `as_bytes`
- [ ] `as_os_str`
- [ ] `to_str`
- [ ] `to_owned`
- [ ] `into_owned`
- [ ] `to_string`
- [ ] `into_string`
- [ ] `from_utf8_unchecked`
- [ ] `as_ptr`
- [ ] `as_mut_ptr`
- [ ] `into_bytes`


### Operations (13)

- [ ] slicing
- [ ] indexing
- [ ] concatenation
- [ ] trimming
- [ ] splitting
- [ ] pattern matching
- [ ] substring search
- [ ] char iteration
- [ ] byte iteration
- [ ] grapheme clusters
- [ ] replacement
- [ ] case conversion
- [ ] normalization


### Traits a string type implements (15)

- [ ] `Display`
- [ ] `Debug`
- [ ] `ToString`
- [ ] `FromStr`
- [ ] `Deref<Target=str>`
- [ ] `AsRef<str>`
- [ ] `Borrow<str>`
- [ ] `Into<String>`
- [ ] `From<String>`
- [ ] `PartialEq`
- [ ] `Eq`
- [ ] `Ord`
- [ ] `Hash`
- [ ] `Clone`
- [ ] `Copy` (for `&str`)


### Memory and layout (11)

- [ ] heap allocation
- [ ] stack allocation
- [ ] capacity
- [ ] length
- [ ] reallocation
- [ ] fat pointer
- [ ] thin pointer
- [ ] string interning
- [ ] small-string optimization
- [ ] null-terminated
- [ ] non-null-terminated


### Raw and FFI (10)

- [ ] `std::ffi`
- [ ] `std::os::raw`
- [ ] `c_char`
- [ ] NUL byte
- [ ] pointer casting
- [ ] `transmute`
- [ ] manual UTF-8 validation
- [ ] `unsafe`
- [ ] `extern "C"`
- [ ] zero-copy


### Iterators and views (14)

- [ ] `Chars`
- [ ] `CharIndices`
- [ ] `Bytes`
- [ ] `Lines`
- [ ] `Split`
- [ ] `SplitWhitespace`
- [ ] `RSplit`
- [ ] `MatchIndices`
- [ ] `Matches`
- [ ] `RMatchIndices`
- [ ] `EncodeUtf16`
- [ ] `EscapeDebug`
- [ ] `EscapeDefault`
- [ ] `EscapeUnicode`


### Crates and ecosystem (15)

- [ ] `unicode-segmentation`
- [ ] `unicode-normalization`
- [ ] `encoding_rs`
- [ ] `icu`
- [ ] `bstr`
- [ ] `byteorder`
- [ ] `widestring`
- [ ] `utf16string`
- [ ] `compact_str`
- [ ] `smartstring`
- [ ] `smallstr`
- [ ] `tinystr`
- [ ] `arraystring`
- [ ] `flexstr`
- [ ] `cow-utils`


### Unicode concepts (12)

- [ ] grapheme
- [ ] code unit
- [ ] code point
- [ ] scalar value
- [ ] combining character
- [ ] normalization forms (NFC, NFD, NFKC, NFKD)
- [ ] case folding
- [ ] canonical equivalence
- [ ] compatibility equivalence
- [ ] invisible characters
- [ ] zero-width joiner
- [ ] variation selector


### Error types (6)

- [ ] `Utf8Error`
- [ ] `FromUtf8Error`
- [ ] `FromUtf16Error`
- [ ] `ParseError`
- [ ] `TryFromIntError`
- [ ] `Infallible`


### Common methods (33)

- [ ] `len()`
- [ ] `is_empty()`
- [ ] `contains()`
- [ ] `starts_with()`
- [ ] `ends_with()`
- [ ] `find()`
- [ ] `rfind()`
- [ ] `replace()`
- [ ] `replacen()`
- [ ] `to_lowercase()`
- [ ] `to_uppercase()`
- [ ] `repeat()`
- [ ] `push()`
- [ ] `push_str()`
- [ ] `pop()`
- [ ] `insert()`
- [ ] `insert_str()`
- [ ] `remove()`
- [ ] `truncate()`
- [ ] `clear()`
- [ ] `drain()`
- [ ] `split_off()`
- [ ] `retain()`
- [ ] `reserve()`
- [ ] `shrink_to_fit()`
- [ ] `with_capacity()`
- [ ] `from_raw_parts()`
- [ ] `make_ascii_lowercase()`
- [ ] `make_ascii_uppercase()`
- [ ] `is_ascii()`
- [ ] `eq_ignore_ascii_case()`
- [ ] `escape_default()`
- [ ] `parse::<T>()`


### Pointer and safety (11)

- [ ] dangling pointer
- [ ] null pointer
- [ ] pointer alignment
- [ ] memory leak
- [ ] buffer overflow
- [ ] out-of-bounds
- [ ] invalid UTF-8
- [ ] data race (via `unsafe`)
- [ ] `MaybeUninit`
- [ ] `NonNull`
- [ ] `UnsafeCell`


### Patterns and matching (9)

- [ ] string literal
- [ ] raw string (`r"..."`, `r#"..."#`)
- [ ] byte string (`b"..."`)
- [ ] raw byte string (`br"..."`)
- [ ] multiline string
- [ ] escape sequences (`\n`, `\t`, `\u{...}`, `\x..`)
- [ ] char literal
- [ ] the `Pattern` trait
- [ ] `Sealed`


### Smart pointers and wrappers (7)

- [ ] `Rc<str>`
- [ ] `Arc<str>`
- [ ] `Mutex<String>`
- [ ] `RwLock<String>`
- [ ] `RefCell<String>`
- [ ] `Cell<&str>`
- [ ] `Pin<Box<str>>`

