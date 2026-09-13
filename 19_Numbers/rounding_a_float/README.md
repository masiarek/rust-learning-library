# Making a float whole

**Level:** 101 → 201 · working knowledge

**One line:** `floor`, `ceil`, `trunc` and `round` each pick a whole number from a float — four different ones once the value is negative or ends in .5 — and every one of them hands back an `f64`, so `9.0` prints as `9`, `-0.4` can round to `-0`, and the integer you wanted is still one `as` away.

```rust
fn main() {
    let x = -2.5_f64;
    println!("{} {} {} {}", x.floor(), x.ceil(), x.trunc(), x.round());   // -3 -2 -2 -3
    println!("{}", x.round_ties_even());                                  // -2
    println!("{}", x.round() as i64);                                     // -3
}
```

## Four directions

| method | goes toward | 9.1 | -1.1 | 2.5 | -2.5 |
|---|---|---|---|---|---|
| [`floor` ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.floor) | −∞ | 9 | -2 | 2 | -3 |
| [`ceil` ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.ceil) | +∞ | 10 | -1 | 3 | -2 |
| [`trunc` ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.trunc) | 0 | 9 | -1 | 2 | -2 |
| [`round` ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.round) | the nearest; a tie goes **away from 0** | 9 | -1 | 3 | -3 |
| [`round_ties_even` ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.round_ties_even) | the nearest; a tie goes **to the even neighbour** | 9 | -1 | 2 | -2 |

On the positive side the four read like their names. The negative column is where *down* stops meaning *smaller*: `floor` still goes down the number line, so -1.1 becomes -2, while `trunc` drops the fraction and lands on -1, nearer zero.

`round` is not "up at .5". The rule std documents is *away from 0.0*: 2.5 rounds up to 3, and -2.5 rounds **down** to -3. A rule stated as *0.5 or more goes to the higher number* is true only above zero, and it is the version most introductions print.

