//! What a union is: fields that share one piece of storage.
//!
//!   rustc --edition 2024 what_a_union_is.rs -o /tmp/wau && /tmp/wau

use std::mem::{align_of, size_of};

// Side by side: every field gets its own bytes.
struct Both {
    int: u32,
    float: f32,
}

// On top of each other: one 4-byte slot, two ways to read it.
// repr(C) is what guarantees both fields start at offset 0.
#[repr(C)]
union Either {
    int: u32,
    float: f32,
}

// The unit struct, which only SOUNDS related. No fields, no bytes, no overlap.
struct Unit;

// What you almost always actually want.
enum Number {
    Int(u32),
    Float(f32),
}

fn main() {
    println!("1. Same two fields, two layouts");
    println!(
        "   struct Both    size {}  align {}   u32 AND f32, side by side",
        size_of::<Both>(),
        align_of::<Both>()
    );
    println!(
        "   union  Either  size {}  align {}   u32 OR  f32, the same bytes",
        size_of::<Either>(),
        align_of::<Either>()
    );
    let _unit = Unit;
    println!(
        "   struct Unit    size {}              no fields at all — unrelated to `union`",
        size_of::<Unit>()
    );
    let b = Both { int: 7, float: 2.5 };
    println!("   Both answers both questions at once:  {} and {}", b.int, b.float);
    println!("   Either answers exactly one, and will not tell you which.");

    println!("\n2. Writing is safe. Reading is not.");
    let mut u = Either { int: 0 };
    u.float = 1.0; // safe: this only overwrites bytes
    let bits = unsafe { u.int }; // unsafe: YOU assert this reading is valid
    println!("   wrote 1.0f32, read those same 4 bytes as u32:  {bits}  = 0x{bits:08X}");
    println!(
        "   f32::to_bits(1.0) = 0x{:08X}  <- the safe stdlib way to ask the same question",
        1.0f32.to_bits()
    );

    println!("\n3. Why the unsafe: a union has no tag");
    println!("   Nothing in those 4 bytes records whether they mean 1.0 or 1065353216.");
    println!("   Both readings are legal HERE only because every bit pattern is a valid");
    println!("   u32 and a valid f32. A `bool` field read as 1065353216 is instant UB —");
    println!("   the only valid bit patterns for bool are 0 and 1.");

    println!("\n4. The Rust answer: an enum is a tagged union");
    for n in [Number::Int(1065353216), Number::Float(1.0)] {
        let told = match n {
            // the match cannot forget to check the tag
            Number::Int(i) => format!("an integer {i}"),
            Number::Float(f) => format!("a float {f}"),
        };
        println!("   {told}");
    }
    println!("   Same overlapping bytes, plus a tag the compiler makes you read.");
    println!("   That is the whole difference — and it is why `union` is a tool for FFI");
    println!("   and bit reinterpretation, not something you reach for to model a choice.");

    println!("\n5. Borrowing one field borrows the whole union");
    println!("   let a = &mut u.int;");
    println!("   let b = &mut u.float;   // E0499: second mutable borrow");
    println!("   The fields overlap, so the borrow checker treats them as one place.");
    println!("   For a struct the same two lines are fine — the fields are disjoint.");

    println!("\n6. What a union field may hold");
    println!("   Copy types, references, ManuallyDrop<T>, and tuples/arrays of those.");
    println!("   A String field is refused (E0740): with no tag, dropping the union could");
    println!("   not know whether there is a String in there to free. ManuallyDrop is the");
    println!("   escape hatch, and it makes freeing it your job.");
}
