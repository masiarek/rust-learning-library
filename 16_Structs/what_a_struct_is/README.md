# What a struct is

**Level:** 101 → 201 · for newcomers

**One line:** A struct names a group of values and makes that name a *type*. It holds no behaviour.

```rust
struct Ballot {
    voter: String,
    scores: Vec<u8>,
}
```

- Named fields, so nothing depends on position (unlike `(String, Vec<u8>)`).
- A new type. Kept apart from every other type, including an identical one under a different name.
- No behaviour. Methods go in an `impl` block; shared behaviour in traits.

---

## Data here, behaviour there

```rust
struct Ballot { … }        // data

impl Ballot {              // behaviour
    fn new(voter: &str) -> Self { … }   // associated function — no self
    fn total(&self) -> u32 { … }        // method — takes self
}
```

Struct = record. `impl` = functions. `trait` = behaviour many types share. This is why there are no classes and no inheritance; traits do that job. Details: [`impl` blocks](../impl_blocks/README.md).

## Three flavors, and a fourth spelling

| Flavor | Written | Fields by |
|---|---|---|
| named-field | `struct Ballot { voter: String }` | `.voter` |
| tuple struct | `struct Precinct(u32);` | `.0` |
| unit struct | `struct Sealed;` | none |

A tuple struct is a named-field struct whose names are digits — `Precinct { 0: 7 }` compiles and equals `Precinct(7)`.

Not "a classic C struct": layout is **undefined by default** (fields may be reordered), fields are **private by default**, assignment **moves** unless the type is [`Copy`](../copy_vs_clone/README.md).

`struct AlsoEmpty {}` also has no fields but is **not** a unit struct — it needs braces:

```text title="Real rustc output"
error[E0423]: expected value, found struct `AlsoEmpty`
   |     println!("{:?}", AlsoEmpty);
   |                      ^^^^^^^^^ help: use struct literal syntax instead: `AlsoEmpty {}`
```

A unit struct holds nothing, so behaviour is all it can hold. Use one when you need something to `impl` a trait on.

`struct Sealed;` declares **two** things sharing a name: the type `Sealed` and a constant value `Sealed`. A type alias aliases only the type:

```rust
type Alias = Sealed;
let a: Sealed = Alias {};   // fine — type namespace
let b: Sealed = Alias;      // error[E0423]: expected value, found type alias `Alias`
```

## Names

`UpperCamelCase` for types, `snake_case` for fields and methods — `rustc` lints both (`non_camel_case_types`, `non_snake_case`). Name the struct for what the group *means*: `Ballot`, not `Data`.

## Privacy is per module

Private = **visible inside the defining module**, not "only to its own methods". Any code in that module reads any field.

For a tuple struct, a private field makes the *constructor* private:

```text
error[E0603]: tuple struct constructor `Ballot` is private
   |  a constructor is private if any of the fields is private
```

This is what [the newtype](../newtype_score/README.md) relies on: one checked door.

## What you cannot do

- **Mark one field mutable.** `mut` is on the binding — all of it or none.
- **Put `&str` in a field without a lifetime.** `error[E0106]: missing lifetime specifier`. Owned `String` sidesteps it; that is why beginner code uses it.
- **Print with `{}`.** No `Display` unless you write one. See [Debug and Display](../../15_First_Programs/debug_vs_display/README.md).

## If you are coming from another language

**Python.** Closest match is a `@dataclass` or `NamedTuple`. The field list looks the same, and `self` is explicit in both.

| | Python | Rust |
|---|---|---|
| wrong field name | `AttributeError` at runtime | build error |
| add a field at runtime | `obj.new_field = 1` works | impossible; the field set is the type |
| data + methods | one `class` body | `struct` + `impl`, separate |

`__slots__` is the nearest Python analogue to a fixed field set, but it is opt-in and Rust's is not.

**ABAP.** `TYPES: BEGIN OF ty_ballot, voter TYPE string, ... END OF ty_ballot.` maps almost directly; `DATA ls_ballot TYPE ty_ballot` is the instance.

| | ABAP | Rust |
|---|---|---|
| methods on the type | none — behaviour goes in a class beside it | `impl` block |
| flat vs deep | a distinction you manage | none; a `String` field always owns its data |
| uninitialised | initial value, free | must supply every field or it does not compile |

`impl` is the half ABAP has no place for, which is why a Rust struct behaves more like a small class than like a `TYPES` group.

---

## Practice

**Three flavors, and the two things the compiler keeps apart.**