`round_ties_even` (since 1.77) is the other rule: 2.5 → 2, 3.5 → 4, -2.5 → -2. It is the rule IEEE 754 arithmetic itself uses [when it stores a float](../what_a_float_stores/README.md#it-rounds-it-does-not-truncate), it is what Python's `round()` does, and accountants call it banker's rounding because over many ties it does not drift.

## Still a float

All five return `f64`. Nothing became an integer, and three things follow.

**`{}` hides the `.0`.** Display prints a whole-valued float without its fraction, so `9.1.floor()` prints as `9` and looks like an integer on the page; `{:?}` prints `9.0`. When a book shows `floor: 9`, that is Display at work, not a change of type.

**There is a negative zero.** `(-0.4).ceil()` is `-0.0`: it compares equal to `0.0`, and `{}` prints it as `-0`. An integer has no such value, so a label built from a rounded float can carry a minus sign in front of nothing. `round` and `trunc` produce it on the same inputs.

**NaN and infinity pass straight through.** `f64::NAN.round()` is NaN and `f64::INFINITY.floor()` is inf — there is no whole number to go to, and no error either. Python's `math.floor` raises on both; Rust's returns the value and lets a later line find out.

A fourth is a fact about the type rather than the methods. Above 2⁵² every `f64` is already whole — the gap between neighbours is 1 or more — so `4503599627370496.5` cannot even be written down and `round()` has nothing left to do, while `1e300.round()` is a 301-digit whole number that no integer type holds. The methods matter only in the range where a float still has a fraction.

## Three tie rules, and they disagree

std decides a .5 in three places, and only two of them agree:

| | 0.5 | 1.5 | 2.5 | -0.5 | -2.5 | rule |
|---|---|---|---|---|---|---|
| `x.round()` | 1 | 2 | 3 | -1 | -3 | away from zero |
| `x.round_ties_even()` | 0 | 2 | 2 | -0 | -2 | to even |
| `format!("{x:.0}")` | 0 | 2 | 2 | -0 | -2 | to even, on the digits actually stored |

The third row is the one to know about: **formatting rounds too**, with the even rule, and it judges the *exact* stored value rather than the digits you typed. So `{:.1}` prints 0.25 as `0.2` — 1/4 is exact, a true tie, and it goes to even — but 0.35 as `0.3` and 0.45 as `0.5`. Those two were never ties: [0.35 is stored below its half and 0.45 above it](../what_a_float_stores/README.md), and each rounds toward the value it actually is.

The trap this sets is *round to two decimals*. `round` takes no digits argument, so the folk answer is scale, round, divide:

```rust
let p = 2.675_f64;
println!("{p:.2}");                              // 2.67
println!("{:?}", (p * 100.0).round() / 100.0);   // 2.68
```

Both are correct answers to different questions. 2.675 is stored as 2.67499999999999982236, and `{:.2}` rounds that value: 2.67. But `2.675 * 100.0` is exactly `267.5` — the multiply rounded up onto a tie that was not in the data — and `round` then sends a tie away from zero: 2.68. For display, use the format precision. For arithmetic, do not round a float to two decimals at all: [scale to whole cents before the value ever becomes a float](../../09_Advanced/scaled_integers/README.md).

## `fract`, and the integer you wanted

[`fract` ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.fract) is what the others threw away — `x - x.trunc()`, with the sign of `x`. It is exact, which makes it a lens: `9.1.fract()` prints `0.09999999999999964`, because 9.1 was never 9.1. The same cut as the previous page, visible without a `{:.20}`.

To get an actual integer, cast — and say which rounding first:

| you mean | write | on -9.7 |
|---|---|---|
| truncate toward zero | `x as i64` | -9 |
| the same, said out loud | `x.trunc() as i64` | -9 |
| the nearest | `x.round() as i64` | -10 |
| down | `x.floor() as i64` | -10 |

A bare `as` is `trunc` plus saturation — `1e300 as i64` is `i64::MAX` and `NAN as i64` is 0 — which [Casting with `as`](../../29_Conversion/casting_with_as/README.md) covers. `x.round() as i64` costs one method call and reads as what it does.

## What the program prints

<!-- output:rounding_a_float -->
*Verified output of [`rounding_a_float.rs`](examples/rounding_a_float.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. FOUR DIRECTIONS
          x  floor   ceil  trunc  round  ties_even
        9.1      9     10      9      9          9
      100.7    100    101    100    101        101
       -1.1     -2     -1     -1     -1         -1
      -19.9    -20    -19    -19    -20        -20
        2.5      2      3      2      3          2
       -2.5     -3     -2     -2     -3         -2
        0.5      0      1      0      1          0
       -0.5     -1     -0     -0     -1         -0
   floor goes toward -inf, ceil toward +inf, trunc toward 0.
   round goes to the nearest, and a .5 goes AWAY from 0 — so -2.5 rounds
   to -3, which is lower. round_ties_even sends a .5 to the even neighbour.

2. STILL A FLOAT
   9.1.floor() with {}    : 9
   9.1.floor() with {:?}  : 9.0      Display drops the .0; Debug keeps it
   (-0.4).ceil()           : -0 / -0.0   == 0.0 ? true   is_sign_negative ? true
   an integer has no negative zero; this whole number does.
   NAN.round()             : NaN
   INFINITY.floor()        : inf
   1e300.round()           : a whole number 301 digits long, and no integer type holds it
   4503599627370496.5      : 4503599627370496   2^52 + 0.5 cannot be written down;
                             above 2^52 every f64 is already whole and round() changes nothing.

3. THREE TIE RULES IN std
        x   round  ties_even  {:.0}
      0.5       1          0      0
      1.5       2          2      2
      2.5       3          2      2
     -0.5      -1         -0     -0
     -1.5      -2         -2     -2
     -2.5      -3         -2     -2
   {:.N} sends a tie to even, like round_ties_even — judged on the digits actually stored:
   0.25 with {:.1} : 0.2    stored: 0.25000000000000000000
   0.35 with {:.1} : 0.3    stored: 0.34999999999999997780
   0.45 with {:.1} : 0.5    stored: 0.45000000000000001110
   0.25 is a true tie (1/4 is exact) and goes to even. 0.35 and 0.45 never were ties:
   one is stored below its half and one above, and each rounds toward the value it is.

4. TWO DECIMALS, TWO ANSWERS
   2.675 with {:.2}                : 2.67
   (2.675 * 100.0).round() / 100.0 : 2.68
   because 2.675 * 100.0 is exactly 267.5 — the multiply rounded UP onto a tie —
   while 2.675 itself is stored as 2.67499999999999982236, below it.
   {:.2} rounds the value you have; the scale idiom rounds a product that moved.
   1.005: {:.2} = 1.00, scale idiom = 1.0   (1.005 * 100.0 = 100.49999999999999: this one moved DOWN)

5. fract IS THE PART THE OTHERS THREW AWAY
     9.1.fract() = 0.09999999999999964    trunc + fract == x ? true
    -9.1.fract() = -0.09999999999999964   trunc + fract == x ? true
     2.5.fract() = 0.5                    trunc + fract == x ? true
   9.1 - 9 is not 0.1, because 9.1 was never 9.1 — the cut from the previous page, made visible.

6. THE INTEGER YOU WANTED IS STILL ONE `as` AWAY
     9.7: x as i64 =   9   x.trunc() as i64 =   9   x.round() as i64 =  10
    -9.7: x as i64 =  -9   x.trunc() as i64 =  -9   x.round() as i64 = -10
   a bare `as` is trunc plus saturation: 1e300 as i64 = 9223372036854775807, NAN as i64 = 0
   say which rounding first, then cast: `x.round() as i64` reads as what it does.
```
<!-- /output -->

## If you are coming from another language

**Python.** The four are there under the same names — `math.floor`, `math.ceil`, `math.trunc` and `round` — but they return `int`, not `float`, and that moves every edge. `math.floor(1e300)` is the exact 301-digit integer, because a Python `int` has no width; `math.floor(float('inf'))` raises `OverflowError` and `math.floor(float('nan'))` raises `ValueError`, where Rust hands back inf and NaN. The one to memorise: **Python's `round()` is Rust's `round_ties_even`, not Rust's `round`.** `round(2.5)` is `2` in Python and `2.5_f64.round()` is `3.0` in Rust — the same programmer writing the same word gets a different tie rule. `round(x, 2)` exists and returns a float; Rust has no digits argument, and the scale idiom above is what people write instead. `f'{x:.0f}'` is Rust's `{x:.0}`, both rounding ties to even on the stored value. `int(x)` is `x as i64` with one difference: Python never saturates, it raises or gives the exact integer. And `math.modf(x)` returns `fract` and `trunc` as one pair.

**ABAP.** The built-in numeric functions `floor`, `ceil`, `trunc` and `frac` are the same four operations with the same negative-side behaviour — the keyword documentation says of `trunc` and `frac` that the result is *negative if the argument is negative*, which is Rust's toward-zero rule ([num_func ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abennumerical_functions.htm)). Two things differ. ABAP's functions are overloaded on the argument, so `floor( pack )` on a packed number returns a packed number and the exactness question never arises; Rust's `floor` exists only on `f32` and `f64`. And an *assignment* rounds where Rust's cast truncates: moving a `TYPE f` value into a `TYPE i` field rounds it to an integer ([conversion rules ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abenconversion_type_f.htm)), so the ABAP habit of letting the target field do the rounding is the silent step Rust makes you spell out as `.round() as i64`. The tie rule, when you do round explicitly, is a parameter rather than a method: `round( val = x dec = 0 mode = cl_abap_math=>round_half_even )` names it per call, and with no `mode` the default is *commercial rounding* — halfway goes away from zero, which is Rust's `round` ([num_func - round ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abendec_floating_point_functions.htm)). So an ABAP instinct about a .5 is right in Rust and wrong in Python. What ABAP's `round` has that Rust's cannot: it works on `decfloat34`, a decimal type, so rounding 2.675 to two places gives 2.68 by the documented rule with no binary cut anywhere — the two-decimals trap above is a property of binary floats, and that function never meets one.

## Practice

**Half up, by hand.** Schoolbook rounding sends a .5 *up* — toward +∞ — so 2.5 is 3 and -2.5 is -2. std has ties away from zero and ties to even, and nothing for this. Write `half_up(x: f64) -> f64`.

Start with the one-liner everyone writes first, `(x + 0.5).floor()`, and check it on the six ties. Then find the two inputs where it is wrong — one is the largest `f64` below one half, the other is a number that was already whole — and write the version that reads `x` instead of adding to it.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:rounding_a_float_kata -->
*[`rounding_a_float_kata.rs`](examples/rounding_a_float_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the rounding Rust does not ship.
//!
//! Schoolbook rounding sends a .5 UP — toward +infinity — so 2.5 is 3 and
//! -2.5 is -2. std has ties-away-from-zero (`round`) and ties-to-even
//! (`round_ties_even`) and nothing for this. The one-liner everyone writes
//! first, `(x + 0.5).floor()`, is wrong on two kinds of input; the kata is
//! to find them and then write the version that is not.
//!
//!   rustc --edition 2024 rounding_a_float_kata.rs -o /tmp/rafk && /tmp/rafk

/// The one-liner. It adds to `x` before it looks, and the addition rounds.
fn half_up_naive(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// Let std find the nearest whole number; decide only the ties yourself.
/// `fract()` is exact — it is `x - x.trunc()`, and subtracting two floats
/// within a factor of two of each other loses nothing — so `== 0.5` is a real
/// test here, not a float comparison to be nervous about: `x` either IS
/// `n + 1/2` or it is not.
fn half_up(x: f64) -> f64 {
    if x.fract().abs() == 0.5 { x.floor() + 1.0 } else { x.round() }
}

fn main() {
    println!("1. ON THE TIES, BOTH VERSIONS AGREE");
    println!("   {:>6} {:>6} {:>8} {:>6} {:>10}", "x", "naive", "by hand", "round", "ties_even");
    for x in [0.5_f64, 1.5, 2.5, -0.5, -1.5, -2.5] {
        println!(
            "   {x:>6} {:>6} {:>8} {:>6} {:>10}",
            half_up_naive(x),
            half_up(x),
            x.round(),
            x.round_ties_even()
        );
    }
    println!("   -2.5 -> -2 is the schoolbook answer. round says -3; ties_even says -2, by a different rule.");

    println!("\n2. THE FIRST INPUT THAT BREAKS THE ONE-LINER");
    let edge = 0.49999999999999994_f64; // the largest f64 below one half
    println!("   x          = {edge:?}   (the largest f64 below one half)");
    println!("   x + 0.5    = {:?}   the exact sum has no f64, and the nearest one is 1.0", edge + 0.5);
    println!("   naive(x)   = {}   wrong: x is below the half and must round down", half_up_naive(edge));
    println!("   by hand(x) = {}   fract is {:?}, not 0.5, so std's round decides", half_up(edge), edge.fract());

    println!("\n3. THE SECOND: A NUMBER THAT WAS ALREADY WHOLE");
    let whole = 4503599627370497.0_f64; // 2^52 + 1: from here up, the spacing between floats is 1
    println!("   x          = {whole}   (2^52 + 1; floats this big are 1 apart)");
    println!("   x + 0.5    = {}   the .5 cannot exist, and the tie went to even — UP", whole + 0.5);
    println!("   naive(x)   = {}   a whole number moved by one", half_up_naive(whole));
    println!("   by hand(x) = {}   fract is 0, round returns x unchanged", half_up(whole));

    println!("\n4. WHY THE FIX HOLDS");
    println!("   none of floor, ceil, trunc, round or fract ever adds to x; each reads it.");
    println!("   the one-liner performs an addition first, and an addition can round.");
}
```
<!-- /source -->

<!-- output:rounding_a_float_kata -->
*Verified output of [`rounding_a_float_kata.rs`](examples/rounding_a_float_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. ON THE TIES, BOTH VERSIONS AGREE
        x  naive  by hand  round  ties_even
      0.5      1        1      1          0
      1.5      2        2      2          2
      2.5      3        3      3          2
     -0.5      0        0     -1         -0
     -1.5     -1       -1     -2         -2
     -2.5     -2       -2     -3         -2
   -2.5 -> -2 is the schoolbook answer. round says -3; ties_even says -2, by a different rule.

2. THE FIRST INPUT THAT BREAKS THE ONE-LINER
   x          = 0.49999999999999994   (the largest f64 below one half)
   x + 0.5    = 1.0   the exact sum has no f64, and the nearest one is 1.0
   naive(x)   = 1   wrong: x is below the half and must round down
   by hand(x) = 0   fract is 0.49999999999999994, not 0.5, so std's round decides

3. THE SECOND: A NUMBER THAT WAS ALREADY WHOLE
   x          = 4503599627370497   (2^52 + 1; floats this big are 1 apart)
   x + 0.5    = 4503599627370498   the .5 cannot exist, and the tie went to even — UP
   naive(x)   = 4503599627370498   a whole number moved by one
   by hand(x) = 4503599627370497   fract is 0, round returns x unchanged

4. WHY THE FIX HOLDS
   none of floor, ceil, trunc, round or fract ever adds to x; each reads it.
   the one-liner performs an addition first, and an addition can round.
```
<!-- /output -->

</details>

## See also

- [What a float actually stores](../what_a_float_stores/README.md) — why `9.1.fract()` is not 0.1, and why 0.35 and 0.45 were never ties
- [Letting the compiler reorder a float sum](../letting_the_compiler_reorder/README.md) — the next page: the order of a float sum is part of its answer
- [Casting with `as`](../../29_Conversion/casting_with_as/README.md) — the cast at the end of every row above, and what it does out of range
- [`f64` in std ↗](https://doc.rust-lang.org/std/primitive.f64.html#method.round) — `round` documents *away from 0.0*; `round_ties_even` sits directly under it
- *Learn Rust in a Month of Lunches* (MacLeod, 2024), §20.4 — the four methods on one page; its rule for `round` is the positive-side one

## Po polsku

Cztery metody robią z liczby zmiennoprzecinkowej liczbę całkowitą — i każda innej: `floor` to matematyczna **podłoga** (w stronę −∞, więc -1.1 daje -2), `ceil` to **sufit** (w stronę +∞), `trunc` to **obcięcie** części ułamkowej (w stronę zera, więc -1.1 daje -1), a `round` to **zaokrąglenie** do najbliższej. Pułapka siedzi w tym ostatnim: szkolna reguła „pięć zaokrąglamy w górę” opisuje Rusta tylko dla liczb dodatnich. `round` odsyła połówkę **od zera**, więc 2.5 daje 3, ale -2.5 daje -3, czyli mniej. Druga reguła ma własną metodę: `round_ties_even` (od wersji 1.77) odsyła połówkę do sąsiada **parzystego** — 2.5 → 2, 3.5 → 4 — i to jest to, co po polsku nazywa się *zaokrąglaniem bankierskim*; tak samo zaokrągla sam standard IEEE 754 przy zapisie liczby i tak samo działa `round()` w Pythonie.

Żadna z tych metod nie zwraca typu całkowitego — wynik nadal jest `f64`. Stąd trzy niespodzianki. `{}` nie drukuje `.0`, więc `9.1.floor()` wygląda na stronie jak `9`, a dopiero `{:?}` pokazuje `9.0`. Istnieje **ujemne zero**: `(-0.4).ceil()` daje `-0.0`, równe `0.0`, ale drukowane jako `-0` — liczba całkowita nie ma takiej wartości. A NaN i nieskończoność przechodzą bez błędu, tam gdzie Python rzuca wyjątek. Do prawdziwej liczby całkowitej prowadzi dopiero rzutowanie, i warto powiedzieć, które zaokrąglenie ma je poprzedzić: samo `x as i64` obcina w stronę zera i nasyca przy przepełnieniu, `x.round() as i64` mówi, co robi.

Trzecie miejsce, w którym std rozstrzyga połówkę, to formatowanie: `{:.0}` i `{:.2}` zaokrąglają do parzystej, ale na **faktycznie zapisanej** wartości, a nie na cyfrach, które wpisałeś. Dlatego `{:.1}` drukuje 0.25 jako `0.2`, ale 0.35 jako `0.3`, a 0.45 jako `0.5` — te dwie nigdy nie były połówkami. I dlatego popularny sposób na dwa miejsca po przecinku, `(x * 100.0).round() / 100.0`, potrafi dać inną odpowiedź niż `{:.2}`: dla 2.675 to 2.68 kontra 2.67, bo mnożenie samo zaokrągliło wynik dokładnie na 267.5. Do wyświetlania służy precyzja formatu; do liczenia — grosze jako liczby całkowite, zanim cokolwiek stanie się `f64`.

**Szukaj po polsku:** funkcja podłoga i sufit · obcięcie części ułamkowej · zaokrąglanie bankierskie · zaokrąglanie od zera · ujemne zero · `rust f64 round vs round_ties_even` · `rust floor ceil trunc fract`
