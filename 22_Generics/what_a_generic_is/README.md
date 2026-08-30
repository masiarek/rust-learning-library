# What a generic is

**Level:** 101 → 201 · newcomer to working knowledge

**One line:** `<T>` is a type the caller fills in — you write the definition once, the compiler checks it once, and then stamps out a separate copy for each type your program actually uses.

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

let text = Container::new("Thought is free.");
let count = Container::new(4u8);
println!("{} {}", text.value, count.value);  // Thought is free. 4
```

Two containers, two types, one definition. Nothing at either call site names a type: the compiler reads `T` off the value it was handed.

## Reading the angle brackets

`<T>` is a **parameter list for types**, in the same way `(value: T)` is a parameter list for values. It appears wherever a new one is being *declared*, and again wherever an already-declared one is being *used*:

| Written | What the `<T>` is doing |
|---|---|
| `struct Container<T>` | declaring: this type takes one type argument |
| `impl<T> Container<T>` | declaring on the left, using on the right |
| `fn largest<T>(items: &[T]) -> T` | declaring a parameter local to this function |
| `Container<u8>` | using: `T` is filled in, and this is now a concrete type |

The doubled `T` in `impl<T> Container<T>` is the one that looks redundant and is not. The first introduces the name for the block, the second says which type the block is *for* — that is what makes `impl<T> Container<T>` different from `impl Container<u8>`, which is a legal block that applies to exactly one container.

`T` is a convention, not a keyword. Any name works, and a longer one is often better: `Container<Payload>` reads more clearly than `Container<T>` in a type with three parameters.

## What you get

**One definition instead of many.** Without generics, a container of a `u8` and a container of a `String` are two structs with two impl blocks and two sets of tests, differing only in one word. That is the DRY argument, and it is the usual reason people reach for generics.

**A separate type per fill-in.** `Container<u8>` and `Container<&str>` are as unrelated as `u8` and `String` are. One `Vec` holds one type:

```rust
let mut batch = vec![Container::new(1u8), Container::new(2u8)];
// batch.push(Container::new("three"));
```

```text
error[E0308]: mismatched types
 --> e0308.rs:7:35
  |
7 |     batch.push(Container { value: "three" });
  |                                   ^^^^^^^ expected `u8`, found `&str`
```

That is generics working, not generics complaining. If you need one collection holding several types at once, no amount of `<T>` will do it — that is what [`dyn Trait`](../../12_Traits/static_vs_dynamic_dispatch/README.md) is for, and the choice between them is a question about your program, not about performance.

**Nothing at run time.** A generic parameter is not a box, a tag or a pointer. `Container<T>` is laid out as exactly its `T`:

| Type | `size_of` |
|---|---|
| `u8` | 1 |
| `Container<u8>` | 1 |
| `Container<i64>` | 8 |
| `Container<[u8; 16]>` | 16 |

The compiler achieves that by **[monomorphization](../../GLOSSARY.md)** — compiling one copy of the code per concrete type, with the type filled in. `largest(&[3, 9, 4])` and `largest(&['a', 'q', 'f'])` are two functions in the finished binary, each calling that type's comparison directly. You pay in compile time and code size, never in speed. [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) is the page that measures it.

## A generic function

```rust
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

println!("{} {}", largest(&[3, 9, 4]), largest(&['a', 'q', 'f']));  // 9 q
```

`T: PartialOrd + Copy` is a **trait bound**: it is what makes `item > best` legal, because `>` is not something every type has. Leave the bound off any generic function and the *definition* stops compiling, long before anybody calls it — here, one that tries to add two values:

```text
error[E0369]: cannot add `T` to `T`
 --> e0369.rs:2:7
  |
2 |     a + b
  |     - ^ - T
  |     |
  |     T
  |
help: consider restricting type parameter `T` with trait `Add`
  |
1 | fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
  |         +++++++++++++++++++++++++++
