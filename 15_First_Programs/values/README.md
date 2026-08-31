# Values

**Level:** 101 → 201 · for newcomers

**One line:** The dozen-odd types you can write down before defining one of your own — what the literal looks like, how wide it is, and the fact that a width is a promise the compiler checks.

```rust
let count: u32 = 1_000;
let ratio: f64 = 3.14;
let grade: char = 'A';
let voted: bool = true;
println!("{count} {ratio} {grade} {voted}");  // 1000 3.14 A true
```

## The census

| Kind | Types | Literals |
|---|---|---|
| Signed integers | `i8` `i16` `i32` `i64` `i128` `isize` | `-10`, `0`, `1_000`, `123_i64` |
| Unsigned integers | `u8` `u16` `u32` `u64` `u128` `usize` | `0`, `123`, `10_u16` |
| Floating point | `f32` `f64` | `3.14`, `-10.0e20`, `2_f32` |
| Unicode scalar value | `char` | `'a'`, `'α'`, `'∞'` |
| Boolean | `bool` | `true`, `false` |

And the widths:

- `iN`, `uN` and `fN` are **N bits** wide — the number is in the name.
- `isize` and `usize` are the width of a **pointer**. Not a fixed 64: it is 8 bytes on this machine and 4 on a 32-bit target, and it is the type of every length, index and byte count in the standard library, which is why `.len()` hands you a `usize`.
- `char` is **32 bits**. It holds one Unicode scalar value, and the largest of those is U+10FFFF — which does not fit in 8 bits, or 16.
- `bool` is **8 bits**. One bit of information in one byte of space, because a byte is the smallest thing a machine can address. Packing eight of them into one byte is [bit flags](../../19_Numbers/bit_flags/README.md), and it is deliberate work.

