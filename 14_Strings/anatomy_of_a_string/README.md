# The anatomy of a `String`

**Level:** 101 → 201 · working knowledge

**One line:** A `String` is three words on the stack — pointer, length, capacity — and the text itself on the heap. `len` is what you have, `capacity` is what you paid for, and the gap between them is why growth is cheap.

```text
let s1 = String::from("Hello");

        stack                          heap
  ┌───────────┬────┐
  │ ptr       │  ●─┼──────────▶ ┌───┬───┬───┬───┬───┐
  │ len       │  5 │            │ H │ e │ l │ l │ o │
  │ capacity  │  5 │            └───┴───┴───┴───┴───┘
  └───────────┴────┘
```

Those three words are the whole fixed-size part — which is why `size_of::<String>()` is 24 on a 64-bit machine no matter how long the text is, and why moving a `String` moves *three words*, never the bytes. A `&str` is the same picture minus `capacity`: a pointer and a length, no ownership, no room to grow.

---

## Born empty, and free

```rust
let mut s = String::new();
println!("{} {}", s.len(), s.capacity());   // 0 0
```

Capacity zero means **no heap allocation has happened at all**. The pointer is a well-known dangling-but-valid placeholder; the first push buys the real buffer. Creating empty `String`s is free, which is why `String::new()` in a struct initializer costs nothing to write.

## Growth runs ahead of you

