# What an enum is

**Level:** 101 → 201 · newcomer to working knowledge

**One line:** An `enum` names a closed set of alternatives and makes that set a type — a value is exactly one of them, and a `match` that forgets one does not compile.

```rust
enum HouseLocation {
    Number(u32),                             // tuple variant: one field
    Name(String),                            // tuple variant: owns its data
    GridRef { easting: u32, northing: u32 }, // struct variant: named fields
    Unknown,                                 // unit variant: carries nothing
}

let where_i_live = HouseLocation::Number(4);
```

Four **variants**, four shapes, one type. `where_i_live` is a `HouseLocation` — not a `Number`, which is not a type and cannot be one. A variant is a way of *building* the type, never a type of its own.

That is the whole declaration. Everything below is what you get for it.

---

## The four shapes

| Shape | Written | Read back out with |
|---|---|---|
| unit | `Unknown` | `HouseLocation::Unknown` |
| tuple, one field | `Number(u32)` | `HouseLocation::Number(n)` |
| tuple, several fields | `Note(String, u32)` | `HouseLocation::Note(text, n)` |
| struct, named fields | `GridRef { easting: u32, northing: u32 }` | `HouseLocation::GridRef { easting, northing }` |

Mixing them in one enum is normal, not a code smell. The rule of thumb: **two fields or fewer and obvious from the variant name, use a tuple; otherwise name them.** `GridRef(u32, u32)` compiles fine and nobody will remember which number is which.

## Behaviour goes in an `impl` block

Enums take [`impl` blocks](../../16_Structs/impl_blocks/README.md) exactly the way structs do — same receivers, same associated functions. There is nothing struct-specific about them.

```rust
impl HouseLocation {
    fn describe(&self) -> String {
        match self {
            HouseLocation::Number(n) => format!("house number {n}"),
            HouseLocation::Name(name) => format!("the house called {name}"),
            HouseLocation::GridRef { easting, northing } => {
                format!("grid reference {easting} {northing}")
            }
            HouseLocation::Unknown => String::from("not known"),
        }
    }
}
```

Each arm does two jobs at once: it asks *which variant is this*, and it **binds a name to the payload** — `n`, `name`, `easting`. There is no separate unwrapping step, and no way to read `n` without having established that you are in the `Number` arm.

## The payoff: adding a variant breaks the right builds

Add a fifth variant six months later:

```rust
enum HouseLocation {
    // ...
    PoBox(u32),
}
```

```text
error[E0004]: non-exhaustive patterns: `&HouseLocation::PoBox(_)` not covered
  --> e0004.rs:12:15
   |
12 |         match self {
   |               ^^^^ pattern `&HouseLocation::PoBox(_)` not covered
   |
note: `HouseLocation` defined here
   |
 7 |     PoBox(u32),
   |     ----- not covered
```

The compiler hands you the list of every place that now has a hole. This is [exhaustiveness](../../GLOSSARY.md), and it is most of the reason to reach for an enum instead of a string or an integer: *the list of places to revisit is computed, not remembered.*

A `_` arm opts out of it permanently — it is a standing promise that every variant anyone ever adds belongs in that bucket:

```rust
fn postable(&self) -> bool {
    match self {
        HouseLocation::Unknown => false,
        _ => true,                       // PoBox will silently be postable
    }
}
```

Sometimes that promise is true and the `_` is right. Make it deliberately, because nothing will ask you again.

## `use Enum::*` shortens the arms, and costs more than it looks

```rust
use HouseLocation::*;
let a = Number(4);        // instead of HouseLocation::Number(4)
```

Tempting inside a long `match`. It is also the single most expensive habit in this section, because a mistyped arm stops being an error and becomes a silent catch-all — [a typo becomes a binding](../a_typo_becomes_a_binding/README.md) is that page, and it is worth reading before you adopt the import.

## Fieldless enums are numbers; enums with payloads are not

An enum where *no* variant carries data can be cast to an integer, and given explicit values:

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

enum Code { Ok = 200, NotFound = 404 }

