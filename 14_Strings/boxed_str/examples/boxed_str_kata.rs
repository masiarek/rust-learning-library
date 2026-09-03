//! Kata solution: freeze a candidate column three ways, and count what changed.
//!
//!   rustc --edition 2024 boxed_str_kata.rs -o /tmp/bsk && /tmp/bsk

use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;

/// Eight ballot rows naming three candidates — the shape that makes interning pay.
const COLUMN: [&str; 8] = [
    "Ada Lovelace",
    "Grace Hopper",
    "Ada Lovelace",
    "Barbara Liskov",
    "Ada Lovelace",
    "Grace Hopper",
    "Barbara Liskov",
    "Ada Lovelace",
];

/// How many distinct text buffers are alive behind these handles?
fn distinct_buffers(ptrs: impl Iterator<Item = *const u8>) -> usize {
    ptrs.map(|p| p as usize).collect::<std::collections::HashSet<_>>().len()
}

fn main() {
    println!("The column: {} rows, {} distinct names",
        COLUMN.len(),
        COLUMN.iter().collect::<std::collections::HashSet<_>>().len());

    println!("\n1. Vec<String> — grow-able, and nobody is going to grow it");
    let as_strings: Vec<String> = COLUMN.iter().map(|s| s.to_string()).collect();
    let handles = as_strings.len() * size_of::<String>();
    println!("   handles      {:>3} x {:>2} = {:>3} bytes", as_strings.len(), size_of::<String>(), handles);
    println!("   text buffers {:>3}", distinct_buffers(as_strings.iter().map(|s| s.as_ptr())));

    println!("\n2. Vec<Box<str>> — same text, one word less per handle");
    let as_boxed: Vec<Box<str>> = as_strings.into_iter().map(|s| s.into_boxed_str()).collect();
    let boxed_handles = as_boxed.len() * size_of::<Box<str>>();
    println!("   handles      {:>3} x {:>2} = {:>3} bytes   ({} saved)",
        as_boxed.len(), size_of::<Box<str>>(), boxed_handles, handles - boxed_handles);
    println!("   text buffers {:>3}   <- unchanged: freezing a handle copies no text",
        distinct_buffers(as_boxed.iter().map(|b| b.as_ptr())));

    println!("\n3. Vec<Rc<str>> — intern, and the repeats stop paying");
    let mut pool: HashMap<&str, Rc<str>> = HashMap::new();
    let as_rc: Vec<Rc<str>> = COLUMN
        .iter()
        .map(|&name| Rc::clone(pool.entry(name).or_insert_with(|| Rc::from(name))))
        .collect();
    println!("   handles      {:>3} x {:>2} = {:>3} bytes",
        as_rc.len(), size_of::<Rc<str>>(), as_rc.len() * size_of::<Rc<str>>());
    println!("   text buffers {:>3}   <- three names, three buffers, eight rows",
        distinct_buffers(as_rc.iter().map(|r| r.as_ptr())));
    let ada = &pool["Ada Lovelace"];
    println!("   \"Ada Lovelace\" appears 4 times; strong_count = {}", Rc::strong_count(ada));
    println!("   (4 rows + 1 held by the pool)");

    println!("\n4. The trap — same method name, two different jobs");
    let row: &Rc<str> = &as_rc[0];
    let cheap: Rc<str> = row.to_owned();           // a new handle
    let view: &str = row;
    let real: String = view.to_owned();            // an allocation and a memcpy
    println!("   Rc<str>.to_owned()  same buffer? {}   <- clones the POINTER", cheap.as_ptr() == row.as_ptr());
    println!("   (&str).to_owned()   same buffer? {}  <- clones the TEXT", real.as_ptr() == row.as_ptr());

    println!("\nWhat to reach for:");
    println!("   still being built            -> String");
    println!("   finished, stored in bulk     -> Box<str>   (8 bytes/value, no copy to make)");
    println!("   finished, and it REPEATS     -> Rc<str>    (one buffer per distinct value)");
    println!("   finished, and crosses threads-> Arc<str>");
    println!("   The first choice is about growth; the rest are about how many you keep.");
}
