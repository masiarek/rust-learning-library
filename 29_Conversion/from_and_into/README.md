# `From` and `Into`

**Level:** 201 · working knowledge

**One line:** Write `From` and you get `Into` free — and the `?` operator starts converting your errors for you, which is the reason this trait matters more than it looks.

```rust
#[derive(Debug)]
struct Score(u8);

impl From<u8> for Score {
    fn from(n: u8) -> Self { Score(n.min(5)) }
}

fn main() {
    let a = Score::from(4u8);
    let b: Score = 4u8.into();
    println!("{a:?} {b:?}");   // Score(4) Score(4)
}
```

**Never implement `Into` by hand.** std carries a blanket impl — `impl<T, U: From<T>> Into<U> for T` — so one `From` gives you both call syntaxes, while an `Into` written directly gives you only one.

## Which one goes in a signature

| Where | Write |
|---|---|
| implementing a conversion | `From` |
| a function argument that should accept several types | `impl Into<T>` |
| a trait bound on a return or a struct field | `From`, and call `T::from(x)` |

`fn record(b: impl Into<Ballot>)` accepts a `Ballot` as well as everything convertible into one, because `From<T> for T` exists for every `T` — the **reflexive impl**, and the reason `.into()` sometimes converts nothing at all. The cost is monomorphisation: one copy of the function per argument type, which is why std uses `impl Into<String>` sparingly rather than everywhere.

## Where it earns its place: `?`

```rust
fn read_cell(cell: &str) -> Result<Score, TallyError> {
    let n = digits(cell)?;    // BadNumber  -> TallyError
    let n = in_range(n)?;     // OutOfRange -> TallyError
    Ok(Score::from(n))
}
```

Two helpers, two unrelated error types, one return type, and no `map_err` anywhere: **`?` calls `From::from` on the error whenever the types differ.** That single rule is what makes an error enum with a `From` impl per variant the standard shape of Rust error handling, and it is what `thiserror`'s `#[from]` attribute generates.

Delete the impls and both lines stop compiling, with an error that says so in as many words:

```text
error[E0277]: `?` couldn't convert the error to `TallyError`
  --> q.rs:14:25
   |
13 | fn read_cell(cell: &str) -> Result<u8, TallyError> {
   |                             ---------------------- expected `TallyError` because of this
14 |     let n = digits(cell)?;
   |             ------------^ the trait `From<BadNumber>` is not implemented for `TallyError`
   |             |
   |             this can't be annotated with `?` because it has type `Result<_, BadNumber>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
