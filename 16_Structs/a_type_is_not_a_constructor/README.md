# A type is not a constructor

**Level:** 101 → 201 · for newcomers

**One line:** Rust has no `Type()` call that makes a value — `Player { .. }` is a literal, `Player::new()` is a function somebody wrote, and `let p: Player;` names a *type* in the slot where the value was supposed to go.

```rust
struct Player { name: String, score: u8 }
struct Meters(u32);
#[derive(Debug)]
struct Sealed;

fn main() {
    let named = Player { name: String::from("Ada"), score: 5 };
    let tuple = Meters(7);
    let unit = Sealed;
    println!("{} {} {:?}", named.name, tuple.0, unit);  // Ada 7 Sealed
}
```

Three flavors, three spellings, and **only the middle one is a call**.

---

## Where a value comes from

| Flavor | Declared | Built with | Is the bare name a value? |
|---|---|---|---|
| named-field | `struct Player { .. }` | `Player { name, score }` — a literal | no |
| tuple struct | `struct Meters(u32);` | `Meters(7)` — a real function call | **yes**, a `fn(u32) -> Meters` |
| unit struct | `struct Sealed;` | `Sealed` — the value itself | **yes**, and it takes no arguments |

Only the tuple struct gets a function out of its declaration. That is why "constructor" is literal for one flavor and a figure of speech for the other two, and why the same fact shows up twice — once as a refusal, once as a convenience:

```rust
let make: fn(u32) -> Meters = Meters;                                  // it is a function value
let ms: Vec<Meters> = vec![3u32, 7, 12].into_iter().map(Meters).collect();
println!("{:?} {:?}", make(12), ms);  // Meters(12) [Meters(3), Meters(7), Meters(12)]
```

`.map(Player)` is `E0423`. There is nothing to pass.

## `new` is a convention

Nothing in the language has heard of it. `Player::new` is an ordinary [associated function](../impl_blocks/README.md) that happens to return `Self`, and you can call it whatever the domain calls it:

```rust
impl Player {
    fn new(name: &str, score: u8) -> Self { Player { name: name.to_string(), score } }
    fn blank() -> Self { Player { name: String::from("(unnamed)"), score: 0 } }
}
```

Standard library, four names for the same job: `Vec::new()`, `String::from("Cara")`, `u8::default()`, `Player::blank()`. A type with no `new` is not missing anything.

## `:` supplies a type, `=` supplies a value

Only the second is required. This compiles, with no `mut`:

```rust
let high_score = 431;
let chosen: Player;               // declared, not initialized
if high_score > 400 {
    chosen = Player::new("Cara", 4);
} else {
    chosen = Player::blank();
}
println!("{}", chosen.score);     // 4
```

One assignment per path is not a mutation, so the binding stays immutable — and the compiler proves it was set before the read. Miss a path and you get `E0381`. The case where this is the *right* tool rather than an `Option` is [initial values](../../17_Option_and_Result/initial_values/README.md).

## The refusals

Four ways to reach for a value and miss, and they are four different error codes.

**`let b: Player();`** — both mistakes at once, which is why it prints two errors:

```text title="Real rustc output — rustc 1.98.0, edition 2024"
error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
 --> scratch.rs:9:12
  |
9 |     let b: Player();
  |            ^^^^^^^^ only `Fn` traits may use parentheses

error[E0381]: used binding `b` isn't initialized
  --> scratch.rs:10:16
   |
 9 |     let b: Player();
   |         - binding declared here but left uninitialized
10 |     println!("{b:?}");
   |                ^ `b` used here but it isn't initialized
   |
help: consider assigning a value
   |
 9 |     let b: Player() = /* value */;
   |                     +++++++++++++
```

After `:` rustc is reading a **type**, and `Name(A) -> B` in type position is the sugar for the `Fn` traits — `E0214` is the parser saying `Player` is not one of them. `E0381` is separate and downstream: no `=` ever appeared, so nothing was assigned.

**`let b = Player();`** — the value slot, so a different complaint, and it names the fields for you:

```text title="Real rustc output"
error[E0423]: expected function, tuple struct or tuple variant, found struct `Player`
 --> scratch.rs:9:13
  |
2 | struct Player { name: String, score: u8 }
  | ----------------------------------------- `Player` defined here
...
9 |     let b = Player();
  |             ^^^^^^^^ help: use struct literal syntax instead: `Player { name: val, score: val }`
```

