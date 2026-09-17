//! A pointer is one word, or two — and the second word is a length or a vtable.
//!
//!   rustc --edition 2024 wide_pointers.rs -o /tmp/wp && /tmp/wp

use std::fmt::Debug;
use std::mem::{size_of, size_of_val};
use std::rc::Rc;

static NAME: [u8; 10] = *b"carrytowel";

fn words<T: ?Sized>() -> usize {
    size_of::<&T>() / size_of::<usize>()
}

fn main() {
    println!("1. Thin: the address and nothing else");
    for (ty, bytes) in [
        ("&u8", size_of::<&u8>()),
        ("&[u8; 10]", size_of::<&[u8; 10]>()),
        ("&String", size_of::<&String>()),
        ("*const u8", size_of::<*const u8>()),
        ("Box<u64>", size_of::<Box<u64>>()),
    ] {
        println!("   {ty:<16} {bytes:>2} bytes");
    }
    println!("   `&[u8; 10]` is thin: the 10 is part of the TYPE, so the pointer");
    println!("   has nothing more to carry.");

    println!();
    println!("2. Wide, second word a length");
    for (ty, bytes) in [
        ("&[u8]", size_of::<&[u8]>()),
        ("&str", size_of::<&str>()),
        ("*const [u8]", size_of::<*const [u8]>()),
        ("Box<[u8]>", size_of::<Box<[u8]>>()),
        ("Rc<str>", size_of::<Rc<str>>()),
    ] {
        println!("   {ty:<16} {bytes:>2} bytes");
    }
    let whole: &[u8; 10] = &NAME;
    let slice: &[u8] = &NAME[2..7];
    let raw: *const [u8] = slice;
    println!("   &NAME       as &[u8; 10]  {} bytes", size_of_val(&whole));
    println!("   &NAME[2..7] as &[u8]      {} bytes: the length left the type and joined the pointer",
        size_of_val(&slice));
    println!("   raw.len() = {}   read from the pointer, not from the bytes", raw.len());

    println!();
    println!("3. Wide, second word a vtable");
    let n = 7u32;
    let s = String::from("seven");
    let dn: &dyn Debug = &n;
    let ds: &dyn Debug = &s;
    println!("   &dyn Debug       {:>2} bytes, {} words", size_of::<&dyn Debug>(), words::<dyn Debug>());
    println!("   Box<dyn Debug>   {:>2} bytes", size_of::<Box<dyn Debug>>());
    println!("   size_of_val(dn) = {}   size_of_val(ds) = {}", size_of_val(dn), size_of_val(ds));
    println!("   Same type, &dyn Debug, two different sizes: the size is looked up");
    println!("   in the vtable the second word points at. It is not a length.");
    let data_half = dn as *const dyn Debug as *const ();
    println!("   first word is the address of n: {}", data_half == &n as *const u32 as *const ());

    println!();
    println!("4. Casting to a thin pointer drops the second word");
    let thin = raw as *const u8;
    println!("   raw as *const u8: {} bytes, same address: {}",
        size_of_val(&thin), thin.addr() == raw.addr());
    println!("   The length is gone. Nothing in `thin` remembers there were 5 bytes.");

    println!();
    println!("5. {{:p}} shows the second word");
    let shown = format!("{raw:p}");
    println!("   {{:p}} of *const [u8] mentions `metadata: 5`: {}", shown.contains("metadata: 5"));
    println!("   {{:p}} of *const u8 has no metadata part:     {}", !format!("{thin:p}").contains("metadata"));

    println!();
    println!("6. The niche survives the second word");
    println!("   Option<&[u8]> {:>2} bytes   Option<&dyn Debug> {:>2} bytes",
        size_of::<Option<&[u8]>>(), size_of::<Option<&dyn Debug>>());
}
