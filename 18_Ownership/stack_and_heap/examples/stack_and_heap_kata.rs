//! Kata solution: predict six sizes, then predict which of seven lines allocates.
//!
//!   rustc --edition 2024 stack_and_heap_kata.rs -o /tmp/sahk && /tmp/sahk
//!
//! Nothing here prints an address. A raw pointer differs every run and would
//! poison the recorded key, so every claim about "the same buffer" is a
//! comparison, printed as the boolean it produces.

use std::alloc::{GlobalAlloc, Layout, System};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Run `work` and report only what it cost. Nothing is printed inside the
/// measured region: `println!` allocates.
fn measure<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let before = ALLOCS.load(Relaxed);
    let out = work();
    let after = ALLOCS.load(Relaxed);
    println!("    {label:<32} alloc {}", after - before);
    out
}

#[derive(Clone, Copy, Debug)]
struct Precinct {
    id: u32,
    registered: u32,
}

fn main() {
    println!("Part 1 — predict the stack size of six types.\n");
    println!("    {:<14} {:>4}  {}", "type", "size", "why");
    println!("    {:<14} {:>4}  {}", "i32", size_of::<i32>(), "the slot IS the value");
    println!("    {:<14} {:>4}  {}", "[i32; 5]", size_of::<[i32; 5]>(), "five of them, still no header");
    println!("    {:<14} {:>4}  {}", "Precinct", size_of::<Precinct>(), "two u32 fields, nothing more");
    println!("    {:<14} {:>4}  {}", "&str", size_of::<&str>(), "pointer + length: a view needs both");
    println!("    {:<14} {:>4}  {}", "Box<i32>", size_of::<Box<i32>>(), "one pointer, the i32 is elsewhere");
    println!("    {:<14} {:>4}  {}", "String", size_of::<String>(), "pointer + length + capacity");
    println!("    {:<14} {:>4}  {}", "Vec<i32>", size_of::<Vec<i32>>(), "same three words, any length");

    println!("\n  The pair to compare is `[i32; 5]` at 20 and `Vec<i32>` at 24.");
    println!("  The array's 20 bytes ARE its five integers. The Vec's 24 are a");
    println!("  header and none of its data, so a Vec of a million elements is");
    println!("  still 24 and a `[i32; 1_000_000]` would be four megabytes of stack.");

    println!("\nPart 2 — what size_of_val can and cannot see.\n");
    let mut roomy = String::with_capacity(64);
    roomy.push_str("abc");
    println!("    size_of_val(&roomy)   {}   the header, again", size_of_val(&roomy));
    println!("    size_of_val(&*roomy)  {}    the str behind it: live length", size_of_val(&*roomy));
    println!("    roomy.capacity()      {}   bought, and invisible to both", roomy.capacity());
    println!("\n  Neither number is what the value is holding. 61 bytes are reserved,");
    println!("  untouched, and reported by nothing in std. Counting them takes an");
    println!("  allocator you can instrument, which is what part 3 uses.");

    println!("\nPart 3 — predict which of seven lines allocates, then count.\n");

    let source = String::from("Riverside precinct");
    let before_move = source.as_ptr();
    let moved = measure("let moved = source;", || source);
    println!("      ...and the buffer did not move: {}", before_move == moved.as_ptr());

    let precinct = Precinct { id: 7, registered: 431 };
    let copied = measure("let copied = precinct;", || precinct);
    println!("      ...a Copy type has no heap side to touch, and both names");
    println!("         are still alive: precinct {} / copied {} registered",
             precinct.id, copied.registered);

    let _view: &str = measure("let view = &moved[..];", || &moved[..]);
    println!("      ...a borrow is a pointer and a length, both on the stack");

    let cloned = measure("moved.clone()", || moved.clone());
    println!("      ...the one row that buys a second buffer: {}",
             moved.as_ptr() != cloned.as_ptr());

    let boxed = measure("Box::new(41u32)", || Box::new(41u32));

    let shared = measure("Rc::new(cloned)", || Rc::new(cloned));
    let _second = measure("Rc::clone(&shared)", || Rc::clone(&shared));
    println!("      ...one owner more, one counter up, no bytes copied: {}",
             Rc::strong_count(&shared));

    println!("\n  Four of the seven allocate nothing. The three that do are the");
    println!("  three that put something NEW on the heap -- a clone's buffer, a");
    println!("  Box's payload, an Rc's counted allocation. Everything else moves");
    println!("  or copies a header between stack slots, which is why Rust can");
    println!("  afford to move by default however long the text is.");
    println!("\n  boxed still says {boxed}, and the rule to carry away is that");
    println!("  `.clone()` names the only one you have to look at the TYPE to");
    println!("  price: on a String it allocates, on an Rc it does not.");
}
