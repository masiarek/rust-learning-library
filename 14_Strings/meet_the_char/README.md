# Meet the `char`

**Level:** 101 → 201 · working knowledge

**One line:** A `char` is one Unicode scalar value — four bytes, always. Inside a `String` the same character is one to four UTF-8 bytes, which is why `.len()` is not "how many characters", `s[0]` refuses to compile, and "how long is this string" has three honest answers.

```rust
let c: char = 'a';    // single quotes: a char
let s: &str = "a";    // double quotes: a string that happens to hold one char
```

Different types, and the difference is not pedantry: `'a'` is a **decoded** character you can compare, classify and range over (`'0'..='9'`); `"a"` is **encoded** bytes. The same letter costs 4 bytes as a `char` value and 1 byte inside a string — decoded values are fixed-width so they can be worked on, encoded bytes are variable-width so they can be stored small.

---

## Three answers to "how long"

```rust
let name = "Zoë";
name.len()             // 4 — bytes
name.chars().count()   // 3 — chars (Unicode scalar values)
```

The `ë` costs two bytes in UTF-8, so byte count and char count part ways on the first accent. [Meet the byte](../../19_Numbers/meet_the_byte/README.md) already warned that `.len()` counts bytes; this page is the other half — what those bytes encode:

| answer | asks | costs |
|---|---|---|
| **bytes** | `.len()` | O(1) — it is the `len` field |
| **chars** | `.chars().count()` | O(n) — UTF-8 must be walked |
| **graphemes** | what a *reader* calls one character | a crate — std stops before this |

