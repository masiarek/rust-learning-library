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
