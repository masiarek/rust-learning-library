# Coercion: the conversion you never write

**Level:** 201 · working knowledge

**One line:** Four of Rust's conversions are things you type; the fifth is one the compiler performs silently at a handful of named places — and knowing where those places are is what turns *"why does `&owned` work here and not there"* into a rule.

```rust
fn width(s: &str) -> usize {
    s.len()
}

fn main() {
    let owned = String::from("ballot");
    println!("{}", width(&owned));   // 6
}
```

`&owned` is a `&String`. The function wants a `&str`. Nothing on that line says so, no method is called, and it compiles — because a function argument is a **coercion site**, and `String` derefs to `str`.

## The list is short and closed

| From | To | Called |
|---|---|---|
| `&T` | `&U` where `T: Deref<Target = U>` | deref coercion |
| `&mut T` | `&mut U` where `T: DerefMut<Target = U>` | deref coercion |
| `&mut T` | `&T` | — |
| `&[T; N]` | `&[T]` | unsizing |
| `&T` / `Box<T>` | `&dyn Trait` / `Box<dyn Trait>` | unsizing |
| a `fn` item, or a closure that captures nothing | `fn(…) -> …` pointer | — |
| `!` (from `return`, `break`, `panic!`) | any type | — |

That is nearly all of it. Everything else in Rust is a conversion you write: [`From`/`Into`](../from_and_into/README.md), [`TryFrom`](../tryfrom_and_tryinto/README.md), [`as`](../casting_with_as/README.md), or a named method. The [Reference's coercion chapter ↗](https://doc.rust-lang.org/reference/type-coercions.html) is the full statement of it.

**There is no numeric promotion.** Not on the list, not anywhere:

```text title="Abridged — real rustc output; a second error (E0277, cannot add u16 to u8) follows it"
error[E0308]: mismatched types
 --> no_coercion.rs:4:23
  |
4 |     let sum = small + big;
  |                       ^^^ expected `u8`, found `u16`
```

C promotes, Java widens, Python has one integer type. Rust makes you write `u16::from(small) + big`, and that is the single most common reason a newcomer thinks coercion is broader than it is.

## Coercion happens at *sites*, not everywhere

A coercion needs a target type to aim at, so it fires only where the compiler already knows what it wants:

```rust
let s: &str = &owned;                       // a let with a type annotation
fn give_back(s: &String) -> &str { s }      // a return value
struct Holder<'a> { text: &'a str }
let h = Holder { text: &owned };            // a struct or enum field
width(&owned);                              // a function or method argument
```

Take the annotation off the `let` and there is nothing to coerce *to*, so `s` is simply a `&String`. That is not a failure — it is the same rule, with no target.

## Three places it will not help you

**Inside another type.** A coercion adjusts one expression; it never reaches into a generic to adjust a parameter:

```text title="Abridged — real rustc output, without the trailing help block"
error[E0308]: mismatched types
 --> option_coercion.rs:6:26
  |
6 |     println!("{}", takes(held));
  |                    ----- ^^^^ expected `Option<&str>`, found `Option<&String>`
  |                    |
  |                    arguments to this function are incorrect
  |
  = note: expected enum `Option<&str>`
             found enum `Option<&String>`
```

`Option<&String>` and `Option<&str>` are unrelated types, and the same goes for `Vec<String>` against `Vec<&str>`. The repairs are named methods — [`as_deref` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref) for the `Option`, `.iter().map(String::as_str).collect()` for the `Vec` — and each element really is converted, because nothing else could be true.

**At a generic parameter.** `T` is inferred from the first argument that mentions it, and the rest have to already *be* that type:

```text title="Abridged — real rustc output, without the note and the fn-defined-here block"
error[E0308]: mismatched types
 --> generic_position.rs:5:35
  |
5 |     println!("{}", longer(&owned, "vote"));
  |                    ------ ------  ^^^^^^ expected `&String`, found `&str`
  |                    |      |
  |                    |      expected all arguments to be this `&String` type because they need to match the type of this parameter
```

The signature is `fn longer<T: AsRef<str>>(a: T, b: T)`, so both arguments share one `T`. Pass `owned.as_str()` first and the whole call works. This is the everyday cost of a generic parameter over a plain `&str`, and it is worth knowing before you reach for `impl AsRef<str>` in a signature.

**On a value rather than a reference.** Deref coercion adjusts references:

```rust
width(&owned);   // fine
// width(owned)  // error[E0308]: expected `&str`, found `String`
```

`String` is not `&String`, so there is nothing for `Deref` to work through. The compiler's `help: consider borrowing here` is telling you exactly that.

## Method calls are a *different* rule that looks the same

```rust
owned.len();     // String has no `len`… except it does, via str
v.first();       // Vec has no `first`… except it does, via [T]
```

That is **auto-deref on the receiver**: for a method call, the compiler tries `T`, then `&T`, then `&mut T`, then dereferences and tries again, until a method is found. It is a separate mechanism from coercion — it applies to a receiver instead of a site, and it inserts `&`/`*` instead of changing a type — but it is powered by the same `Deref` impls, which is why the two are so easily confused. The visible difference: auto-deref needs no target type, so it works in expressions where coercion has nothing to aim at.

## If you are coming from another language

- **C.** Coercion is the opposite of C's conversion story in both directions. C converts *values* implicitly and constantly — integer promotion, the usual arithmetic conversions, array-to-pointer decay, anything to `void *` — while Rust converts almost no values implicitly and instead adjusts *reference types* at a fixed list of sites. The one that maps cleanly is decay: `&[i32; 4]` → `&[i32]` is C's array-to-pointer, except the length travels with it instead of being lost. And `char c = 300;` compiling is exactly what Rust's missing promotion is there to stop.
- **C++.** You know this as implicit conversion, and Rust's version is deliberately far smaller — no user-defined conversion operators, no converting constructors, no `explicit` keyword needed because nothing is implicit in the first place. `Deref` looks like `operator->` chaining and does a similar job for smart pointers; the difference is that implementing `Deref` for a type that is not a pointer is considered an abuse in Rust, precisely because it makes methods appear from nowhere. If you have been bitten by an overload resolved through two user-defined conversions, that is the failure mode Rust's closed list rules out.
- **Python.** Duck typing means the question never arises: you pass whatever object has the method. The nearest thing to coercion is `__str__`/`__index__` being called for you at particular syntactic positions, which is the same shape — a *site*, not a general rule. What transfers badly is the assumption that "a string-like thing is a string": `str` and `String` are two types here, and the compiler's willingness to bridge them at a call site is a specific, enumerable favour rather than a general tolerance.
- **ABAP.** ABAP converts almost anything to almost anything on assignment — `MOVE` between different types is normal, numeric strings become numbers, and the conversion rules fill a chapter of the reference. Rust does the reverse: assignment converts nothing, and the four written conversions plus this list are the whole of it. The habit to unlearn is treating a type mismatch as something the runtime will smooth over; the habit to keep is that `MOVE-CORRESPONDING`-style widening is what [`From`](../from_and_into/README.md) is for, spelled out.

## The verified output

<!-- output:coercion -->
*Verified output of [`coercion.rs`](examples/coercion.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Seven coercions, none of them written down
     &String  -> &str      width(&owned)      = 6
     &Vec<i32>-> &[i32]    count(&v)          = 3
     &[i32; 4]-> &[i32]    count(&arr)        = 4
     &mut Vec -> &Vec      peek(&mut m)       = 7
     fn item  -> fn ptr    apply(double, 21)  = 42
     Box<Square> -> Box<dyn Shape>            = 9
     &u8      -> &dyn Debug                   = 42
   Every one of those arguments was written as `&x` and arrived as
   a different type. Nothing on the call site says so.

2. It happens at named places, not everywhere
     let with a type          let s: &str = &owned;   -> ballot
     a return value           fn f(s: &String) -> &str -> ballot
     a struct field           Holder { text: &owned } -> ballot
     ...plus function arguments, as above. Those are `coercion
     sites`: the compiler only looks for one where it already
     knows the type it wants.

3. Where it does not happen
     Option<&String> -> Option<&str>   : no. Coercion does not
       reach inside another type. Fix: `.as_deref()`, or
       `.map(|s| s.as_str())`.
       held.as_deref() = Some("x")
     u8 + u16                         : no. Rust has no numeric
       promotion at all. Fix: `u16::from(small) + big`.
       u16::from(small) + big = 3
     longer(&owned, "vote")           : no. A generic parameter
       is not a coercion site; `T` is inferred from the first
       argument and the second must already be that type.
       longer(owned.as_str(), "vote") = true
     width(owned)                     : no. Deref coercion works
       on references; `String` by value is not `&String`.
       width(&owned) = 6

4. Method calls look like coercion and are a separate rule
     owned.len()      = 6   <- auto-deref: String, then str
     v.first()        = Some(10)   <- auto-deref: Vec, then [i32]
     (&owned).len()   = 6   <- auto-deref again, through &
   The receiver of a method call gets `&`, `&mut` and `*` inserted
   until something fits. That is why `v.first()` finds a slice
   method — and why removing a `&` often changes nothing.

5. What to write when nothing fires for you
     &String -> &str      &owned  ·  owned.as_str()
     Vec<String> -> Vec<&str>          v.iter().map(String::as_str)
     Option<String> -> Option<&str>    held.as_deref()
     Vec<T>  -> &[T]      &v      ·  v.as_slice()
     u8      -> u16       u16::from(small)
   The named method is never wrong, and it is what to reach for
   the moment the type sits inside anything else.
```
<!-- /output -->

## Practice

**Six call sites, and three of them need a repair.** Write three functions — one taking `&str`, one taking `&[i32]`, one taking `Option<&str>` — and try to call them with a `String`, a `Vec<i32>`, an array literal, a `String` by value, an `Option<String>`, and a `Vec<String>`.

Three compile with nothing written at the call site. For each of the other three, say which rule stopped it — *not a reference*, *inside another type*, or *a generic parameter* — and write the smallest repair. Two of the three repairs are one method call; one of them cannot be, and the reason is worth stating in a sentence.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:coercion_kata -->
*[`coercion_kata.rs`](examples/coercion_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: six call sites, three of which need a repair.
//!
//!   rustc --edition 2024 coercion_kata.rs -o /tmp/coek && /tmp/coek

fn takes_str(s: &str) -> usize {
    s.len()
}

fn takes_slice(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn takes_opt(o: Option<&str>) -> bool {
    o.is_some()
}

fn main() {
    let owned = String::from("ballot");
    let counts: Vec<i32> = vec![4, 5, 6];
    let held: Option<String> = Some(String::from("Ada"));
    let names: Vec<String> = vec!["Ada".to_string(), "Ben".to_string()];

    println!("Compiles as written — a coercion fires at the argument:");
    println!("  1. takes_str(&owned)          = {}", takes_str(&owned));
    println!("  2. takes_slice(&counts)       = {}", takes_slice(&counts));
    println!("  3. takes_slice(&[1, 2, 3])    = {}", takes_slice(&[1, 2, 3]));
    println!("     &String -> &str and &Vec<i32> -> &[i32] are deref coercions;");
    println!("     &[i32; 3] -> &[i32] is an unsizing coercion. All three are");
    println!("     invisible at the call site.");

    println!("\nRejected as written, and the repair:");

    // 4.  takes_str(owned)
    //     error[E0308]: mismatched types — expected `&str`, found `String`
    //     Coercion works on references; a value is not one.
    println!("  4. takes_str(owned)      -> takes_str(&owned)          = {}",
             takes_str(&owned));

    // 5.  takes_opt(held.as_ref())
    //     error[E0308]: expected `Option<&str>`, found `Option<&String>`
    //     Coercion does not reach inside another type.
    println!("  5. takes_opt(held.as_ref()) -> takes_opt(held.as_deref()) = {}",
             takes_opt(held.as_deref()));

    // 6.  takes_str(names)  /  passing Vec<String> where Vec<&str> is wanted
    //     Same reason as 5: a Vec is another type, and its element type is
    //     never adjusted for you. Every element has to be converted.
    let borrowed: Vec<&str> = names.iter().map(String::as_str).collect();
    println!("  6. Vec<String> -> Vec<&str> -> {:?}, first is {} chars",
             borrowed, takes_str(borrowed[0]));

    println!("\nThe rule the three failures share: a coercion adjusts the type of");
    println!("an expression at a site where the wanted type is already known, and");
    println!("it never descends into a generic type to adjust a parameter. The");
    println!("moment your value is INSIDE something — an Option, a Vec, a tuple —");
    println!("you write the conversion yourself.");
}
```
<!-- /source -->

<!-- output:coercion_kata -->
*Verified output of [`coercion_kata.rs`](examples/coercion_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Compiles as written — a coercion fires at the argument:
  1. takes_str(&owned)          = 6
  2. takes_slice(&counts)       = 15
  3. takes_slice(&[1, 2, 3])    = 6
     &String -> &str and &Vec<i32> -> &[i32] are deref coercions;
     &[i32; 3] -> &[i32] is an unsizing coercion. All three are
     invisible at the call site.

Rejected as written, and the repair:
  4. takes_str(owned)      -> takes_str(&owned)          = 6
  5. takes_opt(held.as_ref()) -> takes_opt(held.as_deref()) = true
  6. Vec<String> -> Vec<&str> -> ["Ada", "Ben"], first is 3 chars

The rule the three failures share: a coercion adjusts the type of
an expression at a site where the wanted type is already known, and
it never descends into a generic type to adjust a parameter. The
moment your value is INSIDE something — an Option, a Vec, a tuple —
you write the conversion yourself.
```
<!-- /output -->

</details>

## See also

- [`From` and `Into`](../from_and_into/README.md) — the conversion you write, and the one that arrives free
- [Casting with `as`](../casting_with_as/README.md) — the built-in one that never fails and silently loses data
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — the pair this fires on more than any other
- [`ToOwned`](../../12_Traits/to_owned/README.md) — going the other way, from borrowed to owned
- [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) — the unsizing coercion, from the array's side
- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — where `Box<T>` → `Box<dyn Trait>` leads

## Po polsku

Konwersje w Ruscie pisze się ręcznie — `From`/`Into`, `TryFrom`, `parse`, `as` — z jednym wyjątkiem: **koercja** (*coercion*), którą kompilator wykonuje sam, po cichu, w kilku ściśle wyliczonych miejscach. Dlatego `width(&owned)` działa, choć `&owned` to `&String`, a funkcja chce `&str`.

Lista koercji jest **zamknięta i krótka**: `&T` → `&U`, gdy `T: Deref<Target = U>` (stąd `&String` → `&str` i `&Vec<T>` → `&[T]`), `&mut T` → `&T`, `&[T; N]` → `&[T]`, typ konkretny → `dyn Trait`, funkcja → wskaźnik na funkcję, oraz typ `!` → cokolwiek. **Nie ma na niej żadnej konwersji liczbowej** — `u8 + u16` się nie skompiluje, bo Rust nie zna promocji typów całkowitych znanej z C. To jest najczęstsza przyczyna wrażenia, że koercja „powinna” działać szerzej.

Koercja potrzebuje **celu**, więc zachodzi tylko tam, gdzie kompilator już wie, jakiego typu oczekuje: przy argumencie funkcji, przy `let` z adnotacją typu, w wartości zwracanej i w polu struktury. Zdejmij adnotację z `let`, a nie ma do czego konwertować — i to nie jest błąd, tylko ta sama reguła bez celu.

Trzy miejsca, w których nie pomoże: **wewnątrz innego typu** (`Option<&String>` to nie `Option<&str>` — trzeba `as_deref()`), **przy parametrze generycznym** (`T` jest wywnioskowane z pierwszego argumentu i reszta musi już być tego typu) oraz **na wartości zamiast referencji** (`String` to nie `&String`). Osobną, choć myloną z koercją regułą jest automatyczne dereferencjonowanie **odbiorcy metody**: `owned.len()` znajduje metodę z `str`, bo kompilator próbuje kolejno `T`, `&T`, `&mut T` i dereferencji — bez żadnego typu docelowego.

**Szukaj po polsku:** koercja typów w Ruscie · `rust deref coercion` · `String` a `&str` · `as_deref` · promocja typów liczbowych
