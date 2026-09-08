# Why a `char` is 32 bits wide

**Level:** 201 → 301 · deep dive

**One line:** The largest Unicode scalar value is U+10FFFF, which needs 21 bits — and 21 is not a width a machine can address, so `char` rounds up to 32. The 11 bits left over are not waste: they are why `char::from_u32` can refuse, and why `Option<char>` still costs four bytes.

```rust
let max = char::MAX as u32;
println!("{max:X}");                     // 10FFFF
println!("{}", 32 - max.leading_zeros()); // 21
println!("{}", size_of::<char>());        // 4
```

Every table of Rust's primitive types says *`char` — Unicode character, 32 bits wide*, and leaves it there. This page is the arithmetic behind the number. [Meet the `char`](../meet_the_char/README.md) is the other half — what a `char` *is*, and why `.len()` and `.chars().count()` disagree.

## 21 bits, rounded up

A `char` holds one **[Unicode scalar value ↗](https://www.unicode.org/glossary/#unicode_scalar_value)**: a code point in `0..=0x10FFFF` that is not half of a surrogate pair. Write the top of that range in binary and the width falls out:

```text
0x10FFFF = 1 0000 1111 1111 1111 1111      21 bits
```

21 bits does not fit in 16, so the next size up is what `char` gets. The [`char` type ↗](https://doc.rust-lang.org/reference/types/textual.html) is defined by that range, not by what Unicode has assigned — which is why `char::MAX` has been `U+10FFFF` since 1.0 and will not move when new scripts are added.

## Why not 16

Unicode was 16 bits until 1996, and the languages designed in that window — Java, JavaScript, and Windows' own API — still carry the decision. Then the range grew past `U+FFFF`, and **UTF-16** kept those APIs working by encoding anything above it as a *pair* of 16-bit units drawn from a reserved block, `D800..=DFFF`.

That is where the 2,048-value hole in the range comes from, and it is worth being precise about who pays for it: the hole is in **Unicode**, not in UTF-16 — every encoding has to route around it, including UTF-8, which needs no surrogates of its own.

16 bits reaches 63,488 scalar values of 1,112,064, under 6% of the space — but very nearly all everyday text, which is exactly why it looked sufficient. A width chosen for the common case is a width that fails on a schedule you do not control.

## Why not 21, or 24

Machines address memory in units of 1, 2, 4 and 8 bytes. A 21-bit `char` would not be addressable at all, and a 3-byte one would make `arr[i]` a multiply-by-3 with a misaligned load on every element — paying in instructions, on every access, to save one byte on a type most programs hold a handful of at a time. `char` is a *value* type, not a storage format; the storage format is UTF-8 and it is variable-width precisely because that is where the bytes are.

## Not every 32-bit pattern is a `char`

```rust
char::from_u32(0x000041)  // Some('A')
char::from_u32(0x00D800)  // None      — half of a surrogate pair
char::from_u32(0x110000)  // None      — one past the top
```

`from_u32` returns `Option<char>` rather than `char` for this reason, and `transmute`-ing a `u32` into a `char` is undefined behaviour rather than merely unwise: the compiler is entitled to assume the invariant holds.

`U+FFFE` is accepted, which is the distinction worth holding onto — it is a permanent *noncharacter*, and still a perfectly good scalar value. `char` promises the **range**, not that Unicode has assigned anything there.

## The bits left over are spent, not wasted

A type with impossible bit patterns has *niches*, and the compiler spends them on discriminants:

| type | size | why |
|---|---|---|
| `char` | 4 | the value |
| [`Option<char>`](../../17_Option_and_Result/some_and_none/README.md) | 4 | `None` lives in an invalid pattern |
| `Option<Option<char>>` | 4 | there are 4.29 billion spare patterns; two of them are cheap |
| `u32` | 4 | every pattern is a valid `u32` |
| `Option<u32>` | **8** | no niche, so the tag needs its own space, then padding |

The 11 "wasted" bits buy back the word that `Option<u32>` has to spend.

## Four bytes is what a `char` costs, not what text costs

```rust
let s = "Hello, 字!";
println!("{}", s.len());                      // 11 — UTF-8 bytes
println!("{}", s.chars().count() * 4);        // 36 — if you stored it as [char]
```

A `String` is UTF-8, so this is the trade the language already made for you: **decoded values are fixed-width so they can be worked on; encoded bytes are variable-width so they can be stored small.** Storing text as `Vec<char>` costs ASCII 4× and buys nothing an index into UTF-8 could not give you — see [String slices](../string_slices/README.md) for what that index does instead.

## `\x` stops at 7F

The escape in that primitive-types table (`'\x7f'`) is not a general one. Rust's `\x` takes exactly two hex digits and refuses anything above `7F`:

```text
error: out of range hex escape
 --> escapes.rs:2:14
  |
2 |     let c = '\xCA';
  |              ^^^^ must be a character in the range [\x00-\x7f]
  |
  = help: if you want to write a byte literal, use `b'\xCA'`
  = help: if you want to write a Unicode character, use `'\u{CA}'`
```

Both suggestions are real, and they are different types: `b'\xCA'` is a `u8`, `'\u{CA}'` is a `char`. The `\x` form is capped at ASCII so that a source file cannot smuggle in a byte that is not valid UTF-8 on its own — [Raw strings, escapes and the literal prefixes](../raw_strings_and_escapes/README.md) has the rest of the family.

## If you are coming from another language

**Python.** No `char` type at all — a single character is a length-1 `str`, and `ord()` / `chr()` cross between the character and its code point. But CPython does the same 21-bit accounting one level down: since [PEP 393 ↗](https://peps.python.org/pep-0393/) a `str` is stored at a fixed width *chosen per string* — 1, 2 or 4 bytes per character, whichever the widest character needs — so a string of ASCII with one emoji appended becomes 4 bytes per character throughout. That is Rust's `char` decision applied to whole strings, and it buys the same thing: `s[0]` in O(1).

| Python | | Rust |
|---|---|---|
| `ord('字')` → 23383 | character → code point | `'字' as u32` |
| `chr(0x0CA0)` → `'ಠ'` | code point → character | [`char::from_u32(0xCA0)` ↗](https://doc.rust-lang.org/std/primitive.char.html#method.from_u32) → `Some('ಠ')` |
| `chr(0xD800)` → `'\ud800'` | a surrogate | `None` — Rust refuses it |
| `sys.maxunicode` → 1114111 | the top of the range | `char::MAX` → the same number |
| 1, 2 or 4 bytes per char, per string | storage | UTF-8, 1–4 bytes per char, per character |

The row that matters is the third: Python will hand you a lone surrogate and let you carry it around until something tries to encode it. Rust has no value that can hold one.

**ABAP.** ABAP text on a Unicode system is UTF-16, so it inherits the 1996 decision in full. `TYPE c LENGTH 1` is one 16-bit **code unit**, not one character — a character above `U+FFFF` occupies *two* of them, `strlen` counts units, and an offset slice (`lv+2(1)`) can land between the halves of a surrogate pair and hand you back half a character at run time.

| ABAP | | Rust |
|---|---|---|
| `TYPE c LENGTH 1` | one 16-bit code unit | `char` — one scalar value, 32 bits |
| an emoji costs 2 units | above the BMP | one `char`, or 4 UTF-8 bytes in a `String` |
| `strlen( lv )` | counts code units | `.len()` counts bytes, `.chars().count()` counts scalars |
| `lv+2(1)` mid-pair | yields half a character, silently | `&s[2..3]` panics, `.get(2..3)` returns `None` |
| `string` vs `xstring` | text vs raw bytes | `String` vs `Vec<u8>` |

What changes is not the arithmetic but *when you find out*. Both languages have a unit smaller than a character; ABAP lets a bad offset produce a value, and Rust has no way to represent the bad value at all. (ABAP behaviour here is prose, not machine-checked — CI cannot run ABAP. Verify code-page specifics against your own system.)

## The bytes underneath

This page stops at the type. What UTF-8 and UTF-16 actually *do* with those 21 bits — and why the surrogate block has to exist — is the subject of the sibling [encodings learning library ↗](https://masiarek.github.io/encodings-learning-library/):

- [Unicode code points ↗](https://masiarek.github.io/encodings-learning-library/02_Characters/unicode_code_points/index.html) — where `U+10FFFF` comes from, and why the ceiling is that number and not a round one
- [UTF-16 and surrogates ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/utf16_and_surrogates/index.html) — the pairing scheme that reserved `D800..=DFFF`
- [UTF-8 by hand ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/utf8_by_hand/index.html) — encoding a scalar value into 1–4 bytes with a pencil

---

## Practice

**Rebuild `char::from_u32`, then price the width.** Write `is_scalar(n: u32) -> bool` from the definition alone — at or below `U+10FFFF`, not in the surrogate block — and prove it agrees with `char::from_u32` on every value from 0 to a little past the top. Report the exact number it refuses inside the range, and say which encoding reserved them and why UTF-8 pays for the hole anyway.

Then measure what a fixed width costs: for ASCII, Polish, CJK and emoji text, print the char count, the UTF-8 byte count, the `[char]` byte count, and the ratio — then say which of the four pays most and whether that is the text the fixed width was for. Finish with `size_of` on `char`, `Option<char>`, `Option<Option<char>>`, `Result<char, ()>`, `u32` and `Option<u32>`, and explain the one row that is twice the size of the others.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:why_char_is_32_bits_kata -->
*[`why_char_is_32_bits_kata.rs`](examples/why_char_is_32_bits_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: rebuild `char::from_u32` from the definition, count the exact
//! holes in the range, then price the fixed width against UTF-8.
//!
//!   rustc --edition 2024 why_char_is_32_bits_kata.rs -o /tmp/wc32k && /tmp/wc32k

use std::mem::size_of;

/// A Unicode *scalar value*: a code point at or below U+10FFFF that is not one
/// half of a UTF-16 surrogate pair. That sentence is the whole definition of
/// what a `char` may hold.
fn is_scalar(n: u32) -> bool {
    n <= 0x10FFFF && !(0xD800..=0xDFFF).contains(&n)
}

fn main() {
    println!("Round 1 — agree with the standard library, then count the holes");
    let probe = 0..=0x11_FFFFu32; // a little past the top, to catch an off-by-one
    let disagreements = probe.clone().filter(|&n| is_scalar(n) != char::from_u32(n).is_some()).count();
    println!("   probed {} values, disagreements with char::from_u32: {disagreements}", probe.clone().count());

    let accepted = probe.clone().filter(|&n| is_scalar(n)).count();
    let surrogates = (0..=0x10FFFFu32).filter(|&n| !is_scalar(n)).count();
    println!("   accepted            {accepted}");
    println!("   0x110000 - accepted {}", 0x110000 - accepted);
    println!("   refused inside the range (the surrogates) {surrogates} = 0x{surrogates:X}");
    println!("   The hole is exactly D800..=DFFF, which UTF-16 reserved to encode");
    println!("   everything above U+FFFF. UTF-8 needs no such reservation and pays");
    println!("   for the hole anyway, because the hole is in Unicode, not in UTF-16.");

    println!("\nRound 2 — the price of a fixed width");
    println!("   {:>5}  {:>5}  {:>6}  {:>5}   text", "chars", "UTF-8", "[char]", "ratio");
    for s in ["plain ascii", "zażółć gęślą jaźń", "日本語のテキスト", "🦀🦀🦀"] {
        let n = s.chars().count();
        let utf8 = s.len();
        let fixed = n * size_of::<char>();
        println!("   {n:>5}  {utf8:>5}  {fixed:>6}  {:>4.1}x   {s:?}", fixed as f64 / utf8 as f64);
    }
    println!("   The ratio is worst for the text that never needed the range: ASCII");
    println!("   pays 4x, and the emoji that genuinely needs 21 bits pays 1.0x. A");
    println!("   fixed width taxes the common case to make the rare case uniform.");

    println!("\nRound 3 — where the 11 spare bits went");
    println!("   size_of::<char>()                 = {}", size_of::<char>());
    println!("   size_of::<Option<char>>()         = {}", size_of::<Option<char>>());
    println!("   size_of::<Option<Option<char>>>() = {}", size_of::<Option<Option<char>>>());
    println!("   size_of::<Result<char, ()>>()     = {}", size_of::<Result<char, ()>>());
    println!("   size_of::<u32>()                  = {}", size_of::<u32>());
    println!("   size_of::<Option<u32>>()          = {}", size_of::<Option<u32>>());
    let niches = (u32::MAX as u64 + 1) - accepted as u64;
    println!("   Every invalid bit pattern is a niche the compiler can spend on a");
    println!("   discriminant. char has {niches} of them; u32 has none, so the");
    println!("   same Option doubles its size.");
}
```
<!-- /source -->

<!-- output:why_char_is_32_bits_kata -->
*Verified output of [`why_char_is_32_bits_kata.rs`](examples/why_char_is_32_bits_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Round 1 — agree with the standard library, then count the holes
   probed 1179648 values, disagreements with char::from_u32: 0
   accepted            1112064
   0x110000 - accepted 2048
   refused inside the range (the surrogates) 2048 = 0x800
   The hole is exactly D800..=DFFF, which UTF-16 reserved to encode
   everything above U+FFFF. UTF-8 needs no such reservation and pays
   for the hole anyway, because the hole is in Unicode, not in UTF-16.

Round 2 — the price of a fixed width
   chars  UTF-8  [char]  ratio   text
      11     11      44   4.0x   "plain ascii"
      17     26      68   2.6x   "zażółć gęślą jaźń"
       8     24      32   1.3x   "日本語のテキスト"
       3     12      12   1.0x   "🦀🦀🦀"
   The ratio is worst for the text that never needed the range: ASCII
   pays 4x, and the emoji that genuinely needs 21 bits pays 1.0x. A
   fixed width taxes the common case to make the rare case uniform.

Round 3 — where the 11 spare bits went
   size_of::<char>()                 = 4
   size_of::<Option<char>>()         = 4
   size_of::<Option<Option<char>>>() = 4
   size_of::<Result<char, ()>>()     = 4
   size_of::<u32>()                  = 4
   size_of::<Option<u32>>()          = 8
   Every invalid bit pattern is a niche the compiler can spend on a
   discriminant. char has 4293855232 of them; u32 has none, so the
   same Option doubles its size.
```
<!-- /output -->

</details>

---

## The verified output

<!-- source:why_char_is_32_bits -->
*[`why_char_is_32_bits.rs`](examples/why_char_is_32_bits.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Why a `char` is 32 bits: the largest Unicode scalar value needs 21 of them,
//! 21 is not a width a machine can address, and the 11 left over are not waste.
//!
//!   rustc --edition 2024 why_char_is_32_bits.rs -o /tmp/wc32 && /tmp/wc32

use std::mem::size_of;

fn main() {
    println!("1. The measurement");
    let max = char::MAX as u32;
    println!("   size_of::<char>()  {} bytes = {} bits", size_of::<char>(), size_of::<char>() * 8);
    println!("   char::MAX          U+{max:X} = {max}");
    println!("   in binary          {max:b}");
    println!("   bits it needs      {}", u32::BITS - max.leading_zeros());

    println!("\n2. Why not 16 bits");
    for c in ['*', '字', '😀'] {
        let n = c as u32;
        let verdict = if n <= 0xFFFF { "fits in 16" } else { "does NOT fit in 16" };
        println!("   U+{:<6X} {:>2} bits  {:<19} {c:?}", n, u32::BITS - n.leading_zeros(), verdict);
    }
    let total = (0..=max).filter(|&n| char::from_u32(n).is_some()).count();
    let in_bmp = (0..=0xFFFF).filter(|&n| char::from_u32(n).is_some()).count();
    println!("   16 bits reaches {in_bmp} scalar values of {total} -- under 6% of the space.");
    println!("   Almost all everyday text lives in those, which is why 16 bits looked");
    println!("   like enough in 1991. It was not, and UTF-16 pairs up the rest.");

    println!("\n3. Not every 32-bit pattern is a char");
    for n in [0x41u32, 0xD800, 0xFFFE, 0x10FFFF, 0x110000] {
        println!("   char::from_u32(0x{n:06X}) = {:?}", char::from_u32(n));
    }
    println!("   0xD800 is half of a UTF-16 surrogate pair -- never a character alone.");
    println!("   0x110000 is one past the top. Neither is a bit pattern a char can hold.");
    println!("   U+FFFE is accepted: a permanent noncharacter is still a scalar value,");
    println!("   and `char` is defined by the scalar range, not by what Unicode assigns.");

    println!("\n4. What the spare bit patterns buy");
    println!("   21 bits could hold       {} values", 1u32 << 21);
    println!("   Unicode actually defines {total}");
    println!("   32 bits could hold       {} values", u32::MAX as u64 + 1);
    println!("   size_of::<Option<char>>()         = {}", size_of::<Option<char>>());
    println!("   size_of::<Option<Option<char>>>() = {}", size_of::<Option<Option<char>>>());
    println!("   size_of::<Option<u32>>()          = {}   <- u32 has no spare patterns,",
        size_of::<Option<u32>>());
    println!("      so its Option needs a tag beside the value, and then padding.");

    println!("\n5. Four bytes is what a char COSTS, not what text costs");
    let s = "Hello, 字!";
    let chars: Vec<char> = s.chars().collect();
    println!("   {s:?}");
    println!("   as a &str, UTF-8           {} bytes", s.len());
    println!("   as [char], one per scalar  {} bytes", chars.len() * size_of::<char>());
    println!("   Text is stored UTF-8 for this reason; char is the decoded form you");
    println!("   compare, classify and range over.");

    println!("\n6. The five literals from the primitive-types table");
    println!("   code pt   UTF-8  written        prints back as");
    for c in ['*', '\n', '字', '\x7f', '\u{CA0}'] {
        let written = match c {
            '*' => "'*'",
            '\n' => r"'\n'",
            '字' => "'字'",
            '\x7f' => r"'\x7f'",
            _ => r"'\u{CA0}'",
        };
        println!("   U+{:<6X} {} B      {:<14} {:?}", c as u32, c.len_utf8(), written, c);
    }
    println!("   Every one of them is 4 bytes as a char. The UTF-8 column is what the");
    println!("   same character costs inside a String -- and \\x is ASCII-only, so");
    println!("   anything above U+007F has to be written \\u{{...}}.");
}
```
<!-- /source -->

<!-- output:why_char_is_32_bits -->
*Verified output of [`why_char_is_32_bits.rs`](examples/why_char_is_32_bits.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The measurement
   size_of::<char>()  4 bytes = 32 bits
   char::MAX          U+10FFFF = 1114111
   in binary          100001111111111111111
   bits it needs      21

2. Why not 16 bits
   U+2A      6 bits  fits in 16          '*'
   U+5B57   15 bits  fits in 16          '字'
   U+1F600  17 bits  does NOT fit in 16  '😀'
   16 bits reaches 63488 scalar values of 1112064 -- under 6% of the space.
   Almost all everyday text lives in those, which is why 16 bits looked
   like enough in 1991. It was not, and UTF-16 pairs up the rest.

3. Not every 32-bit pattern is a char
   char::from_u32(0x000041) = Some('A')
   char::from_u32(0x00D800) = None
   char::from_u32(0x00FFFE) = Some('\u{fffe}')
   char::from_u32(0x10FFFF) = Some('\u{10ffff}')
   char::from_u32(0x110000) = None
   0xD800 is half of a UTF-16 surrogate pair -- never a character alone.
   0x110000 is one past the top. Neither is a bit pattern a char can hold.
   U+FFFE is accepted: a permanent noncharacter is still a scalar value,
   and `char` is defined by the scalar range, not by what Unicode assigns.

4. What the spare bit patterns buy
   21 bits could hold       2097152 values
   Unicode actually defines 1112064
   32 bits could hold       4294967296 values
   size_of::<Option<char>>()         = 4
   size_of::<Option<Option<char>>>() = 4
   size_of::<Option<u32>>()          = 8   <- u32 has no spare patterns,
      so its Option needs a tag beside the value, and then padding.

5. Four bytes is what a char COSTS, not what text costs
   "Hello, 字!"
   as a &str, UTF-8           11 bytes
   as [char], one per scalar  36 bytes
   Text is stored UTF-8 for this reason; char is the decoded form you
   compare, classify and range over.

6. The five literals from the primitive-types table
   code pt   UTF-8  written        prints back as
   U+2A     1 B      '*'            '*'
   U+A      1 B      '\n'           '\n'
   U+5B57   3 B      '字'            '字'
   U+7F     1 B      '\x7f'         '\u{7f}'
   U+CA0    3 B      '\u{CA0}'      'ಠ'
   Every one of them is 4 bytes as a char. The UTF-8 column is what the
   same character costs inside a String -- and \x is ASCII-only, so
   anything above U+007F has to be written \u{...}.
```
<!-- /output -->

## See also

- [Meet the `char`](../meet_the_char/README.md) — what a `char` is, and the four honest answers to "how long is this string"
- [`char` is four bytes ↗](https://masiarek.github.io/encodings-learning-library/05_Rust/char_is_four_bytes/index.html) — the encodings library on the same four bytes: the 21-bit ceiling and the 2,048-code-point hole as facts about Unicode first and about `char` second, and the width put beside what the character costs once it is encoded
- [Values](../../15_First_Programs/values/README.md) — the primitive-types table this page is a footnote to
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — the unit `.len()` counts in
- [Raw strings, escapes and the literal prefixes](../raw_strings_and_escapes/README.md) — `\u{…}`, `\x`, `b'…'` and the rest
- [`char` ↗](https://doc.rust-lang.org/std/primitive.char.html) · [Textual types ↗](https://doc.rust-lang.org/reference/types/textual.html) — the std page and the Reference's definition of the range

## Po polsku

Każda tabela typów podstawowych Rusta podaje, że `char` ma **32 bity**, i na tym poprzestaje. Rachunek jest krótki: `char` mieści jedną **wartość skalarną Unicode** (*Unicode scalar value*), czyli punkt kodowy z zakresu `0..=0x10FFFF`, który nie jest połówką pary zastępczej (*surrogate pair*). Największa taka wartość zapisana dwójkowo zajmuje **21 bitów** — a 21 nie jest szerokością, którą maszyna potrafi zaadresować, więc typ zaokrągla się w górę do 32. Warto zapamiętać, że zakres jest częścią *definicji typu*, a nie stanem tablic Unicode: `char::MAX` to `U+10FFFF` od wersji 1.0 i nie przesunie się, gdy dojdą nowe pisma.

Dlaczego nie 16 bitów? Bo do 1996 roku Unicode faktycznie w nich mieścił się — i języki zaprojektowane w tamtym oknie (Java, JavaScript, API Windows, a także tekst w ABAP-ie) noszą tę decyzję do dziś. Gdy zakres urósł ponad `U+FFFF`, ratunkiem stało się **UTF-16**: wszystko powyżej koduje się *parą* jednostek z zarezerwowanego bloku `D800..=DFFF`. Stąd dziura licząca 2048 wartości — i tu jedno sprostowanie, które łatwo przeoczyć: ta dziura jest w **Unicode**, a nie w UTF-16, więc omija ją każde kodowanie, łącznie z UTF-8, które własnych par zastępczych nie potrzebuje. Szesnaście bitów sięga 63 488 wartości z 1 112 064, czyli niecałych 6% przestrzeni — ale niemal całego tekstu, jaki spotyka się na co dzień, i dlatego wyglądało na wystarczające.

Bity, które zostają, nie są zmarnowane. Skoro nie każdy układ 32 bitów jest poprawnym `char`em, kompilator ma **nisze** (*niches*) i wydaje je na znacznik wariantu: `Option<char>` zajmuje nadal 4 bajty, a nawet `Option<Option<char>>`, podczas gdy `Option<u32>` puchnie do **8**, bo `u32` nie ma ani jednego nieużywanego układu bitów. To ta sama własność, dzięki której `char::from_u32` może zwrócić `None` zamiast czegokolwiek — a `transmute` z `u32` na `char` jest niezdefiniowanym zachowaniem, nie tylko nieostrożnością.

Na koniec rozróżnienie, które w praktyce oszczędza najwięcej zaskoczeń: **cztery bajty to koszt `char`a, a nie koszt tekstu.** W `String`u obowiązuje UTF-8, więc polskie `ą ż ł` zajmują tam po dwa bajty, a nie po cztery; `"Hello, 字!"` to 11 bajtów jako `&str` i 36 jako `[char]`. Wartość zdekodowana ma stałą szerokość, żeby dało się na niej *liczyć* (porównywać, klasyfikować, robić zakresy `'0'..='9'`), a bajty zakodowane mają zmienną, żeby dało się je *przechowywać* małym kosztem. Trzymanie tekstu jako `Vec<char>` kosztuje przy ASCII czterokrotnie więcej i nie kupuje niczego, czego nie dałby indeks w UTF-8.

**Szukaj po polsku:** wartość skalarna Unicode · punkt kodowy · para zastępcza · `rust char 4 bajty` · `rust char::from_u32` · `rust niche optimization Option` · `dlaczego char ma 32 bity`
