//! `Some` and `None`: reading an `Option` with `match`.
//!
//! The first thing anyone does with an `Option` is ask which of the two shapes
//! it is in. `match` is the form that always works and the one the compiler
//! checks; `unwrap_or` is the shortcut for "give me the value or this default";
//! `unwrap` is the one that panics, and the one to be suspicious of.
//!
//!   rustc --edition 2024 some_and_none.rs -o /tmp/san && /tmp/san

use std::panic;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
// Rust's standard library defines:
//     enum Option<T> { Some(T), None }
// Nothing more. Every method below is ordinary code written on top of `match`.
fn describe(favnum: Option<i32>) {
    match favnum {
        Some(n) => println!("  Some({n}) -> your favourite number is {n}, good choice"),
        None => println!("  None    -> you don't have a favourite number... what?!"),
    }
}

fn step1() {
    banner(1, "One variable, two possible shapes, one match");

    describe(Some(3));
    describe(None);
    println!("      Same variable, same match, both shapes handled. Delete an arm and");
    println!("      the program stops compiling — 'I forgot the None case' is not a");
    println!("      bug you can ship.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "`Some(3)` and `Option::Some(3)` are the same thing");

    let short: Option<i32> = Some(3);
    let long: Option<i32> = Option::Some(3);
    let short_none: Option<i32> = None;
    let long_none: Option<i32> = Option::None;

    println!("  Some(3) == Option::Some(3) -> {}", short == long);
    println!("  None    == Option::None    -> {}", short_none == long_none);
    println!("      Both variants are in the prelude, so the `Option::` prefix is only");
    println!("      needed when some other name in scope collides. Write the short form.");
    println!("  The annotation is not decoration: `let x = None;` on its own does not");
    println!("      compile, because nothing in that line says what T is.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "Getting the value out");

    let present: Option<i32> = Some(3);
    let absent: Option<i32> = None;

    println!("  Some(3).unwrap_or(42)          -> {}", present.unwrap_or(42));
    println!("  None.unwrap_or(42)             -> {}", absent.unwrap_or(42));
    println!("  None.unwrap_or_default()       -> {}", absent.unwrap_or_default());
    println!("  None.unwrap_or_else(|| 7 * 6)  -> {}", absent.unwrap_or_else(|| 7 * 6));

    let prior = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // keep the demo output clean
    let outcome = panic::catch_unwind(|| absent.unwrap());
    panic::set_hook(prior);

    match outcome {
        Ok(v) => println!("  None.unwrap()                  -> {v}"),
        Err(_) => println!("  None.unwrap()                  -> panicked (caught here only to keep this demo running)"),
    }
    println!("      `unwrap` and `expect` are the same call; only `expect` leaves behind");
    println!("      the sentence you will want six months from now. Neither belongs in");
    println!("      code that ships unless you can write down why None is impossible.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "`Some(0)` is not `None` — the trap Python and ABAP both set");

    let scored_zero: Option<i32> = Some(0);
    let never_scored: Option<i32> = None;

    println!("  Some(0).unwrap_or(42) -> {}", scored_zero.unwrap_or(42));
    println!("  None.unwrap_or(42)    -> {}", never_scored.unwrap_or(42));
    println!("  Some(0).is_some() -> {}", scored_zero.is_some());
    println!("  Some(0) == None   -> {}", scored_zero == never_scored);
    println!("      Python's `favnum or 42` answers 42 for BOTH of these, because 0 is");
    println!("      falsy. ABAP says the same with IS INITIAL: 0 and 'never set' are one");
    println!("      value. Rust keeps them apart, which is the difference between a");
    println!("      ballot that scored a candidate 0 and one that left them blank.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "The trap: `match favnum` MOVES the value, and i32 hides it");

    let fav_num: Option<i32> = Some(3);
    match fav_num {
        Some(n) => println!("  match fav_num  -> Some({n})"),
        None => println!("  match fav_num  -> None"),
    }
    println!("  fav_num is still usable afterwards -> {}", fav_num.unwrap_or(42));
    println!("      i32 is Copy, so the match copied it. Nothing moved, nothing broke.");

    let fav_name: Option<String> = Some("Ada".to_string());
    match &fav_name {
        Some(name) => println!("  match &fav_name -> Some({name})"),
        None => println!("  match &fav_name -> None"),
    }
    println!(
        "  fav_name is still usable afterwards -> {}",
        fav_name.as_deref().unwrap_or("nobody")
    );
    println!("      String is NOT Copy. Written the first way — `match fav_name` — the");
    println!("      match would move it and the next line would not compile. Matching on");
    println!("      a reference borrows instead, which is why `&` is there.");
}

fn main() {
    println!("Some and None: reading an Option");
    step1();
    step2();
    step3();
    step4();
    step5();
    println!();
}
