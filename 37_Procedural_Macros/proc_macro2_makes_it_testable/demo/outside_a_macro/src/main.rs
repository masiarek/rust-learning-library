//! The same parse, done twice in an ordinary program: once with `proc_macro2`,
//! then with the compiler's own `proc_macro`.

// Cargo links `proc_macro` into proc-macro crates only; any other crate names it.
extern crate proc_macro;

fn main() {
    let tokens: proc_macro2::TokenStream = "struct Point { x: i32 }".parse().unwrap();
    println!("proc_macro2: {tokens}");

    let tokens: proc_macro::TokenStream = "struct Point { x: i32 }".parse().unwrap();
    println!("proc_macro: {tokens}"); // never printed: the parse above panics
}
