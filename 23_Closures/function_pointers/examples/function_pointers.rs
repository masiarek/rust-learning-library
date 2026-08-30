//! `fn` as a TYPE: a pointer to code and nothing else.
//!
//!   rustc --edition 2024 function_pointers.rs -o /tmp/fp && /tmp/fp
//!
//! Everything printed here is measured, not asserted. The two refusals the
//! lesson is built around cannot live in this file, because a file that does
//! not compile has no output to record — their transcripts are on the page,
//! produced by `fn_items.rs` and `modify_all.rs`.

use std::collections::HashMap;

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn double(v: u32) -> u32 {
    v * 2
}

fn halve(v: u32) -> u32 {
    v / 2
}

/// Takes a bare function pointer: one concrete type, no generic parameter.
fn apply_ptr(v: u32, f: fn(u32) -> u32) -> u32 {
    f(v)
}

/// Takes anything callable. `F` is stamped out once per caller.
fn apply_bound<F: Fn(u32) -> u32>(v: u32, f: F) -> u32 {
    f(v)
}

/// A struct field holding code. No `Box`, no `dyn`, no lifetime parameter.
struct Rule {
    name: &'static str,
    apply: fn(u32) -> u32,
}

fn main() {
    println!("1. Naming a function does not give you a `fn` pointer");
    println!(
        "   size_of_val(&sum)                 = {} bytes   the fn ITEM, `fn(i32, i32) -> i32 {{sum}}`",
        std::mem::size_of_val(&sum)
    );
    let op: fn(i32, i32) -> i32 = sum; // the coercion, written out
    println!(
        "   size_of_val(&op)                  = {} bytes   the fn POINTER, after coercion",
        std::mem::size_of_val(&op)
    );
    println!("   op(2, 3) = {}   sum(2, 3) = {}   same code, two types", op(2, 3), sum(2, 3));
    println!("   the item is zero-sized because the function it names is already known;");
    println!("   the pointer is 8 bytes because which function it names is not.");

    println!("\n2. The item type cannot be compared, stored or passed around");
    println!("   let op1 = sum; let op2 = sum; op1 == op2");
    println!("     -> E0369: binary operation `==` cannot be applied to");
    println!("               type `fn(i32, i32) -> i32 {{sum}}`   (see the page)");
    println!("   Coerce first and it compiles — but the compiler now argues with you:");
    let a: fn(u32) -> u32 = double;
    let b: fn(u32) -> u32 = double;
    let c: fn(u32) -> u32 = halve;
    println!("     a == b   -> warning: unpredictable_function_pointer_comparisons");
    println!("     std::ptr::fn_addr_eq(a, b) = {}", std::ptr::fn_addr_eq(a, b));
    println!("     std::ptr::fn_addr_eq(a, c) = {}", std::ptr::fn_addr_eq(a, c));

    println!("\n3. A method is a value too — the dot is the sugar, not the function");
    let len: fn(&str) -> usize = str::len;
    println!("   let len: fn(&str) -> usize = str::len;");
    println!("   [\"Ada\", \"Ben\", \"Cara\"].map(len) = {:?}", ["Ada", "Ben", "Cara"].map(len));
    let owned: Vec<String> = ["a", "bb"].into_iter().map(String::from).collect();
    println!("   .map(String::from)              = {owned:?}   an associated function");
    let wrapped: Vec<Option<u32>> = [1u32, 2].into_iter().map(Some).collect();
    println!("   .map(Some)                      = {wrapped:?}   an enum variant is one too");

    println!("\n4. What a `fn` pointer can carry: nothing");
    println!("   apply_ptr(10, double)      = {}", apply_ptr(10, double));
    println!("   apply_ptr(10, |v| v + 1)   = {}   a closure that captured nothing coerces", apply_ptr(10, |v| v + 1));
    let bonus = 5;
    println!("   apply_ptr(10, |v| v + bonus)");
    println!("     -> E0308: expected fn pointer, found closure");
    println!("        note: closures can only be coerced to `fn` types if they");
    println!("              do not capture any variables");

    println!("\n5. So take an `Fn` bound, not a `fn` parameter");
    println!("   apply_bound(10, double)        = {}   a fn item", apply_bound(10, double));
    println!("   apply_bound(10, a)             = {}   a fn pointer", apply_bound(10, a));
    println!("   apply_bound(10, |v| v + 1)     = {}   a free closure", apply_bound(10, |v| v + 1));
    println!("   apply_bound(10, |v| v + bonus) = {}   a CAPTURING closure", apply_bound(10, |v| v + bonus));
    println!("   Every `fn` implements Fn, FnMut and FnOnce — it borrows nothing, so");
    println!("   it satisfies all three. The bound accepts four callers; `fn` accepts two.");

    println!("\n6. Where a bare `fn` still wins: it is ONE concrete type");
    let table: [fn(u32) -> u32; 2] = [double, halve];
    println!("   [fn(u32) -> u32; 2] applied to 8 = {:?}", table.iter().map(|f| f(8)).collect::<Vec<_>>());
    println!("   size_of_val(&table)              = {} bytes   two pointers, no allocation", std::mem::size_of_val(&table));
    let mut by_name: HashMap<&str, fn(u32) -> u32> = HashMap::new();
    by_name.insert("double", double);
    by_name.insert("halve", halve);
    println!("   HashMap<&str, fn(u32) -> u32>    -> by_name[\"halve\"](8) = {}", by_name["halve"](8));
    let r = Rule { name: "double", apply: double };
    println!("   struct Rule {{ name, apply }}      -> {} of 21 = {}", r.name, (r.apply)(21));
    println!("   size_of::<Rule>()                = {} bytes", std::mem::size_of::<Rule>());
    const OP: fn(u32) -> u32 = double;
    println!("   const OP: fn(u32) -> u32         -> OP(3) = {}", OP(3));
    println!("   A `dyn Fn` needs a Box and a vtable to do any of this. A generic");
    println!("   parameter cannot do it at all: two closures are two types.");

    println!("\n7. And it is never null, so `Option` costs nothing");
    println!("   size_of::<fn(u32) -> u32>()         = {} bytes", std::mem::size_of::<fn(u32) -> u32>());
    println!("   size_of::<Option<fn(u32) -> u32>>() = {} bytes   same — None is the niche", std::mem::size_of::<Option<fn(u32) -> u32>>());
}
