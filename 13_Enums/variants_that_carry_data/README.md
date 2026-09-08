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

## When the largest variant is the rare one

The rule cuts both ways: every value is as big as the largest payload, including the values carrying the smallest one.

```rust
enum RecordData      { A(Ipv4Addr), Aaaa(Ipv6Addr), Naptr(Naptr) }
enum BoxedRecordData { A(Ipv4Addr), Aaaa(Ipv6Addr), Naptr(Box<Naptr>) }
```

| Type | Bytes | Why |
|---|---|---|
| `Ipv4Addr` | 4 | one A record |
| `Ipv6Addr` | 16 | one AAAA record |
| `Naptr` | 104 | two integers and four `String`s |
| `RecordData` | **104** | so an A record spends 104 bytes to store 4 |
| `BoxedRecordData` | **24** | the rare variant moved behind a pointer |

The tag behaves differently in the two, and it changes nothing. In `RecordData` it is free by the niche rule above — `Naptr`'s `String` pointers donate one — so the enum is exactly `size_of::<Naptr>()`. In `BoxedRecordData` it costs 8, because `Ipv6Addr` uses every one of its bit patterns and there are three payloads competing for one spare pattern. The enum still fell from 104 bytes to 24, since the payload rather than the tag was the problem. Boxing does not shrink `Naptr`; it moves it, so the enum need only be the largest payload still stored inline plus that tag — 16 + 8 — which is a number independent of how big the boxed variant is.

Cloudflare made this exact change to 1.1.1.1's DNS cache and [published the numbers ↗](https://blog.cloudflare.com/dns-cache-memory-optimization-1111/) in August 2026. Its `RecordData` was 144 bytes because `NAPTR` is 136, while `A` needs 4 and `AAAA` 16 — and those two are over 80% of the traffic, so most records carried over 120 bytes of padding, on a cache holding over 250 billion entries. Boxing the large variants took the enum to 24 bytes: the same number as the table above, for the same reason.

The trade is paid at the allocator. A boxed payload is a separate allocation rounded up to the next size class — jemalloc puts a 40-byte record in a 48-byte bin — and it lives elsewhere on the heap, so reading it is a pointer hop and possibly another cache line. **Rare and large** is what makes the trade pay; box a common variant instead and every value is charged the allocation.

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

## Practice

**Adds, or multiplies?** Model a shape four ways in one enum — a point, a circle with a radius, a rectangle with two sides, and a labelled one carrying a `String`. Then model the same four as a single struct with every field always present.

Predict `size_of` for both before printing them, and explain the difference in one sentence. Then say what the enum makes impossible that the struct permits — the answer is not about memory.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:variants_that_carry_data_kata -->
*[`variants_that_carry_data_kata.rs`](examples/variants_that_carry_data_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a struct multiplies, an enum adds.
//!
//!   rustc --edition 2024 variants_that_carry_data_kata.rs -o /tmp/vcd && /tmp/vcd

use std::mem::size_of;

#[derive(Debug)]
enum Shape {
    Point,
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Label(String),
}

// The same four cases modelled the wrong way round: every field always
// present, and four of the five combinations meaningless.
#[allow(dead_code)]
#[derive(Debug)]
struct ShapeStruct {
    kind: u8,
    radius: f64,
    w: f64,
    h: f64,
    label: String,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Point => 0.0,
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rect { w, h } => w * h,
        Shape::Label(name) => name.len() as f64 * 0.0,
    }
}

fn main() {
    println!("ONE TYPE, FOUR SHAPES OF VALUE");
    for s in [Shape::Point, Shape::Circle { radius: 1.0 },
              Shape::Rect { w: 2.0, h: 3.0 }, Shape::Label(String::from("origin"))] {
        println!("  {:<34} area {:.4}", format!("{s:?}"), area(&s));
    }
    println!();

    println!("  the Label arm can read its own String: {:?} is {} chars",
             "origin", "origin".len());
    println!();
    println!("ADDS, RATHER THAN MULTIPLIES");
    println!("  A struct with fields A and B can be any combination of the two,");
    println!("  so its possibilities MULTIPLY. An enum is exactly one variant at");
    println!("  a time, so they ADD. That is what \"algebraic data type\" means,");
    println!("  and it is a modelling decision before it is a memory one:");
    println!("  a Circle has no width, and with the enum there is no field for");
    println!("  one to be wrong in.");
    println!();

    println!("WHAT IT COSTS");
    println!("  size_of::<Shape>()        {:>3} bytes", size_of::<Shape>());
    println!("  size_of::<ShapeStruct>()  {:>3} bytes", size_of::<ShapeStruct>());
    println!("  size_of::<String>()       {:>3} bytes", size_of::<String>());
    println!("  size_of::<f64>()          {:>3} bytes", size_of::<f64>());
    println!();
    println!("  An enum is as big as its LARGEST variant plus a discriminant,");
    println!("  rounded for alignment -- so it costs what the biggest case");
    println!("  needs, not what all the cases need together. The struct pays");
    println!("  for every field on every value, and most of them are unused on");
    println!("  most values.");
    println!();

    println!("AND THE MATCH IS WHERE IT PAYS AGAIN");
    println!("  Each arm destructures its own variant, so `radius` exists in");
    println!("  the Circle arm and nowhere else. With the struct you would read");
    println!("  `s.radius` in a branch chosen by `s.kind`, and nothing checks");
    println!("  that you picked the right field for the right kind.");

    assert_eq!(area(&Shape::Rect { w: 2.0, h: 3.0 }), 6.0);
    assert!(size_of::<Shape>() < size_of::<ShapeStruct>());
}
```
<!-- /source -->

<!-- output:variants_that_carry_data_kata -->
*Verified output of [`variants_that_carry_data_kata.rs`](examples/variants_that_carry_data_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
ONE TYPE, FOUR SHAPES OF VALUE
  Point                              area 0.0000
  Circle { radius: 1.0 }             area 3.1416
  Rect { w: 2.0, h: 3.0 }            area 6.0000
  Label("origin")                    area 0.0000

  the Label arm can read its own String: "origin" is 6 chars

ADDS, RATHER THAN MULTIPLIES
  A struct with fields A and B can be any combination of the two,
  so its possibilities MULTIPLY. An enum is exactly one variant at
  a time, so they ADD. That is what "algebraic data type" means,
  and it is a modelling decision before it is a memory one:
  a Circle has no width, and with the enum there is no field for
  one to be wrong in.

WHAT IT COSTS
  size_of::<Shape>()         24 bytes
  size_of::<ShapeStruct>()   56 bytes
  size_of::<String>()        24 bytes
  size_of::<f64>()            8 bytes

  An enum is as big as its LARGEST variant plus a discriminant,
  rounded for alignment -- so it costs what the biggest case
  needs, not what all the cases need together. The struct pays
  for every field on every value, and most of them are unused on
  most values.

AND THE MATCH IS WHERE IT PAYS AGAIN
  Each arm destructures its own variant, so `radius` exists in
  the Circle arm and nowhere else. With the struct you would read
  `s.radius` in a branch chosen by `s.kind`, and nothing checks
  that you picked the right field for the right kind.
```
<!-- /output -->

