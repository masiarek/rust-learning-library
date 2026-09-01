# Writing a number down

**Level:** 101 → 201 · for newcomers

**One line:** Four things can be attached to a number literal in Rust — a base prefix, underscores, a type suffix, and a leading `b` — and each of them is you telling the compiler something it would otherwise have to guess.

```rust
let score  = 5u8;              // suffix: this is a byte-wide number
let mask   = 0b0001_1110;      // prefix: read these as bits
let mode   = 0o644;            // prefix: read these as three-bit groups
let letter = b'A';             // b: one byte, 65, not a character
let ratio  = 2.0f32;           // dot and suffix: a single-precision float
println!("{score} {mask} {mode} {letter} {ratio}");  // 5 30 420 65 2
```

## The prefix: `0` says "not decimal", the letter names the base

| Prefix | Base | Mnemonic | Digits allowed | Example |
|---|---|---|---|---|
| `0x` | 16 | he**x**adecimal | `0`–`9`, `a`–`f` (either case) | `0x2A` = 42 |
| `0o` | 8 | **o**ctal | `0`–`7` | `0o77` = 63 |
| `0b` | 2 | **b**inary | `0`, `1` | `0b101` = 5 |
| *(none)* | 10 | — | `0`–`9` | `42` |

That is the whole rule, and it is the answer to *how do I remember which is which*: the `0` is a flag meaning **this is not a decimal number**, and the letter after it is the first letter of the base's English name. There is nothing else to memorise — no fourth prefix, no per-type variation, and the same three prefixes work on every integer type in the language.

Case is irrelevant in the digits (`0xff == 0xFF`) and the prefix letter is always lowercase.

## A leading zero on its own means nothing

```rust
assert_eq!(0755, 755);      // decimal seven hundred fifty-five
assert_eq!(0o755, 493);     // octal: rwx r-x r-x
```

In C, `0755` **is** octal — the bare leading zero is the prefix — and that has produced a long tail of bugs in which a zero-padded decimal quietly became a different number. Rust never adopted it, so a leading zero in Rust is inert padding and `0o` is the only way to ask for base 8. Python 3 removed the C form too, and its error message names the replacement: *"leading zeros in decimal integer literals are not permitted; use an 0o prefix for octal integers"*.

Which base to reach for is not taste — it is whether the digits line up with the field. One hex digit is four bits, so a byte is always two digits; one octal digit is three bits, so a nine-bit permission mask is always three. [Why hexadecimal](../why_hexadecimal/README.md) is that argument in full.

## The underscore is for your eyes

It may go anywhere inside the literal, any number of times, and it changes nothing:

```rust
assert_eq!(1_000_000, 1000000);
assert_eq!(0xDEAD_BEEFu32, 3735928559);
assert_eq!(1_0_0, 100);          // legal, and nobody should
```

Group at the boundary that matters: thousands in a decimal quantity, bytes in hex (`0xDEAD_BEEF`), nibbles in binary (`0b1011_1110`).

## The suffix is the type, written on the number

```rust
let a = 57u8;         // identical to
let b = 57_u8;        // identical to
let c: u8 = 57;
assert!(a == b && b == c);
```

All three are one instruction to the compiler, said in three places. The underscore before the suffix is optional and purely cosmetic. Reach for the suffix when the value is handed straight to something (`vec![0u8; 4]`, `x as f32 * 2.0_f32`) and the annotation when the value is being named.

With no suffix and nothing else to go on, Rust falls back to **`i32`** for an integer and **`f64`** for a float. `1` is not "an integer of whatever size fits" — it is an `i32` unless something in the function says otherwise, and it does not become a `u8` for being small. How the deciding works is [type inference](../../15_First_Programs/type_inference/README.md).

## `b` in front of a quote means bytes, not text

```rust
let byte:  u8       = b'A';    // 65 — one byte, a number
let ch:    char     = 'A';     // 65 as a scalar value — four bytes
let bytes: &[u8; 2] = b"Hi";   // [72, 105]
let text:  &str     = "Hi";    // UTF-8
assert_eq!(b'A' + 2, b'C');    // b'A' is a number; you may do arithmetic on it
```