Day::Sun as u8;            // 6 — position, counting from zero
Code::NotFound as i32;     // 404
Day::Mon < Day::Fri;       // true, once PartialOrd is derived — declaration order
```

Add one payload-carrying variant and the cast is gone:

```text
error[E0605]: non-primitive cast: `HasData` as `u8`
  = note: an `as` expression can be used to convert enum types to numeric types
          only if the enum type is unit-only or field-less
```

And the reverse never works, for any enum — `1 as Day` is `E0605` too. An integer is not a `Day`, because most integers are not days. Going that direction is a fallible conversion, which is what `TryFrom` is for.

## If you are coming from another language

**Python.** `enum.Enum` is the closest thing, and it stops at the fieldless case: members are singletons with a fixed `.value`, so there is no per-member payload. The Rust equivalent of `Number(4)` is a `Union` of dataclasses, and the equivalent of the exhaustiveness check is running mypy in strict mode — the interpreter itself will never object to a `match` with a missing case. What transfers is the *habit*: if you already reach for `Enum` instead of a string constant, you already have the instinct. What is new is that Rust's version carries data and that missing a case is a build failure rather than an `else: pass`.

**ABAP.** Before 7.51 you wrote a constants structure and a `CASE`, and the compiler checked nothing:

```abap
CONSTANTS: BEGIN OF co_status,
             blank  TYPE c LENGTH 1 VALUE ' ',
             marked TYPE c LENGTH 1 VALUE 'M',
             cast   TYPE c LENGTH 1 VALUE 'C',
           END OF co_status.
```

Any `c` value fits that variable, `WHEN OTHERS` is optional, and adding a fourth status is a grep. 7.51's `TYPES: BEGIN OF ENUM t_status ... END OF ENUM t_status.` fixes the first half — the variable now holds only declared values — but **not** the second: a `CASE` on an enumerated type still compiles with a missing `WHEN`. That second half is what Rust adds, and it is the half that finds the code you forgot.

**C.** A C `enum` is an `int` in a hat: any integer fits in one, `switch` needs no `default`, and there is nowhere to put a payload — for that you hand-build a `struct` holding a tag and a `union`, and remember to check the tag on every read. Rust's `enum` *is* that struct, with the check compulsory. [What a union is](../../09_Advanced/what_a_union_is/README.md) builds both side by side.

**Ada.** The closest ancestor, and the one that will mislead you in a specific way. Ada's enumerations are richer than C's in exactly the directions Rust's are not:

```ada
type Day     is (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
subtype Weekday is Day range Mon .. Fri;
type Colour  is (White, Red, Yellow, Green, Blue, Brown, Black);
type Light   is (Red, Amber, Green);   --  Red and Green are overloaded
```

Three of those have no Rust spelling. There are **no range subtypes** — `Weekday` would be a separate enum, or a runtime check. **Literals are never overloaded**: `Red` cannot belong to two enums and be sorted out by context, which is precisely why Rust makes you write `Colour::Red` and why the glob import that removes the qualifier is a trap. And Ada's `'Succ`, `'Pred`, `'Pos` and `'Val` have no built-in equivalent; you derive `PartialOrd` for comparison and write the rest.

What Rust has that Ada's enumerations do not is the payload. Ada puts that in a **variant record** (`case Kind is when ...`) — a separate feature, with a discriminant you can leave inconsistent. Rust folds the two into one construct, which is why a Rust enum feels like both at once.

## The verified output

[`examples/what_an_enum_is.rs`](examples/what_an_enum_is.rs) compiled and run:

<!-- output:what_an_enum_is -->
*Verified output of [`what_an_enum_is.rs`](examples/what_an_enum_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
house number 4                postable: true
the house called Dunroamin    postable: true
grid reference 51820 17450    postable: true
not known                     postable: false

Number(4) once imported: house number 4
both are HouseLocation: 2
```
<!-- /output -->

---

## See also

- [`Some` and `None`](../../17_Option_and_Result/some_and_none/README.md) — `Option` is an ordinary enum with two variants and no special powers
- [Variants that carry data](../variants_that_carry_data/README.md) — what the payload costs, measured
- [`if let`](../../17_Option_and_Result/if_let/README.md) — a `match` with one arm, and the exhaustiveness you trade for it
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — `A | B` and ranges, for when several variants share an answer
- [Six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) — writing your own enum when `Option` runs out of shapes