`push_str` appends bytes. When `len` would pass `capacity`, the `String` allocates a bigger buffer — roughly **double** — and copies the bytes over. Watch the capacity column in the [verified output](#the-verified-output): `0 → 8 → 16 → 32`, while `len` creeps. The exact numbers are the allocator's choice; the shape is the contract. Doubling is what makes a build-by-appending loop cheap: most pushes land in room already bought, and the occasional copy is amortized away.

Two consequences worth owning:

- **Knowing the size beats discovering it.** `String::with_capacity(n)` buys once; every push then lands without a reallocation. `reserve(n)` does the same mid-flight, and `shrink_to_fit()` hands unused room back.
- **Capacity is bookkeeping, not content.** Two `String`s with the same text are equal whatever their capacities; `==`, hashing and printing read `ptr` and `len` only.

## Why the borrow checker cares

Growth may **move the bytes** — the old buffer is freed and the text lives at a new address. Which is exactly why this refuses to compile:

```rust
let mut s = String::from("STAR");
let view = &s[..2];      // a &str into s's buffer
s.push_str(" voting");   // may reallocate — the buffer can move
println!("{view}");
```

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

Without that rule, `view` would point into freed memory the moment a push reallocates. The [borrowing](../../18_Ownership/borrowing/README.md) rules are not string-specific — but a growable buffer is where they earn their keep most visibly.

## A `Vec<u8>` that promises UTF-8

A `String` is not "a `Vec` of characters" — the second tutorial trap this page exists for. It is a `Vec<u8>`: the heap holds **bytes**, and one character may span several of them ([Meet the `char`](../meet_the_char/README.md) takes that on). Everything `Vec` taught you transfers:

| | `Vec<T>` | `String` |
|---|---|---|
| frees its buffer on drop | yes | yes |
| `::new()` · `::with_capacity()` | yes | yes |
| `.len()` · `.capacity()` · `.reserve()` | yes | yes |
| push / pop | one `T` | one `char` (`push_str` for text) |
| a slice of it | `&v[a..b]` is `&[T]` | `&s[a..b]` is `&str` |
| what the bytes must be | anything | **valid UTF-8, always** |

The last row is the entire difference, and it is enforced at the door:

```rust
String::from_utf8(vec![83, 84, 65, 82])   // Ok("STAR")
String::from_utf8(vec![83, 84, 0xFF])     // Err: invalid utf-8 sequence …
```

Checked once on the way in, never again inside — which is what lets every method on `str` assume the bytes are sound.

## If you are coming from another language

**Python.** You already know this growth pattern — from `list`, not `str`.

| Python | | Rust |
|---|---|---|
| `list.append` | amortized O(1) by over-allocating | `push_str`, same strategy |
| `s += "x"` in a loop | builds a **new** `str` each time — text is immutable | appends in place, occasionally reallocating |
| `sys.getsizeof(lst)` jumps in steps | the over-allocation, visible | `.capacity()`, queryable directly |

What changes: Python hides the buffer entirely (there is no `capacity()` on a list, and strings refuse to grow at all); Rust puts the bookkeeping on the type and lets you pre-pay with `with_capacity`.

**ABAP.** An internal table is the growable buffer you already manage by hand.

| ABAP | | Rust |
|---|---|---|
| `DATA itab TYPE TABLE OF … INITIAL SIZE n` | pre-allocating for a known load | `String::with_capacity(n)` |
| `APPEND` growing the table in blocks | the kernel over-allocates for you | doubling growth |
| `lv = lv && lv_more` | concatenation builds new text | `push_str` appends in place |

What changes: ABAP's allocation strategy is the kernel's business and invisible; Rust's is one method call away (`capacity()`), and the compiler — not a dump — is what stops a stale reference into a moved buffer.

---

## Practice

**Predict, then verify.** Write down a table of `len` and `capacity` after each step: `String::new()`, then `push_str` of `"STAR"`, `" voting"`, `" is score"`, `" + runoff"`. Which pushes reallocate?

Then build the same text a second time with `String::with_capacity`, sized so that **no** push reallocates, and prove the two builds are `==` even though their capacities differ.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:anatomy_of_a_string_kata -->
*[`anatomy_of_a_string_kata.rs`](examples/anatomy_of_a_string_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: predict len and capacity through five operations,
//! then make one allocation serve all of them.
//!
//!   rustc --edition 2024 anatomy_of_a_string_kata.rs -o /tmp/anatk && /tmp/anatk

fn row(step: &str, s: &String, note: &str) {
    println!("   {step:<26} len {:>2}  capacity {:>2}   {note}", s.len(), s.capacity());
}

fn main() {
    println!("Round 1 — the naive build, watching the buffer move");
    let mut s = String::new();
    row("String::new()", &s, "no heap buffer at all");
    s.push_str("STAR");
    row("push_str(\"STAR\")", &s, "first allocation");
    s.push_str(" voting");
    row("push_str(\" voting\")", &s, "outgrew it — reallocated, bytes moved");
    s.push_str(" is score");
    row("push_str(\" is score\")", &s, "outgrew it again");
    s.push_str(" + runoff");
    row("push_str(\" + runoff\")", &s, "fits — no reallocation this time");

    println!("\nRound 2 — the same text, one allocation, because we knew the size");
    let mut planned = String::with_capacity(29);
    row("with_capacity(29)", &planned, "bought once, up front");
    for piece in ["STAR", " voting", " is score", " + runoff"] {
        planned.push_str(piece);
    }
    row("four push_str calls", &planned, "capacity never changed");

    println!("\nRound 3 — the two builds are equal; capacity is not content");
    println!("   s == planned?  {}   (capacities {} vs {})",
        s == planned, s.capacity(), planned.capacity());

    println!("\nThe rule to carry away:");
    println!("   len is the text, capacity is the room. Growth doubles so that");
    println!("   repeated pushes stay cheap; with_capacity skips the moves when");
    println!("   you can name the size; shrink_to_fit hands the spare room back.");
}
```
<!-- /source -->

<!-- output:anatomy_of_a_string_kata -->
*Verified output of [`anatomy_of_a_string_kata.rs`](examples/anatomy_of_a_string_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Round 1 — the naive build, watching the buffer move
   String::new()              len  0  capacity  0   no heap buffer at all
   push_str("STAR")           len  4  capacity  8   first allocation
   push_str(" voting")        len 11  capacity 16   outgrew it — reallocated, bytes moved
   push_str(" is score")      len 20  capacity 32   outgrew it again
   push_str(" + runoff")      len 29  capacity 32   fits — no reallocation this time

Round 2 — the same text, one allocation, because we knew the size
   with_capacity(29)          len  0  capacity 29   bought once, up front
   four push_str calls        len 29  capacity 29   capacity never changed

Round 3 — the two builds are equal; capacity is not content
   s == planned?  true   (capacities 32 vs 29)

The rule to carry away:
   len is the text, capacity is the room. Growth doubles so that
   repeated pushes stay cheap; with_capacity skips the moves when
   you can name the size; shrink_to_fit hands the spare room back.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:anatomy_of_a_string -->
*Verified output of [`anatomy_of_a_string.rs`](examples/anatomy_of_a_string.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. String::new() allocates nothing
   String::new()                  len  0   capacity  0

2. The first push buys a buffer; growth roughly doubles
   push_str("aa")                 len  2   capacity  8
   push_str("bbbbbbb")            len  9   capacity 16
   push_str("cccccccccc")         len 19   capacity 32
   (the exact numbers are the allocator's choice — the SHAPE is the
    lesson: capacity jumps ahead of len, so most pushes cost nothing)

3. Paying up front: with_capacity
   String::with_capacity(32)      len  0   capacity 32
   push_str("scores: 5, 4, 3")    len 15   capacity 32
   push_str(", 2, 1, 0")          len 24   capacity 32
   two pushes, zero reallocations — the buffer never moved

4. Giving it back: shrink_to_fit
   shrink_to_fit()                len 24   capacity 24

5. Capacity is bookkeeping, not content
   "hi" with capacity  2 == "hi" with capacity 64?  true

6. A String is a Vec<u8> that promises UTF-8
   from_utf8([83, 84, 65, 82]) = Ok("STAR")
   from_utf8([83, 84, 0xFF])   = Err: invalid utf-8 sequence of 1 bytes from index 2
   the promise is checked at the door, so it never needs checking inside
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/anatomy_of_a_string/examples/anatomy_of_a_string.rs -o /tmp/anat && /tmp/anat
```

## See also

- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the capacity ladder above, watched from the allocator's side: `0 → 8 → 16 → 32` is one allocation and two reallocations, and you can count them
- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [`String` vs `&str`](../string_vs_str/README.md) — who owns, who looks
- [Meet the `char`](../meet_the_char/README.md) — what those heap bytes encode
- [Borrowing](../../18_Ownership/borrowing/README.md) — the rule that stopped `view` above
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — the unit `len` and `capacity` count in
- [What an address shows](../../18_Ownership/what_an_address_shows/README.md) — which of these two places `&s` actually points at, and what a move does to each
- [100 Exercises — String slices ↗](https://rust-exercises.com/100-exercises/04_traits/06_str_slice) — the same three words drawn as a diagram, and what a `&str` into the middle of them carries instead of a capacity
- [std docs — `String` ↗](https://doc.rust-lang.org/std/string/struct.String.html) · [`std::alloc` ↗](https://doc.rust-lang.org/std/alloc/index.html), where the buffer actually comes from

## Po polsku

`String` to trzy słowa maszynowe na stosie — wskaźnik, długość (`len`) i pojemność (`capacity`) — a sam tekst leży na stercie. Dlatego `size_of::<String>()` wynosi 24 bajty niezależnie od tego, czy trzymasz w nim jedno słowo, czy całą książkę, i dlatego przeniesienie własności (*move*) kopiuje **trzy słowa, nigdy bajtów tekstu**. `&str` to ten sam obrazek bez pojemności: wskaźnik i długość, bez prawa do rozbudowy i bez obowiązku zwolnienia pamięci.

Jeśli miałeś na studiach `std::vector` z C++, to znasz już połowę tej strony pod polskimi nazwami: `size()`/`capacity()`, `reserve()`, `shrink_to_fit()` i **koszt zamortyzowany** przy podwajaniu tablicy dynamicznej. Rust nazywa to tak samo, tylko po angielsku. Jedna rzecz jest tu jednak inna dla polskiego tekstu: **pojemność liczy się w bajtach, nie w literach**. „zażółć gęślą jaźń” ma siedemnaście znaków, ale zajmuje **26 bajtów** — dziewięć liter z ogonkiem lub kreską liczy się po dwa — więc `String::with_capacity(17)` i tak dokupi bufor po drodze. Rezerwując miejsce z góry, licz bajty gotowego napisu, a nie litery na palcach.

Z tego samego C++ pochodzi też najlepsza intuicja do błędu `E0502`: **unieważnienie iteratorów przy realokacji**. `push_str` może przenieść bufor pod nowy adres i zwolnić stary, a wycinek łańcucha zrobiony wcześniej wskazywałby wtedy w zwolnioną pamięć. W C++ to jest niezdefiniowane zachowanie, które trzeba pamiętać; w Ruscie pożyczanie zamienia to w komunikat kompilatora — *cannot borrow `s` as mutable because it is also borrowed as immutable* — i program po prostu się nie buduje.

Ostatnia pułapka jest nazewnicza: `String` **nie jest** „wektorem znaków”, tylko `Vec<u8>` z obietnicą poprawnego UTF-8. Obietnica jest sprawdzana raz, przy wejściu (`String::from_utf8` zwraca `Err` dla śmieci), i właśnie dlatego żadna metoda `str` nie musi jej sprawdzać ponownie w środku.

**Szukaj po polsku:** pojemność a długość · tablica dynamiczna koszt zamortyzowany · unieważnienie iteratorów przy realokacji · `rust String capacity vs len` · `rust with_capacity`
