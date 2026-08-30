//! What a closure is: a function that carries the values it mentioned.
//!
//!   rustc --edition 2024 what_a_closure_is.rs -o /tmp/wci && /tmp/wci

use std::mem::size_of_val;

/// A plain function. It can see its arguments and nothing else.
fn add_one(n: i32) -> i32 {
    n + 1
}

/// A higher-order function: the caller supplies the operation.
fn apply(n: i32, op: impl Fn(i32) -> i32) -> i32 {
    op(n)
}

/// Same job, taking a bare function pointer instead. Only a NON-capturing
/// closure can be passed here.
fn apply_ptr(n: i32, op: fn(i32) -> i32) -> i32 {
    op(n)
}

fn main() {
    println!("1. The syntax, next to the function it replaces");
    let add_one_closure = |n: i32| n + 1;
    println!("   fn add_one(n: i32) -> i32 {{ n + 1 }}   add_one(41)         = {}", add_one(41));
    println!("   |n: i32| n + 1                        add_one_closure(41) = {}", add_one_closure(41));
    let inferred = |n| n + 1;
    println!("   |n| n + 1        (types inferred)     inferred(41)        = {}", inferred(41));

    println!();
    println!("2. The difference is the capture: a closure can see the scope around it");
    let bonus = 10;
    let with_bonus = |n: i32| n + bonus;
    println!("   bonus = {bonus}, so with_bonus(1) = {}", with_bonus(1));
    println!("   a plain `fn` written in the same spot cannot: E0434, see the page");

    println!();
    println!("3. A closure IS a struct the compiler wrote — its size is what it captured");
    let nothing = |n: i32| n + 1;
    let one_int = move |n: i32| n + bonus;
    let name = String::from("Ada");
    let name2 = String::from("Lovelace");
    let one_string = move || name.len();
    println!("   |n| n + 1                 captures nothing   {} bytes", size_of_val(&nothing));
    println!("   move |n| n + bonus        captures one i32   {} bytes", size_of_val(&one_int));
    println!("   move || name.len()        captures a String  {} bytes", size_of_val(&one_string));
    println!("   for comparison: size_of::<String>()        = {} bytes", size_of::<String>());
    println!("   for comparison: size_of::<fn(i32) -> i32>()= {} bytes", size_of::<fn(i32) -> i32>());
    println!("   a closure that captured nothing is ZERO-SIZED — smaller than a fn pointer.");
    let borrowing_int = |n: i32| n + bonus;
    let borrowing_string = || name2.len();
    println!("   |n| n + bonus             borrows one i32    {} bytes", size_of_val(&borrowing_int));
    println!("   || name2.len()            borrows a String   {} bytes", size_of_val(&borrowing_string));
    println!("   the same two closures without `move`: each field is now a reference,");
    println!("   so both are one pointer wide. (calling them: {} {})",
             borrowing_int(1), borrowing_string());
    println!("   (calling them, so the compiler keeps them: {} {} {})",
             nothing(1), one_int(1), one_string());

    println!();
    println!("4. Which means every closure has its own anonymous type");
    println!("   two closures with identical text are two different types — E0308,");
    println!("   and rustc says so in as many words: \"no two closures, even if");
    println!("   identical, have the same type\". See the page for the transcript.");

    println!();
    println!("5. A closure that captured nothing coerces to a plain `fn` pointer");
    println!("   apply_ptr(41, |n| n + 1)          = {}", apply_ptr(41, |n| n + 1));
    println!("   apply_ptr(41, add_one)            = {}", apply_ptr(41, add_one));
    println!("   apply_ptr(41, |n| n + bonus)      -> E0308: expected fn pointer, found closure");

    println!();
    println!("6. Three ways to accept one, and what each costs");
    println!("   fn apply(n, op: impl Fn(i32)->i32)  apply(41, |n| n + bonus) = {}",
             apply(41, |n| n + bonus));
    let boxed: Box<dyn Fn(i32) -> i32> = Box::new(move |n| n + bonus);
    println!("   Box<dyn Fn(i32)->i32>               boxed(41)                = {}", boxed(41));
    println!("   size of the Box                     {} bytes (a fat pointer: data + vtable)",
             size_of_val(&boxed));
    println!("   `impl Fn` is one stamped-out copy per closure type — no indirection.");
    println!("   `Box<dyn Fn>` is one allocation and a virtual call, and it is what");
    println!("   you need the moment two different closures must share a variable.");

    println!();
    println!("7. Higher-order: the caller supplies the operation");
    let ops: Vec<(&str, Box<dyn Fn(i32) -> i32>)> = vec![
        ("add one", Box::new(|n| n + 1)),
        ("double", Box::new(|n| n * 2)),
        ("add bonus", Box::new(move |n| n + bonus)),
    ];
    for (label, op) in &ops {
        println!("   {label:<10} applied to 20 -> {}", op(20));
    }
    println!("   all three live in one Vec because `dyn Fn` erased their three types.");
}
