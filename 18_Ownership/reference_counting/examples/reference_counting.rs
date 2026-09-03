//! `Rc`: the clone that copies a pointer.
//! Every cost below is a counted allocation, not an inferred one.
//!
//!   rustc --edition 2024 reference_counting.rs -o /tmp/rc && /tmp/rc

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Wraps the system allocator and counts what passes through it. Same shape as
/// the one on the global-allocator page; it allocates nothing itself.
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
/// measured region: `println!` allocates, and a counter that includes its own
/// reporting is the first way this measurement goes wrong.
fn measure<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let before = ALLOCS.load(Relaxed);
    let out = work();
    let after = ALLOCS.load(Relaxed);
    println!("   {label:<34} alloc {}", after - before);
    out
}

/// The back edge is the only thing that differs between a leak and a clean
/// teardown, so it is the only thing this enum makes a choice about.
enum Back {
    /// An owner. Keeps the target alive, and in a loop that is the leak.
    Strong(Rc<Node>),
    /// An observer. Sees the target while somebody else owns it.
    Weak(Weak<Node>),
}

struct Node {
    name: &'static str,
    next: RefCell<Option<Rc<Node>>>,
    back: RefCell<Option<Back>>,
}

impl Node {
    fn new(name: &'static str) -> Rc<Node> {
        Rc::new(Node { name, next: RefCell::new(None), back: RefCell::new(None) })
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("     drop ran for {}", self.name);
    }
}

/// Read the back edge. The weak arm has to be `upgrade()`d before it can be
/// used at all, which is the API saying out loud that the target may be gone.
fn back_edge(node: &Rc<Node>) -> String {
    match &*node.back.borrow() {
        Some(Back::Strong(target)) => format!("strong -> {}, an owner", target.name),
        Some(Back::Weak(w)) => match w.upgrade() {
            Some(target) => format!("weak -> {}, upgrade() gave Some", target.name),
            None => "weak -> upgrade() gave None; the target is already freed".to_string(),
        },
        None => "none".to_string(),
    }
}

/// Build `a -> b` with a strong forward edge, and `b -> a` with whichever kind
/// of back edge the caller chose.
fn two_nodes(strong_back: bool) {
    let a = Node::new("a");
    let b = Node::new("b");
    let back = if strong_back {
        Back::Strong(Rc::clone(&a))
    } else {
        Back::Weak(Rc::downgrade(&a))
    };
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.back.borrow_mut() = Some(back);
    println!("     b's back edge: {}", back_edge(&b));
    println!(
        "     while alive: strong a {}  b {}   weak a {}",
        Rc::strong_count(&a),
        Rc::strong_count(&b),
        Rc::weak_count(&a)
    );
    println!("     leaving the scope, so both locals are about to drop:");
}

fn main() {
    println!("1. The count is the whole mechanism");
    let roster = Rc::new(vec!["Ada".to_string(), "Ben".to_string(), "Cara".to_string()]);
    println!("   after Rc::new                      {}", Rc::strong_count(&roster));
    let second = Rc::clone(&roster);
    println!("   after Rc::clone                    {}", Rc::strong_count(&roster));
    {
        let _third = Rc::clone(&roster);
        println!("   a third owner, inside a block      {}", Rc::strong_count(&roster));
    }
    println!("   the block ended, so its owner left {}", Rc::strong_count(&roster));
    println!("   The value is freed at zero, not at the end of any one owner's scope.");

    println!();
    println!("2. What the clone copied: nothing");
    let deep: Vec<String> = measure("(*roster).clone()", || (*roster).clone());
    let _shared = measure("Rc::clone(&roster)", || Rc::clone(&roster));
    println!("   same allocation as the original?");
    println!("     Rc::clone   {}", Rc::ptr_eq(&roster, &second));
    println!("     deep clone  {}   <- a separate Vec, and separate Strings inside it",
             roster.as_ptr() == deep.as_ptr());
    println!("   One Vec buffer plus one buffer per String, against nothing at all.");

    println!();
    println!("3. `Rc::clone(&x)` and `x.clone()` are the same call");
    let name: Rc<String> = Rc::new("Ada".to_string());
    let by_method = name.clone();
    println!("   Rc<String>, cloned with .clone():");
    println!("     strong_count {}   same String? {}",
             Rc::strong_count(&name), Rc::ptr_eq(&name, &by_method));
    println!("   The String was NOT cloned. `Rc<T>` implements `Clone` itself, so");
    println!("   method resolution finds it first and never reaches `String::clone`.");
    println!("   Both spellings compile to that. Only one of them says so on the line.");
    let inner: String = (*name).clone();
    println!("   To reach the String you have to ask: (*name).clone() = {inner:?}");

    println!();
    println!("4. An `Rc` hands out `&T`, so uniqueness is what buys a write");
    let mut solo = Rc::new(vec![1, 2, 3]);
    match Rc::get_mut(&mut solo) {
        Some(v) => {
            v.push(4);
            println!("   strong_count 1 -> get_mut gave a &mut, pushed: {solo:?}");
        }
        None => println!("   unreachable here"),
    }
    let _rival = Rc::clone(&solo);
    println!("   strong_count {} -> get_mut is {:?}",
             Rc::strong_count(&solo),
             Rc::get_mut(&mut solo).map(|_| "Some(&mut)"));
    println!("   Shared and mutable is the pair Rust never hands out for free.");

    println!();
    println!("5. `Rc<RefCell<T>>` is how a shared value gets written to");
    let shared = Rc::new(RefCell::new(0u32));
    let counter_a = Rc::clone(&shared);
    let counter_b = Rc::clone(&shared);
    *counter_a.borrow_mut() += 5;
    *counter_b.borrow_mut() += 3;
    println!("   two owners wrote through the same cell: {}", shared.borrow());
    println!("   `Rc` grants the ownership; `RefCell` grants the write. Neither");
    println!("   substitutes for the other, and the borrow rule moves to run time.");

    println!();
    println!("6. The one leak safe Rust still permits");
    println!("   a <-> b, back edge STRONG:");
    two_nodes(true);
    println!("     ...nothing printed. Each still holds the other at 1, so neither");
    println!("     reaches zero. Both nodes are unreachable and never freed.");
    println!("   a <-> b, back edge WEAK:");
    two_nodes(false);
    println!("     Both ran. A `Weak` does not own, so the loop never closes.");
    println!("   `Rc::downgrade` makes one; `upgrade()` returns an Option, because");
    println!("   the value it points at may already be gone.");
}
