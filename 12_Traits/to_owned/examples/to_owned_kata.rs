//! Kata solution: `.to_owned()` answers to the SOURCE, not to `String`.
//!
//!   rustc --edition 2024 to_owned_kata.rs -o /tmp/tok && /tmp/tok

use std::any::type_name_of_val;
use std::path::Path;
use std::rc::Rc;

fn main() {
    println!("1. Four receivers, four different owned twins");
    let text: &str = "Ada";
    let nums: &[i32] = &[5, 2, 0];
    let path: &Path = Path::new("/tmp/ballot");
    let already: String = String::from("Ben");

    let a = text.to_owned();
    let b = nums.to_owned();
    let c = path.to_owned();
    let d = (&already).to_owned();

    println!("   &str     .to_owned() -> {}", type_name_of_val(&a));
    println!("   &[i32]   .to_owned() -> {}", type_name_of_val(&b));
    println!("   &Path    .to_owned() -> {}", type_name_of_val(&c));
    println!("   &String  .to_owned() -> {}", type_name_of_val(&d));
    println!("   One method name, four answers: `type Owned` is chosen by the source.");

    println!();
    println!("2. So it is NOT a stringifying operation");
    // The prediction most people get wrong. `i32: Clone`, so the blanket
    // `impl<T: Clone> ToOwned for T` applies and the owned twin of an i32 is
    // an i32 — this does not produce text and never could.
    let n = 42.to_owned();
    println!("   42.to_owned()  -> {:<3}  ({})", n, type_name_of_val(&n));
    println!("   42.to_string() -> {:<5}({})", format!("{:?}", 42.to_string()), type_name_of_val(&42.to_string()));
    println!("   `to_string` is about TEXT. `to_owned` is about OWNERSHIP.");
    println!("   They coincide on a &str and nowhere else.");

    println!();
    println!("3. The trap: on an Rc it clones the POINTER");
    let shared = Rc::new(String::from("ballot"));
    let second = shared.to_owned();
    println!("   type          {}", type_name_of_val(&second));
    println!("   strong_count  {}", Rc::strong_count(&shared));
    println!("   same buffer?  {}", Rc::ptr_eq(&shared, &second));
    let deep: String = (*shared).clone();
    println!("   the real copy is (*shared).clone() -> {deep:?}");

    println!();
    println!("4. Why `Owned: Borrow<Self>` is in the trait");
    // Every owned twin above can lend its borrowed half back, which is what
    // lets one signature take either side of the pair.
    fn shout(s: &str) -> String { s.to_uppercase() }
    println!("   shout(&a) = {:?}   <- String lends a &str back", shout(&a));
    println!("   the bound is the round trip, and it is what makes Cow possible.");
}
