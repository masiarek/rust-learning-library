//! Kata solution: six call sites, three of which need a repair.
//!
//!   rustc --edition 2024 coercion_kata.rs -o /tmp/coek && /tmp/coek

fn takes_str(s: &str) -> usize {
    s.len()
}

fn takes_slice(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn takes_opt(o: Option<&str>) -> bool {
    o.is_some()
}

fn main() {
    let owned = String::from("ballot");
    let counts: Vec<i32> = vec![4, 5, 6];
    let held: Option<String> = Some(String::from("Ada"));
    let names: Vec<String> = vec!["Ada".to_string(), "Ben".to_string()];

    println!("Compiles as written — a coercion fires at the argument:");
    println!("  1. takes_str(&owned)          = {}", takes_str(&owned));
    println!("  2. takes_slice(&counts)       = {}", takes_slice(&counts));
    println!("  3. takes_slice(&[1, 2, 3])    = {}", takes_slice(&[1, 2, 3]));
    println!("     &String -> &str and &Vec<i32> -> &[i32] are deref coercions;");
    println!("     &[i32; 3] -> &[i32] is an unsizing coercion. All three are");
    println!("     invisible at the call site.");

    println!("\nRejected as written, and the repair:");

    // 4.  takes_str(owned)
    //     error[E0308]: mismatched types — expected `&str`, found `String`
    //     Coercion works on references; a value is not one.
    println!("  4. takes_str(owned)      -> takes_str(&owned)          = {}",
             takes_str(&owned));

    // 5.  takes_opt(held.as_ref())
    //     error[E0308]: expected `Option<&str>`, found `Option<&String>`
    //     Coercion does not reach inside another type.
    println!("  5. takes_opt(held.as_ref()) -> takes_opt(held.as_deref()) = {}",
             takes_opt(held.as_deref()));

    // 6.  takes_str(names)  /  passing Vec<String> where Vec<&str> is wanted
    //     Same reason as 5: a Vec is another type, and its element type is
    //     never adjusted for you. Every element has to be converted.
    let borrowed: Vec<&str> = names.iter().map(String::as_str).collect();
    println!("  6. Vec<String> -> Vec<&str> -> {:?}, first is {} chars",
             borrowed, takes_str(borrowed[0]));

    println!("\nThe rule the three failures share: a coercion adjusts the type of");
    println!("an expression at a site where the wanted type is already known, and");
    println!("it never descends into a generic type to adjust a parameter. The");
    println!("moment your value is INSIDE something — an Option, a Vec, a tuple —");
    println!("you write the conversion yourself.");
}
