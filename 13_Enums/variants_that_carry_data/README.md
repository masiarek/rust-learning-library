# Variants that carry data

**Level:** 201 · working knowledge

**One line:** A struct multiplies its possibilities and an enum adds them — which is what "algebraic data type" means, and it decides both how you model a problem and what the value costs in memory.

```rust
enum Payment {
    InCoin(Coin),   // 3 possible values
    InNote(Note),   // 2 possible values
}                   // 3 + 2 = 5 payments exist
```

Compare the struct that holds both:

```rust
struct Wallet {
    coin: Coin,     // 3
    note: Note,     // 2
}                   // 3 x 2 = 6 wallets exist
```

A struct is a **product type**: field *and* field, so the counts multiply. An enum is a **sum type**: variant *or* variant, so they add. Every data model you will write in Rust is these two composed, and choosing the wrong one is the most common modelling mistake there is — a struct with three `Option` fields, two of which must never be `Some` at the same time, is a sum type wearing a product's clothes.

---

## The counts are literally countable

The example program builds every possible `Wallet` and every possible `Payment` and counts them, rather than asserting the arithmetic:

```text
Coin has 3 values, Note has 2
Wallet  (struct, AND) has 6 values  = 3 x 2
Payment (enum,   OR ) has 5 values  = 3 + 2
```

## What a payload costs

The rule you will read most often is *"an enum is as big as its largest variant, plus a tag."* The first half is right. The second is often wrong, and the difference is worth understanding, because it is why `Option` is free in the cases where you would have worried about it.

Measured on a 64-bit target:

| Type | Bytes | Why |
|---|---|---|
| `String` | 24 | pointer, capacity, length |
| `HouseLocation` | **24** | its largest payload is a `String` — and the tag costs nothing |
| `u64` | 8 | |
| `TagCosts` — `A(u64)`, `B(u64)` | **16** | 8 for the payload, 8 more for a tag that has nowhere to hide |
| `Box<u64>` | 8 | |
| `TagFree` — `A(Box<u64>)`, `B` | **8** | a `Box` is never null, so *null* means `B` |
| `Option<Box<u64>>` | 8 | the same trick, in the standard library |
| `Never` — no variants at all | **0** | no value can exist, so none needs storing |

The mechanism is the **niche**: a spare bit pattern the payload can never itself produce. A `Box` is never null, so 256 variants could hide in the low addresses. A `u64` uses every one of its bit patterns, so `TagCosts` has to store the tag separately — and alignment rounds that one byte up to eight.

So the honest rule is: **as big as the largest payload, plus a tag only when there is nowhere free to put it.** Every time you have hesitated over whether `Option<Box<T>>` is worth the wrapper, the answer was that it is the same eight bytes as the pointer.

`Never` is not a curiosity: an enum with no variants is a type no value can inhabit, which is how [`Infallible`](../../17_Option_and_Result/result_aliases/README.md) says *"this `Result` cannot be an `Err`"* and pays nothing for saying it.

## Which variant is this, without a `match`

`std::mem::discriminant` compares the tag and ignores the payload:

```rust
let a = Payment::InCoin(Coin::Penny);
let b = Payment::InCoin(Coin::Dime);
discriminant(&a) == discriminant(&b);   // true  — same variant, different payload
```

It is the right tool for *"are these two values the same kind of thing"* when the payloads do not implement `PartialEq`, or when you deliberately want to ignore them. What it will not do is hand you the number: the value it returns is opaque and comparable, nothing more, because for a niche-optimized enum there may be no number stored anywhere to hand over.

## Reaching for the sum type

The test is a sentence: **can these two facts be true at once?** If no, it is an enum.

```rust
// A product type, describing something that is really a sum.
struct Reading {
    value: Option<f64>,
    error: Option<String>,   // both Some? both None? the type permits it
}

// The sum type, where those two states cannot be spelled.
enum Reading {
    Measured(f64),
    Failed(String),
}
```

The first version has four states, two of which are nonsense that every reader of the struct has to defend against forever. The second has two. This is the same argument [six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) makes with six cases instead of two, and the same one that makes `Result` a better return type than a value-plus-a-flag.

## If you are coming from another language

**Python.** The direct translation is `Union[Measured, Failed]` over two dataclasses, matched with `match` — and `enum.Enum` is *not* it, because its members cannot carry per-instance data. Python's built-in sum type you have already used is `None | T`, which is Rust's `Option` with no name and no compiler check. What transfers: if you have written `if result is None:` you have been hand-checking a discriminant for years.

**ABAP.** There is no sum type. The idiom is a flat structure with a `kind` field and one set of fields per kind, all of them present all of the time — a product type standing in for a sum, with a comment explaining which fields to read for which kind. That comment is exactly what a Rust enum makes into a compiler-checked rule; the desync it warns you about is the bug the type system removes.

**C.** `struct { enum Tag tag; union { ... } data; }` — the same layout Rust generates, built by hand. What Rust adds is not the memory trick, which C had first, but the impossibility of reading `data.name` when `tag` says `NUMBER`.

**Haskell / OCaml / F#.** This is the feature, under the name you already use: `data Payment = InCoin Coin | InNote Note`. Rust's version is monomorphic and unboxed rather than heap-allocated, which is why the size table above is interesting at all — and why recursion needs an explicit `Box`, since a type cannot contain itself by value.

## The verified output

[`examples/variants_that_carry_data.rs`](examples/variants_that_carry_data.rs) compiled and run:

<!-- output:variants_that_carry_data -->
*Verified output of [`variants_that_carry_data.rs`](examples/variants_that_carry_data.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Coin has 3 values, Note has 2
Wallet  (struct, AND) has 6 values  = 3 x 2
Payment (enum,   OR ) has 5 values  = 3 + 2

  InCoin(Penny)
  InCoin(Nickel)
  InCoin(Dime)
  InNote(Five)
  InNote(Ten)

sizes on this target (64-bit pointers):
  String                 24
  HouseLocation          24   <- same as its largest payload
  u64                     8
  TagCosts               16   <- payload + a tag it must store
  Box<u64>                8
  TagFree                 8   <- tag hidden in the null pointer
  Option<Box<u64>>        8   <- the same trick, in the library
  Never                   0   <- no variants, so no bytes

same variant, different payload: true
different variant:               false
```
<!-- /output -->

---

## See also

- [What an enum is](../what_an_enum_is/README.md) — the declaration, and the four shapes of variant
- [`Option` is a one-item collection](../../17_Option_and_Result/option_as_collection/README.md) — the tag, the niche, and `None` as variant zero
- [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) — where the free tag matters most, and what makes a recursive type possible
- [What a union is](../../09_Advanced/what_a_union_is/README.md) — the untagged version, and the desync only it can have
- [Six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) — when two variants are not enough
