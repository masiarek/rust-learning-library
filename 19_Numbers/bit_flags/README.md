# Bit flags

**Level:** 201 · working knowledge

**One line:** A flag is a one-bit field and a header field is an n-bit flag — the same mask-and-shift either way — and seeing them as one mechanism turns the two bugs that make bit twiddling feel like folklore into things you can predict.

[Meet the byte](../meet_the_byte/README.md) ends on a limitation: the smallest thing in memory with an address is a byte, so you cannot fetch one bit. This page is what people do about that. If a bit is not separately addressable, then several values can live in one integer, and the only question left is which positions mean what.

Almost everyone meets this as folklore — a pile of `<<` and `&` copied out of a man page, adjusted until the number looks right. It is one mechanism, and it has exactly two traps. Both of them are cases where the wrong code *works* on the example you learn it from.

## A flag is a one-bit field

The canonical set is the flags C's `open()` takes. Each is a constant with one bit set, you combine them with `|`, and you test with `&`:

```rust
const O_RDWR:   u32 = 0o2;     //      2  -- see the next section; this one is not a flag
const O_CREAT:  u32 = 0o100;   //     64
const O_TRUNC:  u32 = 0o1000;  //    512
const O_APPEND: u32 = 0o2000;  //   1024

let flags = O_RDWR | O_CREAT | O_TRUNC;
assert!(flags & O_CREAT != 0);
assert!(flags & O_APPEND == 0);
```

```text
  O_RDWR | O_CREAT | O_TRUNC =   578  = 000000001001000010
    O_CREAT      64 = 000000000001000000  set? true
    O_TRUNC     512 = 000000001000000000  set? true
    O_APPEND   1024 = 000000010000000000  set? false
```

Those numbers are **Linux's**. On macOS `O_CREAT` is `0x0200`, not `0o100`. That difference is the whole argument for writing the name rather than the number, and it is why the constants live in a header you did not write. (Here they are hard-coded as data so the example prints the same everywhere — this program is about the mechanism, not about your libc.)

One thing that looks like a shortcut and is not: `flags.count_ones()` is **3**, which is the number of set *bits*, not the number of flags you passed. `O_RDWR` contributed one of those bits without being a flag at all — which is the next section.

## Trap 1: a zero-valued flag cannot be tested with `&`

`O_RDONLY` is `0`. So this test, which reads perfectly well in English, is false for every input that has ever existed:

```text
    read-only   flags & O_RDONLY != 0  ->  false   (wrong for all three)
    write-only  flags & O_RDONLY != 0  ->  false   (wrong for all three)
    read-write  flags & O_RDONLY != 0  ->  false   (wrong for all three)
```

`x & 0` is `0`. There is no value of `flags` that makes it anything else, so the bug has no failing case to find in testing — it is uniformly, silently wrong.

The fix is not a cleverer test. It is noticing that `O_RDONLY` was **never a flag**. The low two bits of that integer are a two-bit *field* holding one of three values, and `O_ACCMODE` (`0o3`) is the mask that isolates it. Once you know that, the right question is not "is this bit set" but "what value is in this field":

```rust
fn access_mode(flags: u32) -> &'static str {
    match flags & O_ACCMODE {
        O_RDONLY => "O_RDONLY",
        O_WRONLY => "O_WRONLY",
        O_RDWR   => "O_RDWR",
        _        => "invalid",
    }
}
```

So a single `u32` handed to `open()` is already carrying both shapes at once: a 2-bit field at the bottom, and a row of 1-bit flags above it. They are not two techniques. A flag is just the case where the field is one bit wide and its two values are named "set" and "unset".

## The same mechanism, wider

Which means an n-bit field needs nothing new. Word 12 of a TCP header packs three of them into 16 bits — a 4-bit data offset, 3 reserved bits, and 9 flags:

