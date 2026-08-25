# Numbers and bytes

**One line:** The unit everything else is measured in — what a byte is, how to write one down, what people pack into one, and the one number type that cannot represent the value you typed.

This section is where `.len()` finally means something. A byte is the thing `size_of` counts, hexadecimal is how you read one without counting bits, a flag is a one-bit field, and `f64` is the type Rust withholds `Eq` and `Ord` from — for a reason the last page measures rather than asserts.

| Lesson | Level | What it teaches |
|---|---|---|
| [Meet the byte](meet_the_byte/README.md) | 101 → 201 | `u8` is one byte and the unit `size_of` counts in — plus the three bills a width comes with: overflow that differs by build, the shift the type picks, and a `.len()` measured in bytes |
| [Why hexadecimal](why_hexadecimal/README.md) | 101 → 201 | Why a byte is two hex digits and always will be — plus the three traps that follow: unpadded `{:x}` losing the byte boundary, `from_str_radix` refusing the `0x` it just printed, and hex of a negative showing two's complement |
| [Bit flags](bit_flags/README.md) | 201 | Several values in one integer: a flag is a one-bit field and a header field is an n-bit flag — plus the zero-valued flag `&` cannot test, and the missing mask only a middle field punishes |
| [What a float actually stores](what_a_float_stores/README.md) | 201 | The one division that ends exactness — why `0.1` is not 0.1, why the error goes both ways, and why Rust withholds `Eq` and `Ord` from `f64` |

## Related sections

- [Meet the `char`](../14_Strings/meet_the_char/README.md) — what those bytes encode, once you know what one is
- [Exactness](../09_Advanced/README.md) — where the float page hands off