**`let s = Sealed();`** — a unit struct is already a value, so the parentheses are a call on something that is not a function:

```text title="Real rustc output"
error[E0618]: expected function, found struct `Sealed`
 --> scratch.rs:9:13
  |
6 | struct Sealed;
  | ------------- struct `Sealed` defined here
...
9 |     let s = Sealed();
  |             ^^^^^^--
  |             |
  |             call expression requires function
  |
help: `Sealed` is a unit struct, and does not take parentheses to be constructed
  |
9 -     let s = Sealed();
9 +     let s = Sealed;
  |
```

**`let p = Meters;`** — the one that **compiles**. `p` is the constructor function, not a `Meters`, and you find out on the next line:

```text title="Real rustc output"
error[E0277]: `fn(u32) -> Meters {Meters}` doesn't implement `Debug`
  --> scratch.rs:10:15
   |
 4 | struct Meters(u32);
   | ------------- consider calling the constructor for `Meters`
...
10 |     println!("{p:?}");
   |               ^^^^^ `fn(u32) -> Meters {Meters}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for fn item `fn(u32) -> Meters {Meters}`
   = help: use parentheses to construct this tuple struct: `Meters(/* u32 */)`
```

The eight refusals of *defining* a struct are a separate page: [when a struct refuses](../when_a_struct_refuses/README.md).

## If you are coming from another language

**Python.** `Person()` calls the class, and calling it is construction — `__call__` on the metaclass runs `__new__` then `__init__`. Rust splits those roles: the type name is a name in the *type* namespace, and construction is a literal (`Player { .. }`) or a function you wrote (`Player::new(..)`). Three transfers to make explicitly:

| | Python | Rust |
|---|---|---|
| build one | `Player(name, score)` — call the class | `Player { name, score }` — a literal |
| the "constructor" | `__init__`, known to the language | `new`, known to nobody; any name works |
| declare without a value | `p: Player` — an annotation, no object | `let p: Player;` — same, and the compiler tracks it |

The last row is the trap that looks harmless. In Python an un-assigned annotation is a comment for a type checker and a `NameError` at runtime if you use it; in Rust `let p: Player;` is a real declaration the borrow checker follows, and reading it early is a *compile* error rather than a runtime one. So the habit transfers — it is the `()` that does not.

And a genuine difference worth keeping: Python has no equivalent of `Meters` *being* a function value. `Player` is not callable at all, while `Meters` can be handed to `.map()` — same declaration keyword, two different namespaces populated.

**ABAP.** The two halves of `let p = Player { .. };` are two statements you already write separately:

```abap
DATA lo_player TYPE REF TO lcl_player.   " the ':' half — a name and a type, no object
CREATE OBJECT lo_player.                  " the '=' half — the value arrives
lo_player->say_hello( ).
```

`let p: Player();` is that first line, with a call bolted onto the type name, and then a method call on a reference that was never filled. ABAP lets you compile it and dumps with `CX_SY_REF_IS_INITIAL` when the third line runs; Rust refuses the build. Two more mappings:

| | ABAP | Rust |
|---|---|---|
| the constructor | `CONSTRUCTOR` method, known to the language | `new`, a convention with no special status |
| a structure with no object | `DATA ls TYPE ty_player` — initialised free | every field supplied, or it does not compile |
| deferred fill | reference stays `IS INITIAL` until `CREATE OBJECT` | `let p: Player;` — and reading it early is `E0381` |

The structural change is the second row. ABAP hands you an initial value for free and lets `IS INITIAL` stand in for "not set yet"; Rust has no such state for a struct, so "not built" and "built" are the only options, and the check moved from your `IF` to the compiler.

---

## Practice

**Four spellings, four error codes — and one of them compiles.**

```rust
struct Player { name: String, level: u8 }
struct Meters(u32);
struct Sealed;
```

1. Predict the refusal for each before compiling — one of the four produces **two** errors, and one produces none:
   `let v: Player();` · `let s = Sealed();` · `let w = Player;` · `let p = Meters;`
2. Write the shortest spelling of each that works.
3. The one that compiled: what type is `p`, and what does the *next* line say when you try to print it?
4. One of the three names can be passed straight to `.map(..)`. Which, and why not the other two?
5. Rewrite one binding as a deferred initialization (`let x: T;`, then assign). Why does it need no `mut`?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_type_is_not_a_constructor_kata -->
*[`a_type_is_not_a_constructor_kata.rs`](examples/a_type_is_not_a_constructor_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four spellings, four error codes — and the one that compiles.
//!
//!   rustc --edition 2024 a_type_is_not_a_constructor_kata.rs -o /tmp/atinack && /tmp/atinack

#[derive(Debug)]
struct Player {
    name: String,
    level: u8,
}

#[derive(Debug)]
struct Meters(u32);

#[derive(Debug)]
struct Sealed;

// The four bindings as they arrive:
//
//     let v: Player();     // 1
//     let s = Sealed();    // 2
//     let w = Player;      // 3
//     let p = Meters;      // 4  <- this one COMPILES
//
// Written correctly below, with what each refusal was actually about.

fn main() {
    println!("Four spellings of 'make me one'. Three refuse, and they refuse differently.\n");

    println!("1. let v: Player();     TWO errors, not one");
    println!("     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait");
    println!("     error[E0381]: used binding `v` isn't initialized");
    println!("   After `:` rustc reads a TYPE. `Player()` in type position is the");
    println!("   `Fn(A) -> B` sugar, and only the Fn traits may use it. Then, separately,");
    println!("   no `=` ever appeared, so nothing was assigned.");
    let v = Player { name: String::from("Ada"), level: 7 };
    println!("   fixed:  let v = Player {{ .. }};       {v:?}\n");

    println!("2. let s = Sealed();    E0618 — expected function, found struct `Sealed`");
    println!("   help: `Sealed` is a unit struct, and does not take parentheses");
    println!("         to be constructed");
    println!("   A unit struct declares a type AND a value of that type sharing one name.");
    let s = Sealed;
    println!("   fixed:  let s = Sealed;               {s:?}\n");

    println!("3. let w = Player;      E0423 — expected value, found struct `Player`");
    println!("   help: use struct literal syntax instead: `Player {{ name: val, level: val }}`");
    println!("   A named-field struct declares ONLY the type. The bare name is not a value.");
    let w = Player { name: String::from("Ben"), level: 3 };
    println!("   fixed:  let w = Player {{ .. }};       {w:?}");
    println!("   (fields read the ordinary way: w.name = {:?}, w.level = {})\n", w.name, w.level);

    println!("4. let p = Meters;      COMPILES — and gives you the wrong thing");
    println!("   `p` is not a Meters. It is the tuple struct's constructor function,");
    println!("   of type `fn(u32) -> Meters {{Meters}}`, and the next line is where");
    println!("   you find out:");
    println!("     error[E0277]: `fn(u32) -> Meters {{Meters}}` doesn't implement `Debug`");
    println!("     help: use parentheses to construct this tuple struct: `Meters(/* u32 */)`");
    let p = Meters(431);
    println!("   fixed:  let p = Meters(431);        {p:?}   (p.0 = {})\n", p.0);

    println!("5. Which name can be passed straight to `.map(..)`?");
    let ms: Vec<Meters> = vec![12u32, 40, 431].into_iter().map(Meters).collect();
    println!("   .map(Meters) works: {ms:?}");
    println!("   Only the tuple struct. `Meters` IS a function value, which is exactly");
    println!("   what refusal 4 was complaining about — the same fact, once as a bug and");
    println!("   once as a feature. `Player` is a type only, and `Sealed` is a type plus a");
    println!("   value that takes no arguments; neither is callable.");
    println!("   .map(Player) -> E0423 expected value, found struct `Player`");
    println!("   .map(Sealed) -> E0618 expected function, found struct `Sealed`\n");

    println!("6. Deferred initialization needs no `mut`");
    let high_score = 512;
    let level: u8;
    if high_score > 500 {
        level = 1;
    } else {
        level = 2;
    }
    println!("   let level: u8;  then one assignment per path  ->  level = {level}");
    println!("   `mut` permits a SECOND write. There is no second write here, so the");
    println!("   binding stays immutable and the compiler still proves it was set");
    println!("   before use. That is what `let v: Player();` was reaching for — it just");
    println!("   put a call in the type slot and never supplied the value.");
}
```
<!-- /source -->