`b'A'` is not "a character stored efficiently" — it is the number 65 with a spelling that shows which byte you meant. The digits between the quotes must be ASCII (0–255); `b'é'` does not compile, because é is two bytes in UTF-8 and a byte literal is exactly one.

**And this is where the flashcard claim *"`u8` is the only integer type suitable for byte representation"* is worth unpicking**, because it is true in a narrow way and misleading in a broad one. Nothing stops you storing 65 in an `i32`. What is true is that `u8` is the only type whose range is *exactly* a byte's — `0..=255`, no value spare and none missing — so it is the type the language and the standard library use whenever the thing genuinely is a byte: `b'A'` produces one, `b"Hi"` is an array of them, `String::as_bytes()` returns `&[u8]`, `File::read` fills a `&mut [u8]`. Picking `i32` there would be picking a type that can hold `-1` and `300`, neither of which any byte is. [Meet the byte](../meet_the_byte/README.md) is the page on what follows from that.

## Floats: the dot is required, and there is no unsigned one

```rust
let a = 2.0;       // f64
let b = 2.;        // f64 — legal; plain 2 would be an integer
let c = 1e6;       // f64
let d = 2.0f32;    // f32
let e = 2_f32;     // f32 — no dot needed once the suffix says float
println!("{a} {b} {c} {d} {e}");   // 2 2 1000000 2 2
```

`e` is the fifth decoration, and it is a float-only one: **scientific notation**, `mEn` meaning *m × 10ⁿ*. The `e` may be either case, the exponent may be negative, and the exponent itself takes underscores like any other digit run:

```rust
assert_eq!(12.3e4, 123000.0);
assert_eq!(1E-8, 0.00000001);
assert_eq!(1e6, 1_000_000.0);      // no dot needed — the exponent makes it a float
assert_eq!(1.5e-3_f32, 0.0015);    // the suffix still goes last
```

`1e6` is an `f64` despite having no decimal point, which is the one thing to remember: the exponent is enough to make the literal a float, so `let n = 1e6;` is not the integer a million. An exponent that overruns the type is caught at compile time rather than becoming an infinity — `1e400_f64` is `error: literal out of range for f64`, from the deny-by-default `overflowing_literals` lint.

The one that trips people is `2.f32`, which looks like the integer suffix form and is not:

```text title="Abridged — real rustc output for bad.rs"
error[E0610]: `{integer}` is a primitive type and therefore doesn't have fields
 --> bad.rs:3:15
  |
3 |     let f = 2.f32;
  |               ^^^
  |
help: if intended to be a floating point literal, consider adding a `0` after the period
  |
3 |     let f = 2.0f32;
  |               +
```

Rust parsed `2.f32` as *field `f32` of the number 2*. Write `2.0f32` or `2_f32`.

**There is no `uf32` or `uf64`, and there cannot be.** IEEE 754 puts a sign bit at the top of every float, so signedness is not a choice the type makes — it is part of the format. The bit is there even at zero, which is why floats have two zeros that compare equal:

```rust
assert!(-0.0 == 0.0);
assert!((-0.0f64).is_sign_negative());   // the bit is set anyway
```

If you need "a float that cannot be negative", that is a [newtype with a checked constructor](../../16_Structs/newtype_score/README.md), not a primitive.

## Single and double precision

`f64` is the default because it is roughly as fast as `f32` on any machine you will meet and remembers more than twice as much:

| | significand bits | reliable decimal digits | IEEE 754 name |
|---|---|---|---|
| `f32` | 24 | ~6 | binary32, **single precision** |
| `f64` | 53 | ~15 | binary64, **double precision** |

*Precision* here is a count of significant bits, not of decimal places — the significand is the part of the number that carries the digits, and everything past bit 24 (or 53) is cut. "Double precision" is the historical name for the wider of the two, from an era when the narrow one was the machine's native float and the wide one took two registers. Both are read straight from `f32::MANTISSA_DIGITS` / `f64::MANTISSA_DIGITS` in the [verified output](#the-verified-output).

