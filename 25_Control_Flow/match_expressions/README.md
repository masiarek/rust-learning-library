# `match` expressions

**Level:** 101 → 201 · for newcomers

**One line:** `match` checks a value against a list of options top to bottom, takes the first that fits, and — unlike a `switch` — **never falls through** and **must cover every case**, which is the property that turns "I forgot one" from a runtime surprise into a compile error.

```rust
let n = 3u8;
let word = match n {
    0 => "none",
    1 => "one",
    _ => "many",
};
println!("{word}");   // many
```

One value after `match`, then a list of **arms**, each `pattern => expression`. The first arm whose pattern fits is the one that runs, and its expression is the value of the whole `match`. `_` fits anything.

## The first arm that fits wins

```rust
let band = match n {
    1..=5 => "low",
    3..=9 => "middle",   // 3, 4 and 5 fit here too, and never arrive
    _ => "out of range",
};
```

`4` fits both ranges and takes the first. rustc says nothing about the overlap, because the second arm still has `6..=9` to itself. An arm that **no** value can reach does get a warning — put `_` first and every arm below it is dead:

```text title="Abridged — real rustc output for wildcard_first.rs, second warning and summary dropped"
warning: unreachable pattern
 --> wildcard_first.rs:5:9
  |
4 |         _ => "many",
  |         - matches any value
5 |         0 => "none",
  |         ^ no value can reach this
  |
  = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default
```

It is a warning, not an error, so the program builds and every value gets `"many"`. [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) goes further: rustc checks each alternative of an `a | b | c` arm separately, and has a second lint for two ranges left one value apart.

## Exactly one arm runs: no fall-through, so no `break`

```rust
match n {
    1 => println!("ran the 1 arm"),
    2 | 3 => println!("ran the 2 | 3 arm"),   // one arm, two values
    _ => println!("ran the _ arm"),
}
```

There is no mechanism for running a second arm, so there is nothing to stop with a `break` and no way to forget one. When two values should share code, they share an arm: `2 | 3`. An arm's body is one expression, and a [block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md), so `=> { … }` holds as many statements as you need. The comma after a `}` arm is optional; after any other arm it is not — rustc stops with *expected `,` following `match` arm*.

## `match` has a value, so every arm has the same type

```rust
let word = match n {
    0 => "none".to_string(),
    1 => "one".to_string(),
    many => {
        let noun = "items";
        format!("{many} {noun}")   // "5 items" for 5
    }
};
```

Every arm is a `String`, so the `match` is one. `many` is a pattern too: a bare name fits anything, like `_`, and also binds the value so the arm can use it. The rule is the one [`if` expressions](../if_expressions/README.md) have for two branches, applied to however many arms there are:

```text title="Abridged — real rustc output for match_arms_disagree.rs, summary lines dropped"
error[E0308]: `match` arms have incompatible types
 --> match_arms_disagree.rs:5:14
  |
3 |       let word = match n {
  |  ________________-
4 | |         0 => "none",
  | |              ------ this is found to be of type `&str`
5 | |         1 => 1,
  | |              ^ expected `&str`, found integer
6 | |         _ => "many",
7 | |     };
  | |_____- `match` arms have incompatible types
```

The first arm sets the type, and the first arm that disagrees gets the `^`. One kind of arm is exempt — an arm that never finishes:

```rust
for text in ["12", "seven", "30"] {
    let n: u32 = match text.parse() {
        Ok(n) => n,
        Err(_) => continue,   // type `!`: sits beside a u32 arm
    };
    println!("{n}");          // 12, then 30
}
```

`continue`, `return`, `break` and `panic!` have [the never type `!`](../../15_First_Programs/the_never_type/README.md), which fits a slot of any type. *Bind it, or leave* is common enough to have its own shorter spelling, [`let else`](../../30_Pattern_Matching/let_else/README.md).

## Every value must be covered

A `match` that could meet a value with no arm for it does not compile, and rustc names the value:

```text title="Abridged — real rustc output for match_bool_one_arm.rs, summary lines dropped"
error[E0004]: non-exhaustive patterns: `false` not covered
 --> match_bool_one_arm.rs:3:22
  |
3 |     let coat = match raining {
  |                      ^^^^^^^ pattern `false` not covered
  |
  = note: the matched value is of type `bool`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
4 ~         true => "umbrella",
5 ~         false => todo!(),
  |
```

That makes a two-arm `match` on a `bool` an honest `if`/`else`: `match is_even(n) { true => "even", false => "odd" }` covers every case and compiles, and the sidebar after *Rust in Action* §2.4.6 shows it beside the `if` version. The `if` is shorter; the `match` is the one that can grow a third arm.

On an integer, rustc counts ranges, so the message is a range:

```text title="Abridged — real rustc output for match_misses_a_value.rs, summary lines dropped"
error[E0004]: non-exhaustive patterns: `10_u8..=u8::MAX` not covered
 --> match_misses_a_value.rs:3:22
  |
3 |     let word = match n {
  |                      ^ pattern `10_u8..=u8::MAX` not covered
  |
  = note: the matched value is of type `u8`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
6 ~         2..=9 => "a few",
7 ~         10_u8..=u8::MAX => todo!(),
  |
```

The `help:` fills the new arm with `todo!()`, a placeholder that panics — the arm is yours to write. Counting also works the other way: ranges that meet end to end cover a type with no `_` at all.

```rust
fn sign(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=i32::MAX => "positive",
    }
}
```

A `char` is counted the same way, over every Unicode scalar value, which is why a `match` on letters nearly always ends in `_`:

```text title="Abridged — real rustc output for match_char_gap.rs, summary lines dropped"
error[E0004]: non-exhaustive patterns: `'\0'..='@'`, `'['..='`'`, `'{'..='\u{d7ff}'` and 1 more not covered
 --> match_char_gap.rs:3:22
  |
3 |     let kind = match c {
  |                      ^ patterns `'\0'..='@'`, `'['..='`'`, `'{'..='\u{d7ff}'` and 1 more not covered
  |
  = note: the matched value is of type `char`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms
  |
5 ~         'A'..='Z' => "upper",
6 ~         _ => todo!(),
  |
```

The gaps are the characters before `A`, between `Z` and `a`, and after `z` up to the surrogate range; the *1 more* is `'\u{e000}'..='\u{10ffff}'`, the rest of Unicode above it — cover the first three and rustc names that one. *Rust in Action* §2.4.7 says `match` *warns* you about a case you did not consider; it is an error, and [the claims page](../flow_control_claims_checked/README.md) runs the book's own arms to show it.

## Where `_` earns its keep, and where it costs you

On `char`, `u32` or `&str`, "everything else" is most of the type, and `_` is the honest arm. On an `enum` you own, `_` is a promise about variants that do not exist yet:

```rust
enum Delivery { Standard, Express, Pickup }   // Pickup arrived a year later

fn fee(d: Delivery) -> u32 {
    match d {
        Delivery::Standard => 599,
        _ => 1_299,          // meant Express, when Express was all there was
    }
}
```

The day `Pickup` was added, this compiled without a word and started charging $12.99 for collecting a parcel yourself — section 9 of the output. Name every variant instead and the same addition is `E0004` in every `match` that has not decided what `Pickup` means. That is the reason [enums](../../13_Enums/README.md) and `match` are taught together, and [a typo becomes a binding](../../13_Enums/a_typo_becomes_a_binding/README.md) is the same hole dug by accident: a lowercase name in an arm is a catch-all binding, like `many` above, not a comparison.

## Matching through a reference, and on a `String`

The book's Listing 2.8 matches `for item in &haystack`, so `item` is a `&i32`, against plain integer patterns — `42 | 132 => "hit!"` — and it compiles and prints `42: hit!` and `132: hit!`. A reference matched against a pattern that is not one is stepped through automatically; [match ergonomics](../../30_Pattern_Matching/match_ergonomics/README.md) is that rule.

