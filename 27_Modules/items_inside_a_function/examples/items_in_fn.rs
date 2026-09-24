//! Items inside a function: a nested `fn`, `struct`, `impl`, `use`, `const`
//! or `static` is an ordinary item scoped to the block. Items see each
//! other and nothing the enclosing function's `let`s hold; a block and a
//! closure do see the locals. Each numbered block prints one such fact on
//! rustc 1.98.0. The page is 27_Modules/items_inside_a_function/README.md.
//!
//!   rustc --edition 2024 items_in_fn.rs -o /tmp/iif && /tmp/iif

use std::any::type_name_of_val;
use std::fmt;

/// The struct and its impl live inside this function. The value gets out
/// through `impl Display`; the name `Point` does not.
fn origin_report() -> impl fmt::Display {
    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    Point { x: 3, y: 4 }
}

/// A nested fn cannot use the outer `T` (E0401); it declares its own.
fn width<T>() -> usize {
    fn inner<U>() -> usize {
        size_of::<U>()
    }
    inner::<T>()
}

fn apply(f: fn(i32) -> i32, n: i32) -> i32 {
    f(n)
}

fn main() {
    println!("1. A nested fn is callable above its own line; a let is not");
    println!("   helper(2) = {}", helper(2));
    fn helper(n: i32) -> i32 {
        n * 10
    }

    println!();
    println!("2. What a nested fn sees: const, static and other items, never a let");
    const STEP: u32 = 2;
    static LIMIT: u32 = 3;
    let bonus: u32 = 100;
    fn next(n: u32) -> u32 {
        (n + STEP).min(LIMIT) // `+ bonus` here would be E0434
    }
    let in_a_block = { bonus + 1 };
    let in_a_closure = |n: u32| n + bonus;
    println!("   next(2) = {} (reads STEP and LIMIT)", next(2));
    println!("   a block sees the let: {in_a_block}");
    println!("   a closure captures it: {}", in_a_closure(1));

    println!();
    println!("3. A nested struct: the value leaves through impl Display, the name stays inside");
    println!("   origin_report() = {}", origin_report());

    println!();
    println!("4. A use inside a function shortens names for that block only");
    fn count_distinct(text: &str) -> usize {
        use std::collections::HashSet;
        let unique: HashSet<&str> = text.split_whitespace().collect();
        unique.len()
    }
    println!("   count_distinct(\"a b a c\") = {}", count_distinct("a b a c"));

    println!();
    println!("5. Nested fn versus closure: type, size, and coercion to a fn pointer");
    fn double(n: i32) -> i32 {
        n * 2
    }
    let offset = 7;
    let triple = |n: i32| n * 3;
    let add_offset = move |n: i32| n + offset;
    println!("   double:     {} ({} bytes)", type_name_of_val(&double), size_of_val(&double));
    println!("   triple:     {} ({} bytes)", type_name_of_val(&triple), size_of_val(&triple));
    println!("   add_offset: {} ({} bytes)", type_name_of_val(&add_offset), size_of_val(&add_offset));
    println!("   apply(double, 1) = {}, apply(triple, 1) = {}", apply(double, 1), apply(triple, 1));
    println!("   both coerce to fn(i32) -> i32; add_offset captured offset and cannot (E0308)");

    println!();
    println!("6. A nested fn cannot use the outer fn's type parameter; it declares its own");
    println!("   width::<u32>() = {}, width::<u8>() = {}", width::<u32>(), width::<u8>());
}
