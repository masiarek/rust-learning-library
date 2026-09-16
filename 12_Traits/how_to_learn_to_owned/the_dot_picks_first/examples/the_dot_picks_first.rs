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

    println!();
    println!("Where the search goes past the first entry, and where it does not");
    let data = String::from("data");
    let r1: &String = &data;
    let r2: &&String = &r1;
    // rustc warns on this one too (`suspicious_double_ref_op`): it copies the
    // inner reference, not the String. Silenced for the same reason as above.
    #[allow(suspicious_double_ref_op)]
    let inner = r2.clone();
    println!("   r2.clone()        r2: &&String    -> {}   the inner reference, copied", type_name_of_val(&inner));
    println!("   Clone::clone(*r2)                 -> {}", type_name_of_val(&Clone::clone(*r2)));
    let mut scratch = String::from("data");
    let mu: &mut String = &mut scratch;
    println!("   mu.clone()        mu: &mut String -> {}   &mut is not Clone: deref, then &", type_name_of_val(&mu.clone()));
    println!("   r2.capacity()     r2: &&String    -> {}           capacity takes &String: one deref", type_name_of_val(&r2.capacity()));
}
