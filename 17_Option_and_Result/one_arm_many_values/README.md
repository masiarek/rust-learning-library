# One arm, many values

**Level:** 101 → 201 · for newcomers

**One line:** `8 | 12 | 18` and `0..=7` are two ways to write a single `match` arm that accepts more than one value — and the interesting half of the lesson is which mistakes the compiler catches when you use them, and which one it cannot.

Almost every `match` starts out with one arm per value, because that is the shape the problem arrives in. Twenty-five hours of the day, each with an advertising slot:

```rust
match hour {
    0 => "Classic video bundle",
    1 => "Classic video bundle",
    2 => "Classic video bundle",
    // ...twenty more...
    24 => "Season ticket",
    _ => "NOT A VALID HOUR",
}
```

Nothing is wrong with it. It is just twenty-six lines of a function whose entire content is four answers, and every one of those lines is a place to make a typo.

## The two ways to widen an arm

**An or-pattern** — `|` between patterns — makes one arm accept any of several:

```rust
8 | 12 | 18 => "Food",
```

**A range pattern** — `..=` between endpoints — makes one arm accept a span:

```rust
0..=7 => "Classic video bundle",
```

And they compose, because `|` joins *patterns*, not numbers — so each side of it may itself be a range:

```rust
9..=11 | 13..=17 => "Clothing",
```

Which is the whole function, in five arms:

```rust
fn commercials(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic video bundle",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}
```

Same behaviour, twenty-one fewer lines, and now the four answers are visible as four answers.

## `|` in a pattern is not `|` in an expression

This is the one piece of pure syntax to get straight, because the same three tokens mean two unrelated things depending on where they sit:

```rust
let mask = 8 | 12 | 18;              // 30 — bitwise or, in an EXPRESSION
matches!(hour, 8 | 12 | 18)          // "any one of three" — alternation, in a PATTERN
```

An expression computes; a pattern describes a shape. `30` does not match the pattern `8 | 12 | 18`, and there is no way to ask a pattern for the bitwise answer — patterns cannot compute at all. If you want the [bit mask](../../19_Numbers/bit_flags/README.md), that is a different tool on the other side of the `=>`.

The same split applies to `..=`. As an expression `0..=7` is a `RangeInclusive` you can iterate; as a pattern it is a test, and the exclusive form is the usual off-by-one away:

```rust
matches!(7, 0..=7)   // true  — inclusive, and what you almost always want
matches!(7, 0..7)    // false — stops one short
```

Hours, scores and grades all have a real value at the top of the range, so `..=` is the default and `..` is the one to justify.

## The hazard: the first matching arm wins

`match` takes the **first** arm that matches, so the moment an arm gets wider, every later arm it now covers is dead code. Halfway through collapsing the hours you will have exactly that — the new wide arm above, the old single-value arms still below it — and the compiler says so:

```text
warning: unreachable pattern
 --> main.rs:5:9
  |
3 |         8 | 12 | 18 => "Food commercials",
  |         ----------- matches all the relevant values
4 |         9 => "Clothing commercials",
5 |         12 => "Food commercials",
  |         ^^ no value can reach this
  |
  = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default
```

Mid-refactor that warning is bookkeeping: the dead arm and the live one give the same answer, so deleting it changes nothing. Worth [reading rather than clearing](../../15_First_Programs/what_a_warning_is_asking/README.md), though, because it is the *same warning* when they disagree.

## What rustc actually checks, which is finer than you would guess

Put the ranges in the wrong order and one of them eats a value belonging to the arm below:

```rust
0..=7 => "Classic video bundle",
9..=17 => "Clothing",           // too wide — 12 is a Food hour
8 | 12 | 18 => "Food",
```

That arm is **not** dead. Hours 8 and 18 reach it every day, so "is this arm reachable?" answers yes and a checker asking only that question would have nothing to report. rustc asks it of **each alternative separately**:

```text
warning: unreachable pattern
  |
4 |         9..=17 => "Clothing",
  |         ------ matches all the relevant values
5 |         8 | 12 | 18 => "Food",
  |             ^^ no value can reach this
```

The caret is under the `12`, not under the arm. So an or-pattern is not merely shorthand for repeating the arm — it is a list the compiler tracks element by element, and it will tell you which element you lost.

There is a second lint here too, and it is not about reachability at all. Write `0..7` where you meant `0..=7` and every arm stays reachable — hour 7 just quietly falls through to the catch-all — but rustc still notices the shape:

