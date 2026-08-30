//! What `let` does, and what `mut` adds to it.
//!
//! Every claim on the page comes from here. Run it:
//!     rustc --edition 2024 variables.rs -o /tmp/variables && /tmp/variables

use std::any::type_name_of_val;

fn main() {
    println!("1. `let` binds a name to a value");
    let x: i32 = 10;
    let y = 10;
    println!("   let x: i32 = 10;   x = {x}   {}", type_name_of_val(&x));
    println!("   let y = 10;        y = {y}   {}", type_name_of_val(&y));
    println!("   Same program. The annotation is a check, not a requirement —");
    println!("   nothing here needed it, because 10 has a fallback type.");
    println!();

    println!("2. The binding is immutable by default");
    println!("   Adding a second line does not work:");
    println!("      let x: i32 = 10;");
    println!("      x = 20;   // error[E0384]: cannot assign twice to immutable variable");
    println!("   `x` is not a box holding 10 that you may refill. It is a name for");
    println!("   a value, and the name is spoken for.");
    println!();

    println!("3. `mut` is the permission that assignment needs");
    let mut count = 0;
    println!("   let mut count = 0;   count = {count}");
    count = 1;
    println!("   count = 1;           count = {count}");
    count += 1;
    println!("   count += 1;          count = {count}");
    println!("   One word, and every later line may write to it.");
    println!();

    println!("4. `mut` you did not use is a warning, not a shrug");
    println!("      let mut total = 0;      // never assigned again");
    println!("      println!(\"{{total}}\");");
    println!("   warning: variable does not need to be mutable");
    println!("   The compiler holds you to the promise: `mut` says *this changes*,");
    println!("   so a `mut` nothing writes to is a claim the code does not keep.");
    println!();

    println!("5. Mutability belongs to the binding, not to the value");
    let owned = String::from("Ada");
    let mut moved = owned; // `owned` was not mut; `moved` is
    moved.push_str(" Lovelace");
    println!("   let owned = String::from(\"Ada\");   // not mut");
    println!("   let mut moved = owned;             // now it is");
    println!("   moved.push_str(\" Lovelace\");       moved = {moved:?}");
    println!("   The same bytes were immutable under one name and writable under");
    println!("   the next. Nothing about the String changed — only the binding.");
    println!();

    println!("6. A binding is scoped to its block");
    let outer = "visible to the end of main";
    {
        let inner = "visible only inside these braces";
        println!("   inner: {inner}");
    }
    println!("   outer: {outer}");
    println!("      println!(\"{{inner}}\");   // error[E0425]: cannot find value `inner`");
    println!();

    println!("7. `let` again is a new variable, not an assignment");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("   let spaces = \"   \";        &str");
    println!("   let spaces = spaces.len();  {}   spaces = {spaces}", type_name_of_val(&spaces));
    println!("   That is shadowing, and it needed no `mut` — because nothing was");
    println!("   assigned. A second variable took over the name.");
    println!();

    println!("The rule");
    println!("   `let` names a value; `mut` lets a later line write through the name;");
    println!("   `let` again replaces the name and may change its type.");
}
