# *Rust: The Practical Guide*, chapter 3 exercises, run

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 3 exercises** · the book: *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025), §3.4 Practice Exercises and §3.5 Solutions

**Level:** 101 → 201 · a companion to the book's chapter 3

**One line:** Six exercises on loops, conditions and functions — two sums over `1..n`, an assembly line, a palindrome, a Pythagorean triple and a cinema rule — and the book's solutions all compile on rustc 1.98.0, with three that are wrong in a way the compiler cannot see: `as i32` truncates the rate it says it rounds (16 for 16.575), the byte-by-byte palindrome answers `false` for the book's own example sentence and for `"été"`, and the triple search looks at 80,778,750 candidates where computing `c` needs 69,676. Each exercise below is restated, compiled as given, and solved by a program whose output is the answer key; the two that read the keyboard in the book run over a fixed table here, so that the output can be one.

## The verdict

| # | Exercise | What rustc says about the given code | The book's solution | Verdict | Lesson |
|---|---|---|---|---|---|
| 3.1 | Square of the sum minus the sum of squares | the template compiles, with five `unused` warnings | `for i in 1..=n`, then `.pow(2)` on the sum | correct for the N it is tested on; its `i32` overflows at N = 304 | [`for` loops](../../../25_Control_Flow/for_loops/README.md) · [The integer types](../../../19_Numbers/the_integer_types/README.md) |
| 3.2 | Multiples of 3 or 5 below N | the same template, the same five warnings | `for i in 1..n` with `%` and `\|\|` | correct; the template's `i32` became `u32` without a word, which moves a negative N from an empty loop to a failed parse | [`for` loops](../../../25_Control_Flow/for_loops/README.md) |
| 3.3 | An assembly line | `E0308` twice: a body that is only `let success_rate: f32;` returns `()` | an `if` chain over the speed, then `hours * 221 * rate * speed` | the arithmetic is right; `as i32` truncates 16.575 to 16 where `round()` gives 17; `hours` cancels out of the per-minute rate; speeds above 10 are not rejected | [`if` expressions](../../../25_Control_Flow/if_expressions/README.md) · [Casting with `as`](../../../29_Conversion/casting_with_as/README.md) · [Making a float whole](../../../19_Numbers/rounding_a_float/README.md) |
| 3.4 | A palindrome | `E0308`: `bool` expected, `()` found | a two-index `while` over `as_bytes()` | `false` for "Able was I ere I saw Elba" and for `"été"`: right only for ASCII input in one case; clippy `len_zero`; named `palindrome`, takes a `String` by value | [Walking a `String`](../../../14_Strings/walking_a_string/README.md) · [Comparing and sorting text](../../../14_Strings/comparing_strings/README.md) · [Meet the byte](../../../19_Numbers/meet_the_byte/README.md) |
| 3.5 | The Pythagorean triple that sums to 1000 | no template | three loops and a flag | finds (200, 375, 425) after 80,778,750 candidates; `flag == false` draws clippy `bool_comparison`; two loops with `c` computed and `break 'search` need 69,676 | [Loop labels](../../../25_Control_Flow/loop_labels/README.md) |
| 3.6 | Who may see the movie | the template compiles, with two unused parameters, and prints `false` | `(age >= 17) \|\| (age >= 13 && permission)` | correct; the printed solution has lost its spaces (`fncan_see_movie`, `fnmain`) and does not compile as printed; `main` says John is 18 and passes 17 | [Meet the `bool`](../../../15_First_Programs/meet_the_bool/README.md) · [`if` expressions](../../../25_Control_Flow/if_expressions/README.md) |

Every transcript on this page is rustc 1.98.0, or its clippy, compiling a file with the name in the fence title. Every solution is a `pg3_*` program compiled and run by `tools/run_examples.py`, with its output pasted in below the source. Exercises 3.1, 3.2 and 3.5 are Project Euler's problems [6 ↗](https://projecteuler.net/problem=6), [1 ↗](https://projecteuler.net/problem=1) and [9 ↗](https://projecteuler.net/problem=9), and the programs reproduce their published answers for N = 100 and 1000.

## Practice

### 1. Square of the sum minus the sum of squares

For the first N natural numbers, compute the square of their sum minus the sum of their squares; N = 5 gives 225 − 55 = 170. The book reads N from the keyboard, and its template ends where your code starts:

```rust
fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;
    /* Complete the code after this line */
}
```

The template compiles, with five warnings — `n` and the two accumulators are unused, and the two `mut`s are not needed yet. Completed the book's way, still reading the keyboard:

