# Function pointers

**Level:** 201 · working knowledge

**One line:** `fn(u32) -> u32` is a pointer to code and nothing else — eight bytes, no captured environment — which is why a closure that captured something can never become one, and why naming a function does not give you one either.

```rust
fn double(v: u32) -> u32 { v * 2 }

fn main() {
    let op: fn(u32) -> u32 = double;   // explicit coercion
    println!("{}", op(21));            // 42
}
```

Lowercase `fn` is doing two different jobs in that snippet. On the first line it is the **keyword** that declares a function. On the third it is a **type** — the type of a value that points at one.

## Naming a function does not give you one

`double` on its own is not of type `fn(u32) -> u32`. It has a type of its own that you cannot write down:

```text
fn(i32, i32) -> i32 {sum}
```

That is the **`fn` item** type, and it names one specific function as well as its signature. Since the function is already known, the value carries nothing at all:

```text
size_of_val(&sum)   0 bytes    the item — which function is part of the TYPE
size_of_val(&op)    8 bytes    the pointer — which function is part of the VALUE
```

The item exists so the compiler can inline and specialize a call it can see; the coercion to a pointer is what throws that knowledge away in exchange for a value you can store. Most of the time it happens silently on the way into a `fn`-typed slot, which is why the distinction only surfaces when you try to do something to the item itself:

```rust
fn sum(x: i32, y: i32) -> i32 { x + y }

fn main() {
    let op1 = sum;
    let op2 = sum;
    // println!("{}", op1 == op2);              // E0369 — both are fn ITEMS
    println!("{}", op1(2, 3) == op2(2, 3));     // true — comparing two RESULTS

    let p1: fn(i32, i32) -> i32 = sum;          // coerce, and `==` compiles
    let p2: fn(i32, i32) -> i32 = sum;
    println!("{}", std::ptr::fn_addr_eq(p1, p2));   // true
}
```

```text title="Abridged — real rustc output for fn_items.rs, without the file-and-line header"
error[E0369]: binary operation `==` cannot be applied to type `fn(i32, i32) -> i32 {sum}`
  |
6 |     println!("{}", op1 == op2);
  |                    --- ^^ --- fn(i32, i32) -> i32 {sum}
  |                    |
  |                    fn(i32, i32) -> i32 {sum}
  |
help: use parentheses to call these
```

*"Use parentheses to call these"* is the compiler guessing you meant to compare two **results**. It is a good guess, because comparing two functions is rarely what anyone wants.

### `==` compiles on the pointer — and the compiler argues anyway

`fn` pointers do implement `PartialEq`, so the coerced version above compiles. Since Rust 1.85 it also warns, on by default:

```text title="Real rustc output"
warning: function pointer comparisons do not produce meaningful results since their addresses are not guaranteed to be unique
   = note: the address of the same function can vary between different codegen units
   = note: furthermore, different functions could have the same address after being merged together
   = note: `#[warn(unpredictable_function_pointer_comparisons)]` on by default
help: refactor your code, or use `std::ptr::fn_addr_eq` to suppress the lint
```

Both notes are real behaviours, not theoretical ones: the same function compiled into two codegen units gets two addresses, and two functions with identical bodies get merged into one. So `==` on `fn` pointers answers a question with no stable answer, and [`std::ptr::fn_addr_eq` ↗](https://doc.rust-lang.org/std/ptr/fn.fn_addr_eq.html) exists to say *"I know, I want the address comparison anyway."* Books written before 1.85 — including *Effective Rust* — show `assert!(op1 == op2)` as the way to demonstrate that `fn` implements `Eq`. The trait is still implemented; the demonstration now comes with a lecture.

## A method is a value too

The `fn` item type is what `str::len` *is*, so a method can be handed straight to something expecting a function:

```rust
fn main() {
    let len: fn(&str) -> usize = str::len;
    println!("{:?}", ["Ada", "Ben", "Cara"].map(len));            // [3, 3, 4]
    println!("{:?}", ["a", "bb"].map(String::from));              // ["a", "bb"]
    println!("{:?}", [1, 2].map(Some));                           // [Some(1), Some(2)]
}
```

Three different things, one mechanism. `str::len` is a **method** — [the dot is sugar for naming the type](../../16_Structs/impl_blocks/README.md), so `s.len()` and `str::len(s)` are the same call and the second one is a value when you stop short of the parentheses. `String::from` is an **associated function**: no receiver, so nothing about it is special here. And `Some` is an **enum variant** carrying a field, which Rust implements as a function from the field to the enum — which is why `.map(Some)` works and reads better than `.map(|x| Some(x))`.

That is the concrete answer to *"what is the difference between a function and a method"* at the value level: none. The difference is entirely in the **first parameter** — a method has a `self` receiver and can therefore be called with a dot — and it disappears the moment you name one without calling it.

## Nothing can ride along

This is the whole limitation, and the book's own example is the clearest form of it:

```rust
pub fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32) {
    for value in data {
        *value = mutator(*value);
    }
}

