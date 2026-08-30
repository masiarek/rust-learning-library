# Static vs dynamic dispatch

**Level:** 201 → 301 · working knowledge

**One line:** `<P: Processor>` and `&dyn Processor` say almost the same thing and compile to entirely different programs — one function stamped out per type with direct calls, or one shared function that looks the answer up in a vtable.

```rust
trait Processor { fn compute(&self, x: i64) -> i64; }

fn run_static<P: Processor>(p: &P, x: i64) -> i64 { p.compute(x) }   // one copy per type
fn run_dynamic(p: &dyn Processor, x: i64) -> i64 { p.compute(x) }    // one copy, ever

struct Risc;
impl Processor for Risc { fn compute(&self, x: i64) -> i64 { x + 42 } }

fn main() {
    println!("{} {}", run_static(&Risc, 1), run_dynamic(&Risc, 1));  // 43 43
}
```

Both lines print `43`. Correctness is never what is at stake here.

## What the compiler does with each

**Static.** `run_static` is not a function — it is a *template*. Call it with a `Risc` and a `Cisc` and the compiler emits two functions, each with that type's `compute` called directly and available for inlining. This is **monomorphization**, and it is the same machinery behind `Vec<T>`. You pay in compiled size: N types means N copies.

**Dynamic.** `run_dynamic` is one function, compiled once. Its argument is a **fat pointer** — two words, one to the value and one to a **vtable** for that value's type. Calling `compute` loads a function pointer out of the table and calls it indirectly, so the same machine code serves every implementor and nothing across that call can be inlined.

The vtable is built once per (type, trait) pair at compile time and shared by every value of that type. It holds the destructor, the size and alignment, and then a pointer per method. Note where it does **not** live: not in your value. The last section of the run below shows `Risc` at 0 bytes while `&dyn Processor` is 16 — a C++ object with virtual methods carries its `vptr` inside the object, Rust puts it in the pointer instead.

## The question that actually decides it

Not speed. **Can you name the type?**

```rust
let fleet: Vec<Box<dyn Processor>> = vec![Box::new(Risc), Box::new(Cisc)];
```

A `Vec<P>` holds one concrete `P`, and no amount of generics will let it hold a `Risc` and a `Cisc` at once — that is what "the compiler knows the type" means. The `Vec<Box<dyn Processor>>` above works precisely because every element *is* the same type: a boxed trait object. When you need a heterogeneous collection, a plugin registry, or a value chosen at run time, `dyn` is not a performance trade-off — it is the only spelling that expresses the program.

Going the other way: if you can name the type, generics cost nothing at run time and let the optimiser see through the call. Default to static; reach for `dyn` when the type genuinely is not known until run time, or when the code-size multiplication starts to hurt.

| | `<P: Trait>` | `&dyn Trait` |
|---|---|---|
| Copies of the function | one per concrete type | one, shared |
| Call | direct, inlinable | indirect, through the vtable |
| Types in one collection | one | any number |
| Decided | compile time | run time |
| Restriction | none | the trait must be **dyn compatible** |

## `dyn` compatibility — and the rename nobody's blog post has caught up with

Not every trait can become a trait object. A vtable is a table of function pointers, so a method the compiler cannot put in one — a generic method, which has no single machine-code body — disqualifies the whole trait:

```text
error[E0038]: the trait `Configurable` is not dyn compatible
  |
4 | fn tune(_c: &dyn Configurable) {}
  |              ^^^^^^^^^^^^^^^^ `Configurable` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
1 | trait Configurable {
  |       ------------ this trait is not dyn compatible...
2 |     fn set<T: Into<i64>>(&mut self, value: T);
  |        ^^^ ...because method `set` has generic type parameters
  = help: consider moving `set` to another trait
```

**This used to be called *object safety*, and every book, blog post and Stack Overflow answer still calls it that.** The compiler renamed it to *dyn compatibility*, so a learner who searches the words in front of them finds nothing, and a learner who searches the literature finds a term the compiler never prints. They are the same concept. The other common disqualifier is a method returning `Self` by value, for the same reason: the caller of a `dyn` method cannot know how big the answer is.

## `dyn` does not check the type at run time

The cost of dynamic dispatch is one indirect call and everything the optimiser can no longer see through. It is **not** a run-time check of what the value is, and a good deal of writing about `dyn` says otherwise.

There is nothing to check with. A trait object has no type tag: the vtable pointer is chosen once, where the `&dyn` is built, and after that the call is a load and a jump. Ask a trait object what it is and the compiler has no answer to give you:

```text
error[E0599]: no method named `is` found for reference `&Box<dyn Animal>` in the current scope
  |
9 |         if p.is::<Dog>() { }
  |              ^^ method not found in `&Box<dyn Animal>`
```

