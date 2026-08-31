# Modules and visibility

**Level:** 201 · working knowledge

**One line:** A `mod` is a namespace and a wall — everything inside is private by default, and "private" means *to this module and its descendants*, which is a bigger circle than most people expect.

```rust
mod election {
    pub const SEATS: u32 = 3;
    const QUORUM: u32 = 10;               // no pub: invisible outside

    pub fn seats_and_quorum() -> (u32, u32) { (SEATS, QUORUM) }
}

fn main() {
    println!("{:?}", election::seats_and_quorum());   // (3, 10)
}
```

`election::QUORUM` from outside is `E0603`, *"constant `QUORUM` is private"*. What is private is the **name**, not the data — the module can read its own private items and hand the values out, which is exactly what an accessor is.

## Privacy runs one way

A child module sees everything its ancestors have, public or not, through `super::`. A parent cannot see into a child at all. That asymmetry is what makes a module a useful boundary rather than a filing cabinet: you can put a helper in a submodule and know that only that submodule's own code can reach it.

## `pub` is relative, and there are four of them

| Written | Visible to |
|---|---|
| *(nothing)* | this module and its descendants |
| `pub(super)` | the parent module and its descendants |
| `pub(crate)` | anywhere in this crate, nowhere outside |
| `pub(in path)` | one named ancestor module |
| `pub` | as far as whoever can see the module itself |

That last row is the one that surprises people: `pub fn` inside a private module is not public to anybody, because the path to it does not exist. **`pub` publishes an item to the same audience the module already had.**

## A public struct does not have public fields

```rust
mod election {
    pub struct Result { pub winner: &'static str, margin: u32 }

    impl Result {
        pub fn new(winner: &'static str, margin: u32) -> Self { Result { winner, margin } }
        pub fn margin(&self) -> u32 { self.margin }
    }
}
```

Fields are published one at a time. `r.margin` from outside is `E0616`, and writing the struct literal `election::Result { winner: "Ada", margin: 7 }` is `E0451` — *"field `margin` of struct `Result` is private"* — so a caller cannot construct one at all except through `new`. That is the mechanism behind every type in Rust that maintains an invariant, [the newtype](../../16_Structs/newtype_score/README.md) included.

## The trap: the access list is the module, not the type

```rust
mod sealed {
    pub struct Score { value: u8 }
    impl Score {
        pub fn new(value: u8) -> Option<Self> {
            if value <= 5 { Some(Score { value }) } else { None }
        }
    }
    pub fn from_raw(value: u8) -> Score { Score { value } }   // the open door
}
```

`from_raw` is inside `sealed`, so the private field is visible to it, and it is `pub`, so it is visible to everyone. The careful constructor is now advice. **Privacy is per module, so every line in the same file as the type can build one** — when an invariant escapes, the thing to audit is the module, not the callers.

Which is the real argument for one type per module in a library: the fewer lines that share a module with a type, the shorter the list of code that can break its rules.

## If you are coming from another language

- **Python.** The leading-underscore convention becomes a compiler rule. Three differences worth carrying. Python's `_private` is advisory and `import` reaches anything; Rust's `E0603` is a build failure, so the boundary is real rather than polite. Python's unit of privacy is the *module file* and Rust's is the `mod`, which may be a block inside a file — so two types in one file share a privacy scope even if they are in different `mod` blocks only when nested. And `__all__` controlling `from x import *` is the closest thing to `pub`, with the same job and none of the enforcement.
- **ABAP.** This is `PUBLIC SECTION` / `PROTECTED SECTION` / `PRIVATE SECTION`, and the mapping is close enough to reason with: `pub` is the public section, no keyword is the private section, and `pub(crate)` is roughly a package-visible interface in a package with `Package Check` switched on. The instructive difference is what the unit is. ABAP's boundary is the CLASS: a private attribute is invisible to every other class, including one in the same include. Rust's boundary is the MODULE, which can hold several types — so two structs in one `mod` can read each other's private fields, and a friend-class relationship that ABAP needs `FRIENDS` for is simply the default here. The trap section above is that difference biting: an ABAP developer expects the private attribute to be safe from a function in the same file, and it is not.
- **Java.** `package-private` (no modifier) is the default in both languages, and Rust's default is stricter — Java's package is flat, Rust's module tree nests, and a child module gets its parent's privates where a Java subpackage gets nothing. `public` on a Java class is unconditional; `pub` is relative to the path.
- **C++.** `namespace` plus `class` access specifiers, split across two features that Rust merges into one. The anonymous namespace is `pub(crate)`'s neighbour, and `friend` is the default relationship between items in one module.

