# *Rust: The Practical Guide*, chapter 6 exercises, run

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 6 exercises**

**Level:** 101 → 201 · a companion to *Rust: The Practical Guide*, chapter 6

**One line:** Section 6.7 of *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) sets five short module exercises and §6.8 answers them. Every exercise fails as printed for the reason the book intends, every book solution compiles on rustc 1.98.0, and four of the five compile with warnings the book does not mention. Two have a better answer than the book's: exercise 4 wants a constructor and getters rather than three `pub` fields, and exercise 5's `self::` is not needed.

| # | Exercise | What rustc says about the given code | The book's solution | Verdict | Lesson |
|---|---|---|---|---|---|
| 1 | [Fixing visibility in nested modules](#1-fixing-visibility-in-nested-modules) | `E0603` *enum `D` is private* | `pub enum D`: compiles, two `dead_code` warnings | right; the explanation is the rule of [privacy running one way](../../../27_Modules/modules_and_visibility/README.md#privacy-runs-one-way) | [Modules and visibility](../../../27_Modules/modules_and_visibility/README.md) |
| 2 | [Module visibility and paths](#2-module-visibility-and-paths) | `E0603` *module `m2` is private* | `pub mod m2`: compiles, three `dead_code` warnings | right, and rustc's label says why: *enum `D` is not publicly re-exported* | [Modules and visibility](../../../27_Modules/modules_and_visibility/README.md) |
| 3 | [Module import and function usage](#3-module-import-and-function-usage) | `E0433` *cannot find type `Season`* and `E0425` *cannot find function `is_holiday`* | `use seasons::{is_holiday, Season};`: compiles, one `dead_code` warning | right; a full path with no `use` works too | [Bringing names in with `use`](../../../27_Modules/the_use_declaration/README.md) |
| 4 | [Access to private fields](#4-access-to-private-fields) | `E0616` *field `name` of struct `Student` is private*, twice | `pub` on three fields: compiles with `non_snake_case`, `unused_mut` and `dead_code` warnings | compiles; the better fix keeps the fields private | [A score is not a number](../../../16_Structs/newtype_score/README.md) |
| 5 | [Re-exporting functions](#5-re-exporting-functions) | `E0432` *unresolved import `___`*, twice | `pub use self::display::show_area;` and its twin: compiles clean | right; `self::` can go, and one `use` line does the job of two | [Bringing names in with `use`](../../../27_Modules/the_use_declaration/README.md) |

The exercise statements are paraphrased; the code is the book's. Each transcript is rustc 1.98.0 compiling a file with the name in the fence's title, edition 2024, and each solution is a program this library compiles and runs.

## Practice

### 1. Fixing visibility in nested modules

`A` in `m1` holds a `D` from the child module `m2`. Make it compile with the smallest change:

```rust,compile_fail
mod m1 {
    struct A {
        d: m2::D,
    }
    mod m2 {
        enum D {
            B,
            C,
        }
    }
}
fn main() {}
```

```text title="Abridged — real rustc output for exercise_1.rs"
error[E0603]: enum `D` is private
 --> exercise_1.rs:3:16
  |
3 |         d: m2::D,
  |                ^ private enum
  |
note: the enum `D` is defined here
 --> exercise_1.rs:6:9
  |
6 |         enum D {
  |         ^^^^^^
```

The book's answer is `pub enum D`, and its explanation — *child module items are not visible to the parent module* — is the rule. `m2` itself needs no `pub`: `A` sits in `m2`'s parent, which can see the module, and only the enum inside it was private. As printed, the book's solution compiles with two warnings, *struct `A` is never constructed* and *enum `D` is never used*, because `fn main() {}` uses nothing. The solution below adds a function inside `m1` that builds two `A`s, so it prints something and warns about nothing.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg6_01_nested_visibility -->
*[`pg6_01_nested_visibility.rs`](examples/pg6_01_nested_visibility.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 1, solved: `pub enum D`, which is the book's whole fix, plus a
//! function that builds an `A`, so the solution compiles without the two
//! dead_code warnings the book's version prints.
//!
//!   rustc --edition 2024 pg6_01_nested_visibility.rs -o /tmp/pg601 && /tmp/pg601

mod m1 {
    struct A {
        d: m2::D,
    }

    mod m2 {
        pub enum D {
            B,
            C,
        }
    }

    /// `A` and `m2` stay private to `m1`; this is the one door out.
    pub fn describe_both() -> String {
        let first = A { d: m2::D::B };
        let second = A { d: m2::D::C };
        format!("{} and {}", name(&first), name(&second))
    }

    fn name(a: &A) -> &'static str {
        match a.d {
            m2::D::B => "A holding D::B",
            m2::D::C => "A holding D::C",
        }
    }
}

fn main() {
    println!("{}", m1::describe_both()); // A holding D::B and A holding D::C
    println!("D is pub, so m1 can name m2::D; m2 itself stays private to m1");
}
```
<!-- /source -->

<!-- output:pg6_01_nested_visibility -->
*Verified output of [`pg6_01_nested_visibility.rs`](examples/pg6_01_nested_visibility.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
A holding D::B and A holding D::C
D is pub, so m1 can name m2::D; m2 itself stays private to m1
```
<!-- /output -->

</details>

### 2. Module visibility and paths

`D` is already `pub`. A struct in a third module, `m3`, names it by its full path and still cannot reach it:

```rust,compile_fail
mod m1 {
    struct A {
        d: m2::D,
    }
    mod m2 {
        pub enum D {
            B,
            C,
        }
    }
}
mod m3 {
    struct C {
        e: crate::m1::m2::D,
    }
}
fn main() {}
```

```text title="Abridged — real rustc output for exercise_2.rs"
error[E0603]: module `m2` is private
  --> exercise_2.rs:14:23
   |
14 |         e: crate::m1::m2::D,
   |                       ^^  - enum `D` is not publicly re-exported
   |                       |
   |                       private module
   |
note: the module `m2` is defined here
  --> exercise_2.rs:5:5
   |
 5 |     mod m2 {
   |     ^^^^^^
```

The book's answer is `pub mod m2`, with the right explanation: public items of a private child module can be reached by the parent and nobody else, so from `m3` the path `crate::m1::m2::D` runs into a wall at `m2`. rustc's second label, *enum `D` is not publicly re-exported*, names the other way out: `pub use m2::D;` in `m1` would let `m3` write `crate::m1::D` with `m2` still private. That is the pattern the chapter's own store settles on in §6.4. The book's solution compiles with three `dead_code` warnings, one per unused struct and enum; the solution below builds one of each.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg6_02_module_paths -->
*[`pg6_02_module_paths.rs`](examples/pg6_02_module_paths.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 2, solved: `pub mod m2`, the book's fix, so that `m3` can reach
//! `crate::m1::m2::D`. Both structs get a function that builds one.
//!
//!   rustc --edition 2024 pg6_02_module_paths.rs -o /tmp/pg602 && /tmp/pg602

mod m1 {
    struct A {
        d: m2::D,
    }

    pub mod m2 {
        #[derive(Debug)]
        pub enum D {
            B,
            C,
        }
    }

    pub fn build_a() -> String {
        let a = A { d: m2::D::B };
        format!("m1::A {{ d: {:?} }}", a.d)
    }
}

mod m3 {
    struct C {
        e: crate::m1::m2::D,
    }

    pub fn build_c() -> String {
        let c = C { e: crate::m1::m2::D::C };
        format!("m3::C {{ e: {:?} }}", c.e)
    }
}

fn main() {
    println!("{}", m1::build_a()); // m1::A { d: B }
    println!("{}", m3::build_c()); // m3::C { e: C }
    println!("m3 reaches D only because m2 is pub AND D is pub: two doors, both open");
}
```
<!-- /source -->

<!-- output:pg6_02_module_paths -->
*Verified output of [`pg6_02_module_paths.rs`](examples/pg6_02_module_paths.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
m1::A { d: B }
m3::C { e: C }
m3 reaches D only because m2 is pub AND D is pub: two doors, both open
```
<!-- /output -->

</details>

### 3. Module import and function usage

`main` uses `Season` and `is_holiday` as bare names, and both live in `seasons`:

```rust,compile_fail
mod seasons {
    pub enum Season {
        Spring,
        Summer,
        Autumn,
        Winter,
    }

    pub fn is_holiday(season: &Season) -> bool {
        match season {
            Season::Summer => true,
            _ => false,
        }
    }
}

fn main() {
    let current_season = Season::Autumn;
    if is_holiday(&current_season) {
        println!("It's a holiday season! Time for a vacation!");
    } else {
        println!("Regular work season. Keep hustling!");
    }
}
```

```text title="Abridged — real rustc output for exercise_3.rs"
error[E0433]: cannot find type `Season` in this scope
  --> exercise_3.rs:18:26
   |
18 |     let current_season = Season::Autumn;
   |                          ^^^^^^ use of undeclared type `Season`
   |
help: consider importing this enum
   |
 1 + use crate::seasons::Season;
   |

error[E0425]: cannot find function `is_holiday` in this scope
  --> exercise_3.rs:19:8
   |
19 |     if is_holiday(&current_season) {
   |        ^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
 1 + use crate::seasons::is_holiday;
   |
```

Two errors with two codes: a missing type is `E0433` here, a missing function `E0425`, and rustc's help writes the fix twice, one `use` line each. The book's single line, `use seasons::{is_holiday, Season};`, does both, and the program then prints `Regular work season. Keep hustling!` — the book's expected output for `Autumn`. A `use` is not the only fix: `seasons::Season::Autumn` and `seasons::is_holiday(..)` written in full compile with no `use` at all, and the solution below runs one call that way. The book's solution compiles with one warning, *variants `Spring`, `Summer`, and `Winter` are never constructed*, since only `Autumn` is ever built; the solution loops over all four.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg6_03_use_seasons -->
*[`pg6_03_use_seasons.rs`](examples/pg6_03_use_seasons.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 3, solved: one `use` line brings `Season` and `is_holiday` into
//! `main`'s scope. Every season is checked, so no variant is dead.
//!
//!   rustc --edition 2024 pg6_03_use_seasons.rs -o /tmp/pg603 && /tmp/pg603

mod seasons {
    #[derive(Debug)]
    pub enum Season {
        Spring,
        Summer,
        Autumn,
        Winter,
    }

    pub fn is_holiday(season: &Season) -> bool {
        match season {
            Season::Summer => true,
            _ => false,
        }
    }
}

use seasons::{is_holiday, Season};

fn main() {
    let current_season = Season::Autumn;
    if is_holiday(&current_season) {
        println!("It's a holiday season! Time for a vacation!");
    } else {
        println!("Regular work season. Keep hustling!"); // this one
    }

    // The same call with no `use` at all: a full path works everywhere.
    println!("{}", seasons::is_holiday(&seasons::Season::Summer)); // true

    for season in [Season::Spring, Season::Summer, Season::Autumn, Season::Winter] {
        println!("{season:?}: holiday = {}", is_holiday(&season));
    }
}
```
<!-- /source -->

<!-- output:pg6_03_use_seasons -->
*Verified output of [`pg6_03_use_seasons.rs`](examples/pg6_03_use_seasons.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Regular work season. Keep hustling!
true
Spring: holiday = false
Summer: holiday = true
Autumn: holiday = false
Winter: holiday = false
```
<!-- /output -->

</details>

### 4. Access to private fields

`Student` is public, its three fields are not, and `main` both builds one with a struct literal and reads two fields:

```rust,compile_fail
mod University {
    pub struct Student {
        name: String,
        marks: u8,
        grade: char,
    }
}

use University::Student;

fn main() {
    let mut student_1 = Student {
        name: String::from("Alice"),
        marks: 75,
        grade: 'A',
    };
    println!("{} got {} grade", student_1.name, student_1.grade);
}
```

```text title="Abridged — real rustc output for exercise_4.rs"
error[E0616]: field `name` of struct `Student` is private
  --> exercise_4.rs:17:43
   |
17 |     println!("{} got {} grade", student_1.name, student_1.grade);
   |                                           ^^^^ private field

error[E0616]: field `grade` of struct `Student` is private
  --> exercise_4.rs:17:59
   |
17 |     println!("{} got {} grade", student_1.name, student_1.grade);
   |                                                           ^^^^^ private field
```

rustc reports the two reads and stops. The struct literal on the lines above them is refused too, but only once the reads are gone: with the `println!` deleted, the same file is one `E0451`, *fields `name`, `marks` and `grade` of struct `Student` are private*, naming all three. The book's answer makes the three fields `pub`, and the program prints `Alice got A grade`. It also prints three warnings, all of them about the book's own lines:

```text title="Abridged — real rustc output for exercise_4_solution.rs"
warning: variable does not need to be mutable
  --> exercise_4_solution.rs:12:9
   |
12 |     let mut student_1 = Student {
   |         ----^^^^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: field `marks` is never read
 --> exercise_4_solution.rs:4:13
  |
2 |     pub struct Student {
  |                ------- field in this struct
3 |         pub name: String,
4 |         pub marks: u8,
  |             ^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: module `University` should have a snake case name
 --> exercise_4_solution.rs:1:5
  |
1 | mod University {
  |     ^^^^^^^^^^ help: convert the identifier to snake case (notice the capitalization): `university`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: 3 warnings emitted
```

`mod University` should be `mod university`; `let mut` binds a value that is never changed; and `marks` is written and never read, which is the warning that says something about the design — a grade that lives beside the marks that decide it can disagree with them, and three `pub` fields let any caller make it disagree. The solution below keeps every field private, computes the grade from the marks in `Student::new`, hands the values out through getters, and drops the `mut`. That is [the newtype's door](../../../16_Structs/newtype_score/README.md#one-door-checked-once) at struct size, and it is what §6.4.2 of the chapter arrives at for `Product` two pages after this exercise's `pub` fields.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg6_04_private_fields -->
*[`pg6_04_private_fields.rs`](examples/pg6_04_private_fields.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 4, solved the other way: the fields stay private, a constructor
//! computes the grade from the marks, and getters hand the values out. The
//! module is snake_case, the binding is not `mut`, and `marks` is read.
//!
//!   rustc --edition 2024 pg6_04_private_fields.rs -o /tmp/pg604 && /tmp/pg604

mod university {
    pub struct Student {
        name: String,
        marks: u8,
        grade: char,
    }

    impl Student {
        pub fn new(name: String, marks: u8) -> Self {
            let grade = match marks {
                90..=100 => 'A',
                75..=89 => 'B',
                50..=74 => 'C',
                _ => 'F',
            };
            Self { name, marks, grade }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn marks(&self) -> u8 {
            self.marks
        }

        pub fn grade(&self) -> char {
            self.grade
        }
    }
}

use university::Student;

fn main() {
    let student_1 = Student::new(String::from("Alice"), 75);
    println!("{} got {} grade", student_1.name(), student_1.grade()); // Alice got B grade
    println!("{} marks, and no way to write a grade that does not match them", student_1.marks()); // 75 marks, ...
}
```
<!-- /source -->

<!-- output:pg6_04_private_fields -->
*Verified output of [`pg6_04_private_fields.rs`](examples/pg6_04_private_fields.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Alice got B grade
75 marks, and no way to write a grade that does not match them
```
<!-- /output -->

</details>

### 5. Re-exporting functions

`graphics` holds `shapes::calculate_area` and `display::show_area` in two public submodules. Fill in the two `use` lines so that `main` can call both by their short names, and add the re-exports the comments ask for:

```rust,compile_fail
mod graphics {
    // Re-export the 'show_area' function for easier access
    // Re-export the 'calculate_area' function for easier access

    pub mod shapes {
        pub fn calculate_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }

    pub mod display {
        pub fn show_area(shape: &str, area: f64) {
            println!("The area of the {} is: {}", shape, area);
        }
    }
}

use ___::calculate_area; // fix this line
use ___::show_area; // fix this line

fn main() {
    let radius = 3.0;
    let area = calculate_area(radius);
    show_area("circle", area);
}
```

```text title="Abridged — real rustc output for exercise_5.rs"
error[E0432]: unresolved import `___`
  --> exercise_5.rs:18:5
   |
18 | use ___::calculate_area; // fix this line
   |     ^^^ use of unresolved module or unlinked crate `___`
   |
   = help: you might be missing a crate named `___`

error[E0432]: unresolved import `___`
  --> exercise_5.rs:19:5
   |
19 | use ___::show_area; // fix this line
   |     ^^^ use of unresolved module or unlinked crate `___`
   |
   = help: you might be missing a crate named `___`
```

`___` is a legal identifier, so the placeholder is refused as an unknown crate rather than as a syntax error. The book's answer puts `pub use self::display::show_area;` and `pub use self::shapes::calculate_area;` inside `graphics` and then writes `use graphics::calculate_area;` and `use graphics::show_area;`. It compiles clean and prints `The area of the circle is: 28.274333882308138`. The `self::` is not needed: since the 2018 edition a `use` path starts from the current module, so `pub use display::show_area;` inside `graphics` means the same thing, and the solution below writes it that way, on edition 2024, with the two root-level lines folded into `use graphics::{calculate_area, show_area};`. The re-exports add names; they move nothing, and the long paths keep working.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg6_05_reexports -->
*[`pg6_05_reexports.rs`](examples/pg6_05_reexports.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 5, solved: two `pub use` lines in `graphics`, written without the
//! `self::` the book's solution uses, and one `use` line in place of two.
//!
//!   rustc --edition 2024 pg6_05_reexports.rs -o /tmp/pg605 && /tmp/pg605

mod graphics {
    pub use display::show_area;
    pub use shapes::calculate_area;

    pub mod shapes {
        pub fn calculate_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }

    pub mod display {
        pub fn show_area(shape: &str, area: f64) {
            println!("The area of the {} is: {}", shape, area);
        }
    }
}

use graphics::{calculate_area, show_area};

fn main() {
    let radius = 3.0;
    let area = calculate_area(radius);
    show_area("circle", area); // The area of the circle is: 28.274333882308138

    // The long paths still work; the re-exports added names, they moved nothing.
    graphics::display::show_area("same circle", graphics::shapes::calculate_area(radius));
}
```
<!-- /source -->

<!-- output:pg6_05_reexports -->
*Verified output of [`pg6_05_reexports.rs`](examples/pg6_05_reexports.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The area of the circle is: 28.274333882308138
The area of the same circle is: 28.274333882308138
```
<!-- /output -->

</details>

## If you are coming from another language

- **Python.** Exercises 1, 2 and 4 have no Python form: a module's names and a class's attributes are all reachable, and `_name` is a request rather than a rule. Exercise 3 is `from seasons import Season, is_holiday` against `seasons.Season.AUTUMN`, the same two spellings with the same trade-off, minus the difference that Python's `import` runs the module and Rust's `use` runs nothing. Exercise 5 is what `__init__.py` does with `from .display import show_area`, which is a re-export in the same sense, so that a user writes `graphics.show_area`. Rust adds that the re-export is the *only* way to reach an item in a private module; Python's re-export is a courtesy.
- **Java.** Exercise 4 is the one Java programmers solve by reflex, and the reflex is right: private fields, a constructor, getters. The difference is where the wall stands — Rust's boundary is the module, so a function written inside `university`, next to the struct, can read `marks` without a getter, where Java's `private` shuts out everything but the class. Exercises 1 and 2 have no Java form, since a Java class in a nested package is either `public` to everyone or package-private to one directory; the "visible to the parent but not to a cousin" rule of exercise 2 needs a tree, which Java packages do not have.
- **C.** Exercise 5 is a header that includes two others, so a user includes one; exercise 4 is an opaque struct, a `typedef struct student student;` in the header with the definition in the `.c` file, which is the strongest privacy C has and the one this library's C pages reach for. What C cannot express is exercise 1: a `static` function is private to its file, and there is no "private to this file and the files below it".

## See also

- [What *Rust: The Practical Guide* says about packages, crates and modules, run](../../../27_Modules/modules_claims_checked/README.md) — the chapter the exercises belong to, checked listing by listing
- [*Rust: The Practical Guide*, run](../README.md) — the book's home in this library
- [Modules and visibility](../../../27_Modules/modules_and_visibility/README.md) — the rule behind exercises 1, 2 and 4
- [Bringing names in with `use`](../../../27_Modules/the_use_declaration/README.md) — exercises 3 and 5
- [A crate prelude](../../../27_Modules/a_crate_prelude/README.md) — a whole module of the `pub use` lines exercise 5 writes two of
- [A score is not a number](../../../16_Structs/newtype_score/README.md) — the constructor-and-getters answer to exercise 4, and why it beats `pub` fields
- [What an attribute is](../../../27_Modules/what_an_attribute_is/README.md) — the lint names in the warnings above, and what `#[allow]` would silence
- [Katas](../../../KATAS.md) — where these five sit in the practice track

## Po polsku

Pięć ćwiczeń z rozdziału 6 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) sprawdza widoczność (*visibility*) w zagnieżdżonych modułach, ścieżki (*paths*), deklarację `use`, prywatne pola i re-eksport (*re-export*) funkcji. Każdy kod z treści zadania nie kompiluje się z powodu, o który autorowi chodziło: prywatne wyliczenie (`E0603`), prywatny moduł na ścieżce (`E0603`), nazwy bez `use` (`E0433` i `E0425`), odczyt prywatnych pól (`E0616`) oraz `___` w miejscu ścieżki (`E0432`, bo `___` to poprawny identyfikator). Wszystkie rozwiązania z książki kompilują się na rustc 1.98.0, ale cztery z pięciu z ostrzeżeniami: `dead_code` dla nieużywanych typów, a w ćwiczeniu 4 dodatkowo `non_snake_case` dla `mod University`, `unused_mut` i nieczytane pole `marks`. To ostatnie ostrzeżenie mówi o projekcie: ocena zapisana obok punktów może się z nimi rozminąć, więc lepsze rozwiązanie zostawia pola prywatne, liczy ocenę w konstruktorze i udostępnia wartości przez gettery. W ćwiczeniu 5 przedrostek `self::` w `pub use self::display::show_area;` jest zbędny — od edycji 2018 ścieżka w `use` zaczyna się w bieżącym module.

**Szukaj po polsku:** ćwiczenia z modułów w Ruście · widoczność modułów zagnieżdżonych · re-eksport funkcji `pub use` · prywatne pola struktury · `rust E0603 private enum` · `rust E0616 private field` · `rust E0432 unresolved import` · `rust pub use self`
