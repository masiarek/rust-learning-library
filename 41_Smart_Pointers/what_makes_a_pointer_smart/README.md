# What makes a pointer smart

[Smart pointers](../README.md) › **What makes a pointer smart**

**Level:** 201 · working knowledge

**One line:** A smart pointer is a struct that implements `Deref`, so it can be used like the thing it points at, and usually `Drop`, so something happens when it goes away — owning the pointee is common, and not part of the definition.

```rust
use std::ops::Deref;

struct Loud<T> {
    value: Box<T>,
}

impl<T> Deref for Loud<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.value }
}

impl<T> Drop for Loud<T> {
    fn drop(&mut self) { println!("letting go"); }
}

fn main() {
    let name = Loud { value: Box::new(String::from("ferris")) };
    println!("{}", name.to_uppercase());  // FERRIS — a str method, reached through two Derefs
}                                         // letting go
```

## `Deref` is the "pointer" half

std's [`Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html) docs say types that implement `Deref` or `DerefMut` "are often called smart pointers", and that deref coercion exists for them. `Deref` gives three things, and section 1 of the run uses each:

| You write | What the compiler does | Rule |
|---|---|---|
| `*greeting` | calls `*Deref::deref(&greeting)` | the `*` operator |
| `greeting.to_uppercase()` | derefs until a type with that method turns up: `Loud<String>` → `String` → `str` | [Method resolution](../../12_Traits/method_resolution/README.md) |
| `shout(&greeting)` where `shout` takes `&str` | inserts derefs at the call: `&Loud<String>` → `&String` → `&str` | [Coercion](../../29_Conversion/coercion/README.md) |

`DerefMut` does the same for `&mut`, which is how `greeting.push_str(", world")` reaches the `String`. A type with `Deref` and no `DerefMut` hands out only shared access — which is `Rc`:

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("state"));
    shared.push_str(" changed");
    println!("{shared}");
}
```

```text title="Real rustc 1.98.0 output — rc_is_not_deref_mut.rs, --edition 2024"
error[E0596]: cannot borrow data in an `Rc` as mutable
 --> rc_is_not_deref_mut.rs:5:5
  |
5 |     shared.push_str(" changed");
  |     ^^^^^^ cannot borrow as mutable
  |
  = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<String>`
```

The ways to write through an `Rc` are [`Rc::make_mut` ↗](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.make_mut), which clones when the value is shared, and a [`RefCell`](../../09_Advanced/interior_mutability/README.md) inside it.

## `Drop` is the "smart" half

Section 2 of the run shows the other half: `Loud`'s `drop` runs at the end of the block that owns it, or earlier at `drop(greeting)`. What a smart pointer does in `Drop` is its job — `Box` frees its allocation, `Rc` decrements a count and frees at zero, a `Ref` from a `RefCell` gives the borrow back. [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) has the rules for when it runs.

The Book's [chapter 15 ↗](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) puts both halves in its definition: smart pointers are usually structs that implement `Deref` and `Drop`.

## Each one derefs to something

| Smart pointer | `Deref::Target` | What its `Drop` does |
|---|---|---|
| [`Box<T>`](../../26_Collections/the_box/README.md) | `T` | frees the heap allocation |
| [`Rc<T>`](../../18_Ownership/reference_counting/README.md), `Arc<T>` | `T` | decrements the strong count; drops `T` at zero |
| [`String`](../../14_Strings/anatomy_of_a_string/README.md) | `str` | frees the byte buffer |
| [`Vec<T>`](../../26_Collections/README.md) | `[T]` | drops the elements, frees the buffer |
| [`Cow<'_, B>`](../../12_Traits/how_to_learn_to_owned/README.md) | `B` | drops the owned value, if it holds one |
| `Ref<'_, T>`, `RefMut<'_, T>` | `T` | gives the `RefCell` borrow back |
| `MutexGuard<'_, T>` | `T` | unlocks the mutex |

