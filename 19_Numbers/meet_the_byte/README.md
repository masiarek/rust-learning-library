# Meet the byte

**Level:** 101 → 201 · working knowledge

**One line:** `u8` is not "a small integer" — it is one byte, the unit every `size_of` in Rust counts in, and giving a number a width hands you three jobs at once: deciding what happens on overflow, knowing which way a shift pads, and remembering that a string's `.len()` counts bytes rather than characters.

Most of what a program touches stays invisible at this level. You write `let total = 12;` and the machine underneath — the bits, the addresses, the eight-bit groups they arrive in — never surfaces. Rust puts it in the names of the types instead. `u8` says *unsigned, eight bits*. `i32` says *signed, thirty-two*. `usize` says *whatever this machine calls a pointer*. There is no type in the language that means simply "a number".

That is the trade this page is about. A language with one unbounded integer never asks you where the number ends, and in exchange it cannot tell you either. Rust asks on the first line you write, and then holds you to the answer — which is a real cost, paid in the three places below, and the reason its arithmetic is boring in production.

## Two readings of the same eight bits

A byte is eight bits, and it is an integer from 0 to 255. Not two things that correspond — one thing, seen twice, and `u8` is the type for both readings at once:

```rust
for b in [0u8, 1, 2, 89, 255] {
    println!("  {b:08b}  =  {b:3}  =  0x{b:02x}");
}
```

```text
  00000000  =    0  =  0x00
  01011001  =   89  =  0x59
  11111111  =  255  =  0xff
```

`0b0101_1001`, `89` and `0x59` are three spellings of one `u8` literal — binary, decimal and hexadecimal are notation, not different values, and the underscore is just a thousands separator for bits. Rust confirms the width itself: `u8::BITS` is `8` and `u8::MAX` is `255`, and both are associated constants you can read at compile time rather than facts you have to remember.

Note what Rust does *not* offer: a way to say "a byte is nine bits here". The zine's last panel is right that machines once disagreed — 2, 3, 4, 5, 6, 8 and 10-bit bytes all shipped — and Rust simply declines to target any of them. C keeps an escape hatch for it (`CHAR_BIT`); Rust has none, and `size_of` returns a count of eight-bit bytes on every platform the compiler supports.

## Memory is addressed one byte at a time

The unit is not a convention in Rust, it is arithmetic you can perform. Take the address of two neighbouring elements of a `[u8; 3]` and subtract:

```rust
let cells: [u8; 3] = [0b0101_0011, 0b0101_0101, 0b1011_0111];

let first  = &cells[0] as *const u8 as usize;
let second = &cells[1] as *const u8 as usize;
println!("{}", second - first);   // 1
```

One. Always one — and that is the definition of *byte-addressed*: consecutive bytes have consecutive addresses, so the smallest thing in memory that has an address of its own is a byte, never a bit. Casting a reference to a raw pointer and then to `usize` is safe (it is *dereferencing* one that needs `unsafe`), so this is a fact you can check rather than take on faith.

Those three bytes are worth one more look, because they say what a byte *is* better than any definition:

```text
  three bytes            : 01010011 01010101 10110111
  the same three as ints : [83, 85, 183]
  the same three as text : 'S' 'U' (183: no ASCII letter)
  str::from_utf8(&cells) : Err("invalid utf-8 sequence of 1 bytes from index 2")
```

Two of them happen to spell letters. The third is a perfectly ordinary byte — 183 is as valid a `u8` as 83 is — and there is simply no character at that number. The bytes are not text that failed; they are bytes, and text is one *interpretation* you can attempt on them. `str::from_utf8` is where that attempt either succeeds or, as here, reports precisely which byte it choked on.

## You cannot fetch one bit

Because the address belongs to the byte, "read bit 3" is never a single instruction. You fetch the whole byte and mask, and Rust gives you exactly the operators C does:

```rust
let b: u8 = 0b0101_1001;
(b >> 3) & 1          // read  -> 1
b & (1 << 6) != 0     // test  -> true
b | (1 << 1)          // set   -> 01011011
b & !(1 << 3)         // clear -> 01010001
!b                    // flip  -> 10100110
b.count_ones()        // popcount -> 4
```

The line worth staring at is `!b`. It is `10100110` — eight bits, still a `u8`, no cleanup required. That sounds too obvious to mention until you have written the same thing in a language whose integers have no width, where flipping the bits of 89 gives you −90 and you re-impose the byte by hand with `& 0xFF` every single time. Here the type is the mask.

The same goes for the right shift, which is really **two** operations wearing one symbol — and in Rust the *type* picks which one you get:

```text
  253u8 >> 1  = 126   unsigned: pad the top with 0
  (-3i8) >> 1 = -2    signed:   pad the top with the sign bit
```

There is no `>>>` in Rust, and there does not need to be. `u8` and `i8` are different types, so the compiler already knows whether the top bit is a sign or a value, and it cannot pick wrong. Choosing the signedness of the variable *is* choosing the shift. Why padding with the sign bit yields −2 rather than something arbitrary is [two's complement](../why_hexadecimal/README.md), which is easiest to *see* in hex: `-1i8` prints as `ff` — every bit set, no minus sign anywhere.

## What is exactly one byte

```text
  u8                     : 1
  bool                   : 1
  Option<bool>           : 1   <- the None hides in an unused bit pattern
  Option<NonZeroU8>      : 1   <- the None hides in the zero
  char                   : 4   <- NOT one byte: a Unicode scalar value
```

`bool` being one byte is the ordinary answer — it needs one bit and gets a byte, because a byte is the smallest addressable thing. The two `Option` rows are the interesting ones: wrapping a type in `Option` normally costs a tag, but a `bool` only uses two of its 256 patterns and a `NonZeroU8` never uses `0`, so the compiler stores `None` *in* the value rather than beside it. That is the [niche optimization](../../17_Option_and_Result/option_as_collection/README.md), and it is the same mechanism that makes [`Option<Box<T>>`](../../17_Option_and_Result/nullable_pointers/README.md) free.

`char` is the trap. In C, `char` **is** the byte, which is why the zine can list "the ASCII character F" as a one-byte thing. In Rust `char` is a Unicode scalar value and occupies **four** bytes, so the two languages use the same word for different objects. When you want the byte, Rust makes you say so with a separate literal syntax:

```rust
b'F'      // u8,   70
'F'       // char, 4 bytes wide, 70 as a scalar value
b"Sw"     // &[u8; 2]
"Sw"      // &str, UTF-8
```

## Everything else is several

```text
  i32 / u64 / f64        : 4 / 8 / 8
  usize (this target)    : 8
  &str  (pointer + len)  : 16
  String (ptr+len+cap)   : 24   <- the text itself is on the heap
```

`&str` is sixteen bytes because it is a *fat pointer* — an address plus a length — and `String` is twenty-four because it adds a capacity. Neither number includes a single character of the text; those live on the heap, and `size_of` never follows a pointer.

Which brings up the byte fact that catches people most often. **A Rust string's length is measured in bytes**, because `String` and `&str` are guaranteed UTF-8 and UTF-8 is a variable-width encoding:

```text
  heart.len()            = 3   bytes
  heart.chars().count()  = 1   char
  heart.as_bytes()       = [226, 157, 164]
  is_char_boundary(1)    = false
```

So `"❤".len()` is 3. Rust then does something unusual with that fact: it refuses to let you pretend otherwise. Indexing a string by a number does not compile at all —

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
  |     let _c = s[0];
  |                ^ string indices are ranges of `usize`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

— and slicing to a byte offset that lands mid-character compiles but panics, with a message that names the character you cut in half:

```text
end byte index 1 is not a char boundary; it is inside '❤' (bytes 0..3 of string)
```

That is not Rust being difficult. A string is bytes; a character is one to four of them; and the two indexings answer different questions. Most languages let you ask the wrong one and hand back mojibake. This one makes you say `.chars()` when you meant characters, and `is_char_boundary` exists so you can check before you slice.

The boundary between the two worlds is `String::from_utf8`, and it hands back a [`Result`](../../17_Option_and_Result/option_vs_result/README.md) rather than a string — because arbitrary bytes are not necessarily text:

```text
  String::from_utf8(0xFF) = Err("invalid utf-8 sequence of 1 bytes from index 0")
```

## The bill for having a width

Here is what you bought when the number stopped being unbounded. Compile the same line two ways and it does two different things:

```text
  plain  255u8 + 1       = panic 'attempt to add with overflow'   (a debug build)
  plain  255u8 + 1       = 0                                      (a release build)
```

That is not a bug and it is not undefined behaviour — it is a [documented, deliberate split](../../15_First_Programs/rustc_without_cargo/README.md): overflow checks are on in debug so you find the bug, and off in release so the arithmetic stays fast, with wrapping as the defined fallback. The consequence is the one to internalise: **an overflow bug can pass every test you run and still wrap in production**, because the two builds do not agree.

Which is why the fix is to stop using bare `+` anywhere the width is genuinely in question, and say what you meant:

```text
  wrapping_add(1)        = 0            "wrap around" — modular arithmetic, on purpose
  checked_add(1)         = None         "raise an error" — the Option makes you handle it
  saturating_add(1)      = 255          "clamp at the ceiling"
  overflowing_add(1)     = (0, true)    the wrapped value AND whether it wrapped
```

Every integer type carries all four, and the names are the whole point: a reader of `saturating_add` knows the ceiling was a decision. There is no such thing as an accidental `wrapping_add`.

## Bytes have no order; numbers made of them do

A single byte has no endianness — there is nothing to order. The question only appears once a value spans two or more bytes, and then it is a real fork with no default answer:

```text
  271u16.to_be_bytes()   = [1, 15]   00000001 00001111
  271u16.to_le_bytes()   = [15, 1]   00001111 00000001
```

Rust makes you name it. `to_be_bytes` / `to_le_bytes` / `from_be_bytes` / `from_le_bytes` are the ones to reach for at any boundary — a file format, a network protocol, a checksum — because they produce the same bytes on every machine. `to_ne_bytes` uses whatever this CPU prefers, which is convenient for in-memory work and a portability bug in a wire format.

And because a pile of bytes carries no type with it, the same eight bytes decode into as many different values as you have decoders. This is `b"computer"`, read nine ways:

```text
  8 x u8       : [99, 111, 109, 112, 117, 116, 101, 114]
  4 x u16 LE   : [28515, 28781, 29813, 29285]
  2 x u32 LE   : [1886220131, 1919251573]
  1 x u64 LE   : 8243122740717776739
  1 x u64 BE   : 7165065861944075634
  1 x f64 LE   : 1.1444935686054472e243
  8 x ASCII    : "computer"
```

None of those readings is more correct than the others. What decides is the type you named, and that is the whole job `u8`, `u16`, `u32` and their friends are doing — not describing the bytes, but declaring how to read them.

## If you are coming from another language

- **Python** — you already know the byte; Python just never lets you hold one. `bytes` and `bytearray` are containers of them, and the tell is that `data[0]` gives you an `int` while `data[0:1]` gives you `bytes` — an asymmetry that exists because there is no one-byte type to return. What does *not* transfer is the width. A Python `int` is unbounded (`255 + 1` is `256`, always), so `~x` is not eight bits wide, there is no unsigned right shift to have, and the `& 0xFF` you sprinkle everywhere is you re-imposing by hand the width Rust puts in the type. Two corollaries worth keeping: `len()` swaps meaning across the two languages — Python counts *characters* and Rust counts *bytes*, so `len("❤")` is 1 and `"❤".len()` is 3 — and Python does overflow after all, the moment you opt into a fixed width somewhere else (`numpy.uint8(255) + 1` is `0`, as are `struct`, `array`, `ctypes` and every integer column in your database). The bugs did not go away; they moved to the edges of the program, where nothing warns you.
- **ABAP** — the closest counterpart is real and often overlooked: type `X` **is** a byte, `XSTRING` is a byte string, and ABAP is the one of the three languages that gives you a *named* single-bit operation with `GET BIT` / `SET BIT` and the `BIT-AND` / `BIT-OR` / `BIT-XOR` operators — sugar Rust does not have, since you write the mask yourself. The split that transfers exactly is `STRING` versus `XSTRING`, which is `String` versus `Vec<u8>`, with the conversion classes standing in for `from_utf8`. Overflow is where the three languages each pick a different one of the zine's three options: ABAP raises (`CX_SY_ARITHMETIC_OVERFLOW` on integer arithmetic), Python grows the number, and Rust makes *you* pick — per operation, in the method name. And where an ABAP program's numeric type is often chosen for the data dictionary's sake, in Rust `u8` versus `u32` is a claim about the range you are prepared to defend, which the compiler will hold you to at the first `+`.

## Practice

**Eight candidates fit in one byte. The ninth is where it gets interesting.**

Build an approval ballot as a single `u8` — seat *n* is bit *n* — with three methods: `approve(seat)`, `is_approved(seat)` and `count()`. Use only bit operations; no `Vec<bool>`, no `HashSet`. Print each ballot in binary beside the names it approves, then tally all eight candidates by reading one bit at a time.

Then let a ninth candidate sign up, and find out what `1u8 << 8` actually does. Answer three things before you run it: what a debug build does, what a release build does, and which of those two is worse. Then widen the type and say what the new ceiling is.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:meet_the_byte_kata -->
*[`meet_the_byte_kata.rs`](examples/meet_the_byte_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: an approval ballot packed into one byte, and the ninth candidate.

use std::hint::black_box;
use std::mem::size_of;

const NAMES: [&str; 8] = ["Ada", "Ben", "Cara", "Dev", "Elif", "Fay", "Gus", "Hal"];

/// Eight approvals in eight bits. Seat `n` is bit `n`, counting from the right.
#[derive(Clone, Copy, Default)]
struct Ballot(u8);

impl Ballot {
    fn approve(self, seat: u32) -> Self {
        Ballot(self.0 | (1 << seat))
    }
    fn is_approved(self, seat: u32) -> bool {
        self.0 & (1 << seat) != 0
    }
    fn count(self) -> u32 {
        self.0.count_ones()
    }
    fn names(self) -> Vec<&'static str> {
        (0..8).filter(|&s| self.is_approved(s)).map(|s| NAMES[s as usize]).collect()
    }
}

fn main() {
    println!("=== three ballots, one byte each ===");
    let ballots = [
        Ballot::default().approve(0).approve(2).approve(5),
        Ballot::default().approve(2).approve(3),
        Ballot::default().approve(0).approve(2).approve(7),
    ];
    for (i, b) in ballots.iter().enumerate() {
        println!(
            "  voter {}: {:08b}  {} approval(s)  {:?}",
            i + 1,
            b.0,
            b.count(),
            b.names()
        );
    }
    println!("  size_of::<Ballot>() = {}   for all eight candidates", size_of::<Ballot>());

    println!("\n=== the tally, read one bit at a time ===");
    for seat in 0..8u32 {
        let votes = ballots.iter().filter(|b| b.is_approved(seat)).count();
        println!("  {:<5} {}{}", NAMES[seat as usize], "#".repeat(votes), if votes == 0 { " -" } else { "" });
    }

    println!("\n=== then a ninth candidate signs up ===");
    let ninth: u32 = black_box(8);
    println!("  1u8.checked_shl({ninth})  = {:?}          <- the honest answer", 1u8.checked_shl(ninth));
    println!("  1u8.wrapping_shl({ninth}) = {}              <- what a RELEASE build silently does", 1u8.wrapping_shl(ninth));
    println!("       ...which is bit 0: Ivy's approval lands on {}", NAMES[0]);

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let plain = std::panic::catch_unwind(|| 1u8 << black_box(8u32));
    std::panic::set_hook(hook);
    match plain {
        Ok(v) => println!("  plain 1u8 << {ninth}       = {v}              (overflow checks OFF)"),
        Err(_) => println!("  plain 1u8 << {ninth}       = panic 'attempt to shift left with overflow'"),
    }

    println!("\n=== the fix is a wider byte-count, and it has its own ceiling ===");
    #[derive(Clone, Copy, Default)]
    struct Wide(u16);
    let ivy = Wide(1 << 8);
    println!("  size_of::<Wide>()     = {}   (two bytes, sixteen seats)", size_of::<Wide>());
    println!("  Ivy at seat 8         = {:016b}", ivy.0);
    println!("  u16 ceiling           = seat {} is the last one that fits", u16::BITS - 1);
    println!("  u128 would buy you    = {} seats, and no more", u128::BITS);
}
```
<!-- /source -->

<!-- output:meet_the_byte_kata -->
*Verified output of [`meet_the_byte_kata.rs`](examples/meet_the_byte_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== three ballots, one byte each ===
  voter 1: 00100101  3 approval(s)  ["Ada", "Cara", "Fay"]
  voter 2: 00001100  2 approval(s)  ["Cara", "Dev"]
  voter 3: 10000101  3 approval(s)  ["Ada", "Cara", "Hal"]
  size_of::<Ballot>() = 1   for all eight candidates

=== the tally, read one bit at a time ===
  Ada   ##
  Ben    -
  Cara  ###
  Dev   #
  Elif   -
  Fay   #
  Gus    -
  Hal   #

=== then a ninth candidate signs up ===
  1u8.checked_shl(8)  = None          <- the honest answer
  1u8.wrapping_shl(8) = 1              <- what a RELEASE build silently does
       ...which is bit 0: Ivy's approval lands on Ada
  plain 1u8 << 8       = panic 'attempt to shift left with overflow'

=== the fix is a wider byte-count, and it has its own ceiling ===
  size_of::<Wide>()     = 2   (two bytes, sixteen seats)
  Ivy at seat 8         = 0000000100000000
  u16 ceiling           = seat 15 is the last one that fits
  u128 would buy you    = 128 seats, and no more
```
<!-- /output -->

The ninth candidate is the point. A debug build panics with `attempt to shift left with overflow`, which is loud and survivable. A release build **masks the shift amount** — for a `u8` it is taken modulo 8 — so `1u8 << 8` is `1u8 << 0`, and Ivy's approval is silently recorded for Ada. That is the worse one by a distance: no panic, no warning, a wrong tally, and a bug that only exists in the build you ship. `checked_shl` returns `None` and is the honest answer whenever the shift amount is not a literal you can see.

</details>

## The verified output

<!-- output:meet_the_byte -->
*Verified output of [`meet_the_byte.rs`](examples/meet_the_byte.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== one byte, read two ways ===
  00000000  =    0  =  0x00
  00000001  =    1  =  0x01
  00000010  =    2  =  0x02
  01011001  =   89  =  0x59
  11111111  =  255  =  0xff
  u8::MAX = 255, u8::BITS = 8
  0b0101_1001 == 89 == 0x59 : true

=== memory is addressed one byte at a time ===
  three bytes            : 01010011 01010101 10110111
  addr(cells[1]) - addr(cells[0]) = 1   <- one, always
  size_of::<[u8; 3]>()   = 3
  the same three as ints : [83, 85, 183]
  the same three as text : 'S' 'U' (183: no ASCII letter)
  str::from_utf8(&cells) : Err("invalid utf-8 sequence of 1 bytes from index 2")

=== you cannot fetch one bit -- you fetch the byte and mask ===
  b                      = 01011001
  read  bit 3            = 1
  test  bit 6            = true
  set   bit 1            = 01011011
  clear bit 3            = 01010001
  flip  all              = 10100110   <- still 8 bits wide
  b.count_ones()         = 4

=== what is exactly one byte ===
  u8                     : 1
  bool                   : 1
  Option<bool>           : 1   <- the None hides in an unused bit pattern
  Option<NonZeroU8>      : 1   <- the None hides in the zero
  char                   : 4   <- NOT one byte: a Unicode scalar value
  b'F' (a byte literal)  : 70 = 01000110
  'F' as u32             : 70

=== what is more than one byte ===
  i32 / u64 / f64        : 4 / 8 / 8
  usize (this target)    : 8
  &str  (pointer + len)  : 16
  String (ptr+len+cap)   : 24   <- the text itself is on the heap

=== a string's length is counted in BYTES ===
  heart.len()            = 3   bytes
  heart.chars().count()  = 1   char
  heart.as_bytes()       = [226, 157, 164]
  as bits                = 11100010 10011101 10100100
  is_char_boundary(1)    = false   <- &heart[0..1] would panic
  String::from_utf8(0xFF)= Err("invalid utf-8 sequence of 1 bytes from index 0")

=== the bill for having a width: overflow ===
  wrapping_add(1)        = 0
  checked_add(1)         = None
  saturating_add(1)      = 255
  overflowing_add(1)     = (0, true)
  plain  255u8 + 1       = panic 'attempt to add with overflow'  (checks ON)
  cfg!(debug_assertions) = true

=== the two right shifts -- the TYPE decides which one you get ===
  253u8 >> 1             = 126   <- unsigned: pad with 0
  (-3i8) >> 1            = -2   <- signed: pad with the sign bit

=== the byte has no order; a MULTI-byte number does ===
  271u16.to_be_bytes()   = [1, 15]   00000001 00001111
  271u16.to_le_bytes()   = [15, 1]   00001111 00000001
  this target is little  = true

=== eight bytes, many meanings ===
  8 x u8       : [99, 111, 109, 112, 117, 116, 101, 114]
  4 x u16 LE   : [28515, 28781, 29813, 29285]
  2 x u32 LE   : [1886220131, 1919251573]
  1 x u64 LE   : 8243122740717776739
  1 x u64 BE   : 7165065861944075634
  1 x f64 LE   : 1.1444935686054472e243
  8 x ASCII    : "computer"
```
<!-- /output -->

## See also

- [What is a ballot, in memory?](../../16_Structs/representing_a_ballot/README.md) — the layer above: once you have the bytes, which container to put them in
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the other half of "pick a type that states the range", at the domain level rather than the machine level
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — what actually happens when the overflow check fires
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — the debug/release split this page leans on, in the build command itself
- [Why hexadecimal](../why_hexadecimal/README.md) — how a byte gets written down: one hex digit is four bits, so the two-digit spelling this page uses throughout is not a convention but a consequence
- [What a float actually stores](../what_a_float_stores/README.md) — the sequel: what the eight bytes of an `f64` are spent on, and why exactness ends at the first division
- [What `i128` is exact about](../../09_Advanced/i128_exactness/README.md) — where "just use a wider integer" does and does not deliver
- Julia Evans, *How Integers and Floats Work* ([wizardzines.com ↗](https://wizardzines.com/)) — the zine this page follows; its "meet the byte" and "8 bytes, many meanings" pages are the source of the `b"computer"` decode, whose integer rows the example above reproduces exactly
