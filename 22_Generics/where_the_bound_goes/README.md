# Where the bound goes

**Level:** 201 → 301 · working knowledge

**One line:** An unbounded `T` can be stored, moved and dropped, and nothing else — a trait bound buys the rest, and it belongs on the `impl` that needs it rather than on the struct.

```rust
struct Container<T> {
    value: T,          // no bound here, on purpose
}

impl<T> Container<T> {
    fn new(value: T) -> Self { Self { value } }
    fn into_inner(self) -> T { self.value }
}

// The bound sits on the impl block that actually clones.
impl<T: Clone> Container<T> {
    fn duplicate(&self) -> (T, T) {
        (self.value.clone(), self.value.clone())
    }
}
```

`Container<Handle>` — where `Handle` is not `Clone` — still builds, stores, moves and unwraps. It just has no `duplicate` method. The struct stays open to every type; only the one operation that needs a promise asks for one.

## What an unbounded `T` can do

The whole list:

- be stored in a field, a `Vec`, a `Box`
- be passed, returned, moved
- be dropped

Not printed, not compared, not added, not cloned, not defaulted. None of those are properties of *types*; they are traits, and a `T` with no bound has promised nothing. The compiler enforces that at the **definition**, so the error arrives in the function you are writing:

```text
error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> e0277_def.rs:2:14
  |
2 |     format!("{v}!")
  |              ^^^ `T` cannot be formatted with the default formatter
  |
help: consider restricting type parameter `T` with trait `Display`
  |
1 | fn shout<T: std::fmt::Display>(v: T) -> String {
  |           +++++++++++++++++++
```

Take the help and the function compiles for every type that implements `Display`, forever, with no further checking at any call site. That trade — declare what you need once, get every conforming type free — is the whole deal a bound is.

## Three spellings, and only two of them mean the same thing

```rust
fn shout<T: Display>(v: T) -> String { format!("{v}!") }

fn shout_where<T>(v: T) -> String
where
    T: Display,
{ format!("{v}!") }

fn shout_impl(v: impl Display) -> String { format!("{v}!") }
```

The first two are indistinguishable to the compiler; pick by line length. `where` is not merely the long form, though — it is the **only** spelling available when the thing being bounded is not a bare parameter. `where Vec<T>: Debug` and `where T::Item: Clone` cannot be written inline at all.

The third is genuinely different, in two ways:

- **There is no `T` to name**, so the caller cannot [turbofish](../../15_First_Programs/what_an_annotation_does/README.md) it: `shout_impl::<f64>(3.5)` does not compile.
- **Two `impl Trait` parameters are two independent types.** `fn pair(a: impl Display, b: impl Display)` accepts a `u8` and a `&str` — the run below does exactly that — while `fn pair<T: Display>(a: T, b: T)` refuses, because there one `T` is filled in once. When the point of the generic is that two arguments agree, the named parameter is the only spelling that says so.

Several bounds join with `+`: `T: Clone + Debug + PartialOrd`.

## The trap: a bound on the struct definition

This compiles, and it is the wrong place:

```rust
struct Wrapper<T: Clone> {
    value: T,
}
```

The bound is now part of the *type*, so it applies to every use of `Wrapper` anywhere — including the ones that never clone. Constructing one is where it bites:

```text
error[E0277]: the trait bound `Handle: Clone` is not satisfied
 --> struct_bound.rs:9:30
  |
9 |     let w = Wrapper { value: Handle(7) };
  |                              ^^^^^^^^^ the trait `Clone` is not implemented for `Handle`
  |
note: required by a bound in `Wrapper`
 --> struct_bound.rs:4:19
  |
4 | struct Wrapper<T: Clone> {
  |                   ^^^^^ required by this bound in `Wrapper`
```

`Handle` cannot be *put in* the wrapper, even by a caller who only ever wanted to store it. And the cost lands on you as much as on the caller: every `impl` block, every function and every other struct that so much as names `Wrapper<T>` must repeat the bound to be accepted. Books print this shape often — the linked-list chapter's node is usually written like this:

```rust
struct ListItem<T>
where
    T: Clone + Debug,
{
    data: Box<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self { ListItem { data: Box::new(data) } }
}
```