```

## The orphan rule decides what you may write

An impl is allowed when **either** the trait or the type is local to your crate:

| impl | allowed? |
|---|---|
| `From<u8> for Score` | yes — `Score` is yours |
| `From<Score> for u8` | yes — `Score` is yours |
| `From<Score> for String` | yes — `Score` is yours |
| `From<Vec<u8>> for String` | **no** — neither is |

The last one is `E0117`, *"only traits defined in the current crate can be implemented for types defined outside of the crate"*, and the reason is coherence: two crates could each write that impl with different answers, and a program using both would have no way to choose. The way round it is the [newtype](../../16_Structs/newtype_score/README.md) — wrap one side in a struct of your own, and it becomes local.

## The trap: `From` cannot fail

The trait has no error type, so a conversion that cannot honour its input has exactly two options — force it into range, or panic. `Score::from(200u8)` above returns `Score(5)`, silently, because `min(5)` was the only thing the signature allowed.

That is a real decision, and often the wrong one. **If the input can be invalid, the trait you want is [`TryFrom`](../tryfrom_and_tryinto/README.md)**, whose whole difference is a `type Error`.

## If you are coming from another language

- **Python.** The closest thing is `__init__` plus a constructor classmethod — `Score.from_int(n)` — and Python's `int(x)`, `str(x)`, `list(x)` protocol (`__int__`, `__str__`, `__iter__`) is the other half. Two differences worth holding. Python's conversion protocol is defined on the *source* type (`x.__int__()`), so adding a conversion to a type you did not write means monkey-patching; Rust's is a trait implementable from either side, subject to the orphan rule above, which is monkey-patching made safe rather than forbidden. And Python's `int("abc")` raises, while `From` is not allowed to fail at all — the fallible half is a different trait, so a signature tells you in advance whether the conversion can go wrong.
- **ABAP.** `MOVE` and the implicit conversions between compatible types are the language doing this for you, with rules you learn by being surprised — `MOVE '12X' TO lv_int` and the truncation of a `CHAR10` into a `CHAR5` both happen quietly. `From` is the same idea made explicit and opt-in: nothing converts unless somebody wrote the impl, and the impl is a named function you can read. The nearest deliberate counterpart is a conversion method on a class (`lo_score->to_int( )`) or a `CONVERT` routine, and the improvement Rust brings is `?`: an `EXPORTING` error from one method being re-raised as the caller's own exception type is what `From` on an error enum automates, where ABAP makes you write `CATCH … RAISE EXCEPTION TYPE … EXPORTING previous = …` at every level.
- **C#.** `implicit operator` and `explicit operator` map almost exactly onto `From` and `as`, and C# teaches the same lesson: implicit conversions must not lose information or throw. Rust enforces that by giving the fallible case a different trait rather than a convention.
- **Java.** No operator overloading, so this is a static factory — `Score.of(n)` — and the `?` behaviour has no counterpart at all; wrapping a checked exception in your own is written by hand at every level.

---

## The verified output

<!-- output:from_and_into -->
*Verified output of [`from_and_into.rs`](examples/from_and_into.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One impl, two call syntaxes
   Score::from(4u8) = Score(4)
   let b: Score = 4u8.into() = Score(4)   same value: true
   `Into` is never implemented by hand. std has a blanket impl —
   `impl<T, U: From<T>> Into<U> for T` — so writing From gives you
   both, and writing Into gives you one.

2. Which one to write in a signature
   takes_score(Score::from(3)) = 3
   takes_anything(3u8)         = 3
   takes_anything(Score(3))    = 3
   `impl Into<T>` in an argument accepts the converted type too,
   because From<T> for T is implemented for every T. That is the
   reflexive impl, and it is why `.into()` sometimes converts nothing.

3. The conversion clamps, which is a decision this impl made
   Score::from(9u8) = Score(5)   <- 9 became 5, silently
   `From` must not fail: the trait has no error type, so any input
   the conversion cannot honour has to be forced into range or
   panicked on. When neither is acceptable, the trait you want is
   TryFrom.

4. Where From earns its place: the `?` operator
   read_cell("4") = Ok(Score(4))
   read_cell("x") = Err(not a number: "x")
   read_cell("9") = Err(9 is above the 0-5 range)
   Two helper functions, two unrelated error types, and `?` converted
   each one on the way out — because `?` calls From::from on the error
   whenever the types differ. Delete the two impls and both `?` lines
   stop compiling with E0277, whose first line is exactly the sentence
   you want: "`?` couldn't convert the error to `TallyError`", followed
   by "the trait `From<BadNumber>` is not implemented for `TallyError`".

5. The conversions you already use
   String::from("Ada") = "Ada", "Ada".into() = "Ada"
   u64::from(42u32) = 42 — every widening integer conversion is a From
   impl, which is why `as` is never needed for one that cannot lose data.
```
<!-- /output -->

## Practice

**Three conversions, one of which the compiler forbids.** For a `Score` newtype, write `From<u8> for Score`, `From<Score> for u8` and `From<Score> for String`, then try `From<Vec<u8>> for String` and read the error out loud — including which of the two types it names, and why *both* being foreign is the problem rather than either one.

