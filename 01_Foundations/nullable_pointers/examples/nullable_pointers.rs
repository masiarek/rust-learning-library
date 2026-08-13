//! Nullable pointers: Rust has none, so `Option<Box<T>>` is how you say "maybe a pointer".
//!
//! Every Rust pointer type — `&T`, `&mut T`, `Box<T>` — must point at a valid
//! value. There is no null to check for and no way to write one. When a pointer
//! genuinely might be absent you widen the type instead: `Option<Box<T>>`, which
//! the compiler stores in the same 8 bytes and refuses to let you read blind.
//!
//!   rustc --edition 2024 nullable_pointers.rs -o /tmp/np && /tmp/np

use std::mem::size_of;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn check_optional(optional: Option<Box<i32>>) {
    match optional {
        Some(p) => println!("  has value {p}"),
        None => println!("  has no value"),
    }
}

fn step1() {
    banner(1, "An optional owned box");

    check_optional(None);
    check_optional(Some(Box::new(9000)));

    println!("      To use the i32 you must first say what happens when it is not");
    println!("      there. The match is not ceremony — it IS the null check, and the");
    println!("      compiler will not let you skip it.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn describe(p: Option<&i32>) -> String {
    match p {
        Some(v) => format!("points at {v}"),
        None => String::from("points at nothing"),
    }
}

fn step2() {
    banner(2, "The same trick for borrowed pointers");

    let n = 7;
    println!("  describe(Some(&n)) -> {}", describe(Some(&n)));
    println!("  describe(None)     -> {}", describe(None));
    println!("      `&i32` is the non-nullable reference; `Option<&i32>` is the");
    println!("      nullable one. Two different types, so a function's signature");
    println!("      says which it accepts and no caller can get it wrong.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "The optional pointer is free — with one exception");

    println!("  Box<i32>                  {} bytes", size_of::<Box<i32>>());
    println!("  Option<Box<i32>>          {}", size_of::<Option<Box<i32>>>());
    println!("  &i32                      {}", size_of::<&i32>());
    println!("  Option<&i32>              {}", size_of::<Option<&i32>>());
    println!("  *const i32                {}", size_of::<*const i32>());
    println!("  Option<*const i32>        {}   <- the exception", size_of::<Option<*const i32>>());
    println!("      A Box or a reference can never be null, so the compiler spends that");
    println!("      impossible bit pattern on None and the tag costs nothing. A RAW");
    println!("      pointer is allowed to be null, so it has no spare pattern left and");
    println!("      Option has to store a real tag beside it. Same idea as C's nullable");
    println!("      pointer, then, but you cannot forget the check.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "In practice you rarely write the whole match");

    let some: Option<Box<i32>> = Some(Box::new(9000));
    let none: Option<Box<i32>> = None;

    if let Some(p) = &some {
        println!("  if let           got {p}");
    }
    println!("  map              {:?}", some.as_deref().map(|v| v * 2));
    println!("  map (on None)    {:?}", none.as_deref().map(|v| v * 2));
    println!("  unwrap_or        {}", none.as_deref().copied().unwrap_or(-1));
    println!("  is_some_and      {}", some.as_deref().is_some_and(|v| *v > 100));

    let p = some.unwrap();
    println!("  deref            *p + 1 = {}", *p + 1);
    println!("      Note what `p` is in the Some arm: a Box<i32>, not an i32. It prints");
    println!("      and dereferences like the number because Box derefs to its contents.");
    println!("      `as_deref()` is the one to remember: Option<Box<T>> -> Option<&T>,");
    println!("      so you can look inside without consuming the box.");
}

// ─────────────────────────────────────────────────────────── Step 5
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn step5() {
    banner(5, "Why this type exists at all: a structure that ends");

    let list = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: Some(Box::new(Node { value: 3, next: None })),
        })),
    };

    let mut cur: Option<&Node> = Some(&list);
    let mut walked: Vec<String> = Vec::new();
    let mut sum = 0;
    while let Some(n) = cur {
        walked.push(n.value.to_string());
        sum += n.value;
        cur = n.next.as_deref();
    }
    println!("  {} -> end   (sum {sum})", walked.join(" -> "));

    println!("  size_of::<Node>()         {} bytes", size_of::<Node>());
    println!("      `next: Option<Box<Node>>` carries both halves of the answer. The Box");
    println!("      gives Node a known size, without which the type is infinitely large");
    println!("      and does not compile; the Option is what lets the chain STOP. A C");
    println!("      linked list writes that as a null next-pointer and hopes; here the");
    println!("      end of the list is a value the type system knows about — and it still");
    println!("      costs 16 bytes a node, with no separate 'is there a next?' flag.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    println!();
}