The third row is real: `"e\u{301}"` — an `e` plus a combining accent — is **two** chars that render as one `é`. The [verified output](#the-verified-output) shows the two spellings printing identically and comparing unequal. Normalizing them into one form is the [`unicode-normalization` ↗](https://crates.io/crates/unicode-normalization) crate's job; counting reader-characters is [`unicode-segmentation` ↗](https://crates.io/crates/unicode-segmentation)'s.

## Why `s[0]` does not compile

```rust
let first = name[0];
```

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
```

An index promises O(1), and the only O(1) thing in a `String` is a byte — but byte 0 might be the *middle third* of some character, and handing you a raw byte from text would be an answer to a question you did not ask. Rust refuses to guess which you meant, and offers each meaning under its own name:

```rust
name.bytes().nth(0)     // Some(90) — the byte, if you really mean bytes
name.chars().nth(0)     // Some('Z') — the char, at the honest O(n) price
name.get(0..1)          // Some("Z") — a subslice, refused as None off-boundary
```

Byte-range slicing (`&name[0..1]`) works too — but **panics** on a range that cuts a character in half. `get` is the same question with `None` instead of a crash, and `is_char_boundary(i)` is how you ask first. The output below shows `get(2..3)` refusing the middle of `ë`.

## Case is not one-to-one

```rust
let upper: String = 'ß'.to_uppercase().collect();   // "SS"
```

`to_uppercase` on a `char` returns an **iterator**, not a `char` — because German `ß` uppercases to two letters. A whole `&str` has `.to_uppercase() -> String`, which absorbs that quietly. Anything assuming "one char in, one char out" — a fixed-size buffer, an index math shortcut — is wrong in exactly the languages where you will not notice for months.

## If you are coming from another language

**Python.** The two languages disagree about what `len` counts, and it is the #1 cross-language string surprise in both directions.

| Python | | Rust |
|---|---|---|
| `len("Zoë")` → 3 | counts code points | `.len()` → 4 counts **bytes**; `.chars().count()` → 3 |
| `s[0]` → `'Z'` | O(1), allowed | refused — `.chars().nth(0)`, honestly O(n) |
| no char type — `'a'` is a 1-char `str` | | `char` is its own 4-byte type |
| `"é" != "é"` | same problem | same problem — `unicodedata.normalize` ↔ the normalization crate |

Python affords `s[0]` by storing each string at a fixed width — 1, 2 or 4 bytes per character, chosen per string (PEP 393) — paying memory for O(1) indexing. Rust stores UTF-8 and refuses to pretend indexing is cheap. Neither counts graphemes; both leave that to a library.

**ABAP.** ABAP text is UTF-16, so it has the *same* mismatch one layer up.

| ABAP | | Rust |
|---|---|---|
| `strlen( lv )` | counts UTF-16 **code units** | `.len()` counts UTF-8 **bytes** |
| `lv+2(1)` | slices by code unit — can cut a surrogate pair in half | `&s[2..3]` panics / `get` returns `None` at a cut |
| an emoji costs 2 units | the BMP boundary | an emoji costs 4 bytes |
| `string` vs `xstring` | text vs raw bytes | `String` vs `Vec<u8>` — the same split |

What changes: ABAP lets the bad slice happen and hands you half a surrogate at runtime; Rust makes the cut refuse loudly (`None`) or immediately (a panic at the slice), never silently.

---

## Practice

**One name, three lengths.** Write `inventory(s: &str)` printing one row per char: byte offset, the char, its UTF-8 width, its `U+XXXX` code point (`char_indices`, `len_utf8`, `as u32`). Run it on `"Łódź"`.

Then spell `"Zoé"` twice — composed (`é`) and decomposed (`e\u{301}`) — and: prove the two are `!=` while printing identically; give byte and char counts for both; and say which of the three length answers std cannot compute. Finish with `get` on a range that cuts `Ł` in half.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:meet_the_char_kata -->
*[`meet_the_char_kata.rs`](examples/meet_the_char_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one name, three lengths — a per-char inventory, and the
//! accent that makes two equal-looking strings unequal.
//!
//!   rustc --edition 2024 meet_the_char_kata.rs -o /tmp/mtck && /tmp/mtck

/// One row per char: byte offset, the char, its UTF-8 width, its code point.
fn inventory(label: &str, s: &str) {
    println!("   {label} = {s:?}");
    for (offset, c) in s.char_indices() {
        println!("      byte {offset}: {c:?}   {} byte(s)   U+{:04X}", c.len_utf8(), c as u32);
    }
    println!("      -> {} bytes, {} chars", s.len(), s.chars().count());
}

fn main() {
    println!("Round 1 — the inventory");
    inventory("city", "Łódź");

    println!("\nRound 2 — the same-looking name, spelled two ways");
    let composed = "Zoé"; // é as one char, U+00E9
    let decomposed = "Zoe\u{301}"; // e + combining acute, two chars
    inventory("composed", composed);
    inventory("decomposed", decomposed);
    println!("   composed == decomposed?  {}", composed == decomposed);

    println!("\nRound 3 — which counts can std give you?");
    println!("   bytes:     .len()            -> {} vs {}", composed.len(), decomposed.len());
    println!("   chars:     .chars().count()  -> {} vs {}", composed.chars().count(), decomposed.chars().count());
    println!("   graphemes: what a reader sees -> 3 vs 3, but std cannot count");
    println!("              these; the unicode-segmentation crate can");

    println!("\nRound 4 — safe slicing needs a boundary");
    let city = "Łódź";
    println!("   city.get(0..1) = {:?}   <- inside 'Ł', refused as None", city.get(0..1));
    println!("   city.get(0..2) = {:?}", city.get(0..2));
    let first = city.chars().next();
    println!("   city.chars().next() = {first:?}   <- the honest way to ask for one char");
}
```
<!-- /source -->

<!-- output:meet_the_char_kata -->
*Verified output of [`meet_the_char_kata.rs`](examples/meet_the_char_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Round 1 — the inventory
   city = "Łódź"
      byte 0: 'Ł'   2 byte(s)   U+0141
      byte 2: 'ó'   2 byte(s)   U+00F3
      byte 4: 'd'   1 byte(s)   U+0064
      byte 5: 'ź'   2 byte(s)   U+017A
      -> 7 bytes, 4 chars

Round 2 — the same-looking name, spelled two ways
   composed = "Zoé"
      byte 0: 'Z'   1 byte(s)   U+005A
      byte 1: 'o'   1 byte(s)   U+006F
      byte 2: 'é'   2 byte(s)   U+00E9
      -> 4 bytes, 3 chars
   decomposed = "Zoe\u{301}"
      byte 0: 'Z'   1 byte(s)   U+005A
      byte 1: 'o'   1 byte(s)   U+006F
      byte 2: 'e'   1 byte(s)   U+0065
      byte 3: '\u{301}'   2 byte(s)   U+0301
      -> 5 bytes, 4 chars
   composed == decomposed?  false

Round 3 — which counts can std give you?
   bytes:     .len()            -> 4 vs 5
   chars:     .chars().count()  -> 3 vs 4
   graphemes: what a reader sees -> 3 vs 3, but std cannot count
              these; the unicode-segmentation crate can

Round 4 — safe slicing needs a boundary
   city.get(0..1) = None   <- inside 'Ł', refused as None
   city.get(0..2) = Some("Ł")
   city.chars().next() = Some('Ł')   <- the honest way to ask for one char
```
<!-- /output -->

</details>

---

**Case and whitespace, and the two of them that are not one operation.** Trim a string padded with spaces, a tab and newlines. Convert `MyVariableName` to `my_variable_name` and back again — then run the pair on `STARVoting` and report what it does. Swap the case of only the ASCII letters, with `make_ascii_uppercase` / `make_ascii_lowercase`, on input that also contains `é` and Cyrillic.

Then two that need characters rather than bytes: alternating `SpOnGeBoB` case where a space does not consume a turn, and an anagram checker that ignores case and whitespace. Finish by saying why `'ß'.to_uppercase()` is the reason the ASCII methods exist alongside the Unicode ones.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:case_and_whitespace_kata -->
*[`case_and_whitespace_kata.rs`](examples/case_and_whitespace_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: six transformations, and the two places ASCII case and
//! Unicode case stop being the same operation.
//!
//!   rustc --edition 2024 case_and_whitespace_kata.rs -o /tmp/cwk && /tmp/cwk

/// `MyVariableName` -> `my_variable_name`. An underscore goes before every
/// uppercase letter that is not the first character.
fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// `my_variable_name` -> `MyVariableName`. Split on the underscores, uppercase
/// the first character of each piece — and the first character may be two.
fn to_camel_case(s: &str) -> String {
    s.split('_')
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            let mut chars = piece.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Swap the case of the ASCII letters and leave everything else exactly as it
/// was — using the in-place `char` methods, which is what makes this ASCII-only.
fn swap_ascii_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            let mut c = c;
            if c.is_ascii_uppercase() {
                c.make_ascii_lowercase();
            } else if c.is_ascii_lowercase() {
                c.make_ascii_uppercase();
            }
            c
        })
        .collect()
}

/// Alternating case, counting only the letters — so a space does not consume a
/// turn and the pattern survives it.
fn spongebob(s: &str) -> String {
    let mut upper = false;
    s.chars()
        .map(|c| {
            if c.is_alphabetic() {
                upper = !upper;
                if upper {
                    c.to_uppercase().collect::<String>()
                } else {
                    c.to_lowercase().collect::<String>()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Same letters, ignoring case and whitespace.
fn is_anagram(a: &str, b: &str) -> bool {
    fn key(s: &str) -> Vec<char> {
        let mut v: Vec<char> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(|c| c.to_lowercase())
            .collect();
        v.sort_unstable();
        v
    }
    key(a) == key(b)
}

fn main() {
    println!("1. trim");
    let messy = "\t  Ada Lovelace \n\r\n ";
    println!("   {messy:?}");
    println!("   .trim()       {:?}", messy.trim());
    println!("   .trim_start() {:?}", messy.trim_start());
    println!("   .trim_end()   {:?}", messy.trim_end());
    println!("   trim removes any Unicode whitespace, not just the space character:");
    println!("   {:?}.trim() = {:?}   <- U+00A0, a non-breaking space",
        "\u{a0}x\u{a0}", "\u{a0}x\u{a0}".trim());
    println!("   {:?}.trim_ascii() = {:?}   <- the ASCII-only version leaves it",
        "\u{a0}x\u{a0}", "\u{a0}x\u{a0}".trim_ascii());

    println!("\n2. CamelCase -> snake_case");
    for s in ["MyVariableName", "STARVoting", "Ballot", "alreadySnakeish"] {
        println!("   {:<16} -> {:?}", s, to_snake_case(s));
    }
    println!("   `STARVoting` shows the limit of the one-rule version: an acronym");
    println!("   becomes s_t_a_r_voting. Real converters carry a second rule for a");
    println!("   run of capitals — say so rather than pretending the rule is complete.");

    println!("\n3. snake_case -> CamelCase");
    for s in ["my_variable_name", "ballot", "top_two_runoff", "trailing_"] {
        println!("   {:<16} -> {:?}", s, to_camel_case(s));
    }
    println!("   Round trip: {:?} -> {:?} -> {:?}",
        "my_variable_name",
        to_camel_case("my_variable_name"),
        to_snake_case(&to_camel_case("my_variable_name")));
    println!("   It survives here, and does not for STARVoting — a converter pair is");
    println!("   only a round trip on the names both rules agree about.");

    println!("\n4. Swapping ASCII case only");
    for s in ["Hello World", "café AU LAIT", "ЖУРНАЛ"] {
        println!("   {:<14} -> {:?}", s, swap_ascii_case(s));
    }
    println!("   The é and the Cyrillic are untouched: make_ascii_*case is defined");
    println!("   to leave every non-ASCII byte alone. That is a promise, not a bug —");
    println!("   it is the one case conversion that cannot change a string's length.");
    println!("   The Unicode version can: {:?}.to_uppercase() = {:?} ({} chars)",
        'ß', 'ß'.to_uppercase().collect::<String>(), 'ß'.to_uppercase().count());

    println!("\n5. Spongebob case");
    for s in ["hello world", "star voting is good"] {
        println!("   {:?}", spongebob(s));
    }
    let naive: String = "hello world".chars().enumerate()
        .map(|(i, c)| if i % 2 == 0 { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() })
        .collect();
    println!("   Alternating on every char instead of every letter: {naive:?}");
    println!("   The space took a turn, so the two disagree from the w onward:");
    println!("   letters only {:?} against every char {:?}.",
        &spongebob("hello world")[6..], &naive[6..]);
    println!("   Which is right is a taste question; which one you WROTE should not be.");

    println!("\n6. Anagrams");
    for (a, b) in [
        ("Listen", "Silent"),
        ("The eyes", "They see"),
        ("Dormitory", "Dirty Room"),
        ("ballot", "ballots"),
    ] {
        println!("   {:<14} vs {:<14} {}", format!("{a:?}"), format!("{b:?}"), is_anagram(a, b));
    }
    println!("   Sorting chars is the whole trick, and it is why this is a `char`");
    println!("   exercise: sorting BYTES would compare halves of multibyte letters.");
}
```
<!-- /source -->

<!-- output:case_and_whitespace_kata -->
*Verified output of [`case_and_whitespace_kata.rs`](examples/case_and_whitespace_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. trim
   "\t  Ada Lovelace \n\r\n "
   .trim()       "Ada Lovelace"
   .trim_start() "Ada Lovelace \n\r\n "
   .trim_end()   "\t  Ada Lovelace"
   trim removes any Unicode whitespace, not just the space character:
   "\u{a0}x\u{a0}".trim() = "x"   <- U+00A0, a non-breaking space
   "\u{a0}x\u{a0}".trim_ascii() = "\u{a0}x\u{a0}"   <- the ASCII-only version leaves it

2. CamelCase -> snake_case
   MyVariableName   -> "my_variable_name"
   STARVoting       -> "s_t_a_r_voting"
   Ballot           -> "ballot"
   alreadySnakeish  -> "already_snakeish"
   `STARVoting` shows the limit of the one-rule version: an acronym
   becomes s_t_a_r_voting. Real converters carry a second rule for a
   run of capitals — say so rather than pretending the rule is complete.

3. snake_case -> CamelCase
   my_variable_name -> "MyVariableName"
   ballot           -> "Ballot"
   top_two_runoff   -> "TopTwoRunoff"
   trailing_        -> "Trailing"
   Round trip: "my_variable_name" -> "MyVariableName" -> "my_variable_name"
   It survives here, and does not for STARVoting — a converter pair is
   only a round trip on the names both rules agree about.

4. Swapping ASCII case only
   Hello World    -> "hELLO wORLD"
   café AU LAIT   -> "CAFé au lait"
   ЖУРНАЛ         -> "ЖУРНАЛ"
   The é and the Cyrillic are untouched: make_ascii_*case is defined
   to leave every non-ASCII byte alone. That is a promise, not a bug —
   it is the one case conversion that cannot change a string's length.
   The Unicode version can: 'ß'.to_uppercase() = "SS" (2 chars)

5. Spongebob case
   "HeLlO wOrLd"
   "StAr VoTiNg Is GoOd"
   Alternating on every char instead of every letter: "HeLlO WoRlD"
   The space took a turn, so the two disagree from the w onward:
   letters only "wOrLd" against every char "WoRlD".
   Which is right is a taste question; which one you WROTE should not be.

6. Anagrams
   "Listen"       vs "Silent"       true
   "The eyes"     vs "They see"     true
   "Dormitory"    vs "Dirty Room"   true
   "ballot"       vs "ballots"      false
   Sorting chars is the whole trick, and it is why this is a `char`
   exercise: sorting BYTES would compare halves of multibyte letters.
```
<!-- /output -->

</details>

---

**The third ruler.** `"cafe\u{301}"` is `café` written with a combining accent. Reverse it with `.chars().rev().collect()` and look at where the accent ended up. Then count the characters in `"👨‍👩‍👧‍👦"` — the answer is seven — and say what a reader would have answered.

Now write the grouper: a base character plus the marks, joiners, variation selectors and skin tones that attach to it, and a regional-indicator pair for a flag. Reverse by cluster and check the accent stays home. Then state precisely which cases your grouper does *not* cover — that list is the reason grapheme segmentation is a crate rather than a method on `str`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:grapheme_clusters_kata -->
*[`grapheme_clusters_kata.rs`](examples/grapheme_clusters_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the third ruler. `chars().rev()` is not "reverse the text",
//! and `chars().count()` is not "how many characters a reader sees".
//!
//!   rustc --edition 2024 grapheme_clusters_kata.rs -o /tmp/gck && /tmp/gck
//!
//! std has no grapheme segmentation, on purpose: the rules are a Unicode annex
//! (UAX #29) that changes with each Unicode release, so they live in a crate
//! that can ship on Unicode's schedule rather than Rust's. In real code:
//!
//!     use unicode_segmentation::UnicodeSegmentation;
//!     let n = s.graphemes(true).count();
//!     let back: String = s.graphemes(true).rev().collect();
//!
//! The segmenter below is NOT that. It handles exactly the five joiners this
//! page uses — combining marks, ZWJ sequences, variation selectors, skin-tone
//! modifiers and regional-indicator pairs — and nothing else. It is here so the
//! *shape* of the answer is visible; it is not a substitute for the crate.

/// A mark that attaches to the character before it.
fn is_extender(c: char) -> bool {
    matches!(c,
        '\u{0300}'..='\u{036F}'      // combining diacritical marks
        | '\u{1AB0}'..='\u{1AFF}'    // combining diacritical marks extended
        | '\u{20D0}'..='\u{20FF}'    // combining marks for symbols
        | '\u{FE00}'..='\u{FE0F}'    // variation selectors
        | '\u{1F3FB}'..='\u{1F3FF}') // skin-tone modifiers
}

fn is_regional_indicator(c: char) -> bool {
    ('\u{1F1E6}'..='\u{1F1FF}').contains(&c)
}

/// Split into clusters: a base character plus whatever attaches to it.
fn clusters(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let mut cluster = String::new();
        cluster.push(chars[i]);
        // A flag is exactly two regional indicators.
        if is_regional_indicator(chars[i])
            && i + 1 < chars.len()
            && is_regional_indicator(chars[i + 1])
        {
            cluster.push(chars[i + 1]);
            i += 2;
        } else {
            i += 1;
        }
        // Then absorb marks, and anything a zero-width joiner attaches.
        while i < chars.len() {
            if is_extender(chars[i]) {
                cluster.push(chars[i]);
                i += 1;
            } else if chars[i] == '\u{200D}' && i + 1 < chars.len() {
                cluster.push(chars[i]);
                cluster.push(chars[i + 1]);
                i += 2;
            } else {
                break;
            }
        }
        out.push(cluster);
    }
    out
}

fn grapheme_count(s: &str) -> usize {
    clusters(s).len()
}

fn reverse_graphemes(s: &str) -> String {
    clusters(s).into_iter().rev().collect()
}

fn escaped(s: &str) -> String {
    s.chars().map(|c| c.escape_unicode().to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    // "café" written the second legal way: e + U+0301 COMBINING ACUTE ACCENT.
    let cafe = "cafe\u{301}";
    let family = "👨\u{200D}👩\u{200D}👧\u{200D}👦";
    let flag = "🇵🇱";

    println!("1. Three answers to \"how long is it\"");
    println!("   {:<12} {:>5} {:>6} {:>10}", "string", "bytes", "chars", "graphemes");
    for (label, s) in [("cafe+U+0301", cafe), ("family", family), ("flag", flag), ("plain ada", "ada")] {
        println!("   {:<12} {:>5} {:>6} {:>10}", label, s.len(), s.chars().count(), grapheme_count(s));
    }
    println!("   The family emoji is 4 people and 3 zero-width joiners: 7 chars, 1");
    println!("   thing a reader points at. len() and chars().count() are both right");
    println!("   answers to questions nobody asked.");

    println!("\n2. Reversing by char breaks it");
    println!("   original          {cafe:?}  -> renders as {cafe}");
    let by_char: String = cafe.chars().rev().collect();
    println!("   chars().rev()     {by_char:?}  -> renders as {by_char}");
    println!("   the accent is now FIRST, with no base character in front of it — so it");
    println!("   renders on its own, and would land on whatever this string is glued to:");
    println!("   {}", escaped(&by_char));

    println!("\n3. Reversing by grapheme keeps it");
    let by_cluster = reverse_graphemes(cafe);
    println!("   clusters          {:?}", clusters(cafe));
    println!("   reversed          {by_cluster:?}  -> renders as {by_cluster}");
    println!("   {}", escaped(&by_cluster));
    println!("   The é travelled as one unit, because the mark moved with its base.");

    println!("\n4. The same test on the emoji");
    let emoji = format!("{family}{flag}!");
    println!("   input             {emoji:?}");
    println!("   chars().rev()     {:?}", emoji.chars().rev().collect::<String>());
    println!("   graphemes rev     {:?}", reverse_graphemes(&emoji));
    println!("   The joiners survive, but the sequence does not: boy-girl-woman-man is");
    println!("   not a defined emoji sequence, so it renders as four separate people —");
    println!("   and the flag's two regional indicators swapped into a pair that is not");
    println!("   a country. Neither string is corrupt UTF-8; every char is still valid,");
    println!("   which is why nothing errors and the bug reaches a screen.");

    println!("\n5. What this segmenter is not");
    println!("   It knows five joiners. UAX #29 also covers Hangul syllable blocks,");
    println!("   Indic conjuncts, prepend characters and more, and the rules move with");
    println!("   each Unicode release — which is exactly why std does not have them.");
    println!("   Reach for unicode-segmentation the moment the input is not yours.");
}
```
<!-- /source -->

<!-- output:grapheme_clusters_kata -->
*Verified output of [`grapheme_clusters_kata.rs`](examples/grapheme_clusters_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three answers to "how long is it"
   string       bytes  chars  graphemes
   cafe+U+0301      6      5          4
   family          25      7          1
   flag             8      2          1
   plain ada        3      3          3
   The family emoji is 4 people and 3 zero-width joiners: 7 chars, 1
   thing a reader points at. len() and chars().count() are both right
   answers to questions nobody asked.

2. Reversing by char breaks it
   original          "cafe\u{301}"  -> renders as café
   chars().rev()     "\u{301}efac"  -> renders as ́efac
   the accent is now FIRST, with no base character in front of it — so it
   renders on its own, and would land on whatever this string is glued to:
   \u{301} \u{65} \u{66} \u{61} \u{63}

3. Reversing by grapheme keeps it
   clusters          ["c", "a", "f", "e\u{301}"]
   reversed          "e\u{301}fac"  -> renders as éfac
   \u{65} \u{301} \u{66} \u{61} \u{63}
   The é travelled as one unit, because the mark moved with its base.

4. The same test on the emoji
   input             "👨\u{200d}👩\u{200d}👧\u{200d}👦🇵🇱!"
   chars().rev()     "!🇱🇵👦\u{200d}👧\u{200d}👩\u{200d}👨"
   graphemes rev     "!🇵🇱👨\u{200d}👩\u{200d}👧\u{200d}👦"
   The joiners survive, but the sequence does not: boy-girl-woman-man is
   not a defined emoji sequence, so it renders as four separate people —
   and the flag's two regional indicators swapped into a pair that is not
   a country. Neither string is corrupt UTF-8; every char is still valid,
   which is why nothing errors and the bug reaches a screen.

5. What this segmenter is not
   It knows five joiners. UAX #29 also covers Hangul syllable blocks,
   Indic conjuncts, prepend characters and more, and the rules move with
   each Unicode release — which is exactly why std does not have them.
   Reach for unicode-segmentation the moment the input is not yours.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:meet_the_char -->
*Verified output of [`meet_the_char.rs`](examples/meet_the_char.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One string, two counts
   "Zoë".len()           = 4 bytes
   "Zoë".chars().count() = 3 chars

2. Where each char sits, and what it costs in the string
   byte 0: 'Z'   1 byte(s) in UTF-8, U+005A
   byte 1: 'o'   1 byte(s) in UTF-8, U+006F
   byte 2: 'ë'   2 byte(s) in UTF-8, U+00EB

3. A char VALUE is always 4 bytes — decoded, not encoded
   size_of::<char>() = 4 bytes
   'ë' costs 4 bytes as a char, 2 bytes inside a String

4. Slicing is by byte, and only at char boundaries
   name.get(0..1) = Some("Z")
   name.get(2..4) = Some("ë")
   name.get(2..3) = None      <- byte 3 is the middle of 'ë'
   name.is_char_boundary(3) = false

5. Case is not one-to-one
   'ß'.to_uppercase() = "SS"   <- one char in, two out

6. Two spellings that look identical
   composed   "é"  1 char(s), 2 bytes
   decomposed "e\u{301}"  2 char(s), 3 bytes
   composed == decomposed?  false
   what a READER calls one character is a third counting — the
   grapheme — and std stops before it; that one needs a crate
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/meet_the_char/examples/meet_the_char.rs -o /tmp/mtc && /tmp/mtc
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — the unit; this page is what the bytes *mean*
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — where those bytes live
- [Six kinds of string](../six_kinds_of_string/README.md) — the types for text that does *not* keep UTF-8's promise
- [The Rust Book, ch. 8.2 ↗](https://doc.rust-lang.org/book/ch08-02-strings.html) — its "bytes, scalar values, grapheme clusters" section is this page's origin story

## Po polsku

Ta strona po polsku czyta się inaczej niż po angielsku, bo dla nas nie opisuje egzotycznego przypadku brzegowego, tylko codzienność. Ćwiczenie wyżej liczy `"Łódź"`: **7 bajtów, 4 znaki**. Każda polska litera diakrytyczna — ą ć ę ł ń ó ś ź ż i wersalikowe odpowiedniki — zajmuje w UTF-8 dwa bajty, więc `.len()` i `.chars().count()` rozjeżdżają się właściwie na każdym prawdziwym polskim słowie, a nie dopiero na emotikonie. Przeszkadza w tym odruch wyniesiony ze stron kodowych: w Windows-1250 czy ISO-8859-2 `Ł` było jednym bajtem i `strlen` naprawdę zwracał liczbę znaków. Ta epoka zostawiła po sobie krzaki, a UTF-8 je usunął — kosztem tego, że długość tekstu przestała mieć jedną odpowiedź. Warto też trzymać w głowie asymetrię: znak (*char*) jako wartość ma **zawsze** 4 bajty, bo jest zdekodowany i stałej szerokości, a ten sam znak wewnątrz łańcucha zajmuje 1–4 bajty, bo jest zakodowany i ma być mały.

Dlatego `s[0]` się nie kompiluje, i po polsku powód widać od razu: bajt 0 w `"Łódź"` to dopiero **połowa** litery `Ł`. Indeksowanie obiecuje czas stały, a jedyną rzeczą dostępną w czasie stałym jest bajt — więc Rust nie zgaduje, o co ci chodziło, tylko każe wybrać: `.bytes().nth(0)` da bajt, `.chars().nth(0)` da znak (uczciwie w czasie liniowym), a `.get(0..2)` da `Some("Ł")`, podczas gdy `.get(0..1)` zwróci `None`, bo tam nie ma granicy znaku. Wycinanie po zakresie bajtów (`&s[0..1]`) też działa, ale w tym samym miejscu **panikuje** — `get` to to samo pytanie zadane grzecznie, a `is_char_boundary(i)` pozwala zapytać wcześniej.

Zostają dwie pułapki, które trafiają w polski tekst szczególnie celnie. Pierwsza: `ó` ma dwa legalne zapisy — jako gotowy znak U+00F3 albo jako `o` plus łącząca kreska U+0301 — a `==` w Ruscie porównuje bajty, więc dwa wyglądające identycznie nazwiska mogą być różne. Zdarza się to dokładnie wtedy, gdy tekst przychodzi z różnych systemów (nazwy plików z macOS-a bywają zapisane w postaci rozłożonej), i lekiem jest normalizacja z crate'a [`unicode-normalization` ↗](https://crates.io/crates/unicode-normalization), a nie kolejny `if`. Druga: metody z rodziny `ascii` po cichu omijają polskie litery, bo mają to obiecane — `"żółw".to_ascii_uppercase()` daje `"żółW"`, zmienia się samo `w`. Wersja unikodowa, `to_uppercase()`, daje `"ŻÓŁW"`; sięgaj po `ascii_` tylko wtedy, gdy naprawdę chcesz ruszyć wyłącznie ASCII. I trzecia miarka na koniec: to, co czytelnik nazywa jednym znakiem, to grafem (*grapheme*), a tego std nie liczy w ogóle — od tego jest [`unicode-segmentation` ↗](https://crates.io/crates/unicode-segmentation).

**Szukaj po polsku:** polskie znaki w UTF-8 · liczba bajtów a liczba znaków · normalizacja Unicode NFC NFD · `rust str cannot be indexed by integer E0277` · `rust bytes chars graphemes`
