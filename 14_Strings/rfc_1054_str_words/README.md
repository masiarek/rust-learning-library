# RFC 1054 — the method that renamed itself to promise less

**Level:** 201 · working knowledge

**One line:** `str::words` became `str::split_whitespace` because nobody could say what a word is — and the new name is the whole design: it describes the mechanism instead of claiming a result.

```rust
let messy = "  the quick\tbrown\nfox  ";
println!("{:?}", messy.split_whitespace().collect::<Vec<&str>>());
// ["the", "quick", "brown", "fox"]
```

Two things about that method are decisions rather than facts, and both were taken in one two-page RFC in April 2015. It is a *method* rather than a pattern you pass to `split`. And it is called `split_whitespace` rather than `words`, because Rust's standard library concluded it should not ship an answer to "what is a word".

---

## What it asked for

Simon Sapin filed [RFC 1054 ↗](https://github.com/rust-lang/rfcs/blob/master/text/1054-str-words.md) on 2015-04-10 — the author of [RFC 69](../rfc_69_byte_literals/README.md), and the person whose reading of the Unicode standard settled the [RFC 1212](../rfc_1212_line_endings/README.md) thread four months later. Those three interventions — two RFCs and one decisive comment — are the same instinct applied to strings: find the question std is quietly answering, and ask whether it should be.

`str::words` existed and was `#[unstable]`, with the reason written into the attribute: *"the precise algorithm to use is unclear"*. [rust#15628 ↗](https://github.com/rust-lang/rust/issues/15628) proposed fixing that by implementing [UAX #29 ↗](https://www.unicode.org/reports/tr29/#Word_Boundaries), Unicode's word-boundary algorithm — the issue the implementing PR eventually closed, by doing the opposite. The RFC argued the opposite — that such an implementation belongs on crates.io, for two reasons:

- it carries complexity that would be **surprising from something that looks as simple as a parameter-less `words` method**, and
- it is not a final answer anyway. The RFC quotes the standard against itself: UAX #29 says it is not possible to give one set of rules that resolves all issues across languages, and calls its own algorithm a workable default.

So the proposal is a retreat, and the rename is what makes the retreat legible. `split_whitespace` cannot be wrong about words, because it never mentions them.

## The argument, in one line of Japanese

<!-- source:rfc_1054_str_words -->
*[`rfc_1054_str_words.rs`](examples/rfc_1054_str_words.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
fn parts(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn main() {
    let messy = "  the quick\tbrown\nfox  ";

    println!("WHAT SHIPPED IS THE RFC'S FORMULA, VERBATIM");
    let rfc = messy.split(char::is_whitespace).filter(|s| !s.is_empty());
    println!("  split_whitespace()            {:?}", parts(messy));
    println!("  split(is_whitespace).filter() {:?}", rfc.clone().collect::<Vec<&str>>());
    println!("  identical: {}", messy.split_whitespace().eq(rfc));

    println!();
    println!("WHY IT IS A METHOD AND NOT A PATTERN (lilyball's objection)");
    println!("  \" a b \".split(is_whitespace)  {:?}", " a b ".split(char::is_whitespace).collect::<Vec<&str>>());
    println!("  \" a b \".split_whitespace()    {:?}", parts(" a b "));
    println!("  a Pattern cannot drop the leading and trailing empties;");
    println!("  the .filter() is what the method exists to carry.");

    println!();
    println!("THE ANSWER EVERYONE REACHES FOR FIRST");
    println!("  split(' ')  {:?}", messy.split(' ').collect::<Vec<&str>>());
    println!("  it neither collapses runs nor knows a tab is whitespace.");

    println!();
    println!("THE ARGUMENT THE RFC WON: \"A WORD\" HAS NO PORTABLE DEFINITION");
    let ja = "私は学生です";
    println!("  {ja}  (\"I am a student\") has no spaces in it at all,");
    println!("  so split_whitespace finds {} piece for its {} chars,",
             ja.split_whitespace().count(), ja.chars().count());
    println!("  where a Japanese reader finds several words.");
    println!("  punctuation rides along too: {:?}", parts("don't stop, e.g. now"));
    println!("  UAX #29 would answer all of this. std declined to ship an answer.");

    println!();
    println!("...AND \"WHITESPACE\" TURNS OUT TO BE AMBIGUOUS TOO");
    println!("  {:<16}{:<15}{:<18}{:<16}{}", "char", "is_whitespace", "split_whitespace", "split_ascii_ws", "lines");
    for (name, c) in [
        ("SPACE  U+0020", ' '),      ("TAB    U+0009", '\t'),
        ("VT     U+000B", '\u{0B}'), ("FF     U+000C", '\u{0C}'),
        ("NBSP   U+00A0", '\u{A0}'), ("NEL    U+0085", '\u{85}'),
        ("OGHAM  U+1680", '\u{1680}'), ("LS     U+2028", '\u{2028}'),
        ("PS     U+2029", '\u{2029}'), ("ZWSP   U+200B", '\u{200B}'),
        ("FS     U+001C", '\u{1C}'),
    ] {
        let s = format!("a{c}b");
        println!(
            "  {:<16}{:<15}{:<18}{:<16}{}",
            name,
            c.is_whitespace(),
            s.split_whitespace().count(),
            s.split_ascii_whitespace().count(),
            s.lines().count(),
        );
    }
    println!("  VT is ASCII and IS whitespace, yet split_ascii_whitespace skips it,");
    println!("  because is_ascii_whitespace is the WhatWG set, minus U+000B.");
    println!("  ZWSP is called a space and is not one (White_Space=no).");
    println!("  FS is not whitespace here; Python's split() DOES split on it.");
    println!("  LS and PS split words here but are NOT line endings -- see RFC 1212.");
}
```
<!-- /source -->

<!-- output:rfc_1054_str_words -->
*Verified output of [`rfc_1054_str_words.rs`](examples/rfc_1054_str_words.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
WHAT SHIPPED IS THE RFC'S FORMULA, VERBATIM
  split_whitespace()            ["the", "quick", "brown", "fox"]
  split(is_whitespace).filter() ["the", "quick", "brown", "fox"]
  identical: true

WHY IT IS A METHOD AND NOT A PATTERN (lilyball's objection)
  " a b ".split(is_whitespace)  ["", "a", "b", ""]
  " a b ".split_whitespace()    ["a", "b"]
  a Pattern cannot drop the leading and trailing empties;
  the .filter() is what the method exists to carry.

THE ANSWER EVERYONE REACHES FOR FIRST
  split(' ')  ["", "", "the", "quick\tbrown\nfox", "", ""]
  it neither collapses runs nor knows a tab is whitespace.

THE ARGUMENT THE RFC WON: "A WORD" HAS NO PORTABLE DEFINITION
  私は学生です  ("I am a student") has no spaces in it at all,
  so split_whitespace finds 1 piece for its 6 chars,
  where a Japanese reader finds several words.
  punctuation rides along too: ["don't", "stop,", "e.g.", "now"]
  UAX #29 would answer all of this. std declined to ship an answer.

...AND "WHITESPACE" TURNS OUT TO BE AMBIGUOUS TOO
  char            is_whitespace  split_whitespace  split_ascii_ws  lines
  SPACE  U+0020   true           2                 2               1
  TAB    U+0009   true           2                 2               1
  VT     U+000B   true           2                 1               1
  FF     U+000C   true           2                 2               1
  NBSP   U+00A0   true           2                 1               1
  NEL    U+0085   true           2                 1               1
  OGHAM  U+1680   true           2                 1               1
  LS     U+2028   true           2                 1               1
  PS     U+2029   true           2                 1               1
  ZWSP   U+200B   false          1                 1               1
  FS     U+001C   false          1                 1               1
  VT is ASCII and IS whitespace, yet split_ascii_whitespace skips it,
  because is_ascii_whitespace is the WhatWG set, minus U+000B.
  ZWSP is called a space and is not one (White_Space=no).
  FS is not whitespace here; Python's split() DOES split on it.
  LS and PS split words here but are NOT line endings -- see RFC 1212.
```
<!-- /output -->

A sentence in a language that does not put spaces between words comes back as **one piece**. That is not a bug in `split_whitespace` — it is the method doing exactly what its name says, on input where whitespace and words have nothing to do with each other. A method called `words()` returning one element there would be simply wrong, and that difference is the entire RFC.

The English row is milder and more common: `"don't stop, e.g. now"` gives four pieces, one of which is `"stop,"`. Whitespace splitting keeps punctuation attached to whichever token it touched. UAX #29 has rules for all of this. `std` has none, on purpose.

## Why it is a method and not a pattern

The RFC's own *Drawbacks* section concedes the awkward part: `split_whitespace` sits right next to the general `split<P: Pattern>(&self, P)`, and adding a special method for one pattern looks like weak API design. huonw made the obvious counter-proposal in the thread — a `Whitespaces` pattern you would pass to `split`, ideally reachable as `str::Whitespaces` so it needs no import.

lilyball's reply is the reason it did not happen, and it is a property of the data rather than a matter of taste: a pattern splits, and splitting `" a b "` on whitespace yields a leading and a trailing empty piece. The run above shows both. `split_whitespace` is `split(char::is_whitespace).filter(|s| !s.is_empty())` — and **the `.filter` is the part a pattern cannot carry**, because by the time `split` has a pattern it has already decided to emit a piece between every pair of matches.

That is still true, and so is the other half of the objection. The `Pattern` trait was unstable in 2015 and is unstable now:

```text title="Abridged — real rustc output for whitespaces.rs, second error dropped"
error[E0658]: use of unstable library feature `pattern`: API not fully fleshed out and ready to be stabilized
 --> whitespaces.rs:3:6
  |
3 | impl std::str::pattern::Pattern for Whitespaces {}
  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #27721 <https://github.com/rust-lang/rust/issues/27721> for more information
```

A decade on, the alternative the RFC listed still cannot be written by anyone outside `core`. The inherent method was the ergonomic choice in 2015 and is the only choice today.

## The author argued against their own RFC

Two days after filing it, Simon Sapin wrote in the thread: *"I'm becoming less and less convinced that this should be included at all. I'd like to see use cases."* If "a word" is too ambiguous for std, the reasoning goes, is "whitespace" not ambiguous in the same way — and should you not be looking for word boundaries anyway?

What kept the method was ordinary, and worth reading as a counterweight to the theory. lilyball had wanted exactly this two days earlier, forgot `words()` existed, and wrote `s.split(" ")` instead — the wrong answer, in code that shipped. nagisa observed that people who reach for whitespace splitting reliably reach for the ASCII space alone. tshepang put the case in one line: `s.split_whitespace()` is easier on the eyes than the formula it replaces. Against that, tafia argued std should stay small and let people find the one-liner by searching for it.

The line the merge landed on is the useful one: **whitespace splitting is a common enough thing to want that std should make it easy, and "words" is a different problem std should not touch at all.** Those are not in tension once the name stops overstating.

## And "whitespace" turns out to be ambiguous too

Simon Sapin's objection to their own proposal has an answer, and it is not a comfortable one: **std ships two different definitions of whitespace, and they disagree on an ASCII character.**

The run above puts the ten interesting characters side by side. `split_whitespace` uses `char::is_whitespace`, which is Unicode's `White_Space` property; `split_ascii_whitespace` — added much later, in 1.34 — uses the [WhatWG Infra ↗](https://infra.spec.whatwg.org/#ascii-whitespace) set, which is `\t`, `\n`, `\x0C`, `\r` and space. **U+000B VERTICAL TAB is ASCII, and is whitespace, and is not ASCII whitespace.** std documents the trap in as many words: `c.is_ascii_whitespace()` is not equivalent to `c.is_ascii() && c.is_whitespace()`.

std's own note on that method is worth reading in full, because it is RFC 1054's thesis restated for a smaller word: it points out that POSIX includes VERTICAL TAB, that the Bourne shell's field splitting — *in the same specification* — counts only space, tab and line feed, and that you should go and check what your file format means before trusting either. Three published definitions of "ASCII whitespace" and no default that is right for all of them.

Two more rows earn their place:

- **ZWSP U+200B is called ZERO WIDTH SPACE and is not whitespace** (`White_Space=no`), so nothing in this family splits on it. It is a line-break *opportunity*, not a separator — which is why text pasted from the web can come back with two visible words fused into one piece.
- **LS U+2028 and PS U+2029 split words but not lines.** That is the seam [RFC 1212](../rfc_1212_line_endings/README.md) left when its thread argued about Unicode line separators and decided against them. Read from this side it is sharper: `split_whitespace` was already Unicode-aware when `lines()` chose not to be, so the two methods have disagreed about those two characters since 2015.

## What actually shipped, and when

[rust#24563 ↗](https://github.com/rust-lang/rust/pull/24563) landed five days after the RFC merged, written by kwantam from the thread. All of this happened **before 1.0**: the PR merged on 2015-04-22 and Rust 1.0.0 shipped three weeks later. Because `words()` was `#[unstable]`, it had never been reachable from stable Rust at all, so the rename cost its users nothing — it was left in as a deprecated wrapper and `Words` became a type alias for `SplitWhitespace`, cushioning nightly only. `split_whitespace` itself was still unstable at the 1.0 cut and became stable in **1.1.0**. Today `words` is gone from `std` entirely; there is nothing to trip over.

One detail from the thread is a good reminder that stability attributes are hand-written: kwantam noticed that while `words()` was `#[unstable]`, the `Words` **struct** it returned was marked `#[stable]`. alexcrichton's answer was that this was simply a mistake and the marking could be reverted. An unstable method whose return type was stable is not a state anyone designed; it is what happens when two attributes are maintained separately.

## If you are coming from another language

**Python.** Python had this method the whole time, and the thread did not notice. `str.split()` **with no argument** is `split_whitespace` — it splits on runs of Unicode whitespace and drops the empty pieces — while `str.split(" ")` is Rust's `split(' ')`, empties and all. nagisa's comment in the thread says Python provides no proper convenience wrapper; run both languages over the first ten rows of the table above and they agree on every one, including that ZWSP is not whitespace.

The last row is where they part company. Python's `split()` follows `str.isspace()`, which counts the four ASCII separator controls U+001C–U+001F; Rust's `White_Space` property does not, so `"a\u{1C}b"` is two pieces in Python and one in Rust.

| Python | | Rust |
|---|---|---|
| `s.split()` | no argument: runs of whitespace, empties dropped | [`split_whitespace()`](../str_methods/str_split_whitespace/README.md) — same set, same result |
| `s.split(" ")` | one literal space; keeps every empty | [`split(' ')`](../str_methods/str_split/README.md) — identical |
| `str.isspace()` | Unicode, plus `\x1c`–`\x1f` | `char::is_whitespace` — Unicode `White_Space` only |
| no ASCII-only variant | you write `s.split(None)` and accept Unicode | [`split_ascii_whitespace()`](../str_methods/str_split_ascii_whitespace/README.md) — five characters, faster |
| `re.findall(r"\w+", s)` | the usual "words" answer, and `\w` is not UAX #29 either | no `std` answer at all; `regex` or `unicode-segmentation` on crates.io |

The last row is where the two libraries actually differ in philosophy. Python's answer to "give me the words" is a regex in the standard library, which is a *different* wrong definition of a word rather than no definition — `\w+` splits `don't` into `don` and `t`. Rust's standard library has no regex engine, so RFC 1054's retreat cost it nothing it had.

**ABAP.** `SPLIT text AT space INTO TABLE lt_words` is the direct equivalent of Rust's `split(' ')`, with the same two problems: a run of spaces produces empty rows, and a tab is not a space. `CONDENSE text` first collapses runs and trims the ends, which is the closest ABAP gets to `split_whitespace` — two statements where Rust has one method, and still ASCII-space-only. ABAP has no character-class predicate answering to `char::is_whitespace`: `cl_abap_char_utilities` gives you named constants (`horizontal_tab`, `cr_lf`, `newline`) that you assemble into a set by hand, or you reach for `FIND REGEX` and inherit whatever that engine means by `\s` — the same ambiguity, one layer down and undocumented at the call site. That is the position Rust deliberately left crates.io in for *words*, and deliberately did not leave anyone in for *whitespace*.

## See also

- [`str::split_whitespace`](../str_methods/str_split_whitespace/README.md) — the reference page: signature, edge cases, and why it must never touch delimited data
- [`str::split_ascii_whitespace`](../str_methods/str_split_ascii_whitespace/README.md) — the narrower, faster set, and the VERTICAL TAB it drops
- [`str::split`](../str_methods/str_split/README.md) — the general form the RFC's *Drawbacks* section worried about competing with
- [Walking a `String`](../walking_a_string/README.md) — the split family in one place, and the silently-shortened row this method causes on CSV
- [RFC 1212 — how `lines()` learned about `\r\n`](../rfc_1212_line_endings/README.md) — the other side of the U+2028 seam, same author in the thread
- [RFC 69 — how Rust got `b'A'`](../rfc_69_byte_literals/README.md) — the third RFC this library reads line by line, also Simon Sapin's
- [RFC 1054 ↗](https://github.com/rust-lang/rfcs/blob/master/text/1054-str-words.md) — two pages, and the *Motivation* is the part to read
- [rust-lang/rfcs#1054 ↗](https://github.com/rust-lang/rfcs/pull/1054) — the thread, including the author arguing against it
- [rust#24563 ↗](https://github.com/rust-lang/rust/pull/24563) — the implementation, five days later
- [UAX #29 ↗](https://www.unicode.org/reports/tr29/#Word_Boundaries) — the algorithm `std` declined to ship

## Po polsku

RFC 1054 to zmiana nazwy: metoda `str::words` została przemianowana na `str::split_whitespace`, a zachowanie zostało bez zmian. Wbrew pozorom to nie kosmetyka — nowa nazwa **obiecuje mniej**, i o to chodziło. „Słowo" nie ma przenośnej definicji, więc biblioteka standardowa Rusta postanowiła w ogóle na to pytanie nie odpowiadać: `split_whitespace` nie może się pomylić co do słów, bo o słowach nie mówi. Pełny algorytm (UAX #29) trafił do osobnych bibliotek na crates.io.

Najlepiej widać to na zdaniu w języku, który nie stawia spacji między wyrazami — japońskie `私は学生です` daje **jeden** element, choć czytelnik widzi tam kilka słów. Po polsku problem jest łagodniejszy, ale ten sam: `"nie mów, np. teraz"` daje cztery kawałki, a jeden z nich to `"mów,"` — interpunkcja zostaje przyklejona do wyrazu.

Dwie rzeczy warte zapamiętania przy pracy z polskim tekstem. **Spacja nierozdzielająca** (U+00A0, w Wordzie i w LaTeX-u wstawiana po spójnikach `i`, `w`, `z`) **jest** białym znakiem dla `split_whitespace`, ale **nie jest** dla `split_ascii_whitespace` — to realna różnica na tekście przeklejonym z edytora. I odwrotnie: `ZWSP` (U+200B), mimo nazwy „spacja o zerowej szerokości", białym znakiem **nie jest** wcale, więc tekst skopiowany ze strony WWW potrafi skleić dwa wyrazy w jeden kawałek. Nawiasem: `split_ascii_whitespace` pomija też tabulator pionowy U+000B, który jest znakiem ASCII i jest białym znakiem — biblioteka standardowa ostrzega o tym wprost.

**Szukaj po polsku:** podział na wyrazy · białe znaki · spacja nierozdzielająca · granice wyrazów Unicode · `rust split_whitespace vs split` · `rust words method renamed`