Run-time type identity is opt-in, and its name is [`std::any::Any` ↗](https://doc.rust-lang.org/std/any/trait.Any.html) — the one trait whose vtable carries a `TypeId`, which is what makes `downcast_ref::<T>()` able to answer:

```rust
let boxed: Box<dyn Any> = Box::new(Risc);
println!("{:?}", boxed.downcast_ref::<Risc>().is_some());  // true
```

`Any` is worth knowing about and worth reaching for rarely: a `match` on a downcast chain is a type switch, which is the shape `dyn` existed to replace.

## The third strategy the two-way framing hides: an enum

Static or dynamic is the usual question, and it skips the answer that is often right — one `enum` with a variant per implementor, and a `match` in the trait impl.

```rust
enum AnyProcessor { Risc(Risc), Cisc(Cisc) }

impl Processor for AnyProcessor {
    fn compute(&self, x: i64, y: i64) -> i64 {
        match self {
            AnyProcessor::Risc(p) => p.compute(x, y),
            AnyProcessor::Cisc(p) => p.compute(x, y),
        }
    }
}
```

That buys the heterogeneous `Vec` — the thing generics could not do — while keeping every call static and every value inline: one byte here, against sixteen for a `Box<dyn Processor>`. The `match` is an ordinary branch, not a vtable load. The [`enum_dispatch` ↗](https://docs.rs/enum_dispatch/latest/enum_dispatch/) crate writes the boilerplate for you.

What it costs is **openness**. The variant list is closed and lives in your crate, so nobody else's code can join in, and each new implementor edits every `match`. That is the real question behind all three:

| | who may implement | decided | the call |
|---|---|---|---|
| `<P: Trait>` | anyone | compile time | direct, inlinable |
| `enum` + `match` | you, in this crate | run time | a branch |
| `dyn Trait` | anyone, including a plugin loaded later | run time | indirect, through a vtable |

Reach for the enum when the set of implementors is genuinely closed — a token kind, a message type, a ballot format. Reach for `dyn` when it is not, or when the enum's variants would differ so much in size that every value pays for the largest.

## The verified output

<!-- output:static_vs_dynamic_dispatch -->
*Verified output of [`static_vs_dynamic_dispatch.rs`](examples/static_vs_dynamic_dispatch.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Same answers, two dispatch strategies
   run_static (&risc, 1) =  43   run_dynamic(&risc, 1) =  43
   run_static (&cisc, 2) =  84   run_dynamic(&cisc, 2) =  84

2. The thing generics cannot do: one collection, two types
   Risc computes 6 and 7 as 13
   Cisc computes 6 and 7 as 42

3. What the second pointer costs
   &Risc             8 bytes
   &dyn Processor   16 bytes   data pointer + vtable pointer
   Box<dyn Proc>    16 bytes   the same pair, owned
   Risc itself       0 bytes   a unit struct: the vtable is shared,
                            not stored in the value, unlike a C++ vptr.

4. Static dispatch survives inlining; dynamic dispatch is a real call
   Both printed the same numbers above, so correctness is never the
   question. The question is one machine-code copy per type and a
   direct call, or one copy shared and an indirect one.

5. Dynamic dispatch does NOT check the type at run time
   A trait object carries no type tag. The vtable is picked once,
   where the `&dyn` is made; the call is one indirect jump through
   it. Asking a `&dyn Processor` what it is does not even compile:
     p.is::<Risc>()
     error[E0599]: no method named `is` found for reference
                   `&Box<dyn Processor>` in the current scope
   Run-time type identity is opt-in, via std::any::Any, which is
   the trait whose vtable does carry a TypeId:
     downcast_ref::<Risc>() -> Some("Risc")   ::<Cisc>() -> None
     downcast_ref::<Risc>() -> None   ::<Cisc>() -> Some("Cisc")
   So `dyn` costs an indirect call and a lost inline. It does not
   cost a type lookup, because no lookup happens.

6. The two-way framing hides a third strategy: an enum
   Risc computes 6 and 7 as 13
   Cisc computes 6 and 7 as 42
   One collection, two behaviours, and no vtable: the `match` is
   an ordinary branch the optimiser can see through.
   size_of::<AnyProcessor>() = 1, against 16 for a Box<dyn Processor>:
   the value itself, inline, rather than a pointer pair.
   What it costs: the list of implementors is closed. Nobody else's
   crate can add a variant, and each new one edits every match.
   `dyn` is what you reach for when that list must stay open.
```
<!-- /output -->

## See also

- [Returning a trait](../returning_a_trait/README.md) — the same two spellings in *return* position, where the problem is stated by the compiler rather than chosen by you
- [Supertraits](../supertraits/README.md) — what a trait object inherits from the traits its own trait requires
- [What a trait is](../what_a_trait_is/README.md) — the declaration both spellings are quantifying over
- [The Book, ch. 10 — performance of code using generics ↗](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics) — monomorphization, with the generated code written out
- [LogRocket — disambiguating Rust traits: `Copy`, `Clone` and `Dynamic` ↗](https://blog.logrocket.com/disambiguating-rust-traits-copy-clone-dynamic/) — a clear introduction whose cost section says `dyn` values "have to be checked for their type at runtime". They are not; the `E0599` above is what asking actually gets you. Its closing refactor is the enum above, filed under the wrong name — an enum and a `match` is not monomorphization, which is what the *generic* half of this page does