It reads as documentation and behaves as an obligation. That plainest-possible `impl` block — which clones nothing and prints nothing — is **seven errors**, of which the first is the readable one:

```text
error[E0277]: the trait bound `T: Clone` is not satisfied
  --> structbound.rs:10:9
   |
10 | impl<T> ListItem<T> {
   |         ^^^^^^^^^^^ the trait `Clone` is not implemented for `T`
   |
note: required by a bound in `ListItem`
  --> structbound.rs:5:8
   |
 3 | struct ListItem<T>
   |        -------- required by a bound in this struct
 4 | where
 5 |     T: Clone + Debug,
   |        ^^^^^ required by this bound in `ListItem`
help: consider restricting type parameter `T` with trait `Clone`
   |
10 | impl<T: std::clone::Clone> ListItem<T> {
   |       +++++++++++++++++++
```

Take that `help:` and the bound spreads one block further, which is the whole problem in miniature.

The standard library is the precedent worth copying. `Vec<T>`, `Option<T>` and `HashMap<K, V>` all declare their parameters bare — `HashMap`'s `K: Hash + Eq` lives on the impl blocks that actually hash a key, which is why a `HashMap<K, V>` type can be *named* in a signature whose `K` hashes nothing. The [Rust API Guidelines ↗](https://rust-lang.github.io/api-guidelines/future-proofing.html#data-structures-do-not-duplicate-derived-trait-bounds-c-struct-bounds) say it outright — data structures do not duplicate derived trait bounds — and the asymmetry behind the rule is that **relaxing** a published bound is a breaking change for everyone downstream, while adding one to a new impl block is not.

Bounds on a struct are right in one case: when the type cannot be constructed meaningfully without them, which in practice means a parameter used by an associated type or a [`PhantomData` invariant](../../12_Traits/phantom_types/README.md), not `Clone`.

## `derive` writes the bound for you — and sometimes one too many

```rust
#[derive(Clone, Debug)]
struct Pair<T> {
    left: T,
    right: T,
}
```

That expands to `impl<T: Clone> Clone for Pair<T>` — bound on the impl, exactly as above. So `Pair<Handle>` is a perfectly good type you can build, print and move; it simply has no `.clone()`. Writing `struct Pair<T: Clone>` to "help" the derive would take that away and buy nothing.

The derive does not read your fields, though. It bounds every parameter whether the fields need it or not, and that is wrong whenever a field is clonable for reasons of its own:

```rust
#[derive(Clone)]
struct Shared<T> {
    inner: Rc<T>,       // Rc<T> is Clone for ANY T — cloning bumps the count
}
```

```text
error[E0599]: the method `clone` exists for struct `Shared<NotClone>`, but its trait bounds were not satisfied
   |
 7 | struct Shared<T> {
   | ---------------- method `clone` not found for this struct because it doesn't satisfy `Shared<NotClone>: Clone`
   |
note: trait bound `NotClone: Clone` was not satisfied
   |
 6 | #[derive(Clone, Debug)]
   |          ----- in this derive macro expansion
 7 | struct Shared<T> {
   |               ^ type parameter would need to implement `Clone`
   = help: consider manually implementing the trait to avoid undesired bounds
```

The compiler's own `help:` names the fix, and it is the rare case where writing the impl by hand is *less* restrictive than deriving it:

```rust
impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared { inner: Rc::clone(&self.inner) }
    }
}
```

Worth recognising on sight: a mysterious "does not implement `Clone`" on a type whose fields are all cheap `Rc`s or `Arc`s is almost always the derive, not a real requirement.

The error you get from the right arrangement is also the better error — it names the method, the concrete type and the bound in one place, and suggests the fix on the type that is actually missing something:

```text
error[E0599]: the method `duplicate` exists for struct `Container<Handle>`, but its trait bounds were not satisfied
   |
16 |     let _ = opaque.duplicate();
   |                    ^^^^^^^^^ method cannot be called on `Container<Handle>` due to unsatisfied trait bounds
   |
note: trait bound `Handle: Clone` was not satisfied
   |
 8 | impl<T: Clone> Container<T> {
   |         ^^^^^  ------------
help: consider annotating `Handle` with `#[derive(Clone)]`
```

## If you are coming from another language

**Python.** A bound is the thing duck typing leaves implicit. `def shout(v): return f"{v}!"` works on whatever supports `__format__`, and you find out which types those are by trying. `typing.Protocol` is the closest deliberate counterpart — a structural interface a class satisfies without declaring it — and mypy checks it if you run mypy. Two differences on the way to Rust: the bound is compulsory rather than documentary, so an unbounded `T` genuinely cannot be printed; and the trait is *nominal* — a type gets `Display` by someone writing `impl Display for X`, never by happening to have the right method. That second one is what makes the compiler's error precise enough to include the fix. The other half of the trade is the guarantee: because the bound is checked at the definition, a generic function that compiles cannot surprise you at a call site you did not write, which is why Rust library code re-checks its arguments so much less than Python's does.

**ABAP.** You have written bounds for years under a different name. `IMPORTING io_source TYPE REF TO if_serializable` is a bound: it says *any class at all, provided it implements this interface*, and the compiler checks the call. `TYPE any` is the unbounded `T` — accepted by everything, capable of nothing until you `ASSIGN` and hope, with `CL_ABAP_TYPEDESCR` to ask at run time what actually arrived. So the bridge is exact: **`T: Display` is `TYPE REF TO if_display`, and `T` alone is `TYPE any`.** Two things Rust adds. The check is fully compile-time, so the `CX_SY_MOVE_CAST_ERROR` and the RTTI branch are both gone. And the bound keeps the concrete type — `T: Clone + Debug` is still a `u8` inside the function, where `TYPE REF TO zif_x` has replaced your value with a reference and thrown the type away. ABAP has no way to say "any type at all, provided it implements these two interfaces" without paying that price; a trait can also be implemented for `i32`, which ABAP cannot do at all.

**Java / C#.** `<T extends Comparable<T>>` is `T: PartialOrd`, and the mechanics line up closely — nominal interfaces, declared up front, checked at compile time. The difference is who may implement one: in Java the type's own author, in its own file. Rust lets you write `impl MyTrait for i32` in your crate, so a bound can be satisfied by a type from the standard library or from somebody else's crate. That is the [orphan rule ↗](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)'s territory, and it is why Rust needs far fewer adapter types than Java does.

**C++.** Pre-C++20 templates had no bounds at all, which is why an unsupported operation produced a wall of errors from inside the library. C++20 concepts (`template <std::totally_ordered T>`) are the same idea as a trait bound and arrive at the same place: state the requirement, get a readable error. Two differences remain. A concept is checked structurally — the type just has to have the operations — while a Rust bound needs an `impl` to exist, so "accidentally satisfies the concept" is not a category of bug Rust has. And a concept can require an *expression* to be well-formed (`{ a + b } -> std::same_as<T>`), where a Rust bound can only name a trait, so some C++ generic code has no direct translation until somebody writes the trait it implies.

## The verified output

[`examples/where_the_bound_goes.rs`](examples/where_the_bound_goes.rs) compiled and run:

<!-- output:where_the_bound_goes -->
*Verified output of [`where_the_bound_goes.rs`](examples/where_the_bound_goes.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
a Container<Handle> is fine: Handle(7)
duplicate() on Container<String>: "Ada" "Ada"

1! two! 3.5!
two independent types: 1/two

Pair<Handle>  built and printable: Pair { left: Handle(1), right: Handle(2) }
              and readable:        1 2
Pair<String>  cloneable:          Pair { left: "yes", right: "no" }

Shared<Handle> cloned by hand: value 9, 2 owners
```
<!-- /output -->

## See also

- [What a generic is](../what_a_generic_is/README.md) — the `<T>` a bound constrains
- [Generic enums](../generic_enums/README.md) — the same rule on an `enum`: the bound goes on the `impl`, not the declaration
- [What a trait is](../../12_Traits/what_a_trait_is/README.md) — the promise on the other side of the colon
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Sized`, the bound every `T` already has and nobody writes
- [Phantom types](../../12_Traits/phantom_types/README.md) — the one case where the parameter is not in the data at all, and the bound question changes shape
- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — `<T: Trait>` against `&dyn Trait`, and when a bound is not what you want
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — the two bounds used most often on this page