</details>

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

the largest variant sets the size, and every value pays it:
  Ipv4Addr                4
  Ipv6Addr               16
  Naptr                 104   <- two integers and four Strings
  RecordData            104   <- an A record spends this to store 4 bytes
  BoxedRecordData        24   <- the rare big variant moved behind a pointer

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
- [`Vec`](../../26_Collections/the_vec/README.md) — the other half of that cache: what the third number costs when nothing will grow again

## Po polsku

Polska szkoła daje tu przewagę, z której warto skorzystać: to jest arytmetyka liczności zbiorów z matematyki dyskretnej, tylko przepisana na typy. Struktura zachowuje się jak iloczyn kartezjański swoich pól — pole *i* pole, więc liczności się mnożą (3 × 2 = 6 portfeli); wyliczenie zachowuje się jak suma rozłączna wariantów — wariant *albo* wariant, więc liczności się dodają (3 + 2 = 5 płatności). Stąd nazwa **algebraiczne typy danych**, i stąd też polska terminologia w dwóch odmianach, obu spotykanych: typ produktowy (iloczynowy) i typ sumaryczny (wariantowy). Test praktyczny mieści się w jednym pytaniu, które warto zadawać na głos: **czy te dwa fakty mogą być prawdziwe naraz?** Jeśli nie, to wyliczenie. Struktura z dwoma polami `Option`, z których nigdy oba nie powinny być `Some`, jest typem sumarycznym przebranym za produkt — ma cztery stany, z czego dwa są bez sensu, a bronić się przed nimi musi już na zawsze każdy, kto tę strukturę czyta.

Druga połowa strony to cena ładunku, i tu najczęściej powtarzana reguła — „wyliczenie jest tak duże jak jego największy wariant plus znacznik” — jest prawdziwa w pierwszej części, a myląca w drugiej. Znacznik bywa **darmowy**, jeśli znajdzie się nisza (*niche*): układ bitów, którego sam ładunek nigdy nie wyprodukuje. `Box` nigdy nie jest zerowy, więc `None` mieści się we wskaźniku zerowym i `Option<Box<u64>>` zajmuje te same 8 bajtów co goły wskaźnik — czyli wahanie „czy to opakowanie się opłaca” ma odpowiedź „nie kosztuje nic”. Gdy niszy nie ma, płacisz uczciwie: wyliczenie o dwóch wariantach niosących `u64` to 16 bajtów, bo `u64` zużywa wszystkie swoje układy bitów, a jednobajtowy znacznik wyrównanie zaokrągla do ośmiu. Na drugim końcu skali stoi wyliczenie **bez wariantów**: zero bajtów, bo żadna wartość takiego typu nie może istnieć — i dokładnie w ten sposób `Infallible` mówi „ten `Result` nie ma jak być `Err`”, nie płacąc za to ani bajta.

Zostaje jeszcze `std::mem::discriminant`, czyli odpowiedź na pytanie „czy to ten sam rodzaj rzeczy?” bez pisania `match`a i bez zaglądania w ładunek: `InCoin(Penny)` i `InCoin(Dime)` są sobie równe, `InCoin` i `InNote` już nie. Przydaje się, gdy ładunki nie implementują `PartialEq` albo gdy świadomie chcesz je pominąć. Czego ta funkcja **nie** zrobi, to nie poda numeru wariantu — zwracana wartość jest nieprzezroczysta i nadaje się wyłącznie do porównywania, właśnie dlatego, że w wyliczeniu zoptymalizowanym niszą żadnego numeru może nigdzie w pamięci nie być.

**Szukaj po polsku:** algebraiczne typy danych · typ sumaryczny · suma rozłączna · `rust enum size niche optimization` · `rust std::mem::discriminant`
