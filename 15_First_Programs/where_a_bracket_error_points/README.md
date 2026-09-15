# Where a bracket error points

**Level:** 101 → 201 · for newcomers

**One line:** `unexpected closing delimiter` puts its `-->` line where rustc ran out of brackets to pair, not where you typed the extra one — the bracket to delete sits under a `-` label higher up, and until it is gone that error is the only one rustc prints for the file.

A *delimiter*, in rustc's messages, is any of [the three kinds of bracket ↗](https://doc.rust-lang.org/reference/tokens.html#delimiters): `( )`, `[ ]` and `{ }`. Angle brackets are not among them, so the `<` in `Vec<&str>` takes no part.

---

## The fix: delete the bracket the `missing open` label points at

```rust
fn main() {
    let s = "aaa".to_string(); // one `)` after `to_string(`, not two
    println!("{s}");           // aaa
}
```

The broken version had `to_string())` on line 2 and `println!(s)` on line 3. rustc reported only the first, and put it on line 4.

## The line number is not the line with the typo

```text title="Real rustc output for refusals/extra_paren.rs, edition 2024, rustc 1.98.0"
error: unexpected closing delimiter: `}`
 --> extra_paren.rs:4:1
  |
1 | fn main() {
  |           - the nearest open delimiter
2 |     let s = "aaa".to_string());
  |                              - missing open `(` for this delimiter
3 |     println!(s)
4 | }
  | ^ unexpected closing delimiter

error: aborting due to 1 previous error
```

[`refusals/extra_paren.rs`](refusals/extra_paren.rs) is those four lines. Three marks point at three brackets:

| mark | points at | says |
|---|---|---|
| `-->` and `^` | the `}` on line 4 that ends `main` | *unexpected closing delimiter* — and this `}` is correct |
| `-` on line 1 | `main`'s `{` | *the nearest open delimiter* |
| `-` on line 2 | the second `)` | *missing open `(` for this delimiter* — **the typo** |

Read the labels in order and they explain the headline. The second `)` needs an open `(`, and none is open: the only bracket still open is `main`'s `{`, *the nearest open delimiter*. So that `)` closes the `{`, `main`'s body ends on line 2 as far as pairing is concerned, and the `}` on line 4 arrives with nothing left to close. That `}` is the first bracket with no partner at all, and the headline is about it.

## The further up the typo, the further away the line number

```text title="Real rustc output for refusals/far_away.rs, edition 2024, rustc 1.98.0"
error: unexpected closing delimiter: `}`
  --> far_away.rs:12:1
   |
 1 | fn count_long_words(text: &str) -> usize {
   |                                          - the nearest open delimiter
 2 |     let words: Vec<&str> = text.split_whitespace().collect();
 3 |     let long = words.iter().filter(|w| w.len() > 4).count());
   |                                                            - missing open `(` for this delimiter
...
12 | }
   | ^ unexpected closing delimiter

error: aborting due to 1 previous error
```

The typo is on line 3 and `-->` says line 12. The stray `)` closes the function's `{` again; the `if` and `else` braces on lines 6 to 10 still pair with each other; and the first bracket with nothing left to close is the `}` that ends the function. The `...` is rustc leaving out lines 4 to 11.

So for this message, skip the `-->` line and look up for the `-` that says `missing open`.

## Three messages, three places to look

| rustc says | what went wrong | `-->` points at | what to fix |
|---|---|---|---|
| `unexpected closing delimiter` | one closer too many | the first later closer with nothing left to close — here, the end of the function | the bracket labelled *missing open … for this delimiter*: delete it |
| `mismatched closing delimiter` | a closer missing, or the wrong kind of closer | the opener that was never closed | the `^` labelled *unclosed delimiter*: give it its partner |
| `this file contains an unclosed delimiter` | a closer missing, with nothing later of the wrong kind | the end of the file | the `-` labelled *unclosed delimiter* — and, when rustc adds one, the `{` it says *might not be properly closed* |

A missing `)`. Here `-->` is right: the `(` on line 2 has no partner, and the `}` on line 4 is labelled only because it was the next closer to arrive while that `(` was open.

```text title="Real rustc output for refusals/missing_paren.rs, edition 2024, rustc 1.98.0"
error: mismatched closing delimiter: `}`
 --> missing_paren.rs:2:21
  |
1 | fn main() {
  |           - closing delimiter possibly meant for this
2 |     let n = u32::max(3, 7;
  |                     ^ unclosed delimiter
3 |     println!("{n}");
4 | }
  | ^ mismatched closing delimiter

error: aborting due to 1 previous error
```

