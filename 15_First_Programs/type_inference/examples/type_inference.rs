//! Type inference: the compiler reads the whole function before deciding.
//!
//!     rustc --edition 2024 type_inference.rs -o /tmp/ti && /tmp/ti

use std::any::type_name_of_val;
use std::mem::size_of_val;

fn takes_u32(x: u32) -> u32 {
    x
}
fn takes_i8(y: i8) -> i8 {
    y
}

fn main() {
    println!("1. The same literal, two types — decided by a LATER line");
    let x = 10;
    let y = 20;
    let _ = takes_u32(x);
    let _ = takes_i8(y);
    println!("   let x = 10;   ...then takes_u32(x)   x is {}", type_name_of_val(&x));
    println!("   let y = 20;   ...then takes_i8(y)    y is {}", type_name_of_val(&y));
    println!("   Nothing on the `let` lines differs. Delete the two calls and both");
    println!("   become i32. The type came from how the value is used.");
    println!();

    println!("2. This is not \"any type\"");
    println!("   x occupies {} byte(s), y occupies {}", size_of_val(&x), size_of_val(&y));
    println!("   A dynamically typed value carries a tag at runtime and costs the");
    println!("   same whatever it holds. These are 4 bytes and 1 byte, fixed at");
    println!("   compile time — the identical machine code the annotated spelling");
    println!("   would have produced.");
    println!();

    println!("3. Inference reaches forward through a whole chain");
    let mut names = Vec::new();
    names.push("Ada");
    println!("   let mut names = Vec::new();   // no element type yet");
    println!("   names.push(\"Ada\");            // ...decided here");
    println!("   names is {}", type_name_of_val(&names));
    println!("   The `let` line is not where the type is known; it is only where");
    println!("   the question is asked.");
    println!();

    println!("4. The two fallbacks, for when nothing decides");
    let n = 1;
    let f = 1.0;
    println!("   let n = 1;     -> {}", type_name_of_val(&n));
    println!("   let f = 1.0;   -> {}", type_name_of_val(&f));
    println!("   Rust never picks a *width* for you by guessing at the value: 1");
    println!("   does not become u8 because it is small. It becomes i32 because");
    println!("   that is the written-down fallback.");
    println!();

    println!("5. Before it settles, the type has a placeholder name");
    println!("      let x = 3.14;");
    println!("      let y = 20;");
    println!("      assert_eq!(x, y);");
    println!("   error[E0277]: can't compare `{{float}}` with `{{integer}}`");
    println!("   {{float}} and {{integer}} are not types you can write. They are the");
    println!("   compiler saying \"a number whose width is still undecided\" — and");
    println!("   the reason this line fails is that no fallback ever makes two");
    println!("   different fallbacks equal.");
    println!();

    println!("6. Where inference stops: a signature is never inferred");
    println!("      fn double(n) {{ n * 2 }}       // not Rust");
    println!("      fn double(n: i32) -> i32     // every parameter, every return");
    println!("   Inside a body the compiler solves; at a boundary you declare. That");
    println!("   is why changing a function body cannot silently change its callers.");
    println!();

    println!("7. When there is genuinely nothing to go on");
    println!("      let v = Vec::new();          // error[E0282]: type annotations needed");
    println!("      let x = \"42\".parse();        // error[E0284]");
    println!("   Two ways to answer, and they are the same answer written twice:");
    let parsed: i32 = "42".parse().unwrap();
    let turbo = "42".parse::<i32>().unwrap();
    println!("      let parsed: i32 = \"42\".parse().unwrap();   {parsed}");
    println!("      let turbo = \"42\".parse::<i32>().unwrap();  {turbo}");
    println!("   equal? {}", parsed == turbo);
    println!();

    println!("The rule");
    println!("   An unannotated `let` is not an unknown type — it is a type the");
    println!("   compiler works out from everything you do with the value, falling");
    println!("   back to i32 or f64 only when nothing in the function decides.");
}
