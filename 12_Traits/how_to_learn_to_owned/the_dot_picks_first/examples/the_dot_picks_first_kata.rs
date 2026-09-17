//! Kata solution for step 5: predict which receiver the dot settles on, then
//! take the choice away from it — once by dereferencing to the rung you want,
//! once with a fully qualified call that names the type, so nothing is searched.
//!
//!   rustc --edition 2024 the_dot_picks_first_kata.rs -o /tmp/tdpfk && /tmp/tdpfk

use std::any::type_name_of_val;
use std::rc::Rc;

fn main() {
    let owner = String::from("Ada");
    let r: &String = &owner;
    let rr: &&String = &r;
    let s: &str = "Ada";
    let ss: &&str = &s;
    let shared: Rc<String> = Rc::new(String::from("Ada"));
    let mut scratch = String::from("Ada");
    let m: &mut String = &mut scratch;

    println!("1. Predict, then check");
    println!("   r.clone()       r: &String     -> {}", type_name_of_val(&r.clone()));
    // rustc warns here (`suspicious_double_ref_op`) — the warning is the point.
    #[allow(suspicious_double_ref_op)]
    let peeled = rr.clone();
    println!("   rr.clone()      rr: &&String   -> {}", type_name_of_val(&peeled));
    println!("   ss.to_owned()   ss: &&str      -> {}", type_name_of_val(&ss.to_owned()));
    println!("   shared.clone()  shared: Rc     -> {}", type_name_of_val(&shared.clone()));
    println!("   m.clone()       m: &mut String -> {}", type_name_of_val(&m.clone()));

    println!();
    println!("2. The three that were not a String, fixed by dereferencing to the right rung");
    println!("   (*rr).clone()        -> {}", type_name_of_val(&(*rr).clone()));
    println!("   (*ss).to_owned()     -> {}", type_name_of_val(&(*ss).to_owned()));
    println!("   (*shared).clone()    -> {}", type_name_of_val(&(*shared).clone()));

    println!();
    println!("3. The same three with no search at all: name the type, and the argument is coerced");
    println!("   <String as Clone>::clone(rr)       -> {}", type_name_of_val(&<String as Clone>::clone(rr)));
    println!("   <str as ToOwned>::to_owned(ss)     -> {}", type_name_of_val(&<str as ToOwned>::to_owned(ss)));
    println!("   <String as Clone>::clone(&shared)  -> {}", type_name_of_val(&<String as Clone>::clone(&shared)));
}
