# What a union is

**Level:** 301 · deep dive

**One line:** A union is a struct whose fields sit on top of each other instead of side by side — and Rust already gives you the safe version of it under a different name, because an `enum` **is** a union with a tag the compiler makes you read.

---

## Three things that only sound alike

Sort these out first; two of the three are unrelated to unions entirely.

| | What it is | Tagged? |
|---|---|---|
| `struct Unit;` | a **unit struct** — no fields, no bytes | n/a |
| `union U { a: u32, b: f32 }` | fields sharing one slot | **no** |
| `enum E { A(u32), B(f32) }` | fields sharing one slot | **yes** |

`Unit` shares nothing with `union` but four letters. The pair that *is* genuinely related is the bottom two — and the difference between them is the whole lesson.

Python users have a fourth trap: `typing.Union[int, str]` is **the opposite** of a C union. It is a checked either/or, so it lines up with Rust's `enum`. `ctypes.Union` is the real analogue.

## Side by side, or on top of each other

```rust
struct Both   { int: u32, float: f32 }   // 8 bytes: each field owns its own
union  Either { int: u32, float: f32 }   // 4 bytes: one slot, two readings
```

A struct's size is the sum of its fields plus padding. A union's is its **largest** field, rounded up to alignment. `Both` can answer both questions at once; `Either` answers exactly one, and will not tell you which.

## Writing is safe. Reading is not.

```rust
let mut u = Either { int: 0 };
u.float = 1.0;                  // safe — this only overwrites bytes
let bits = unsafe { u.int };    // unsafe — you assert this reading is valid
```

The asymmetry is the point. A write cannot cause undefined behaviour: it just puts bytes somewhere. A read **interprets** bytes, and a union carries nothing that says which interpretation is the live one. `unsafe` here is you signing for that.

Note what it is *not*: reading `1.0f32`'s bytes as a `u32` is perfectly defined, because every 32-bit pattern is a valid `u32`. Make the field a `bool` and the same read is instant UB — only `0` and `1` are valid `bool` patterns. **The field type decides, not the union.**

For this particular job, reach for std first: `f32::to_bits` does exactly the above with no `unsafe` at all.

## An enum is a tagged union

This is the sentence worth keeping. A Rust `enum` has the same overlapping payload a union does, plus a discriminant, and `match` forces you through it:

```rust
match n {
    Number::Int(i)   => …,   // the tag has already been checked for you
    Number::Float(f) => …,
}
```

The Reference spells the equivalence out: a `repr(C)` enum with fields **is** a `repr(C)` struct holding a tag enum and a `repr(C)` union of the variants' payloads. That is what "tagged union" means, and it is why `union` is a tool for FFI and bit reinterpretation rather than a way to model a choice.

## Two rules that surprise people

**Borrowing one field borrows the whole union.** The fields overlap, so the borrow checker treats them as one place — `&mut u.int` and `&mut u.float` at once is `E0499`. For a struct the same two lines are fine, because the fields are disjoint.

**A union field cannot need dropping.** `Copy` types, references, `ManuallyDrop<T>`, and tuples or arrays of those. A `String` field is refused (`E0740`): with no tag, dropping the union could not know whether there is a `String` in there to free. `ManuallyDrop` is the escape hatch, and it makes freeing it your job.

## If you are coming from another language

**C.** This is the one language where `union` is ordinary rather than exotic, and the C idiom translates directly:

| C | Rust |
|---|---|
| `union number { float f; short i; }` | `#[repr(C)] union Number { f: f32, i: i16 }` |
| `struct tagged { enum tag t; union number n; }` | `#[repr(C)] struct Tagged { tag: Tag, n: Number }` |
| …and you remember to check `t` | `enum Number { F(f32), I(i16) }` — checked for you |

The third row is the whole difference. C gives you the tagged-union *pattern* and trusts you to maintain it; Rust gives you the same layout as a language feature and maintains it for you. So Rust code carries `union` almost exclusively at the FFI boundary — where the layout is C's to choose, not yours — and `#[repr(C)]` on all three types is what makes the two columns interchangeable in memory.

**Python.** `ctypes.Union` is the direct analogue and carries the same hazard. But the everyday `typing.Union[int, str]` is a *checked* either/or that the type checker verifies — that is Rust's `enum`, not Rust's `union`. Same word, opposite guarantee.

