# Buffer overflow — the write past the end

**Level:** 201 · for C and C++ programmers

**One line:** The same missing length as [the read next door](../buffer_overruns/README.md), but writing — `strcpy` copies past the end of a buffer it was never told the size of, and because a write past the end is undefined behaviour, what you see (a trap, `*** buffer overflow detected ***`, `*** stack smashing detected ***`, or a segfault) is decided by the mitigation that happens to notice, not by the language.

## The program

```c
#include <stdio.h>
#include <string.h>

static const char *const names[] = { "Ada", "Grace Brewster Murray Hopper" };

int main(int argc, char **argv) {
    (void)argv;
    char buf[8];
    const char *src = (argc >= 2) ? names[0] : names[1];   /* no args -> the 28-byte one */
    strcpy(buf, src);                                       /* writes strlen(src)+1 bytes */
    printf("copied: %s\n", buf);
    return 0;
}
```

`strcpy` copies until the source's `\0`, into `buf`, writing exactly as many bytes as the source is long plus one — twenty-nine of them, into eight. It never looks at how big `buf` is, because it was never told. The source is chosen through `argc` so the compiler cannot fold its length and warn at compile time: run with an argument and the three-byte name fits; run with none and the copy runs off the end.

## What it did

The same program, built four ways, dies four different ways — and none of the deaths is the program's "right" output, because a program with no defined behaviour has none.

```text title="Real runs — Apple clang 21.0.0, x86_64 macOS, cc -std=c17 overflow.c"
$ ./overflow x          # a short argument: the copy fits
copied: Ada
$ ./overflow            # no argument: 29 bytes into an 8-byte buffer
(nothing printed; killed by SIGILL, exit 132)
```

```text title="Real runs — Debian, GCC 13.4.0, glibc 2.36, x86-64"
$ cc overflow.c -o overflow                                    # Debian default: canary on, FORTIFY off
$ ./overflow
(nothing printed; killed by SIGSEGV, exit 139)

$ cc -O2 -D_FORTIFY_SOURCE=2 overflow.c -o overflow            # what a distro release build does
$ ./overflow
*** buffer overflow detected ***: terminated

$ cc -O0 -fstack-protector-all -U_FORTIFY_SOURCE overflow.c -o overflow
$ ./overflow
*** stack smashing detected ***: terminated
```

Three mitigations and their absence, and the *when* is the whole lesson:

- **FORTIFY** replaces `strcpy` with `__strcpy_chk`, which the compiler hands the destination's size — eight, a fact it can see — so it checks *before* the write and aborts cleanly with `*** buffer overflow detected ***`. It is the only one of the three that stops the bad write from happening.
- **The stack canary** (`-fstack-protector`) sits a known value between `buf` and the saved return address and re-reads it in the function epilogue. That is *after* the overflow has already happened; the canary can report a smashed frame, not prevent the smashing — `*** stack smashing detected ***`, at `return`.
- **With both off**, nothing checks. Here the corrupted stack faulted on its own (`SIGSEGV`) — but that is luck. In the exploit this page is named after, the overwritten return address points at code the attacker chose, and the program does not crash at all: it obeys.

AddressSanitizer names it for what it is, a write, and does so at the write rather than after:

```text title="Real output — clang -fsanitize=address -g -O0 overflow.c, abridged"
==24835==ERROR: AddressSanitizer: stack-buffer-overflow on address 0x7ff7bd923948
WRITE of size 29 at 0x7ff7bd923948 thread T0
    #0 0x000102b2efe9 in strcpy
    #1 0x0001025db8de in main overflow.c:15

Address 0x7ff7bd923948 is located in stack of thread T0 at offset 40 in frame
    #0 0x0001025db7ff in main
  This frame has 1 object(s):
    [32, 40) 'buf' (line 13) <== Memory access at offset 40 overflows this variable
```

## Why the standard allows it

