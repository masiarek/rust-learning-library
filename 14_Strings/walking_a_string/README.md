# Walking a `String`

**Level:** 101 → 201 · working knowledge

**One line:** Three item types — `u8` from `bytes()`, `char` from `chars()`, `&str` from the whole split family — and every split is *the gaps between the matches*, which is where all those surprising empty strings come from.

| iterator | item | on `"bête noir  d'Arrrgh "` |
|---|---|---|
| [`.bytes()` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.bytes) | `u8` | 21 items |
| [`.chars()` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.chars) | `char` | 20 items |
| [`.char_indices()` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices) | `(usize, char)` | 20 items, offsets 0, 1, 3, 4… |
| [`.split(' ')` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split) | `&str` | 5 — `["bête", "noir", "", "d'Arrrgh", ""]` |
| [`.split_terminator(' ')` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split_terminator) | `&str` | 4 — the trailing empty is dropped |
| [`.split_whitespace()` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) | `&str` | 3 — runs collapse, ends trimmed |
| [`.splitn(3, ' ')` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.splitn) | `&str` | 3 — the remainder stays whole |
| [`.matches("rr")` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.matches) | `&str` | 1 |
| `.split("rr")` | `&str` | 2 |

---

## Two rulers over the same text

```rust
let slice = "bête noir  d'Arrrgh ";
slice.bytes().count()    // 21
slice.chars().count()    // 20
```

`bytes()` yields the UTF-8 bytes as `u8`; `chars()` yields Unicode scalars as `char`. `'ê'` is one item in the second and two in the first — [Meet the `char`](../meet_the_char/README.md) is why.

## `char_indices()` is not `chars().enumerate()`

```rust
slice.char_indices()      // (0,'b') (1,'ê') (3,'t') (4,'e') …
slice.chars().enumerate() // (0,'b') (1,'ê') (2,'t') (3,'e') …
```

They agree until the first multi-byte character and then never again. `char_indices` gives **byte offsets** — numbers you can slice with, guaranteed to be char boundaries. `enumerate` gives ordinals, which are not offsets into anything. Slicing with an ordinal is the bug: it works on ASCII, and panics or silently cuts the wrong place on the first accented name.

## Splits are the gaps between matches

```rust
slice.matches("rr")   // ["rr"]                        — 1 match
slice.split("rr")     // ["bête noir  d'A", "rgh "]    — 2 pieces
```

*n* matches always yield *n+1* pieces. Every empty string in the table above falls straight out of that: two adjacent spaces have nothing between them, and a separator at the very end has nothing after it. Nothing is being helpful or unhelpful — the count is forced.

That is also the difference nobody remembers:

```rust
slice.split(' ')            // ["bête", "noir", "", "d'Arrrgh", ""]
slice.split_whitespace()    // ["bête", "noir", "d'Arrrgh"]
```

`split(' ')` is mechanical — it reports the gaps. `split_whitespace()` is *editorial* — runs of whitespace collapse and the ends are trimmed. Use the first when position matters (a CSV column, a fixed layout), the second for prose and hand-typed input. Reaching for the second on delimited data silently shortens the row, so column 3 becomes column 2 and nothing errors; the kata below makes that happen.

`split_terminator` is the middle ground: it drops only a *trailing* empty piece, which is what you want for text that ends in its own separator, like a file ending in a newline.

## The `n` variants stop early

```rust
slice.splitn(3, ' ')    // ["bête", "noir", " d'Arrrgh "]
slice.rsplitn(3, ' ')   // ["", "d'Arrrgh", "bête noir "]
```

After *n−1* splits the remainder comes back whole, separators and all. `rsplit*` does the same from the right, and yields its pieces in right-to-left order — the reversed order surprises people more than the direction does.

## The pattern is not just a character

Anything implementing `Pattern` works, which is three useful shapes:

```rust
slice.split('r')                    // a char
slice.split("rr")                   // a &str
slice.split(char::is_whitespace)    // a closure or fn(char) -> bool
slice.split(|c: char| c == '\'')    // an inline closure
```

## The two you will actually reach for

```rust
for line in config.lines() {
    if let Some((key, value)) = line.split_once(" = ") {
        …
    }
}
```

