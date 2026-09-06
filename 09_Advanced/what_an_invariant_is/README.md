# What an invariant is

**Level:** 301 · deep dive

**One line:** A property every value of a type has — established at the one door that can build it, assumed by everything downstream — which is why `.chars()` validates nothing, and why a `// SAFETY:` comment is a proof rather than a note.

```rust
fn main() {
    let bytes = [0xE2, 0x9D, 0xA4];           // UTF-8 for U+2764
    let s = str::from_utf8(&bytes).unwrap();  // the only check, and it happens here
    println!("{}", s.chars().count());        // 1
}
```

The loop calls no validator. Nothing in `chars()` re-reads those bytes to ask whether they are still UTF-8, and there is no `Result` anywhere in its signature to say what would happen if they were not. That is an invariant being spent.

An **invariant** is a property true of *every* value of a type, at all times: established by every door that can construct one, preserved by every operation that touches it, and therefore assumable by any code that receives one **without re-checking**. The last clause is the whole payoff. An invariant converts a check you would otherwise pay for on every use into one you pay for once, at the boundary.

## std writes this one down as a heading

The [`str` primitive ↗](https://doc.rust-lang.org/std/primitive.str.html#invariant) docs carry a section called, literally, `# Invariant`:

> Rust libraries may assume that string slices are always valid UTF-8.

Followed by the sentence that says what kind of promise it is:

> Constructing a non-UTF-8 string slice is not immediate undefined behavior, but any function called on a string slice may assume that it is valid UTF-8, which means that a non-UTF-8 string slice can lead to undefined behavior down the road.

So every way of getting a `&str` either proves the property or hands you the bill for it:

| Door | Run-time cost | Who proves it |
|---|---|---|
| [`str::from_utf8`](../../14_Strings/str_methods/str_from_utf8/README.md) | one linear scan | std proves it, you handle the `Err` |
| [`String::from_utf8`](../../14_Strings/string_methods/string_from_utf8/README.md) | one linear scan | the same, for owned bytes |
| a `"…"` literal | none | `rustc`, when it read your source file |
| [`str::from_utf8_unchecked`](../../14_Strings/str_methods/str_from_utf8_unchecked/README.md) | none | **you**, and nothing checks your work |

The list is longer than four — `str::from_utf8_mut`, `String::from_utf8_unchecked` and the rest — but every entry on it is one of those two shapes: it checks, or it is `unsafe` and says whose job the checking was. That is what makes the assumption safe to make everywhere else.

## Spending it: `Chars::next` checks nothing

This is the whole of `next` for the iterator `.chars()` returns, from `library/core/src/str/iter.rs` in Rust 1.98.0:

```rust
fn next(&mut self) -> Option<char> {
    // SAFETY: `str` invariant says `self.iter` is a valid UTF-8 string and
    // the resulting `ch` is a valid Unicode Scalar Value.
    unsafe { next_code_point(&mut self.iter).map(|ch| char::from_u32_unchecked(ch)) }
}
```

Two `unsafe` calls, no validation of its own, and a comment where the validation would have been. The comment is not documentation — it is the argument that this is sound, and it has one clause per call:

| The `unsafe` call | What it requires of the caller | Discharged by |
|---|---|---|
| `next_code_point` | *"`bytes` must produce a valid UTF-8-like (UTF-8 or WTF-8) string"* | the `str` invariant — the bytes came from a `str` |
| [`char::from_u32_unchecked` ↗](https://doc.rust-lang.org/std/primitive.char.html#method.from_u32_unchecked) | the `u32` must be a Unicode scalar value | well-formed UTF-8 cannot encode anything else |

`next_code_point` is an internal helper, and its `# Safety` section is the sentence quoted above. Neither obligation is checked by the compiler; both are *paid* by an invariant established somewhere else entirely, at a door this function never sees.

## The two clauses are not the same kind of hazard

They read alike and they are not. A `char` outside the scalar ranges is on the [Nomicon's list of immediately invalid values ↗](https://doc.rust-lang.org/nomicon/what-unsafe-does.html) — *"a `char` outside the ranges [0x0, 0xD7FF] and [0xE000, 0x10FFFF]"* — so merely producing one is undefined behaviour. A `&str` over bad bytes is the delayed kind std described above: not UB on the spot, but UB as soon as anything reads it.

The second clause follows from the first, and the reason is worth seeing rather than taking on trust. U+D800 is a surrogate, so it is not a `char` — and the byte sequence that would encode it is exactly the one the validator refuses:

```text
char::from_u32(0xD7FF) = Some('\u{d7ff}')
char::from_u32(0xD800) = None                    <- the gap
char::from_u32(0xE000) = Some('\u{e000}')
str::from_utf8([ED, A0, 80]).is_err() = true     <- U+D800's would-be encoding
```

The UTF-8 rules rule out every sequence that could decode into that gap. So a decoder walking a *valid* `str` cannot reach `from_u32_unchecked` with a non-`char` — which is why one invariant, checked once, at a different door, is enough for both lines.

## What the invariant costs

`&h[1..2]` on `"héllo"` panics. Not because the index is out of range — the string is six bytes — but because byte 2 is the middle of `é`, and the two-byte answer would be a `&str` that is not valid UTF-8.

```rust
fn main() {
    let h = "héllo";
    println!("{}", h.is_char_boundary(1));  // true
    println!("{}", h.is_char_boundary(2));  // false — &h[1..2] panics here
}
```

std would rather abort your program than hand back a value that breaks the promise. That trade is what an invariant *is*: the guarantee outranks the answer, every time, or downstream code could not have skipped its checks.

## Most of your own invariants are not about safety

`str`'s is unusual in that breaking it leads to undefined behaviour. The ordinary kind just produces wrong answers, and it is the kind you will write:

```rust
mod ascii_text {
    pub struct AsciiText(String);   // INVARIANT: every byte is < 0x80

    impl AsciiText {
        pub fn new(s: String) -> Option<AsciiText> {
            if s.is_ascii() { Some(AsciiText(s)) } else { None }
        }

        /// O(1) — one byte is one char here, which `new` is the reason for.
        pub fn nth(&self, i: usize) -> Option<char> {
            self.0.as_bytes().get(i).copied().map(char::from)
        }
    }
}
```

`nth` indexes bytes directly and is right to. On a general `&str` the same operation is `chars().nth(i)`, which walks from the start, because a `&str` promises UTF-8 and nothing more. The narrower promise buys the faster method — and it needs no `unsafe` at all, which is the part worth carrying away. `unsafe` is where an invariant gets *spent* in std; it is not where invariants come from. [A score is not a number](../../16_Structs/newtype_score/README.md) is this same design at 101, and [`NonZeroU8` ↗](https://doc.rust-lang.org/std/num/struct.NonZeroU8.html) is std's.

## The trap: an invariant is a property of a module, not of a constructor

Nothing about `new` stops a second function in the same module from building an `AsciiText` out of anything at all. Privacy is per module, so the audit unit is the file, not the call sites — [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) works that door from the other side, and [What `unsafe` turns off](../what_unsafe_turns_off/README.md) shows the version where the leak costs undefined behaviour rather than a wrong answer.

Two habits follow. Keep a module that maintains an invariant small enough to read in one sitting, since that module *is* the review. And write the invariant down where the field is declared — `// INVARIANT: every byte is < 0x80` — so that the person adding a method has something to notice.

## If you are coming from another language

- **Python.** An invariant is what you establish in `__init__` and then stop re-checking. `def __init__(self, num, den): if den == 0: raise ValueError(den); g = math.gcd(num, den); self.num, self.den = num // g, den // g` — every method afterwards may assume lowest terms, and none of them re-reduces. What Python cannot do is *enforce* it: `f.den = 0` from outside is legal, `_private` is a naming convention the interpreter does not police, and even `__slots__` or a `@property` only narrows the opening. So the promise degrades into a docstring plus a habit, and the bug surfaces three methods away from the assignment that caused it. Rust closes exactly that gap and nothing else — the field is private *to the module*, the validating constructor is the only door, and the check happens where the value is born instead of where it is used. You already rely on the string version daily without calling it an invariant: `bytes.decode("utf-8")` is `str::from_utf8`, it raises `UnicodeDecodeError` at the boundary for the same reason, and afterwards `for ch in s` never re-validates. The one thing that does *not* transfer is the escape hatch — Python's equivalent of `from_utf8_unchecked` is `errors="surrogateescape"`, which produces a `str` that most of the language then handles, whereas Rust's produces undefined behaviour. And `assert` is the closest thing you have to `debug_assert!`, including that `-O` removes it.
- **ABAP.** You have been writing invariants for years under a different name: a `PRIVATE SECTION` attribute plus a `CONSTRUCTOR` that raises, and every method below it trusting the range. `CLASS lcl_percent DEFINITION. PUBLIC SECTION. METHODS constructor IMPORTING iv_value TYPE i RAISING cx_sy_range_error. PRIVATE SECTION. DATA mv_value TYPE i. " 0..100` — once the constructor rejects 101, no method re-tests it, and `READ-ONLY` on a public attribute is the same idea with the door left ajar for reading only. The database layer gives you a second, stronger version: a **DDIC domain with fixed values, or a check table**, is an invariant enforced outside your program entirely, which is why nobody writes a range check after `SELECT SINGLE` on a field with a value table behind it. Two things change in Rust. The privacy is per **module**, not per class, so the audit list is every routine in the file rather than every method on the type — closer to ABAP's `FRIENDS` than to `PRIVATE`. And there is no `unsafe`-shaped hole in ABAP to compare with: the nearest is `ASSIGN … CASTING` on a field symbol, where a wrong assumption about the source field corrupts memory rather than raising, and the discipline good ABAP developers already apply there — check `sy-subrc`, keep the block tiny, comment what you assumed — is precisely what a `// SAFETY:` comment is for. The transfer to make is the habit of writing the assumption beside the `DATA` declaration rather than in the method that depends on it.
- **C.** Every invariant is this kind, since nothing is enforced anywhere: a struct with a `len` field that must match its buffer, a string that must be NUL-terminated, a `FILE*` that must be open. The difference is not the concept but *where the reader finds it* — in C it is a comment in a header if you are lucky, and in Rust the constructor's signature returns `Option` or `Result` and says so in the type. `strlen` on a non-terminated buffer is `str::from_utf8_unchecked`'s failure mode exactly, including that it is undefined behaviour rather than a wrong length.
- **Java / C#.** A final field set by a validating constructor is the same shape, and the same limit applies as in Python: reflection re-opens it, and `equals`/`clone`/deserialization are three back doors that skip the constructor entirely. Deserialization is the closest analogue in either language to `from_utf8_unchecked` — a value materialised without the door that was supposed to prove it, which is why `readObject` is where invariant bugs live.

---

## The verified output

<!-- output:what_an_invariant_is -->
*Verified output of [`what_an_invariant_is.rs`](examples/what_an_invariant_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. An invariant is a promise, and a door that will not let it through
   str::from_utf8([E2, 9D, A4])  = Ok("❤")
   str::from_utf8([68, 69, FF])  = Err(valid_up_to = 2)
   std states it as a heading, `# Invariant`, on the `str` primitive:
   "Rust libraries may assume that string slices are always valid UTF-8."

2. Spending it: `Chars::next` validates nothing at all
   "héllo ❤" is 10 bytes, 7 chars
   chars().next() is Option<char>, not Option<Result<char, _>> -> 'h'
   The missing error case IS the invariant, showing up in the type.

3. Why the SAFETY comment's second clause follows from its first
   char::from_u32(0xD7FF) = Some('\u{d7ff}')
   char::from_u32(0xD800) = None   <- a surrogate is not a scalar value
   char::from_u32(0xE000) = Some('\u{e000}')
   str::from_utf8([ED, A0, 80]).is_err() = true
   The validator refuses the only bytes that could decode into
   that gap, so `from_u32_unchecked` can never be reached with one.

4. What the invariant costs: std panics rather than break it
   "héllo": is_char_boundary(1) = true
   "héllo": is_char_boundary(2) = false   <- &h[1..2] panics here
   Handing back half of 'é' would be a &str that is not UTF-8,
   so the slice aborts instead. The promise outranks the answer.

5. Your own invariant needs no `unsafe` to pay off
   AsciiText::new("status: 404") accepted
   t.nth(8) = Some('4')   <- a byte index, O(1)
   AsciiText::new("héllo").is_none() = true
   `nth` re-checks nothing. `new` paid once, for every call after it.
```
<!-- /output -->

## Practice

**The invariant nobody wrote down.** Build a `Tally` with two private fields — a `Vec<u32>` of counts and a running `total` — and one method that maintains both. Say in a comment what relates them, then write the O(1) `total()` that is only correct because of it.

Now add an ordinary `pub fn` in the same module that pushes a count and forgets the total. No `unsafe`, no panic, no compiler complaint. Call it, and print the cached answer beside the recomputed one. Then say which lines are on the suspect list, give three fixes in the order you would prefer them, and turn the comment into something a test can fail on.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_an_invariant_is_kata -->
*[`what_an_invariant_is_kata.rs`](examples/what_an_invariant_is_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the invariant nobody wrote down, and the ordinary `pub fn`
//! that broke it.
//!
//!   rustc --edition 2024 what_an_invariant_is_kata.rs -o /tmp/ik && /tmp/ik

mod tally {
    /// Word counts, plus the running total nobody wants to recompute.
    pub struct Tally {
        counts: Vec<u32>,
        total: u32, // INVARIANT: total == counts.iter().sum()
    }

    impl Tally {
        pub fn new() -> Tally {
            Tally { counts: Vec::new(), total: 0 }
        }

        /// Maintains the invariant, so it is safe to call from anywhere.
        pub fn record(&mut self, n: u32) {
            self.counts.push(n);
            self.total += n;
        }

        /// O(1): it reads the cache instead of the data. Correct only while
        /// the invariant holds.
        pub fn total(&self) -> u32 {
            self.total
        }

        /// The leak. `pub`, ordinary, no `unsafe` anywhere — and it forgets
        /// the second half of what `record` does.
        pub fn record_unchecked(&mut self, n: u32) {
            self.counts.push(n);
        }

        /// The fix, if the fast path really is needed: restore the invariant
        /// before returning.
        pub fn resync(&mut self) {
            self.total = self.counts.iter().sum();
        }

        /// The invariant, written as code so a test can ask.
        pub fn holds(&self) -> bool {
            self.total == self.counts.iter().sum::<u32>()
        }

        pub fn counts(&self) -> &[u32] {
            &self.counts
        }
    }
}

use tally::Tally;

fn main() {
    let mut t = Tally::new();
    for n in [12, 30, 8] {
        t.record(n);
    }
    println!("1. While every door maintains it");
    println!("   counts = {:?}", t.counts());
    println!("   total() = {}   holds() = {}", t.total(), t.holds());
    println!();

    println!("2. One ordinary `pub fn` later");
    t.record_unchecked(100);
    println!("   counts = {:?}", t.counts());
    println!("   total() = {}   holds() = {}", t.total(), t.holds());
    println!("   The number is wrong by exactly the value that skipped the cache.");
    println!("   No `unsafe`, no panic, no error -- a quietly wrong answer.");
    println!();

    println!("3. Where the audit has to look");
    println!("   `total` is private, so only this module can desync it.");
    println!("   The suspect list is every fn in `mod tally`, not every caller.");
    println!();

    println!("4. Three fixes, in the order to prefer them");
    println!("   a. delete `record_unchecked` -- the invariant has one door again");
    println!("   b. make it private, so the leak cannot escape the module");
    println!("   c. keep it and call `resync` -- last resort, because it is");
    println!("      now a rule a reader has to remember rather than one the");
    println!("      type enforces");
    t.resync();
    println!("   after resync: total() = {}   holds() = {}", t.total(), t.holds());
    println!();

    println!("5. Make it checkable, not just documented");
    debug_assert!(t.holds(), "Tally invariant: total == sum(counts)");
    println!("   debug_assert!(t.holds()) costs nothing in release and turns");
    println!("   the comment into something a test run can fail on.");
}
```
<!-- /source -->

<!-- output:what_an_invariant_is_kata -->
*Verified output of [`what_an_invariant_is_kata.rs`](examples/what_an_invariant_is_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. While every door maintains it
   counts = [12, 30, 8]
   total() = 50   holds() = true

2. One ordinary `pub fn` later
   counts = [12, 30, 8, 100]
   total() = 50   holds() = false
   The number is wrong by exactly the value that skipped the cache.
   No `unsafe`, no panic, no error -- a quietly wrong answer.

3. Where the audit has to look
   `total` is private, so only this module can desync it.
   The suspect list is every fn in `mod tally`, not every caller.

4. Three fixes, in the order to prefer them
   a. delete `record_unchecked` -- the invariant has one door again
   b. make it private, so the leak cannot escape the module
   c. keep it and call `resync` -- last resort, because it is
      now a rule a reader has to remember rather than one the
      type enforces
   after resync: total() = 150   holds() = true

5. Make it checkable, not just documented
   debug_assert!(t.holds()) costs nothing in release and turns
   the comment into something a test run can fail on.
```
<!-- /output -->

</details>

## See also

- [What `unsafe` turns off](../what_unsafe_turns_off/README.md) — the five powers the keyword grants, and why the unit of review is the module
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the same design at 101, without any of the undefined behaviour
- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — the privacy that makes a single door possible, and the back door that undoes it
- [`str::from_utf8_unchecked`](../../14_Strings/str_methods/str_from_utf8_unchecked/README.md) — the door that hands you the bill, and the only honest reason to use it
- [Meet the `char`](../../14_Strings/meet_the_char/README.md) — why a surrogate is not one
- [Anatomy of a `String`](../../14_Strings/anatomy_of_a_string/README.md) — the bytes the invariant is about

## Sources

The [`str` primitive ↗](https://doc.rust-lang.org/std/primitive.str.html#invariant) docs for the invariant itself, and [`Chars` ↗](https://doc.rust-lang.org/std/str/struct.Chars.html) for the iterator whose `next` is quoted above — its source is one click away under `source` on that page. The Nomicon's [what unsafe can do ↗](https://doc.rust-lang.org/nomicon/what-unsafe-does.html) for the list of values that are invalid on production rather than on use.

## Po polsku

Polskie słowo **niezmiennik** jest tu jednocześnie właściwe i mylące. Właściwe, bo to termin, którego używa polska informatyka od zawsze — „niezmiennik pętli” z logiki Hoare'a to dokładnie ta sama idea: zdanie prawdziwe przed każdym obrotem i po nim. Mylące, bo brzmi jak `niezmienny` — a niezmienne (*immutable*) jest coś zupełnie innego. `AsciiText` z tej strony można modyfikować do woli; niezmienna jest **własność**, nie zawartość. Warto to rozdzielić od razu, bo w Ruscie `let` kontra `let mut` już zajmuje słowo „niezmienny”, i te dwa pojęcia leżą obok siebie na każdej stronie o typach.

Sedno strony w jednym zdaniu: niezmiennik to obietnica, którą **płaci się raz, przy drzwiach**, i którą potem wolno zakładać bez sprawdzania. Dlatego `str::from_utf8` zwraca `Result` (tam odbywa się jedyna kontrola), a `.chars()` nie zwraca już niczego, co mogłoby się nie udać — brak `Result` w sygnaturze **jest** widocznym śladem niezmiennika. I dlatego `&h[1..2]` na `"héllo"` panikuje: zwrócenie połowy litery `é` dałoby `&str`, który nie jest poprawnym UTF-8, więc biblioteka standardowa woli przerwać program niż złamać obietnicę.

Jedno rozróżnienie, które w polskich omówieniach `unsafe` prawie nie pada, a jest tu najważniejsze: **złamanie niezmiennika zwykle daje po prostu zły wynik, a nie niezdefiniowane zachowanie**. Kata na tej stronie jest właśnie o tym — zwykła publiczna metoda rozsynchronizowuje licznik z danymi, nie ma tam ani jednego `unsafe`, nic się nie wywala, a odpowiedź jest cicho błędna. Niezmiennik `str`-a jest rzadkim przypadkiem drugiego rodzaju, i sama dokumentacja rozróżnia dwa stopnie: `char` spoza zakresów [0x0, 0xD7FF] i [0xE000, 0x10FFFF] jest niepoprawny **natychmiast**, a `&str` na złych bajtach dopiero wtedy, gdy ktoś go odczyta.

Praktyczna rada na koniec, ta sama co przy `unsafe`: pisz niezmiennik **przy deklaracji pola**, nie w metodzie, która z niego korzysta — `// INVARIANT: every byte is < 0x80` — bo audytowaną jednostką jest **moduł**, czyli plik, a nie miejsce wywołania. Osoba dopisująca nową metodę dwa tygodnie później przeczyta pole, a nie twoją metodę.

**Szukaj po polsku:** niezmiennik klasy · niezmiennik pętli · logika Hoare'a · `rust type invariant` · `rust safety invariant utf8` · `rust newtype validation`