fn add2(v: u32) -> u32 { v + 2 }

fn main() {
    let mut data = vec![1, 2, 3];
    modify_all(&mut data, add2);
    println!("{data:?}");           // [3, 4, 5]

    // let amount_to_add = 3;
    // modify_all(&mut data, |y| y + amount_to_add);   // E0308 — it captured
    modify_all(&mut data, |y| y + 3);                  // fine — it captured nothing
    println!("{data:?}");           // [6, 7, 8]
}
```

```text title="Abridged — real rustc output for modify_all.rs, without the file-and-line headers"
error[E0308]: mismatched types
   |
10 |     modify_all(&mut data, |y| y + amount_to_add);
   |     ----------            ^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found closure
   |
   = note: expected fn pointer `fn(u32) -> u32`
                 found closure `{closure@modify_all.rs:10:27: 10:30}`
note: closures can only be coerced to `fn` types if they do not capture any variables
   |
10 |     modify_all(&mut data, |y| y + amount_to_add);
   |                                   ^^^^^^^^^^^^^ `amount_to_add` captured here
```

The note is the rule, stated by the compiler: **a closure coerces to `fn` exactly when it captured nothing.** [A closure is a struct the compiler wrote](../what_a_closure_is/README.md), and a `fn` pointer has nowhere to put the struct. Capture nothing and the struct is zero-sized, so there is nothing to lose in the conversion.

## So take an `Fn` bound, not a `fn` parameter

Swap the parameter for a trait bound and the same function accepts strictly more callers:

```rust
pub fn modify_all<F: FnMut(u32) -> u32>(data: &mut [u32], mut mutator: F) {
    for value in data {
        *value = mutator(*value);
    }
}
```

| passed in | `mutator: fn(u32) -> u32` | `F: FnMut(u32) -> u32` |
|---|---|---|
| a named function | ✅ | ✅ |
| a `fn` pointer variable | ✅ | ✅ |
| a closure capturing nothing | ✅ | ✅ |
| a closure capturing something | ❌ E0308 | ✅ |

The right column is a superset because **every `fn` implements all three `Fn` traits** — a function pointer borrows nothing from any environment, so there is nothing it could mutate or consume, and it qualifies for the strictest rung of [the ladder](../three_closure_traits/README.md) and therefore all of them. The reverse does not hold. So the advice is one-directional: prefer an `Fn*` bound over a bare `fn` parameter, and take [the loosest rung your body can live with](../three_closure_traits/README.md).

## Where a bare `fn` still wins

A `fn` pointer is **one concrete type**, and that is a thing neither a generic parameter nor a closure can be. Two closures with identical text are two types, so they cannot share an array, a map, or a struct field without `Box<dyn Fn>` — a heap allocation and a [virtual call](../../12_Traits/static_vs_dynamic_dispatch/README.md). A `fn` pointer needs neither:

```rust
fn double(v: u32) -> u32 { v * 2 }
fn halve(v: u32) -> u32 { v / 2 }