```rust
const OFF_SHIFT: u32 = 12;  const OFF_MASK:   u16 = 0xF;      // bits 12..15
const RSV_SHIFT: u32 =  9;  const RSV_MASK:   u16 = 0x7;      // bits  9..11
/*  flags need no shift  */ const FLAGS_MASK: u16 = 0x01FF;   // bits  0..8

fn pack(offset: u16, reserved: u16, flags: u16) -> u16 {
    ((offset & OFF_MASK) << OFF_SHIFT)      // mask, THEN shift
        | ((reserved & RSV_MASK) << RSV_SHIFT)
        | (flags & FLAGS_MASK)
}
```

```text
  pack(offset=8, reserved=0, flags=ACK) = 0x8010 = 1000000000010000
  offset   = (w >> 12) & 0xF    = 8
  reserved = (w >> 9)  & 0x7    = 0
  flags    =  w        & 0x1FF = 000010000
```

Two habits worth fixing in place. **Packing is mask-then-shift** — mask first so a caller passing an out-of-range `offset` cannot spill into the field above it. **Unpacking is shift-then-mask.** And note the masks are written `0x1FF` rather than `511`: a 9-bit mask is precisely where [hexadecimal](../why_hexadecimal/README.md) stops being a preference, because `0x1FF` shows you the bit count and `511` does not.

## Trap 2: the top field forgives a missing mask

```text
  offset   w >> 12          =     8  correct -- nothing lives above it
  reserved w >> 9           =    64  WRONG -- the offset bled in
  reserved (w >> 9) & 0x7    =     0  right
```

The offset is the topmost field, so shifting it down leaves nothing above it and the mask is genuinely redundant. Do the same to a field in the *middle* and everything above it comes along for the ride.

This is why the mistake is so durable: the first field anyone unpacks is usually the top one, the shortcut works, and the habit is formed before it meets a field that punishes it. Always mask on the way out, even where you can see it is unnecessary — the version that is correct by luck and the version that is correct by construction look identical, right up until someone adds a field above yours.

## Why a bare integer is the wrong type to stop there on

Everything above works and it is what C does. The cost is that a `u32` full of file-mode flags and a `u16` full of TCP flags are, to the compiler, just numbers — so `mode | ACK` is arithmetic it will happily perform. A [newtype](../../16_Structs/newtype_score/README.md) is the whole fix:

```rust
use std::ops::BitOr;

#[derive(Clone, Copy, PartialEq)]
struct Mode(u32);

impl BitOr for Mode {
    type Output = Mode;
    fn bitor(self, rhs: Mode) -> Mode { Mode(self.0 | rhs.0) }
}
```

`BitOr` defaults its right-hand side to `Self`, so `Mode(O_RDWR) | ACK` is now `error[E0308]: mismatched types — expected Mode, found u16`. Not a lint, not a convention: the wrong combination stops being expressible.

The second thing a wrapper buys is a `Debug` that prints names instead of a number, which is the difference between reading `Mode(O_RDWR | O_CREAT | O_APPEND)` in a log and reading `578`. And the third is that containment gets written **once**, correctly:

```rust
impl Mode {
    fn contains(self, other: Mode) -> bool { self.0 & other.0 == other.0 }
}
```

`& x == x`, not `& x != 0`. For a single bit the two agree, which is why the wrong one spreads; for a multi-bit mask `!= 0` means *any* of those bits and `== x` means *all* of them, and the output shows them disagreeing.

