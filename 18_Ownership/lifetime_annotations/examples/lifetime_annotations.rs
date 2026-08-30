//! `<'a>` names a relationship the compiler checks. It grants nothing.
//!
//!   rustc --edition 2024 lifetime_annotations.rs -o /tmp/la && /tmp/la

use std::mem::size_of;

// 1. Two references in, one out. `'a` is what says the answer is borrowed
//    from BOTH, so it may not outlive either.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// 2. Elision: three shapes that need no annotation at all, because the rules
//    fill the hole unambiguously.
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or(s)
}

// 4. Two DIFFERENT lifetimes. The answer is cut out of `text`, so it borrows
//    from `text` alone; `delimiter` is read and never returned, and gets its
//    own elided lifetime. That is information the one-lifetime version throws
//    away — and it is exactly the shape `str::split` itself has.
fn trimmed_to<'a>(text: &'a str, delimiter: &str) -> &'a str {
    text.split(delimiter).next().unwrap_or(text)
}

// 3. A struct that holds a borrow. The parameter is now part of the type.
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(part: &'a str) -> Self {
        Excerpt { part }
    }

    // Third elision rule: with `&self` present, the elided output lifetime is
    // self's. No annotation needed even though one is returned.
    fn shout(&self) -> &str {
        self.part
    }
}

fn main() {
    println!("1. `'a` ties the answer to both arguments");
    let long = String::from("a long string");
    let short = String::from("short");
    println!("   longest(&long, &short) = {}", longest(&long, &short));
    println!("   `'a` is the SHORTER of the two lifetimes, not a duration the");
    println!("   annotation hands out — move `short` into an inner scope and");
    println!("   the identical signature gives E0597.");

    println!();
    println!("2. Most signatures need no annotation — elision fills the hole");
    println!("   first_word(\"Call me Ishmael\") = {}", first_word("Call me Ishmael"));
    let novel = String::from("Call me Ishmael. Some years ago");
    println!("   first_word(&novel)            = {}", first_word(&novel));
    println!("   One input reference, one output: there is only one source it");
    println!("   could borrow from, so the compiler writes `'a` for you.");

    println!();
    println!("3. On a struct the parameter becomes part of the type");
    let excerpt = Excerpt::new(novel.split('.').next().unwrap());
    println!("   excerpt.part  = {}", excerpt.part);
    println!("   excerpt.shout() = {}", excerpt.shout());
    println!("   size_of::<Excerpt>() = {}   the same as the &str it holds ({})",
        size_of::<Excerpt<'_>>(), size_of::<&str>());
    println!("   `impl<'a> Excerpt<'a>` declares it and then uses it — the same");
    println!("   say-it-twice shape as `impl<T> Container<T>`.");

    println!();
    println!("4. Two lifetimes say more than one");
    let kept = String::from("the part we keep. the rest");
    let head = trimmed_to(&kept, &String::from("."));
    println!("   trimmed_to(&kept, &temporary) = {head}");
    println!("   The delimiter was a temporary, freed at the end of that very");
    println!("   statement, and the answer outlives it. Write both parameters as");
    println!("   `'a` and the same line is E0716: the temporary would have to");
    println!("   live as long as the borrow it has nothing to do with.");

    println!();
    println!("5. `'static` is not a different kind of thing, just the longest");
    let literal: &'static str = "lives as long as the program";
    println!("   longest(&long, literal) = {}", longest(&long, literal));
    println!("   A `&'static str` is accepted wherever `&'a str` is wanted,");
    println!("   because 'static outlives every 'a. The reverse never holds.");
}