struct Rule { name: &'static str, apply: fn(u32) -> u32 }

fn main() {
    let table: [fn(u32) -> u32; 2] = [double, halve];      // 16 bytes, no allocation
    println!("{:?}", table.map(|f| f(8)));                 // [16, 4]

    const OP: fn(u32) -> u32 = double;                     // a const — `dyn Fn` cannot be
    let r = Rule { name: "double", apply: double };        // a field with no lifetime parameter
    println!("{} of 21 = {}", r.name, (r.apply)(21));      // double of 21 = 42
}
```

A dispatch table of same-signature operations — parsers keyed by extension, tally rules keyed by method name, a jump table read out of config — is the case where `fn` is the right tool rather than the limiting one. And it is never null, so it gets [the niche optimization](../../17_Option_and_Result/nullable_pointers/README.md): `Option<fn(u32) -> u32>` is 8 bytes, the same as the pointer.

## If you are coming from another language

**Python.** Functions are already objects, so `f = str.lower` and `map(len, words)` are the same move as the section above, and `Some` as a value is `map(Optional, …)` if `Optional` were a callable. What Python has no equivalent of is the split this page is about. A Python function object *always* carries its `__closure__` cell, whether or not anything is in it — there is no cheaper form for the non-capturing case, and no type that refuses the capturing one. So the CPython answer to `modify_all` is simply "pass the lambda", and the E0308 above has no Python counterpart at all.

The nearest Python experience is the one place the difference leaks: `functools.partial(add, 3)` exists because you sometimes need a callable that carries data *and* is a plain value you can store. In Rust that is `Box<dyn Fn>`, and `fn` is what you use when there is nothing to carry — a distinction Python makes at runtime by allocating either way.

```python
ops = {"double": lambda v: v * 2, "halve": lambda v: v // 2}   # a dict of closures
ops["halve"](8)                                                 # 4
```

That dict is `HashMap<&str, fn(u32) -> u32>` in the `fn` case and `HashMap<String, Box<dyn Fn(u32) -> u32>>` the moment one of the lambdas captures. Python spends the allocation unconditionally; Rust makes you say which you are building, and the first one has no allocation in it.

**ABAP.** There is no function value at all — a `FORM`, a `FUNCTION` and a `METHOD` are names, not things you can put in a variable. The two workarounds are worth lining up against the Rust ones, because they are the two halves of this page:

| ABAP | Rust | What it costs |
|---|---|---|
| `PERFORM (lv_name) IN PROGRAM (lv_prog)` — dispatch by name at runtime | `fn(…) -> …` in a `HashMap` | ABAP resolves a string every call and fails at runtime if it is wrong; Rust resolved it at compile time |
| `CALL METHOD lo_handler->process( )` through an `INTERFACE` reference | `Box<dyn Fn(…)>` or `&dyn Trait` | Both are a virtual call through a reference. ABAP needs a class to exist first |

The interface-reference form is the one an ABAP developer reaches for by habit, and it is the *heavier* of the two Rust options. If the thing you are passing is genuinely just a function — no state, no object — the `fn` type is the one with nothing in it, and there is no ABAP construct that light.

What ABAP does have and Rust does not: dynamic dispatch **by name**, on a string computed at runtime. `fn` pointers are values, not names, so the map above has to be built by code that mentions every function in it. That is a real loss of flexibility and a real gain in safety — a typo in the key is a `None` you must handle, never a short dump in production.

## The verified output

<!-- output:function_pointers -->
*Verified output of [`function_pointers.rs`](examples/function_pointers.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Naming a function does not give you a `fn` pointer
   size_of_val(&sum)                 = 0 bytes   the fn ITEM, `fn(i32, i32) -> i32 {sum}`
   size_of_val(&op)                  = 8 bytes   the fn POINTER, after coercion
   op(2, 3) = 5   sum(2, 3) = 5   same code, two types
   the item is zero-sized because the function it names is already known;
   the pointer is 8 bytes because which function it names is not.

2. The item type cannot be compared, stored or passed around
   let op1 = sum; let op2 = sum; op1 == op2
     -> E0369: binary operation `==` cannot be applied to
               type `fn(i32, i32) -> i32 {sum}`   (see the page)
   Coerce first and it compiles — but the compiler now argues with you:
     a == b   -> warning: unpredictable_function_pointer_comparisons
     std::ptr::fn_addr_eq(a, b) = true
     std::ptr::fn_addr_eq(a, c) = false

3. A method is a value too — the dot is the sugar, not the function
   let len: fn(&str) -> usize = str::len;
   ["Ada", "Ben", "Cara"].map(len) = [3, 3, 4]
   .map(String::from)              = ["a", "bb"]   an associated function
   .map(Some)                      = [Some(1), Some(2)]   an enum variant is one too

4. What a `fn` pointer can carry: nothing
   apply_ptr(10, double)      = 20
   apply_ptr(10, |v| v + 1)   = 11   a closure that captured nothing coerces
   apply_ptr(10, |v| v + bonus)
     -> E0308: expected fn pointer, found closure
        note: closures can only be coerced to `fn` types if they
              do not capture any variables

5. So take an `Fn` bound, not a `fn` parameter
   apply_bound(10, double)        = 20   a fn item
   apply_bound(10, a)             = 20   a fn pointer
   apply_bound(10, |v| v + 1)     = 11   a free closure
   apply_bound(10, |v| v + bonus) = 15   a CAPTURING closure
   Every `fn` implements Fn, FnMut and FnOnce — it borrows nothing, so
   it satisfies all three. The bound accepts four callers; `fn` accepts two.

6. Where a bare `fn` still wins: it is ONE concrete type
   [fn(u32) -> u32; 2] applied to 8 = [16, 4]
   size_of_val(&table)              = 16 bytes   two pointers, no allocation
   HashMap<&str, fn(u32) -> u32>    -> by_name["halve"](8) = 4
   struct Rule { name, apply }      -> double of 21 = 42
   size_of::<Rule>()                = 24 bytes
   const OP: fn(u32) -> u32         -> OP(3) = 6
   A `dyn Fn` needs a Box and a vtable to do any of this. A generic
   parameter cannot do it at all: two closures are two types.

7. And it is never null, so `Option` costs nothing
   size_of::<fn(u32) -> u32>()         = 8 bytes
   size_of::<Option<fn(u32) -> u32>>() = 8 bytes   same — None is the niche
```
<!-- /output -->

## See also

- [What a closure is](../what_a_closure_is/README.md) — the struct the compiler writes, and why a capturing one has nowhere to fit
- [The three closure traits](../three_closure_traits/README.md) — the ladder `fn` sits at the top of
- [`impl` blocks](../../16_Structs/impl_blocks/README.md) — associated function vs method, and the dot that is sugar
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — `impl Fn` vs `Box<dyn Fn>` when the function is the return value
- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — what the `Box<dyn Fn>` alternative actually costs
- [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) — writing `F: FnMut(u32) -> u32` in the three places it can go
- [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) — why `Option<fn(…)>` is free
- [`std::ptr::fn_addr_eq` ↗](https://doc.rust-lang.org/std/ptr/fn.fn_addr_eq.html) · [The Reference: function pointer types ↗](https://doc.rust-lang.org/reference/types/function-pointer.html) · [The Reference: function item types ↗](https://doc.rust-lang.org/reference/types/function-item.html)

## Po polsku

Małe `fn` gra w Ruscie dwie różne role i stąd bierze się pierwsze nieporozumienie: raz jest **słowem kluczowym** deklarującym funkcję, a raz **typem** — `fn(u32) -> u32`, czyli wskaźnikiem na funkcję (*function pointer*). Kto zna C, ma tu przewagę, bo „wskaźnik do funkcji” to dokładnie ta sama rzecz: osiem bajtów wskazujących na kod i nic poza tym. Niespodzianka jest gdzie indziej — samo **nazwanie** funkcji jeszcze nie daje takiego wskaźnika. `sum` ma własny typ, którego nie sposób zapisać w kodzie, `fn(i32, i32) -> i32 {sum}`, i ten typ waży **0 bajtów**, bo informacja o tym, którą konkretnie funkcję nazywa, siedzi w *typie*, a nie w *wartości*. Dopiero niejawna konwersja (*coercion*) przenosi ją do wartości — i stąd te 8 bajtów. Polszczyzna nie ma na to rozróżnienie żadnej nazwy, więc mów wprost `fn` item i `fn` pointer; kompilator i tak wypisze je po angielsku.

Różnica ujawnia się dopiero wtedy, gdy próbujesz coś z takim elementem zrobić. `op1 == op2` na dwóch `fn` items to `E0369`, a podpowiedź *„use parentheses to call these”* jest zgadywaniem kompilatora, że chodziło o porównanie **wyników** — zgadywaniem trafnym, bo porównywanie funkcji rzadko komukolwiek jest potrzebne. Po konwersji na wskaźnik `==` się kompiluje, ale od Rusta 1.85 domyślnie ostrzega (`unpredictable_function_pointer_comparisons`), i nie jest to czepialstwo: ta sama funkcja trafiająca do dwóch `codegen units` dostaje dwa adresy, a dwie funkcje o identycznym ciele bywają scalane w jedną. Adres nie jest więc tożsamością funkcji. Kto naprawdę chce porównać adresy, ma do tego `std::ptr::fn_addr_eq`. Uwaga na starsze książki (również *Effective Rust*), w których `assert!(op1 == op2)` jest pokazane jako dowód, że `fn` implementuje `Eq` — cecha (*trait*) nadal jest zaimplementowana, zmieniło się tylko to, że demonstracja przychodzi teraz z wykładem.

Zdanie, dla którego warto tę stronę przeczytać, wypisuje sam kompilator: **domknięcie (*closure*) da się przekonwertować na `fn` dokładnie wtedy, gdy niczego nie przechwyciło.** Domknięcie jest strukturą wygenerowaną przez kompilator, a we wskaźniku na funkcję nie ma miejsca na żadną strukturę — wystarczy jedno przechwycone pole i dostajesz `E0308` z notatką *„closures can only be coerced to `fn` types if they do not capture any variables”*, wskazującą palcem przechwyconą zmienną. Praktyczny wniosek jest jednokierunkowy: we własnych sygnaturach pisz ograniczenie cechy (*trait bound*) `F: FnMut(u32) -> u32`, a nie parametr typu `fn(u32) -> u32`. Wersja z ograniczeniem przyjmuje ściśle więcej wywołujących, a jedyny wiersz tabeli, w którym `fn` przegrywa, to właśnie domknięcie, które coś przechwyciło — bo każdy `fn` implementuje wszystkie trzy cechy `Fn` / `FnMut` / `FnOnce` naraz: skoro nic nie pożycza z otoczenia, to nie ma tam czego zmienić ani skonsumować.

Goły `fn` nie jest jednak wersją uboższą — bywa dokładnie tym, czego trzeba, i to z jednego powodu: jest **jednym konkretnym typem**. Dwa domknięcia o identycznej treści to dwa różne typy, więc nie zmieszczą się razem w tablicy, w `HashMap` ani w polu struktury bez `Box<dyn Fn>`, czyli bez alokacji na stercie i skoku przez tablicę metod wirtualnych. Wskaźnik na funkcję wchodzi tam bez żadnego z tych kosztów: `[fn(u32) -> u32; 2]` to 16 bajtów, pole `apply: fn(u32) -> u32` nie wciąga do struktury parametru czasu życia, `const OP: fn(u32) -> u32` jest legalne (a `dyn Fn` nie), i skoro taki wskaźnik nigdy nie jest pusty, `Option<fn(u32) -> u32>` waży te same 8 bajtów. Tablica rozsyłająca — parsery po rozszerzeniu pliku, reguły liczenia głosów po nazwie metody — to właśnie ten przypadek.

**Szukaj po polsku:** wskaźnik do funkcji · wskaźniki na funkcje w Ruscie · `rust fn item vs fn pointer` · `rust closure coerce fn pointer` · `rust unpredictable_function_pointer_comparisons`
