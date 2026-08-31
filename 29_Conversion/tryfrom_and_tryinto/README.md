# `TryFrom` and `TryInto`

**Level:** 201 · working knowledge

**One line:** The conversion that is allowed to say no — same shape as [`From`](../from_and_into/README.md), plus a `type Error`, which is the whole difference and the whole point.

```rust
use std::convert::TryFrom;

#[derive(Debug)]
struct Score(u8);

#[derive(Debug)]
struct OutOfRange(u8);

impl TryFrom<u8> for Score {
    type Error = OutOfRange;
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        if n <= 5 { Ok(Score(n)) } else { Err(OutOfRange(n)) }
    }
}

fn main() {
    println!("{:?}", Score::try_from(4u8));   // Ok(Score(4))
    println!("{:?}", Score::try_from(9u8));   // Err(OutOfRange(9))
}
```

`TryInto` comes free from `TryFrom` by the same blanket impl that gives `Into` from `From`. Both are in the prelude since the 2021 edition, so the `use` line above is habit rather than necessity.

## std's own, which you meet first

```rust
fn main() {
    println!("{:?}", u8::try_from(300i32));   // Err(TryFromIntError(PosOverflow))
    println!("{:?}", u8::try_from(200i32));   // Ok(200)
    println!("{:?}", u8::try_from(-1i32));    // Err(TryFromIntError(NegOverflow))
}
```

Every narrowing integer conversion has one, and the error is `TryFromIntError`, which carries **nothing** — the failure has exactly one cause, so there is nothing to report. Compare `OutOfRange(9)` above, which keeps the offending value so the message can name it. Deciding between those two is most of the work of writing a `TryFrom`.

## Collecting stops at the first refusal

```rust
fn read_ballot(cells: &[u8]) -> Result<Vec<Score>, OutOfRange> {
    cells.iter().map(|&n| Score::try_from(n)).collect()
}
```

`Vec<Result<T, E>>` collected into `Result<Vec<T>, E>` is one of the most useful shapes in std: it short-circuits, so nothing after the bad cell is converted, and the caller gets one error rather than a vector of them. When you want *every* failure instead, `partition` is the other half of that pattern.

## For text, implement `FromStr` instead

```rust
use std::str::FromStr;

impl FromStr for Score {
    type Err = ScoreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u32 = s.trim().parse().map_err(|_| ScoreError::NotANumber(s.into()))?;
        Score::try_from(n)
    }
}
```

`FromStr` is what `.parse()` calls, so implementing it gives you `"4".parse::<Score>()` and — more usefully — `map(str::parse)` inside an iterator chain. `TryFrom<&str>` would work and give you neither. Note the field is `type Err`, not `type Error`: the two traits spell it differently, which costs everybody one compile error exactly once.

## The trap: `as` is the same conversion with the check deleted

```rust
fn main() {
    println!("{:?}", u8::try_from(300i32));   // Err(TryFromIntError(PosOverflow))
    println!("{}", 300i32 as u8);             // 44
}
```

Both are "convert `i32` to `u8`". One reports that it could not; the other returns 44 and says nothing — no panic, no warning, in debug builds too. The practice below runs a ballot parser both ways: the `as` version accepts a score of 9 on a 0–5 ballot and turns a non-numeric cell into a zero indistinguishable from a real one.

**The test:** if you cannot say out loud what should happen to the out-of-range case, you wanted `try_from`.

## If you are coming from another language