In real code you would reach for the [`bitflags` ↗](https://docs.rs/bitflags/) crate, which generates exactly this — the newtype, the operators, the named `Debug`, `contains`, `intersects` — from a declaration. It is not used here because every example in this repo compiles with bare `rustc` and no dependencies, and hand-writing it once is arguably the better lesson anyway: the macro is not doing anything you have not now done yourself.

## If you are coming from another language

- **Python** — the stdlib already has the newtype, and it makes you choose the same trade Rust makes for you. `enum.Flag` and `enum.IntFlag` both give you named members that combine with `|` and `repr` as `<M.READ|CREATE: 65>`. The difference is the safety: `Flag` refuses a bare int (`S.READ | 5` raises `TypeError`), while `IntFlag` **is** an int, so `m | 5` quietly returns `<M.READ|CREATE|4: 69>` — it absorbed an undeclared bit and told you nothing. `IntFlag` exists for talking to C, and that is exactly what it costs. The other half that does not transfer is the width: a Python `int` has no top, so `~FLAGS` is not a 32-bit complement and the `1 << n` overflow in this page's kata cannot happen to you — nor can it warn you when your field is full.
- **ABAP** — you have the operations and not the type, which is the mirror image of Rust. `GET BIT n OF x INTO y`, `SET BIT n OF x`, and `BIT-AND` / `BIT-OR` / `BIT-XOR` on `X` fields give you a *named single-bit* access that Rust never offers — no mask arithmetic to get wrong, so Trap 2 is largely not available to you. What is missing is anything stopping two unrelated flag sets from being combined, since they are all just `X` fields. Worth noticing too that the packed form is rare in SAP by choice rather than by ignorance: the house idiom is one `CHAR1` `'X'`/space field per boolean — eight bits per flag instead of one — because storage was never the binding constraint, and a field you can see in SE16 beats one you have to decode. Packed bits show up where a protocol or an external interface dictates the layout, which is precisely the case this page is about.

## Practice

**A tic-tac-toe game in 18 bits — then a fourth field arrives.**

Encode a position in a single `u32`: X's nine cells in bits 0–8, O's nine in bits 9–17, cell *n* being bit *n* in reading order. Write `mark`, `occupied`, `cells(player)`, and a `winner` that tests all eight lines. Make the win check a single comparison per line — if you find yourself counting cells, you have written the `!= 0` version of `contains`.

Then add a 4-bit move counter at bits 18–21 and answer two questions before you run it: which of your two `cells` accessors breaks, and which one cannot break no matter what you add above it.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:bit_flags_kata -->
*[`bit_flags_kata.rs`](examples/bit_flags_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a tic-tac-toe position in 18 bits, plus a move counter above it.
//!
//! bits  0..8  X cells      bits 9..17  O cells      bits 18..21  moves played
//! Cells are numbered in reading order, cell n is bit n.

const CELLS: u32 = 0x1FF; // nine bits
const O_SHIFT: u32 = 9;
const MOVES_SHIFT: u32 = 18;
const MOVES_MASK: u32 = 0xF;

const WINS: [(u32, &str); 8] = [
    (0b000_000_111, "top row"),
    (0b000_111_000, "middle row"),
    (0b111_000_000, "bottom row"),
    (0b001_001_001, "left column"),
    (0b010_010_010, "middle column"),
    (0b100_100_100, "right column"),
    (0b100_010_001, "diagonal \\"),
    (0b001_010_100, "diagonal /"),
];

#[derive(Clone, Copy, Default)]
struct Board(u32);

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Board {
    fn cells(self, p: Player) -> u32 {
        match p {
            Player::X => self.0 & CELLS,
            Player::O => (self.0 >> O_SHIFT) & CELLS, // the mask is load-bearing
        }
    }
    fn occupied(self) -> u32 {
        self.cells(Player::X) | self.cells(Player::O)
    }
    fn moves(self) -> u32 {
        (self.0 >> MOVES_SHIFT) & MOVES_MASK
    }
    fn mark(self, p: Player, cell: u32) -> Board {
        let bit = 1 << cell;
        if self.occupied() & bit != 0 {
            return self; // already taken
        }
        let placed = match p {
            Player::X => self.0 | bit,
            Player::O => self.0 | (bit << O_SHIFT),
        };
        // bump the counter field without disturbing the two below it
        let n = ((placed >> MOVES_SHIFT) & MOVES_MASK) + 1;
        Board((placed & !(MOVES_MASK << MOVES_SHIFT)) | (n << MOVES_SHIFT))
    }
    fn winner(self) -> Option<(Player, &'static str)> {
        for p in [Player::X, Player::O] {
            let mine = self.cells(p);
            for (mask, name) in WINS {
                if mine & mask == mask {
                    return Some((p, name));
                }
            }
        }
        None
    }
    fn render(self) -> String {
        let (x, o) = (self.cells(Player::X), self.cells(Player::O));
        (0..9)
            .map(|c| {
                let ch = if x >> c & 1 == 1 {
                    'X'
                } else if o >> c & 1 == 1 {
                    'O'
                } else {
                    '.'
                };
                if c % 3 == 2 { format!("{ch}\n  ") } else { format!("{ch} ") }
            })
            .collect::<String>()
    }
}

fn main() {
    println!("=== a game, played into one u32 ===");
    let moves = [
        (Player::X, 0u32),
        (Player::O, 1),
        (Player::X, 4),
        (Player::O, 2),
        (Player::X, 8),
    ];
    let mut b = Board::default();
    for (p, cell) in moves {
        b = b.mark(p, cell);
    }
    println!("  {}", b.render().trim_end());
    let raw = format!("{:022b}", b.0);
    println!("  raw            = {raw}");
    println!("  the same bits  = {} {} {}   <- moves | O cells | X cells", &raw[0..4], &raw[4..13], &raw[13..22]);
    println!("  X cells        = {:09b}", b.cells(Player::X));
    println!("  O cells        = {:09b}", b.cells(Player::O));
    println!("  moves played   = {}", b.moves());
    println!("  occupied       = {:09b}  ({} of 9)", b.occupied(), b.occupied().count_ones());

    println!("\n=== the win check is one mask comparison ===");
    let x = b.cells(Player::X);
    for (mask, name) in WINS {
        println!("  x & {mask:09b} == mask  ->  {:<5}  {name}", x & mask == mask);
    }
    match b.winner() {
        Some((p, how)) => println!("  winner: {} on the {how}", if p == Player::X { 'X' } else { 'O' }),
        None => println!("  no winner yet"),
    }

    println!("\n=== the trap, once a third field sits above the other two ===");
    println!("  O cells  (b >> {O_SHIFT}) & 0x1FF = {:09b}   right", (b.0 >> O_SHIFT) & CELLS);
    println!("  O cells   b >> {O_SHIFT}          = {:09b}   WRONG once moves is nonzero",
        (b.0 >> O_SHIFT) & 0x1FFF);
    println!("  ...the move counter is sitting directly on top of O, exactly where");
    println!("     the TCP offset sat on top of reserved. Same bug, different header.");
    println!("  X cells   b & 0x1FF       = {:09b}   no shift needed: X is the bottom field", b.0 & CELLS);
}
```
<!-- /source -->

<!-- output:bit_flags_kata -->
*Verified output of [`bit_flags_kata.rs`](examples/bit_flags_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== a game, played into one u32 ===
  X O O
  . X .
  . . X
  raw            = 0101000000110100010001
  the same bits  = 0101 000000110 100010001   <- moves | O cells | X cells
  X cells        = 100010001
  O cells        = 000000110
  moves played   = 5
  occupied       = 100010111  (5 of 9)

=== the win check is one mask comparison ===
  x & 000000111 == mask  ->  false  top row
  x & 000111000 == mask  ->  false  middle row
  x & 111000000 == mask  ->  false  bottom row
  x & 001001001 == mask  ->  false  left column
  x & 010010010 == mask  ->  false  middle column
  x & 100100100 == mask  ->  false  right column
  x & 100010001 == mask  ->  true   diagonal \
  x & 001010100 == mask  ->  false  diagonal /
  winner: X on the diagonal \

=== the trap, once a third field sits above the other two ===
  O cells  (b >> 9) & 0x1FF = 000000110   right
  O cells   b >> 9          = 101000000110   WRONG once moves is nonzero
  ...the move counter is sitting directly on top of O, exactly where
     the TCP offset sat on top of reserved. Same bug, different header.
  X cells   b & 0x1FF       = 100010001   no shift needed: X is the bottom field
```
<!-- /output -->

The eight win masks make the check one line: `mine & mask == mask` is true exactly when every cell in that line is yours. `!= 0` would report a win for holding *one* square of it.

The counter is Trap 2 arriving on your own data structure. `X` is the bottom field, so `board & 0x1FF` needs no shift and can never be disturbed by anything added above it. `O` is now a middle field, so `board >> 9` picks up the counter — the move count sitting on top of O exactly as the TCP data offset sat on top of reserved. Same bug, different header.

</details>

## The verified output

<!-- output:bit_flags -->
*Verified output of [`bit_flags.rs`](examples/bit_flags.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== a flag is a one-bit field ===
  O_RDWR | O_CREAT | O_TRUNC =   578  = 000000001001000010
    O_CREAT      64 = 000000000001000000  set? true
    O_TRUNC     512 = 000000001000000000  set? true
    O_APPEND   1024 = 000000010000000000  set? false
  flags.count_ones() = 3   <- BITS set, not flags set: one of them is the O_RDWR field

=== TRAP 1: a zero-valued flag cannot be tested with & ===
  O_RDONLY = 0, so `flags & O_RDONLY` is 0 for EVERY flags:
    read-only   flags & O_RDONLY != 0  ->  false   (wrong for all three)
    write-only  flags & O_RDONLY != 0  ->  false   (wrong for all three)
    read-write  flags & O_RDONLY != 0  ->  false   (wrong for all three)
  the low two bits are a FIELD, not three flags. mask it and compare:
    read-only   flags & O_ACCMODE == ...  ->  O_RDONLY
    write-only  flags & O_ACCMODE == ...  ->  O_WRONLY
    read-write  flags & O_ACCMODE == ...  ->  O_RDWR

=== an n-bit field is the same mechanism, wider ===
  pack(offset=8, reserved=0, flags=ACK) = 0x8010 = 1000000000010000
                                           offset|rsv|flags
  offset   = (w >> 12) & 0xF    = 8
  reserved = (w >> 9)  & 0x7    = 0
  flags    =  w        & 0x1FF = 000010000
  ACK set? true   SYN set? false   FIN set? false

=== TRAP 2: the top field forgives a missing mask; a middle field does not ===
  offset   w >> 12          =     8  correct -- nothing lives above it
  reserved w >> 9           =    64  WRONG -- the offset bled in
  reserved (w >> 9) & 0x7    =     0  right
  ...which is how the habit forms: you learn >> on the top field, where it works.

=== the newtype: names in the output, and no cross-type mixing ===
  Mode(O_RDWR | O_CREAT | O_APPEND)
  contains(O_CREAT)  = true
  contains(O_TRUNC)  = false
  bare u16 vs Mode   : `Mode(O_RDWR) | ACK` does not compile (E0308)
  contains() uses `& x == x`, not `!= 0` -- so a multi-bit mask means ALL of it:
    Mode(O_CREAT).contains(both)      = false
    Mode(O_CREAT|O_TRUNC).contains(both) = true
```
<!-- /output -->

## See also

- [Meet the byte](../meet_the_byte/README.md) — the rung below, and where the "you cannot fetch one bit" limitation this page routes around is established; its kata is the one-bit-per-candidate version of the packing here
- [Why hexadecimal](../why_hexadecimal/README.md) — why every mask on this page is written `0x1FF` and not `511`
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the newtype pattern `Mode` uses, at the domain level rather than the bit level
- [What is a ballot, in memory?](../../16_Structs/representing_a_ballot/README.md) — the other direction: when *not* to pack, and which bugs each layout makes writeable
- Julia Evans, *How Integers and Floats Work* ([wizardzines.com ↗](https://wizardzines.com/)) — pages 15 and 16 are the source of the TCP word and the tic-tac-toe encoding

## Po polsku

Flaga to pole jednobitowe — i to jest cała treść tej lekcji. Polskiemu czytelnikowi termin „pole bitowe” (*bit field*) kojarzy się najpierw ze składnią C (`struct { unsigned tryb : 4; }`), a Rust nie ma niczego takiego: maski i przesunięcia pisze się tu ręcznie, przez co pojedyncza flaga i kilkubitowe pole nagłówka okazują się tym samym mechanizmem. Składasz przez `|`, sprawdzasz przez `&`. Jedna rzecz, która wygląda na skrót, a nim nie jest: `count_ones()` zlicza ustawione **bity**, nie flagi — w przykładzie z `open()` daje 3, choć jedna z tych jedynek należy do pola `O_RDWR`, a nie do żadnej flagi.

Pierwsza pułapka to flaga o wartości zero. `O_RDONLY` to `0`, więc warunek `flags & O_RDONLY != 0` jest fałszywy dla każdego możliwego wejścia — `x & 0` to zawsze `0`. Nie da się tego złapać testem, bo ten błąd nie ma przypadku, w którym akurat zawodzi — jest po cichu i jednakowo zły dla każdego wejścia. Lekarstwem nie jest sprytniejszy warunek, tylko spostrzeżenie, że `O_RDONLY` nigdy nie było flagą. Dwa najmłodsze bity to pole o trzech wartościach; izoluje się je maską `O_ACCMODE` i porównuje w `match`u, czyli pytanie brzmi nie „czy ten bit jest ustawiony”, lecz „jaka wartość siedzi w tym polu”.

Kolejność maski i przesunięcia warto zapamiętać jako parę odwrotności:

- **pakowanie: najpierw maska, potem przesunięcie** — `(offset & OFF_MASK) << OFF_SHIFT`, żeby za duża wartość od wywołującego nie wlała się do pola powyżej;
- **rozpakowanie: najpierw przesunięcie, potem maska** — `(w >> RSV_SHIFT) & RSV_MASK`.

Druga pułapka bierze się właśnie z tej drugiej reguły: pole najwyższe wybacza brak maski, środkowe nie. `w >> 12` daje poprawne 8, bo nad polem offsetu nie ma już nic; to samo `w >> 9` daje 64 zamiast 0, bo offset zjechał w dół razem z resztą. Dlatego nawyk jest tak trwały — pierwsze pole, które ktokolwiek rozpakowuje, to zwykle to na górze, skrót tam działa i utrwala się, zanim trafi na pole, które za niego karze. Maskuj zawsze, także tam, gdzie widać, że to zbędne: wersja poprawna przez przypadek i wersja poprawna z konstrukcji wyglądają identycznie do dnia, w którym ktoś doda pole ponad twoim.

Na koniec dwie rzeczy, które w polskich poradnikach o operacjach bitowych zwykle nie padają. Test zawierania pisze się `self.0 & other.0 == other.0`, a nie `& other.0 != 0` — dla jednego bitu obie wersje dają to samo (i dlatego zła się rozprzestrzenia), ale dla maski wielobitowej `!= 0` znaczy „którykolwiek z tych bitów”, a `== x` znaczy „wszystkie”. I gołe `u32` jest złym typem, na którym można poprzestać: dla kompilatora to zwykła liczba, więc `mode | ACK` policzy się bez mrugnięcia okiem. Struktura krotkowa `struct Mode(u32)` z własnym `impl BitOr` sprawia, że błędne złożenie przestaje być wyrażalne — dostajesz `error[E0308]: mismatched types`, a nie ostrzeżenie. W praktyce generuje to za ciebie crate `bitflags`; tutaj piszemy to ręcznie, bo makro nie robi nic ponad to, co właśnie zrobiłeś sam.

**Szukaj po polsku:** operacje bitowe · maska bitowa · pole bitowe · przesunięcie bitowe · `rust bitflags crate`