Every one of those numbers is measured with `size_of` in the [verified output](#the-verified-output) below rather than quoted, so this table cannot drift away from the compiler.

## Writing one down

Underscores are for your eyes only:

```rust
assert_eq!(1000, 1_000);
assert_eq!(1_000, 10_00);   // legal, and nobody should
```

A suffix is the type, written on the literal rather than on the `let`:

```rust
let a = 123_i64;   // identical to
let b: i64 = 123;
assert_eq!(a, b);
```

The underscore before the suffix is optional too — `123i64` is the same literal. Prefer the suffix when the value is being handed straight to something (`vec![0u8; 4]`, `x as f32 * 2.0_f32`) and the annotation when it is being named; both are the same instruction to the compiler.

Integers can be written in four bases, and there is a byte literal for a single ASCII character:

```rust
assert_eq!(65, 0x41);        // hex
assert_eq!(65, 0o101);       // octal
assert_eq!(65, 0b100_0001);  // binary
assert_eq!(65, b'A' as i32); // byte literal — a u8, not a char
```

`b'A'` is one byte; `'A'` is four. They print the same and are not the same type. [Why hexadecimal](../../19_Numbers/why_hexadecimal/README.md) is where the `0x` form earns its keep, and [meet the `char`](../../14_Strings/meet_the_char/README.md) is the other half of that pair.

## The two fallbacks

```rust
let n = 1;     // i32
let f = 1.0;   // f64
```

`i32` is not what `1` *means*. It is what Rust settles on when nothing else in the function decides — and `1` does not become a `u8` because it happens to be small. Before it settles, the type has a placeholder name you will meet in error messages:

```rust
// let x = 3.14;
// let y = 20;
// assert_eq!(x, y);
```

```text title="Abridged — real rustc output for float_vs_integer.rs"
error[E0277]: can't compare `{float}` with `{integer}`
 --> float_vs_integer.rs:4:5
  |
4 |     assert_eq!(x, y);
  |     ^^^^^^^^^^^^^^^^ no implementation for `{float} == {integer}`
  |
  = help: the trait `PartialEq<{integer}>` is not implemented for `{float}`
```

`{float}` and `{integer}` are not types you can write; they are the compiler saying *a number whose width is still undecided*. Worth knowing that this three-line program produces **two** errors, not one — the `E0277` above and an `E0308` for the same line — which is the normal shape of a numeric mismatch and not a sign you broke two things. How the settling works is [type inference](../type_inference/README.md).

## A width is a promise, and it is checked

The literal that cannot fit is rejected outright:

```rust
// let big: u8 = 1_000_000;
//    error: literal out of range for `u8`  (range is 0..=255)
```

The arithmetic that overflows is a different matter, and it is the one worth knowing early: **a debug build panics and a release build wraps**. So the same expression is a crash while you are developing and a plausible wrong number in production. Rust's answer is to make you choose:

```rust
let almost = u8::MAX;              // 255
almost.wrapping_add(1);            // 0
almost.checked_add(1);             // None
almost.saturating_add(1);          // 255
```

[Meet the byte](../../19_Numbers/meet_the_byte/README.md) is where that bill is itemised, and the kata below is where you meet it by accident.

## If you are coming from another language

**Python.** The one that matters is the first row, and it is not a small difference: Python's `int` is arbitrary precision, so `2 ** 200` is exact and `x + 1` cannot overflow. Every width on this page is a constraint Python does not have, which means a whole category of Rust bug — the one the kata below is about — has no Python counterpart at all. In exchange, a Python `int` is a heap object with a header, so a list of a million of them is not a million machine words; a `Vec<i64>` is.

| | Python | Rust |
|---|---|---|
| integer | `int` — arbitrary precision, heap | `i8`…`i128`, fixed width, on the stack |
| float | `float` — always C double | `f64`, and `f32` if you ask |
| boolean | `bool`, a **subclass of `int`** (`True + 1 == 2`) | `bool`, no arithmetic at all |
| single character | no such type — a 1-length `str` | `char`, four bytes, one scalar value |
| pointer-width integer | not exposed | `usize` / `isize` |
| digit grouping | `1_000` | `1_000` — the same |

The `bool` row bites in practice: `sum(flags)` is idiomatic Python for counting `True`s and there is no such thing in Rust, because `bool` is not a number. You write `flags.iter().filter(|b| **b).count()`.

**ABAP.** The type names are unfamiliar but the ideas line up, with one genuine gap in each direction:

| | ABAP | Rust |
|---|---|---|
| integers | `b` (1 byte), `s` (2), `i` (4), `int8` (8) — no unsigned types at all | `i8`…`i128` **and** `u8`…`u128` |
| exact decimal | `p` — packed decimal, `DECIMALS 2`, exact | none built in; `i64` of cents, or a crate |
| binary float | `f` | `f64` |
| character | `c LENGTH n`, fixed-width, blank-padded | `char` is one scalar value; text is `String` |
| overflow | short dump `COMPUTE_INT_TIMES_OVERFLOW` | panic in debug, wrap in release |

The gap worth naming is `p`. ABAP hands you exact decimal arithmetic as a built-in type, so money is a solved problem in the language; Rust does not, and the standard answer is to hold an integer number of cents — which is exactly what [scale the denominator away](../../09_Advanced/scaled_integers/README.md) does for election weights. Going the other way, ABAP has no unsigned integer, so the `u32` habit of *this can never be negative, and the type says so* has nothing to transfer to.

---

## Practice

**Fibonacci, and the width that runs out.** The sequence begins `[0, 1]`, and for `n > 1` each number is the sum of the previous two. Write `fn fib(n: u32) -> u32` recursively, with a base case for `n < 2`.

Then answer the question the exercise is really asking: **when does this function panic?** Find the exact `n` — not by reasoning about it, by running it — and then explain why the same `n` does *not* panic in `cargo run --release`, and what it prints instead. That second half is the one worth getting right.

Three more, each a line or two:

1. **Rewrite `fib` without `return`.** The last expression of a block is its value, so `if n < 2 { n } else { ... }` is the whole body. Same function, and [a block is an expression](../a_block_is_an_expression/README.md) is why.
2. **Widen it.** For each of `u8`, `u16`, `u32`, `u64`, `u128`, find the largest `n` whose `fib(n)` still fits. Use `checked_add` in a loop rather than waiting for a panic. Does a wider type fix the bug or move it?
3. **Count the calls, don't time them.** Add a `calls: &mut u64` counter to the recursive version and print it for `n = 10, 20, 30` beside the number of steps an iterative version takes. A timing would vary per machine; the call count is the same everywhere, and it is the thing that actually explains the difference.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:values_kata -->
*[`values_kata.rs`](examples/values_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: Fibonacci, and the width that runs out.
//!
//!     rustc --edition 2024 values_kata.rs -o /tmp/vk && /tmp/vk

/// The classic recursive spelling, with an explicit `return`.
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

/// The same function as an expression — no `return`, no semicolon on the last line.
fn fib_expr(n: u32) -> u32 {
    if n < 2 { n } else { fib_expr(n - 1) + fib_expr(n - 2) }
}

/// Iterative, and it never recurses.
fn fib_iter(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let (mut prev, mut cur) = (0u32, 1u32);
    for _ in 2..=n {
        let next = prev + cur;
        prev = cur;
        cur = next;
    }
    cur
}

/// Counts its own calls, so "expensive" is a number rather than an adjective.
fn fib_counted(n: u32, calls: &mut u64) -> u64 {
    *calls += 1;
    if n < 2 {
        return n as u64;
    }
    fib_counted(n - 1, calls) + fib_counted(n - 2, calls)
}

/// The largest n whose fib(n) still fits in `max`, and that value.
fn last_fitting(max: u128) -> (u32, u128) {
    let (mut prev, mut cur) = (0u128, 1u128); // fib(0), fib(1)
    let mut n = 0u32;
    while cur <= max {
        let Some(next) = prev.checked_add(cur) else {
            return (n + 1, cur);
        };
        prev = cur;
        cur = next;
        n += 1;
    }
    (n, prev)
}

fn main() {
    println!("1. fib, three ways, same answers");
    print!("   n        ");
    for n in 0..11 {
        print!("{n:>4}");
    }
    println!();
    print!("   fib      ");
    for n in 0..11 {
        print!("{:>4}", fib(n));
    }
    println!();
    print!("   fib_expr ");
    for n in 0..11 {
        print!("{:>4}", fib_expr(n));
    }
    println!();
    print!("   fib_iter ");
    for n in 0..11 {
        print!("{:>4}", fib_iter(n));
    }
    println!();
    let agree = (0..30).all(|n| fib(n) == fib_expr(n) && fib(n) == fib_iter(n));
    println!("   all three agree for n = 0..30? {agree}");
    println!();

    println!("2. When does it panic?");
    println!("   fib(47) = {}", fib_iter(47));
    println!("   u32::MAX = {}", u32::MAX);
    println!("   fib(48) needs {} , which is {} more than u32 can hold.",
        4_807_526_976u64,
        4_807_526_976u64 - u32::MAX as u64);
    let a: u32 = 2_971_215_073; // fib(47)
    let b: u32 = 1_836_311_903; // fib(46)
    println!("   The addition that dies is fib(47) + fib(46) = {a} + {b}:");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let boom = std::panic::catch_unwind(|| a + b);
    std::panic::set_hook(hook);
    match boom {
        Ok(v) => println!("      a + b            = {v}   (release build: overflow checks off)"),
        Err(_) => println!("      a + b            panicked: attempt to add with overflow"),
    }
    println!("      a.checked_add(b) = {:?}", a.checked_add(b));
    println!("      a.wrapping_add(b)= {}   <- the wrong answer a release build prints",
        a.wrapping_add(b));
    println!("   So: n = 48 in a debug build, and n = 48 gives a plausible, wrong");
    println!("   number in a release build. The second one is the dangerous half.");
    println!();

    println!("3. The same function, one word wider");
    for (name, max) in [
        ("u8", u8::MAX as u128),
        ("u16", u16::MAX as u128),
        ("u32", u32::MAX as u128),
        ("u64", u64::MAX as u128),
        ("u128", u128::MAX),
    ] {
        let (n, value) = last_fitting(max);
        println!("   {name:<5} holds up to fib({n:>3}) = {value}");
    }
    println!("   Widening buys arithmetic, not safety: u128 dies at 187 instead of");
    println!("   48. Picking a type is picking where the program stops being right.");
    println!();

    println!("4. Recursion is not the slow part — recomputation is");
    for n in [10u32, 20, 30] {
        let mut calls = 0u64;
        let value = fib_counted(n, &mut calls);
        println!("   fib({n:>2}) = {value:<8}  recursive calls: {calls:>9}   iterative steps: {n:>2}");
    }
    println!("   Every call recomputes what the sibling call already worked out.");
    println!("   The loop keeps two numbers and never asks the same question twice.");
}
```
<!-- /source -->

<!-- output:values_kata -->
*Verified output of [`values_kata.rs`](examples/values_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. fib, three ways, same answers
   n           0   1   2   3   4   5   6   7   8   9  10
   fib         0   1   1   2   3   5   8  13  21  34  55
   fib_expr    0   1   1   2   3   5   8  13  21  34  55
   fib_iter    0   1   1   2   3   5   8  13  21  34  55
   all three agree for n = 0..30? true

2. When does it panic?
   fib(47) = 2971215073
   u32::MAX = 4294967295
   fib(48) needs 4807526976 , which is 512559681 more than u32 can hold.
   The addition that dies is fib(47) + fib(46) = 2971215073 + 1836311903:
      a + b            panicked: attempt to add with overflow
      a.checked_add(b) = None
      a.wrapping_add(b)= 512559680   <- the wrong answer a release build prints
   So: n = 48 in a debug build, and n = 48 gives a plausible, wrong
   number in a release build. The second one is the dangerous half.

3. The same function, one word wider
   u8    holds up to fib( 13) = 233
   u16   holds up to fib( 24) = 46368
   u32   holds up to fib( 47) = 2971215073
   u64   holds up to fib( 93) = 12200160415121876738
   u128  holds up to fib(186) = 332825110087067562321196029789634457848
   Widening buys arithmetic, not safety: u128 dies at 187 instead of
   48. Picking a type is picking where the program stops being right.

4. Recursion is not the slow part — recomputation is
   fib(10) = 55        recursive calls:       177   iterative steps: 10
   fib(20) = 6765      recursive calls:     21891   iterative steps: 20
   fib(30) = 832040    recursive calls:   2692537   iterative steps: 30
   Every call recomputes what the sibling call already worked out.
   The loop keeps two numbers and never asks the same question twice.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:values -->
*Verified output of [`values.rs`](examples/values.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Signed integers — i8 i16 i32 i64 i128 isize
   type     bytes                                       min                                       max
   i8           1                                      -128                                       127
   i16          2                                    -32768                                     32767
   i32          4                               -2147483648                                2147483647
   i64          8                      -9223372036854775808                       9223372036854775807
   i128        16  -170141183460469231731687303715884105728   170141183460469231731687303715884105727
   isize        8                      -9223372036854775808                       9223372036854775807

2. Unsigned integers — u8 u16 u32 u64 u128 usize
   type     bytes                                       min                                       max
   u8           1                                         0                                       255
   u16          2                                         0                                     65535
   u32          4                                         0                                4294967295
   u64          8                                         0                      18446744073709551615
   u128        16                                         0   340282366920938463463374607431768211455
   usize        8                                         0                      18446744073709551615

3. isize and usize are the width of a pointer
   size_of::<usize>()      = 8
   size_of::<*const u8>()  = 8
   equal? true   <- that is the definition, not a coincidence of this machine
   It is the type of a length, an index, and a byte count — which is
   why `.len()` gives you a usize and not an i32.

4. Floats, char and bool
   f32     4 bytes      3.14, -10.0e20, 2_f32
   f64     8 bytes        3.14 (the fallback)
   char    4 bytes              'a', 'α', '∞'
   bool    1 bytes                true, false
   char is 32 bits wide because it holds one Unicode scalar value,
   and the largest of those is U+10FFFF = 1114111.
   bool is 8 bits wide because a byte is the smallest addressable
   unit — it carries one bit of information in one byte of space.

5. Writing one down
   1000 == 1_000 == 10_00 ?  true
   Underscores are legibility only. The compiler removes them.
   123_i64 == 123i64 ?       true
   The suffix is the type, written on the literal instead of the let.
      let a = 123_i64;   is   let a: i64 = 123;

6. Other bases, and the byte literal
   decimal      65        65
   hex          0x41      65
   octal        0o101     65
   binary       0b100_0001 65
   byte         b'A'      65
   all the same u8: true
   b'A' is a u8, not a char: 1 vs 4

7. The two fallbacks
   let n = 1;     i32   <- i32 when nothing else decides
   let f = 1.0;   f64   <- f64 when nothing else decides
   In an error message these appear as {integer} and {float}:
      let x = 3.14; let y = 20; assert_eq!(x, y);
      error[E0277]: can't compare `{float}` with `{integer}`

8. The width is a promise, and it is checked
      let big: u8 = 1_000_000;
      error: literal out of range for `u8`  (range is 0..=255)
   u8::MAX = 255, and one more is:
      almost + 1              panics in a debug build, wraps in release
      wrapping_add(1)  = 0
      checked_add(1)   = None
      saturating_add(1)= 255
   Four answers, and the type does not pick for you — you do.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 15_First_Programs/values/examples/values.rs -o /tmp/values && /tmp/values
```

## See also

- [Variables](../variables/README.md) — the `let` these go on the right of
- [Type inference](../type_inference/README.md) — where `i32` and `f64` come from when you do not say
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — `u8` in full, and the three bills a width comes with
- [What a float actually stores](../../19_Numbers/what_a_float_stores/README.md) — why `f64` gets no `Eq` and no `Ord`
- [Meet the `char`](../../14_Strings/meet_the_char/README.md) — four bytes here, one to four bytes inside a `String`
- [Bit flags](../../19_Numbers/bit_flags/README.md) — putting eight `bool`s where one would have gone
- [Primitive types ↗](https://doc.rust-lang.org/book/ch03-02-data-types.html) · [Literal expressions ↗](https://doc.rust-lang.org/reference/expressions/literal-expr.html) · [Comprehensive Rust: Values ↗](https://google.github.io/comprehensive-rust/types-and-values/values.html)

## Po polsku

Tour of Rust nazywa tę garść typów **podstawowymi typami** (*primitive types*) i to dobra nazwa: nic tu nie trzeba definiować, wystarczy napisać wartość. Szerokość siedzi w nazwie — `i8` to osiem bitów, `u32` to trzydzieści dwa — a osobno warto zapamiętać trzy, których nazwa nie mówi wprost. `char` ma **32 bity**, bo mieści jedną wartość skalarną Unicode (największa to U+10FFFF). `bool` ma **8 bitów**, bo bajt jest najmniejszą adresowalną porcją pamięci — jeden bit informacji zajmuje cały bajt miejsca. A `usize` i `isize` są szerokie **jak wskaźnik**, czyli 8 bajtów na tej maszynie i 4 na celu 32-bitowym; to jest typ każdej długości, każdego indeksu i każdej liczby bajtów w bibliotece standardowej. Stąd bierze się `E0308`, na który natyka się niemal każdy przychodzący od C-owego `int i` — `.len()` oddaje `usize`, a nie `i32`, i to nie jest kaprys, tylko wymóg, żeby indeks nie mógł być ujemny.

Zapis liczb ma dla piszącego po polsku dwie pułapki, jedną miłą i jedną kosztowną. Miła: podkreślnik w `4_807_526_976` grupuje cyfry dokładnie tak, jak polska typografia grupuje je spacją, i jest wyłącznie dla oka — kompilator je usuwa, więc `1_000 == 10_00` jest prawdą (legalną i szkodliwą). Kosztowna: separatorem dziesiętnym w literale jest **kropka i tylko kropka**, więc `3,14` nie jest liczbą, tylko dwiema rzeczami rozdzielonymi przecinkiem. To samo dotyczy wypisywania — `println!("{ratio}")` da `3.14` niezależnie od ustawień regionalnych systemu, bo biblioteka standardowa w ogóle nie zna pojęcia lokalizacji; przecinek dziesiętny na ekranie trzeba zrobić samemu albo sięgnąć po `crate` od formatowania. Trzecia rzecz z tej samej rodziny: literał bajtowy `b'A'` obsługuje wyłącznie ASCII, więc `b'ą'` się nie skompiluje.

Dla polskiego tekstu najważniejsza jest różnica między `char` a bajtem, i ta strona stawia ją w najlepszym możliwym miejscu — obok siebie. `'ą'`, `'ż'` i `'ł'` to najzupełniej poprawne `char`y, każdy po cztery bajty w pamięci. Ale w łańcuchu znaków (`String`) obowiązuje UTF-8 i każda z tych liter zajmuje **dwa** bajty, więc `"żółw".len()` daje **7**, a nie 4 — `len()` liczy bajty, a znaki liczy `.chars().count()`. Zapamiętanie tego tutaj oszczędza całą serię zaskoczeń przy krojeniu tekstu na wycinki (*slice*).

Ostatnia część strony jest najbardziej praktyczna: **szerokość to obietnica i jest sprawdzana**. Literał, który się nie mieści, zostaje odrzucony od razu (`literal out of range for u8`), ale arytmetyka to inna sprawa i tu jest sedno: **w trybie debug program panikuje, a w trybie release zawija**. Kata na dole pokazuje to konkretną liczbą — `fib(47) + fib(46)`, czyli 2 971 215 073 + 1 836 311 903, daje 4 807 526 976, a to więcej, niż mieści `u32`; kompilacja debugowa pada z „attempt to add with overflow”, a wydaniowa wypisuje **512 559 680**, czyli wynik wiarygodny i fałszywy. Groźniejsza jest ta druga połowa, dlatego Rust każe wybrać jawnie: `wrapping_add`, `checked_add` albo `saturating_add`. I nie łudź się szerszym typem — `u128` wystarcza do `fib(186)` i psuje się na 187, zamiast, jak `u32`, na 48; poszerzenie kupuje arytmetykę, a nie bezpieczeństwo. Osobna uwaga dla przychodzących z ABAP-a: nie ma tu odpowiednika typu `p`, więc kwoty trzyma się w groszach jako `i64` albo sięga po `crate` z dziesiętną arytmetyką.

**Szukaj po polsku:** podstawowe typy w Ruscie · przepełnienie liczb całkowitych · liczba bajtów a liczba znaków · `rust usize vs i32` · `rust integer overflow debug release` · `rust char vs u8`