`lines()` splits on `\n` and handles a trailing `\r`, and drops the final empty piece — so a file ending in a newline gives you no phantom last line. `split_once` takes the **first** separator and returns the rest whole, which is what you want for `key = value` where the value may itself contain the separator. Reaching for `split(" = ")` there loses everything after the second one.

And `trim()` — plus `trim_start`, `trim_end`, and the `trim_matches` family — returns a **slice**, not a new `String`. Nothing is copied.

## If you are coming from another language

**Python.** The distinction that catches everyone here is one you already survived there — `.split()` and `.split(' ')` are different functions in Python too, for exactly the same reason.

| Python | | Rust |
|---|---|---|
| `s.split()` | collapses runs, trims ends | `s.split_whitespace()` |
| `s.split(' ')` | keeps empties | `s.split(' ')` |
| `s.split(sep, 2)` | maxsplit | `s.splitn(3, sep)` — note: **pieces**, not splits |
| `s.rsplit(sep, 1)` | from the right | `s.rsplitn(2, sep)` |
| `s.partition(sep)` | head, sep, tail | `s.split_once(sep)` → `Option<(&str, &str)>` |
| `s.splitlines()` | by line | `s.lines()` |
| `enumerate(s)` | index *is* character position | `s.char_indices()` — byte offsets, not ordinals |
| `for c in s` | yields 1-char strings | `s.chars()` yields `char`; `s.bytes()` yields `u8` |

Two real differences. Python's `maxsplit` counts *splits*, Rust's `splitn` counts *pieces* — `s.split(sep, 2)` and `s.splitn(3, sep)` are the same call. And iterating a Python string gives you one-character strings, so `enumerate` positions are indexable; Rust separates the two rulers and makes you pick, which is why `char_indices` exists at all.

**ABAP.** `SPLIT … AT … INTO TABLE` is the whole family in one statement, and it keeps empty fields — so it behaves like `split(sep)`, not `split_whitespace()`.

| ABAP | | Rust |
|---|---|---|
| `SPLIT s AT ',' INTO TABLE lt.` | every field, empties kept | `s.split(',')` |
| `SPLIT s AT ',' INTO a b.` | first fields, rest into the last | `s.splitn(2, ',')` |
| `CONDENSE s.` | collapse and trim | `s.split_whitespace().collect::<Vec<_>>().join(" ")` |
| `SHIFT s LEFT DELETING LEADING space.` | trim the front | `s.trim_start()` |
| `FIND ALL OCCURRENCES OF …` | the matches | `s.matches(…)` |

What changes: `SPLIT … INTO TABLE` materialises an internal table — every field is copied. Rust's split returns an **iterator of slices** into the original, so nothing is copied until you `.collect()`, and often you never do. The other change is that a two-target `SPLIT … INTO a b` quietly puts the remainder in `b`, which is `splitn(2, …)` — the behaviour is the same, but Rust makes you write the number.

---

## Practice

**An empty field is data.** Parse ballot rows like `"5,2,0"` and `"5,,0"` into `Vec<Option<u8>>`, where a missing score is `None` (an abstention) and not `Some(0)` (a real score of zero).

