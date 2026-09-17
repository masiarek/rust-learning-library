# An array in a `const` or a `static`

[Arrays: the map](../README.md) › **Lesson 4** · previous: [Where an array lives](../where_an_array_lives/README.md) · then: [What array explanations get wrong, run](../array_claims_checked/README.md)

**Level:** 201 · working knowledge

**One line:** A `const` array is a value substituted wherever it is named; a `static` array is one place in the binary for the whole run. Put a table of records with borrowed strings in a `static`, and the `'static` in its type does not say the table lives forever — the `static` keyword says that. It says the borrows inside each record do.

```rust
#[derive(Debug)]
pub enum Dimension {
    Length,
    Mass,
}

pub struct Unit<'a> {
    pub names: [&'a str; 2],
    pub dimension: Dimension,
}

pub static UNITS: [Unit<'static>; 2] = [
    Unit { names: ["m", "meter"], dimension: Dimension::Length },
    Unit { names: ["g", "gram"], dimension: Dimension::Mass },
];

fn find(name: &str) -> Option<&'static Unit<'static>> {
    UNITS.iter().find(|u| u.names.contains(&name))
}

fn main() {
    println!("{:?}", find("gram").map(|u| &u.dimension)); // Some(Mass)
}
```

This is a *static array*: a lookup table built when the program is compiled, with nothing to initialise at run time.

## `const` or `static`

| | `const TABLE: [T; N]` | `static TABLE: [T; N]` |
|---|---|---|
| what the name is | a value, substituted at each use | one place in memory |
| address | not guaranteed to be the same twice | one address for the whole run |
| a big array | may be copied into each place that uses it | stored once |

For a table you look things up in, `static` is the usual choice. Clippy's [`large_const_arrays`](../array_lints/README.md#large_const_arrays) says so for any `const` array over 16,384 bytes. [`const` and `static`](../../../27_Modules/const_and_static/README.md) covers both keywords in full, including what `&CONST` gives you.

## What `'static` says in `Unit<'static>`

An explanation of a table like this said *"the `'static` lifetime means that the data in the `BaseAtom` instance can live for the entire duration of the program"*. Two things are mixed together there:

- **`UNITS` lives for the whole program because it is a `static` item.** That is the keyword's job, and it would be true of a `static` holding no references at all.
- **`'static` in `Unit<'static>` is about the `&str`s inside:** each one must be valid for the whole program. String literals are, so the table compiles.

