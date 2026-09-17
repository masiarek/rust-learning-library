//! What makes a pointer smart: `Deref` so it acts like one, `Drop` so it does
//! something when it goes — and, usually but not always, owning what it points at.
//!
//!   rustc --edition 2024 what_makes_a_pointer_smart.rs -o /tmp/wmps && /tmp/wmps

use std::any::type_name;
use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;

/// A smart pointer in twenty lines: it owns a value on the heap, derefs to it,
/// and says so when it lets go.
struct Loud<T> {
    name: &'static str,
    value: Box<T>,
}

impl<T> Loud<T> {
    fn new(name: &'static str, value: T) -> Self {
        Loud { name, value: Box::new(value) }
    }
}

impl<T> Deref for Loud<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Loud<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Drop for Loud<T> {
    fn drop(&mut self) {
        println!("   drop: {} lets go of its value", self.name);
    }
}

fn shout(text: &str) -> String {
    text.to_uppercase()
}

/// The `Target` a `Deref` impl names, read off the type.
fn target<P: Deref>(_: &P) -> &'static str {
    type_name::<P::Target>()
}

fn main() {
    println!("1. Deref: a struct that acts like a pointer");
    let mut greeting = Loud::new("greeting", String::from("hello"));
    println!("   (*greeting).len()         {}   explicit: *greeting is the String", (*greeting).len());
    println!("   greeting.len()            {}   the method search derefs for you", greeting.len());
    println!("   greeting.to_uppercase()   {}   a str method, two derefs away", greeting.to_uppercase());
    println!("   shout(&greeting)          {}   &Loud<String> coerced to &str at the call", shout(&greeting));
    greeting.push_str(", world"); // DerefMut: &mut Loud<String> -> &mut String
    println!("   after push_str via DerefMut: {:?}", *greeting);

    println!();
    println!("2. Drop: something happens when it goes");
    {
        let temp = Loud::new("temp", 41);
        println!("   *temp + 1 = {}", *temp + 1);
        println!("   end of block:");
    }
    println!("   drop(greeting) before the end of main:");
    drop(greeting);

    println!();
    println!("3. std's smart pointers, and what each derefs to");
    let text = String::from("hi");
    let bytes = vec![1u8];
    let boxed = Box::new(1u8);
    let counted = Rc::new(1u64);
    let cow: Cow<str> = Cow::Borrowed("hi");
    let cell = RefCell::new(5i32);
    let guard: Ref<i32> = cell.borrow();
    for (ty, t) in [
        ("String", target(&text)),
        ("Vec<u8>", target(&bytes)),
        ("Box<u8>", target(&boxed)),
        ("Rc<u64>", target(&counted)),
        ("Cow<str>", target(&cow)),
        ("Ref<i32>", target(&guard)),
    ] {
        println!("   {ty:<10} -> {t}");
    }

    println!();
    println!("4. \"Owns what it points at\" is usually true, and not always");
    let borrowed: Cow<str> = Cow::Borrowed(&text);
    println!("   Cow::Borrowed points into `text`, which it does not own: {}",
        ptr::eq(borrowed.as_ptr(), text.as_ptr()));
    println!("   while the Ref guard lives, try_borrow_mut is_err: {}", cell.try_borrow_mut().is_err());
    drop(guard);
    println!("   after drop(guard),         try_borrow_mut is_ok:  {}", cell.try_borrow_mut().is_ok());
    println!("   The guard never owned the 5. Its Drop gives the borrow back.");

    println!();
    println!("5. Weak: a pointer to shared data with no Deref at all");
    let strong = Rc::new(7);
    let weak = Rc::downgrade(&strong);
    println!("   weak.upgrade()                    {:?}", weak.upgrade());
    drop(strong);
    println!("   after the last Rc is dropped      {:?}", weak.upgrade());
    println!("   `*weak` does not compile. You ask for an Rc, and may get None.");
}
