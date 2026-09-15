//! A `const` in a function body is an item: the literal's machine code, a name
//! for the whole block, and no view of anything local.
//!
//!   rustc --edition 2024 const_in_a_function.rs -o /tmp/cif && /tmp/cif

fn named() -> String {
    const GREETING: &str = "hello";
    GREETING.to_string()
}

fn literal() -> String {
    "hello".to_string()
}

/// Compiles: an item's name covers its whole block, lines above it included.
fn used_above_its_line() -> String {
    let s = GREETING.to_string();
    const GREETING: &str = "hello";
    s
}

/// Two items with one name, each in its own block.
fn sibling_blocks() -> (u32, u32) {
    let a = {
        const LIMIT: u32 = 10;
        LIMIT
    };
    let b = {
        const LIMIT: u32 = 20;
        LIMIT
    };
    (a, b)
}

/// `const N: usize = size_of::<T>();` here is E0401. An inline `const { }` is an
/// expression, not an item, so it sees `T`.
fn width<T>() -> usize {
    const { size_of::<T>() }
}

fn main() {
    println!("1. A named const and the literal it names");
    println!("   named()   = {:?}", named());
    println!("   literal() = {:?}", literal());
    println!("   Same value and the same machine code: the const is pasted in where");
    println!("   it is named. Both allocate a String in to_string(); the name makes");
    println!("   nothing cheaper at run time.");

    println!();
    println!("2. In scope for the whole block, above its own line too");
    println!("   used_above_its_line() = {:?}", used_above_its_line());
    println!("   A `let` is in scope from its own line down. A `const` in a block is");
    println!("   an item, and an item's name covers the block it is declared in.");

    println!();
    println!("3. Not in scope outside that block");
    println!("   sibling_blocks() = {:?}", sibling_blocks());
    println!("   Two LIMITs in sibling blocks do not collide, and named()'s GREETING");
    println!("   is not in scope in main: naming it there is E0425.");

    println!();
    println!("4. It cannot see the function it sits in");
    println!("   a `let` binding above it     -> E0435, use `let` instead");
    println!("   the function's own type T    -> E0401, use `const {{ }}` instead");
    println!("   width::<u32>() = {}, from `const {{ size_of::<T>() }}`", width::<u32>());
}