**ABAP.** There is no union type. The nearest thing is `ASSIGN … CASTING TYPE`, which points a field symbol at existing bytes and reinterprets them as another type — the same idea and the same danger, minus the `unsafe` keyword that would tell a reviewer where to look.

---

## Practice

**Build the same choice twice, and find the bug only one of them can have.** Model *"either a `u32` or an `f32`"* two ways: a `#[repr(C)] union` with a `Tag` field you maintain by hand, and a plain `enum`.

1. Give the hand-rolled version constructors and a safe `describe()`. What exactly are you promising when you call that wrapper safe?
2. Skip the constructors and build one directly with the tag saying `Int` and the payload holding a float. What does `describe()` print? Is it a crash, a warning, or something worse?
3. Change the union's integer field to `bool`. Why does the same desync become undefined behaviour, when it was merely wrong before?
4. Write the equivalent bug with the `enum`. What stops you?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_a_union_is_kata -->
*[`what_a_union_is_kata.rs`](examples/what_a_union_is_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: build the same choice twice — by hand with a union, then as an enum.
//!
//!   rustc --edition 2024 what_a_union_is_kata.rs -o /tmp/wauk && /tmp/wauk

#[derive(Clone, Copy, PartialEq)]
enum Tag {
    Int,
    Float,
}

#[repr(C)]
union Payload {
    int: u32,
    float: f32,
}

// (a) The C shape: a tag you maintain BY HAND, beside storage that overlaps.
struct Tagged {
    tag: Tag,
    payload: Payload,
}

impl Tagged {
    fn int(v: u32) -> Self {
        Tagged { tag: Tag::Int, payload: Payload { int: v } }
    }
    fn float(v: f32) -> Self {
        Tagged { tag: Tag::Float, payload: Payload { float: v } }
    }

    /// The promise: `tag` truly describes what was last written to `payload`.
    /// Nothing in the type system holds anyone to it. This is a *safe* wrapper
    /// only because every constructor above is correct — and that is a claim
    /// about code review, not a claim the compiler checked.
    fn describe(&self) -> String {
        unsafe {
            match self.tag {
                Tag::Int => format!("an integer {}", self.payload.int),
                Tag::Float => format!("a float {}", self.payload.float),
            }
        }
    }
}

// (b) The Rust shape: the tag IS the discriminant, and it cannot desynchronise.
enum Number {
    Int(u32),
    Float(f32),
}

impl Number {
    fn describe(&self) -> String {
        match self {
            Number::Int(i) => format!("an integer {i}"),
            Number::Float(f) => format!("a float {f}"),
        }
    }
}

fn main() {
    println!("1. Both model the same thing, and both work when used correctly");
    println!("   hand-rolled: {}", Tagged::int(42).describe());
    println!("   hand-rolled: {}", Tagged::float(2.5).describe());
    println!("   enum:        {}", Number::Int(42).describe());
    println!("   enum:        {}", Number::Float(2.5).describe());

    println!("\n2. Now desynchronise the tag from the payload");
    println!("   The constructors are careful. The STRUCT LITERAL is not, and it");
    println!("   is just as legal — nothing marks this line as the mistake:");
    let lying = Tagged { tag: Tag::Int, payload: Payload { float: 1.0 } };
    println!("       Tagged {{ tag: Tag::Int, payload: Payload {{ float: 1.0 }} }}");
    println!("   describe() says: {}", lying.describe());
    println!("   ...which is 1.0's bit pattern read as an integer. Not a crash, not a");
    println!("   warning, not even undefined behaviour here — just a silently wrong");
    println!("   answer, which is the worse outcome of the two.");

    println!("\n3. When it IS undefined behaviour");
    println!("   Make the field a `bool` instead of a `u32` and the same desync reads");
    println!("   1065353216 as a bool. Only 0 and 1 are valid bit patterns for bool, so");
    println!("   that read is UB and the optimiser may assume it never happens.");
    println!("   The union did not get less safe — the FIELD TYPE decided it.");

    println!("\n4. The enum cannot express the bug at all");
    println!("   There is no way to write a Number that says Int and stores an f32.");
    println!("   `Number::Int(1.0)` is a type error, caught before the program runs:");
    println!("       error[E0308]: mismatched types — expected `u32`, found floating-point");
    println!("   The tag and the payload are the same construction, so they cannot drift.");
    println!("   That is the entire trade: you give up choosing the layout, and you get");
    println!("   an invariant the compiler maintains instead of one you promised to.");

    println!("\n5. So when is a union still the right tool?");
    println!("   When the layout is not yours to choose — a C API handing you a tagged");
    println!("   union across FFI — or when reinterpreting bits IS the job. For the");
    println!("   second, check std first: f32::to_bits, u32::from_ne_bytes and friends");
    println!("   already cover most of it, safely.");
}
```
<!-- /source -->

<!-- output:what_a_union_is_kata -->
*Verified output of [`what_a_union_is_kata.rs`](examples/what_a_union_is_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Both model the same thing, and both work when used correctly
   hand-rolled: an integer 42
   hand-rolled: a float 2.5
   enum:        an integer 42
   enum:        a float 2.5

2. Now desynchronise the tag from the payload
   The constructors are careful. The STRUCT LITERAL is not, and it
   is just as legal — nothing marks this line as the mistake:
       Tagged { tag: Tag::Int, payload: Payload { float: 1.0 } }
   describe() says: an integer 1065353216
   ...which is 1.0's bit pattern read as an integer. Not a crash, not a
   warning, not even undefined behaviour here — just a silently wrong
   answer, which is the worse outcome of the two.

3. When it IS undefined behaviour
   Make the field a `bool` instead of a `u32` and the same desync reads
   1065353216 as a bool. Only 0 and 1 are valid bit patterns for bool, so
   that read is UB and the optimiser may assume it never happens.
   The union did not get less safe — the FIELD TYPE decided it.

4. The enum cannot express the bug at all
   There is no way to write a Number that says Int and stores an f32.
   `Number::Int(1.0)` is a type error, caught before the program runs:
       error[E0308]: mismatched types — expected `u32`, found floating-point
   The tag and the payload are the same construction, so they cannot drift.
   That is the entire trade: you give up choosing the layout, and you get
   an invariant the compiler maintains instead of one you promised to.

5. So when is a union still the right tool?
   When the layout is not yours to choose — a C API handing you a tagged
   union across FFI — or when reinterpreting bits IS the job. For the
   second, check std first: f32::to_bits, u32::from_ne_bytes and friends
   already cover most of it, safely.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:what_a_union_is -->
*Verified output of [`what_a_union_is.rs`](examples/what_a_union_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Same two fields, two layouts
   struct Both    size 8  align 4   u32 AND f32, side by side
   union  Either  size 4  align 4   u32 OR  f32, the same bytes
   struct Unit    size 0              no fields at all — unrelated to `union`
   Both answers both questions at once:  7 and 2.5
   Either answers exactly one, and will not tell you which.

2. Writing is safe. Reading is not.
   wrote 1.0f32, read those same 4 bytes as u32:  1065353216  = 0x3F800000
   f32::to_bits(1.0) = 0x3F800000  <- the safe stdlib way to ask the same question

3. Why the unsafe: a union has no tag
   Nothing in those 4 bytes records whether they mean 1.0 or 1065353216.
   Both readings are legal HERE only because every bit pattern is a valid
   u32 and a valid f32. A `bool` field read as 1065353216 is instant UB —
   the only valid bit patterns for bool are 0 and 1.

4. The Rust answer: an enum is a tagged union
   an integer 1065353216
   a float 1
   Same overlapping bytes, plus a tag the compiler makes you read.
   That is the whole difference — and it is why `union` is a tool for FFI
   and bit reinterpretation, not something you reach for to model a choice.

5. Borrowing one field borrows the whole union
   let a = &mut u.int;
   let b = &mut u.float;   // E0499: second mutable borrow
   The fields overlap, so the borrow checker treats them as one place.
   For a struct the same two lines are fine — the fields are disjoint.

6. What a union field may hold
   Copy types, references, ManuallyDrop<T>, and tuples/arrays of those.
   A String field is refused (E0740): with no tag, dropping the union could
   not know whether there is a String in there to free. ManuallyDrop is the
   escape hatch, and it makes freeing it your job.
```
<!-- /output -->

## See also

- [What a struct is](../../01_Foundations/what_a_struct_is/README.md) — the side-by-side version, and where the unit struct actually belongs
- [STRUCTS.md](../../STRUCTS.md) — the map, including layout and `repr`
- [Reference — unions](https://doc.rust-lang.org/reference/items/unions.html) · [`repr(C)` enums with fields](https://doc.rust-lang.org/reference/type-layout.html#reprc-enums-with-fields) · [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html)
- [The Nomicon — exotic sizes](https://doc.rust-lang.org/nomicon/exotic-sizes.html) for the zero-sized types the unit struct is an instance of