Then write `fn record<B: Into<Ballot>>(b: B)` and call it twice: once with something that converts, once with a `Ballot` itself. Say why the second call compiles at all, and what that generic parameter costs compared with taking a `Ballot` and making the caller type `.into()`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:from_and_into_kata -->
*[`from_and_into_kata.rs`](examples/from_and_into_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three conversions, one of which the compiler forbids.
//!
//!   rustc --edition 2024 from_and_into_kata.rs -o /tmp/fik && /tmp/fik

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Score(u8);

#[derive(Debug, PartialEq)]
struct Ballot {
    voter: String,
    score: Score,
}

// 1. Local type FROM a foreign one. Always allowed: Score is ours.
impl From<u8> for Score {
    fn from(n: u8) -> Self {
        Score(n.min(5))
    }
}

// 2. Foreign type FROM a local one. Also allowed, for the same reason —
//    the *local* type is what the orphan rule looks for, on either side.
impl From<Score> for u8 {
    fn from(s: Score) -> Self {
        s.0
    }
}

impl From<Score> for String {
    fn from(s: Score) -> Self {
        format!("{}/5", s.0)
    }
}

// 3. A tuple of borrowed pieces into an owned struct.
impl From<(&str, u8)> for Ballot {
    fn from((voter, score): (&str, u8)) -> Self {
        Ballot { voter: voter.to_string(), score: Score::from(score) }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/5", self.0)
    }
}

/// The signature that takes anything convertible. One function, three callers.
fn record<B: Into<Ballot>>(b: B) -> String {
    let ballot: Ballot = b.into();
    format!("{} scored {}", ballot.voter, ballot.score)
}

fn main() {
    println!("1. Both directions, written once each");
    let s = Score::from(4u8);
    let back: u8 = s.into();
    let printed: String = s.into();
    println!("   Score::from(4u8)      = {s:?}");
    println!("   u8 from that Score    = {back}");
    println!("   String from it        = {printed:?}");

    println!();
    println!("2. What the orphan rule actually says");
    println!("   impl From<u8> for Score     OK   — Score is local");
    println!("   impl From<Score> for u8     OK   — Score is local");
    println!("   impl From<Score> for String OK   — Score is local");
    println!("   impl From<Vec<u8>> for String    E0117: \"only traits defined in");
    println!("   the current crate can be implemented for types defined outside");
    println!("   of the crate\". Neither Vec nor String is ours, so this impl");
    println!("   could be written twice, in two crates, with different answers.");
    println!("   The way round it is a newtype: wrap one side in a local struct.");

    println!();
    println!("3. `impl Into<T>` in an argument position");
    println!("   record((\"Ada\", 4))                 -> {}", record(("Ada", 4u8)));
    println!("   record(Ballot {{ .. }})              -> {}",
             record(Ballot { voter: "Ben".into(), score: Score(3) }));
    println!("   The second call works because of the reflexive impl — From<T> for");
    println!("   T exists for every T — so `Into<Ballot>` accepts a Ballot and");
    println!("   converts nothing. That is why this signature is a superset of");
    println!("   taking `Ballot` directly, and costs the caller nothing.");

    println!();
    println!("4. Where the bound goes, and what it costs");
    println!("   `fn record<B: Into<Ballot>>(b: B)` is monomorphised once per");
    println!("   argument type, so the convenience is paid for in code size, not");
    println!("   at run time. Taking `Ballot` and making the caller write");
    println!("   `.into()` is one generic parameter cheaper and one keystroke");
    println!("   more — which is the trade, and it is why std's own APIs use");
    println!("   `impl Into<String>` sparingly.");

    println!();
    println!("5. The conversion that should not be a From at all");
    println!("   Score::from(200u8) = {:?}   <- clamped, silently", Score::from(200u8));
    println!("   `From` promises success, so this impl had to choose between");
    println!("   clamping and panicking, and it chose the one that loses data");
    println!("   quietly. A score arriving as 200 is a bug in the caller, and the");
    println!("   right trait for reporting it is TryFrom.");
}
```
<!-- /source -->

<!-- output:from_and_into_kata -->
*Verified output of [`from_and_into_kata.rs`](examples/from_and_into_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Both directions, written once each
   Score::from(4u8)      = Score(4)
   u8 from that Score    = 4
   String from it        = "4/5"

2. What the orphan rule actually says
   impl From<u8> for Score     OK   — Score is local
   impl From<Score> for u8     OK   — Score is local
   impl From<Score> for String OK   — Score is local
   impl From<Vec<u8>> for String    E0117: "only traits defined in
   the current crate can be implemented for types defined outside
   of the crate". Neither Vec nor String is ours, so this impl
   could be written twice, in two crates, with different answers.
   The way round it is a newtype: wrap one side in a local struct.

3. `impl Into<T>` in an argument position
   record(("Ada", 4))                 -> Ada scored 4/5
   record(Ballot { .. })              -> Ben scored 3/5
   The second call works because of the reflexive impl — From<T> for
   T exists for every T — so `Into<Ballot>` accepts a Ballot and
   converts nothing. That is why this signature is a superset of
   taking `Ballot` directly, and costs the caller nothing.

4. Where the bound goes, and what it costs
   `fn record<B: Into<Ballot>>(b: B)` is monomorphised once per
   argument type, so the convenience is paid for in code size, not
   at run time. Taking `Ballot` and making the caller write
   `.into()` is one generic parameter cheaper and one keystroke
   more — which is the trade, and it is why std's own APIs use
   `impl Into<String>` sparingly.

5. The conversion that should not be a From at all
   Score::from(200u8) = Score(5)   <- clamped, silently
   `From` promises success, so this impl had to choose between
   clamping and panicking, and it chose the one that loses data
   quietly. A score arriving as 200 is a bug in the caller, and the
   right trait for reporting it is TryFrom.
```
<!-- /output -->

</details>

---

## See also

- [`TryFrom` and `TryInto`](../tryfrom_and_tryinto/README.md) — the same shape with a `type Error`, for when the conversion can refuse
- [Casting with `as`](../casting_with_as/README.md) — the built-in conversion that has neither a trait nor a check
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the newtype the orphan rule sends you to
- [`main` can return a `Result`](../../02_Errors/main_returns_result/README.md) — where `?` and its `From` conversion end up
- [`ToOwned`](../../12_Traits/to_owned/README.md) — `Clone` for types whose owned twin is a different type, and the fourth conversion trait
- [What a trait is](../../12_Traits/what_a_trait_is/README.md) — if the word *blanket impl* above was new

## Sources

[Conversion: From and Into ↗](https://doc.rust-lang.org/rust-by-example/conversion/from_into.html) in Rust by Example; [`std::convert::From` ↗](https://doc.rust-lang.org/std/convert/trait.From.html), whose own docs state the "prefer implementing From" rule; and the [orphan rules ↗](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules) in the Reference, which rustc's `E0117` note links to directly.

## Po polsku

Napisz `From`, a `Into` dostajesz za darmo — w bibliotece standardowej stoi implementacja ogólna, która robi to za ciebie, więc obu nigdy się nie pisze. Stąd reguła, o którą pytają najczęściej: **`From` się implementuje, `Into` się używa w sygnaturze** (`fn f(x: impl Into<String>)`), bo ta druga forma przyjmuje więcej typów po stronie wywołania.

Prawdziwy powód, dla którego ta cecha znaczy więcej, niż wygląda, to operator `?`. Przy wyjściu z funkcji `?` **sam konwertuje** błąd na typ zadeklarowany w sygnaturze, o ile istnieje pasujące `From` — i to jest cały mechanizm, dzięki któremu jedna funkcja zwraca jeden typ błędu, choć w środku woła trzy biblioteki o trzech różnych błędach. Kto o tym nie wie, pisze ręczne `map_err` przy każdym wywołaniu i uznaje obsługę błędów w Ruscie za rozwlekłą.

Dwie granice. **Reguła sieroty** (*orphan rule*) zabrania implementować cudzą cechę dla cudzego typu — w praktyce konwersji między dwoma typami z obcych bibliotek po prostu nie napiszesz i trzeba opakować jedną stronę we własny typ. I rzecz do sprawdzenia, zanim sięgniesz po `From`: **ta konwersja nie może zawieść**. Nie ma w niej miejsca na błąd, więc gdy przeliczenie bywa niemożliwe, właściwą cechą jest `TryFrom`.

**Szukaj po polsku:** konwersje typów w Ruscie · reguła sieroty · operator znaku zapytania · `rust From Into blanket impl` · `rust ? error conversion`