Section 3 of the run reads six of these targets off the types with `type_name::<P::Target>()`. `String` and `Vec` in this table surprise people; the [Rust Design Patterns ↗](https://rust-unofficial.github.io/patterns/idioms/deref.html) book calls collections smart pointers for exactly this reason, and [the claims page](../smart_pointer_claims_checked/README.md) checks what that does and does not mean.

## Owning is usual, not required

The Book says smart pointers own their data "in many cases". Section 4 shows two that do not. `Cow::Borrowed(&text)` points into `text`, which it does not own. The `Ref` guard from `cell.borrow()` never owned the `5` inside the `RefCell`; while it lives, `try_borrow_mut` fails, and its `Drop` is what makes the next one succeed.

## `Weak` has no `Deref`

A [`Weak<T>` ↗](https://doc.rust-lang.org/std/rc/struct.Weak.html) points at an `Rc`'s allocation without keeping the value alive, so there may be nothing there to deref to:

```rust
use std::rc::Rc;

fn main() {
    let strong = Rc::new(7);
    let weak = Rc::downgrade(&strong);
    println!("{}", *weak);
}
```

```text title="Real rustc 1.98.0 output — weak_has_no_deref.rs, --edition 2024"
error[E0614]: type `std::rc::Weak<{integer}>` cannot be dereferenced
 --> weak_has_no_deref.rs:6:20
  |
6 |     println!("{}", *weak);
  |                    ^^^^^ can't be dereferenced
```

You ask with `weak.upgrade()`, which returns `Option<Rc<T>>`: `Some(7)` while an `Rc` lives and `None` after the last one is dropped (section 5).

## The verified output

<!-- output:what_makes_a_pointer_smart -->
*Verified output of [`what_makes_a_pointer_smart.rs`](examples/what_makes_a_pointer_smart.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Deref: a struct that acts like a pointer
   (*greeting).len()         5   explicit: *greeting is the String
   greeting.len()            5   the method search derefs for you
   greeting.to_uppercase()   HELLO   a str method, two derefs away
   shout(&greeting)          HELLO   &Loud<String> coerced to &str at the call
   after push_str via DerefMut: "hello, world"

2. Drop: something happens when it goes
   *temp + 1 = 42
   end of block:
   drop: temp lets go of its value
   drop(greeting) before the end of main:
   drop: greeting lets go of its value

3. std's smart pointers, and what each derefs to
   String     -> str
   Vec<u8>    -> [u8]
   Box<u8>    -> u8
   Rc<u64>    -> u64
   Cow<str>   -> str
   Ref<i32>   -> i32

4. "Owns what it points at" is usually true, and not always
   Cow::Borrowed points into `text`, which it does not own: true
   while the Ref guard lives, try_borrow_mut is_err: true
   after drop(guard),         try_borrow_mut is_ok:  true
   The guard never owned the 5. Its Drop gives the borrow back.

5. Weak: a pointer to shared data with no Deref at all
   weak.upgrade()                    Some(7)
   after the last Rc is dropped      None
   `*weak` does not compile. You ask for an Rc, and may get None.
```
<!-- /output -->

## If you are coming from another language

- **C++.** The name comes from here: `std::unique_ptr` is `Box`, `std::shared_ptr` is `Arc` (its count is atomic), `std::weak_ptr` is `Weak`, and overloading `operator*` and `operator->` is `Deref`. The destructor is `Drop`. What Rust adds is that `*` on a moved-from `Box` does not compile, where a moved-from `unique_ptr` is a null pointer at run time.
- **Python.** Every Python reference is already a counted smart pointer — CPython's `Py_INCREF` and `Py_DECREF` are `Rc::clone` and `drop` — and `weakref.ref(obj)()` returning `None` is `Weak::upgrade` returning `None`. The `__enter__`/`__exit__` pair of a context manager is the closest thing to a guard like `Ref` or `MutexGuard`, with `Drop` making the `with` implicit.
- **Swift and Objective-C.** ARC is `Arc`-style counting inserted by the compiler, and `weak` references are `Weak`; Rust makes you choose `Box`, `Rc` or `Arc` per value instead of counting everything.

---

## Practice

**A pointer that counts its reads.** Write `Counted<T>`, a smart pointer that owns a `Box<T>` and counts every call to its `deref`. `deref` takes `&self`, so decide what the counter's type has to be. Give it an associated function `Counted::reads(&c)` rather than a method, and say why. Then predict the count after each of `name.len()`, `name.to_uppercase()`, `takes_str(&name)`, `&*name` and `&name` for a `Counted<String>`, and check.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_makes_a_pointer_smart_kata -->
*[`what_makes_a_pointer_smart_kata.rs`](examples/what_makes_a_pointer_smart_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a smart pointer that counts how often it is dereferenced.
//!
//!   rustc --edition 2024 what_makes_a_pointer_smart_kata.rs -o /tmp/wmpsk && /tmp/wmpsk

use std::cell::Cell;
use std::ops::Deref;

/// Owns a value and counts every `deref`. `deref` takes `&self`, so the
/// counter needs a `Cell`: a write through a shared reference.
struct Counted<T> {
    value: Box<T>,
    reads: Cell<usize>,
}

impl<T> Counted<T> {
    fn new(value: T) -> Self {
        Counted { value: Box::new(value), reads: Cell::new(0) }
    }

    /// An associated function, not a method, so that calling it cannot
    /// itself go through `deref` — the same choice std makes for
    /// `Rc::strong_count(&rc)`.
    fn reads(this: &Self) -> usize {
        this.reads.get()
    }
}

impl<T> Deref for Counted<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.reads.set(self.reads.get() + 1);
        &self.value
    }
}

fn takes_str(text: &str) -> usize {
    text.len()
}

fn main() {
    let name = Counted::new(String::from("ferris"));
    let mut seen = Vec::new();

    let _ = name.len(); //              method search: one deref, Counted -> String
    seen.push(("name.len()", Counted::reads(&name)));

    let _ = name.to_uppercase(); //     Counted -> String (by deref) -> str (by String's own Deref)
    seen.push(("name.to_uppercase()", Counted::reads(&name)));

    let _ = takes_str(&name); //         coercion: &Counted<String> -> &String -> &str
    seen.push(("takes_str(&name)", Counted::reads(&name)));

    let _ = &*name; //                   explicit
    seen.push(("&*name", Counted::reads(&name)));

    let _ = &name; //                    a reference to the pointer itself: no deref
    seen.push(("&name", Counted::reads(&name)));

    println!("Reads counted after each line:");
    for (line, total) in seen {
        println!("  {line:<22} {total}");
    }
    println!();
    println!("Each line that reads through the pointer added one call to deref, and");
    println!("`&name`, which only borrows the pointer, added none. The second hop in");
    println!("to_uppercase and takes_str, String -> str, is String's Deref, not ours.");
}
```
<!-- /source -->

<!-- output:what_makes_a_pointer_smart_kata -->
*Verified output of [`what_makes_a_pointer_smart_kata.rs`](examples/what_makes_a_pointer_smart_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Reads counted after each line:
  name.len()             1
  name.to_uppercase()    2
  takes_str(&name)       3
  &*name                 4
  &name                  4

Each line that reads through the pointer added one call to deref, and
`&name`, which only borrows the pointer, added none. The second hop in
to_uppercase and takes_str, String -> str, is String's Deref, not ours.
```
<!-- /output -->

</details>

## See also

- [What a smart pointer costs](../what_a_smart_pointer_costs/README.md) — the allocations, counts and flags behind the `Deref`
- [What a smart pointer is](../../18_Ownership/what_a_smart_pointer_is/README.md) — the outline in Ownership: the family by ownership and threads, and the `Deref`-on-a-newtype trap
- [Smart pointer claims, run](../smart_pointer_claims_checked/README.md) — "boiled away", "always owns", "Rc is a raw pointer", checked
- [Method resolution](../../12_Traits/method_resolution/README.md) — the search `name.len()` goes through
- [Coercion](../../29_Conversion/coercion/README.md) — the call-site rule `takes_str(&name)` uses
- [Step 3: Owned and borrowed are two different types](../../12_Traits/how_to_learn_to_owned/owned_and_borrowed_types/README.md) — `String: Deref<Target = str>` from the `ToOwned` side
- [Raw pointers](../../36_Pointers/raw_pointers/README.md) — the pointer a smart pointer is built on

## Sources

std's [`Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html), [`DerefMut` ↗](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) and [`Weak` ↗](https://doc.rust-lang.org/std/rc/struct.Weak.html); The Book, [ch. 15, Smart Pointers ↗](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html), as shipped with 1.98.0; [Rust Design Patterns — Collections are smart pointers ↗](https://rust-unofficial.github.io/patterns/idioms/deref.html). The two refusals are rustc 1.98.0.

## Po polsku

**Inteligentny wskaźnik** (*smart pointer*) to struktura, która implementuje `Deref` — więc można jej używać jak tego, na co wskazuje — i zwykle `Drop` — więc coś się dzieje, gdy znika. `Deref` daje operator `*`, wyszukiwanie metod przez kolejne dereferencje (`Loud<String>` → `String` → `str`) i koercję w miejscu wywołania (`&Loud<String>` → `&str`). `DerefMut` robi to samo dla `&mut`; `Rc` go nie ma, dlatego `shared.push_str(…)` kończy się błędem `E0596`.

Posiadanie wskazywanej wartości jest częste, ale nie należy do definicji: `Cow::Borrowed` wskazuje na cudzy `String`, a strażnik `Ref` z `RefCell` niczego nie posiada — jego `Drop` tylko oddaje pożyczkę. `Weak` w ogóle nie ma `Deref` (`E0614`), bo wartości może już nie być; pytasz przez `upgrade()` i dostajesz `Option<Rc<T>>`. `String` i `Vec<T>` też są inteligentnymi wskaźnikami w tym sensie: mają `Deref` (do `str` i `[T]`) i `Drop`.

**Szukaj po polsku:** inteligentne wskaźniki w Ruście · cecha Deref · koercja dereferencji · `rust smart pointer Deref Drop` · `rust Weak upgrade`
