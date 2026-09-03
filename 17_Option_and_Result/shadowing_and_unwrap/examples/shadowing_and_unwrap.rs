//! Shadowing and unwrapping: two unrelated ideas that meet in one idiom.
//!
//! Shadowing is about NAMES: `let x = …` a second time makes a new variable that
//! hides the old one, possibly with a different type. Unwrapping is about VALUES:
//! getting the `T` out of an `Option<T>`. Neither needs the other.
//!
//! The widely-copied explanation — that shadowing lets you keep working with an
//! unwrapped value "without affecting the original container" — credits shadowing
//! for something `Copy` is doing. This program shows which is which.
//!
//!   rustc --edition 2024 shadowing_and_unwrap.rs -o /tmp/sau && /tmp/sau

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

/// What the compiler thinks this expression's type is.
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "The example as it is usually written");

    let maybe_number: Option<i32> = Some(42);

    if let Some(number) = maybe_number {
        let number = number * 2; // the shadow
        println!("  The doubled value is: {number}");
    } else {
        println!("  No value found!");
    }

    let unwrapped_number = maybe_number.unwrap_or(0);
    println!("  The unwrapped value is: {unwrapped_number}");
    println!("      Two questions this raises: what did the shadow do, and why is");
    println!("      maybe_number still usable on the last line?");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "The shadow did nothing to protect the original");

    let maybe_number: Option<i32> = Some(42);

    // Same code with the shadow removed entirely.
    if let Some(number) = maybe_number {
        println!("  no shadow at all      -> doubled = {}", number * 2);
    }
    println!("  maybe_number afterwards -> {maybe_number:?}");
    println!("      Still intact. The shadow was never what kept it alive: `if let`");
    println!("      COPIED the i32 out, because Option<i32> is Copy.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "Take Copy away and the same code stops compiling");

    let maybe_name: Option<String> = Some("Ada".to_string());

    // `if let Some(name) = maybe_name` would MOVE the String out, and the line
    // after the block would not compile: E0382, use of partially moved value.
    // Borrowing is the fix, and it has nothing to do with shadowing.
    if let Some(name) = &maybe_name {
        let name = format!("{name} {name}"); // a shadow, purely for the new name
        println!("  borrowed, then shadowed -> {name}");
    }
    println!("  maybe_name afterwards   -> {maybe_name:?}");
    println!("      Option<String> is not Copy, so the by-value version is a compile");
    println!("      error no matter how the inside of the block is written.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "What shadowing IS for: the name stays, the type changes");

    let limit: Option<i32> = None;
    println!("  before: limit is {:?}   type {}", limit, type_of(&limit));

    let limit = limit.unwrap_or(0); // shadowing across the unwrap
    println!("  after:  limit is {}      type {}", limit, type_of(&limit));
    println!("      This is the one place the two ideas genuinely meet: unwrapping");
    println!("      changes the TYPE, and shadowing lets the NAME survive the change.");
    println!("      Without it you would invent limit_opt / limit_value pairs.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "A shadow ends with its block; the outer name comes back");

    let score = 42;
    {
        let score = score * 2;
        println!("  inside the block  -> {score}");
    }
    println!("  after the block   -> {score}");
    println!("      So shadowing cannot 'hold onto' an unwrapped value for later —");
    println!("      the shadow is the thing that disappears first.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "Shadowing is not mutation");

    let mut counted = 0;
    counted += 1;
    println!("  let mut: {counted}, type {}", type_of(&counted));
    // counted = "one";  <- error[E0308]: mismatched types. `mut` cannot retype.

    let raw = "  12  ";
    let raw = raw.trim();
    let raw = raw.parse::<u32>().unwrap_or_default();
    println!("  shadowed three times: {raw}, type {}", type_of(&raw));
    println!("      `mut` reuses one variable and its type is fixed. Each `let` makes a");
    println!("      NEW variable, so the type may change — which is the whole point.");
}

// ─────────────────────────────────────────────────────────── Step 7
fn columns_for(config: Option<u32>) -> u32 {
    // let-else: shadow the name, drop the wrapper, keep the happy path unindented.
    let Some(config) = config else {
        return 1;
    };
    config.max(1)
}

fn step7() {
    banner(7, "The idiom worth copying: let-else");

    println!("  columns_for(Some(3)) -> {}", columns_for(Some(3)));
    println!("  columns_for(None)    -> {}", columns_for(None));
    println!("      Inside the function, `config` is a u32 rather than an Option<u32>");
    println!("      from that line on, and nothing downstream can forget to unwrap it.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    step7();
    println!();
}
