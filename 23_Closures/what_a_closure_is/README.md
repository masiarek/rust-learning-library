# What a closure is

**Level:** 101 → 201 · working knowledge

**One line:** A closure is a function that can also see the variables around where it was written — so the compiler writes you a struct holding them, which is why every closure has a type of its own and why its size is exactly what it captured.

```rust
fn main() {
    let bonus = 10;
    let with_bonus = |n: i32| n + bonus;   // `bonus` came along
    println!("{}", with_bonus(1));         // 11
}
```

Two pipes, a parameter list, an expression. The braces are optional for a single expression, and the parameter types are usually inferred, so the same thing is often written `|n| n + bonus`.

## The capture is the whole difference

Rust already has functions, and you can declare one anywhere — including inside another function. What you cannot do is look outward from it:

```rust
fn main() {
    let bonus = 10;
    // fn with_bonus(n: i32) -> i32 { n + bonus }   // E0434
    let with_bonus = |n: i32| n + bonus;            // fine
    println!("{}", with_bonus(1));                  // 11
}
```

```text title="Real rustc output"
error[E0434]: can't capture dynamic environment in a fn item
 --> scratch.rs:3:40
  |
3 |     fn with_bonus(n: i32) -> i32 { n + bonus }
  |                                        ^^^^^
  |
  = help: use the `|| { ... }` closure form instead
```

*"Can't capture dynamic environment in a fn item"* is the whole definition read backwards. A `fn` is one item, compiled once, with no per-call baggage; a closure is an item **plus** the baggage. That is also the reason the syntax is different: `|n|` marks the thing that is allowed to look around it.

## A closure is a struct the compiler wrote

Not a metaphor. The compiler generates an anonymous type with one field per captured variable and an `impl` of the call traits on it, so a closure's size is the size of what it captured — nothing else:

```text
|n| n + 1                 captured nothing     0 bytes
move |n| n + bonus        captured one i32     4 bytes
move || name.len()        captured a String   24 bytes
|n| n + bonus             borrowed one i32     8 bytes
|| name.len()             borrowed a String    8 bytes
```

A closure that captured nothing is **zero-sized** — smaller than the 8-byte `fn` pointer you might have reached for instead, because there is nothing to point at. Passing it costs no memory traffic at all, and the call is a direct call the optimizer can inline.

