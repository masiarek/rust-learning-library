# Generic enums

**Level:** 201 · working knowledge

**One line:** An enum takes type parameters exactly as a struct does — every variant shares the list, which is what makes `Option<T>` and `Result<T, E>` ordinary library code rather than compiler magic.

```rust
enum Slot<T> {
    Filled(T),
    Reserved,
    Empty,
}

let seat = Slot::Filled("Ada");
```

One parameter, three variants, and only one of them uses it. `Reserved` and `Empty` carry nothing and still belong to `Slot<T>` — the parameter is on the *type*, not on the variant.

## Every variant shares the parameters

There is no way to give one variant a `T` of its own: `<T>` is declared once, after the enum's name, and every variant is built from that same list. Two consequences, and the second one surprises people.

**A payload-free variant still needs a `T`.** Nothing in `Slot::Empty` says what kind of slot it is, so this is [`E0282`](../when_the_compiler_cannot_infer/README.md) unless something else in the function settles it:

```rust
// let nothing = Slot::Empty;      // error[E0282]: type annotations needed
let nothing = Slot::<u8>::Empty;   // or let it come from a later use
```

**The variants are not types.** `Slot::Filled` is a way of building a `Slot<T>`, not a type you can name in a signature — the same rule as [any other enum](../../13_Enums/what_an_enum_is/README.md), and it is why a function returning "either a score or a reason" returns one enum rather than two things.

## Two parameters

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn parse_score(text: &str) -> Either<u8, String> { /* … */ }
```

That is `Result<T, E>` with the names changed, and `Result` really is exactly this — two parameters, two variants, one of each, declared in the standard library and no more built-in than your `Either`. `Option<T>` is the same idea with one parameter and a payload-free second variant, which is the shape at the top of this page.

Both parameters may be filled with the same type, and they are still two independent parameters: `Result<u8, u8>` is a legal type, and `Ok(3)` and `Err(3)` are different values in it.

## What it costs

An enum is laid out as its largest variant plus a tag, so `T` decides the size:

| Type | `size_of` | Why |
|---|---|---|
| `Slot<u8>` | 2 | one byte of payload, one of tag |
| `Slot<u64>` | 16 | eight of payload, and the tag padded to the alignment |
| `Either<u8, String>` | 24 | the `String` variant is the largest, and the tag is **free** |

The third row is the [niche](../../17_Option_and_Result/nullable_pointers/README.md) again: a `String`'s pointer is never null, so the discriminant hides in a bit pattern the type could not otherwise hold, and `Either<u8, String>` is the same 24 bytes as a bare `String`.

## Methods, and where a bound goes

`impl` blocks work as they do for any enum, with the parameter declared on the block:

```rust
impl<T> Slot<T> {
    fn is_filled(&self) -> bool {
        matches!(self, Slot::Filled(_))
    }
}

impl<T: Display> Slot<T> {
    fn describe(&self) -> String { /* … */ }
}
```

`is_filled` works for every `T`; `describe` needs to print the payload, so its block asks for `Display`. The bound belongs on the block that spends it, never on the `enum` line — the same rule, for the same reasons, as on a struct.

## If you are coming from another language

**Python.** The counterpart is a union of dataclasses — `Filled(value: T) | Reserved | Empty` with `typing.Generic[T]` — and `Optional[T]` is literally `Union[T, None]`, which is `Option<T>` written the other way round. Two things change. The generic is erased at run time in Python, so `Slot[int]` and `Slot[str]` are one class and the parameter exists only for the type-checker, while `Slot<u8>` and `Slot<u64>` are different types of different sizes. And the `match` becomes exhaustive: forgetting `Reserved` is a build error rather than a fall-through to `None`.

**ABAP.** There is no sum type at all, which is why this shape is unfamiliar rather than merely differently spelled. The usual ABAP model of "one of these, with different data each" is a flat structure holding a `kind` field plus every component any kind might need, most of them empty on any given row, and a `CASE` that nobody checks for completeness. `Slot<T>` says the same thing with the impossible combinations unwritable — a `Reserved` slot has no payload field to leave blank — and the compiler lists every `CASE` that a new variant breaks. The generic half then adds what a structure could never do: the same three states over an integer, a string, or a reference, without three copies of the type.

**Java.** `Optional<T>` is the closest single type, and since Java 17 the general shape exists too: a sealed interface with record implementations gives you variants, a shared parameter, and exhaustive `switch`. The difference left is erasure — `Slot<Integer>` boxes, `Slot<u8>` does not — and that Rust's enum is one value with a tag rather than a pointer to one of several classes.

**C++.** `std::variant<A, B>` is the same idea with worse ergonomics — `std::visit` in place of `match`, and no exhaustiveness guarantee unless you construct one. `std::optional<T>` and `std::expected<T, E>` (C++23) are `Option` and `Result` arriving separately, as library types, which is what they are in Rust too.

## The verified output

[`examples/generic_enums.rs`](examples/generic_enums.rs) compiled and run:

<!-- output:generic_enums -->
*Verified output of [`generic_enums.rs`](examples/generic_enums.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
filled with Ada          filled: true
reserved                 filled: false
empty                    filled: false

Slot::<u8>::Empty is Empty, filled: false

     4 -> score 4
     9 -> rejected: 9 is above the 0-5 range
  five -> rejected: "five": invalid digit found in string

size_of::<Slot<u8>>()           2
size_of::<Slot<u64>>()          16
size_of::<Either<u8, String>>() 24
size_of::<String>()             24

Result<u8, u8> is a real type: Err(3)
```
<!-- /output -->

## See also

- [What an enum is](../../13_Enums/what_an_enum_is/README.md) — the non-generic version: four variant shapes, and the exhaustiveness that comes free
- [A generic recursive type](../a_generic_recursive_type/README.md) — a generic enum that contains itself, and the `Box` it needs
- [When the compiler cannot infer](../when_the_compiler_cannot_infer/README.md) — the `E0282` a payload-free variant produces
- [Variants that carry data](../../13_Enums/variants_that_carry_data/README.md) — what a payload costs, measured
- [Six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) — writing your own enum when `Option`'s two variants run out
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — the two library enums this page is a general case of