- **Python.** `int("abc")` raising `ValueError` is `TryFrom` with the error moved into the return type instead of the control flow. The port of a `try: … except ValueError:` block is a `match` on the `Result` or a `?`, and the difference that matters is that Rust's version is **in the signature** — you cannot fail to notice that a conversion can fail, where in Python the only way to know is to read the docs or be surprised in production. Python's `int()` is also happy to truncate a float silently (`int(3.9)` is 3); the Rust equivalent of that decision is `as`, and it is deliberately a different spelling.
- **ABAP.** This is the `CATCH cx_sy_conversion_no_number` you write around a `MOVE` from a `CHAR` field, and the improvement is the same one Python gets: the failure is in the type rather than in an exception class you have to remember exists. The closer analogy is the pair `MOVE` / `MOVE … ?TO` for object references — the second one can fail and raises `CX_SY_MOVE_CAST_ERROR`, which is exactly the `From` / `TryFrom` split with a runtime check instead of a compile-time one. What transfers directly: the ABAP habit of validating before assigning (`IF lv_input CO '0123456789'`) is the same instinct as writing a `TryFrom` impl, but the impl does it once, in a named place, rather than at every call site.
- **C#.** `int.Parse` and `int.TryParse` are exactly this pair, and the naming is close enough to be a mnemonic. Rust's version returns the value *inside* the result rather than through an `out` parameter, which is what lets it chain.
- **Go.** `strconv.Atoi` returning `(int, error)` is the same signature; `?` is what Go's `if err != nil { return err }` would be if it were an operator.

---

## The verified output