A `String` is not a reference, so there is nothing to step through, and a string literal pattern is a `&str`:

```rust
let command = String::from("stop");
// match command { "go" => 1, "stop" => 0, _ => -1 }   // E0308 on each literal arm
let code = match command.as_str() { "go" => 1, "stop" => 0, _ => -1 };
println!("{code}");   // 0
```

```text title="Abridged — real rustc output for match_string_literal.rs, first of two errors and summary dropped"
error[E0308]: mismatched types
 --> match_string_literal.rs:4:9
  |
3 |     let code = match command {
  |                      ------- this expression has type `String`
4 |         "go" => 1,
  |         ^^^^ expected `String`, found `&str`
  |
help: consider converting the `String` to a `&str` using `.as_str()`
  |
3 |     let code = match command.as_str() {
  |                             +++++++++
```

## Where the patterns go from here

Everything left of a `=>` is a pattern, and patterns are a section of their own — [Pattern matching](../../30_Pattern_Matching/README.md). This page used four kinds (a literal, a range, `|`, a name); that section has the rest:

| You want to | Page |
|---|---|
| take a struct or tuple apart in the arm | [Destructuring structs](../../30_Pattern_Matching/destructuring_structs/README.md) |
| get the data out of an enum variant | [Destructuring enums](../../30_Pattern_Matching/destructuring_enums/README.md) |
| add a condition the pattern cannot express | [Match guards](../../30_Pattern_Matching/match_guards/README.md) |
| test a range and keep the value | [Binding with `@`](../../30_Pattern_Matching/binding_at/README.md) |
| know exactly what `_` does not bind | [The wildcard `_`](../../30_Pattern_Matching/the_wildcard/README.md) |
| handle one pattern and ignore the rest | [`if let`](../../17_Option_and_Result/if_let/README.md) |

## Run it