The `move` rows are the ones to read twice: 24 bytes is `size_of::<String>()`, so what that closure holds is the `String` itself — pointer, length, capacity — and not a reference to it. Drop the `move` and the field becomes a reference, so both fall to one pointer wide whatever they captured. Which of the two you get is [the `move` keyword's job](../the_move_keyword/README.md).

## Every closure has its own type

If the compiler writes a fresh struct per closure, two closures cannot share a type, however identical their text:

```rust
fn main() {
    let bonus = 1;
    let flag = true;
    // let f = if flag { |x: i32| x + bonus } else { |x: i32| x + bonus };   // E0308
    let f: Box<dyn Fn(i32) -> i32> = if flag {
        Box::new(move |x: i32| x + bonus)
    } else {
        Box::new(move |x: i32| x * bonus)
    };
    println!("{}", f(1));   // 2
}
```

```text title="Abridged — real rustc output, without the file-and-line headers and the two type notes"
error[E0308]: `if` and `else` have incompatible types
  |
4 |     let f = if flag { |x: i32| x + bonus } else { |x: i32| x + bonus };
  |                       ------------------          ^^^^^^^^^^^^^^^^^^ expected closure, found a different closure
  |
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

*"No two closures, even if identical, have the same type"* is rustc's own sentence, and the help line is the fix: to put two closures in one variable, one `Vec`, or one struct field, you need a trait object — `Box<dyn Fn(i32) -> i32>` — because that is the only thing wide enough to hold both.

## What a signature can say about one

Three spellings, and the choice is between code size and indirection.

| Spelling | What it means | Cost |
|---|---|---|
| `fn f<F: Fn(i32) -> i32>(op: F)` | generic — one stamped-out copy of `f` per closure type | no indirection; larger binary |
| `fn f(op: impl Fn(i32) -> i32)` | the same thing, without naming the parameter | identical; cannot be turbofished |
| `fn f(op: Box<dyn Fn(i32) -> i32>)` | one type at run time, whatever closure is inside | an allocation and a virtual call |
| `fn f(op: fn(i32) -> i32)` | a bare function pointer — **not** a closure type | 8 bytes; refuses anything that captured |

The last row is the trap. A closure that captures nothing coerces to a `fn` pointer, so `apply_ptr(41, |n| n + 1)` compiles and reads like proof that closures are function pointers. Add one captured variable and it stops:

```text title="Abridged — real rustc output, without the file-and-line headers and the type notes"
error[E0308]: mismatched types
  |
4 |     println!("{}", apply_ptr(41, |n| n + bonus));
  |                    ---------     ^^^^^^^^^^^^^ expected fn pointer, found closure
  |
note: closures can only be coerced to `fn` types if they do not capture any variables
  |
4 |     println!("{}", apply_ptr(41, |n| n + bonus));
  |                                          ^^^^^ `bonus` captured here
```

A `fn` pointer is an address. There is nowhere in it to keep `bonus`.

## If you are coming from another language

- **Python.** `lambda n: n + bonus` is the same idea and Python's version is the looser one, which is worth knowing precisely because the code looks identical. A Python closure captures the **variable**, not the value: rebind `bonus` after building the lambda and the lambda sees the new number — the late-binding surprise everyone meets in `[lambda: i for i in range(3)]`, where all three functions return `2`. Rust captures the *place* too, but the borrow checker will not let you have it both ways: a closure holding `&bonus` blocks assignment to `bonus` while it lives, and `move` takes a copy so nothing later can change it. So the loop that misbehaves in Python is a compile error or a copy here, never a silent surprise. What transfers otherwise is nearly everything, including that `def` nested in a function *does* close over its scope — Python has no equivalent of the `fn`/`E0434` split above, since every Python function is already a closure. And Python does not distinguish the three call traits at all: a lambda is callable any number of times and nothing tracks whether it consumed what it captured, which is the next page.
- **ABAP.** There is no closure, and the honest translation is a **local class**: attributes for what would have been captured, a `constructor` that takes them, and one method — the object *is* the closure, and `NEW lcl_scorer( bonus = 10 )->apply( 1 )` is the call. That is not an analogy invented for this page; it is what the compiler does for you here, field for field. The two habits that transfer badly: a `FORM` or a method sees only its own parameters and `CLASS-DATA`/global state, exactly like the `fn` refused above, so ABAP developers reach for a global rather than a capture — and `PERFORM` cannot be passed as a value, so "supply the operation" is done in ABAP with a subclass or an interface reference. Both are the `Box<dyn Fn>` row of the table above, arrived at the long way round: an interface reference with one method **is** a boxed closure, allocation and virtual call included. What changes is that Rust hands you the zero-cost version too — `impl Fn` — which stamps out a copy per operation and calls it directly, with no object anywhere.
- **JavaScript / TypeScript.** `n => n + bonus` behaves like the Python case (captures the binding, lives on the heap, callable forever). The one thing to unlearn is that a JS closure is always a heap-allocated object with a hidden reference to its scope, so passing one is always a pointer; here it may be **zero bytes**, and where it is not, you choose whether the environment travels by reference or by value.

---

## The verified output

<!-- output:what_a_closure_is -->
*Verified output of [`what_a_closure_is.rs`](examples/what_a_closure_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The syntax, next to the function it replaces
   fn add_one(n: i32) -> i32 { n + 1 }   add_one(41)         = 42
   |n: i32| n + 1                        add_one_closure(41) = 42
   |n| n + 1        (types inferred)     inferred(41)        = 42

2. The difference is the capture: a closure can see the scope around it
   bonus = 10, so with_bonus(1) = 11
   a plain `fn` written in the same spot cannot: E0434, see the page

3. A closure IS a struct the compiler wrote — its size is what it captured
   |n| n + 1                 captures nothing   0 bytes
   move |n| n + bonus        captures one i32   4 bytes
   move || name.len()        captures a String  24 bytes
   for comparison: size_of::<String>()        = 24 bytes
   for comparison: size_of::<fn(i32) -> i32>()= 8 bytes
   a closure that captured nothing is ZERO-SIZED — smaller than a fn pointer.
   |n| n + bonus             borrows one i32    8 bytes
   || name2.len()            borrows a String   8 bytes
   the same two closures without `move`: each field is now a reference,
   so both are one pointer wide. (calling them: 11 8)
   (calling them, so the compiler keeps them: 2 11 3)

4. Which means every closure has its own anonymous type
   two closures with identical text are two different types — E0308,
   and rustc says so in as many words: "no two closures, even if
   identical, have the same type". See the page for the transcript.

5. A closure that captured nothing coerces to a plain `fn` pointer
   apply_ptr(41, |n| n + 1)          = 42
   apply_ptr(41, add_one)            = 42
   apply_ptr(41, |n| n + bonus)      -> E0308: expected fn pointer, found closure

6. Three ways to accept one, and what each costs
   fn apply(n, op: impl Fn(i32)->i32)  apply(41, |n| n + bonus) = 51
   Box<dyn Fn(i32)->i32>               boxed(41)                = 51
   size of the Box                     16 bytes (a fat pointer: data + vtable)
   `impl Fn` is one stamped-out copy per closure type — no indirection.
   `Box<dyn Fn>` is one allocation and a virtual call, and it is what
   you need the moment two different closures must share a variable.

7. Higher-order: the caller supplies the operation
   add one    applied to 20 -> 21
   double     applied to 20 -> 40
   add bonus  applied to 20 -> 30
   all three live in one Vec because `dyn Fn` erased their three types.
```
<!-- /output -->

---

## See also

- [The three closure traits](../three_closure_traits/README.md) — `Fn`, `FnMut`, `FnOnce`: which one a closure gets, and what decides it
- [The `move` keyword](../the_move_keyword/README.md) — whether the captured `String` above is the string or a borrow of it
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — the `impl Trait` / `Box<dyn Trait>` decision the table above is one instance of
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — what a capture does to a value that is not `Copy`
- [`unwrap_or_else`](../../17_Option_and_Result/unwrap_or_else/README.md) — the first closure most people write in Rust, and it is an `FnOnce`
- [What a struct is](../../16_Structs/what_a_struct_is/README.md) — the thing the compiler is writing for you here
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — where most of the closures you write actually go, and what the adapter is allowed to do with yours

## Sources

[Closures ↗](https://doc.rust-lang.org/book/ch13-01-closures.html) in the Book, and the reference's [closure expressions ↗](https://doc.rust-lang.org/reference/expressions/closure-expr.html) for the coercion rule quoted above.