<!-- output:tryfrom_and_tryinto -->
*Verified output of [`tryfrom_and_tryinto.rs`](examples/tryfrom_and_tryinto.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One impl, two call syntaxes, one of them fallible
   Score::try_from(4) = Ok(Score(4))
   Score::try_from(9) = Err(ScoreOutOfRange(9))
   4u8.try_into()     = Ok(Score(4))
   `TryInto` comes free from `TryFrom`, exactly as `Into` comes free
   from `From` — same blanket impl, one Result deeper.

2. The difference from From, in one line
   From has no error type, so a conversion that cannot honour its
   input must clamp, wrap or panic. TryFrom has `type Error`, so it
   can hand the bad value back and let the caller decide.

3. std's own, which you will meet first
   u8::try_from(300i32)  = Err(TryFromIntError(PosOverflow))
   u8::try_from(200i32)  = Ok(200)
   u8::try_from(-1i32)   = Err(TryFromIntError(NegOverflow))
   i32::try_from(9u64)   = Ok(9)
   The error is `TryFromIntError`, and it carries nothing — the
   failure has exactly one cause, so there is nothing to report.
   Compare that with ScoreOutOfRange above, which keeps the value.

4. Collecting into a Result stops at the first refusal
   read_ballot([5, 3, 0])    = Ok([Score(5), Score(3), Score(0)])
   read_ballot([5, 9, 0])    = Err(ScoreOutOfRange(9))
   `Vec<Result<T, E>>` collected into `Result<Vec<T>, E>` is one of
   the most useful shapes in std: it short-circuits, so the third
   cell is never converted, and the caller gets one error rather
   than a vector of them.

5. The trap: `as` is the same conversion with the check deleted
   u8::try_from(300) = Err(TryFromIntError(PosOverflow))
   300 as u8         = 44
   Both are "convert i32 to u8". One reports that it could not; the
   other returns 44 and says nothing. Reach for `as` when truncation
   is what you meant, and for try_from when it is not.
```
<!-- /output -->

## Practice

**A ballot line, parsed twice, and one of the two accepts junk.** Write a `Score` that refuses anything above 5, give it both a `TryFrom<u32>` and a `FromStr`, and parse `"5,3,0"` into a `Vec<Score>` with one `collect`. Then run three broken lines through it: a non-numeric cell, a score of 9, and one with stray spaces — one of those three is not an error, and deciding that is part of writing the impl.

Now write the same parser with `parse().unwrap_or(0) as u8` instead, run the same lines, and look at what comes out. Both versions return a full, plausible ballot. Say what each of the two bad cells became, and why *neither* is detectable downstream.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:tryfrom_and_tryinto_kata -->
*[`tryfrom_and_tryinto_kata.rs`](examples/tryfrom_and_tryinto_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a ballot line, parsed three ways, two of which accept junk.
//!
//!   rustc --edition 2024 tryfrom_and_tryinto_kata.rs -o /tmp/tfk && /tmp/tfk

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Score(u8);

#[derive(Debug, PartialEq)]
enum ScoreError {
    NotANumber(String),
    OutOfRange(u32),
}

impl fmt::Display for ScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreError::NotANumber(s) => write!(f, "{s:?} is not a number"),
            ScoreError::OutOfRange(n) => write!(f, "{n} is outside 0-5"),
        }
    }
}

impl TryFrom<u32> for Score {
    type Error = ScoreError;
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n <= 5 {
            Ok(Score(n as u8))
        } else {
            Err(ScoreError::OutOfRange(n))
        }
    }
}

/// `FromStr` is what `.parse()` calls. Implement this rather than
/// `TryFrom<&str>` when the source is text: you get `"4".parse()?` free.
impl FromStr for Score {
    type Err = ScoreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u32 = s
            .trim()
            .parse()
            .map_err(|_| ScoreError::NotANumber(s.to_string()))?;
        Score::try_from(n)
    }
}

/// The version that reports the first bad cell.
fn parse_line(line: &str) -> Result<Vec<Score>, ScoreError> {
    line.split(',').map(str::parse).collect()
}

/// The version that quietly accepts everything.
fn parse_line_as(line: &str) -> Vec<Score> {
    line.split(',')
        .map(|c| Score(c.trim().parse::<u32>().unwrap_or(0) as u8))
        .collect()
}

fn main() {
    println!("1. The good line");
    println!("   parse_line(\"5,3,0\")    = {:?}", parse_line("5,3,0"));
    println!("   `.parse()` reached FromStr, FromStr reached TryFrom, and collect");
    println!("   turned Vec<Result<_>> into Result<Vec<_>>.");

    println!();
    println!("2. The three ways one line can be wrong");
    for line in ["5,x,0", "5,9,0", "5, 3 ,0"] {
        match parse_line(line) {
            Ok(v) => println!("   {line:12} -> Ok({:?})", v.iter().map(|s| s.0).collect::<Vec<_>>()),
            Err(e) => println!("   {line:12} -> Err({e})"),
        }
    }
    println!("   The third is not an error: FromStr trims, so whitespace is fine.");
    println!("   Deciding that is part of writing the impl, and it is written down");
    println!("   in one place instead of at every call site.");

    println!();
    println!("3. The same lines through `as`");
    for line in ["5,x,0", "5,9,0"] {
        let v = parse_line_as(line);
        println!("   {line:12} -> {:?}", v.iter().map(|s| s.0).collect::<Vec<_>>());
    }
    println!("   Both produce a full, plausible, wrong ballot. `x` became a zero");
    println!("   score — indistinguishable from a real zero — and 9 became 9,");
    println!("   which is not a legal score at all: the `as u8` cast let it past");
    println!("   the type that exists to prevent exactly that.");

    println!();
    println!("4. Two traits, and which to implement");
    println!("   TryFrom<u32> for Score  -> Score::try_from(4u32), 4u32.try_into()");
    println!("   FromStr     for Score   -> \"4\".parse::<Score>()");
    println!("   Implementing FromStr also gives you `.parse()` inside iterator");
    println!("   chains, as `map(str::parse)` above — which is why `TryFrom<&str>`");
    println!("   is the wrong choice for a type parsed out of text.");

    println!();
    println!("5. The error type is the deliverable");
    let e = "9".parse::<Score>().unwrap_err();
    println!("   \"9\".parse::<Score>() error = {e}");
    println!("   debug form: {e:?}");
    println!("   ScoreError keeps the offending value, so the message can name it.");
    println!("   std's TryFromIntError does not, because there is only ever one");
    println!("   thing to say. Decide which of those your caller needs before you");
    println!("   write `type Error = String`.");
}
```
<!-- /source -->

<!-- output:tryfrom_and_tryinto_kata -->
*Verified output of [`tryfrom_and_tryinto_kata.rs`](examples/tryfrom_and_tryinto_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The good line
   parse_line("5,3,0")    = Ok([Score(5), Score(3), Score(0)])
   `.parse()` reached FromStr, FromStr reached TryFrom, and collect
   turned Vec<Result<_>> into Result<Vec<_>>.

2. The three ways one line can be wrong
   5,x,0        -> Err("x" is not a number)
   5,9,0        -> Err(9 is outside 0-5)
   5, 3 ,0      -> Ok([5, 3, 0])
   The third is not an error: FromStr trims, so whitespace is fine.
   Deciding that is part of writing the impl, and it is written down
   in one place instead of at every call site.

3. The same lines through `as`
   5,x,0        -> [5, 0, 0]
   5,9,0        -> [5, 9, 0]
   Both produce a full, plausible, wrong ballot. `x` became a zero
   score — indistinguishable from a real zero — and 9 became 9,
   which is not a legal score at all: the `as u8` cast let it past
   the type that exists to prevent exactly that.

4. Two traits, and which to implement
   TryFrom<u32> for Score  -> Score::try_from(4u32), 4u32.try_into()
   FromStr     for Score   -> "4".parse::<Score>()
   Implementing FromStr also gives you `.parse()` inside iterator
   chains, as `map(str::parse)` above — which is why `TryFrom<&str>`
   is the wrong choice for a type parsed out of text.

5. The error type is the deliverable
   "9".parse::<Score>() error = 9 is outside 0-5
   debug form: OutOfRange(9)
   ScoreError keeps the offending value, so the message can name it.
   std's TryFromIntError does not, because there is only ever one
   thing to say. Decide which of those your caller needs before you
   write `type Error = String`.
```
<!-- /output -->

</details>

---

## See also

- [`From` and `Into`](../from_and_into/README.md) — the infallible half, and where `?`'s conversion rule is explained
- [Casting with `as`](../casting_with_as/README.md) — the conversion with no check at all
- [Parsing out of a string](../../14_Strings/parsing_a_string/README.md) — `.parse()` from the text side
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — why this returns a `Result` and not an `Option`
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the type this conversion is protecting

## Sources

[Conversion: TryFrom and TryInto ↗](https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html) in Rust by Example, plus [`std::convert::TryFrom` ↗](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) and [`std::str::FromStr` ↗](https://doc.rust-lang.org/std/str/trait.FromStr.html) for the `type Err` spelling.

## Po polsku

`TryFrom` to konwersja, która **ma prawo odmówić**: ten sam kształt co `From`, plus `type Error`. Cała różnica mieści się w tym jednym elemencie i cały sens także — zwraca `Result`, więc możliwość niepowodzenia stoi w typie, a nie w komentarzu.

Najpierw spotyka się implementacje ze standardu, choćby `u8::try_from(300i32)`, i właśnie na nich najlepiej widać, po co to jest. Rzutowanie przez `as` to **ta sama konwersja z usuniętym sprawdzeniem**: `300i32 as u8` daje 44 i nie mówi ani słowa. Warto zapamiętać tę różnicę w takiej właśnie formie — nie „`as` jest szybsze", tylko „`as` jest tym samym, tylko bez pytania, czy wynik ma sens".

Dwie rzeczy przydatne w praktyce. Przy zbieraniu do `Result` przetwarzanie **zatrzymuje się na pierwszej odmowie** i oddaje ten jeden błąd, co zwykle jest dokładnie tym, o co chodziło. A gdy konwersja idzie **z tekstu**, właściwą cechą nie jest `TryFrom`, tylko `FromStr` — to ona stoi za `.parse()`, którego i tak używasz.

**Szukaj po polsku:** konwersje zawodne · rzutowanie przez `as` · `FromStr` i `parse` · `rust TryFrom vs as` · `rust collect into Result`