The wrong kind of closer — `vec![` closed with `)`. Both ends of the pair are marked, on one line:

```text title="Real rustc output for refusals/wrong_kind.rs, edition 2024, rustc 1.98.0"
error: mismatched closing delimiter: `)`
 --> wrong_kind.rs:2:17
  |
2 |     let v = vec![1, 2, 3);
  |                 ^       ^ mismatched closing delimiter
  |                 |
  |                 unclosed delimiter

error: aborting due to 1 previous error
```

A missing last `}`. The missing character has no position, so `-->` is the end of the file and the `-` marks the `{` whose partner never came:

```text title="Real rustc output for refusals/missing_brace.rs, edition 2024, rustc 1.98.0"
error: this file contains an unclosed delimiter
 --> missing_brace.rs:3:22
  |
1 | fn main() {
  |           - unclosed delimiter
2 |     let s = "aaa".to_string();
3 |     println!("{s}");
  |                     ^

error: aborting due to 1 previous error
```

A `}` missing from the middle of a file needs one more clue, because every later `}` then pairs one level off and the file runs out of `}` only at its end. Here `get` has lost its `}`:

```text title="Real rustc output for refusals/missing_inner_brace.rs, edition 2024, rustc 1.98.0"
error: this file contains an unclosed delimiter
  --> missing_inner_brace.rs:23:3
   |
 5 | impl Counter {
   |              - unclosed delimiter
 6 |     fn get(&self) -> u32 {
   |                          - this delimiter might not be properly closed...
...
16 | }
   | - ...as it matches this but it has different indentation
...
23 | }
   |  ^

error: aborting due to 1 previous error
```

The clue is indentation. By count, the `}` on line 16 is `get`'s; by indentation it is `impl`'s, and rustc says so — *this delimiter might not be properly closed…* on line 6, *…as it matches this but it has different indentation* on line 16. The missing `}` belongs after line 7. The model [below](#the-whole-thing-running) has no rule like that: it is something rustc adds on top of pairing.

`unexpected` is the odd one out. A missing closer leaves an opener behind, and an opener has a position to point at. An extra closer leaves nothing behind where it was typed — it closed something that was open — so that spot only shows up as a label.

## The label is not asking you to add a `(`

*missing open `(` for this delimiter* names the bracket that would give the stray `)` a partner. Adding one does make the brackets pair, and rustc's next run asks you to take it out:

```text title="Abridged — real rustc output after adding a ( to line 2 of extra_paren.rs, edition 2024, rustc 1.98.0"
warning: unnecessary parentheses around assigned value
 --> extra_paren.rs:2:13
  |
2 |     let s = ("aaa".to_string());
  |             ^                 ^
  |
  = note: `#[warn(unused_parens)]` (part of `#[warn(unused)]`) on by default
help: remove these parentheses
  |
2 -     let s = ("aaa".to_string());
2 +     let s = "aaa".to_string();
  |
```

Pairing brackets can find that a `)` has no partner. It cannot find where the partner was meant to be, or whether there was meant to be one. You can: delete the `)`.

## One bracket hides every other error

Three mistakes, in three functions:

```rust,compile_fail
fn one_bracket_too_many() {
    let s = "aaa".to_string());
}

fn wrong_type() {
    let n: i32 = "five";
}

fn wrong_format() {
    let s = "x";
    println!(s);
}

fn main() {}
```

rustc prints one of them:

```text title="Real rustc output for refusals/hides_others.rs, edition 2024, rustc 1.98.0"
error: unexpected closing delimiter: `}`
 --> hides_others.rs:3:1
  |
1 | fn one_bracket_too_many() {
  |                           - the nearest open delimiter
2 |     let s = "aaa".to_string());
  |                              - missing open `(` for this delimiter
3 | }
  | ^ unexpected closing delimiter