```

This is the difference between a Rust generic and a C++ template, and it is worth understanding early: **the definition is type-checked on its own**, against the bounds it declares, before anyone calls it. A `T` with no bounds can be stored, moved and dropped, and that is the entire list — it cannot be compared, printed or added, because nothing has promised those exist. [Where the bound goes](../where_the_bound_goes/README.md) is that page.

## You have been using them all along

Every one of these is a generic type you have already written:

| Type | What the parameters are |
|---|---|
| [`Option<T>`](../../17_Option_and_Result/some_and_none/README.md) | the value that may be absent |
| `Result<T, E>` | the success value and the failure value |
| `Vec<T>` | the element |
| `Box<T>` | the thing on the heap |
| `HashMap<K, V>` | the key and the value |

`Option` is not a language feature with a `T`-shaped hole in the compiler; it is [an ordinary enum](../../13_Enums/what_an_enum_is/README.md) with one type parameter, declared in the standard library in four lines. Once `<T>` reads as ordinary syntax, most of the standard library stops looking magic.

## If you are coming from another language

**Python.** You already have this, and it costs you nothing to use — `def largest(items)` accepts a list of anything, and works as long as the elements happen to support `>`. The word for that is duck typing, and the check happens when the line runs. `typing`'s `TypeVar` and `Generic[T]` bring the *notation* closer to Rust's, but not the enforcement: they are annotations, checked by mypy if you run it and ignored by the interpreter entirely. Two things change on the way to Rust. The check moves to compile time and becomes non-optional — a `T` that might not support `>` is a build failure in the function you wrote, not a `TypeError` in a customer's log. And the erasure goes away: `list[int]` and `list[str]` are the same class at run time in Python, while `Container<u8>` and `Container<String>` are different types with different sizes, so the compiler needs a bound where Python needed only the runtime to find out.

**ABAP.** The closest thing is a generically-typed formal parameter — `IMPORTING iv_value TYPE any`, or a field-symbol `FIELD-SYMBOLS <fs> TYPE any` — and the resemblance ends at the syntax. `TYPE any` means *the compiler stops checking*: what happens to a value you cannot describe is decided when the statement runs, and the failure mode is a short dump (`CX_SY_CONVERSION_NO_NUMBER`, `CX_SY_MOVE_CAST_ERROR`) rather than a syntax error. To find out what you were actually handed you ask RTTI, at run time, and branch. Rust's `<T>` is the opposite arrangement with the same reach: the caller fills the type in, but the compiler knows what it is at every line, so the branch and the dump are both gone. The other ABAP habit this replaces is the one nobody enjoys — `ZCL_TALLY_INT`, `ZCL_TALLY_STRING`, `ZCL_TALLY_DATE`, three classes and one idea, kept in sync by hand. That is exactly the duplication `<T>` deletes, and the reason ABAP has so much of it is that it has no `<T>`.

**C++.** `template <typename T>` is the direct ancestor, and the one difference that matters is *when* the definition is checked. A C++ template is checked at instantiation, so an unsupported operation surfaces as a page of errors inside a header you did not write, blaming a line you did not call. Rust checks the definition against its declared bounds, so the error lands in your function and names the bound to add — and the call site's error, when there is one, is one line long. C++20 concepts are the same idea arriving forty years later; a Rust trait bound is a concept you cannot forget to write. Code generation is the same in both: one copy per instantiation, no runtime cost.

**Java / C#.** Java erases: `List<String>` and `List<Integer>` are one class at run time, `T` becomes `Object`, and a `List<int>` is impossible — you box. Rust monomorphizes instead, which is why `Vec<u8>` really is a run of bytes and `Container<u8>` is one byte. C# sits between the two, specialising value types and sharing a single implementation for reference types. What transfers from either is the *notation* — `<T extends Comparable<T>>` is `T: PartialOrd` — and what changes is that Rust's version is a compile-time template rather than a run-time contract.

## The verified output

[`examples/what_a_generic_is.rs`](examples/what_a_generic_is.rs) compiled and run:

<!-- output:what_a_generic_is -->
*Verified output of [`what_a_generic_is.rs`](examples/what_a_generic_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
text    Thought is free.
count   4
scores  [5, 3, 0]

size_of::<u8>()                   1
size_of::<Container<u8>>()        1
size_of::<Container<i64>>()       8
size_of::<Container<[u8; 16]>>()  16

largest(&[3, 9, 4])         9
largest(&['a', 'q', 'f'])   q
largest(&[0.5, 0.25])       0.5

a Vec<Container<u8>> holds 2 of them, and nothing else
```
<!-- /output -->

## Practice

**A container with two holes.** `Container<T>` holds one value. Write `Pair<A, B>`, which holds two of possibly different types, with a `new` and a `swap` that exchanges the fields.

The interesting part is `swap`'s return type — it is not `Self`. Write the signature before the body and the reason will be obvious; write the body first and the compiler will explain it to you. Then check what happens when both parameters are filled with the same type.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_a_generic_is_kata -->
*[`what_a_generic_is_kata.rs`](examples/what_a_generic_is_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Kata solution: a struct with two type parameters, and the swap that
// returns a different type from the one it was called on.

#[derive(Debug)]
struct Pair<A, B> {
    left: A,
    right: B,
}

impl<A, B> Pair<A, B> {
    fn new(left: A, right: B) -> Self {
        Self { left, right }
    }

    // Note the return type: Pair<B, A>, not Self. Swapping the fields
    // swaps the type parameters with them.
    fn swap(self) -> Pair<B, A> {
        Pair { left: self.right, right: self.left }
    }
}

fn main() {
    let ballot = Pair::new("Ada", 5u8);
    println!("ballot          {ballot:?}");
    println!("ballot.swap()   {:?}", ballot.swap());

    // Both parameters may be filled with the same type. They are still two.
    let both = Pair::new(3u8, 5u8);
    println!("same type twice {both:?}");

    // And they may be filled with anything, independently.
    let nested = Pair::new(vec!['a', 'b'], Pair::new(1i8, "one"));
    println!("nested          {nested:?}");
    println!("sizes: Pair<u8, u8> {} · Pair<u8, u64> {}",
        size_of::<Pair<u8, u8>>(), size_of::<Pair<u8, u64>>());
}
```
<!-- /source -->

<!-- output:what_a_generic_is_kata -->
*Verified output of [`what_a_generic_is_kata.rs`](examples/what_a_generic_is_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
ballot          Pair { left: "Ada", right: 5 }
ballot.swap()   Pair { left: 5, right: "Ada" }
same type twice Pair { left: 3, right: 5 }
nested          Pair { left: ['a', 'b'], right: Pair { left: 1, right: "one" } }
sizes: Pair<u8, u8> 2 · Pair<u8, u64> 16
```
<!-- /output -->

</details>

## See also

- [When the compiler cannot infer](../when_the_compiler_cannot_infer/README.md) — `E0282`, and the three ways to name the type it is asking for
- [Where the bound goes](../where_the_bound_goes/README.md) — what an unbounded `T` can do, and why the bound does not belong on the struct
- [Generic enums](../generic_enums/README.md) — the same brackets on an `enum`, where every variant shares them
- [A generic recursive type](../a_generic_recursive_type/README.md) — a type that contains itself, and the pointer that makes it possible
- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — what monomorphization costs, and the collection that forces `dyn`
- [What a struct is](../../16_Structs/what_a_struct_is/README.md) — the non-generic version of the type above
- [The Book, ch. 10 — generic data types ↗](https://doc.rust-lang.org/book/ch10-01-syntax.html) — the same material with the generated code written out