<!-- output:match_expressions -->
*Verified output of [`match_expressions.rs`](examples/match_expressions.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── 1. Arms are tried top to bottom; the first that fits wins
   4 -> low
   7 -> middle
  12 -> out of range
  4 fits both ranges and takes the first. rustc does not warn:
  the second arm still has 6..=9 to itself.

──── 2. Exactly one arm runs: there is no fall-through, so no break
  n = 1: ran the 1 arm
  n = 2: ran the 2 | 3 arm
  n = 3: ran the 2 | 3 arm

──── 3. match has a value, and an arm may be a block
  0 -> "none"
  1 -> "one"
  5 -> "5 items"

──── 4. A bool: two arms are every case (the book's is_even)
  6: match says even, if says even
  7: match says odd, if says odd

──── 5. Integers: ranges that meet end to end need no _
    7u8 -> one digit
   42u8 -> two digits
  255u8 -> three digits
  -2147483648i32 -> negative
            0i32 -> zero
           17i32 -> positive

──── 6. A char: the ranges you care about, and _ for the rest of Unicode
  'q' -> lowercase
  'Q' -> uppercase
  '7' -> digit
  'ż' -> something else

──── 7. Listing 2.8 from the book: item is a &i32, the patterns are not
  42: hit!
  132: hit!

──── 8. A diverging arm fits any type
  "12": added, total = 12
  "seven": not a number, skipped
  "30": added, total = 42

──── 9. _ is exhaustive forever: a new variant goes where _ sends it
  Standard catch-all:  599 cents   every variant named:  599 cents
  Express  catch-all: 1299 cents   every variant named: 1299 cents
  Pickup   catch-all: 1299 cents   every variant named:    0 cents
  Pickup costs 1299 through the catch-all, and nothing warned.
```
<!-- /output -->

## If you are coming from another language

**Python.** The `match` statement (3.10) looks nearly identical — `case 0:`, `case 2 | 3:`, `case _:` — and first match wins there too, with no fall-through. Three things differ. It is a statement, so `word = match n:` is a `SyntaxError`, and the value has to be assigned or returned inside each `case`. It is not exhaustive: a subject no `case` fits runs nothing and says nothing, so a function built on it quietly returns `None`. And the unreachable arm is judged the other way round — Python refuses to compile a `case _:` or a bare `case other:` that is not last (`wildcard makes remaining patterns unreachable`), where rustc builds it and warns. The trap does transfer exactly: `case RED:` is a capture that matches everything, and only a dotted name like `case Color.RED:` compares, which is [a typo becomes a binding](../../13_Enums/a_typo_becomes_a_binding/README.md) in Python spelling. There are no range patterns; `case n if 1 <= n <= 5:` is a [guard](../../30_Pattern_Matching/match_guards/README.md).

**C.** `switch` falls through by default: without a `break`, `case 1:` runs into `case 2:` and on into `default:`. Clang says so only under `-Wimplicit-fallthrough`, and C23's `[[fallthrough]];` marks the times it was meant. Rust has no fall-through to mark — `1 | 2 =>` is the deliberate case. Exhaustiveness exists in C as a warning: clang's `-Wswitch`, on by default, reports an enum value with no `case`, but only when there is no `default:`, and never for a plain `int`. `default:` silences it exactly as `_` does here; the difference is that Rust checks every type, and refuses rather than warns. `switch` is a statement with integer-constant labels, so a value means an assignment in every case, and a range means `case 1 ... 5:`, a GNU extension that clang 21 under `-pedantic` calls a C2y extension.

**ABAP.** `CASE … WHEN … WHEN OTHERS. … ENDCASE.` transfers almost exactly: first matching `WHEN`, at most one block, then on past `ENDCASE` — no fall-through, and `WHEN 2 OR 3.` is `2 | 3`. `WHEN OTHERS` is `_`, and it is optional: a `CASE` with no matching `WHEN` runs nothing, and a `CASE` on a 7.51 enumerated type [still compiles with a value missing](../../13_Enums/what_an_enum_is/README.md). Rust makes the missing value `E0004`. The value-producing form is `SWITCH #( lv_mode WHEN … THEN … ELSE … )`, and it differs from `match` in the same two places `COND` differs from `if`: without `ELSE`, an unmatched operand yields the type's initial value rather than a compile error, and `THEN` results are converted to the result type rather than required to share one. `ELSE THROW cx_…( )` is the diverging arm. Neither form takes a range or binds a name (`SWITCH` accepts only literals and constants after `WHEN`), so most of [pattern matching](../../30_Pattern_Matching/README.md) has no ABAP counterpart; the nearest is `CASE TYPE OF`, whose `WHEN TYPE … INTO` tests a class and binds the cast reference in one step.

## Practice

**The grade table with a hole in it.** Write `grade(score: u8) -> char` — `90..=100` is `A`, then `B`, `C`, `D` in bands of ten, below 60 is `F`, and anything above 100 is `'?'` — three ways: as an `if` ladder, as a `match` ending in `_ => '?'`, and as a `match` with no `_` that names `101..=u8::MAX` itself. Check the three agree on all 256 values of `u8`.

Then delete the C band from each. **Before compiling**, write down which of the three still compile, and what a score of 75 becomes in each one that does.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:match_expressions_kata -->
*[`match_expressions_kata.rs`](examples/match_expressions_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the grade table with a hole in it.
//!
//! One function, three spellings, and then the same mistake made in each: the
//! C band deleted. Two of the three still compile.
//!
//!   rustc --edition 2024 match_expressions_kata.rs -o /tmp/mxk && /tmp/mxk

/// Spelling 1: an `if` ladder. Order matters, and nothing checks the rungs.
fn grade_ladder(score: u8) -> char {
    if score > 100 {
        '?'
    } else if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

/// Spelling 2: a `match` whose last arm is `_`.
fn grade_catch_all(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    }
}

/// Spelling 3: a `match` that names every `u8`, including the invalid ones.
fn grade_named(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        101..=u8::MAX => '?',
    }
}

/// Spelling 1 with the C rung deleted. Compiles.
fn grade_ladder_holed(score: u8) -> char {
    if score > 100 {
        '?'
    } else if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

/// Spelling 2 with the C arm deleted. Compiles.
fn grade_catch_all_holed(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    }
}

fn main() {
    println!("1. Three spellings, checked against each other on every u8");
    let disagreements = (0..=u8::MAX)
        .filter(|&s| {
            let a = grade_ladder(s);
            a != grade_catch_all(s) || a != grade_named(s)
        })
        .count();
    println!("   scores checked: 256, disagreements: {disagreements}");
    for s in [95u8, 75, 42, 101, 255] {
        println!(
            "   {s:>3} -> ladder {}  catch-all {}  named {}",
            grade_ladder(s),
            grade_catch_all(s),
            grade_named(s)
        );
    }

    println!();
    println!("2. Delete the C band from each, and predict before compiling");
    for s in [75u8, 79] {
        println!(
            "   {s} -> ladder {}  catch-all {}  named: does not compile",
            grade_ladder_holed(s),
            grade_catch_all_holed(s)
        );
    }
    println!("   The ladder hands 70..=79 to the next rung down, so a C is a D.");
    println!("   The catch-all hands them to `_`, so a C is an invalid score.");
    println!("   Both compile without a warning, and both are wrong.");
    println!("   The named match is error[E0004]: non-exhaustive patterns:");
    println!("   `70_u8..=79_u8` not covered. It names the hole exactly, because");
    println!("   no arm was allowed to mean \"everything else\".");
}
```
<!-- /source -->

<!-- output:match_expressions_kata -->
*Verified output of [`match_expressions_kata.rs`](examples/match_expressions_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three spellings, checked against each other on every u8
   scores checked: 256, disagreements: 0
    95 -> ladder A  catch-all A  named A
    75 -> ladder C  catch-all C  named C
    42 -> ladder F  catch-all F  named F
   101 -> ladder ?  catch-all ?  named ?
   255 -> ladder ?  catch-all ?  named ?

2. Delete the C band from each, and predict before compiling
   75 -> ladder D  catch-all ?  named: does not compile
   79 -> ladder D  catch-all ?  named: does not compile
   The ladder hands 70..=79 to the next rung down, so a C is a D.
   The catch-all hands them to `_`, so a C is an invalid score.
   Both compile without a warning, and both are wrong.
   The named match is error[E0004]: non-exhaustive patterns:
   `70_u8..=79_u8` not covered. It names the hole exactly, because
   no arm was allowed to mean "everything else".
```
<!-- /output -->

</details>

## See also

- [`if` expressions](../if_expressions/README.md) — the two-case version, and when it reads better
- [Flow control](../flow_control/README.md) — every construct in this chapter, what it evaluates to, and how you leave it
- [Pattern matching](../../30_Pattern_Matching/README.md) — everything that can go left of the `=>`
- [A typo becomes a binding](../../13_Enums/a_typo_becomes_a_binding/README.md) — the catch-all that was not meant to be one
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — ranges and `|` collapsing twenty-six arms into five
- [What an enum is](../../13_Enums/what_an_enum_is/README.md) — the type `match` was built for, and where exhaustiveness pays
- [Six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) — an exhaustive match as a domain model, and the arm nobody wrote
- [The never type `!`](../../15_First_Programs/the_never_type/README.md) — why a `continue` arm fits beside a `u32` one
- [`match` expressions ↗](https://doc.rust-lang.org/reference/expressions/match-expr.html) · [Comprehensive Rust: `match` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/match.html)

## Sources

Tim McNamara, *Rust in Action* (Manning, 2021), §2.4.7 — Listing 2.8 and the claim about a case you did not consider — and the sidebar after §2.4.6 for `match is_even(n)`. [The Rust Reference: `match` expressions ↗](https://doc.rust-lang.org/reference/expressions/match-expr.html) and [`E0004` ↗](https://doc.rust-lang.org/error_codes/E0004.html). Every rustc transcript above was produced by rustc 1.98.0 with `--edition 2024` from a scratch file of the name its header shows. Python's side, run on 3.14: [the `match` statement ↗](https://docs.python.org/3/reference/compound_stmts.html#the-match-statement). C's side, run on Apple clang 21. ABAP's side, from the 7.58 keyword documentation: [`CASE` ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abapcase.htm) and [`SWITCH` ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abenconditional_expression_switch.htm).

## Po polsku

`match` sprawdza wartość po kolei, od góry, i wygrywa pierwsze pasujące ramię — nawet gdy pasowałoby też następne, bo nakładające się zakresy `1..=5` i `3..=9` kompilują się bez ostrzeżenia, a `4` trafia do pierwszego. Ostrzeżenie (`unreachable pattern`) pojawia się dopiero wtedy, gdy do ramienia nie może dotrzeć żadna wartość, na przykład pod `_` postawionym na początku. Od `switch`a znanego z C czy Javy różnią go dwie rzeczy i obie są tu najważniejsze: nie ma przechodzenia między przypadkami (*fall-through*), więc nie ma też `break`a, o którym dałoby się zapomnieć — dwie wartości, które mają dzielić kod, dzielą jedno ramię: `2 | 3` — oraz obowiązuje **kompletność** dopasowania: każdy możliwy przypadek musi być pokryty, w przeciwnym razie `E0004`, a kompilator sam nazwie wartość, o którą nie zadbaliśmy. Dla `bool` będzie to `false`, dla `u8` zakres w rodzaju `10_u8..=u8::MAX`, a dla `char` lista przedziałów Unicode. Liczenie działa też w drugą stronę: zakresy, które stykają się końcami, pokrywają cały typ i `_` nie jest wtedy potrzebne.

Sam `match` jest przy tym wyrażeniem, więc wszystkie ramiona muszą mieć ten sam typ — dokładnie tak jak gałęzie `if` — z jednym wyjątkiem: ramię, które nigdy się nie kończy (`continue`, `return`, `panic!`), ma typ `!` i pasuje wszędzie. Ciało ramienia może być blokiem `{ … }`, a przecinek po zamykającej klamrze jest opcjonalny. Osobna, częsta pułapka dotyczy napisów: `match` na `String` z literałami `"go"` w ramionach to `E0308`, bo literał jest typu `&str` — kompilator sam podpowiada `.as_str()`. Na referencji (`&i32`) zwykłe literały działają, bo dopasowanie samo przechodzi przez referencję.

Pułapka siedzi w `_`: ramię `_ => …` czyni dopasowanie kompletnym **na zawsze**, więc wariant dołożony do wyliczenia rok później skompiluje się bez słowa i po cichu wpadnie do gałęzi domyślnej — a bliźniacza wersja tego błędu to nazwa wariantu zapisana w ramieniu małą literą, bo wtedy zamiast porównania powstaje nowe wiązanie, które łapie wszystko. W Pythonie (`match` od wersji 3.10) jest tak samo z gołą nazwą w `case`, ale tamtejszy `match` jest instrukcją, nie ma wartości i nie sprawdza kompletności. W ABAP-ie `CASE … WHEN … WHEN OTHERS … ENDCASE` też nie przechodzi między przypadkami, lecz `WHEN OTHERS` jest opcjonalne, a wyrażenie `SWITCH #( … )` bez `ELSE` zwraca po cichu wartość początkową typu tam, gdzie Rust odmawia kompilacji.

**Szukaj po polsku:** dopasowanie wzorców · kompletność dopasowania · `match` a `switch` · przechodzenie między przypadkami · `rust E0004 non-exhaustive patterns` · `rust match catch-all underscore` · `rust match String as_str` · `rust unreachable pattern warning`
