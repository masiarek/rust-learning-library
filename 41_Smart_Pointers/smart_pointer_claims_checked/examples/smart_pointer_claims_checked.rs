//! Claims about smart pointers from notes, articles and books, each run.
//!
//!   rustc --edition 2024 smart_pointer_claims_checked.rs -o /tmp/spcc && /tmp/spcc

use std::any::type_name;
use std::borrow::Cow;
use std::cell::RefCell;
use std::mem::{needs_drop, size_of};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn deref_target<P: Deref>(_: &P) -> &'static str {
    type_name::<P::Target>()
}

/// Compiles only for a type that holds no borrowed reference.
fn holds_no_reference<T: 'static>() -> bool {
    true
}

fn main() {
    println!("1. \"A compile-time construct the compiler boils away\"");
    let shared = Rc::new(String::from("state"));
    let before = Rc::strong_count(&shared);
    let other = Rc::clone(&shared);
    println!("   strong_count before clone {before}, after {}", Rc::strong_count(&other));
    let cell = RefCell::new(0);
    let guard = cell.borrow();
    println!("   RefCell borrow_mut while borrowed: {:?}", cell.try_borrow_mut().map(|_| ()));
    drop(guard);
    println!("   A count that changes and a borrow refused at run time are state the");
    println!("   program carries. The allocations are counted on the costs page.");

    println!();
    println!("2. \"String and Vec<T> are smart pointers\"");
    let text = String::from("hi");
    let list = vec![1u8, 2];
    println!("   String derefs to {}, Vec<u8> to {}", deref_target(&text), deref_target(&list));
    println!("   needs_drop: String {}, Vec<u8> {}", needs_drop::<String>(), needs_drop::<Vec<u8>>());
    println!("   String: 'static {}, Vec<u8>: 'static {}",
        holds_no_reference::<String>(), holds_no_reference::<Vec<u8>>());
    println!("   Deref and Drop: smart pointers in the sense std's Deref docs use.");
    println!("   Neither has a lifetime parameter, so the buffer is not held by a");
    println!("   reference: std stores a raw pointer (a NonNull) and frees it in Drop.");

    println!();
    println!("3. \"A smart pointer owns the data it refers to\"");
    let borrowed: Cow<str> = Cow::Borrowed(&text);
    println!("   Cow::Borrowed(&text) is Borrowed: {}", matches!(borrowed, Cow::Borrowed(_)));
    let strong = Rc::new(1);
    let weak = Rc::downgrade(&strong);
    drop(strong);
    println!("   a Weak outliving its value: upgrade() = {:?}", weak.upgrade());
    println!("   Cow::Borrowed owns nothing and a Weak keeps nothing alive; The Book");
    println!("   says smart pointers own their data \"in many cases\".");

    println!();
    println!("4. \"Some raw pointers, such as Rc<T>, offer shared ownership\"");
    let counted = Rc::new(5u8);
    println!("   Rc::as_ptr(&counted) is a {}", type_name_of(Rc::as_ptr(&counted)));
    println!("   needs_drop: Rc<u8> {}, *const u8 {}", needs_drop::<Rc<u8>>(), needs_drop::<*const u8>());
    println!("   size_of Option<Rc<u8>> {}, Option<*const u8> {}",
        size_of::<Option<Rc<u8>>>(), size_of::<Option<*const u8>>());
    println!("   Rc is a smart pointer built on a NonNull. A raw pointer is what");
    println!("   Rc::as_ptr hands you: no count, no drop, and nullable.");

    println!();
    println!("5. \"Use a smart pointer in place of its referent without fuss\"");
    let boxed = Box::new(41i32);
    println!("   boxed.checked_add(1)  {:?}   a method: found through Deref", boxed.checked_add(1));
    println!("   *boxed + 1            {}         an operator: needs the *", *boxed + 1);
    println!("   `boxed + 1` is E0369. Method calls and coercion sites deref for you;");
    println!("   operators and patterns do not.");

    println!();
    println!("6. Rc is \"single-threaded\"; Arc is the one that crosses");
    let across = Arc::new(String::from("sent"));
    let handle = {
        let mine = Arc::clone(&across);
        thread::spawn(move || mine.len())
    };
    println!("   thread read Arc<String>: len {}", handle.join().unwrap());
    println!("   strong_count after join: {}", Arc::strong_count(&across));
    println!("   The same spawn with an Rc is E0277: Rc<String> cannot be sent");
    println!("   between threads safely.");
}

fn type_name_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
