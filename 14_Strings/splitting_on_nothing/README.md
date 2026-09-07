# Splitting on nothing

**Level:** 201 · working knowledge

**One line:** `"abc".split("")` gives five pieces and not three — the empty pattern matches at every char boundary, and `split` has no special case for it, where JavaScript and Go both do and Python refuses the call outright.

```rust
let s = "abc";
println!("{:?}", s.split("").collect::<Vec<&str>>());  // ["", "a", "b", "c", ""]
println!("{:?}", s.chars().collect::<Vec<char>>());    // ['a', 'b', 'c']
```

Three characters in, five pieces out, two of them empty. Nothing has gone wrong, and nothing is being helpful either: the count is the same arithmetic every other pattern obeys.

## Where the five come from

`split` yields the gaps *between* the matches, so *n* matches give *n+1* pieces — the rule [Walking a `String`](../walking_a_string/README.md) sets up and [`str::split`](../str_methods/str_split/README.md) states in full. The only open question is how many times an empty pattern matches in `"abc"`, and [`match_indices`](../str_methods/str_match_indices/README.md) answers it without any guessing:

```rust
println!("{:?}", "abc".match_indices("").collect::<Vec<(usize, &str)>>());
// [(0, ""), (1, ""), (2, ""), (3, "")]
```

Four positions — before `a`, between `a` and `b`, between `b` and `c`, after `c`. Four matches, five pieces. The shortest way to hold it is steffahn's picture from the forum thread in *See also*: write an arrow at each of those positions and `"abc".split("")` is `"↓a↓b↓c↓".split("↓")`, which surprises nobody.

| call | matches | pieces |
|---|---|---|
| `"a,b,c".split(',')` | 2 | 3 — `["a", "b", "c"]` |
| `"xaxbxcx".split('x')` | 4 | 5 — `["", "a", "b", "c", ""]` |
| `"abc".split("")` | 4 | 5 — `["", "a", "b", "c", ""]` |
| `"".split(',')` | 0 | 1 — `[""]` |
| `"".split("")` | 1 | 2 — `["", ""]` |

The last two rows are the sharpest pair here. An empty string has no characters and one *position*, so a comma finds nothing in it and the empty pattern finds one match — and the string with nothing in it is the one that splits into two.

## Not infinitely many, because matches do not overlap

The objection the forum thread opens with is a good one: an empty string occurs infinitely often at every position, so the call ought to have no answer at all. It has one because `split` counts **non-overlapping** matches — the searcher takes a match, steps past it, looks again — so each position contributes exactly one and the total is finite. std documents the outcome rather than leaving it to be derived: an empty separator, its docs say, separates every character, plus the two ends.

The empty pattern is also the one case with a searcher of its own. [Inside a `Split`](../inside_a_split/README.md) reads the plan `split` hands back and lists four machines, one per pattern shape; an empty `&str` needle gets a fifth, and it is the only one whose fields are all positions and flags rather than string-search constants:

```text title="Abridged — just the searcher field of each dump; rustc 1.98.0, and these are std's private internals, not a stable API"
"abc".split("")   searcher: Empty(EmptyNeedle { position: 0, end: 3, is_match_fw: true, is_match_bw: true, is_finished: false })
"abc".split("b")  searcher: TwoWay(TwoWaySearcher { crit_pos: 0, crit_pos_back: 1, period: 1, byteset: 17179869184, … })
```

There is nothing to search for, so the searcher only has to walk.

## The positions are char boundaries, not byte offsets

This is the half the thread never reaches, and it is the half that starts mattering the moment the text is not ASCII. `"añb"` is four bytes, and the matches land at 0, 1 and 3 — never at 2, which is inside `ñ`:

```rust
let u = "añb";
println!("{}", u.len());                               // 4
println!("{:?}", u.split("").collect::<Vec<&str>>());  // ["", "a", "ñ", "b", ""]
println!("{}", u.is_char_boundary(2));                 // false
```