1. Define `Color(i32, i32, i32)` and `Point(i32, i32, i32)`, write a function taking one, pass it the other. Read `E0308` — then note what a bare `(i32, i32, i32)` would have accepted.
2. Put a tuple struct in a module with `pub` on the struct but not the field; construct it from outside. `E0603`: *"a constructor is private if any of the fields is private."* Why does that follow, rather than being an extra rule?
3. Give a unit struct a `Display` impl — a type whose only content is behaviour.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_a_struct_is_kata -->
*[`what_a_struct_is_kata.rs`](examples/what_a_struct_is_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three flavors, and the two things the compiler keeps apart.
//!
//!   rustc --edition 2024 what_a_struct_is_kata.rs -o /tmp/wasik && /tmp/wasik

use std::fmt;

// Same three fields, same types, two different TYPES. This is the whole
// point of a tuple struct over a bare tuple: the name is load-bearing.
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn paint(c: &Color) -> String {
    format!("#{:02x}{:02x}{:02x}", c.0, c.1, c.2)
}

mod sealed {
    // `pub` on the STRUCT is not `pub` on the FIELD, and for a tuple struct
    // that also makes the CONSTRUCTOR private — you cannot write Ballot(9)
    // from outside this module, because doing so would set a private field.
    #[derive(Debug)]
    pub struct Ballot(u32);

    impl Ballot {
        pub fn new(n: u32) -> Self {
            Ballot(n)
        }
        pub fn get(&self) -> u32 {
            self.0
        }
    }
}

// A unit struct carries no data, so the only thing it can be is a place to
// hang behaviour. That is not a consolation prize — it is the use case.
struct Blank;

impl fmt::Display for Blank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(no ballot returned)")
    }
}

fn main() {
    println!("1. Identical shapes, different types");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("   Color(0,0,0) -> {}", paint(&black));
    println!("   {origin:?} is the same THREE i32s and paint(&origin) will not compile:");
    println!("     error[E0308]: mismatched types — expected `&Color`, found `&Point`");
    println!("   A bare (i32, i32, i32) would have accepted either. The name is the guard.");

    println!("\n2. Destructuring — the tuple struct comes apart like a tuple");
    let Color(r, g, b) = black;
    println!("   let Color(r, g, b) = black;  ->  r={r} g={g} b={b}");
    let Point(x, y, z) = origin;
    println!("   let Point(x, y, z) = origin; ->  x={x} y={y} z={z}");
    println!("   Same shape, same destructuring, and still not interchangeable.");

    println!("\n3. A private field makes a tuple struct's CONSTRUCTOR private");
    let ballot = sealed::Ballot::new(431);
    println!("   sealed::Ballot::new(431) -> {ballot:?}, get() = {}", ballot.get());
    println!("   sealed::Ballot(431) from out here is E0603:");
    println!("     `a constructor is private if any of the fields is private`");
    println!("   That is the newtype's whole guarantee: one door, and the module owns it.");

    println!("\n4. A unit struct holds nothing, so behaviour is all it can hold");
    println!("   {Blank}   <- via its Display impl; the value carries no data at all");
}
```
<!-- /source -->

<!-- output:what_a_struct_is_kata -->
*Verified output of [`what_a_struct_is_kata.rs`](examples/what_a_struct_is_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Identical shapes, different types
   Color(0,0,0) -> #000000
   Point(0, 0, 0) is the same THREE i32s and paint(&origin) will not compile:
     error[E0308]: mismatched types — expected `&Color`, found `&Point`
   A bare (i32, i32, i32) would have accepted either. The name is the guard.

2. Destructuring — the tuple struct comes apart like a tuple
   let Color(r, g, b) = black;  ->  r=0 g=0 b=0
   let Point(x, y, z) = origin; ->  x=0 y=0 z=0
   Same shape, same destructuring, and still not interchangeable.

3. A private field makes a tuple struct's CONSTRUCTOR private
   sealed::Ballot::new(431) -> Ballot(431), get() = 431
   sealed::Ballot(431) from out here is E0603:
     `a constructor is private if any of the fields is private`
   That is the newtype's whole guarantee: one door, and the module owns it.

4. A unit struct holds nothing, so behaviour is all it can hold
   (no ballot returned)   <- via its Display impl; the value carries no data at all
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:what_a_struct_is -->
*Verified output of [`what_a_struct_is.rs`](examples/what_a_struct_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three flavors, and a fourth spelling
   named   Ballot { voter: "Ada", scores: [] }
   tuple   Precinct(7)   field reached by index: 7
   unit    Sealed          — the type has exactly one value
   braces  AlsoEmpty      — `AlsoEmpty {}`, NOT the same as a unit struct

2. A tuple struct is a named-field struct whose fields are numbers
   Precinct { 0: 7 } == Precinct(7)  ->  true

3. Field ORDER in the expression is free; the value is the same
   Ballot { voter: "Ben", scores: [5, 2] }
   Ballot { voter: "Ben", scores: [5, 2] }
   same value: true

4. Data in the struct, behaviour in the impl block
   Cara scored [5, 2, 0], total 7
   Ballot::total(&ballot) == ballot.total()  ->  true

5. Privacy is per module — `count` is private, and that is why
   the module has to hand you a method to read it
   id (pub)        12
   count (private) 431   <- via count(), not s.count
   s.count would be E0616: field `count` of struct `Sealed` is private
```
<!-- /output -->