An array decays to a bare pointer the moment it is passed anywhere, and the length does not travel with it: `buf[i]` is defined as `*(buf + i)`, and `strcpy` walks the source to its `\0` with no bound on the far side to test against. Forming or writing through a pointer past the end of the array is [undefined behaviour ↗](https://en.cppreference.com/w/c/language/behavior). There is nothing at run time that *knows* `buf` was eight bytes.

That is the only difference from [the read cousin](../buffer_overruns/README.md), and it is a difference of direction, not of kind. A read past the end hands you a wrong value; a write past the end changes memory that belongs to something else — a saved register, the canary, the return address — which is why this is the overflow that decades of exploits are built on and the read is merely a wrong number. And the canary, `__strcpy_chk` and ASan above are not the language noticing. They are extra code a flag inserts around a language that still cannot see the size of its own buffer.

## What Rust does instead

Rust has no `strcpy`. A write into a fixed buffer states an index or a length, the length travels with the slice, and one past the end is a checked panic *at the write* — not a silent overwrite of whatever sat after it:

```rust
let mut buf = [0u8; 8];
buf[8] = b'!';            // panics: index out of bounds
buf.copy_from_slice(src); // panics unless src is exactly 8 long
```

<!-- output:buffer_overflow -->
*Verified output of [`buffer_overflow.rs`](examples/buffer_overflow.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
byte-by-byte copy of 28 bytes into [u8; 8]: panicked at the 9th write, buffer's end intact
buf.copy_from_slice(28 bytes into 8): panicked, nothing written
copied 8 of 28 bytes: "Grace Br"
Vec::extend_from_slice: len 28, holds "Grace Brewster Murray Hopper"
```
<!-- /output -->

Every path refuses in the same direction: the byte-by-byte loop panics at the ninth write with the buffer's end still intact, `copy_from_slice` will not even accept a source of the wrong length, and the two safe forms — copy what fits, or let a `Vec` grow — are the only ones that write anything at all. The bound C asks you to remember is, in Rust, the only thing the API will let you express.

## The refusal

Most out-of-range indexing is a run-time panic, because the length is known only then. With a *constant* index into a fixed-size array, though, `rustc` refuses to build it — the length is part of the type `[u8; 8]`, so the mistake is visible without running anything:

```text title="Real rustc output — constant_write.rs, --edition 2024"
error: this operation will panic at runtime
 --> constant_write.rs:3:5
  |
3 |     buf[8] = b'!';           // constant index, one past the end
  |     ^^^^^^ index out of bounds: the length is 8 but the index is 8
  |
  = note: `#[deny(unconditional_panic)]` on by default
```

This is the mirror of the C page's mitigations, moved to a place none of them can reach: not a check inserted around the write, but a refusal to compile it.

## If you are coming from another language

- **C++** — `std::string` and `std::vector` own their storage and grow, so the destination cannot overrun; `std::array<T, N>::at()` throws where `operator[]` is UB, the same pair of doors as Rust's checked index and `get_unchecked`, with the default the other way round. `std::span` (C++20) is a slice: pointer plus length in one value. But `std::strcpy` and `std::memcpy` are inherited from C whole, and a `std::copy` into a too-small output is UB just the same — which is what [Safe Buffers](../safe_buffers/README.md) is C++ beginning to check.
- **Python** — a `str`, `bytes` or `bytearray` grows as needed and there is no fixed destination to overflow; writing past the end of a `bytearray` by index raises `IndexError` rather than touching memory. The whole family of bugs is absent, and the cost moved to encoding and decoding at the edges.
- **ABAP** — *(Not machine-checked — CI cannot run ABAP.)* Strings grow on their own, with no terminator and no fixed destination to overrun. Fixed-length `c` fields exist and *truncate or pad* rather than overwrite past their end, so the failure there is silent truncation — often at a width measured in the wrong unit, the [fixed-width byte fields ↗](https://masiarek.github.io/encodings-learning-library/07_Real_Data/fixed_width_byte_fields/index.html) problem — not a smashed frame.

## Practice

**The write, four ways, and the one caught without running.** For a `[u8; 4]` and a six-byte source, work out what happens with a runtime-indexed write in a loop, a whole-source `copy_from_slice`, a constant index of `4`, and the checked `buf[..n].copy_from_slice(&src[..n])`. Say which one never runs, and why it can be caught at compile time — then say, for the C version, what "caught" even means when the language itself checks nothing.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:buffer_overflow_kata -->
*[`buffer_overflow_kata.rs`](examples/buffer_overflow_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four ways to write into a buffer too small, and the one the
//! compiler rejects before it runs.
//!
//!   rustc --edition 2024 buffer_overflow_kata.rs -o /tmp/bok && /tmp/bok
use std::panic::{catch_unwind, AssertUnwindSafe, set_hook};

fn main() {
    let src = b"lambda";        // 6 bytes
    set_hook(Box::new(|_| {})); // hush the panic banners; we report outcomes ourselves

    println!("THE C SHAPE");
    println!("  char buf[4]; strcpy(buf, \"lambda\");");
    println!("  Six bytes plus a NUL written into four. The write runs off the");
    println!("  end into whatever follows buf -- a saved register, the canary,");
    println!("  the return address. Nothing in the language stops it; what you");
    println!("  see is decided by what the compiler bolted on (a stack canary, a");
    println!("  FORTIFY _chk call) or, with those off, by luck.");
    println!();

    println!("1. THE RUNTIME INDEX -- a bounds check on every write");
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let mut b = [0u8; 4];
        for (i, &byte) in src.iter().enumerate() {
            b[i] = byte;        // panics at i == 4
        }
    }))
    .is_err();
    println!("  buf[i] = byte for i in 0..6  ->  {}",
             if panicked { "panic at i == 4, nothing past the end is written" } else { "ok" });
    println!();

    println!("2. THE BULK COPY -- lengths must match, so it cannot even start");
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let mut b = [0u8; 4];
        b.copy_from_slice(src); // 6 into 4
    }))
    .is_err();
    println!("  buf.copy_from_slice(6 bytes)  ->  {}",
             if panicked { "panic, buffer untouched" } else { "ok" });
    println!();

    println!("3. THE ONE THE COMPILER CATCHES OUTRIGHT");
    println!("  buf[4] = 0;  with buf: [u8; 4]");
    println!("  A constant index into a fixed-size array: rustc rejects it at");
    println!("  compile time -- \"this operation will panic at runtime\", lint");
    println!("  unconditional_panic. No run needed; the length is in the type.");
    println!();

    println!("4. THE CHECKED WRITE -- copy what fits, and know how much");
    let mut buf = [0u8; 4];
    let n = src.len().min(buf.len());
    buf[..n].copy_from_slice(&src[..n]);
    println!("  copied {n} of {}: {:?}", src.len(), std::str::from_utf8(&buf[..n]).unwrap());
    println!("  The bound C asks you to remember is the only thing the API lets");
    println!("  you express -- the length is not optional here.");

    assert_eq!(&buf, b"lamb");
}
```
<!-- /source -->

<!-- output:buffer_overflow_kata -->
*Verified output of [`buffer_overflow_kata.rs`](examples/buffer_overflow_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
THE C SHAPE
  char buf[4]; strcpy(buf, "lambda");
  Six bytes plus a NUL written into four. The write runs off the
  end into whatever follows buf -- a saved register, the canary,
  the return address. Nothing in the language stops it; what you
  see is decided by what the compiler bolted on (a stack canary, a
  FORTIFY _chk call) or, with those off, by luck.

1. THE RUNTIME INDEX -- a bounds check on every write
  buf[i] = byte for i in 0..6  ->  panic at i == 4, nothing past the end is written

2. THE BULK COPY -- lengths must match, so it cannot even start
  buf.copy_from_slice(6 bytes)  ->  panic, buffer untouched

3. THE ONE THE COMPILER CATCHES OUTRIGHT
  buf[4] = 0;  with buf: [u8; 4]
  A constant index into a fixed-size array: rustc rejects it at
  compile time -- "this operation will panic at runtime", lint
  unconditional_panic. No run needed; the length is in the type.

4. THE CHECKED WRITE -- copy what fits, and know how much
  copied 4 of 6: "lamb"
  The bound C asks you to remember is the only thing the API lets
  you express -- the length is not optional here.
```
<!-- /output -->

</details>

## See also

- [Buffer overruns](../buffer_overruns/README.md) — the read cousin: the same absent length, reading instead of writing, a wrong number instead of a smashed frame
- [Safe Buffers](../safe_buffers/README.md) — C++'s own reply to exactly this: a length on every buffer, a check on every index, raw-pointer indexing flagged
- [Signed overflow](../signed_overflow/README.md) — the other undefined behaviour whose *visible* result changes with the build
- [The bugs Rust is a reply to](../README.md) — the chapter, and the nine benefit-mapped bugs this write cousin sits beside
- [The functions that do not check ↗](https://masiarek.github.io/c-learning-library/03_Strings/the_functions_that_do_not_check/index.html) — the C library's runnable tour of `strcpy` and the `strncpy`/`snprintf`/`strlcpy`/`fgets` written to replace it: the family this crash comes from
- [Smashing the Stack for Fun and Profit ↗](http://phrack.org/issues/49/14.html) — Aleph One, Phrack 49 (1996), the write-up that named the exploit the third build above is one flag away from

## Po polsku

To jest ta sama brakująca długość co u [sąsiada obok](../buffer_overruns/README.md), tylko przy *zapisie*. `strcpy` kopiuje aż do `\0` w źródle — dwadzieścia dziewięć bajtów do ośmiobajtowego `buf` — i nigdy nie pyta, jak duży jest cel, bo nikt mu tego nie powiedział. Zapis poza koniec to **zachowanie niezdefiniowane**, więc nie ma jednego „poprawnego” wyjścia: ten sam program zbudowany na cztery sposoby ginie na cztery różne sposoby. FORTIFY podmienia `strcpy` na `__strcpy_chk`, któremu kompilator podaje rozmiar celu (osiem — to widać), więc sprawdza *przed* zapisem i przerywa czysto: `*** buffer overflow detected ***`. Kanarek na stosie (`-fstack-protector`) leży między buforem a adresem powrotu i jest sprawdzany dopiero w epilogu funkcji — czyli *po* nadpisaniu; potrafi zgłosić rozbitą ramkę, nie zapobiec rozbiciu: `*** stack smashing detected ***`. Bez obu nic nie sprawdza i tu akurat program się wywalił (`SIGSEGV`), ale to przypadek — w klasycznym exploicie nadpisany adres powrotu wskazuje na kod wybrany przez atakującego i program wcale się nie wywraca, tylko robi, co mu kazano.

Różnica wobec odczytu poza zakresem jest tylko w kierunku, nie w rodzaju. Odczyt oddaje złą wartość; zapis zmienia pamięć należącą do czegoś innego — zapisanego rejestru, kanarka, adresu powrotu — i dlatego to *na tym* przepełnieniu (właściwie: „przepełnieniu bufora” w potocznym, exploitowym sensie) zbudowano dziesięciolecia ataków, a odczyt to tylko zła liczba. Rust nie ma `strcpy`: zapis do bufora o stałym rozmiarze podaje indeks albo długość, długość jedzie razem z wycinkiem, a jeden za koniec to sprawdzona panika *w miejscu zapisu*, nie ciche nadpisanie. Przy stałym indeksie `rustc` odrzuca to już przy kompilacji lintem `unconditional_panic` — czego żadna z ceowych łatek nie potrafi, bo one otaczają zapis, a nie odmawiają go skompilować.

**Szukaj po polsku:** przepełnienie bufora · zapis poza zakresem · rozbicie stosu · kanarek na stosie · `stack smashing detected` · `rust index out of bounds` · `_FORTIFY_SOURCE` · `-fstack-protector`
