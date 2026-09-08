//! Kata solution: the call the compiler cannot check, and the one it can.
//!
//!   rustc --edition 2024 an_enum_instead_of_a_bool_kata.rs -o /tmp/eib && /tmp/eib

use std::mem::size_of;

#[derive(Debug, Clone, Copy)]
enum Casing { Lower, Preserve }
#[derive(Debug, Clone, Copy)]
enum Trimming { Trim, Keep }

fn clean_bools(s: &str, lowercase: bool, trim: bool) -> String {
    let t = if trim { s.trim() } else { s };
    if lowercase { t.to_lowercase() } else { t.to_string() }
}

fn clean(s: &str, casing: Casing, trimming: Trimming) -> String {
    let t = match trimming { Trimming::Trim => s.trim(), Trimming::Keep => s };
    match casing { Casing::Lower => t.to_lowercase(), Casing::Preserve => t.to_string() }
}

fn main() {
    let input = "  Hello World  ";

    println!("THE BOOL VERSION, CALLED TWO WAYS");
    println!("  clean_bools(input, true, false)  {:?}", clean_bools(input, true, false));
    println!("  clean_bools(input, false, true)  {:?}", clean_bools(input, false, true));
    println!("  Both compile. One of them is the call the author meant and the");
    println!("  other is the arguments swapped, and nothing anywhere can tell");
    println!("  you which -- the types are identical, so the compiler has no");
    println!("  question to ask.");
    println!();

    println!("AT THE CALL SITE, WHICH IS WHERE IT IS READ");
    println!("  clean_bools(s, true, false)      <- true what? false what?");
    println!("  clean(s, Casing::Lower, Trimming::Keep)");
    println!("  The second one is readable without opening the function. That");
    println!("  is most of the value, and it costs nothing.");
    println!();

    println!("THE ENUM VERSION, WITH THE ARGUMENTS SWAPPED");
    println!("  clean(input, Trimming::Trim, Casing::Lower)");
    println!("  does not compile: expected `Casing`, found `Trimming`. The");
    println!("  backwards call became a build error rather than a wrong answer");
    println!("  in production.");
    println!("  clean(input, Casing::Lower,    Trimming::Trim) = {:?}",
             clean(input, Casing::Lower, Trimming::Trim));
    println!("  clean(input, Casing::Preserve, Trimming::Keep) = {:?}",
             clean(input, Casing::Preserve, Trimming::Keep));
    println!();

    println!("AND IT IS FREE");
    println!("  size_of::<bool>()      {} byte", size_of::<bool>());
    println!("  size_of::<Casing>()    {} byte", size_of::<Casing>());
    println!("  Two named variants cost exactly what two unnamed states cost.");
    println!("  There is no runtime price for the check, because the check");
    println!("  happened at compile time.");
    println!();

    println!("WHEN A bool IS STILL RIGHT");
    println!("  When the name at the call site already says everything:");
    println!("  `v.is_empty()`, `s.contains(c)`, `if found {{ ... }}`. The rule");
    println!("  is about PARAMETERS -- a bool you pass in is two unnamed states");
    println!("  arriving somewhere the name is gone.");

    assert_eq!(size_of::<Casing>(), 1);
    assert_eq!(clean(input, Casing::Lower, Trimming::Trim), "hello world");
}
