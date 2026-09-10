//! Kata solution: price an order before you run it.
//!
//!   rustc --edition 2024 what_a_clone_costs_kata.rs -o /tmp/wacck && /tmp/wacck

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::Arc;
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

/// Run `work`, then print the prediction beside what the allocator saw.
fn check<T>(label: &str, predicted: usize, work: impl FnOnce() -> T) -> T {
    let before = ALLOCS.load(Relaxed);
    let out = work();
    let actual = ALLOCS.load(Relaxed) - before;
    let verdict = if actual == predicted { "" } else { "   <- recount" };
    println!("  {label:<44} predicted {predicted:>2}   actual {actual:>2}{verdict}");
    out
}

#[derive(Clone)]
struct Order {
    id: u64,
    customer: String,
    items: Vec<String>,
}

#[derive(Clone)]
struct SharedOrder {
    id: u64,
    customer: Arc<str>,
    items: Arc<[String]>,
}

fn main() {
    println!("An order: an id, a customer, three items.\n");
    let order = Order {
        id: 7,
        customer: "Acme".to_string(),
        items: vec!["bolts".to_string(), "nuts".to_string(), "washers".to_string()],
    };

    println!("Part 1 — String and Vec<String> fields");
    let copy = check("order.clone()", 5, || order.clone());
    let globex = String::from("Globex");
    let lazy = check("Order { customer, ..order.clone() }", 5, || Order { customer: globex, ..order.clone() });
    let globex = String::from("Globex");
    let careful = check("field by field, cloning only items", 4, || Order {
        id: order.id,
        customer: globex,
        items: order.items.clone(),
    });
    println!("  1 + 1 + 3: the customer, the Vec's buffer, one per item. The lazy");
    println!("  update pays all five and drops the cloned \"{}\" unread; the", copy.customer);
    println!("  careful one skips it. Both give {} with {} items.\n", lazy.customer, careful.items.len());

    println!("Part 2 — convert once, then clone for free");
    let shared = check("SharedOrder from the Order (consumes it)", 2, move || SharedOrder {
        id: order.id,
        customer: Arc::from(order.customer),
        items: Arc::from(order.items),
    });
    let again = check("shared.clone()", 0, || shared.clone());
    let globex: Arc<str> = Arc::from("Globex");
    let renamed = check("SharedOrder { customer, ..shared.clone() }", 0, || SharedOrder {
        customer: Arc::clone(&globex),
        ..shared.clone()
    });
    println!("  The conversion is 2, not 5. Arc<str> copies the customer's bytes,");
    println!("  but Arc<[String]> MOVES the three Strings into its buffer, and each");
    println!("  keeps the heap text it already had. After that, counts only:");
    println!(
        "  #{} {} and #{} {} share one item list: {}",
        again.id,
        again.customer,
        renamed.id,
        renamed.customer,
        Arc::ptr_eq(&again.items, &renamed.items)
    );
}
