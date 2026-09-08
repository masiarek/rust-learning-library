//! Kata solution: match a reference against a non-reference pattern.
//!
//!   rustc --edition 2024 match_ergonomics_kata.rs -o /tmp/mek && /tmp/mek

fn main() {
    let opt: Option<String> = Some(String::from("hello"));

    println!("1. MATCHING A REFERENCE WITH AN ORDINARY PATTERN");
    match &opt {
        Some(name) => println!("  name is a &String: {name:?}, len {}", name.len()),
        None => println!("  nothing"),
    }
    println!("  `opt` is still ours afterwards: {opt:?}");
    println!();
    println!("  The scrutinee was `&Option<String>` and the pattern was");
    println!("  `Some(name)`, which is not a reference pattern. Rather than");
    println!("  refusing, the compiler DEREFERENCED the scrutinee and made every");
    println!("  binding inside a reference -- so `name: &String`, and nothing");
    println!("  moved out of `opt`. That is match ergonomics, RFC 2005.");
    println!();

    println!("2. WHAT IT SAVED YOU WRITING");
    match &opt {
        &Some(ref name) => println!("  the pre-2018 spelling: {name:?}"),
        &None => println!("  nothing"),
    }
    println!("  Both forms still compile and mean the same thing. The second is");
    println!("  what everyone had to write before, and it is why old code is");
    println!("  full of `ref`.");
    println!();

    println!("3. THE CASE THAT SURPRISES PEOPLE");
    let pair: Option<(String, u32)> = Some((String::from("k"), 1));
    if let Some((key, n)) = &pair {
        println!("  key: {key:?} is a &String, n: {n} is a &u32");
        println!("  n + 1 = {}", *n + 1);
    }
    println!("  Every binding became a reference, including the u32 -- which is");
    println!("  Copy, so people expect a value and get `&u32`. It usually still");
    println!("  works because of auto-deref in arithmetic and method calls, and");
    println!("  then fails the one time you need it by value.");
    println!();

    println!("4. WHEN YOU DO WANT TO MOVE");
    let taken = match opt {
        Some(name) => name,
        None => String::new(),
    };
    println!("  matching the VALUE (no &) moves it out: {taken:?}");
    println!("  ...and `opt` is no longer usable. Which one you get is decided");
    println!("  entirely by whether the scrutinee is a reference -- so `match");
    println!("  opt` and `match &opt` are two different programs, one character");
    println!("  apart.");
    println!();

    println!("THE RULE TO CARRY");
    println!("  Ask what the SCRUTINEE is, not what the pattern looks like. A");
    println!("  reference scrutinee gives reference bindings, all the way down,");
    println!("  and `as_ref()` is the explicit spelling when you want that from");
    println!("  an owned value.");

    let o2: Option<String> = Some(String::from("x"));
    let len = match &o2 { Some(s) => s.len(), None => 0 };
    assert_eq!(len, 1);
    assert!(o2.is_some());
}
