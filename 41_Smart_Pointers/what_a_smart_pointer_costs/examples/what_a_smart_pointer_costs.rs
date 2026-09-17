//! What a smart pointer costs at run time, counted by a global allocator that
//! tallies every call — and the part that really is free.
//!
//!   rustc --edition 2024 what_a_smart_pointer_costs.rs -o /tmp/wspc && /tmp/wspc

use std::alloc::{GlobalAlloc, Layout, System};
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::mem::size_of;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static FREES: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

/// Passes every request to `System` and counts it on the way.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        FREES.fetch_add(1, Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Runs `work` and prints only what it cost. Nothing is printed inside the
/// measured region, because `println!` can allocate too.
fn cost<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let (a, f, b) = (ALLOCS.load(Relaxed), FREES.load(Relaxed), BYTES.load(Relaxed));
    let out = work();
    let (a2, f2, b2) = (ALLOCS.load(Relaxed), FREES.load(Relaxed), BYTES.load(Relaxed));
    println!("   {label:<34} alloc {}  free {}  bytes {:>2}", a2 - a, f2 - f, b2 - b);
    out
}

fn main() {
    println!("1. The pointer itself: one word, and the niche is kept");
    for (ty, bytes) in [
        ("*const u64", size_of::<*const u64>()),
        ("Box<u64>", size_of::<Box<u64>>()),
        ("Rc<u64>", size_of::<Rc<u64>>()),
        ("Arc<u64>", size_of::<Arc<u64>>()),
        ("Weak<u64>", size_of::<Weak<u64>>()),
        ("Option<Rc<u64>>", size_of::<Option<Rc<u64>>>()),
    ] {
        println!("   {ty:<18} {bytes} bytes");
    }
    println!("   This is the zero-cost part: the handle is no bigger than the raw");
    println!("   pointer you would have kept by hand.");

    println!();
    println!("2. Making one: an allocation, sized by what is stored beside the value");
    let _boxed = cost("Box::new(7u64)", || Box::new(7u64));
    let shared = cost("Rc::new(7u64)", || Rc::new(7u64));
    let _atomic = cost("Arc::new(7u64)", || Arc::new(7u64));
    let _text = cost("String::from(\"hello\")", || String::from("hello"));
    let _nums = cost("Vec::<u32>::with_capacity(4)", || Vec::<u32>::with_capacity(4));
    let _cow: Cow<str> = cost("Cow::Borrowed(\"hello\")", || Cow::Borrowed("hello"));
    let _cell = cost("RefCell::new(5i32)", || RefCell::new(5i32));
    println!("   Box asks for the u64 alone. Rc and Arc ask for 24: a strong count,");
    println!("   a weak count, then the u64. The two that allocate nothing are the");
    println!("   two that store nothing on the heap.");

    println!();
    println!("3. Sharing one: no allocation, but a count changes at run time");
    let second = cost("Rc::clone(&shared)", || Rc::clone(&shared));
    let weak = cost("Rc::downgrade(&shared)", || Rc::downgrade(&shared));
    println!("   strong_count {}   weak_count {}", Rc::strong_count(&shared), Rc::weak_count(&shared));

    println!();
    println!("4. Letting go: only the last strong owner drops the value");
    cost("drop(second)", || drop(second));
    println!("   strong_count {}", Rc::strong_count(&shared));
    cost("drop(shared), the last Rc", || drop(shared));
    println!("   weak.upgrade() {:?}", weak.upgrade());
    cost("drop(weak), the last Weak", || drop(weak));
    println!("   The value was dropped with the last Rc, but its 24 bytes were freed");
    println!("   only with the last Weak, because the weak count lives there too.");

    println!();
    println!("5. RefCell: a flag in the value, and a check on every borrow");
    println!("   size_of i32 {}   Cell<i32> {}   RefCell<i32> {}",
        size_of::<i32>(), size_of::<Cell<i32>>(), size_of::<RefCell<i32>>());
    let cell = RefCell::new(5);
    let reading = cell.borrow();
    println!("   borrow_mut while a borrow is live: {:?}", cell.try_borrow_mut().map(|_| ()));
    drop(reading);
    println!("   after the borrow ends:             {:?}", cell.try_borrow_mut().map(|_| ()));
    println!("   Cell hands out no references, so it needs no flag. RefCell counts");
    println!("   its borrows in a word beside the value and checks it at run time.");
}
