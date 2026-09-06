//! Kata solution: the conversion you did not need, and the method that changed
//! under you.
//!
//!   rustc --edition 2024 str_as_str_kata.rs -o /tmp/sask && /tmp/sask

use std::any::type_name_of_val as type_of;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

fn shout(s: &str) -> String {
    s.to_uppercase()
}

/// Part two: a trait that hands out a view, on a type somebody else owns.
struct Reading([u8; 3]);

trait Samples {
    fn as_mut_slice(&mut self) -> &mut [u8];
}

impl Samples for Reading {
    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// ...and the inherent method that arrives in a later release.
impl Reading {
    fn as_mut_slice(&mut self) -> &mut [u8; 3] {
        &mut self.0
    }
}

/// Only a `&mut [u8]` will do here -- an array is a different type.
fn zero_out(samples: &mut [u8]) {
    for s in samples.iter_mut() {
        *s = 0;
    }
}

fn main() {
    println!("Part 1: five owners, and how many conversions you actually write");
    let owned = String::from("hello");
    let boxed: Box<str> = "hello".into();
    let counted: Rc<str> = "hello".into();
    let shared: Arc<str> = "hello".into();
    let maybe: Cow<'_, str> = Cow::Borrowed("hello");

    println!("   shout(&owned)    {:?}", shout(&owned));
    println!("   shout(&boxed)    {:?}", shout(&boxed));
    println!("   shout(&counted)  {:?}", shout(&counted));
    println!("   shout(&shared)   {:?}", shout(&shared));
    println!("   shout(&maybe)    {:?}", shout(&maybe));
    println!("   Answer: none of them. Every one of these types derefs to str,");
    println!("   so a plain `&x` at a call site expecting &str coerces, and the");
    println!("   whole as_str question never arises. That is why the missing");
    println!("   method is a papercut rather than a wall.");

    println!();
    println!("Part 1b: where the coercion has nothing to aim at");
    let all: Vec<Rc<str>> = vec!["ada".into(), "bob".into()];
    let views: Vec<&str> = all.iter().map(|r| &**r).collect();
    println!("   .map(|r| &**r)   {views:?}");
    println!("   A closure's return type is inferred, not demanded, so there is");
    println!("   no &str for the compiler to coerce toward and you must say it.");
    println!("   This is the case String::as_str exists for, and the case the");
    println!("   str_as_str feature wanted to cover for every other owner.");

    println!();
    println!("Part 2: the method that changed under you");
    let mut r = Reading([7, 8, 9]);
    println!("   r.as_mut_slice()               -> {}", type_of(&r.as_mut_slice()));
    println!("   Samples::as_mut_slice(&mut r)  -> {}", type_of(&Samples::as_mut_slice(&mut r)));
    println!("   The inherent method wins, so the trait's is unreachable");
    println!("   through a dot -- and zero_out(r.as_mut_slice()) now fails:");
    println!("     error[E0308]: mismatched types");
    println!("       expected `&mut [u8]`, found `&mut [u8; 3]`");

    println!();
    println!("   Two repairs, and only one of them is yours to make:");
    zero_out(Samples::as_mut_slice(&mut r));
    println!("   Samples::as_mut_slice(&mut r)  {:?}   name the trait", r.0);
    let mut r2 = Reading([7, 8, 9]);
    zero_out(&mut r2.as_mut_slice()[..]);
    println!("   &mut ..[..]                    {:?}   reslice the array", r2.0);
    println!("   The second works because [u8; 3] derefs to [u8]. Neither is a");
    println!("   fix for the crate that broke: its users' call sites are the");
    println!("   ones that changed meaning, and they did not ask for either.");
}
