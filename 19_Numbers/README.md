# Numbers and bytes

**One line:** The unit everything else is measured in — what a byte is, how to write one down, what people pack into one, and the one number type that cannot represent the value you typed.

This section is where `.len()` finally means something. A byte is the thing `size_of` counts, hexadecimal is how you read one without counting bits, a flag is a one-bit field, and `f64` is the type Rust withholds `Eq` and `Ord` from — for a reason the float pages measure rather than assert.

| Lesson | Level | What it teaches |
|---|---|---|
| [Meet the byte](meet_the_byte/README.md) | 101 → 201 | `u8` is one byte and the unit `size_of` counts in — plus the three bills a width comes with: overflow that differs by build, the shift the type picks, and a `.len()` measured in bytes |
| [The integer types](the_integer_types/README.md) | 101 → 201 | Twelve widths, `usize` for anything that indexes, `i32` when nothing says otherwise — and the unsigned subtraction that wraps in release — *stub* |
| [Writing a number down](writing_a_number_down/README.md) | 101 → 201 | The four things you can attach to a literal — the base prefix (`0` says *not decimal*, the letter names the base), the underscore, the type suffix, and the `b` that means bytes — plus the range each width promises, and why `b'A'` is not Python's `b'A'` |
| [Why hexadecimal](why_hexadecimal/README.md) | 101 → 201 | Why a byte is two hex digits and always will be — plus the three traps that follow: unpadded `{:x}` losing the byte boundary, `from_str_radix` refusing the `0x` it just printed, and hex of a negative showing two's complement |
| [Printing bytes](printing_bytes/README.md) | 101 → 201 | A byte slice has no `{}`, and `{:?}` prints numbers — `escape_ascii()` prints what Python's `b'…'` shows, keeps every byte, and is valid source for the same bytes; plus the hand-rolled loop that escapes the letters too |
| [Bit flags](bit_flags/README.md) | 201 | Several values in one integer: a flag is a one-bit field and a header field is an n-bit flag — plus the zero-valued flag `&` cannot test, and the missing mask only a middle field punishes |
| [What a float actually stores](what_a_float_stores/README.md) | 201 | The one division that ends exactness — why `0.1` is not 0.1, why the error goes both ways, and why Rust withholds `Eq` and `Ord` from `f64` |
| [Making a float whole](rounding_a_float/README.md) | 101 → 201 | `floor`, `ceil`, `trunc` and `round` pick four different whole numbers once a value is negative or ends in .5, and every one hands back an `f64` — so `9.0` prints as `9`, `-0.4` rounds to `-0`, and the three places std decides a tie (`round`, `round_ties_even`, `{:.N}`) can disagree on the same value |
| [Letting the compiler reorder a float sum](letting_the_compiler_reorder/README.md) | 201 → 301 | Why `a + b + c` is pinned to one grouping, what that costs a hot loop, and the five `algebraic_*` methods Rust 1.98 added to lift the ban one operation at a time |
| [Comparing two numbers of different types](comparing_two_number_types/README.md) | 101 → 201 | `i32 < u16` is `E0308`: compare in a type that holds every value of both, converted with `From` — because `-1i32 as u64 < 1u64` is `false`, and `(2⁵³ + 1) as f64` equals `2⁵³` |
| [*Rust in Action* §2.3.3, run](comparing_numbers_claims_checked/README.md) | 201 | The book section on comparing numbers, fifteen claims checked: an `EPSILON` example whose difference is exactly zero, a `-42.0.sqrt()` that is −6.48, and a CPU exception that is really a flag |
| [Other number types](other_number_types/README.md) | 101 → 201 | What std leaves out and Python ships — complex numbers, fractions, big integers and decimals — with the crate for each, Python run beside every example, and *Rust in Action* §2.3.4 checked claim by claim |

## Related sections

- [Meet the `char`](../14_Strings/meet_the_char/README.md) — what those bytes encode, once you know what one is
- [Exactness](../09_Advanced/README.md) — where the float page hands off

## Po polsku

Ta sekcja mierzy wszystko w bajtach: `size_of` liczy właśnie je, `.len()` na łańcuchu znaków zwraca ich liczbę, a nie liczbę znaków, i to bajt jest jednostką, do której odwołują się pozostałe lekcje. Dwie rzeczy sprawiają tu polskiemu czytelnikowi więcej kłopotu niż angielskiemu: separatorem dziesiętnym jest u nas przecinek, a Rust — jak każdy język programowania — pisze `0.1` z kropką, więc polskie teksty o liczbach zmiennoprzecinkowych (*floating-point*) czyta się z tą podmianą w głowie; oraz system o podstawie 16 nazywany bywa raz „szesnastkowym”, raz „heksadecymalnym”, podczas gdy w kodzie i w komunikatach kompilatora zobaczysz zawsze `hex` i przedrostek `0x`. Ostatnie dwie lekcje wyjaśniają, dlaczego `f64` nie implementuje `Eq` ani `Ord` — to nie kaprys projektantów języka, tylko konsekwencja tego, że `NaN` nie jest równe samemu sobie.

**Szukaj po polsku:** bajt · system szesnastkowy · liczby zmiennoprzecinkowe · `rust size_of` · `rust f64 Eq Ord`
