//! The hover an editor shows on `.clone()` over a `&str`, one line at a time,
//! each claim run on 1.98.0.
//!
//!   rustc --edition 2024 reading_the_clone_hover.rs -o /tmp/rtch && /tmp/rtch

use std::any::type_name_of_val;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// Deliberately not `Clone`.
struct Ticket {
    seat: u32,
}

/// The body std writes for `&T`, with the types spelled out:
/// `self` is `&&T`, `Self` is `&T`, and one `*` gets from the first to the second.
fn clone_like_std<'a, T: ?Sized>(this: &&'a T) -> &'a T {
    *this
}

fn main() {
    println!("1. impl<T: PointeeSized> Clone for &T: every T, even with no size, even not Clone");
    let text: &str = "Hello";
    let slice: &[i32] = &[1, 2, 3];
    let shown: &dyn Display = &42;
    let ticket = &Ticket { seat: 12 };
    let a = Clone::clone(&text);
    let b = Clone::clone(&slice);
    let c = Clone::clone(&shown);
    let d = Clone::clone(&ticket);
    println!("   T = str         -> {:<36} same address: {}", type_name_of_val(&a), std::ptr::eq(a, text));
    println!("   T = [i32]       -> {:<36} same address: {}", type_name_of_val(&b), std::ptr::eq(b, slice));
    println!("   T = dyn Display -> {:<36} same address: {}", type_name_of_val(&c), std::ptr::addr_eq(c, shown));
    println!("   T = Ticket      -> {:<36} same address: {}", type_name_of_val(&d), std::ptr::eq(d, ticket));
    println!("   Ticket is not Clone, and &Ticket still is: seat {}", d.seat);

    println!();
    println!("2. fn clone(&self) -> Self, with Self = &str");
    let e = clone_like_std(&text);
    println!("   self is &&str, the body is *self, the result is {}   same address: {}", type_name_of_val(&e), std::ptr::eq(e, text));

    println!();
    println!("3. The three kinds of \"duplicate\" the doc lists, measured");
    let s = String::from("Hello");
    let s2 = s.clone();
    println!("   String::clone     new buffer: {}", s.as_ptr() != s2.as_ptr());
    let r = &s;
    let r2 = Clone::clone(&r);
    println!("   <&String>::clone  same String: {}", std::ptr::eq(r, r2));
    let shared = Arc::new(String::from("Hello"));
    let shared2 = shared.clone();
    println!("   Arc::clone        same String: {}   strong_count: {}", Arc::ptr_eq(&shared, &shared2), Arc::strong_count(&shared));
    let list = vec![Rc::new(String::from("Hello"))];
    let list2 = list.clone();
    println!(
        "   Vec<Rc<String>>   new Vec buffer: {}   but same String inside: {}   (\"most types\" is not all)",
        list.as_ptr() != list2.as_ptr(),
        Rc::ptr_eq(&list[0], &list2[0])
    );

    println!();
    println!("4. The first doc example: its source starts with a hidden allow line");
    let hello = "Hello"; // &str implements Clone
    #[allow(noop_method_call)] // the hidden line; without it rustc warns here
    let cloned = hello.clone();
    assert_eq!("Hello", cloned);
    println!("   hello.clone() is {}, same address: {}   nothing was duplicated", type_name_of_val(&cloned), std::ptr::eq(hello, cloned));

    println!();
    println!("5. The second doc example: two Arc handles, one Mutex");
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    println!("   strong_count after the clone: {}", Arc::strong_count(&data));
    {
        let mut lock = data.lock().unwrap();
        lock.push(4);
    }
    assert_eq!(*data_clone.lock().unwrap(), vec![1, 2, 3, 4]);
    println!("   pushed 4 through data, read through data_clone: {:?}", data_clone.lock().unwrap());
    drop(data);
    println!("   after drop(data): strong_count {}, the vector still there: {:?}", Arc::strong_count(&data_clone), data_clone.lock().unwrap());

    println!();
    println!("6. The hover only lands on &T when T has no clone of its own");
    let owned = String::from("Hello");
    let borrowed = &owned;
    let got = borrowed.clone();
    println!("   (&String).clone() is {}   new buffer: {}   the hover shows impl Clone for String", type_name_of_val(&got), got.as_ptr() != owned.as_ptr());
}