```text
warning: multiple ranges are one apart
  |
3 |         0..7 => "Classic video bundle",
  |         ^^^^ this range doesn't match `7_u32` because `..` is an exclusive range
4 |         8 | 12 | 18 => "Food",
  |         - this could appear to continue range `0_u32..7_u32`, but `7_u32` isn't matched by either of them
  |
  = help: use an inclusive range instead: `0_u32..=7_u32`
```

Two ranges left exactly one value apart is a typo often enough that it is worth a default-on lint, and the lint names the value you dropped.

## The one it cannot catch

Write `19..=25` instead of `19..=24` and there is nothing for either lint to say. No arm is dead — the catch-all below still serves 26, 27, 28. No two ranges are one apart. Hour 25 has simply stopped being an invalid hour, and the program is internally consistent about it.

This is the honest boundary of exhaustiveness checking, and it is worth stating plainly because the guarantee is so often overstated: **the compiler proves every value is handled, never that it is handled correctly.** Collapsing a `match` is a refactor, and the only claim a refactor makes is that the answers did not change — which, for a function of one small integer, you can check on *every* input in about four lines. That is what the [kata](#practice) below does, and it finds all three mistakes including the invisible one.

## Every alternative must bind the same names

Alternatives may bind variables, but all of them must bind the *same* names at the *same* types — otherwise the arm's body would face a variable that sometimes does not exist:

```rust
Some(x) | None => x,     // error[E0408]: variable `x` is not bound in all patterns
```

When both sides genuinely carry the same thing, this is useful rather than a restriction, and it works in a `let` too when the alternatives are exhaustive between them:

```rust
let (Ok(n) | Err(n)) = outcome;   // n, whichever way it went
```

`|` may also sit **inside** a constructor rather than outside it, which is usually the tidier spelling:

```rust
Some(8 | 12 | 18)                    // since Rust 1.53
Some(8) | Some(12) | Some(18)        // the older form, and why old code repeats itself
```

## A guard belongs to the arm, not the last alternative

```rust
8 | 12 | 18 if !fasting => "Food",
8 | 12 | 18 => "Water",
```

The `if` is not attached to `18`. Any alternative may match, and the guard then decides whether the arm runs at all — if it declines, matching resumes at the **next** arm, which is how the same three values reach two different answers above. One consequence worth carrying: a guarded arm is invisible to the exhaustiveness checker, because the compiler cannot evaluate `!fasting`, so it still demands a catch-all.

## Run it

```bash
rustc --edition 2024 17_Option_and_Result/one_arm_many_values/examples/one_arm_many_values.rs -o /tmp/oamv && /tmp/oamv
```

<!-- output:one_arm_many_values -->
*Verified output of [`one_arm_many_values.rs`](examples/one_arm_many_values.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Twenty-six arms become five
  checked every hour 0..=30, disagreements: 0
  hour 8 -> "Food"   hour 12 -> "Food"   hour 25 -> "NOT A VALID HOUR"
      `8 | 12 | 18` is ONE arm that accepts any of three values, and
      `0..=7` is one arm that accepts eight. The two compose — `9..=11 |
      13..=17` is a single pattern — because `|` joins patterns, not
      numbers. Nothing about the behaviour changed; 21 lines went away.

──── Step 2: The first arm that matches wins, and the rest are dead
  halfway(12) -> "Food"
      The `12 => "Clothing"` arm below it never runs. `match` takes the
      FIRST arm that matches, so widening an arm silently retires every
      later arm it now covers — and rustc says so:
          warning: unreachable pattern
          8 | 12 | 18 => ...   matches all the relevant values
          12 => ...            ^^ no value can reach this
      Here the dead arm agreed with the live one, so deleting it changes
      nothing. That is the lucky case, and it is why the warning is worth
      reading rather than clearing: it is the same warning either way.

──── Step 3: rustc checks each ALTERNATIVE, not each arm
  too_wide( 8) -> "Food"                 correct: "Food"
  too_wide(12) -> "Clothing"             correct: "Food"   <- wrong
  too_wide(18) -> "Food"                 correct: "Food"
      One arm of the pair is still perfectly reachable — 8 and 18 arrive
      there every day — so "is this arm dead?" is the wrong question, and
      a compiler that asked it would have nothing to report. rustc asks it
      of each alternative separately and puts the caret under the one that
      lost:
          9..=17 => ...          matches all the relevant values
          8 | 12 | 18 => ...         ^^ no value can reach this
      That is a real bug caught for free. The version of this mistake the
      compiler CANNOT catch is a range that is too wide with no later arm
      to contradict it — nothing is unreachable, and hour 12 just quietly
      sells clothing. Step 1's agreement loop is how you find that one.

──── Step 4: Three tokens, two meanings: `8 | 12 | 18`
  as an expression:  let mask = 8 | 12 | 18;   -> 30
  as a pattern:      matches!(12, 8 | 12 | 18) -> true
  as a pattern:      matches!(30, 8 | 12 | 18) -> false
      Identical characters, unrelated jobs. In an EXPRESSION `|` is bitwise
      or, and 8|12|18 is the single number 30. In a PATTERN it is
      alternation, and the same text means "any one of these three" — which
      is why 30 does not match it. Position decides, and there is no way to
      ask for the other one: a pattern cannot compute.
  0..=7 includes 7:  matches!(7, 0..=7) -> true
  0..7  excludes 7:  matches!(7, 0..7)  -> false
      `..=` is the inclusive form and the one you almost always want for
      hours, scores and grades, because the top of the range is a real
      value. `..` stops one short — the same off-by-one as everywhere else.

──── Step 5: Every alternative must bind the same names
  let (Ok(n) | Err(n)) = Err(7);   -> n = 7
      Useful when both arms carry the same thing and you do not care yet
      which one you got — a line number, a parsed value, a ballot id.
  Some(x) | None => x              -> does not compile
      error[E0408]: variable `x` is not bound in all patterns
      There is no value for `x` when the None side matched, and Rust will
      not invent one. Same names, same types, on every alternative.
  Some(8)  Some(8 | 12 | 18) -> true  Some(8) | Some(12) | Some(18) -> true
  Some(9)  Some(8 | 12 | 18) -> false Some(8) | Some(12) | Some(18) -> false
  None     Some(8 | 12 | 18) -> false Some(8) | Some(12) | Some(18) -> false
      Those two spellings mean the same thing. The nested form has been
      allowed since Rust 1.53; before that `|` only worked at the top of an
      arm, which is why older code repeats the constructor.

──── Step 6: A guard covers the whole arm
  hour 12, fasting true  -> "Water"
  hour 12, fasting false -> "Food"
      The `if` is not attached to `18`; it is attached to the arm. Any
      alternative may match, and then the guard decides whether the arm
      runs at all — if it says no, matching continues at the NEXT arm,
      which is how the same three values reach two different answers here.
      Note also that a guard makes an arm invisible to the exhaustiveness
      checker: the compiler cannot evaluate `!fasting`, so it assumes the
      arm might not run and still demands a catch-all.
```
<!-- /output -->

## If you are coming from another language

- **Python.** `case 8 | 12 | 18:` is the same character doing the same job — `match`/`case` (3.10) borrowed or-patterns more or less directly, and `case 0 | 1 | 2` reads identically. What is new is the checking either side of it: Python's `match` has no exhaustiveness requirement and no unreachable-case warning, so a `case` shadowed by an earlier one is simply never taken and nobody mentions it. Python also has no range pattern — `case n if 0 <= n <= 7` is a guard, which means it is opaque to any analysis. And the old spelling, `if hour in (8, 12, 18)`, stays perfectly good Python; the pattern form earns its place only when you are destructuring something.
- **ABAP.** `CASE hour. WHEN 8 OR 12 OR 18.` is exactly this, and the slide that starts most Rust courses on the topic says so by writing both lines above each other. Two things change. `WHEN OTHERS` is optional and a `CASE` that falls off the end silently does nothing, whereas Rust will not compile a `match` with a hole in it — the catch-all is a decision you have to make in writing. And ABAP has no range in a `WHEN`, so `0..=7` is a `WHEN 0 OR 1 OR 2 OR …` or an `IF` ladder; if you have ever written `IF hour BETWEEN 0 AND 7` in a chain of `ELSEIF`s to avoid the typing, `0..=7` is that, with the compiler now checking the chain covers everything and that no rung is dead.

## Practice

**The hour that changed its ad.** Take the twenty-six-arm `commercials` above and collapse it to five arms yourself. Then write the check that a refactor actually owes you: a loop over every hour `0..=30` comparing the new function against the old one, printing any input where they disagree.

Now break it three ways, one character each, and predict *before compiling* which the compiler will catch:

1. write the Clothing arm as `9..=17`, above the Food arm;
2. write the Classic arm as `0..7`;
3. write the Season ticket arm as `19..=25`.

For each: does it change an answer, and does rustc say anything? The tally is not three, and it is not zero.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:one_arm_many_values_kata -->
*[`one_arm_many_values_kata.rs`](examples/one_arm_many_values_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: collapse twenty-six arms into five, then find the three
//! collapses that look right and are not.
//!
//! The point is which of the three the compiler finds for you (one) and which it
//! cannot possibly find (two), and what you have to write yourself to cover the
//! difference.
//!
//!   rustc --edition 2024 one_arm_many_values_kata.rs -o /tmp/oamvk && /tmp/oamvk

/// The original: one arm per hour. This is the answer key for everything below.
fn original(hour: u32) -> &'static str {
    match hour {
        0 => "Classic",
        1 => "Classic",
        2 => "Classic",
        3 => "Classic",
        4 => "Classic",
        5 => "Classic",
        6 => "Classic",
        7 => "Classic",
        8 => "Food",
        9 => "Clothing",
        10 => "Clothing",
        11 => "Clothing",
        12 => "Food",
        13 => "Clothing",
        14 => "Clothing",
        15 => "Clothing",
        16 => "Clothing",
        17 => "Clothing",
        18 => "Food",
        19 => "Season ticket",
        20 => "Season ticket",
        21 => "Season ticket",
        22 => "Season ticket",
        23 => "Season ticket",
        24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 1 — the collapse.
fn collapsed(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 2a — the Clothing range swallows the middle Food hour.
/// rustc: `warning: unreachable pattern` with the caret under `12`.
#[allow(unreachable_patterns)]
fn wrong_swallowed(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic",
        9..=17 => "Clothing",
        8 | 12 | 18 => "Food",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 2b — `..` where `..=` was meant. Every arm is still reachable, so the
/// unreachable check has nothing to say — but a SECOND lint does, because the
/// gap it opens is exactly one wide and the next arm starts on the far side of it.
#[allow(non_contiguous_range_endpoints)]
fn wrong_exclusive(hour: u32) -> &'static str {
    match hour {
        0..7 => "Classic",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 2c — one hour too generous at the top. This one nothing catches.
fn wrong_too_far(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=25 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// The proof: every input, compared against the answer key.
fn disagreements(f: fn(u32) -> &'static str) -> Vec<(u32, &'static str, &'static str)> {
    (0..=30)
        .filter(|&h| f(h) != original(h))
        .map(|h| (h, original(h), f(h)))
        .collect()
}

fn main() {
    let candidates: [(&str, fn(u32) -> &'static str, &str); 4] = [
        ("collapsed", collapsed, "—"),
        ("wrong_swallowed", wrong_swallowed, "yes: unreachable pattern, caret under `12`"),
        ("wrong_exclusive", wrong_exclusive, "yes: non_contiguous_range_endpoints"),
        ("wrong_too_far", wrong_too_far, "no"),
    ];

    println!("Every hour 0..=30, against the twenty-six-arm original:\n");
    for (name, f, warned) in candidates {
        let bad = disagreements(f);
        println!("  {name:<16} disagreements: {}", bad.len());
        for (hour, want, got) in &bad {
            println!("      hour {hour:>2}: original {want:?}, this version {got:?}");
        }
        println!("      compiler warned? {warned}");
        println!();
    }

    println!("What the three mistakes have in common is that each is one character:");
    println!("  9..=11 | 13..=17  ->  9..=17     a range that ate a value from the arm below");
    println!("  0..=7             ->  0..7       inclusive to exclusive");
    println!("  19..=24           ->  19..=25    one hour past the end of the day");
    println!();
    println!("What they do not have in common is whether anyone tells you — and the");
    println!("count is two out of three, from two different lints:");
    println!();
    println!("  warning: unreachable pattern");
    println!("      9..=17 => ...            matches all the relevant values");
    println!("      8 | 12 | 18 => ...           ^^ no value can reach this");
    println!();
    println!("  warning: multiple ranges are one apart");
    println!("      0..7 => ...              this range doesn't match `7_u32` because");
    println!("                               `..` is an exclusive range");
    println!("      8 | 12 | 18 => ...       this could appear to continue range `0..7`,");
    println!("                               but `7_u32` isn't matched by either of them");
    println!("      help: use an inclusive range instead: `0_u32..=7_u32`");
    println!();
    println!("The second one is worth knowing about, because it is not an exhaustiveness");
    println!("check at all — every hour is still handled, hour 7 merely falls through to");
    println!("the catch-all. `non_contiguous_range_endpoints` is a lint about the SHAPE of");
    println!("your arms: two ranges left exactly one value apart is a typo often enough");
    println!("that rustc says so on sight, and it names the missing value.");
    println!();
    println!("Which leaves the third, and nothing catches the third. `19..=25` makes hour");
    println!("25 a valid hour; the catch-all below it still has 26, 27, 28 to serve, so no");
    println!("arm is dead and no two ranges are one apart. There is nothing structurally");
    println!("wrong for a compiler to notice. Exhaustiveness says every value is handled;");
    println!("it never claimed the handler was the right one.");
    println!();
    println!("So the loop above is not ceremony. Collapsing a match is a refactor, and the");
    println!("only claim a refactor makes is that the answers did not change — which is a");
    println!("claim you can check on every input, for a function this small, in four lines.");
}
```
<!-- /source -->

<!-- output:one_arm_many_values_kata -->
*Verified output of [`one_arm_many_values_kata.rs`](examples/one_arm_many_values_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Every hour 0..=30, against the twenty-six-arm original:

  collapsed        disagreements: 0
      compiler warned? —

  wrong_swallowed  disagreements: 1
      hour 12: original "Food", this version "Clothing"
      compiler warned? yes: unreachable pattern, caret under `12`

  wrong_exclusive  disagreements: 1
      hour  7: original "Classic", this version "NOT A VALID HOUR"
      compiler warned? yes: non_contiguous_range_endpoints

  wrong_too_far    disagreements: 1
      hour 25: original "NOT A VALID HOUR", this version "Season ticket"
      compiler warned? no

What the three mistakes have in common is that each is one character:
  9..=11 | 13..=17  ->  9..=17     a range that ate a value from the arm below
  0..=7             ->  0..7       inclusive to exclusive
  19..=24           ->  19..=25    one hour past the end of the day

What they do not have in common is whether anyone tells you — and the
count is two out of three, from two different lints:

  warning: unreachable pattern
      9..=17 => ...            matches all the relevant values
      8 | 12 | 18 => ...           ^^ no value can reach this

  warning: multiple ranges are one apart
      0..7 => ...              this range doesn't match `7_u32` because
                               `..` is an exclusive range
      8 | 12 | 18 => ...       this could appear to continue range `0..7`,
                               but `7_u32` isn't matched by either of them
      help: use an inclusive range instead: `0_u32..=7_u32`

The second one is worth knowing about, because it is not an exhaustiveness
check at all — every hour is still handled, hour 7 merely falls through to
the catch-all. `non_contiguous_range_endpoints` is a lint about the SHAPE of
your arms: two ranges left exactly one value apart is a typo often enough
that rustc says so on sight, and it names the missing value.

Which leaves the third, and nothing catches the third. `19..=25` makes hour
25 a valid hour; the catch-all below it still has 26, 27, 28 to serve, so no
arm is dead and no two ranges are one apart. There is nothing structurally
wrong for a compiler to notice. Exhaustiveness says every value is handled;
it never claimed the handler was the right one.

So the loop above is not ceremony. Collapsing a match is a refactor, and the
only claim a refactor makes is that the answers did not change — which is a
claim you can check on every input, for a function this small, in four lines.
```
<!-- /output -->

</details>

## See also

- [`Some` and `None`](../some_and_none/README.md) — where `match` is introduced, and the arms that make it exhaustive
- [`if let`](../if_let/README.md) — one arm instead of all of them, and what the compiler stops checking in exchange
- [Six kinds of zero](../six_kinds_of_zero/README.md) — a `match` over ballot markers, and the catch-all that quietly refiled two of them
- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — including the unreachable arm, and why the fix is rarely to silence it
- [Bit flags](../../19_Numbers/bit_flags/README.md) — the *other* `|`, on the computing side of the `=>`
- [Rust reference: pattern syntax ↗](https://doc.rust-lang.org/reference/patterns.html) · [`unreachable_patterns` ↗](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unreachable-patterns)

## Po polsku

Jedno ramię `match`a może przyjmować wiele wartości na dwa sposoby: przez **alternatywę wzorców** (*or-pattern*) — `8 | 12 | 18` — i przez **wzorzec zakresowy** (*range pattern*) — `0..=7`. Składają się ze sobą, bo `|` łączy *wzorce*, a nie liczby, więc każda jego strona może sama być zakresem: `9..=11 | 13..=17` to jeden wzorzec. Z dwudziestu sześciu ramion robi się pięć i dopiero wtedy widać, że ta funkcja ma cztery odpowiedzi, a nie dwadzieścia sześć.

Polszczyzna ma tu przewagę nad angielskim, bo na dwa znaczenia tego samego znaku ma dwa różne słowa. W **wyrażeniu** `8 | 12 | 18` to **suma bitowa** i wynosi 30; we **wzorcu** to **alternatywa** i znaczy „którakolwiek z tych trzech” — dlatego 30 do wzorca `8 | 12 | 18` nie pasuje i nie ma sposobu, żeby poprosić wzorzec o wynik bitowy: wzorzec niczego nie oblicza, tylko opisuje kształt. Tak samo rozdwaja się `..=`: jako wyrażenie to `RangeInclusive`, po którym można iterować, a jako wzorzec to test — i `matches!(7, 0..7)` daje `false`. Przy godzinach, ocenach i punktach górny kraniec jest prawdziwą wartością, więc formą domyślną jest zakres domknięty `..=`, a użycie `..` trzeba sobie uzasadnić.

Dalej zaczyna się właściwa lekcja, czyli co kompilator sprawdza, a czego nie. W `switch`u z C czy Javy powtórzony `case 12:` jest po prostu błędem kompilacji, bo `case` to etykieta. W Ruscie ramiona to wzorce, wygrywa **pierwsze pasujące**, więc poszerzenie ramienia nie jest błędem — cicho unieważnia wszystkie późniejsze ramiona, które właśnie przykryło, a rustc zgłasza to jako `warning: unreachable pattern`. Ciekawsze jest to, że pytanie „czy to ramię jest osiągalne?” bywa za grube: gdy zbyt szeroki zakres `9..=17` stanie nad `8 | 12 | 18`, tamto ramię wciąż żyje, bo godziny 8 i 18 docierają do niego codziennie. rustc zadaje pytanie **każdej alternatywie z osobna** i stawia `^` pod `12`, nie pod całym ramieniem. Trzy jednoznakowe pomyłki z ćwiczenia układają się przez to w rachunek dwa na trzy:

- `9..=11 | 13..=17` → `9..=17` — złapane, `unreachable pattern`, ze wskazaniem na `12`;
- `0..=7` → `0..7` — złapane przez zupełnie inny lint, `non_contiguous_range_endpoints` („multiple ranges are one apart”), który nawet nazywa zgubioną godzinę 7;
- `19..=24` → `19..=25` — **nie łapie tego nic**.

Ta trzecia jest uczciwą granicą całego mechanizmu i warto ją wypowiedzieć wprost, bo gwarancja bywa w polskich materiałach przesadzona: kompletność dopasowania (*exhaustiveness*) dowodzi, że **każda wartość jest obsłużona**, nigdy że jest obsłużona **poprawnie**. Po zmianie `19..=25` żadne ramię nie jest martwe (dla 26, 27, 28 wciąż zostaje ramię `_`), żadne dwa zakresy nie mijają się o jeden, a godzina 25 po prostu przestała być nieprawidłowa i program jest w tym wewnętrznie spójny. Zwijanie `match`a to refaktoryzacja, a jedyne, co refaktoryzacja obiecuje, to że odpowiedzi się nie zmieniły — przy funkcji jednego małego całkowitego sprawdzisz tę obietnicę na **wszystkich** wejściach pętlą po `0..=30`, w cztery linijki.

Na koniec dwie reguły, o które łatwo się potknąć. Wszystkie alternatywy muszą wiązać **te same nazwy tych samych typów**, inaczej ciało ramienia dostałoby zmienną, której czasem nie ma — stąd `error[E0408]` przy `Some(x) | None => x`; gdy natomiast obie strony niosą to samo, działa nawet w `let`u: `let (Ok(n) | Err(n)) = outcome;`. I strażnik (*guard*) należy do **ramienia**, a nie do ostatniej alternatywy: w `8 | 12 | 18 if !fasting` `if` nie dotyczy samej `18`, a gdy strażnik odmówi, dopasowanie rusza dalej od **następnego** ramienia — dlatego te same trzy godziny potrafią trafić do dwóch różnych odpowiedzi, „Food” albo „Water”. Konsekwencja: ramię ze strażnikiem jest niewidoczne dla sprawdzania kompletności, bo kompilator nie umie policzyć `!fasting`, więc i tak zażąda ramienia `_`.

**Szukaj po polsku:** dopasowanie wzorców · alternatywa wzorców · zakres domknięty · `rust unreachable pattern warning` · `rust non_contiguous_range_endpoints` · `rust E0408 variable not bound in all patterns`