## See also

- [STRUCTS.md](../../STRUCTS.md) · [`impl` blocks](../impl_blocks/README.md) · [the newtype](../newtype_score/README.md) · [`Option` fields](../../17_Option_and_Result/option_fields/README.md) · [Debug and Display](../../15_First_Programs/debug_vs_display/README.md)

## Po polsku

Struktura to nazwa dla grupy wartości, która sama staje się **typem** — i nic ponadto, bo **zachowania nie trzyma żadnego**. Dane siedzą w `struct`, funkcje w bloku `impl`, a zachowanie wspólne dla wielu typów w cesze (*trait*); dlatego w Ruscie nie ma klas ani dziedziczenia. Polskiego czytelnika najłatwiej zmyli tu samo słowo, bo „strukturę” większość z nas poznała w C, gdzie jest ona rekordem o ustalonym układzie w pamięci. Rustowa struktura to co innego: **układ pól jest domyślnie nieokreślony** (kompilator może je poprzestawiać — jeśli potrzebujesz gwarancji, żądasz jej przez `#[repr(C)]`), pola są **domyślnie prywatne**, a przypisanie **przenosi własność**, o ile typ nie jest `Copy`.

Odmiany są trzy plus jedna pisownia na doczepkę: struktura nazwana (`Ballot { voter: String }`, pola po nazwie), **struktura krotkowa** (`Precinct(u32)`, pola po numerze — `Precinct { 0: 7 }` kompiluje się i równa `Precinct(7)`, bo krotkowa to po prostu nazwana struktura, której pola nazywają się cyframi) oraz **pusta struktura** (`struct Sealed;`), która nie mieści nic, więc jedyne, co może nieść, to zachowanie — i po to właśnie się ją pisze. Uwaga na czwartą pisownię: `struct AlsoEmpty {}` też nie ma pól, ale **pustą strukturą nie jest** i przy użyciu domaga się klamer, inaczej dostaniesz `E0423` z podpowiedzią „use struct literal syntax instead: `AlsoEmpty {}`”. Przy okazji widać wtedy rzecz, o której podręczniki zwykle nie wspominają: `struct Sealed;` deklaruje **dwie** rzeczy o jednej nazwie — typ i stałą wartość — bo Rust prowadzi osobne przestrzenie nazw dla typów i dla wartości. Alias `type Alias = Sealed;` kopiuje tylko tę pierwszą, więc `Alias {}` przejdzie, a samo `Alias` już nie.

Nazwa typu jest w Ruscie strażnikiem, bo typowanie jest **nominalne**, a nie strukturalne: `Color(i32, i32, i32)` i `Point(i32, i32, i32)` mają identyczny kształt i mimo to nie da się jednego podać tam, gdzie oczekiwany jest drugi — `E0308`, podczas gdy goła krotka `(i32, i32, i32)` przyjęłaby oba bez słowa. Prywatność, jak zwykle w Ruscie, obowiązuje na poziomie **modułu**, a nie typu, i w strukturze krotkowej ma dodatkowy skutek: prywatne pole czyni prywatnym sam **konstruktor** (`E0603`, „a constructor is private if any of the fields is private”) — na tym stoi cały wzorzec newtype. Na koniec drobiazg dla piszących po polsku: typy nazywamy w `UpperCamelCase`, pola w `snake_case` (pilnują tego linty `non_camel_case_types` i `non_snake_case`), a polskie znaki diakrytyczne w identyfikatorach są dozwolone — `struct Głos { wyborca: String }` kompiluje się bez mrugnięcia (sprawdzone na rustc 1.98, edycja 2024). Praktyka jest jednak inna: kod trzyma się angielskiego, bo po angielsku są komunikaty, dokumentacja i wyniki wyszukiwania.

**Szukaj po polsku:** struktura krotkowa · pusta struktura · prywatność na poziomie modułu · `rust tuple struct vs tuple E0308` · `rust repr(C) struct layout`