---

## The verified output

<!-- output:modules_and_visibility -->
*Verified output of [`modules_and_visibility.rs`](examples/modules_and_visibility.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Private by default, at every level
   election::SEATS            = 3
   election::QUORUM           -> E0603: "constant `QUORUM` is private"
   election::seats_and_quorum() = (3, 10)
   The module can read its own private items and hand the values
   out. What is private is the NAME, not the data.

2. Privacy is one-way: children see up, parents do not see in
   ballots::quorum_from_child() = 10
   A child module reaches its parent's private items through
   `super::`. The parent cannot reach a child's private items at
   all — which is the whole reason a module is a useful boundary.

3. `pub` is relative, and `pub(super)` / `pub(crate)` say how far
   election::tally::total()          = 60
   election::tally::internal_total() -> E0603: "function
   `internal_total` is private" — same code as QUORUM above, because
   pub(super) stops at `election` and main is outside it.
   pub            as far as whoever can see the module
   pub(crate)     anywhere in this crate, nowhere outside it
   pub(super)     the parent module and its descendants
   pub(in path)   one named ancestor module

4. A public struct with private fields
   r.winner   = Ada   <- pub field
   r.margin   -> E0616: "field `margin` of struct `Result` is private"
   r.margin() = 7   <- the accessor is the door that stayed open
   Fields are private individually, so `pub struct` says nothing
   about them. This is what stops a caller building a Result with
   a margin that does not match the ballots.

5. Paths: `crate::`, `super::`, `self::`
   crate::election::SEATS  = 3   absolute, from the crate root
   self::election::SEATS   = 3   relative to this module
   super::               one level up — used inside a module, and
                         an error at the crate root, where there is
                         nothing above.
```
<!-- /output -->

## Practice

**The door your own module left open.** Write a `Score` newtype whose constructor refuses anything above 5, three times: once with a `pub` field, once with the field private, and once with a private field *and* a `pub fn from_raw` helper beside it in the same module.

Build an invalid `Score` through each. Two of the three let you, for different reasons, and only one of those two looks like a mistake in the diff. Then say which of the three fixes — deleting the helper, `pub(crate)`-ing it, or moving the type to a module of its own — you would actually apply, and what each one costs.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:modules_and_visibility_kata -->
*[`modules_and_visibility_kata.rs`](examples/modules_and_visibility_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the door your own module left open.
//!
//!   rustc --edition 2024 modules_and_visibility_kata.rs -o /tmp/mvk && /tmp/mvk

/// Attempt 1: `pub` everywhere, which validates nothing.
mod leaky {
    #[derive(Debug)]
    pub struct Score {
        pub value: u8,
    }

    impl Score {
        pub fn new(value: u8) -> Option<Self> {
            if value <= 5 { Some(Score { value }) } else { None }
        }
    }
}

/// Attempt 2: the field is private, so the constructor is the only door.
mod sealed {
    #[derive(Debug)]
    pub struct Score {
        value: u8,
    }

    impl Score {
        pub fn new(value: u8) -> Option<Self> {
            if value <= 5 { Some(Score { value }) } else { None }
        }
        pub fn value(&self) -> u8 {
            self.value
        }
    }

    /// ...but anything INSIDE this module can still build one directly, and
    /// this helper is `pub`, so the invariant leaves by the back door.
    pub fn from_raw(value: u8) -> Score {
        Score { value }
    }

    /// The version that keeps the invariant in one place.
    pub fn clamped(value: u8) -> Score {
        Score { value: value.min(5) }
    }
}

fn main() {
    println!("1. The leaky version");
    let mut s = leaky::Score::new(4).unwrap();
    println!("   leaky::Score::new(9) = {:?}   <- the constructor works", leaky::Score::new(9));
    s.value = 200;
    println!("   ...and then: s.value = 200 -> {s:?}");
    println!("   A `pub` field makes the constructor advice, not a rule. Nothing");
    println!("   in the type system stops the assignment, because there is no");
    println!("   invariant recorded anywhere — only a habit.");

    println!();
    println!("2. The sealed version");
    let t = sealed::Score::new(4).unwrap();
    println!("   sealed::Score::new(4) = {t:?}, .value() = {}", t.value());
    println!("   t.value = 200      -> E0616: field `value` of struct `Score` is private");
    println!("   sealed::Score {{ value: 9 }} -> E0451: field `value` of struct");
    println!("   `Score` is private — you cannot even write the literal.");

    println!();
    println!("3. The door that is still open, and it is inside the module");
    let cheat = sealed::from_raw(200);
    println!("   sealed::from_raw(200) = {cheat:?}   <- 200 on a 0-5 scale");
    println!("   `from_raw` is inside `sealed`, so the private field is visible to");
    println!("   it, and it is `pub`, so it is visible to everyone. Privacy is per");
    println!("   MODULE, not per struct: every line in the same file as the type");
    println!("   can construct one. That is the thing to check when an invariant");
    println!("   escapes — not the callers, the module.");
    println!("   sealed::clamped(200) = {:?}   <- the same door, honestly labelled",
             sealed::clamped(200));

    println!();
    println!("4. What actually closes it");
    println!("   a. Delete from_raw, or make it private — then the only way in is");
    println!("      new() and clamped(), and both enforce the range.");
    println!("   b. pub(crate) it, if the crate's own tests need the shortcut but");
    println!("      no downstream user should have it.");
    println!("   c. Move the type into a module of its own with NOTHING else in");
    println!("      it. The smaller the module, the fewer lines can reach the");
    println!("      private field — which is the real argument for one type per");
    println!("      module in a library.");

    println!();
    println!("5. The reading that generalises");
    println!("   `pub struct` publishes the TYPE. Each field is published");
    println!("   separately, and a private field is what makes a constructor the");
    println!("   only entrance. Then check who shares the module with it, because");
    println!("   that list — not the `pub` keywords — is the real access list.");
}
```
<!-- /source -->

<!-- output:modules_and_visibility_kata -->
*Verified output of [`modules_and_visibility_kata.rs`](examples/modules_and_visibility_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The leaky version
   leaky::Score::new(9) = None   <- the constructor works
   ...and then: s.value = 200 -> Score { value: 200 }
   A `pub` field makes the constructor advice, not a rule. Nothing
   in the type system stops the assignment, because there is no
   invariant recorded anywhere — only a habit.

2. The sealed version
   sealed::Score::new(4) = Score { value: 4 }, .value() = 4
   t.value = 200      -> E0616: field `value` of struct `Score` is private
   sealed::Score { value: 9 } -> E0451: field `value` of struct
   `Score` is private — you cannot even write the literal.

3. The door that is still open, and it is inside the module
   sealed::from_raw(200) = Score { value: 200 }   <- 200 on a 0-5 scale
   `from_raw` is inside `sealed`, so the private field is visible to
   it, and it is `pub`, so it is visible to everyone. Privacy is per
   MODULE, not per struct: every line in the same file as the type
   can construct one. That is the thing to check when an invariant
   escapes — not the callers, the module.
   sealed::clamped(200) = Score { value: 5 }   <- the same door, honestly labelled

4. What actually closes it
   a. Delete from_raw, or make it private — then the only way in is
      new() and clamped(), and both enforce the range.
   b. pub(crate) it, if the crate's own tests need the shortcut but
      no downstream user should have it.
   c. Move the type into a module of its own with NOTHING else in
      it. The smaller the module, the fewer lines can reach the
      private field — which is the real argument for one type per
      module in a library.

5. The reading that generalises
   `pub struct` publishes the TYPE. Each field is published
   separately, and a private field is what makes a constructor the
   only entrance. Then check who shares the module with it, because
   that list — not the `pub` keywords — is the real access list.
```
<!-- /output -->

</details>

---

## See also

- [Bringing names in with `use`](../the_use_declaration/README.md) — the shortcut that does not change any of this
- [One module per file](../one_module_per_file/README.md) — the same tree, written as directories
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the invariant this page's privacy is protecting, and the same open door found from the other side
- [What a struct is](../../16_Structs/what_a_struct_is/README.md) — where the fields came from
- [`const` and `static`](../const_and_static/README.md) — the two item kinds that most often want a `pub`

## Sources

[Modules ↗](https://doc.rust-lang.org/rust-by-example/mod.html) and [Visibility ↗](https://doc.rust-lang.org/rust-by-example/mod/visibility.html) in Rust by Example; the Reference's [visibility and privacy ↗](https://doc.rust-lang.org/reference/visibility-and-privacy.html) chapter for the exact rules. The three error codes above were produced by compiling each violation.

## Po polsku

Wszystko wewnątrz modułu jest domyślnie prywatne, tylko słowo „prywatne” znaczy tu „dla tego modułu **i jego potomków**”, czyli obejmuje szerszy krąg, niż się zwykle zakłada. Widoczność biegnie przy tym w jedną stronę: moduł potomny sięga w górę przez `super::` i widzi wszystko, co mają jego przodkowie, a rodzic do dziecka nie zajrzy w ogóle. Warto też zauważyć, co dokładnie jest prywatne — **nazwa**, a nie dane. Moduł swobodnie czyta własne prywatne elementy i może oddać ich wartość na zewnątrz; dokładnie tym jest akcesor. Sięgnięcie po `election::QUORUM` z zewnątrz kończy się błędem `E0603`, *constant `QUORUM` is private*.

`pub` nie znaczy „publiczne dla świata”, tylko „widoczne dla tych, którzy i tak widzą ten moduł”. Dlatego `pub fn` w prywatnym module nie jest publiczne dla nikogo — ścieżka do niego po prostu nie istnieje. Do wskazania konkretnego zasięgu służą warianty `pub(super)` (rodzic i jego potomkowie), `pub(crate)` (cały crate, ani kroku dalej) i `pub(in ścieżka)` (jeden wskazany przodek). Osobno działają pola: `pub struct` publikuje **typ**, a nie jego zawartość, i pola udostępnia się po jednym. Odczyt prywatnego pola z zewnątrz to `E0616`, a napisanie literału struktury z takim polem to `E0451` — czyli z zewnątrz nie da się takiej struktury zbudować inaczej niż przez konstruktor, i to jest cały mechanizm, na którym stoi pilnowanie niezmienników.

Największa różnica wobec nawyków z Javy, C# czy ABAP-a jest w tym, **czym jest jednostka prywatności**: tam klasą, tutaj modułem. Dwie struktury leżące w jednym `mod` czytają nawzajem swoje prywatne pola, więc to, co w C++ wymaga deklaracji `friend`, a w ABAP-ie klauzuli `FRIENDS`, jest w Ruscie stanem domyślnym. Stąd bierze się pułapka z tej strony: `Score::new` odrzuca wartości powyżej 5, ale `pub fn from_raw` napisane obok, w tym samym module, buduje `Score { value: 200 }` bez mrugnięcia okiem i wynosi to na zewnątrz. Praktyczny wniosek jest krótki — gdy niezmiennik ucieka, audytuj **moduł**, a nie wywołujących, i w bibliotece trzymaj jeden typ na moduł, bo im mniej linii dzieli moduł z typem, tym krótsza lista kodu, który może złamać jego regułę.

**Szukaj po polsku:** widoczność w modułach Rusta · prywatność modułu a klasa · `rust module visibility pub(crate)` · `rust E0603 is private` · `rust E0451 private field struct literal`
