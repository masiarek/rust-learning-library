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
