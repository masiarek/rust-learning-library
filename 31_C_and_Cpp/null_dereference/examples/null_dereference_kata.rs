//! Kata solution: not a better pointer -- a different type.
//!
//!   rustc --edition 2024 null_dereference_kata.rs -o /tmp/ndk && /tmp/ndk

use std::mem::size_of;

#[derive(Debug)]
struct Config { retries: u32 }

fn find(name: &str) -> Option<Config> {
    if name == "known" { Some(Config { retries: 3 }) } else { None }
}

fn main() {
    println!("THE C SHAPE");
    println!("  Config *c = find(name);   /* may return NULL */");
    println!("  return c->retries;        /* and here it is dereferenced */");
    println!("  The type `Config *` promises to point at a Config. NULL fits in");
    println!("  it and answers none of the questions the type promises, so the");
    println!("  promise is checked by a convention in a comment.");
    println!();

    println!("RUST'S ANSWER IS A DIFFERENT TYPE");
    for name in ["known", "missing"] {
        match find(name) {
            Some(c) => println!("  find({name:?}) -> Some({c:?}), retries {}", c.retries),
            None => println!("  find({name:?}) -> None -- and there is no field to reach for"),
        }
    }
    println!();
    println!("  Option<Config> is not a Config that might be null. It is a");
    println!("  two-variant enum, and the ONLY way to the Config inside is a");
    println!("  match, an if-let, or a method that names what to do when there");
    println!("  is nothing. Forgetting the None case is E0004, not a segfault.");
    println!();

    println!("AND IT COSTS NOTHING");
    println!("  size_of::<&Config>()          {} bytes", size_of::<&Config>());
    println!("  size_of::<Option<&Config>>()  {} bytes", size_of::<Option<&Config>>());
    println!("  Identical. A reference can never be null, so the compiler uses");
    println!("  the all-zero bit pattern to mean None -- the NICHE optimisation.");
    println!("  The safety and the C representation are the same eight bytes,");
    println!("  which is also why passing Option<&T> across an FFI boundary is");
    println!("  sound and idiomatic.");
    println!();
    println!("  It works for anything with a spare pattern:");
    println!("  size_of::<Option<Box<u32>>>()   {} bytes", size_of::<Option<Box<u32>>>());
    println!("  size_of::<Option<u32>>()        {} bytes  <- no niche in a u32,",
             size_of::<Option<u32>>());
    println!("                                    so this one pays for a tag");
    println!();

    println!("THE HABITS THAT REPLACE THE NULL CHECK");
    let c = find("missing");
    println!("  unwrap_or_default()   {:?}", c.as_ref().map(|c| c.retries).unwrap_or_default());
    println!("  map + unwrap_or       {}", find("known").map(|c| c.retries).unwrap_or(1));
    println!("  the ? operator        propagates the None to the caller");
    println!("  Each one is a decision about what 'nothing' means HERE, written");
    println!("  where it is made -- which is the thing a null check never says.");

    assert_eq!(size_of::<&Config>(), size_of::<Option<&Config>>());
    assert!(find("missing").is_none());
}