Do it with `split(',')`, then try the same rows with `split_whitespace()` and record what happens to `"5,,0"` and to `",,"`. Then repeat on space-separated data — `"5 2 0"` versus `"5  0"` — and say which of the two functions can tell "no score here" from "no gap here", and what a row parsed by the wrong one does to column 3.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:walking_a_string_kata -->
*[`walking_a_string_kata.rs`](examples/walking_a_string_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: an empty field is data, and split_whitespace eats it.
//!
//!   rustc --edition 2024 walking_a_string_kata.rs -o /tmp/wsk && /tmp/wsk

/// A ballot row: three scores, and a missing score is a real, different thing.
fn parse_row(line: &str) -> Vec<Option<u8>> {
    line.split(',')
        .map(|f| f.trim())
        .map(|f| if f.is_empty() { None } else { f.parse().ok() })
        .collect()
}

fn main() {
    let rows = ["5,2,0", "5, ,0", "5,,0", ",,", "5, 2 , 0 "];

    println!("split(',') keeps every field, including the empty ones:");
    for r in rows {
        println!("   {:<10} -> {:?}", r, r.split(',').collect::<Vec<_>>());
    }

    println!("\nsplit_whitespace() does not — it cannot even see the commas:");
    for r in rows {
        println!("   {:<10} -> {:?}", r, r.split_whitespace().collect::<Vec<_>>());
    }

    println!("\nAnd on whitespace-separated data it still drops the gap:");
    let spaced = ["5 2 0", "5  0", "5 2 "];
    for r in spaced {
        println!("   {:<8} split(' ')          {:?}", r, r.split(' ').collect::<Vec<_>>());
        println!("   {:<8} split_whitespace()  {:?}", "", r.split_whitespace().collect::<Vec<_>>());
    }

    println!("\nParsed as ballots — None is an abstention, not a zero:");
    for r in rows {
        let parsed = parse_row(r);
        let counted = parsed.iter().filter(|s| s.is_some()).count();
        println!("   {:<10} -> {:?}   ({counted} of {} scored)", r, parsed, parsed.len());
    }

    println!("\nThe rule:");
    println!("   split(sep)          n separators -> n+1 fields, empties included.");
    println!("                       Use it when position matters: CSV, fixed columns.");
    println!("   split_whitespace()  runs of whitespace collapse, ends are trimmed.");
    println!("                       Use it for prose and hand-typed input.");
    println!("   Reaching for split_whitespace() on delimited data silently shortens");
    println!("   the row, so column 3 becomes column 2 and nothing errors.");
}
```
<!-- /source -->

<!-- output:walking_a_string_kata -->
*Verified output of [`walking_a_string_kata.rs`](examples/walking_a_string_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
split(',') keeps every field, including the empty ones:
   5,2,0      -> ["5", "2", "0"]
   5, ,0      -> ["5", " ", "0"]
   5,,0       -> ["5", "", "0"]
   ,,         -> ["", "", ""]
   5, 2 , 0   -> ["5", " 2 ", " 0 "]

split_whitespace() does not — it cannot even see the commas:
   5,2,0      -> ["5,2,0"]
   5, ,0      -> ["5,", ",0"]
   5,,0       -> ["5,,0"]
   ,,         -> [",,"]
   5, 2 , 0   -> ["5,", "2", ",", "0"]

And on whitespace-separated data it still drops the gap:
   5 2 0    split(' ')          ["5", "2", "0"]
            split_whitespace()  ["5", "2", "0"]
   5  0     split(' ')          ["5", "", "0"]
            split_whitespace()  ["5", "0"]
   5 2      split(' ')          ["5", "2", ""]
            split_whitespace()  ["5", "2"]

Parsed as ballots — None is an abstention, not a zero:
   5,2,0      -> [Some(5), Some(2), Some(0)]   (3 of 3 scored)
   5, ,0      -> [Some(5), None, Some(0)]   (2 of 3 scored)
   5,,0       -> [Some(5), None, Some(0)]   (2 of 3 scored)
   ,,         -> [None, None, None]   (0 of 3 scored)
   5, 2 , 0   -> [Some(5), Some(2), Some(0)]   (3 of 3 scored)

The rule:
   split(sep)          n separators -> n+1 fields, empties included.
                       Use it when position matters: CSV, fixed columns.
   split_whitespace()  runs of whitespace collapse, ends are trimmed.
                       Use it for prose and hand-typed input.
   Reaching for split_whitespace() on delimited data silently shortens
   the row, so column 3 becomes column 2 and nothing errors.
```
<!-- /output -->

</details>

---

**Two rulers over one string.** Report both lengths of `"Hello 🦀!"` — `len()` and `chars().count()` — and say which one the heap cares about. Then collect every second character into a new `String`, and count how many raw bytes are above 127.

Now print the string twice: once with `.chars().enumerate()`, once with `.char_indices()`. Line the two indices up side by side, say exactly where they part company, and say which of the two numbers you are allowed to hand to `&s[..i]`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:two_rulers_kata -->
*[`two_rulers_kata.rs`](examples/two_rulers_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: two rulers over one string — bytes and chars, and the two
//! indices that look alike until a crab turns up.
//!
//!   rustc --edition 2024 two_rulers_kata.rs -o /tmp/trk && /tmp/trk

/// Both answers to "how long is it", returned together so neither can be
/// mistaken for the other.
fn lengths(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}

/// Every second character, starting with the first.
fn alternate(s: &str) -> String {
    s.chars().step_by(2).collect()
}

/// How many raw bytes are outside ASCII — i.e. part of a multibyte character.
fn non_ascii_bytes(s: &str) -> usize {
    s.bytes().filter(|&b| b > 127).count()
}

fn main() {
    let s = "Hello 🦀!";

    println!("1. Two lengths, one string");
    let (bytes, chars) = lengths(s);
    println!("   {s:?}");
    println!("   len()          {bytes:>2}   <- bytes, what the heap holds");
    println!("   chars().count() {chars:>1}   <- Unicode scalars, what you meant");
    println!("   The gap is one crab: 🦀 is 4 bytes and 1 char.");

    println!("\n2. Every second character");
    println!("   alternate({s:?})      = {:?}", alternate(s));
    println!("   alternate(\"abcdefgh\")     = {:?}", alternate("abcdefgh"));
    println!("   step_by(2) counts *characters*, so the crab is never cut in half.");

    println!("\n3. Bytes above 127");
    for t in [s, "plain ascii", "café", "こんにちは"] {
        println!("   {:>2} of {:>2} bytes are non-ASCII   {t:?}", non_ascii_bytes(t), t.len());
    }
    println!("   A byte > 127 is never a whole character on its own: it is a lead");
    println!("   byte or a continuation byte of a 2-, 3- or 4-byte sequence.");

    println!("\n4. chars().enumerate() — the counter runs 0,1,2,…");
    for (i, c) in s.chars().enumerate() {
        println!("   {i}  {c:?}");
    }

    println!("\n5. char_indices() — the index is a BYTE offset");
    for (i, c) in s.char_indices() {
        println!("   {i:>2}  {c:?}  ({} byte{})", c.len_utf8(), if c.len_utf8() == 1 { "" } else { "s" });
    }

    println!("\nThe two side by side:");
    println!("   {:>9}  {:>12}  char", "enumerate", "char_indices");
    for ((n, c), (b, _)) in s.chars().enumerate().zip(s.char_indices()) {
        println!("   {n:>9}  {b:>12}  {c:?}");
    }
    println!("   They agree until the first multibyte character and never again.");
    println!("   Only the char_indices number can be handed to &s[..i]; the");
    println!("   enumerate number is a position in a sequence, not an offset.");
}
```
<!-- /source -->

<!-- output:two_rulers_kata -->
*Verified output of [`two_rulers_kata.rs`](examples/two_rulers_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Two lengths, one string
   "Hello 🦀!"
   len()          11   <- bytes, what the heap holds
   chars().count() 8   <- Unicode scalars, what you meant
   The gap is one crab: 🦀 is 4 bytes and 1 char.

2. Every second character
   alternate("Hello 🦀!")      = "Hlo🦀"
   alternate("abcdefgh")     = "aceg"
   step_by(2) counts *characters*, so the crab is never cut in half.

3. Bytes above 127
    4 of 11 bytes are non-ASCII   "Hello 🦀!"
    0 of 11 bytes are non-ASCII   "plain ascii"
    2 of  5 bytes are non-ASCII   "café"
   15 of 15 bytes are non-ASCII   "こんにちは"
   A byte > 127 is never a whole character on its own: it is a lead
   byte or a continuation byte of a 2-, 3- or 4-byte sequence.

4. chars().enumerate() — the counter runs 0,1,2,…
   0  'H'
   1  'e'
   2  'l'
   3  'l'
   4  'o'
   5  ' '
   6  '🦀'
   7  '!'

5. char_indices() — the index is a BYTE offset
    0  'H'  (1 byte)
    1  'e'  (1 byte)
    2  'l'  (1 byte)
    3  'l'  (1 byte)
    4  'o'  (1 byte)
    5  ' '  (1 byte)
    6  '🦀'  (4 bytes)
   10  '!'  (1 byte)

The two side by side:
   enumerate  char_indices  char
           0             0  'H'
           1             1  'e'
           2             2  'l'
           3             3  'l'
           4             4  'o'
           5             5  ' '
           6             6  '🦀'
           7            10  '!'
   They agree until the first multibyte character and never again.
   Only the char_indices number can be handed to &s[..i]; the
   enumerate number is a position in a sequence, not an offset.
```
<!-- /output -->

</details>

---

**Find it without a regex engine.** Six searches over plain text: a palindrome checker that survives punctuation, case and multibyte letters; a word count with `split_whitespace()`; a CSV row split into a `Vec<&str>`; the first and last byte offset of a substring, with `find` and `rfind`; a `starts_with` / `ends_with` check for something URL-shaped; and every occurrence of a character collected with `matches()`.

Then the two jobs people reach for `regex` for: pull the email addresses out of a paragraph, and censor a word list with `****`. Write both with `std` alone — then say what your versions get wrong that a regex would not. That answer is the exercise; the code is just how you earn it.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:searching_text_kata -->
*[`searching_text_kata.rs`](examples/searching_text_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: finding things in text with std alone — palindromes, words,
//! fields, offsets, prefixes, every match, and the two jobs that finally do
//! want a regex engine.
//!
//!   rustc --edition 2024 searching_text_kata.rs -o /tmp/stk && /tmp/stk

/// Reads the same forwards and backwards, ignoring case and punctuation.
/// Compares `char`s, so multibyte text works — but see the note in main():
/// a `char` is still not what a reader calls a character.
fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

/// Words, by the only definition std offers: runs between whitespace.
fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// One CSV row into its fields. Every field is a view into `row` — no allocation.
fn fields(row: &str) -> Vec<&str> {
    row.split(',').collect()
}

/// Does it look like a URL? Two prefixes and a non-empty rest.
fn looks_like_url(s: &str) -> bool {
    (s.starts_with("http://") || s.starts_with("https://")) && !s.ends_with('/')
}

/// A crude address finder: split on whitespace, strip trailing punctuation,
/// keep what has one `@` with something either side and a dot after it.
fn find_emails(text: &str) -> Vec<&str> {
    text.split_ascii_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| {
            let mut parts = w.split('@');
            match (parts.next(), parts.next(), parts.next()) {
                (Some(user), Some(host), None) => {
                    !user.is_empty() && host.contains('.') && !host.starts_with('.')
                }
                _ => false,
            }
        })
        .collect()
}

/// Replace whole words, case-insensitively, with `****`. Whitespace runs are
/// normalised to single spaces — which is the first thing a regex would not do.
fn censor(text: &str, banned: &[&str]) -> String {
    text.split_whitespace()
        .map(|word| {
            let bare = word.trim_matches(|c: char| !c.is_alphanumeric());
            if banned.iter().any(|b| b.eq_ignore_ascii_case(bare)) {
                word.replace(bare, "****")
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    println!("1. Palindromes");
    for s in ["racecar", "A man, a plan, a canal: Panama", "Ada", "ala", "kajak"] {
        println!("   {:<32} {}", format!("{s:?}"), is_palindrome(s));
    }
    println!("   to_lowercase() on a char returns an ITERATOR, not a char — 'İ'");
    println!("   lowercases to two chars — which is why flat_map is here.");

    println!("\n2. Counting words");
    let para = "Score every candidate.\n  Then the top two\thave an automatic runoff.";
    println!("   {} words in {} bytes", word_count(para), para.len());
    println!("   split_whitespace() splits on RUNS of any whitespace, so the double");
    println!("   space, the newline and the tab each separate one pair of words:");
    println!("   {:?}", para.split_whitespace().collect::<Vec<_>>());
    println!("   split(' ') only knows the space character, so it keeps the newline");
    println!("   and the tab inside words and reports the gap as an empty field:");
    println!("   {:?}", para.split(' ').collect::<Vec<_>>());
    println!("   On \"a  b\" that is {:?} against {:?}.",
        "a  b".split(' ').collect::<Vec<_>>(),
        "a  b".split_whitespace().collect::<Vec<_>>());

    println!("\n3. CSV fields");
    let row = "Ada,5,2,,0";
    println!("   {row:?} -> {:?}", fields(row));
    println!("   The empty field survives: split() reports the gaps between matches,");
    println!("   so \"a,,b\" is three fields and the middle one is \"\".");

    println!("\n4. Where is it? find and rfind");
    let text = "runoff between the top two, then a second runoff if tied";
    println!("   {text:?}");
    println!("   find(\"runoff\")   = {:?}", text.find("runoff"));
    println!("   rfind(\"runoff\")  = {:?}", text.rfind("runoff"));
    println!("   find(\"instant\")  = {:?}   <- Option, not -1", text.find("instant"));
    if let (Some(a), Some(b)) = (text.find("runoff"), text.rfind("runoff")) {
        println!("   both are BYTE offsets, so they slice directly: {:?} … {:?}",
            &text[a..a + 6], &text[b..]);
    }

    println!("\n5. Does it look like a URL?");
    for s in [
        "https://masiarek.github.io/rust-learning-library",
        "http://example.com",
        "ftp://example.com",
        "example.com",
        "https://example.com/",
    ] {
        println!("   {:<52} {}", s, looks_like_url(s));
    }

    println!("\n6. Every match, not just the first");
    let ballots = "5,4,0 5,5,1 0,0,5 5,2,3";
    let fives: Vec<&str> = ballots.matches('5').collect();
    println!("   {ballots:?}");
    println!("   matches('5')       {:?}  ({} of them)", fives, fives.len());
    println!("   match_indices('5') {:?}",
        ballots.match_indices('5').map(|(i, _)| i).collect::<Vec<_>>());
    println!("   matches gives you the text, match_indices gives you where — and");
    println!("   overlapping matches are not reported: \"aaa\".matches(\"aa\") is {}",
        "aaa".matches("aa").count());

    println!("\n7. Addresses, without a regex engine");
    let inbox = "write to ada@example.com or (ben@sub.example.org). \
                 not-an-email@, nor @example.com, nor plain.text";
    println!("   {:?}", find_emails(inbox));
    println!("   Every one of those is a &str borrowed from `inbox` — no allocation.");

    println!("\n8. Censoring, without a regex engine");
    let post = "The Spoiler effect is a spoiler, and SPOILER talk is everywhere.";
    println!("   before  {post:?}");
    println!("   after   {:?}", censor(post, &["spoiler"]));
    println!("   Both of these are where a regex earns its place: word boundaries,");
    println!("   case folding and capture groups in one pass, instead of a hand-rolled");
    println!("   scanner per rule. `regex` is a crate, not std — deliberately, because");
    println!("   a regex engine is a compiler and std does not ship one.");
}
```
<!-- /source -->

<!-- output:searching_text_kata -->
*Verified output of [`searching_text_kata.rs`](examples/searching_text_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Palindromes
   "racecar"                        true
   "A man, a plan, a canal: Panama" true
   "Ada"                            true
   "ala"                            true
   "kajak"                          true
   to_lowercase() on a char returns an ITERATOR, not a char — 'İ'
   lowercases to two chars — which is why flat_map is here.

2. Counting words
   11 words in 67 bytes
   split_whitespace() splits on RUNS of any whitespace, so the double
   space, the newline and the tab each separate one pair of words:
   ["Score", "every", "candidate.", "Then", "the", "top", "two", "have", "an", "automatic", "runoff."]
   split(' ') only knows the space character, so it keeps the newline
   and the tab inside words and reports the gap as an empty field:
   ["Score", "every", "candidate.\n", "", "Then", "the", "top", "two\thave", "an", "automatic", "runoff."]
   On "a  b" that is ["a", "", "b"] against ["a", "b"].

3. CSV fields
   "Ada,5,2,,0" -> ["Ada", "5", "2", "", "0"]
   The empty field survives: split() reports the gaps between matches,
   so "a,,b" is three fields and the middle one is "".

4. Where is it? find and rfind
   "runoff between the top two, then a second runoff if tied"
   find("runoff")   = Some(0)
   rfind("runoff")  = Some(42)
   find("instant")  = None   <- Option, not -1
   both are BYTE offsets, so they slice directly: "runoff" … "runoff if tied"

5. Does it look like a URL?
   https://masiarek.github.io/rust-learning-library     true
   http://example.com                                   true
   ftp://example.com                                    false
   example.com                                          false
   https://example.com/                                 false

6. Every match, not just the first
   "5,4,0 5,5,1 0,0,5 5,2,3"
   matches('5')       ["5", "5", "5", "5", "5"]  (5 of them)
   match_indices('5') [0, 6, 8, 16, 18]
   matches gives you the text, match_indices gives you where — and
   overlapping matches are not reported: "aaa".matches("aa") is 1

7. Addresses, without a regex engine
   ["ada@example.com", "ben@sub.example.org"]
   Every one of those is a &str borrowed from `inbox` — no allocation.

8. Censoring, without a regex engine
   before  "The Spoiler effect is a spoiler, and SPOILER talk is everywhere."
   after   "The **** effect is a ****, and **** talk is everywhere."
   Both of these are where a regex earns its place: word boundaries,
   case folding and capture groups in one pass, instead of a hand-rolled
   scanner per rule. `regex` is a crate, not std — deliberately, because
   a regex engine is a compiler and std does not ship one.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:walking_a_string -->
*Verified output of [`walking_a_string.rs`](examples/walking_a_string.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The slice: "bête noir  d'Arrrgh "
   21 bytes, 20 chars

1. Item type u8 — .bytes()
   first 6: [98, 195, 170, 116, 101, 32]   (21 in total)

2. Item type char — .chars()
   first 6: ['b', 'ê', 't', 'e', ' ', 'n']   (20 in total)
   'ê' is one char here and two bytes above. Same text, two rulers.

3. .char_indices() is not .chars().enumerate()
   char_indices():      (0,'b') (1,'ê') (3,'t') (4,'e') (5,' ') (6,'n') 
   chars().enumerate(): (0,'b') (1,'ê') (2,'t') (3,'e') (4,' ') (5,'n') 
   They diverge at 't'. char_indices gives BYTE OFFSETS you can slice with;
   enumerate gives ordinals you cannot.

4. Item type &str — the split family
   .split(' ')              5 -> ["bête", "noir", "", "d'Arrrgh", ""]
   .split_terminator(' ')   4 -> ["bête", "noir", "", "d'Arrrgh"]
   .rsplit(' ')             5 -> ["", "d'Arrrgh", "", "noir", "bête"]
   .split_whitespace()      3 -> ["bête", "noir", "d'Arrrgh"]
   .splitn(3, ' ')          3 -> ["bête", "noir", " d'Arrrgh "]
   .rsplitn(3, ' ')         3 -> ["", "d'Arrrgh", "bête noir "]

5. Splits are the gaps between matches
   .matches("rr")           1 -> ["rr"]
   .split("rr")             2 -> ["bête noir  d'A", "rgh "]
   One match, two pieces. n matches always give n+1 pieces — which is
   where the empty strings above come from: two adjacent spaces have
   nothing between them, and so does a separator at the very end.

6. The pattern can be a char, a &str, or a closure
   .split('r')              5 -> ["bête noi", "  d'A", "", "", "gh "]
   .split("rr")             2 -> ["bête noir  d'A", "rgh "]
   .split(char::is_whitespace) 5 -> ["bête", "noir", "", "d'Arrrgh", ""]
   .split(|c: char| c == '\'') 2 -> ["bête noir  d", "Arrrgh "]

7. The two you will actually reach for
   config = "name = Ada\nscore = 5\nnote = has = signs\n"
      split_once  name   -> "Ada"
      split_once  score  -> "5"
      split_once  note   -> "has = signs"
   split_once takes the FIRST separator and returns the rest whole, so
   the value containing ' = ' survives. splitn(2, …) does the same job.
   trim():  "  padded  " -> "padded"
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/walking_a_string/examples/walking_a_string.rs -o /tmp/ws && /tmp/ws
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [Meet the `char`](../meet_the_char/README.md) — why `bytes()` and `chars()` give different counts
- [String slices](../string_slices/README.md) — what those byte offsets are for, and how they panic
- [Inside a `Split`](../inside_a_split/README.md) — what a split *is* before you consume it, and why printing one shows a struct
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — the `u8` that `bytes()` yields
- [`Option` as a collection](../../17_Option_and_Result/option_as_collection/README.md) — the `Option<u8>` the kata builds, iterated
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — the three-door question for a collection, where the answer is different from a string's
- [`str::split` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split) · [`Pattern` ↗](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html) · [`str::split_once` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split_once)

## Po polsku

Po łańcuchu znaków nie chodzi się „po prostu” — najpierw trzeba wybrać miarkę, bo każdy z trzech iteratorów daje inny typ elementu. `bytes()` wydaje bajty jako `u8`, `chars()` wydaje znaki jako `char`, a cała rodzina `split*` wydaje wycinki `&str`. Na przykładzie z tej strony to 21 bajtów wobec 20 znaków dla tego samego tekstu, i nic tych dwóch liczb za ciebie nie pogodzi: `'ê'` jest jednym elementem w drugim iteratorze i dwoma w pierwszym.

Najostrzejsza pułapka to `char_indices()` kontra `chars().enumerate()`. Pierwsze daje **offsety bajtowe**, czyli liczby, którymi wolno ciąć (`&s[..i]`), i z definicji trafiające w granicę znaku; drugie daje zwykłe numery porządkowe 0, 1, 2, które nie są offsetem donikąd. Obie zgadzają się dokładnie do pierwszego znaku wielobajtowego — a w polskim tekście to zwykle znaczy „do drugiej litery”. Dla `"Łódź"` (7 bajtów, 4 znaki) `char_indices()` daje 0, 2, 4, 5, a `enumerate()` uparcie 0, 1, 2, 3. Ciąć wolno tylko tą pierwszą liczbą; druga na danych ASCII wygląda identycznie, więc błąd przechodzi przez testy i panikuje na pierwszym nazwisku z `ó`.

Cała reszta strony wynika z jednego zdania: **podział zwraca przerwy między dopasowaniami**, więc *n* dopasowań daje zawsze *n+1* kawałków. Stąd biorą się wszystkie zaskakujące puste łańcuchy — dwie spacje obok siebie nie mają nic pomiędzy, a separator na samym końcu nie ma nic po sobie. Nikt tu nie jest uprzejmy ani nieuprzejmy, liczba kawałków jest wymuszona arytmetyką. Praktycznie sprowadza się to do wyboru:

- `split(sep)` — mechaniczny: raportuje przerwy razem z pustymi. Bierz go, gdy liczy się **pozycja**: CSV, stałe kolumny, pole, które wolno zostawić puste.
- `split_whitespace()` — redakcyjny: ciągi białych znaków sklejają się w jeden, końce są przycięte. Do prozy i tekstu wpisywanego ręcznie.
- `split_terminator(sep)` — pośredni: usuwa tylko *końcowy* pusty kawałek, czyli dokładnie to, czego chcesz dla pliku kończącego się separatorem.
- `splitn(n, sep)` — uwaga na liczbę: `n` to **kawałki**, nie cięcia. Pythonowe `maxsplit=2` to tutaj `splitn(3, …)`.

Sięgnięcie po `split_whitespace()` przy danych rozdzielanych przecinkiem nie zgłasza żadnego błędu — po cichu skraca wiersz, kolumna trzecia staje się drugą i nikt się o tym nie dowie. W codziennej pracy i tak najczęściej używa się dwóch innych: `lines()` (radzi sobie z `\r` i nie dokleja widmowego pustego wiersza na końcu pliku) oraz `split_once(sep)`, który bierze **pierwszy** separator i oddaje resztę w całości — dlatego `note = has = signs` przeżywa parsowanie `klucz = wartość`, a `split(" = ")` gubi wszystko po drugim. Warto też pamiętać, że wzorcem może być `char`, `&str` albo domknięcie (`|c: char| c == '\''`), że cała ta rodzina zwraca **iteratory wycinków** oryginału — nic nie jest kopiowane, dopóki nie wywołasz `.collect()` — i że `trim()` również zwraca wycinek, a nie nowy `String`. Wyrażeń regularnych w `std` nie ma, i to świadomie — silnik wyrażeń regularnych jest w gruncie rzeczy kompilatorem, więc `regex` jest osobnym crate'em.

**Szukaj po polsku:** iterowanie po znakach łańcucha · dzielenie łańcucha znaków · bajty a znaki UTF-8 · `rust char_indices vs chars enumerate` · `rust split vs split_whitespace`