More bits does not mean *exact*: `0.1` is not representable in either, and `f64` gets the same wrong answer 29 bits further right. [What a float actually stores](../what_a_float_stores/README.md) is where that stops being trivia.

## What the width promises

The suffix is a range, and the range follows from the bit count by one formula each:

```text
signed   iN : -2^(N-1)  ..=  2^(N-1) - 1        i8:  -128 ..= 127
unsigned uN :         0  ..=  2^N - 1           u8:     0 ..= 255
```

Both are checked against the compiler's own `MIN`/`MAX` in the output below rather than quoted. The reason for the shape is [two's complement](../why_hexadecimal/README.md): the top bit of a signed integer carries the sign, which halves the magnitude available and shifts the whole range down by one — so there is exactly **one more value below zero than above it**, and the most negative number has no positive counterpart:

```rust
assert_eq!(i8::MIN, -128);
assert_eq!(i8::MIN.checked_neg(), None);   // -(-128) is not an i8
assert_eq!(i8::MIN.checked_abs(), None);   // neither is its absolute value
```

`i8::MIN.abs()` therefore panics in a debug build and returns `-128` in a release one — an absolute value that is negative. It is the sharpest small example of the [debug/release overflow split](../meet_the_byte/README.md), and it is why `checked_abs` exists.

`isize` and `usize` are the odd pair: their width is the machine's pointer width, not a number in the name, and their job is **positions in memory** — every length, index, capacity and byte count in the standard library is a `usize`:

```rust
let names = ["Ada", "Ben", "Cara"];
let n: usize = names.len();          // len() is a usize, always
let i: usize = 1;
println!("{} of {n}", names[i]);     // Ben of 3 — an index must be a usize
```

That is the answer to *when do I use `usize`*: when the number counts or locates elements. A vote total is a `u32` because it is a quantity; the position of a candidate in a slice is a `usize` because it is an index. `usize` is also the reason a cast shows up when the two mix — `scores[i as usize]` when `i` arrived as a `u32`.

## If you are coming from another language

**Python.** Three of the four literal decorations transfer exactly, and the fourth is a trap dressed as a match.

| | Python | Rust |
|---|---|---|
| bases | `0x2A` `0o77` `0b101` — the same three prefixes | `0x2A` `0o77` `0b101` |
| bare leading zero | `SyntaxError` since Python 3 | legal and inert: `0755` is 755 |
| digit grouping | `1_000_000` | `1_000_000` — the same |
| type suffix | none — there is one `int` | `57u8`, `57_u8`, `2.0f32` |
| float default | `float` is always a C double | `f64`, and `f32` if you ask |
| single character | no such type — a 1-length `str` | `char`, four bytes |

The trap is `b'A'`. Both languages spell it the same way and mean different things: in Python `b'A'` is a **`bytes` object of length 1** — a container — and you have to index into it to get the number, which is why `b'A'[0]` is `65` while `b'A'` itself is not. In Rust `b'A'` **is** the number, a `u8`, with no container around it. The Python counterpart of Rust's `b'A'` is `ord('A')` or `b'A'[0]`; the Python counterpart of Rust's `b"Hi"` is `b'Hi'`. So the syntax you already know maps to the *plural* form and not the singular, which is exactly the direction that produces a confusing type error rather than a wrong number.

```python
b'A'        # bytes, length 1
b'A'[0]     # 65, an int      <- this is Rust's b'A'
b'Hi'       # bytes, length 2 <- this is Rust's b"Hi"
```

One more Python habit that does not survive: every width on this page is a constraint Python does not have. `2 ** 200` is exact there and `x + 1` cannot overflow, so choosing a suffix is a decision Python never asks you to make — and the `& 0xFF` sprinkled through Python bit-twiddling code is you re-imposing by hand the width a Rust suffix states once.

**ABAP.** The literal syntax is where the two languages diverge most, because ABAP's numeric types are declared rather than written on the value. There is no base prefix at all: a hexadecimal constant is a character string assigned to a type `X` field (`CONSTANTS c TYPE x LENGTH 1 VALUE '2A'`), so the quotes do the work Rust's `0x` does, and the "base" is a property of the *target type* rather than of the text. Grouping separators are likewise absent — `1_000_000` is not ABAP, and a literal that long is simply typed out.

