//! Step 5 of the `ToOwned` path: `x.method()` tries the receiver's own type
//! first, then `&` of it, then `&mut` of it, and only then dereferences and
//! repeats. The first method that fits wins.
//!
//!   rustc --edition 2024 the_dot_picks_first.rs -o /tmp/tdpf && /tmp/tdpf

use std::any::type_name_of_val;

fn main() {
    println!("Checkpoint. What type does each clone return?");
    let s = String::from("Ada");
    let name: &str = "Adam";
    let r: &String = &s;
    // rustc warns on this one (`noop_method_call`): cloning a &str copies the
    // reference. The warning is the lesson, so it is silenced here, not fixed.
    #[allow(noop_method_call)]
    let from_str = name.clone();
    println!("   name.clone()      name: &str    -> {}", type_name_of_val(&from_str));
    println!("   r.clone()         r: &String    -> {}", type_name_of_val(&r.clone()));
    println!("   Clone::clone(&r)                -> {}", type_name_of_val(&Clone::clone(&r)));

    println!();
    println!("Name only the trait, and the argument's type decides Self");
    println!("   Clone::clone(r)     r: &String    Self = String  -> {}", type_name_of_val(&Clone::clone(r)));
    println!("   Clone::clone(&r)    &r: &&String  Self = &String -> {}", type_name_of_val(&Clone::clone(&r)));
    println!("Name the type too, and the argument is coerced to fit it");
    println!("   <String as Clone>::clone(&r)      Self = String  -> {}", type_name_of_val(&<String as Clone>::clone(&r)));
    println!("   <&String as Clone>::clone(&r)     Self = &String -> {}", type_name_of_val(&<&String as Clone>::clone(&r)));

    println!();
    println!("The same search, for to_owned");
    let t: &str = "hi";
    let mut owner = String::from("hi");
    let m: &mut str = owner.as_mut_str();
    println!("   t.to_owned()      t: &str       -> {}", type_name_of_val(&t.to_owned()));
    println!("   m.to_owned()      m: &mut str   -> {}", type_name_of_val(&m.to_owned()));
    let tt: &&str = &t;
    println!("   tt.to_owned()     tt: &&str     -> {}", type_name_of_val(&tt.to_owned()));
}