<!-- output:a_type_is_not_a_constructor_kata -->
*Verified output of [`a_type_is_not_a_constructor_kata.rs`](examples/a_type_is_not_a_constructor_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Four spellings of 'make me one'. Three refuse, and they refuse differently.

1. let v: Player();     TWO errors, not one
     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
     error[E0381]: used binding `v` isn't initialized
   After `:` rustc reads a TYPE. `Player()` in type position is the
   `Fn(A) -> B` sugar, and only the Fn traits may use it. Then, separately,
   no `=` ever appeared, so nothing was assigned.
   fixed:  let v = Player { .. };       Player { name: "Ada", level: 7 }

2. let s = Sealed();    E0618 — expected function, found struct `Sealed`
   help: `Sealed` is a unit struct, and does not take parentheses
         to be constructed
   A unit struct declares a type AND a value of that type sharing one name.
   fixed:  let s = Sealed;               Sealed

3. let w = Player;      E0423 — expected value, found struct `Player`
   help: use struct literal syntax instead: `Player { name: val, level: val }`
   A named-field struct declares ONLY the type. The bare name is not a value.
   fixed:  let w = Player { .. };       Player { name: "Ben", level: 3 }
   (fields read the ordinary way: w.name = "Ben", w.level = 3)

4. let p = Meters;      COMPILES — and gives you the wrong thing
   `p` is not a Meters. It is the tuple struct's constructor function,
   of type `fn(u32) -> Meters {Meters}`, and the next line is where
   you find out:
     error[E0277]: `fn(u32) -> Meters {Meters}` doesn't implement `Debug`
     help: use parentheses to construct this tuple struct: `Meters(/* u32 */)`
   fixed:  let p = Meters(431);        Meters(431)   (p.0 = 431)

5. Which name can be passed straight to `.map(..)`?
   .map(Meters) works: [Meters(12), Meters(40), Meters(431)]
   Only the tuple struct. `Meters` IS a function value, which is exactly
   what refusal 4 was complaining about — the same fact, once as a bug and
   once as a feature. `Player` is a type only, and `Sealed` is a type plus a
   value that takes no arguments; neither is callable.
   .map(Player) -> E0423 expected value, found struct `Player`
   .map(Sealed) -> E0618 expected function, found struct `Sealed`

6. Deferred initialization needs no `mut`
   let level: u8;  then one assignment per path  ->  level = 1
   `mut` permits a SECOND write. There is no second write here, so the
   binding stays immutable and the compiler still proves it was set
   before use. That is what `let v: Player();` was reaching for — it just
   put a call in the type slot and never supplied the value.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:a_type_is_not_a_constructor -->
*Verified output of [`a_type_is_not_a_constructor.rs`](examples/a_type_is_not_a_constructor.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three flavors, three spellings — and only ONE of them is a call
   named-field   Player { name, score }   ->  Player { name: "Ada", score: 5 }
   tuple struct  Meters(7)                ->  Meters(7)
   unit struct   Sealed                   ->  Sealed
   ...ordinary values, with ordinary fields: named.name = "Ada", named.score = 5, tuple.0 = 7
   The braces are a literal; the bare name is a value. `Player()` is
   not a shorter spelling of either one:
     error[E0423]: expected function, tuple struct or tuple variant,
                   found struct `Player`
     help: use struct literal syntax instead: `Player { name: val, score: val }`

2. The tuple struct's name really IS a function — the other two are not
   let make: fn(u32) -> Meters = Meters;   make(12) = Meters(12)
   [3, 7, 12].map(Meters) = [Meters(3), Meters(7), Meters(12)]
   `.map(Player)` is E0423: `expected value, found struct `Player``.
   So 'constructor' is literal for one flavor and a figure of speech for the rest.

3. `Sealed` is two declarations sharing a name
   let s: Sealed = Sealed;   Sealed
   Left of the `=` is the TYPE namespace; right of it is the VALUE namespace.
   `struct AlsoEmpty {}` declares only the type, so it needs `AlsoEmpty {}`.

4. `new` is a convention, and std does not even keep to it
   Player::new("Ben", 3)   Player { name: "Ben", score: 3 }
   Player::blank()         Player { name: "(unnamed)", score: 0 }
   Vec::<u8>::new()        []
   String::from("Cara")    "Cara"
   u8::default()           0
   Four names for 'make me one'. None of them is a keyword.

5. `:` supplies a TYPE, `=` supplies a VALUE — and only the second is required
   let chosen: Player;  assigned on every path, exactly once
   high_score = 431  ->  Player { name: "Cara", score: 4 }
   No `mut`: one assignment is not a mutation. Read it before assigning
   and the compiler stops you:
     error[E0381]: used binding `chosen` isn't initialized

6. So `let p: Player();` is both mistakes in eight characters
     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
       after `:` rustc is reading a TYPE, and `Name(..)` in type position is
       the `Fn(A) -> B` sugar, which only the Fn traits may use.
     error[E0381]: used binding `p` isn't initialized
       and no value was ever supplied, because `=` never appeared.
   The fix is one character and one pair of braces: `let p = Player { .. };`
```
<!-- /output -->

## See also

- [What a struct is](../what_a_struct_is/README.md) — the three flavors in full, and the privacy that makes a tuple struct's constructor private
- [When a struct refuses](../when_a_struct_refuses/README.md) — the eight errors of *defining* one
- [`impl` blocks](../impl_blocks/README.md) — where `new` lives, and why it is an associated function
- [Initial values](../../17_Option_and_Result/initial_values/README.md) — deferred initialization as the deliberate tool
- [STRUCTS.md](../../STRUCTS.md) — the map

## Po polsku

W Ruscie nie ma wywołania `Type()`, które tworzy wartość — i najkrócej tłumaczy to podział na dwie **przestrzenie nazw**: osobną dla typów i osobną dla wartości. `struct Player { .. }` zapisuje nazwę tylko w tej pierwszej, więc samo `Player` nie jest niczym, co da się wywołać; wartość powstaje z literału `Player { name, score }`. `struct Meters(u32);` zapisuje nazwę w obu i to jedyna z trzech odmian, która naprawdę dostaje z deklaracji funkcję — `Meters` ma typ `fn(u32) -> Meters`, więc nadaje się wprost na argument `.map(Meters)`, podczas gdy `.map(Player)` to `E0423`, bo nie ma czego przekazać. `struct Sealed;` też zapisuje nazwę w obu, tyle że po stronie wartości leży gotowa pusta struktura, a nie funkcja. Stąd zdanie tytułowe: „konstruktor” jest tu dosłowny dla jednej odmiany, a dla dwóch pozostałych to przenośnia.

Samo `new` również nie jest słowem kluczowym — język o nim nie słyszał. `Player::new` to zwyczajna funkcja powiązana z typem, która akurat zwraca `Self`, i wolno ją nazwać tak, jak nazywa to dziedzina (`blank()`, `from_row()`). Biblioteka standardowa sama trzyma cztery nazwy do tej samej roboty: `Vec::new()`, `String::from("Cara")`, `u8::default()`. Typ bez `new` niczego nie jest pozbawiony. Osobno warto zapamiętać podział pracy między `:` a `=`: dwukropek podaje **typ**, znak równości podaje **wartość**, i tylko to drugie jest obowiązkowe. Dlatego `let chosen: Player;` z przypisaniem w każdej gałęzi `if` kompiluje się **bez `mut`** — jedno przypisanie na ścieżkę to jeszcze nie mutacja, a `mut` zezwala dopiero na *drugi* zapis. Pominięta gałąź to `E0381`.

Cztery sposoby, żeby sięgnąć po wartość i nie trafić, dają cztery różne kody błędu i warto je czytać właśnie jako różne. `let b: Player();` wypisuje **dwa** błędy naraz: po dwukropku kompilator czyta typ, a `Nazwa(A) -> B` w pozycji typu to lukier składniowy dla cech `Fn` — stąd `E0214` — po czym całkiem niezależnie zgłasza `E0381`, bo `=` nigdy się nie pojawiło. `let b = Player();` trafia już w miejsce na wartość, więc dostaje `E0423` razem z podpowiedzią wypisującą pola. `let s = Sealed();` to `E0618`: pusta struktura *jest* wartością, a nawias próbuje wywołać coś, co nie jest funkcją. Najciekawszy jest czwarty, `let p = Meters;`, bo on **się kompiluje** — `p` nie jest żadną długością, tylko funkcją konstruującą `Meters`, i dowiadujesz się o tym dopiero linijkę dalej, gdy `println!("{p:?}")` zgłasza `E0277` na typie `fn(u32) -> Meters {Meters}`. To dokładnie ten sam fakt, który wyżej był wygodą przy `.map` — raz jako funkcja, raz jako błąd.

**Szukaj po polsku:** konstruktor w Ruscie · struktura krotkowa · przestrzeń nazw typów i wartości · `rust E0423 expected value found struct` · `rust unit struct E0618`