Two things transfer cleanly and one does not. The suffix's job — *say the width at the point of the value* — is done in ABAP by `TYPE b` / `TYPE s` / `TYPE i` / `TYPE int8` on the declaration, so the idea of committing to a width is familiar even though the syntax is not; and the byte/text split is real in both, since ABAP's `X` and `XSTRING` are Rust's `u8` and `Vec<u8>`, with `STRING` on the other side. What does not transfer is signedness: ABAP has **no unsigned integer type**, so the `u8`/`u32` habit of *this quantity can never be negative, and the type says so* has no ABAP counterpart to lean on — the nearest thing is a domain with a value range, which is checked by the dictionary rather than by the compiler. Going the other way, ABAP's packed decimal `p` has no Rust primitive at all, which is why exact money in Rust is [an integer of cents](../../09_Advanced/scaled_integers/README.md).

---

## Practice

**Two decisions, one literal.** A number literal commits you to a base and a width, and both choices are readable by the next person.

Take three fields — a Unix file mode, an RGB colour, a set of permission flags — and write each in the base whose digits line up with its structure, printing enough to show that the alignment is real (three-bit groups for the mode, byte boundaries for the colour, one bit per flag). Then take five quantities from a real program — a STAR score, a byte of a scanned ballot, the ballots in one precinct, the votes in a US election, a Unicode code point — and pick the narrowest unsigned type that holds each.