error: aborting due to 1 previous error
```

Delete that one `)` and the same file has two errors and a warning. The `unused variable` is the `s` on line 2, which was just as unused in the first run:

```text title="Abridged, headline lines only — real rustc output for refusals/hides_others.rs after deleting that ), edition 2024, rustc 1.98.0"
error: format argument must be a string literal
error[E0308]: mismatched types
warning: unused variable: `s`
error: aborting due to 2 previous errors; 1 warning emitted
```

So `1 previous error` under a bracket message counts what rustc got to, not what is wrong. `extra_paren.rs` was the same: delete its `)` and line 3 is next.

```text title="Real rustc output for refusals/extra_paren.rs after deleting that ), edition 2024, rustc 1.98.0"
error: format argument must be a string literal
 --> extra_paren.rs:3:14
  |
3 |     println!(s)
  |              ^
  |
help: you might be missing a string literal to format with
  |
3 |     println!("{}", s)
  |              +++++

error: aborting due to 1 previous error
```

`println!` reads its first argument while your program compiles, so it has to be a string literal — [the braces take a name](../braces_take_a_name/README.md#the-format-string-itself-must-be-a-literal) has the rest.

## The whole thing, running

A model of the pairing, in one function. It is not rustc's code; it is the rules rustc's labels describe:

- an opening bracket waits for its partner;
- a closer whose partner is open further out ends the check: what is open inside that partner was never closed, and the message is `mismatched`;
- a closer with no partner open anywhere closes the nearest opener anyway, and the check carries on — until a later closer finds nothing open, and the message is `unexpected`.

Brackets inside a string literal are skipped, since `"{n}"` is one token to rustc. The program reads the five refusal files with `include_str!`, so it looks at the same bytes rustc did, and for each one it names the same brackets at the same line and column as the transcripts above. The one place it differs is the end of `missing_brace.rs`, which it calls `end of file` where rustc prints `3:22`.

<!-- source:where_a_bracket_error_points -->
*[`where_a_bracket_error_points.rs`](examples/where_a_bracket_error_points.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Where a bracket error points.
//!
//! rustc pairs every `(`, `[` and `{` in a file with its closing partner. This
//! is a MODEL of that pairing, not rustc's code: the smallest set of rules that
//! prints the same message, naming the same brackets at the same line:column,
//! as rustc 1.98.0 does for the five files in `../refusals/`. They are read
//! with `include_str!`, so the model sees exactly the bytes rustc saw.
//!
//!   rustc --edition 2024 where_a_bracket_error_points.rs -o /tmp/wabep && /tmp/wabep

const FILES: [(&str, &str); 5] = [
    ("extra_paren.rs", include_str!("../refusals/extra_paren.rs")),
    ("far_away.rs", include_str!("../refusals/far_away.rs")),
    ("missing_paren.rs", include_str!("../refusals/missing_paren.rs")),
    ("wrong_kind.rs", include_str!("../refusals/wrong_kind.rs")),
    ("missing_brace.rs", include_str!("../refusals/missing_brace.rs")),
];

/// One bracket and where it sits. Line and column both start at 1, as in rustc.
#[derive(Clone, Copy)]
struct Bracket {
    ch: char,
    line: usize,
    col: usize,
}

enum Verdict {
    /// Every bracket found its partner.
    Balanced,
    /// A closer arrived with nothing open at all. `stray` is the earlier closer
    /// that took the nearest opener, because nothing open was its kind.
    Unexpected {
        closer: Bracket,
        stray: Option<(Bracket, Bracket)>,
    },
    /// A closer met an opener of another kind.
    Mismatched {
        closer: Bracket,
        unclosed: Bracket,
        meant_for: Option<Bracket>,
    },
    /// The file ended with an opener still waiting.
    Unclosed { opener: Bracket },
}

fn partner(close: char) -> char {
    match close {
        ')' => '(',
        ']' => '[',
        _ => '{',
    }
}

fn check(src: &str) -> Verdict {
    let mut open: Vec<Bracket> = Vec::new();
    let mut stray: Option<(Bracket, Bracket)> = None; // (the opener it took, the closer)
    let mut in_string = false;
    let mut escaped = false;

    for (i, text) in src.lines().enumerate() {
        for (j, ch) in text.chars().enumerate() {
            let here = Bracket { ch, line: i + 1, col: j + 1 };
            if in_string {
                // Inside "..." a bracket is only text: `"{n}"` opens nothing.
                if escaped {
                    escaped = false;
                } else if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    in_string = false;
                }
                continue;
            }
            match ch {
                '"' => in_string = true,
                '(' | '[' | '{' => open.push(here),
                ')' | ']' | '}' => {
                    let want = partner(ch);
                    let Some(&top) = open.last() else {
                        return Verdict::Unexpected { closer: here, stray };
                    };
                    if top.ch == want {
                        open.pop();
                    } else if let Some(&outer) = open.iter().rev().find(|b| b.ch == want) {
                        // Its partner IS open, further out: what is open inside it was never closed.
                        return Verdict::Mismatched { closer: here, unclosed: top, meant_for: Some(outer) };
                    } else {
                        // Nothing open is its kind. It closes the nearest opener anyway,
                        // and the damage shows up later, at a closer with nothing left.
                        open.pop();
                        stray = stray.or(Some((top, here)));
                    }
                }
                _ => {}
            }
        }
    }

    if let Some((top, closer)) = stray {
        return Verdict::Mismatched { closer, unclosed: top, meant_for: None };
    }
    match open.last() {
        Some(&opener) => Verdict::Unclosed { opener },
        None => Verdict::Balanced,
    }
}

fn label(b: Bracket, says: &str) {
    let at = format!("`{}` at {}:{}", b.ch, b.line, b.col);
    println!("    {at:<12} {says}");
}

fn report(verdict: Verdict) {
    match verdict {
        Verdict::Balanced => println!("  every bracket has its partner"),
        Verdict::Unexpected { closer, stray } => {
            println!("  error: unexpected closing delimiter: `{}`", closer.ch);
            println!("  --> {}:{}", closer.line, closer.col);
            if let Some((nearest, extra)) = stray {
                label(nearest, "the nearest open delimiter");
                label(extra, &format!("missing open `{}` for this delimiter", partner(extra.ch)));
            }
            label(closer, "unexpected closing delimiter");
        }
        Verdict::Mismatched { closer, unclosed, meant_for } => {
            println!("  error: mismatched closing delimiter: `{}`", closer.ch);
            println!("  --> {}:{}", unclosed.line, unclosed.col);
            if let Some(outer) = meant_for {
                label(outer, "closing delimiter possibly meant for this");
            }
            label(unclosed, "unclosed delimiter");
            label(closer, "mismatched closing delimiter");
        }
        Verdict::Unclosed { opener } => {
            println!("  error: this file contains an unclosed delimiter");
            println!("  --> end of file");
            label(opener, "unclosed delimiter");
        }
    }
}

fn banner(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    for (name, src) in FILES {
        banner(name);
        report(check(src));
    }

    banner("extra_paren.rs with the ) the label points at deleted");
    let fixed = FILES[0].1.replacen("to_string());", "to_string();", 1);
    assert_ne!(fixed, FILES[0].1, "the replacement must have found the stray )");
    report(check(&fixed));
    println!("  This check has nothing left to say, so rustc goes on to the");
    println!("  next mistake in the file: `println!(s)` on line 3.");
}
```
<!-- /source -->

<!-- output:where_a_bracket_error_points -->
*Verified output of [`where_a_bracket_error_points.rs`](examples/where_a_bracket_error_points.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── extra_paren.rs
  error: unexpected closing delimiter: `}`
  --> 4:1
    `{` at 1:11  the nearest open delimiter
    `)` at 2:30  missing open `(` for this delimiter
    `}` at 4:1   unexpected closing delimiter

──── far_away.rs
  error: unexpected closing delimiter: `}`
  --> 12:1
    `{` at 1:42  the nearest open delimiter
    `)` at 3:60  missing open `(` for this delimiter
    `}` at 12:1  unexpected closing delimiter

──── missing_paren.rs
  error: mismatched closing delimiter: `}`
  --> 2:21
    `{` at 1:11  closing delimiter possibly meant for this
    `(` at 2:21  unclosed delimiter
    `}` at 4:1   mismatched closing delimiter

──── wrong_kind.rs
  error: mismatched closing delimiter: `)`
  --> 2:17
    `[` at 2:17  unclosed delimiter
    `)` at 2:25  mismatched closing delimiter

──── missing_brace.rs
  error: this file contains an unclosed delimiter
  --> end of file
    `{` at 1:11  unclosed delimiter

──── extra_paren.rs with the ) the label points at deleted
  every bracket has its partner
  This check has nothing left to say, so rustc goes on to the
  next mistake in the file: `println!(s)` on line 3.
```
<!-- /output -->

## If you are coming from another language

- **Python.** Python 3.14 puts the caret on the stray character itself. The same slip inside a dict, `words = {"first": str("aaa"))}`, is `SyntaxError: closing parenthesis ')' does not match opening parenthesis '{'` with `^` under the `)` — the same two brackets rustc's labels name, but with the typo as the headline instead of a label. A missing `)`, as in `n = max(1, 2`, is `SyntaxError: '(' was never closed` with the caret on the `(`, which is where rustc's `mismatched closing delimiter` points too. What changes is the extra-closer case only: in Rust, read the labels, not the `-->` line.
- **JavaScript.** Node 20 calls the `)` unexpected — `SyntaxError: Unexpected token ')'`, caret on the `)` — even inside a function body. rustc uses the same word for a `}` two lines further down. Same vocabulary, a different bracket.
- **C.** Apple clang 21 puts the caret on the stray `)` and keeps going. In a file with `return abs(-3));` in one function and an undeclared name in another, it prints `error: expected ';' after return statement` under the `)`, then `error: use of undeclared identifier 'missing_name'`, then `2 errors generated`. A C compiler recovers from the bracket and reports the rest of the file; rustc 1.98, on every refusal on this page, stopped at the bracket. The wording also moves with context in C: the same extra `)` after `int n = abs(-3)` is `extraneous ')' before ';'`.

## See also

- [The braces take a name](../braces_take_a_name/README.md) — the `println!(s)` error the first file on this page was hiding
- [What a warning is asking](../what_a_warning_is_asking/README.md) — `unnecessary parentheses` is one of those questions, and here the answer is to take the `(` back out
- [A block is an expression](../a_block_is_an_expression/README.md) — the `{ }` a stray `)` closes, and what one misplaced `;` does to the same braces
- [Running a scratch program](../rustc_without_cargo/README.md) — the bare `rustc` every transcript on this page came from
- [Reading a compilation failure](../../20_Compilers/reading_a_compilation_failure/README.md) — how to tell which stage of the compiler a message comes from

## Po polsku

Ta strona jest o błędzie, którego numer linii wprowadza w błąd. Jeden nawias `)` za dużo w linii 2 `rustc` zgłasza jako `unexpected closing delimiter` w linii 4 — przy klamrze `}` kończącej `main`, która jest napisana poprawnie. *Delimiter* to w komunikatach `rustc` każdy z trzech rodzajów nawiasów: okrągły `( )`, kwadratowy `[ ]` i klamrowy `{ }`. Nawiasy ostre `< >` się nie liczą.

Skąd ta linia 4? Kompilator paruje nawiasy. Drugi `)` szuka otwartego `(`, nie znajduje go, więc zamyka najbliższy otwarty nawias — klamrę `{` funkcji `main`. Od tej chwili ciało `main` jest „zamknięte”, a prawdziwa klamra `}` na końcu nie ma już czego zamykać. Nagłówek `-->` wskazuje więc miejsce, w którym zabrakło nawiasów do parowania, a nie miejsce literówki. Literówka jest pod etykietą `-` z tekstem *missing open `(` for this delimiter* i to ten znak trzeba usunąć. Dopisanie brakującego `(` też usuwa błąd, ale wtedy `rustc` ostrzega `unnecessary parentheses` i prosi o usunięcie nawiasów.

Trzy komunikaty, trzy miejsca: `unexpected closing delimiter` — nawiasu jest za dużo, szukaj etykiety *missing open*; `mismatched closing delimiter` — nawiasu brakuje albo jest złego rodzaju (`vec![1, 2, 3)`), a nagłówek wskazuje nawias, który nie został zamknięty; `this file contains an unclosed delimiter` — brakuje nawiasu zamykającego; jeśli brakuje klamry w środku pliku, `rustc` podpowiada wcięciem (*different indentation*), gdzie jej szukać.

I rzecz najważniejsza na początku nauki: dopóki nawiasy się nie parują, `rustc` nie zgłasza w tym pliku niczego innego. Plik z trzema błędami w trzech funkcjach dał jeden komunikat, a po usunięciu jednego `)` — dwa błędy i ostrzeżenie. `1 previous error` pod błędem nawiasu to nie jest liczba twoich pomyłek. Python, JavaScript i C stawiają daszek `^` na samym nadmiarowym `)`; Rust przy tym błędzie wskazuje późniejszą klamrę.

**Szukaj po polsku:** błąd nawiasu w Ruście · nawias za dużo · niedomknięty nawias · `rust unexpected closing delimiter` · `rust mismatched closing delimiter` · `rust this file contains an unclosed delimiter`