```rust
fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    let mut sum = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum += i;
        sum_of_squares += i.pow(2);
    }
    let difference = sum.pow(2) - sum_of_squares;
    println!("The difference of the square of the sum and the sum of squares for N = {n} is {difference}");
}
```

Two notes on the book's version. Its accumulator is called `square_of_sum` while it holds the plain sum, and only the last line squares it; `sum` is the honest name. And every number is an `i32` because the template says so — enough for 5, 10 and 100, and not for long. The recorded program measures it: the square of the sum passes `i32::MAX` at N = 304, where a debug build panics with *attempt to multiply with overflow* and a release build wraps. The answer for N = 304, `2139838520`, would itself still fit; it is the intermediate that does not. An `i64` lasts until N = 77936. [`for` loops](../../../25_Control_Flow/for_loops/README.md) is the `1..=n`, and [The integer types](../../../19_Numbers/the_integer_types/README.md) the widths.

The recorded program takes the fixed table `[5, 10, 100]` in place of the keyboard, so that its output can be an answer key; the loop is the book's.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg3_01_squares -->
*[`pg3_01_squares.rs`](examples/pg3_01_squares.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 3.1: (1 + 2 + ... + n)^2 - (1^2 + 2^2 + ... + n^2). The book reads n
// from stdin; here a fixed table stands in for the keyboard so the output is an
// answer key. The second half measures where an i32 stops being enough.
fn difference(n: i32) -> i32 {
    let mut sum = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum += i;
        sum_of_squares += i.pow(2);
    }
    sum.pow(2) - sum_of_squares
}

/// The smallest n whose square of the sum is larger than `max`.
fn first_n_whose_square_exceeds(max: u128) -> u128 {
    let mut sum: u128 = 0;
    for n in 1.. {
        sum += n;
        if sum * sum > max {
            return n;
        }
    }
    unreachable!()
}

fn main() {
    for n in [5, 10, 100] {
        println!("The difference of the square of the sum and the sum of squares for N = {n} is {}", difference(n));
    }
    // The difference ... for N = 5 is 170
    // The difference ... for N = 10 is 2640
    // The difference ... for N = 100 is 25164150

    let n32 = first_n_whose_square_exceeds(i32::MAX as u128);
    let n64 = first_n_whose_square_exceeds(i64::MAX as u128);
    println!("the square of the sum outgrows an i32 at N = {n32}, an i64 at N = {n64}"); // ... an i32 at N = 304, an i64 at N = 77936

    let sum: u128 = (1..=n32).sum();
    let sum_of_squares: u128 = (1..=n32).map(|i| i * i).sum();
    println!("at N = {n32}: the square {} > i32::MAX {}", sum * sum, i32::MAX); // at N = 304: the square 2149249600 > i32::MAX 2147483647
    println!("but the difference {} would still fit", sum * sum - sum_of_squares); // but the difference 2139838520 would still fit
    println!("N = {}: an i32 keeps the answer {}", n32 - 1, difference(n32 as i32 - 1)); // N = 303: an i32 keeps the answer 2111836472
}
```
<!-- /source -->

<!-- output:pg3_01_squares -->
*Verified output of [`pg3_01_squares.rs`](examples/pg3_01_squares.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The difference of the square of the sum and the sum of squares for N = 5 is 170
The difference of the square of the sum and the sum of squares for N = 10 is 2640
The difference of the square of the sum and the sum of squares for N = 100 is 25164150
the square of the sum outgrows an i32 at N = 304, an i64 at N = 77936
at N = 304: the square 2149249600 > i32::MAX 2147483647
but the difference 2139838520 would still fit
N = 303: an i32 keeps the answer 2111836472
```
<!-- /output -->

</details>

### 2. Multiples of 3 or 5 below N

Sum every natural number below N that is a multiple of 3 or of 5, each counted once; N = 20 gives 78. The template is exercise 1's with a different comment. Completed the book's way:

```rust
fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("failed to read input.");
    let n: u32 = n.trim().parse().expect("invalid input");
    let mut sum: u32 = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("The sum of the multiples of 3 or 5 below {n} is {sum}");
}
```

