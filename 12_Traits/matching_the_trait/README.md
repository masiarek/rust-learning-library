# When the `impl` does not match the trait

**Level:** 201 · working knowledge

**One line:** The trait owns the signature and the `impl` owns the body — so an impl may replace what a method *does*, but its name, `self`, parameter types, return type and bounds are copied from the trait, and every way of copying one wrong has its own error code.

```rust
trait Animal {
    fn name(&self) -> String;          // required: no body
    fn speak(&self, times: u32) {      // provided: a default body
        for _ in 0..times {
            println!("{} makes a sound", self.name());
        }
    }
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> String { "Rex".to_string() }
    fn speak(&self, mut n: u32) {      // the trait's types; your own name and body
        while n > 0 {
            println!("{}: woof", self.name());
            n -= 1;
        }
    }
}

fn main() {
    Dog.speak(1); // Rex: woof
}
```

`Dog` replaced the default `speak` and renamed its parameter. It kept every type the trait wrote: `&self`, `u32`, and the `()` that a missing `->` means.

## What an impl may change, and what it may not

| In the `impl` | Result |
|---|---|
| The body | allowed — it is what the impl is for |
| A parameter's name (`times` → `n`) | allowed |
| `mut` on a parameter (`mut n: u32`) | allowed — it belongs to the body's binding, not to the method's type |
| A bound the trait declared, dropped (`T: Display` → `T`) | allowed — the impl asks less of its callers |
| A provided method, left out | allowed — the trait's default body runs |
| A required method, left out | [`E0046`](#e0046-and-e0407-one-method-missing-or-one-too-many) |
| A method the trait does not declare | [`E0407`](#e0046-and-e0407-one-method-missing-or-one-too-many) |
| A parameter type, the return type, or `&self` → `&mut self` | [`E0053`](#e0053-the-right-method-a-different-type) |
| A parameter added or removed | [`E0050`](#e0050-e0185-and-e0186-a-parameter-too-few-or-too-many) |
| `self` on one side only | [`E0185` / `E0186`](#e0050-e0185-and-e0186-a-parameter-too-few-or-too-many) |
| A bound the trait did not declare, added | [`E0276`](#e0276-a-bound-the-trait-never-asked-for) |
| `pub` on the method | [`E0449`](#e0449-pub-on-a-trait-method) |

Every transcript below is rustc 1.98 compiling the program above with one line of the `impl` changed, saved as `dog.rs`.

## E0046 and E0407: one method missing, or one too many

```rust
impl Animal for Dog {
    fn name(&self) -> String { "Rex".to_string() }   // delete this line: E0046
    fn speak(&self, times: u32) { println!("woof x{times}"); }
}
```

```text
error[E0046]: not all trait items implemented, missing: `name`
  --> dog.rs:12:1
   |
 2 |     fn name(&self) -> String;          // required: no body
   |     ------------------------- `name` from trait
...
12 | impl Animal for Dog {
   | ^^^^^^^^^^^^^^^^^^^ missing `name` in implementation
```

Only a **required** method can be missing. `speak` has a body in the trait, so leaving it out is never an error — that is the `Cat` in [the output below](#the-verified-output).

```rust
impl Animal for Dog {
    fn name(&self) -> String { "Rex".to_string() }
    // fn speek(&self, times: u32) { println!("woof x{times}"); }   // E0407: meant `speak`
}
```

```text
error[E0407]: method `speek` is not a member of trait `Animal`
  --> dog.rs:14:5
   |
14 |     fn speek(&self, times: u32) { println!("woof x{times}"); }
   |     ^^^-----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `speak`
   |     not a member of trait `Animal`
```

`E0407` is the typo detector. In a language where overriding is implicit, `speek` quietly becomes a second method and the default `speak` keeps running. Here, an `impl Trait for Type` block may only hold items the trait declared, so the typo cannot compile — and rustc names the method you meant.

## E0053: the right method, a different type

```rust
impl Animal for Dog {
    fn name(&self) -> String { "Rex".to_string() }
    // fn speak(&self, times: usize) { println!("woof x{times}"); }   // E0053: the trait says u32
}
```

```text
error[E0053]: method `speak` has an incompatible type for trait
  --> dog.rs:14:28
   |
14 |     fn speak(&self, times: usize) { println!("woof x{times}"); }
   |                            ^^^^^ expected `u32`, found `usize`
   |
note: type in trait
  --> dog.rs:3:28
   |
 3 |     fn speak(&self, times: u32) {      // provided: a default body
   |                            ^^^
   = note: expected signature `fn(&Dog, u32)`
              found signature `fn(&Dog, usize)`
help: change the parameter type to match the trait
   |
14 -     fn speak(&self, times: usize) { println!("woof x{times}"); }
14 +     fn speak(&self, times: u32) { println!("woof x{times}"); }
   |
```

The `note:` lines are the part to read: the trait's signature over yours, written as function types. `self` is always printed; a later parameter that matches prints as `_`, and a return type that matches is left off, so each pair shows the receiver and the disagreement. The same code covers three more mistakes:

| You wrote | rustc's label | `expected signature` / `found signature` |
|---|---|---|
| `fn speak(&mut self, times: u32)` | types differ in mutability | `fn(&Dog, _)` / `fn(&mut Dog, _)` |
| `fn speak(&self, times: u32) -> u32` | expected `()`, found `u32` | `fn(&Dog, _) -> ()` / `fn(&Dog, _) -> u32` |
| `fn name(&self) -> &str` | expected `String`, found `&str` | `fn(&Dog) -> String` / `fn(&Dog) -> &str` |

`&mut self` looks like it asks for *more* access, and that is exactly why it is refused: code calling `speak` through the trait holds a `&Dog`, and cannot produce the `&mut Dog` your version needs.

## The trap: the help line fixes the arrow, not the body

The second row of that table is the one you write on purpose — you wanted `speak` to report how many times it spoke:

```text
error[E0053]: method `speak` has an incompatible type for trait
  --> dog.rs:14:36
   |
14 |     fn speak(&self, times: u32) -> u32 { times }
   |                                    ^^^ expected `()`, found `u32`
   |
note: type in trait
  --> dog.rs:3:32
   |
 3 |     fn speak(&self, times: u32) {      // provided: a default body
   |                                ^
   = note: expected signature `fn(&Dog, _) -> ()`
              found signature `fn(&Dog, _) -> u32`
help: change the output type to match the trait
   |
14 -     fn speak(&self, times: u32) -> u32 { times }
14 +     fn speak(&self, times: u32) -> () { times }
   |
```

Take the suggestion and the signature matches, so the error moves into the body:

```text
error[E0308]: mismatched types
  --> dog.rs:14:41
   |
14 |     fn speak(&self, times: u32) -> () { times }
   |                                    --   ^^^^^ expected `()`, found `u32`
   |                                    |
   |                                    expected `()` because of return type
```

No `help:` this time, because there is nothing left to suggest. `E0053`'s help rewrites the signature to agree with the trait and never reads the body, so for a return type it hands you a function whose body no longer fits. (The `&str` row goes the same way, then recovers: the `E0308` it leads to suggests `.to_string()`.)

What you wanted is not *this* method of *this* trait, and no edit to the `impl` will make it one. Two things will:

- **Change the trait**, if it is yours — and then every implementor changes with it.
- **Write an inherent method** — `impl Cat { fn speak(&self, times: u32) -> u32 { … } }`. It compiles with no warning, and the dot call on a `Cat` now picks it, while generic code and `Animal::speak(&cat, 1)` still reach the trait's version. Sections 4 and 5 of [the output](#the-verified-output) show both. Two methods under one name is a trap of its own, and [Extension traits](../extension_traits/README.md) is about it.

## E0050, E0185 and E0186: a parameter too few, or too many

```rust
impl Animal for Dog {
    fn name(&self) -> String { "Rex".to_string() }
    // fn speak(&self) { println!("woof"); }   // E0050: the trait takes `times` too
}
```

```text
error[E0050]: method `speak` has 1 parameter but the declaration in trait `Animal::speak` has 2
  --> dog.rs:14:14
   |
 3 |     fn speak(&self, times: u32) {      // provided: a default body
   |              ----------------- trait requires 2 parameters
...
14 |     fn speak(&self) { println!("woof"); }
   |              ^^^^^ expected 2 parameters, found 1
```

The count includes `&self` — "has 1 parameter" is the receiver alone. Drop the receiver instead and you get a code of its own:

```rust
impl Animal for Dog {
    fn name(&self) -> String { "Rex".to_string() }
    // fn speak(times: u32) { println!("woof x{times}"); }   // E0186: no `&self`
}
```

```text
error[E0186]: method `speak` has a `&self` declaration in the trait, but not in the impl
  --> dog.rs:14:5
   |
 3 |     fn speak(&self, times: u32) {      // provided: a default body
   |     --------------------------- `&self` used in trait
...
14 |     fn speak(times: u32) { println!("woof x{times}"); }
   |     ^^^^^^^^^^^^^^^^^^^^ expected `&self` in impl
```

The mirror image — `&self` in the impl for a function the trait declared without one, like `fn new() -> Self` — is `E0185`: *"has a `&self` declaration in the impl, but not in the trait"*. A method and an associated function are called differently (`dog.speak(1)` against `Dog::new()`), so this is never a detail rustc can paper over.

## E0276: a bound the trait never asked for

```rust
use std::fmt::Display;

trait Tag {
    fn label<T: Display>(&self, value: T) -> String;
}

struct Price;

impl Tag for Price {
    fn label<T: Display>(&self, value: T) -> String { format!("${value}") }
    // fn label<T: Display + Copy>(&self, value: T) -> String { format!("${value}") }   // E0276
}

fn main() {
    println!("{}", Price.label(5)); // $5
}
```

```text
error[E0276]: impl has stricter requirements than trait
  --> tag.rs:10:27
   |
 4 |     fn label<T: Display>(&self, value: T) -> String;
   |     ------------------------------------------------ definition of `label` from trait
...
10 |     fn label<T: Display + Copy>(&self, value: T) -> String { format!("${value}") }
   |                           ^^^^ impl has extra requirement `T: Copy`
```

A caller that has only the trait in front of it may pass any `T: Display`, including a `String`, which is not `Copy`. An impl that demands more would break calls the trait already promised would work. The other direction is allowed: `Dog`'s `tag` in [the output](#the-verified-output) drops `T: Display` because its body never formats the value.

## E0449: pub on a trait method

```rust
impl Animal for Dog {
    fn name(&self) -> String { "Rex".to_string() }
    // pub fn speak(&self, times: u32) { println!("woof x{times}"); }   // E0449
}
```

```text
error[E0449]: visibility qualifiers are not permitted here
  --> dog.rs:14:5
   |
14 |     pub fn speak(&self, times: u32) { println!("woof x{times}"); }
   |     ^^^ help: remove the qualifier
   |
   = note: trait items always share the visibility of their trait
```

The note is the rule: a trait method is exactly as visible as its trait, so there is nothing to declare — and the help line, for once, is the whole fix.

## If you are coming from another language

**Python.** Overriding is implicit, and nothing compares the two signatures when the program runs. A subclass whose `speak(self, times: int) -> int` replaces a base method returning `None` runs and returns its value; a subclass that forgot an `@abstractmethod` fails only when someone constructs it — `TypeError: Can't instantiate abstract class Cat without an implementation for abstract method 'name'`, which is `E0046` moved from compile time to the first `Cat()`.

mypy does compare, and its message is `E0053`'s: `Return type "int" of "speak" incompatible with return type "None" in supertype "Animal"  [override]`. The typo is the case neither catches by default. `def speek` is a new method, the inherited `speak` keeps running, and mypy says nothing — unless the method carries `@typing.override` (3.12+), when mypy reports `Method "speek" is marked as an override, but no base method was found with this name`. That decorator is `E0407` as an opt-in, per method. In an `impl Trait for Type` block every method is marked, and there is no way to leave the mark off. What changed: the checks you ran mypy for are part of compiling now, and the one you had to remember to ask for is no longer optional.

**Java.** The same two habits, reversed. `@Override` is Java's opt-in `E0407` — `method does not override or implement a method from a supertype` — and it catches nothing on a method that leaves it off. And an interface method is implicitly public, so Java *requires* the implementing method to say `public`: without it, `name() in Dog cannot implement name() in Animal` / `attempting to assign weaker access privileges; was public`. Rust has the same underlying rule — the method is as visible as the trait — and draws the opposite conclusion: since `pub` could only ever repeat what the trait already decided, writing it is `E0449`.

**ABAP.** A class implementing an interface writes `METHOD zif_animal~speak.` and nothing after the name: the parameters live only in the interface's definition and are never restated in the class. So `E0053`, `E0050` and `E0186` have no ABAP counterpart — there is no second copy to get wrong. Rust makes you write the signature again inside the `impl`, which keeps each impl readable on its own, and then checks the copy against the original. What transfers is the authority: the trait decides the signature, exactly as the interface does. What changed is that you type it twice, and the compiler holds you to the first.

## The verified output

<!-- output:matching_the_trait -->
*Verified output of [`matching_the_trait.rs`](examples/matching_the_trait.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The required method, written by each type
   Dog.name() = Rex
   Cat.name() = Tom

2. Dog replaced the provided method: the trait's types, its own parameter name
   Rex: woof
   Rex: woof

3. Dog's tag drops the trait's Display bound; Cat's is the default
   Dog.tag(7) = dog Rex
   Cat.tag(7) = 7: Tom

4. Cat's inherent speak returns a value, and the dot call picks it
   Tom: meow x3
   Cat.speak(3) returned 3

5. Anything that reaches Cat through the trait gets the default body
   chorus(&Cat):
   Tom makes a sound
   Animal::speak(&Cat, 1):
   Tom makes a sound
```
<!-- /output -->

## Practice

**Four impls that disagree with their trait.** Compile this once — rustc reports all four mismatches in a single run. Before you read past each error code, say what that impl claimed that the trait did not. Then fix all four **without touching the trait**; one of them needs more than its `help:` line offers.

```rust
trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) -> String {
        format!("area {:.1}", self.area())
    }
}

struct Square { side: f32 }
struct Circle { radius: f64 }
struct Strip { width: f64, height: f64 }

impl Shape for Square {
    fn area(&self) -> f32 { self.side * self.side }
}

impl Shape for Circle {
    pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

impl Shape for Strip {
    fn area(self) -> f64 { self.width * self.height }
    fn describe(&self, digits: usize) -> String { format!("area {:.digits$}", self.area()) }
}

fn main() {
    println!("{}", Square { side: 2.0 }.describe());
    println!("{}", Circle { radius: 1.0 }.describe());
    println!("{}", Strip { width: 3.0, height: 0.25 }.describe());
}
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:matching_the_trait_kata -->
*[`matching_the_trait_kata.rs`](examples/matching_the_trait_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four places an impl disagreed with its trait, four error
//! codes, and the trait left exactly as it was. The code each mistake earned is
//! named beside its fix.
//!
//!   rustc --edition 2024 matching_the_trait_kata.rs -o /tmp/mttk && /tmp/mttk

trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) -> String {
        format!("area {:.1}", self.area())
    }
}

struct Square {
    side: f32,
}

struct Circle {
    radius: f64,
}

struct Strip {
    width: f64,
    height: f64,
}

impl Shape for Square {
    // E0053 — was `-> f32`. The help line changes only the arrow, and the body
    // then fails with E0308, because `self.side * self.side` is still an f32.
    // f32 to f64 loses nothing, so `From` does the widening.
    fn area(&self) -> f64 {
        f64::from(self.side * self.side)
    }
}

impl Shape for Circle {
    // E0449 — was `pub fn`. A trait method is exactly as visible as its trait,
    // so there is nothing to declare.
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Strip {
    // E0053 — was `fn area(self)`. The trait's callers lend the value; an impl
    // cannot demand to be handed it instead.
    fn area(&self) -> f64 {
        self.width * self.height
    }
    // E0050 — was `describe(&self, digits: usize)`. The trait fixes the arity,
    // so the extra parameter moves to an inherent method with its own name.
    fn describe(&self) -> String {
        format!("strip, {}", self.describe_to(2))
    }
}

impl Strip {
    fn describe_to(&self, digits: usize) -> String {
        format!("area {:.digits$}", self.area())
    }
}

fn main() {
    let square = Square { side: 2.0 };
    let circle = Circle { radius: 1.0 };
    let strip = Strip { width: 3.0, height: 0.25 };

    println!("square.describe()    = {}", square.describe());
    println!("circle.describe()    = {}", circle.describe());
    println!("strip.describe()     = {}", strip.describe());
    println!("strip.describe_to(4) = {}", strip.describe_to(4));
}
```
<!-- /source -->

<!-- output:matching_the_trait_kata -->
*Verified output of [`matching_the_trait_kata.rs`](examples/matching_the_trait_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
square.describe()    = area 4.0
circle.describe()    = area 3.1
strip.describe()     = strip, area 0.75
strip.describe_to(4) = area 0.7500
```
<!-- /output -->

</details>

## See also

- [What a trait is](../what_a_trait_is/README.md) — required and provided methods, and the semicolon that decides which one you wrote
- ["No method named …"](../no_method_named/README.md) — the call side of the same contract: `E0599` when a method the impl should have supplied cannot be found
- [A trait must be in scope](../trait_in_scope/README.md) — `Animal::speak(&cat, 1)`, the spelling that reaches the trait's method past an inherent one
- [Extension traits](../extension_traits/README.md) — the inherent method that wins the dot call with no warning
- [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) — bounds on the `impl` rather than on the struct, which is where `E0276` comes from
- [Supertraits](../supertraits/README.md) — `E0277` on the `impl` line, when the trait requires another trait first

## Sources

- David MacLeod, *Learn Rust in a Month of Lunches* (Manning, 2024), §7.1, pp. 130–131, asks whether an impl can change `run`'s signature and answers with the return-type case. Its transcript is an older rustc's (`expected fn pointer`); 1.98 prints `expected signature`, and adds the `help:` line this page's trap is about.
- rustc's error index: [E0046 ↗](https://doc.rust-lang.org/error_codes/E0046.html) · [E0050 ↗](https://doc.rust-lang.org/error_codes/E0050.html) · [E0053 ↗](https://doc.rust-lang.org/error_codes/E0053.html) · [E0185 ↗](https://doc.rust-lang.org/error_codes/E0185.html) · [E0186 ↗](https://doc.rust-lang.org/error_codes/E0186.html) · [E0276 ↗](https://doc.rust-lang.org/error_codes/E0276.html) · [E0407 ↗](https://doc.rust-lang.org/error_codes/E0407.html) · [E0449 ↗](https://doc.rust-lang.org/error_codes/E0449.html)

## Po polsku

Cecha (*trait*) jest właścicielką **sygnatury** (*signature*), a blok `impl` odpowiada tylko za **ciało** (*body*) metody. Implementacja może więc zmienić to, *co* metoda robi, ale nazwa, `self`, typy parametrów, typ zwracany i ograniczenia (*bounds*) są przepisane z cechy, a kompilator porównuje tę kopię z oryginałem. Wolno zmienić tylko to, czego wywołujący i tak nie widzi: nazwę parametru, `mut` przy jego wiązaniu (to część wzorca, nie typu metody) oraz — co zaskakuje — poluzować ograniczenie, bo wtedy implementacja żąda od wywołującego mniej, a nie więcej.

Każdy sposób pomylenia kopii ma własny kod, więc warto je rozpoznawać po numerze. `E0046` — brakuje metody **wymaganej** (bez ciała); metoda z ciałem domyślnym nigdy nie „brakuje”. `E0407` — metoda, której cecha nie deklaruje, najczęściej literówka, a kompilator sam podpowiada właściwą nazwę. `E0053` — ta sama metoda, inny typ: parametru, wyniku albo `&mut self` zamiast `&self`. `E0050` — inna liczba parametrów. `E0185` i `E0186` — `self` tylko po jednej stronie. `E0276` — ograniczenie, którego cecha nie żądała. `E0449` — `pub` przy metodzie cechy, bo jej widoczność jest zawsze widocznością samej cechy.

Najważniejsza jest pułapka z `E0053`: podpowiedź `help:` poprawia wyłącznie sygnaturę. Kto napisze `fn speak(&self, times: u32) -> u32`, bo chce, żeby metoda coś zwracała, dostanie propozycję `-> ()` — a wtedy ciało `{ times }` kończy się błędem `E0308`, pod którym nie ma już żadnej podpowiedzi. Tego, czego chciałeś, nie da się zapisać jako *tej* metody *tej* cechy. Albo zmieniasz cechę (jeśli jest twoja — i wtedy wszystkie implementacje razem z nią), albo piszesz metodę własną typu (*inherent method*), która wygrywa wywołanie z kropką bez żadnego ostrzeżenia, podczas gdy kod generyczny nadal widzi wersję z cechy.

Kto przychodzi z Javy, ma tu dwa odwrócone nawyki. Tam `@Override` trzeba dopisać samemu, żeby literówka stała się błędem, a metoda implementująca interfejs **musi** mieć `public`. W Ruscie każda metoda w bloku `impl Trait for Type` jest sprawdzana tak, jakby miała `@Override`, a `pub` jest zabronione — z tego samego powodu, dla którego Java go wymaga.

**Szukaj po polsku:** sygnatura metody cechy · metoda wymagana i metoda domyślna · `rust E0053 incompatible type for trait` · `rust E0046 not all trait items implemented` · `rust E0407 not a member of trait` · `rust E0449 visibility qualifiers are not permitted here`
