//! What a `.clone()` costs, counted rather than guessed.
//!
//!   rustc --edition 2024 what_a_clone_costs.rs -o /tmp/wacc && /tmp/wacc

use std::alloc::{GlobalAlloc, Layout, System};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Counts trips to the allocator and lets `System` do the work. The pattern,
/// and the rules that keep it honest, are on the global allocator page. No
/// `realloc` override: nothing measured here grows a buffer.
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

/// Run `work` and report what IT allocated. The report is printed after the
/// measured region closes, because `println!` allocates too.
fn count<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let before = ALLOCS.load(Relaxed);
    let out = work();
    let n = ALLOCS.load(Relaxed) - before;
    println!("   {label:<50} alloc {n}");
    out
}

#[derive(Clone)]
struct Person {
    id: u64,
    name: String,
    email: String,
}

#[derive(Clone)]
struct SharedPerson {
    id: u64,
    name: Arc<str>,
    email: Arc<str>,
}

fn main() {
    // This first line also warms up stdout, so its one-time setup is not
    // charged to the first measurement.
    println!("1. What a clone costs is decided by the type");
    let n: u64 = 7;
    let text = String::from("alice");
    let boxed = Box::new(7_u64);
    let colours: Vec<String> = vec!["red".into(), "green".into(), "blue".into()];
    let rc: Rc<str> = Rc::from("alice");
    let arc: Arc<str> = Arc::from("alice");
    count("u64           the bits, copied", || n.clone());
    count("String        a new buffer, the bytes copied in", || text.clone());
    count("Box<u64>      a new box", || boxed.clone());
    count("Vec<String>   the buffer, then each of 3 Strings", || colours.clone());
    count("Rc<str>       one count goes up", || Rc::clone(&rc));
    count("Arc<str>      one count goes up, atomically", || Arc::clone(&arc));
    println!("   One trait, one method name, and anything from nothing to 1 + n.");

    println!();
    println!("2. A derived Clone clones every field, so add up the fields");
    let a = Person { id: 1, name: "alice".into(), email: "alice@example.com".into() };
    let b = count("Person { u64, String, String }.clone()", || a.clone());
    println!("   equal text: {}   same buffer: {}", a.name == b.name, a.name.as_ptr() == b.name.as_ptr());
    let s = SharedPerson { id: 1, name: Arc::from("alice"), email: Arc::from("alice@example.com") };
    let t = count("SharedPerson { u64, Arc<str>, Arc<str> }.clone()", || s.clone());
    println!("   equal text: {}   same buffer: {}", s.name == t.name, Arc::ptr_eq(&s.name, &t.name));
    println!("   0 + 1 + 1 against 0 + 0 + 0, and both lines read `.clone()`.");
    let owned = String::from("alice");
    let _converted = count("Arc::<str>::from(a String you already have)", move || Arc::<str>::from(owned));
    println!("   Switching costs one copy, once: an Arc keeps its counts in front");
    println!("   of the text, so the String's buffer cannot be adopted as it is.");

    println!();
    println!("3. Change one field, keep the original: three spellings");
    let (n1, n2, n3) = (String::from("bob"), String::from("bob"), String::from("bob"));
    let u1 = count("Person { name, ..a.clone() }", || Person { name: n1, ..a.clone() });
    let u2 = count("Person { id: a.id, name, email: a.email.clone() }", || Person {
        id: a.id,
        name: n2,
        email: a.email.clone(),
    });
    let spare = a.clone();
    let u3 = count("Person { name, ..spare }   (spare is used up)", move || Person { name: n3, ..spare });
    assert!(u1.name == u2.name && u2.name == u3.name && u1.email == u3.email);
    println!("   All three hold {} <{}>.", u3.name, u3.email);
    println!("   `..a.clone()` clones a.name as well, and drops it unread.");
    println!("   Naming the kept fields clones only those. Moving clones nothing.");
    let bob: Arc<str> = Arc::from("bob");
    let v = count("SharedPerson { name, ..s.clone() }", || SharedPerson { name: Arc::clone(&bob), ..s.clone() });
    println!("   With Arc fields the careless spelling is free too: #{} {} <{}>", v.id, v.name, v.email);

    println!();
    println!("4. A collection multiplies it");
    let tags: Vec<String> = (0..1_000).map(|i| format!("tag-{i}")).collect();
    let copied = count("Vec<String> of 1,000: .clone()", || tags.clone());
    let shared: Arc<[String]> = tags.into();
    let handle = count("Arc<[String]> of 1,000: Arc::clone", || Arc::clone(&shared));
    assert_eq!(copied.len(), handle.len());
    println!("   {} tags either way: one buffer per String plus one for the Vec,", copied.len());
    println!("   against a single increment.");
    let mut list: Arc<Vec<String>> = Arc::new(colours);
    let other = Arc::clone(&list);
    count("Arc::make_mut, another handle alive", || {
        Arc::make_mut(&mut list);
    });
    count("Arc::make_mut, now the only handle", || {
        Arc::make_mut(&mut list);
    });
    println!("   Clone on write: the first write while shared pays for all of it,");
    println!("   1 for the Arc + 1 for the Vec + 1 per String. The copy is unique");
    println!("   after that, so the next write pays nothing. `other` kept {} items.", other.len());
}
