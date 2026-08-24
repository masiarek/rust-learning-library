# Walking a `String`

**Level:** 101 → 201 · working knowledge

**One line:** Three item types — `u8` from `bytes()`, `char` from `chars()`, `&str` from the whole split family — and every split is *the gaps between the matches*, which is where all those surprising empty strings come from.

| iterator | item | on `"bête noir  d'Arrrgh "` |
|---|---|---|
| `.bytes()` | `u8` | 21 items |
| `.chars()` | `char` | 20 items |
| `.char_indices()` | `(usize, char)` | 20 items, offsets 0, 1, 3, 4… |
| `.split(' ')` | `&str` | 5 — `["bête", "noir", "", "d'Arrrgh", ""]` |
| `.split_terminator(' ')` | `&str` | 4 — the trailing empty is dropped |
| `.split_whitespace()` | `&str` | 3 — runs collapse, ends trimmed |
| `.splitn(3, ' ')` | `&str` | 3 — the remainder stays whole |
| `.matches("rr")` | `&str` | 1 |
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
rustc --edition 2024 01_Foundations/walking_a_string/examples/walking_a_string.rs -o /tmp/ws && /tmp/ws
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [Meet the `char`](../meet_the_char/README.md) — why `bytes()` and `chars()` give different counts
- [String slices](../string_slices/README.md) — what those byte offsets are for, and how they panic
- [Meet the byte](../meet_the_byte/README.md) — the `u8` that `bytes()` yields
- [`Option` as a collection](../option_as_collection/README.md) — the `Option<u8>` the kata builds, iterated
- [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split) · [`Pattern`](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html) · [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once)