Correct, and `||` is what makes "counted once" true: 15 is a multiple of both, and adding the two sums separately gives 93 for N = 20, not 78. One silent change: the template types `n` as an `i32`, the solution as a `u32`. Either compiles, and they fail differently on `-5`. As an `i32` it parses, `1..-5` is an empty range, and the answer is 0 with no complaint; as a `u32` the `parse()` itself fails and the `expect` stops the program with *invalid input*. The recorded program prints both. `1..n` excludes `n`, which is what "below N" needs — [`for` loops](../../../25_Control_Flow/for_loops/README.md).

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg3_02_multiples -->
*[`pg3_02_multiples.rs`](examples/pg3_02_multiples.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 3.2: the multiples of 3 or 5 below N, each counted once. A fixed
// table stands in for the book's stdin read.
fn sum_of_multiples(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn sum_of_multiples_chain(n: u32) -> u32 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

fn main() {
    for n in [10, 20, 1000] {
        println!("The sum of the multiples of 3 or 5 below {n} is {}", sum_of_multiples(n)); // 23, 78, 233168
        assert_eq!(sum_of_multiples(n), sum_of_multiples_chain(n));
    }

    // "Counted once": 15 is a multiple of both. Two separate sums count it twice.
    let threes: u32 = (1..20).filter(|i| i % 3 == 0).sum();
    let fives: u32 = (1..20).filter(|i| i % 5 == 0).sum();
    println!("below 20: multiples of 3 sum to {threes}, of 5 to {fives}, together {}", threes + fives); // ... 63, of 5 to 30, together 93
    println!("the || version says {}: 15 was counted once", sum_of_multiples(20)); // the || version says 78: 15 was counted once

    // The template types n as i32, the book's solution as u32. The type decides
    // where a negative input is caught.
    let as_i32: Result<i32, _> = "-5".trim().parse();
    let as_u32: Result<u32, _> = "-5".trim().parse();
    println!("\"-5\" as i32: {as_i32:?}; the loop 1..-5 then runs {} times", (1..-5).count()); // "-5" as i32: Ok(-5); the loop 1..-5 then runs 0 times
    println!("\"-5\" as u32: {as_u32:?}"); // "-5" as u32: Err(ParseIntError { kind: InvalidDigit })
}
```
<!-- /source -->

<!-- output:pg3_02_multiples -->
*Verified output of [`pg3_02_multiples.rs`](examples/pg3_02_multiples.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The sum of the multiples of 3 or 5 below 10 is 23
The sum of the multiples of 3 or 5 below 20 is 78
The sum of the multiples of 3 or 5 below 1000 is 233168
below 20: multiples of 3 sum to 63, of 5 to 30, together 93
the || version says 78: 15 was counted once
"-5" as i32: Ok(-5); the loop 1..-5 then runs 0 times
"-5" as u32: Err(ParseIntError { kind: InvalidDigit })
```
<!-- /output -->

</details>

### 3. An assembly line

The line runs at a speed from 0 (off) to 10. At speed 1 it makes 221 cars an hour, and the rate grows linearly with the speed. Of the cars made, all pass at speeds 1 to 4, 90% at 5 to 8 and 77% at 9 and 10. Write `total_production(hours, speed)`, the cars that pass in that many hours, and `cars_produced_per_minute(hours, speed)`. The template:

```rust,compile_fail
fn total_production(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    /* Your code below this line*/
}
fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    /* Your code below this line*/
}
fn main() {
    println!("{}", total_production(6, 5) as i32); // to round the values we use i32, which you may just ignore for now
    println!("{}", cars_produced_per_minutes(6, 5) as i32);
}
```

```text title="Abridged — real rustc output for total_production_template.rs, the same error for the second function dropped"
error[E0308]: mismatched types
 --> total_production_template.rs:1:46
  |
1 | fn total_production(hours: u8, speed: u8) -> f32 {
  |    ----------------                          ^^^ expected `f32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
```

The book's arithmetic is right: `hours as f32 * 221.0 * success_rate * speed as f32`, with the rate from an `if` / `else if` chain over `speed`, gives exactly `5967` for six hours at speed 5, and dividing by `60.0 * hours as f32` gives `16.575` a minute. Four things around it:

- **`as i32` does not round.** The template's comment says the cast is there "to round the values", and for `16.575` it prints `16`. A float-to-integer `as` truncates toward zero — [the four losses](../../../29_Conversion/casting_with_as/README.md#the-four-losses) — and `per_minute.round()` prints `17` ([Making a float whole](../../../19_Numbers/rounding_a_float/README.md)). The total happens to be a whole number, so the cast hides nothing there.
- **`hours` cancels out of the rate.** Cars per minute is `221 * speed * rate / 60` whatever the shift length: the recorded program prints the same `16.575` for 1, 6 and 24 hours, and the solution's function ignores its `hours`, named `_hours` to say so.
- **Nothing rejects a speed the line does not have.** The chain's `else` takes every speed above 8, so `total_production(1, 11)` returns `1871.87` without complaint. The `u8` the book chose for `speed` stops negatives at the type and nothing else; the recorded program shows the gap rather than closing it, because closing it is a design decision — a panic, an `Option`, or a clamp to 10.
- **Smaller.** The template's `cars_produced_per_minutes` is the exercise's `cars_produced_per_minute`. `match speed { 0..=4 => 1.0, 5..=8 => 0.9, _ => 0.77 }` says the three bands in three lines where the chain needs `speed >= 5 && speed <= 8` ([`if` expressions](../../../25_Control_Flow/if_expressions/README.md)). And `16.575` is not an `f32` exactly, or any binary float; it prints that way because Rust prints the shortest decimal that reads back to the same float — [Machine numbers ↗](https://masiarek.github.io/math-learning-library/01_Precision/machine_numbers/index.html).

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg3_03_assembly -->
*[`pg3_03_assembly.rs`](examples/pg3_03_assembly.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 3.3: an assembly line makes 221 cars per hour per unit of speed, and
// some of them fail. `as i32` is what the book prints with; `round()` is what it meant.
fn success_rate(speed: u8) -> f32 {
    match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        _ => 0.77,
    }
}

fn total_production(hours: u8, speed: u8) -> f32 {
    hours as f32 * 221.0 * speed as f32 * success_rate(speed)
}

fn cars_produced_per_minute(_hours: u8, speed: u8) -> f32 {
    221.0 * speed as f32 * success_rate(speed) / 60.0
}

fn main() {
    let total = total_production(6, 5);
    let per_minute = cars_produced_per_minute(6, 5);
    println!("total_production(6, 5) = {total}"); // total_production(6, 5) = 5967
    println!("cars_produced_per_minute(6, 5) = {per_minute}"); // cars_produced_per_minute(6, 5) = 16.575
    println!("the book prints them `as i32`: {} and {}", total as i32, per_minute as i32); // the book prints them `as i32`: 5967 and 16
    println!("rounded, as the comment says: {} and {}", total.round(), per_minute.round()); // rounded, as the comment says: 5967 and 17
    println!("the hours cancel out of the per-minute rate: {} {} {}", cars_produced_per_minute(1, 5), cars_produced_per_minute(6, 5), cars_produced_per_minute(24, 5)); // 16.575 16.575 16.575
    println!("speed 0 is off: {}", total_production(6, 0)); // speed 0 is off: 0
    println!("nothing rejects speed 11: {}", total_production(1, 11)); // nothing rejects speed 11: 1871.87
}
```
<!-- /source -->

<!-- output:pg3_03_assembly -->
*Verified output of [`pg3_03_assembly.rs`](examples/pg3_03_assembly.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
total_production(6, 5) = 5967
cars_produced_per_minute(6, 5) = 16.575
the book prints them `as i32`: 5967 and 16
rounded, as the comment says: 5967 and 17
the hours cancel out of the per-minute rate: 16.575 16.575 16.575
speed 0 is off: 0
nothing rejects speed 11: 1871.87
```
<!-- /output -->

</details>

### 4. A palindrome

Write `is_palindrome`, which takes a string and says whether it reads the same backwards; the book's examples are "Able was I ere I saw Elba" and 1881. The template names it `palindrome` and hands it a `String`:

```rust,compile_fail
fn palindrome(input: String) -> bool {
    /* Your Code here */
}
fn main() {
    let input = String::from("1211");
    println!("It is {:?} that the given string is palindrome", palindrome(input));
}
```

```text title="Abridged — real rustc output for palindrome_template.rs"
error[E0308]: mismatched types
 --> palindrome_template.rs:1:33
  |
1 | fn palindrome(input: String) -> bool {
  |    ----------                   ^^^^ expected `bool`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
```

The book's solution walks `input.as_bytes()` from both ends with two indexes and a `while first < last`, and returns `false` at the first pair of bytes that differ. It compiles, passes clippy with one warning, and prints `false` for `"1211"`, which is right:

```text title="Abridged — real clippy-driver output for palindrome_book.rs, the book's solution"
warning: length comparison to zero
 --> palindrome_book.rs:3:8
  |
3 |     if input.len() == 0 {
  |        ^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `input.is_empty()`
  |
  = note: `#[warn(clippy::len_zero)]` on by default
```

It is also wrong for both inputs the exercise itself names:

- **"Able was I ere I saw Elba" is `false`.** `A` and `a` are different bytes, and the spaces do not mirror. A sentence palindrome is a statement about letters: keep the `is_alphanumeric()` characters, lowercase them, then compare. That is `is_palindrome_sentence` below, and it says `true`.
- **`"été"` is `false`, and so is every string with a non-ASCII character in it.** `é` is the two bytes `c3 a9`; read backwards they are `a9 c3`, which no `char` has. In UTF-8 a multi-byte character starts with a lead byte, `c2` or above, and continues with bytes in `80`–`bf`, so a valid string never ends in a lead byte, and the bytes of a string that contains one are never their own mirror. `s.chars().eq(s.chars().rev())` compares characters and says `true` for `"été"` and `"éé"` — [Walking a `String`](../../../14_Strings/walking_a_string/README.md#two-rulers-over-the-same-text) is the two rulers, [Meet the byte](../../../19_Numbers/meet_the_byte/README.md) the byte.

Three smaller things. `fn palindrome(input: String)` takes ownership of a `String` it only reads, so `main` cannot use `input` afterwards; `&str` is the parameter a read-only function wants ([one parameter serves every caller](../../../14_Strings/string_vs_str/README.md#one-parameter-serves-every-caller)). The exercise asks for `is_palindrome` and the template supplies `palindrome`. And "lowercase both sides" is the folklore fix with a known edge: `to_lowercase` is case mapping, not case folding — [Case folding is not case mapping](../../../14_Strings/comparing_strings/README.md#case-folding-is-not-case-mapping) — which does not bite on `Elba` and does on `ß`. The book's two-index loop is a [`while`](../../../25_Control_Flow/while_loops/README.md) whose progress is `first += 1` and `last -= 1`; the three functions below use iterator equality instead, and the recorded program prints all three verdicts for eight inputs.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg3_04_palindrome -->
*[`pg3_04_palindrome.rs`](examples/pg3_04_palindrome.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 3.4: a palindrome reads the same in both directions. Three readings
// of "the same": byte by byte (the book's), char by char, and letters only.
fn is_palindrome_bytes(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.iter().eq(bytes.iter().rev())
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn is_palindrome_sentence(s: &str) -> bool {
    let letters: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect();
    letters.iter().eq(letters.iter().rev())
}

fn main() {
    println!("{:<28} {:>5} {:>5} {:>8}", "input", "bytes", "chars", "letters");
    for input in ["1211", "1881", "12321", "Able was I ere I saw Elba", "été", "éé", "", "a"] {
        println!(
            "{:<28} {:>5} {:>5} {:>8}",
            format!("{input:?}"),
            is_palindrome_bytes(input),
            is_palindrome(input),
            is_palindrome_sentence(input)
        );
    }
    // "1211"                    false false    false
    // "1881"                     true  true     true
    // "12321"                    true  true     true
    // "Able was I ere I saw Elba" false false    true
    // "été"                     false  true     true
    // "éé"                      false  true     true
    // ""                         true  true     true
    // "a"                        true  true     true

    let bytes = "été".as_bytes();
    let reversed: Vec<u8> = bytes.iter().rev().copied().collect();
    println!("\"été\" is the bytes {bytes:02x?}"); // "été" is the bytes [c3, a9, 74, c3, a9]
    println!("read backwards      {reversed:02x?}"); // read backwards      [a9, c3, 74, a9, c3]
    println!("chars: {:?} reversed {:?}", "été".chars().collect::<String>(), "été".chars().rev().collect::<String>()); // chars: "été" reversed "été"
}
```
<!-- /source -->

<!-- output:pg3_04_palindrome -->
*Verified output of [`pg3_04_palindrome.rs`](examples/pg3_04_palindrome.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
input                        bytes chars  letters
"1211"                       false false    false
"1881"                        true  true     true
"12321"                       true  true     true
"Able was I ere I saw Elba"  false false     true
"été"                        false  true     true
"éé"                         false  true     true
""                            true  true     true
"a"                           true  true     true
"été" is the bytes [c3, a9, 74, c3, a9]
read backwards      [a9, c3, 74, a9, c3]
chars: "été" reversed "été"
```
<!-- /output -->

</details>

### 5. The Pythagorean triple that sums to 1000

Find the one triple of natural numbers with a < b < c, a² + b² = c² and a + b + c = 1000; print it and check both conditions. The book gives no template.

The book's solution runs three nested `for` loops — `a` from 1, `b` from `a + 1`, `c` from `b + 1` — tests both conditions on every `(a, b, c)`, and leaves the loops through a `flag` that the innermost loop sets and each outer loop checks with `if flag == false { break; }`. It finds `(200, 375, 425)`, and clippy asks for the comparison to be a negation:

```text title="Abridged — real clippy-driver output for triplet_book.rs, the book's solution, the second identical warning dropped"
warning: equality checks against false can be replaced by a negation
  --> triplet_book.rs:12:16
   |
12 |             if flag == false {
   |                ^^^^^^^^^^^^^ help: try: `!flag`
   |
   = note: `#[warn(clippy::bool_comparison)]` on by default
```

The flag is the pattern a [loop label replaces](../../../25_Control_Flow/loop_labels/README.md#what-a-label-replaces-the-flag-variable): `break 'search` leaves every loop out to the label from where the answer is found, and no variable is needed. The bigger saving is the third loop. Once `a` and `b` are chosen, `a + b + c = 1000` decides `c`, so there is nothing to search for: `let c = 1000 - a - b;`, stop the inner loop when `c <= b`, and test only `a² + b² = c²`. The recorded program counts what each search looks at: 80,778,750 candidates for the book's loops, 69,676 with `c` computed — about a thousand times fewer. On the machine that wrote this page that was about 40 ms against 50 µs in an optimised build, and about 0.7 s against under a millisecond unoptimised. The counts are in the answer key and the times are not, because [a timing is one sample of a noisy process](../../../33_Time_and_Benchmarking/timing_a_block/README.md#what-you-can-safely-say-about-the-number).

Both searches print the same triple, and the program verifies it: 200² + 375² = 180625 = 425², the sum is 1000, and the product — the number the Project Euler problem asks for — is 31875000.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg3_05_triplet -->
*[`pg3_05_triplet.rs`](examples/pg3_05_triplet.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 3.5: the Pythagorean triple with a + b + c = 1000. The book's search
// tries every c; the second search computes c and leaves both loops with a label.
// Each search also counts how many candidates it looked at.
fn search_every_c(total: i32) -> (Option<(i32, i32, i32)>, u64) {
    let mut candidates = 0;
    let mut found = None;
    let mut keep_going = true;
    for a in 1..=total {
        for b in a + 1..total {
            for c in b + 1..total {
                candidates += 1;
                if a * a + b * b == c * c && a + b + c == total {
                    found = Some((a, b, c));
                    keep_going = false;
                    break;
                }
            }
            if !keep_going {
                break;
            }
        }
        if !keep_going {
            break;
        }
    }
    (found, candidates)
}

fn search_with_c_computed(total: i32) -> (Option<(i32, i32, i32)>, u64) {
    let mut candidates = 0;
    let mut found = None;
    'search: for a in 1..total {
        for b in a + 1..total {
            let c = total - a - b;
            if c <= b {
                break; // b has grown past c: no larger b can work for this a
            }
            candidates += 1;
            if a * a + b * b == c * c {
                found = Some((a, b, c));
                break 'search;
            }
        }
    }
    (found, candidates)
}

fn main() {
    let (every_c, looked_at) = search_every_c(1000);
    println!("trying every c: {every_c:?} after {looked_at} candidates"); // trying every c: Some((200, 375, 425)) after 80778750 candidates
    let (computed, looked_at) = search_with_c_computed(1000);
    println!("computing c:    {computed:?} after {looked_at} candidates"); // computing c:    Some((200, 375, 425)) after 69676 candidates

    let (a, b, c) = computed.expect("the triple exists");
    println!("a < b < c: {}", a < b && b < c); // a < b < c: true
    println!("a² + b² = {} and c² = {}", a * a + b * b, c * c); // a² + b² = 180625 and c² = 180625
    println!("a + b + c = {}", a + b + c); // a + b + c = 1000
    println!("a · b · c = {}", a * b * c); // a · b · c = 31875000
}
```
<!-- /source -->

<!-- output:pg3_05_triplet -->
*Verified output of [`pg3_05_triplet.rs`](examples/pg3_05_triplet.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
trying every c: Some((200, 375, 425)) after 80778750 candidates
computing c:    Some((200, 375, 425)) after 69676 candidates
a < b < c: true
a² + b² = 180625 and c² = 180625
a + b + c = 1000
a · b · c = 31875000
```
<!-- /output -->

</details>

### 6. Who may see the movie

`can_see_movie(age, permission)` is `true` at 17 or older, or at 13 or older with a parent's permission. The template:

```rust
fn can_see_movie(age: i32, permission: bool) -> bool {
    // Write your code here
    return false; // Remove 'return false' once you have written the code
}
fn main() {
    println!("John who is 18, can see the move: {}", can_see_movie(17, true));
}
```

It compiles, with two `unused variable` warnings for the parameters the placeholder ignores, and prints `false`. The book's body, `(age >= 17) || (age >= 13 && permission)`, is right, and it is the whole function: the condition already *is* the `bool` to return, so there is no `if … { true } else { false }` around it — [Meet the `bool`](../../../15_First_Programs/meet_the_bool/README.md). Two slips in the printed solution. The listing has lost its spaces (`fncan_see_movie`, `fnmain`, `"Johnwhois18,canseethemove:{}"`), so as printed it does not compile:

```text title="Abridged — real rustc output for fncan_see_movie.rs, the solution as the book prints it"
error: missing `fn` for function definition
 --> fncan_see_movie.rs:1:1
  |
1 | fncan_see_movie(age: i32, permission: bool) -> bool {
  | ^^^^^^^^^^^^^^^
  |
help: add `fn` here to parse `fncan_see_movie` as a function
  |
1 | fn fncan_see_movie(age: i32, permission: bool) -> bool {
  | ++
```

And `main` says John is 18 but passes `17`. Both print `true`, which is why nobody noticed. The solution below prints the whole truth table for 12, 13, 16, 17 and 18, with and without permission, so that each edge of the rule is visible: 13 turns the permission branch on, and 17 makes it unnecessary.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg3_06_movie -->
*[`pg3_06_movie.rs`](examples/pg3_06_movie.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 3.6: 17 or older, or 13 or older with a parent's permission.
fn can_see_movie(age: i32, permission: bool) -> bool {
    age >= 17 || (age >= 13 && permission)
}

fn main() {
    println!("John who is 18, can see the movie: {}", can_see_movie(18, true)); // John who is 18, can see the movie: true
    println!("  age  permission  can_see_movie");
    for age in [12, 13, 16, 17, 18] {
        for permission in [false, true] {
            println!("  {age:>3}  {permission:<10}  {}", can_see_movie(age, permission));
        }
    }
    // 12: false false; 13: false true; 16: false true; 17: true true; 18: true true
}
```
<!-- /source -->

<!-- output:pg3_06_movie -->
*Verified output of [`pg3_06_movie.rs`](examples/pg3_06_movie.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
John who is 18, can see the movie: true
  age  permission  can_see_movie
   12  false       false
   12  true        false
   13  false       false
   13  true        true
   16  false       false
   16  true        true
   17  false       true
   17  true        true
   18  false       true
   18  true        true
```
<!-- /output -->

</details>

## If you are coming from another language

**Python.** An `int` has no width, so exercise 1's limit does not exist: `(304 * 305 // 2) ** 2` is `2149249600` and Python does not care. `int(16.575)` truncates to `16` exactly as `as i32` does, and `round(16.575)` is `17`; but Python's `round` sends a tie to the even neighbour — `round(16.5)` is `16` and `round(17.5)` is `18` — where Rust's `round` goes away from zero, and `round_ties_even` is the Python rule ([four directions](../../../19_Numbers/rounding_a_float/README.md#four-directions)). The idiomatic palindrome test, `s == s[::-1]`, compares code points and says `True` for `"été"`, like `chars().rev()`; the book's byte reading is `s.encode()[::-1] == s.encode()`, which is `False` — [`str` is not `bytes` ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/str_is_not_bytes/index.html) and [Counting characters ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/counting_characters/index.html). Lowercasing before the compare has the same `ß` gap in both languages, and Python's `casefold()` is the fix Rust's std lacks — [Lowercasing is not folding ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/lowercasing_is_not_folding/index.html). `input()` is the `read_line` of the two templates ([Standard in, standard out, and pipes ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/stdin_stdout_and_pipes/index.html)). For exercise 6 `if age:` would be accepted, since Python has truthiness and Rust's `if` takes only a `bool`, and Python has no loop labels either: exercise 5 needs the flag, a `return` from a function, or `for … else`.

**C.** A `char` is a byte, so the book's palindrome is exactly what a C loop over `strlen` indexes computes, and it fails the same way on `été` — [A `char` is a byte, not a character ↗](https://masiarek.github.io/c-learning-library/03_Strings/char_is_a_byte_not_a_character/index.html). `(int)16.575f` is `16`: the C cast truncates like `as`. Exercise 1's overflow is a panic here and undefined behaviour there — [Signed overflow](../../../31_C_and_Cpp/signed_overflow/README.md). Reading N from the terminal is `strtol`, the one of C's three parsers that reports where it stopped — [Parsing a number from text ↗](https://masiarek.github.io/c-learning-library/03_Strings/parsing_a_number_from_text/index.html) — and Rust's `parse()` returns a `Result` for the same reason. Leaving two loops at once is `goto` or a flag; Rust has the label and no `goto` ([Loop labels](../../../25_Control_Flow/loop_labels/README.md#no-goto-and-the-cleanup-it-was-for-is-drop)).

**ABAP.** There is no boolean type, so exercise 6 returns `abap_bool`, and `xsdbool( age >= 17 OR ( age >= 13 AND permission = abap_true ) )` is the body — [`xsdbool( )` and `boolc( )` ↗](https://masiarek.github.io/abap-learning-library/02_Keywords/boolean_functions/index.html). `EXIT` leaves only the innermost loop and there are no labels, so exercise 5 is written the book's way, with a flag — [`CHECK`, `CONTINUE`, `EXIT`, `RETURN` ↗](https://masiarek.github.io/abap-learning-library/02_Keywords/check_continue_exit/index.html); the counting loops themselves are `DO n TIMES` and `WHILE` ([`DO` and `WHILE` ↗](https://masiarek.github.io/abap-learning-library/02_Keywords/do_while/index.html)). Exercise 3's cast goes the other way: assigning `16.575` to an `i` field *rounds*, to 17, by the table on [Conversion and comparison rules ↗](https://masiarek.github.io/abap-learning-library/03_Topics/conversion_and_comparison_rules/index.html), so the book's "to round the values" would have been true in ABAP. An ABAP `string` is a sequence of characters, not bytes, so its built-in `reverse( )` ([Strings and text ↗](https://masiarek.github.io/abap-learning-library/03_Topics/strings_and_text/index.html)) is the `chars().rev()` reading, not the book's.

## See also

- [*Rust: The Practical Guide*, run](../README.md) — the shelf page for the book, and the [chapter 2 exercises, run](../ch2_katas/README.md) before this one
- [`for` loops](../../../25_Control_Flow/for_loops/README.md) — `1..=n` and `1..n`, exercises 1 and 2
- [The integer types](../../../19_Numbers/the_integer_types/README.md) — where exercise 1's `i32` runs out
- [`if` expressions](../../../25_Control_Flow/if_expressions/README.md) — the speed bands of exercise 3, and the condition that is already the answer in exercise 6
- [Casting with `as`](../../../29_Conversion/casting_with_as/README.md) and [Making a float whole](../../../19_Numbers/rounding_a_float/README.md) — why `16.575 as i32` is 16 and `round()` is 17
- [Walking a `String`](../../../14_Strings/walking_a_string/README.md) and [Meet the byte](../../../19_Numbers/meet_the_byte/README.md) — bytes against chars in exercise 4
- [Comparing and sorting text](../../../14_Strings/comparing_strings/README.md) — the case-folding caveat the lowercased palindrome inherits
- [`while` loops](../../../25_Control_Flow/while_loops/README.md) — the book's two-index loop
- [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md) — the parameter type `palindrome` should have had
- [Loop labels](../../../25_Control_Flow/loop_labels/README.md) — `break 'search` in place of exercise 5's flag
- [Timing a block](../../../33_Time_and_Benchmarking/timing_a_block/README.md) — why the candidate counts are in the answer key and the milliseconds are not
- [Meet the `bool`](../../../15_First_Programs/meet_the_bool/README.md) — exercise 6
- [Exercises](../../exercises/README.md) — the other practice tracks, and which to start with
- [Katas](../../../KATAS.md) — every kata in the library, in order

## Po polsku

Sześć ćwiczeń z §3.4 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) i rozwiązania z §3.5, wszystkie sprawdzone na rustc 1.98.0. Dwie sumy w pętli `for` (`1..=n` i `1..n`) dają 170, 78 oraz odpowiedzi Project Euler 25164150 i 233168; kwadrat sumy przekracza `i32` już przy N = 304 (przepełnienie, *overflow*), choć sama różnica by się jeszcze zmieściła. W linii montażowej rzutowanie `as i32` **obcina** (*truncates*), a nie zaokrągla: 16,575 daje 16, a `round()` daje 17; godziny skracają się w stawce na minutę, a prędkości powyżej 10 nikt nie odrzuca. Palindrom liczony bajt po bajcie (`as_bytes`) odpowiada `false` dla zdania z książki „Able was I ere I saw Elba” (wielkość liter, spacje) i dla „été”, bo znak wielobajtowy w UTF-8 czytany od tyłu nie jest żadnym znakiem; `s.chars().eq(s.chars().rev())` porównuje znaki, a wersja „tylko litery, małe” obsługuje zdania. Trójka pitagorejska (200, 375, 425) o sumie 1000: potrójna pętla z flagą sprawdza 80 778 750 kandydatów, a dwie pętle z `c = 1000 - a - b` i `break 'search` (etykieta pętli, *loop label*) 69 676. Funkcja `can_see_movie` zwraca po prostu warunek, bo warunek już jest wartością `bool`; wydrukowane rozwiązanie zgubiło spacje (`fncan_see_movie`) i się nie kompiluje.

**Szukaj po polsku:** `rust pętla for zakres 1..=n suma kwadratów` · `rust as i32 obcinanie zaokrąglanie round` · `rust palindrom chars rev UTF-8` · `rust etykieta pętli break label` · `rust trójka pitagorejska suma 1000` · `rust bool bez if zwracanie warunku`

---

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 3 exercises** · back to [Chapter 2 exercises, run](../ch2_katas/README.md)
