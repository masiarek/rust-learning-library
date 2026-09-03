//! The owned strings that are not `String`, and what each one costs.
//!
//! Every comparison here is printed as a number or a yes/no, never as an
//! address: addresses move on every run, and this file's output is an answer
//! key that CI diffs.
//!
//!   rustc --edition 2024 boxed_str.rs -o /tmp/bs && /tmp/bs

use std::mem::size_of;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    println!("1. Four owned strings, four handle sizes");
    println!("   String      {:>3} bytes  (pointer + length + capacity)", size_of::<String>());
    println!("   Box<str>    {:>3} bytes  (pointer + length)", size_of::<Box<str>>());
    println!("   Rc<str>     {:>3} bytes  (pointer + length)", size_of::<Rc<str>>());
    println!("   Arc<str>    {:>3} bytes  (pointer + length)", size_of::<Arc<str>>());
    println!("   &str        {:>3} bytes  (pointer + length) — borrowed, owns nothing", size_of::<&str>());
    println!("   Drop the capacity word and the handle is a &str's size — but it OWNS.");

    println!("\n2. The round trip, and what the capacity word was holding");
    let mut s = String::with_capacity(64);
    s.push_str("Ada Lovelace");
    println!("   String::with_capacity(64) + push_str  len {} cap {}", s.len(), s.capacity());
    let boxed: Box<str> = s.into_boxed_str();
    println!("   .into_boxed_str()                     len {}  (no capacity field at all)", boxed.len());
    let back: String = boxed.into_string();
    println!("   .into_string()                        len {} cap {}", back.len(), back.capacity());
    println!("   52 bytes of slack are gone: into_boxed_str shrinks first, so the");
    println!("   trip back cannot restore a capacity nobody recorded.");

    println!("\n3. When the 8 bytes matter");
    for n in [1_000usize, 1_000_000] {
        let as_string = n * size_of::<String>();
        let as_boxed = n * size_of::<Box<str>>();
        println!(
            "   {n:>9} values:  Vec<String> {:>8} KB   Vec<Box<str>> {:>8} KB   saved {} KB",
            as_string / 1024,
            as_boxed / 1024,
            (as_string - as_boxed) / 1024
        );
    }
    println!("   Count the values, not the bytes. At a thousand it is noise; at a");
    println!("   million it is eight million bytes you did not have to touch. And it");
    println!("   is a saving on the HANDLES only — the text is the same either way.");

    println!("\n4. Rc<str> is one hop; Rc<String> is two");
    let flat: Rc<str> = Rc::from("Ada Lovelace");
    let nested: Rc<String> = Rc::new(String::from("Ada Lovelace"));
    let flat_inline = Rc::as_ptr(&flat) as *const u8 == flat.as_ptr();
    let nested_inline = Rc::as_ptr(&nested) as *const u8 == nested.as_ptr();
    println!("   Rc<str>     handle {:>2} bytes   text stored inside the Rc? {}", size_of::<Rc<str>>(), flat_inline);
    println!("   Rc<String>  handle {:>2} bytes   text stored inside the Rc? {}", size_of::<Rc<String>>(), nested_inline);
    println!("   Rc<String> has the smaller handle and costs MORE: the Rc holds a");
    println!("   String, and that String points somewhere else again — two heap");
    println!("   allocations per value, and two dereferences to reach a byte.");

    println!("\n5. Many owners, one buffer");
    let name: Rc<str> = Rc::from("Ada Lovelace");
    let table: Vec<Rc<str>> = (0..5).map(|_| Rc::clone(&name)).collect();
    let shared = table.iter().all(|h| h.as_ptr() == name.as_ptr());
    println!("   5 clones of one Rc<str>: all pointing at the same buffer? {shared}");
    println!("   strong_count = {}  (the 5 clones plus the original)", Rc::strong_count(&name));
    println!("   text allocated once: {} bytes", name.len());

    let copies: Vec<String> = (0..5).map(|_| String::from("Ada Lovelace")).collect();
    let distinct = copies
        .iter()
        .map(|s| s.as_ptr() as usize)
        .collect::<std::collections::HashSet<_>>()
        .len();
    println!("   the same table as Vec<String>: {distinct} separate buffers, {} bytes of text",
        copies.iter().map(|s| s.len()).sum::<usize>());

    println!("\n6. The trap: .to_owned() on an Rc clones the POINTER");
    let again = name.to_owned(); // type is Rc<str>, NOT String
    println!("   name.to_owned() points at the same buffer? {}", again.as_ptr() == name.as_ptr());
    println!("   strong_count is now {} — a new owner, no new text", Rc::strong_count(&name));
    let view: &str = &name;
    let real_copy: String = view.to_owned(); // type is String
    println!("   (&*name).to_owned() points at the same buffer? {}", real_copy.as_ptr() == name.as_ptr());
    println!("   Same method name, two different jobs. On the smart pointer it is a");
    println!("   cheap new handle; one deref away it is an allocation and a memcpy.");
}