One of those five is where the obvious answer is wrong. Find it, then show both ways Rust tells you: what `u16::try_from` does at run time, and what the compiler does when the literal itself is too big.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:writing_a_number_down_kata -->
*[`writing_a_number_down_kata.rs`](examples/writing_a_number_down_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the two decisions a literal makes — which base shows the
//! structure, and which width is narrow enough to be a claim.

fn main() {
    println!("=== part 1: write each field in the base its digits line up with ===");

    // A Unix file mode is nine bits in three groups of three: octal.
    let mode = 0o644;
    println!("  file mode   0o644  = {:>10}  = {:09b}  = rw- r-- r--", mode, mode);
    println!("              same value in hex, 0x1A4, hides the three groups: {:#x}", mode);

    // An RGB colour is three bytes: hex, two digits each.
    let teal: u32 = 0x00_8B_8B;
    println!(
        "  colour      0x008B8B = {:>10}  = r {:3}  g {:3}  b {:3}",
        teal,
        (teal >> 16) & 0xFF,
        (teal >> 8) & 0xFF,
        teal & 0xFF
    );
    println!("              same value in decimal, {}, hides the byte edges", teal);

    // A flag set is one bit per option: binary.
    const READ: u8 = 0b0000_0001;
    const WRITE: u8 = 0b0000_0010;
    const EXEC: u8 = 0b0000_0100;
    let perms = READ | WRITE;
    println!("  flags       0b0000_0011 = {:>4}  = {:08b}   read {} write {} exec {}",
             perms, perms, perms & READ != 0, perms & WRITE != 0, perms & EXEC != 0);

    println!("\n=== part 2: the narrowest type that holds the quantity ===");
    println!("  {:<26} {:>13}  {:<6} {:>21}", "quantity", "worst case", "type", "that type's MAX");
    let rows: [(&str, u128, &str, u128); 5] = [
        ("a STAR score, 0-5", 5, "u8", u8::MAX as u128),
        ("a byte of a ballot scan", 255, "u8", u8::MAX as u128),
        ("ballots in one precinct", 100_000, "u32", u32::MAX as u128),
        ("votes in a US election", 160_000_000, "u32", u32::MAX as u128),
        ("a Unicode code point", 0x10_FFFF, "u32", u32::MAX as u128),
    ];
    for (what, worst, ty, max) in rows {
        println!("  {:<26} {:>13}  {:<6} {:>21}", what, worst, ty, max);
    }

    println!("\n=== the row where the obvious answer is wrong ===");
    println!("  'a precinct is small, u16 is plenty' -- u16::MAX = {}", u16::MAX);
    for ballots in [50_000u32, 100_000u32] {
        match u16::try_from(ballots) {
            Ok(n) => println!("    {ballots:>7} ballots -> u16 holds it: {n}"),
            Err(e) => println!("    {ballots:>7} ballots -> u16 refuses: {e}"),
        }
    }
    println!("  the literal itself is refused at compile time, not at run time --");
    println!("  `let n = 100_000u16;` is rejected by a deny-by-default lint, verbatim:");
    println!("    error: literal out of range for `u16`");
    println!("    = note: the literal `100_000u16` does not fit into the type `u16` whose range is `0..=65535`");
    println!("    = note: `#[deny(overflowing_literals)]` on by default");

    println!("\n=== and the signed row, where the hole is at the bottom ===");
    let margin: i8 = -128;
    println!("  a margin of {margin} fits i8 (MIN is {})", i8::MIN);
    println!("  but its own absolute value does not: {:?}", margin.checked_abs());
    println!("  which is why `i8::MIN.abs()` panics in debug and wraps to {} in release",
             i8::MIN.wrapping_abs());
}
```
<!-- /source -->

<!-- output:writing_a_number_down_kata -->
*Verified output of [`writing_a_number_down_kata.rs`](examples/writing_a_number_down_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== part 1: write each field in the base its digits line up with ===
  file mode   0o644  =        420  = 110100100  = rw- r-- r--
              same value in hex, 0x1A4, hides the three groups: 0x1a4
  colour      0x008B8B =      35723  = r   0  g 139  b 139
              same value in decimal, 35723, hides the byte edges
  flags       0b0000_0011 =    3  = 00000011   read true write true exec false

=== part 2: the narrowest type that holds the quantity ===
  quantity                      worst case  type         that type's MAX
  a STAR score, 0-5                      5  u8                       255
  a byte of a ballot scan              255  u8                       255
  ballots in one precinct           100000  u32               4294967295
  votes in a US election         160000000  u32               4294967295
  a Unicode code point             1114111  u32               4294967295

=== the row where the obvious answer is wrong ===
  'a precinct is small, u16 is plenty' -- u16::MAX = 65535
      50000 ballots -> u16 holds it: 50000
     100000 ballots -> u16 refuses: out of range integral type conversion attempted
  the literal itself is refused at compile time, not at run time --
  `let n = 100_000u16;` is rejected by a deny-by-default lint, verbatim:
    error: literal out of range for `u16`
    = note: the literal `100_000u16` does not fit into the type `u16` whose range is `0..=65535`
    = note: `#[deny(overflowing_literals)]` on by default

=== and the signed row, where the hole is at the bottom ===
  a margin of -128 fits i8 (MIN is -128)
  but its own absolute value does not: None
  which is why `i8::MIN.abs()` panics in debug and wraps to -128 in release
```
<!-- /output -->

</details>

## The verified output

<!-- output:writing_a_number_down -->
*Verified output of [`writing_a_number_down.rs`](examples/writing_a_number_down.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== the 0 says 'not decimal', the letter names the base ===
  0x2A   hex     = 42
  0o77   octal   = 63
  0b101  binary  = 5
  42     decimal = 42
  and all four spell the same u8 when the number is the same:
    0xFF == 0o377 == 0b1111_1111 == 255 : true
  case does not matter in hex: 0xff == 0xFF : true

=== a leading zero on its own means nothing ===
  0755  = 755   <- decimal seven hundred fifty-five, NOT octal
  0o755 = 493   <- octal: rwx r-x r-x
  0000042 = 42  <- leading zeros are legal and inert

=== the underscore is for your eyes, anywhere in the literal ===
  1_000_000     = 1000000
  0xDEAD_BEEFu32 = 3735928559
  0b1011_1110   = 190
  1_0_0         = 100   <- legal, and nobody should

=== the suffix is the type, written on the number ===
  57u8  = 57 : u8
  57_u8 = 57 : u8   <- same literal, the _ is optional
  let c: u8 = 57 -> 57 : u8   <- same instruction, said on the let
  57    = 57 : i32   <- the integer fallback
  5.7   = 5.7 : f64   <- the float fallback

=== b in front of a quote means bytes, not text ===
  b'A'  = 65  : u8         1 byte
  'A'   = 65  : char       4 bytes
  b"Hi" = [72, 105] : &[u8; 2]
  "Hi"  = "Hi" : &str
  b'A' is a NUMBER you may do arithmetic on: b'A' + 2 = 67 = 'C'

=== floats: the dot is required, and there is no unsigned one ===
  2.0   = 2 : f64
  2.    = 2 : f64   <- legal; 2 alone would be an integer
  1e6   = 1000000 : f64
  2.0f32 = 2 : f32
  2_f32  = 2 : f32   <- no dot needed once the suffix says float
  there is no uf32/uf64: every float carries a sign bit, always
    -0.0 == 0.0                    : true
    (-0.0f64).is_sign_negative()   : true   <- the bit is there either way

=== how much a float remembers ===
  f32  24 significand bits, ~6 reliable decimal digits (single precision)
  f64  53 significand bits, ~15 reliable decimal digits (double precision)
  0.1f32 = 0.10000000149011611938
  0.1f64 = 0.10000000000000000555   <- same wrong answer, 29 bits further right

=== what the width promises, by formula ===
  signed   iN : -2^(N-1)  ..=  2^(N-1) - 1
  unsigned uN :         0  ..=  2^N - 1
  type                     MIN                   MAX   formula check
  i8                      -128                   127   true
  i16                   -32768                 32767   true
  i32              -2147483648            2147483647   true
  i64     -9223372036854775808   9223372036854775807   true
  u8                         0                   255   true
  u16                        0                 65535   true
  u32                        0            4294967295   true
  u64                        0  18446744073709551615   true

=== the asymmetry the formula predicts ===
  i8::MIN = -128, i8::MAX = 127   <- one more below zero than above
  i8::MIN.checked_neg()  = None   <- -(-128) is not an i8
  i8::MIN.checked_abs()  = None   <- neither is its absolute value
  u8 has no such hole: 0 is its own negation, and there is nothing below it
```
<!-- /output -->

## See also

- [Values](../../15_First_Programs/values/README.md) — the census this page zooms into: every type you can write a literal for, and how wide it is
- [Meet the byte](../meet_the_byte/README.md) — what `u8` is once you can write one down, and the bill a width comes with
- [Why hexadecimal](../why_hexadecimal/README.md) — why `0x` is the right prefix for a byte and `0o` for a file mode
- [What a float actually stores](../what_a_float_stores/README.md) — where "53 significand bits" stops being trivia
- [Type inference](../../15_First_Programs/type_inference/README.md) — what decides the type when there is no suffix
- [Meet the `char`](../../14_Strings/meet_the_char/README.md) — the other half of `b'A'` versus `'A'`
- [The Rust Reference: Literal expressions ↗](https://doc.rust-lang.org/reference/expressions/literal-expr.html) — the grammar itself, including every suffix the parser accepts

## Po polsku

Do literału liczbowego można w Ruscie dokleić cztery rzeczy i każda z nich mówi kompilatorowi coś, czego inaczej musiałby się domyślać: przedrostek podstawy, podkreślniki, przyrostek typu oraz `b` przed cudzysłowem. Przedrostek zapamiętuje się jedną regułą — **zero znaczy „to nie jest liczba dziesiętna”, a litera po nim to pierwsza litera angielskiej nazwy podstawy**: `0x` to *he**x**adecimal* (szesnastkowa), `0o` to ***o**ctal* (ósemkowa), `0b` to ***b**inary* (dwójkowa). Nic więcej nie ma do zapamiętania. Wielkość liter w cyfrach nie ma znaczenia (`0xff` to to samo co `0xFF`), a sama litera przedrostka jest zawsze mała.

Samo zero z przodu nic nie znaczy: `0755` to dziewięćset… to znaczy siedemset pięćdziesiąt pięć, zapisane dziesiętnie. W C `0755` **jest** liczbą ósemkową i to źródło długiej listy błędów; Rust tej składni nigdy nie przyjął, a Python 3 się jej pozbył — jego komunikat wprost podaje zamiennik: *„leading zeros in decimal integer literals are not permitted; use an 0o prefix for octal integers”*. Wybór podstawy nie jest kwestią gustu, tylko tego, czy cyfry pokrywają się z polami: jedna cyfra szesnastkowa to cztery bity, więc bajt to zawsze dwie cyfry; jedna cyfra ósemkowa to trzy bity, więc dziewięciobitowa maska uprawnień to zawsze trzy.

Przyrostek to typ zapisany na liczbie: `57u8`, `57_u8` i `let c: u8 = 57` to jedna instrukcja powiedziana w trzech miejscach, a podkreślnik przed przyrostkiem jest wyłącznie kosmetyczny. Gdy nic nie wskazuje typu, Rust przyjmuje `i32` dla liczby całkowitej i `f64` dla zmiennoprzecinkowej — `1` nie staje się `u8` dlatego, że jest małe.

`b` przed cudzysłowem oznacza bajty, nie tekst, i tu czeka pułapka na osoby przychodzące z Pythona, bo **oba języki piszą `b'A'` tak samo, a znaczą co innego**. W Pythonie `b'A'` to obiekt `bytes` o długości 1 — pojemnik — i dopiero `b'A'[0]` daje liczbę 65. W Ruscie `b'A'` **jest** liczbą: `u8` o wartości 65, bez żadnego pojemnika. Odpowiednikiem rustowego `b'A'` jest więc pythonowe `ord('A')` albo `b'A'[0]`, a odpowiednikiem rustowego `b"Hi"` — pythonowe `b'Hi'`. Znana składnia odwzorowuje się na liczbę mnogą, nie pojedynczą.

Liczby zmiennoprzecinkowe wymagają kropki (`2.0`, albo `2.`), a `2.f32` się nie skompiluje — Rust czyta to jako „pole `f32` liczby 2” i podpowiada dopisanie zera. Typu bez znaku nie ma i być nie może: IEEE 754 umieszcza bit znaku na szczycie każdej liczby zmiennoprzecinkowej, więc znakowość jest częścią formatu, a nie wyborem typu — dlatego istnieją dwa zera, `-0.0` i `0.0`, które są sobie równe, choć bit znaku jest ustawiony tylko w jednym. **Podwójna precyzja** (*double precision*) to historyczna nazwa szerszego z dwóch formatów IEEE 754: `f32` (binary32, pojedyncza precyzja) pamięta 24 bity znaczące, czyli około 6 wiarygodnych cyfr dziesiętnych, a `f64` (binary64) — 53 bity, czyli około 15. Precyzja liczy się tu w bitach znaczących, nie w miejscach po przecinku, i więcej bitów nie znaczy „dokładnie”: `0.1` nie jest reprezentowalne w żadnym z nich.

Na koniec zakresy, bo przyrostek jest w istocie deklaracją zakresu. Dla typu ze znakiem jest to `-2^(N-1) ..= 2^(N-1) - 1`, dla typu bez znaku `0 ..= 2^N - 1` — stąd `i8` obejmuje −128…127, a `u8` 0…255. Kształt bierze się z kodu uzupełnień do dwóch (*two's complement*): najstarszy bit liczby ze znakiem niesie znak, więc **poniżej zera mieści się dokładnie o jedną wartość więcej niż powyżej**, a najbardziej ujemna liczba nie ma dodatniego odpowiednika. Dlatego `i8::MIN.checked_neg()` i `i8::MIN.checked_abs()` zwracają `None`, a `i8::MIN.abs()` panikuje w kompilacji debug i zwraca −128 w release — wartość bezwzględna, która jest ujemna. Osobno stoją `isize` i `usize`, których szerokość to szerokość wskaźnika tej maszyny; ich zadaniem są **pozycje w pamięci**, więc każda długość, indeks, pojemność i liczba bajtów w bibliotece standardowej jest typu `usize`. Prosta reguła: wielkość mierzalna (liczba głosów) to `u32`, a pozycja w kolekcji to `usize`.

**Szukaj po polsku:** literał liczbowy · system szesnastkowy ósemkowy dwójkowy · kod uzupełnień do dwóch · podwójna precyzja · `rust literał 0x 0o 0b` · `rust u8 usize kiedy`