A `Unit<'static>` value does not have to live forever. [The program](#the-verified-output) builds one as a local inside a block, passes it to a function requiring `T: 'static`, and it is dropped at the end of the block like any other local. A `String` local passes the same test. `T: 'static` means *T holds no borrow shorter than the program*, not *this value is never freed*.

The same explanation called `names` "an array with a single element, the string "meter"". The element is a `&'static str`, a borrow of text stored in the binary, not a `String`. `type_name_of_val(&UNITS[0].names[1])` prints `&str`. A `String` could not be built there at all: `String::from` is not a `const fn`, so `[String::from("meter")]` in a `static` is [`E0015`](../array_errors/README.md#20-a-string-built-in-a-static).

When a function says `Unit<'static>`, borrowing a local `String` is refused: [`E0597`, *argument requires that `name` is borrowed for `'static`*](../array_errors/README.md#29-a-borrowed-local-where-static-is-required).

## Where the lifetime has to be written

| Place | Lifetime | What happens without it |
|---|---|---|
| the struct: `struct Unit<'a> { names: [&'a str; 2] }` | **required** | [`E0106`: missing lifetime specifier](../array_errors/README.md#28-a-str-field-with-no-lifetime) |
| the static's type: `static UNITS: [Unit<'static>; 3]` | optional | `[Unit; 3]` compiles and means `Unit<'static>` |
| a `&str` in a static: `static NAMES: [&str; 2]` | optional | means `&'static str` |

So *"the lifetime has to be spelled out in a static's type"* is false on 1.98.0. The Reference's [const and static elision ↗](https://doc.rust-lang.org/reference/lifetime-elision.html#const-and-static-elision) rule makes a left-out lifetime in a `static` or `const` type `'static`, since RFC 1623. Writing `[Unit; 3]` does draw one allow-by-default lint when you turn it on, `elided_lifetimes_in_paths`, with *"hidden lifetime parameters in types are deprecated"*, and suggests `Unit<'_>`, which also compiles. Spelling out `Unit<'static>` keeps the reader from having to know the rule.

## A `const` array, and a local that changes its contents

```rust
const MY_DATA: [i8; 3] = [1, 2, 3];

fn main() {
    let mut my_data = [2; 3];
    my_data[0] = 1;
    my_data[2] = 3;
    assert_eq!(MY_DATA, my_data);
    println!("{}", MY_DATA[..] == [1i8, 2, 3, 4][..]); // false
}
```

`mut` let the program overwrite two elements. It cannot change the length, because nothing on `[i8; 3]` adds a fourth: the size of a fixed-size array is part of its type, `mut` or not. For the same reason, comparing `MY_DATA` with a `[i8; 4]` is not `false` but a compile error, [`E0277`: can't compare `[i8; 3]` with `[i8; 4]`](../array_errors/README.md#3-comparing-arrays-of-two-lengths). Compare slices, `MY_DATA[..] == MY_DATA_4[..]`, when lengths may differ.

"Because the size is known at compile time, a fixed-size array can be stored anywhere, even as a const" is right with one condition: every element has to be computable at compile time, which is why the table holds `&str` and not `String`. A `Vec` cannot be a `const` for the same reason, not because of its size.

## If you are coming from another language

- **C.** `static const char *const names[] = {"m", "g"};` is the same table, and C's `static` inside a function or at file scope is also one place for the whole run. C types carry no lifetimes, so the rule `E0597` enforces is left to the programmer.
- **Go.** Go has no constant arrays: `const units = [3]string{"m", "g", "s"}` is refused with *"is not constant"*. The idiom is a package-level `var`, which any code in the package can change.
- **Java.** `static final String[] UNITS = {"m", "g"};` fixes the reference, not the contents: `UNITS[0] = "x";` compiles and runs. A Rust `static` array of plain values cannot be written to without `static mut` and `unsafe`.
- **Python.** A module-level tuple of tuples is the nearest thing: the tuples cannot be changed, but the name can be rebound. Python has no counterpart to the `const`/`static` split, since every name refers to one object.

---

## The verified output

<!-- output:static_arrays -->
*Verified output of [`static_arrays.rs`](examples/static_arrays.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A table of records in a static
   ["m", "meter"] -> Length
   ["g", "gram"] -> Mass
   ["s", "second"] -> Time
   find("gram") = Some(Mass)
   find("inch") = None

2. What is in `names`
   UNITS[0].names    is [&str; 2]
   UNITS[0].names[1] is &str = "meter": a borrowed literal, not a String
   UNITS_ELIDED, written [Unit; 1], holds ["m", "meter"]

3. A Unit<'static> that does not live forever
   inside a block: ["h", "hour"]
   wants_static(&local) -> accepted
   the block ended and `local` is gone; UNITS is still here: ["s", "second"]
   a String local is 'static too: wants_static(&text) -> accepted
   T: 'static means T holds no short borrows, not that the value is immortal

4. A const array, and a local that changes its contents
   MY_DATA = [1, 2, 3], my_data = [1, 2, 3], equal: true
   `mut` let us overwrite two elements; no method on [i8; 3] adds a fourth
   MY_DATA[..] == MY_DATA_4[..] -> false  (as arrays: E0277, can't compare)
   MY_DATA == MY_DATA_4[..3]    -> true

5. const or static
   a const is a value pasted in at each use; a static is one place
   find("m") points into UNITS itself, no copy: true
   size_of_val(&UNITS) = 120 bytes: 3 x (two &str of 16 + a 1-byte enum, padded)
```
<!-- /output -->

## Practice

**A static lookup table, checked while it compiles.** Write `static REASONS: [(u16, &str); 6]` holding six HTTP status codes and their reason phrases, sorted by code, and `fn reason(code: u16) -> Option<&'static str>` that finds one with `binary_search_by_key`. Then explain why the return type can say `'static` when the parameter is not a reference at all.

Binary search is only right if the table is sorted, and nothing checks that. Write a `const fn` that checks it, and call it from `const _: () = assert!(...)`, so that swapping two rows stops the build.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:static_arrays_kata -->
*[`static_arrays_kata.rs`](examples/static_arrays_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a static lookup table, checked for order while it compiles.
//!
//!   rustc --edition 2024 static_arrays_kata.rs -o /tmp/sak && /tmp/sak

/// Sorted by code, so a lookup can binary-search it.
static REASONS: [(u16, &str); 6] = [
    (200, "OK"),
    (201, "Created"),
    (301, "Moved Permanently"),
    (404, "Not Found"),
    (418, "I'm a teapot"),
    (500, "Internal Server Error"),
];

/// `binary_search_by_key` is not a `const fn`, so the order check is a loop.
const fn strictly_sorted(table: &[(u16, &str)]) -> bool {
    let mut i = 1;
    while i < table.len() {
        if table[i - 1].0 >= table[i].0 {
            return false;
        }
        i += 1;
    }
    true
}

// Evaluated while compiling: swap two rows above and the build fails.
const _: () = assert!(strictly_sorted(&REASONS), "REASONS must be sorted by code");

/// The answer borrows from the table, not from `code`, so it can be `'static`.
fn reason(code: u16) -> Option<&'static str> {
    REASONS
        .binary_search_by_key(&code, |&(c, _)| c)
        .ok()
        .map(|i| REASONS[i].1)
}

fn main() {
    println!("1. Lookups");
    for code in [200, 404, 418, 302] {
        println!("   reason({code}) = {:?}", reason(code));
    }

    println!();
    println!("2. Why the return type may say 'static");
    let found: &'static str = reason(301).unwrap();
    println!("   {found:?} is a borrow of REASONS, which lives for the whole run;");
    println!("   `code` is a u16 copied in, so there is no input borrow to tie it to");

    println!();
    println!("3. The order check ran before main did");
    println!("   strictly_sorted(&REASONS) = {}", strictly_sorted(&REASONS));
    println!("   with rows 404 and 301 swapped, rustc stops with E0080:");
    println!("   evaluation panicked: REASONS must be sorted by code");
}
```
<!-- /source -->

<!-- output:static_arrays_kata -->
*Verified output of [`static_arrays_kata.rs`](examples/static_arrays_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Lookups
   reason(200) = Some("OK")
   reason(404) = Some("Not Found")
   reason(418) = Some("I'm a teapot")
   reason(302) = None

2. Why the return type may say 'static
   "Moved Permanently" is a borrow of REASONS, which lives for the whole run;
   `code` is a u16 copied in, so there is no input borrow to tie it to

3. The order check ran before main did
   strictly_sorted(&REASONS) = true
   with rows 404 and 301 swapped, rustc stops with E0080:
   evaluation panicked: REASONS must be sorted by code
```
<!-- /output -->

</details>

## See also

- [Arrays: the map](../README.md) — every array page, in reading order
- [`const` and `static`](../../../27_Modules/const_and_static/README.md) — the two keywords in full, with the addresses measured
- [`&'static str`](../../../14_Strings/static_str/README.md) — the element type of every table on this page, and what `'static` means as a bound
- [Lifetime annotations](../../../18_Ownership/lifetime_annotations/README.md) — why the struct needs `'a` at all
- [Where an array lives](../where_an_array_lives/README.md) — the `static` is one of four homes

## Sources

[The Reference — static items ↗](https://doc.rust-lang.org/reference/items/static-items.html) and [constant items ↗](https://doc.rust-lang.org/reference/items/constant-items.html); [const and static elision ↗](https://doc.rust-lang.org/reference/lifetime-elision.html#const-and-static-elision); [RFC 1623, `'static` in statics and consts ↗](https://rust-lang.github.io/rfcs/1623-static.html).

## Po polsku

Tablica w `const` to wartość wklejana w każdym miejscu użycia, a tablica w `static` to jedno miejsce w pliku binarnym, istniejące przez cały czas działania programu. Tabelę rekordów do wyszukiwania trzyma się zwykle w `static`. Clippy (`large_const_arrays`) ostrzega przed stałą tablicą większą niż 16 384 bajty.

Częsta pomyłka w wyjaśnieniach: `'static` w typie `Unit<'static>` **nie** oznacza, że ta wartość żyje przez cały program. To gwarantuje słowo kluczowe `static`. `'static` w typie mówi tylko, że referencje `&str` w środku są ważne tak długo. Lokalna wartość `Unit<'static>` znika na końcu bloku jak każda inna. Pole `names` to tablica `&'static str`, nie `String`. `String::from` w `static` to błąd `E0015`. Czas życia trzeba zapisać w definicji struktury (`struct Unit<'a>`), ale w typie `static` można go pominąć: `[Unit; 3]` znaczy `[Unit<'static>; 3]`. Stała tablica porównana ze zmienną: `mut` pozwala zmieniać elementy, nigdy długość.

**Szukaj po polsku:** tablica statyczna · static array · stała tablica `const` · `rust static array of structs` · `rust 'static lifetime meaning`