That is forced rather than chosen. Every piece `split` yields is a `&str`, and a `&str` is UTF-8 by construction, so a piece beginning mid-character would be a value the type cannot hold. The empty pattern is the one case where the searcher's step is the *character* width rather than the needle's, which makes this page [`is_char_boundary`](../str_methods/str_is_char_boundary/README.md) wearing a different hat.

## A char boundary is not a letter boundary

It does not follow that the pieces are letters. `"a\u{301}"` is an `a` followed by a combining acute — one `á` on screen, two `char`s, four pieces. `"👍🏽"` is a thumbs-up followed by a skin-tone modifier — one emoji on screen, two `char`s, four pieces again, and the two middle ones render as a yellow thumb and a lone brown square.

So `split("")` never cuts a *character* in half and cuts a *letter* in half routinely. Counting what a reader calls one character is a job std does not do at all — that is the [`unicode-segmentation` ↗](https://crates.io/crates/unicode-segmentation) crate's, as [Meet the `char`](../meet_the_char/README.md) sets out.

## The family on an empty pattern

| call on `"abc"` | result | why |
|---|---|---|
| [`split("")`](../str_methods/str_split/README.md) | `["", "a", "b", "c", ""]` | n+1, both ends kept |
| [`split_terminator("")`](../str_methods/str_split_terminator/README.md) | `["", "a", "b", "c"]` | drops a *trailing* empty only; the leading one is not its business |
| [`split_inclusive("")`](../str_methods/str_split_inclusive/README.md) | `["", "a", "b", "c"]` | each piece keeps its terminator, and an empty terminator adds nothing |
| [`rsplit("")`](../str_methods/str_rsplit/README.md) | `["", "c", "b", "a", ""]` | the same five, back to front |
| [`splitn(3, "")`](../str_methods/str_splitn/README.md) | `["", "a", "bc"]` | two splits, then stop |
| [`split_once("")`](../str_methods/str_split_once/README.md) | `Some(("", "abc"))` | the first match is at offset 0, so it splits off nothing |
| [`rsplit_once("")`](../str_methods/str_rsplit_once/README.md) | `Some(("abc", ""))` | reading order, where [`rsplitn(2, "")`](../str_methods/str_rsplitn/README.md) gives `["", "abc"]` |
| [`matches("").count()`](../str_methods/str_matches/README.md) | `4` | the matches, not the gaps |

`splitn` is the row to watch. Reached for as *"the first three characters"* it returns one character, one empty string, and the remainder of the word.

The two `_once` rows are the sharpest, because their `Option` exists to report **no match** and against an empty pattern there is no such thing: `split_once("")` is `Some(("", s))` for every string in the language, the empty one included. The advantage that method has over `splitn(2, …)` — a miss you are made to handle rather than a one-element iterator you might not notice — is worth nothing here, since the two return the same two pieces and the miss cannot happen.

## What to reach for instead

| you want | write |
|---|---|
| the characters | `s.chars()` — yields `char`, not `&str` |
| the characters as `&str` | `char_indices()`, then slice `c.len_utf8()` bytes from each index — the kata below writes it |
| …or, shorter | `s.split("")` with the empties filtered out |
| the bytes | `s.bytes()` or `s.as_bytes()` |
| what a reader calls characters | the [`unicode-segmentation` ↗](https://crates.io/crates/unicode-segmentation) crate; std has no answer |

## If you are coming from another language

Three languages give three different answers to the same call, and a fourth cannot be asked it at all. Rust is the only one of them with no rule of its own for this case.

**Python** refuses. `"abc".split("")` raises `ValueError: empty separator`, and the idiom for the characters is `list("abc")`. Worth noticing on the way past: Python's argument-less `s.split()` is a *third* behaviour again — it splits on runs of whitespace and drops the empties — and that one is `split_whitespace()` in Rust, never `split`. One Python method spans three behaviours that Rust keeps as three methods, which is why the habit does not transfer.

**JavaScript** special-cases it. `"abc".split("")` is `["a","b","c"]`, with no empties at the ends. The sharper difference is what it splits *into*: UTF-16 code units, so `"👍🏽".split("")` hands back four lone surrogates — four strings that are not valid text, printing as `\ud83d`, `\udc4d`, `\ud83c`, `\udffd`. Rust cannot return that, because `&str` is UTF-8 by construction, and the char-boundary rule above is the price of making it impossible.

**Go** special-cases it too, and more carefully: `strings.Split("abc", "")` is `["a" "b" "c"]`, splitting after each UTF-8 sequence, so every piece is valid text. What the special case costs is Go's own arithmetic. `len(strings.Split(s, sep))` is normally `strings.Count(s, sep) + 1` — 3 and 2 on `("a,b,c", ",")` — but on the empty separator `Count("abc", "")` is 4 while `Split` returns 3, so the identity is off by two in exactly the place a reader would use it to check their understanding. Rust's two empty pieces are the price of keeping that identity.

**ABAP** cannot be asked the question, and the obstacle is the literal rather than the method. `''` is a *text field literal* — type `c`, and a `c` field cannot have length zero, so it is one blank rather than an empty string. The nearest-looking spelling, `SPLIT text AT '' INTO TABLE lt`, therefore hands `SPLIT` a separator that is a space, and is asking something else entirely. The zero-length value has to be written with backquotes, `` `` ``, which is type `string`. An ABAP reader arriving here should distrust the literal first and the method second.

## The verified output

<!-- source:splitting_on_nothing -->
*[`splitting_on_nothing.rs`](examples/splitting_on_nothing.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! `"abc".split("")` yields FIVE pieces, not three. `split` reports the gaps
//! between matches — n matches, n+1 pieces — and the empty pattern matches at
//! every char boundary, of which "abc" has four. There is no special case
//! anywhere in this; the arithmetic is the whole explanation.
//!
//! Run:  rustc --edition 2024 splitting_on_nothing.rs && ./splitting_on_nothing

fn main() {
    let s = "abc";

    println!("1. Three characters, five pieces");
    println!("   split(\"\")           {:?}", s.split("").collect::<Vec<&str>>());
    println!("   chars()             {:?}", s.chars().collect::<Vec<char>>());

    println!();
    println!("2. Where the five come from — the matches, at their byte offsets");
    println!("   match_indices(\"\")   {:?}", s.match_indices("").collect::<Vec<(usize, &str)>>());
    println!("   {} matches, {} pieces — the same n+1 as any other pattern",
             s.matches("").count(), s.split("").count());
    println!("   \"xaxbxcx\".split('x') {:?}   <- four matches too",
             "xaxbxcx".split('x').collect::<Vec<&str>>());

    println!();
    println!("3. Even the empty string has one position in it");
    println!("   \"\".split(',')       {:?}   0 matches, 1 piece", "".split(',').collect::<Vec<&str>>());
    println!("   \"\".split(\"\")        {:?}   1 match,  2 pieces", "".split("").collect::<Vec<&str>>());

    println!();
    println!("4. The positions are CHAR boundaries, not byte offsets");
    let u = "añb";
    println!("   \"añb\".len()         {} bytes, {} chars", u.len(), u.chars().count());
    println!("   char_indices()      {:?}", u.char_indices().collect::<Vec<(usize, char)>>());
    println!("   split(\"\")           {:?}", u.split("").collect::<Vec<&str>>());
    println!("   byte 2 is inside 'ñ': is_char_boundary(2) = {}, and no piece starts there",
             u.is_char_boundary(2));

    println!();
    println!("5. ...but a char boundary is not a LETTER boundary");
    let accented = "a\u{301}";                  // 'a' + combining acute, one á on screen
    println!("   {:?} renders as {}    -> {:?}",
             accented, accented, accented.split("").collect::<Vec<&str>>());
    let modified = "\u{1F44D}\u{1F3FD}";        // thumbs-up + skin-tone modifier
    println!("   {:?} renders as {}      -> {} pieces: the two ends, and the emoji cut in two",
             modified, modified, modified.split("").count());

    println!();
    println!("6. The rest of the family, all on the empty pattern");
    println!("   split_terminator    {:?}", s.split_terminator("").collect::<Vec<&str>>());
    println!("   split_inclusive     {:?}", s.split_inclusive("").collect::<Vec<&str>>());
    println!("   rsplit              {:?}", s.rsplit("").collect::<Vec<&str>>());
    println!("   splitn(3, \"\")       {:?}   <- 3 pieces, and only ONE of them a character",
             s.splitn(3, "").collect::<Vec<&str>>());
    println!("   split_once(\"\")      {:?}   <- never None, for any string",
             s.split_once(""));
    println!("   rsplit_once(\"\")     {:?}   <- reading order, where rsplitn(2) gives {:?}",
             s.rsplit_once(""), s.rsplitn(2, "").collect::<Vec<&str>>());

    println!();
    println!("7. Each character as a &str, which is what split(\"\") is usually reached for");
    println!("   filtered            {:?}", s.split("").filter(|p| !p.is_empty()).collect::<Vec<&str>>());
    let sliced: Vec<&str> = u.char_indices().map(|(i, c)| &u[i..i + c.len_utf8()]).collect();
    println!("   from char_indices   {:?}   <- same idea, and it never made an empty", sliced);
}
```
<!-- /source -->

<!-- output:splitting_on_nothing -->
*Verified output of [`splitting_on_nothing.rs`](examples/splitting_on_nothing.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three characters, five pieces
   split("")           ["", "a", "b", "c", ""]
   chars()             ['a', 'b', 'c']

2. Where the five come from — the matches, at their byte offsets
   match_indices("")   [(0, ""), (1, ""), (2, ""), (3, "")]
   4 matches, 5 pieces — the same n+1 as any other pattern
   "xaxbxcx".split('x') ["", "a", "b", "c", ""]   <- four matches too

3. Even the empty string has one position in it
   "".split(',')       [""]   0 matches, 1 piece
   "".split("")        ["", ""]   1 match,  2 pieces

4. The positions are CHAR boundaries, not byte offsets
   "añb".len()         4 bytes, 3 chars
   char_indices()      [(0, 'a'), (1, 'ñ'), (3, 'b')]
   split("")           ["", "a", "ñ", "b", ""]
   byte 2 is inside 'ñ': is_char_boundary(2) = false, and no piece starts there

5. ...but a char boundary is not a LETTER boundary
   "a\u{301}" renders as á    -> ["", "a", "\u{301}", ""]
   "👍🏽" renders as 👍🏽      -> 4 pieces: the two ends, and the emoji cut in two

6. The rest of the family, all on the empty pattern
   split_terminator    ["", "a", "b", "c"]
   split_inclusive     ["", "a", "b", "c"]
   rsplit              ["", "c", "b", "a", ""]
   splitn(3, "")       ["", "a", "bc"]   <- 3 pieces, and only ONE of them a character
   split_once("")      Some(("", "abc"))   <- never None, for any string
   rsplit_once("")     Some(("abc", ""))   <- reading order, where rsplitn(2) gives ["", "abc"]

7. Each character as a &str, which is what split("") is usually reached for
   filtered            ["a", "b", "c"]
   from char_indices   ["a", "ñ", "b"]   <- same idea, and it never made an empty
```
<!-- /output -->

## Practice

**Three questions about nothing**, each answered by a program rather than from memory.

1. `"".split(',')` yields one piece and `"".split("")` yields two. Print both, then say which of the two counts is the odd one out — and defend the answer with the n+1 rule rather than with a special case.
2. `"żółw"` is four letters and seven bytes. Print the byte offsets `match_indices("")` reports, and the pieces `split("")` gives. Then compute the offsets that are **missing** from that list, and say what a piece starting at one of them would have to be.
3. Write `chars_as_strs(s: &str) -> Vec<&str>` returning every character as a borrowed slice — no `String`, no allocation per character, no empties to filter afterwards. Check it against `s.chars().count()` on `"żółw"`, on `"a\u{301}"` and on `""`, and say which of those three `split("")` alone gets wrong.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:splitting_on_nothing_kata -->
*[`splitting_on_nothing_kata.rs`](examples/splitting_on_nothing_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three questions about the empty pattern, each answered by a
//! program rather than from memory — the two counts an empty haystack gives,
//! the byte offsets a Polish word does NOT report, and every character as a
//! borrowed &str with no allocation and no empties.
//!
//! Run:  rustc --edition 2024 splitting_on_nothing_kata.rs && ./splitting_on_nothing_kata

/// Every character as a borrowed slice of `s`. No `String`, no allocation per
/// character, and no empty pieces to filter out afterwards — `char_indices`
/// gives the start, `len_utf8` gives the width, and the two make the range.
fn chars_as_strs(s: &str) -> Vec<&str> {
    s.char_indices().map(|(i, c)| &s[i..i + c.len_utf8()]).collect()
}

fn main() {
    println!("Q1. Two empty haystacks, two different counts");
    println!("    \"\".split(',')  -> {:?}   0 matches, 1 piece",
             "".split(',').collect::<Vec<&str>>());
    println!("    \"\".split(\"\")   -> {:?}   1 match,  2 pieces",
             "".split("").collect::<Vec<&str>>());
    println!("    Neither is the odd one out: both are n+1. The empty pattern");
    println!("    matches once, because even an empty string has one position.");

    println!();
    println!("Q2. Four letters, seven bytes, five positions");
    let w = "żółw";
    println!("    len() = {} bytes, chars().count() = {}", w.len(), w.chars().count());
    println!("    match_indices(\"\") -> {:?}",
             w.match_indices("").map(|(i, _)| i).collect::<Vec<usize>>());
    println!("    split(\"\")         -> {:?}  ({} pieces)",
             w.split("").collect::<Vec<&str>>(), w.split("").count());
    let inside: Vec<usize> = (0..=w.len()).filter(|&i| !w.is_char_boundary(i)).collect();
    println!("    missing: {:?} — each one is inside a two-byte letter, and a", inside);
    println!("    piece starting there would not be UTF-8, so it cannot be a &str.");

    println!();
    println!("Q3. Every character as a &str");
    for s in ["żółw", "a\u{301}", ""] {
        let mine = chars_as_strs(s);
        let naive = s.split("").collect::<Vec<&str>>();
        println!("    {:<12} chars_as_strs -> {:<26} split(\"\") -> {:?}",
                 format!("{s:?}"), format!("{mine:?}"), naive);
        assert_eq!(mine.len(), s.chars().count());
    }
    println!("    split(\"\") alone is wrong on all three: it adds an empty at each");
    println!("    end, so it answers 2 for the empty string, which has no characters.");
}
```
<!-- /source -->

<!-- output:splitting_on_nothing_kata -->
*Verified output of [`splitting_on_nothing_kata.rs`](examples/splitting_on_nothing_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Q1. Two empty haystacks, two different counts
    "".split(',')  -> [""]   0 matches, 1 piece
    "".split("")   -> ["", ""]   1 match,  2 pieces
    Neither is the odd one out: both are n+1. The empty pattern
    matches once, because even an empty string has one position.

Q2. Four letters, seven bytes, five positions
    len() = 7 bytes, chars().count() = 4
    match_indices("") -> [0, 2, 4, 6, 7]
    split("")         -> ["", "ż", "ó", "ł", "w", ""]  (6 pieces)
    missing: [1, 3, 5] — each one is inside a two-byte letter, and a
    piece starting there would not be UTF-8, so it cannot be a &str.

Q3. Every character as a &str
    "żółw"       chars_as_strs -> ["ż", "ó", "ł", "w"]       split("") -> ["", "ż", "ó", "ł", "w", ""]
    "a\u{301}"   chars_as_strs -> ["a", "\u{301}"]           split("") -> ["", "a", "\u{301}", ""]
    ""           chars_as_strs -> []                         split("") -> ["", ""]
    split("") alone is wrong on all three: it adds an empty at each
    end, so it answers 2 for the empty string, which has no characters.
```
<!-- /output -->

</details>

## See also

- [`str::split`](../str_methods/str_split/README.md) — the reference page: the four pattern shapes, and the n+1 rule in full
- [`str::match_indices`](../str_methods/str_match_indices/README.md) · [`str::matches`](../str_methods/str_matches/README.md) — the matches rather than the gaps, which is how the four gets counted
- [Walking a `String`](../walking_a_string/README.md) — where the arithmetic is set up, and the whole split family in one table
- [Inside a `Split`](../inside_a_split/README.md) — the plan `split` hands back, field by field; the empty pattern is the fifth searcher
- [Meet the `char`](../meet_the_char/README.md) — why two `char`s can be one letter, and the crate that counts letters
- [Replacing part of a string](../replacing_in_a_string/README.md) — `"ab".replace("", "-")` is `"-a-b-"`: the same empty match at every boundary, doing something more obviously useful
- [`str::is_char_boundary`](../str_methods/str_is_char_boundary/README.md) — the positions an empty pattern is allowed to land on
- [String slices](../string_slices/README.md) — what happens when a byte index is *not* one of them
- [`str::split` in the standard library ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split) — where the empty-separator sentence lives
- [Splitting a string by empty string yields empty strings ↗](https://users.rust-lang.org/t/splitting-a-string-by-empty-string-yields-empty-strings/77702) — the users forum, June 2022: 01mf02's question, simonbuchan's should-it-panic objection, BurntSushi's non-overlapping answer, and steffahn's arrow picture. A forum thread, so weigh it as one; the answers are right and the page above adds the char-boundary half they do not reach.

## Po polsku

`"abc".split("")` zwraca **pięć** kawałków, a nie trzy: `["", "a", "b", "c", ""]`. To nie jest złośliwość metody ani błąd — `split` zgłasza **przerwy między dopasowaniami**, więc *n* dopasowań to zawsze *n+1* kawałków, a pusty wzorzec dopasowuje się na **każdej granicy znaku**. W słowie `"abc"` takich pozycji są cztery (przed `a`, między literami, po `c`), stąd cztery dopasowania i pięć przerw. Najkrótszy sposób, żeby to zapamiętać: `"abc".split("")` zachowuje się dokładnie tak jak `"xaxbxcx".split('x')`. Dopasowania nie nachodzą na siebie, więc pustych ciągów nie ma nieskończenie wiele — wyszukiwarka bierze jedno dopasowanie i przechodzi dalej.

Polskiemu czytelnikowi ta strona przydaje się bardziej niż angielskiemu, bo druga połowa dotyczy dokładnie naszych liter. Granice, na których ląduje pusty wzorzec, to **granice znaków**, a nie bajtów. `"żółw"` ma 7 bajtów i 4 litery, więc `match_indices("")` zgłasza offsety `[0, 2, 4, 6, 7]`, a `split("")` daje sześć kawałków: `["", "ż", "ó", "ł", "w", ""]`. Offsetów 1, 3 i 5 na tej liście nie ma i być nie może — każdy z nich siedzi w środku dwubajtowej litery, a kawałek zaczynający się tam nie byłby poprawnym UTF-8, czyli nie mógłby być typu `&str`. To ten sam mechanizm, który sprawia, że `&s[0..1]` na słowie „żółw” panikuje.

Uwaga na skrót myślowy w drugą stronę: granica znaku to nie granica **litery**. `"a\u{301}"` (litera `a` plus łącząca kreska) wygląda na ekranie jak jedno `á`, ale to dwa `char`y, więc `split("")` przetnie je na pół. Tego std nie liczy w ogóle — od grafemów jest crate [`unicode-segmentation` ↗](https://crates.io/crates/unicode-segmentation).

I porównanie, które tłumaczy, skąd zaskoczenie: w Pythonie `"abc".split("")` w ogóle nie działa (`ValueError: empty separator`, a znaki bierze się przez `list("abc")`), w JavaScripcie i w Go ten przypadek jest **obsłużony osobnym wyjątkiem w regule** i daje `["a","b","c"]` bez pustych końców. Rust jako jedyny nie robi tu żadnego wyjątku — i dlatego jego odpowiedź wygląda dziwnie, a jest jedyną, która trzyma się własnej arytmetyki.

**Szukaj po polsku:** dzielenie łańcucha po pustym wzorcu · puste kawałki po `split` · granica znaku a granica bajtu · `rust split empty string` · `rust split("") extra empty strings`
